use serde_yaml::{Error, Value};
use tracing::error;

use crate::graph::{
    graph::{AsyncHandleTrait, Graph, Operator, OperatorRole, OperatorState, OperatorType},
    message::Message,
    meta::NodeMeta,
};
use core::panic;
use std::{fmt::Debug, sync::Arc};

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

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        self.node_fqn = node_meta.fqn().into();
    }

    fn control(&mut self, message: Message) {
        match message {
            Message::Init {
                id,
                next,
                start,
                end,
            } => {
                for n in next {
                    self.add_next(n);
                }
            }
            _ => {
                error!("Unexpected message type {}", message);
            }
        }
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

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<std::sync::Arc<std::sync::Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }

    fn send(&self, message: Message) {
        self.next(OperatorRole::default(), message);
    }

    fn state(&self) -> OperatorState {
        OperatorState::Ready
    }

    fn finalize(&mut self) -> bool {
        tracing::debug!("Default finalize operator {} - no action", self.name());
        true
    }

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }

    fn handle_ptr(&self, message: Arc<Message>) -> Arc<Message> {
        tracing::debug!("default handle (passthrough)... {}", self.name());
        return message;
    }

    fn next_ptr(&self, message: Arc<Message>) {
        // Sending message to next operator
        for n in self.get_output_channels() {
            n.lock().unwrap().send_ptr(message.to_owned());
            //break;
        }
    }
}

impl AITool {
    fn next(&self, role: String, _message: Message) {
        for node in self.next.iter().filter(|o| o.role == role) {
            node.operator.lock().unwrap().send(_message);
            break;
        }
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }
}
