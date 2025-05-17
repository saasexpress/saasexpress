use std::sync::{Arc, Mutex};

use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Filter2Operator, Graph, OperatorType};

use crate::graph::message::{Message, OriginMessage};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;

use super::check_fs::CheckFsImpl;
use super::check_fs::CheckStorage;

#[derive(Clone, Debug)]
pub(crate) struct ClaimCheck {
    fqn: String,
    engine: String,
    getter: bool,
}

impl From<serde_yaml::Value> for ClaimCheck {
    fn from(_value: serde_yaml::Value) -> Self {
        let getter = _value["getter"].as_bool().map(|s| s).unwrap_or(false);
        ClaimCheck {
            fqn: "".to_string(),
            engine: "filesystem".to_string(),
            getter,
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
        info!("ClaimCheck: {} - {:?}", self.getter, _message);
        match _message {
            Message::Standard { message, origin } => {
                if self.getter {
                    return Message::Error {
                        error: "Can not use a Standard message for Getters".to_string(),
                        origin,
                    };
                } else {
                    self.put(message, origin)
                }
            }
            Message::JSON { message, origin } => match serde_json::to_vec(&message) {
                Ok(m) => {
                    if self.getter {
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
                    } else {
                        self.put(m, origin)
                    }
                }
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
                getter: self.getter,
                fqn: self.fqn.clone(),
            }),
        }
    }

    fn name(&self) -> String {
        "ClaimCheck".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        panic!("[ClaimCheck] not implemented");
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.fqn = node_meta.fqn();
    }

    fn control(&mut self, _: Message) {
        warn!("Control Not implemented");
    }

    fn send(&self, _: Message) {
        panic!("Send Not implemented");
    }

    fn wait(&self) -> Message {
        panic!("Wait Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}
