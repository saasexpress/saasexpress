use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    ptr::null,
    sync::{Arc, Mutex},
    thread::sleep,
};

use crate::operators::message_translator::cel_to_json::cel_value_to_json;
use axum::extract::Query;
use cel_interpreter::{Context, Program, Value, extractors::This, objects::Map};
use fastrace::{local::LocalSpan, trace};
use opentelemetry::{KeyValue, trace::get_active_span};
use saasexpress_core::{
    graph::{
        graph::{AsyncHandleTrait, Graph, Operator, OperatorType},
        message::{Message, OriginMessage},
    },
    settings::settings::{Setting, env_settings},
    timestamp::{NaiveDateTimeExt, now},
};
use saasexpress_core::{
    graph::{message::ControlCommand, meta::NodeMeta},
    settings::settings::ToHashMap,
};
use serde_json::{Value as JsonValue, json};
use tracing::{Level, Span, debug, error, info, info_span, instrument, span};
//use tracing_opentelemetry::OpenTelemetrySpanExt;
use opentelemetry::trace::TraceContextExt;

#[derive(Debug)]
pub(crate) enum MessageTranslatorEngine {
    CelInterpreter { program: Program },
}

impl Display for MessageTranslatorEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageTranslatorEngine::CelInterpreter { .. } => write!(f, "cel-interpreter"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MessageTranslatorMode {
    Expression,
    JSON,
}

impl MessageTranslatorMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "expression" => MessageTranslatorMode::Expression,
            "json" => MessageTranslatorMode::JSON,
            _ => panic!("Unknown mode: {}", value),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MessageTranslator {
    node_fqn: Option<String>,
    template: String,
    engine: MessageTranslatorEngine,

    mode: MessageTranslatorMode,
    in_temp: bool,
    temp_group: Option<String>,
    settings: Vec<Setting>,
}

impl From<serde_yaml::Value> for MessageTranslator {
    fn from(value: serde_yaml::Value) -> Self {
        MessageTranslator {
            node_fqn: None,
            template: value["template"].as_str().unwrap_or("").to_string(),
            in_temp: value["in_temp"].as_bool().unwrap_or(false),
            temp_group: value["temp_group"].as_str().map(|s| s.to_string()),
            settings: Vec::new(),
            mode: MessageTranslatorMode::from(value["mode"].as_str().unwrap_or("json").to_string()),
            engine: value
                .get("engine")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "cel-interpreter" => MessageTranslatorEngine::CelInterpreter {
                        program: Program::compile(value["template"].as_str().unwrap()).unwrap(),
                    },
                    _ => panic!("Unknown engine: {}", s),
                })
                .unwrap_or_else(|| MessageTranslatorEngine::CelInterpreter {
                    program: Program::compile(value["template"].as_str().unwrap()).unwrap(),
                }),
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
            Message::Standard { mut origin, .. } => {
                let input = json!({
                    "resource": self.node_fqn,
                });

                let message = json!({});

                let cel_value = {
                    let temp = &origin.as_ref().unwrap().temp;

                    let temp = temp.lock().unwrap();
                    self.parse(&message, input, &temp)
                };

                if self.in_temp {
                    let temp_group = self.temp_group.clone().unwrap();

                    let og = origin.unwrap();
                    debug!("Pushing into temp: [{}] = {}", temp_group, cel_value);
                    let og = og.temp_push(temp_group, cel_value);

                    Message::JSON {
                        message,
                        origin: Some(
                            OriginMessage::new(og.respond_to)
                                .with_span(og.span)
                                .with_temp(og.temp),
                        ),
                    }
                } else {
                    Message::JSON {
                        message: cel_value,
                        origin,
                    }
                }
            }
            Message::JSON { message, origin } => {
                let input = json!({
                    "resource": self.node_fqn,
                });

                let cel_value = {
                    let temp = &origin.as_ref().unwrap().temp;

                    let temp = temp.lock().unwrap();
                    self.parse(&message, input, &temp)
                };

                if self.in_temp {
                    let temp_group = self.temp_group.clone().unwrap();

                    let og = origin.unwrap();
                    debug!("Pushing into temp: [{}] = {}", temp_group, cel_value);
                    let og = og.temp_push(temp_group, cel_value);

                    Message::JSON {
                        message,
                        origin: Some(
                            OriginMessage::new(og.respond_to)
                                .with_span(og.span)
                                .with_temp(og.temp),
                        ),
                    }
                } else {
                    Message::JSON {
                        message: cel_value,
                        origin,
                    }
                }
            }
            Message::ReqReply {
                message,
                respond_to,
                span,
                temp,
                ..
            } => {
                debug!(
                    "Data message {:?}",
                    String::from_utf8(message.clone()).unwrap()
                );

                let input = json!({
                    "resource": self.node_fqn,
                });

                let json: serde_json::Value = match &message {
                    message if message.is_empty() => serde_json::from_str("{}").unwrap(),
                    _ => serde_json::from_slice(&message).unwrap(),
                };

                let cel_value = {
                    let temp = temp.lock().unwrap();
                    self.parse(&json, input, &temp)
                };

                if self.in_temp {
                    let temp_group = self.temp_group.clone().unwrap();

                    let origin = OriginMessage::new(Some(respond_to))
                        .with_span(span)
                        .with_temp(temp);

                    debug!("Pushing into temp: [{}] = {}", temp_group, cel_value);
                    let origin = origin.temp_push(temp_group, cel_value);

                    Message::Standard {
                        message,
                        origin: Some(origin),
                    }
                } else {
                    Message::JSON {
                        message: cel_value,
                        origin: Some(
                            OriginMessage::new(Some(respond_to))
                                .with_span(span)
                                .with_temp(temp),
                        ),
                    }
                }
            }
            Message::Exit { origin } => Message::Exit { origin },
            Message::Error { error, origin } => return Message::Error { error, origin },
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                    origin: None,
                }
            }
        }
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        self.node_fqn = Some(node_meta.fqn());
        if self.temp_group.is_none() {
            self.temp_group = Some(node_meta.name.to_string());
        }

        self.settings = env_settings(graph.base_env_vars_settings(node_meta))
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

