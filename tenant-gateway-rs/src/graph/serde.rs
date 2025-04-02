use log::error;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use std::io;
use std::sync::Arc;

use super::graph::Graph;
use super::graph::Operator;
use super::graph::OperatorType;
use super::operators::http_in::http_in::HTTPIn;

/// Raw representation of a Graph as loaded from YAML
#[derive(Deserialize)]
pub(super) struct RawGraph {
    /// Name of the Graph
    name: String,
    /// Collection of nodes in the Graph
    nodes: Vec<RawNode>,
    /// Collection of edges between nodes
    edges: Vec<Edge>,
}

/// Raw representation of a node in the Graph
#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct RawNode {
    /// Unique identifier for the node
    pub(super) id: String,
    /// The operation/action this node represents
    pub(super) action: String,
    /// Configuration values for this node
    pub(super) config: Value,
}

/// Represents a directed edge between two nodes
#[derive(Deserialize)]
struct Edge {
    /// Source node ID
    from: String,
    /// Destination node ID
    to: String,
}

impl RawGraph {
    /// Loads a Graph from a YAML file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML is invalid
    /// - The Graph structure is invalid (e.g., contains cycles)
    pub fn load_yaml(file_path: &str) -> Result<RawGraph, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path).map_err(|e| {
            error!("Failed to read Graph file {}: {}", file_path, e);
            io::Error::new(
                e.kind(),
                format!("Failed to read Graph file {}: {}", file_path, e),
            )
        })?;

        let raw_dag: RawGraph = serde_yaml::from_str(&file_content).map_err(|e| {
            error!("Failed to parse Graph YAML from {}: {}", file_path, e);
            e
        })?;
        Ok(raw_dag)
    }

    pub fn load_yaml_from_value(
        value: serde_json::Value,
    ) -> Result<RawGraph, Box<dyn std::error::Error>> {
        let raw_dag: RawGraph = serde_json::from_value(value).map_err(|e| {
            error!("Failed to parse Graph YAML from value: {}", e);
            e
        })?;

        Ok(raw_dag)
    }
}

impl From<RawNode> for OperatorType {
    fn from(node: RawNode) -> Self {
        let config = serde_json::to_value(node.config).unwrap();
        HTTPIn::from(config);
        match node.action.as_str() {
            // "BufferToJSON" => OperatorType::BufferToJSON { node },
            // "JSONToBuffer" => OperatorType::JSONToBuffer { node },
            _ => panic!("Unknown operator type: {}", node.action),
        }
    }
}

impl From<RawGraph> for Graph {
    fn from(raw_graph: RawGraph) -> Self {
        let mut graph = Graph::new(raw_graph.name);

        for node in raw_graph.nodes {
            // let op: Arc<dyn Operator + 'static>;

            // let operator = OperatorType::from(node);
            // graph.add_node(&node.id, op);
        }

        for edge in raw_graph.edges {
            graph.add_edge(edge.from, edge.to);
        }

        graph
    }
}
