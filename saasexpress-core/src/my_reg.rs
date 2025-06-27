use once_cell::sync::Lazy;
use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, error, event, info};

use crate::graph::graph::GraphStatus;

#[derive(Clone, Serialize, Debug, PartialEq)]
pub enum ControlEventType {
    Notice,
    OperatorUpdated,
    GraphReplaced,
}

#[derive(Clone, Serialize, Debug)]
pub struct ControlEvent {
    pub graph_id: String,
    pub graph_name: String,
    pub operator_names: Vec<String>,
    pub event_type: ControlEventType,
    pub reason: String,
}

// Define our Registry type
struct Registry {
    // Use Any to allow different sender types
    channels: Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>,
    last_events: Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            channels: Mutex::new(HashMap::new()),
            last_events: Mutex::new(HashMap::new()),
        }
    }

    // Register a sender
    fn register<T: 'static + Send + Sync>(&self, name: &str, sender: mpsc::Sender<T>) {
        let mut channels = self.channels.lock().unwrap();
        channels.insert(name.to_string(), Box::new(sender));
    }

    // Deregister a sender
    fn deregister(&self, name: &str) {
        let mut channels = self.channels.lock().unwrap();
        channels.remove(name.to_string().as_str());
    }

    fn exists(&self, name: &str) -> bool {
        let channels = self.channels.lock().unwrap();
        channels.contains_key(name)
    }

    // Get a sender
    fn get<T: 'static + Send + Sync>(&self, name: &str) -> Option<mpsc::Sender<T>> {
        let channels = self.channels.lock().unwrap();
        channels
            .get(name)
            .and_then(|sender| sender.downcast_ref::<mpsc::Sender<T>>().cloned())
    }
}

// Use Lazy for thread-safe lazy initialization
static REGISTRY: Lazy<Arc<Registry>> = Lazy::new(|| Arc::new(Registry::new()));

// A cleaner API for the registry
pub fn register<T: 'static + Send + Sync>(name: &str, sender: mpsc::Sender<T>) {
    if get::<T>(name).is_some() {
        error!("Channel with name {} already exists - removing", name);
        REGISTRY.deregister(name);

        //return;
    }
    info!("[{}] REGISTERED", name);
    tokio::spawn(async move {});

    REGISTRY.register(name, sender);
}

pub fn deregister<T: 'static + Send + Sync>(name: &str) {
    if REGISTRY.exists(name) == false {
        error!("Channel with name {} does not exist", name);
        error!("Names {}", names().join(", "));
        return;
    }
    REGISTRY.deregister(name);
}

pub fn get<T: 'static + Send + Sync>(name: &str) -> Option<mpsc::Sender<T>> {
    REGISTRY.get(name)
}

// pub fn clear_registry() {
//     REGISTRY.channels.lock().unwrap().clear();
// }

// pub async fn replay_events<T: 'static + Send + Sync>(name: &str) {
//     let sender = REGISTRY.get(name).unwrap();
//     let last_events = REGISTRY.last_events.lock().unwrap();
//     for event in last_events.iter() {
//         let event = event.1.downcast_ref::<T>();
//         let result = sender.send(event.clone()).await;
//         match result {
//             Ok(_) => (),
//             Err(err) => error!("[broadcast] Failed to send message to {:}", err),
//         }
//     }
// }

pub fn save_last_event<T: 'static + Send + Sync>(name: &str, event: T) {
    let mut last_events = REGISTRY.last_events.lock().unwrap();
    last_events.insert(name.to_string(), Box::new(event));
}

pub fn names() -> Vec<String> {
    REGISTRY.channels.lock().unwrap().keys().cloned().collect()
}

pub async fn broadcast_event(event: ControlEvent) {
    let all_channels = names();
    debug!(
        "[{}] Broadcasting to {} ({:?})",
        event.graph_name,
        all_channels.len(),
        all_channels
    );
    save_last_event::<ControlEvent>(&event.graph_name, event.clone());

    for nm in all_channels {
        let sender = get::<ControlEvent>(&nm).unwrap();
        let result = sender.send(event.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("[broadcast] Failed to send message to {}: {:}", nm, err),
        }
    }
}
