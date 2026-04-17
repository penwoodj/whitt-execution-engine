# Task 05: OpenAI Backend

**Files:**
- Create: `src/backends/openai/mod.rs`
- Create: `src/backends/openai/client.rs`
- Modify: `src/backends/mod.rs` (add openai module)
- Test: `tests/backends/openai_test.rs`

---

## Overview

Implement OpenAI backend integration. OpenAI provides the reference API at api.openai.com with SSE streaming and rate limiting (HTTP 429 responses).

---

## Implementation Steps

### Step 1: Create OpenAI client

- [ ] **Step 1.1: Write client implementation**

```rust
// src/backends/openai/client.rs
use super::super::{LlmBackend, types::*, errors::Result};
use super::super::streaming::parse_sse_stream;
use reqwest::Client as HttpClient;
use std::time::Duration;

pub struct OpenAIBackend {
    client: HttpClient,
    api_key: String,
    base_url: String,
    timeout: Duration,
}

impl OpenAIBackend {
    pub fn new(api_key: &str, timeout_secs: u64) -> Self {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key: api_key.to_string(),
            base_url: "https://api.openai.com".to_string(),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let mut response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .timeout(self.timeout)
            .send()
            .await?;

        // Handle rate limiting
        if response.status() == 429 {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return Err(super::super::errors::LlmError::RateLimit(format!(
                "Rate limit exceeded, retry after {}s",
                retry_after
            )));
        }

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(super::super::errors::LlmError::Backend(
                format!("HTTP {}: {}", status, text)
            ));
        }

        Ok(response.json().await?)
    }

    async fn post_stream<T>(&self, path: &str, body: &T) -> Result<reqwest::Response>
    where
        T: serde::Serialize,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let mut response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .timeout(self.timeout)
            .send()
            .await?;

        // Handle rate limiting
        if response.status() == 429 {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return Err(super::super::errors::LlmError::RateLimit(format!(
                "Rate limit exceeded, retry after {}s",
                retry_after
            )));
        }

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(super::super::errors::LlmError::Backend(
                format!("HTTP {}: {}", status, text)
            ));
        }

        Ok(response)
    }

    async fn get<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let mut response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(super::super::errors::LlmError::Backend(
                format!("HTTP {}: {}", status, text)
            ));
        }

        Ok(response.json().await?)
    }
}

#[async_trait::async_trait]
impl LlmBackend for OpenAIBackend {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        self.post("/chat/completions", &request).await
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<StreamChunk>> + Send>>> {
        let response = self.post_stream("/chat/completions", &request).await?;
        let byte_stream = response.bytes_stream();
        Ok(parse_sse_stream(byte_stream))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        #[derive(serde::Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelInfo>,
        }

        let response: ModelsResponse = self.get("/models").await?;
        Ok(response.data)
    }

    async fn health_check(&self) -> Result<super::super::types::HealthStatus> {
        // OpenAI doesn't have a dedicated health endpoint, so we list models
        match self.list_models().await {
            Ok(_) => Ok(super::super::types::HealthStatus {
                status: "ok".to_string(),
                message: None,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }),
            Err(e) => Ok(super::super::types::HealthStatus {
                status: "unavailable".to_string(),
                message: Some(e.to_string()),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }),
        }
    }

    async fn load_model(&self, _model_id: &str) -> Result<()> {
        // OpenAI doesn't support explicit model loading
        Ok(())
    }

    async fn unload_model(&self, _model_id: &str) -> Result<()> {
        // OpenAI doesn't support explicit model unloading
        Ok(())
    }

    fn capabilities(&self) -> super::super::types::BackendCapabilities {
        super::super::types::BackendCapabilities {
            streaming: true,
            tools: true,
            tool_choice: true,
            response_format: true,
            system_messages: true,
            temperature: true,
            max_tokens: true,
            parallel_requests: true,
        }
    }

    fn name(&self) -> &'static str {
        "openai"
    }
}
```

- [ ] **Step 1.2: Write module exports**

```rust
// src/backends/openai/mod.rs
pub mod client;

pub use client::OpenAIBackend;
```

- [ ] **Step 1.3: Update backends mod.rs**

