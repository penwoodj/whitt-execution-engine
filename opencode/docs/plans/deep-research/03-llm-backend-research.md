# LLM Backend Research Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Document exact API protocols for all 4 LLM backends (LM Studio, Ollama, llama.cpp, OpenAI), including endpoints, request/response schemas, streaming formats, error codes, behavioral quirks, unified Rust trait design, and mock strategies for testing.

**Architecture:** API protocol documentation for LM Studio (localhost:1234, OpenAI-compatible), Ollama (localhost:11434, native API), llama.cpp (localhost:8080, OpenAI+Anthropic compatible), and OpenAI (api.openai.com), with unified Rust trait design and wiremock mocking strategy.

**Tech Stack**: HTTP/HTTPS, SSE (Server-Sent Events), NDJSON streaming, async/await, reqwest client, async-trait, tokio runtime, JSON Schema.

---

## Research Questions Being Answered

### Question 1: What are the Exact API Protocols for Each Backend?
**Why it matters**: Understanding exact API protocols ensures correct implementation, avoiding integration bugs and enabling proper error handling.

**Success criteria**: Documented API protocols for all 4 backends with endpoints, request/response schemas, streaming formats, error codes, and behavioral quirks.

**Integration point**: Phase 2 (Backend Abstraction)

---

### Question 2: How to Design a Unified Rust Trait for All Backends?
**Why it matters**: A unified trait enables backend-agnostic code, simplifying integration and enabling backend switching without code changes.

**Success criteria**: Documented unified Rust trait design with async methods, streaming support, error handling, and implementation for all 4 backends.

**Integration point**: Phase 2 (Backend Abstraction)

---

### Question 3: How to Mock Backends for Testing?
**Why it matters**: Mocking enables reliable, fast testing without real LLM connections, improving test coverage and CI speed.

**Success criteria**: Documented mock strategies using wiremock for all 4 backends with test examples and best practices.

**Integration point**: Phase 1 (Testing Infrastructure), Phase 2 (Backend Testing)

---

## Findings with Evidence

### Finding 1: LM Studio Uses OpenAI-Compatible API with SSE Streaming

**Evidence sources**:
- LM Studio documentation: https://lmstudio.ai/docs
- LM Studio API reference: https://lmstudio.ai/docs/server
- OpenAI API reference: https://platform.openai.com/docs/api-reference

**Summary**:
LM Studio provides an OpenAI-compatible API on localhost:1234 with SSE streaming. It supports chat completions, embeddings, and tool calling using the same request/response formats as OpenAI.

**Key details**:
- Base URL: `http://localhost:1234`
- Chat endpoint: `POST /v1/chat/completions`
- Streaming: SSE (Server-Sent Events)
- Tool calling: Supported via `tools` parameter
- Models: Loaded from GGUF files
- Authentication: None (local only)

**Request schema (OpenAI-compatible)**:
```json
{
  "model": "llama-3.2-3b-instruct",
  "messages": [
    {"role": "user", "content": "Hello, world!"}
  ],
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": true,
  "tools": [...]
}
```

**Response schema (OpenAI-compatible)**:
```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1699012345,
  "model": "llama-3.2-3b-instruct",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 20,
    "total_tokens": 30
  }
}
```

**Streaming format (SSE)**:
```
data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

**Behavioral quirks**:
- No authentication required
- Model name must match loaded GGUF file
- Streaming chunks can be empty
- Tool calling requires explicit tool definitions

---

### Finding 2: Ollama Uses Native API with NDJSON Streaming

**Evidence sources**:
- Ollama documentation: https://ollama.ai/docs
- Ollama API reference: https://github.com/ollama/ollama/blob/main/docs/api.md
- Ollama streaming guide: https://ollama.ai/docs/api#streaming

**Summary**:
Ollama provides a native API on localhost:11434 with NDJSON streaming. It supports chat completions, embeddings, and tool calling using a proprietary request/response format.

**Key details**:
- Base URL: `http://localhost:11434`
- Chat endpoint: `POST /api/chat`
- Streaming: NDJSON (Newline-Delimited JSON)
- Tool calling: Supported via `tools` parameter
- Models: Downloaded from ollama.ai library
- Authentication: None (local only)
- 'think' parameter: Supported for Chain-of-Thought

