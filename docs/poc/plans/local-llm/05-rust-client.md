# Phase 05: Rust Client Script

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement complete Rust client for streaming completions with container lifecycle management, CLI interface, and error handling.

**Tech Stack:** Rust, reqwest, serde, serde_json, tokio, anyhow, clap, docker-compose

**Dependencies:**
- Phase 05 depends on Phase 01 (config structs)
- Phase 05 depends on Phase 03 (docker-compose)
- Phase 05 depends on Phase 04 (HTTP API)

**Important Note:** llama-cpp-2 (Rust FFI bindings) is NOT used in Phase 05. This phase implements an HTTP client that communicates with llama-server running in Docker. llama-cpp-2 is reserved for future embedded mode where llama.cpp runs in-process. Current version (April 2026) is 0.1.144 (no 0.2.x series exists).

---

## Research Foundation: Rust HTTP Client Architecture

### Core Constraint

**HTTP only.** No gRPC, no Unix sockets, no shared memory.

**Rationale:** llama.cpp server exposes HTTP endpoints only. No native Rust IPC support planned.

### HTTP Overhead Analysis

**Breakdown:**
- TCP connection: 1-2ms
- TLS handshake (if HTTPS): 1-3ms
- HTTP framing: <1ms
- **Total overhead per request:** ~2-6ms

**Optimization Impact:**
- Baseline (HTTPS, new connection): ~6ms
- Optimized (HTTP, connection pool): ~2-3ms

**Recommendation:** Use HTTP with connection pooling for lowest latency.

### Reqwest Client Configuration

**Version:** reqwest 0.13.2 with TLS features

**CRITICAL:** reqwest 0.13 switched default TLS from native-tls to rustls.

**Compatible Dependency:**
```toml
[dependencies]
reqwest = { version = "0.13.2", features = ["rustls", "stream"] }
```

**Connection Pooling (Recommended):**
```rust
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

let client = ClientBuilder::new()
    .connect_timeout(Duration::from_secs(10))
    .timeout(Duration::from_secs(300))
    .pool_idle_timeout(Duration::from_secs(60))
    .pool_max_idle_per_host(10)
    .http2_prior_knowledge()
    .build()
    .expect("Failed to build client");
```

### Connection Pool Error Handling (Issue #2956)

**Known Issue:** Connection pool corruption after network errors.

**Workaround:** Recreate client on persistent errors.

```rust
async fn send_with_retry(client: &Client, request: &Request) -> Result<Response> {
    let mut retries = 0;
    let max_retries = 3;

    loop {
        match send_request(client, request).await {
            Ok(response) => return Ok(response),
            Err(e) if retries < max_retries => {
                // Check if error indicates pool corruption
                if is_pool_corruption(&e) {
                    // Recreate client to reset connection pool
                    let new_client = ClientBuilder::new()
                        .connect_timeout(Duration::from_secs(10))
                        .timeout(Duration::from_secs(300))
                        .pool_idle_timeout(Duration::from_secs(60))
                        .pool_max_idle_per_host(10)
                        .build()?;
                    *client = new_client;
                }
                retries += 1;
                tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### SSE Streaming Options

**1. eventsource-stream (Recommended - Transport-Agnostic)**
- **Version:** 0.2.3
- **Maturity:** Production-ready
- **Features:** Reconnection, error handling, event filtering
- **Compatibility:** Works with reqwest 0.13 (transport-agnostic)

```toml
[dependencies]
eventsource-stream = "0.2.3"
```

**2. reqwest-eventsource (INCOMPATIBLE - DO NOT USE)**
- **Version:** 0.5.x
- **Issue:** Locked to reqwest 0.12 - **INCOMPATIBLE with reqwest 0.13**
- **Workaround:** Use `eventsource-stream` instead

**3. eventsrc (Newer)**
- **Version:** 0.2.x
- **Features:** Minimal, async-native
- **Note:** Less mature than eventsource-stream

### serde-saphyr 0.0.24: #[non_exhaustive] Enums

**Breaking Change:** All public enums are now `#[non_exhaustive]`.

**Impact on Error Matching:**
```rust
// Before (serde-saphyr 0.0.21):
match error {
    ParseError::InvalidYaml => { /* handle */ }
    ParseError::IoError(_) => { /* handle */ }
}

// After (serde-saphyr 0.0.24) - Wildcard required:
match error {
    ParseError::InvalidYaml => { /* handle */ }
    ParseError::IoError(_) => { /* handle */ }
    _ => { /* Required: wildcard for #[non_exhaustive] */ }
}
```

