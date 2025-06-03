use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};

use crate::graph::operator::{
    Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorRuntimeType, OperatorType,
};
use crate::graph::{
    graph::{AsyncHandleTrait, Graph},
    message::Message,
};

use fastrace::{Span, local::LocalSpan, prelude::SpanContext};
use tokio::sync::mpsc;
use tracing::{debug, error, info, info_span, instrument, span, warn};

use fastrace::future::FutureExt;
//use opentelemetry::trace::FutureExt;
use tracing::Instrument;

#[derive(Debug)]
pub(crate) struct OpActor {
    name: String,
    receiver: mpsc::Receiver<Message>,
    runtime: Arc<dyn OperatorRuntime + 'static>,
    //op_runtime: Option<Arc<dyn OperatorRuntime + 'static>>,
    next_nodes: Vec<OperatorRole>,
}

impl Drop for OpActor {
    fn drop(&mut self) {
        error!("Dropping OpActor: {}", self.name);
    }
}

impl OpActor {
    pub fn new(
        name: String,
        receiver: mpsc::Receiver<Message>,
        runtime: OperatorRuntimeType,
        next_nodes: Vec<OperatorRole>,
    ) -> Self {
        OpActor {
            name,
            runtime,
            receiver,
            next_nodes,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let msg = self.receiver.recv().await;
            if msg.is_none() {
                warn!("OperatorActor is stopping for {}", self.runtime.name());
                break;
            }
            let msg = msg.unwrap();

            match msg {
                Message::Init2 { .. } => {
                    panic!("This message type should not be used in OpActor: {}", msg);
                }
                Message::Init { .. } => {
                    panic!("This message type should not be used in OpActor: {}", msg);
                }
                Message::Control { .. } => {
                    panic!("This message type should not be used in OpActor: {}", msg);
                }
                _ => {
                    let hdl = self.runtime.get();

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

                        debug!("Handle {:?} {:?}", self.name, self.runtime._type());

                        let result = match self.runtime._type() {
                            OperatorType::Filter2 { operator } => {
                                debug!("Filter2 operator");
                                operator.handle(msg)
                            }
                            _ => self.runtime.handle(msg),
                        };

                        self.next(result);
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
        for node in &self.next_nodes {
            node.operator.send(_message);
            break;
        }
    }
}
