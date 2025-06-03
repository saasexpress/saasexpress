use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use fastrace::Span;
use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde_json::Value;
use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState, OperatorType,
};

use crate::graph::message::{ControlCommand, Message, OriginMessage};

use crate::graph::meta::NodeMeta;
use crate::graph::registry::GraphRegistry;

#[derive(Clone, Debug)]
pub(crate) struct Settings {
    id: String,
    graph_name: String,
    // graphs: Vec<Arc<Mutex<Graph>>>,
    next: Vec<OperatorRole>,
}

impl From<serde_yaml::Value> for Settings {
    fn from(_value: serde_yaml::Value) -> Self {
        Settings {
            id: String::new(),
            graph_name: String::new(),
            // graphs: vec![Arc::new(Mutex::new(Graph::new()))],
            //            graphs: Vec::new(),
            next: Vec::new(),
        }
    }
}

impl Operator for Settings {
    fn _type(&self) -> OperatorType {
        // Need to use finalize() so has to be Endpoint
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "Settings".to_string()
    }

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        let next_nodes = {
            // let graph = GraphRegistry::get_graph(&self.graph_name);
            // if graph.is_none() {
            //     error!("Graph not found - incomplete runtime : {}", self.graph_name);
            //     Vec::new()
            // } else {
            // let graph = graph.unwrap();
            // let graph = graph.lock().unwrap();

            Graph::get_next_nodes(&self.id, mut_nodes.clone(), edges.clone())
            // }
        };
        Arc::new(Settings {
            graph_name: self.graph_name.clone(),
            id: self.id.clone(),
            next: next_nodes,
        })
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.id = node_meta.name.clone();
        self.graph_name = node_meta.graph.clone();
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Control { .. } => {
                debug!("Control");
            }

            _ => {
                panic!("Unexpected message type for control {}", _message);
            }
        }
    }
}

impl OperatorRuntime for Settings {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        let root_span = Span::root(format!("settings"), SpanContext::random());

        root_span.set_local_parent();

        match _message {
            Message::JSON { message, origin } => {
                // Loop through the keys (graph -> operator -> settings)
                // Send a Control message to the particular operator
                let registry = GraphRegistry::get_instance();

                let graph_names = registry.lock().unwrap().graph_names();

                info!("Handling! {:?}", graph_names.len());

                graph_names
                    .iter()
                    .for_each(|graph| match GraphRegistry::get_graph(graph) {
                        Some(graph) => {
                            graph.lock().unwrap().apply_settings(&message);
                        }
                        None => {
                            warn!("Graph {} not found", graph);
                        }
                    });
                // Send the message to the next operator
                return Message::JSON { message, origin };
            }
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                }
            }
        }
    }

    fn send(&self, message: Message) {
        let next_node = self.next.get(0).unwrap();
        next_node.operator.send(message);
    }
}
