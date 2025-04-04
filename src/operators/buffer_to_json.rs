use core::panic;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tracing::debug;

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType, OriginMessage};

use crate::graph::graph::{Message, Operator};
use chrono::{NaiveDate, TimeZone, Utc};

#[derive(Debug)]
pub(crate) struct BufferToJSON;

impl From<serde_yaml::Value> for BufferToJSON {
    fn from(_value: serde_yaml::Value) -> Self {
        BufferToJSON {}
    }
}

impl Operator for BufferToJSON {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "BufferToJSON".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        debug!("BufferToJSON Processing...");

        match _message {
            Message::ReqReply {
                message,
                respond_to,
                ..
            } => {
                debug!("Passthrough message");

                let result: Value = serde_json::from_slice(&message).expect("JSON parse error");

                let origin = Some(OriginMessage { respond_to });

                return to_json(result, origin);
            }
            Message::Standard { message, origin } => {
                debug!("Standard not expected");
                let result: Value = serde_json::from_slice(&message).expect("JSON parse error");
                return to_json(result, origin);
            }
            Message::JSON { .. } => {
                return _message;
            }
            _ => panic!("Unexpected message type {}", _message),
        };
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

fn to_json(mut data: Value, origin: Option<OriginMessage>) -> Message {
    let naive_date = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap();
    let naive_datetime = naive_date.and_hms_opt(9, 10, 11).unwrap();
    let dt = Utc.from_utc_datetime(&naive_datetime);

    data.as_object_mut()
        .unwrap()
        .insert("_ts".to_string(), Value::String(dt.to_rfc3339()));

    return Message::JSON {
        message: data,
        origin,
    };
}
