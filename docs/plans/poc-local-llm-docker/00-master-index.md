# PoC Local LLM Docker — Master Index

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan suite task-by-task.

**Goal:** Build a complete Proof of Concept demonstrating Rust CLI managing containerized llama.cpp with full YAML configuration, GPU passthrough, and streaming completions.

**Architecture:** Rust CLI → HTTP API → Docker Container → llama-server (Vulkan GPU) → GGUF Model

---

## PoC Objective

Demonstrate end-to-end local LLM inference where:
1. User writes YAML config describing model, hardware, sampling parameters
2. Rust CLI validates config and launches Docker container
3. Container downloads GGUF model from HuggingFace (if missing)
4. llama-server starts with Vulkan GPU acceleration
5. Rust client sends completion requests, receives streaming SSE responses
6. Full lifecycle: start, configure, infer, shutdown

**Non-Goals:**
- Multi-container orchestration beyond single LLM server
- Model fine-tuning or LoRA
- RAG or retrieval systems
- Distributed inference
- Production security hardening

---

## Dependency Graph

```
00-master-index (this file)
    │
    ├── 01-yaml-config-schema
    │       └── provides: Rust config structs, CLI parser
    │
    ├── 02-docker-container
    │       └── provides: Dockerfile, entrypoint.sh
    │
    ├── 03-config-injection
    │       ├── depends: 01 (YAML schema)
    │       ├── depends: 02 (Dockerfile)
    │       └── provides: docker-compose.yml, config mount logic
    │
    ├── 04-http-interface
    │       ├── depends: 02 (container)
    │       └── provides: llama-server tuning, HTTP client patterns
    │
    └── 05-rust-client
            ├── depends: 01 (config structs)
            ├── depends: 03 (docker-compose)
            ├── depends: 04 (HTTP interface)
            └── provides: src/bin/poc_client.rs
```

**Critical Path:** 01 → 02 → 03 → 05 (Phase 04 runs in parallel)

---

## Phase Overview Table

| Phase | Name | Goal | Deliverables | Verification |
|-------|------|------|--------------|--------------|
| 01 | YAML Config Schema | Define unified YAML configuration system | Config structs, CLI parser, env var translation | Parse valid YAML, reject invalid, translate to CLI args |
| 02 | Docker Container | Build llama.cpp container with auto-install | Dockerfile, entrypoint.sh, .dockerignore | Build succeeds, container starts, /health returns 200 |
| 03 | Config Injection | Mount and apply YAML config to container | docker-compose.yml, config override logic | Change YAML, restart, verify new settings applied |
| 04 | HTTP Interface | Optimize llama-server for Rust client | Server tuning docs, benchmark scripts | Curl benchmarks, concurrent request test, streaming |
| 05 | Rust Client | Implement streaming completion client | src/bin/poc_client.rs, integration test | cargo run, see streamed output from model |

---

## Prerequisites Checklist

### System Requirements

- [ ] Docker 20.10+ with GPU support (NVIDIA or AMD)
- [ ] Docker Compose 2.0+
- [ ] 8GB+ GPU VRAM (tested on AMD RX 6800, NVIDIA RTX 3060)
- [ ] 16GB+ system RAM
- [ ] 20GB+ disk space for models

### Software

- [ ] Rust 1.75+ with Cargo
- [ ] Vulkan SDK (for local dev, not needed in container)
- [ ] curl for manual API testing
- [ ] huggingface-cli for model downloads

### Project Setup

- [ ] Clone github.com/penwoodj/whitt-execution-engine
- [ ] Existing Cargo.toml dependencies verified:
  - `llama-cpp-2` with vulkan feature
  - `serde-saphyr` for YAML
  - `clap` for CLI
  - `tokio` for async
  - `anyhow` for errors
  - `garde` for validation

### Environment

- [ ] `HF_TOKEN` environment variable (optional, for private models)
- [ ] `GGML_VK_VISIBLE_DEVICES` set to target GPU (default: 0)
- [ ] Network: 8080 port available for llama-server

---

## Success Criteria

### Phase 01 Success
- [ ] YAML schema validates all 8 config sections (model, context, hardware, sampling, server, cache, features, vulkan)
- [ ] Rust structs compile with garde validation rules
- [ ] CLI parser accepts subcommands: `config validate`, `config translate`, `config init`
- [ ] Translation produces correct `LLAMA_ARG_*` env vars for llama.cpp

