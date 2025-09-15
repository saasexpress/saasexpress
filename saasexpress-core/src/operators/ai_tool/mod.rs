use serde_json::json;
use serde_yaml::Error;

use serde::Deserialize;
use tracing::info;

use crate::graph::{message::Message, operator_types::ai_tool::AIToolOperator};

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
pub(super) struct AIToolV1 {
    schema: Option<serde_yaml::Value>,
}

impl From<serde_yaml::Value> for AIToolV1 {
    fn from(_value: serde_yaml::Value) -> Self {
        let schema = _value.get("schema").cloned();
        AIToolV1 { schema }
    }
}

impl AIToolOperator for AIToolV1 {
    fn name(&self) -> String {
        match &self.schema {
            Some(schema) => schema
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            None => "Unknown".to_string(),
        }
    }

    fn get_schema(&self) -> Result<serde_yaml::Value, Error> {
        match &self.schema {
            Some(schema) => Ok(schema.clone()),
            None => Err(serde::ser::Error::custom("No schema found")),
        }
    }

    fn invoke(&self, _message: Message) -> Message {
        info!("Invoked AIToolV1 {:?}", _message);
        return _message;
        // match _message {
        //     Message::JSON {
        //         message, origin, ..
        //     } => Message::JSON {
        //         message: json!({"input": message, "schema": &self.schema}),
        //         origin,
        //     },
        //     _ => Message::Error {
        //         error: "Invalid message type".to_string(),
        //         origin: None,
        //     },
        // }
    }
}
