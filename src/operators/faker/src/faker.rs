use std::sync::{Arc, Mutex};

use fake::faker::name::en::Name;
use saasexpress_core::graph::{
    graph::{AsyncHandleTrait, Graph, Operator, OperatorType},
    message::Message,
};
use tracing::warn;

use fake::Fake;

#[derive(Clone, Debug)]
pub struct Faker;

impl From<serde_yaml::Value> for Faker {
    fn from(_value: serde_yaml::Value) -> Self {
        Faker {}
    }
}

impl Operator for Faker {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "Faker".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::JSON {
                message, origin, ..
            } => {
                let mut newmsg = message.as_object().unwrap().clone();

                let val: String = Name().fake();
                newmsg.insert("fake".to_string(), val.into());

                return Message::JSON {
                    message: serde_json::Value::Object(newmsg),
                    origin,
                };
            }
            _ => panic!("Unexpected message type {}", _message),
        }
    }

    fn init(&mut self, _: &mut Graph) {
        warn!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        warn!("Not implemented");
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