### Phase 02 Success
- [ ] Dockerfile multi-stage build produces <500MB image
- [ ] Entrypoint script downloads models with resume capability
- [ ] Container passes healthcheck on /health endpoint
- [ ] Graceful shutdown on SIGTERM (<5 seconds)

### Phase 03 Success
- [ ] docker-compose.yml launches container with config volume
- [ ] YAML config overrides container defaults correctly
- [ ] Switching models by changing config works without rebuild
- [ ] GPU passthrough functional (both AMD and NVIDIA)

### Phase 04 Success
- [ ] llama-server handles 10+ concurrent requests
- [ ] First-token latency <200ms on 7B Q4_K_M model
- [ ] Streaming tokens arrive with <50ms inter-arrival
- [ ] /metrics endpoint exposes Prometheus-compatible metrics

### Phase 05 Success
- [ ] `cargo run --bin poc_client -- --prompt "Hello" --stream` outputs tokens in real-time
- [ ] Client handles connection errors, timeouts, malformed responses
- [ ] Integration test passes: start container, infer, verify response, shutdown

### Overall PoC Success
- [ ] End-to-end flow: YAML config → Docker → inference → streaming output
- [ ] No manual model placement outside container
- [ ] Config-driven without code changes
- [ ] GPU utilization >80% during inference
- [ ] Total inference time documented (first token, total tokens, TPS)

---

## Risk Register

| Risk | Impact | Probability | Mitigation |
|------|--------|--------------|------------|
| **GPU passthrough fails** | High | Medium | Support both AMD (--device /dev/dri) and NVIDIA (--gpus all); add GPU detection in entrypoint |
| **Model download fails** | Medium | Medium | Use huggingface-cli with resume; cache models in volume; support local model mount fallback |
| **Vulkan not initialized** | High | Low | Include full Vulkan runtime deps in Dockerfile; add Vulkan initialization test in entrypoint |
| **Container port conflicts** | Low | Medium | Use random port in docker-compose (--publish "8080"); document port mapping |
| **SSE streaming parse errors** | Medium | Low | Use robust SSE parser; implement reconnection logic; add debug logging |
| **Config validation edge cases** | Medium | Low | Extensive garde rules; unit tests for all sections; example configs |
| **Memory OOM on large models** | Medium | Low | Add GGML_VK_FORCE_MAX_ALLOCATION_SIZE defaults; document model size requirements |
| **HTTP timeout mismatches** | Low | Low | Align container timeouts with client timeouts; exponential backoff retry |

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         User (Developer)                         │
│  Writes config.yml, runs cargo run --bin poc_client              │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    src/bin/poc_client.rs                         │
│  • Parse CLI args (--config, --prompt, --stream)                 │
│  • Validate YAML config using garde                            │
│  • Start Docker container via docker-compose                     │
│  • Wait for /health endpoint                                    │
│  • Send POST /v1/chat/completions                               │
│  • Parse SSE stream, print tokens in real-time                  │
│  • Shutdown container on completion/error                       │
└───────────────────────────┬─────────────────────────────────────┘
                            │ HTTP (reqwest)
                            │ POST /v1/chat/completions
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Docker Container                               │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  docker-entrypoint.sh                                   │   │
│  │  • Download GGUF model from HuggingFace                │   │
│  │  • Translate YAML → LLAMA_ARG_* env vars              │   │
│  │  • Validate GPU (Vulkan)                                │   │
│  │  • Launch llama-server with CLI flags                  │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  llama-server (llama.cpp)                                │   │
│  │  • HTTP API on 0.0.0.0:8080                             │   │
│  │  • Endpoints: /health, /v1/chat/completions, /metrics   │   │
│  │  • Streaming via SSE                                    │   │
│  │  • Continuous batching (--parallel)                    │   │
│  │  • KV cache optimization (--cache-type-k f16)           │   │
│  └─────────────────────────────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  llama.cpp Engine (Vulkan GPU)                          │   │
│  │  • Load GGUF model                                      │   │
│  │  • GPU layer offload (-ngl 999)                         │   │
│  │  • KV cache in GPU memory                               │   │
│  │  • Sampling (temp, top-p, top-k)                       │   │
│  │  • Token generation                                    │   │
│  └─────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    GGUF Model File                               │
│  • Downloaded from HuggingFace on container start                │
│  • Cached in /models volume                                      │
│  • Examples: Llama 3.2 1B, Qwen 2.5 1.5B, Gemma 3 4B             │
│  • Format: Q4_K_M (recommended)                                  │
└─────────────────────────────────────────────────────────────────┘

