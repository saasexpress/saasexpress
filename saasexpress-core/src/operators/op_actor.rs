use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};

use crate::graph::message::Message;
use fastrace::{Span, local::LocalSpan, prelude::SpanContext};
use tokio::sync::mpsc;
use tracing::{debug, error, info, info_span, instrument, span, warn};

use crate::graph::graph::Operator;
use fastrace::future::FutureExt;
//use opentelemetry::trace::FutureExt;
use tracing::Instrument;

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

        loop {
            let msg = self.receiver.recv().await.unwrap();

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
                        let nm = format!("op_actor_handler ({})", self.name);
                        let parent_span = msg.get_span();
                        let span = match parent_span {
                            Some(span) => Span::enter_with_parent(nm, span),
                            None => {
                                error!("No span found {}", nm);
                                Span::root(nm, SpanContext::random())
                            }
                        };
                        let _guard = span.set_local_parent();

                        let response = self.handle.handle(msg);
                        self.next(response);
                    } else {
                        debug!("Async handle found {}", self.name);

                        let nm = format!("op_actor_handler (async) ({})", self.name);
                        let parent_span = msg.get_span();
                        let child_span = match parent_span {
                            Some(span) => Span::enter_with_parent(nm, span),
                            None => {
                                error!("No span found {}", nm);
                                Span::root(nm, SpanContext::random())
                            }
                        };

                        let r = {
                            let response = hdl.as_ref().unwrap().async_handle(msg);
                            response.in_span(child_span).await
                        };

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
