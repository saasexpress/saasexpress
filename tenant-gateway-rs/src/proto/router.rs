use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

// Type definitions for Request and Response
pub struct Request {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// Handler trait - defines how to process a request
pub trait Handler: Send + Sync + Debug {
    fn handle(&self, req: &Request) -> Response;
}

// Middleware trait - can modify requests or responses
pub trait Middleware: Send + Sync + Debug {
    fn pre_process(&self, req: &mut Request);
    fn post_process(&self, req: &Request, res: &mut Response);
}

pub struct Router {
    routes: HashMap<String, Arc<dyn Handler>>,
    middleware: Vec<Arc<dyn Middleware>>,
}

// impl<S> Router<S>
// where
//     S: Clone + Send + Sync + 'static,
impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    // Add any type of handler
    pub fn add_handler<H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.routes.insert(path.to_string(), Arc::new(handler));
        self
    }

    // Add any type of middleware
    pub fn add_middleware<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware + 'static,
    {
        self.middleware.push(Arc::new(middleware));
        self
    }

    // Process request just like before
    pub fn process(&self, mut req: Request) -> Response {
        for m in &self.middleware {
            m.pre_process(&mut req);
        }

        let handler = self.routes.get(&req.path);
        let mut response = match handler {
            Some(h) => h.handle(&req),
            None => Response {
                status: 404,
                headers: HashMap::new(),
                body: b"Not Found".to_vec(),
            },
        };

        for m in self.middleware.iter().rev() {
            m.post_process(&req, &mut response);
        }

        response
    }
}