**Feature Split:**
- Use `features = ["deserialize"]` for config parsing only (reduces compile time)
- Full serde support available with default features

### Streaming Chunk Splitting (Critical Gotcha)

**Issue:** SSE events may split JSON across events (llama.cpp PR #9519).

**Symptoms:**
- Parse errors on chunk boundaries
- Incomplete JSON objects

**Solution:** Buffer incomplete JSON
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
    if let Ok(value) = serde_json::from_str::<Value>(&buffer) {
        process_chunk(value);
    }
}
```

### OpenAI-Compatible Streaming Format

**Termination signal:** `data: [DONE]`

**Handling:**
```rust
if event.data.trim() == "[DONE]" {
    break;
}
```

### Performance Benchmarks

**First-Token Latency:**
- Model load: 200-500ms (cached: 50-100ms)
- First token generation: 50-100ms
- **Total first-token:** 250-600ms

**Subsequent Token Latency:**
- Per token: 5-20ms (hardware-dependent)
- Network overhead: 2-6ms
- **Total per token:** 7-26ms

**End-to-End (100 tokens):**
- OpenAI /v1/chat/completions: ~1.0-2.6s
- Native /completion: ~0.7-2.0s (no template overhead)

### Pre-Built OpenAI Clients

**1. Lancor**
- **Version:** 0.2.0
- **License:** GPL-3.0
- **Features:** OpenAI-compatible with streaming
- **Constraint:** GPL-3.0 license (copyleft)

**2. async-openai Types**
- **Version:** 0.20.x
- **License:** MIT
- **Features:** Production-ready response types only

---

## Task 1: Define API Response Types

### File Structure

Create: `src/client/types.rs`

### Complete Type Definitions

```rust
use serde::{Deserialize, Serialize};

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,  // "system", "user", "assistant"
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

/// Chat completion response (non-streaming)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

/// Completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,  // "stop", "length"
}

/// Token usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// Chat completion chunk (streaming)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

/// Chunk choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

/// Delta (incremental content)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub slots_idle: usize,
    pub slots_processing: usize,
}

/// Server properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerProps {
    pub model_filename: String,
    pub n_ctx: usize,
    pub n_embd: usize,
    pub n_layer: usize,
    pub n_gpu_layers: usize,
    pub use_mmap: bool,
}

/// Server slot status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotStatus {
    pub id: usize,
    pub state: String,
    pub n_processed: usize,
    pub n_tokens: usize,
}
```

---

## Task 2: Implement HTTP Client

### File Structure

Create: `src/client/http_client.rs`

### Complete HTTP Client Implementation

```rust
use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::{Stream, StreamExt};

use crate::client::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse,
    HealthResponse, ServerProps, SlotStatus,
};

/// HTTP client for llama-server
pub struct LlamaHttpClient {
    client: Client,
    base_url: String,
}

