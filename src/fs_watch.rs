use std::path::{Path, PathBuf};
use std::time::Duration;

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, WatcherKind};
use notify::{PollWatcher, Watcher};
use saasexpress_core::graph::registry::GraphRegistry;
use saasexpress_core::start_graphs;
use serde_yaml::Value;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::bootstrap::{build_graph, reload_graph};

pub fn watch_fs(path: String) -> Result<(), notify::Error> {
    let base_path = std::fs::canonicalize(&path).unwrap_or_else(|_| PathBuf::from(path));
    let get_relative = |full_path: &Path| -> String {
        full_path
            .strip_prefix(&base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| full_path.to_string_lossy().to_string())
    };

    let (tx, rx) = std::sync::mpsc::channel();
    // This example is a little bit misleading as you can just create one Config and use it for all watchers.
    // That way the pollwatcher specific stuff is still configured, if it should be used.
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        // custom config for PollWatcher kind
        // you
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        // use default config for everything else
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    // watch some stuff
    watcher
        .watch(base_path.as_path(), RecursiveMode::Recursive)
        .unwrap();

    // just print all events, this blocks forever
    for e in rx {
        println!("{:?}", e);
        match e {
            Ok(Event {
                kind: EventKind::Modify(kind),
                paths,
                ..
            }) => match kind {
                notify::event::ModifyKind::Data(_) => {
                    for path in paths {
                        info!("File modified: {:?}", get_relative(path.as_path()));

                        reload_graph(path.as_os_str().to_string_lossy().to_string());
                    }
                }
                _ => {}
            },
            _ => {
                info!("Event: {:?}", e);
            }
        }
    }

    Ok(())
}
