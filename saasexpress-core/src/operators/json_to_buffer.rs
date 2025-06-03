use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::{debug, error};

use crate::graph::graph::{AsyncHandleTrait, Graph};

use crate::graph::message::Message;

use crate::graph::meta::NodeMeta;
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRuntime, OperatorType,
};

#[derive(Debug, Clone)]
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

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("Not implemented");
    }
}

impl OperatorRuntime for JSONToBuffer {
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
        let (json, origin) = match _message {
            Message::JSON { message, origin } => (message, origin),
            Message::Error { error, origin } => return Message::Error { error, origin },
            _ => {
                error!("Unexpected message type {}", _message);
                return Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                };
            }
        };

        let response_message = serde_json::to_vec(&json).expect("JSON serialization error");
        Message::Standard {
            message: response_message,
            origin,
        }
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
