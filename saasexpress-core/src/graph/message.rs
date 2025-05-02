use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};

use fastrace::Span;

// Wrapper for Span to implement Debug manually
//#[derive(Debug)]
pub struct DebuggableSpan(pub Span);

impl std::fmt::Debug for DebuggableSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span(Debug not implemented)")
    }
}
use futures::channel::oneshot;
use serde_json::Value;

use tokio::sync::mpsc;
use tracing::error;

use super::graph::Operator;

#[derive(Debug)]
pub struct OriginMessage {
    pub respond_to: oneshot::Sender<Message>,
    pub session: Option<String>,
    pub mpsc_respond_to: Option<mpsc::Sender<Message>>,
    pub span: Option<DebuggableSpan>,
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
            span: None,
        }
    }
    pub fn with_span(mut self, span: Option<DebuggableSpan>) -> Self {
        if let Some(span) = span {
            self.span = Some(span);
        }
        self
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
    HTTP {
        message: Vec<u8>,
        status: u16,
        headers: HashMap<String, String>,
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
        span: Option<DebuggableSpan>,
    },
    Init {
        id: String,
        next: Vec<Arc<Mutex<dyn Operator>>>,
        end: Arc<Mutex<dyn Operator + 'static>>,
        start: Arc<Mutex<dyn Operator + 'static>>,
    },
    Error {
        error: String,
    },
    NoOp {},
}

impl Message {
    pub fn with_origin(mut self, new_origin: Option<OriginMessage>) -> Self {
        match self {
            Message::Standard { .. } => {
                if let Message::Standard { ref mut origin, .. } = self {
                    *origin = new_origin;
                }
            }
            Message::JSON { .. } => {
                if let Message::JSON { ref mut origin, .. } = self {
                    *origin = new_origin;
                }
            }
            Message::HTTP { .. } => {
                if let Message::HTTP { ref mut origin, .. } = self {
                    *origin = new_origin;
                }
            }
            _ => {}
        }
        self
    }

    pub fn take_origin(&mut self) -> Option<OriginMessage> {
        match self {
            Message::Standard { origin, .. } => origin.take(),
            Message::JSON { origin, .. } => origin.take(),
            Message::HTTP { origin, .. } => origin.take(),
            _ => None,
        }
    }

    pub fn get_origin(&mut self) -> Option<&OriginMessage> {
        match self {
            Message::Standard { origin, .. } => origin.as_ref(),
            Message::JSON { origin, .. } => origin.as_ref(),
            Message::HTTP { origin, .. } => origin.as_ref(),
            _ => None,
        }
    }

    pub fn get_span(&self) -> Option<&Span> {
        match self {
            Message::ReqReply { span, .. } => span.as_ref().map(|s| &s.0),
            Message::JSON { origin, .. } => {
                if let Some(origin) = origin {
                    origin.span.as_ref().map(|s| &s.0)
                } else {
                    None
                }
            }
            Message::HTTP { origin, .. } => {
                if let Some(origin) = origin {
                    origin.span.as_ref().map(|s| &s.0)
                } else {
                    error!("HTTP Message without origin {:?}", self);

                    None
                }
            }
            Message::Standard { origin, .. } => {
                if let Some(origin) = origin {
                    origin.span.as_ref().map(|s| &s.0)
                } else {
                    None
                }
            }
            _ => {
                error!("No span for {} message", self);
                None
            }
        }
    }

    pub fn with_span(mut self, og_span: Span) -> Self {
        match self {
            Message::ReqReply { ref mut span, .. } => *span = Some(DebuggableSpan(og_span)),
            Message::JSON { .. } => {
                match self {
                    Message::JSON { ref mut origin, .. } => {
                        *origin = Some(
                            origin
                                .take()
                                .unwrap()
                                .with_span(Some(DebuggableSpan(og_span))),
                        );
                    }
                    _ => {}
                };
            }
            Message::HTTP { .. } => {
                match self {
                    Message::HTTP { ref mut origin, .. } => {
                        *origin = Some(
                            origin
                                .take()
                                .unwrap()
                                .with_span(Some(DebuggableSpan(og_span))),
                        );
                    }
                    _ => {}
                };
            }
            _ => {
                error!("No span for {} message", self);
            }
        }
        self
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::NoOp { .. } => write!(f, "NoOp message"),
            Message::Exit { .. } => write!(f, "Exit message"),
            Message::Split { .. } => write!(f, "Split message"),
            Message::Standard { message, .. } => write!(f, "Standard message: {:?}", message),
            Message::JSON { message, .. } => write!(f, "JSON message: {:?}", message),
            Message::HTTP { message, .. } => write!(f, "HTTP message: {:?}", message),
            Message::ReqReply { message, .. } => write!(f, "ReqReply message: {:?}", message),
            Message::Init { .. } => write!(f, "Init message"),
            Message::Error { .. } => write!(f, "Error message"),
        }
    }
}
