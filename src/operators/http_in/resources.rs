use fastrace::{Event, Span, local::LocalSpan, prelude::SpanContext};
use opentelemetry::{
    Context, InstrumentationScope, KeyValue,
    global::{self, ObjectSafeTracerProvider},
    propagation::{Extractor, Injector},
    trace::SpanKind,
};
use saasexpress_core::{
    graph::{
        graph::Operator,
        message::{DebuggableSpan, OriginMessage},
    },
    timestamp::now,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
    thread::sleep,
    time::Duration,
};
use tokio_stream::StreamExt as _;
use tonic::{IntoStreamingRequest, metadata::MetadataMap};

use axum::response::sse::Event as SseEvent;
use axum_extra::extract::cookie::CookieJar;
use saasexpress_core::graph::message::Message;

use crate::operators::http_in::{cookies::set_cookies, sse::sse_start, websocket::ws_handler};
use axum::{
    Json, Router,
    body::{Bytes, to_bytes},
    extract::{ConnectInfo, Request, State, WebSocketUpgrade},
    http::HeaderName,
    response::{IntoResponse, Sse},
    routing::{any, delete, get, post, put},
};
use axum::{extract::Path, http::HeaderValue};
use axum_extra::{
    TypedHeader,
    headers::{self},
};

use axum_extra::extract::cookie::Cookie;

use futures::{channel::oneshot, stream};
use hyper::{HeaderMap, Method, StatusCode};
use opentelemetry::trace::Tracer;
use serde_json::json;
use std::fmt::Debug;
use tokio::net::TcpListener;
use tracing::Instrument;
use tracing::span;
use tracing::{debug, error, info, instrument, warn};
//use tracing_opentelemetry::OpenTelemetrySpanExt;
use axum::extract::FromRequest;
use fastrace::future::FutureExt;

use futures::stream::Stream;
use std::{convert::Infallible, path::PathBuf};
use tokio_stream::StreamExt as _;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use saasexpress_core::timestamp::NaiveDateTimeExt;

#[derive(Debug)]
pub struct MySharedState {
    sse: bool,
    pub start: Arc<Mutex<dyn Operator + 'static>>,
    pub counter: Arc<Mutex<u32>>,
}

impl MySharedState {
    fn next_counter(&self) -> String {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        format!("{:0>8}", *counter)
    }
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
        sse: bool,
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

                    let req_id = state.next_counter();

                    let root = Span::root(format!("http_in_request_ws"), SpanContext::random())
                        .with_property(|| ("http.request_id", req_id.clone()));

                    root.add_event(Event::new("Request received".to_string()));

                    let root_span = Span::enter_with_parent("response_span", &root);

