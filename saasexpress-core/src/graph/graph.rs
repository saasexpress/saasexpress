use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use serde_json::Value;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::operators::op_wrapper::OperatorWrapper;
use crate::ports::ports::Ports;

use super::super::operators::op_actor_handle::OperatorActorHandle;

use super::message::{Message, OriginMessage};
use super::meta::NodeMeta;
use super::processors::basic::BasicProcessor;
use super::processors::port::Port;
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
pub struct Graph {
    pub name: String,
    pub start_node: String,

    /// Collection of nodes in the Graph, indexed by their unique ID
    pub nodes: HashMap<String, Arc<Mutex<dyn Operator + 'static>>>,

    /// Mapping of node IDs to their outgoing edges (children)
    edges: HashMap<String, HashSet<String>>,

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

#[derive(PartialEq)]
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
}

// Operator trait - defines how to process a message
pub trait Operator: Send + Sync + Debug {
    fn _type(&self) -> OperatorType;
    fn name(&self) -> String;
    //fn meta(&self) -> NodeMeta;

    /// performs the work of the operator
    fn handle(&self, message: Message) -> Message;

    /// commands and events for controlling the operator
    fn control(&mut self, message: Message);

    /// Sends a message to the next operators in the graph
    fn send(&self, message: Message);

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>>;
    fn wait(&self) -> Message;
    fn init(&mut self, graph: &mut Graph);

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
    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>>;
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
            start_node: String::new(),
            //_nodes: HashMap::new(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            processor: None,
            ports: Ports {
                ports: HashMap::new(),
            },
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

        operator.init(self);

        info!("Node: {}({})", operator.name(), id.to_string());

        let typ = operator._type();

        let wrapped_op = OperatorWrapper::new(operator);
        //let wrapped_op = operator;

        match typ {
            OperatorType::Endpoint => {
                self.add_new_node(id, wrapped_op);
                Port::new(self, "abc".to_string());
            }
            OperatorType::Filter => {
                self.add_new_node(id, OperatorActorHandle::new(wrapped_op));
            }
        }

        self
    }

    /// Adds a directed edge between two nodes
    ///
    /// Returns an error if:
    /// - Either node doesn't exist in the Graph
    /// - Adding the edge would create a cycle
    pub fn add_edge(&mut self, from: String, to: String) -> &mut Self {
        // if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
        //     panic!("Both nodes must be in the Graph");
        // }
        // if self.has_path(&to, &from) {
        //     return Err("Adding this edge would create a cycle".to_string());
        // }

        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);

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

            let mut childs: Vec<Arc<Mutex<dyn Operator>>> = Vec::new();

            let children = self.edges.get(id);
            match children {
                Some(children) => {
                    for child in children {
                        debug!("Adding child {}", child);
                        let opsch = self.nodes.get(child);
                        match opsch {
                            Some(opsc) => childs.push(Arc::clone(opsc)),
                            None => {}
                        }
                    }
                }
                None => {}
            }

            // if there are no edges/children for the node, then have it go to "_end"
            // if it exists
            if children.iter().len() == 0 {
                let opsch = self.nodes.get("_end");
                match opsch {
                    Some(opsc) => childs.push(Arc::clone(opsc)),
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

        for (id, _) in self.nodes.iter() {
            debug!("Inventory: {}", id);
        }
        for (id, _) in self.ports.ports.iter() {
            debug!("Ports: {}", id);
        }
        self
    }
}

impl GraphRun for Graph {
    async fn end_to_end(&mut self, message: Vec<u8>) -> Message {
        let node = self.nodes.get(&self.start_node).unwrap().clone();

        let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        node.send(Message::ReqReply {
            message,
            respond_to,
            path: "".to_string(),
            method: "".to_string(),
            query: "".to_string(),
            span: None,
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
                OriginMessage::new(_tx)
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
