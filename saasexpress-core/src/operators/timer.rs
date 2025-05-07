use std::sync::{Arc, Mutex};
use std::thread::sleep;

use fastrace::Span;
use fastrace::local::LocalSpan;
use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;

use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(crate) struct Timer {
    on_start: bool,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl From<serde_yaml::Value> for Timer {
    fn from(_value: serde_yaml::Value) -> Self {
        Timer {
            next: Vec::new(),
            on_start: _value
                .get("on_start")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    }
}

impl Operator for Timer {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "Timer".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        // Timer does not handle any messages
        panic!("Timer does not handle any messages");
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {}

    fn finalize(&mut self) {
        if self.on_start {
            let root_span = Span::root("timer", SpanContext::random());

            let _guard = root_span.set_local_parent();

            let (mpsc_respond_to, mut receiver) = mpsc::channel::<Message>(10);

            let message = Message::JSON {
                message: json!({"timer": "started"}),
                origin: Some(OriginMessage::new(None).mpsc_respond_to(mpsc_respond_to)),
            };

            self.next(message);

            tokio::spawn(
                async move {
                    while let Some(message) = receiver.recv().await {
                        match message {
                            Message::JSON { message, .. } => {
                                info!("Timer received message: {:?}", message);
                            }
                            _ => {
                                error!("Unexpected message type {:?}", message);
                            }
                        }
                    }
                }
                .in_span(root_span),
            );
        } else {
            let sender = self.next.get(0).unwrap().clone();

            let name = self.name();

            tokio::spawn(async move {
                loop {
                    sleep(std::time::Duration::from_secs(30));

                    let root_span = Span::root(format!("timer {:?}", name), SpanContext::random());

                    let recv_span = Span::enter_with_parent("recv_span", &root_span);
                    // let _guard = root_span.set_local_parent();

                    let (tx, rx) = oneshot::channel::<Message>();

                    let message = Message::JSON {
                        message: json!({"timer": "started"}),
                        origin: Some(
                            OriginMessage::new(Some(tx)).with_span(Some(DebuggableSpan(root_span))),
                        ),
                    };

                    info!("Timer sending message: {:?}", message);
                    sender.lock().unwrap().send(message);

                    rx.in_span(recv_span).await.unwrap();
                }
            });
        }
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }
            }
            Message::Control { .. } => {
                debug!("Control");
            }

            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, message: Message) {
        self.next(message)
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}

impl Timer {
    fn next(&self, message: Message) {
        for n in &self.next {
            let operator = n.lock().unwrap();
            operator.send(message);
            break;
        }
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }
}
