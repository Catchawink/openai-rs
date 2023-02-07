use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub suffix: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub lob_probs: Option<i32>,
    pub echo: Option<bool>,
    pub stop: Option<bool>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub best_of: Option<i32>,
    pub logit_bias: Option<String>,
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompletionResponse {
    pub model: String,
    pub prompt: String,
    pub suffix: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub lob_probs: Option<i32>,
    pub echo: Option<bool>,
    pub stop: Option<bool>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub best_of: Option<i32>,
    pub logit_bias: Option<String>,
    pub user: Option<String>,
}
