use fastrace::Span;
use fastrace::local::LocalSpan;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use futures::channel::oneshot;
use futures::future::join_all;
use tracing::{debug, error, info, span, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType, Origin};

use crate::graph::message::OriginMessage;
use crate::graph::message::{DebuggableSpan, Message};

use crate::graph::graph::Operator;
use crate::graph::meta::NodeMeta;
use fastrace::future::FutureExt;

#[derive(Debug)]
pub(crate) struct FanOut {
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
    senders: Vec<mpsc::Sender<Message>>,
}

impl From<serde_yaml::Value> for FanOut {
    fn from(_value: serde_yaml::Value) -> Self {
        FanOut {
            next: Vec::new(),
            senders: Vec::new(),
        }
    }
}

impl Operator for FanOut {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "FanOut".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
        //        panic!("Not implemented");
        // return _message;
        // // send message to "processor"
        // // processor will create a new message for each next operator
        // // and send it to them and wait for response
        // // once it gets all the responses, it will send final message back to the original sender
        // //
        // match _message {
        //     Message::Standard { message, origin } => Message::Standard {
        //         message: message.to_owned(),
        //         origin,
        //     },
        //     _ => panic!("Unexpected message type in FanOut::handle {}", _message),
        // }
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("FanOut::init - nothing to initialize");
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
        self.next(message);
    }

    fn wait(&self) -> Message {
        todo!("FanOut::wait is not implemented yet");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        todo!("FanOut::get_output_channels is not implemented yet");
    }
}

