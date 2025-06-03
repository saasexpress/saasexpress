use saasexpress_core::graph::operator_types::canonical_model::CanonicalModelService;
use serde::Deserialize;
use serde_json::{Error, Value};
use tracing::debug;

fn default_empty() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug)]
//#[serde(deny_unknown_fields)]
struct ThisModel {
    #[serde(default = "default_empty")]
    name: String,
}

#[derive(Debug)]
pub(super) struct CanonicalModelSample;

impl From<serde_yaml::Value> for CanonicalModelSample {
    fn from(_value: serde_yaml::Value) -> Self {
        CanonicalModelSample {}
    }
}
impl CanonicalModelService for CanonicalModelSample {
    fn validate_json(&self, json: Value) -> Result<(), Error> {
        serde_json::from_value::<ThisModel>(json)
            .map(|o| {
                debug!("CanonicalModelSample: {:?}", o);
                ()
            })
            .map_err(|e| e)
    }
}
