//! OpenAI-compatible API types for the Samaryn Gateway.
//!
//! These types mirror the OpenAI Chat Completions API, allowing the gateway
//! to act as a transparent proxy while intercepting and scanning content.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A chat completion request, compatible with the OpenAI API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// The model to use for the completion (e.g., "gpt-4o", "claude-3-sonnet").
    pub model: String,

    /// The list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// Sampling temperature between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,

    /// Whether to stream back partial progress via SSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming responses (e.g., include_usage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// Catch-all for any additional fields the client sends.
    /// This ensures we forward everything the upstream provider might need.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A single message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message author: "system", "user", "assistant", or "tool".
    pub role: String,

    /// The content of the message. Can be a string or an array of content parts
    /// (for multi-modal messages with text and images).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,

    /// The name of the author (optional, used for multi-participant conversations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tool calls made by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<serde_json::Value>,

    /// The ID of the tool call this message is responding to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    /// Catch-all for additional fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Options for streaming responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptions {
    /// Whether to include usage information in the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// A non-streaming chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// Catch-all for additional fields from the provider.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A single choice in the completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u64,

    /// The generated message (used in non-streaming responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    /// The delta content (used in streaming responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<Delta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// A delta update in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<serde_json::Value>,

    /// Catch-all for additional fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,

    /// Catch-all for additional usage fields (e.g., cached tokens).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
