use crate::proto::router::{Middleware, Request, Response};

// Example authentication middleware
#[derive(Debug)]
pub(crate) struct AuthMiddleware {
    pub(crate) secret_key: String,
}

impl Middleware for AuthMiddleware {
    fn pre_process(&self, req: &mut Request) {
        let auth_header = req.headers.get("Authorization");
        if auth_header.is_none() || auth_header.unwrap() != &self.secret_key {
            // Add a flag to indicate authentication failed
            req.headers
                .insert("__auth_failed".to_string(), "true".to_string());
        }
    }

    fn post_process(&self, req: &Request, res: &mut Response) {
        if req.headers.get("__auth_failed").is_some() {
            // Override response if authentication failed
            res.status = 401;
            res.body = b"Unauthorized".to_vec();
        }
    }
}