impl FanOut {
    fn next(&self, mut _message: Message) {
        debug!("FanOut::next {:?}", _message);

        let senders = self.senders.clone();

        let parent_span = _message.get_span().expect("Failed to get span");

        let fanout_span = Span::enter_with_parent("fanout", parent_span);
        let fanout2_span = Span::enter_with_parent("fanout2", parent_span);

        //let _guard = fanout_span.set_local_parent();

        // let _origin = _message
        //     .get_origin()
        //     .expect("Failed to get origin from message");

        error!("FanOut::next - message: {:?}", _message);
        //let origin = _message.take_origin().unwrap();

        // let respond_to;
        // {
        //     respond_to = _message.as_ref().unwrap().respond_to;
        // }

        // this will remove origin from the message!

        fn all_data(
            fanout_span: Span,
            msg: Message,
        ) -> (
            serde_json::Value,
            Option<oneshot::Sender<Message>>,
            Option<DebuggableSpan>,
            Arc<Mutex<serde_json::Value>>,
        ) {
            let info = match msg {
                // Message::ReqReply {
                //     message,
                //     respond_to,
                //     span,
                //     ..
                // } => {
                //     let s: String = message
                //         .iter()
                //         .map(|b| *b as char)
                //         .collect::<String>()
                //         .to_string();
                //     (serde_json::from_str(&s).unwrap(), Some(respond_to), span)
                // }
                Message::HTTP {
                    message,
                    status,
                    headers,
                    origin,
                } => {
                    // let og = match origin {
                    //     Some(o) => o,
                    //     None => OriginMessage::new(oneshot::channel().0),
                    // };
                    let og = origin.unwrap();
                    if status > 299 {
                        error!("HTTP error: {}", status);
                        let respond_to = og.respond_to.expect("No respond_to");

                        respond_to
                            .send(Message::HTTP {
                                message,
                                status,
                                headers,
                                origin: Some(
                                    OriginMessage::new(None)
                                        .with_span(Some(DebuggableSpan(fanout_span))),
                                ),
                            })
                            .unwrap();
                        let json = serde_json::from_str("{}".to_string().as_str()).unwrap();
                        (json, None, og.span, og.temp)
                    } else if status == 204 {
                        let respond_to = og.respond_to.expect("No respond_to");
                        let json = serde_json::from_str("{}".to_string().as_str()).unwrap();
                        (json, Some(respond_to), og.span, og.temp)
                    } else {
                        let respond_to = og.respond_to.expect("No respond_to");
                        let s: String = message
                            .iter()
                            .map(|b| *b as char)
                            .collect::<String>()
                            .to_string();
                        let json = serde_json::from_str(&s).unwrap();
                        (json, Some(respond_to), og.span, og.temp)
                    }
                }
                Message::JSON {
                    message, origin, ..
                } => {
                    let og = origin.unwrap();
                    let respond_to = og.respond_to.expect("No respond_to");
                    (message, Some(respond_to), og.span, og.temp)
                }
                _ => panic!("Unexpected message type in FanOut::next {}", msg),
            };
            info
        }
        let all_data = all_data(fanout_span, _message);

        let data = all_data;

        error!("FanOut::next - data: {:?}", data.3);
        //let respond_to = data.1;
        //let span = data.2;
        //let _origin = all_data.1;

        //let _origin = _message.take_origin();
        //let _origin = OriginMessage::new(respond_to).with_span(span);

        let future = async move {
            //let respond_to = origin.respond_to;
            let to = data.1;
            //let message = data.0.to_owned();

            let temp = data.3.clone();

            let origin = OriginMessage::new(None)
                .with_span(data.2)
                .with_temp(Arc::clone(&temp));

            let mut response_receivers = Vec::new();

            let span = Span::enter_with_local_parent("fanout_send");
            //let _guard = span.set_local_parent();

            let mut index = 0;
            for _sender in &senders {
                index += 1;
                debug!("Sending message to sender {}", index);

                let temp = temp.clone();

                let sender = _sender.clone();
                let (resp_tx1, resp_rx1) = oneshot::channel::<Message>();

                response_receivers.push(resp_rx1);

                let s = sender.to_owned();

                //let fan_span = Span::enter_with_local_parent("fanout");
                let fan_span = Span::enter_with_parent(format!("fanout:{}", index), &span);

                let result = async {
                    let fan_inner_span =
                        Span::enter_with_local_parent(format!("fanout2:{}", index));

                    let result = s
                        .send(Message::JSON {
                            message: data.0.to_owned(),
                            origin: Some(
                                OriginMessage::new(Some(resp_tx1))
                                    .with_span(Some(DebuggableSpan(fan_inner_span)))
                                    .with_temp(temp),
                            ),
                        })
                        .await;

                    return result;
                }
                .in_span(fan_span)
                .await;

                debug!("Message sent to sender {}", index);
                if let Err(e) = result {
                    error!("Failed to send message: {}", e);
                }
            }

            let results = join_all(response_receivers)
                .await
                .into_iter()
                .filter_map(|res| match res {
                    Ok(response) => {
                        debug!("Received response {:?}", response);
                        Some(response)
                    }
                    Err(e) => {
                        // this can be acceptable if one of the fanouts uses the "Terminate" operator
                        // where it returns a NoOp message (which causes the respond_to to drop)
                        warn!("Failed to receive response: {}", e);
                        None
                    }
                })
                .collect::<Vec<Message>>();

            let mut merged = Vec::new();
            for r in &results {
                match r {
                    Message::ReqReply { message, .. } => {
                        let value = json!({ "text": String::from_utf8_lossy(message) });

                        merged.push(value);
                    }
                    Message::Standard { message, .. } => {
                        let value = json!({ "text": String::from_utf8_lossy(message) });

                        merged.push(value);
                    }
                    Message::JSON { message, .. } => {
                        merged.push(message.to_owned());
                    }
                    Message::NoOp {} => {
                        warn!("NoOp - do not include in merged results")
                    }
                    _ => {
                        error!("Unexpected message type in FanOut::next {}", r);
                    }
                }
            }

            debug!("Merged results: {:?}", merged);
            if senders.len() != merged.len() {
                error!(
                    "FanOut: not all responses received. Expected {}, got {}",
                    senders.len(),
                    merged.len()
                );
            }

            error!("Sending with origin {:?}", origin);
            if let Some(to) = to {
                if merged.len() == 1 {
                    let value = merged.pop().unwrap();
                    to.send(Message::JSON {
                        message: value,
                        origin: Some(origin),
                    })
                    .unwrap();
                } else {
                    to.send(Message::JSON {
                        message: serde_json::Value::Array(merged),
                        origin: Some(origin),
                    })
                    .unwrap();
                }
            }
            /*
            match message {
                Message::ReqReply {
                    message,
                    respond_to,
                    ..
                } => {
                    let to = respond_to;

                    to.send(Message::Standard {
                        message: print.join(", ").as_bytes().to_vec(),
                        origin: None,
                    })
                    .unwrap();
                }
                Message::JSON {
                    message, origin, ..
                } => {
                    let to = origin.unwrap().respond_to;

                    to.send(Message::Standard {
                        message: print.join(", ").as_bytes().to_vec(),
                        origin: None,
                    })
                    .unwrap();
                }
                _ => panic!("Unexpected message type in FanOut::send {}", message),
            }
            */
        }
        .in_span(fanout2_span);
        tokio::spawn(future);
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        //self.next.push(operator);

        let (tx1, mut rx1) = mpsc::channel::<Message>(1);

        self.senders.push(tx1);

        tokio::spawn(async move {
            while let Some(msg) = rx1.recv().await {
                //let og_span = msg.get_span().unwrap();

                //let parent_span = Some(DebuggableSpan(msg.get_span().unwrap()));
                // Process the message (this would be your worker logic)
                match msg {
                    Message::ReqReply {
                        message,
                        respond_to,
                        span,
                        temp,
                        ..
                    } => {
                        let r_to = respond_to;

                        operator.lock().unwrap().send(Message::Standard {
                            message,
                            origin: Some(
                                OriginMessage::new(Some(r_to))
                                    .with_span(span)
                                    .with_temp(temp),
                            ),
                        });
                    }
                    Message::JSON {
                        message, origin, ..
                    } => {
                        // let og = origin.unwrap();
                        // let r_to = og.respond_to.expect("No respond_to");
                        // let span = og.span.unwrap();

                        operator.lock().unwrap().send(Message::JSON {
                            message,
                            origin, // origin: Some(OriginMessage::new(Some(r_to)).with_span(Some(span))),
                        });
                    }
                    _ => {
                        error!("Unexpected message type in FanOut::add_next {}", msg);
                    }
                }
            }
        });
    }

