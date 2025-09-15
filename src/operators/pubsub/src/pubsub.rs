use async_nats::{Client, Message};
use futures::StreamExt;
use std::error::Error;

pub async fn subscribe_to_durable_topic(id: u8, client: Client) -> Result<(), Box<dyn Error>> {
    // Get a JetStream context
    let jetstream = async_nats::jetstream::new(client);

    // Get or create the stream
    let stream_name = "ORDERS";
    let stream = match jetstream.get_stream(stream_name).await {
        Ok(stream) => stream,
        Err(_) => {
            // Create stream if it doesn't exist
            let stream_config = async_nats::jetstream::stream::Config {
                name: stream_name.to_string(),
                subjects: vec!["orders.>".to_string()],
                ..Default::default()
            };

            jetstream.create_stream(stream_config).await?
        }
    };

    println!("Stream '{}' is ready", stream_name);

    // Create a durable consumer
    let consumer_config = async_nats::jetstream::consumer::pull::Config {
        durable_name: Some("order-processor".to_string()),
        deliver_policy: async_nats::jetstream::consumer::DeliverPolicy::All,
        ack_policy: async_nats::jetstream::consumer::AckPolicy::Explicit,
        ..Default::default()
    };

    let consumer = stream.create_consumer(consumer_config).await?;
    println!("Durable consumer 'order-processor' created");

    // Subscribe and get messages
    let mut messages = consumer.messages().await?;

    // Spawn a task to process messages
    tokio::spawn(async move {
        println!("Subscribed to durable topic, waiting for messages...");

        while let Some(message) = messages.next().await {
            match message {
                Ok(msg) => {
                    // Process message
                    let payload = String::from_utf8_lossy(&msg.payload);
                    println!("[{}] Received message: {}", id, payload);

                    // Acknowledge the message
                    if let Err(e) = msg.ack().await {
                        eprintln!("Failed to acknowledge message: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }
    });

    Ok(())
}

pub async fn publish_to_durable_topic(client: Client) -> Result<(), Box<dyn Error>> {
    // Get a JetStream context
    let jetstream = async_nats::jetstream::new(client);

    // Publish messages to the stream
    println!("Publishing messages to durable topic...");

    for i in 1..=5 {
        let subject = format!("orders.new");
        let payload = format!(
            "{{\"order_id\": \"ORD-{}\", \"amount\": {}}}",
            1000 + i,
            i * 10
        );

        let pb = String::from_utf8(payload.into_bytes()).unwrap();

        // Publish with JetStream
        match jetstream.publish(subject, pb.into()).await {
            Ok(ack) => {
                println!("Published message {}: {:?}", i, ack);
                println!("  - Acknowledged by server");
            }
            Err(e) => {
                println!("Failed to publish message {}: {}", i, e);
            }
        }
    }

    Ok(())
}
