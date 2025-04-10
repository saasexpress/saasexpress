use std::process::Child;
use std::sync::Arc;
use std::{marker::PhantomData, process::ChildStdin};

use crate::graph::graph::Message as GraphMessage;
use futures::SinkExt;
use futures::StreamExt;
use futures::channel::mpsc;
use std::io::Write;
use tracing::{debug, error};

pub struct ShellActor<T, F>
where
    T: ShellWorker + Sync + Send + 'static,
    F: FnMut(mpsc::Receiver<GraphMessage>) -> T + Send + 'static,
{
    sender: mpsc::Sender<GraphMessage>,
    builder: PhantomData<F>,
}

impl<T, F> ShellActor<T, F>
where
    T: ShellWorker + Sync + Send + 'static,
    F: FnMut(mpsc::Receiver<GraphMessage>) -> T + Send + 'static,
{
    pub fn new(mut f: F) -> Self {
        let (sender, receiver) = mpsc::channel(8);

        let mut worker = f(receiver);

        tokio::spawn(async move { worker.run().await });

        ShellActor {
            sender,
            builder: PhantomData,
        }
    }

    pub fn send(&mut self, msg: GraphMessage) {
        match self.sender.try_send(msg) {
            Ok(_) => debug!("GraphMessage sent"),
            Err(e) => panic!("Failed to send: {}", e),
        }
    }
}

pub trait ShellWorker: Send {
    // fn execute(&self, command: &str) -> Result<String, String>;
    //fn init(&self, receiver: Receiver<GraphMessage>);
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;
}

pub struct ActorStdin {
    receiver: mpsc::Receiver<GraphMessage>,
    //stdin: ChildStdin,
}

impl ActorStdin {
    pub fn new(receiver: mpsc::Receiver<GraphMessage>) -> Self {
        //        let stdin = stdin.expect("handle present");

        ActorStdin { receiver }
    }
}

impl ShellWorker for ActorStdin {
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        async move {
            debug!("running..");
            while let Some(msg) = self.receiver.next().await {
                debug!("Received GraphMessage: {:?}", msg);

                // send to next Operator
                match msg {
                    GraphMessage::Standard { message, .. } => {
                        error!("Not implemented");

                        // if let Err(e) = self.stdin.write_all(&message) {
                        //     error!("Error writing to stdin: {}", e);
                        // }
                    }
                    _ => {
                        panic!("Unknown GraphMessage type");
                    }
                }
            }
        }
    }
}

pub struct ActorStdout {
    receiver: mpsc::Receiver<GraphMessage>,
    out_channel: mpsc::Sender<GraphMessage>,
}

impl ActorStdout {
    pub fn new(
        receiver: mpsc::Receiver<GraphMessage>,
        out_channel: mpsc::Sender<GraphMessage>,
    ) -> Self {
        ActorStdout {
            receiver,
            out_channel,
        }
    }
}

impl ShellWorker for ActorStdout {
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        async move {
            debug!("running..");
            while let Some(msg) = self.receiver.next().await {
                debug!("Received Message: {:?}", msg);
                self.out_channel
                    .send(msg)
                    .await
                    .expect("Failed to send GraphMessage to out_channel");
            }
        }
    }
}
