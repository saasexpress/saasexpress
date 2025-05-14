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

use crate::broker::Broker;
use crate::control_bus::ControlEvent;
use crate::graph::operator_types::canonical_model::CanonicalModel;

use crate::my_reg::{broadcast_event, register};
use crate::operators::op_wrapper::OperatorWrapper;
use crate::ports::ports::Ports;

use super::super::operators::op_actor_handle::OperatorActorHandle;

use super::message::{self, DebuggableSpan, Message, OriginMessage};
use super::meta::NodeMeta;
use super::operator_types::ai_tool::AIToolOperator;
use super::operator_types::canonical_model::CanonicalModelOperator;
use super::processors::basic::BasicProcessor;
use super::processors::port::Port;
use super::registry::GraphRegistry;
use async_trait::async_trait;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Origin<T> {
    pub inner: Arc<T>,
}

impl<T> Origin<T> {
    pub fn new(inner: T) -> Self {
        Origin {
            inner: Arc::new(inner),
        }
    }
}
impl<T> Clone for Origin<T> {
    fn clone(&self) -> Self {
        Origin {
            inner: Arc::clone(&self.inner),
        }
    }
}
impl<T> DerefMut for Origin<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::get_mut(&mut self.inner).unwrap()
    }
}
impl<T> Deref for Origin<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<Arc<T>> for Origin<T> {
    fn from(inner: Arc<T>) -> Self {
        Origin { inner }
    }
}
impl<T> From<Origin<T>> for Arc<T> {
    fn from(origin: Origin<T>) -> Self {
        origin.inner
    }
}

pub trait GraphMod {
    fn add_new_node<O>(&mut self, id: &str, operator: O) -> &mut Self
    where
        O: Operator + 'static;
}

#[derive(Debug)]
pub struct EdgeDestination {
    pub to: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct OperatorRole {
    pub role: String,
    pub operator: OperatorRef,
}

impl OperatorRole {
    pub fn default() -> String {
        "default".to_string()
    }
}

#[derive(Debug)]
pub struct Graph {
    pub name: String,
    pub state: GraphStatus,
    start_node: String,
    broker: &'static std::sync::Mutex<Broker>,
    // monitor: mpsc::Sender<GraphBroadcastMessage>,
    /// Collection of nodes in the Graph, indexed by their unique ID
    pub nodes: HashMap<String, Arc<Mutex<dyn Operator + 'static>>>,

    pub node_meta_map: HashMap<String, NodeMeta>,

    /// Mapping of node IDs to their outgoing edges (children)
    edges: HashMap<String, HashSet<(String, String)>>,

    pub processor: Option<Arc<Mutex<BasicProcessor>>>,
    ports: Ports,
}

pub trait InitOperator {
    fn start(&self) -> impl std::future::Future<Output = ()> + Send;
}

#[async_trait]
pub trait AsyncHandleTrait: Sync + Send + Debug {
    async fn async_handle(&self, message: Message) -> Message;
    async fn async_handle_ptr(&self, message: Arc<Message>) -> Arc<Message>;
}

pub trait Filter2Operator: Sync + Send + Debug {
    fn handle(&self, message: Message) -> Message;
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum GraphStatus {
    // The graph is starting
    Starting,
    /// The graph is running
    Running,
    /// The graph is stopped
    Stopped,
    /// The graph is paused
    Paused,
}

#[derive(Debug)]
pub struct GraphBroadcastMessage {
    pub name: String,
    pub status: GraphStatus,
}
pub type OperatorRef = Arc<Mutex<(dyn Operator + 'static)>>;

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorState {
    Pending,
    Ready,
}

#[derive(Debug)]
pub enum OperatorType {
    /// Represents an endpoint operator
    /// that receives messages from the outside world
    /// and sends them to the next operator
    /// in the graph
    /// e.g. HTTPIn, MQTTIn, etc.
    Endpoint,
    /// Represents a filter operator
    /// that processes messages and sends them to the next operator
    /// in the graph
    /// e.g. HTTPOut, MQTTOut, etc.
    Filter,

    Filter2 {
        operator: Arc<dyn Filter2Operator + 'static>,
    },

    // Represents a canonical model operator
    CanonicalModel,

    /// Represents an AI agent operator
    AIAgent,

    /// Represents an AI tool operator
    AITool {
        tool: Arc<dyn AIToolOperator + 'static>,
    },
}

impl PartialEq for OperatorType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OperatorType::Endpoint, OperatorType::Endpoint) => true,
            (OperatorType::Filter, OperatorType::Filter) => true,
            (OperatorType::Filter2 { operator: _ }, OperatorType::Filter2 { operator: _ }) => true,
            (OperatorType::CanonicalModel, OperatorType::CanonicalModel) => true,
            (OperatorType::AIAgent, OperatorType::AIAgent) => true,
            (OperatorType::AITool { tool: _ }, OperatorType::AITool { tool: _ }) => true,
            _ => false,
        }
    }
}

