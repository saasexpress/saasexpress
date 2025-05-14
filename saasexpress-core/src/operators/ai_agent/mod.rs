use serde::Deserialize;
use serde_json::{Error, Value};
use tracing::info;

use crate::graph::operator_types::ai_agent::AIAgentOperator;

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
pub(super) struct AIAgentV1;

impl From<serde_yaml::Value> for AIAgentV1 {
    fn from(_value: serde_yaml::Value) -> Self {
        AIAgentV1 {}
    }
}
impl AIAgentOperator for AIAgentV1 {
    fn process(&self, _json: Value) -> Result<(), Error> {
        info!("AIAgentV1 process");
        Ok(())
    }
}
