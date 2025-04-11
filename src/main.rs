use std::path::PathBuf;

use commands::config::config;
use commands::samples::samples;
use commands::stdin::stdin;
use commands::{args::parse_commands, get::get};
use saasexpress_tenants::TenantsService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod bootstrap;
mod commands;
mod operators;

//#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
#[tokio::main]
async fn main() {
    let matches = parse_commands();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "saasexpress_tenants=debug,saasexpress_core=info,saasexpress=info,tower_http=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    bootstrap::bootstrap(Vec::new());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
