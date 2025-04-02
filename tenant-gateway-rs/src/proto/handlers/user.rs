use std::collections::HashMap;

use crate::proto::router::Handler;
use crate::proto::router::{Middleware, Request, Response};

#[derive(Debug)]
pub struct UserHandler {
    pub users: HashMap<String, String>,
}

impl Handler for UserHandler {
    fn handle(&self, req: &Request) -> Response {
        // Check for user ID in path or query params (simplified)
        let user_id = req.path.split('/').nth(2).unwrap_or("");

        match self.users.get(user_id) {
            Some(name) => Response {
                status: 200,
                headers: {
                    let mut h = HashMap::new();
                    h.insert("Content-Type".to_string(), "text/plain".to_string());
                    h
                },
                body: format!("User: {}", name).into_bytes(),
            },
            None => Response {
                status: 404,
                headers: HashMap::new(),
                body: b"User not found".to_vec(),
            },
        }
    }
}
