use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

use async_std::io::Empty;
use async_std::task::Builder;
use fastrace::Span;
use fastrace::local::LocalSpan;
use futures::channel::{mpsc, oneshot};
use hyper::Method;
use reqwest::{Client, Response};
use reqwest_eventsource::{Event, EventSource};
use saasexpress_core::graph::message::{ControlCommand, Message, OriginMessage};
use saasexpress_core::graph::meta::NodeMeta;
use saasexpress_core::settings::settings::{Setting, env_settings};
use serde_json::{Value, json};
use tokio::runtime::{Handle, Runtime};
use tracing::{debug, warn};
use tracing::{error, info};

use saasexpress_core::graph::graph::{AsyncHandleTrait, Graph};
use saasexpress_core::graph::operator::{Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorType};

use fastrace::future::FutureExt;
use futures::future;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::Message as WsMessage;
use reqwest_websocket::RequestBuilderExt;

use crate::operators::api_call::session::SocketSession;
use crate::operators::api_call::socket_close;

use super::http::HTTPBuilder;
use super::session::{Closer, CloserAction};
use super::settings::TempParams;
//use crate::operators::api_call::socket_close::socket_close;
use saasexpress_core::graph::message::{DebuggableSpan, Message as GraphMessage};


#[derive(Clone, Debug)]
pub(crate) struct APICall {
    pub node_name: Option<String>,
    pub method: Option<String>,
    pub url: String,
    pub path: String,
    pub content_type: Option<String>,
    pub forward: bool,
    pub ws: bool,
    pub sse: bool,
    settings: Vec<Setting>,
    client: Client,
}

pub struct SessionRegistry {
    sessions: HashMap<String, Arc<Mutex<dyn CloserAction + 'static>>>,
}

impl SessionRegistry {
    fn new() -> Self {
        SessionRegistry {
            sessions: HashMap::new(),
        }
    }

    fn add_session(&mut self, session_id: String, sender: Arc<Mutex<dyn CloserAction + 'static>>) {
        self.sessions
            .insert(session_id, sender);
    }
    pub fn has_session(&self, session_id: &str) -> bool {
        self.sessions.get(session_id).is_some()
    }

    pub fn get_session(&self, session_id: &str) -> Arc<Mutex<dyn CloserAction + 'static>> {
        Arc::clone(self.sessions.get(session_id).unwrap())
    }

    pub fn get_instance() -> &'static Mutex<SessionRegistry> {
        INSTANCE.get_or_init(|| Mutex::new(SessionRegistry::new()))
    }
}

static INSTANCE: OnceLock<Mutex<SessionRegistry>> = OnceLock::new();

impl From<serde_yaml::Value> for APICall {
    fn from(value: serde_yaml::Value) -> Self {
        let url = value["url"].as_str().unwrap().to_string();
        let path = value["path"].as_str().unwrap_or("").to_string();
        let forward = value["forward"].as_bool().unwrap_or(false);
        let ws = value["ws"].as_bool().unwrap_or(false);
        let sse = value["sse"].as_bool().unwrap_or(false);
        let method = value["method"].as_str().map(|s| s.to_string());
        let content_type = value["content_type"].as_str().map(|s| s.to_string());

        APICall {
            node_name: None,
            method,
            url,
            path,
            content_type,
            forward,
            ws,
            sse,
            settings: Vec::new(),
            client: Client::new(),
        }
    }
}

