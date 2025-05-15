use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AiAgentModel {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub service_tier: String,
    pub system_fingerprint: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub logprobs: Option<serde_json::Value>, // Assuming logprobs can be any JSON value
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub annotations: Vec<serde_json::Value>, // Assuming annotations can be any JSON value
    pub content: Option<String>,
    pub refusal: Option<String>,
    pub role: String,
    #[serde(default = "Vec::new")]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub function: Function,
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String, // Renamed to avoid conflict with Rust's `type` keyword
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub arguments: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub completion_tokens_details: CompletionTokensDetails,
    pub prompt_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}
