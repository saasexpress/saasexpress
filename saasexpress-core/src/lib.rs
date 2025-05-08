use graph::graph::Graph;
use operators::factory::add_node_to_graph;
use serde_yaml::Value;

pub mod graph;
pub mod operators;
mod ports;
pub mod settings;
pub mod timestamp;

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
    use serde_json::json;
    use tracing::{Level, debug, info};

    use crate::graph::graph::GraphRun;
    use crate::graph::registry::GraphRegistry;
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
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

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
    async fn claimcheck_works() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: claim_check
        nodes:
          - id: start
            action: ClaimCheck
        "#;
        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "claim_check");

        let response = graph.end_to_end_standard("{}".as_bytes().to_vec()).await;

        debug!("Message: {:?}", response);
        let Message::JSON { message, .. } = response else {
            panic!("Expected JSON message");
        };

        assert_eq!(
            message
                .get("claim_type")
                .unwrap_or(&serde_json::Value::String("".to_string())),
            "filesystem"
        );
    }

    #[tokio::test]
    async fn shell_works() {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();

        const GRAPH: &str = r#"
        name: shell
        nodes:
          - id: start
            action: BufferToJSON
          - id: shell
            action: Shell
            config:
              command: bash
              args:
                - pwd

        edges:
          - from: start
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

    #[tokio::test]
    async fn test_fan_out() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: fan_out
        nodes:
        - id: fanout
          action: FanOut
        - id: fanout_1
          action: Passthrough
        - id: fanout_2
          action: Passthrough
        edges:
        - from: fanout
          to: fanout_1
        - from: fanout
          to: fanout_2
        "#;

        info!("Graph: {}", GRAPH);
        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "fan_out");

        let response = graph.end_to_end_json(json!({"name":"joe"})).await;

        let Message::JSON { message, .. } = response else {
            panic!("Expected Standard message");
        };

        assert_eq!(message.as_array().unwrap().len(), 2);
        assert_eq!(message[0].get("name").unwrap(), "joe");
        assert_eq!(message[1].get("name").unwrap(), "joe");
    }

    #[tokio::test]
    async fn test_callout() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: callout
        nodes:
        - id: callout
          action: Callout
          config:
            graph_name: worker
        edges: []
        "#;

        const GRAPH_WORKER: &str = r#"
        name: worker
        nodes:
        - id: start
          action: Stub
          config:
            name: Joe
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph_worker = build_graph(serde_yaml::from_str(GRAPH_WORKER).unwrap());
        let graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        let graph_registry = GraphRegistry::get_instance();
        graph_registry.lock().unwrap().add_graph(graph_worker);
        graph_registry.lock().unwrap().add_graph(graph);

        let reg = graph_registry
            .lock()
            .unwrap()
            .get_graph_by_name("callout")
            .unwrap();

        let mut graph = reg.lock().unwrap();

        graph.finalize();

        assert_eq!(graph.name, "callout");

        let response = graph.end_to_end_standard("hello".as_bytes().to_vec()).await;

        info!("Response : {:?}", response);

        let Message::Tuple {
            message_1,
            message_2,
            ..
        } = response
        else {
            panic!("Expected Standard message");
        };

        let Message::Standard { message, .. } = message_1.as_ref() else {
            panic!("Expected Standard message");
        };

        assert_eq!(message.to_vec(), "hello".as_bytes().to_vec());

        let Message::JSON { message, .. } = message_2.as_ref() else {
            panic!("Expected Standard message");
        };

        let nm = message.get("name").unwrap();
        assert_eq!(nm, "Joe");
    }

    #[tokio::test]
    async fn test_settings() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: settings
        nodes:
        - id: settings
          action: Settings
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        let graph_registry = GraphRegistry::get_instance();
        {
            graph_registry.lock().unwrap().add_graph(graph);
        }

        let graphs = graph_registry.lock().unwrap().get_graphs();

        graphs.iter().for_each(|graph| {
            let mut graph = graph.lock().unwrap();
            info!("Graph: {} {}", graph.name, graph.nodes.len());
            graph.finalize();
        });

        let reg = {
            graph_registry
                .lock()
                .unwrap()
                .get_graph_by_name("settings")
                .unwrap()
        };
        let mut graph = { reg.lock().unwrap() };

        assert_eq!(graph.name, "settings");

        let response = graph.end_to_end_json(json!({"name": "Joe"})).await;

        info!("Response : {:?}", response);

        let Message::JSON { message, .. } = response else {
            panic!("Expected Standard message");
        };

        assert_eq!(message.get("name").unwrap(), "Joe");
    }

    #[tokio::test]
    async fn test_canodamo_sample_ok() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: canonical_model
        nodes:
        - id: start
          action: CanonicalModelSample
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "canonical_model");

        let response = graph.end_to_end_json(json!({"name": "Joe"})).await;

        info!("Response : {:?}", response);

        let Message::JSON { message, .. } = response else {
            panic!("Expected Standard message");
        };

        assert_eq!(message.get("name").unwrap(), "Joe");
    }

    #[tokio::test]
    async fn test_canodamo_sample_error() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: canonical_model
        nodes:
        - id: start
          action: CanonicalModelSample
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let mut graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        assert_eq!(graph.name, "canonical_model");

        let response = graph.end_to_end_json(json!({"first": "Joe"})).await;

        info!("Response : {:?}", response);

        let Message::Error { error, .. } = response else {
            panic!("Expected Error message");
        };

        assert_eq!(
            error,
            "Canonical Model Validation Error - missing field `name`"
        );
    }
}
