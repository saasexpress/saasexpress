use graph::graph::Graph;
use rust_embed::Embed;
use serde_yaml::Value;
use tracing::info;

use crate::graph::{
    graph::Operator,
    operators::{
        factory::{self, add_node_to_graph, OpXX},
        http_in::{http_in::HTTPIn, resources::get_instance},
    },
};

use super::graph;

#[derive(Embed)]
#[folder = "bootstrap"]
struct Asset;

pub fn aa(config: Value) -> impl Operator {
    HTTPIn::from(config)
}

pub fn bootstrap() {
    for file_name in Asset::iter() {
        let file = Asset::get(file_name.as_ref()).unwrap();

        let yaml = serde_yaml::from_slice::<serde_yaml::Value>(file.data.as_ref()).unwrap();
        info!("YAML: {} : {:?}", file_name, yaml);

        build_graph(yaml);
    }

    // Start the resources that the graphs are dependend on
    let singleton = get_instance().lock().unwrap();
    singleton.start();
}

fn build_graph(yaml: Value) {
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