**Request schema (Ollama native)**:
```json
{
  "model": "llama3.2",
  "messages": [
    {"role": "user", "content": "Hello, world!"}
  ],
  "stream": true,
  "format": "json",
  "options": {
    "temperature": 0.7,
    "num_predict": 1000
  },
  "tools": [...],
  "think": {
    "type": "chain_of_thought"
  }
}
```

**Response schema (Ollama native)**:
```json
{
  "model": "llama3.2",
  "created_at": "2024-11-03T14:16:47.724Z",
  "message": {
    "role": "assistant",
    "content": "Hello! How can I help you today?"
  },
  "done": true,
  "total_duration": 1234567890,
  "load_duration": 12345678,
  "prompt_eval_count": 10,
  "prompt_eval_duration": 123456789,
  "eval_count": 20,
  "eval_duration": 1234567890
}
```

**Streaming format (NDJSON)**:
```json
{"model":"llama3.2","created_at":"2024-11-03T14:16:47.724Z","message":{"role":"assistant","content":"Hello"},"done":false}

{"model":"llama3.2","created_at":"2024-11-03T14:16:47.724Z","message":{"role":"assistant","content":"!"},"done":false}

{"model":"llama3.2","created_at":"2024-11-03T14:16:47.724Z","message":{"role":"assistant","content":""},"done":true}
```

**Behavioral quirks**:
- No authentication required
- Model name must match ollama.ai library name
- Streaming includes performance metrics
- 'think' parameter enables Chain-of-Thought reasoning
- Tool calling response format differs from OpenAI

---

### Finding 3: llama.cpp Uses OpenAI+Anthropic Compatible API

**Evidence sources**:
- llama.cpp documentation: https://github.com/ggerganov/llama.cpp
- llama.cpp server docs: https://github.com/ggerganov/llama.cpp/blob/master/examples/server/README.md
- OpenAI API reference: https://platform.openai.com/docs/api-reference
- Anthropic API reference: https://docs.anthropic.com/claude/reference

**Summary**:
llama.cpp provides an OpenAI-compatible API with some Anthropic extensions on localhost:8080. It supports chat completions with SSE streaming and a /health endpoint for health checks.

**Key details**:
- Base URL: `http://localhost:8080`
- Chat endpoint: `POST /v1/chat/completions`
- Streaming: SSE (Server-Sent Events)
- Tool calling: Supported via `tools` parameter (OpenAI-compatible)
- Models: Loaded from GGUF files via command line
- Authentication: None (local only)
- Health endpoint: `GET /health`

**Request schema (OpenAI+Anthropic compatible)**:
```json
{
  "model": "llama-3.2-3b-instruct",
  "messages": [
    {"role": "user", "content": "Hello, world!"}
  ],
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": true,
  "tools": [...],
  "anthropic_version": "2023-06-01"
}
```

**Response schema (OpenAI+Anthropic compatible)**:
```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1699012345,
  "model": "llama-3.2-3b-instruct",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 20,
    "total_tokens": 30
  }
}
```

**Streaming format (SSE)**:
```
data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

**Health endpoint**:
```json
{
  "status": "ok",
  "model": "llama-3.2-3b-instruct",
  "slots_idle": 4,
  "slots_processing": 0
}
```

**Behavioral quirks**:
- No authentication required
- Model name must match loaded GGUF file
- /health endpoint for health checks
- Anthropic version field for compatibility
- Tool calling format may vary

---

### Finding 4: OpenAI is the Reference Standard

**Evidence sources**:
- OpenAI API documentation: https://platform.openai.com/docs/api-reference
- OpenAI streaming guide: https://platform.openai.com/docs/api-reference/chat/create
- OpenAI error codes: https://platform.openai.com/docs/guides/error-codes

**Summary**:
OpenAI provides the reference standard for LLM APIs on api.openai.com with SSE streaming. All other backends (LM Studio, Ollama, llama.cpp) aim for OpenAI compatibility.

**Key details**:
- Base URL: `https://api.openai.com`
- Chat endpoint: `POST /v1/chat/completions`
- Streaming: SSE (Server-Sent Events)
- Tool calling: Supported via `tools` parameter
- Models: GPT-4, GPT-3.5, etc.
- Authentication: Bearer token (API key)

**Request schema**:
```json
{
  "model": "gpt-4",
  "messages": [
    {"role": "user", "content": "Hello, world!"}
  ],
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": true,
  "tools": [...]
}
```

