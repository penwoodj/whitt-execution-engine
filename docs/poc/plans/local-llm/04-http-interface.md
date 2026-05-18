# Phase 04: Fast HTTP Interface (llama-server in Container)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Optimize llama-server configuration for maximum throughput, document HTTP API usage, and provide performance tuning guidelines for Rust client.

**Tech Stack:** llama.cpp HTTP API, curl, reqwest (Rust), Prometheus metrics

**Dependencies:**
- Phase 04 depends on Phase 02 (container)
- Phase 04 runs in parallel with Phase 03
- Phase 04 output used by Phase 05 (Rust client)

---

## Research Foundation: HTTP Performance and Interface

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

### OpenAI vs Native Endpoint Latency

**/v1/chat/completions:**
- Adds chat template processing overhead: 10-30ms
- Standard OpenAI-compatible format
- Better for multi-turn conversations

**/completion:**
- Raw completion with minimal overhead: 2-5ms
- Native llama.cpp format
- Better for single-turn generation, fine-grained timing

### Performance Characteristics

**Latency Breakdown:**
- Model load: 200-1000ms (model-dependent)
- First token: 50-100ms after model load
- Subsequent tokens: 5-20ms per token (hardware-dependent)

**End-to-End (100 tokens):**
- OpenAI /v1/chat/completions: ~1.0-2.6s
- Native /completion: ~0.7-2.0s (no template overhead)

---

## Task 1: Optimize llama-server for Throughput

### Server Configuration Flags

**Key Flags for Performance:**

| Flag | Purpose | Recommended Value | Performance Impact |
|------|---------|-------------------|-------------------|
| `--parallel` | Enable continuous batching | `true` | High (+50-100% throughput) |
| `--n-slot` | Max concurrent requests | `8-16` | Medium (more slots = higher concurrency) |
| `--cache-type-k` | KV cache type (key) | `f16` | High (f16 = 2x faster than f32) |
| `--cache-type-v` | KV cache type (value) | `f16` | High (f16 = 2x faster than f32) |
| `--n-gpu-layers` | GPU layers offload | `999` (all) | Critical (CPU-only = 10x slower) |
| `--ctx-size` | Context window | `4096-8192` | Medium (larger = more memory) |
| `--batch-size` | Batch size (tokens) | `512` | Medium (larger = better batching) |
| `--ubatch-size` | Micro-batch size | `512` | Medium (affects batching) |
| `--timeout` | Request timeout (s) | `300` | Low (timeout only) |

### Optimal Config Example

```yaml
# config.yml - Performance-optimized settings
model:
  path: /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf
context:
  size: 4096
  batch_size: 512
  ubatch_size: 512
hardware:
  threads: 8
  gpu_layers: 999
  mmap_size: 8
  use_mmap: true
sampling:
  temperature: 0.7
  top_p: 0.95
  top_k: 40
  repeat_penalty: 1.1
  max_tokens: 512
server:
  host: 0.0.0.0
  port: 8080
  parallel: true        # CRITICAL: Enable continuous batching
  timeout: 300
  max_slots: 16         # Increase for higher concurrency
  metrics: true
  slots_endpoint: true
cache:
  cache_type_k: f16     # CRITICAL: Use f16 for speed
  cache_type_v: f16     # CRITICAL: Use f16 for speed
  kv_cache_size: 4
features:
  log_level: warn       # Reduce log overhead
  verbose: false
vulkan:
  visible_devices: "0"
  disable_debug: true
```

### KV Cache Optimization

**KV Cache Types:**
- `f32`: Full precision (32-bit) - Slowest, most accurate
- `f16`: Half precision (16-bit) - Recommended, 2x faster than f32
- `q8_0`: 8-bit quantized - Faster, minor quality loss
- `q4_0`: 4-bit quantized - Fastest, noticeable quality loss

**Recommendation:** Use `f16` for best balance of speed and quality.

**Cache Size Calculation:**
```
KV cache size (GB) = 2 * n_ctx * n_embd * n_layer * sizeof(type) / 1e9

Example (Llama 3.2 1B):
- n_ctx = 4096
- n_embd = 2048
- n_layer = 16
- sizeof(f16) = 2 bytes

KV cache size = 2 * 4096 * 2048 * 16 * 2 / 1e9 ≈ 0.5 GB per slot
16 slots ≈ 8 GB total

Set kv_cache_size = 4-8 GB for 8-16 concurrent requests
```

