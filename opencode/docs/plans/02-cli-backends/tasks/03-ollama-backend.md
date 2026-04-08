# Task 03: Ollama Backend

**Files:**
- Create: `src/backends/ollama/mod.rs`
- Create: `src/backends/ollama/client.rs`
- Modify: `src/backends/mod.rs` (add ollama module)
- Test: `tests/backends/ollama_test.rs`

---

## Overview

Implement Ollama backend integration. Ollama provides a native API on localhost:11434 with NDJSON streaming, 'think' parameter for chain-of-thought, and 'format' parameter for structured output.

---

## Implementation Steps

### Step 1: Create Ollama client

- [ ] **Step 1.1: Write client implementation**

```rust
// src/backends/ollama/client.rs
use super::super::{LlmBackend, types::*, errors::Result};
use super::super::streaming::parse_ndjson_stream;
use reqwest::Client as HttpClient;
use std::time::Duration;

pub struct OllamaBackend {
    client: HttpClient,
    base_url: String,
    timeout: Duration,
}

impl OllamaBackend {
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
        let url = format!("{}/api{}", self.base_url, path);

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
        let url = format!("{}/api{}", self.base_url, path);

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
        let url = format!("{}/api{}", self.base_url, path);

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
impl LlmBackend for OllamaBackend {
    async fn chat(&self, mut request: ChatRequest) -> Result<ChatResponse> {
        // Convert OpenAI-style request to Ollama format
        #[derive(serde::Serialize)]
        struct OllamaRequest {
            model: String,
            messages: Vec<OllamaMessage>,
            tools: Option<Vec<Tool>>,
            stream: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            format: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            think: Option<bool>,
        }

        #[derive(serde::Serialize)]
        struct OllamaMessage {
            role: String,
            content: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            tool_calls: Option<Vec<ToolCall>>,
        }

        let ollama_messages: Vec<OllamaMessage> = request.messages.iter().map(|m| match m {
            Message::System { content } => OllamaMessage {
                role: "system".to_string(),
                content: content.clone(),
                tool_calls: None,
            },
            Message::User { content } => OllamaMessage {
                role: "user".to_string(),
                content: content.clone(),
                tool_calls: None,
            },
            Message::Assistant { content, tool_calls } => OllamaMessage {
                role: "assistant".to_string(),
                content: content.clone().unwrap_or_default(),
                tool_calls: tool_calls.clone(),
            },
            Message::Tool { tool_call_id, content } => OllamaMessage {
                role: "tool".to_string(),
                content: format!("[Tool: {}] {}", tool_call_id, content),
                tool_calls: None,
            },
        }).collect();

        let format_val = request.format.take();
        let think_val = request.think.take();

        let ollama_request = OllamaRequest {
            model: request.model,
            messages: ollama_messages,
            tools: request.tools,
            stream: false,
            format: format_val,
            think: think_val,
        };

        #[derive(serde::Deserialize)]
        struct OllamaResponse {
            model: String,
            message: OllamaMessage,
            done: bool,
            #[serde(default)]
            eval_count: Option<u32>,
            #[serde(default)]
            prompt_eval_count: Option<u32>,
        }

        let ollama_response: OllamaResponse = self.post("/chat", &ollama_request).await?;

        // Convert back to OpenAI format
        Ok(ChatResponse {
            id: uuid::Uuid::new_v4().to_string(),
            object: "chat.completion".to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            model: ollama_response.model,
            choices: vec![Choice {
                index: 0,
                message: ResponseMessage {
                    role: ollama_response.message.role,
                    content: Some(ollama_response.message.content),
                    tool_calls: ollama_response.message.tool_calls,
                },
                finish_reason: if ollama_response.done { "stop" } else { "length" }.to_string(),
            }],
            usage: Usage {
                prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
                completion_tokens: ollama_response.eval_count.unwrap_or(0),
                total_tokens: ollama_response.prompt_eval_count.unwrap_or(0) +
                             ollama_response.eval_count.unwrap_or(0),
            },
        })
    }

    async fn chat_stream(
        &self,
        mut request: ChatRequest,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<StreamChunk>> + Send>>> {
        // Convert OpenAI-style request to Ollama format (same as non-streaming)
        #[derive(serde::Serialize)]
        struct OllamaRequest {
            model: String,
            messages: Vec<OllamaMessage>,
            tools: Option<Vec<Tool>>,
            stream: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            format: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            think: Option<bool>,
        }

        #[derive(serde::Serialize)]
        struct OllamaMessage {
            role: String,
            content: String,
        }

        let ollama_messages: Vec<OllamaMessage> = request.messages.iter().map(|m| match m {
            Message::System { content } => OllamaMessage {
                role: "system".to_string(),
                content: content.clone(),
            },
            Message::User { content } => OllamaMessage {
                role: "user".to_string(),
                content: content.clone(),
            },
            Message::Assistant { content, .. } => OllamaMessage {
                role: "assistant".to_string(),
                content: content.clone().unwrap_or_default(),
            },
            Message::Tool { tool_call_id, content } => OllamaMessage {
                role: "tool".to_string(),
                content: format!("[Tool: {}] {}", tool_call_id, content),
            },
        }).collect();

        let format_val = request.format.take();
        let think_val = request.think.take();

        let ollama_request = OllamaRequest {
            model: request.model,
            messages: ollama_messages,
            tools: request.tools,
            stream: true,
            format: format_val,
            think: think_val,
        };

        let response = self.post_stream("/chat", &ollama_request).await?;
        let byte_stream = response.bytes_stream();
        Ok(parse_ndjson_stream(byte_stream))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        #[derive(serde::Deserialize)]
        struct ModelsResponse {
            models: Vec<OllamaModel>,
        }

        #[derive(serde::Deserialize)]
        struct OllamaModel {
            name: String,
            modified_at: String,
            size: u64,
        }

        let response: ModelsResponse = self.get("/tags").await?;

        Ok(response.models.iter().map(|m| ModelInfo {
            id: m.name.clone(),
            object: "model".to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            owned_by: "ollama".to_string(),
        }).collect())
    }

    async fn health_check(&self) -> Result<super::super::types::HealthStatus> {
        // Ollama has a health endpoint
        #[derive(serde::Deserialize)]
        struct HealthResponse {
            status: String,
        }

        match self.get::<HealthResponse>("/health").await {
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

    async fn load_model(&self, model_id: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct LoadRequest {
            name: String,
        }

        let request = LoadRequest {
            name: model_id.to_string(),
        };

        self.post("/generate", &request).await
    }

    async fn unload_model(&self, model_id: &str) -> Result<()> {
        // Ollama automatically unloads models when not in use
        Ok(())
    }

    fn capabilities(&self) -> super::super::types::BackendCapabilities {
        super::super::types::BackendCapabilities {
            streaming: true,
            tools: true,
            tool_choice: false,  // Ollama doesn't support tool_choice
            response_format: true,
            system_messages: true,
            temperature: true,
            max_tokens: true,
            parallel_requests: false,
        }
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}
```

