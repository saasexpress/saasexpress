use std::sync::{Arc, Mutex};

use tracing::{error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Filter2Operator, Graph, OperatorType};

use crate::graph::message::{Message, OriginMessage};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;

use super::check_fs::CheckFs;
use super::check_fs::CheckFsImpl;

#[derive(Clone, Debug)]
pub(crate) struct ClaimCheck {
    engine: String,
}

impl From<serde_yaml::Value> for ClaimCheck {
    fn from(_value: serde_yaml::Value) -> Self {
        ClaimCheck {
            engine: "filesystem".to_string(),
        }
    }
}

impl ClaimCheck {
    fn save(&self, message: Vec<u8>, origin: Option<OriginMessage>) -> Message {
        let cfs = CheckFsImpl {};

        let claim_check = cfs.check_fs(message);
        info!("ClaimCheck: {:?}", claim_check);
        Message::JSON {
            message: serde_json::to_value(claim_check).unwrap(),
            origin,
        }
    }
}
impl Filter2Operator for ClaimCheck {
    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { message, origin } => self.save(message, origin),
            Message::JSON { message, origin } => match serde_json::to_vec(&message) {
                Ok(m) => self.save(m, origin),
                Err(e) => {
                    error!("Error serializing JSON to Vec<u8>: {}", e);
                    return Message::Error {
                        error: "Error serializing JSON".to_string(),
                    };
                }
            },
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
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
        warn!("Init Not implemented");
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
