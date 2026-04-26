//! LLM backend trait and common types.
//!
//! Defines the interface that all LLM backends must implement.
//! This trait must compile WITHOUT the "client" feature.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur when interacting with LLM backends.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// ---------------------------------------------------------------------------
// Common types
// ---------------------------------------------------------------------------

/// Health status of a backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Capabilities of a backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCapabilities {
    /// Supports streaming responses.
    pub streaming: bool,

    /// Supports tool/function calling.
    pub tools: bool,

    /// Supports function calling (alternative to tools).
    pub function_calling: bool,
}

/// Chat message in the OpenAI format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role: system, user, assistant, tool.
    pub role: String,

    /// Message content.
    pub content: String,
}

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Generated text content.
    pub content: String,

    /// Model identifier used.
    pub model: String,

    /// Usage statistics.
    pub usage: HashMap<String, u64>,
}

/// Model information from list_models endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier.
    pub id: String,

    /// Object type (typically "model").
    pub object: String,

    /// Owner/creator of the model.
    pub owned_by: String,
}

// ---------------------------------------------------------------------------
// LLM Backend trait
// ---------------------------------------------------------------------------

/// Trait that all LLM backends must implement.
///
/// This trait provides a unified interface for:
/// - Chat completion (non-streaming and streaming)
/// - Model management (list, load, unload)
/// - Health checking
/// - Capability introspection
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// Send a chat completion request (non-streaming).
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<ChatResponse, LlmError>;

    /// Send a chat completion request (streaming).
    ///
    /// Returns a stream of text chunks as they are generated.
    async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<Pin<Box<dyn futures::Stream<Item = Result<String, LlmError>> + Send>>, LlmError>;

    /// List available models.
    async fn list_models(&self) -> Result<Vec<ModelInfo>, LlmError>;

    /// Perform health check.
    async fn health_check(&self) -> Result<HealthStatus, LlmError>;

    /// Load a model into memory.
    async fn load_model(&self, model_id: &str) -> Result<(), LlmError>;

    /// Unload a model from memory.
    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError>;

    /// Get backend capabilities.
    fn capabilities(&self) -> BackendCapabilities;

    /// Get base URL for the backend.
    fn base_url(&self) -> String;
}