impl LlamaHttpClient {
    /// Create new HTTP client
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();

        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(300))
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .http2_prior_knowledge()
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, base_url })
    }

    /// Get full URL for endpoint
    fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint)
    }

    /// Health check
    pub async fn health(&self) -> Result<HealthResponse> {
        let response = self
            .client
            .get(self.url("health"))
            .send()
            .await
            .context("Failed to send health check")?;

        response
            .json()
            .await
            .context("Failed to parse health check response")
    }

    /// Get server properties
    pub async fn props(&self) -> Result<ServerProps> {
        let response = self
            .client
            .get(self.url("props"))
            .send()
            .await
            .context("Failed to send props request")?;

        response
            .json()
            .await
            .context("Failed to parse props response")
    }

    /// Get slot status
    pub async fn slots(&self) -> Result<Vec<SlotStatus>> {
        let response = self
            .client
            .get(self.url("slots"))
            .send()
            .await
            .context("Failed to send slots request")?;

        response
            .json()
            .await
            .context("Failed to parse slots response")
    }

    /// Wait for server to be healthy
    pub async fn wait_for_healthy(&self, max_wait: Duration) -> Result<()> {
        println!("Waiting for server to be healthy...");

        let start = std::time::Instant::now();

        while start.elapsed() < max_wait {
            match self.health().await {
                Ok(response) if response.status == "ok" => {
                    println!("Server is healthy");
                    return Ok(());
                }
                Ok(_) | Err(_) => {
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }

        anyhow::bail!("Server did not become healthy within {:?}", max_wait);
    }

    /// Error types for common failure modes
    #[derive(Debug, thiserror::Error)]
    pub enum LlmClientError {
        #[error("Server not ready: {0}")]
        ServerNotReady(String),

        #[error("Model load failed: {0}")]
        ModelLoadFailed(String),

        #[error("Container not started: {0}")]
        ContainerNotStarted(String),

        #[error("Health check failed: {0}")]
        HealthCheckFailed(String),

        #[error("Request timeout after {0}s")]
        RequestTimeout(u64),

        #[error("HTTP error: {0}")]
        HttpError(reqwest::Error),

        #[error("Failed to parse response: {0}")]
        ParseError(String),
    }

    /// Non-streaming chat completion
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let response = self
            .client
            .post(self.url("v1/chat/completions"))
            .json(&request)
            .send()
            .await
            .context("Failed to send chat completion request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error".to_string());
            anyhow::bail!("Chat completion failed: {} - {}", status, error_text);
        }

        response
            .json()
            .await
            .context("Failed to parse chat completion response")
    }

    /// Streaming chat completion
    pub async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<impl Stream<Item = Result<ChatCompletionChunk>>> {
        let mut stream_request = request.clone();
        stream_request.stream = true;

        let response = self
            .client
            .post(self.url("v1/chat/completions"))
            .json(&stream_request)
            .send()
            .await
            .context("Failed to send chat completion request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error".to_string());
            anyhow::bail!("Chat completion failed: {} - {}", status, error_text);
        }

        Ok(sse_stream(response.bytes_stream()))
    }
}

/// Parse SSE stream with chunk splitting handling (PR #9519)
fn sse_stream(
    byte_stream: reqwest::ByteStream,
) -> impl Stream<Item = Result<ChatCompletionChunk>> {
    use bytes::Bytes;

    async_stream::stream! {
        let mut buffer = String::new();

        use futures::StreamExt;
        let mut bytes = byte_stream;

        while let Some(chunk_result) = bytes.next().await {
            let chunk = chunk_result.context("Failed to read stream chunk")?;
            buffer.push_str(std::str::from_utf8(&chunk).context("Invalid UTF-8 in stream")?);

            // Process complete lines
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].to_string();
                buffer = buffer[pos + 1..].to_string();

                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                // Skip comments
                if line.starts_with(':') {
                    continue;
                }

                // Parse data line
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        return;
                    }

                    // Try to parse JSON
                    match serde_json::from_str::<ChatCompletionChunk>(data) {
                        Ok(chunk) => yield Ok(chunk),
                        Err(e) => yield Err(anyhow::anyhow!("Failed to parse SSE chunk: {}", e)),
                    }
                }
            }
        }
    }
}
```

---

## Task 3: Implement Docker Container Management

### File Structure

Create: `src/client/docker_manager.rs`

### Complete Docker Manager Implementation

```rust
use anyhow::{Context, Result};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

/// Docker container manager
pub struct DockerManager {
    project_name: String,
    compose_file: Option<String>,
}

