use serde_json::{Error, Value};
use tracing::error;

use crate::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::Message,
    meta::NodeMeta,
    operator::{Operator, OperatorRef, OperatorRole, OperatorState, OperatorType},
};
use core::panic;
use std::{fmt::Debug, sync::Arc};

pub trait CanonicalModelOperator: Sync + Send + Debug {
    fn validate_json(&self, json: Value) -> Result<(), Error>;
}

// impl From<serde_yaml::Value> for CanonicalModel {
//     fn from(_value: serde_yaml::Value) -> Self {
//         CanonicalModel { operator: None }
//     }
// }

#[derive(Debug)]
pub struct CanonicalModel {
    name: String,
    pub(crate) operator: Arc<dyn CanonicalModelOperator + Send + Sync + 'static>,
}

impl CanonicalModel {
    pub fn new(name: &str, operator: impl CanonicalModelOperator + Send + Sync + 'static) -> Self {
        CanonicalModel {
            name: name.to_string(),
            operator: Arc::new(operator),
        }
    }
}

impl Operator for CanonicalModel {
    fn _type(&self) -> OperatorType {
        OperatorType::CanonicalModel {}
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn init(&mut self, _: &mut Graph, _: &NodeMeta) {}

    fn control(&mut self, _: Message) {}

    fn handle(&self, mut in_message: Message) -> Message {
        let origin = in_message.take_origin();

        match &in_message {
            Message::JSON { message: json, .. } => {
                match self.operator.validate_json(json.clone()) {
                    Ok(_model) => in_message.with_origin(origin),
                    Err(e) => {
                        error!("Error deserializing JSON to CanonicalModel: {}", e);
                        return Message::Error {
                            error: format!("Canonical Model Validation Error - {}", e).to_string(),
                            origin,
                        };
                    }
                }
            }
            _ => {
                error!("Unexpected message type {}", in_message);
                return Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin,
                };
            }
        }
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<std::sync::Arc<std::sync::Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }

    fn finalize(&mut self) -> bool {
        tracing::debug!("Default finalize operator {} - no action", self.name());
        true
    }

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }

    fn handle_ptr(&self, message: Arc<Message>) -> Arc<Message> {
        tracing::debug!("default handle (passthrough)... {}", self.name());
        return message;
    }

    fn next_ptr(&self, message: Arc<Message>) {
        // Sending message to next operator
        for n in self.get_output_channels() {
            n.lock().unwrap().send_ptr(message.to_owned());
            //break;
        }
    }
}