// Operator trait - defines how to process a message
pub trait Operator: Send + Sync + Debug {
    fn _type(&self) -> OperatorType;
    fn name(&self) -> String;
    fn state(&self) -> OperatorState {
        OperatorState::Ready
    }

    //fn meta(&self) -> NodeMeta;

    //fn init(&mut self, graphs: Vec<&mut Graph>);
    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        debug!("Default init operator {} - no action", self.name());
    }

    fn finalize(&mut self) -> bool {
        debug!("Default finalize operator {} - no action", self.name());
        true
    }

    /// performs the work of the operator
    fn handle(&self, message: Message) -> Message;

    /// commands and events for controlling the operator
    fn control(&mut self, message: Message);

    /// Sends a message to this operator for handling
    ///
    /// Examples:
    /// OperatorType::Endpoint (HTTPIn) : self.next(self.handle(message));
    /// OperatorType::Endpoint (FanOut) : self.next(message); (handle Not implemented)
    /// ActorHandle : Send to actor to handle message; (handle Not implemented)
    /// OperatorType::Filter (ActorHandle) : Send to actor to handle message and actor sends to next operators
    ///
    fn send(&self, message: Message);

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>>;
    fn wait(&self) -> Message;

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }
    fn handle_ptr(&self, message: Arc<Message>) -> Arc<Message> {
        debug!("default handle (passthrough)... {}", self.name());
        return message;
    }
    fn next_ptr(&self, message: Arc<Message>) {
        // Sending message to next operator
        for n in self.get_output_channels() {
            n.lock().unwrap().send_ptr(message.to_owned());
            //break;
        }
    }
    fn get_output_channels(&self) -> &Vec<OperatorRef>;
}

impl GraphMod for Graph {
    fn add_new_node<O>(&mut self, id: &str, operator: O) -> &mut Self
    where
        O: Operator + 'static,
    {
        // if self.nodes.len() == 0 {
        //     self.start_node = id.to_string();
        // }
        //self.start_node = "in".to_string();

        self.nodes
            .insert(id.to_string(), Arc::new(Mutex::new(operator)));

        self
    }
}

pub trait GraphRun {
    async fn end_to_end(&mut self, message: Vec<u8>) -> Message;
    async fn end_to_end_json(&mut self, message: Value) -> Message;
    async fn end_to_end_standard(&mut self, message: Vec<u8>) -> Message;

    async fn end_to_end_2(&mut self, message: Vec<u8>) -> Message;

    async fn process(&mut self, message: Message) -> Message;
}

// impl<S> Graph<S>
// where
//     S: Clone + Send + Sync + 'static,
impl Graph {
    pub fn new(name: String) -> Self {
        Graph {
            name,
            state: GraphStatus::Starting,
            start_node: String::new(),
            //_nodes: HashMap::new(),
            nodes: HashMap::new(),
            node_meta_map: HashMap::new(),
            edges: HashMap::new(),
            processor: None,
            ports: Ports {
                ports: HashMap::new(),
            },
            broker: Broker::get_instance(),
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
        }

        let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());

        operator.init(self, &node_meta);

        self.node_meta_map.insert(id.to_string(), node_meta);

        info!("Node: {}({})", operator.name(), id.to_string());

        let typ = operator._type();

        match typ {
            OperatorType::Endpoint => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
                //Port::new(self, "abc".to_string());
            }
            OperatorType::Filter2 { .. } => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(OperatorActorHandle::new(operator));
                op.init(self, &node_meta);