impl AsyncHandleTrait for APICall {
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn async_handle_ptr<'life0, 'async_trait>(
        &'life0 self,
        _message: Arc<Message>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Arc<Message>> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            Arc::new(Message::Standard {
                message: b"APICall".to_vec(),
                origin: None,
            })
        })
    }

    fn async_handle<'life0, 'async_trait>(
        &'life0 self,
        mut message: Message,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Message> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let node_name = self.node_name.clone().unwrap();
        let name = Operator::name(self);
        let client = self.client.clone();
        let url = self.url.clone();
        let forward = self.forward;
        let ws = self.ws;
        let default_path = self.path.clone();

        let parent_span = message.get_span().expect("No span found");
        let _span = Span::enter_with_parent("api_call", parent_span);

        Box::pin(
            async move {
                let s = Span::enter_with_local_parent("api_call_inner");
                s.set_local_parent();
                //let _span = LocalSpan::enter_with_local_parent("api_call");

                //let temp_params = &temp_params;
                // let path = if temp_params.is_some() {
                //     temp_params.unwrap().0
                // } else {
                //     "".to_string().as_str()
                // };

                let temp_params = {
                    let og = message.get_origin();

                    match og {
                        Some(og) => {
                            let temp = message.get_origin().unwrap().temp.clone();
                            let temp = temp.lock().unwrap();

                            if temp.get(&node_name).is_some() {
                                let temp = temp.get(&node_name).unwrap();
                                let path = temp.get("path").unwrap().as_str().unwrap();
                                let url = temp.get("url").unwrap().as_str().unwrap();
                                let content_type =
                                    temp.get("content_type").unwrap().as_str().unwrap();
                                let body = temp.get("body").unwrap();
                                let headers = serde_json::from_value::<HashMap<String, String>>(
                                    temp.get("headers").unwrap().clone(),
                                )
                                .unwrap();

                                Some(TempParams {
                                    body: body.clone(),
                                    content_type: content_type.to_string(),
                                    path: path.to_string(),
                                    url: url.to_string(),
                                    headers,
                                })
                            } else {
                                debug!("No temp params found for {}", name);
                                None
                            }
                        }
                        None => None,
                    }
                    // if og.is_none() {
                    //     return None;
                    // }
                };

                match message {
                    Message::ReqReply {
                        respond_to,
                        temp,
                        message,
                        span,
                    } => {
                        if ws {
                            panic!("WebSocket not supported in ReqReply");
                        }
                        let (method, path, query) = {
                            let temp = temp.lock().unwrap();
                            let http_in = temp.get("http_in");
                            if let Some(http_in) = http_in {
                                let method = http_in.get("method").unwrap().as_str().unwrap();
                                let path = http_in.get("path").unwrap().as_str().unwrap();
                                let query = http_in.get("query_string").unwrap().as_str().unwrap();
                                (method.to_string(), path.to_string(), query.to_string())
                            } else {
                                ("GET".to_string(), "".to_string(), "".to_string())
                            }
                        };

                        let method = self.method.clone().unwrap_or(method);
                        let url =
                            HTTPBuilder::derive_url(&self.url, &path, &self.path, query.clone());

                        let builder = HTTPBuilder::new(method.as_str(), url.as_str())
                            .set_headers(&self.settings)
                            .set_header("Content-Type", "application/json")
                            .set_body(message);

                        let response = {
                            let s = Span::enter_with_local_parent("upstream_request");
                            s.set_local_parent();

                            builder.send().await
                        };

                        // let temp = json!({
                        //     "resource": self.url,
                        //     "http_method": method,
                        //     "query": query,
                        // });

                        //response = client.get(&url).send().await;

                        let origin = Some(
                            OriginMessage::new(Some(respond_to))
                                .with_span(span)
                                .with_temp(temp),
                        );

                        match response {
                            Ok(response) => {
                                debug!("<-- (ReqReply) {}", response.status());
                                if !response.status().is_success() {
                                    error!("Failed [{}] {}", method, url);
                                    log_error_info(response).await.with_origin(origin)
                                } else if is_json_response(&response) {
                                    // return Message::JSON {
                                    //     message: response.json().await.unwrap(),
                                    //     //message: response.bytes().await.unwrap().to_vec(),
                                    //     origin: Some(OriginMessage::new(respond_to)),
                                    // };
                                    let status = response.status().as_u16();
                                    let headers = response
                                        .headers()
                                        .iter()
                                        .map(|h| {
                                            (
                                                String::from(h.0.as_str()).to_lowercase(),
                                                String::from(h.1.to_str().unwrap()),
                                            )
                                        })
                                        .collect::<HashMap<String, String>>();

                                    return Message::HTTP {
                                        message: response.bytes().await.unwrap().to_vec(),
                                        headers,
                                        status,
                                        origin,
                                    };
                                } else {
                                    //while let Some(chunk) = res.chunk().await? {
                                    //    println!("Chunk: {chunk:?}");
                                    //}

                                    let status = response.status().as_u16();
                                    let headers = response
                                        .headers()
                                        .iter()
                                        .map(|h| {
                                            (
                                                String::from(h.0.as_str()).to_lowercase(),
                                                String::from(h.1.to_str().unwrap()),
                                            )
                                        })
                                        .collect::<HashMap<String, String>>();

                                    return Message::HTTP {
                                        message: response.bytes().await.unwrap().to_vec(),
                                        headers,
                                        status,
                                        origin,
                                    };
                                }
                            }
                            Err(e) => {
                                error!("[ReqReply] Error making request {:?}", e);

                                Message::Error {
                                    error: format!(
                                        "Error making request (connect? {:?}, status? {:?}, request? {:?}, timeout? {:?})",
                                        e.is_connect(),
                                        e.is_status(),
                                        e.is_request(),
                                        e.is_timeout()
                                    ),
                                    origin,
                                }
                            }
                        }
                    }
                    Message::Standard { message, origin } => {
                        debug!("APICall handle (passthrough)... {}", url);

                        if forward {
                            //url = format!("{}?{}", url, String::from_utf8_lossy(&message));
                        }

                        // Make a GET request
                        if ws {
                            // see if there is a SocketSession already established
                            // if so, forward message to it
                            // otherwise, create a new SocketSession

                            let session = origin
                                .as_ref()
                                .and_then(|o| o.session.clone())
                                .unwrap();
                            debug!("WebSocket - request to = {}", session);


                            let a_session = {
                                let socket_registry = SessionRegistry::get_instance();
                                let socket_registry = socket_registry.lock().unwrap();
                                socket_registry.has_session(&session)
                            };
                            if a_session {
                                debug!("WebSocket - session already exists");
                                
                                let socket_registry = SessionRegistry::get_instance();

                                let socket = socket_registry
                                    .lock()
                                    .unwrap()
                                    .get_session(&session);
                                let socket = socket.lock().unwrap();

                                socket.send(Message::Standard { message, origin });
                                // tokio::spawn(async move {
                                //    socket_close(&session)
                                // });

                                // let message = WsMessage::Text(String::from_utf8_lossy(&message));

                                // sender.send(message).await.unwrap();
                                Message::NoOp {}

                            } else {

                                debug!("WebSocket request to {}", url);

                                let method = self.method.clone().unwrap();

                                tokio::spawn(async move {
                                    let mut url_path = "".to_string();
                                    if !default_path.is_empty() {
                                        url_path = default_path;
                                    }
                                    let url = format!("{}{}?", url, url_path);

                                    debug!("--> [{}] {}", method, url);

                                    let response = client
                                        .request(Method::try_from(method.as_str()).unwrap(), url)
                                        //.header("Content-Type", "application/json")
                                        //.header("Accept", "application/json")
                                        //.body(message)
                                        .upgrade()
                                        .send()
                                        .await;

                                    // let response = client.get(&url).upgrade().send().await;
                                    match response {
                                        Ok(ws_response) => {
                                            debug!("WebSocket UPGRADED");

                                            // Turns the response into a WebSocket stream.
                                            let _websocket = ws_response.into_websocket().await;
                                            let websocket = match _websocket {
                                                Ok(ws) => ws,
                                                Err(e) => {
                                                    warn!("Error making request: {}", e);
                                                    return Message::Standard {
                                                        message: b"Error".to_vec(),
                                                        origin,
                                                    };
                                                }
                                            };

                                            let graph_sender = origin
                                                .as_ref()
                                                .and_then(|o| o.mpsc_respond_to.clone())
                                                .unwrap();

                                            let session = origin.as_ref().and_then(|o| o.session.clone()).unwrap();


                                            let span = origin
                                            .unwrap().span.unwrap().0;

                                            let (tx, rx) = tokio::sync::mpsc::channel::<GraphMessage>(5);

                                            let ss = SocketSession::new(
                                                websocket,
                                                graph_sender,
                                                session.clone(),
                                                span,
                                                rx
                                            );


                                            tokio::spawn(async move {
                                                ss.process(message).await;
                                            });

                                            let socket_registry = SessionRegistry::get_instance();

                                            socket_registry
                                                .lock()
                                                .unwrap()
                                                .add_session(session, Arc::new(Mutex::new(Closer{tx})));

                                            return Message::Standard {
                                                message: b"WebSocket".to_vec(),
                                                origin:None,
                                            };
                                        }
                                        Err(e) => {
                                            error!("[WS] Error making request: {:?}", e);
                                            return Message::Standard {
                                                message: b"Error".to_vec(),
                                                origin,
                                            };
                                        }
                                    }
                                });
                                Message::Standard {
                                    message: b"do nothing".to_vec(),
                                    origin: None,
                                }
                            }
                        } else {
                            let temp_params = &temp_params;

                            let empty = "".to_string();
                            let empty_map = HashMap::new();
                            let app_json = "application/json".to_string();

                            let path = temp_params
                                .is_some()
                                .then(|| &temp_params.as_ref().unwrap().path)
                                .unwrap_or(&empty);
                            let content_type = temp_params
                                .is_some()
                                .then(|| &temp_params.as_ref().unwrap().content_type)
                                .unwrap_or(&app_json);
                            let base_url = temp_params
                                .is_some()
                                .then(|| &temp_params.as_ref().unwrap().url)
                                .unwrap_or(&self.url);
                            let url = HTTPBuilder::derive_url(
                                &base_url,
                                path,
                                &self.path,
                                "".to_string(),
                            );
                            let extra_headers = temp_params
                                .is_some()
                                .then(|| &temp_params.as_ref().unwrap().headers)
                                .unwrap_or(&empty_map);

                            let method = self.method.clone().unwrap_or("GET".to_string());

                            if self.sse {
                                if method != "GET" {
                                    error!("{} {}", method, url);
                                    return Message::Error {
                                        error: "Only GET method is supported".to_string(),
                                        origin,
                                    };
                                }

                                let mpsc_respond_to = origin
                                    .as_ref()
                                    .and_then(|o| o.mpsc_respond_to.clone())
                                    .unwrap();

                                let mut es = EventSource::get(url);

                                while let Some(event) = es.next().await {
                                    match event {
                                        Ok(Event::Open) => info!("SSE Connection Open!"),
                                        Ok(Event::Message(message)) => {
                                            let json = serde_json::from_str(&message.data);
                                            if json.is_err() {
                                                error!("Error parsing JSON: {}", json.unwrap_err());
                                                continue;
                                            }
                                            mpsc_respond_to
                                                .send(Message::JSON {
                                                    message: json.unwrap(),
                                                    origin: None,
                                                })
                                                .await
                                                .unwrap();
                                        }
                                        Err(err) => {
                                            info!("Error - closing event stream: {}", err);
                                            es.close();
                                        }
                                    }
                                }
                                Message::NoOp {}
                            } else {
                                let builder;

                                if method == "POST"
                                    && content_type == "application/x-www-form-urlencoded"
                                {
                                    let body = temp_params
                                        .is_some()
                                        .then(|| &temp_params.as_ref().unwrap().body)
                                        .unwrap()
                                        .as_object()
                                        .unwrap();

                                    let query_tuples = serde_html_form::to_string(body).unwrap();

                                    builder = HTTPBuilder::new(method.as_str(), url.as_str())
                                        .set_headers(&self.settings)
                                        .set_body(query_tuples.to_string().as_bytes().to_vec())
                                        .set_header(
                                            "Content-Type",
                                            "application/x-www-form-urlencoded",
                                        );
                                } else {
                                    builder = HTTPBuilder::new(method.as_str(), url.as_str())
                                        .set_headers(&self.settings)
                                        .set_body(message)
                                        .payloads_json();
                                }

                                let builder = builder.set_headers_with_map(extra_headers);

                                let response = builder.send().await;

                                match response {
                                    Ok(response) => {
                                        let status = response.status();
                                        debug!("<-- {}", response.status());
                                        if !status.is_success() {
                                            warn!("Error: {}", status);
                                            log_error_info(response).await.with_origin(origin)
                                        } else {
                                            let message = response.json().await.unwrap();
                                            debug!("Message: {:?}", message);
                                            return Message::JSON { message, origin };
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Error making request: {}", e);

                                        return Message::Error {
                                            error: format!("Error making request: {}", e),
                                            origin,
                                        };
                                    }
                                }
                            }
                        }
                    }
                    Message::Error { error, origin } => Message::Error { error, origin },
                    Message::Exit {origin} => {
                        if !ws {
                            warn!("Exit should only happen for WebSockets");
                            return Message::Exit { origin };

                        }

                        let session = origin
                            .as_ref()
                            .and_then(|o| o.session.clone())
                            .unwrap();

                        debug!("Exit - close session = {}", session);

                        // see if there is a SocketSession already established
                        // if so, forward message to it
                        // otherwise, create a new SocketSession
                        let socket_registry = {
                            let socket_registry = SessionRegistry::get_instance();
                            let socket_registry = socket_registry.lock().unwrap();
                            socket_registry
                        };
                        let session = socket_registry.get_session(&session);
                        let session = session.lock().unwrap();
                        session.send(GraphMessage::Exit { origin: None });



                        let og = origin.unwrap();

                        return Message::Exit { origin: Some(og) };

                    }
                    _ => {
                        error!("Unexpected message type {}", message);
                        return Message::Error {
                            error: "Unexpected message type".to_string(),
                            origin: None,
                        };
                    }
                }
            }
            .in_span(_span),
        )
    }
    // Implement required methods for AsyncHandleTrait here
}

