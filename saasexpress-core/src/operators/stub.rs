use std::sync::{Arc, Mutex};

use tracing::{error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::Message;

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;

#[derive(Clone, Debug)]
pub(crate) struct Stub {
    value: Option<serde_yaml::Value>,
}

impl From<serde_yaml::Value> for Stub {
    fn from(_value: serde_yaml::Value) -> Self {
        Stub {
            value: Some(_value),
        }
    }
}

impl Operator for Stub {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "Stub".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { origin, .. } => {
                return Message::JSON {
                    message: self.yaml_to_json(self.value.as_ref().unwrap()),
                    origin,
                };
            }
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                }
            }
        }
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {}
    fn control(&mut self, _: Message) {}

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}

impl Stub {
    fn yaml_to_json(&self, _value: &serde_yaml::Value) -> serde_json::Value {
        let json_value = serde_json::to_value(_value).unwrap();
        json_value
    }
}
