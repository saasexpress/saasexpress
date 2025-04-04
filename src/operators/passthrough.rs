use std::sync::{Arc, Mutex};

use tracing::debug;

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::graph::{Message, Operator};

#[derive(Clone, Debug)]
pub(crate) struct Passthrough;

impl From<serde_yaml::Value> for Passthrough {
    fn from(_value: serde_yaml::Value) -> Self {
        Passthrough {}
    }
}

impl Operator for Passthrough {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "Passthrough".to_string()
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
