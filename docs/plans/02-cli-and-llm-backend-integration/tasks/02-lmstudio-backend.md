# Task 02: LM Studio Backend

**Files:**
- Create: `src/backends/lmstudio/mod.rs`
- Create: `src/backends/lmstudio/client.rs`
- Modify: `src/backends/mod.rs` (add lmstudio module)
- Test: `tests/backends/lmstudio_test.rs`

---

## Overview

Implement LM Studio backend integration. LM Studio provides an OpenAI-compatible API on localhost:1234 with SSE streaming support, tool calling, and response formatting.

---

## Implementation Steps

### Step 1: Create LM Studio client

- [ ] **Step 1.1: Write client implementation**

```rust
// src/backends/lmstudio/client.rs
use super::super::{LlmBackend, types::*, errors::Result};
use super::super::streaming::parse_sse_stream;
use reqwest::Client as HttpClient;
use std::time::Duration;

pub struct LMStudioBackend {
    client: HttpClient,
    base_url: String,
    timeout: Duration,
}

impl LMStudioBackend {
    pub fn new(host: &str, port: u16, timeout_secs: u64) -> Self {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: format!("http://{}:{}", host, port),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
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

    async fn post_stream<T>(&self, path: &str, body: &T) -> Result<reqwest::Response>
    where
        T: serde::Serialize,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
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

        Ok(response)
    }

    async fn get<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{}/v1{}", self.base_url, path);

        let response = self.client
            .get(&url)
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
impl LlmBackend for LMStudioBackend {
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
        // LM Studio doesn't have a dedicated health endpoint, so we list models
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
        // LM Studio doesn't support explicit model loading
        Ok(())
    }

    async fn unload_model(&self, _model_id: &str) -> Result<()> {
        // LM Studio doesn't support explicit model unloading
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
            parallel_requests: false,
        }
    }

    fn name(&self) -> &'static str {
        "lmstudio"
    }
}
```

- [ ] **Step 1.2: Write module exports**

```rust
// src/backends/lmstudio/mod.rs
pub mod client;

pub use client::LMStudioBackend;
```

- [ ] **Step 1.3: Update backends mod.rs**

```rust
// src/backends/mod.rs
pub mod trait;
pub mod types;
pub mod errors;
pub mod streaming;
pub mod lmstudio;

pub use trait::{LlmBackend, MockBackend};
pub use types::*;
pub use errors::{LlmError, Result};
pub use lmstudio::LMStudioBackend;
```

- [ ] **Step 1.4: Commit**

```bash
git add src/backends/lmstudio/mod.rs src/backends/lmstudio/client.rs src/backends/mod.rs
git commit -m "feat(backends): add LM Studio backend implementation"
```

---

### Step 2: Write tests with wiremock

- [ ] **Step 2.1: Write integration tests**

```rust
// tests/backends/lmstudio_test.rs
use whitt_execution_engine::backends::{LMStudioBackend, types::*};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_lmstudio_chat() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "test-id",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "test-model",
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

    let backend = LMStudioBackend::new(
        mock_server.host().as_str(),
        mock_server.port(),
        30,
    );
    backend.base_url = mock_server.uri();

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

    let response = backend.chat(request).await.unwrap();
    assert_eq!(response.choices[0].message.content, Some("Hello!".to_string()));
}

#[tokio::test]
async fn test_lmstudio_chat_stream() {
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

    let backend = LMStudioBackend::new(
        mock_server.host().as_str(),
        mock_server.port(),
        30,
    );
    backend.base_url = mock_server.uri();

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

    let stream = backend.chat_stream(request).await.unwrap();
    let chunks: Vec<_> = futures::StreamExt::collect(stream).await;

    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].is_ok());
    assert!(chunks[1].is_ok());
}

#[tokio::test]
async fn test_lmstudio_list_models() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "object": "list",
            "data": [{
                "id": "test-model",
                "object": "model",
                "created": 1234567890,
                "owned_by": "lmstudio"
            }]
        })))
        .mount(&mock_server)
        .await;

    let backend = LMStudioBackend::new(
        mock_server.host().as_str(),
        mock_server.port(),
        30,
    );
    backend.base_url = mock_server.uri();

    let models = backend.list_models().await.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "test-model");
}

#[tokio::test]
async fn test_lmstudio_health_check() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "object": "list",
            "data": []
        })))
        .mount(&mock_server)
        .await;

    let backend = LMStudioBackend::new(
        mock_server.host().as_str(),
        mock_server.port(),
        30,
    );
    backend.base_url = mock_server.uri();

    let health = backend.health_check().await.unwrap();
    assert_eq!(health.status, "ok");
}
```

- [ ] **Step 2.2: Commit**

```bash
git add tests/backends/lmstudio_test.rs
git commit -m "test(backends): add LM Studio backend wiremock tests"
```

---

## Summary

This task implements LM Studio backend including:

1. **Client implementation** with POST/GET methods for chat completions and model listing
2. **SSE streaming** support using the unified SSE parser
3. **Health check** using model list endpoint
4. **Wiremock tests** for all backend methods

**API Details:**
- Base URL: `http://localhost:1234/v1`
- Chat: `POST /v1/chat/completions`
- Models: `GET /v1/models`
- Streaming: Server-Sent Events (SSE)

**Next:** Task 03 - Ollama Backend Implementation
