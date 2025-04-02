use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tracing::{debug, error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use super::{
    super::graph::{Message, Operator},
    op_actor::OpActor,
};

#[derive(Debug)]
pub(crate) struct OperatorActorHandle {
    sender: mpsc::Sender<Message>,
    name: String,
    _nodes: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl OperatorActorHandle {
    pub fn new<T>(operator: T) -> Self
    where
        T: Operator + 'static,
    {
        let nm = operator.name();
        let name = operator.name().clone();
        let (sender, receiver) = mpsc::channel(8);

        let mut actor = OpActor::new(name, receiver, operator);
        tokio::spawn(async move { actor.run().await });

        Self {
            name: String::clone(&nm),
            sender,
            _nodes: Vec::new(),
        }
    }
}

impl Operator for OperatorActorHandle {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        panic!("Not implemented");
    }

    fn init(&mut self, _: &mut Graph) {
        warn!("Not implemented");
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { .. } => match self.sender.try_send(_message) {
                Ok(_) => debug!("Message sent to {}", self.name),
                Err(e) => panic!("Failed to send: {}", e),
            },
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, _message: Message) {
        match _message {
            Message::Init { .. } => {
                panic!("Unexpected message type for send");
            }
            _ => match self.sender.try_send(_message) {
                Ok(_) => debug!("Message sent to {}", self.name),
                Err(e) => {
                    error!("Failed to send: {}", e)
                }
            },
        }
    }
    fn wait(&self) -> Message {
        panic!("Not implemented");
    }
    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        self._nodes.as_ref()
    }
}