Data Flow:
1. User creates config.yml with model settings
2. Rust CLI validates config, starts container
3. Entrypoint downloads model (if missing), translates config to env vars
4. llama-server loads model with GPU layers, starts HTTP API
5. Rust client waits for /health, sends completion request
6. Server generates tokens, streams via SSE
7. Client parses SSE, prints tokens in real-time
8. Container shuts down, model cached in volume for next run

Key Integration Points:
- **Phase 01 → 03:** YAML schema passed as Docker volume
- **Phase 02 → 04:** Container image with llama-server binary
- **Phase 03 → 05:** docker-compose manages container lifecycle
- **Phase 04 → 05:** HTTP API specification matches Rust client
- **Phase 01 → 05:** Config structs reused for CLI validation

Performance Targets:
- First token latency: <200ms (7B Q4_K_M, 80% GPU layers)
- Tokens per second: >20 TPS (streaming)
- Concurrent requests: 10+ without degradation
- Memory usage: <8GB GPU VRAM (7B Q4_K_M)
- Container startup: <30s (including model download if cached)
```

---

## Research Findings (Embedded)

### llama.cpp Configuration Constraints

**Critical:** llama.cpp has NO native YAML/JSON/TOML config format. Configuration is via:
- CLI arguments (e.g., `-m model.gguf -c 4096 -ngl 999`)
- Environment variables with `LLAMA_ARG_` prefix (e.g., `LLAMA_ARG_CTX_SIZE=4096`)

This PoC implements a translation layer: YAML config → `LLAMA_ARG_*` env vars → llama.cpp internal state.

### Official Docker Images

**Primary Image:** `ghcr.io/ggml-org/llama.cpp:server-vulkan`
- **Base OS:** Ubuntu 26.04
- **Build configuration:** `cmake -DGGML_VULKAN=ON`
- **Architecture:**
  - Single-threaded context management
  - Multi-threaded HTTP workers (default 4, controlled by `--parallel`)
  - Queue-based task submission with automatic scheduling
- **Includes:** llama-server binary (no model included)

### Key CLI Defaults (llama-server)

**Server Network:**
- `--host`: Default `127.0.0.1` (bind to localhost)
- `--port`: Default `8080` (HTTP port)
- `--timeout`: Default `600` seconds

**Model and Context:**
- `-m, --model <path>`: Model path (GGUF format required)
- `-c, --ctx-size <n>`: Context size, default `2048` (model-dependent, use `0` for model default)
- `-t, --threads <n>`: Thread count, default `-1` (auto-detect)
- `-b, --batch-size <n>`: Global batch size, default `2048`
- `-ub, --ubatch-size <n>`: Micro-batch size, default `512`

**Constraint:** Embeddings mode requires `n_batch <= n_ubatch` (code enforcement).

**GPU Offloading:**
- `-ngl, --gpu-layers <n>`: GPU layers, default auto (all possible)
- `--split-mode <mode>`: Split mode: none, layer, row (default none)
- `--main-gpu <n>`: Main GPU index, default `0`

**Sampling Parameters:**
- `--temp <float>`: Temperature, default `0.80`
- `--top-k <n>`: Top-K sampling, default `40`
- `--top-p <float>`: Top-P sampling, default `0.95`
- `--min-p <float>`: Min-P sampling, default `0.05`
- `--repeat-penalty <float>`: Repetition penalty, default `1.00`
- `-n, --n-predict <n>`: Max tokens to predict, default `-1` (infinity)

**Concurrency:**
- `--parallel <n>`: Parallel requests, default `-1` (auto=4)
- `--cont-batching`: Continuous batching enabled by default
- `--slot-save-path <path>`: Slot persistence path (experimental)

**KV Cache:**
- `--cache-type-k <type>`: Key cache type: f16 (default), q8_0, q4_0
- `--cache-type-v <type>`: Value cache type: f16 (default), q8_0, q4_0
- `--cache-ram <MiB>`: KV cache RAM limit, default `8192` MiB
- `--kv-unified`: Unified KV cache (auto-enabled)

**Memory Management:**
- `--mmap`: Enable memory mapping (default: true)
- `--mmap-load`: Full mmap load (default: true)
- **Performance Impact:** Faster model load, may cause pageouts under memory pressure

### Vulkan Runtime Dependencies

Required packages in container:
```
libvulkan1          # Vulkan loader
mesa-vulkan-drivers # GPU drivers (AMD/intel)
libglvnd0           # GL/VND dispatch
libgl1              # OpenGL library
libglx0             # GLX library
libegl1             # EGL library
libgles2            # OpenGL ES library
```

NVIDIA requires additional: `nvidia-container-toolkit`

### GPU Passthrough Configuration

**AMD GPUs:**
```bash
--device /dev/dri:/dev/dri
--group-add video
--device /dev/kfd
```
- **Prerequisites:** ROCm drivers installed on host
- No NVIDIA toolkit required
- `--device=/dev/kfd` for KFD (AMD GPU device)
- `--device=/dev/dri` for Direct Rendering Infrastructure
- `--group-add video` for video device group permissions

**NVIDIA GPUs:**
```bash
--gpus all
--runtime=nvidia
```
- **Prerequisites:**
  - `nvidia-container-toolkit` installed on host
  - `nvidia-docker` version 2.0+
  - NVIDIA driver >= 470.x
- **Toolkit Installation:**
  ```bash
  distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
  curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
  curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | \
    sudo tee /etc/apt/sources.list.d/nvidia-docker.list

  sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
  sudo systemctl restart docker
  ```

**Intel GPUs:**
```bash
--device=/dev/dri
--group-add video
--group-add render
```
- **Recommendation:** SYCL preferred over Vulkan for Intel GPUs (better performance)
- **Prerequisites:** Intel oneAPI Base Toolkit, Intel GPU drivers (latest)

### Vulkan Environment Variables

**Device Selection:**
- `GGML_VK_VISIBLE_DEVICES=0,1,2`: GPU indices to use (comma-separated)
- Indices match GPU order from `vulkaninfo`

**Memory Management:**
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE=1073741824`: Max allocation in bytes (1GB example)
- `GGML_VK_DISABLE_COOPMAT=1`: Disable cooperative matrices

