use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

use super::graph::{Graph, GraphMod, GraphRun};

pub struct GraphRegistry {
    graphs: Vec<Arc<Mutex<Graph>>>,
}

impl GraphRegistry {
    fn new() -> Self {
        GraphRegistry { graphs: Vec::new() }
    }

    pub fn add_graph(&mut self, graph: Graph) {
        if self.exists(&graph.name) {
            panic!("Graph with name {} already exists", graph.name);
        }
        self.graphs.push(Arc::new(Mutex::new((graph))));
    }

    pub fn get_graphs(&self) -> Vec<Arc<Mutex<Graph>>> {
        self.graphs.clone()
    }

    pub fn get_graph_by_name(&self, name: &str) -> Option<Arc<Mutex<Graph>>> {
        Some(Arc::clone(
            self.graphs
                .iter()
                .find(|graph| graph.lock().unwrap().name == name)
                .unwrap(),
        ))
    }

    pub fn iterate_graphs(&self) -> Vec<Arc<Mutex<Graph>>> {
        self.graphs.iter().map(|graph| Arc::clone(graph)).collect()
    }

    pub fn get_instance() -> &'static Mutex<GraphRegistry> {
        INSTANCE.get_or_init(|| Mutex::new(GraphRegistry::new()))
    }

    fn exists(&self, name: &str) -> bool {
        self.graphs
            .iter()
            .any(|graph| graph.lock().unwrap().name == name)
    }
}

static INSTANCE: OnceLock<Mutex<GraphRegistry>> = OnceLock::new();