/**
 * APICall operator
 * This operator is used to forward messages to a different endpoint.
 *
 * Reference: https://github.com/junkurihara/rust-rpxy
 *
 * - start the reverse proxy server during the Init Control message
 *
 */
impl Operator for APICall {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "APICall".to_string()
    }

    fn new_runtime(&self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(self.clone())
    }

    fn init(&mut self, _graph: &mut Graph, node_meta: &NodeMeta) {
        self.node_name = Some(node_meta.name.clone());
        self.settings = env_settings(node_meta.base_env_vars_settings(node_meta));

        let a = self.settings.iter().find(|x| x.key == "URL");
        if a.is_some() {
            info!("Overriding URL from settings {:?}", a);
            self.url = a.unwrap().value.clone();
        }
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Control { command, .. } => {
                let mut current_settings = self.settings.to_owned();
                match command {
                    ControlCommand::SetSettings { settings } => {
                        settings.iter().for_each(|(k, v)| {
                            current_settings.push(Setting {
                                key: k.replace("-", "_").to_uppercase().to_string(),
                                value: v.as_str().unwrap_or("").to_string(),
                            });
                        });
                    }
                    ControlCommand::Start {..} => {}
                    _ => {
                        panic!("Invalid control command {:?}", command);
                    }
                }
                self.settings = current_settings;
            }

            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }
}

