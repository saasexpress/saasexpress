use std::sync::{Arc, Mutex};

use crate::graph::message::Message;
use tokio::sync::mpsc;
use tracing::debug;

use crate::graph::graph::Operator;

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
                Message::Init {
                    id,
                    next,
                    start,
                    end,
                } => {
                    for n in next {
                        self.add_next(n);
                    }
                    self.handle.control(Message::Init {
                        id,
                        next: Vec::new(),
                        start,
                        end,
                    });
                }

                _ => {
                    let hdl = self.handle.get();
                    if hdl.is_none() {
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
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }
}
