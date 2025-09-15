use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde::Serialize;
use serde_json::{Value, json};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::graph;
use crate::graph::message::ControlCommand;
use crate::graph::operator::{GraphOperatorContext, OperatorRole, OperatorState, OperatorType};
use crate::graph::operator_types::canonical_model::CanonicalModel;

use crate::my_reg::{ControlEvent, ControlEventType, broadcast_event, register};
use crate::operators::op_wrapper::OperatorWrapper;
use crate::ports::ports::Ports;
use crate::random::generate_random_id;
use crate::shared_resource::{SharedService, SharedServiceRef};

use super::super::operators::op_actor_handle::OperatorActorHandle;

use super::hooks::GraphHook;
use super::message::{self, DebuggableSpan, Message, OriginMessage};
use super::meta::NodeMeta;
use super::operator::{Operator, OperatorRef, OperatorRuntime, OperatorRuntimeType};
use super::operator_types::ai_tool::AIToolOperator;
use super::operator_types::canonical_model::CanonicalModelService;
use super::processors::basic::BasicProcessor;
use super::processors::port::Port;
use super::registry::GraphRegistry;
use super::watcher::watch_control_bus;
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
    //graph: Option<Arc<Mutex<Graph>>>,
    start_node: String,
    nodes: HashMap<String, Arc<dyn OperatorRuntime + 'static>>,
    hooks: Vec<Arc<dyn GraphHook + 'static>>,
}

pub trait IntoGraphRunner {
    fn into_graph_runner(self) -> Arc<GraphRunner>;
}

impl IntoGraphRunner for String {
    fn into_graph_runner(self) -> Arc<GraphRunner> {
        let graph_name = self;
        let graph = GraphRegistry::get_graph(&graph_name);
        if graph.is_none() {
            error!("Graph {} not found in registry", graph_name);
            panic!("Graph not found {}", graph_name);
        }
        let graph = graph.unwrap();
        let graph = graph.lock().unwrap();
        Arc::clone(&graph.runner)
    }
}

impl GraphRunner {
    pub fn new(name: String) -> Self {
        GraphRunner {
            name,
            //graph: None,
            state: GraphStatus::Inactive,
            start_node: "".to_string(),
            nodes: HashMap::new(),
            hooks: Vec::new(),
        }
    }

    // pub fn set_graph(&mut self, graph: Arc<Mutex<Graph>>) {
    //     self.graph = Some(graph);
    // }

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
        debug!("DROP GraphRunner for graph {}", self.name);
        // self.nodes.iter().for_each(|(id, op)| {
        //     debug!("Dropping node {} in GraphRunner", id);
        //     drop(op.to_owned());
        //     // op.lock().unwrap().finalize();
        // });
        // if let Some(graph) = &self.graph {
        //     let mut graph = graph.lock().unwrap();
        // }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub revision: u64,
    pub id: String,
    pub name: String,
    pub state: GraphStatus,
    start_node: String,
    /// Collection of nodes in the Graph, indexed by their unique ID
    pub mut_nodes: HashMap<String, Arc<Mutex<dyn Operator + 'static>>>,

    pub runner: Arc<GraphRunner>,

    pub node_meta_map: HashMap<String, NodeMeta>,

    /// Mapping of node IDs to their outgoing edges (children)
    edges: HashMap<String, HashSet<(String, String)>>,

    pub processor: Option<Arc<Mutex<BasicProcessor>>>,
    ports: Ports,
}

impl Drop for Graph {
    fn drop(&mut self) {
        debug!("DROP Graph for graph {}", self.name);
        self.mut_nodes.clear();
    }
}

#[async_trait]
pub trait AsyncHandleTrait: Sync + Send + Debug {
    async fn async_handle(&self, message: Message) -> Message;
    async fn async_handle_ptr(&self, message: Arc<Message>) -> Arc<Message>;
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum GraphStatus {
    Active,
    Inactive,
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
        let op = OperatorWrapper::new(self.name.clone(), id.to_string(), operator);

        // Get the current runtime and register it with the graph runner
        // let runtime = op.new_runtime();
        // self.runner.nodes.insert(id.to_string(), runtime);

        self.mut_nodes
            .insert(id.to_string(), Arc::new(Mutex::new(op)));

        self
    }
}

