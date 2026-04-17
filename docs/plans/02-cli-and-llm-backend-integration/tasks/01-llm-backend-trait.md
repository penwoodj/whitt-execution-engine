# Task 01: LLM Backend Trait

**Files:**
- Create: `src/backends/mod.rs`
- Create: `src/backends/trait.rs`
- Create: `src/backends/types.rs`
- Create: `src/backends/errors.rs`
- Create: `src/backends/streaming/mod.rs`
- Create: `src/backends/streaming/sse.rs`
- Create: `src/backends/streaming/ndjson.rs`
- Modify: `src/lib.rs` (add backends module)
- Test: `tests/backends/trait_test.rs`
- Test: `tests/backends/streaming_test.rs`

---

## Overview

Define the unified `LlmBackend` trait and all supporting types for interacting with LLM providers. This trait abstracts away provider differences (LM Studio, Ollama, llama.cpp, OpenAI) and provides a consistent interface for chat completion, streaming, model management, and health checks.

---

## Implementation Steps

### Step 1: Define core types

- [ ] **Step 1.1: Write shared types**

```rust
// src/backends/types.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for chat completion
#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Option<Vec<Tool>>,
    pub response_format: Option<ResponseFormat>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,  // Ollama-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,  // Ollama-specific
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "role")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    System { content: String },
    User { content: String },
    Assistant { content: String, tool_calls: Option<Vec<ToolCall>> },
    Tool { tool_call_id: String, content: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub r#type: String,  // "function"
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    Json,
    #[serde(rename = "json_object")]
    JsonObject,
}

/// Response from chat completion
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: ResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Streaming chunk
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub index: usize,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<StreamToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamToolCall {
    pub index: usize,
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub function: Option<StreamToolCallFunction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamToolCallFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

/// Model information
#[derive(Debug, Clone, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

/// Health status
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthStatus {
    pub status: String,  // "ok", "degraded", "unavailable"
    pub message: Option<String>,
    pub timestamp: u64,
}

/// Backend capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendCapabilities {
    pub streaming: bool,
    pub tools: bool,
    pub tool_choice: bool,
    pub response_format: bool,
    pub system_messages: bool,
    pub temperature: bool,
    pub max_tokens: bool,
    pub parallel_requests: bool,
}

impl Default for BackendCapabilities {
    fn default() -> Self {
        Self {
            streaming: true,
            tools: true,
            tool_choice: true,
            response_format: true,
            system_messages: true,
            temperature: true,
            max_tokens: true,
            parallel_requests: false,
        }
    }
}
```

- [ ] **Step 1.2: Write backend-specific errors**

```rust
// src/backends/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Backend error: {0}")]
    Backend(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Timeout after {0}s")]
    Timeout(u64),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Health check failed: {0}")]
    HealthCheck(String),
}

pub type Result<T> = std::result::Result<T, LlmError>;
```

- [ ] **Step 1.3: Test types**

```rust
// tests/backends/types_test.rs
use glyphnova::backends::types::*;

#[test]
fn test_chat_request_serialization() {
    let request = ChatRequest {
        model: "test-model".to_string(),
        messages: vec![
            Message::System {
                content: "You are a helpful assistant.".to_string(),
            },
            Message::User {
                content: "Hello!".to_string(),
            },
        ],
        tools: None,
        response_format: None,
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: true,
        think: None,
        format: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("test-model"));
    assert!(json.contains("system"));
    assert!(json.contains("user"));
}

#[test]
fn test_chat_response_deserialization() {
    let json = r#"{
        "id": "test-id",
        "object": "chat.completion",
        "created": 1234567890,
        "model": "test-model",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello!",
                "tool_calls": null
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 5,
            "total_tokens": 15
        }
    }"#;

    let response: ChatResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "test-id");
    assert_eq!(response.choices[0].message.content, Some("Hello!".to_string()));
    assert_eq!(response.usage.total_tokens, 15);
}

#[test]
fn test_backend_capabilities_default() {
    let caps = BackendCapabilities::default();
    assert!(caps.streaming);
    assert!(caps.tools);
    assert!(!caps.parallel_requests);
}
```

