use std::pin::Pin;

use futures::stream::Stream;
use futures::stream::{select_all, StreamExt};
use rand::Rng;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;

pub async fn fanout() {
    let (tx1, rx1) = mpsc::channel::<String>(100);
    let (tx2, rx2) = mpsc::channel::<String>(100);
    let (tx3, rx3) = mpsc::channel::<String>(100);

    // Spawn multiple producers
    let producers = vec![tx1.clone(), tx2.clone(), tx3.clone()];

    for (i, tx) in producers.into_iter().enumerate() {
        task::spawn(async move {
            // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            for _ in 0..5 {
                let delay = rand::rng().random_range(500..1500);
                sleep(Duration::from_millis(delay)).await;

                let msg = format!("Message from Producer {}", i + 1);
                tx.send(msg).await.unwrap();
            }
        });
    }

    let streams = vec![
        ReceiverStream::new(rx1),
        ReceiverStream::new(rx2),
        ReceiverStream::new(rx3),
    ];
    let mut merged = select_all(streams);

    // Spawn consumer to process incoming messages
    let consumer = task::spawn(async move {
        let mut num: u32 = 0;
        while let Some(message) = merged.next().await {
            println!("[{}] Received: {}", num, message);
            num = num + 1;
            if num == 10 {
                break;
            }
        }
    });

    consumer.await.unwrap();
}