impl DockerManager {
    /// Create new Docker manager
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            compose_file: None,
        }
    }

    /// Set custom docker-compose file
    pub fn with_compose_file(mut self, compose_file: impl Into<String>) -> Self {
        self.compose_file = Some(compose_file.into());
        self
    }

    /// Build docker-compose command
    fn compose_cmd(&self) -> Command {
        let mut cmd = Command::new("docker");
        cmd.args(["compose"]);

        if let Some(ref compose_file) = self.compose_file {
            cmd.args(["-f", compose_file]);
        }

        cmd.arg("-p").arg(&self.project_name);
        cmd
    }

    /// Start container
    pub async fn start(&self) -> Result<()> {
        println!("Starting Docker container...");

        let output = self
            .compose_cmd()
            .args(["up", "-d"])
            .output()
            .context("Failed to start container")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to start container: {}", stderr);
        }

        println!("Container started");

        Ok(())
    }

    /// Stop container
    pub async fn stop(&self) -> Result<()> {
        println!("Stopping Docker container...");

        let output = self
            .compose_cmd()
            .args(["down"])
            .output()
            .context("Failed to stop container")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to stop container: {}", stderr);
        }

        println!("Container stopped");

        Ok(())
    }

    /// Restart container
    pub async fn restart(&self) -> Result<()> {
        println!("Restarting Docker container...");

        let output = self
            .compose_cmd()
            .args(["restart"])
            .output()
            .context("Failed to restart container")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to restart container: {}", stderr);
        }

        println!("Container restarted");

        Ok(())
    }

    /// Get container status
    pub async fn status(&self) -> Result<String> {
        let output = self
            .compose_cmd()
            .args(["ps", "--format", "{{.State}}"])
            .output()
            .context("Failed to get container status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to get container status: {}", stderr);
        }

        let status = String::from_utf8_lossy(output.stdout).trim().to_string();

        Ok(status)
    }

    /// Wait for container to be healthy
    pub async fn wait_for_healthy(&self, max_wait: Duration) -> Result<()> {
        println!("Waiting for container to be healthy...");

        let start = std::time::Instant::now();

        while start.elapsed() < max_wait {
            let status = self.status().await?;

            if status.contains("healthy") {
                println!("Container is healthy");
                return Ok(());
            }

            sleep(Duration::from_secs(2)).await;
        }

        anyhow::bail!("Container did not become healthy within {:?}", max_wait);
    }

    /// Get container logs
    pub async fn logs(&self, follow: bool) -> Result<()> {
        let mut cmd = self.compose_cmd();
        cmd.args(["logs"]);

        if follow {
            cmd.arg("-f");
        }

        let output = cmd
            .output()
            .context("Failed to get container logs")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to get container logs: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(output.stdout);
        println!("{}", stdout);

        Ok(())
    }
}
```

---

## Task 4: Implement PoC Client Binary

### File Structure

Create: `src/bin/poc_client.rs`

### Complete PoC Client Implementation

```rust
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

use crate::client::docker_manager::DockerManager;
use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};

/// PoC LLM client
#[derive(Parser, Debug)]
#[command(name = "poc_client")]
#[command(about = "PoC client for local LLM with Docker", long_about = None)]
struct Cli {
    /// Server URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    url: String,

    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Prompt text
    #[arg(short, long)]
    prompt: String,

    /// Stream output
    #[arg(short, long)]
    stream: bool,

    /// Max tokens to generate
    #[arg(short, long, default_value = "512")]
    max_tokens: usize,

    /// Temperature
    #[arg(short = 't', long, default_value = "0.7")]
    temperature: f32,

    /// Top-P
    #[arg(short = 'p', long, default_value = "0.95")]
    top_p: f32,

    /// Start container
    #[arg(long)]
    start: bool,

    /// Stop container after completion
    #[arg(long)]
    stop: bool,

    /// Wait for container to be healthy (seconds)
    #[arg(long, default_value = "60")]
    wait: u64,

    /// Docker compose file
    #[arg(long)]
    compose_file: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create HTTP client
    let client = LlamaHttpClient::new(&cli.url)?;

    // Start container if requested
    if cli.start {
        start_container(&cli).await?;
    }

    // Wait for container to be healthy
    if cli.start {
        println!("Waiting for server to be ready...");
        client
            .wait_for_healthy(Duration::from_secs(cli.wait))
            .await
            .context("Server did not become healthy")?;
    }

    // Check server health
    match client.health().await {
        Ok(health) => {
            println!("Server health: {}", health.status);
            println!("Slots idle: {}", health.slots_idle);
            println!("Slots processing: {}", health.slots_processing);
        }
        Err(e) => {
            anyhow::bail!("Health check failed: {}", e);
        }
    }

    // Get server properties
    if cli.verbose {
        match client.props().await {
            Ok(props) => {
                println!("\nServer properties:");
                println!("  Model: {}", props.model_filename);
                println!("  Context: {}", props.n_ctx);
                println!("  GPU layers: {}", props.n_gpu_layers);
            }
            Err(e) => {
                eprintln!("Failed to get props: {}", e);
            }
        }
    }

    // Prepare request
    let request = ChatCompletionRequest {
        model: "model".to_string(),
        messages: vec![ChatMessage::user(&cli.prompt)],
        max_tokens: Some(cli.max_tokens),
        temperature: Some(cli.temperature),
        top_p: Some(cli.top_p),
        stream: cli.stream,
        ..Default::default()
    };

