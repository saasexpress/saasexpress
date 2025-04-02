use log::error;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};

use crate::operators::operator::OperatorNode;
use crate::operators::registry::Registry;

use super::raw::{RawGraph, RawNode};

/// Represents a Directed Acyclic Graph (Graph) for workflow processing
pub struct Graph {
    /// Name of the Graph
    name: String,
    /// Set first node to start Node
    start_node: String,
    /// Collection of nodes in the Graph, indexed by their unique ID
    nodes: HashMap<String, Node>,
    /// Mapping of node IDs to their outgoing edges (children)
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    /// Creates a new empty Graph with the given name
    pub fn new(name: String, start_node: String) -> Self {
        Graph {
            name,
            start_node,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Returns a mutable reference to the nodes collection
    pub fn get_nodes(&mut self) -> &mut HashMap<String, Node> {
        &mut self.nodes
    }

    /// Returns the name of the Graph
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the name of the starting node
    pub fn get_start_node(&self) -> &String {
        &self.start_node
    }

    /// Adds a node to the Graph from a RawNode configuration
    pub fn add_node(&mut self, node: RawNode) {
        let node = Node {
            id: node.id,
            action: node.action,
            config: node.config,
            children: HashSet::new(),
        };

        self.nodes.insert(node.id.clone(), node);
    }

    /// Adds a directed edge between two nodes
    ///
    /// Returns an error if:
    /// - Either node doesn't exist in the Graph
    /// - Adding the edge would create a cycle
    pub fn add_edge(&mut self, from: String, to: String) -> Result<(), String> {
        let to_k = to.clone();
        if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
            return Err("Both nodes must be in the Graph".to_string());
        }
        if self.has_path(&to, &from) {
            return Err("Adding this edge would create a cycle".to_string());
        }
        if let Some(from_node) = self.nodes.get_mut(&from) {
            from_node.children.insert(to_k.clone());
        }

        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);

        Ok(())
    }

    /// Checks if there's a path from one node to another
    ///
    /// Used for cycle detection when adding edges
    pub fn has_path(&self, from: &String, to: &String) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![from.clone()];

        while let Some(node) = stack.pop() {
            if node == *to {
                return true;
            }

            if !visited.contains(&node) {
                visited.insert(node.clone());
                if let Some(neighbors) = self.edges.get(&node) {
                    for neighbor in neighbors {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        false
    }

    /// Creates a new Graph from a YAML file specification
    pub fn new_using_yaml(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
        RawGraph::load_yaml(file_path)
    }

    pub fn new_using_value(value: serde_json::Value) -> Result<Graph, Box<dyn std::error::Error>> {
        RawGraph::load_yaml_from_value(value)
    }

    /// Returns a list of all root nodes (nodes with no incoming edges)
    pub fn get_root_nodes(&self) -> Vec<&Node> {
        let mut incoming_edges: HashMap<&String, usize> = HashMap::new();

        // Initialize all nodes with zero incoming edges
        for node_id in self.nodes.keys() {
            incoming_edges.insert(node_id, 0);
        }

        // Count incoming edges for each node
        for (_node_id, children) in &self.edges {
            for child in children {
                *incoming_edges.entry(child).or_insert(0) += 1;
            }
        }

        // Return nodes with zero incoming edges
        self.nodes
            .values()
            .filter(|node| {
                incoming_edges
                    .get(node.get_id())
                    .map_or(true, |&count| count == 0)
            })
            .collect()
    }
}

/// Represents a single node in the Graph
pub struct Node {
    /// Unique identifier for the node
    id: String,
    /// The operation/action this node represents
    action: String,
    /// Configuration values for this node
    pub config: Value,
    /// Set of child node IDs
    children: HashSet<String>,
}

impl Node {
    /// Returns the unique ID of this node
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Returns the action type of this node
    pub fn get_action(&self) -> &String {
        &self.action
    }

    /// Returns a copy of the set of child node IDs
    pub fn children(&self) -> HashSet<String> {
        self.children.clone()
    }

    /// Initializes the operator for this node using the provided registry
    ///
    /// # Panics
    ///
    /// Panics if node setup fails
    pub fn init_operator(&self, registry: &Registry) -> Box<dyn OperatorNode + Send> {
        let op = registry
            .get(&self.action)
            .expect("Operator not found in registry");
        //let _ = op.setup_node_2(self);

        match op.setup_node(self) {
            Ok(node) => {
                let node_ref = node.as_ref();
                node_ref.speak();
                node
            }
            Err(e) => {
                error!("Failed to setup node {}: {}", self.id, e);
                panic!("Failed to setup node: {}", e);
            }
        }
    }
}
