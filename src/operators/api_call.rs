use std::sync::{Arc, Mutex};

use hyper::Method;
use reqwest::{Client, Request, RequestBuilder, Url};
use tracing::error;
use tracing::{debug, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType, OriginMessage};

use crate::graph::graph::{Message, Operator};

// Extends the `reqwest::RequestBuilder` to allow WebSocket upgrades.
use futures::future;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::Message as WsMessage;
use reqwest_websocket::RequestBuilderExt;

#[derive(Clone, Debug)]
pub(crate) struct APICall {
    pub method: Option<String>,
    pub url: String,
    pub path: String,
    pub content_type: Option<String>,
    pub forward: bool,
    pub ws: bool,
}

impl From<serde_yaml::Value> for APICall {
    fn from(value: serde_yaml::Value) -> Self {
        let url = value["url"].as_str().unwrap().to_string();
        let path = value["path"].as_str().unwrap_or("").to_string();
        let forward = value["forward"].as_bool().unwrap_or(false);
        let ws = value["ws"].as_bool().unwrap_or(false);
        let method = value["method"].as_str().map(|s| s.to_string());
        let content_type = value["content_type"].as_str().map(|s| s.to_string());

        APICall {
            method,
            url,
            path,
            content_type,
            forward,
            ws,
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
        message: Arc<Message>,
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

        // let client = Client::new();
        // let url = self.url.clone();
        // let forward = self.forward;
        // let ws = self.ws;
        // let default_path = self.path.clone();

        // error!("APICall handle (passthrough)... {}", url);

        // Box::pin(async move {
        //     let m = Message::ReqReply {
        //         respond_to,
        //         path,
        //         query,
        //         method,
        //         message,
        //     } = message;
        //     return m;
        // })
        // let a = match message {
        //     Message::ReqReply {
        //         respond_to,
        //         path,
        //         query,
        //         method,
        //         message,
        //     } => {
        //         let mut url_path = path;
        //         if !default_path.is_empty() {
        //             url_path = default_path;
        //         }
        //         let response;
        //         let url = format!("{}{}?{}", url, url_path, query);

        //         debug!("Request [{}] {}", method, url);
        //         //let req = Request::new(method, u);

        //         response = client
        //             .request(Method::try_from(method.as_str()).unwrap(), url)
        //             .header("Content-Type", "application/json")
        //             .header("Accept", "application/json")
        //             .body(message)
        //             .send()
        //             .await;

        //         //response = client.get(&url).send().await;
        //         match response {
        //             Ok(response) => {
        //                 if !response.status().is_success() {
        //                     warn!(
        //                         "Error: ({}) {}",
        //                         response.status(),
        //                         response.text().await.unwrap()
        //                     );
        //                     return Message::Standard {
        //                         message: b"Error".to_vec(),
        //                         origin: Some(OriginMessage { respond_to }),
        //                     };
        //                 } else {
        //                     return Message::JSON {
        //                         message: response.json().await.unwrap(),
        //                         //message: response.bytes().await.unwrap().to_vec(),
        //                         origin: Some(OriginMessage { respond_to }),
        //                     };
        //                 }
        //             }
        //             Err(e) => {
        //                 warn!("Error making request: {}", e);
        //                 return Message::Standard {
        //                     message: b"Error".to_vec(),
        //                     origin: Some(OriginMessage { respond_to }),
        //                 };
        //             }
        //         }
        //     }
        //     Message::Standard { message, origin } => {
        //         warn!("APICall handle (passthrough)... {}", url);

        //         if forward {
        //             //url = format!("{}?{}", url, String::from_utf8_lossy(&message));
        //         }

        //         // Make a GET request
        //         if ws {
        //             debug!("WebSocket request to {}", url);

        //             tokio::spawn(async move {
        //                 let response = client.get(&url).upgrade().send().await;
        //                 match response {
        //                     Ok(ws_response) => {
        //                         debug!("WebSocket UPGRADED");

        //                         // Turns the response into a WebSocket stream.
        //                         let websocket = ws_response.into_websocket().await.unwrap();

        //                         let (mut tx, mut rx) = websocket.split();

        //                         tokio::spawn(future::join(
        //                             async move {
        //                                 for i in 1..11 {
        //                                     debug!("Sending message {i}");
        //                                     tx.send(WsMessage::Text(format!("Hello, World! #{i}")))
        //                                         .await
        //                                         .unwrap();
        //                                 }
        //                             },
        //                             async move {
        //                                 loop {
        //                                     let result = rx.try_next().await;
        //                                     match result {
        //                                         Ok(Some(message)) => match message {
        //                                             WsMessage::Close { code, reason } => {
        //                                                 debug!("Received close {code} {reason}");
        //                                             }
        //                                             WsMessage::Text(text) => {
        //                                                 debug!("Received text message: {text}");
        //                                             }
        //                                             WsMessage::Binary(_) => {
        //                                                 debug!("Received binary message");
        //                                             }
        //                                             _ => {
        //                                                 debug!("Received other message");
        //                                             }
        //                                         },
        //                                         Ok(None) => {
        //                                             debug!("WebSocket closed");
        //                                             break;
        //                                         }
        //                                         Err(e) => {
        //                                             warn!("Error receiving message: {}", e);
        //                                             break;
        //                                         }
        //                                     }
        //                                 }
        //                             },
        //                         ));

        //                         // let split = websocket.try_next();

        //                         // tokio::spawn(async move {
        //                         //     // The WebSocket implements `Sink<Message>`.
        //                         //     let result = websocket
        //                         //         .send(WsMessage::Text("Hello, World".into()))
        //                         //         .await;

        //                         //     // need a way of sending messages to the websocket
        //                         //     // and receiving messages from the websocket
        //                         //     match result {
        //                         //         Ok(_) => {
        //                         //             debug!("[WS] sent message")
        //                         //         }
        //                         //         Err(e) => {
        //                         //             warn!("[WS] Error sending message: {}", e);
        //                         //         }
        //                         //     }
        //                         // });
        //                         // //The WebSocket is also a `TryStream` over `Message`s.
        //                         // while let Some(message) = split.await.unwrap() {
        //                         //     if let WsMessage::Text(text) = message {
        //                         //         println!("received: {text}")
        //                         //     }
        //                         // }

        //                         // need a handler similar to the http_in websocket
        //                         // but in reverse
        //                         return Message::Standard {
        //                             message: b"WebSocket".to_vec(),
        //                             origin,
        //                         };
        //                     }
        //                     Err(e) => {
        //                         warn!("Error making request: {}", e);
        //                         return Message::Standard {
        //                             message: b"Error".to_vec(),
        //                             origin,
        //                         };
        //                     }
        //                 }
        //             });
        //             Message::Standard {
        //                 message: b"do nothing".to_vec(),
        //                 origin: None,
        //             }
        //         } else {
        //             let response;
        //             response = client.get(&url).send().await;
        //             match response {
        //                 Ok(response) => {
        //                     if !response.status().is_success() {
        //                         warn!("Error: {}", response.status());
        //                         return Message::Standard {
        //                             message: b"Error".to_vec(),
        //                             origin,
        //                         };
        //                     } else {
        //                         return Message::Standard {
        //                             message: response.bytes().await.unwrap().to_vec(),
        //                             origin,
        //                         };
        //                     }
        //                 }
        //                 Err(e) => {
        //                     warn!("Error making request: {}", e);
        //                     return Message::Standard {
        //                         message: b"Error".to_vec(),
        //                         origin,
        //                     };
        //                 }
        //             }
        //         }
        //     }
        //     _ => panic!("Unexpected message type {}", message),
        // };
        // Box::pin(async move { Arc::new(a) })
    }

    fn async_handle<'life0, 'async_trait>(
        &'life0 self,
        message: Message,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Message> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let client = Client::new();
        let url = self.url.clone();
        let forward = self.forward;
        let ws = self.ws;
        let default_path = self.path.clone();

        Box::pin(async move {
            match message {
                Message::ReqReply {
                    respond_to,
                    path,
                    query,
                    method,
                    message,
                } => {
                    let mut url_path = path;
                    if !default_path.is_empty() {
                        url_path = default_path;
                    }
                    let response;
                    let url = format!("{}{}?{}", url, url_path, query);

                    debug!("--> [{}] {}", method, url);

                    response = client
                        .request(Method::try_from(method.as_str()).unwrap(), url)
                        .header("Content-Type", "application/json")
                        .header("Accept", "application/json")
                        .body(message)
                        .send()
                        .await;

                    //response = client.get(&url).send().await;
                    match response {
                        Ok(response) => {
                            debug!("<-- {}", response.status());
                            if !response.status().is_success() {
                                warn!(
                                    "Failed [{}] {} ({}) {}",
                                    method,
                                    url_path,
                                    response.status(),
                                    response.text().await.unwrap()
                                );
                                return Message::Standard {
                                    message: b"Error".to_vec(),
                                    origin: Some(OriginMessage { respond_to }),
                                };
                            } else {
                                return Message::JSON {
                                    message: response.json().await.unwrap(),
                                    //message: response.bytes().await.unwrap().to_vec(),
                                    origin: Some(OriginMessage { respond_to }),
                                };
                            }
                        }
                        Err(e) => {
                            warn!("Error making request: {}", e);
                            return Message::Standard {
                                message: b"Error".to_vec(),
                                origin: Some(OriginMessage { respond_to }),
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
                            let mut url_path = "/".to_string();
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
                                                            debug!("Received text message: {text}");
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
                        let method = self.method.as_ref().unwrap();
                        let mut url_path = "/".to_string();
                        if !default_path.is_empty() {
                            url_path = default_path;
                        }
                        let response;
                        let url = format!("{}{}?", url, url_path);

                        debug!("--> [{}] {}", method, url);

                        response = client
                            .request(Method::try_from(method.as_str()).unwrap(), url)
                            .header("Content-Type", "application/json")
                            .header("Accept", "application/json")
                            .body(message)
                            .send()
                            .await;

                        //let response;
                        //response = client.get(&url).send().await;
                        match response {
                            Ok(response) => {
                                debug!("<-- {}", response.status());
                                if !response.status().is_success() {
                                    warn!("Error: {}", response.status());
                                    return Message::Standard {
                                        message: b"Error".to_vec(),
                                        origin,
                                    };
                                } else {
                                    return Message::JSON {
                                        message: response.json().await.unwrap(),
                                        origin,
                                    };
                                }
                            }
                            Err(e) => {
                                warn!("Error making request: {}", e);
                                return Message::Standard {
                                    message: b"Error".to_vec(),
                                    origin,
                                };
                            }
                        }
                    }
                }
                _ => panic!("Unexpected message type {}", message),
            }
        })
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

    // fn get(&self) -> AsyncHandle {
    //     let closure = move |message: Message| {
    //         let url = self.url.clone();
    //         task::spawn(async move {
    //             let client = Client::new();
    //             let response = client.get(url).send();
    //             match response.await {
    //                 Ok(response) => {
    //                     if !response.status().is_success() {
    //                         warn!("Error: {}", response.status());
    //                     }
    //                 }
    //                 Err(e) => {
    //                     warn!("Error making request: {}", e);
    //                 }
    //             }
    //         })
    //     };

    //     AsyncHandle {
    //         closure: Box::new(closure),
    //     }
    // }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::Standard { message, origin } => {
                warn!("APICall handle (passthrough)... {}", self.url);

                // Create a client
                // Make a GET request
                let response = reqwest::blocking::get(&self.url);

                match response {
                    Ok(response) => {
                        if !response.status().is_success() {
                            warn!("Error: {}", response.status());
                            return Message::Standard {
                                message: b"Error".to_vec(),
                                origin,
                            };
                        }
                    }
                    Err(e) => {
                        warn!("Error making request: {}", e);
                        return Message::Standard {
                            message: b"Error".to_vec(),
                            origin,
                        };
                    }
                }
                // if let Err(e) = response {
                //     warn!("Error making request: {}", e);
                //     return Message::Standard {
                //         message: b"Error".to_vec(),
                //         origin,
                //     };
                // } else {
                //     let response = response.unwrap();
                //     if !response.status().is_success() {
                //         warn!("Error: {}", response.status());
                //         return Message::Standard {
                //             message: b"Error".to_vec(),
                //             origin,
                //         };
                //     }
                //     let body = response.text().unwrap();
                //     return Message::Standard {
                //         message: body.as_bytes().to_vec(),
                //         origin,
                //     };
                // }
                return Message::Standard {
                    message: b"Success".to_vec(),
                    origin,
                };
            }
            _ => panic!("Unexpected message type"),
        }
    }

    fn init(&mut self, _: &mut Graph) {
        debug!("Not implemented");
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, start, .. } => {
                // start reverse proxy server
                debug!("Configuring RP {}", self.url);
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
