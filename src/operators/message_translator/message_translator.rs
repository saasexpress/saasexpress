use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    sync::{Arc, Mutex},
};

use cel_interpreter::{Context, Program};
use serde_json::{Value as JsonValue, json};
use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType, OriginMessage};
use crate::graph::graph::{Message, Operator};
use crate::operators::message_translator::cel_to_json::cel_value_to_json;

#[derive(Clone, Debug)]
pub(crate) enum MessageTranslatorEngine {
    CelInterpreter,
}

impl Display for MessageTranslatorEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageTranslatorEngine::CelInterpreter => write!(f, "cel-interpreter"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MessageTranslator {
    template: String,
    engine: MessageTranslatorEngine,
}

impl From<serde_yaml::Value> for MessageTranslator {
    fn from(value: serde_yaml::Value) -> Self {
        MessageTranslator {
            template: value["template"].as_str().unwrap_or("").to_string(),
            engine: value
                .get("engine")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "cel-interpreter" => MessageTranslatorEngine::CelInterpreter,
                    _ => panic!("Unknown engine: {}", s),
                })
                .unwrap_or(MessageTranslatorEngine::CelInterpreter),
        }
    }
}

impl Operator for MessageTranslator {
    fn _type(&self) -> OperatorType {
        OperatorType::Filter
    }

    fn name(&self) -> String {
        "MessageTranslator".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::JSON { message, origin } => {
                let cel_value = self.parse(&message);

                match self.engine {
                    MessageTranslatorEngine::CelInterpreter => Message::JSON {
                        message: cel_value,
                        origin,
                    },
                    _ => {
                        panic!("Unknown engine: {}", self.engine);
                    }
                }
            }
            Message::ReqReply {
                message,
                respond_to,
                ..
            } => {
                let json = serde_json::to_value(message).unwrap();

                let cel_value = match self.engine {
                    MessageTranslatorEngine::CelInterpreter => self.parse(&json),
                    _ => {
                        panic!("Unknown engine: {}", self.engine);
                    }
                };

                Message::JSON {
                    message: cel_value,
                    origin: Some(OriginMessage { respond_to }),
                }
            }
            // Message::ReqReply {
            //     message,
            //     respond_to,
            //     ..
            // } => {
            //     return Message::Standard {
            //         message,
            //         origin: Some(OriginMessage { respond_to }),
            //     };
            // }
            _ => panic!("Unexpected message type {}", _message),
        }
    }

    fn init(&mut self, _: &mut Graph) {
        debug!("Not implemented");
    }

    fn control(&mut self, _: Message) {
        debug!("Not implemented");
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

impl MessageTranslator {
    fn parse(&self, data: &JsonValue) -> JsonValue {
        let program = Program::compile(&self.template).unwrap();

        let cel_data = cel_interpreter::to_value(data).unwrap();

        // Add any variables or functions that the program will need
        let mut context = Context::default();
        context.add_function("add", |a: i64, b: i64| a + b);

        debug!("Templ {}", self.template);
        debug!("In {}", serde_json::to_string_pretty(data).unwrap());

        let input = json!({
            "resource": "Tenant",
            "http_method": "POST",
        });

        context
            .add_variable("data", cel_data)
            .expect("Variable data problem");

        context
            .add_variable("input", input)
            .expect("Variable input problem");

        // Run the program
        let value = program.execute(&context).unwrap();

        let val = cel_value_to_json(&value);
        debug!("Out {}", serde_json::to_string_pretty(&val).unwrap());
        val
    }
}
