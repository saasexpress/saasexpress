use saasexpress_core::settings::settings::Setting;

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
