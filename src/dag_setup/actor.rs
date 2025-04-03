use log::{info, warn};
use tokio::sync::{mpsc, oneshot};
use tracing::error;

use crate::{
    dag_context::execute::GraphContext,
    operators::{
        //httpin::HTTPInNodeDo,
        operator::{Message, OperatorExecutor},
    },
};

use super::actor_handle::MyActorHandle;

// REFERENCE: https://ryhl.io/blog/actors-with-tokio/

pub struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
pub enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    Process {
        message: Message,
        actors: Vec<Box<dyn OperatorExecutor>>,
    },
}

impl MyActor {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                info!("Great, we are handling message and sending it back");
                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }
            _ => {
                error!("Unsupport message type");
            }
        }
    }
}

// async fn run_my_actor(mut actor: MyActor) {
//     while let Some(msg) = actor.receiver.recv().await {
//         actor.handle_message(msg);
//     }
// }
