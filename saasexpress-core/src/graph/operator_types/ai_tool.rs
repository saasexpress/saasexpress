use serde_yaml::{Error, Value};
use tracing::{error, info};

use crate::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::{ControlCommand, Message},
    meta::NodeMeta,
    operator::{
        GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState,
        OperatorType,
    },
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
    id: String,
    node_fqn: Option<String>,

    name: String,
    operator: OperatorType,
    next_nodes: Vec<OperatorRole>,
    //pub(crate) operator: Arc<dyn AIToolOperator + Send + Sync + 'static>,
}

impl AITool {
    pub fn new(name: &str, tool: impl AIToolOperator + Send + Sync + 'static) -> Self {
        AITool {
            id: "".to_string(),
            node_fqn: None,
            name: name.to_string(),
            operator: OperatorType::AITool {
                tool: Arc::new(tool),
            },
            next_nodes: Vec::new(),
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
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let next_nodes = graph_operator_context.get_next_nodes();

        Arc::new(AITool {
            id: self.id.clone(),
            name: self.name.clone(),
            node_fqn: self.node_fqn.clone(),
            operator: Operator::_type(self),
            next_nodes,
        })
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.node_fqn = node_meta.fqn().into();
    }

    fn control(&mut self, message: Message) {
        match message {
            Message::Control { command, .. } => {
                match command {
                    ControlCommand::Start { runtime } => {
                        info!("Starting AITool with runtime: {:?}", runtime);
                        //self.start_agent();
                    }
                    _ => {
                        error!("Invalid control command for AIAgent: {:?}", command);
                    }
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
        for node in self.next_nodes.iter().filter(|o| o.role == role) {
            node.operator.send(_message);
            break;
        }
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
