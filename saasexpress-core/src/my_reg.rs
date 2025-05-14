use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use crate::control_bus::ControlEvent;
use crate::graph::graph::OperatorState;

// Define our Registry type
struct Registry {
    // Use Any to allow different sender types
    channels: Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            channels: Mutex::new(HashMap::new()),
        }
    }

    // Register a sender
    fn register<T: 'static + Send + Sync>(&self, name: &str, sender: mpsc::Sender<T>) {
        let mut channels = self.channels.lock().unwrap();
        channels.insert(name.to_string(), Box::new(sender));
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
        error!("Channel with name {} already exists", name);
        return;
    }
    REGISTRY.register(name, sender);
}

pub fn get<T: 'static + Send + Sync>(name: &str) -> Option<mpsc::Sender<T>> {
    REGISTRY.get(name)
}

pub fn names() -> Vec<String> {
    REGISTRY.channels.lock().unwrap().keys().cloned().collect()
}
// pub fn get_all() -> HashMap<String, Box<dyn Any + Send + Sync>> {

// pub fn broadcast<T: 'static + Send + Sync + Clone>(name: &str, message: T) {
//     for c in REGISTRY.channels.lock().unwrap().iter_mut() {
//         let sender = c.1;

//         let _ = sender.try_send(message.clone());
//     }
// }

// Example usage
pub async fn example() -> JoinHandle<()> {
    // Create a channel
    let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

    // Register it
    register("my_channel", tx.clone());

    tokio::spawn(async move {
        loop {
            // Receive the message
            if let Some(msg) = rx.recv().await {
                info!("Received: {} {:?}", msg.graph_name, msg.state);
                break;
            }
        }
    })
}

pub async fn broadcast_event(event: ControlEvent) {
    let all_channels = names();
    debug!("Broadcasting to {}", all_channels.len());
    for nm in all_channels {
        let sender = get::<ControlEvent>(&nm).unwrap();
        let result = sender.send(event.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("[broadcast] Failed to send message {:}", err),
        }
    }
}
