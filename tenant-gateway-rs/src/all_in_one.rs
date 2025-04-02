use axum::body::{self, Body};
use serde::{Deserialize, Serialize};

use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use futures::future::{self, Future, FutureExt};
use futures::future::{ready, BoxFuture};
use serde_json::{json, Value};
use tracing::info;

use crate::operators::operator::MessageContext;
use crate::{dag::dag::Graph, execute_dag::execute_dag, operators::operator::Message};

#[derive(Deserialize)]
pub struct GwDagRequest {
    // Define the fields for your request payload
    field1: String,
    field2: i32,
    graph: Value,
}

#[derive(Serialize)]
struct GwDagResponse {
    // Define the fields for your response payload
    message: String,
}

struct AXUMMessage {
    body: Body,
}

#[axum::debug_handler]
pub async fn gw_dag(Json(payload): Json<GwDagRequest>) -> impl IntoResponse {
    // Process the request payload
    let response = GwDagResponse {
        message: format!(
            "Received field1: {}, field2: {}",
            payload.field1, payload.field2
        ),
    };

    let dag = Graph::new_using_value(payload.graph);

    info!("Graph successfully loaded.");

    let message = MessageContext(AXUMMessage {
        body: Body::empty(),
    });

    let msg = Message {
        state: 5,
        log: Vec::new(),
    };

    let msgctx = MessageContext(msg);

    // match dag {
    //     Ok(dag) => {
    //         if let Err(err) = execute_dag(dag).await {
    //             eprintln!("Error executing Graph: {:?}", err);
    //             return (
    //                 StatusCode::INTERNAL_SERVER_ERROR,
    //                 Json(GwDagResponse {
    //                     message: "Failed to execute Graph".to_string(),
    //                 }),
    //             );
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error creating Graph: {:?}", e);
    //         return (
    //             StatusCode::BAD_REQUEST,
    //             Json(GwDagResponse {
    //                 message: "Invalid Graph configuration".to_string(),
    //             }),
    //         );
    //     }
    // }

    // let dag = Graph::new_using_yaml("samples/test.yaml");
    // if dag.is_err() {
    //     return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    // }

    let result = execute_dag(dag.unwrap(), msgctx).await;

    match result {
        Ok(result) => (
            StatusCode::OK,
            Json(GwDagResponse {
                message: format!("Graph executed successfully: {}", result),
            }),
        ),
        Err(e) => {
            eprintln!("Error executing Graph: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GwDagResponse {
                    message: "Failed to execute Graph".to_string(),
                }),
            );
        }
    }
    // if dag.is_ok() {
    //     execute_dag(dag.unwrap_err()).await;
    // } else {
    //     panic!();
    // }
    // match dag {
    //     Ok(dag) => execute_dag(dag).await,

    //     Err(e) => Ok(()),
    // };

    //return (StatusCode::OK, Json(response));
}
