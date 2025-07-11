use crate::graph::graph::GraphMod;
use crate::graph::graph_run::GraphRun;
use graph::{
    graph::{Graph, GraphStatus},
    registry::GraphRegistry,
};
use my_reg::{ControlEvent, ControlEventType, broadcast_event, deregister, register};
use operators::factory::add_node_to_graph;
use serde_yaml::Value;
use tokio::sync::{
    broadcast::{Receiver, Sender},
    mpsc,
};
use tokio::time::timeout;
use tracing::{debug, error, info};

pub mod graph;
pub mod my_reg;
pub mod operators;
mod ports;
pub mod random;
pub mod settings;
mod shared_resource;
pub mod timestamp;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_graph(yaml: Value) -> String {
    let graph_name = yaml["name"].as_str().unwrap().to_string();

    let mut graph = Graph::new(graph_name.clone());

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

    graph.no_processor();
    graph.register();

    let graph = GraphRegistry::get_graph(graph_name.as_str());
    if graph.is_none() {
        error!("Graph not found: {}", graph_name);
        panic!("Graph not found: {}", graph_name);
    }
    let graph = graph.unwrap();

    let mut graph = graph.lock().unwrap();

    //    graph.runner = graph.replace_all_runtimes();
    //graph.refresh_runtime_nodes();

    //graph.watch();

    graph.make_active_if_ready();

    //graph.init(graph.runner.nodes.clone());

    graph.replace_runtime();

    graph_name
}

pub fn get_pending() -> Vec<String> {
    let graph_registry = GraphRegistry::get_instance();

    let pending_graphs = {
        let graph_names = {
            let graph_registry = graph_registry.lock().unwrap();
            graph_registry.graph_names()
        };
        info!("GetPending graph_list={:?}", graph_names);
        graph_names
            .iter()
            .filter(|name| {
                let g = GraphRegistry::get_graph(name);
                if g.is_none() {
                    error!("Graph not found: {}", name);
                    return true;
                }
                let g = g.unwrap();
                let graph = g.lock();
                if graph.is_err() {
                    error!("Graph is locked, skipping: {:?}", g);
                    return true;
                }
                let graph = graph.unwrap();

                graph.runner.state == GraphStatus::Inactive
            })
            .map(|name| name.to_string())
            .collect::<Vec<String>>()
    };
    info!("Pending graphs: {:?}", pending_graphs);
    pending_graphs
}

pub async fn start_graphs() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ControlEvent>(100);

    register("startup", tx);

    let my_duration = tokio::time::Duration::from_millis(2000);

    loop {
        let mut pending = get_pending();

        if pending.len() == 0 {
            info!("No pending graphs to start.");
            break;
        }
        let msg = timeout(my_duration, rx.recv()).await;
        match msg {
            Ok(msg) => match msg {
                Some(msg) => {
                    if msg.event_type == ControlEventType::GraphReplaced
                        && msg.graph_status == GraphStatus::Active
                    {
                        let matched = pending.iter().position(|x| x == &msg.graph_name);
                        if matched.is_some() {
                            pending.remove(matched.unwrap());
                        }
                    }
                    info!(
                        "Received Event: {:?} (Remaining={})",
                        serde_json::to_string(&msg),
                        pending.len(),
                    );
                    if pending.len() == 0 {
                        break;
                    }
                }
                None => {
                    panic!("No message received");
                }
            },
            Err(_) => {
                let pending_graphs = get_pending();

                // if pending_graphs.is_empty() {
                //     info!("All graphs are active.");
                //     break;
                // }
                panic!("Timeout waiting for message: {}", pending_graphs.join(", "));
            }
        }
    }
    deregister::<ControlEvent>("startup");
    info!("All systems a go!");

    post_graph_hook();
    info!("Post graph hook executed.");
}

pub fn post_graph_hook() {
    let graph_registry = GraphRegistry::get_instance();

    let graph_registry = graph_registry.lock().unwrap();
    for graph in graph_registry.get_graphs() {
        let graph = graph.lock().unwrap();

        info!("Post start hook for graph: {}", graph.name);
        graph.post_start_hook();
    }
}

#[cfg(test)]
mod saasexpress_core_tests {
    use std::panic;
    use std::thread::sleep;

    use serde_json::json;
    use tracing::{Level, debug, info, instrument};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::graph::graph::IntoGraphRunner;
    use crate::graph::registry::GraphRegistry;
    use crate::my_reg::broadcast_event;
    use crate::operators::global_space::resource::WidgetsSharedService;
    //use crate::operators::global_space::resource::SharedWidgets;
    //use crate::operators::global_space::resource::get_shared_service;
    //use crate::shared_resource::SharedService;
    use crate::{graph::message::Message, settings::settings::env_settings};

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            let console_layer = console_subscriber::spawn();

