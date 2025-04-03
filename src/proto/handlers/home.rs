use crate::proto::router::Handler;
use std::collections::HashMap;

use crate::proto::router::{Middleware, Request, Response};

// Example handlers
#[derive(Debug)]
pub(crate) struct HomeHandler;

impl Handler for HomeHandler {
    fn handle(&self, _req: &Request) -> Response {
        Response {
            status: 200,
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/plain".to_string());
                h
            },
            body: b"Welcome Home!".to_vec(),
        }
    }
}
