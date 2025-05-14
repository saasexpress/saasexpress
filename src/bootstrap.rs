use fastrace::trace;
use saasexpress_core::graph::graph::Graph;
use serde_yaml::Value;
use tracing::{Span, debug, info, instrument};

use crate::operators::factory::add_node_to_graph;
use crate::operators::http_in;

pub fn bootstrap() {
    info!("Starting HTTP service");
    let singleton = http_in::resources::get_instance().lock().unwrap();
    singleton.start();
}

#[trace]
pub fn build_graph(yaml: Value) -> Graph {
    let graph_name = yaml["name"].as_str().unwrap().to_string();

    // Record graph name in the current span
    //Span::current().record("graph_name", &graph_name);

    debug!(graph_name = %graph_name, "Building graph");
    let mut graph = Graph::new(graph_name);

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

    graph.no_processor().init();
    graph
}