    // async fn handle_responses(&self) {
    //     // Wait for all responses

    //     let mut ls = Vec::new();
    //     for rcv in self.response_receivers {
    //         ls.push(rcv)
    //     }
    //     //let mut response_receivers = &self.response_receivers;
    //     let results = join_all(ls)
    //         .await
    //         .into_iter()
    //         .filter_map(|res| match res {
    //             Ok(response) => Some(response),
    //             Err(e) => {
    //                 error!("Failed to receive response: {}", e);
    //                 None
    //             }
    //         })
    //         .collect::<Vec<Message>>();
    // }

    fn setup_routes(&self, _start: Arc<Mutex<dyn Operator + 'static>>) {}
}
// /// Distributes a message to three receivers, collects all their responses,
// /// and sends the combined result back through the original oneshot sender.
// ///
// /// # Arguments
// /// * `original_sender` - The oneshot sender to send the final result back
// /// * `message` - The message to distribute to all receivers
// ///
// /// # Type Parameters
// /// * `T` - The type of message sent to receivers, must be Clone + Send + 'static
// /// * `R` - The type of response expected from receivers, must be Send + 'static
// /// * `F` - The type of final response sent back, must be Send + 'static
// async fn distribute_and_collect<T, R, F>(
//     original_sender: oneshot::Sender<F>,
//     message: T,
//     combine_responses: impl FnOnce(Vec<R>) -> F + Send + 'static,
// ) where
//     T: Clone + std::fmt::Debug + Send + 'static,
//     R: Send + 'static + From<String>,
//     F: Send + 'static,
// {
//     // Create three mpsc channels
//     let (tx1, mut rx1) = mpsc::channel::<T>(1);
//     let (tx2, mut rx2) = mpsc::channel::<T>(1);
//     let (tx3, mut rx3) = mpsc::channel::<T>(1);

//     // Create response channels for each receiver
//     let (resp_tx1, resp_rx1) = oneshot::channel::<R>();
//     let (resp_tx2, resp_rx2) = oneshot::channel::<R>();
//     let (resp_tx3, resp_rx3) = oneshot::channel::<R>();

//     // Spawn tasks for each receiver
//     let senders = vec![tx1, tx2, tx3];
//     let mut response_receivers = vec![resp_rx1, resp_rx2, resp_rx3];

//     // Spawn three separate tasks to represent the receivers processing the message
//     tokio::spawn(async move {
//         if let Some(msg) = rx1.recv().await {
//             // Process the message (this would be your worker logic)
//             let response = process_message(&msg, "worker1").await;
//             let _ = resp_tx1.send(response.into()); // Send response back
//         }
//     });

//     tokio::spawn(async move {
//         if let Some(msg) = rx2.recv().await {
//             let response = process_message(&msg, "worker2").await;
//             let _ = resp_tx2.send(response.into());
//         }
//     });

//     tokio::spawn(async move {
//         if let Some(msg) = rx3.recv().await {
//             let response = process_message(&msg, "worker3").await;
//             let _ = resp_tx3.send(response.into());
//         }
//     });

//     // Send the message to all receivers
//     for tx in senders {
//         let _ = tx.send(message.clone()).await;
//     }

//     // Wait for all responses
//     let results = join_all(response_receivers)
//         .await
//         .into_iter()
//         .filter_map(|res| match res {
//             Ok(response) => Some(response),
//             Err(e) => {
//                 error!("Failed to receive response: {}", e);
//                 None
//             }
//         })
//         .collect::<Vec<R>>();

//     // Combine the results and send back through the original sender
//     let final_result = combine_responses(results);
//     if let Err(e) = original_sender.send(final_result) {
//         error!("Failed to send final result");
//     }
// }

// // Example processing function
// async fn process_message<T>(msg: &T, worker_id: &str) -> String
// where
//     T: std::fmt::Debug + Send + 'static,
// {
//     // Simulate some processing time
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//     format!("{} processed: {:?}", worker_id, msg)
// }

// // Usage example
// async fn example_usage() {
//     // Create the original oneshot channel
//     let (original_tx, original_rx) = oneshot::channel::<Vec<String>>();

//     // The message to distribute
//     let message = "Hello, workers!";

//     // Start the distribution
//     if let Err(e) = distribute_and_collect(
//         original_tx,
//         message,
//         |responses| responses, // Identity function to just collect all responses
//     )
//     .await
//     {
//         eprintln!("Error during distribution: {}", e);
//     }

//     // Wait for the final result
//     match original_rx.await {
//         Ok(result) => {
//             println!("Received results: {:?}", result);
//         }
//         Err(e) => {
//             eprintln!("Error receiving results: {}", e);
//         }
//     }
// }

// // Main function for testing
// #[tokio::main]
// async fn main() {
//     example_usage().await;
// }
