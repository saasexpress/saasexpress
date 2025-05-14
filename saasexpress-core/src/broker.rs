use std::sync::{Mutex, OnceLock};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::mpsc;
use tracing::{error, info, subscriber};

pub struct SubscribeMessage {
    topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct Broker {
    subscribers: HashMap<String, Vec<mpsc::Sender<SubscribeMessage>>>,
}

static INSTANCE: OnceLock<Mutex<Broker>> = OnceLock::new();

impl Broker {
    pub fn get_instance() -> &'static Mutex<Broker> {
        INSTANCE.get_or_init(|| Mutex::new(Broker::new()))
    }

    fn new() -> Self {
        Broker {
            subscribers: HashMap::new(),
        }
    }

    pub async fn publish(&mut self, topic: String, payload: Vec<u8>) {
        if let Some(subscribers) = self.subscribers.get_mut(&topic) {
            for subscriber in subscribers {
                info!("Publishing to topic: {}", topic);
                let result = subscriber
                    .send(SubscribeMessage {
                        topic: topic.clone(),
                        payload: payload.clone(),
                    })
                    .await;
                match result {
                    Ok(_) => {
                        info!("Message sent to subscriber");
                    }
                    Err(_) => {
                        error!("Failed to send message to subscriber");
                    }
                }
            }
        }
    }

    pub fn subscribe(&mut self, topic: String) -> mpsc::Receiver<SubscribeMessage> {
        let (tx, rx) = mpsc::channel(10);
        self.subscribers
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(tx);
        rx
    }
}
