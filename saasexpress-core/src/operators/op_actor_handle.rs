use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use fastrace::local::LocalSpan;
use fastrace::prelude::SpanContext;
use fastrace::{Span, trace};
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, error, info, info_span, instrument, warn};

use crate::graph;
use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime,
    OperatorRuntimeType, OperatorType,
};

use crate::graph::message::Message;

use crate::graph::meta::NodeMeta;
use crate::graph::registry::GraphRegistry;
use crate::my_reg::{ControlEvent, register};

use super::op_actor::OpActor;

use fastrace::future::FutureExt;
use tracing::Instrument;

#[derive(Debug)]
pub(crate) struct OperatorActorHandle {
    //sender: mpsc::Sender<Message>,
    graph_name: String,
    id: String,
    name: String,
    operator: Box<dyn Operator + 'static>,
    //_nodes: Vec<OperatorRef>,
}

impl OperatorActorHandle {
    //#[instrument[name = "op-actor-handle", skip_all]]
    #[trace]
    pub fn new<T>(graph_name: String, id: String, operator: T) -> Self
    where
        T: Operator + 'static,
    {
        let nm = operator.name();
        //let name = operator.name().clone();
        // let (sender, receiver) = mpsc::channel(8);

        // let mut actor = OpActor::new(name, receiver, operator);

        // let future = async move {
        //     actor.run().await;
        // };

        // tokio::spawn(future);

        Self {
            id,
            operator: Box::new(operator),
            name: String::clone(&nm),
            graph_name,
        }
    }
}

impl Operator for OperatorActorHandle {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn new_runtime(
        &self,
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let next_nodes = graph_operator_context.get_next_nodes();

        let (sender, receiver) = mpsc::channel(8);

        let runtime = self.operator.new_runtime(graph_operator_context.clone());

        let mut actor = OpActor::new(
            self.name.clone(),
            receiver,
            Arc::clone(&runtime),
            next_nodes,
        );

        let future = async move {
            actor.run().await;
        };

        tokio::spawn(future);

        Arc::new(OperatorActorHandleRuntime {
            sender,
            name: self.name.clone(),
        })
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        debug!("Not implemented");
    }

    fn control(&mut self, _message: Message) {
        debug!(
            "Control message received - sending to operator: {:?}",
            _message
        );
        self.operator.control(_message);
        // match _message {
        //     Message::Init2 { .. } => match self.sender.try_send(_message) {
        //         Ok(_) => debug!("Message sent to {}", self.name),
        //         Err(e) => panic!("Failed to send: {}", e),
        //     },
        //     Message::Init { .. } => match self.sender.try_send(_message) {
        //         Ok(_) => debug!("Message sent to {}", self.name),
        //         Err(e) => panic!("Failed to send: {}", e),
        //     },
        //     Message::Control { .. } => match self.sender.try_send(_message) {
        //         Ok(_) => debug!("Message sent to {}", self.name),
        //         Err(e) => panic!("Failed to send: {}", e),
        //     },
        //     _ => {
        //         panic!("Unexpected message type for control");
        //     }
        // }
    }
}

#[derive(Debug)]
pub(crate) struct OperatorActorHandleRuntime {
    sender: mpsc::Sender<Message>,
    name: String,
    //next_nodes: Vec<OperatorRef>,
}

impl OperatorRuntime for OperatorActorHandleRuntime {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
    }

    fn send(&self, _message: Message) {
        match _message {
            Message::Control { .. } => {
                error!("Unexpected message type {}", _message);
            }
            _ => match self.sender.try_send(_message) {
                Ok(_) => debug!("Message sent to {}", self.name),
                Err(e) => {
                    error!("Failed to send: {}", e)
                }
            },
        }
    }
}
