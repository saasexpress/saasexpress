use fastrace::Span;
use futures::channel::oneshot::{self, Canceled};
use serde_json::{Error, Value, json};
use tokio::sync::{mpsc, oneshot::Receiver};
use tracing::{debug, error, info, warn};

use crate::{
    graph::{
        graph::{AsyncHandleTrait, Graph, GraphStatus},
        message::{Message, OriginMessage},
        meta::NodeMeta,
        operator::{
            Operator, OperatorRef, OperatorRole, OperatorRuntime, OperatorRuntimeType,
            OperatorState, OperatorType,
        },
        registry::GraphRegistry,
    },
    my_reg::register,
    settings::settings::env_settings,
};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{Arc, Mutex},
};

use super::ai_tool::AIToolOperator;

pub trait AIAgentOperator: Sync + Send + Debug {
    // fn process(&self, json: Value) -> Result<(), Error>;
    fn process(
        &self,
        origin: Option<OriginMessage>,
        user_prompt: String,
        next: Vec<OperatorRole>,
        tools: HashMap<String, OperatorRuntimeType>,
    );
}

#[derive(Debug)]
pub struct AIAgent {
    node_fqn: Option<String>,
    graph_name: Option<String>,
    name: String,
    state: OperatorState,
    //tool_graph_names: Vec<String>,
    pub(crate) operator: Arc<dyn AIAgentOperator + Send + Sync + 'static>,

    tools: HashMap<String, OperatorRuntimeType>,

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

    fn new_runtime(
        &self,
        mut_nodes: HashMap<String, OperatorRef>,
        edges: HashMap<String, HashSet<(String, String)>>,
    ) -> Arc<dyn OperatorRuntime> {
        Arc::new(AIAgent {
            node_fqn: self.node_fqn.clone(),
            graph_name: self.graph_name.clone(),
            name: self.name.clone(),
            state: self.state.clone(),
            operator: Arc::clone(&self.operator),
            next: self.next.clone(),
            tools: self.tools.clone(),
        })
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        //let settings = env_settings(node_meta.base_env_vars_settings(node_meta));

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
            Message::Init { next, .. } => {
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
}

impl AIAgent {
    async fn req_reply(&self, role: String, mut json: Value) {
        let mut is_match = true;
        for node in self.next.iter().filter(|o| o.role == role) {
            let operator = &node.operator;

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

        //tool_operator...
        self.operator.process(origin, user_prompt, next, tools);
    }

    fn add_next(&mut self, operator: OperatorRole) {
        self.next.push(operator);
    }

    fn start(&mut self) -> bool {
        for operator in self.next.iter() {
            debug!(
                "Finalizing AIAgent {:?} {:?}",
                operator.role, operator.operator
            );
            if operator.role == "tool" {
                let tool_operator = Arc::clone(&operator.operator);

                let operator = &operator.operator;
                let op_type = operator._type();
                info!("Operator type: {:?}", op_type);

                if let OperatorType::AITool { tool } = op_type {
                    debug!("Tool match {} {:?}", tool.name(), tool);
                    self.tools.insert(tool.name(), tool_operator);
                } else {
                    error!("Invalid operator type {:?} {}", op_type, operator.name());
                    return false;
                }
            } else if operator.role == "prompt" {
                let operator = &operator.operator;
                let op_type = operator._type();
                info!("PROMPT Operator type: {:?}", op_type);
            } else if operator.role == "llm" {
                let operator = &operator.operator;
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

    #[fastrace::trace]
    fn start_agent(&self) {
        self.next.iter().for_each(|n| {
            let (tx, rx) = oneshot::channel();

            let origin = Some(OriginMessage::new(Some(tx)));

            let operator = Arc::clone(&n.operator);

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

impl OperatorRuntime for AIAgent {
    fn _type(&self) -> OperatorType {
        Operator::_type(self)
    }

    fn name(&self) -> String {
        Operator::name(self)
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
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

    fn send(&self, message: Message) {
        self.next(message);
    }
}
