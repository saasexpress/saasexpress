use log::{error, info};
use std::{any::Any, collections::HashSet, error::Error, rc::Rc, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::warn;

use crate::{
    dag::dag::{Graph, Node},
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
pub struct JSONToBuffer {
    pub width: u32,
}

#[derive(Clone)]
pub struct JSONToBufferNode {
    id: String,
    settings: JSONToBufferSettings,
    actors: Vec<MyActorHandle<JSONToBufferDo>>,
    children: HashSet<String>,
}

impl OperatorExecutor for JSONToBufferNode {
    // fn process(&self, message: &Message) -> Message {
    //     info!("JSONToBuffer PROCESSED: pass through");
    //     //        Ok(Some(message.clone()))
    //     let mut m = message.clone();
    //     m.log.push("JSON to Buffer".to_string());
    //     m
    // }
    fn notify(&self, context: &GraphContext, message: Message) {
        let actor = self.actors.get(0).unwrap();

        let fut = async {
            actor.get_unique_id().await;
            //actor.process(context, message);
        };
        async_std::task::block_on(fut);
    }

    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }
}

impl OperatorNode for JSONToBufferNode {
    fn speak(&self) {
        println!("TEMPLATE SPEAK!")
    }
    fn name(&self) -> &str {
        return &self.id;
    }
    fn operator(&self) -> &str {
        return "JSONToBuffer";
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
        return OperatorType::JSONToBuffer { node: self.clone() };
    }

    fn as_executor(&self) -> Box<dyn OperatorExecutor> {
        let executor: Box<dyn OperatorExecutor> = Box::new(JSONToBufferNode {
            id: self.id.clone(),
            actors: Vec::new(),
            children: HashSet::new(),
            settings: self.settings.clone(),
        });

        return executor;
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
        let actor = MyActorHandle::<JSONToBufferDo>::new(context);
        self.actors.push(actor);
    }
}

// Implement the operator trait for the template
impl Operator for JSONToBuffer {
    fn get_name(&self) -> &str {
        "JSONToBuffer"
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
            name: "JSONToBuffer".to_string(),
        }
    }

    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>> {
        let actors = Vec::new();

        let otype: OperatorType = OperatorType::JSONToBuffer {
            node: JSONToBufferNode {
                id: node.get_id().clone(),
                actors: actors,
                children: node.children(),
                settings: JSONToBufferSettings {},
            },
        };

        // let vec = self.nodes.get_mut();
        // vec.push(nd);

        // self.nodes.set(vec.to_vec());

        Ok(otype)
    }

    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>> {
        let settings = JSONToBufferSettings::default();
        // Assuming MapSettings is a function that maps settings from node.config to JSONToBufferSettings
        //map_settings(&node.config, &settings)?;
        //node.config = settings;
        let mut actors = Vec::new();
        // actors.push(MyActorHandle::<String>::new());

        Ok(Box::new(JSONToBufferNode {
            id: node.get_id().clone(),
            actors: actors,
            children: node.children(),
            settings: JSONToBufferSettings {},
        }))
    }
}

// Define other necessary structs and enums
// #[derive(Clone)]
// pub struct BaseOperator;

#[derive(Default, Clone)]
struct JSONToBufferSettings;

// struct Node {
//     config: JSONToBufferSettings,
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
pub struct JSONToBufferDo {
    something: String,
}

impl MyActorHandle<JSONToBufferDo> {
    pub fn new(graph_context: GraphContext) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = JSONToBufferActor::new(graph_context, receiver);
        tokio::spawn(async move { actor.run().await });

        let handle = JSONToBufferDo {
            something: "Huh".to_string(),
        };

        Self { sender, handle }
    }

    pub fn process(&self, message: Message) {
        let msg = ActorMessage::Process {
            message: message,
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

pub struct JSONToBufferActor {
    graph_context: GraphContext,
    receiver: mpsc::Receiver<ActorMessage>,
}

impl JSONToBufferActor {
    pub fn new(graph_context: GraphContext, receiver: mpsc::Receiver<ActorMessage>) -> Self {
        JSONToBufferActor {
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
                    "We got an actor message {} (actors={})",
                    message.state,
                    actors.len()
                );
            }
            ActorMessage::GetUniqueId { respond_to } => todo!(),
        }
    }
}