impl MessageTranslator {
    // #[trace(short_name = true, properties = {
    //     "template":"{template:?}"
    // })]
    // fn compile(template: &str) -> Program {
    //     Program::compile(template).unwrap()
    // }

    #[trace(short_name = true)]
    fn parse(&self, data: &JsonValue, input: JsonValue, temp: &JsonValue) -> JsonValue {
        let program = {
            let _guard = LocalSpan::enter_with_local_parent("program");
            match &self.engine {
                MessageTranslatorEngine::CelInterpreter { program } => program,
            }
        };

        let cel_data = {
            let _guard = LocalSpan::enter_with_local_parent("data_serde");
            cel_interpreter::to_value(data).unwrap()
        };

        // Add any variables or functions that the program will need
        let context = {
            let mut context = Context::default();
            let _guard = LocalSpan::enter_with_local_parent("context");

            context = add_functions(context);

            debug!("Templ {}", self.template);
            debug!("Data {}", serde_json::to_string_pretty(data).unwrap());
            debug!("Input {}", serde_json::to_string_pretty(&input).unwrap());
            debug!("Temp {}", serde_json::to_string_pretty(&temp).unwrap());

            context
                .add_variable("data", cel_data)
                .expect("Variable data problem");

            debug!("Settings {:?}", self.settings.to_hash_map());
            context
                .add_variable("settings", self.settings.to_hash_map())
                .expect("Variable data problem");

            context
                .add_variable("input", input)
                .expect("Variable input problem");

            context
                .add_variable("temp", temp)
                .expect("Variable temp problem");

            //sleep(std::time::Duration::from_millis(100));
            context
        };

        {
            let _guard = LocalSpan::enter_with_local_parent("execute");

            // Run the program
            let _value = program.execute(&context);
            match _value {
                Ok(value) => {
                    if self.mode == MessageTranslatorMode::JSON {
                        let val = cel_value_to_json(&value);
                        debug!("Out {}", serde_json::to_string_pretty(&val).unwrap());
                        return val;
                    } else {
                        if let Value::String(value) = &value {
                            return JsonValue::String(value.to_string());
                        } else {
                            error!("Parsing issue - expecting expression not json");
                            return JsonValue::String("".to_string());
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                    return JsonValue::String("".to_string());
                }
            }
        }
    }
}

fn is_empty(This(s): This<Arc<String>>) -> bool {
    error!("is_empty {} {}", s, s.is_empty());
    s.is_empty()
}

fn use_one(a: Arc<String>, b: Arc<String>) -> String {
    error!("use_one {} {}", a, b);
    if !a.is_empty() {
        return a.to_string();
    }
    if !b.is_empty() {
        return b.to_string();
    }
    return "".to_string();
}

fn add_functions(mut context: Context) -> Context {
    context.add_function("add", |a: i64, b: i64| a + b);

    context.add_function("use_one", |a: Arc<String>, b: Arc<String>| use_one(a, b));
    context.add_function("is_empty", is_empty);
    context.add_function("now", || now().to_rfc3339());
    context
}