**Response schema**:
```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1699012345,
  "model": "gpt-4-0613",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 20,
    "total_tokens": 30
  }
}
```

**Streaming format (SSE)**:
```
data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

**Error codes**:
- `invalid_request_error`: 400 - Invalid request
- `invalid_api_key`: 401 - Invalid API key
- `rate_limit_exceeded`: 429 - Rate limit exceeded
- `server_error`: 500 - Server error

**Behavioral quirks**:
- Requires API key authentication
- Strict rate limiting
- Multiple model versions
- Tool calling response format varies by model

---

### Finding 5: Unified Rust Trait Design

**Evidence sources**:
- async-trait documentation: https://docs.rs/async-trait/latest/async_trait/
- Rust async patterns: https://rust-lang.github.io/async-book/
- Backend abstraction patterns: https://github.com/rust-lang-async/async-std

**Summary**:
A unified Rust trait with async methods enables backend-agnostic code. The trait should support chat (blocking and streaming), tool calling, and error handling.

**Trait design**:
```rust
use async_trait::async_trait;
use futures::Stream;
use anyhow::Result;

/// Unified backend trait for all LLM backends
#[async_trait]
pub trait Backend: Send + Sync {
    /// Chat completion (blocking)
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Chat completion (streaming)
    async fn chat_stream(
        &self,
        request: ChatRequest
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>>;

    /// Check backend health
    async fn health(&self) -> Result<HealthStatus>;
}

/// Chat request (unified format)
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
    pub tools: Option<Vec<Tool>>,
    pub extra_params: HashMap<String, serde_json::Value>,
}

/// Chat response (unified format)
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// Streaming chunk (unified format)
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: Option<String>,
    pub finish_reason: Option<String>,
}

/// Health status
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy { model: String },
    Unhealthy { reason: String },
}
```

---

### Finding 6: Wiremock Mocking Strategy

**Evidence sources**:
- wiremock documentation: https://docs.rs/wiremock/latest/wiremock/
- wiremock examples: https://github.com/LukeMathWalker/wiremock-rs
- HTTP mocking patterns: https://blog.logrocket.com/rust-http-mocking/

**Summary**:
wiremock enables mocking HTTP backends for testing. Each backend can be mocked with specific request matchers and response stubs, enabling reliable and fast tests without real LLM connections.

**Mocking example for LM Studio**:
```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_lmstudio_backend() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1699012345,
            "model": "llama-3.2-3b-instruct",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello, world!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        })))
        .mount(&mock_server)
        .await;

    let backend = LMStudioBackend {
        client: reqwest::Client::new(),
        base_url: mock_server.uri(),
    };

    let request = ChatRequest {
        model: "llama-3.2-3b-instruct".to_string(),
        messages: vec![],
        temperature: Some(0.7),
        max_tokens: Some(1000),
        stream: false,
        tools: None,
        extra_params: HashMap::new(),
    };

    let response = backend.chat(request).await?;
    assert_eq!(response.choices[0].message.content, "Hello, world!");
}
```

---

## Recommendations with Rationale

### Recommendation 1: Implement Unified Backend Trait with async-trait

**Why**: A unified trait enables backend-agnostic code, simplifying integration and enabling backend switching without code changes.

**Trade-offs**:
- Pros: Backend-agnostic code, easy backend switching, type-safe
- Cons: Slight overhead for dynamic dispatch, complex trait bounds

**Alternatives considered**:
- Enum-based backends: Less flexible, more boilerplate
- Separate implementations: More code duplication
- No abstraction: Tightly coupled to specific backends

**Adoption priority**: **P0 (Critical for Phase 2)**

---

### Recommendation 2: Support SSE Streaming for LM Studio and llama.cpp

**Why**: SSE is the standard streaming format for OpenAI-compatible APIs (LM Studio, llama.cpp). It enables real-time response generation.

**Trade-offs**:
- Pros: Real-time streaming, standard format, widely supported
- Cons: Requires SSE parsing, can be complex

**Alternatives considered**:
- NDJSON (Ollama): Not compatible with LM Studio/llama.cpp
- WebSockets: Overkill, not widely supported
- No streaming: Poor user experience

**Adoption priority**: **P0 (Critical for Phase 2)**

---

### Recommendation 3: Support NDJSON Streaming for Ollama

**Why**: Ollama uses NDJSON streaming, which differs from SSE. Supporting both formats enables all 4 backends.

**Trade-offs**:
- Pros: Ollama compatibility, simple parsing
- Cons: Multiple streaming formats to support

**Alternatives considered**:
- Only SSE: Ollama incompatible
- Only NDJSON: LM Studio/llama.cpp incompatible
- No streaming: Poor user experience

**Adoption priority**: **P1 (Important for Phase 2)**

---

### Recommendation 4: Use wiremock for All Backend Testing

**Why**: wiremock enables reliable, fast testing without real LLM connections. It supports all HTTP methods, matchers, and streaming.

**Trade-offs**:
- Pros: Reliable, fast, comprehensive mocking
- Cons: Requires async runtime in tests, learning curve

**Alternatives considered**:
- httpmock: Similar, but less ergonomic
- mockito: Simpler, but fewer features
- No mocking: Tests depend on external services

**Adoption priority**: **P1 (Important for Phase 1)**

---

## Integration Instructions

### Integration Point 1: Backend Abstraction Trait

**What to implement**:
Define unified Backend trait with async methods. Implement trait for all 4 backends (LM Studio, Ollama, llama.cpp, OpenAI).

**File locations**:
- `src/backend/mod.rs`: Backend trait definition
- `src/backend/lmstudio.rs`: LM Studio backend implementation
- `src/backend/ollama.rs`: Ollama backend implementation
- `src/backend/llamacpp.rs`: llama.cpp backend implementation
- `src/backend/openai.rs`: OpenAI backend implementation

**Code patterns**:

```rust
// src/backend/mod.rs
use async_trait::async_trait;
use futures::Stream;
use anyhow::Result;

