use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::{debug, info};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::message::Message;
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorType,
};

use crate::graph::meta::NodeMeta;

#[derive(Clone, Debug)]
pub(crate) struct Passthrough;

impl From<&serde_yaml::Value> for Passthrough {
    fn from(_value: &serde_yaml::Value) -> Self {
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

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
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

impl OperatorRuntime for Passthrough {
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
        _message
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
