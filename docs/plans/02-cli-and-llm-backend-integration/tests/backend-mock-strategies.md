# Backend Mock Strategies

This document provides detailed mock strategies for testing LLM backends using wiremock. All backend tests should use these mock strategies instead of calling real APIs.

---

## Why Wiremock?

**Benefits:**
- **Fast:** No network latency, tests run in milliseconds
- **Reliable:** No flakiness from network issues
- **Deterministic:** Same input always produces same output
- **Isolated:** Tests don't depend on external services
- **Offline:** Tests work without internet

**When to Use:**
- Unit tests for backend implementations
- Integration tests for CLI-backend interaction
- Tests that need specific responses (errors, rate limits)
- CI/CD pipelines (to avoid rate limits and costs)

**When NOT to Use:**
- Manual testing with real backends
- Benchmarking performance
- Validating backend compatibility
- Testing streaming with real latency

---

## Mock Strategy Overview

### Generic Pattern

```rust
use wiremock::{
    MockServer, Mock, ResponseTemplate,
    matchers::{method, path, header, body_json},
    http::StatusCode,
};

async fn test_with_mock() {
    // 1. Start mock server
    let mock_server = MockServer::start().await;

    // 2. Set up mock responses
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "id": "test-id",
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "Hello!"
                    }
                }]
            })))
        .mount(&mock_server)
        .await;

    // 3. Test with mock
    let mut backend = Backend::new("config");
    backend.base_url = mock_server.uri();
    let result = backend.chat(request).await.unwrap();

    // 4. Verify result
    assert_eq!(result.choices[0].message.content, Some("Hello!".to_string()));
}
```

---

## LM Studio Mock Strategy

### Successful Chat Completion

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "test-model",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "This is a test response from LM Studio",
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
```

### Streaming Chat Completion (SSE)

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(200).set_body(
        r#"data: {"id":"test","choices":[{"delta":{"role":"assistant","content":"Hello"}}]}
data: {"id":"test","choices":[{"delta":{"content":" world"}}]}
data: {"id":"test","choices":[{"delta":{}, "finish_reason":"stop"}]}
data: [DONE]
"#
    ))
    .mount(&mock_server)
    .await;
```

### Model List

```rust
let mock = Mock::given(method("GET"))
    .and(path("/v1/models"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "object": "list",
        "data": [
            {
                "id": "llama-2-7b",
                "object": "model",
                "created": 1677610602,
                "owned_by": "lmstudio"
            },
            {
                "id": "mistral-7b",
                "object": "model",
                "created": 1677610603,
                "owned_by": "lmstudio"
            }
        ]
    })))
    .mount(&mock_server)
    .await;
```

### With Tools

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": "chatcmpl-123",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": null,
                "tool_calls": [{
                    "id": "call_123",
                    "type": "function",
                    "function": {
                        "name": "file.read",
                        "arguments": "{\"path\":\"/tmp/file.txt\"}"
                    }
                }]
            },
            "finish_reason": "tool_calls"
        }],
        "usage": {
            "prompt_tokens": 15,
            "completion_tokens": 10,
            "total_tokens": 25
        }
    })))
    .mount(&mock_server)
    .await;
```

---

## Ollama Mock Strategy

### Successful Chat Completion

```rust
let mock = Mock::given(method("POST"))
    .and(path("/api/chat"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "model": "llama2",
        "created_at": "2024-01-15T10:00:00Z",
        "message": {
            "role": "assistant",
            "content": "This is a test response from Ollama"
        },
        "done": true,
        "eval_count": 5,
        "prompt_eval_count": 10
    })))
    .mount(&mock_server)
    .await;
```

### Streaming Chat Completion (NDJSON)

```rust
let mock = Mock::given(method("POST"))
    .and(path("/api/chat"))
    .respond_with(ResponseTemplate::new(200).set_body(
        r#"{"model":"llama2","message":{"role":"assistant","content":"Hello"},"done":false}
{"model":"llama2","message":{"role":"assistant","content":" world"},"done":true}
"#
    ))
    .mount(&mock_server)
    .await;
```

### With Think Parameter

```rust
let mock = Mock::given(method("POST"))
    .and(path("/api/chat"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "model": "llama2",
        "created_at": "2024-01-15T10:00:00Z",
        "message": {
            "role": "assistant",
            "content": "Final response after thinking"
        },
        "done": true,
        "eval_count": 15,
        "prompt_eval_count": 20
    })))
    .mount(&mock_server)
    .await;
