use std::collections::{HashMap, HashSet};
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

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRefRead, OperatorRole, OperatorRuntime,
    OperatorRuntimeType, OperatorState, OperatorType,
};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::meta::NodeMeta;

use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(crate) struct Timer {
    id: String,
    on_start: bool,
    interval_ms: Option<Duration>,
    iterations: u16,
    next_nodes: Vec<OperatorRole>,
}

impl From<serde_yaml::Value> for Timer {
    fn from(_value: serde_yaml::Value) -> Self {
        Timer {
            id: "".to_string(),
            next_nodes: Vec::new(),
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

    fn new_runtime(
        &self,
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let next_nodes = graph_operator_context.get_next_nodes();

        Arc::new(Timer {
            id: self.id.clone(),
            next_nodes,
            on_start: self.on_start,
            interval_ms: self.interval_ms,
            iterations: self.iterations,
        })
    }

    fn init(&mut self, _: &mut Graph, _node_meta: &NodeMeta) {}

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Control { .. } => {
                debug!("Control");
            }

            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }
}

impl Timer {
    fn next(&self, message: Message) {
        for n in &self.next_nodes {
            n.operator.send(message);
            break;
        }
    }

    fn start(&mut self) -> bool {
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
            let sender = self.next_nodes.get(0).unwrap().clone().operator;

            let name = Operator::name(self);

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

                    sender.send(message);

                    rx.in_span(recv_span).await.unwrap();
                }
            });
        }
        true
    }
}

async fn interval_trigger(
    name: String,
    sender: OperatorRuntimeType,
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
        sender.send(message);

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

impl OperatorRuntime for Timer {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, mut _message: Message) -> Message {
        // Only handle the message if we want to start a timer
        if self.on_start == false && self.interval_ms.is_some() {
            let sender = self.next_nodes.get(0).unwrap().clone().operator;

            let mpsc_respond_to = _message
                .get_origin()
                .unwrap()
                .mpsc_respond_to
                .clone()
                .unwrap();

            let interval = self.interval_ms.unwrap();
            let iterations = self.iterations;

            let name = OperatorRuntime::name(self);

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

    fn send(&self, message: Message) {
        self.next(message)
    }
}