### Continuous Batching (CB)

**How CB Works:**
- Multiple requests share same KV cache
- New requests join ongoing generation
- Batches change dynamically as requests complete
- Reduces redundant computation

**Enabling CB:**
```yaml
server:
  parallel: true
  max_slots: 16
```

**CB Performance Gain:**
- 10-50% speedup for concurrent requests
- Higher utilization of GPU
- Better throughput for chat applications

**AMD Polaris GPU Tuning:**
- **Flash Attention:** MUST be disabled (`-fa 0`) due to bug (llama.cpp issue #20465)
  - Enabling causes garbled output on Polaris GPUs
- **Conservative Batch Sizes:** Use smaller batch sizes on Polaris
  - Recommended: `batch_size: 512`, `ubatch_size: 512`
  - Larger batches may cause instability
- **GPU Layers:** Use `n_gpu_layers=99` (near-max) for Polaris
- **Driver Requirement:** MUST use RADV (NOT AMDVLK)
  - AMDVLK has 2GB allocation limit (llama.cpp issue #15054)

**Polaris-Specific Config:**
```yaml
server:
  parallel: true
  max_slots: 8          # Moderate concurrency

context:
  batch_size: 512       # Conservative for Polaris
  ubatch_size: 512

cache:
  cache_type_k: f16
  cache_type_v: f16

vulkan:
  flash_attention: false  # CRITICAL: Disable for Polaris
```
```

**Authentication Request:**
```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-secret-api-key" \
  -d '{
    "model": "model",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 100
  }'
```

**Security Best Practices:**
- Use strong, randomly generated API keys (minimum 32 characters)
- Pass API key via environment variable (not build arguments)
- Rotate API keys regularly
- Never log API keys
- Use HTTPS in production (adds TLS handshake overhead of 1-3ms)

### CORS Configuration

**Production Warning:** `cors_origins: "*"` is insecure for production deployments.

**Secure CORS Configuration:**
```yaml
# config.yml
server:
  cors_origins: "http://localhost:8080,https://yourdomain.com"  # Specify allowed origins
```

**Environment Variable:**
```bash
LLAMA_ARG_CORS_ORIGINS="http://localhost:8080,https://yourdomain.com"
```

**Command Line:**
```bash
llama-server --cors-origins "http://localhost:8080,https://yourdomain.com"
```

**Why NOT use "*" in production:**
- Allows any origin to make requests
- Vulnerable to CSRF attacks
- Bypasses same-origin policy protections

### Health Endpoint

**Endpoint:** `GET /health`

**Purpose:** Health check for container orchestration

**Request:**
```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "status": "ok",
  "slots_idle": 8,
  "slots_processing": 0
}
```

**Use Cases:**
- Docker healthcheck
- Kubernetes liveness probe
- Service discovery

**CRITICAL GOTCHA (Issue #20684):** `/health` endpoint gets queued with other requests under high load.
**Workaround:** Use `/props` endpoint instead for health checks under load.

### Properties Endpoint

**Endpoint:** `GET /props`

**Purpose:** Get model and server properties

**Request:**
```bash
curl http://localhost:8080/props
```

**Response:**
```json
{
  "model_alias": "model",
  "model_filename": "/models/Llama-3.2-1B-Instruct.Q4_K_M.gguf",
  "model_type": "llama",
  "model_f16": true,
  "model_n_vocab": 128256,
  "model_n_ctx_train": 131072,
  "n_ctx": 4096,
  "n_embd": 2048,
  "n_layer": 16,
  "n_head": 32,
  "n_head_kv": 8,
  "n_rot": 64,
  "n_gqa": 4,
  "n_gqa_group_size": 4,
  "rope_freq_base": 500000,
  "rope_freq_scale":1,
  "f16_kv": true,
  "use_sycl": false,
  "n_gpu_layers": 999,
  "use_mmap": true,
  "use_mlock": false,
  "gpu_split": "auto"
}
```

**Use Cases:**
- Validate model loaded correctly
- Check context size and KV cache settings
- Verify GPU layer offload

### Slots Endpoint

**Endpoint:** `GET /slots`

**Purpose:** Get status of processing slots

**Request:**
```bash
curl http://localhost:8080/slots
```

**Response:**
```json
{
  "id": 0,
  "task": "infill",
  "n_processed": 0,
  "n_tokens": 0,
  "n_predict": 512,
  "t_start_processing": 0,
  "t_start_generation": 0,
  "t_token_times": [],
  "t_prompt_processing": 0,
  "t_prompt_eval_per_token_ms": 0,
  "t_sample_ms": 0,
  "state": "idle",
  "id_slot": 0,
  "n_past_tokens": 0
}
```

**Use Cases:**
- Monitor active requests
- Debug stuck slots
- Analyze performance metrics

### Metrics Endpoint

**Endpoint:** `GET /metrics`

**Purpose:** Prometheus-compatible metrics

**Request:**
```bash
curl http://localhost:8080/metrics
```

**Response:**
```
# HELP llama_slots_processing Number of slots processing requests
# TYPE llama_slots_processing gauge
llama_slots_processing 0