// impl<S> Graph<S>
// where
//     S: Clone + Send + Sync + 'static,
impl Graph {
    pub fn new(name: String) -> Self {
        Graph {
            revision: 0,
            id: generate_random_id(5).to_uppercase(),
            name: name.clone(),
            state: GraphStatus::Inactive,
            start_node: String::new(),
            //_nodes: HashMap::new(),
            mut_nodes: HashMap::new(),
            runner: Arc::new(GraphRunner::new(name)),
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
        if self.mut_nodes.len() == 0 {
            self.start_node = id.to_string();
            //self.runner.start_node = id.to_string();
        }

        let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());

        operator.init(self, &node_meta);

        self.node_meta_map.insert(id.to_string(), node_meta);

        info!("Node: {}({})", operator.name(), id.to_string());

        let typ = operator._type();

        let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());

        let op_id = id.to_string();
        let graph_name = self.name.clone();

        match typ {
            OperatorType::Filter2 { .. } => {
                let mut op = OperatorActorHandle::new(graph_name, op_id, operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::Filter => {
                let mut op = OperatorActorHandle::new(graph_name, op_id, operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::CanonicalModel {} => {
                let mut op = OperatorActorHandle::new(graph_name, op_id, operator);
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
        let mut nodes = self.mut_nodes.clone();
        for (id, operator) in self.mut_nodes.iter() {
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

    pub fn make_active_if_ready(&mut self) {
        // if self.runner.nodes.len() == 0 {
        //     warn!("Graph {} has no nodes, cannot make active", self.name);
        //     self.state = GraphStatus::Inactive;
        //     return;
        // }

        let mut pending = 0;
        for (id, runner) in self.runner.nodes.iter() {
            if runner.state() == OperatorState::Pending {
                debug!("Operator {} is pending", id);
                pending += 1;
            }
        }
        let current_state = self.state.clone();

        match pending {
            0 => {
                info!("Graph {} is now active", self.name);
                self.state = GraphStatus::Active;
            }
            _ => {
                warn!("Graph {} is not ready ({} pending)", self.name, pending);
                self.state = GraphStatus::Inactive;
            }
        };

        if current_state != self.state {
            info!("Graph {} TRANSITIONED TO {:?}", self.name, self.state);
        }
    }

    pub fn register(self) {
        let registry = GraphRegistry::get_instance();
        let mut registry = registry.lock().unwrap();
        info!("[{}] REGISTER GRAPH", self.name);
        registry.add_graph(self);
    }

    pub fn deregister(graph_name: String) {
        let graph_registry = GraphRegistry::get_instance();
        let mut graph_registry = graph_registry.lock().unwrap_or_else(|err| {
            error!("Failed to lock graph registry: {}", err);
            panic!("Failed to lock graph registry: {}", err);
        });
        let graph = graph_registry.delete_graph(&graph_name);
        match graph {
            Ok(graph) => {
                info!("Graph removed: {}", graph_name);
                let _graph = graph.lock().unwrap();
            }
            Err(err) => {
                error!("Failed to remove graph: {}", err);
            }
        }
    }

    // pub fn start_node(&self) -> &Arc<dyn OperatorRuntime + 'static> {
    //     self.runner.nodes.get(&self.start_node).unwrap()
    // }

    pub fn get_next_nodes(graph_operator_context: GraphOperatorContext) -> Vec<OperatorRole> {
        info!(
            "[{}] Getting next nodes for {}",
            graph_operator_context.id, graph_operator_context.node_fqn
        );
        let mut childs: Vec<OperatorRole> = Vec::new();

        let id = graph_operator_context.id.clone();

        let nodes = graph_operator_context.mut_nodes.clone();

        let children = graph_operator_context.get_next_edges();

        match children {
            Some(children) => {
                for child in children {
                    debug!("[{}] Adding child {} (role {})", id, child.1, child.0);
                    let opsch = nodes.get(&child.1);
                    match opsch {
                        Some(opsc) => {
                            let op = opsc.lock();
                            if op.is_err() {
                                warn!("Failed to lock operator {} - skipping", child.1);
                                continue;
                            }
                            let op = op.unwrap();

                            let node_fqn = graph_operator_context
                                .node_meta_map
                                .get(&child.1)
                                .map(|meta| meta.fqn())
                                .unwrap_or_else(|| "N/A".to_string());

                            let child_graph_operator_context = GraphOperatorContext {
                                mut_nodes: graph_operator_context.mut_nodes.clone(),
                                edges: graph_operator_context.edges.clone(),
                                node_meta_map: graph_operator_context.node_meta_map.clone(),
                                node_fqn,
                                id: child.1.clone(),
                            };

                            let rt = op.new_runtime(child_graph_operator_context);

                            childs.push(OperatorRole {
                                role: child.0.clone(),
                                operator: rt,
                            });
                        }
                        None => {
                            panic!(
                                "Child {} not found in graph: {} (role {})",
                                child.1, child.0, graph_operator_context.node_fqn
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
            let opsch = nodes.get("_end");
            match opsch {
                Some(opsc) => childs.push(OperatorRole {
                    role: OperatorRole::default(),
                    operator: opsc
                        .lock()
                        .unwrap()
                        .new_runtime(graph_operator_context.clone()),
                }),
                None => {}
            };
        }

        childs
    }

    pub fn watch(&self) -> &Self {
        watch_control_bus(self.id.clone(), self.name.clone());
        self
    }

    pub fn post_start_hook(&self) {
        let start = self.mut_nodes.get(&self.start_node);

        if start.is_none() {
            error!("No start node found in graph {}", self.name);
            return;
        }

        let start_runtime = self.runner.start_node();
        if start_runtime.is_none() {
            error!("No start node runtime found in graph {}", self.name);
            return;
        }

        let start = start.unwrap();
        start.lock().unwrap().control(Message::Control {
            command: ControlCommand::Start {
                runtime: Arc::clone(&start_runtime.unwrap()),
            },
            origin: None,
        })
    }

    pub fn shared_resources(&self) -> Vec<SharedServiceRef> {
        let mut list = vec![];
        self.mut_nodes.iter().for_each(|(_id, op)| {
            let op = op.lock().unwrap();
            let resources = op.shared_resources();

            info!("{} :Adding...{} shared resources", _id, resources.len());
            list.extend(resources);
        });
        list
    }

    // pub fn init(&self, nodes: HashMap<String, Arc<dyn OperatorRuntime>>) -> &Self {
    //     //self.init_ports();

    //     // let nodes = &self.runner.nodes;

    //     debug!("INIT with start node {}", &self.runner.start_node);
    //     let start = nodes.get(&self.runner.start_node).unwrap();
    //     let end = nodes.get("_end").unwrap();

    //     //let end_ro = self.runner.nodes.get("_end").unwrap();

    //     //let processor = self.processor.unwrap();

    //     // Initialize the graph
    //     for (id, operator) in nodes.iter() {
    //         if id == "_end" {
    //             continue;
    //         }

    //         let childs = self.get_next_nodes(&nodes, id);

    //         // send a control message to the Operator
    //         let mut_op = self.mut_nodes.get(id).unwrap();

    //         // tell the operator to initialize itself
    //         mut_op.lock().unwrap().control(Message::Init {
    //             next: childs,
    //             id: id.to_string(),
    //             end: Arc::clone(end),
    //             start: Arc::clone(start),
    //             op_runtime: Arc::clone(operator),
    //         });
    //     }

    //     for (id, op) in self.mut_nodes.iter() {
    //         debug!("Inventory: [{:?}] {}", op.lock().unwrap().state(), id);
    //     }
    //     for (id, _) in self.ports.ports.iter() {
    //         debug!("Ports: {}", id);
    //     }

    //     //watch_control_bus(self.id.clone(), self.name.clone());

    //     self
    // }

    // pub fn deprecate_finalize(&mut self) -> bool {
    //     let mut pending = 0;
    //     for (id, op) in self.mut_nodes.iter() {
    //         debug!("Inventory: [{:?}] {}", op.lock().unwrap().state(), id);
    //         if op.lock().unwrap().state() == OperatorState::Pending {
    //             pending += 1;
    //         }
    //     }

    //     let graph_id = self.id.clone();
    //     let graph_name = self.name.clone();

    //     if pending > 0 {
    //         info!(
    //             "[{}] {} STARTING (pending {})",
    //             graph_id, graph_name, pending
    //         );
    //         tokio::spawn(async move {
    //             broadcast_event(ControlEvent {
    //                 graph_id,
    //                 graph_name,
    //                 state: GraphStatus::Inactive,
    //                 operator_names: vec![],
    //             })
    //             .await;
    //         });
    //     } else {
    //         info!("[{}] {} READY", graph_id, graph_name);
    //         tokio::spawn(async move {
    //             broadcast_event(ControlEvent {
    //                 graph_id,
    //                 graph_name: graph_name.clone(),
    //                 state: GraphStatus::Active,
    //                 operator_names: vec![],
    //             })
    //             .await;
    //         });
    //     }
    //     pending == 0
    // }

    // pub fn poke(&mut self) {
    //     info!("Poke ENTER");
    //     self.mut_nodes.iter().for_each(|(_id, op)| {
    //         let mut op = op.lock().unwrap();
    //         op.finalize();
    //     });
    //     self.finalize();
    //     info!("Poke EXIT");
    // }

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

    /**
     * Operator has new information, so recreate the
     */
    pub fn runtime_expired(self_graph_name: String, node_id: String) {
        info!("[{}] Operator expired - {}", self_graph_name, node_id);

        let self_graph = GraphRegistry::get_graph(&self_graph_name);
        if self_graph.is_none() {
            error!("Graph {} not found in registry", self_graph_name);
            panic!("Graph not found {}", self_graph_name);
        }
        let self_graph = self_graph.unwrap();
        let mut self_graph = self_graph.lock().unwrap();

        self_graph.replace_runner();

        //self_graph.init(self_graph.runner.nodes.clone());
    }

    // pub fn replace_all_runtimes(&self) -> Arc<GraphRunner> {
    //     info!("Replacing all runtimes for graph {}", self.name);
    //     // Setup the first iteration of the graph runner

    //     let mut nodes = HashMap::new();
    //     // This is NOT GOING TO WORK
    //     // -- get_next_nodes() is using the old nodes
    //     // and we are building a new set of nodes
    //     for (id, operator) in self.mut_nodes.iter() {
    //         let op = operator.lock().unwrap();

    //         let runtime = op.new_runtime();
    //         nodes.insert(id.clone(), runtime);
    //     }

    //     info!("Getting graph {} from registry", self.name);
    //     let graph = GraphRegistry::get_graph(&self.name);

    //     info!("Setting running graph for {}", self.name);
    //     Arc::new(GraphRunner {
    //         name: self.name.clone(),
    //         state: self.state.clone(),
    //         start_node: self.start_node.clone(),
    //         graph,
    //         nodes,
    //         hooks: self.runner.hooks.clone(),
    //     })
    // }

    pub fn replace_runner(&mut self) {
        info!("Replacing runner for graph: {}", self.name);

        self.revision += 1;

        let revision = self.revision;

        let start_node = self.start_node.clone();
        let mut_nodes = self.mut_nodes.clone();
        let edges = self.edges.clone();
        let node_meta = self.node_meta_map.clone();

        let mut new_runner = GraphRunner {
            name: self.name.clone(),
            state: GraphStatus::Inactive,
            start_node: self.start_node.clone(),
            nodes: HashMap::new(),
            hooks: self.runner.hooks.clone(),
        };

        let mut event = ControlEvent {
            graph_id: self.id.clone(),
            graph_name: self.name.clone(),
            graph_status: GraphStatus::Inactive,
            operator_names: Vec::new(),
            event_type: ControlEventType::GraphReplaced,
            reason: format!("Graph runner updated (rev.{})", revision),
        };

        let timelimit = Duration::from_secs(10);

        tokio::spawn(timeout(timelimit, async move {
            info!(
                "Generating new runtimes for graph: {} (rev.{})",
                new_runner.name, revision
            );
            let new_runtimes =
                Graph::generate_new_runtimes(mut_nodes, start_node, edges, node_meta);
            info!("Done new runtimes");
            let self_name = new_runner.name.clone();

            let pending_count = new_runtimes
                .values()
                .filter(|op| op.state() == OperatorState::Pending)
                .count();

            new_runner.replace_nodes(new_runtimes);

            let self_graph = GraphRegistry::get_graph(&self_name);
            if self_graph.is_none() {
                error!("Graph {} not found in registry", self_name);
                return;
            }

            let self_graph = self_graph.unwrap();
            let mut self_graph = self_graph.lock().unwrap();

            if revision < self_graph.revision {
                warn!(
                    "[Graph={}] Revision mismatch - not performing update: {} != {}",
                    self_name, self_graph.revision, revision
                );
                return;
            }

            info!("[Graph={}] Pending count: {}", self_name, pending_count);
            self_graph.state = if pending_count > 0 {
                GraphStatus::Inactive
            } else {
                GraphStatus::Active
            };

            event.graph_status = self_graph.state.clone();

            if &self_graph.state == &GraphStatus::Inactive {
                event.reason = format!("{} - INACTIVE", event.reason);
            }
            new_runner.state = self_graph.state.clone();

            self_graph.runner = Arc::new(new_runner);

            // info!("New runner for graph {} replaced.", self.name);

            info!(
                "Graph BUILT: {} : Manager:{:?}, Runner:{:?}",
                self_name, self_graph.state, self_graph.runner.state
            );

            tokio::spawn(async move {
                broadcast_event(event).await;
            });
        }));
    }

    pub fn generate_new_runtimes(
        mut_nodes: HashMap<String, OperatorRef>,
        start_node: String,
        edges: HashMap<String, HashSet<(String, String)>>,
        node_meta: HashMap<String, NodeMeta>,
    ) -> HashMap<String, OperatorRuntimeType> {
        let mut nodes = HashMap::new();

        // let id = start_node.clone();
        // let operator = mut_nodes.get(&start_node).unwrap();
        for (id, operator) in mut_nodes.iter() {
            let runtime = {
                let op = operator.lock().unwrap();

                let node_fqn = node_meta
                    .get(id)
                    .map(|meta| meta.fqn())
                    .unwrap_or_else(|| "N/A".to_string());

                let graph_operator_context = GraphOperatorContext {
                    mut_nodes: mut_nodes.clone(),
                    edges: edges.clone(),
                    node_meta_map: node_meta.clone(),
                    node_fqn,
                    id: id.clone(),
                };
                let runtime = op.new_runtime(graph_operator_context);
                runtime
            };
            nodes.insert(id.clone(), runtime);
        }

        nodes
    }

    // pub fn refresh_runtime_node(&mut self, id: String) {
    //     let mut nodes = self.runner.nodes.clone();

    //     let operator = self.mut_nodes.get(&id).unwrap();

    //     let mut op = operator.lock().unwrap();

    //     op.control(Message::Init2 {
    //         id: id.clone(),
    //         next: self.get_next_nodes(&nodes, &id),
    //     });

    //     let runtime = op.new_runtime();
    //     nodes.insert(id.clone(), runtime);

    //     info!("Setting running graph for {}", self.name);
    //     self.runner = Arc::new(GraphRunner {
    //         name: self.name.clone(),
    //         state: self.state.clone(),
    //         start_node: self.start_node.clone(),
    //         nodes,
    //         hooks: self.runner.hooks.clone(),
    //     });

    //     let event = ControlEvent {
    //         graph_id: self.id.clone(),
    //         graph_name: self.name.clone(),
    //         operator_names: self.mut_nodes.keys().cloned().collect::<Vec<String>>(),
    //         event_type: ControlEventType::Notice,
    //         reason: "Graph runner updated".to_string(),
    //     };

    //     tokio::spawn(async move {
    //         broadcast_event(event).await;
    //     });
    // }

    // fn new_runtime(&self, id: String) -> Arc<GraphRunner> {
    //     info!("Replacing all runtimes for graph {}", self.name);
    //     // Setup the first iteration of the graph runner

    //     let mut nodes = self.runner.nodes.clone();

    //     // This is NOT GOING TO WORK
    //     // -- get_next_nodes() is using the old nodes
    //     // and we are building a new set of nodes
    //     let operator = self.mut_nodes.get(&id).unwrap();
    //     {
    //         let op = operator.lock().unwrap();
    //         let runtime = op.new_runtime();
    //         nodes.remove(&id);
    //         nodes.insert(id.clone(), runtime);
    //     }

    //     info!("Getting graph {} from registry", self.name);
    //     let graph = GraphRegistry::get_graph(&self.name);

    //     info!("Setting running graph for {}", self.name);
    //     Arc::new(GraphRunner {
    //         name: self.name.clone(),
    //         state: self.state.clone(),
    //         start_node: self.start_node.clone(),
    //         graph,
    //         nodes,
    //         hooks: self.runner.hooks.clone(),
    //     })
    // }

    pub fn apply_settings(&self, settings: &Value) {
        let graph_name = self.name.clone();
        let graph_name = graph_name.as_str();

        let update_settings = settings.get(graph_name);
        if update_settings.is_none() {
            return;
        }

        self.mut_nodes.iter().for_each(|operator| {
            info!("[{}] Operator: {}", graph_name, operator.0);

            let update_settings = update_settings.unwrap().get(operator.0);
            if update_settings.is_none() {
                return;
            }
            let update_settings = update_settings.unwrap();

            let set_settings = update_settings
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect::<HashMap<String, serde_json::Value>>();

            match operator.1.try_lock() {
                Ok(mut op) => {
                    info!("Operator: {} {:?}", op.name(), set_settings);
                    op.control(Message::Control {
                        command: ControlCommand::SetSettings {
                            settings: set_settings,
                        },
                        origin: None,
                    });
                }
                Err(_) => {
                    warn!("Failed to lock operator - skipping settings");
                }
            }
        });
    }

    pub fn call(&self, message: Message) {
        //let node = self.nodes.get(&self.start_node).unwrap().clone();

        // let node = node.lock().unwrap();
        // node.send(message);

        self.runner.call(message);
    }
}