                self.add_new_node(id, op);
            }
            OperatorType::Filter => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(OperatorActorHandle::new(operator));
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::CanonicalModel {} => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(OperatorActorHandle::new(operator));
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::AIAgent {} => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
            }
            OperatorType::AITool { .. } => {
                let node_meta = NodeMeta::new(self.name.as_str(), id, operator.name());
                let mut op = OperatorWrapper::new(operator);
                op.init(self, &node_meta);
                self.add_new_node(id, op);
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

    pub fn start_node(&mut self) -> &Arc<Mutex<dyn Operator + 'static>> {
        self.nodes.get(&self.start_node).unwrap()
    }

    pub fn init(&mut self) -> &mut Self {
        self.init_ports();

        debug!("INIT with start node {}", &self.start_node);
        let start = self.nodes.get(&self.start_node).unwrap();
        let end = self.nodes.get("_end").unwrap();

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

        watch_control_bus(self.name.clone());

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

        let graph_name = self.name.clone();

        if pending > 0 {
            info!("[{}] PENDING (count={})", graph_name, pending);
            info!("Graph {} has {} pending.", graph_name, pending);
            tokio::spawn(async move {
                broadcast_event(ControlEvent {
                    graph_name: graph_name.clone(),
                    state: GraphStatus::Starting,
                    operator_names: vec![],
                })
                .await;
            });
        } else {
            info!("[{}] READY", graph_name);
            tokio::spawn(async move {
                broadcast_event(ControlEvent {
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
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();
        node.send(message);
    }

    pub fn base_env_vars_settings(&self, node_meta: &NodeMeta) -> String {
        format!("{}_{}_", node_meta.graph, node_meta.name)
            .replace("-", "_")
            .to_uppercase()
    }
}

impl GraphRun for Graph {
    async fn end_to_end(&mut self, message: Vec<u8>) -> Message {
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let temp = json!({
            "path": "".to_string(),
            "method": "".to_string(),
            "query": "".to_string(),
        });

        node.send(Message::ReqReply {
            message,
            respond_to,
            temp: Arc::new(Mutex::new(temp)),
            span: Some(DebuggableSpan(root_span)),
        });

        recv.await.unwrap()
    }

    async fn end_to_end_json(&mut self, message: Value) -> Message {
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let origin = OriginMessage::new(Some(respond_to))
            .session("0".to_string())
            .with_span(Some(DebuggableSpan(root_span)));

        node.send(Message::JSON {
            message,
            origin: Some(origin),
        });

        match recv.await {
            Ok(message) => message,
            Err(_) => {
                error!("Failed to receive message");
                Message::Error {
                    error: "Failed to receive message".to_string(),
                    origin: None,
                }
            }
        }
    }

    async fn end_to_end_standard(&mut self, message: Vec<u8>) -> Message {
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let origin = OriginMessage::new(Some(respond_to))
            .session("0".to_string())
            .with_span(Some(DebuggableSpan(root_span)));

        node.send(Message::Standard {
            message,
            origin: Some(origin),
        });

        recv.await.unwrap()
    }

    async fn end_to_end_2(&mut self, message: Vec<u8>) -> Message {
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();

        let (_tx, _rx) = oneshot::channel();

        let (tx, mut rx) = mpsc::channel(10);

        node.send(Message::Standard {
            message,
            origin: Some(
                OriginMessage::new(Some(_tx))
                    .session("0".to_string())
                    .mpsc_respond_to(tx),
            ),
        });

        let mut lines = Vec::new();

        while let Some(message) = rx.recv().await {
            debug!("Message: {:?}", message);

            match message {
                Message::Standard { message, .. } => {
                    debug!("Message: {:?}", message);
                    let msg = String::from_utf8(message).unwrap();

                    let json: Value = serde_json::from_str(&msg).unwrap();

                    lines.push(json);
                }
                Message::Exit { origin } => {
                    debug!("Exit: {:?}", origin);
                    break;
                }
                _ => {}
            }
        }

        info!("DONE");
        //recv.await.unwrap()
        Message::JSON {
            message: serde_json::Value::Array(lines),
            origin: None,
        }
    }

    // Process request just like before
    async fn process(&mut self, message: Message) -> Message {
        debug!("Starting with {}", &self.start_node);

        let graph = self;

        graph
            .nodes
            .get(&graph.start_node)
            .unwrap()
            .lock()
            .unwrap()
            .send(message);

        //let p = graph.processor.unwrap().lock().unwrap();

        //let processor = graph.processor.as_mut().unwrap();
        let a = graph.processor.as_mut().unwrap();
        let mut b = a.lock().unwrap();
        let message = b.req_reply().await;
        //return processor.req_reply().await;
        message
    }
}

fn watch_control_bus(self_graph_name: String) {
    let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

    // Register it
    register(&self_graph_name, tx);

    tokio::spawn(async move {
        loop {
            // Receive the message
            if let Some(msg) = rx.recv().await {
                info!("Received: {:?}", serde_json::to_string(&msg));

                {
                    let graph = GraphRegistry::get_graph(&self_graph_name);

                    if graph.is_none() {
                        warn!("Graph {} not found", self_graph_name);
                        continue;
                    }
                }

                if msg.state == GraphStatus::Starting {
                    info!("Graph {} is starting", msg.graph_name);
                    if msg.graph_name == self_graph_name {
                        let graph = GraphRegistry::get_graph(&self_graph_name);

                        let graph = graph.expect("Failed to get graph!");

                        let mut graph = graph.lock().unwrap();

                        graph.poke();
                    }
                } else if msg.state == GraphStatus::Running {
                    info!("Graph {} is running", msg.graph_name);
                } else if msg.state == GraphStatus::Stopped {
                    info!("Graph {} is stopped", msg.graph_name);
                } else if msg.state == GraphStatus::Paused {
                    info!("Graph {} is paused", msg.graph_name);
                }
            } else {
                warn!("Channel is closed");
                break;
            }
        }
    });
}
