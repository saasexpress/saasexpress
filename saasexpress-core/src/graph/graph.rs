use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde::Serialize;
use serde_json::{Value, json};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::graph::operator::{OperatorRole, OperatorState, OperatorType};
use crate::graph::operator_types::canonical_model::CanonicalModel;

use crate::graph::watcher::watch_control_bus;
use crate::my_reg::{ControlEvent, broadcast_event, register};
use crate::operators::op_wrapper::OperatorWrapper;
use crate::ports::ports::Ports;
use crate::random::generate_random_id;

use super::super::operators::op_actor_handle::OperatorActorHandle;

use super::hooks::GraphHook;
use super::message::{self, DebuggableSpan, Message, OriginMessage};
use super::meta::NodeMeta;
use super::operator::{Operator, OperatorRuntime};
use super::operator_types::ai_tool::AIToolOperator;
use super::operator_types::canonical_model::CanonicalModelOperator;
use super::processors::basic::BasicProcessor;
use super::processors::port::Port;
use super::registry::GraphRegistry;
use async_trait::async_trait;
use std::ops::Deref;

pub trait GraphMod {
    fn add_new_node<O>(&mut self, id: &str, operator: O) -> &mut Self
    where
        O: Operator + 'static;
}

#[derive(Debug, Clone)]
pub struct GraphRunner {
    pub name: String,
    pub state: GraphStatus,
    graph: Option<Arc<Mutex<Graph>>>,
    start_node: String,
    nodes: HashMap<String, Arc<dyn OperatorRuntime + 'static>>,
    hooks: Vec<Arc<dyn GraphHook + 'static>>,
}

impl GraphRunner {
    pub fn new(name: String) -> Self {
        GraphRunner {
            name,
            graph: None,
            state: GraphStatus::Starting,
            start_node: "".to_string(),
            nodes: HashMap::new(),
            hooks: Vec::new(),
        }
    }

    pub fn set_graph(&mut self, graph: Arc<Mutex<Graph>>) {
        self.graph = Some(graph);
    }

    pub fn start_node(&self) -> Option<&Arc<dyn OperatorRuntime + 'static>> {
        self.nodes.get(&self.start_node)
    }

    pub fn call(&self, message: Message) {
        let node = self.nodes.get(&self.start_node);
        if node.is_none() {
            error!("No start node found in graph {}", self.name);
            return;
        }

        // Call all the on_call hooks, then send to starting node
        node.unwrap().send(message);
    }

    pub fn replace_nodes(&mut self, nodes: HashMap<String, Arc<dyn OperatorRuntime + 'static>>) {
        self.nodes = nodes;
    }
}

