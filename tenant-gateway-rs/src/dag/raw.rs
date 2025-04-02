use log::error;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use std::io;

use super::dag::Graph;

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
    pub fn load_yaml(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
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

        raw_dag.translate_to_dag()
    }

    pub fn load_yaml_from_value(
        value: serde_json::Value,
    ) -> Result<Graph, Box<dyn std::error::Error>> {
        let raw_dag: RawGraph = serde_json::from_value(value).map_err(|e| {
            error!("Failed to parse Graph YAML from value: {}", e);
            e
        })?;

        raw_dag.translate_to_dag()
    }

    /// Translates a RawGraph into a Graph
    ///
    /// # Errors
    ///
    /// Returns an error if the Graph structure is invalid (e.g., contains cycles)
    fn translate_to_dag(self) -> Result<Graph, Box<dyn std::error::Error>> {
        let mut dag = Graph::new(self.name, self.nodes[0].id.clone());

        // First add all nodes
        for node in self.nodes {
            dag.add_node(node);
        }

        // Then add all edges
        for edge in self.edges {
            dag.add_edge(edge.from, edge.to).map_err(|e| {
                error!("Failed to add edge: {}", e);
                io::Error::new(io::ErrorKind::InvalidData, e)
            })?;
        }

        Ok(dag)
    }
}