**AMD-Specific:**
- `GGML_VK_ALLOW_GRAPHICS_QUEUE=1`: Allow graphics queue usage (AMD)

**Debugging:**
- `VK_LAYER_PATH=/path/to/layers`
- `VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation`

### Vulkan ICD Loader Issue (Critical Gotcha)

**Issue #1392:** NVIDIA ICD files mounted at `/etc/vulkan/icd.d/` but applications expect `/usr/share/vulkan/icd.d/`.

**Symptoms:**
- `vkCreateInstance` failures
- "No compatible GPU found" errors

**Workaround (add to Dockerfile):**
```dockerfile
RUN ln -s /etc/vulkan/icd.d /usr/share/vulkan/icd.d
```

**Verification:**
```bash
docker run --rm --gpus all ghcr.io/ggml-org/llama.cpp:server-vulkan \
    vulkaninfo | grep "GPU id"
```

### Multi-GPU Configuration

**Environment variable:**
```bash
GGML_VK_VISIBLE_DEVICES=0,1
```

**Usage:**
```bash
docker run \
    --gpus all \
    -e GGML_VK_VISIBLE_DEVICES=0,1 \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**KV cache split:** Automatic across visible GPUs

### Server API Endpoints

**OpenAI-Compatible:**
- `POST /v1/chat/completions`: Chat completions with template processing (streaming support)
- `POST /v1/completions`: Standard completions
- `POST /v1/embeddings`: Text embeddings

**Native (Lower Latency):**
- `GET /health`: Health check (returns 200 if healthy)
- `GET /props`: Model properties (context size, n_ctx, etc.)
- `GET /slots`: Active slots status
- `POST /completion`: Raw completion endpoint (no chat template overhead)
- `GET /models`: Model listing
- `POST /models/unload`: Model unload endpoint
- `GET /metrics`: Prometheus scraping endpoint

### Streaming Implementation

**Format:** Server-Sent Events (SSE)
- Content-Type: `text/event-stream`
- Events start with `data: ` prefix
- Last event is `data: [DONE]`
- Client must parse incrementally

**OpenAI /v1/chat/completions stream format:**
```json
{
  "id": "chatcmpl-xxx",
  "object": "chat.completion.chunk",
  "created": 1234567890,
  "model": "llama-3-8b-instruct",
  "choices": [
    {
      "index": 0,
      "delta": {
        "role": "assistant",
        "content": "Hello"
      },
      "finish_reason": null
    }
  ],
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 1,
    "total_tokens": 11
  }
}
```

**Termination signal:** `data: [DONE]`

**Native /completion stream format:**
```json
{
  "content": "Hello",
  "tokens": [1234],
  "stop": false
}
```

### Streaming Chunk Splitting (Critical Gotcha)

**PR #9519:** Tested and verified SSE events may split JSON across events.

**Symptoms:**
- Parse errors on chunk boundaries
- Incomplete JSON objects

**Solution:** Buffer incomplete JSON until parseable:
```rust
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
```

### Health Check Under Load (Known Issue)

**Issue #20684:** `/health` endpoint gets queued with other requests under high load.
**PR #20799:** Adds bypass mechanism to check server status without queueing.

**Workaround for Phase 02:** Use `/props` endpoint for health checks under load instead of `/health`.

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

### Performance Characteristics

**OpenAI vs Native Endpoints:**
- `/v1/chat/completions`: Adds chat template processing overhead (10-30ms)
- `/completion`: Raw completion with minimal overhead (2-5ms)

**Latency Breakdown:**
- Model load: 200-1000ms (model-dependent)
- First token: 50-100ms after model load
- Subsequent tokens: 5-20ms per token (hardware-dependent)

### Prometheus Metrics

**Available Metrics:**
- `llamacpp:prompt_tokens_seconds`: Prompt processing time
- `llamacpp:predicted_tokens_seconds`: Token generation time
- `llamacpp:kv_cache_usage_ratio`: KV cache utilization

**Endpoint:**
- `GET /metrics`: Prometheus scraping endpoint

### GGUF Model Formats

**Recommended:** Q4_K_M
- Size: ~4.2GB (7B parameter model)
- Quality: 96% of full precision (evaluated on MMLU)
- Speed: 1.5x faster than Q8_0
- VRAM: ~5.5GB

**Fast Small Models:**
- Llama 3.2 1B: 226 TPS, <1GB VRAM
- Qwen 2.5 1.5B: 180 TPS, ~1.5GB VRAM
- Gemma 3 1B: 200 TPS, <1GB VRAM
- Gemma 3 4B: 80 TPS, ~3GB VRAM

**Quantization Options:**
- Q4_0, Q4_K, Q5_0, Q5_K, Q6_K, Q8_0, F16, F32
- Q4_K_M recommended for best balance of size/speed/quality

### HuggingFace Download

**CLI tool:**
```bash
huggingface-cli download \
  --repo-type model \
  --local-dir /models \
  --local-dir-use-symlinks False \
  {org}/{repo} \
  {filename}
