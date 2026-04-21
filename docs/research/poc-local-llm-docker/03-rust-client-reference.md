# Rust Client + HTTP + SSE Research Reference

## Communication Architecture

### Core Constraint
**HTTP only.** No gRPC, no Unix sockets, no shared memory.

**Rationale:** llama.cpp server exposes HTTP endpoints only. No native Rust IPC support planned.

## HTTP Overhead Analysis

### Breakdown
- TCP connection: 1-2ms
- TLS handshake (if HTTPS): 1-3ms
- HTTP framing: <1ms
- **Total overhead per request:** ~2-6ms

### Optimization Impact
```rust
// Baseline (HTTPS, new connection): ~6ms
// Optimized (HTTP, connection pool): ~2-3ms
```

**Recommendation:** Use HTTP with connection pooling for lowest latency.

## Reqwest Client Configuration

### Version
```toml
[dependencies]
reqwest = { version = "0.13", features = ["stream"] }
tokio = { version = "1.35", features = ["full"] }
```

### Connection Pooling (Recommended)
```rust
use reqwest::Client;

let client = Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(90))
    .http2_prior_knowledge() // Disable HTTP/2 if not needed
    .build()
    .expect("Failed to build client");

// Reuse client for all requests
let response = client.get("http://localhost:8080/health")
    .send()
    .await?;
```

### Disable TLS for Local Development
```rust
let client = Client::builder()
    .danger_accept_invalid_certs(true) // Only for development
    .build()?;
```

## SSE Streaming Options

### 1. eventsource-stream (Recommended)
**Version:** 0.2.3
**Maturity:** Production-ready
**Features:** Reconnection, error handling, event filtering

```toml
[dependencies]
eventsource-stream = "0.2.3"
```

```rust
use eventsource_stream::Eventsource;

let response = client
    .post("http://localhost:8080/v1/chat/completions")
    .json(&request)
    .send()
    .await?;

let stream = response.bytes_stream().eventsource();

while let Some(event) = stream.next().await {
    let event = event?;
    println!("Event: {}", event.data);
}
```

### 2. reqwest-eventsource
**Version:** 0.5.x
**Features:** Built on reqwest, automatic retry

```toml
[dependencies]
reqwest-eventsource = "0.5"
```

```rust
use reqwest_eventsource::{Event, RequestBuilderExt};

let stream = client
    .post("http://localhost:8080/v1/chat/completions")
    .json(&request)
    .eventsource()
    .await?;

while let Some(event) = stream.next().await {
    match event {
        Ok(Event::Open) => println!("Connection opened"),
        Ok(Event::Message(message)) => println!("Message: {}", message.data),
        Err(error) => eprintln!("Error: {}", error),
    }
}
```

### 3. eventsrc (Newer)
**Version:** 0.2.x
**Features:** Minimal, async-native

```toml
[dependencies]
eventsrc = "0.2"
```

**Note:** Less mature than eventsource-stream.

## OpenAI-Compatible Streaming Format

### Request Type
```rust
#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool, // Must be true
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}
```

### Response Chunk (Serde Types)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ChatCompletionStreamResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoiceStream>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceStream {
    index: u32,
    delta: ChatCompletionStreamResponseDelta,
    finish_reason: Option<FinishReason>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionStreamResponseDelta {
    role: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}
```

### Termination Signal
```
data: [DONE]
```

**Handling:**
```rust
if event.data.trim() == "[DONE]" {
    break;
}
```

## Native /completion Streaming Format

### Response Chunk
```rust
#[derive(Debug, Deserialize)]
struct CompletionStreamResponse {
    content: String,
    tokens: Vec<u32>,
    stop: bool,
}
```

**Note:** Minimal fields only, no metadata.

## Chunk Splitting Handling

**Issue:** SSE events may split JSON across events (llama.cpp PR #9519).

**Symptoms:**
- Parse errors on chunk boundaries
- Incomplete JSON objects

**Solution: Buffer incomplete JSON**
```rust
use serde_json::Value;

let mut buffer = String::new();

while let Some(event) = stream.next().await {
    let event = event?;
    buffer.push_str(&event.data);

    // Try to parse
    if let Ok(value) = serde_json::from_str::<Value>(&buffer) {
        // Success: process value, clear buffer
        process_chunk(value);
        buffer.clear();
    } else {
        // Failure: wait for next event
        continue;
    }
}

// Process remaining buffer if not empty
if !buffer.is_empty() {
    // Final chunk
    if let Ok(value) = serde_json::from_str::<Value>(&buffer) {
        process_chunk(value);
    }
}
```

## Pre-Built OpenAI Clients

### 1. Lancor
**Version:** 0.2.0
**License:** GPL-3.0
**Features:** OpenAI-compatible with streaming

```toml
[dependencies]
lancor = "0.2.0"
```

```rust
use lancor::Lancor;

