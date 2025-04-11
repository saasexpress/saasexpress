use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use futures::channel::oneshot;
use serde_json::Value;

use tokio::sync::mpsc;

use super::graph::Operator;

#[derive(Debug)]
pub struct OriginMessage {
    pub respond_to: oneshot::Sender<Message>,
    pub session: Option<String>,
    pub mpsc_respond_to: Option<mpsc::Sender<Message>>,
}

// impl Drop for OriginMessage {
//     fn drop(&mut self) {
//         println!("Dropping OriginMessage {:?}!", self);
//     }
// }

impl OriginMessage {
    pub fn new(respond_to: oneshot::Sender<Message>) -> Self {
        OriginMessage {
            respond_to,
            session: None,
            mpsc_respond_to: None,
        }
    }
    pub fn session(mut self, session: String) -> Self {
        self.session = Some(session);
        self
    }
    pub fn mpsc_respond_to(mut self, mpsc_respond_to: mpsc::Sender<Message>) -> Self {
        self.mpsc_respond_to = Some(mpsc_respond_to);
        self
    }
}

#[derive(Debug)]
pub enum Message {
    Split {
        message: Vec<u8>,
        respond_to: Arc<mpsc::Sender<Message>>,
        origin: Option<OriginMessage>,
    },
    Exit {
        origin: Option<OriginMessage>,
    },
    Standard {
        message: Vec<u8>,
        origin: Option<OriginMessage>,
    },
    JSON {
        message: Value,
        origin: Option<OriginMessage>,
    },
    ReqReply {
        path: String,
        query: String,
        method: String,
        message: Vec<u8>,
        respond_to: oneshot::Sender<Message>,
    },
    Init {
        next: Vec<Arc<Mutex<dyn Operator>>>,
        end: Arc<Mutex<dyn Operator + 'static>>,
        start: Arc<Mutex<dyn Operator + 'static>>,
    },
    Error {
        error: String,
    },
    NoOp {},
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::NoOp { .. } => write!(f, "NoOp message"),
            Message::Exit { .. } => write!(f, "Exit message"),
            Message::Split { .. } => write!(f, "Split message"),
            Message::Standard { message, .. } => write!(f, "Standard message: {:?}", message),
            Message::JSON { message, .. } => write!(f, "JSON message: {:?}", message),
            Message::ReqReply { message, .. } => write!(f, "ReqReply message: {:?}", message),
            Message::Init { .. } => write!(f, "Init message"),
            Message::Error { .. } => write!(f, "Error message"),
        }
    }
}