/// Unified backend trait
#[async_trait]
pub trait Backend: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn chat_stream(&self, request: ChatRequest) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>>;
    async fn health(&self) -> Result<HealthStatus>;
}

// Unified types
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
    pub tools: Option<Vec<Tool>>,
    pub extra_params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: Option<String>,
    pub finish_reason: Option<String>,
}
```

**Testing requirements**:
- Unit tests for trait methods
- Integration tests for each backend implementation
- Mock tests with wiremock for all backends

---

### Integration Point 2: Streaming Support

**What to implement**:
Implement SSE streaming for LM Studio and llama.cpp. Implement NDJSON streaming for Ollama. Parse streaming responses and yield chunks.

**File locations**:
- `src/backend/streaming.rs`: Streaming utilities
- `src/backend/sse.rs`: SSE parsing
- `src/backend/ndjson.rs`: NDJSON parsing

**Code patterns**:

```rust
// src/backend/sse.rs
use futures::Stream;
use pin_project::pin_project;
use anyhow::Result;

/// Parse SSE stream
pub async fn parse_sse_stream(
    response: reqwest::Response
) -> impl Stream<Item = Result<StreamChunk>> {
    let mut stream = response.bytes_stream();

    async_stream::stream! {
        let mut buffer = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);

            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = std::str::from_utf8(&buffer[..pos])?;
                buffer.drain(..=pos);

                if line.starts_with("data: ") {
                    let json_str = &line[6..];
                    if json_str == "[DONE]" {
                        yield Ok(StreamChunk { content: None, finish_reason: Some("stop".to_string()) });
                        break;
                    }

                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let chunk = parse_sse_chunk(&value)?;
                        yield Ok(chunk);
                    }
                }
            }
        }
    }
}

fn parse_sse_chunk(value: &serde_json::Value) -> Result<StreamChunk> {
    let delta = &value["choices"][0]["delta"];
    let content = delta["content"].as_str().map(|s| s.to_string());
    let finish_reason = value["choices"][0]["finish_reason"].as_str().map(|s| s.to_string());
    Ok(StreamChunk { content, finish_reason })
}
```

**Testing requirements**:
- Unit tests for SSE parsing
- Unit tests for NDJSON parsing
- Integration tests with real backends (optional)
- Mock tests with wiremock

---

### Integration Point 3: Wiremock Testing

**What to implement**:
Set up wiremock mock servers for all 4 backends. Create mock fixtures for common scenarios (success, error, streaming).

**File locations**:
- `tests/backend/mocks/lmstudio.rs`: LM Studio mocks
- `tests/backend/mocks/ollama.rs`: Ollama mocks
- `tests/backend/mocks/llamacpp.rs`: llama.cpp mocks
- `tests/backend/mocks/openai.rs`: OpenAI mocks

**Code patterns**:

```rust
// tests/backend/mocks/lmstudio.rs
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, json_body};
use serde_json::json;

