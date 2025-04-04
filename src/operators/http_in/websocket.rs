//! Example websocket server.
//!
//! Run the server with
//! ```not_rust
//! cargo run -p example-websockets --bin example-websockets
//! ```
//!
//! Run a browser client with
//! ```not_rust
//! firefox http://localhost:3000
//! ```
//!
//! Alternatively you can run the rust client (showing two
//! concurrent websocket connections being established) with
//! ```not_rust
//! cargo run -p example-websockets --bin example-client
//! ```

use axum::{
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use std::{
    net::SocketAddr,
    sync::Arc,
};

use axum_extra::{TypedHeader, headers};
use tracing::{debug, error};

use std::ops::ControlFlow;


//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};


use super::resources::MySharedState;

use crate::graph::graph::Message as GraphMessage;

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    state: State<Arc<MySharedState>>,
    //    state: Arc<Mutex<dyn Operator>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    debug!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional debug such as address.

    ws.on_upgrade(move |socket| handle_socket(state, socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(state: State<Arc<MySharedState>>, socket: WebSocket, who: SocketAddr) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();
    // send message to the first operator of the flow

    state.start.lock().unwrap().send(GraphMessage::Standard {
        message: format!("WebSocket connection from {who}").into_bytes(),
        origin: None,
    });

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 5;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if sender
                .send(Message::Text(format!("Server message {i} ...").into()))
                .await
                .is_err()
            {
                error!("Error sending message to {who}");
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        debug!("Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye from SaaS Express Gateway"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
        n_msg
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
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

    // returning from the handler closes the websocket connection
    debug!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            debug!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            debug!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
