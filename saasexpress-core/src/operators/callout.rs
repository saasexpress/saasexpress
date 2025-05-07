use std::sync::{Arc, Mutex};

use fastrace::Span;
use fastrace::local::LocalSpan;
use futures::channel::oneshot;
use tracing::{debug, info};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;
use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(crate) struct Callout {
    graph_name: String,
    graph: Option<Arc<Mutex<Graph>>>,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl From<serde_yaml::Value> for Callout {
    fn from(_value: serde_yaml::Value) -> Self {
        let graph_name = _value
            .get("graph_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Callout {
            graph_name,
            graph: None,
            next: Vec::new(),
        }
    }
}

impl Operator for Callout {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "Callout".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {}

    fn finalize(&mut self) {
        let graph_registry = crate::graph::registry::GraphRegistry::get_instance();
        let graph = graph_registry
            .lock()
            .unwrap()
            .get_graph_by_name(&self.graph_name)
            .unwrap();
        self.graph = Some(graph);
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
                panic!("Unexpected message type for control {}", _message);
            }
        }
    }

    fn send(&self, message: Message) {
        self.next(message);
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}

impl Callout {
    fn next(&self, mut _message: Message) {
        // setup span for tracing
        let parent_span = _message.get_span().expect("Failed to get span");
        let callout_span = Span::enter_with_parent("callout", parent_span);
        let callout_inner_span = Span::enter_with_parent("callout_inner", parent_span);

        // let _origin = _message
        //     .get_origin()
        //     .expect("Failed to get origin from message");

        let graph = self.graph.as_ref().expect("Graph not initialized").clone();

        let next_node_mutex = self.next.get(0).unwrap();

        let next_nd = Arc::clone(next_node_mutex);

        let (tx, rx) = oneshot::channel::<Message>();

        tokio::spawn(
            async move {
                match _message {
                    Message::Standard {
                        message, origin, ..
                    } => {
                        let (sender, recv) = oneshot::channel();

                        let callout_message = Message::Standard {
                            message: Vec::new(),
                            origin: Some(
                                OriginMessage::new(Some(sender))
                                    .with_span(Some(DebuggableSpan(callout_inner_span))),
                            ),
                        };

                        {
                            let _lspan = LocalSpan::enter_with_local_parent("start_graph");

                            let _graph = graph.as_ref();
                            let graph = _graph.lock().unwrap();

                            info!("Calling out to graph: {}", graph.name);
                            graph.start(callout_message);
                        }

                        let response = recv.await.unwrap();
                        info!("Callout received response {:?}", response);

                        let tup = Message::Tuple {
                            message_1: Box::new(Message::Standard {
                                message,
                                origin: None,
                            }),
                            message_2: Box::new(response),
                            origin,
                        };

                        tx.send(tup).expect("Failed to send response");
                    }
                    Message::ReqReply {
                        message,
                        respond_to,
                        ..
                    } => {
                        let (sender, recv) = oneshot::channel();

                        let callout_message = Message::Standard {
                            message: Vec::new(),
                            origin: Some(
                                OriginMessage::new(Some(sender))
                                    .with_span(Some(DebuggableSpan(callout_inner_span))),
                            ),
                        };

                        {
                            let _lspan = LocalSpan::enter_with_local_parent("start_graph");

                            let _graph = graph.as_ref();
                            let graph = _graph.lock().unwrap();

                            info!("Calling out to graph: {}", graph.name);
                            graph.start(callout_message);
                        }

                        let response = recv.await.unwrap();
                        info!("Callout received response {:?}", response);

                        let tup = Message::Tuple {
                            message_1: Box::new(Message::Standard {
                                message,
                                origin: None,
                            }),
                            message_2: Box::new(response),
                            origin: Some(OriginMessage::new(Some(respond_to))),
                        };

                        tx.send(tup).expect("Failed to send response");
                    }
                    _ => {
                        panic!("Unexpected message type {}", _message);
                    }
                }
            }
            .in_span(callout_span),
        );
        tokio::spawn(async move {
            let response = rx.await.expect("Failed to receive response");

            let next_node = next_nd.clone();
            next_node.lock().unwrap().send(response);
        });
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }
}
