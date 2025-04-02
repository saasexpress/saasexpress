use log::{error, info, warn};
use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::{any::Any, cell::Cell};
use tokio::sync::{mpsc, oneshot};

use crate::dag::dag::Node;
use crate::dag_context::execute::GraphContext;
use crate::dag_setup::actor::{ActorMessage, MyActor};
use crate::dag_setup::actor_handle::MyActorHandle;

use super::enums::OperatorType;
use super::operator::{Message, Operator, OperatorExecutor, OperatorNode, OperatorSpec};
// use crate::dag::message_handler;

// Define the struct for the template operator
#[derive()]
pub struct HTTPIn {
    pub width: u32,
}

#[derive(Clone)]
pub struct HTTPInNode {
    id: String,
    settings: HTTPInSettings,
    actors: Vec<MyActorHandle<HTTPInNodeDo>>,
    children: HashSet<String>,
}

pub struct HTTPInActor {
    receiver: mpsc::Receiver<ActorMessage>,
}

impl HTTPInActor {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        HTTPInActor { receiver }
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

#[derive(Clone)]
pub struct HTTPInNodeDo {
    something: String,
}

impl MyActorHandle<HTTPInNodeDo> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = HTTPInActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        let handle = HTTPInNodeDo {
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

// impl MyActorHandle<HTTPInNode> {
//     pub fn new(handle: HTTPInNode) -> Self {
//         let (sender, receiver) = mpsc::channel(8);
//         let mut actor = MyActor::new(receiver);
//         tokio::spawn(async move { actor.run().await });

//         Self {
//             sender,
//             handle: handle,
//         }
//     }

// }

impl HTTPInNode {
    async fn do_some_work() {
        println!("do some work httpin");
    }
}

impl OperatorExecutor for HTTPInNode {
    fn process(&self, message: &Message) -> Message {
        println!("Not implemented HTTPIn yet {}", message.state);
        // println!("Settings {}", self.settings.method);

        // let actor = self.actors.get(0).unwrap();
        // let fut = async {
        //     warn!("DO {}", actor.handle.something);
        // };
        // async_std::task::block_on(fut);

        message.clone()
        //        Ok(Some(message.clone()))
    }
    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }

    fn notify(&self, context: &GraphContext, message: Message) {
        println!("Notify node of a new message");
        let actor = self.actors.get(0).unwrap();

        let rp = context.find(&"rp".to_string()).unwrap();

        error!("Found Child {}", rp.name());

        let fut = async {
            //actor.get_unique_id().await;
            actor.process(message);
        };
        async_std::task::block_on(fut);
    }
}

impl OperatorNode for HTTPInNode {
    fn speak(&self) {
        println!("HTTPIn SPEAK!")
    }
    fn name(&self) -> &str {
        return &self.id;
    }

    fn operator(&self) -> &str {
        return "HTTPIn";
    }

    // fn actors(&self) -> Vec<MyActorHandle<HTTPInNodeDo>> {
    //     return self.actors.to_vec();
    // }

    fn children(&self) -> HashSet<String> {
        return self.children.clone();
    }

    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }
    fn as_any(&self) -> Arc<&dyn Any> {
        info!("as_any HTTPInNode");
        Arc::new(self)
    }

    fn as_executor_2(&self) -> OperatorType {
        return OperatorType::HTTPIn { node: self.clone() };
    }

    fn as_executor(&self) -> Box<dyn OperatorExecutor> {
        let executor: Box<dyn OperatorExecutor> = Box::new(self.clone());

        return executor;
    }
    // fn prepare(&self) {
    //     let pnd = self.node.get_id().clone();
    //     let chl = &self.node.node.ports[0];
    //     let receiver = Arc::clone(&chl.1);

    //     let executor = self.as_executor();

    //     info!("prepare");
    //     tokio::spawn(async move {
    //         info!("prepare inside thread");
    //         message_handler(pnd.to_string(), receiver, executor).await;
    //     });
    // }

    fn process1(&self, message: &Message) -> Result<Option<Message>, Box<dyn Error>> {
        println!("Not implemented HTTPIn yet {}", message.state);
        println!("Settings {}", self.settings.method);
        Ok(Some(message.clone()))
    }
}

impl HTTPIn {
    pub fn new(width: u32) -> Self {
        HTTPIn { width: width }
    }
}

// Implement the operator trait for the template
impl Operator for HTTPIn {
    fn get_name(&self) -> &str {
        "HTTPIn"
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
            name: "HTTPIn".to_string(),
        }
    }

    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>> {
        let mut settings = HTTPInSettings::default();
        let yaml = &node.config;
        println!("Yeah HTTPin getting setup");
        if let Some(field) = yaml.as_mapping() {
            println!("Field value: {:?}", field);
        } else {
            println!("Field 'field_name' not found in YAML");
        }

        let mut actors = Vec::new();

        let actor = MyActorHandle::<HTTPInNodeDo>::new();

        actors.push(actor);

        // Assuming MapSettings is a function that maps settings from node.config to HTTPInSettings
        //map_settings(&node.config, &settings)?;
        //node.config = settings;
        //let mut s = Settings { node: settings };
        settings.method = yaml.get("method").unwrap().as_str().unwrap().to_string();

        let otype: OperatorType = OperatorType::HTTPIn {
            node: HTTPInNode {
                id: node.get_id().to_string(),
                actors,
                children: node.children(),
                settings,
            },
        };

        // let vec = self.nodes.get_mut();
        // vec.push(nd);

        // self.nodes.set(vec.to_vec());

        Ok(otype)
    }

    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>> {
        let mut settings = HTTPInSettings::default();
        let yaml = &node.config;
        println!("Yeah HTTPin getting setup");
        if let Some(field) = yaml.as_mapping() {
            println!("Field value: {:?}", field);
        } else {
            println!("Field 'field_name' not found in YAML");
        }

        let mut actors = Vec::new();

        let actor = MyActorHandle::<HTTPInNodeDo>::new();

        actors.push(actor);

        // Assuming MapSettings is a function that maps settings from node.config to HTTPInSettings
        //map_settings(&node.config, &settings)?;
        //node.config = settings;
        //let mut s = Settings { node: settings };
        settings.method = yaml.get("method").unwrap().as_str().unwrap().to_string();

        let nd = HTTPInNode {
            id: node.get_id().to_string(),
            actors,
            children: node.children(),
            settings,
        };

        //let otype: OperatorType = OperatorType::HTTPIn { node: nd };

        // let vec = self.nodes.get_mut();
        // vec.push(nd);

        // self.nodes.set(vec.to_vec());

        Ok(Box::new(nd))
    }
}

// Define other necessary structs and enums
// #[derive(Clone)]
// pub struct BaseOperator;

#[derive(Debug, Clone, Default)]
struct HTTPInSettings {
    method: String,
}

// struct Node {
//     config: HTTPInSettings,
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
