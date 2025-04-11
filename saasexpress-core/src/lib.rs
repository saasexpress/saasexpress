use graph::graph::Graph;
use operators::factory::add_node_to_graph;
use serde_yaml::Value;
use tracing::info;

pub mod graph;
pub mod operators;
mod ports;
pub mod settings;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_graph(yaml: Value) -> Graph {
    let graph_name = yaml["name"].as_str().unwrap().to_string();

    let mut graph = Graph::new(graph_name);

    for node in yaml["nodes"].as_sequence().unwrap() {
        add_node_to_graph(node, &mut graph);
    }

    if yaml.get("edges").is_some() {
        for edge in yaml["edges"].as_sequence().unwrap() {
            let from = edge["from"].as_str().unwrap();
            let to = edge["to"].as_str().unwrap();
            graph.add_edge(String::from(from), String::from(to));
        }
    }

    graph.no_processor().init();
    graph
}

#[cfg(test)]
mod saasexpress_core_tests {
    use tracing::{Level, debug};

    use crate::graph::graph::GraphRun;
    use crate::{graph::message::Message, settings::settings::env_settings};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn env_settings_works() {
        let settings = env_settings("TEST".to_string());
        assert_eq!(settings.len(), 0);
    }

    #[tokio::test]
    async fn buffertojson_works() {
        const GRAPH: &str = r#"
        name: buffer_to_json
        nodes:
          - id: start
            action: BufferToJSON
        "#;
        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "buffer_to_json");

        let response = graph.end_to_end("{}".as_bytes().to_vec()).await;

        debug!("Message: {:?}", response);
        let Message::JSON { message, .. } = response else {
            panic!("Expected JSON message");
        };

        assert_eq!(message.get("_ts").is_some(), true);
    }

    #[tokio::test]
    async fn shell_works() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: shell
        nodes:
          - id: start
            action: BufferToJSON
          - id: j2b
            action: JSONToBuffer
          - id: shell
            action: Shell
            config:
              command: bash
              args:
                - pwd

        edges:
          - from: start
            to: j2b
          - from: j2b
            to: shell
        "#;
        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "shell");

        let response = graph.end_to_end_2("{}".as_bytes().to_vec()).await;

        let Message::JSON { message, .. } = response else {
            panic!("Expected Standard message");
        };

        info!(
            "Message: {}",
            serde_json::to_string_pretty(&message).unwrap()
        );
        assert_eq!(message.as_array().unwrap().len(), 2);
    }
}
