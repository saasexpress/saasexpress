use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
};

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::{ConnectInfo, Request, State, WebSocketUpgrade, path},
    response::IntoResponse,
    routing::{any, delete, get, post, put},
};
use axum_extra::{TypedHeader, headers};
use futures::channel::oneshot;
use hyper::{Method, StatusCode};
use serde_json::json;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

use crate::graph::{
    graph::{Message, Operator},
    operators::http_in::websocket::ws_handler,
};

#[derive(Debug)]
pub struct MySharedState {
    pub start: Arc<Mutex<dyn Operator + 'static>>,
}

pub struct Singleton {
    router: Router,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            router: Router::new(),
        }
    }

    pub(super) fn add_routes(
        &mut self,
        paths: Vec<String>,
        method: String,
        ws: bool,
        _start: Arc<Mutex<dyn Operator + 'static>>,
    ) {
        for _path in paths.iter() {
            let path = _path.clone();
            debug!("Configuring path : {}", path);
            let start = _start.to_owned();

            let main_router = self.router.clone();

            let shared_state = Arc::new(MySharedState {
                start: start.clone(),
            });
            let handler_for_websocket =
                async |state: State<Arc<MySharedState>>,
                       ws: WebSocketUpgrade,
                       user_agent: Option<TypedHeader<headers::UserAgent>>,
                       ConnectInfo(addr): ConnectInfo<SocketAddr>| {
                    // send message to the first operator of the flow

                    ws_handler(state, ws, user_agent, ConnectInfo(addr)).await
                };

            let handler =
                async |state: State<Arc<MySharedState>>, method: Method, request: Request| {
                    debug!("Received request with body");

                    let query = request
                        .uri()
                        .path_and_query()
                        .unwrap()
                        .query()
                        .unwrap_or_default()
                        .to_string();
                    debug!("Using path : {}", path);

                    let body = request.into_body();
                    let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

                    let (send, recv) = oneshot::channel();

                    let message = Message::ReqReply {
                        message: body_bytes.to_vec(),
                        respond_to: send,
                        path,
                        query,
                        method: method.to_string(),
                    };

                    // send message to the first operator of the flow
                    state.start.lock().unwrap().send(message);

                    // wait for the request to complete
                    match recv.await {
                        Ok(msg) => match msg {
                            Message::Standard {
                                message,
                                origin: None,
                            } => {
                                debug!(
                                    "Received a Standard message: {:?}",
                                    String::from_utf8_lossy(&message)
                                );
                                Json(json!({ "data": String::from_utf8_lossy(&message) }))
                            }
                            Message::JSON {
                                message,
                                origin: None,
                            } => {
                                debug!("Received a JSON message");

                                Json(json!({ "data": message }))
                            }

                            _ => panic!("Expected a Standard response"),
                        },
                        Err(e) => {
                            error!("Failed to send: {}", e);
                            Json(json!({ "status": "Error failed to receive response" }))
                        }
                    }
                };
            let path = _path;
            match method.as_str() {
                "^(POST|PUT|DELETE)$" => {
                    let shared_state1 = Arc::new(MySharedState {
                        start: start.clone(),
                    });
                    let shared_state2 = Arc::new(MySharedState {
                        start: start.clone(),
                    });

                    self.router = main_router
                        .nest(
                            path,
                            Router::new()
                                .route("/".to_string().as_str(), post(handler.clone()))
                                .with_state(shared_state1),
                        )
                        .nest(
                            path,
                            Router::new()
                                .route("/".to_string().as_str(), delete(handler.clone()))
                                .with_state(shared_state2),
                        );
                }
                "PUT" => {
                    debug!("Adding POST route: {}", path);
                    self.router = main_router.nest(
                        path,
                        Router::new()
                            .route("/".to_string().as_str(), put(handler))
                            .with_state(shared_state),
                    );
                }
                "POST" | "^(DELETE|POST)$" => {
                    debug!("Adding POST route: {}", path);
                    self.router = main_router.nest(
                        path,
                        Router::new()
                            .route("/".to_string().as_str(), post(handler))
                            .with_state(shared_state),
                    );
                }
                "GET" | "^(GET)$" => {
                    debug!("Adding GET route: {}", path);

                    self.router = main_router.nest(
                        path,
                        Router::new()
                            .route("/".to_string().as_str(), get(handler))
                            .with_state(shared_state),
                    );
                }
                "DELETE" => {
                    debug!("Adding GET route: {}", path);
                    self.router = main_router.nest(
                        path,
                        Router::new()
                            .route("/".to_string().as_str(), delete(handler))
                            .with_state(shared_state),
                    );
                }
                "^(DELETE|POST)$" => {
                    self.router = main_router.nest(
                        path,
                        Router::new()
                            .route("/".to_string().as_str(), delete(handler))
                            .with_state(shared_state),
                    );
                }
                "^(ANY)$" => {
                    info!("Adding ANY route: {}", path);
                    if ws {
                        self.router = main_router.nest(
                            path,
                            Router::new()
                                .route("/".to_string().as_str(), any(handler_for_websocket))
                                .with_state(shared_state),
                        );
                    } else {
                        panic!("ANY can only be set when ws is true");
                    }
                }
                _ => {
                    panic!("Unsupported HTTP method: {}", method);
                }
            }
        }
        let main_router = self.router.clone();
        self.router = main_router
            .fallback(default_fallback)
            .method_not_allowed_fallback(handle_405);
    }

    pub fn start(&self) {
        let router = self.router.to_owned();

        let service = router.into_make_service_with_connect_info::<SocketAddr>();
        tokio::spawn(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], 2500));
            let listener = TcpListener::bind(addr).await.unwrap();

            debug!("[HTTPIn] Listening on: {}", addr);

            let serve = axum::serve(listener, service);

            serve.await.expect("Failed to start server");
        });
    }
}

static INSTANCE: OnceLock<Mutex<Singleton>> = OnceLock::new();

pub fn get_instance() -> &'static Mutex<Singleton> {
    INSTANCE.get_or_init(|| Mutex::new(Singleton::new()))
}

async fn default_fallback(request: Request) -> impl IntoResponse {
    warn!("Default fallback! {:?}", request);
    (
        StatusCode::BAD_REQUEST,
        format!(
            "[HTTPIn] No route for [{}] {}",
            request.method(),
            request.uri()
        ),
    )
        .into_response()
}

async fn handle_405(request: Request) -> impl IntoResponse {
    warn!("No route detected! {:?}", request);
    (
        StatusCode::METHOD_NOT_ALLOWED,
        "[HTTPIn] Method not allowed fallback".to_string(),
    )
        .into_response()
}
