use futures::StreamExt;
use pubsub::publish_to_durable_topic;
use pubsub::subscribe_to_durable_topic;
use std::error::Error;

mod pubsub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to NATS server
    let client = async_nats::connect("nats://localhost:4222").await?;

    println!("Connected to NATS server");

    // Example 1: Subscribe to a durable topic (using JetStream)
    subscribe_to_durable_topic(1, client.clone()).await?;
    subscribe_to_durable_topic(2, client.clone()).await?;

    // Example 2: Publish to a durable topic (using JetStream)
    publish_to_durable_topic(client).await?;

    // Wait to see messages
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}