    // Send request
    if cli.stream {
        println!("\nResponse (streaming):");
        println!("------------------------");

        let mut stream = client
            .chat_completion_stream(request)
            .await
            .context("Failed to start stream")?;

        let mut full_response = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    for choice in chunk.choices {
                        if let Some(content) = choice.delta.content {
                            print!("{}", content);
                            std::io::Write::flush(&mut std::io::stdout())?;
                            full_response.push_str(&content);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\nStream error: {}", e);
                    break;
                }
            }
        }

        println!("\n------------------------");
        println!("Total tokens: {}", full_response.split_whitespace().count());
    } else {
        println!("\nSending request...");

        let response = client
            .chat_completion(request)
            .await
            .context("Failed to get response")?;

        println!("Response:");
        println!("------------------------");

        for choice in response.choices {
            println!("{}", choice.message.content);
        }

        println!("------------------------");
        println!("Usage:");
        println!("  Prompt tokens: {}", response.usage.prompt_tokens);
        println!(
            "  Completion tokens: {}",
            response.usage.completion_tokens
        );
        println!("  Total tokens: {}", response.usage.total_tokens);
    }

    // Stop container if requested
    if cli.stop {
        stop_container(&cli).await?;
    }

    Ok(())
}

/// Start Docker container
async fn start_container(cli: &Cli) -> Result<()> {
    let mut docker = DockerManager::new("whitt");

    if let Some(ref compose_file) = cli.compose_file {
        docker = docker.with_compose_file(compose_file.to_string_lossy().as_ref());
    }

    docker.start().await?;

    Ok(())
}

/// Stop Docker container
async fn stop_container(cli: &Cli) -> Result<()> {
    let mut docker = DockerManager::new("whitt");

    if let Some(ref compose_file) = cli.compose_file {
        docker = docker.with_compose_file(compose_file.to_string_lossy().as_ref());
    }

    docker.stop().await?;

    Ok(())
}

// Implement Default for ChatCompletionRequest
impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            model: "model".to_string(),
            messages: Vec::new(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            top_k: None,
            repeat_penalty: None,
            presence_penalty: None,
            frequency_penalty: None,
            stream: false,
            stop: None,
            seed: None,
        }
    }
}
```

---

## Task 5: Create Module Structure

### mod.rs Updates

Create: `src/client/mod.rs`

```rust
pub mod docker_manager;
pub mod http_client;
pub mod types;

pub use docker_manager::DockerManager;
pub use http_client::LlamaHttpClient;
pub use types::*;
```

Update: `src/lib.rs`

```rust
pub mod client;
pub mod cli;
pub mod config;
pub mod error;

