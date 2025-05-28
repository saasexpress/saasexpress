use crate::{graph::graph::GraphStatus, my_reg::ControlEvent};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{info, warn};

use super::registry::GraphRegistry;
use crate::my_reg::register;

pub(super) fn watch_control_bus(self_graph_id: String, self_graph_name: String) {
    let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

    // Register it
    register(&self_graph_name, tx);

    tokio::spawn(async move {
        loop {
            // Receive the message
            if let Some(msg) = rx.recv().await {
                info!(
                    "[{}] Received: {:?}",
                    self_graph_id,
                    serde_json::to_string(&msg)
                );

                {
                    let graph = GraphRegistry::get_graph(&self_graph_name);

                    if graph.is_none() {
                        info!("[{}] {} : ErrNotFound", self_graph_id, self_graph_name);
                        continue;
                    }
                }

                if msg.state == GraphStatus::Starting {
                    info!("[{}] {} : Starting", msg.graph_id, msg.graph_name);
                    if msg.graph_name == self_graph_name {
                        let graph = GraphRegistry::get_graph(&self_graph_name);

                        let graph = graph.expect("Failed to get graph!");

                        let mut graph = graph.lock().unwrap();

                        graph.poke();
                    }
                } else if msg.state == GraphStatus::Running {
                    info!("[{}] {} : Running", msg.graph_id, msg.graph_name);
                } else if msg.state == GraphStatus::Error {
                    info!("[{}] {} : Error", msg.graph_id, msg.graph_name);
                } else if msg.state == GraphStatus::Paused {
                    info!("[{}] {} : Paused", msg.graph_id, msg.graph_name);
                } else if msg.state == GraphStatus::Replacing {
                    info!("[{}] {} : Replacing", msg.graph_id, msg.graph_name);
                    if msg.graph_name == self_graph_name {
                        // let graph = GraphRegistry::get_graph(&self_graph_name);

                        // let graph = graph.expect("Failed to get graph!");

                        // let mut graph = graph.lock().unwrap();

                        // graph.poke();
                    }
                }
            } else {
                warn!(
                    "WatchControlBus - Channel is closed for {}",
                    self_graph_name
                );
                break;
            }
        }
    });
}
