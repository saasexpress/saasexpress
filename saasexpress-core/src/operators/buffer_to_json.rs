use core::panic;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, warn};

use crate::graph::message::Message;
use crate::graph::message::OriginMessage;

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    Filter2Operator, GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime,
    OperatorType,
};

use crate::graph::meta::NodeMeta;
use crate::timestamp::{NaiveDateTimeExt, now};

#[derive(Debug)]
pub(crate) struct BufferToJSON;

impl From<&serde_yaml::Value> for BufferToJSON {
    fn from(_value: &serde_yaml::Value) -> Self {
        BufferToJSON {}
    }
}

impl Filter2Operator for BufferToJSON {
    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::ReqReply {
                message,
                respond_to,
                span,
                temp,
                ..
            } => {
                debug!("[Filter2] ReqReply to JSON message");

                let origin = Some(
                    OriginMessage::new(Some(respond_to))
                        .with_span(span)
                        .with_temp(temp),
                );

                if message.is_empty() {
                    let empty = json!({});
                    return to_json(empty, origin);
                }

                let result: Value = match serde_json::from_slice(&message) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error serializing JSON to Vec<u8>: {}", e);
                        return Message::Error {
                            error: "Error serializing JSON".to_string(),
                            origin: None,
                        };
                    }
                };

                return to_json(result, origin);
            }
            Message::Standard { message, origin } => {
                debug!("Standard not expected");
                if message.is_empty() {
                    warn!("What??? Empty! {:?}", message);
                    let empty = json!({});
                    return to_json(empty, origin);
                }
                let result: Value = serde_json::from_slice(&message).expect("JSON parse error");
                return to_json(result, origin);
            }
            Message::JSON { .. } => {
                return _message;
            }
            Message::HTTP {
                message, origin, ..
            } => {
                warn!("HTTP not expected");
                let result: Value = serde_json::from_slice(&message).expect("JSON parse error");
                return to_json(result, origin);
            }
            Message::Error { error, origin } => return Message::Error { error, origin },
            Message::Exit { origin } => {
                return Message::Exit { origin };
            }
            _ => {
                error!("Unexpected message type {}", _message);
                return Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                };
            }
        };
    }
}

impl Operator for BufferToJSON {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter2 {
            operator: Arc::new(BufferToJSON {}),
        }
    }

    fn name(&self) -> String {
        "BufferToJSON".to_string()
    }

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(BufferToJSON {})
    }

    // fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
    //     None
    // }

    // fn handle(&self, _message: Message) -> Message {
    //     panic!("BufferToJSON - Handle Not implemented");
    // }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("Init Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("Control Not implemented");
    }

    // fn send(&self, _: Message) {
    //     panic!("Send Not implemented");
    // }

    // fn wait(&self) -> Message {
    //     panic!("Wait Not implemented");
    // }

    // fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
    //     panic!("Not implemented");
    // }
}

fn to_json(mut data: Value, origin: Option<OriginMessage>) -> Message {
    data.as_object_mut()
        .unwrap()
        .insert("_ts".to_string(), Value::String(now().to_rfc3339()));

    return Message::JSON {
        message: data,
        origin,
    };
}

impl OperatorRuntime for BufferToJSON {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn handle(&self, message: Message) -> Message {
        panic!("BufferToJSON - Handle Not implemented");
    }

    fn send(&self, _message: Message) {
        panic!("Send Not implemented");
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }
}
