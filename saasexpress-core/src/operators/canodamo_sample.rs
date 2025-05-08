use serde::Deserialize;
use serde_json::{Error, Value};

use crate::graph::operator_types::canonical_model::CanonicalModelOperator;

#[derive(Deserialize, Debug)]
struct ThisModel {
    name: String,
}

#[derive(Debug)]
pub(super) struct CanonicalModelSample;

impl From<serde_yaml::Value> for CanonicalModelSample {
    fn from(_value: serde_yaml::Value) -> Self {
        CanonicalModelSample {}
    }
}
impl CanonicalModelOperator for CanonicalModelSample {
    fn validate_json(&self, json: Value) -> Result<(), Error> {
        serde_json::from_value::<ThisModel>(json)
            .map(|_o| ())
            .map_err(|e| e)
    }
}
