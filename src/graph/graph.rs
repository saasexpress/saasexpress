use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::future::Future;
use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::channel::{mpsc, oneshot};
use hyper::Method;
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tracing::{debug, error};

use super::super::operators::buffer_to_json::BufferToJSON;
use super::super::operators::factory::{OpXX, OperatorSpec};
use super::super::operators::http_in::http_in::HTTPIn;
use super::super::operators::json_to_buffer::JSONToBuffer;
use super::super::operators::noop::NOOP;
use super::super::operators::op_actor_handle::OperatorActorHandle;
use super::processors::XProcessor;
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

#[derive(Debug)]
pub struct OriginMessage {
    pub respond_to: oneshot::Sender<Message>,
}

#[derive(Debug)]
pub enum Message {
    Split {
        message: Vec<u8>,
        respond_to: Arc<mpsc::Sender<Message>>,
        origin: Option<OriginMessage>,
    },
    Standard {
        message: Vec<u8>,
        origin: Option<OriginMessage>,
    },
    JSON {
        message: Value,
        origin: Option<OriginMessage>,
    },
    ReqReply {
        path: String,
        query: String,
        method: String,
        message: Vec<u8>,
        respond_to: oneshot::Sender<Message>,
    },
    Init {
        next: Vec<Arc<Mutex<dyn Operator>>>,
        //_end: Option<Port>,
        end: Arc<Mutex<dyn Operator + 'static>>,
        start: Arc<Mutex<dyn Operator + 'static>>,
        //processor: Arc<Mutex<dyn XProcessor + 'static>>,
    },
    Error {
        error: String,
    },
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Split { .. } => write!(f, "Split message"),
            Message::Standard { message, .. } => write!(f, "Standard message: {:?}", message),
            Message::JSON { message, .. } => write!(f, "JSON message: {:?}", message),
            Message::ReqReply { message, .. } => write!(f, "ReqReply message: {:?}", message),
            Message::Init { .. } => write!(f, "Init message"),
            Message::Error { .. } => write!(f, "Error message"),
        }
    }
}

pub trait GraphMod {
    fn add_new_node<O>(&mut self, id: &str, operator: O) -> &mut Self
    where
        O: Operator + 'static;
}

#[derive(Debug)]
pub struct Ports {
    pub ports: HashMap<String, Port>,
}

impl Ports {
    pub fn new_port(&mut self, id: String) -> NOOP {
        let id = format!("{}-ext", id);
        let port = Port::create();

        self.ports.insert(id, port.0);
        return port.1;
    }
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
    async fn start(&self);
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
    fn handle(&self, message: Message) -> Message;
    fn control(&mut self, message: Message);
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

        match operator._type() {
            OperatorType::Endpoint => {
                self.add_new_node(id, operator);
                Port::new(self, "abc".to_string());
            }
            OperatorType::Filter => {
                self.add_new_node(id, OperatorActorHandle::new(operator));
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

#[derive(Debug)]
struct MySharedState {
    a: String,
    //start_node: String,
    prep: Arc<Mutex<dyn Operator + 'static>>,
}

impl Graph {
    async fn build(&mut self) -> Graph {
        let mut graph = Graph::new("simple-graph".to_string());

        graph
            .no_processor()
            .add_node(
                "start",
                HTTPIn::new(vec!["/gw/flow".to_string()], "POST".to_string()).await,
            )
            .add_node("in", BufferToJSON)
            .add_node("out", JSONToBuffer)
            .add_edge("start".to_string(), "in".to_string())
            .add_edge("in".to_string(), "out".to_string())
            .init();
        return graph;
    }

    pub async fn process2(&mut self, _message: Message) -> Message {
        let new_g = self.build().await;
        let start = new_g.start_node.clone();
        let node = new_g.nodes.get(&start);

        debug!("Node: {:?}", node);
        debug!("Starting with {}", new_g.start_node);
        //let prep = node.unwrap();
        let prep = node.unwrap().clone();

        let shared_state = Arc::new(MySharedState {
            a: "12".to_string(),
            prep,
        });

        let handler = async |state: State<Arc<MySharedState>>, body: String| {
            //let shared_state = Arc::clone(shared_state);
            let a = &state.a;
            debug!("Received request with body: {}", body);

            //let p = graph.processor.unwrap().lock().unwrap();

            let (send, recv) = oneshot::channel();

            let message = Message::ReqReply {
                message: "{\"a\":\"b\"}".to_string().into_bytes(),
                respond_to: send,
                path: "/".to_string(),
                query: "".to_string(),
                method: "GET".to_string(),
            };

            //let b = prep.lock();

            //b.unwrap().send(message);
            state.prep.lock().unwrap().send(message);

            // let the_processor = graph.processor.as_mut().unwrap();
            // let mut b = the_processor.lock().unwrap();
            // let message = b.req_reply().await;

            match recv.await {
                Ok(msg) => match msg {
                    Message::Standard {
                        message,
                        origin: None,
                    } => {
                        println!(
                            "Received a Standard message: {:?}",
                            String::from_utf8_lossy(&message)
                        );
                        Json(json!({ "a": String::from_utf8_lossy(&message) }))
                    }
                    _ => panic!("Expected a Standard response"),
                },
                Err(e) => {
                    error!("Failed to send - returning error: {}", e);
                    Json(json!({ "a": a }))
                }
            }

            // let response = futures::executor::block_on(recv).unwrap();
            // println!("Got response");

            // Since we're done, we can drop the sender to signal workers to finish
            //drop(recv);
        };

        let app = Router::new()
            .route("/", post(handler))
            .with_state(shared_state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 2243));
        let listener = TcpListener::bind(addr).await.unwrap();

        debug!("Listening on: {}", addr);

        let serve = axum::serve(listener, app);

        serve.await.expect("Failed to start server");

        drop(new_g);

        Message::Standard {
            message: b"Hello, world!".to_vec(),
            origin: None,
        }
    }

    async fn create_user(&mut self, Json(payload): Json<Value>, state: Arc<MySharedState>) {
        debug!("Create user!");
    }
}
