use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use tracing::warn;

/**
 * Shared Service for registering configuration and starting and stopping the service
 *
 */
pub struct SharedService {}

impl SharedService {
    fn new() -> Self {
        SharedService {}
    }

    pub fn start(&self) {
        warn!("SharedService is starting");
        // Here you can add logic to start the service
    }

    pub fn stop(&self) {
        warn!("SharedService is stopping");
        // Here you can add logic to stop the service
    }

    pub fn restart(&self) {
        warn!("SharedService is restarting");
        // Here you can add logic to restart the service
    }

    pub fn register_config(&self, config: HashMap<String, String>) {
        warn!("SharedService is registering configuration: {:?}", config);
        // Here you can add logic to register the configuration
    }
}

static INSTANCE: OnceLock<Mutex<SharedService>> = OnceLock::new();

pub fn get_shared_service() -> &'static Mutex<SharedService> {
    INSTANCE.get_or_init(|| Mutex::new(SharedService::new()))
}