Run: `cargo test test_types --lib`
Expected: PASS

- [ ] **Step 1.4: Commit**

```bash
git add src/backends/types.rs src/backends/errors.rs tests/backends/types_test.rs
git commit -m "feat(backends): add core types and errors"
```

---

### Step 2: Define LLM Backend trait

- [ ] **Step 2.1: Write backend trait**

```rust
// src/backends/trait.rs
use super::types::*;
use super::errors::Result;
use async_trait::async_trait;
use std::pin::Pin;
use futures::Stream;

/// Unified LLM backend trait
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// Non-streaming chat completion
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Streaming chat completion
    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>>;

    /// List available models
    async fn list_models(&self) -> Result<Vec<ModelInfo>>;

    /// Health check
    async fn health_check(&self) -> Result<HealthStatus>;

    /// Load model into memory (for backends that support it)
    async fn load_model(&self, model_id: &str) -> Result<()>;

    /// Unload model from memory
    async fn unload_model(&self, model_id: &str) -> Result<()>;

    /// Get backend capabilities
    fn capabilities(&self) -> BackendCapabilities;

    /// Get backend name
    fn name(&self) -> &'static str;
}
```

- [ ] **Step 2.2: Write mock backend for testing**

```rust
// src/backends/trait.rs (add at end)
use futures::{stream, StreamExt};

pub struct MockBackend {
    name: &'static str,
    capabilities: BackendCapabilities,
}

impl MockBackend {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            capabilities: BackendCapabilities::default(),
        }
    }

    pub fn with_capabilities(name: &'static str, capabilities: BackendCapabilities) -> Self {
        Self { name, capabilities }
    }
}

#[async_trait]
impl LlmBackend for MockBackend {
    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        Ok(ChatResponse {
            id: "mock-id".to_string(),
            object: "chat.completion".to_string(),
            created: 1234567890,
            model: "mock-model".to_string(),
            choices: vec![Choice {
                index: 0,
                message: ResponseMessage {
                    role: "assistant".to_string(),
                    content: Some("Mock response".to_string()),
                    tool_calls: None,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
            },
        })
    }

    async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>> {
        let chunk = StreamChunk {
            id: "mock-id".to_string(),
            object: "chat.completion.chunk".to_string(),
            created: 1234567890,
            model: "mock-model".to_string(),
            choices: vec![StreamChoice {
                index: 0,
                delta: StreamDelta {
                    role: Some("assistant".to_string()),
                    content: Some("Mock".to_string()),
                    tool_calls: None,
                },
                finish_reason: None,
            }],
        };

        Ok(Box::pin(stream::once(async move { Ok(chunk) })))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: "mock-model".to_string(),
            object: "model".to_string(),
            created: 1234567890,
            owned_by: "mock".to_string(),
        }])
    }

    async fn health_check(&self) -> Result<HealthStatus> {
        Ok(HealthStatus {
            status: "ok".to_string(),
            message: None,
            timestamp: 1234567890,
        })
    }

    async fn load_model(&self, _model_id: &str) -> Result<()> {
        Ok(())
    }

    async fn unload_model(&self, _model_id: &str) -> Result<()> {
        Ok(())
    }

    fn capabilities(&self) -> BackendCapabilities {
        self.capabilities.clone()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
```

- [ ] **Step 2.3: Test trait with mock**

