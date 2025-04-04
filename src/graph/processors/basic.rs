use core::panic;

use tokio::sync::mpsc::{self, Receiver};
use tracing::{debug, warn};

use crate::graph::graph::{Graph, GraphMod, Message};
use crate::operators::noop::NOOP;

#[derive(Debug)]
pub struct BasicProcessor {
    end: Receiver<Message>,
}

impl BasicProcessor {
    pub fn new(graph: &mut Graph) -> Self {
        debug!("Creating Basic Processor - and adding _end node");
        let (sender, receiver) = mpsc::channel::<Message>(8);
        let noop = NOOP::new(sender);

        graph.add_new_node("_end", noop);

        BasicProcessor { end: receiver }
    }

    pub async fn req_reply(&mut self) -> Message {
        let msg = self.wait().await;

        match msg {
            _ => {
                warn!("Basic processor returning from ReqReply");
                return msg;
            }
        }
        // return Message::Standard { message: vec![] };
    }

    pub async fn wait(&mut self) -> Message {
        while let Some(msg) = self.end.recv().await {
            match msg {
                _ => {
                    warn!("Basic processor received message");
                    return msg;
                }
            }
        }
        println!("Processing basic processor");
        panic!("Basic processor received no message");
    }
}
