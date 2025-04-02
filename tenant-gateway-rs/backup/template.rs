use log::{error, info};
use std::{any::Any, collections::HashSet, error::Error, rc::Rc, sync::Arc};

use crate::{
    dag::dag::Node, dag_context::execute::GraphContext, dag_setup::actor_handle::MyActorHandle,
};

use super::{
    enums::OperatorType,
    operator::{Message, Operator, OperatorExecutor, OperatorNode, OperatorSpec},
    Settings,
};

// Define the struct for the template operator
#[derive(Clone)]
pub struct Template {
    pub width: u32,
}

pub struct TemplateNode {
    id: String,
    settings: Arc<Settings<TemplateSettings>>,
    actors: Vec<MyActorHandle<String>>,
    children: HashSet<String>,
}

impl OperatorExecutor for TemplateNode {
    fn process(&self, message: &Message) -> Message {
        println!("Not implemented yet");
        //        Ok(Some(message.clone()))
        message.clone()
    }
    fn notify(&self, context: &GraphContext, message: Message) {
        println!("Notify not ready");
    }

    // fn node(&self) -> Arc<Node> {
    //     Arc::clone(&self.node)
    // }
}

impl OperatorNode for TemplateNode {
    fn speak(&self) {
        println!("TEMPLATE SPEAK!")
    }
    fn name(&self) -> &str {
        return &self.id;
    }
    fn operator(&self) -> &str {
        return "Template";
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
        return OperatorType::ReverseProxy {};
    }

    fn as_executor(&self) -> Box<dyn OperatorExecutor> {
        let executor: Box<dyn OperatorExecutor> = Box::new(TemplateNode {
            id: self.id.clone(),
            actors: Vec::new(),
            children: HashSet::new(),
            settings: Arc::clone(&self.settings),
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
}

// Implement the operator trait for the template
impl Operator for Template {
    fn get_name(&self) -> &str {
        "Template"
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
            name: "Template".to_string(),
        }
    }

    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>> {
        todo!();
    }

    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>> {
        let settings = TemplateSettings::default();
        // Assuming MapSettings is a function that maps settings from node.config to TemplateSettings
        //map_settings(&node.config, &settings)?;
        //node.config = settings;
        let mut actors = Vec::new();
        actors.push(MyActorHandle::<String>::new());

        Ok(Box::new(TemplateNode {
            id: node.get_id().clone(),
            actors: actors,
            children: node.children(),
            settings: Arc::new(Settings {
                node: TemplateSettings {},
            }),
        }))
    }
}

// Define other necessary structs and enums
// #[derive(Clone)]
// pub struct BaseOperator;

#[derive(Default)]
struct TemplateSettings;

// struct Node {
//     config: TemplateSettings,
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
