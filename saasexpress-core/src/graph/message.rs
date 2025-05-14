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

use super::graph::{Operator, OperatorRef, OperatorRole};

#[derive(Debug)]
pub enum ControlCommand {
    SetSettings {
        settings: HashMap<String, serde_json::Value>,
    },
    Start,
    Stop,
}

#[derive(Debug)]
pub struct OriginMessageV2<T> {
    session: Option<String>,
    span: Option<DebuggableSpan>,
    context: T,
}

impl<T> OriginMessageV2<T>
where
    T: Send + Sync,
{
    pub fn new(context: T) -> Self {
        OriginMessageV2 {
            context,
            session: None,
            span: None,
        }
    }
}

#[derive(Debug)]
pub struct OriginMessage {
    pub respond_to: Option<oneshot::Sender<Message>>,
    pub session: Option<String>,
    pub mpsc_respond_to: Option<mpsc::Sender<Message>>,
    pub span: Option<DebuggableSpan>,
    pub temp: Arc<Mutex<Value>>,
}

// impl Drop for OriginMessage {
//     fn drop(&mut self) {
//         println!("Dropping OriginMessage {:?}!", self);
//     }
// }

impl OriginMessage {
    pub fn new(respond_to: Option<oneshot::Sender<Message>>) -> Self {
        OriginMessage {
            respond_to,
            session: None,
            mpsc_respond_to: None,
            span: None,
            temp: Arc::new(Mutex::new(Value::Null)),
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

    pub fn with_temp(mut self, temp: Arc<Mutex<Value>>) -> Self {
        self.temp = temp;
        self
    }

    pub fn temp_push(self, key: String, data: Value) -> Self {
        match self.temp.try_lock() {
            Ok(mut temp) => {
                if temp.is_null() {
                    *temp = serde_json::json!({});
                }
                if let Some(obj) = temp.as_object_mut() {
                    obj.insert(key, data);
                }
            }
            Err(_) => {
                error!("Failed to lock temp mutex");
            }
        }
        self
    }

    pub fn copy_from(mut self, other: OriginMessage) -> Self {
        self.session = other.session;
        self.span = other.span;
        self.temp = other.temp;
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
        //origin_v2: Option<OriginMessageV2<mpsc::Sender<Message>>>,
    },
    ReqReply {
        message: Vec<u8>,
        respond_to: oneshot::Sender<Message>,
        temp: Arc<Mutex<Value>>,
        span: Option<DebuggableSpan>,
    },
    Tuple {
        message_1: Box<Message>,
        message_2: Box<Message>,
        origin: Option<OriginMessage>,
    },
    Control {
        command: ControlCommand,
        origin: Option<OriginMessage>,
    },
    Init {
        id: String,
        next: Vec<OperatorRole>,
        end: OperatorRef,
        start: OperatorRef,
    },
    Error {
        error: String,
        origin: Option<OriginMessage>,
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
            Message::Tuple { message_1, .. } => message_1.take_origin(),
            _ => None,
        }
    }

    pub fn get_origin(&mut self) -> Option<&OriginMessage> {
        match self {
            Message::Standard { origin, .. } => origin.as_ref(),
            Message::JSON { origin, .. } => origin.as_ref(),
            Message::HTTP { origin, .. } => origin.as_ref(),
            Message::Tuple { message_1, .. } => message_1.get_origin(),
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
            Message::Exit { origin, .. } => {
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
            Message::Exit { .. } => {
                match self {
                    Message::Exit { ref mut origin, .. } => {
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
            Message::Standard { .. } => {
                match self {
                    Message::Standard { ref mut origin, .. } => {
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
                error!("Unable to set span for {} message", self);
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
            Message::Control { command, .. } => write!(f, "Control message {:?}", command),
            Message::Tuple {
                message_1,
                message_2,
                ..
            } => {
                write!(f, "Tuple message: {:?} {:?}", message_1, message_2)
            }
            Message::Error { .. } => write!(f, "Error message"),
        }
    }
}