pub async fn setup_lmstudio_mock() -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1699012345,
            "model": "llama-3.2-3b-instruct",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello, world!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        })))
        .mount(&mock_server)
        .await;

    mock_server
}

pub async fn setup_lmstudio_streaming_mock() -> MockServer {
    let mock_server = MockServer::start().await;

    let sse_response = "data: {\"id\":\"chatcmpl-123\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\
data: {\"id\":\"chatcmpl-123\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"!\"},\"finish_reason\":null}]}\n\
data: {\"id\":\"chatcmpl-123\",\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}]}\n\
data: [DONE]\n";

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(sse_response))
        .mount(&mock_server)
        .await;

    mock_server
}
```

**Testing requirements**:
- Mock tests for all 4 backends
- Mock tests for streaming
- Mock tests for error scenarios
- Integration tests with real backends (optional)

---

## Validation Criteria

### Criteria 1: Unified Backend Trait Supports All 4 Backends

**How to verify**:
1. Check that all 4 backends implement the Backend trait
2. Verify trait methods are implemented for all backends
3. Verify streaming works for all backends

**Step-by-step verification process**:
```bash
# Check trait implementations
grep -r 'impl Backend for' src/backend/

# Run unit tests
cargo test backend::trait

# Run integration tests
cargo test backend::integration
```

**Expected outcome**:
- All 4 backends implement the Backend trait
- All trait methods are implemented
- Streaming works for all backends

**Integration point**: Phase 2 (Backend Abstraction)

---

### Criteria 2: All Backends Handle Streaming Responses Correctly

**How to verify**:
1. Run streaming tests for all backends
2. Verify streaming chunks are parsed correctly
3. Verify streaming ends with finish_reason

**Step-by-step verification process**:
```bash
# Run streaming tests
cargo test backend::streaming

# Test with wiremock mocks
cargo test backend::mocks::streaming

# Test with real backends (optional)
cargo test backend::streaming -- --ignored
```

**Expected outcome**:
- All streaming tests pass
- Streaming chunks are parsed correctly
- Streaming ends with finish_reason

**Integration point**: Phase 2 (Backend Abstraction)

---

### Criteria 3: All Integration Tests Pass with Wiremock

**How to verify**:
1. Run all backend integration tests
2. Verify wiremock mocks are set up correctly
3. Verify all test scenarios pass

**Step-by-step verification process**:
```bash
# Run all backend tests
cargo test backend

# Verify wiremock setup
cargo test backend::mocks

# Run CI tests
cargo test --workspace
```

**Expected outcome**:
- All backend tests pass with wiremock
- Wiremock mocks are set up correctly
- All test scenarios pass

**Integration point**: Phase 1 (Testing Infrastructure), Phase 2 (Backend Testing)

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1: Prevent Drift into Non-Standard APIs

**Drift risk**: Research could recommend non-standard API protocols or proprietary formats that are not widely adopted.

**Detection method**: Verify all API protocols follow standards (OpenAI API for LM Studio/llama.cpp, native API for Ollama). Reject proprietary or non-standard approaches.

**Validation**:
```bash
# Check for OpenAI compatibility
grep -r "openai.com" src/backend/lmstudio.rs src/backend/llamacpp.rs

# Check for Ollama native API
grep -r "ollama.ai" src/backend/ollama.rs

# Check for streaming standards
grep -r "SSE\|NDJSON" src/backend/streaming.rs
```

**Correction action**: If non-standard APIs are recommended, replace with standard approaches.

---

## Research Tasks

### Task 1: Document API Protocols for All 4 Backends

**Files:**
- Create: `./workspace/plans/research/evidence/lmstudio-api-protocol.yaml`
- Create: `./workspace/plans/research/evidence/ollama-api-protocol.yaml`
- Create: `./workspace/plans/research/evidence/llamacpp-api-protocol.yaml`
- Create: `./workspace/plans/research/evidence/openai-api-protocol.yaml`

- [ ] **Step 1: Document LM Studio API protocol**

Document LM Studio endpoints, request/response schemas, streaming format, error codes

Expected output: `lmstudio-api-protocol.yaml`

- [ ] **Step 2: Document Ollama API protocol**

Document Ollama endpoints, request/response schemas, streaming format, error codes

Expected output: `ollama-api-protocol.yaml`

- [ ] **Step 3: Document llama.cpp API protocol**

Document llama.cpp endpoints, request/response schemas, streaming format, error codes

Expected output: `llamacpp-api-protocol.yaml`

- [ ] **Step 4: Document OpenAI API protocol**

Document OpenAI endpoints, request/response schemas, streaming format, error codes

Expected output: `openai-api-protocol.yaml`

- [ ] **Step 5: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add llm backend api protocols"`
Expected: Git commit successful

