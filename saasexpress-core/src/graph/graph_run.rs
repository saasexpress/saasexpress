use std::sync::{Arc, Mutex};

use fastrace::prelude::SpanContext;
use futures::channel::oneshot;
use serde_json::{Value, json};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::graph::{Graph, GraphRunner};
use super::message::{DebuggableSpan, Message, OriginMessage};

pub trait GraphRun {
    async fn end_to_end(&self, message: Vec<u8>) -> Message;
    async fn end_to_end_json(&self, message: Value) -> Message;
    async fn end_to_end_standard(&self, message: Vec<u8>) -> Message;

    async fn end_to_end_2(&self, message: Vec<u8>) -> Message;

    //async fn process(&mut self, message: Message) -> Message;
}

impl GraphRun for GraphRunner {
    async fn end_to_end(&self, message: Vec<u8>) -> Message {
        //let node = self.nodes.get(&self.start_node).unwrap().clone();

        //let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let temp = json!({
            "path": "".to_string(),
            "method": "".to_string(),
            "query": "".to_string(),
        });

        //let runner = self.runner.clone();

        self.call(Message::ReqReply {
            message,
            respond_to,
            temp: Arc::new(Mutex::new(temp)),
            span: Some(DebuggableSpan(root_span)),
        });

        recv.await.unwrap()
    }

    async fn end_to_end_json(&self, message: Value) -> Message {
        // let node = self.start_node();

        // let node = node.lock().unwrap();

        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let origin = OriginMessage::new(Some(respond_to))
            .session("0".to_string())
            .with_span(Some(DebuggableSpan(root_span)));

        self.call(Message::JSON {
            message,
            origin: Some(origin),
        });

        match recv.await {
            Ok(message) => message,
            Err(_) => {
                error!("Failed to receive message");
                Message::Error {
                    error: "Failed to receive message".to_string(),
                    origin: None,
                }
            }
        }
    }

    async fn end_to_end_standard(&self, message: Vec<u8>) -> Message {
        let (respond_to, recv) = oneshot::channel();

        let root_span = fastrace::Span::root("end_to_end", SpanContext::random());

        let origin = OriginMessage::new(Some(respond_to))
            .session("0".to_string())
            .with_span(Some(DebuggableSpan(root_span)));

        self.call(Message::Standard {
            message,
            origin: Some(origin),
        });

        recv.await.unwrap()
    }

    async fn end_to_end_2(&self, message: Vec<u8>) -> Message {
        let (_tx, _rx) = oneshot::channel();

        let (tx, mut rx) = mpsc::channel(10);

        self.call(Message::Standard {
            message,
            origin: Some(
                OriginMessage::new(Some(_tx))
                    .session("0".to_string())
                    .mpsc_respond_to(tx),
            ),
        });

        let mut lines = Vec::new();

        while let Some(message) = rx.recv().await {
            debug!("Message: {:?}", message);

            match message {
                Message::Standard { message, .. } => {
                    debug!("Message: {:?}", message);
                    let msg = String::from_utf8(message).unwrap();

                    let json: Value = serde_json::from_str(&msg).unwrap();

                    lines.push(json);
                }
                Message::Exit { origin } => {
                    debug!("Exit: {:?}", origin);
                    break;
                }
                _ => {}
            }
        }

        info!("DONE");
        //recv.await.unwrap()
        Message::JSON {
            message: serde_json::Value::Array(lines),
            origin: None,
        }
    }

    // // Process request just like before
    // async fn process(&mut self, message: Message) -> Message {
    //     self.call(message);

    //     //let p = graph.processor.unwrap().lock().unwrap();

    //     //let processor = graph.processor.as_mut().unwrap();
    //     let graph = self;
    //     let a = graph.processor.as_mut().unwrap();
    //     let mut b = a.lock().unwrap();
    //     let message = b.req_reply().await;
    //     //return processor.req_reply().await;
    //     message
    // }
}
