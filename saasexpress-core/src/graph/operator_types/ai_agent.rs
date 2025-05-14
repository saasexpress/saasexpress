use fastrace::Span;
use futures::channel::oneshot::{self, Canceled};
use serde_json::{Error, Value, json};
use tokio::sync::{mpsc, oneshot::Receiver};
use tracing::{debug, error, info, warn};

use crate::{
    broker::Broker,
    control_bus::ControlEvent,
    graph::{
        graph::{
            AsyncHandleTrait, Graph, GraphStatus, Operator, OperatorRef, OperatorRole,
            OperatorState, OperatorType,
        },
        message::{Message, OriginMessage},
        meta::NodeMeta,
        registry::GraphRegistry,
    },
    my_reg::register,
    settings::settings::env_settings,
};
use core::panic;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use super::ai_tool::AIToolOperator;

pub trait AIAgentOperator: Sync + Send + Debug {
    fn process(&self, json: Value) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct AIAgent {
    node_fqn: Option<String>,
    graph_name: Option<String>,
    name: String,
    state: OperatorState,
    //tool_graph_names: Vec<String>,
    pub(crate) operator: Arc<dyn AIAgentOperator + Send + Sync + 'static>,

    tools: HashMap<String, Arc<dyn AIToolOperator + Send + Sync + 'static>>,

    next: Vec<OperatorRole>,
}

impl AIAgent {
    pub fn new(
        name: &str,
        values: serde_yaml::Value,
        operator: impl AIAgentOperator + Send + Sync + 'static,
    ) -> Self {
        // let tool_graph_names = values
        //     .get("tool_graphs")
        //     .and_then(|v| v.as_sequence())
        //     .map(|seq| {
        //         seq.iter()
        //             .filter_map(|v| v.as_str().map(|s| s.to_string()))
        //             .collect::<Vec<String>>()
        //     })
        //     .unwrap_or_default();

        AIAgent {
            node_fqn: None,
            graph_name: None,
            state: OperatorState::Pending,
            name: name.to_string(),
            operator: Arc::new(operator),
            next: Vec::new(),
            tools: HashMap::new(),
        }
    }
}

impl Operator for AIAgent {
    fn _type(&self) -> OperatorType {
        OperatorType::AIAgent {}
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn state(&self) -> OperatorState {
        self.state.clone()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        let settings = env_settings(graph.base_env_vars_settings(node_meta));

        self.node_fqn = node_meta.fqn().into();
        self.graph_name = Some(graph.name.clone());

        //let tool_graph_names = self.tool_graph_names.clone();

        // let mut broker = Broker::get_instance().lock().unwrap();

        // for tool_graph_name in tool_graph_names.iter() {
        //     let pub_msg = broker.publish(tool_graph_name.clone(), vec![]);

        //     tokio::spawn(async move {
        //         pub_msg.await;
        //     });

        //     info!("Subscribing to tool graph {}", tool_graph_name);
        //     let mut rx = broker.subscribe(tool_graph_name.clone());

        //     let tool_graph_name = tool_graph_name.clone();
        //     tokio::spawn(async move {
        //         while let Some(message) = rx.recv().await {
        //             info!("Received message from tool graph {:?}", tool_graph_name);
        //             // Process the message here
        //             // let result = operator.process(message.payload);
        //             // if let Err(e) = result {
        //             //     error!("Error processing message: {}", e);
        //             // }
        //         }
        //     });

        //     // while let Some(message) = rx.recv().await {
        //     //     info!("Received message from tool graph {:?}", tool_graph_name);
        //     // }
        // }

        // let a = settings.iter().find(|x| x.key == "URL");
        // if a.is_some() {
        //     info!("Overriding URL from settings {:?}", a);
        //     self.url = a.unwrap().value.clone();
        // }
    }

    fn control(&mut self, message: Message) {
        match message {
            Message::Init {
                id,
                next,
                start,
                end,
            } => {
                for n in next {
                    self.add_next(n);
                }

                //let graph_name = self.graph_name.clone().unwrap();

                //watch_control_bus(graph_name, self.node_fqn.clone().unwrap());
            }
            _ => {
                error!("Unexpected message type {}", message);
            }
        }
    }

    fn handle(&self, in_message: Message) -> Message {
        return in_message;
        //let origin = in_message.take_origin();

        // if existing conversation, retrieve it
        // prepare the llm message which includes the tool schemas()
        // send to operator (role = llm)
        // wait for response
        // determine the next function (role = tool) and call it
        // pass conversation to operator (role = storage)
        // return response

        // match &in_message {
        //     Message::JSON { message: json, .. } => match self.operator.process(json.clone()) {
        //         Ok(_model) => in_message.with_origin(origin),
        //         Err(e) => {
        //             error!("Error processing message to AIAgent: {}", e);
        //             return Message::Error {
        //                 error: format!("Canonical Model Validation Error - {}", e).to_string(),
        //                 origin,
        //             };
        //         }
        //     },
        //     _ => {
        //         error!("Unexpected message type {}", in_message);
        //         return Message::Error {
        //             error: "Unexpected message type".to_string(),
        //             origin,
        //         };
        //     }
        // }
        // in_message
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<std::sync::Arc<std::sync::Mutex<dyn Operator>>> {
        panic!("Not implemented");
    }

    fn send(&self, message: Message) {
        self.next(message);
    }

    fn finalize(&mut self) -> bool {
        for operator in self.next.iter() {
            debug!(
                "Finalizing AIAgent {:?} {:?}",
                operator.role,
                operator.operator.lock().unwrap()
            );
            if operator.role == "tool" {
                let operator = operator.operator.lock().unwrap();
                let op_type = operator._type();
                info!("Operator type: {:?}", op_type);

                if let OperatorType::AITool { tool } = op_type {
                    debug!("Tool match {} {:?}", tool.name(), tool);
                    self.tools.insert(tool.name(), tool);
                } else {
                    error!("Invalid operator type {:?} {}", op_type, operator.name());
                    return false;
                }
            } else if operator.role == "prompt" {
                let operator = operator.operator.lock().unwrap();
                let op_type = operator._type();
                info!("PROMPT Operator type: {:?}", op_type);
            } else if operator.role == "llm" {
                let operator = operator.operator.lock().unwrap();
                let op_type = operator._type();
                info!("LLM Operator type: {:?}", op_type);
            } else if operator.role == "default" {
                // this is fine - where the final response gets passed onto
            } else {
                error!(
                    "Unexpected operator role {:?} {:?}",
                    operator.role, operator.operator
                );
            }
        }
        self.state = OperatorState::Ready;
        true
    }

    fn send_ptr(&self, _message: Arc<Message>) {
        let message = _message.to_owned();
        self.next_ptr(self.handle_ptr(message));
    }

    fn handle_ptr(&self, message: Arc<Message>) -> Arc<Message> {
        tracing::debug!("default handle (passthrough)... {}", self.name());
        return message;
    }

    fn next_ptr(&self, message: Arc<Message>) {
        // Sending message to next operator
        for n in self.get_output_channels() {
            n.lock().unwrap().send_ptr(message.to_owned());
            //break;
        }
    }
}

impl AIAgent {
    async fn req_reply(&self, role: String, mut json: Value) {
        let mut is_match = true;
        for node in self.next.iter().filter(|o| o.role == role) {
            let operator = node.operator.lock().unwrap();

            let (tx, rx) = oneshot::channel();
            operator.send(Message::JSON {
                message: json.take(),
                origin: Some(OriginMessage::new(Some(tx))),
            });

            let response = rx.await;
            info!("Response = {:?}", response);
            is_match = true;
        }
        if is_match == false {
            error!("No matching operator found for role {}", "tool");
        }
    }

    fn next(&self, mut _message: Message) {
        let next = self.next.clone();

        let tools = self.tools.clone();

        let origin = _message.take_origin();

        info!("Next message: {:?}", _message);
        let request_msg = match _message {
            Message::JSON { message, .. } => message,
            Message::Standard { message, .. } => serde_json::from_slice(&message).unwrap(),
            _ => {
                error!("Unexpected message type: {:?}", _message);
                return;
            }
        };
        let user_prompt = request_msg.get("prompt").unwrap().clone();
        let user_prompt = user_prompt.as_str().unwrap().to_string();

        // before passing on the message, run the engine part
        tokio::spawn(async move {
            // (1) if existing conversation, retrieve it
            //let storage = callout(next.clone(), "storage".to_string(), json!({})).await;
            // storage will be added to the conversation history

            // (2) gather up any special prompts
            let prompts_result = callout(next.clone(), "prompt".to_string(), json!({})).await;

            // (4) wait for response
            if prompts_result.is_err() {
                error!("Error getting Prompts: {:?}", prompts_result.err());
                return;
            }
            let prompts_result = prompts_result.unwrap();

            let system_prompt = match prompts_result {
                Message::JSON { message, .. } => message,
                Message::Standard { message, .. } => serde_json::from_slice(&message).unwrap(),
                _ => {
                    error!("Unexpected Prompts result type: {:?}", prompts_result);
                    return;
                }
            };
            let system_prompt = system_prompt
                .get("content")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();

            // (2) prepare the llm message which includes the tool schemas()
            let llm_request =
                prepare_llm_request(system_prompt, Vec::new(), user_prompt, tools).await;

            info!("{}", serde_yaml::to_string(&llm_request).unwrap());
            // (3) send to operator (role = llm)
            let llm_result = callout(next.clone(), "llm".to_string(), json!(llm_request)).await;

            // (4) wait for response
            if llm_result.is_err() {
                error!("Error calling LLM: {:?}", llm_result.err());
                return;
            }
            let llm_result = llm_result.unwrap();

            let message = match llm_result {
                Message::JSON { message, .. } => message,
                Message::Standard { message, .. } => serde_json::from_slice(&message).unwrap(),
                _ => {
                    error!("Unexpected LLM result type: {:?}", llm_result);
                    return;
                }
            };

            info!("LLM result: {:?}", message);

            // (5) determine the next function (role = tool) and call it
            //let a = next_move(_message, HashMap::new()).await;
            //aa().await;

            // (6) pass conversation to operator (role = storage)

            // (7) return response
            let role = OperatorRole::default();
            let next = next
                .iter()
                .filter(|o| o.role == role)
                .next()
                .map(|n| n.operator.clone())
                .unwrap();

            next_send(Message::JSON { message, origin }, next).await;
        });
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }

    #[fastrace::trace]
    fn start_agent(&self) {
        self.next.iter().for_each(|n| {
            let node = Arc::clone(&n.operator);

            // ReqReply tools to get the schema
            let operator = node.lock().unwrap();

            let (tx, rx) = oneshot::channel();

            let origin = Some(OriginMessage::new(Some(tx)));

            info!("Starting tool {}", operator.name());
            operator.send(Message::Standard {
                message: serde_json::to_vec(&json!({"tool": "setup"})).unwrap(),
                origin,
            });

            let name = operator.name();
            let finalize = async move {
                let response = rx.await.unwrap_or_else(|_| {
                    error!("Failed to receive response from tool");
                    Message::Error {
                        error: "Failed to receive response from tool".to_string(),
                        origin: None,
                    }
                });

                info!("Received response from tool {}: {:?}", name, response);
            };
            tokio::spawn(finalize);
        });
    }
}

// fn watch_control_bus(graph_name: String, id: String) {
//     let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

//     // Register it
//     register(&id, tx);

//     tokio::spawn(async move {
//         // let graph = GraphRegistry::get_graph(&graph_name);

//         // let graph = graph.expect("Failed to get graph!");

//         loop {
//             // Receive the message
//             if let Some(msg) = rx.recv().await {
//                 info!(
//                     "[Node: {}] Received : {:?}",
//                     id,
//                     serde_json::to_string(&msg)
//                 );

//                 // if msg.graph_name == graph_name {
//                 //     info!("Received message from itself {}", graph_name);
//                 // } else {
//                 //     if msg.state == GraphStatus::Running {
//                 //         let mut graph = graph.lock().unwrap();

//                 //         graph.poke();
//                 //     }
//                 // }
//             } else {
//                 warn!("Channel is closed");
//                 break;
//             }
//         }
//     });
// }

async fn next_send(message: Message, next: OperatorRef) {
    let next = next.lock().unwrap();
    next.send(message);
}

async fn prepare_llm_request(
    system_prompt: String,
    mut history: Vec<Value>,
    user_prompt: String,
    tools: HashMap<String, Arc<dyn AIToolOperator + Send + Sync + 'static>>,
) -> Value {
    let tool_schemas = tools
        .iter()
        .map(|(_name, tool)| {
            let schema = tool.get_schema().unwrap();
            json!({
                "type": "function",
                "function": schema
            })
        })
        .collect::<Vec<_>>();

    let mut messages = Vec::new();
    messages.append(
        json!([
        {
            "role": "system",
            "content": system_prompt
        }])
        .as_array_mut()
        .unwrap(),
    );
    messages.append(&mut history);
    messages.append(
        json!([
        {
            "role": "user",
            "content": user_prompt
        }])
        .as_array_mut()
        .unwrap(),
    );

    json!({
      "messages": messages,
      "model": "gpt-4.1",
      "tools": tool_schemas,
      "tool_choice": "auto"
    })
}

async fn do_callout(message: Value, next: OperatorRef) -> oneshot::Receiver<Message> {
    let (tx, rx) = oneshot::channel::<Message>();

    let message = Message::JSON {
        message,
        origin: Some(OriginMessage::new(Some(tx))),
    };

    let next = next.lock().unwrap();
    next.send(message);

    rx
}

async fn aa() {
    info!("OK");
}
async fn next_move(
    in_message: Message,
    tools: HashMap<String, Arc<dyn AIToolOperator + Send + Sync + 'static>>,
) -> Message {
    info!("Tool count: {}", tools.keys().len());

    let tool = tools.iter().next().unwrap().1;

    let schema = tool.get_schema().unwrap();
    info!("Tool schema: {:?}", schema);
    // self.tools.iter().filter(|tool| tool.).for_each(|t| {
    //     info!("Tool: {}", t.name());
    // });
    tool.invoke(in_message)
}

async fn callout(
    next_list: Vec<OperatorRole>,
    role: String,
    json: Value,
) -> Result<Message, Canceled> {
    let next = next_list
        .iter()
        .filter(|o| o.role == role)
        .next()
        .map(|n| n.operator.clone());

    if next.is_none() {
        error!("No matching operator found for role {}", role);
        return Err(Canceled);
    }
    let next = next.unwrap().clone();

    let llm_result = do_callout(json, next).await.await;
    llm_result
}