            tracing_subscriber::registry()
            .with(console_layer)
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| {
                    "saasexpress_tenants=warn,saasexpress_core=debug,saasexpress=debug,tower_http=info,tokio=trace,runtime=trace".into()
                }),
            ))
            .with(tracing_subscriber::fmt::layer().with_thread_ids(true))
            .init();

        });

        GraphRegistry::get_instance().lock().unwrap().clear();
    }

    fn setup() {
        initialize();
    }
    fn teardown() {
        let graph_registry = GraphRegistry::get_instance();
        let mut graph_registry = graph_registry.lock().unwrap();
        graph_registry.clear();
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        teardown();
        assert!(result.is_ok())
    }

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
        initialize();

        const GRAPH: &str = r#"
        name: buffer_to_json
        nodes:
          - id: start
            action: BufferToJSON
        "#;
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let runner = graph_name.into_graph_runner();

        assert_eq!(runner.name, "buffer_to_json");

        let response = runner.end_to_end("{}".as_bytes().to_vec()).await;

        debug!("Message: {:?}", response);
        let Message::JSON { message, .. } = response else {
            panic!("Expected JSON message");
        };

        assert_eq!(message.get("_ts").is_some(), true);

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn claimcheck_works() {
        initialize();

        const GRAPH: &str = r#"
        name: claim_check
        nodes:
          - id: start
            action: ClaimCheck
        "#;
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

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
        initialize();

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
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

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
        initialize();

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
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

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
        initialize();

        const GRAPH: &str = r#"
        name: g_callout
        nodes:
        - id: n_callout
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
        let _ = build_graph(serde_yaml::from_str(GRAPH_WORKER).unwrap());
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

        // let graph_registry = GraphRegistry::get_instance();
        // let reg = graph_registry
        //     .lock()
        //     .unwrap()
        //     .get_graph_by_name("g_callout")
        //     .unwrap();

        // let mut graph = reg.lock().unwrap();

        assert_eq!(graph.name, "g_callout");

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
            panic!("Expected JSON message: {:?}", message_2);
        };

        let nm = message.get("name").unwrap();
        assert_eq!(nm, "Joe");

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_shared_resources() {
        initialize();

        const GRAPH_1: &str = r#"
        name: graph_1
        nodes:
        - id: global
          action: GlobalSpace
        edges: []
        "#;

        const GRAPH_WORKER: &str = r#"
        name: worker
        nodes:
        - id: global
          action: GlobalSpace
        - id: start
          action: Stub
          config:
            name: Joe
        edges:
        - from: global
          to: start
        "#;

        let _ = build_graph(serde_yaml::from_str(GRAPH_1).unwrap());
        let graph_name = build_graph(serde_yaml::from_str(GRAPH_WORKER).unwrap());

        start_graphs().await;

        async fn eval(graph_name: String) {
            let graph = graph_name.clone().into_graph_runner();

            assert_eq!(graph.name, "worker");

            let response = graph.end_to_end_standard("hello".as_bytes().to_vec()).await;

            info!("Response : {:?}", response);

            let Message::JSON { message, .. } = response else {
                panic!("Expected Message to be JSON: {:?}", response);
            };

            let nm = message.get("name").unwrap();
            assert_eq!(nm, "Joe");
        }

        {
            let graph_registry = GraphRegistry::get_instance();
            let graph_registry = graph_registry.lock().unwrap();
            let graph = graph_registry.get_graph_by_name(&graph_name).unwrap();

            let graph = graph.lock().unwrap();
            let ls = graph.shared_resources();
            ls.iter().for_each(|share| {
                let share = share.lock().unwrap();
                info!("Shared Resource: {:?}", share.purpose());

                share.start();
                share.stop();
            });
        }

        WidgetsSharedService::drop_instance();

        //get_shared_service().lock().unwrap().start();

        eval(graph_name.clone()).await;
    }

    #[tokio::test]
    async fn test_graph_upgrade() {
        initialize();

        const GRAPH: &str = r#"
        name: g_callout
        nodes:
        - id: n_callout
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
        - id: global
          action: GlobalSpace
        edges: []
        "#;

        info!("Graph: {}", GRAPH);
        let _ = build_graph(serde_yaml::from_str(GRAPH_WORKER).unwrap());
        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        async fn eval(graph_name: String) {
            let graph = graph_name.clone().into_graph_runner();

            assert_eq!(graph.name, "g_callout");

            let response = graph.end_to_end_standard("hello".as_bytes().to_vec()).await;

            info!("Response : {:?}", response);

            let Message::Tuple {
                message_1,
                message_2,
                ..
            } = response
            else {
                panic!("Expected Tuple message");
            };

            let Message::Standard { message, .. } = message_1.as_ref() else {
                panic!("Expected First Message to be Standard");
            };

            assert_eq!(message.to_vec(), "hello".as_bytes().to_vec());

            let Message::JSON { message, .. } = message_2.as_ref() else {
                panic!("Expected Second Message to be JSON: {:?}", message_2);
            };

            let nm = message.get("name").unwrap();
            assert_eq!(nm, "Joe");
            debug!("Test passed for graph: {}", graph_name);
        }

        eval(graph_name.clone()).await;

        Graph::deregister(graph_name);

        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        // need to wait for the graph to be made active
        start_graphs().await;

        // let shared_service = get_shared_service();
        // shared_service.lock().unwrap().start();
        // shared_service.lock().unwrap().restart();

        eval(graph_name.clone()).await;

        teardown();
    }

    #[tokio::test]
    async fn test_settings() {
        initialize();

        const GRAPH: &str = r#"
        name: settings
        nodes:
        - id: settings
          action: Settings
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

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
        initialize();

        const GRAPH: &str = r#"
        name: canonical_model
        nodes:
        - id: start
          action: CanonicalModelSample
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

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
        initialize();

        const GRAPH: &str = r#"
        name: canonical_model_err
        nodes:
        - id: start
          action: CanonicalModelSample
        edges: []
        "#;

        info!("Graph: {}", GRAPH);

        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

        assert_eq!(graph.name, "canonical_model_err");

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
        initialize();

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

        let graph_name = build_graph(serde_yaml::from_str(GRAPH_TOOL).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

        assert_eq!(graph.name, "ai_tool");

        let response = graph.end_to_end_json(json!({"first": "Joe"})).await;

        info!("Response : {:?}", response);

        let Message::JSON { message, .. } = response else {
            panic!("Expected Error message");
        };

        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "{\"first\":\"Joe\"}"
        );

        //GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[instrument]
    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn test_ai_agent() {
        initialize();

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
            content: |
                You are a shopping assistant. Use these functions:
                
                1. search_products: When user wants to find products (e.g., 'show me shirts')
                2. get_product_details: When user asks about a specific product ID (e.g., 'tell me about product p1')
                3. clarify_request: When user's request is unclear
                

        - id: chatgpt_llm
          action: Stub
          config:
            choices:
            - index: 0
              message:
                role: assistant
                annotations: []
                content: |
                  {"something": {"returned": true}}
              finish_reason: stop
            created: 0
            id: chatgpt-123
            model: gpt-3.5-turbo
            object: chat.completion
            service_tier: free
            system_fingerprint: "fingerprint-123"
            usage:
              completion_tokens: 0
              completion_tokens_details:
                accepted_prediction_tokens: 0
                audio_tokens: 0
                reasoning_tokens: 0
                rejected_prediction_tokens: 0
              prompt_tokens: 0
              prompt_tokens_details:
                audio_tokens: 0
                cached_tokens: 0
              total_tokens: 0

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

        let graph_name = build_graph(serde_yaml::from_str(GRAPH).unwrap());

        let _ = build_graph(serde_yaml::from_str(GRAPH_TOOL).unwrap());

        start_graphs().await;

        let graph = graph_name.into_graph_runner();

        let response = graph
            .end_to_end_json(json!({"prompt": "Do something"}))
            .await;

        let Message::JSON { message, .. } = response else {
            panic!("Expected JSON message - {:?}", response);
        };

        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "{\"response\":\"{\\\"something\\\": {\\\"returned\\\": true}}\\n\"}"
        );
        info!("Great, response as expected!");
        // GraphRegistry::get_instance().lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_reg_example() {
        initialize();

        let (tx, mut rx) = mpsc::channel::<ControlEvent>(100);

        register("my_channel", tx);

        broadcast_event(ControlEvent {
            graph_id: "ai_agent".to_string(),
            graph_name: "ai_agent".to_string(),
            graph_status: GraphStatus::Active,
            event_type: ControlEventType::Notice,
            reason: "Test event".to_string(),
            operator_names: vec![],
        })
        .await;

        let returned = rx.recv().await;
        assert!(returned.is_some(), "Expected a message from the channel");
        let msg = returned.unwrap();
        assert_eq!(msg.graph_name, "ai_agent");
        assert_eq!(msg.event_type, ControlEventType::Notice);
        assert_eq!(msg.reason, "Test event");
    }
}
