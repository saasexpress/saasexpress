use std::{
    collections::HashSet,
    ops::Deref,
    sync::{Arc, Mutex},
};

use async_nats::jetstream::response;
use tokio::sync::mpsc::{self, Sender};
use tracing::{debug, error, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType, Origin};

use crate::graph::graph::{Message, Operator};

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

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        panic!("NOOP - Not implemented");
        // debug!("NOOP - Do nothing");
        // //let msg = _message.to_owned();
        // let msg = _message;
        // match msg {
        //     Message::Standard {
        //         message,
        //         respond_to,
        //     } => {
        //         if respond_to.is_none() {
        //             debug!("NOOP - No origin");
        //         } else {
        //             debug!("NOOP - Origin");
        //             //let origin = origin.unwrap();

        //             respond_to
        //                 .unwrap()
        //                 .send(Message::Standard {
        //                     message,
        //                     respond_to: None,
        //                 })
        //                 .unwrap();
        //         }
        //     }
        //     _ => {
        //         panic!("NOOP - unexpected message type");
        //     }
        // }
        // return Message::Standard {
        //     message: "NOOP".as_bytes().to_vec(),
        //     respond_to: None,
        // };
    }
    fn init(&mut self, _: &mut Graph) {
        warn!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("NOOP - no control to do - Not implemented");
    }

    fn send(&self, _message: Message) {
        debug!("Finished.. sending to respond_to..");
        match _message {
            Message::Standard { message, origin } => {
                if let Some(origin_message) = origin {
                    debug!("Sending response to respond_to channel");

                    let respond_to = origin_message.respond_to;

                    //let to = Arc::into_inner(respond_to);
                    let r_to = respond_to;
                    r_to.send(Message::Standard {
                        message: message.to_owned(),
                        origin: None,
                    })
                    .expect("[Standard] Failed to send response");

                    // respond_to
                    //     .send(Message::Standard {
                    //         message: message.to_owned(),
                    //         origin: None,
                    //     })
                    //     .expect("Failed to send response");
                } else {
                    warn!("No respond_to channel to send to");
                }
            }
            Message::JSON { message, origin } => {
                if let Some(origin_message) = origin {
                    debug!("[JSON] Sending response to respond_to channel");

                    let respond_to = origin_message.respond_to;

                    let r_to = respond_to;
                    //let r_to = Option::expect(to, "Failed to unwrap respond_to channel");
                    r_to.send(Message::JSON {
                        message: message.to_owned(),
                        origin: None,
                    })
                    .expect("[JSON] Failed to send response");
                } else {
                    warn!("No respond_to channel to send to");
                }
            }

            _ => {
                warn!("Message type not supported for respond_to");
            }
        }
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }
    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }
}
