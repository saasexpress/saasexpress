use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    thread::spawn,
};

use tracing::{error, info, warn};

use super::graph::{Graph, GraphMod};

pub struct GraphRegistry {
    graphs: Vec<Arc<Mutex<Graph>>>,
}

impl GraphRegistry {
    fn new() -> Self {
        GraphRegistry { graphs: Vec::new() }
    }

    pub fn add_graph(&mut self, graph: Graph) -> Arc<Mutex<Graph>> {
        if self.exists(&graph.name) {
            panic!("Graph with name {} already exists", graph.name);
        }
        let arc_graph = Arc::new(Mutex::new(graph));
        let returned_graph = Arc::clone(&arc_graph);
        self.graphs.push(arc_graph);
        returned_graph
    }

    pub fn delete_graph(&mut self, graph_name: &str) -> Result<Arc<Mutex<Graph>>, String> {
        if self.exists(&graph_name) {
            let old_graph = self.graphs.remove(
                self.graphs
                    .iter()
                    .position(|g| g.lock().unwrap().name == graph_name)
                    .unwrap(),
            );
            Ok(old_graph)
        } else {
            Err(format!("Graph with name {} does not exist", graph_name))
        }
    }

    pub fn get_graphs(&self) -> Vec<Arc<Mutex<Graph>>> {
        self.graphs.clone()
    }

    pub fn get_graph_by_name(&self, name: &str) -> Option<Arc<Mutex<Graph>>> {
        fn eq(graph: &Arc<Mutex<Graph>>, name: &str) -> bool {
            graph.try_lock().is_ok() && graph.lock().unwrap().name == name
        }

        let graph = self.graphs.iter().find(|graph| eq(graph, name));

        match graph {
            Some(graph) => Some(Arc::clone(graph)),
            None => {
                info!("Graph not found {} - might be fine", name);
                None
            }
        }
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

    pub fn get_graph(graph_name: &str) -> Option<Arc<Mutex<Graph>>> {
        let graph_registry = GraphRegistry::get_instance();

        let graph_registry = graph_registry.lock();
        match graph_registry {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to lock graph registry for {} {}", graph_name, err);
                return None;
            }
        }
        let graph_registry = graph_registry.unwrap();

        graph_registry.get_graph_by_name(&graph_name)
    }

    pub fn clear(&mut self) {
        while self.graphs.len() > 0 {
            info!("POPPING");
            self.graphs.pop();
            info!("POPPED");
        }
    }
}

static INSTANCE: OnceLock<Mutex<GraphRegistry>> = OnceLock::new();
