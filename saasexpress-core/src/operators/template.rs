use std::sync::{Arc, Mutex};

use tracing::{error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::Message;

use crate::graph::graph::Operator;

#[derive(Clone, Debug)]
pub(crate) struct Template;

impl From<serde_yaml::Value> for Template {
    fn from(_value: serde_yaml::Value) -> Self {
        Template {}
    }
}

impl Operator for Template {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "Template".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { message, origin } => {
                return Message::Standard {
                    message: message.to_owned(),
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

    fn init(&mut self, _: &mut Graph) {
        warn!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        warn!("Not implemented");
    }

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
