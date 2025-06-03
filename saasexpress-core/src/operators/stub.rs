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

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {}
    fn control(&mut self, _: Message) {}
}

impl Stub {
    fn yaml_to_json(&self, _value: &serde_yaml::Value) -> serde_json::Value {
        let json_value = serde_json::to_value(_value).unwrap();
        json_value
    }
}

impl OperatorRuntime for Stub {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, mut _message: Message) -> Message {
        let origin = _message.take_origin();
        match _message {
            _ => {
                return Message::JSON {
                    message: self.yaml_to_json(self.value.as_ref().unwrap()),
                    origin,
                };
            }
        }
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
