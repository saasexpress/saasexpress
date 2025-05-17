use std::collections::HashMap;

use saasexpress_core::settings::settings::Setting;
use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) struct APICall {
    headers: Vec<Setting>,
}

impl APICall {
    pub(crate) fn new() -> Self {
        APICall {
            headers: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TempParams {
    pub(crate) body: Value,
    pub(crate) content_type: String,
    pub(crate) path: String,
    pub(crate) url: String,
    pub(crate) headers: HashMap<String, String>,
}
