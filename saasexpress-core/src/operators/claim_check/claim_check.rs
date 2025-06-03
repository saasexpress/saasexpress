use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::{error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::message::{Message, OriginMessage};
use crate::graph::operator::{
    Filter2Operator, GraphOperatorContext, Operator, OperatorRef, OperatorRuntime, OperatorType,
};

use crate::graph::meta::NodeMeta;

use super::check_fs::CheckFsImpl;
use super::check_fs::CheckStorage;

#[derive(Debug, Clone)]
pub(crate) enum ClaimCheckAction {
    Put,
    Get,
    Clear,
}

impl From<&str> for ClaimCheckAction {
    fn from(value: &str) -> Self {
        match value {
            "Put" => ClaimCheckAction::Put,
            "Get" => ClaimCheckAction::Get,
            "Clear" => ClaimCheckAction::Clear,
            _ => ClaimCheckAction::Put,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ClaimCheck {
    fqn: String,
    engine: String,
    action: ClaimCheckAction,
}

impl From<serde_yaml::Value> for ClaimCheck {
    fn from(_value: serde_yaml::Value) -> Self {
        let action = _value["action"]
            .as_str()
            .map(|s| ClaimCheckAction::from(s))
            .unwrap_or(ClaimCheckAction::Put);
        ClaimCheck {
            fqn: "".to_string(),
            engine: "filesystem".to_string(),
            action,
        }
    }
}

impl ClaimCheck {
    fn put(&self, message: Vec<u8>, origin: Option<OriginMessage>) -> Message {
        let cfs = CheckFsImpl {};

        let claim_check = cfs.put(message);
        info!("ClaimCheck: {:?}", claim_check);
        Message::JSON {
            message: serde_json::to_value(claim_check).unwrap(),
            origin,
        }
    }

    fn clear(&self, claim_id: &str) {
        let cfs = CheckFsImpl {};
        cfs.clear(claim_id);
    }

    fn get(&self, claim_id: &str, origin: Option<OriginMessage>) -> Message {
        let cfs = CheckFsImpl {};

        let message = cfs.get(claim_id);
        match message {
            Ok(m) => {
                return Message::Standard { message: m, origin };
            }
            Err(e) => Message::Error {
                error: format!("ClaimCheckInvalidError: [{}] {}", self.fqn, e),
                origin,
            },
        }
    }
}
impl Filter2Operator for ClaimCheck {
    fn handle(&self, _message: Message) -> Message {
        info!("ClaimCheck: {:?} - {:?}", self.action, _message);
        match _message {
            Message::Standard { message, origin } => match self.action {
                ClaimCheckAction::Put => self.put(message, origin),
                ClaimCheckAction::Get => Message::Error {
                    error: "Can not use a Standard message for Getters".to_string(),
                    origin,
                },
                ClaimCheckAction::Clear => Message::Error {
                    error: "Can not use a Standard message for Getters".to_string(),
                    origin,
                },
            },
            Message::JSON { message, origin } => match serde_json::to_vec(&message) {
                Ok(m) => match self.action {
                    ClaimCheckAction::Put => self.put(m, origin),
                    ClaimCheckAction::Get => {
                        let claim_id = message["claim_id"].as_str();
                        match claim_id {
                            Some(claim_id) => self.get(claim_id, origin),
                            None => Message::Error {
                                error: format!(
                                    "ClaimCheckInvalidError: [{}] No claim id provided",
                                    self.fqn
                                ),
                                origin,
                            },
                        }
                    }
                    ClaimCheckAction::Clear => {
                        let claim_id = message["claim_id"].as_str();
                        match claim_id {
                            Some(claim_id) => {
                                self.clear(claim_id);
                                Message::Standard {
                                    message: vec![],
                                    origin,
                                }
                            }
                            None => Message::Error {
                                error: format!(
                                    "ClaimCheckInvalidError: [{}] No claim id provided",
                                    self.fqn
                                ),
                                origin,
                            },
                        }
                    }
                },
                Err(e) => {
                    error!("Error serializing JSON to Vec<u8>: {}", e);
                    return Message::Error {
                        error: "Error serializing JSON".to_string(),
                        origin,
                    };
                }
            },
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                }
            }
        }
    }
}

impl Operator for ClaimCheck {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter2 {
            operator: Arc::new(ClaimCheck {
                engine: self.engine.clone(),
                action: self.action.clone(),
                fqn: self.fqn.clone(),
            }),
        }
    }

    fn name(&self) -> String {
        "ClaimCheck".to_string()
    }

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.fqn = node_meta.fqn();
    }

    fn control(&mut self, _: Message) {
        warn!("Control Not implemented");
    }
}

impl OperatorRuntime for ClaimCheck {
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
        panic!("[ClaimCheck] not implemented");
    }

    fn send(&self, _: Message) {
        panic!("Send Not implemented");
    }
}
