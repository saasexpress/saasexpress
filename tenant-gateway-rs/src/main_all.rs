// mod operators;

use log::{error, info, warn};
use tenant_gateway::dag::dag::Graph;
use tenant_gateway::dag_context::execute::GraphContext;
use tenant_gateway::dag_setup::actor_handle::MyActorHandle;
use tenant_gateway::operators::fanout::fanout;
use tenant_gateway::operators::operator::{Message, OperatorNode};
use tenant_gateway::operators::register;
use tenant_gateway::operators::{do_it, operator};

use axum::{routing::get_service, Router};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_yaml;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use std::time::Duration;
use std::{env, fs::File, io::BufReader, thread, time};
use tracing::Level;

#[derive(Serialize, Deserialize)]
struct Payload {
    foo: String,
    bar: u8,
}

struct Hello {
    name: String,
    base: u8,
}

struct MyWaker;
impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        println!("Woken up!");
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    //fanout().await;

    let hdl = MyActorHandle::<String>::new();
    warn!("get_unq {}", hdl.get_unique_id().await);
    warn!("get_unq {}", hdl.get_unique_id().await);
    warn!("get_unq {}", hdl.get_unique_id().await);

    let operator_registry = register();
    // open Graph json file

    // let mut dag = Graph::new();

    let dag = Graph::new_using_yaml("samples/test.yaml");
    match dag {
        Ok(mut dag) => {
            println!("Graph successfully loaded.");
            println!(
                "Path from A to C: {}",
                dag.has_path(&"start".to_string(), &"rp".to_string())
            );
            println!(
                "Path from C to A: {}",
                dag.has_path(&"C".to_string(), &"A".to_string())
            );

            let op_nodes = dag.init_node_operators(operator_registry);

            // dag.get_nodes().iter().for_each(|(_, node)| {
            //     println!("node={}", node.get_id());

            //     let actor_handler = node.node.actors.get(0).unwrap();
            //     let fut = async { actor_handler.get_unique_id().await };

            //     async_std::task::block_on(fut);
            // });
            // let op_nodes: Vec<Box<dyn OperatorNode>> = dag.prepare(operator_registry);

            let ctx = GraphContext::new(op_nodes);

            println!("Executing Graph...");
            match ctx
                .execute(&"start".to_string(), Message { state: 3 })
                .await
            {
                Ok(_) => {
                    info!("Yippy")
                }
                Err(err) => {
                    error!("Error {}", err)
                }
            }

            // let wait = time::Duration::from_secs(1);
            // thread::sleep(wait);
        }
        Err(e) => println!("Failed to load Graph: {}", e),
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let client = async_nats::connect(nats_url).await?;

    let mut subscriber = client.subscribe("foo").await?.take(2);

    let _hello = Hello {
        name: "Joe".to_string(),
        base: 12,
    };

    let payload = Payload {
        foo: "bar".to_string(),
        bar: 27,
    };
    let bytes = serde_json::to_vec(&json!(payload))?;

    client.publish("foo", bytes.into()).await?;
    client.publish("foo", "not json".into()).await?;

    while let Some(message) = subscriber.next().await {
        if let Ok(payload) = serde_json::from_slice::<Payload>(message.payload.as_ref()) {
            println!(
                "received valid JSON payload: foo={:?} bar={:?}",
                payload.foo, payload.bar
            );
        }
    }

    do_it().await.unwrap();

    Ok(())
}
