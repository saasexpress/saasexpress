use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::{error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState, OperatorType,
};

use crate::graph::message::Message;

use crate::graph::meta::NodeMeta;

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

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        warn!("Init Not implemented");
    }

    fn control(&mut self, _: Message) {
        warn!("Control Not implemented");
    }
}

impl OperatorRuntime for Template {
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
                    origin: None,
                }
            }
        }
    }

    fn send(&self, _: Message) {
        panic!("Send Not implemented");
    }
}