```

**Python huggingface_hub:**
```python
from huggingface_hub import hf_hub_download

model_path = hf_hub_download(
    repo_id="meta-llama/Meta-Llama-3-8B-Instruct",
    filename="llama-3-8b-instruct.Q4_K_M.gguf",
    local_dir="/models",
    token=os.environ.get("HUGGING_FACE_HUB_TOKEN")
)
```

**Resume support:** Built-in to huggingface-cli

**Authentication:** `HUGGING_FACE_HUB_TOKEN` env var for private/gated models

**URL pattern (fallback):**
```
https://huggingface.co/{org}/{repo}/resolve/{branch}/{filename}
```

### HF_TOKEN Handling (Critical Security)

**DO NOT USE BUILD ARGS:**
```dockerfile
# BAD: Visible in docker history
ARG HF_TOKEN=hf_xxx
ENV HUGGING_FACE_HUB_TOKEN=${HF_TOKEN}
```

**USE ENVIRONMENT VARIABLES:**
```bash
# Development: Pass as env var
docker run \
    -e HUGGING_FACE_HUB_TOKEN=hf_xxx \
    ghcr.io/ggml-org/llama.cpp:server-vulkan

# Production: Use Docker secrets
echo "hf_xxx" | docker secret create hf_token -

docker service create \
    --secret source=hf_token,target=hf_token \
    -e HUGGING_FACE_HUB_TOKEN_FILE=/run/secrets/hf_token \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

### Docker Health Check Requirements

**CRITICAL:** curl must be in RUNTIME layer, not build layer.

```dockerfile
# GOOD: curl in runtime layer
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan
RUN apt-get update && apt-get install -y --no-install-recommends curl
HEALTHCHECK CMD curl -f http://localhost:8080/health || exit 1

# BAD: curl in builder layer (not available in runtime)
```

**Healthcheck under load workaround:**
```dockerfile
# Use /props instead of /health to avoid queueing
HEALTHCHECK CMD curl -f http://localhost:8080/props || exit 1
```