let client = Lancor::new("http://localhost:8080", "dummy-key");

let mut stream = client
    .chat()
    .model("llama-3-8b-instruct")
    .message("Hello")
    .stream()
    .await?;

while let Some(chunk) = stream.next().await {
    println!("{}", chunk.content);
}
```

**Constraint:** GPL-3.0 license (copyleft).

### 2. async-openai Types
**Version:** 0.20.x
**License:** MIT
**Features:** Production-ready response types only

```toml
[dependencies]
async-openai = { version = "0.20", features = ["types"] }
```

```rust
use async_openai::types::{
    ChatCompletionRequest, ChatCompletionMessage,
    ChatCompletionRequestMessage, CreateChatCompletionResponseArgs,
};

use reqwest::Client;

let client = Client::new();

let request = ChatCompletionRequest {
    model: "llama-3-8b-instruct".to_string(),
    messages: vec![ChatCompletionRequestMessage::User(
        ChatCompletionMessage {
            content: "Hello".to_string(),
            name: None,
        }
    )],
    stream: Some(true),
    ..Default::default()
};

let response = client
    .post("http://localhost:8080/v1/chat/completions")
    .json(&request)
    .send()
    .await?;

// Stream manually with eventsource-stream
```

## Error Handling Patterns

### 1. Timeout
```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(30),
    client.get("http://localhost:8080/health").send()
).await;

match result {
    Ok(Ok(response)) => { /* success */ },
    Ok(Err(e)) => eprintln!("Request failed: {}", e),
    Err(_) => eprintln!("Timeout after 30s"),
}
```

### 2. Connection Refused
```rust
match response.error_for_status_ref() {
    Ok(_) => { /* success */ },
    Err(e) if e.status() == Some(StatusCode::SERVICE_UNAVAILABLE) => {
        eprintln!("Server not ready");
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

### 3. Partial JSON Buffering
```rust
fn parse_chunk(data: &str) -> Result<ChatCompletionStreamResponse, Error> {
    serde_json::from_str(data).map_err(|e| {
        if e.is_eof() {
            Error::IncompleteJson(data.to_string())
        } else {
            Error::Parse(e)
        }
    })
}
```

## Performance Optimization

### 1. Disable TLS
```rust
let client = Client::builder()
    .http1_only()
    .build()?;

// Use http:// not https://
let url = "http://localhost:8080/v1/chat/completions";
```

### 2. Connection Pooling
```rust
let client = Client::builder()
    .pool_max_idle_per_host(20) // Increase pool size
    .pool_idle_timeout(Duration::from_secs(120))
    .tcp_keepalive(Duration::from_secs(60))
    .build()?;
```

### 3. Batch Size
```rust
let request = ChatCompletionRequest {
    max_tokens: Some(2048), // Increase batch size
    ..Default::default()
};
```

### 4. Reduce JSON Parsing Overhead
```rust
// Use raw string parsing when possible
if let Some(content) = extract_content_from_json(&event.data) {
    println!("{}", content);
} else {
    // Fall back to full parsing
    let chunk: ChatCompletionStreamResponse = serde_json::from_str(&event.data)?;
}
```

## Latency Benchmarks

### First-Token Latency
- Model load: 200-500ms (cached: 50-100ms)
- First token generation: 50-100ms
- **Total first-token:** 250-600ms

### Subsequent Token Latency
- Per token: 5-20ms (hardware-dependent)
- Network overhead: 2-6ms
- **Total per token:** 7-26ms

### End-to-End (100 tokens)
- OpenAI /v1/chat/completions: ~1.0-2.6s
- Native /completion: ~0.7-2.0s (no template overhead)

## Serde Type Optimization

### Use Exact Types
```rust
// BAD: Deserialize entire response
#[derive(Deserialize)]
struct FullResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoiceStream>,
    usage: Option<Usage>,
}

// GOOD: Deserialize only needed fields
#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
}
```

### Skip Unused Fields
```rust
#[derive(Deserialize)]
struct MinimalChunk {
    #[serde(default)]
    content: String,
}

let chunk: MinimalChunk = serde_json::from_str(&data)?;
```

## Async Runtime Configuration

### Tokio Multi-threaded
```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("llm-client")
    .enable_all()
    .build()?;

rt.block_on(async {
    // Run async code
});
```

### Tokio Current-thread (Single Core)
```rust
let rt = Builder::new_current_thread()
    .enable_all()
    .build()?;
```

## References

- **Reqwest Documentation:** https://docs.rs/reqwest
- **eventsource-stream:** https://docs.rs/eventsource-stream
- **llama.cpp PR #9519:** SSE chunk splitting test verification
- **Lancor:** https://docs.rs/lancor
- **async-openai:** https://docs.rs/async-openai
- **OpenAI API Reference:** https://platform.openai.com/docs/api-reference/chat
- **SSE Specification:** https://html.spec.whatwg.org/multipage/server-sent-events.html
