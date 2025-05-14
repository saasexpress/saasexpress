use std::sync::{Arc, Mutex, OnceLock};

use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use tracing::error;

use crate::graph::graph::{GraphStatus, OperatorState};

pub struct ControlBus {
    p2p_channel: tokio::sync::mpsc::Sender<ControlEvent>,
    //pub(crate) senders: Vec<tokio::sync::mpsc::Sender<ControlEvent>>,
}

#[derive(Clone, Serialize)]
pub struct ControlEvent {
    pub graph_name: String,
    pub operator_names: Vec<String>,
    pub state: GraphStatus,
}

pub(crate) trait ControlBusTrait: Send {
    fn send_message(self, message: ControlEvent);
    fn new_subscriber(&mut self, tx: tokio::sync::mpsc::Sender<ControlEvent>);
}

impl ControlBus {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<ControlEvent>(10);
        let senders = Vec::new();

        ControlBus::run(rx, senders);

        ControlBus { p2p_channel: tx }
    }

    pub fn run(
        mut rx: Receiver<ControlEvent>,
        senders: Vec<tokio::sync::mpsc::Sender<ControlEvent>>,
    ) {
        tokio::spawn(async move {
            loop {
                let Some(message) = rx.recv().await else {
                    error!("No!");
                    break;
                };
                // for sender in senders {
                //     if let Err(er) = sender.send(message.clone()).await {
                //         // Handle the error (e.g., log it)
                //         error!("Failed to send message to subscriber {:?}", er);
                //     }
                // }
            }
        });
    }
}

impl ControlBusTrait for ControlBus {
    fn send_message(self, message: ControlEvent) {
        // tokio::spawn(async move {
        //     for sender in self.senders {
        //         if let Err(er) = sender.send(message.clone()).await {
        //             // Handle the error (e.g., log it)
        //             error!("Failed to send message to subscriber {:?}", er);
        //         }
        //     }
        // });
    }

    fn new_subscriber(&mut self, tx: tokio::sync::mpsc::Sender<ControlEvent>) {
        //self.senders.push(tx);
    }
}

static INSTANCE: OnceLock<Arc<Mutex<dyn ControlBusTrait + 'static>>> = OnceLock::new();

pub fn get_control_bus() -> Arc<Mutex<dyn ControlBusTrait + 'static>> {
    INSTANCE
        .get_or_init(|| Arc::new(Mutex::new(ControlBus::new())))
        .clone()
}
