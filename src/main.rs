use std::borrow::Cow;
use std::path::PathBuf;

use bootstrap::build_graph;
use commands::config::config;
use commands::samples::samples;
use commands::stdin::stdin;
use commands::{args::parse_commands, get::get};
use fastrace::prelude::*;
use futures::channel::oneshot;
use otlp::{init_logs, init_tracer};
use saasexpress_tenants::TenantsService;
use tracing::info;
mod bootstrap;
mod commands;
mod operators;

mod otlp;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
//#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();

    let matches = parse_commands();

    init_logs();
    init_tracer();

    {
        // Start tracing
        let root = Span::root("root", SpanContext::random());

        root.add_event(Event::new("event in root"));

        async move {
            let _span1 = LocalSpan::enter_with_local_parent("a child span");

            LocalSpan::add_event(Event::new("event in span1"));
        }
        .in_span(root)
        .await;
    }

    if matches.get_flag("stdin") {
        stdin();
    }
    if matches.get_flag("samples") {
        samples();
    }

    tokio::spawn(TenantsService::start());

    // get config file

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        config(config_path.to_str().unwrap().to_string());
    }
    // let config = matches.get_one::<String>("config").unwrap();
    // println!("Config file: {:?}", config);
    // if matches.get_flag("config") {
    //     let config = matches.get_one::<String>("config").unwrap();
    //     println!("Config file: {:?}", config);
    // }
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            get(sub_matches);
        }
        _ => {}
    }

    {
        let start = Span::root("start_up", SpanContext::random());

        let _guard = start.set_local_parent();

        TenantsService::saasexpress_graphs()
            .iter()
            .for_each(|yaml| build_graph(yaml.to_owned()));

        bootstrap::bootstrap();
    }

    do_it();

    loop {
        const ONE_HOUR: u64 = 3600;
        std::thread::sleep(std::time::Duration::from_secs(ONE_HOUR));
    }

    // Shutdown trace pipeline
    // tracer_provider
    //     .shutdown()
    //     .expect("TracerProvider should shutdown successfully");
    // logger_provider
    //     .shutdown()
    //     .expect("LoggerProvider should shutdown successfully");
}

#[derive(Debug)]
pub struct Origin<T> {
    context: T,
}

impl<T> Origin<T>
where
    T: Send + Sync,
{
    pub fn new(context: T) -> Self {
        Origin { context }
    }
}

#[derive(Debug)]
pub enum XMessage {
    Split {
        message: Vec<u8>,
        origin: Origin<String>,
    },
    Standard {
        message: Vec<u8>,
        origin: Origin<oneshot::Sender<XMessage>>,
    },
}

fn do_it() {
    // Example usage
    let chnl = oneshot::channel();
    let list_messages = vec![
        XMessage::Split {
            message: vec![1, 2, 3],
            origin: Origin::new("example_context".to_string()),
        },
        XMessage::Standard {
            message: vec![4, 5, 6],
            origin: Origin::new(chnl.0),
        },
    ];
    for message in list_messages {
        match message {
            XMessage::Split { message, origin } => {
                println!("Split Message: {:?}", message);
                println!("Origin: {:?}", origin.context);
            }
            XMessage::Standard { message, origin } => {
                println!("Standard Message: {:?}", message);
                println!("Origin: {:?}", origin.context);
            }
        }
    }
}
