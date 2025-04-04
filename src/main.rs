//use crate::bootstrap;
use axum::{
    Router,
    routing::{get, post},
};
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};
// use tenant_gateway::{all_in_one::gw_dag, bootstrap, operators::register, proto::test};
use tokio::net::TcpListener;
use tracing::{Level, info};

use crate::graph::graph::GraphRun;
use graph::graph::{Graph, Message};
use operators::{
    api_call::APICall, buffer_to_json::BufferToJSON, http_in::http_in::HTTPIn,
    json_to_buffer::JSONToBuffer,
};

mod graph;
mod operators;

#[tokio::main]
async fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let js = json!({"routes":["/abc"], "method":"POST"});
    let http_in = HTTPIn::from(js);
    info!("HTTPIn: {:?}", http_in);

    //bootstrap::bootstrap();

    //unit_test().await;

    loop {
        // This will loop forever
        std::thread::sleep(std::time::Duration::from_secs(3600)); // Optional sleep to reduce CPU usage
    }
}

async fn unit_test() {
    let mut graph = Graph::new("sample".to_string());
    // graph
    //     .add_node(
    //         "start",
    //         HTTPIn::new(vec!["/gw/flow".to_string()], "POST".to_string()).await,
    //     )
    //     .add_node("in", BufferToJSON)
    //     .add_node("out", JSONToBuffer)
    //     .add_node(
    //         "rp",
    //         APICall {
    //             url: "http://localhost:8081".to_string(),
    //             forward: true,
    //             ws: false,
    //         },
    //     )
    //     .add_edge("start".to_string(), "in".to_string())
    //     .add_edge("in".to_string(), "out".to_string())
    //     .add_edge("out".to_string(), "rp".to_string())
    //     .no_processor()
    //     .init();
    graph
        .add_node(
            "start",
            HTTPIn::new(vec!["/gw/flow".to_string()], "POST".to_string()).await,
        )
        .add_node("in", BufferToJSON)
        .add_node("out", JSONToBuffer)
        .add_node(
            "rp",
            APICall {
                url: "http://localhost:2243/ws".to_string(),
                path: "".to_string(),
                forward: true,
                ws: true,
                method: None,
                content_type: None,
            },
        )
        .add_edge("start".to_string(), "in".to_string())
        .add_edge("in".to_string(), "out".to_string())
        .add_edge("out".to_string(), "rp".to_string())
        .no_processor()
        .init();

    // Simulate a request
    let message = Message::Standard {
        message: "{\"a\":\"b\"}".to_string().into_bytes(),
        origin: None,
    };

    // Process the request
    let response = graph.process(message).await;

    let message = match response {
        Message::Standard {
            message,
            origin: None,
        } => {
            println!(
                "Received a Standard message: {:?}",
                String::from_utf8_lossy(&message)
            );
            message
        }
        _ => panic!("Expected a Standard response"),
    };

    // // Output response
    println!("End: {}", String::from_utf8_lossy(&message));
}
