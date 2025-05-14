use control_bus::ControlEvent;
use graph::{
    graph::{Graph, GraphBroadcastMessage, GraphStatus, OperatorState},
    registry::GraphRegistry,
};
use tokio::time::timeout;

use my_reg::{broadcast_event, register};
use operators::factory::add_node_to_graph;
use serde_yaml::Value;
use tokio::sync::{
    broadcast::{Receiver, Sender},
    mpsc,
};
use tracing::{debug, info};

mod broker;
pub mod control_bus;
pub mod graph;
pub mod my_reg;
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
            let role = edge["role"].as_str().unwrap_or_else(|| "default");
            graph.add_edge(String::from(from), String::from(to), String::from(role));
        }
    }

    graph.no_processor().init();

    graph
}

pub async fn start_graphs() {
    let graph_registry = GraphRegistry::get_instance();

    let graph_count = {
        let graph_registry = graph_registry.lock().unwrap();
        graph_registry.get_graphs().len()
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ControlEvent>(100);

    register("startup", tx);

    {
        let graph_registry = graph_registry.lock().unwrap();
        let graphs = graph_registry.get_graphs();
        for graph in graphs.iter() {
            let graph_name = graph.lock().unwrap().name.clone();
            broadcast_event(ControlEvent {
                graph_name,
                state: GraphStatus::Starting,
                operator_names: vec![],
            })
            .await;
        }
    }

    let my_duration = tokio::time::Duration::from_millis(1000);
    let mut counter = 0;

    loop {
        let msg = timeout(my_duration, rx.recv()).await;
        match msg {
            Ok(msg) => match msg {
                Some(msg) => {
                    if msg.state == GraphStatus::Running {
                        counter += 1;
                    }
                    info!(
                        "Received message: {:?} (Ready={}/{})",
                        serde_json::to_string(&msg),
                        counter,
                        graph_count
                    );
                    if counter == graph_count {
                        break;
                    }
                }
                None => {
                    panic!("No message received");
                }
            },
            Err(_) => {
                panic!("Timeout waiting for message");
            }
        }
    }
    info!("All systems a go!");
}

#[cfg(test)]
mod saasexpress_core_tests {
    use std::thread::sleep;

    use serde_json::json;
    use tokio::sync::broadcast;
    use tracing::{Level, debug, info};
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::control_bus::ControlEvent;
    use crate::graph::graph::{GraphBroadcastMessage, GraphRun};
    use crate::graph::registry::GraphRegistry;
    use crate::my_reg::{broadcast_event, example, register};
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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn claimcheck_works() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .init();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn shell_works() {
        // tracing_subscriber::fmt().with_max_level(Level::INFO).init();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_fan_out() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .set_default();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_callout() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .init();

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

        graph_worker.register();
        graph.register();

        start_graphs().await;

        let graph_registry = GraphRegistry::get_instance();
        let reg = graph_registry
            .lock()
            .unwrap()
            .get_graph_by_name("callout")
            .unwrap();

        let mut graph = reg.lock().unwrap();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_settings() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .init();

        const GRAPH: &str = r#"
        name: settings
        nodes:
        - id: settings
          action: Settings
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        graph.register();

        start_graphs().await;

        let graph_registry = GraphRegistry::get_instance();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_canodamo_sample_ok() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .set_default();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_canodamo_sample_error() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .init();

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

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_ai_tool() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .set_default();

        const GRAPH_TOOL: &str = r#"
        name: ai_tool
        nodes:
        - id: start
          action: AITool
          config:
            name: Joe
            schema:
              type: object
              properties:
                name:
                  type: string
        edges: []
        "#;

        let graph_tool = build_graph(serde_yaml::from_str(GRAPH_TOOL).unwrap());

        let graph_registry = GraphRegistry::get_instance();
        {
            graph_registry.lock().unwrap().add_graph(graph_tool);
        }

        let reg = {
            graph_registry
                .lock()
                .unwrap()
                .get_graph_by_name("ai_tool")
                .unwrap()
        };

        let mut graph = { reg.lock().unwrap() };

        assert_eq!(graph.name, "ai_tool");

        let response = graph.end_to_end_json(json!({"first": "Joe"})).await;

        info!("Response : {:?}", response);

        let Message::JSON { message, .. } = response else {
            panic!("Expected Error message");
        };

        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "{\"input\":{\"first\":\"Joe\"},\"schema\":{\"properties\":{\"name\":{\"type\":\"string\"}},\"type\":\"object\"}}"
        );

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn test_ai_agent() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        const GRAPH: &str = r#"
        name: ai_agent
        nodes:
        - id: start
          action: AIAgent
        - id: tool_a
          action: Callout
          config:
            graph_name: ai_tool

        - id: system_prompt
          action: Stub
          config:
            prompt: |
                {
                    "content": "You are a shopping assistant. Use these functions:\n1. search_products: When user wants to find products (e.g., 'show me shirts')\n2. get_product_details: When user asks about a specific product ID (e.g., 'tell me about product p1')\n3. clarify_request: When user's request is unclear",
                    "history": { "nice": "not quite"}
                }

        - id: chatgpt_llm
          action: Stub
          config:
            something:
                returned: true

        edges:
        - from: start
          to: tool_a
          role: tool
        - from: start
          to: system_prompt
          role: prompt
        - from: start
          to: chatgpt_llm
          role: llm
        "#;

        const GRAPH_TOOL: &str = r#"
        name: ai_tool
        nodes:
        - id: start
          action: AITool
          config:
            name: Joe
            schema:
              type: object
              properties:
                name:
                  type: string
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph = build_graph(serde_yaml::from_str(GRAPH).unwrap());
        let graph_tool = build_graph(serde_yaml::from_str(GRAPH_TOOL).unwrap());

        graph.register();
        graph_tool.register();

        start_graphs().await;

        let reg = {
            let graph_registry = GraphRegistry::get_instance();
            graph_registry
                .lock()
                .unwrap()
                .get_graph_by_name("ai_agent")
                .unwrap()
        };

        let mut graph = { reg.lock().unwrap() };
        let response = graph.end_to_end_json(json!({"first": "Joe"})).await;

        let Message::JSON { message, .. } = response else {
            panic!("Expected JSON message");
        };

        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "{\"input\":{\"first\":\"Joe\"},\"schema\":{\"properties\":{\"name\":{\"type\":\"string\"}},\"type\":\"object\"}}"
        );

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_reg_example() {
        // tracing_subscriber::fmt()
        //     .with_max_level(Level::DEBUG)
        //     .set_default();

        let j = example().await;

        broadcast_event(ControlEvent {
            graph_name: "ai_agent".to_string(),
            state: GraphStatus::Running,
            operator_names: vec![],
        })
        .await;

        j.await.ok();
    }
}
