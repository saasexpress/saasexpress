use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use futures::stream::Stream;
use futures::stream::{select_all, StreamExt};
use rand::Rng;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;

use super::operator::Message;

type Channel = (Sender<Message>, Receiver<Message>);

struct FanOut {
    sender_channels: Vec<Sender<Message>>,
    receiver_streams: Arc<Vec<ReceiverStream<Message>>>,
}

impl FanOut {
    pub fn new(count: u8) -> Self {
        let mut receiver_streams = Vec::new();
        let mut sender_channels = Vec::new();
        for _ in 1..count {
            let (tx, rx) = mpsc::channel::<Message>(100);
            sender_channels.push(tx);
            receiver_streams.push(ReceiverStream::new(rx))
        }
        FanOut {
            sender_channels,
            receiver_streams: Arc::new(receiver_streams),
        }
    }

    pub async fn produce(&self) {
        let producers = self
            .sender_channels
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Sender<Message>>>();

        for (i, tx) in producers.into_iter().enumerate() {
            task::spawn(async move {
                // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                for _ in 0..5 {
                    let delay = rand::rng().random_range(500..1500);
                    sleep(Duration::from_millis(delay)).await;

                    let msg = Message { state: 10 };
                    tx.send(msg).await.unwrap();
                }
            });
        }
    }

    pub async fn consume(&self) {
        let streams = self.receiver_streams;

        let mut merged = select_all(streams.as_ptr());

        // Spawn consumer to process incoming messages
        let consumer = task::spawn(async move {
            let mut num: u32 = 0;
            while let Some(message) = merged.next().await {
                println!("[{}] Received: {}", num, message.state);
                num = num + 1;
                if num == 10 {
                    break;
                }
            }
        });

        consumer.await.unwrap();
    }
}