---

### Task 2: Design Unified Backend Trait

**Files:**
- Create: `./workspace/plans/research/evidence/unified-backend-trait.md`

- [ ] **Step 1: Design Backend trait with async methods**

Design trait with chat, chat_stream, and health methods. Include unified types.

Expected output: Backend trait definition

- [ ] **Step 2: Design streaming support**

Design streaming support for SSE and NDJSON formats.

Expected output: Streaming utilities design

- [ ] **Step 3: Create unified backend trait document**

Write analysis of unified backend trait design with code examples.

Expected output: `unified-backend-trait.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add unified backend trait design evidence"`
Expected: Git commit successful

---

### Task 3: Create Wiremock Mocking Strategy

**Files:**
- Create: `./workspace/plans/research/evidence/wiremock-mocking-strategy.md`
- Create: `tests/backend/mocks/lmstudio.rs`
- Create: `tests/backend/mocks/ollama.rs`
- Create: `tests/backend/mocks/llamacpp.rs`
- Create: `tests/backend/mocks/openai.rs`

- [ ] **Step 1: Design wiremock mocking strategy**

Design mocking strategy for all 4 backends with test scenarios.

Expected output: Mocking strategy design

- [ ] **Step 2: Create wiremock mock implementations**

Create mock implementations for all 4 backends.

Expected output: Mock implementations

- [ ] **Step 3: Create wiremock mocking strategy document**

Write analysis of wiremock mocking strategy with code examples.

Expected output: `wiremock-mocking-strategy.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ tests/backend/mocks/ && git commit -m "feat: add wiremock mocking strategy evidence"`
Expected: Git commit successful

---

### Task 4: Complete All Research Validation and Integration

**Files:**
- Modify: `opencode/docs/plans/deep-research/03-llm-backend-research.md`
- Create: `./workspace/plans/research/validation/llm-backend-validation-report.md`

- [ ] **Step 1: Run all validation scripts**

Run: `bash scripts/validate-research-completeness.sh opencode/docs/plans/deep-research/03-llm-backend-research.md`
Expected: All validation checks pass

- [ ] **Step 2: Run evidence quality validation**

Run: `bash scripts/validate-evidence-quality.sh opencode/docs/plans/deep-research/03-llm-backend-research.md`
Expected: 100% evidence quality

- [ ] **Step 3: Run integration completeness validation**

Run: `bash scripts/validate-integration-completeness.sh opencode/docs/plans/deep-research/03-llm-backend-research.md`
Expected: 100% integration completeness

- [ ] **Step 4: Run traceability validation**

Run: `bash scripts/validate-traceability.sh opencode/docs/plans/deep-research/03-llm-backend-research.md`
Expected: 100% traceability

- [ ] **Step 5: Create validation report**

Write validation report summarizing all validation results and confirming research completion

Expected output: `llm-backend-validation-report.md`

- [ ] **Step 6: Commit validation report**

Run: `git add ./workspace/plans/research/validation/ && git commit -m "feat: add llm backend research validation report"`
Expected: Git commit successful

---

## References

1. **LM Studio**: https://lmstudio.ai/docs
2. **Ollama**: https://ollama.ai/docs
3. **llama.cpp**: https://github.com/ggerganov/llama.cpp
4. **OpenAI API**: https://platform.openai.com/docs/api-reference
5. **async-trait**: https://docs.rs/async-trait/latest/async_trait/
6. **wiremock**: https://docs.rs/wiremock/latest/wiremock/
7. **ADR-0002**: MVP Queue & Scheduler Architecture Decision
8. **ADR-0003**: CLI & Backend Architecture Decision

---

**End of LLM Backend Research Plan**
