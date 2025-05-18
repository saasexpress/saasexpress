use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_std::io::Empty;
use async_std::task::Builder;
use fastrace::Span;
use fastrace::local::LocalSpan;
use futures::channel::mpsc;
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

use saasexpress_core::graph::graph::{AsyncHandleTrait, Graph, Operator, OperatorType};

use fastrace::future::FutureExt;
use futures::future;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::Message as WsMessage;
use reqwest_websocket::RequestBuilderExt;

use super::http::HTTPBuilder;
use super::settings::TempParams;

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
        let name = self.name();
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
                                warn!("No temp params found for {}", name);
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
                        match response {
                            Ok(response) => {
                                debug!("<-- (ReqReply) {}", response.status());
                                if !response.status().is_success() {
                                    error!("Failed [{}] {}", method, url);
                                    let r_status = response.status();
                                    let r_text = response.text().await.unwrap();
                                    error!("Failed ({}) {}", r_status, r_text);

                                    //let json: Value = serde_json::from_str(&r_text).unwrap();

                                    let result = json!({"status":"Error", "result": r_text});

                                    // return Message::Standard {
                                    //     message: serde_json::to_vec(&result).unwrap(),
                                    //     origin: Some(OriginMessage::new(respond_to)),
                                    // };
                                    return Message::HTTP {
                                        message: serde_json::to_vec(&result).unwrap(),
                                        origin: Some(
                                            OriginMessage::new(Some(respond_to)).with_span(span),
                                        ),
                                        headers: HashMap::new(),
                                        status: r_status.as_u16(),
                                    };
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
                                        origin: Some(
                                            OriginMessage::new(Some(respond_to))
                                                .with_span(span)
                                                .with_temp(temp),
                                        ),
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
                                        origin: Some(
                                            OriginMessage::new(Some(respond_to))
                                                .with_span(span)
                                                .with_temp(temp),
                                        ),
                                    };
                                }
                            }
                            Err(e) => {
                                error!("Error making request {:?}", e);
                                return Message::Standard {
                                    message: b"Error".to_vec(),
                                    origin: Some(OriginMessage::new(Some(respond_to))),
                                };
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

                                        let (mut tx, mut rx) = websocket.split();

                                        tokio::spawn(future::join(
                                            async move {
                                                for i in 1..11 {
                                                    debug!("Sending message {i}");
                                                    tx.send(WsMessage::Text(format!(
                                                        "Hello, World! #{i}"
                                                    )))
                                                    .await
                                                    .unwrap();
                                                }
                                            },
                                            async move {
                                                loop {
                                                    let result = rx.try_next().await;
                                                    match result {
                                                        Ok(Some(message)) => match message {
                                                            WsMessage::Close { code, reason } => {
                                                                debug!(
                                                                    "Received close {code} {reason}"
                                                                );
                                                            }
                                                            WsMessage::Text(text) => {
                                                                debug!(
                                                                    "Received text message: {text}"
                                                                );
                                                            }
                                                            WsMessage::Binary(_) => {
                                                                debug!("Received binary message");
                                                            }
                                                            _ => {
                                                                debug!("Received other message");
                                                            }
                                                        },
                                                        Ok(None) => {
                                                            debug!("WebSocket closed");
                                                            break;
                                                        }
                                                        Err(e) => {
                                                            warn!("Error receiving message: {}", e);
                                                            break;
                                                        }
                                                    }
                                                }
                                            },
                                        ));

                                        // let split = websocket.try_next();

                                        // tokio::spawn(async move {
                                        //     // The WebSocket implements `Sink<Message>`.
                                        //     let result = websocket
                                        //         .send(WsMessage::Text("Hello, World".into()))
                                        //         .await;

                                        //     // need a way of sending messages to the websocket
                                        //     // and receiving messages from the websocket
                                        //     match result {
                                        //         Ok(_) => {
                                        //             debug!("[WS] sent message")
                                        //         }
                                        //         Err(e) => {
                                        //             warn!("[WS] Error sending message: {}", e);
                                        //         }
                                        //     }
                                        // });
                                        // //The WebSocket is also a `TryStream` over `Message`s.
                                        // while let Some(message) = split.await.unwrap() {
                                        //     if let WsMessage::Text(text) = message {
                                        //         println!("received: {text}")
                                        //     }
                                        // }

                                        // need a handler similar to the http_in websocket
                                        // but in reverse
                                        return Message::Standard {
                                            message: b"WebSocket".to_vec(),
                                            origin,
                                        };
                                    }
                                    Err(e) => {
                                        error!("Error making request: {}", e);
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
                                            log_error_info(response).await;

                                            return Message::Error {
                                                error: format!("API Call Error: {}", status),
                                                origin,
                                            };
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

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        Some(Arc::new(self.to_owned()))
    }

    fn handle(&self, _message: Message) -> Message {
        panic!("Should use async_handle");
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        self.node_name = Some(node_meta.name.clone());
        self.settings = env_settings(graph.base_env_vars_settings(node_meta));

        let a = self.settings.iter().find(|x| x.key == "URL");
        if a.is_some() {
            info!("Overriding URL from settings {:?}", a);
            self.url = a.unwrap().value.clone();
        }
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { .. } => {}

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

    fn send(&self, _: Message) {
        panic!("Not implemented");
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Not implemented");
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

async fn log_error_info(response: Response) {
    let status = response.status();
    let headers = response.headers().to_owned();

    error!("Status: {}", status);
    error!("Response Headers:");
    for (key, value) in headers.iter() {
        error!("[{}] {:?}", key, value);
    }

    let content_type = headers
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    error!("Content-Type: {}", content_type);

    if content_type.contains("application/json") {
        let body = response
            .json()
            .await
            .unwrap_or_else(|_| json!({"error": "Failed to read json body"}));
        error!(
            "Response Body: {}",
            serde_json::to_string_pretty(&body).unwrap()
        );
    } else {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read body".to_string());
        error!("Response Body: {}", body);
    }
}
