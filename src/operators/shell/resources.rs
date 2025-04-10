use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use tracing::warn;

use super::process::ShellProcess;

pub struct Singleton {
    processes: HashMap<String, ShellProcess>,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            processes: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, session: String, child: ShellProcess) {
        self.processes.insert(session, child);
    }

    pub fn get_process(&mut self, session: String) -> Option<ShellProcess> {
        //warn!("number of processes is {}", self.processes.len());
        let proc = self.processes.remove(&session);
        if proc.is_none() {
            return None;
        } else {
            return proc;
        }
    }
}

static INSTANCE: OnceLock<Mutex<Singleton>> = OnceLock::new();

pub fn get_instance() -> &'static Mutex<Singleton> {
    INSTANCE.get_or_init(|| Mutex::new(Singleton::new()))
}
