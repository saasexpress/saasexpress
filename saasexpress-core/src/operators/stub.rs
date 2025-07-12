use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde::de::value;
use tracing::{debug, error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState,
    OperatorType,
};

use crate::graph::message::{ControlCommand, Message};

use crate::graph::meta::NodeMeta;

#[derive(Clone, Debug)]
pub(crate) struct Stub {
    yaml: Option<serde_yaml::Value>,
}

impl From<&serde_yaml::Value> for Stub {
    fn from(value: &serde_yaml::Value) -> Self {
        let data = value.get("data").unwrap();

        Stub {
            yaml: Some(data.clone()),
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
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {}
    fn control(&mut self, message: Message) {
        match message {
            Message::Control { command, .. } => match command {
                ControlCommand::Start { .. } => {}
                ControlCommand::SetSettings { settings } => {
                    debug!("Received SetSettings command with value: {:?}", settings);
                    let value = serde_yaml::to_value(&settings)
                        .expect("Failed to convert settings to serde_yaml::Value");
                    self.yaml = Some(value);
                }
                _ => {
                    error!("Invalid control command: {:?}", command);
                    panic!("Invalid control command: {:?}", command);
                }
            },
            _ => {
                debug!("Received non-control message: {:?}", message);
            }
        }

        debug!("Doing nothing with control message");
    }
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
                    message: self.yaml_to_json(self.yaml.as_ref().unwrap()),
                    origin,
                };
            }
        }
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
