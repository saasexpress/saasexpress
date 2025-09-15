use async_nats::jetstream::message;
use fastrace::Span;
use futures::channel::oneshot::Receiver;
//use futures::channel::mpsc::TryRecvError;
use futures::stream::SplitStream;
use reqwest_websocket::{CloseCode, Message, WebSocket};
use std::result;
use std::{io::Read, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::{SendError, TryRecvError};

use axum_extra::{TypedHeader, headers};
use tracing::{debug, error, info, warn};

use std::ops::ControlFlow;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{
    channel::oneshot,
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};

use saasexpress_core::graph::message::{DebuggableSpan, Message as GraphMessage, OriginMessage};

use std::fmt::Debug;

/**
 *
 * Session will establish a two-way communication flow where Operator nodes
 * are able to send mutliple messages back and forth.
 *
 */

#[derive(Debug)]
pub struct Closer {
    pub tx: mpsc::Sender<GraphMessage>,
}
pub trait CloserAction: Send + Sync + Debug {
    fn send(&self, message: GraphMessage);
}

impl CloserAction for Closer {
    fn send(&self, message: GraphMessage) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let result = tx.send(message).await;
            if result.is_err() {
                error!("Error sending close message");
            }
        });
    }
}

pub struct SocketSession {
    //socket: WebSocket,
    graph_sender: mpsc::Sender<GraphMessage>,
    who: String,
    span: fastrace::Span,
    //pub close: oneshot::Sender<()>,
    pub sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
    pub rx: tokio::sync::mpsc::Receiver<GraphMessage>,
}

impl SocketSession {
    pub fn new(
        socket: WebSocket,
        graph_sender: mpsc::Sender<GraphMessage>,
        who: String,
        span: fastrace::Span,
        rx: mpsc::Receiver<GraphMessage>,
    ) -> Self {
        //let s = socket.by_ref();

        let (sender, receiver) = socket.split();

        SocketSession {
            sender,
            receiver,
            graph_sender,
            who,
            span,
            rx,
        }
    }

    pub async fn send_close(&mut self) {
        self.sender
            .send(Message::Close {
                code: CloseCode::Normal,
                reason: "Bye!".to_string(),
            })
            .await
            .unwrap();
    }

    pub async fn close(socket: WebSocket, rx: Receiver<()>) {
        let _ = rx.await;
        info!("Socket session closed");
        let res = socket
            .close(CloseCode::Normal, Some("Session closed"))
            .await;
        if res.is_err() {
            error!("Error closing sender: {:?}", res);
        }
    }

