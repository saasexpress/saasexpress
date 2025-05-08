use super::resources::MySharedState;
use axum::extract::State;
use saasexpress_core::graph::message::Message;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
pub use tokio_stream::{Stream, wrappers::UnboundedReceiverStream as ReceiverStream};
use tracing::{error, info};

pub(super) fn sse_start(
    state: State<Arc<MySharedState>>,
    mut message: Message,
) -> impl Stream<Item = Value> {
    let (graph_tx, mut graph_rx) = mpsc::channel::<Message>(1);

    let (tx, rx) = mpsc::unbounded_channel::<Value>();

    // Set the mpsc_respond_to to the graph_tx channel
    let updated_origin = message.take_origin().unwrap().mpsc_respond_to(graph_tx);
    message = message.with_origin(Some(updated_origin));

    // Send to the next node in the graph
    state.start.lock().unwrap().send(message);

    tokio::spawn(async move {
        loop {
            let result = graph_rx.recv().await;

            match result {
                Some(message) => {
                    if let Message::JSON { message, .. } = message {
                        if tx.send(message).is_err() {
                            error!("Failed to send SSE event");
                            break;
                        }
                    } else {
                        error!("Unexpected message type for SSE: {:?}", message);
                        break;
                    }
                }
                None => {
                    info!("No message received - canceled");
                    break;
                }
            }
        }
    });

    ReceiverStream::new(rx)
}
