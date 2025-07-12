use core::panic;
use std::sync::Arc;

use saasexpress_core::graph::{
    graph::Graph,
    operator::{Operator, OperatorRole, OperatorType},
    operator_types::{
        ai_agent::{AIAgent, AIAgentOperator},
        ai_tool::AITool,
        canonical_model::CanonicalModel,
    },
};

use saasexpress_faker::faker::Faker;

use super::{
    api_call::api_call::APICall, canodamo_sample::CanonicalModelSample, http_in::http_in::HTTPIn,
    message_translator::message_translator::MessageTranslator,
};

use saasexpress_core::operators::factory::add_node_to_graph as add_node_to_graph_core;

// #[derive(Debug)]
// pub enum OperatorSpec {
//     HTTPIn(HTTPIn),
//     APICall(APICall),
//     MessageTranslator(MessageTranslator),
//     Faker(Faker),
// }

// impl From<&serde_yaml::Value> for OperatorSpec {
//     fn from(spec: &serde_yaml::Value) -> Self {
//         let name = spec["action"].as_str().unwrap();
//         let value = spec["config"].clone();
//         match name {
//             "Faker" => OperatorSpec::Faker(Faker::from(value)),
//             "HTTPIn" => OperatorSpec::HTTPIn(HTTPIn::from(value)),
//             "APICall" => OperatorSpec::APICall(APICall::from(value)),
//             "MessageTranslator" => OperatorSpec::MessageTranslator(MessageTranslator::from(value)),
//             _ => panic!("Unknown operator: {}", name),
//         }
//     }
// }

// pub type OpXX = Arc<dyn Operator + 'static>;

// impl Into<OpXX> for OperatorSpec {
//     fn into(self) -> OpXX {
//         match self {
//             OperatorSpec::HTTPIn(op) => Arc::new(op),
//             OperatorSpec::Faker(op) => Arc::new(op),
//             OperatorSpec::APICall(op) => Arc::new(op),
//             OperatorSpec::MessageTranslator(op) => Arc::new(op),
//         }
//     }
// }

pub fn add_node_to_graph(spec: &serde_yaml::Value, graph: &mut Graph) {
    let id = spec["id"].as_str().unwrap();
    let name = spec["operator"].as_str().unwrap();
    let value = spec["config"].clone();
    match name {
        "HTTPIn" => graph.add_node(id, HTTPIn::from(value)),
        "APICall" => graph.add_node(id, APICall::from(value)),
        "Faker" => graph.add_node(id, Faker::from(value)),
        "MessageTranslator" => graph.add_node(id, MessageTranslator::from(value)),
        "CanoName" => graph.add_node(
            id,
            CanonicalModel::new("CanoName", CanonicalModelSample::from(value)),
        ),
        // "AIAgent" => graph.add_node(
        //     id,
        //     AIAgent::new("AIAgent", value.clone(), AIAgentV1::from(value.clone())),
        // ),
        // "AITool" => graph.add_node(id, AITool::new("AITool", AIToolV1::from(value))),
        _ => {
            add_node_to_graph_core(spec, graph);
            graph
        }
    };
}
