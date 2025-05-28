use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use fastrace::Span;
use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde_json::Value;
use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{Operator, OperatorRef, OperatorRole, OperatorState, OperatorType};

use crate::graph::message::{ControlCommand, Message, OriginMessage};

use crate::graph::meta::NodeMeta;
use crate::graph::registry::GraphRegistry;

#[derive(Clone, Debug)]
pub(crate) struct Settings {
    graphs: Vec<Arc<Mutex<Graph>>>,
    next: Vec<OperatorRole>,
    state: OperatorState,
}

impl From<serde_yaml::Value> for Settings {
    fn from(_value: serde_yaml::Value) -> Self {
        Settings {
            graphs: Vec::new(),
            next: Vec::new(),
            state: OperatorState::Pending,
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
                info!("Handling! {:?}", self.graphs.len());

                self.graphs.iter().for_each(|graph| match graph.try_lock() {
                    Ok(graph) => {
                        info!("Graph: {} {:?}", graph.name, graph.nodes.len());
                        graph.nodes.iter().for_each(|operator| {
                            info!("Operator: {}.{}", graph.name, operator.0);
                            {
                                let update_settings = message.get(graph.name.as_str());
                                if update_settings.is_none() {
                                    return;
                                }

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
                            }
                        });
                    }
                    Err(_) => {
                        warn!("Failed to lock graph - skipping settings");
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

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        warn!("Not implemented");
    }

    fn state(&self) -> OperatorState {
        self.state.clone()
    }

    fn finalize(&mut self) -> bool {
        let graph_registry = GraphRegistry::get_instance();
        let graph_registry = graph_registry.lock().unwrap();
        self.graphs = graph_registry.get_graphs();
        info!("Finalizing with {} graph(s)", self.graphs.len());
        self.state = OperatorState::Ready;
        true
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }
            }
            Message::Control { .. } => {
                debug!("Control");
            }

            _ => {
                panic!("Unexpected message type for control {}", _message);
            }
        }
    }

    fn send(&self, message: Message) {
        self.next(message);
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}

impl Settings {
    fn next(&self, message: Message) {
        let next_node = self.next.get(0).unwrap();
        next_node.operator.lock().unwrap().send(message);
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }
}