# HELP llama_slots_idle Number of idle slots
# TYPE llama_slots_idle gauge
llama_slots_idle 8

# HELP llama_n_tokens_processed_total Total tokens processed
# TYPE llama_n_tokens_processed_total counter
llama_n_tokens_processed_total 123456

# HELP llama_prompt_processing_seconds_total Total prompt processing time
# TYPE llama_prompt_processing_seconds_total counter
llama_prompt_processing_seconds_total 123.456

# HELP llama_token_generation_seconds_total Total token generation time
# TYPE llama_token_generation_seconds_total counter
llama_token_generation_seconds_total 456.789

# HELP llama_n_tokens_max Maximum tokens processed (renamed from n_past_max)
# TYPE llama_n_tokens_max gauge
llama_n_tokens_max 8192
```

**Metric Field Rename (llama.cpp PR #16818):**
- Old: `llama_n_past_max` (deprecated)
- New: `llama_n_tokens_max` (use this in dashboards)
- **Action:** Update Prometheus queries and Grafana panels

**Use Cases:**
- Prometheus monitoring
- Grafana dashboards
- Alerting

---

## Task 3: OpenAI-Compatible API

### Chat Completions (Non-Streaming)

**Endpoint:** `POST /v1/chat/completions`

**Purpose:** Generate chat completions

**Request:**
```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model",
    "messages": [
      {"role": "user", "content": "Hello, how are you?"}
    ],
    "max_tokens": 100,
    "temperature": 0.7,
    "top_p": 0.95,
    "stream": false
  }'
```

**Response:**
```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1699000000,
  "model": "model",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "I am doing well, thank you for asking!"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 8,
    "completion_tokens": 12,
    "total_tokens": 20
  }
}
```

### Chat Completions (Streaming)

**Endpoint:** `POST /v1/chat/completions`

**Stream Parameter:** Set `"stream": true`

**Request:**
```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model",
    "messages": [
      {"role": "user", "content": "Hello, how are you?"}
    ],
    "max_tokens": 100,
    "temperature": 0.7,
    "stream": true
  }'
