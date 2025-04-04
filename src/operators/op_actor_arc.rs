use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tracing::{debug, error, warn};

use crate::graph::graph::OriginMessage;

use crate::graph::graph::{Message, Operator};

#[derive(Debug)]
pub(crate) struct OpActor {
    receiver: mpsc::Receiver<Arc<Message>>,
    handle: Box<dyn Operator + 'static>,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl OpActor {
    pub fn new<T>(receiver: mpsc::Receiver<Arc<Message>>, operator: T) -> Self
    where
        T: Operator + 'static,
    {
        OpActor {
            handle: Box::new(operator),
            receiver,
            next: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        debug!("OperatorActor is running for {}", self.handle.name());
        while let Some(msg_arc) = self.receiver.recv().await {
            let msg = msg_arc.as_ref();
            match msg {
                Message::Init { next, start, end } => {
                    let _next = next.to_owned();
                    let _start = start.to_owned();
                    let _end = end.to_owned();
                    for n in _next {
                        self.add_next(n);
                    }
                    self.handle.control(Message::Init {
                        next: Vec::new(),
                        start: _start,
                        end: _end,
                    });
                }

                _ => {
                    let hdl = self.handle.get();
                    let hdl2 = hdl.as_ref();
                    if hdl2.is_none() {
                        let response = self.handle.handle_ptr(msg_arc);
                        self.next(response);
                    } else {
                        let _msg = msg_arc.to_owned();

                        let response = hdl.as_ref().unwrap().async_handle_ptr(_msg);

                        let r = response.await;

                        self.next(r);
                    }
                }
            }
        }
    }

    fn next(&self, _message: Arc<Message>) {
        let msg = Arc::new(_message);
        for n in &self.next {
            n.lock().unwrap().send_ptr(Arc::clone(&msg));
        }
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        // let op = operator.as_ref();
        self.next.push(operator);
    }
}
