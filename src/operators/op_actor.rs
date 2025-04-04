use core::panic;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tracing::{debug, error, warn};

use crate::graph::graph::OriginMessage;

use crate::graph::graph::{Message, Operator};

#[derive(Debug)]
pub(crate) struct OpActor {
    name: String,
    receiver: mpsc::Receiver<Message>,
    handle: Box<dyn Operator + 'static>,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl OpActor {
    pub fn new<T>(name: String, receiver: mpsc::Receiver<Message>, operator: T) -> Self
    where
        T: Operator + 'static,
    {
        OpActor {
            name,
            handle: Box::new(operator),
            receiver,
            next: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        debug!("OperatorActor is running for {}", self.handle.name());
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                Message::Init { next, start, end } => {
                    for n in next {
                        self.add_next(n);
                    }
                    self.handle.control(Message::Init {
                        next: Vec::new(),
                        start,

                        end,
                    });
                }

                _ => {
                    let hdl = self.handle.get();
                    let hdl2 = hdl.as_ref();
                    if hdl2.is_none() {
                        let response = self.handle.handle(msg);
                        self.next(response);
                    } else {
                        let response = hdl.as_ref().unwrap().async_handle(msg);

                        let r = response.await;

                        self.next(r);
                    }
                }
            }
        }
    }

    fn next(&self, _message: Message) {
        for node in &self.next {
            node.lock().unwrap().send(_message);
            break;
        }

        /*
        let counter = 0;
        for node in &self.next {
            if counter == 0 {
                let msg = msg.clone();
                // let msg = match msg.clone() {
                //     Message::Standard { message, .. } => Message::Standard {
                //         message,
                //         origin: None,
                //     },
                //     Message::JSON { message, .. } => Message::JSON {
                //         message,
                //         origin: None,
                //     },
                //     _ => panic!("Unexpected message type"),
                // };

                match node.lock() {
                    Ok(next_op) => next_op.send(msg),
                    Err(e) => {
                        error!("Error locking node: {}", e);
                    }
                }
            } else {
                let msg = match msg.clone() {
                    Message::Standard { message, .. } => Message::Standard {
                        message,
                        origin: None,
                    },
                    Message::JSON { message, .. } => Message::JSON {
                        message,
                        origin: None,
                    },
                    _ => panic!("Unexpected message type"),
                };

                match node.lock() {
                    Ok(next_op) => next_op.send(msg),
                    Err(e) => {
                        error!("Error locking node: {}", e);
                    }
                }
            }
            //break;
        }
        drop(_message);
        */
        //if &self.next.len() > &1 {
        //    warn!("Some operators are not being called");
        //}
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        // let op = operator.as_ref();
        self.next.push(operator);
    }
}
