use std::iter;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use fastrace::Span;
use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorRef, OperatorRole, OperatorType};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;

use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(crate) struct Timer {
    on_start: bool,
    interval_ms: Option<Duration>,
    iterations: u16,
    next: Vec<OperatorRole>,
}

impl From<serde_yaml::Value> for Timer {
    fn from(_value: serde_yaml::Value) -> Self {
        Timer {
            next: Vec::new(),
            iterations: _value
                .get("iterations")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u16,
            interval_ms: _value
                .get("interval_ms")
                .and_then(|v| v.as_u64())
                .map(|v| Duration::from_millis(v)),
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

    fn handle(&self, mut _message: Message) -> Message {
        // Only handle the message if we want to start a timer
        if self.on_start == false && self.interval_ms.is_some() {
            let sender = self.next.get(0).unwrap().clone().operator;

            let mpsc_respond_to = _message
                .get_origin()
                .unwrap()
                .mpsc_respond_to
                .clone()
                .unwrap();

            let interval = self.interval_ms.unwrap();
            let iterations = self.iterations;

            let name = self.name();

            tokio::spawn(async move {
                interval_trigger(name, sender, _message, interval, iterations).await;
            });

            return Message::JSON {
                message: json!({"timer": "started"}),
                origin: Some(OriginMessage::new(None).mpsc_respond_to(mpsc_respond_to)),
            };
        }
        panic!("Timer does not handle any messages");
    }

    fn init(&mut self, _: &mut Graph, _node_meta: &NodeMeta) {}

    fn finalize(&mut self) -> bool {
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
        } else if self.interval_ms.is_none() {
            let sender = self.next.get(0).unwrap().clone().operator;

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
        true
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
            let operator = n.operator.lock().unwrap();
            operator.send(message);
            break;
        }
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }
}

async fn interval_trigger(
    name: String,
    sender: OperatorRef,
    mut message: Message,
    interval: Duration,
    iterations: u16,
) {
    let stream_channel = message
        .take_origin()
        .expect("Failed to get origin from message")
        .mpsc_respond_to
        .expect("Timer expects a mpsc respond to");

    let mut counter = 0;
    loop {
        sleep(interval);

        let root_span = Span::root(format!("timer {:?}", name), SpanContext::random());
        let recv_span = Span::enter_with_parent("recv_span", &root_span);
        // let _guard = root_span.set_local_parent();

        let (tx, rx) = oneshot::channel::<Message>();

        let message = Message::JSON {
            message: json!({"timer": "triggered"}),
            origin: Some(OriginMessage::new(Some(tx)).with_span(Some(DebuggableSpan(root_span)))),
        };

        // Send message to the next operator
        sender.lock().unwrap().send(message);

        // wait for cycle to finish
        let response = rx.in_span(recv_span).await.unwrap();

        // send response back to the stream channel
        stream_channel
            .send(response)
            .await
            .expect("Failed to send response");

        counter += 1;
        if iterations != 0 && counter >= iterations {
            info!("Timer finished after {} iterations", counter);
            break;
        }
    }
}
