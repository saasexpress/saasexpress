use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use fake::faker::name::en::Name;
use saasexpress_core::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::Message,
    meta::NodeMeta,
    operator::{GraphOperatorContext, Operator, OperatorRef, OperatorRuntime, OperatorType},
};
use serde_json::json;
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

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        warn!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        warn!("Not implemented");
    }
}

impl OperatorRuntime for Faker {
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
            Message::JSON {
                message, origin, ..
            } => {
                let mut newmsg: HashMap<String, String> = HashMap::new();

                let val: String = Name().fake();
                newmsg.insert("name".to_string(), val.into());

                let data = json!(&newmsg);

                return Message::JSON {
                    message,
                    origin: Some(origin.unwrap().temp_push("faker".to_string(), data)),
                };
            }
            _ => panic!("Unexpected message type {}", _message),
        }
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
