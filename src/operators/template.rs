use std::sync::{Arc, Mutex};

use tracing::warn;

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::graph::{Message, Operator};

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
            _ => panic!("Unexpected message type"),
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