```rust
// tests/backends/trait_test.rs
use glyphnova::backends::{LlmBackend, trait::MockBackend, types::*};
use tokio_test::block_on;

#[test]
fn test_mock_backend_chat() {
    let backend = MockBackend::new("mock");
    let request = ChatRequest {
        model: "test-model".to_string(),
        messages: vec![Message::User {
            content: "Hello!".to_string(),
        }],
        tools: None,
        response_format: None,
        temperature: None,
        max_tokens: None,
        stream: false,
        think: None,
        format: None,
    };

    let response = block_on(async {
        backend.chat(request).await
    }).unwrap();

    assert_eq!(response.choices[0].message.content, Some("Mock response".to_string()));
}

#[test]
fn test_mock_backend_chat_stream() {
    let backend = MockBackend::new("mock");
    let request = ChatRequest {
        model: "test-model".to_string(),
        messages: vec![Message::User {
            content: "Hello!".to_string(),
        }],
        tools: None,
        response_format: None,
        temperature: None,
        max_tokens: None,
        stream: true,
        think: None,
        format: None,
    };

    let stream = block_on(async {
        backend.chat_stream(request).await
    }).unwrap();

    let chunks: Vec<_> = block_on(async {
        stream.collect::<Vec<_>>().await
    });

    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].is_ok());
    assert_eq!(
        chunks[0].as_ref().unwrap().choices[0].delta.content,
        Some("Mock".to_string())
    );
}

#[test]
fn test_mock_backend_list_models() {
    let backend = MockBackend::new("mock");
    let models = block_on(async {
        backend.list_models().await
    }).unwrap();

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "mock-model");
}

#[test]
fn test_mock_backend_health_check() {
    let backend = MockBackend::new("mock");
    let health = block_on(async {
        backend.health_check().await
    }).unwrap();

    assert_eq!(health.status, "ok");
}

#[test]
fn test_mock_backend_capabilities() {
    let backend = MockBackend::new("mock");
    let caps = backend.capabilities();

    assert!(caps.streaming);
    assert!(caps.tools);
}

#[test]
fn test_mock_backend_name() {
    let backend = MockBackend::new("my-backend");
    assert_eq!(backend.name(), "my-backend");
}
```

Run: `cargo test test_trait --lib`
Expected: PASS

- [ ] **Step 2.4: Commit**

```bash
git add src/backends/trait.rs tests/backends/trait_test.rs
git commit -m "feat(backends): add LlmBackend trait and mock implementation"
```

---

### Step 3: Implement streaming parsers

- [ ] **Step 3.1: Write SSE parser**

```rust
// src/backends/streaming/sse.rs
use super::super::errors::{LlmError, Result};
use super::super::types::StreamChunk;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;

/// Parse Server-Sent Events (SSE) stream
pub fn parse_sse_stream<S>(
    stream: S,
) -> Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>
where
    S: Stream<Item = reqwest::Result<Bytes>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        let mut buffer = String::new();
        let mut stream = Box::pin(stream);

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    yield Err(LlmError::Network(e));
                    return;
                }
            };

            let data = String::from_utf8_lossy(&chunk);
            buffer.push_str(&data);

            // Process complete events
            while let Some(pos) = buffer.find("\n\n") {
                let event = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                // Parse SSE event
                if let Some(data_pos) = event.find("data: ") {
                    let json_data = &event[data_pos + 6..];

                    // Skip [DONE] marker
                    if json_data.trim() == "[DONE]" {
                        return;
                    }

                    // Parse JSON
                    match serde_json::from_str::<StreamChunk>(json_data) {
                        Ok(chunk) => yield Ok(chunk),
                        Err(e) => yield Err(LlmError::Parse(format!("SSE parse error: {}", e))),
                    }
                }
            }
        }
    })
}
```

- [ ] **Step 3.2: Write NDJSON parser**

```rust
// src/backends/streaming/ndjson.rs
use super::super::errors::{LlmError, Result};
use super::super::types::StreamChunk;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;

/// Parse NDJSON (Newline Delimited JSON) stream
pub fn parse_ndjson_stream<S>(
    stream: S,
) -> Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>
where
    S: Stream<Item = reqwest::Result<Bytes>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        let mut buffer = String::new();
        let mut stream = Box::pin(stream);

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    yield Err(LlmError::Network(e));
                    return;
                }
            };

            let data = String::from_utf8_lossy(&chunk);
            buffer.push_str(&data);

            // Process complete lines
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].to_string();
                buffer = buffer[pos + 1..].to_string();

                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Parse JSON
                match serde_json::from_str::<StreamChunk>(line) {
                    Ok(chunk) => yield Ok(chunk),
                    Err(e) => yield Err(LlmError::Parse(format!("NDJSON parse error: {}", e))),
                }
            }
        }
    })
}
```