                    ws_handler(state, ws, user_agent, ConnectInfo(addr), root_span).await
                };

            //let mut rng = rand::rng();
            //let counter = Arc::new(Mutex::new("0"));

            let handler_sse = async |state: State<Arc<MySharedState>>,
                                     user_agent: Option<TypedHeader<headers::UserAgent>>,
                                     method: Method,
                                     ConnectInfo(addr): ConnectInfo<SocketAddr>,
                                     Path(params_map): Path<HashMap<String, String>>,
                                     cookie_jar: CookieJar,
                                     request: Request| {
                let req_id = state.next_counter();

                let root = Span::root(format!("http_in_request {}", method), SpanContext::random())
                    .with_property(|| ("http.request_id", req_id.clone()))
                    .with_property(|| ("http.method", method.to_string()))
                    .with_property(|| ("http.target", request.uri().path().to_string()));

                root.add_event(Event::new("Request received".to_string()));

                let root_span = Span::enter_with_parent("response_span", &root);

                debug!(
                    "Handler [IN] [{}] {} {}",
                    req_id,
                    method,
                    request.uri().path(),
                );

                //let path_query = request.uri().path_and_query().unwrap();

                let path = request.uri().path().to_string();

                let query = {
                    request
                        .uri()
                        .path_and_query()
                        .unwrap()
                        .query()
                        .unwrap_or_default()
                        .to_string()
                };

                // params_vec(request).await;
                // let params = {Path::<HashMap<String, String>>::from_request(request, &state)
                //     .await
                //     .unwrap();

                let body = request.into_body();

                let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

                if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
                    if !body_str.is_empty() {
                        debug!("Body = {}", body_str);
                    }
                }

                debug!("Handler [WAIT] [{}]", req_id);

                let query_tuples =
                    serde_html_form::from_str::<Vec<(String, String)>>(&query).unwrap();

                let query_map = query_tuples
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect::<HashMap<String, String>>();

                let cookies = cookie_jar
                    .iter()
                    .map(|cookie| (cookie.name(), cookie.value()))
                    .collect::<Vec<_>>();

                let temp_key = "http_in".to_string();

                let temp = json!({
                    temp_key: {
                        "query_string": query,
                        "query": query_map,
                        "cookies": cookies,
                        "path": path,
                        "params": params_map,
                        "method": method.to_string()
                    }
                });

                let origin = Some(
                    OriginMessage::new(None)
                        .session(req_id)
                        .with_span(Some(DebuggableSpan(root_span)))
                        .with_temp(Arc::new(Mutex::new(temp))),
                );
                let message = Message::Standard {
                    message: body_bytes.to_vec(),
                    origin,
                };

                let stream = sse_start(state, message);

                Sse::new(
                    stream.map(|value| {
                        Ok::<_, Infallible>(SseEvent::default().data(value.to_string()))
                    }),
                )
                .keep_alive(
                    axum::response::sse::KeepAlive::new()
                        .interval(Duration::from_secs(10))
                        .text("keep-alive-text"),
                )
                .into_response()
            };

            let handler = async |state: State<Arc<MySharedState>>,
                                 user_agent: Option<TypedHeader<headers::UserAgent>>,
                                 method: Method,
                                 ConnectInfo(addr): ConnectInfo<SocketAddr>,
                                 Path(params_map): Path<HashMap<String, String>>,
                                 cookie_jar: CookieJar,
                                 request: Request| {
                let req_id = state.next_counter();

                let root = Span::root(format!("http_in_request {}", method), SpanContext::random())
                    .with_property(|| ("http.request_id", req_id.clone()))
                    .with_property(|| ("http.method", method.to_string()))
                    .with_property(|| ("http.target", request.uri().path().to_string()));

                root.add_event(Event::new("Request received".to_string()));

                let root_span = Span::enter_with_parent("response_span", &root);

                debug!(
                    "Handler [IN] [{}] {} {}",
                    req_id,
                    method,
                    request.uri().path(),
                );

                // Perform automatic upgrade to WebSocket if the request is a WebSocket upgrade
                let is_upgrade = request.headers().get("upgrade").is_some();
                if is_upgrade {
                    info!("WebSocket upgrade detected");
                    let ws_upgrade = WebSocketUpgrade::from_request(request, &state)
                        .await
                        .unwrap();

                    return ws_handler(state, ws_upgrade, user_agent, ConnectInfo(addr), root_span)
                        .await
                        .into_response();
                }

                //let path_query = request.uri().path_and_query().unwrap();

                let path = request.uri().path().to_string();

                let query = {
                    request
                        .uri()
                        .path_and_query()
                        .unwrap()
                        .query()
                        .unwrap_or_default()
                        .to_string()
                };

                let body = request.into_body();
                let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

                if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
                    if !body_str.is_empty() {
                        debug!("Body = {}", body_str);
                    }
                }

                let (send, recv) = oneshot::channel();

                debug!("Handler [WAIT] [{}]", req_id);

                let query_tuples =
                    serde_html_form::from_str::<Vec<(String, String)>>(&query).unwrap();

                let query_map = query_tuples
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect::<HashMap<String, String>>();

                let cookies = cookie_jar
                    .iter()
                    .map(|cookie| (cookie.name(), cookie.value()))
                    .collect::<Vec<_>>();

                let temp_key = "http_in".to_string();

                let temp = json!({
                    temp_key: {
                        "query_string": query,
                        "query": query_map,
                        "cookies": cookies,
                        "path": path,
                        "params": params_map,
                        "method": method.to_string()
                    }
                });

                let temp = Arc::new(Mutex::new(temp));

                let later_temp = temp.clone();

                let message = Message::ReqReply {
                    message: body_bytes.to_vec(),
                    respond_to: send,
                    temp,
                    span: Some(DebuggableSpan(root_span)),
                };

                state.start.lock().unwrap().send(message);

                let msg = recv.await;

                match msg {
                    Ok(msg) => {
                        let response = match msg {
                            Message::Standard {
                                message,
                                origin: None,
                            } => {
                                debug!("Handler (Standard) [OK] [{}]", req_id);

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

                                // Causes issues because we are not returning chunks
                                _headers.remove("transfer-encoding");

                                (StatusCode::from_u16(status).unwrap(), _headers, body)
                                    .into_response()
                            }

                            Message::JSON { message, .. } => {
                                debug!("Handler (JSON) [OK] [{}]", req_id);

                                LocalSpan::add_event(Event::new("Request OK".to_string()));

                                let headers = set_cookies(later_temp);

                                (headers, Json(json!(message))).into_response()
                            }

                            Message::Tuple {
                                message_1,
                                message_2,
                                ..
                            } => {
                                debug!("Handler (JSON) [OK] [{}]", req_id);

                                LocalSpan::add_event(Event::new("Request OK".to_string()));

                                Json(json!(format!("{:?} {:?}", message_1, message_2)))
                                    .into_response()
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

            // let handler_optional_upgrade =
            //     async |state: State<Arc<MySharedState>>,
            //            ws: WebSocketUpgrade,
            //            user_agent: Option<TypedHeader<headers::UserAgent>>,
            //            method: Method,
            //            ConnectInfo(addr): ConnectInfo<SocketAddr>,
            //            request: Request| {
            //         let is_upgrade = request.headers().get("upgrade").is_some();
            //         if is_upgrade {
            //             return handler_for_websocket(state, ws, user_agent, ConnectInfo(addr))
            //                 .await;
            //         } else {
            //             return handler(state, method, request).await;
            //         }
            //     };

            let shared_state = Arc::new(MySharedState {
                sse,
                start: start.clone(),
                counter: Arc::new(Mutex::new(0)),
            });

            //let handler = if sse { handler_sse } else { handler };

            let path = _path;
            match method.as_str() {
                "^(PUT|DELETE)$" => {
                    self.router = main_router
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
                    self.router = main_router.merge(
                        Router::new()
                            .route(path.as_str(), put(handler.clone()))
                            .with_state(shared_state),
                    );
                }
                "POST" | "^(DELETE|POST)$" => {
                    debug!("Adding POST route: {}", path);
                    self.router = main_router.merge(
                        Router::new()
                            .route(path.as_str(), post(handler.clone()))
                            .with_state(shared_state.clone()),
                    );
                }
                "GET" | "^(GET)$" => {
                    debug!("Adding GET route: {}", path);

                    if sse {
                        self.router = main_router.merge(
                            Router::new()
                                .route(path.to_string().as_str(), get(handler_sse))
                                .with_state(shared_state),
                        );
                    } else {
                        self.router = main_router.merge(
                            Router::new()
                                .route(path.to_string().as_str(), get(handler))
                                .with_state(shared_state),
                        );
                    }
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
