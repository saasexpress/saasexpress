// Example implementation:

// Import the HomeHandler struct

use std::collections::HashMap;

use crate::proto::{
    handlers::{home::HomeHandler, user::UserHandler},
    middleware::{auth::AuthMiddleware, logger::LoggerMiddleware},
    router::{Request, Router},
};

/*
let mut graph = Graph::new();
graph.add_node("in", BufferToJSON)
.add_node("out", JSONToBuffer)
.add_edge("in", "out")
.start();

graph.send(bytes::Bytes::from("Hello, World!"));

let mut graph = Graph::new();
graph.add_node("in", AllInOne)
.add_node("json", BufferToJSON)
.add_node("out", JSONToBuffer)
.add_edge("in", "json")
.add_edge("json", "out")
.start();

let mut graph = Graph::load_yaml(...);
graph.start();

-- create MPSC for each Node
-- OperatorNode -> Ports -> MPSC

graph.process(message);


graph.stop();
 */
// Example usage
pub fn main() {
    // Initialize handlers and middleware
    let home_handler = HomeHandler;
    let user_handler = UserHandler {
        users: {
            let mut m = HashMap::new();
            m.insert("1".to_string(), "Alice".to_string());
            m.insert("2".to_string(), "Bob".to_string());
            m
        },
    };

    let logger = LoggerMiddleware;
    let auth = AuthMiddleware {
        secret_key: "secret-token-123".to_string(),
    };

    let mut router = Router::new();
    router
        .add_handler("/", home_handler)
        .add_handler("/users/1", user_handler)
        .add_middleware(logger)
        .add_middleware(auth);

    // Simulate a request
    let request = Request {
        path: "/".to_string(),
        method: "GET".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("Authorization".to_string(), "secret-token-123".to_string());
            h
        },
        body: vec![],
    };

    // Process the request
    let response = router.process(request);

    // Output response
    println!("Status: {}", response.status);
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}
