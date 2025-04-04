use core::panic;

use crate::graph::graph::{Graph, GraphMod, Message};
use crate::operators::noop::NOOP;
use tokio::sync::mpsc::{self, Receiver};
use tracing::warn;

use super::XProcessor;

#[derive(Debug)]
pub struct ComplexProcessor {
    end: Receiver<Message>,
}

impl ComplexProcessor {
    pub fn new(graph: &mut Graph) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>(8);
        let noop = NOOP::new(sender);

        graph.add_new_node("_end", noop);

        ComplexProcessor { end: receiver }
    }
}

impl XProcessor for ComplexProcessor {
    async fn req_reply(&mut self) -> Message {
        let msg = self.wait().await;
        match msg {
            _ => {
                warn!("Complex processor returning from ReqReply");
                return msg;
            }
        }
        // return Message::Standard { message: vec![] };
    }

    async fn wait(&mut self) -> Message {
        while let Some(msg) = self.end.recv().await {
            match msg {
                _ => {
                    warn!("Complex processor received message");
                    return msg;
                }
            }
        }
        println!("Processing basic processor");
        panic!("Complex processor received no message");
    }
}
