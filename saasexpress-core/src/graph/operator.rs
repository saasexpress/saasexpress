use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde::Serialize;
use serde_json::{Value, json};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::graph::operator_types::canonical_model::CanonicalModel;

use crate::my_reg::{broadcast_event, register};
use crate::operators::op_wrapper::OperatorWrapper;
use crate::ports::ports::Ports;
use crate::random::generate_random_id;

use super::super::operators::op_actor_handle::OperatorActorHandle;

use super::graph::{AsyncHandleTrait, Graph};
use super::message::{self, DebuggableSpan, Message, OriginMessage};
use super::meta::NodeMeta;
use super::operator_types::ai_tool::AIToolOperator;
use super::operator_types::canonical_model::CanonicalModelOperator;
use super::processors::basic::BasicProcessor;
use super::processors::port::Port;
use super::registry::GraphRegistry;
use async_trait::async_trait;

pub type OperatorRef = Arc<Mutex<(dyn Operator + 'static)>>;

pub type OperatorRefRead = Arc<dyn Operator + 'static>;

#[derive(Debug, Clone)]
pub struct OperatorRole {
    pub role: String,
    pub operator: OperatorRef,
}

impl OperatorRole {
    pub fn default() -> String {
        "default".to_string()
    }
}

pub trait InitOperator {
    fn start(&self) -> impl std::future::Future<Output = ()> + Send;
}

pub trait Filter2Operator: Sync + Send + Debug {
    fn handle(&self, message: Message) -> Message;
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorState {
    Pending,
    Ready,
}

#[derive(Debug)]
pub enum OperatorType {
    /// Represents an endpoint operator
    /// that receives messages from the outside world
    /// and sends them to the next operator
    /// in the graph
    /// e.g. HTTPIn, MQTTIn, etc.
    Endpoint,
    /// Represents a filter operator
    /// that processes messages and sends them to the next operator
    /// in the graph
    /// e.g. HTTPOut, MQTTOut, etc.
    Filter,

    Filter2 {
        operator: Arc<dyn Filter2Operator + 'static>,
    },

    // Represents a canonical model operator
    CanonicalModel,

    /// Represents an AI agent operator
    AIAgent,

    /// Represents an AI tool operator
    AITool {
        tool: Arc<dyn AIToolOperator + 'static>,
    },
}

impl PartialEq for OperatorType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OperatorType::Endpoint, OperatorType::Endpoint) => true,
            (OperatorType::Filter, OperatorType::Filter) => true,
            (OperatorType::Filter2 { operator: _ }, OperatorType::Filter2 { operator: _ }) => true,
            (OperatorType::CanonicalModel, OperatorType::CanonicalModel) => true,
            (OperatorType::AIAgent, OperatorType::AIAgent) => true,
            (OperatorType::AITool { tool: _ }, OperatorType::AITool { tool: _ }) => true,
            _ => false,
        }
    }
}

// Operator trait - defines how to process a message
pub trait Operator: Send + Sync + Debug {
    fn _type(&self) -> OperatorType;
    fn name(&self) -> String;
    fn state(&self) -> OperatorState {
        OperatorState::Ready
    }

    fn new_runtime(&self) -> Arc<dyn OperatorRuntime> {
        panic!("No runtime defined for operator {}", self.name());
    }

    //fn meta(&self) -> NodeMeta;

    //fn init(&mut self, graphs: Vec<&mut Graph>);
    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        debug!("Default init operator {} - no action", self.name());
    }

    fn finalize(&mut self) -> bool {
        debug!("Default finalize operator {} - no action", self.name());
        true
    }

    /// commands and events for controlling the operator
    fn control(&mut self, message: Message);

    /// performs the work of the operator
    fn handle(&self, message: Message) -> Message;

    /// Sends a message to this operator for handling
    ///
    /// Examples:
    /// OperatorType::Endpoint (HTTPIn) : self.next(self.handle(message));
    /// OperatorType::Endpoint (FanOut) : self.next(message); (handle Not implemented)
    /// ActorHandle : Send to actor to handle message; (handle Not implemented)
    /// OperatorType::Filter (ActorHandle) : Send to actor to handle message and actor sends to next operators
    ///
    fn send(&self, message: Message);

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>>;
    fn wait(&self) -> Message;

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }
    fn handle_ptr(&self, message: Arc<Message>) -> Arc<Message> {
        debug!("default handle (passthrough)... {}", self.name());
        return message;
    }
    fn next_ptr(&self, message: Arc<Message>) {
        // Sending message to next operator
        for n in self.get_output_channels() {
            n.lock().unwrap().send_ptr(message.to_owned());
            //break;
        }
    }
    fn get_output_channels(&self) -> &Vec<OperatorRef>;
}

pub trait OperatorRuntime: Send + Sync + Debug {
    fn _type(&self) -> OperatorType;
    fn name(&self) -> String;
    fn state(&self) -> OperatorState {
        OperatorState::Ready
    }

    /// performs the work of the operator
    fn handle(&self, message: Message) -> Message;

    fn send(&self, message: Message);

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>>;
}
