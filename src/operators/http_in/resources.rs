use rand::{Rng, rngs::ThreadRng};
use saasexpress_core::graph::graph::Operator;
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
};

use saasexpress_core::graph::message::Message;

use crate::operators::http_in::websocket::ws_handler;
use axum::{
    Json, Router,
    body::{Bytes, to_bytes},
    extract::{ConnectInfo, Path, Request, State, WebSocketUpgrade},
    http::HeaderName,
    response::{IntoResponse, Response},
    routing::{any, delete, get, post, put},
};
use axum::{body::Body, http::HeaderValue};
use axum_extra::{TypedHeader, headers};
use futures::channel::oneshot;
use hyper::{HeaderMap, Method, StatusCode};
use serde_json::json;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct MySharedState {
    pub start: Arc<Mutex<dyn Operator + 'static>>,
    pub counter: Arc<Mutex<u32>>,
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
                counter: Arc::new(Mutex::new(0)),
            });
            let handler_for_websocket =
                async |state: State<Arc<MySharedState>>,
                       ws: WebSocketUpgrade,
                       user_agent: Option<TypedHeader<headers::UserAgent>>,
                       ConnectInfo(addr): ConnectInfo<SocketAddr>| {
                    // send message to the first operator of the flow

                    ws_handler(state, ws, user_agent, ConnectInfo(addr)).await
                };

            //let mut rng = rand::rng();
            //let counter = Arc::new(Mutex::new("0"));

            let handler = async |state: State<Arc<MySharedState>>,
                                 method: Method,
                                 request: Request| {
                let req_id;
                {
                    let mut counter = state.counter.lock().unwrap();
                    *counter += 1;
                    req_id = counter.clone();
                }
                let req_id = format!("{:0>8}", req_id);

                debug!(
                    "Handler [IN] [{}] {} {} {}",
                    req_id,
                    method,
                    request.uri().path(),
                    request
                        .headers()
                        .get("content-type")
                        .unwrap()
                        .to_str()
                        .unwrap()
                );

                let query = request
                    .uri()
                    .path_and_query()
                    .unwrap()
                    .query()
                    .unwrap_or_default()
                    .to_string();
                //debug!("Using path : {}", path);

                debug!("Path = {}", request.uri().path());
                let path = request.uri().path().to_string();

                let body = request.into_body();

                let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

                let (send, recv) = oneshot::channel();

                debug!(
                    "Body = {:?}",
                    String::from_utf8(body_bytes.to_vec()).unwrap()
                );

                let message = Message::ReqReply {
                    message: body_bytes.to_vec(),
                    respond_to: send,
                    path,
                    query,
                    method: method.to_string(),
                };

                // send message to the first operator of the flow
                state.start.lock().unwrap().send(message);

                debug!("Handler [WAIT] [{}]", req_id);

                // wait for the request to complete
                match recv.await {
                    Ok(msg) => match msg {
                        Message::Standard {
                            message,
                            origin: None,
                        } => {
                            debug!("Handler [OK] [{}]", req_id);
                            Json(json!({ "data": String::from_utf8_lossy(&message) }))
                                .into_response()
                        }
                        Message::HTTP {
                            message,
                            origin: None,
                            headers,
                            status,
                        } => {
                            debug!(
                                "Handler [OK] [{}] (status={}): {}",
                                req_id,
                                status,
                                headers.get("content-type").unwrap_or(&String::from(""))
                            );

                            //let b: &u8 = message.into_iter().take().collect();

                            //let stream = ReaderStream::new(&*message);

                            // Convert stream to axum HTTP body
                            let body = Bytes::from(message);

                            // if let Ok(body) = message {
                            let mut _headers = HeaderMap::new();
                            // _headers.insert(
                            //     "Content-Type",
                            //     HeaderValue::from_bytes(
                            //         headers.get("content-type").unwrap().as_bytes(),
                            //     )
                            //     .unwrap(),
                            // );

                            let header_keys = headers.keys().cloned().collect::<HashSet<_>>();
                            for key in header_keys {
                                let hdr_name = HeaderName::from_bytes(key.as_bytes()).unwrap();

                                _headers.insert(
                                    hdr_name,
                                    HeaderValue::from_bytes(headers.get(&key).unwrap().as_bytes())
                                        .unwrap(),
                                );
                            }

                            (StatusCode::from_u16(status).unwrap(), _headers, body).into_response()
                            // } else {
                            //     error!("Failed to convert body to string");
                            //     StatusCode::INTERNAL_SERVER_ERROR.into_response()
                            // }
                        }

                        Message::JSON {
                            message,
                            origin: None,
                        } => {
                            debug!("Handler [OK] [{}]", req_id);
                            Json(json!(message)).into_response()
                        }

                        _ => {
                            error!("Handler [PANIC] [{}]", req_id);
                            panic!("Unexpected Message Type {:?}", msg);
                        }
                    },
                    Err(e) => {
                        error!("Handler [ERROR] [{}] {:?}", req_id, e);
                        Json(json!({ "status": "Error failed to receive response" }))
                            .into_response()
                    }
                }
            };

            let path = _path;
            match method.as_str() {
                "^(POST|PUT|DELETE)$" => {
                    let shared_state1 = Arc::new(MySharedState {
                        start: start.clone(),
                        counter: Arc::new(Mutex::new(0)),
                    });
                    let shared_state2 = Arc::new(MySharedState {
                        start: start.clone(),
                        counter: Arc::new(Mutex::new(0)),
                    });
                    let shared_state3 = Arc::new(MySharedState {
                        start: start.clone(),
                        counter: Arc::new(Mutex::new(0)),
                    });
                    self.router = main_router
                        .merge(
                            Router::new()
                                .route(path.as_str(), post(handler.clone()))
                                .with_state(shared_state1),
                        )
                        .merge(
                            Router::new()
                                .route(path.as_str(), put(handler.clone()))
                                .with_state(shared_state2),
                        )
                        .merge(
                            Router::new()
                                .route(path.as_str(), delete(handler.clone()))
                                .with_state(shared_state3),
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

                    self.router = main_router.merge(
                        Router::new()
                            .route(path.to_string().as_str(), get(handler))
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
                        self.router = main_router.merge(
                            Router::new()
                                .route(path.as_str(), any(handler))
                                .with_state(shared_state),
                        );
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
            let addr = SocketAddr::from(([0, 0, 0, 0], 2243));
            let listener = TcpListener::bind(addr).await.unwrap();

            info!("[HTTPIn] Listening on: {}", addr);

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
    warn!(
        "Default fallback! {:?} {:?}",
        request.method(),
        request.uri()
    );
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
