use futures::channel::oneshot;
use reqwest_websocket::Message;
use saasexpress_core::graph::{graph::Graph, message::Message as GraphMessage};
use tracing::error;

use super::api_call::SessionRegistry;

// pub(crate) async fn socket_close(session: &str) {
//     let socket_registry = {
//         let socket_registry = SessionRegistry::get_instance();
//         let socket_registry = socket_registry.lock().unwrap();
//         socket_registry
//     };
//     let session = socket_registry.get_session(&session);
//     let session = session.lock().unwrap();
//     session.send(GraphMessage::Exit { origin: None });
// }