pub use client::*;
pub use config::*;
pub use error::*;
```

---

## Task 6: Add Dependencies to Cargo.toml

Update: `Cargo.toml`

```toml
[package]
name = "whitt-execution-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
# Existing dependencies
llama-cpp-2 = { version = "0.1.144", features = ["vulkan"] }
serde = { version = "1.0", features = ["derive"] }
serde-saphyr = "0.0.24"
serde_json = "1.0"
clap = { version = "4.6", features = ["derive"] }
tokio = { version = "1.50", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
garde = { version = "0.20", features = ["derive"] }

# New dependencies for PoC client
# CRITICAL: reqwest 0.13 defaults to rustls TLS
# CRITICAL: eventsource-stream is transport-agnostic (works with reqwest 0.13)
# CRITICAL: DO NOT use reqwest-eventsource (locked to reqwest 0.12)
reqwest = { version = "0.13.2", features = ["rustls", "json", "stream"] }
eventsource-stream = "0.2.3"
tokio-stream = "0.1"
async-stream = "0.3"
futures = "0.3"
dirs = "5.0"

[dev-dependencies]
tokio-test = "0.4"

[[bin]]
name = "poc_client"
path = "src/bin/poc_client.rs

[[bin]]
name = "whitt"
path = "src/main.rs"
```

### Dependency Upgrade Notes

**Why These Versions Are Required:**

**tokio ^1.50.0:**
- **Requirement:** autoagents crate (used in production) requires tokio 1.50+
- **Current:** Project has 1.35
- **Upgrade:** Must upgrade to ^1.50.0 for autoagents compatibility
- **Impact:** Provides new async features and performance improvements

**clap ^4.6:**
- **Requirement:** autoagents crate requires clap 4.6+
- **Current:** Project has 4.5
- **Upgrade:** Must upgrade to ^4.6 for autoagents compatibility
- **Impact:** New CLI features and improved derive macros

**serde-saphyr 0.0.24:**
- **Status:** Latest version (pre-1.0, actively maintained)
- **Features:** Native merge keys, garde support, safe Rust
- **Note:** No 0.1 series exists (versioning skipped)
- **CRITICAL BREAKING:** All public enums are now `#[non_exhaustive]`
  - Requires wildcard `_ => {}` in all match expressions
  - Feature split: can use `features = ["deserialize"]` only for config parsing
  - See "serde-saphyr 0.0.24: #[non_exhaustive] Enums" section above for examples

**llama-cpp-2 0.1.144:**
- **Latest:** April 2026 release
- **Note:** NO 0.2.x series exists (despite common misconception)
- **Usage:** Reserved for future embedded mode (in-process llama.cpp)

**reqwest 0.13.2:**
- **Latest:** Current stable release
- **Features:** JSON parsing, streaming support, HTTP/2
- **CRITICAL:** Defaults to rustls TLS (not native-tls)
- **CRITICAL:** INCOMPATIBLE with reqwest-eventsource (which requires reqwest 0.12)

**eventsource-stream 0.2.3:**
- **Status:** Current version (no conflicts)
- **Features:** Reconnection, error handling, event filtering
- **Compatibility:** Transport-agnostic, works with reqwest 0.13 rustls

**Connection Pool Bug (reqwest issue #2956):**
- **Issue:** Connection pool corruption after network errors
- **Workaround:** Recreate HTTP client on persistent errors
- **Implementation:** See "Connection Pool Error Handling" section above

**Dependency Conflict Resolution:**
- All upgraded versions are compatible
- No breaking changes between current versions
- Upgrade required for autoagents integration (future phase)
```

---

## Task 7: Create Integration Test

### File Structure

Create: `tests/integration_test.rs`

### Complete Integration Test

```rust
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[tokio::test]
#[ignore]  // Run with: cargo test --test integration_test -- --ignored
async fn test_server_health() {
    let client = Client::new();
    let url = "http://localhost:8080/health";

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let health: HealthResponse = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(health.status, "ok");
}

#[tokio::test]
#[ignore]
async fn test_chat_completion() {
    let client = Client::new();
    let url = "http://localhost:8080/v1/chat/completions";

    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "Say hello"}
        ],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": false
    });

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(json["choices"][0]["message"]["role"], "assistant");
    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].is_number());
}

#[tokio::test]
#[ignore]
async fn test_streaming_completion() {
    let client = Client::new();
    let url = "http://localhost:8080/v1/chat/completions";

    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "Count from 1 to 5"}
        ],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": true
    });

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(response.headers().get("content-type").unwrap(), "text/event-stream");

    let mut stream = response.bytes_stream();
    let mut chunks_received = 0;
    let mut buffer = String::new();

    use futures::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("Failed to read stream chunk");
        buffer.push_str(std::str::from_utf8(&chunk).expect("Invalid UTF-8"));

        // Parse SSE events
        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].to_string();
            buffer = buffer[pos + 1..].to_string();

            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                chunks_received += 1;
            }
        }
    }

    assert!(chunks_received > 0, "No chunks received");
}

#[tokio::test]
#[ignore]
async fn test_end_to_end() {
    // This test assumes server is running
    let url = "http://localhost:8080";

    // 1. Health check
    let client = Client::new();
    let health: HealthResponse = client
        .get(&format!("{}/health", url))
        .send()
        .await
        .expect("Health check failed")
        .json()
        .await
        .expect("Failed to parse health");

    assert_eq!(health.status, "ok");

    // 2. Send request
    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "What is 2 + 2?"}
        ],
        "max_tokens": 20,
        "stream": false
    });

    let response = client
        .post(&format!("{}/v1/chat/completions", url))
        .json(&request)
        .send()
        .await
        .expect("Request failed");

    assert!(response.status().is_success());

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].as_u64().unwrap() > 0);

    // 3. Verify response contains "4"
    let content = json["choices"][0]["message"]["content"].as_str().unwrap();
    assert!(content.contains("4") || content.contains("four"));
}
```

---

## Verification Steps

### Step 1: Build Project

**Test:** Compile PoC client

```bash
# Build all targets
cargo build --release

# Expected: No errors

# Build specific binary
cargo build --bin poc_client --release

# Expected: Binary created at target/release/poc_client
```

### Step 2: Start Server

**Test:** Start LLM server with docker compose

```bash
# Start server
./scripts/start.sh

# Wait for healthy
docker compose ps

# Expected: "healthy" status

