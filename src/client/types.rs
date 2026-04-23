//! API types for llama-server OpenAI-compatible endpoints.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Chat Completion
// ---------------------------------------------------------------------------

/// Chat completion request (OpenAI-compatible).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

/// Chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }
}

/// Chat completion response (non-streaming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

/// Completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Token usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

// ---------------------------------------------------------------------------
// Streaming
// ---------------------------------------------------------------------------

/// Chat completion chunk (streaming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

/// Chunk choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

/// Delta (incremental content).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

// ---------------------------------------------------------------------------
// Server info
// ---------------------------------------------------------------------------

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(default)]
    pub slots_idle: usize,
    #[serde(default)]
    pub slots_processing: usize,
}

/// Server properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerProps {
    #[serde(default)]
    pub default_generation_settings: serde_json::Value,
}

/// Server slot status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotStatus {
    pub id: usize,
    pub state: String,
    pub n_processed: usize,
    #[serde(default)]
    pub n_tokens: usize,
}

// ---------------------------------------------------------------------------
// Model management (router mode API)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLoadRequest {
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLoadResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUnloadRequest {
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub status: ModelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelStatus {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    #[serde(default)]
    pub data: Vec<ModelInfo>,
}
