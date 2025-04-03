use crate::proto::router::{Middleware, Request, Response};

// Example logger middleware
#[derive(Debug)]
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn pre_process(&self, req: &mut Request) {
        println!("Incoming request to: {} {}", req.method, req.path);
    }

    fn post_process(&self, req: &Request, res: &mut Response) {
        println!(
            "Response for: {} {} - Status: {}",
            req.method, req.path, res.status
        );
    }
}
