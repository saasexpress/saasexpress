use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use chrono::Duration;
use fastrace::Span;
use fastrace::local::LocalSpan;
use futures::channel::oneshot;
use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::graph;
use crate::graph::graph::{AsyncHandleTrait, Graph, GraphMod, GraphRunner, GraphStatus};
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorState,
    OperatorType,
};

use crate::graph::message::{DebuggableSpan, Message, OriginMessage};

use crate::graph::meta::NodeMeta;
use crate::graph::registry::GraphRegistry;
use crate::my_reg::{ControlEvent, broadcast_event, register};
use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub(super) struct Callout {
    id: String,
    node_fqn: Option<String>,
    self_graph_name: Option<String>,
    graph_name: String,
    state: OperatorState, // only applicable to runtime
    // graph: Option<Arc<Mutex<Graph>>>,
    //callout_graph: Option<Arc<Mutex<Graph>>>,
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
            id: "".to_string(),
            node_fqn: None,
            self_graph_name: None,
            graph_name,
            graph_runner: None,
            state: OperatorState::Pending,
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
            let op_node = graph.start_node().expect("Failed to get start node");
            op_node._type()
        }
    }

    fn name(&self) -> String {
        "Callout".to_string()
    }

    fn new_runtime(
        &self,
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let self_graph_name = self.self_graph_name.clone().unwrap();
        let callout_graph_name = self.graph_name.clone();

        let callout_graph = { GraphRegistry::get_graph(&callout_graph_name) };

        if callout_graph.is_none() {
            warn!(
                "Graph not found {} - cloning current operator runtime",
                callout_graph_name
            );
            Arc::new(self.clone())
        } else {
            let callout_graph = callout_graph.as_ref().unwrap();
            let callout_graph = callout_graph.lock().unwrap();

            if callout_graph.runner.state == GraphStatus::Inactive {
                warn!(
                    "Callout graph '{}' is inactive, cannot create new operator runtime",
                    callout_graph_name
                );
                return Arc::new(self.clone());
            }

            info!("[{}] Creating new operator runtime", self_graph_name);
            let runner = Arc::clone(&callout_graph.runner);

            let next_nodes = {
                //let self_graph = GraphRegistry::get_graph(&self_graph_name).unwrap();
                //let self_graph = self_graph.lock().unwrap();
                Graph::get_next_nodes(graph_operator_context.clone())
            };

            info!(
                "Next nodes for callout operator: {}, {:?}",
                self.id, next_nodes
            );

            Arc::new(Callout {
                id: self.id.clone(),
                node_fqn: self.node_fqn.clone(),
                self_graph_name: self.self_graph_name.clone(),
                state: OperatorState::Ready,
                graph_name: self.graph_name.clone(),
                //callout_graph: None,
                graph_runner: Some(runner),
                next: next_nodes,
            })
        }
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        self.id = node_meta.name.clone();
        self.node_fqn = Some(node_meta.fqn());
        self.self_graph_name = Some(graph.name.clone());
        self.state = OperatorState::Pending;

        let self_node_id = node_meta.name.clone();
        let self_graph_name = graph.name.clone();
        let fqn = node_meta.fqn().clone();

        let callout_graph_name = self.graph_name.clone();

        // Watch for changes related to the callout graph
        // If the graph is running, then trigger this operator graph to replace its runtimes
        let (sender, mut receiver) = mpsc::channel::<ControlEvent>(10);

        // listen for events related to the graph: self_graph_name
        register(fqn.as_str(), sender);

        //let boxed_self = Arc::new(Mutex::new(self));

        tokio::spawn(async move {
            loop {
                let control_event = receiver.recv().await;
                match control_event {
                    None => {
                        info!("Control channel closed for {}", fqn);
                        break;
                    }
                    Some(message) => {
                        info!("Received control event: {:?}", message);
                        if message.graph_name == callout_graph_name {
                            // Replace the graph runtimes
                            Graph::runtime_expired(self_graph_name.clone(), self_node_id.clone());
                            // broadcast_event(ControlEvent {
                            //     graph_id: message.graph_id,
                            //     graph_name: message.graph_name,
                            //     state: GraphStatus::Active,
                            //     operator_names: vec![self_node_id.clone()],
                            // })
                            // .await;
                        }
                    }
                }
            }
        });
    }

    fn control(&mut self, _message: Message) {
        match _message {
            // Message::Init2 { id, next, .. } => {
            //     info!("Init2 callout operator: {}", id);
            //     self.next = next;

            //     let callout_graph_name = self.graph_name.clone();

            //     let callout_graph = GraphRegistry::get_graph(&callout_graph_name);

            //     if callout_graph.is_none() {
            //         warn!(
            //             "Callout graph {} not found, cannot initialize callout operator",
            //             self.id
            //         );
            //     } else {
            //         self.callout_graph = callout_graph;

            //         info!(
            //             "Callout graph {} found, initializing callout operator",
            //             self.id
            //         );
            //     }

            //     // let event = ControlEvent {
            //     //     graph_id: id,
            //     //     graph_name: self.graph_name.clone(),
            //     //     state: GraphStatus::Active,
            //     //     operator_names: vec![self.id.clone()],
            //     // };
            //     // tokio::spawn(async move {
            //     //     broadcast_event(event).await;
            //     // });
            // }
            // Message::Init { next, .. } => {
            //     info!("Replacing next nodes for callout operator");
            //     self.next = next;

            //     let node_fqn = self.node_fqn.clone().unwrap();

            //     let callout_graph_name = self.graph_name.clone();

            //     let callout_graph = GraphRegistry::get_graph(&callout_graph_name);

            //     if callout_graph.is_none() {
            //         warn!(
            //             "Callout graph {} not found, cannot initialize callout operator",
            //             self.id
            //         );
            //     } else {
            //         self.callout_graph = callout_graph;

            //         info!(
            //             "Callout graph {} found, initializing callout operator",
            //             self.id
            //         );
            //     }
            //     // let self_graph_name = self.self_graph_name.clone().unwrap();
            //     // error!("Callout init {}", node_fqn);
            //     // watch_control_bus(self_graph_name, node_fqn);
            // }
            Message::Control { .. } => {
                debug!("Control");
            }

            _ => {
                panic!("Unexpected message type for control {}", _message);
            }
        }
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

        //let graph = self.graph.as_ref().expect("Graph not initialized").clone();

        //let graph_runner = {
        //let runner = self.graph_runner.as_ref().unwrap();
        //Arc::clone(runner)
        //};
        // let graph_runner = Arc::clone(self.graph_runner);

        //let graph_runner = graph_runner.expect("Graph runner not initialized");

        info!("Next nodes: {}", self.next.len());
        let next_node_mutex = self.next.get(0).unwrap();
        let next_node_mutex = &next_node_mutex.operator;
        let next_nd = Arc::clone(next_node_mutex);

        let graph_runner = self
            .graph_runner
            .as_ref()
            .expect("Graph runner not initialized")
            .clone();

        if graph_runner.state == GraphStatus::Inactive {
            warn!(
                "Graph runner is {:?}, cannot callout to graph: {}",
                graph_runner.state, graph_runner.name
            );
            next_nd.send(Message::Error {
                error: format!("Graph {} is inactive", graph_runner.name),
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
            next_node.send(response);
        });
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

impl OperatorRuntime for Callout {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn state(&self) -> OperatorState {
        self.state.clone()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
    }

    fn send(&self, message: Message) {
        self.next(message);
    }
}
