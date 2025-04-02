use crate::{
    dag::dag::Node, dag_context::execute::GraphContext, dag_setup::actor_handle::MyActorHandle,
};
use futures::Stream;
use log::error;
use std::{any::Any, collections::HashSet, error::Error, rc::Rc, sync::Arc};

use super::{
    enums::OperatorType,
    //httpin::{HTTPIn, HTTPInNode},
};

pub trait OperatorExecutor: Send + Sync {
    //fn process(&self, message: &Message) -> Message;
    fn notify(&self, context: &GraphContext, message: Message);

    //fn node(&self) -> Arc<Node>;
}

pub trait OperatorNode {
    fn name(&self) -> &str;
    fn operator(&self) -> &str;
    fn speak(&self);
    //fn node(&self) -> Arc<Node>;
    // fn prepare(&self);
    // fn actors(&self) -> Vec<MyActorHandle>;
    fn children(&self) -> HashSet<String>;
    fn as_any(&self) -> Arc<&dyn Any>;
    fn as_executor_2(&self) -> OperatorType;
    fn as_executor(&self) -> Box<dyn OperatorExecutor>;
    fn process1(&self, message: &Message) -> Result<Option<Message>, Box<dyn Error>>;
    fn setup_actors(&mut self, context: GraphContext);
}

// Define the factory
struct OperatorNodeFactory;

impl OperatorNodeFactory {
    fn create_operator_node(op_type: &str) -> Box<dyn OperatorNode> {
        match op_type {
            // "HTTPIn" => Box::new(Dog),
            // "ReverseProxy" => Box::new(Cat),
            _ => panic!("Unknown animal type"),
        }
    }
}

// Define the trait for the operator
pub trait Operator {
    fn get_name(&self) -> &str;
    fn register(&self) -> Result<(), Box<dyn Error>>;
    fn deregister(&self);
    // fn handle_hook(
    //     &self,
    //     hook: HookType,
    //     node: &Node,
    //     message: &Message,
    // ) -> Result<(), Box<dyn Error>>;
    fn spec(&self) -> OperatorSpec;
    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>>;
    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>>;
}

pub struct OperatorSpec {
    pub name: String,
}

#[derive(Clone)]
pub struct Message {
    pub state: u8,
    pub log: Vec<String>,
}

pub struct MessageContext<T>(pub T);

pub struct OperatorBase<T> {
    pub meta: T,
}

// impl<Template> OperatorBase<Template> {
//   pub fn new(meta: Template) -> Self {
//       OperatorBase { meta: meta }
//   }
//   pub fn get_name(&self) -> &str {
//       "Template"
//   }
// }

impl<T: Operator> Operator for OperatorBase<T> {
    fn get_name(&self) -> &str {
        todo!()
    }

    fn register(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn deregister(&self) {
        todo!()
    }

    // fn handle_hook(
    //     &self,
    //     hook: HookType,
    //     node: &Node,
    //     message: &Message,
    // ) -> Result<(), Box<dyn Error>> {
    //     todo!()
    // }

    fn spec(&self) -> OperatorSpec {
        todo!()
    }

    fn setup_node_2(&self, node: &Node) -> Result<OperatorType, Box<dyn Error>> {
        todo!();
    }

    fn setup_node(&self, node: &Node) -> Result<Box<dyn OperatorNode + Send>, Box<dyn Error>> {
        todo!()
    }
}
