//use crate::bootstrap;
use commands::samples::samples;
use commands::stdin::stdin;
use commands::{args::parse_commands, get::get};
use tracing::Level;

mod bootstrap;
mod commands;
mod graph;
mod operators;
mod ports;

#[tokio::main]
async fn main() {
    let matches = parse_commands();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if matches.get_flag("stdin") {
        stdin();
    }
    if matches.get_flag("samples") {
        samples();
    }
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
