use serde_yaml::{Error, Value};
use tracing::error;

use crate::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::Message,
    meta::NodeMeta,
    operator::{Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState, OperatorType},
};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};

pub trait AIToolOperator: Sync + Send + Debug {
    fn name(&self) -> String;
    fn get_schema(&self) -> Result<serde_yaml::Value, Error>;
    fn invoke(&self, message: Message) -> Message;
}

// impl From<serde_yaml::Value> for AITool {
//     fn from(_value: serde_yaml::Value) -> Self {
//         AITool { operator: None }
//     }
// }

#[derive(Debug)]
pub struct AITool {
    node_fqn: Option<String>,

    name: String,
    operator: OperatorType,
    next: Vec<OperatorRole>,
    //pub(crate) operator: Arc<dyn AIToolOperator + Send + Sync + 'static>,
}

impl AITool {
    pub fn new(name: &str, tool: impl AIToolOperator + Send + Sync + 'static) -> Self {
        AITool {
            node_fqn: None,
            name: name.to_string(),
            operator: OperatorType::AITool {
                tool: Arc::new(tool),
            },
            next: Vec::new(),
        }
    }
}

impl Operator for AITool {
    fn _type(&self) -> OperatorType {
        match &self.operator {
            OperatorType::AITool { tool } => OperatorType::AITool {
                tool: Arc::clone(tool),
            },
            _ => panic!("Invalid operator type"),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(AITool {
            name: self.name.clone(),
            node_fqn: self.node_fqn.clone(),
            operator: Operator::_type(self),
            next: self.next.clone(),
        })
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.node_fqn = node_meta.fqn().into();
    }

    fn control(&mut self, message: Message) {
        match message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }
            }
            _ => {
                error!("Unexpected message type {}", message);
            }
        }
    }
}

impl AITool {
    fn next(&self, role: String, _message: Message) {
        for node in self.next.iter().filter(|o| o.role == role) {
            node.operator.send(_message);
            break;
        }
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }
}

impl OperatorRuntime for AITool {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, in_message: Message) -> Message {
        let OperatorType::AITool { tool } = &self.operator else {
            error!("Invalid operator type {:?}", self.operator);
            return Message::Error {
                error: "Invalid operator type".to_string(),
                origin: None,
            };
        };

        tool.invoke(in_message)
    }

    fn send(&self, message: Message) {
        self.next(OperatorRole::default(), message);
    }
}
