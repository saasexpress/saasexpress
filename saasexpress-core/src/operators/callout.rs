use std::sync::{Arc, Mutex};

use chrono::Duration;
use fastrace::Span;
use fastrace::local::LocalSpan;
use futures::channel::oneshot;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::graph;
use crate::graph::graph::{AsyncHandleTrait, Graph, GraphRunner, GraphStatus};
use crate::graph::operator::{Operator, OperatorRef, OperatorRole, OperatorState, OperatorType};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::meta::NodeMeta;
use crate::graph::registry::GraphRegistry;
use crate::my_reg::register;
use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(crate) struct Callout {
    node_fqn: Option<String>,
    self_graph_name: Option<String>,

    state: OperatorState,
    graph_name: String,
    // graph: Option<Arc<Mutex<Graph>>>,
    graph_runner: Option<Arc<GraphRunner>>,
    //graph_runner: Option<Arc<GraphRunner>>,
    next: Vec<OperatorRole>,
}

impl From<serde_yaml::Value> for Callout {
    fn from(_value: serde_yaml::Value) -> Self {
        let graph_name = _value
            .get("graph_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Callout {
            node_fqn: None,
            self_graph_name: None,
            state: OperatorState::Pending,
            graph_name,
            //graph: None,
            graph_runner: None,
            next: Vec::new(),
        }
    }
}

impl Operator for Callout {
    fn _type(&self) -> OperatorType {
        if self.graph_runner.is_none() {
            warn!("Callout operator has no graph assigned yet");
            OperatorType::Endpoint
        } else {
            let graph = self.graph_runner.as_ref().unwrap();

            //let mut graph = graph.lock().unwrap();
            let op_node = graph.start_node().expect("Failed to start node");
            op_node._type()
        }
    }

    fn state(&self) -> OperatorState {
        warn!("State! {:?}", self.state);
        self.state.clone()
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

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        self.node_fqn = Some(node_meta.fqn());
        self.self_graph_name = Some(graph.name.clone());
    }

    fn finalize(&mut self) -> bool {
        if self.state == OperatorState::Ready {
            return true;
        }
        let graph_name = self.graph_name.clone();

        let graph = GraphRegistry::get_graph(&graph_name);

        if graph.is_none() {
            warn!("Graph not found {}", graph_name);
            false
        } else {
            let graph = graph.unwrap();
            let graph = graph.lock().unwrap();

            let runner = graph.runner.clone();

            self.graph_runner = Some(Arc::new(runner));
            self.state = OperatorState::Ready;
            true
        }
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }

                let node_fqn = self.node_fqn.clone().unwrap();

                // let self_graph_name = self.self_graph_name.clone().unwrap();
                // error!("Callout init {}", node_fqn);
                // watch_control_bus(self_graph_name, node_fqn);
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

        let graph_runner = self
            .graph_runner
            .as_ref()
            .expect("Graph runner not initialized")
            .clone();

        // let _origin = _message
        //     .get_origin()
        //     .expect("Failed to get origin from message");

        //let graph = self.graph.as_ref().expect("Graph not initialized").clone();

        //let graph_runner = {
        //let runner = self.graph_runner.as_ref().unwrap();
        //Arc::clone(runner)
        //};
        // let graph_runner = Arc::clone(self.graph_runner);

        //let graph_runner = graph_runner.expect("Graph runner not initialized");

        let next_node_mutex = self.next.get(0).unwrap();

        let next_node_mutex = &next_node_mutex.operator;

        let next_nd = Arc::clone(next_node_mutex);

        if graph_runner.state != GraphStatus::Running {
            warn!(
                "Graph runner is {:?}, cannot callout to graph: {}",
                graph_runner.state, graph_runner.name
            );
            next_nd.lock().unwrap().send(Message::Error {
                error: format!("Graph {} is stopped", graph_runner.name),
                origin: _message.take_origin(),
            });
            return;
        }

        let (tx, rx) = oneshot::channel::<Message>();

        tokio::spawn(
            async move {
                match _message {
                    Message::Standard {
                        message, origin, ..
                    } => {
                        let (sender, recv) = oneshot::channel();

                        let callout_message = Message::Standard {
                            message: message.clone(),
                            origin: Some(
                                OriginMessage::new(Some(sender))
                                    .with_span(Some(DebuggableSpan(callout_inner_span))),
                            ),
                        };

                        {
                            let _lspan = LocalSpan::enter_with_local_parent("start_graph");

                            //let _graph = graph.as_ref();
                            //let graph = _graph.lock().unwrap();

                            info!("Calling out to graph: {}", graph_runner.name);
                            graph_runner.call(callout_message);
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

                            //let _graph = graph.as_ref();
                            //let graph = _graph.lock().unwrap();

                            info!("Calling out to graph: {}", graph_runner.name);
                            graph_runner.call(callout_message);
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

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }
}

// fn watch_control_bus(self_graph_name: String, id: String) {
//     let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

//     // Register it
//     register(&id, tx);

//     tokio::spawn(async move {
//         loop {
//             loop {
//                 // Receive the message
//                 if let Some(msg) = rx.recv().await {
//                     info!(
//                         "[Node: {}] Received : {:?}",
//                         id,
//                         serde_json::to_string(&msg)
//                     );
//                 } else {
//                     warn!("Channel is closed");
//                     break;
//                 }
//             }
//         }
//     });
// }