```

**Response (SSE):**
```
data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{"role":"assistant"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{"content":"I"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{"content":" am"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{"content":" doing"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{"content":" well"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1699000000,"model":"model","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

**SSE Parsing:**
- Each line starts with `data: `
- Lines are JSON objects
- Last line is `data: [DONE]`
- Parse incrementally for real-time display

### Chat Completions Request Schema

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub top_k: Option<usize>,
    #[serde(default)]
    pub repeat_penalty: Option<f32>,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    pub frequency_penalty: Option<f32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub seed: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,  // "system", "user", "assistant"
    pub content: String,
}
```

### Chat Completions Response Schema

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,  // "stop", "length"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
```

### Chat Completions Chunk Schema (Streaming)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
```

---

## Task 4: Native API (Legacy)

### Completions Endpoint

**Endpoint:** `POST /completion`

**Purpose:** Legacy completions API (non-chat)

**Request:**
```bash
curl -X POST http://localhost:8080/completion \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "The quick brown fox",
    "n_predict": 50,
    "temperature": 0.7,
    "top_p": 0.95,
    "stream": false
  }'
```

**Response:**
```json
{
  "content": " jumps over the lazy dog.",
  "generation_settings": {
    "n_predict": 50,
    "temperature": 0.7,
    "top_p": 0.95
  },
  "model": "model",
  "timings": {
    "prompt_n": 5,
    "prompt_ms": 12.345,
    "prompt_per_token_ms": 2.469,
    "predicted_n": 6,
    "predicted_ms": 45.678,
    "predicted_per_token_ms": 7.613
  }
}
```

**When to Use:**
- Non-chat generation (code, text completion)
- Fine-grained timing information
- Legacy compatibility

**When NOT to Use:**
- Chat applications (use `/v1/chat/completions`)
- Multi-turn conversations

---

## Task 5: Connection Pooling for Rust Client

### Reqwest Connection Pool

**Configuration:**
```rust
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

let client = ClientBuilder::new()
    // Connection timeout
    .connect_timeout(Duration::from_secs(10))
    // Read timeout
    .timeout(Duration::from_secs(300))
    // Keep-alive timeout
    .pool_idle_timeout(Duration::from_secs(60))
    // Max connections per host
    .pool_max_idle_per_host(10)
    // Build client
    .build()?;
```

**Benefits:**
- Reuse TCP connections
- Reduce connection overhead (from ~6ms to <1ms)
- Better throughput for multiple requests

### Keep-Alive Headers

**Request:**
```rust
let response = client
    .post("http://localhost:8080/v1/chat/completions")
    .header("Connection", "keep-alive")
    .header("Keep-Alive", "timeout=60, max=100")
    .json(&request)
    .send()
    .await?;
```

**Response:**
```rust
// Server should respond with:
// Connection: keep-alive
// Keep-Alive: timeout=60, max=100
```

---

## Task 6: Timeout and Retry Configuration

### Timeout Strategy

**Recommended Timeouts:**
- Connection timeout: 10s
- Read timeout: 300s (5 min) - depends on max_tokens
- Total timeout: 600s (10 min)

```rust
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

let client = ClientBuilder::new()
    .connect_timeout(Duration::from_secs(10))
    .timeout(Duration::from_secs(300))
    .build()?;
```

### Retry Strategy

**Retry on:**
- Connection refused (server not ready)
- Timeout (transient network issues)
- 5xx errors (server errors)

**Do NOT retry on:**
- 4xx errors (client errors, bad request)
- 400 Bad Request
- 401 Unauthorized
- 403 Forbidden
- 404 Not Found

**Exponential Backoff:**
```rust
use tokio::time::{sleep, Duration};

async fn send_with_retry(
    client: &Client,
    request: ChatCompletionRequest,
    max_retries: usize,
) -> Result<ChatCompletionResponse> {
    let mut retries = 0;
    let mut delay = Duration::from_millis(100);

    loop {
        match send_request(client, &request).await {
            Ok(response) => return Ok(response),
            Err(e) if retries < max_retries => {
                retries += 1;
                eprintln!("Request failed (attempt {}/{}): {}", retries, max_retries, e);
                sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
            Err(e) => return Err(e),
        }
    }
}
```

---

## Task 7: Request/Response Logging

### Logging Strategy

**Security Considerations:**
- NEVER log API keys or sensitive tokens
- Mask or truncate sensitive content
- Use request IDs for correlation
- Separate access logs from error logs

### Access Log Configuration

**Enable Access Logging:**
```yaml
# config.yml
server:
  access_log: /var/log/llama-server/access.log
```

**Docker Volume for Logs:**
```yaml
# docker-compose.yml
volumes:
  - ./logs:/var/log/llama-server:rw  # Write access for logs
```

### Structured Access Log Format

```json
{
  "timestamp": "2026-04-20T12:34:56.789Z",
  "request_id": "req_abc123",
  "remote_addr": "127.0.0.1",
  "method": "POST",
  "path": "/v1/chat/completions",
  "protocol": "HTTP/1.1",
  "status_code": 200,
  "request_time_ms": 1234,
  "user_agent": "curl/7.68.0",
  "model": "llama-3.2-1b-instruct",
  "prompt_tokens": 10,
  "completion_tokens": 50,
  "total_tokens": 60,
  "stream": true,
  "api_key_valid": true
}
```

### Log Rotation

**Configure Log Rotation:**
```yaml
# docker-compose.yml
services:
  llama-server:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"  # Keep 3 files max (30MB total)
        compress: "true"  # Compress rotated logs
```

### Security: Sensitive Data Masking

**What to Mask:**
- API keys: `Authorization: Bearer ********`
- Sensitive prompts: Truncate or hash
- Personal data: Mask PII

**Masking Example:**
```bash
# Example log
{
  "timestamp": "2026-04-20T12:34:56.789Z",
  "request_id": "req_abc123",
  "authorization": "Bearer ***************************",  # Masked
  "prompt": "My name is *** and I'm *** years old",  # Masked PII
  "status_code": 200
}
```

### Request ID Correlation

**Generate Request ID:**
```rust
use uuid::Uuid;

let request_id = Uuid::new_v4().to_string();

// Include in headers
let response = client
    .post("http://localhost:8080/v1/chat/completions")
    .header("X-Request-ID", &request_id)
    .json(&request)
    .send()
    .await?;

// Log request ID for correlation
log_info!("Request ID: {}", request_id);
```

**Benefits of Request IDs:**
- Trace requests across services
- Debug distributed issues
- Correlate client and server logs
- Identify performance bottlenecks

---

## Task 8: Performance Benchmarks

### Benchmark Script

Create: `scripts/benchmark.sh`

```bash
#!/bin/bash
set -e

SERVER_URL="${SERVER_URL:-http://localhost:8080}"
PROMPT="The quick brown fox jumps over the lazy dog."
MAX_TOKENS=100
CONCURRENT=1

echo "=== LLM Server Benchmark ==="
echo "Server URL: $SERVER_URL"
echo "Prompt: $PROMPT"
echo "Max tokens: $MAX_TOKENS"
echo "Concurrent requests: $CONCURRENT"
echo ""

# Single request benchmark
echo "--- Single Request ---"
START_TIME=$(date +%s%N)
RESPONSE=$(curl -s -X POST "$SERVER_URL/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"model\",
    \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT\"}],
    \"max_tokens\": $MAX_TOKENS,
    \"temperature\": 0.7,
    \"stream\": false
  }")
END_TIME=$(date +%s%N)

ELAPSED_MS=$(( (END_TIME - START_TIME) / 1000000 ))
TOTAL_TOKENS=$(echo "$RESPONSE" | jq -r '.usage.total_tokens')
TPS=$(echo "scale=2; $TOTAL_TOKENS / ($ELAPSED_MS / 1000)" | bc)

echo "Elapsed time: ${ELAPSED_MS}ms"
echo "Total tokens: $TOTAL_TOKENS"
echo "Tokens per second: $TPS"
echo ""

# Extract timing info (using native endpoint for detailed timings)
TIMING=$(curl -s -X POST "$SERVER_URL/completion" \
  -H "Content-Type: application/json" \
  -d "{
    \"prompt\": \"$PROMPT\",
    \"n_predict\": $MAX_TOKENS,
    \"temperature\": 0.7,
    \"stream\": false
  }" | jq '.timings')

echo "Timings:"
echo "$TIMING" | jq

# Concurrent requests (if specified)
if [ "$CONCURRENT" -gt 1 ]; then
    echo "--- Concurrent Requests ($CONCURRENT) ---"
    START_TIME=$(date +%s%N)

    for i in $(seq 1 $CONCURRENT); do
        curl -s -X POST "$SERVER_URL/v1/chat/completions" \
          -H "Content-Type: application/json" \
          -d "{
            \"model\": \"model\",
            \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT $i\"}],
            \"max_tokens\": $MAX_TOKENS,
            \"temperature\": 0.7,
            \"stream\": false
          }" > /dev/null &
    done

    wait

    END_TIME=$(date +%s%N)
    ELAPSED_MS=$(( (END_TIME - START_TIME) / 1000000 ))

    echo "Total elapsed time: ${ELAPSED_MS}ms"
    echo "Average per request: $(( ELAPSED_MS / CONCURRENT ))ms"
fi

echo ""
echo "=== Benchmark Complete ==="
```

### Performance Targets

**7B Q4_K_M Model:**
- First token latency: <200ms
- Tokens per second: >20 TPS
- Concurrent requests: 10+ without degradation
- Total throughput: >200 tokens/s (10 concurrent)

**1B Q4_K_M Model:**
- First token latency: <100ms
- Tokens per second: >100 TPS
- Concurrent requests: 20+ without degradation
- Total throughput: >1000 tokens/s (10 concurrent)

### AMD RX 580 Performance Expectations

**GPU:** AMD RX 580 (Polaris10) with RADV driver
**Vulkan Version:** 1.4.335

**Performance Metrics:**
- **7B Q4_K_M:** ~39 tokens/second
- **1B Q4_K_M:** ~226 tokens/second
- **First token latency:** 50-100ms (after model load)
- **Subsequent tokens:** 5-20ms per token

**Critical Configuration for Polaris:**
- **Flash Attention:** MUST be disabled (`-fa 0`) due to bug (llama.cpp issue #20465)
  - Enabling causes garbled output on Polaris
- **Driver:** MUST use RADV (NOT AMDVLK)
  - AMDVLK has 2GB allocation limit (llama.cpp issue #15054)
- **Memory Allocation:** Set `GGML_VK_FORCE_MAX_ALLOCATION_SIZE=2147483646`
- **GPU Layers:** Use `n_gpu_layers=99` for near-max offload

**Recommended Server Config for RX 580:**
```yaml
server:
  parallel: true        # Enable continuous batching
  max_slots: 8          # Moderate concurrency for Polaris

sampling:
  temperature: 0.7
  top_p: 0.95

cache:
  cache_type_k: f16
  cache_type_v: f16

vulkan:
  visible_devices: "0"
  flash_attention: false  # CRITICAL: Disable for Polaris
  disable_debug: true
```

**Performance Notes:**
- Throughput varies with quantization (Q4_K_M recommended)
- Continuous batching improves throughput by 10-50%
- F16 KV cache provides 2x speedup vs F32
- Polaris performance is stable across context sizes
```

**Prometheus Config:**
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'llama-server'
    static_configs:
      - targets: ['llama-server:8080']
```

### Key Metrics to Monitor

**Throughput Metrics:**
- `llama_n_tokens_processed_total`: Total tokens processed
- `llama_slots_processing`: Active requests
- `llama_slots_idle`: Available slots

**Latency Metrics:**
- `llama_prompt_processing_seconds_total`: Prompt processing time
- `llama_token_generation_seconds_total`: Token generation time
- Derived: Prompt per token, generation per token

**Derived Metrics:**
- Tokens per second: `rate(llama_n_tokens_processed_total[1m])`
- Average latency: `llama_prompt_processing_seconds_total / llama_n_tokens_processed_total`

### Structured Logging

**Recommendation:** Use structured JSON logging for production deployments.

**Enable Structured Logging:**
```yaml
# config.yml
features:
  log_level: info
  verbose: false  # Structured logs override verbose mode
```

**Structured Log Format:**
```json
{
  "timestamp": "2026-04-20T12:34:56.789Z",
  "level": "info",
  "request_id": "req_abc123",
  "endpoint": "/v1/chat/completions",
  "method": "POST",
  "status_code": 200,
  "duration_ms": 1234,
  "prompt_tokens": 10,
  "completion_tokens": 50,
  "total_tokens": 60,
  "model": "llama-3.2-1b-instruct",
  "ip_address": "127.0.0.1"
}
```

**Request Logging Configuration:**
```yaml
# docker-compose.yml
services:
  llama-server:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
        labels: "request_id,endpoint,method"
```

**Benefits of Structured Logging:**
- Easy to parse with log aggregation tools
- Support for querying and filtering
- Better for debugging production issues
- Enables log-based metrics extraction

### Docker Resource Limits Interaction

**Resource Limits Impact on llama-server:**

| Resource | llama-server Impact | Recommended Value |
|----------|---------------------|-------------------|
| `memory_limit` | Limits KV cache size | 8-16GB for 7B models |
| `shm_size` | Required for Vulkan GPU | 8GB for GPU workloads |
| `cpu_count` | Limits thread count | 4-8 for optimal throughput |
| `pids_limit` | Limits concurrent processes | 1000 for slots + overhead |

**Docker Compose Configuration:**
```yaml
services:
  llama-server:
    # Memory limits (prevents OOM kills)
    mem_limit: 16g
    mem_reservation: 8g
    # Shared memory (required for Vulkan)
    shm_size: 8g
    # CPU limits
    cpus: '4.0'
    cpuset: '0-3'  # Pin to specific cores
    # Process limits
    pids_limit: 1000
```

**Trade-offs:**
- Lower memory limit = Smaller KV cache = Lower context capacity
- Higher CPU count = More throughput but diminishing returns
- Insufficient shm_size = Vulkan errors, crashes

---

## Verification Steps

### Step 1: Verify Server Starts Optimally

**Test:** Start server with performance config

```bash
# Start server
./scripts/start.sh

# Check logs for optimization messages
docker compose logs | grep -i "parallel\|cache\|gpu"

# Expected:
# "Continuous batching: enabled"
# "KV cache type: f16"
# "GPU layers: 999"
```

### Step 2: Verify Health Endpoint

**Test:** Health check

```bash
curl http://localhost:8080/props

# Expected:
# {"status":"ok","slots_idle":8,"slots_processing":0}

# Check Docker health status
docker compose ps --format "{{.Health}}"

# Expected: "healthy"
```

### Step 3: Verify Properties Endpoint

**Test:** Get model properties

```bash
curl http://localhost:8080/props | jq .

# Expected: JSON with model info
# Check n_ctx, n_embd, n_layer, n_gpu_layers
```

### Step 4: Verify Slots Endpoint

**Test:** Get slot status

```bash
curl http://localhost:8080/slots | jq .

# Expected: JSON with 8-16 slots
# Check state field (should be "idle")
```

### Step 5: Verify Non-Streaming Completions

**Test:** Single request

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model",
    "messages": [{"role": "user", "content": "Say hello"}],
    "max_tokens": 50,
    "temperature": 0.7,
    "stream": false
  }' | jq .

# Expected: JSON with assistant message
# Check usage.prompt_tokens, usage.completion_tokens
```

### Step 6: Verify Streaming Completions

**Test:** Stream tokens

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model",
    "messages": [{"role": "user", "content": "Count from 1 to 10"}],
    "max_tokens": 50,
    "temperature": 0.7,
    "stream": true
  }'

# Expected: SSE stream with incremental tokens
# Last line: "data: [DONE]"
```

### Step 7: Verify Metrics Endpoint

**Test:** Get Prometheus metrics

```bash
curl http://localhost:8080/metrics

# Expected: Prometheus-formatted metrics
# Check llama_n_tokens_processed_total, llama_slots_processing
```

### Step 8: Run Benchmarks

**Test:** Performance benchmark

```bash
./scripts/benchmark.sh

# Expected output:
# === LLM Server Benchmark ===
# Server URL: http://localhost:8080
# ...
# --- Single Request ---
# Elapsed time: 5000ms
# Total tokens: 50
# Tokens per second: 10.00
# ...
```

### Step 9: Verify Continuous Batching

**Test:** Concurrent requests

```bash
# Send 10 concurrent requests
for i in {1..10}; do
  curl -s -X POST http://localhost:8080/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d "{\"model\":\"model\",\"messages\":[{\"role\":\"user\",\"content\":\"Request $i\"}],\"max_tokens\":50,\"stream\":false}" > /dev/null &
done

# Wait for completion
wait

# Check slots endpoint during load
curl http://localhost:8080/slots | jq '.[] | select(.state == "processing")'

# Expected: Multiple slots in "processing" state
```

---

## Integration with Other Phases

### Phase 01: YAML Config Schema

**Output:** No direct integration
**Input:** No input from Phase 01

### Phase 02: Docker Container

**Output:** Container with llama-server
**Input:** Phase 04 documents server configuration and API

### Phase 03: Config Injection

**Output:** No direct integration
**Input:** No input from Phase 03

### Phase 05: Rust Client

**Output:** HTTP API documentation, performance tuning
**Input:** Phase 05 implements client using API from Phase 04

---

## File Locations

Create:
- `scripts/benchmark.sh` - Performance benchmark script
- `prometheus.yml` - Prometheus configuration

---

## Success Criteria

- [ ] Server starts with optimal flags
- [ ] Health endpoint returns 200
- [ ] Properties endpoint shows correct model info
- [ ] Slots endpoint shows slot status
- [ ] Non-streaming completions work
- [ ] Streaming completions work with SSE
- [ ] Metrics endpoint provides Prometheus data
- [ ] Continuous batching enabled (parallel=true)
- [ ] KV cache uses f16 for speed
- [ ] Benchmarks meet performance targets (>20 TPS)

---

## Next Steps

After completing Phase 04:
1. Proceed to Phase 05: Rust Client
2. Use API documentation to implement streaming client
3. Apply performance tuning to client code