    pub async fn process(self, message: Vec<u8>) {
        let graph_sender = self.graph_sender.clone();
        //let state = self.state.clone();
        let mut rx = self.rx;
        let who = self.who.clone();
        let sender = self.sender;
        let receiver = self.receiver;

        let span = self.span;
        let inbound_span = Span::enter_with_parent("inbound_span", &span);
        let first_span = Span::enter_with_parent("first_span", &span);

        // send message to the first operator of the flow
        // let rx = self.rx;

        // tokio::spawn(async move {
        //     let _ = rx.await;
        //     self.send_close().await;
        //     // info!("Socket session closed");
        //     // let res = sender.close().await;
        //     // if res.is_err() {
        //     //     error!("Error closing sender: {:?}", res);
        //     // }
        // });

        // anything received from upstream, forward to websocket client
        let (_tx, mut _rx) = mpsc::channel::<GraphMessage>(5);

        let exit_tx = _tx.clone();

        tokio::spawn(async move {
            loop {
                let message = rx.recv().await;
                match message {
                    Some(msg) => {
                        //info!("Received message.. {:?}", msg);
                        let result = exit_tx.send(msg).await;
                        if result.is_err() {
                            error!("Error sending exit message to graph");
                        }
                    }
                    None => {
                        info!("Channel closed");
                        break;
                    }
                }
            }
        });

        // This task will receive messages back from graph and send them to the client
        let mut send_task = tokio::spawn(async move { upstream_to_client(_rx, sender).await });

        // let mut send_task = tokio::spawn(async move {
        //     while let Some(message) = _rx.recv().await {
        //         println!("Received: {}", message);
        //         // Process message immediately
        //     }
        //     0
        // });

        // make sure the first message is sent to the graph
        let _origin = OriginMessage::new(None)
            .session(who.to_string())
            .with_span(Some(DebuggableSpan(first_span)))
            .mpsc_respond_to(_tx.clone());
        let origin = Some(_origin);

        _tx.send(GraphMessage::Standard { message, origin })
            .await
            .unwrap();

        // This task will receive messages from client and start the graph flow
        let mut recv_task = tokio::spawn(async move {
            let graph_sender = self.graph_sender.clone();

            //let (resp_tx1, resp_rx1) = oneshot::channel::<GraphMessage>();

            // let _origin = OriginMessage::new(resp_tx1)
            //     .session(who.to_string())
            //     .mpsc_respond_to(_tx);

            // let origin = Some(_origin);

            //start_it(resp_rx1).await;

            // state.start.lock().unwrap().send(GraphMessage::Standard {
            //     message: "ls\n".as_bytes().to_vec(),
            //     origin,
            // });

            client_to_upstream(receiver, graph_sender, &who, _tx, &inbound_span).await
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            rv_a = (&mut send_task) => {
                match rv_a {
                    Ok(a) => debug!("{a} messages sent to who"),
                    Err(a) => debug!("Error sending messages {a:?}")
                }
                recv_task.abort();
            },
            rv_b = (&mut recv_task) => {
                match rv_b {
                    Ok(b) => debug!("Received {b} messages"),
                    Err(b) => debug!("Error receiving messages {b:?}")
                }
                send_task.abort();
            }
        }

        let origin = Some(
            OriginMessage::new(None)
                //.session(who)
                .with_span(Some(DebuggableSpan(span))),
        );

        warn!("Sending exit message to graph");
        let result = graph_sender.send(GraphMessage::Exit { origin }).await;
        if result.is_err() {
            error!("Error sending exit message to graph",);
        }

        ()
    }
}

async fn start_it(resp_rx1: oneshot::Receiver<GraphMessage>) {
    tokio::spawn(async move {
        let result = resp_rx1.await;
        match result {
            Ok(msg) => {
                debug!("Received message from graph: {:?}", msg);
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
            }
        }
    });

    // sleep
    //tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
}

async fn upstream_to_client(
    mut rcv: mpsc::Receiver<GraphMessage>,
    mut sender: SplitSink<WebSocket, Message>,
) -> usize {
    let mut n_msg = 0;

    loop {
        let _msg = rcv.recv().await;
        match _msg {
            Some(msg) => {
                n_msg = n_msg + 1;
                match msg {
                    GraphMessage::Standard { message, .. } => {
                        let returned = String::from_utf8(message).unwrap();
                        info!("-> Back to client: {:?}", returned);
                        // send to the websocket
                        if sender.send(Message::Text(returned)).await.is_err() {
                            error!("Error sending message");
                            return n_msg;
                        }
                    }
                    GraphMessage::JSON { message, .. } => {
                        let returned = serde_json::to_string(&message).unwrap();
                        info!("-> Back to client: {:?}", returned);
                        // send to the websocket
                        if sender.send(Message::Text(returned)).await.is_err() {
                            error!("Error sending message");
                            return n_msg;
                        }
                    }
                    GraphMessage::Exit { .. } => {
                        info!("Exit message received");
                        // close the websocket
                        match sender.close().await {
                            Ok(_) => {
                                info!("Sender closed");
                            }
                            Err(e) => {
                                error!("Error closing sender: {:?}", e);
                            }
                        }
                        break;
                    }
                    _ => {
                        warn!("Unexpected message type {:?}", msg);
                    }
                }
            }
            None => {
                warn!("Channel closed");
                break;
            } // Err(TryRecvError::Empty) => {
              //     // no messages available
              //     //warn!("Empty");
              //     //break;
              // }
              // Err(TryRecvError::Disconnected) => {
              //     // channel closed
              //     warn!("Channel closed");
              //     break;
              // }
        }
    }
    // while let Some(msg) = r.next().await {
    //     n_msg = n_msg + 1;
    //     match msg {
    //         GraphMessage::Standard { message, origin } => {
    //             // send to the websocket
    //             info!("Sending message to client: {:?}", message);
    //             let returned = String::from_utf8(message).unwrap();
    //             if sender
    //                 .send(Message::Text(Utf8Bytes::from(returned)))
    //                 .await
    //                 .is_err()
    //             {
    //                 error!("Error sending message");
    //                 return n_msg;
    //             }
    //         }
    //         _ => {
    //             warn!("Unexpected message type {:?}", msg);
    //         }
    //     }
    // }
    info!("Ending with {} messages", n_msg);
    n_msg
}

async fn client_to_upstream(
    mut receiver: SplitStream<WebSocket>,
    graph_sender: mpsc::Sender<GraphMessage>,
    who: &str,
    tx: mpsc::Sender<GraphMessage>,
    span: &fastrace::Span,
) -> usize {
    let mut cnt = 0;

    while let Some(Ok(msg)) = receiver.next().await {
        cnt += 1;

        match &msg {
            Message::Text(t) => {
                //let value = serde_json::from_str::<serde_json::Value>(t).unwrap();
                debug!("[WS]: Recv {:?} bytes", t.len());
                // let data = value["data"].as_str().unwrap().to_string();
                // debug!("T = {}", data);

                // let origin = Some(
                //     OriginMessage::new(oneshot::channel::<GraphMessage>().0)
                //         .session(who.to_string()),
                // );
                let span = Span::enter_with_parent("ws-recv", span);

                let _origin = OriginMessage::new(None)
                    .session(who.to_string())
                    .with_span(Some(DebuggableSpan(span)))
                    .mpsc_respond_to(tx.clone());
                let origin = Some(_origin);

                let result = graph_sender
                    .send(GraphMessage::Standard {
                        message: t.as_bytes().to_vec(),
                        origin,
                    })
                    .await;

                if result.is_err() {
                    error!("Error sending message to graph");
                    return cnt;
                }
                // let r = resp_rx1.await;
                // if r.is_err() {
                //     error!("Error receiving message");
                //     return cnt;
                // } else {
                //     let msg = r.unwrap();
                //     _sender.send(msg).await.unwrap();
                // }
            }
            Message::Close { code, reason } => {
                info!("Client closed connection: {:?} {:?}", code, reason);
                // send to the websocket
                match graph_sender
                    .send(GraphMessage::Exit {
                        origin: Some(OriginMessage::new(None)),
                    })
                    .await
                {
                    Ok(_) => {
                        info!("Sender closed");
                    }
                    Err(e) => {
                        error!("Error closing sender: {:?}", e);
                    }
                }
                break;
            }
            _ => {
                error!("Unexpected message type {:?}", msg);
            }
        }

        // print message and break if instructed to do so
        //     if process_message(msg, who).is_break() {
        //         break;
        //     }
    }
    cnt
}
