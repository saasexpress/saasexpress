use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use tokio::sync::mpsc::{self, Sender};
use tracing::{debug, error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorType};

use crate::graph::message::{Message, OriginMessage};

use crate::graph::meta::NodeMeta;

//use futures::SinkExt;

#[derive(Debug)]
pub(crate) struct NOOP {
    sender: mpsc::Sender<Message>,
}

impl NOOP {
    pub fn new(send: Sender<Message>) -> Self {
        NOOP { sender: send }
    }
}

impl Operator for NOOP {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }
    fn name(&self) -> String {
        "NOOP".to_string()
    }

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(NOOP {
            sender: self.sender.clone(),
        })
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("NOOP - no control to do - Not implemented");
    }
}

impl OperatorRuntime for NOOP {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        _message
    }

    fn send(&self, _message: Message) {
        debug!("Finished.. sending to respond_to..");
        match _message {
            Message::HTTP {
                message,
                origin,
                headers,
                status,
            } => {
                if let Some(origin_message) = origin {
                    if let Some(mpsc_respond_to) = origin_message.mpsc_respond_to {
                        tokio::spawn(async move {
                            debug!("Sending MPSC response");
                            mpsc_respond_to
                                .send(Message::HTTP {
                                    message: message.to_owned(),
                                    origin: None,
                                    headers,
                                    status,
                                })
                                .await
                                .expect("[JSON] Failed to send response");
                        });
                    } else {
                        let respond_to = origin_message.respond_to.expect("No respond_to channel");

                        respond_to
                            .send(Message::HTTP {
                                message: message.to_owned(),
                                origin: None,
                                headers,
                                status,
                            })
                            .expect("[Standard] Failed to send response");
                    }
                } else {
                    warn!("No respond_to channel to send to");
                }
            }
            Message::Standard { message, origin } => {
                if let Some(origin_message) = origin {
                    if let Some(mpsc_respond_to) = origin_message.mpsc_respond_to {
                        tokio::spawn(async move {
                            debug!("Sending MPSC response");
                            error!("Sending without an origin!  So no span!");
                            mpsc_respond_to
                                .send(Message::Standard {
                                    message: message.to_owned(),
                                    origin: None, //Some(origin_message),
                                })
                                .await
                                .expect("[JSON] Failed to send response");
                        });
                    } else {
                        let respond_to = origin_message.respond_to.expect("No respond_to channel");
                        let span = origin_message.span;

                        respond_to
                            .send(Message::Standard {
                                message: message.to_owned(),
                                origin: Some(
                                    OriginMessage::new(None)
                                        .with_span(span)
                                        .with_temp(origin_message.temp),
                                ),
                            })
                            .expect("[Standard] Failed to send response");
                    }
                } else {
                    warn!("No respond_to channel to send to");
                }
            }
            Message::JSON { message, origin } => {
                if let Some(origin_message) = origin {
                    if let Some(mpsc_respond_to) = origin_message.mpsc_respond_to {
                        tokio::spawn(async move {
                            debug!("Sending MPSC response");
                            mpsc_respond_to
                                .send(Message::JSON {
                                    message: message.to_owned(),
                                    origin: None,
                                })
                                .await
                                .expect("[JSON] Failed to send response");
                        });
                    } else {
                        let respond_to = origin_message.respond_to.expect("No respond_to channel");

                        let result = respond_to.send(Message::JSON {
                            message: message.to_owned(),
                            origin: None,
                        });
                        match result {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Failed to send: {}", e);
                            }
                        }
                    }
                } else {
                    warn!("No origin provided - so no channel to send to {}", message);
                }
            }

            Message::Tuple {
                message_1,
                message_2,
                origin,
                ..
            } => {
                if let Some(origin_message) = origin {
                    if let Some(mpsc_respond_to) = origin_message.mpsc_respond_to {
                        tokio::spawn(async move {
                            debug!("Sending MPSC response");
                            mpsc_respond_to
                                .send(Message::Tuple {
                                    message_1,
                                    message_2,
                                    origin: None,
                                })
                                .await
                                .expect("[JSON] Failed to send response");
                        });
                    } else {
                        let respond_to = origin_message.respond_to.expect("No respond_to channel");

                        let result = respond_to.send(Message::Tuple {
                            message_1,
                            message_2,
                            origin: None,
                        });
                        match result {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Failed to send: {}", e);
                            }
                        }
                    }
                } else {
                    warn!("No origin provided - so no channel to send to for Tuple");
                }
            }

            Message::NoOp {} => {
                debug!("NOOP - NoOp message");
            }

            Message::Error { error, origin } => {
                if let Some(origin_message) = origin {
                    if let Some(mpsc_respond_to) = origin_message.mpsc_respond_to {
                        tokio::spawn(async move {
                            debug!("Sending MPSC response");
                            mpsc_respond_to
                                .send(Message::Error {
                                    error: error.to_owned(),
                                    origin: None,
                                })
                                .await
                                .expect("[JSON] Failed to send response");
                        });
                    } else {
                        let respond_to = origin_message.respond_to.expect("No respond_to channel");

                        respond_to
                            .send(Message::Error {
                                error: error.to_owned(),
                                origin: None,
                            })
                            .expect("[Standard] Failed to send response");
                    }
                } else {
                    warn!("No respond_to channel to send to");
                }
            }
            Message::Exit { .. } => {
                warn!("Exit - do nothing");
            }
            _ => {
                warn!("Message type not supported for respond_to {}", _message);
            }
        }
    }
}
