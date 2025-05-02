use std::collections::HashMap;

use crate::operators::noop::NOOP;

use super::super::graph::processors::port::Port;

#[derive(Debug)]
pub struct Ports {
    pub ports: HashMap<String, Port>,
}

impl Ports {
    pub fn new_port(&mut self, id: String) -> NOOP {
        let id = format!("{}-ext", id);
        let port = Port::create();

        self.ports.insert(id, port.0);
        return port.1;
    }
}