# Check health endpoint
curl http://localhost:8080/props

# Expected: {"status":"ok","slots_idle":8,"slots_processing":0}
```

### Step 3: Run Non-Streaming Client

**Test:** Send simple request

```bash
# Run client (non-streaming)
cargo run --bin poc_client -- \
  --prompt "Hello, how are you?" \
  --max-tokens 50 \
  --verbose

# Expected output:
# Server health: ok
# Slots idle: 8
# Slots processing: 0
#
# Server properties:
#   Model: /models/...
#   Context: 4096
#   GPU layers: 999
#
# Sending request...
# Response:
# ------------------------
# I am doing well, thank you for asking! How can I help you today?
# ------------------------
# Usage:
#   Prompt tokens: 8
#   Completion tokens: 15
#   Total tokens: 23
```

### Step 4: Run Streaming Client

**Test:** Stream tokens in real-time

```bash
# Run client (streaming)
cargo run --bin poc_client -- \
  --prompt "Write a haiku about Rust" \
  --max-tokens 50 \
  --stream

# Expected output:
# Server health: ok
# Slots idle: 8
# Slots processing: 0
#
# Response (streaming):
# ------------------------
# Safe, fast, bold,
# Memory lives without fear,
# Rust builds to future.
# ------------------------
# Total tokens: 15
```

### Step 5: Test Auto-Start

**Test:** Start container automatically

```bash
# Stop container first
./scripts/stop.sh

# Run client with auto-start
cargo run --bin poc_client -- \
  --prompt "Test auto-start" \
  --max-tokens 20 \
  --start \
  --stop

# Expected:
# Starting Docker container...
# Container started
# Waiting for server to be ready...
# Server health: ok
# ...
# (response)
# Stopping Docker container...
# Container stopped
```

### Step 6: Test Error Handling

**Test:** Handle connection errors

```bash
# Stop container
./scripts/stop.sh

# Try to run client (server not running)
cargo run --bin poc_client -- \
  --prompt "Test" \
  --max-tokens 10

# Expected: Error message about connection refused or health check failed
```

### Step 7: Run Integration Tests

**Test:** Run integration test suite

```bash
# Ensure server is running
./scripts/start.sh
sleep 30

# Run integration tests
cargo test --test integration_test -- --ignored

# Expected: All tests pass
# test server_health ... ok
# test chat_completion ... ok
# test streaming_completion ... ok
# test_end_to_end ... ok
```

### Step 8: Test with Different Models

**Test:** Switch models and test

```bash
# Switch to 8B model
./scripts/switch-model.sh configs/models/llama-3.1-8b.yml

# Wait for healthy
sleep 30

# Run client
cargo run --bin poc_client -- \
  --prompt "Explain quantum computing" \
  --max-tokens 100 \
  --stream

# Expected: Detailed response from 8B model
```

---

## Integration with Other Phases

### Phase 01: YAML Config Schema

**Output:** Config structs and validation
**Input:** Phase 05 uses config structs for CLI validation

### Phase 02: Docker Container

**Output:** Docker image with llama-server
**Input:** Phase 05 starts container via docker-compose

### Phase 03: Config Injection

**Output:** Docker compose files
**Input:** Phase 05 uses docker-compose to manage container

### Phase 04: HTTP Interface

**Output:** API documentation and types
**Input:** Phase 05 implements HTTP client using API specs

---

## File Locations

Create/Modify:
- `src/client/types.rs` - API response types
- `src/client/http_client.rs` - HTTP client implementation
- `src/client/docker_manager.rs` - Docker container manager
- `src/client/mod.rs` - Module exports
- `src/bin/poc_client.rs` - PoC client binary
- `src/lib.rs` - Library exports
- `Cargo.toml` - Add dependencies
- `tests/integration_test.rs` - Integration tests

---

## Success Criteria

- [ ] Project compiles without errors
- [ ] poc_client binary runs successfully
- [ ] Non-streaming completions work
- [ ] Streaming completions work with real-time token display
- [ ] Container auto-start works
- [ ] Container auto-stop works
- [ ] Error handling works for connection refused
- [ ] Integration tests pass
- [ ] Works with different models
- [ ] CLI interface accepts all flags

---

## Next Steps

After completing Phase 05:
1. All phases complete - PoC finished
2. Run end-to-end test: start container, infer, verify, shutdown
3. Document performance metrics
4. Consider next steps: production hardening, multi-model support, RAG
