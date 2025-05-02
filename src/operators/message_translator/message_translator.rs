use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
    thread::sleep,
};

use crate::operators::message_translator::cel_to_json::cel_value_to_json;
use cel_interpreter::{Context, Program, Value};
use fastrace::local::LocalSpan;
use opentelemetry::{KeyValue, trace::get_active_span};
use saasexpress_core::settings::settings::ToHashMap;
use saasexpress_core::{
    graph::{
        graph::{AsyncHandleTrait, Graph, Operator, OperatorType},
        message::{Message, OriginMessage},
    },
    settings::settings::{Setting, env_settings},
};
use serde_json::{Value as JsonValue, json};
use tracing::{Level, debug, error, info, info_span, instrument, span};
//use tracing_opentelemetry::OpenTelemetrySpanExt;
use opentelemetry::trace::TraceContextExt;

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
    template: String,
    engine: MessageTranslatorEngine,
    mode: MessageTranslatorMode,
    settings: Vec<Setting>,
}

impl From<serde_yaml::Value> for MessageTranslator {
    fn from(value: serde_yaml::Value) -> Self {
        MessageTranslator {
            template: value["template"].as_str().unwrap_or("").to_string(),
            settings: env_settings("MESSAGE_TRANSLATOR".to_string()),
            mode: MessageTranslatorMode::from(value["mode"].as_str().unwrap_or("json").to_string()),
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

    //#[instrument(name = "message_translator_parse", skip_all)]
    //#[tracing::instrument(skip(self), fields(name = self.name()))]
    fn handle(&self, _message: Message) -> Message {
        match _message {
            Message::JSON { message, origin } => {
                let cel_value = self.parse(&message);

                Message::JSON {
                    message: cel_value,
                    origin,
                }
            }
            Message::ReqReply {
                message,
                respond_to,
                span,
                ..
            } => {
                debug!(
                    "Data message {:?}",
                    String::from_utf8(message.clone()).unwrap()
                );

                let json: serde_json::Value = match message {
                    message if message.is_empty() => serde_json::from_str("{}").unwrap(),
                    _ => serde_json::from_slice(&message).unwrap(),
                };

                let cel_value = self.parse(&json);

                Message::JSON {
                    message: cel_value,
                    origin: Some(OriginMessage::new(Some(respond_to)).with_span(span)),
                }
            }
            Message::Exit { origin } => Message::Exit { origin },
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
            _ => {
                error!("Unexpected message type {}", _message);
                Message::Error {
                    error: "Unexpected message type".to_string(),
                }
            }
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
    //#[instrument(name = "message_translator_parse", skip_all)]

    fn parse(&self, data: &JsonValue) -> JsonValue {
        let program;

        {
            let _span = LocalSpan::enter_with_local_parent("compile");

            program = Program::compile(&self.template).unwrap();
        }

        let cel_data = cel_interpreter::to_value(data).unwrap();

        // Add any variables or functions that the program will need
        let mut context = Context::default();
        context.add_function("add", |a: i64, b: i64| a + b);

        debug!("Templ {}", self.template);
        debug!("In {}", serde_json::to_string_pretty(data).unwrap());

        // get_active_span(|span| {
        //     let count = 10;
        //     let q = 10.0;
        //     //let q = create_quote_from_float(f);
        //     span.add_event(
        //         "Received Quote".to_string(),
        //         vec![KeyValue::new("app.shipping.cost.total", format!("{}", q))],
        //     );
        //     span.set_attribute(KeyValue::new("app.shipping.items.count", count as i64));
        //     span.set_attribute(KeyValue::new("app.shipping.cost.total", format!("{}", q)));
        //     q
        // });

        //let span = span!(Level::INFO, "my_span");
        // span.in_scope(|| {
        //     // Record attributes in the span
        //     let _gaurd = span.enter();

        //     sleep(std::time::Duration::from_millis(1000));
        // });

        let input = json!({
            "resource": "Tenant",
            "http_method": "POST",
            "query": {
                "prompt": "Hello World"
            }
        });

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

        let _span = LocalSpan::enter_with_local_parent("execute");

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
