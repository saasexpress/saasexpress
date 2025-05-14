use saasexpress_core::{
    graph::{
        graph::{AsyncHandleTrait, Graph, Operator, OperatorRef, OperatorRole, OperatorType},
        message::{ControlCommand, Message},
        meta::NodeMeta,
    },
    settings::settings::{Setting, env_settings},
};
use serde_json::Value;
use tracing::{debug, info};

use super::resources::get_instance;
use core::panic;
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
    engine: Engine,
    ws: bool,
    sse: bool,
    routes: Vec<String>,
    method: String,
    next: Vec<OperatorRole>,
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
            engine,
            ws,
            sse,
            routes,
            method,
            next: Vec::new(),
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
            engine,
            ws,
            sse,
            routes,
            method,
            next: Vec::new(),
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

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
    }

    // fn handle_ptr(&self, _message: Arc<Message>) -> Arc<Message> {
    //     debug!("HTTPIn handle (passthrough)... {}", self.name());
    //     return _message;
    // }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        self.settings = env_settings(graph.base_env_vars_settings(node_meta))
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, start, .. } => {
                for n in next {
                    self.add_next(n);
                }

                self.setup_routes(start);
            }
            Message::Control { command, .. } => {
                let mut current_settings = self.settings.to_owned();
                match command {
                    ControlCommand::SetSettings { settings } => {
                        settings.iter().for_each(|(k, v)| {
                            current_settings.push(Setting {
                                key: k.replace("-", "_").to_uppercase().to_string(),
                                value: v.as_str().unwrap_or("").to_string(),
                            });
                        });
                    }
                    _ => {
                        panic!("Invalid control command {:?}", command);
                    }
                }
                self.settings = current_settings;
            }

            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, message: Message) {
        self.next(message);
    }

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }

    fn next_ptr(&self, message: Arc<Message>) {
        for n in &self.next {
            n.operator.lock().unwrap().send_ptr(message.to_owned());
        }
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<OperatorRef> {
        panic!("Not implemented");
        // self.next
        //     .iter()
        //     .map(|n| n.operator)
        //     .collect::<Vec<OperatorRef>>()
        //     .as_ref()
    }
}

impl HTTPIn {
    fn next(&self, message: Message) {
        let mut counter = 0;
        for n in &self.next {
            if counter == 0 {
                n.operator.lock().unwrap().send(message);
                break;
            } else {
                info!("Not implemented");
            }
            counter = counter + 1;
        }
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }

    fn setup_routes(&self, start: Arc<Mutex<dyn Operator + 'static>>) {
        let mut singleton = get_instance().lock().unwrap();
        singleton.add_routes(
            self.routes.to_owned(),
            self.method.to_owned(),
            self.ws,
            self.sse,
            start,
        );
    }
}
