use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use tracing::{error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime,
    OperatorRuntimeType, OperatorState, OperatorType,
};

use crate::graph::message::{ControlCommand, Message};

use crate::graph::meta::NodeMeta;
// use crate::shared_resource::get_shared_service;
//use crate::operators::global_space::resource::get_shared_service;
use crate::shared_resource::SharedServiceRef;

use super::resource::WidgetsSharedService;

#[derive(Clone, Debug)]
pub(crate) struct GlobalSpace;

impl From<serde_yaml::Value> for GlobalSpace {
    fn from(_value: serde_yaml::Value) -> Self {
        GlobalSpace {}
    }
}

impl Operator for GlobalSpace {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "GlobalSpace".to_string()
    }

    fn new_runtime(
        &self,
        _graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        warn!("Init Not implemented");
    }

    fn control(&mut self, _message: Message) {
        info!("Control message received: {:?}", _message);
        match _message {
            Message::Control { command, .. } => match command {
                ControlCommand::Start { runtime } => {
                    self.setup_shared_resource(runtime);
                }
                _ => {
                    panic!("Invalid control command {:?}", command);
                }
            },
            _ => {
                panic!("Invalid message type {:?}", _message);
            }
        }
    }

    fn shared_resources(&self) -> Vec<SharedServiceRef> {
        let mut list = vec![];
        let s = WidgetsSharedService::get_instance();
        let s = s.unwrap();
        list.push(Arc::clone(&s) as SharedServiceRef);
        list
    }
}

impl OperatorRuntime for GlobalSpace {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { message, origin } => {
                return Message::Standard {
                    message: message.to_owned(),
                    origin,
                };
            }
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                }
            }
        }
    }

    fn send(&self, _: Message) {
        panic!("Send Not implemented");
    }
}

impl GlobalSpace {
    fn setup_shared_resource(&self, start: OperatorRuntimeType) {
        info!("Setting up shared resource for GlobalSpace");
        let singleton = WidgetsSharedService::get_instance().unwrap();
        let mut singleton = singleton.lock().unwrap();

        singleton.add_widget(start.name().to_string());
        // singleton.add_routes(
        //     self.routes.to_owned(),
        //     self.method.to_owned(),
        //     self.ws,
        //     self.sse,
        //     start,
        // );
    }
}
