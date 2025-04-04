use std::collections::HashSet;

use crate::operators::noop::NOOP;
use tokio::sync::mpsc::{self, Receiver};
use tracing::warn;

use crate::graph::graph::{Graph, GraphMod, Message};

#[derive(Debug)]
pub struct Port {
    end: Receiver<Message>,
}

impl Port {
    pub fn new(graph: &mut Graph, id: String) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>(8);
        let noop = NOOP::new(sender);

        graph.add_new_node(&id, noop);

        Port { end: receiver }
    }

    pub fn create() -> (Port, NOOP) {
        let (sender, receiver) = mpsc::channel::<Message>(8);
        let noop = NOOP::new(sender);

        (Port { end: receiver }, noop)
    }

    pub async fn req_reply(&mut self) -> Message {
        while let Some(msg) = self.wait().await {
            match msg {
                _ => {
                    warn!("Basic processor returning from ReqReply");
                    return msg;
                }
            }
        }
        return Message::Standard {
            message: vec![],
            origin: None,
        };
    }

    pub async fn wait(&mut self) -> Option<Message> {
        while let Some(msg) = self.end.recv().await {
            match msg {
                _ => {
                    warn!("Basic processor received message");
                    return Some(msg);
                }
            }
        }
        println!("Processing basic processor");
        return None;
    }
}