impl Drop for GraphRunner {
    fn drop(&mut self) {
        error!("Dropping GraphRunner for graph {}", self.name);
        if let Some(graph) = &self.graph {
            let mut graph = graph.lock().unwrap();
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub id: String,
    pub name: String,
    pub state: GraphStatus,
    start_node: String,

    /// Collection of nodes in the Graph, indexed by their unique ID
    pub nodes: HashMap<String, Arc<Mutex<dyn Operator + 'static>>>,

    pub runner: GraphRunner,
    pub node_meta_map: HashMap<String, NodeMeta>,

    /// Mapping of node IDs to their outgoing edges (children)
    edges: HashMap<String, HashSet<(String, String)>>,

    pub processor: Option<Arc<Mutex<BasicProcessor>>>,
    ports: Ports,
}

impl Drop for Graph {
    fn drop(&mut self) {
        error!("Dropping Graph for graph {}", self.name);
    }
}

#[async_trait]
pub trait AsyncHandleTrait: Sync + Send + Debug {
    async fn async_handle(&self, message: Message) -> Message;
    async fn async_handle_ptr(&self, message: Arc<Message>) -> Arc<Message>;
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum GraphStatus {
    // The graph is starting
    Starting,
    /// The graph is running
    Running,
    /// The graph has an error
    Error,
    /// The graph is paused
    Paused,
    /// The graph is being replaced
    Replacing,
}

// #[derive(Debug)]
// pub struct GraphBroadcastMessage {
//     pub name: String,
//     pub status: GraphStatus,
// }

impl GraphMod for Graph {
    fn add_new_node<O>(&mut self, id: &str, operator: O) -> &mut Self
    where
        O: Operator + 'static,
    {
        let op = OperatorWrapper::new(operator);

        // Get the current runtime and register it with the graph runner
        let runtime = op.new_runtime();
        self.runner.nodes.insert(id.to_string(), runtime);

        self.nodes.insert(id.to_string(), Arc::new(Mutex::new(op)));

        self
    }
}

// impl<S> Graph<S>
// where
//     S: Clone + Send + Sync + 'static,
impl Graph {
    pub fn new(name: String) -> Self {
        Graph {
            id: generate_random_id(5).to_uppercase(),
            name: name.clone(),
            state: GraphStatus::Starting,
            start_node: String::new(),
            //_nodes: HashMap::new(),
            nodes: HashMap::new(),
            runner: GraphRunner::new(name),
            node_meta_map: HashMap::new(),
            edges: HashMap::new(),
            processor: None,
            ports: Ports {
                ports: HashMap::new(),
            },
            //broker: Broker::get_instance(),
        }
    }

    pub fn no_processor(&mut self) -> &mut Self {
        self.processor = Some(Arc::new(Mutex::new(BasicProcessor::new(self))));
        self
    }

    pub fn add_node<O>(&mut self, id: &str, mut operator: O) -> &mut Self
    where
        O: Operator + 'static,
    {
        if self.nodes.len() == 0 {
            self.start_node = id.to_string();
            self.runner.start_node = id.to_string();
        }

        let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());

        operator.init(self, &node_meta);

        self.node_meta_map.insert(id.to_string(), node_meta);

        info!("Node: {}({})", operator.name(), id.to_string());

        let typ = operator._type();

        let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());

        match typ {
            OperatorType::Filter2 { .. } => {
                let mut op = OperatorActorHandle::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::Filter => {
                let mut op = OperatorActorHandle::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::CanonicalModel {} => {
                let mut op = OperatorActorHandle::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            _ => {
                self.add_new_node(id, operator);
            }
        }

        self
    }

    /// Adds a directed edge between two nodes
    ///
    /// Returns an error if:
    /// - Either node doesn't exist in the Graph
    /// - Adding the edge would create a cycle
    pub fn add_edge(&mut self, from: String, to: String, role: String) -> &mut Self {
        // if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
        //     panic!("Both nodes must be in the Graph");
        // }
        // if self.has_path(&to, &from) {
        //     return Err("Adding this edge would create a cycle".to_string());
        // }

        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert((role, to));

        self
    }

    pub fn init_ports(&mut self) {
        let mut nodes = self.nodes.clone();
        for (id, operator) in self.nodes.iter() {
            if id == "_end" {
                continue;
            }

            let op = operator.lock().unwrap();
            if op._type() == OperatorType::Endpoint {
                let port = self.ports.new_port(id.clone());
                nodes.insert(id.to_string(), Arc::new(Mutex::new(port)));
            }

            // ports.push(port);
        }
    }

    pub fn register(self) {
        let registry = GraphRegistry::get_instance();
        let mut registry = registry.lock().unwrap();
        registry.add_graph(self);
    }

    pub fn start_node(&self) -> &Arc<Mutex<dyn Operator + 'static>> {
        self.nodes.get(&self.start_node).unwrap()
    }

    pub fn init(&mut self) -> &mut Self {
        self.init_ports();

        debug!("INIT with start node {}", &self.start_node);
        let start = self.nodes.get(&self.start_node).unwrap();
        let end = self.nodes.get("_end").unwrap();

        //let end_ro = self.runner.nodes.get("_end").unwrap();

        //let processor = self.processor.unwrap();

        // Initialize the graph
        for (id, operator) in self.nodes.iter() {
            if id == "_end" {
                continue;
            }

            let mut childs: Vec<OperatorRole> = Vec::new();

            let children = self.edges.get(id);
            match children {
                Some(children) => {
                    for child in children {
                        debug!("Adding child {} (role {})", child.1, child.0);
                        let opsch = self.nodes.get(&child.1);
                        match opsch {
                            Some(opsc) => childs.push(OperatorRole {
                                role: child.0.clone(),
                                operator: Arc::clone(opsc),
                            }),
                            None => {
                                panic!(
                                    "Child {} not found in graph: {} (role {})",
                                    child.1, child.0, self.name
                                );
                            }
                        }
                    }
                }
                None => {}
            }

            // if there are no edges/children for the node, then have it go to "_end"
            // if it exists
            if children
                .unwrap_or(&HashSet::new())
                .iter()
                .filter(|x| x.0 == OperatorRole::default())
                .count()
                == 0
            {
                let opsch = self.nodes.get("_end");
                match opsch {
                    Some(opsc) => childs.push(OperatorRole {
                        role: OperatorRole::default(),
                        operator: Arc::clone(opsc),
                    }),
                    None => {}
                }
            }

            // tell the operator to initialize itself
            operator.lock().unwrap().control(Message::Init {
                next: childs,
                id: id.to_string(),
                end: Arc::clone(end),
                start: Arc::clone(start),
                //processor: Arc::new(Mutex::new(processor)),
            });
        }

        for (id, op) in self.nodes.iter() {
            debug!("Inventory: [{:?}] {}", op.lock().unwrap().state(), id);
        }
        for (id, _) in self.ports.ports.iter() {
            debug!("Ports: {}", id);
        }

        watch_control_bus(self.id.clone(), self.name.clone());

        self
    }

    pub fn finalize(&mut self) -> bool {
        let mut pending = 0;
        for (id, op) in self.nodes.iter() {
            debug!("Inventory: [{:?}] {}", op.lock().unwrap().state(), id);
            if op.lock().unwrap().state() == OperatorState::Pending {
                pending += 1;
            }
        }

        let graph_id = self.id.clone();
        let graph_name = self.name.clone();

        if pending > 0 {
            info!(
                "[{}] {} STARTING (pending {})",
                graph_id, graph_name, pending
            );
            tokio::spawn(async move {
                broadcast_event(ControlEvent {
                    graph_id,
                    graph_name,
                    state: GraphStatus::Starting,
                    operator_names: vec![],
                })
                .await;
            });
        } else {
            info!("[{}] {} READY", graph_id, graph_name);
            tokio::spawn(async move {
                broadcast_event(ControlEvent {
                    graph_id,
                    graph_name: graph_name.clone(),
                    state: GraphStatus::Running,
                    operator_names: vec![],
                })
                .await;
            });
        }

        // let result = b_tx.send(format!("Graph {} has {} pending.", self.name, pending));
        // match result {
        //     Ok(_) => debug!("Graph {} has {} pending.", self.name, pending),
        //     Err(_) => debug!("Failed to send message"),
        // }

        // if pending > 0 {
        //     tokio::spawn(async move {
        //         let result = b_rx.recv().await;
        //     });
        // }
        // //self.monitor = Some(monitor);
        // let mut waiting = Vec::new();

        // self.nodes.iter().for_each(|(_id, op)| {
        //     let mut op = op.lock().unwrap();
        //     op.finalize();
        //     if op.state() != OperatorState::Ready {
        //         waiting.push(op);
        //     }
        // });

        // waiting.iter_mut().for_each(|op| {
        //     debug!("Waiting for operator {} to be ready", op.name());
        //     op.finalize();
        //     if op.state() != OperatorState::Ready {
        //         panic!("Operator {} is still not ready", op.name());
        //     }
        // });
        // info!("All operators are ready");
        pending == 0
    }

    pub fn poke(&mut self) {
        info!("Poke ENTER");
        self.nodes.iter().for_each(|(_id, op)| {
            let mut op = op.lock().unwrap();
            op.finalize();
        });
        self.finalize();
        info!("Poke EXIT");
    }

    // pub async fn event(&mut self, status: GraphStatus) {
    //     let result = self
    //         .monitor
    //         .send(GraphBroadcastMessage {
    //             name: self.name.clone(),
    //             status,
    //         })
    //         .await;

    //     match result {
    //         Ok(_) => {
    //             info!("Graph {} started", self.name);
    //         }
    //         Err(_) => {
    //             error!("Failed to send message to monitor");
    //         }
    //     }
    // }

    // pub fn broadcast(&self, message: GraphBroadcastMessage) {
    //     for (id, operator) in self.nodes.iter() {
    //         if id == "_end" {
    //             continue;
    //         }

    //         let node = operator.lock().unwrap();
    //         node.send(message.clone());
    //     }
    // }

    pub fn call(&self, message: Message) {
        //let node = self.nodes.get(&self.start_node).unwrap().clone();

        // let node = node.lock().unwrap();
        // node.send(message);

        self.runner.call(message);
    }
}
