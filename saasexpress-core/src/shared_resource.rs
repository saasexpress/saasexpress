use std::sync::{Arc, Mutex};

/**
 * Shared Service for registering configuration and starting and stopping the service
 *
 */
pub trait SharedService {
    fn purpose(&self) -> String;
    fn start(&self);
    fn stop(&self);
    fn restart(&self);
}

pub type SharedServiceRef = Arc<Mutex<dyn SharedService + Send>>;
