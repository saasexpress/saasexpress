use core::panic;
use std::sync::Arc;

use crate::graph::graph::{Graph, Operator};

use super::{
    buffer_to_json::BufferToJSON, callout::Callout, fan_out::fan_out::FanOut,
    json_to_buffer::JSONToBuffer, passthrough::Passthrough, shell::shell::Shell, stub::Stub,
    terminate::Terminate, timer::Timer,
};

#[derive(Debug)]
pub enum OperatorSpec {
    BufferToJSON(BufferToJSON),
    JSONToBuffer(JSONToBuffer),
    Passthrough(Passthrough),
    Terminate(Terminate),
    FanOut(FanOut),
    Shell(Shell),
    Callout(Callout),
    Stub(Stub),
    Timer(Timer),
}

impl From<&serde_yaml::Value> for OperatorSpec {
    fn from(spec: &serde_yaml::Value) -> Self {
        let name = spec["action"].as_str().unwrap();
        let value = spec["config"].clone();
        match name {
            "BufferToJSON" => OperatorSpec::BufferToJSON(BufferToJSON::from(value)),
            "JSONToBuffer" => OperatorSpec::JSONToBuffer(JSONToBuffer::from(value)),
            "Passthrough" => OperatorSpec::Passthrough(Passthrough::from(value)),
            "Terminate" => OperatorSpec::Terminate(Terminate::from(value)),
            "FanOut" => OperatorSpec::FanOut(FanOut::from(value)),
            "Shell" => OperatorSpec::Shell(Shell::from(value)),
            "Callout" => OperatorSpec::Callout(Callout::from(value)),
            "Stub" => OperatorSpec::Stub(Stub::from(value)),
            "Timer" => OperatorSpec::Timer(Timer::from(value)),
            _ => panic!("Unknown operator: {}", name),
        }
    }
}

pub type OpXX = Arc<dyn Operator + 'static>;

impl Into<OpXX> for OperatorSpec {
    fn into(self) -> OpXX {
        match self {
            OperatorSpec::Callout(op) => Arc::new(op),
            OperatorSpec::BufferToJSON(op) => Arc::new(op),
            OperatorSpec::JSONToBuffer(op) => Arc::new(op),
            OperatorSpec::Passthrough(op) => Arc::new(op),
            OperatorSpec::Terminate(op) => Arc::new(op),
            OperatorSpec::FanOut(op) => Arc::new(op),
            OperatorSpec::Shell(op) => Arc::new(op),
            OperatorSpec::Stub(op) => Arc::new(op),
            OperatorSpec::Timer(op) => Arc::new(op),
        }
    }
}

pub fn add_node_to_graph(spec: &serde_yaml::Value, graph: &mut Graph) {
    let id = spec["id"].as_str().unwrap();
    let name = spec["action"].as_str().unwrap();
    let value = spec["config"].clone();
    match name {
        "Callout" => graph.add_node(id, Callout::from(value)),
        "BufferToJSON" => graph.add_node(id, BufferToJSON::from(value)),
        "JSONToBuffer" => graph.add_node(id, JSONToBuffer::from(value)),
        "Passthrough" => graph.add_node(id, Passthrough::from(value)),
        "Terminate" => graph.add_node(id, Terminate::from(value)),
        "FanOut" => graph.add_node(id, FanOut::from(value)),
        "Shell" => graph.add_node(id, Shell::from(value)),
        "Stub" => graph.add_node(id, Stub::from(value)),
        "Timer" => graph.add_node(id, Timer::from(value)),
        _ => panic!("Unknown operator: {}", name),
    };
}
