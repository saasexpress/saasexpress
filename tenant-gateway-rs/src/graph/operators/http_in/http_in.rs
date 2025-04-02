use axum::{
    extract::State,
    routing::{any, post},
    Json, Router,
};
use futures::channel::oneshot;
use serde_json::{json, Value};
use tracing::{debug, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use super::{
    super::super::graph::{Message, Operator},
    resources::get_instance,
    websocket::ws_handler,
};
use core::panic;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

#[derive(Debug)]
pub(crate) struct HTTPIn {
    routes: Vec<String>,
    method: String,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
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

        HTTPIn {
            routes,
            method,
            next: Vec::new(),
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

        HTTPIn {
            routes,
            method,
            next: Vec::new(),
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
        info!(
            "HTTPIn handle (passthrough)... {} {:?}",
            self.name(),
            _message
        );
        return _message;
        // match _message {
        //     Message::Standard { message, origin } => {
        //         debug!("Passthrough message Standard");
        //         return Message::Standard {
        //             message: message.to_owned(),
        //             origin,
        //         };
        //     }
        //     Message::ReqReply {
        //         message,
        //         respond_to,
        //     } => {
        //         debug!("Passthrough message ReqReply");
        //         return Message::ReqReply {
        //             message: message.to_owned(),
        //             respond_to,
        //         };
        //     }
        //     _ => panic!("Unexpected message type"),
        // }
    }

    // fn handle_ptr(&self, _message: Arc<Message>) -> Arc<Message> {
    //     debug!("HTTPIn handle (passthrough)... {}", self.name());
    //     return _message;
    // }

    fn init(&mut self, _: &mut Graph) {
        debug!("HTTPIn Init");
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, start, .. } => {
                for n in next {
                    self.add_next(n);
                }

                self.setup_routes(start);
            }
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, message: Message) {
        self.next(self.handle(message));
    }

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }

    fn next_ptr(&self, message: Arc<Message>) {
        for n in &self.next {
            n.lock().unwrap().send_ptr(message.to_owned());
        }
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        self.next.as_ref()
    }
}

impl HTTPIn {
    pub async fn new(routes: Vec<String>, method: String) -> HTTPIn {
        HTTPIn {
            routes,
            method,
            next: Vec::new(),
        }
    }

    fn next(&self, message: Message) {
        let mut counter = 0;
        for n in &self.next {
            if counter == 0 {
                n.lock().unwrap().send(message);
                break;
            } else {
                info!("Not implemented");
            }
            counter = counter + 1;
        }
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }

    fn setup_routes(&self, start: Arc<Mutex<dyn Operator + 'static>>) {
        let mut singleton = get_instance().lock().unwrap();
        singleton.add_routes(self.routes.to_owned(), self.method.to_owned(), start);
    }
}
