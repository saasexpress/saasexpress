use std::path::Path;

use fastrace::trace;
use saasexpress_core::graph::graph::{Graph, GraphStatus};
use saasexpress_core::graph::operator::OperatorRef;
use saasexpress_core::graph::registry::GraphRegistry;
use saasexpress_core::my_reg::{ControlEvent, broadcast_event, deregister};
use saasexpress_core::{graph, start_graphs};
use serde_yaml::Value;
use tokio::sync::mpsc;
use tracing::{Span, debug, error, info, instrument};

use crate::http_in::resources::get_instance;
use crate::operators::factory::add_node_to_graph;
use crate::operators::http_in;

pub fn bootstrap() {
    info!("Starting HTTP service");
    let singleton = get_instance().unwrap();
    let mut singleton = singleton.lock().unwrap();
    singleton.watch_control_bus();
    singleton.start();
}

#[trace]
pub fn build_graph(yaml: Value) -> String {
    let graph_name = yaml["name"].as_str().unwrap().to_string();

    // Record graph name in the current span
    //Span::current().record("graph_name", &graph_name);

    debug!(graph_name = %graph_name, "Building graph");
    let mut graph = Graph::new(graph_name.clone());

    // Create a span for node processing
    // let nodes_span = tracing::info_span!(
    //     "process_nodes",
    //     node_count = yaml["nodes"].as_sequence().unwrap().len()
    // );
    // let _nodes_guard = nodes_span.enter();

    for (idx, node) in yaml["nodes"].as_sequence().unwrap().iter().enumerate() {
        // let node_span = tracing::info_span!("add_node", node_index = idx);
        // let _node_guard = node_span.enter();

        // let node_type = node["type"].as_str().unwrap_or("unknown");
        // tracing::info!(node_type = %node_type, "Adding node to graph");

        add_node_to_graph(node, &mut graph);
    }

    //drop(_nodes_guard); // Exit the nodes span

    // Create a span for edge processing
    // let edges_span = tracing::info_span!(
    //     "process_edges",
    //     edge_count = yaml["edges"].as_sequence().unwrap().len()
    // );
    // let _edges_guard = edges_span.enter();

    for (idx, edge) in yaml["edges"].as_sequence().unwrap().iter().enumerate() {
        // let edge_span = tracing::info_span!("add_edge", edge_index = idx);
        // let _edge_guard = edge_span.enter();

        let from = edge["from"].as_str().unwrap();
        let to = edge["to"].as_str().unwrap();
        let role = edge["role"].as_str().unwrap_or("default");
        debug!(from = %from, to = %to, "Adding edge");
        graph.add_edge(String::from(from), String::from(to), String::from(role));
    }

    // drop(_edges_guard); // Exit the edges span

    // let init_span = tracing::info_span!("graph_initialization");
    // let _init_guard = init_span.enter();

    graph.no_processor();
    graph.register();

    let graph = GraphRegistry::get_graph(graph_name.as_str()).unwrap();

    let mut graph = graph.lock().unwrap();

    graph.make_active_if_ready();
    graph.replace_runner();

    graph_name
}

pub fn reload_graph(path: String) {
    serde_yaml::from_reader::<_, Value>(std::fs::File::open(path).unwrap())
        .map(|yaml| {
            let graph_name = yaml["name"].as_str().unwrap();
            remove_graph(graph_name);

            let _ = build_graph(yaml);
        })
        .unwrap_or_else(|e| {
            error!("Failed to reload graph: {}", e);
        });
}

#[trace]
pub fn remove_graph(graph_name: &str) {
    info!("Removing graph: {}", graph_name);

    let graph_registry = GraphRegistry::get_instance();
    let mut graph_registry = graph_registry.lock().unwrap_or_else(|err| {
        error!("Failed to lock graph registry: {}", err);
        panic!("Failed to lock graph registry: {}", err);
    });
    let graph = graph_registry.delete_graph(graph_name);
    match graph {
        Ok(graph) => {
            info!("Graph removed: {}", graph_name);
            let _graph = graph.lock().unwrap();
        }
        Err(err) => {
            error!("Failed to remove graph: {}", err);
        }
    }
}
