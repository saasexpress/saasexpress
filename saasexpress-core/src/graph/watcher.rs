use crate::{
    graph::graph::GraphStatus,
    my_reg::{ControlEvent, ControlEventType},
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use super::registry::GraphRegistry;
use crate::my_reg::register;

pub(super) fn watch_control_bus(self_graph_id: String, self_graph_name: String) {
    let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

    // Register it
    register(&self_graph_name, tx);

    tokio::spawn(async move {
        loop {
            if let Some(msg) = rx.recv().await {
                {
                    let graph = GraphRegistry::get_graph(&self_graph_name);

                    if graph.is_none() {
                        error!("[{}] {} : ErrNotFound", self_graph_id, self_graph_name);
                        continue;
                    }
                }

                match msg.event_type {
                    ControlEventType::Notice => {
                        info!(
                            "[{}] {} : NOTICE {}",
                            msg.graph_id, msg.graph_name, msg.reason
                        );
                    }
                    ControlEventType::OperatorUpdated => {
                        info!(
                            "[{}] {} : OPERATOR UPDATED - {} ({})",
                            msg.graph_id,
                            msg.graph_name,
                            msg.reason,
                            msg.operator_names.join(", ")
                        );
                    }
                    ControlEventType::GraphReplaced => {
                        info!(
                            "[{}] {} : GRAPH REPLACED - {}",
                            msg.graph_id, msg.graph_name, msg.reason
                        );
                    }
                    _ => {
                        warn!(
                            "[{}] {} : Unknown event type - {:?}",
                            msg.graph_id, msg.graph_name, msg.event_type
                        );
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
