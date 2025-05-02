use fastrace::{Event, Span, local::LocalSpan, prelude::SpanContext};
use opentelemetry::{
    Context, InstrumentationScope, KeyValue,
    global::{self, ObjectSafeTracerProvider},
    propagation::{Extractor, Injector},
    trace::SpanKind,
};
use saasexpress_core::graph::{graph::Operator, message::DebuggableSpan};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
    thread::sleep,
};
use tonic::metadata::MetadataMap;

use saasexpress_core::graph::message::Message;

use crate::operators::http_in::websocket::ws_handler;
use axum::http::HeaderValue;
use axum::{
    Json, Router,
    body::{Bytes, to_bytes},
    extract::{ConnectInfo, Request, State, WebSocketUpgrade},
    http::HeaderName,
    response::IntoResponse,
    routing::{any, delete, get, post, put},
};
use axum_extra::{TypedHeader, headers};
use futures::channel::oneshot;
use hyper::{HeaderMap, Method, StatusCode};
use opentelemetry::trace::Tracer;
use serde_json::json;
use std::fmt::Debug;
use tokio::net::TcpListener;
use tracing::Instrument;
use tracing::span;
use tracing::{debug, error, info, instrument, warn};
//use tracing_opentelemetry::OpenTelemetrySpanExt;
use fastrace::future::FutureExt;

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
        let start = _start;

        for _path in paths.iter() {
            let path = _path.clone();
            debug!("Configuring path : {}", path);

            let main_router = self.router.clone();

            let handler_for_websocket =
                async |state: State<Arc<MySharedState>>,
                       ws: WebSocketUpgrade,
                       user_agent: Option<TypedHeader<headers::UserAgent>>,
                       ConnectInfo(addr): ConnectInfo<SocketAddr>| {
                    // send message to the first operator of the flow

                    let req_id;
                    {
                        let mut counter = state.counter.lock().unwrap();
                        *counter += 1;
                        req_id = counter.clone();
                    }
                    let req_id = format!("{:0>8}", req_id);

                    let root = Span::root(format!("http_in_request_ws"), SpanContext::random())
                        .with_property(|| ("http.request_id", req_id.clone()));

                    root.add_event(Event::new("Request received".to_string()));

                    let root_span = Span::enter_with_parent("response_span", &root);

                    ws_handler(state, ws, user_agent, ConnectInfo(addr), root_span).await
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

                let root = Span::root(format!("http_in_request {}", method), SpanContext::random())
                    .with_property(|| ("http.request_id", req_id.clone()))
                    .with_property(|| ("http.method", method.to_string()))
                    .with_property(|| ("http.target", request.uri().path().to_string()));

                root.add_event(Event::new("Request received".to_string()));

                let root_span = Span::enter_with_parent("response_span", &root);

                //let _local_root_guard = root_span.set_local_parent();

                debug!(
                    "Handler [IN] [{}] {} {}",
                    req_id,
                    method,
                    request.uri().path(),
                );

                let query = request
                    .uri()
                    .path_and_query()
                    .unwrap()
                    .query()
                    .unwrap_or_default()
                    .to_string();

                debug!("Path = {}", request.uri().path());
                let path = request.uri().path().to_string();

                // Create a span for parsing the request body
                // let parse_span = tracing::info_span!("parse_request", %req_id);
                // let _parse_guard = parse_span.enter();

                let body = request.into_body();
                let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

                if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
                    if !body_str.is_empty() {
                        debug!("Body = {}", body_str);
                    }
                }

                //drop(_parse_guard); // Exit parse span

                let (send, recv) = oneshot::channel();

                //let ping_span = tracing::info_span!("ping span");

                //let cx = Context::current_with_span(span);

                //let propctx = PropagationContext::inject(&trace_ctx);

                // Create a span for operator processing
                // let process_span = tracing::info_span!("process_request",
                //     %req_id,
                //     %path,
                //     operation = "forward_to_operator"
                // );
                // let _process_guard = process_span.enter();

                // send message to the first operator of the flow

                debug!("Handler [WAIT] [{}]", req_id);
                //drop(_process_guard); // Exit process span

                // drop(_root_guard);

                //let req_reply_span = Span::enter_with_local_parent("req_reply_span");

                let message = Message::ReqReply {
                    message: body_bytes.to_vec(),
                    respond_to: send,
                    path: path.clone(),
                    query: query.clone(),
                    method: method.to_string(),
                    span: Some(DebuggableSpan(root_span)),
                };

                state.start.lock().unwrap().send(message);

                //let __span__ = Span::enter_with_local_parent("simple_async");

                //let _guard = root.set_local_parent();

                let msg = recv.await;

                debug!("Handler [RECV] [{:?}]", msg);
                //LocalSpan::add_event(Event::new("event in span1"));

                match msg {
                    Ok(msg) => {
                        // Create a span for creating the response
                        // let format_response_span = tracing::info_span!("format_response", %req_id);
                        // let _format_response_guard = format_response_span.enter();

                        let response = match msg {
                            Message::Standard {
                                message,
                                origin: None,
                            } => {
                                debug!("Handler (Standard) [OK] [{}]", req_id);
                                // Record success in the span
                                //Span::current().record("http.status_code", 200);
                                //Span::current().record("otel.status_code", "OK");

                                //drop(_root_guard);

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

                                // // Record HTTP status in the span
                                // Span::current().record("http.status_code", status);
                                // if status >= 200 && status < 300 {
                                //     Span::current().record("otel.status_code", "OK");
                                // } else {
                                //     Span::current().record("otel.status_code", "ERROR");
                                // }

                                //drop(_root_guard);

                                // Convert stream to axum HTTP body
                                let body = Bytes::from(message);
                                let mut _headers = HeaderMap::new();

                                let header_keys = headers.keys().cloned().collect::<HashSet<_>>();
                                for key in header_keys {
                                    let hdr_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
                                    _headers.insert(
                                        hdr_name,
                                        HeaderValue::from_bytes(
                                            headers.get(&key).unwrap().as_bytes(),
                                        )
                                        .unwrap(),
                                    );
                                }

                                (StatusCode::from_u16(status).unwrap(), _headers, body)
                                    .into_response()
                            }

                            Message::JSON { message, origin } => {
                                debug!("Handler (JSON) [OK] [{}]", req_id);

                                LocalSpan::add_event(Event::new("Request OK".to_string()));

                                Json(json!(message)).into_response()
                            }

                            _ => {
                                error!("Handler [PANIC] [{}]", req_id);
                                // Record error in the span
                                // Span::current().record("http.status_code", 500);
                                // Span::current().record("otel.status_code", "ERROR");
                                // Span::current().record("error", true);
                                // Span::current().record("error.message", "Unexpected message type");

                                //drop(_root_guard);

                                LocalSpan::add_event(Event::new("Error"));

                                panic!("Unexpected Message Type {:?}", msg);
                            }
                        };

                        // Return the response
                        response
                    }
                    Err(e) => {
                        error!("Handler [ERROR] [{}] {:?}", req_id, e);
                        // Record error in the span
                        // Span::current().record("http.status_code", 500);
                        // Span::current().record("otel.status_code", "ERROR");
                        // Span::current().record("error", true);
                        // Span::current().record("error.message", "Failed to receive response");

                        LocalSpan::add_event(Event::new("Error"));

                        Json(json!({ "status": "Error failed to receive response" }))
                            .into_response()
                    }
                }
            };

            let shared_state = Arc::new(MySharedState {
                start: start.clone(),
                counter: Arc::new(Mutex::new(0)),
            });

            let path = _path;
            match method.as_str() {
                "^(POST|PUT|DELETE)$" => {
                    self.router = main_router
                        .merge(
                            Router::new()
                                .route(path.as_str(), post(handler.clone()))
                                .with_state(shared_state.clone()),
                        )
                        .merge(
                            Router::new()
                                .route(path.as_str(), put(handler.clone()))
                                .with_state(shared_state.clone()),
                        )
                        .merge(
                            Router::new()
                                .route(path.as_str(), delete(handler.clone()))
                                .with_state(shared_state.clone()),
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

    //#[instrument(name = "http_server_start", skip_all)]
    pub fn start(&self) {
        let router = self.router.to_owned();

        let service = router.into_make_service_with_connect_info::<SocketAddr>();
        tokio::spawn(async move {
            let addr = SocketAddr::from(([0, 0, 0, 0], 2243));

            info!("[HTTPIn.axum] Binding to address: {}", addr);
            let listener = TcpListener::bind(addr).await.unwrap();

            let root = Span::root("server_up", SpanContext::random());

            root.with_property(|| ("server.address", "0.0.0.0"))
                .with_property(|| ("server.port", "2243"))
                .add_event(Event::new("Server started".to_string()));

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
