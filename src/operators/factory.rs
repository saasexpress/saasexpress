use core::panic;
use std::sync::Arc;


use crate::graph::graph::{Graph, Operator};

use super::{
    api_call::APICall, buffer_to_json::BufferToJSON, fan_out::fan_out::FanOut,
    http_in::http_in::HTTPIn, json_to_buffer::JSONToBuffer,
    message_translator::message_translator::MessageTranslator, passthrough::Passthrough,
    terminate::Terminate,
};

// pub fn operation(name: &str, config: Value) -> impl Operator {
//     if name == "HTTPIn" {
//         return HTTPIn::from(config);
//     } else if name == "BufferToJSON" {
//         return BufferToJSON::from(config);
//     } else {
//         panic!("Unknown operator: {}", name);
//     }
//     // if name == "JSONToBuffer" {
//     //     return JSONToBuffer::from(config);
//     // }
//     // if name == "APICall" {
//     //     return APICall::from(config);
//     // }
//     // if name == "ContentEnricher" {
//     //     return ContentEnricher::from(config);
//     // }
//     // if name == "Passthrough" {
//     //     return Passthrough::from(config);
//     // }
//     // if name == "Terminate" {
//     //     return Terminate::from(config);
//     // }
//     // if name == "Noop" {
//     //     return Passthrough::from(config);
//     // }
// }

#[derive(Debug)]
pub enum OperatorSpec {
    HTTPIn(HTTPIn),
    BufferToJSON(BufferToJSON),
    JSONToBuffer(JSONToBuffer),
    APICall(APICall),
    MessageTranslator(MessageTranslator),
    Passthrough(Passthrough),
    Terminate(Terminate),
    FanOut(FanOut),
}

// impl OperatorSpec {
//     pub fn name(&self) -> String {
//         match self {
//             OperatorSpec::HTTPIn(_) => "HTTPIn".to_string(),
//             OperatorSpec::BufferToJSON(_) => "BufferToJSON".to_string(),
//             OperatorSpec::JSONToBuffer(_) => "JSONToBuffer".to_string(),
//             OperatorSpec::APICall(_) => "APICall".to_string(),
//             OperatorSpec::ContentEnricher(_) => "ContentEnricher".to_string(),
//             OperatorSpec::Passthrough(_) => "Passthrough".to_string(),
//             OperatorSpec::Terminate(_) => "Terminate".to_string(),
//         }
//     }
//     pub fn to_operator(&self) -> Box<dyn Operator> {
//         match self {
//             OperatorSpec::APICall(op) => Box::new(op.clone()),
//             OperatorSpec::ContentEnricher(op) => Box::new(op.clone()),
//             OperatorSpec::Passthrough(op) => Box::new(op.clone()),
//             OperatorSpec::Terminate(op) => Box::new(op.clone()),
//             OperatorSpec::HTTPIn(op) => Box::new(op.clone()),
//             OperatorSpec::BufferToJSON(op) => Box::new(op.clone()),
//             OperatorSpec::JSONToBuffer(op) => Box::new(op.clone()),

//             _ => {
//                 panic!("OperatorSpec::to_operator() called on non-operator type");
//             }
//         }
//     }
// }

impl From<&serde_yaml::Value> for OperatorSpec {
    fn from(spec: &serde_yaml::Value) -> Self {
        let name = spec["action"].as_str().unwrap();
        let value = spec["config"].clone();
        match name {
            "HTTPIn" => OperatorSpec::HTTPIn(HTTPIn::from(value)),
            "BufferToJSON" => OperatorSpec::BufferToJSON(BufferToJSON::from(value)),
            "JSONToBuffer" => OperatorSpec::JSONToBuffer(JSONToBuffer::from(value)),
            "APICall" => OperatorSpec::APICall(APICall::from(value)),
            "MessageTranslator" => OperatorSpec::MessageTranslator(MessageTranslator::from(value)),
            "Passthrough" => OperatorSpec::Passthrough(Passthrough::from(value)),
            "Terminate" => OperatorSpec::Terminate(Terminate::from(value)),
            "FanOut" => OperatorSpec::FanOut(FanOut::from(value)),
            _ => panic!("Unknown operator: {}", name),
        }
    }
}

pub type OpXX = Arc<dyn Operator + 'static>;

impl Into<OpXX> for OperatorSpec {
    fn into(self) -> OpXX {
        match self {
            OperatorSpec::HTTPIn(op) => Arc::new(op),
            OperatorSpec::BufferToJSON(op) => Arc::new(op),
            OperatorSpec::JSONToBuffer(op) => Arc::new(op),
            OperatorSpec::APICall(op) => Arc::new(op),
            OperatorSpec::MessageTranslator(op) => Arc::new(op),
            OperatorSpec::Passthrough(op) => Arc::new(op),
            OperatorSpec::Terminate(op) => Arc::new(op),
            OperatorSpec::FanOut(op) => Arc::new(op),
        }
    }
}

pub fn add_node_to_graph(spec: &serde_yaml::Value, graph: &mut Graph) {
    let id = spec["id"].as_str().unwrap();
    let name = spec["action"].as_str().unwrap();
    let value = spec["config"].clone();
    match name {
        "HTTPIn" => graph.add_node(id, HTTPIn::from(value)),
        "BufferToJSON" => graph.add_node(id, BufferToJSON::from(value)),
        "JSONToBuffer" => graph.add_node(id, JSONToBuffer::from(value)),
        "APICall" => graph.add_node(id, APICall::from(value)),
        "MessageTranslator" => graph.add_node(id, MessageTranslator::from(value)),
        "Passthrough" => graph.add_node(id, Passthrough::from(value)),
        "Terminate" => graph.add_node(id, Terminate::from(value)),
        "FanOut" => graph.add_node(id, FanOut::from(value)),
        _ => panic!("Unknown operator: {}", name),
    };
}
