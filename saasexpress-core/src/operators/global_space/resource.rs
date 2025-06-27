use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use tracing::{info, warn};

pub(crate) struct SharedWidgets {
    data: Vec<String>,
}

trait SharedService<T> {
    fn start(&self);
    fn stop(&self);
    fn restart(&self);
}

impl SharedWidgets {
    fn new() -> Self {
        SharedWidgets { data: Vec::new() }
    }

    pub(super) fn add_widget(&mut self, widget: String) {
        self.data.push(widget);
    }

    fn get_widgets(&self) -> &Vec<String> {
        &self.data
    }

    pub(crate) fn start(&self) {
        info!("Starting with widgets: {:?}", self.get_widgets());
    }
}

// impl SharedService<SharedWidgets> for SharedWidgets {
//     fn start(&self) {
//         info!("Starting with widgets: {:?}", self.get_widgets());
//     }

//     fn stop(&self) {
//         todo!()
//     }

//     fn restart(&self) {
//         todo!()
//     }
// }

static INSTANCE: OnceLock<Mutex<SharedWidgets>> = OnceLock::new();

pub(crate) fn get_shared_service() -> &'static Mutex<SharedWidgets> {
    INSTANCE.get_or_init(|| Mutex::new(SharedWidgets::new()))
}