fn is_json_response(response: &reqwest::Response) -> bool {
    let content_type = response.headers().get("Content-Type");
    if let Some(content_type) = content_type {
        if let Ok(content_type_str) = content_type.to_str() {
            return content_type_str.contains("application/json");
        }
    }
    false
}

async fn log_error_info(response: Response) -> Message {
    let status = response.status();
    let headers = response.headers().to_owned();

    let content_type = headers
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    error!("Content-Type: {}", content_type);

    error!("Status: {}", status);
    error!("Response Headers:");
    let headers = headers
        .iter()
        .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<String, String>>();
    for (key, value) in headers.iter() {
        error!("[{}] {:?}", key, value);
    }

    let message = if content_type.contains("application/json") {
        let body = response
            .json()
            .await
            .unwrap_or_else(|_| json!({"error": "Failed to read json body"}));
        error!(
            "Response Body: {}",
            serde_json::to_string_pretty(&body).unwrap()
        );
        serde_json::to_string_pretty(&body)
            .unwrap()
            .as_bytes()
            .to_vec()
    } else {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read body".to_string());
        error!("Response Body: {}", body);
        body.as_bytes().to_vec()
    };
    Message::HTTP {
        message,
        origin: None,
        headers,
        status: status.into(),
    }
}


impl OperatorRuntime for APICall {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        Some(Arc::new(self.to_owned()))
    }

    fn handle(&self, _message: Message) -> Message {
        panic!("Should use async_handle");
    }

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }
}
