use std::sync::{Arc, Mutex};

use serde_json::json;
use tracing::warn;

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::Message;

use crate::graph::graph::Operator;

#[derive(Clone, Debug)]
pub(crate) struct Callout {
    graph_name: String,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl From<serde_yaml::Value> for Callout {
    fn from(_value: serde_yaml::Value) -> Self {
        let graph_name = _value
            .get("graph_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Callout {
            graph_name,
            next: Vec::new(),
        }
    }
}

impl Operator for Callout {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "Callout".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { message, .. } => {
                let this = self.to_owned();
                tokio::spawn(async move {
                    warn!("Callout... forward message {:?}", message);
                    this.next(Message::JSON {
                        message: json!({}),
                        origin: None,
                    })
                });
                Message::NoOp {}
            }
            _ => panic!("Unexpected message type {}", _message),
        }
    }

    fn init(&mut self, _: &mut Graph) {
        panic!("Not implemented");
        // need all graphs, so we can find the graph that we will be starting
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }
            }
            _ => {
                panic!("Unexpected message type for control {}", _message);
            }
        }
    }

    fn send(&self, message: Message) {
        self.handle(message);
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}

impl Callout {
    fn next(&self, _message: Message) {
        for node in &self.next {
            node.lock().unwrap().send(_message);
            break;
        }
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }
}