### Graceful Shutdown

**Using --init flag:**
```bash
docker run --init ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Using dumb-init:**
```dockerfile
RUN apt-get update && apt-get install -y dumb-init

ENTRYPOINT ["dumb-init", "--"]
CMD ["llama-server", "-m", "/models/model.gguf"]
```

**Model Unload Before Exit:**
```bash
#!/bin/sh
shutdown_handler() {
    curl -X POST http://localhost:8080/models/unload || true
    exit 0
}

trap shutdown_handler SIGTERM SIGINT

llama-server -m /models/model.gguf &
wait $!
```

### Speculative Decoding (Experimental)

**Status:** Experimental with known TODOs.

**Flags:**
- `--draft-n <n>`: Draft tokens, default 16
- `--draft-min <n>`: Min draft tokens, default 0
- `--draft-p-min <float>`: Min accept probability, default 0.75
- `--spec-type <type>`: Speculative type: ngram-cache (default)

**Known Limitation:** Not fully implemented for multi-sample batching (TODO in codebase).

### Rust HTTP Client Configuration

**Reqwest Version:** 0.13 with `stream` feature

**Connection Pooling (Recommended):**
```rust
let client = Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(90))
    .http2_prior_knowledge() // Disable HTTP/2 if not needed
    .build()
    .expect("Failed to build client");
```

**Disable TLS for Local Development:**
```rust
let client = Client::builder()
    .danger_accept_invalid_certs(true) // Only for development
    .build()?;
```

### SSE Streaming Options

**1. eventsource-stream (Recommended)**
- **Version:** 0.2.3
- **Maturity:** Production-ready
- **Features:** Reconnection, error handling, event filtering

```toml
[dependencies]
eventsource-stream = "0.2.3"
```

**2. reqwest-eventsource**
- **Version:** 0.5.x
- **Features:** Built on reqwest, automatic retry

**3. eventsrc (Newer)**
- **Version:** 0.2.x
- **Features:** Minimal, async-native
- **Note:** Less mature than eventsource-stream

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

### Serde Type Optimization

**Use Exact Types:**
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

**Skip Unused Fields:**
```rust
#[derive(Deserialize)]
struct MinimalChunk {
    #[serde(default)]
    content: String,
}
```

### Docker Performance Overhead

**Finding:** No Docker-native performance regression documented in llama.cpp issues.

**Overhead Sources:**
- Containerization layer: <5% CPU overhead
- GPU passthrough: Negligible (<1%)
- Network: Only for external clients

**Benchmark:**
- Native: 50 tokens/sec
- Dockerized: 47-49 tokens/sec (3-6% overhead)

---

## Execution Order

**Sequential (must complete in order):**
1. Phase 01: YAML Config Schema
2. Phase 02: Docker Container
3. Phase 03: Config Injection
4. Phase 05: Rust Client

**Parallel (can run during Phase 03):**
- Phase 04: HTTP Interface

**Critical Path:** 01 → 02 → 03 → 05 (Phase 04 is documentation/research, can proceed independently)

---

## Phase File Locations

```
docs/plans/poc-local-llm-docker/
├── 00-master-index.md          (this file)
├── 01-yaml-config-schema.md
├── 02-docker-container.md
├── 03-config-injection.md
├── 04-http-interface.md
└── 05-rust-client.md
```

---

## Next Steps

1. Review this master index for completeness
2. Begin Phase 01: YAML Config Schema
3. Follow dependency graph for sequential execution
4. Track progress with todo list (see todo_write output)
5. Verify each phase before proceeding to next

**Start:** Execute Phase 01 (01-yaml-config-schema.md)

---

## Appendix: llama.cpp Research References

These findings are embedded throughout the phase plans. Do NOT reference external documents — all necessary info is included inline.

**Official Resources:**
- GitHub: github.com/ggerganov/llama.cpp
- Docker Hub: ghcr.io/ggml-org/llama.cpp
- HuggingFace: huggingface.co/models (search "gguf")

**Key Files in llama.cpp:**
- `examples/server/server.cpp` - HTTP server implementation
- `common/common.h` - Common data structures
- `ggml-vulkan.h` - Vulkan integration
- `Dockerfile.vulkan` - Official Vulkan Dockerfile

**Known Issues and PRs:**
- Issue #20684: Health check queuing under load
- PR #20799: Health check bypass implementation
- PR #9519: SSE chunk splitting test verification
- Issue #1392: Vulkan ICD loader path mismatch
