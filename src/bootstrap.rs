use rust_embed::Embed;
use saasexpress_core::graph::graph::Graph;
use serde_yaml::Value;
use tracing::info;

use crate::operators::factory::add_node_to_graph;
use crate::operators::http_in;

pub fn bootstrap() {
    let singleton = http_in::resources::get_instance().lock().unwrap();
    singleton.start();
}

pub fn build_graph(yaml: Value) {
    let graph_name = yaml["name"].as_str().unwrap().to_string();

    let mut graph = Graph::new(graph_name);

    for node in yaml["nodes"].as_sequence().unwrap() {
        add_node_to_graph(node, &mut graph);
    }

    for edge in yaml["edges"].as_sequence().unwrap() {
        let from = edge["from"].as_str().unwrap();
        let to = edge["to"].as_str().unwrap();
        info!("Edge: {} -> {}", from, to);
        graph.add_edge(String::from(from), String::from(to));
    }

    graph.no_processor().init();
}
