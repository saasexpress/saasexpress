use super::actor::{ActorMessage, MyActor};
use log::info;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct MyActorHandle<T> {
    pub sender: mpsc::Sender<ActorMessage>,
    pub handle: T,
}

impl MyActorHandle<String> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self {
            sender,
            handle: "s".to_string(),
        }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: send };

        //let a = self.sender.try_send(msg);

        match self.sender.try_send(msg) {
            Ok(_) => println!("Message sent!"),
            Err(e) => println!("Failed to send: {}", e),
        }

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        // let _ = self.sender.send(msg).await;
        match recv.await {
            Ok(msg) => {
                info!("Message returned! {}", msg);
                msg
            }
            Err(e) => {
                println!("Failed to send: {}", e);
                0
            }
        }

        //recv.await.expect("Actor task has been killed")
    }
}
