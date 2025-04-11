use rust_embed::Embed;
use saasexpress_core::graph::graph::Graph;
use serde_yaml::Value;
use tracing::info;

use crate::operators::factory::add_node_to_graph;
use crate::operators::http_in::resources::get_instance;

#[derive(Embed)]
#[folder = "src/commands"]
struct Asset;

pub fn bootstrap(graphs: Vec<Value>) {
    graphs.iter().for_each(|yaml| build_graph(yaml.to_owned()));

    // Start the resources that the graphs are dependend on
    let singleton = get_instance().lock().unwrap();
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

fn gather_files() -> Vec<Value> {
    Asset::iter()
        .filter(|file_name| file_name.ends_with(".yaml"))
        .map(|file_name| {
            let file = Asset::get(file_name.as_ref()).unwrap();
            let yaml = serde_yaml::from_slice::<serde_yaml::Value>(file.data.as_ref()).unwrap();
            info!("YAML: {} : {:?}", file_name, yaml);
            yaml
        })
        .collect()
}
