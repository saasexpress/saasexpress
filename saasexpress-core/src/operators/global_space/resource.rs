use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

use tracing::{info, warn};

use crate::shared_resource::SharedService;

#[derive(Debug)]
pub(crate) struct WidgetsSharedService {
    data: Vec<String>,
}

impl WidgetsSharedService {
    fn new() -> Self {
        WidgetsSharedService { data: Vec::new() }
    }

    pub(super) fn add_widget(&mut self, widget: String) {
        info!("Adding widget: {}", widget);
        if self.data.contains(&widget) {
            warn!("Widget {} already exists, not adding again", widget);
            return;
        }
        self.data.push(widget);
    }

    fn get_widgets(&self) -> &Vec<String> {
        &self.data
    }

    pub(crate) fn get_instance() -> Option<Arc<Mutex<WidgetsSharedService>>> {
        let mut singleton = INSTANCE.lock().unwrap();
        if singleton.is_none() {
            *singleton = Some(Arc::new(Mutex::new(WidgetsSharedService::new())));
        }
        singleton.clone()
    }

    pub(crate) fn drop_instance() {
        let mut singleton = INSTANCE.lock().unwrap();
        if singleton.is_some() {
            info!("Dropping WidgetsSharedService instance");
            *singleton = None;
        }
    }
}

impl Drop for WidgetsSharedService {
    fn drop(&mut self) {
        warn!(
            "WidgetsSharedService is being dropped, current data: {:?}",
            self.data
        );
    }
}

impl SharedService for WidgetsSharedService {
    fn purpose(&self) -> String {
        format!("Global Space Shared Widgets with {:?}", self.data)
    }
    fn start(&self) {
        info!("Starting with widgets: {:?}", self.get_widgets());
    }

    fn stop(&self) {
        info!("Stopping");
    }

    fn restart(&self) {
        todo!()
    }
}

static INSTANCE: Mutex<Option<Arc<Mutex<WidgetsSharedService>>>> = Mutex::new(None);