- [ ] **Step 3.3: Write streaming module**

```rust
// src/backends/streaming/mod.rs
pub mod sse;
pub mod ndjson;

pub use sse::parse_sse_stream;
pub use ndjson::parse_ndjson_stream;
```

- [ ] **Step 3.4: Add async-stream dependency**

```toml
# Cargo.toml
[dependencies]
async-stream = "0.3"
```

- [ ] **Step 3.5: Test streaming parsers**

```rust
// tests/backends/streaming_test.rs
use glyphnova::backends::streaming::{parse_sse_stream, parse_ndjson_stream};
use bytes::Bytes;
use futures::StreamExt;

#[test]
fn test_sse_parser() {
    let sse_data = "data: {\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\"Hello\"}}]}\n\n\
                    data: {\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\n\
                    data: [DONE]\n\n";

    let stream = futures::stream::iter(vec![
        Ok(Bytes::from(sse_data)),
    ]);

    let parsed_stream = parse_sse_stream(stream);
    let chunks: Vec<_> = tokio_test::block_on(async {
        parsed_stream.collect::<Vec<_>>().await
    });

    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].is_ok());
    assert_eq!(
        chunks[0].as_ref().unwrap().choices[0].delta.content,
        Some("Hello".to_string())
    );
}

#[test]
fn test_ndjson_parser() {
    let ndjson_data = "{\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\"Hello\"}}]}\n\
                        {\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n";

    let stream = futures::stream::iter(vec![
        Ok(Bytes::from(ndjson_data)),
    ]);

    let parsed_stream = parse_ndjson_stream(stream);
    let chunks: Vec<_> = tokio_test::block_on(async {
        parsed_stream.collect::<Vec<_>>().await
    });

    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].is_ok());
    assert_eq!(
        chunks[0].as_ref().unwrap().choices[0].delta.content,
        Some("Hello".to_string())
    );
}
```

Run: `cargo test test_streaming --lib`
Expected: PASS

- [ ] **Step 3.6: Commit**

```bash
git add src/backends/streaming/mod.rs src/backends/streaming/sse.rs src/backends/streaming/ndjson.rs Cargo.toml tests/backends/streaming_test.rs
git commit -m "feat(backends): add SSE and NDJSON streaming parsers"
```

---

### Step 4: Wire up backends module

- [ ] **Step 4.1: Write module exports**

```rust
// src/backends/mod.rs
pub mod trait;
pub mod types;
pub mod errors;
pub mod streaming;

pub use trait::{LlmBackend, MockBackend};
pub use types::*;
pub use errors::{LlmError, Result};
```

- [ ] **Step 4.2: Update lib.rs**

```rust
// src/lib.rs
pub mod backends;

pub use backends::{LlmBackend, MockBackend, types::*, errors::LlmError};
```

- [ ] **Step 4.3: Commit**

```bash
git add src/backends/mod.rs src/lib.rs
git commit -m "feat(backends): wire up backends module"
```

---

## Summary

This task implements the complete LLM backend abstraction including:

1. **Core types** (ChatRequest, ChatResponse, StreamChunk, ModelInfo, HealthStatus, BackendCapabilities)
2. **LlmBackend trait** with async methods for chat, streaming, model management, and health checks
3. **Mock backend** for testing all trait methods
4. **Streaming parsers** for SSE (LM Studio, llama.cpp, OpenAI) and NDJSON (Ollama)
5. **Comprehensive tests** for all components

**Mock Strategy:** The MockBackend provides in-memory implementation of all trait methods, enabling unit tests without external dependencies.

**Next:** Task 02 - LM Studio Backend Implementation
