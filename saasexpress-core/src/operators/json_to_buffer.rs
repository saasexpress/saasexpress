use std::sync::{Arc, Mutex};

use tracing::debug;

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::Message;

use crate::graph::graph::Operator;

#[derive(Debug)]
pub(crate) struct JSONToBuffer;

impl From<serde_yaml::Value> for JSONToBuffer {
    fn from(_value: serde_yaml::Value) -> Self {
        JSONToBuffer {}
    }
}

impl Operator for JSONToBuffer {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "JSONToBuffer".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        let (json, origin) = match _message {
            Message::JSON { message, origin } => (message, origin),
            _ => panic!("Unexpected message type {}", _message),
        };

        let response_message = serde_json::to_vec(&json).expect("JSON serialization error");
        Message::Standard {
            message: response_message,
            origin,
        }
    }

    fn init(&mut self, _: &mut Graph) {
        debug!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("Not implemented");
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
