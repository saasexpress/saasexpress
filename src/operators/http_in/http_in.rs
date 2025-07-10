use saasexpress_core::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRole, OperatorRuntime,
    OperatorRuntimeType, OperatorState, OperatorType,
};
use saasexpress_core::{
    graph::{
        graph::{AsyncHandleTrait, Graph},
        message::{ControlCommand, Message},
        meta::NodeMeta,
    },
    settings::settings::{Setting, env_settings},
};
use serde_json::Value;
use tracing::{debug, info};

use super::resources::get_instance;
use core::panic;
use std::collections::{HashMap, HashSet};
use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub(crate) enum Engine {
    Axum,
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::Axum => write!(f, "axum"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct HTTPIn {
    id: String,
    fqn: String,
    engine: Engine,
    ws: bool,
    sse: bool,
    routes: Vec<String>,
    method: String,
    next_nodes: Vec<OperatorRole>,
    settings: Vec<Setting>,
}

impl From<Value> for HTTPIn {
    fn from(value: Value) -> Self {
        let routes = value["routes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let method = value["method"].as_str().unwrap().to_string();
        let engine = value
            .get("engine")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "axum" => Engine::Axum,
                _ => panic!("Unknown engine: {}", s),
            })
            .unwrap_or(Engine::Axum);
        let ws = value["ws"].as_bool().unwrap_or(false);
        let sse = value["sse"].as_bool().unwrap_or(false);

        HTTPIn {
            id: "".to_string(),
            fqn: "".to_string(),
            engine,
            ws,
            sse,
            routes,
            method,
            next_nodes: Vec::new(),
            settings: env_settings("HTTPIN_AXUM".to_string()),
        }
    }
}

impl From<serde_yaml::Value> for HTTPIn {
    fn from(value: serde_yaml::Value) -> Self {
        let routes = value["routes"]
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let method = value["method"].as_str().unwrap().to_string();
        let engine = value
            .get("engine")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "axum" => Engine::Axum,
                _ => panic!("Unknown engine: {}", s),
            })
            .unwrap_or(Engine::Axum);
        let ws = value["ws"].as_bool().unwrap_or(false);
        let sse = value["sse"].as_bool().unwrap_or(false);

        HTTPIn {
            id: "".to_string(),
            fqn: "".to_string(),
            engine,
            ws,
            sse,
            routes,
            method,
            next_nodes: Vec::new(),
            settings: env_settings("HTTPIN_AXUM".to_string()),
        }
    }
}

impl Operator for HTTPIn {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }
    fn name(&self) -> String {
        "HTTPIn".to_string()
    }

    fn new_runtime(
        &self,
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let next_nodes = graph_operator_context.get_next_nodes();

        Arc::new(HTTPIn {
            id: self.id.clone(),
            fqn: self.fqn.clone(),
            engine: self.engine.clone(),
            ws: self.ws,
            sse: self.sse,
            routes: self.routes.clone(),
            method: self.method.clone(),
            next_nodes,
            settings: self.settings.clone(),
        })
    }

    // fn handle_ptr(&self, _message: Arc<Message>) -> Arc<Message> {
    //     debug!("HTTPIn handle (passthrough)... {}", self.name());
    //     return _message;
    // }

    fn init(&mut self, _graph: &mut Graph, node_meta: &NodeMeta) {
        self.id = node_meta.name.clone();
        self.fqn = node_meta.fqn();
        self.settings = env_settings(node_meta.base_env_vars_settings(node_meta))
    }

    fn control(&mut self, _message: Message) {
        match _message {
            // Message::Init { next, start, .. } => {
            //     for n in next {
            //         self.add_next(n);
            //     }

            //     self.setup_routes(start);
            // }
            Message::Control { command, .. } => match command {
                ControlCommand::Start { runtime } => {
                    info!("HTTPIn - Start command received for {:}", self.fqn);
                    self.setup_routes(runtime);
                }
                ControlCommand::SetSettings { settings } => {
                    let mut current_settings = self.settings.to_owned();
                    settings.iter().for_each(|(k, v)| {
                        current_settings.push(Setting {
                            key: k.replace("-", "_").to_uppercase().to_string(),
                            value: v.as_str().unwrap_or("").to_string(),
                        });
                    });
                    self.settings = current_settings;
                }
                _ => {
                    panic!("Invalid control command {:?}", command);
                }
            },

            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }
}

impl HTTPIn {
    fn next(&self, message: Message) {
        let mut counter = 0;
        for n in &self.next_nodes {
            if counter == 0 {
                n.operator.send(message);
                break;
            } else {
                info!("Not implemented");
            }
            counter = counter + 1;
        }
    }

    fn setup_routes(&self, start: OperatorRuntimeType) {
        let singleton = get_instance().unwrap();
        let mut singleton = singleton.lock().unwrap();
        singleton.add_routes(
            self.fqn.clone(),
            self.routes.to_owned(),
            self.method.to_owned(),
            self.ws,
            self.sse,
            start,
        );
    }
}

impl OperatorRuntime for HTTPIn {
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
        return _message;
    }

    fn send(&self, message: Message) {
        self.next(message);
    }
}