```rust
// src/backends/mod.rs
pub mod trait;
pub mod types;
pub mod errors;
pub mod streaming;
pub mod lmstudio;
pub mod ollama;
pub mod llamacpp;
pub mod openai;

pub use trait::{LlmBackend, MockBackend};
pub use types::*;
pub use errors::{LlmError, Result};
pub use lmstudio::LMStudioBackend;
pub use ollama::OllamaBackend;
pub use llamacpp::LlamaCppBackend;
pub use openai::OpenAIBackend;
```

- [ ] **Step 1.4: Commit**

```bash
git add src/backends/openai/mod.rs src/backends/openai/client.rs src/backends/mod.rs
git commit -m "feat(backends): add OpenAI backend implementation"
```

---

### Step 2: Write tests with wiremock

- [ ] **Step 2.1: Write integration tests**

```rust
// tests/backends/openai_test.rs
use whitt_execution_engine::backends::{OpenAIBackend, types::*};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_openai_chat() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("Authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "test-id",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello!",
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        })))
        .mount(&mock_server)
        .await;

    let mut backend = OpenAIBackend::new("test-api-key", 30);
    backend.base_url = mock_server.uri();

    let request = ChatRequest {
        model: "gpt-4".to_string(),
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

    let response = backend.chat(request).await.unwrap();
    assert_eq!(response.choices[0].message.content, Some("Hello!".to_string()));
}

#[tokio::test]
async fn test_openai_rate_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(429)
            .insert_header("Retry-After", "60"))
        .mount(&mock_server)
        .await;

    let mut backend = OpenAIBackend::new("test-api-key", 30);
    backend.base_url = mock_server.uri();

    let request = ChatRequest {
        model: "gpt-4".to_string(),
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

    let result = backend.chat(request).await;
    assert!(result.is_err());

    if let Err(LlmError::RateLimit(msg)) = result {
        assert!(msg.contains("60s"));
    } else {
        panic!("Expected RateLimit error");
    }
}

#[tokio::test]
async fn test_openai_chat_stream() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body(
            "data: {\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\"Hello\"}}]}\n\n\
             data: {\"id\":\"test\",\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\n\
             data: [DONE]\n\n"
        ))
        .mount(&mock_server)
        .await;

    let mut backend = OpenAIBackend::new("test-api-key", 30);
    backend.base_url = mock_server.uri();

    let request = ChatRequest {
        model: "gpt-4".to_string(),
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

    let stream = backend.chat_stream(request).await.unwrap();
    let chunks: Vec<_> = futures::StreamExt::collect(stream).await;

    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].is_ok());
    assert!(chunks[1].is_ok());
}

#[tokio::test]
async fn test_openai_list_models() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "object": "list",
            "data": [{
                "id": "gpt-4",
                "object": "model",
                "created": 1234567890,
                "owned_by": "openai"
            }, {
                "id": "gpt-3.5-turbo",
                "object": "model",
                "created": 1234567890,
                "owned_by": "openai"
            }]
        })))
        .mount(&mock_server)
        .await;

    let mut backend = OpenAIBackend::new("test-api-key", 30);
    backend.base_url = mock_server.uri();

    let models = backend.list_models().await.unwrap();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, "gpt-4");
    assert_eq!(models[1].id, "gpt-3.5-turbo");
}

#[tokio::test]
async fn test_openai_health_check() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "object": "list",
            "data": []
        })))
        .mount(&mock_server)
        .await;

    let mut backend = OpenAIBackend::new("test-api-key", 30);
    backend.base_url = mock_server.uri();

    let health = backend.health_check().await.unwrap();
    assert_eq!(health.status, "ok");
}
```

- [ ] **Step 2.2: Commit**

```bash
git add tests/backends/openai_test.rs
git commit -m "test(backends): add OpenAI backend wiremock tests with rate limiting"
```

---

## Summary

This task implements OpenAI backend including:

1. **Client implementation** with API key authentication
2. **Rate limiting** handling (HTTP 429 with Retry-After header)
3. **SSE streaming** support using the unified SSE parser
4. **Health check** using model list endpoint
5. **Parallel requests** capability (unique to OpenAI)
6. **Wiremock tests** for all backend methods including rate limiting

**API Details:**
- Base URL: `https://api.openai.com/v1`
- Chat: `POST /v1/chat/completions`
- Models: `GET /v1/models`
- Rate Limit: HTTP 429 with `Retry-After` header
- Streaming: Server-Sent Events (SSE)

**Next:** Task 06 - Backend Registry
