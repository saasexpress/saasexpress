use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::debug;

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState, OperatorType,
};

use crate::graph::message::Message;

use crate::graph::meta::NodeMeta;

#[derive(Clone, Debug)]
pub(crate) struct Terminate;

impl From<serde_yaml::Value> for Terminate {
    fn from(_value: serde_yaml::Value) -> Self {
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

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("Not implemented");
    }
}

impl OperatorRuntime for Terminate {
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
        return Message::NoOp {};
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
