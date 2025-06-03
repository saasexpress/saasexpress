use serde_json::{Error, Value};
use tracing::error;

use crate::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::Message,
    meta::NodeMeta,
    operator::{Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState, OperatorType},
};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};

pub trait CanonicalModelService: Sync + Send + Debug {
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
    pub(crate) service: Arc<dyn CanonicalModelService + Send + Sync + 'static>,
}

impl CanonicalModel {
    pub fn new(name: &str, service: impl CanonicalModelService + Send + Sync + 'static) -> Self {
        CanonicalModel {
            name: name.to_string(),
            service: Arc::new(service),
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

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(CanonicalModel {
            name: self.name.clone(),
            service: Arc::clone(&self.service),
        })
    }

    fn init(&mut self, _: &mut Graph, _: &NodeMeta) {}

    fn control(&mut self, _: Message) {}
}

impl OperatorRuntime for CanonicalModel {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, mut in_message: Message) -> Message {
        let origin = in_message.take_origin();

        match &in_message {
            Message::JSON { message: json, .. } => match self.service.validate_json(json.clone()) {
                Ok(_model) => in_message.with_origin(origin),
                Err(e) => {
                    error!("Error deserializing JSON to CanonicalModel: {}", e);
                    return Message::Error {
                        error: format!("Canonical Model Validation Error - {}", e).to_string(),
                        origin,
                    };
                }
            },
            _ => {
                error!("Unexpected message type {}", in_message);
                return Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin,
                };
            }
        }
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
