use log::{error, info};
use std::{any::Any, collections::HashSet, error::Error, f32::consts::E, rc::Rc, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::warn;

use crate::{
    dag::dag::Node,
    dag_context::execute::GraphContext,
    dag_setup::{actor::ActorMessage, actor_handle::MyActorHandle},
};

use super::{
    enums::OperatorType,
    operator::{Message, Operator, OperatorExecutor, OperatorNode, OperatorSpec},
    Settings,
};

// Define the struct for the template operator
#[derive(Clone)]
pub struct BufferToJSON {
    pub width: u32,
}

#[derive(Clone)]
pub struct BufferToJSONNode {
    id: String,
    settings: BufferToJSONSettings,
    actors: Vec<MyActorHandle<BufferToJSONDo>>,
    children: HashSet<String>,
}

impl OperatorExecutor for BufferToJSONNode {
    // fn process(&self, message: &Message) -> Message {
    //     info!("BufferToJSON PROCESSED: pass through");
    //     //        Ok(Some(message.clone()))

    //     let mut m = message.clone();
    //     m.log.push("Buffer to JSON".to_string());
    //     m
    // }
    fn notify(&self, context: &GraphContext, message: Message) {
        println!("Notify node of a new message");
        let actor = self.actors.get(0).unwrap();

        //let rp = context.find(&"rp".to_string()).unwrap();

        //error!("Found Child {}", rp.name());

        let fut = async {
            //actor.get_unique_id().await;
            actor.process(message);
        };
        async_std::task::block_on(fut);
    }

    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }
}

impl OperatorNode for BufferToJSONNode {
    fn speak(&self) {
        println!("[BufferToJSONNode] TEMPLATE SPEAK!")
    }

    fn name(&self) -> &str {
        return &self.id;
    }

    fn operator(&self) -> &str {
        return "BufferToJSON";
    }

    // fn actors(&self) -> Vec<MyActorHandle> {
    //     return self.actors.to_vec();
    // }

    fn children(&self) -> HashSet<String> {
        return self.children.clone();
    }

    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }
    fn as_any(&self) -> Arc<&dyn Any> {
        Arc::new(self)
    }
    fn as_executor_2(&self) -> OperatorType {
        return OperatorType::BufferToJSON { node: self.clone() };
    }

    fn as_executor(&self) -> Box<dyn OperatorExecutor> {
        let executor: Box<dyn OperatorExecutor> = Box::new(self.clone());

        return executor;

        // let executor: Box<dyn OperatorExecutor> = Box::new(BufferToJSONNode {
        //     id: self.id.clone(),
        //     actors: Vec::new(),
        //     children: HashSet::new(),
        //     settings: self.settings,
        // });

        // return executor;
    }

    // fn prepare(&self) {
    //     tokio::spawn(async move {
    //         info!("prepare");
    //         // message_handler(pnd.to_string(), receiver, executor);
    //     });
    // }

    fn process1(&self, message: &Message) -> Result<Option<Message>, Box<dyn Error>> {
        println!("Not implemented yet");
        Ok(Some(message.clone()))
    }

    fn setup_actors(&mut self, context: GraphContext) {
        let actor = MyActorHandle::<BufferToJSONDo>::new(context);
        self.actors.push(actor);
    }
}

// Implement the operator trait for the template
impl Operator for BufferToJSON {
    fn get_name(&self) -> &str {
        "BufferToJSON"
    }

    fn register(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn deregister(&self) {}

    // fn handle_hook(
    //     &self,
    //     hook: HookType,
    //     node: &Node,
    //     message: &Message,
    // ) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }

    fn spec(&self) -> OperatorSpec {
        OperatorSpec {
            name: "BufferToJSON".to_string(),
        }
    }

    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>> {
        return Err(("Not implemented").into());
    }
    //     let mut actors = Vec::new();

    //     let actor = MyActorHandle::<BufferToJSONDo>::new();

    //     actors.push(actor);

    //     let settings = BufferToJSONSettings::default();

    //     let otype: OperatorType = OperatorType::BufferToJSON {
    //         node: BufferToJSONNode {
    //             id: node.get_id().to_string(),
    //             actors,
    //             children: node.children(),
    //             settings,
    //         },
    //     };

    //     // let vec = self.nodes.get_mut();
    //     // vec.push(nd);

    //     // self.nodes.set(vec.to_vec());

    //     Ok(otype)
    // }

    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>> {
        let settings = BufferToJSONSettings::default();
        // Assuming MapSettings is a function that maps settings from node.config to BufferToJSONSettings
        //map_settings(&node.config, &settings)?;
        //node.config = settings;
        let actors = Vec::new();

        //actors.push(MyActorHandle::<String>::new());

        Ok(Box::new(BufferToJSONNode {
            id: node.get_id().clone(),
            actors,
            children: node.children(),
            settings,
        }))
    }
}

// Define other necessary structs and enums
// #[derive(Clone)]
// pub struct BaseOperator;

#[derive(Default, Clone)]

struct BufferToJSONSettings;

// struct Node {
//     config: BufferToJSONSettings,
// }

enum HookType {
    // Define your hook types here
    ExampleHookType,
}

// Assume a function that maps settings from one type to another
fn map_settings<T, U>(source: &T, target: &U) -> Result<(), Box<dyn Error>> {
    // Implement the mapping logic
    Ok(())
}

#[derive(Clone)]
pub struct BufferToJSONDo {
    something: String,
}

impl MyActorHandle<BufferToJSONDo> {
    pub fn new(graph_context: GraphContext) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = BufferToJSONActor::new(graph_context, receiver);
        tokio::spawn(async move { actor.run().await });

        let handle = BufferToJSONDo {
            something: "Huh".to_string(),
        };

        Self { sender, handle }
    }

    pub fn process(&self, message: Message) {
        info!("[BufferToJSONDo] Publish to MPSC {}", message.state);
        let msg = ActorMessage::Process {
            message,
            actors: Vec::new(),
        };
        match self.sender.try_send(msg) {
            Ok(_) => println!("Message sent!"),
            Err(e) => println!("Failed to send: {}", e),
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
    }
}

pub struct BufferToJSONActor {
    graph_context: GraphContext,
    receiver: mpsc::Receiver<ActorMessage>,
}

impl BufferToJSONActor {
    pub fn new(graph_context: GraphContext, receiver: mpsc::Receiver<ActorMessage>) -> Self {
        BufferToJSONActor {
            graph_context,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::Process { message, actors } => {
                warn!(
                    "We got a BufferToJSON actor message {} (actors={})",
                    message.state,
                    actors.len()
                );

                // do the processing here..

                actors.iter().for_each(|f| {
                    f.notify(&self.graph_context, message.clone());
                });
            }
            ActorMessage::GetUniqueId { respond_to } => todo!(),
        }
    }
}
