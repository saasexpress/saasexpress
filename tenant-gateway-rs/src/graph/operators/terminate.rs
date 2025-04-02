use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use tracing::{debug, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use super::super::graph::{Message, Operator};

#[derive(Clone, Debug)]
pub(crate) struct Terminate;

impl From<serde_yaml::Value> for Terminate {
    fn from(value: serde_yaml::Value) -> Self {
        Terminate {}
    }
}

impl Operator for Terminate {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "Terminate".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        _message
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