- [ ] **Step 1.2: Write module exports**

```rust
// src/backends/ollama/mod.rs
pub mod client;

pub use client::OllamaBackend;
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

pub use trait::{LlmBackend, MockBackend};
pub use types::*;
pub use errors::{LlmError, Result};
pub use lmstudio::LMStudioBackend;
pub use ollama::OllamaBackend;
```

- [ ] **Step 1.4: Add uuid dependency**

```toml
# Cargo.toml
[dependencies]
uuid = { version = "1.6", features = ["v4"] }
```

- [ ] **Step 1.5: Commit**

```bash
git add src/backends/ollama/mod.rs src/backends/ollama/client.rs src/backends/mod.rs Cargo.toml
git commit -m "feat(backends): add Ollama backend implementation"
```

---

### Step 2: Write tests with wiremock

- [ ] **Step 2.1: Write integration tests**

```rust
// tests/backends/ollama_test.rs
use glyphnova::backends::{OllamaBackend, types::*};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_ollama_chat() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "model": "test-model",
            "message": {
                "role": "assistant",
                "content": "Hello!"
            },
            "done": true,
            "eval_count": 5,
            "prompt_eval_count": 10
        })))
        .mount(&mock_server)
        .await;

    let backend = OllamaBackend::new(
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
        think: Some(true),
        format: Some("json".to_string()),
    };

    let response = backend.chat(request).await.unwrap();
    assert_eq!(response.choices[0].message.content, Some("Hello!".to_string()));
}

#[tokio::test]
async fn test_ollama_chat_stream() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200).set_body(
            "{\"model\":\"test\",\"message\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"done\":false}\n\
             {\"model\":\"test\",\"message\":{\"role\":\"assistant\",\"content\":\" world\"},\"done\":true}\n"
        ))
        .mount(&mock_server)
        .await;

    let backend = OllamaBackend::new(
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
async fn test_ollama_list_models() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{
                "name": "llama2:latest",
                "modified_at": "2024-01-01T00:00:00Z",
                "size": 3734928384
            }]
        })))
        .mount(&mock_server)
        .await;

    let backend = OllamaBackend::new(
        mock_server.host().as_str(),
        mock_server.port(),
        30,
    );
    backend.base_url = mock_server.uri();

    let models = backend.list_models().await.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "llama2:latest");
}

#[tokio::test]
async fn test_ollama_health_check() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .mount(&mock_server)
        .await;

    let backend = OllamaBackend::new(
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
git add tests/backends/ollama_test.rs
git commit -m "test(backends): add Ollama backend wiremock tests"
```

---

## Summary

This task implements Ollama backend including:

1. **Client implementation** with format conversion between OpenAI and Ollama APIs
2. **NDJSON streaming** support using the unified NDJSON parser
3. **Think and format parameters** for chain-of-thought and structured output
4. **Model load/unload** support
5. **Health check** using dedicated /health endpoint
6. **Wiremock tests** for all backend methods

**API Details:**
- Base URL: `http://localhost:11434/api`
- Chat: `POST /api/chat`
- Models: `GET /api/tags`
- Health: `GET /api/health`
- Streaming: NDJSON (Newline Delimited JSON)

**Next:** Task 04 - llama.cpp Backend Implementation