```

### Model List

```rust
let mock = Mock::given(method("GET"))
    .and(path("/api/tags"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "models": [
            {
                "name": "llama2:7b",
                "modified_at": "2024-01-15T10:00:00Z",
                "size": 3734928384
            },
            {
                "name": "mistral:7b",
                "modified_at": "2024-01-15T10:00:00Z",
                "size": 4089546240
            }
        ]
    })))
    .mount(&mock_server)
    .await;
```

---

## llama.cpp Mock Strategy

### Successful Chat Completion

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "llama-2-7b",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "This is a test response from llama.cpp"
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
```

### Health Check

```rust
let mock = Mock::given(method("GET"))
    .and(path("/v1/health"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "status": "ok"
    })))
    .mount(&mock_server)
    .await;
```

### Slots Endpoint

```rust
let mock = Mock::given(method("GET"))
    .and(path("/v1/slots"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "slots": [
            {
                "id": 0,
                "model": "llama-2-7b",
                "state": "busy",
                "prompt": 100,
                "prompt_tokens": 10,
                "completion": 50,
                "completion_tokens": 5
            },
            {
                "id": 1,
                "model": null,
                "state": "idle",
                "prompt": 0,
                "prompt_tokens": 0,
                "completion": 0,
                "completion_tokens": 0
            }
        ]
    })))
    .mount(&mock_server)
    .await;
```

---

## OpenAI Mock Strategy

### Successful Chat Completion

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .and(header("Authorization", "Bearer test-api-key"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "gpt-4",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "This is a test response from OpenAI"
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
```

### Rate Limiting (429)

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(429)
        .insert_header("Retry-After", "60")
        .set_body_json(serde_json::json!({
            "error": {
                "message": "Rate limit exceeded",
                "type": "rate_limit_error"
            }
        })))
    .mount(&mock_server)
    .await;
```

### Model List

```rust
let mock = Mock::given(method("GET"))
    .and(path("/v1/models"))
    .and(header("Authorization", "Bearer test-api-key"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "object": "list",
        "data": [
            {
                "id": "gpt-4",
                "object": "model",
                "created": 1687882410,
                "owned_by": "openai"
            },
            {
                "id": "gpt-3.5-turbo",
                "object": "model",
                "created": 1677610602,
                "owned_by": "openai"
            }
        ]
    })))
    .mount(&mock_server)
    .await;
```

---

## Error Scenarios

### Network Timeout

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(504)
        .set_delay(std::time::Duration::from_secs(30)))
    .mount(&mock_server)
    .await;
```

### Server Error (500)

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
        "error": {
            "message": "Internal server error",
            "type": "server_error"
        }
    })))
    .mount(&mock_server)
    .await;
```

### Invalid API Key (401)

```rust
let mock = Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
        "error": {
            "message": "Invalid API key",
            "type": "invalid_request_error"
        }
    })))
    .mount(&mock_server)
    .await;
```

---

## Test Utilities

### Helper Functions

```rust
// Setup mock server with response
async fn setup_mock_with_response(
    method: &str,
    path: &str,
    response: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method(method))
        .and(matchers::path(path))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    mock_server
}

// Create backend with mock URL
fn create_backend_with_mock_url<T>(
    mock_server: &MockServer,
    backend_factory: impl FnOnce(&str) -> T,
) -> T {
    backend_factory(mock_server.uri())
}

// Verify mock was called
async fn verify_mock_called(
    mock_server: &MockServer,
    times: usize,
) {
    // Wiremock doesn't expose call count directly
    // Tests verify behavior instead of mock calls
}
```

---

## Best Practices

1. **Always use wiremock** for backend tests
2. **Set up mocks before** creating backend instances
3. **Verify behavior**, not mock calls
4. **Test error scenarios** (429, 500, timeout)
5. **Clean up mocks** after each test
6. **Use random ports** to avoid conflicts
7. **Mock all responses**, not just happy path
8. **Test streaming** for all backends
9. **Mock rate limiting** specifically for OpenAI
10. **Keep mocks simple**, don't over-engineer

---

## Running Tests with Mocks

```bash
# Run backend tests (all use wiremock)
cargo test --test backends

# Run specific backend test
cargo test --test lmstudio

# Run with RUST_LOG to debug mocks
RUST_LOG=wiremock=debug cargo test --test backends

# Run tests sequentially for deterministic mock port usage
cargo test --test backends -- --test-threads=1
```
