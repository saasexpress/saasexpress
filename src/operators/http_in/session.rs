use axum::extract::ws::WebSocket;

use axum::{
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use fastrace::Span;
//use futures::channel::mpsc::TryRecvError;
use futures::stream::SplitStream;
use std::result;
use std::{io::Read, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

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

use super::resources::MySharedState;

use saasexpress_core::graph::message::{DebuggableSpan, Message as GraphMessage, OriginMessage};

/**
 *
 * Session will establish a two-way communication flow where Operator nodes
 * are able to send mutliple messages back and forth.
 *
 */

pub struct SocketSession {
    socket: WebSocket,
    state: State<Arc<MySharedState>>,
    who: SocketAddr,
    span: fastrace::Span,
}

impl SocketSession {
    pub fn new(
        socket: WebSocket,
        state: State<Arc<MySharedState>>,
        who: SocketAddr,
        span: fastrace::Span,
    ) -> Self {
        SocketSession {
            socket,
            state,
            who,
            span,
        }
    }

    pub async fn process(self) {
        let state = self.state.clone();
        let who = self.who;
        let span = self.span;
        let inbound_span = Span::enter_with_parent("inbound_span", &span);

        let (sender, receiver) = self.socket.split();
        // send message to the first operator of the flow

        // anything received from upstream, forward to websocket client
        let (_tx, mut _rx) = mpsc::channel(5);

        // This task will receive messages back from graph and send them to the client
        let mut send_task = tokio::spawn(async move { upstream_to_client(_rx, sender).await });

        // let mut send_task = tokio::spawn(async move {
        //     while let Some(message) = _rx.recv().await {
        //         println!("Received: {}", message);
        //         // Process message immediately
        //     }
        //     0
        // });

        // This task will receive messages from client and start the graph flow
        let mut recv_task = tokio::spawn(async move {
            let state = self.state.clone();

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

            client_to_upstream(receiver, state, who, _tx, &inbound_span).await
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            rv_a = (&mut send_task) => {
                match rv_a {
                    Ok(a) => debug!("{a} messages sent to {who}"),
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
                .session(who.to_string())
                .with_span(Some(DebuggableSpan(span))),
        );

        warn!("Sending exit message to graph");
        state.start.send(GraphMessage::Exit { origin });

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
                        if sender
                            .send(Message::Text(Utf8Bytes::from(returned)))
                            .await
                            .is_err()
                        {
                            error!("Error sending message");
                            return n_msg;
                        }
                    }
                    GraphMessage::JSON { message, .. } => {
                        let returned = serde_json::to_string(&message).unwrap();
                        info!("-> Back to client: {:?}", returned);
                        // send to the websocket
                        if sender
                            .send(Message::Text(Utf8Bytes::from(returned)))
                            .await
                            .is_err()
                        {
                            error!("Error sending message");
                            return n_msg;
                        }
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
    state: State<Arc<MySharedState>>,
    who: SocketAddr,
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

                state.start.send(GraphMessage::Standard {
                    message: t.as_bytes().to_vec(),
                    origin,
                });

                // let r = resp_rx1.await;
                // if r.is_err() {
                //     error!("Error receiving message");
                //     return cnt;
                // } else {
                //     let msg = r.unwrap();
                //     _sender.send(msg).await.unwrap();
                // }
            }
            Message::Close { .. } => {
                info!("Client closed connection");
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
