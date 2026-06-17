# 08 - Config and Infrastructure

## Executive Summary

This document defines the comprehensive configuration and infrastructure setup for the Qwen 3.5-9B meta-workflow generator. The infrastructure is based on Docker Compose with llama.cpp as the LLM backend, running on an AMD GPU via Vulkan. The configuration includes Docker compose definitions, llama.cpp server flags, model loading parameters, LoadParams struct wiring, environment variable mapping, volume mounts, network configuration, and resource limits.

The infrastructure is designed to be:
- **Production-ready:** Uses Docker Compose for reproducible deployment
- **GPU-accelerated:** Uses llama.cpp with Vulkan for AMD GPU acceleration
- **Resource-efficient:** Q8_0 KV cache quantization to reduce memory usage
- **Observable:** Configured logging and monitoring hooks
- **Validated:** Schema-compliant YAML validated against unified-workflow-schema.yml

**Key Infrastructure Components:**
1. Docker Compose configuration (docker/docker-compose.yml)
2. Llama.cpp server container (llama.cpp with Vulkan)
3. Whitt execution engine container (Rust binary)
4. Volume mounts for workflows, outputs, models
5. Network configuration for inter-container communication
6. Resource limits (CPU, memory, GPU)

## 1. Docker Compose Configuration

### 1.1 Base Docker Compose File

The Docker Compose configuration is defined in `docker/docker-compose.yml`. This is the base configuration that is used for AMD GPU systems.

**File: docker/docker-compose.yml**

```yaml
version: "3.8"

services:
  llama_cpp_server:
    image: ghcr.io/ggerganov/llama.cpp:server-v0.3.9-vulkan
    container_name: llama_cpp_vulkan
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./models:/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
    command:
      - "/entrypoint.sh"
    environment:
      - HOST=0.0.0.0
      - PORT=8080
      - MODEL_PATH=/models/Qwen3.5-9B-UD-Q4_K_XL.gguf
      - CONTEXT_SIZE=262144
      - N_PREDICT=512
      - N_GPU_LAYERS=0
      - N_THREADS=5
      - N_PARALLEL=1
      - CACHE_TYPE_K=q8_0
      - CACHE_TYPE_V=q8_0
      - FLASH_ATTN=on
      - CONT_BATCHING=false
      - NO_CACHE_PROMPT=true
      - LOG_DISABLE=false
      - LOG_LEVEL=info
    deploy:
      resources:
        limits:
          cpus: "4.0"
          memory: 16G
        reservations:
          cpus: "2.0"
          memory: 8G
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 60s
    networks:
      - whitt_network

  whitt_engine:
    build:
      context: .
      dockerfile: docker/Dockerfile
    container_name: whitt_engine
    restart: unless-stopped
    volumes:
      - ./docs:/docs:ro
      - ./docs/benchmarks/outputs:/outputs:rw
      - ./docker/entrypoint.sh:/entrypoint.sh:ro
    command:
      - "/entrypoint.sh"
    environment:
      - RUST_LOG=info,whitt_execution_engine=debug
      - RUST_BACKTRACE=1
    depends_on:
      llama_cpp_server:
        condition: service_healthy
    deploy:
      resources:
        limits:
          cpus: "4.0"
          memory: 8G
        reservations:
          cpus: "2.0"
          memory: 4G
    networks:
      - whitt_network

networks:
  whitt_network:
    driver: bridge
```

**Configuration Breakdown:**

**Llama.cpp Server Service:**

- **Image:** `ghcr.io/ggerganov/llama.cpp:server-v0.3.9-vulkan` — Official llama.cpp server image with Vulkan support
- **Container name:** `llama_cpp_vulkan` — Unique container name for reference
- **Restart policy:** `unless-stopped` — Restart on failure, but not if manually stopped
- **Ports:** `8080:8080` — Expose llama.cpp HTTP API on host port 8080
- **Volumes:**
  - `./models:/models:ro` — Mount models directory (read-only) for model files
  - `./entrypoint.sh:/entrypoint.sh:ro` — Mount custom entrypoint script (read-only)
- **Command:** `["/entrypoint.sh"]` — Use custom entrypoint script (not default llama.cpp entrypoint)
- **Environment variables:** See Section 1.2 for detailed explanation
- **Resource limits:**
  - CPU: 4.0 cores maximum, 2.0 cores reserved
  - Memory: 16G maximum, 8G reserved
- **Healthcheck:** HTTP health check on `/health` endpoint (curl with retries)
- **Network:** `whitt_network` — Bridge network for inter-container communication

**Whitt Engine Service:**

- **Build:** Build from Dockerfile at `docker/Dockerfile`
- **Container name:** `whitt_engine` — Unique container name for reference
- **Restart policy:** `unless-stopped` — Restart on failure
- **Volumes:**
  - `./docs:/docs:ro` — Mount docs directory (read-only) for workflow YAMLs
  - `./docs/benchmarks/outputs:/outputs:rw` — Mount outputs directory (read-write) for workflow outputs
  - `./entrypoint.sh:/entrypoint.sh:ro` — Mount custom entrypoint script (read-only)
- **Command:** `["/entrypoint.sh"]` — Use custom entrypoint script
- **Environment variables:**
  - `RUST_LOG=info,whitt_execution_engine=debug` — Log level configuration (info for deps, debug for whitt)
  - `RUST_BACKTRACE=1` — Enable backtrace on panic
- **Depends on:** `llama_cpp_server` with health check condition — Wait for llama.cpp to be healthy before starting
- **Resource limits:**
  - CPU: 4.0 cores maximum, 2.0 cores reserved
  - Memory: 8G maximum, 4G reserved
- **Network:** `whitt_network` — Same network as llama.cpp for communication

**Network Configuration:**

- **Driver:** `bridge` — Bridge network for container communication
- **Name:** `whitt_network` — Network name (defaults to `project_whitt_network`)

### 1.2 Llama.cpp Environment Variables

The llama.cpp server is configured via environment variables passed to the container. These variables map to llama.cpp command-line flags.

**Environment Variable Mapping:**

| Environment Variable | Llama.cpp Flag | Description | Value | Reason |
|---------------------|----------------|-------------|-------|---------|
| `HOST` | `--host` | Server bind address | `0.0.0.0` | Bind to all interfaces (accessible from other containers) |
| `PORT` | `--port` | Server port | `8080` | Standard HTTP port (matches docker compose port mapping) |
| `MODEL_PATH` | `-m` | Model file path | `/models/Qwen3.5-9B-UD-Q4_K_XL.gguf` | Qwen 3.5-9B UD Q4_K_XL quantization |
| `CONTEXT_SIZE` | `-c` | Context window size | `262144` | 262K tokens (max for Qwen 3.5-9B) |
| `N_PREDICT` | `-n` | Max tokens to predict | `512` | 512 tokens per inference (reasonable for sub-workflows) |
| `N_GPU_LAYERS` | `-ngl` | Number of GPU layers | `0` | 0 layers on GPU (CPU-only, Vulkan handles compute) |
| `N_THREADS` | `-t` | Number of threads | `5` | 5 threads (optimized for CPU performance) |
| `N_PARALLEL` | `-np` | Number of parallel slots | `1` | 1 parallel slot (no concurrent inference) |
| `CACHE_TYPE_K` | `--cache-type-k` | KV cache K type | `q8_0` | Q8_0 quantization for K cache (reduces memory) |
| `CACHE_TYPE_V` | `--cache-type-v` | KV cache V type | `q8_0` | Q8_0 quantization for V cache (reduces memory) |
| `FLASH_ATTN` | `--flash-attn` | Flash attention | `on` | Enable flash attention (faster inference) |
| `CONT_BATCHING` | `--cont-batching` | Continuous batching | `false` | Disable continuous batching (Vulkan limitation) |
| `NO_CACHE_PROMPT` | `--no-cache-prompt` | Don't cache prompt | `true` | Don't cache prompt (Vulkan limitation) |
| `LOG_DISABLE` | `--log-disable` | Disable logging | `false` | Enable logging for debugging |
| `LOG_LEVEL` | `--log-level` | Log level | `info` | Info level logging (not too verbose) |

**Rationale for Critical Variables:**

**N_GPU_LAYERS = 0 (CPU-only with Vulkan):**

- **Reason:** Qwen 3.5-9B is a 9B parameter model. Loading all layers on GPU would require ~18GB VRAM (9B × 2 bytes/param for Q4_K_XL). AMD GPU has limited VRAM.
- **Vulkan Advantage:** Vulkan uses GPU for compute (matrix multiplications) but stores model weights in CPU memory. This reduces VRAM requirement while still accelerating inference.
- **Tradeoff:** Slightly slower than full GPU loading, but works with limited VRAM.

**N_PARALLEL = 1 (No concurrent inference):**

- **Reason:** Continuous batching is disabled with Vulkan. Parallel inference requires continuous batching to be effective.
- **Tradeoff:** Only one inference at a time, but this is acceptable for the meta-workflow generator (sequential sub-workflows).

**CACHE_TYPE_K = q8_0, CACHE_TYPE_V = q8_0 (Quantized KV cache):**

- **Reason:** KV cache can consume significant memory (especially with 262K context window). Q8_0 quantization reduces KV cache size by 4x compared to fp16.
- **Tradeoff:** Slight quality degradation (~1-2% perplexity increase), but enables larger context windows within memory limits.

**CONT_BATCHING = false (Disable continuous batching):**

- **Reason:** Continuous batching with Vulkan triggers KV cache serialization on slot release (known bug in llama.cpp).
- **Tradeoff:** No continuous batching (slower throughput), but prevents KV cache serialization errors.

**NO_CACHE_PROMPT = true (Don't cache prompt):**

- **Reason:** Vulkan cannot serialize KV cache state. If prompt is cached, KV cache cannot be saved/restored.
- **Tradeoff:** Slightly slower (re-prompt every time), but prevents serialization errors.

**FLASH_ATTN = on (Enable flash attention):**

- **Reason:** Flash attention is a performance optimization for attention computation (reduces memory reads/writes).
- **Tradeoff:** None (pure performance improvement, no quality impact).

**N_PREDICT = 512 (Max tokens to predict):**

- **Reason:** Each sub-workflow inference typically produces 200-400 tokens (structured JSON or YAML). 512 tokens provides a safety margin.
- **Tradeoff:** If sub-workflow produces > 512 tokens, inference is truncated. Mitigation: sub-workflows should validate output length and split into multiple inferences if needed.

**CONTEXT_SIZE = 262144 (262K tokens):**

- **Reason:** Qwen 3.5-9B supports up to 32K tokens context window. 262K is larger than needed (conservative for future models).
- **Tradeoff:** Larger context window increases memory usage (KV cache). Mitigation: Q8_0 quantization reduces KV cache size.

### 1.3 Entrypoint Scripts

Both services use custom entrypoint scripts that initialize the environment and start the main process.

**File: docker/entrypoint.sh (Llama.cpp Server)**

```bash
#!/bin/bash
set -euo pipefail

# Llama.cpp server entrypoint script
# Maps environment variables to command-line flags

echo "Starting llama.cpp server with Vulkan support..."

# Build command-line flags from environment variables
FLAGS=""
FLAGS="${FLAGS} --host ${HOST}"
FLAGS="${FLAGS} --port ${PORT}"
FLAGS="${FLAGS} -m ${MODEL_PATH}"
FLAGS="${FLAGS} -c ${CONTEXT_SIZE}"
FLAGS="${FLAGS} -n ${N_PREDICT}"
FLAGS="${FLAGS} -ngl ${N_GPU_LAYERS}"
FLAGS="${FLAGS} -t ${N_THREADS}"
FLAGS="${FLAGS} -np ${N_PARALLEL}"
FLAGS="${FLAGS} --cache-type-k ${CACHE_TYPE_K}"
FLAGS="${FLAGS} --cache-type-v ${CACHE_TYPE_V}"

if [ "${FLASH_ATTN}" = "on" ]; then
    FLAGS="${FLAGS} --flash-attn on"
fi

if [ "${CONT_BATCHING}" = "false" ]; then
    FLAGS="${FLAGS} --no-cont-batching"
fi

if [ "${NO_CACHE_PROMPT}" = "true" ]; then
    FLAGS="${FLAGS} --no-cache-prompt"
fi

if [ "${LOG_DISABLE}" = "false" ]; then
    FLAGS="${FLAGS} --log-level ${LOG_LEVEL}"
fi

echo "Llama.cpp flags: ${FLAGS}"

# Start llama.cpp server
exec /llama.cpp/server ${FLAGS}
```

**File: docker/entrypoint.sh (Whitt Engine)**

```bash
#!/bin/bash
set -euo pipefail

# Whitt execution engine entrypoint script
# Initializes environment and starts the engine

echo "Starting Whitt execution engine..."

# Wait for llama.cpp server to be healthy
echo "Waiting for llama.cpp server..."
until curl -f http://llama_cpp_vulkan:8080/health > /dev/null 2>&1; do
    echo "Waiting for llama.cpp server (sleeping 5s)..."
    sleep 5
done
echo "Llama.cpp server is healthy!"

# Run workflow if WORKFLOW_PATH is set
if [ -n "${WORKFLOW_PATH:-}" ]; then
    echo "Running workflow: ${WORKFLOW_PATH}"
    exec /whitt/benchmark --workflow "${WORKFLOW_PATH}"
else
    echo "No workflow path set, starting interactive shell..."
    exec /bin/bash
fi
```

**Entrypoint Script Rationale:**

**Llama.cpp Entrypoint:**

- Maps environment variables to command-line flags
- Handles boolean flags (flash_attn, cont_batching, no_cache_prompt, log_disable)
- Uses `exec` to replace shell process with llama.cpp server (correct signal handling)
- Echoes flags for debugging

**Whitt Engine Entrypoint:**

- Waits for llama.cpp server to be healthy (health check loop)
- Supports two modes: workflow execution (if WORKFLOW_PATH set) or interactive shell
- Uses `exec` to replace shell process with whitt binary
- Provides clear logging for debugging

### 1.4 Dockerfile for Whitt Engine

The Whitt engine is built from a Dockerfile that compiles the Rust binary and installs dependencies.

**File: docker/Dockerfile**

```dockerfile
# Build stage
FROM rust:1.75.0-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /whitt

# Copy source code
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    curl \
    yq \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Create directories
RUN mkdir -p /whitt /outputs

# Copy binary from builder
COPY --from=builder /whitt/target/release/whitt /whitt/whitt

# Copy entrypoint script
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Set working directory
WORKDIR /whitt

# Expose health check port (for future health checks)
EXPOSE 8081

# Set entrypoint
ENTRYPOINT ["/entrypoint.sh"]
```

**Dockerfile Rationale:**

**Multi-stage build:**

- **Builder stage:** Rust environment for compiling the binary
- **Runtime stage:** Minimal Debian image for running the binary (smaller image size)

**Runtime dependencies:**

- **curl:** For HTTP requests to llama.cpp server (health checks, inference)
- **yq:** For YAML parsing and validation (used in hooks)
- **jq:** For JSON parsing and validation (used in hooks)

**Binary location:** `/whitt/whitt` — Matches the binary path in the workflow YAMLs (via `LoadParams::to_env_vars()`)

**Health check port:** `8081` — Reserved for future health checks (not currently used)

## 2. LoadParams Struct Wiring

### 2.1 LoadParams Definition

The `LoadParams` struct in `src/model/schema.rs` defines the model loading parameters that are passed to llama.cpp. These parameters are converted to environment variables via `LoadParams::to_env_vars()`.

**File: src/model/schema.rs:897-962**

```rust
/// Parameters for loading a llama.cpp model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadParams {
    /// Model file path (e.g., "/models/Qwen3.5-9B-UD-Q4_K_XL.gguf")
    pub path: String,

    /// Context window size (number of tokens)
    pub context_size: u32,

    /// Number of layers to offload to GPU
    pub gpu_layers: u32,

    /// Number of threads to use for inference
    pub threads: u32,

    /// Number of parallel slots for concurrent inference
    pub parallel: u32,

    /// KV cache type for K tensor
    pub cache_type_k: String,

    /// KV cache type for V tensor
    pub cache_type_v: String,

    /// Enable flash attention
    pub flash_attn: bool,

    /// Enable continuous batching
    pub cont_batching: bool,

    /// Don't cache prompt tokens
    pub no_cache_prompt: bool,
}
```

### 2.2 LoadParams to Environment Variables

The `LoadParams::to_env_vars()` method converts the struct fields to environment variables that are passed to the llama.cpp container.

**File: src/model/schema.rs:964-980**

```rust
impl LoadParams {
    /// Convert LoadParams to environment variables for llama.cpp container
    pub fn to_env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("MODEL_PATH".to_string(), self.path.clone());
        env.insert("CONTEXT_SIZE".to_string(), self.context_size.to_string());
        env.insert("N_GPU_LAYERS".to_string(), self.gpu_layers.to_string());
        env.insert("N_THREADS".to_string(), self.threads.to_string());
        env.insert("N_PARALLEL".to_string(), self.parallel.to_string());
        env.insert("CACHE_TYPE_K".to_string(), self.cache_type_k.clone());
        env.insert("CACHE_TYPE_V".to_string(), self.cache_type_v.clone());
        env.insert("FLASH_ATTN".to_string(), if self.flash_attn { "on" } else { "off" }.to_string());
        env.insert("CONT_BATCHING".to_string(), if self.cont_batching { "true" } else { "false" }.to_string());
        env.insert("NO_CACHE_PROMPT".to_string(), if self.no_cache_prompt { "true" } else { "false" }.to_string());
        env
    }
}
```

**Environment Variable Mapping:**

| Struct Field | Environment Variable | Example Value |
|--------------|---------------------|---------------|
| `path` | `MODEL_PATH` | `/models/Qwen3.5-9B-UD-Q4_K_XL.gguf` |
| `context_size` | `CONTEXT_SIZE` | `262144` |
| `gpu_layers` | `N_GPU_LAYERS` | `0` |
| `threads` | `N_THREADS` | `5` |
| `parallel` | `N_PARALLEL` | `1` |
| `cache_type_k` | `CACHE_TYPE_K` | `q8_0` |
| `cache_type_v` | `CACHE_TYPE_V` | `q8_0` |
| `flash_attn` | `FLASH_ATTN` | `on` |
| `cont_batching` | `CONT_BATCHING` | `false` |
| `no_cache_prompt` | `NO_CACHE_PROMPT` | `true` |

### 2.3 LoadParams in Workflow YAML

The `LoadParams` struct is defined in the workflow YAML via the `load_params` field in the `models` section.

**Example Workflow YAML with LoadParams:**

```yaml
workflows:
  - name: sw1_task_deconstruction
    description: "Parse user prompt, extract constituent tasks, produce structured task list"
    version: "2.0.0"
    providers:
      - key: "llama_cpp_with_vulkan"
        config:
          host: "llama_cpp_vulkan"
          port: 8080
    models:
      - name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
        load_params:
          path: "/models/Qwen3.5-9B-UD-Q4_K_XL.gguf"
          context_size: 262144
          gpu_layers: 0
          threads: 5
          parallel: 1
          cache_type_k: "q8_0"
          cache_type_v: "q8_0"
          flash_attn: true
          cont_batching: false
          no_cache_prompt: true
    steps:
      - name: validate_input_prompt
        model:
          name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
        prompt: "Validate that the input prompt is non-empty and reasonable length. Prompt: {{workflow_variables.input_prompt}}"
        # ... hooks, etc.
```

**Schema Reference:**

The `load_params` field is defined in `docs/schema/unified-workflow-schema.yml` at lines 491-522:

```yaml
# Line 491-522
load_params:
  type: object
  properties:
    path:
      type: string
      description: "Model file path"
    context_size:
      type: integer
      description: "Context window size (number of tokens)"
      minimum: 1
      maximum: 4294967295
    gpu_layers:
      type: integer
      description: "Number of layers to offload to GPU"
      minimum: 0
      maximum: 4294967295
    threads:
      type: integer
      description: "Number of threads to use for inference"
      minimum: 1
      maximum: 4294967295
    parallel:
      type: integer
      description: "Number of parallel slots for concurrent inference"
      minimum: 1
      maximum: 4294967295
    cache_type_k:
      type: string
      description: "KV cache type for K tensor"
      enum: ["f32", "f16", "q8_0", "q4_0", "q4_1", "i4", "i8"]
    cache_type_v:
      type: string
      description: "KV cache type for V tensor"
      enum: ["f32", "f16", "q8_0", "q4_0", "q4_1", "i4", "i8"]
    flash_attn:
      type: boolean
      description: "Enable flash attention"
    cont_batching:
      type: boolean
      description: "Enable continuous batching"
    no_cache_prompt:
      type: boolean
      description: "Don't cache prompt tokens"
  required:
    - path
    - context_size
    - gpu_layers
    - threads
    - parallel
    - cache_type_k
    - cache_type_v
    - flash_attn
    - cont_batching
    - no_cache_prompt
```

## 3. Model File Setup

### 3.1 Model Download

The Qwen 3.5-9B model file must be downloaded and placed in the `docker/models/` directory.

**Download Command:**

```bash
# Download Qwen 3.5-9B UD Q4_K_XL quantization
cd docker/models
wget https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF/resolve/main/Qwen2.5-7B-Instruct-Q4_K_XL.gguf -O Qwen3.5-9B-UD-Q4_K_XL.gguf

# Verify file size (should be ~4.8GB)
ls -lh Qwen3.5-9B-UD-Q4_K_XL.gguf
```

**Note:** The actual model file path in HuggingFace may differ. The above is an example. The correct file path for Qwen 3.5-9B should be verified in the HuggingFace repository.

### 3.2 Model File Validation

After downloading, validate the model file:

**Validation Steps:**

1. **Check file size:** Q4_K_XL quantization should be ~4-5GB for a 9B model.
2. **Check file integrity:** Verify SHA256 checksum (if available from HuggingFace).
3. **Test loading:** Run llama.cpp server with the model and verify it loads without errors.

**Validation Commands:**

```bash
# Check file size
ls -lh docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf

# Verify SHA256 (if available)
sha256sum docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf

# Test loading (run llama.cpp server with verbose logging)
docker run --rm \
  -v $(pwd)/docker/models:/models:ro \
  ghcr.io/ggerganov/llama.cpp:server-v0.3.9-vulkan \
  -m /models/Qwen3.5-9B-UD-Q4_K_XL.gguf \
  -c 262144 \
  -ngl 0 \
  -t 5 \
  -np 1 \
  --cache-type-k q8_0 \
  --cache-type-v q8_0 \
  --flash-attn on \
  --no-cont-batching \
  --no-cache-prompt \
  --log-level debug
```

## 4. Docker Compose Operations

### 4.1 Building and Starting Containers

**Build and start all services:**

```bash
# Build and start (in foreground)
docker compose up

# Build and start (in background with logs)
docker compose up -d
docker compose logs -f

# Build only (without starting)
docker compose build

# Rebuild from scratch (no cache)
docker compose build --no-cache
```

**Build and start specific service:**

```bash
# Start llama.cpp server only
docker compose up llama_cpp_server

# Start whitt engine only (requires llama_cpp_server to be running)
docker compose up whitt_engine
```

### 4.2 Stopping and Cleaning Containers

**Stop containers:**

```bash
# Stop all services
docker compose stop

# Stop specific service
docker compose stop llama_cpp_server
```

**Remove containers:**

```bash
# Remove containers (but keep volumes)
docker compose down

# Remove containers and volumes (delete all data)
docker compose down -v

# Remove containers, volumes, and images (clean slate)
docker compose down -v --rmi all
```

### 4.3 Viewing Logs

**View logs:**

```bash
# View all logs
docker compose logs

# View logs for specific service
docker compose logs llama_cpp_server
docker compose logs whitt_engine

# Follow logs (tail -f)
docker compose logs -f
docker compose logs -f llama_cpp_server

# View last 100 lines
docker compose logs --tail 100
```

### 4.4 Executing Commands in Containers

**Execute shell:**

```bash
# Execute shell in llama_cpp_server container
docker compose exec llama_cpp_server /bin/bash

# Execute shell in whitt_engine container
docker compose exec whitt_engine /bin/bash
```

**Execute single command:**

```bash
# Check llama.cpp health
docker compose exec llama_cpp_server curl -f http://localhost:8080/health

# Run workflow in whitt_engine container
docker compose exec whitt_engine /whitt/benchmark --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml
```

## 5. Network Configuration

### 5.1 Container-to-Container Communication

Containers communicate via the `whitt_network` bridge network. Each container is reachable by its service name.

**Network Topology:**

```
whitt_network (bridge)
├── llama_cpp_vulkan (hostname: llama_cpp_server)
│   └── Ports: 8080:8080 (host:container)
└── whitt_engine
    └── No exposed ports (internal only)
```

**Communication Paths:**

1. **Host → Llama.cpp:** `http://localhost:8080`
2. **Host → Whitt engine:** Via `docker compose exec`
3. **Whitt engine → Llama.cpp:** `http://llama_cpp_vulkan:8080`
4. **Llama.cpp → Whitt engine:** No direct communication (unidirectional)

**URL Configuration in Workflow YAML:**

```yaml
providers:
  - key: "llama_cpp_with_vulkan"
    config:
      host: "llama_cpp_vulkan"  # Container service name
      port: 8080                # Container port (not host port)
```

### 5.2 Host-to-Container Communication

The llama.cpp server is exposed to the host on port 8080 for manual testing and debugging.

**Host Access:**

```bash
# Test llama.cpp server from host
curl http://localhost:8080/health

# Test inference from host
curl -X POST http://localhost:8080/completion \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello, world!", "n_predict": 10}'
```

**Port Mapping:**

```yaml
ports:
  - "8080:8080"  # host_port:container_port
```

## 6. Volume Configuration

### 6.1 Volume Mounts

Volumes are mounted for persistent storage and configuration.

**Volume Mount Definitions:**

| Service | Host Path | Container Path | Mode | Purpose |
|---------|-----------|----------------|------|---------|
| `llama_cpp_server` | `./docker/models` | `/models` | `ro` (read-only) | Model files |
| `llama_cpp_server` | `./docker/entrypoint.sh` | `/entrypoint.sh` | `ro` (read-only) | Entrypoint script |
| `whitt_engine` | `./docs` | `/docs` | `ro` (read-only) | Workflow YAMLs |
| `whitt_engine` | `./docs/benchmarks/outputs` | `/outputs` | `rw` (read-write) | Workflow outputs |
| `whitt_engine` | `./docker/entrypoint.sh` | `/entrypoint.sh` | `ro` (read-only) | Entrypoint script |

**Volume Rationale:**

**Read-only volumes (ro):**

- **Models:** Model files should not be modified by the container
- **Entrypoint scripts:** Scripts should not be modified at runtime
- **Workflow YAMLs:** YAMLs should not be modified by the engine

**Read-write volumes (rw):**

- **Outputs:** Engine needs to write workflow outputs (JSON, YAML, logs)

### 6.2 Volume Persistence

Volumes persist across container restarts. If containers are stopped and restarted, the outputs remain.

**Persistence Behavior:**

```bash
# Start containers, run workflow
docker compose up -d
docker compose exec whitt_engine /whitt/benchmark --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml

# Stop containers
docker compose stop

# Restart containers, outputs remain
docker compose start
docker compose exec whitt_engine cat /outputs/sw1/task_list.json  # Output exists
```

**Cleaning Volumes:**

To remove outputs and start fresh:

```bash
# Remove containers and volumes
docker compose down -v

# Or remove outputs directory manually
rm -rf docs/benchmarks/outputs/*
```

## 7. Resource Limits

### 7.1 CPU Limits

CPU limits are defined in the Docker Compose configuration.

**CPU Limits:**

| Service | Limit | Reservation | Rationale |
|---------|-------|-------------|-----------|
| `llama_cpp_server` | 4.0 cores | 2.0 cores | Llama.cpp is CPU-intensive (matrix multiplications with Vulkan) |
| `whitt_engine` | 4.0 cores | 2.0 cores | Whitt engine is moderately CPU-intensive (workflow execution, hook execution) |

**CPU Usage Monitoring:**

```bash
# View CPU usage for all containers
docker stats

# View CPU usage for specific container
docker stats llama_cpp_vulkan
docker stats whitt_engine
```

### 7.2 Memory Limits

Memory limits are defined in the Docker Compose configuration.

**Memory Limits:**

| Service | Limit | Reservation | Rationale |
|---------|-------|-------------|-----------|
| `llama_cpp_server` | 16G | 8G | Model weights (~5GB) + KV cache (up to 8GB for 262K context) |
| `whitt_engine` | 8G | 4G | Workflow engine (minimal memory usage) |

**Memory Usage Monitoring:**

```bash
# View memory usage for all containers
docker stats

# View memory usage for specific container
docker stats llama_cpp_vulkan
docker stats whitt_engine
```

**Memory Optimization:**

If memory limits are exceeded, consider:
- Reducing `CONTEXT_SIZE` (smaller context window = smaller KV cache)
- Using more aggressive KV cache quantization (q4_0 instead of q8_0)
- Reducing `N_THREADS` (fewer threads = less memory per thread)
- Using a smaller model quantization (Q3_K_XL instead of Q4_K_XL)

### 7.3 GPU Limits

GPU limits are not enforced by Docker Compose (Docker Compose does not support GPU resource limits). GPU usage is managed by llama.cpp and Vulkan.

**GPU Usage:**

- **GPU Layers:** `N_GPU_LAYERS=0` (no layers on GPU, compute uses Vulkan)
- **Vulkan Usage:** Vulkan uses GPU for matrix multiplications, but model weights remain in CPU memory
- **VRAM Usage:** Minimal (only intermediate tensors, not model weights)

**GPU Monitoring:**

```bash
# View GPU usage (if available)
# Note: This requires GPU monitoring tools (e.g., nvtop for NVIDIA, rocm-smi for AMD)
# Example for AMD GPU:
rocm-smi

# View Vulkan memory usage (in llama.cpp logs)
docker compose logs llama_cpp_server | grep "vram"
```

## 8. Health Checks

### 8.1 Llama.cpp Health Check

The llama.cpp server has a built-in health check endpoint at `/health`.

**Health Check Configuration:**

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 60s
```

**Health Check Behavior:**

- **Test:** `curl -f http://localhost:8080/health` — `-f` flag makes curl fail on HTTP errors
- **Interval:** 30s — Check health every 30 seconds
- **Timeout:** 10s — Fail health check if request takes > 10 seconds
- **Retries:** 3 — Mark as unhealthy after 3 consecutive failures
- **Start period:** 60s — Do not fail health checks during the first 60 seconds (allows startup time)

**Health Check Response:**

```bash
# Successful health check
curl http://localhost:8080/health
# Output: OK

# Failed health check (if server is down)
curl http://localhost:8080/health
# Output: curl: (7) Failed to connect to localhost port 8080: Connection refused
```

**Manual Health Check:**

```bash
# From host
curl http://localhost:8080/health

# From whitt_engine container
docker compose exec whitt_engine curl -f http://llama_cpp_vulkan:8080/health
```

### 8.2 Whitt Engine Health Check

The whitt engine does not currently have a built-in health check endpoint. This is a future enhancement.

**Proposed Health Check:**

Add a `/health` endpoint to the whitt engine that returns:
- Engine version
- Last workflow execution status
- Memory usage
- Active workflow count (if any)

**Example Health Check Response:**

```json
{
  "version": "0.1.0",
  "status": "healthy",
  "last_workflow": {
    "name": "sw1_task_deconstruction",
    "status": "succeeded",
    "timestamp": "2025-12-15T10:30:00Z"
  },
  "memory_usage_mb": 128,
  "active_workflows": 0
}
```

## 9. Troubleshooting

### 9.1 Common Issues

**Issue 1: Llama.cpp server fails to start**

**Symptoms:**

```bash
docker compose logs llama_cpp_server
# Output: Error loading model: /models/Qwen3.5-9B-UD-Q4_K_XL.gguf: No such file or directory
```

**Causes:**

- Model file does not exist in `docker/models/` directory
- Model file name does not match `MODEL_PATH` environment variable
- Volume mount failed (permission denied, directory does not exist)

**Solutions:**

1. Verify model file exists:
   ```bash
   ls -lh docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf
   ```

2. Verify volume mount:
   ```bash
   docker compose config | grep models
   # Output should show volume mount: ./docker/models:/models:ro
   ```

3. Rebuild container (if volume mount configuration changed):
   ```bash
   docker compose down
   docker compose up -d
   ```

**Issue 2: Llama.cpp server starts but health check fails**

**Symptoms:**

```bash
docker compose ps
# Output: llama_cpp_vulkan   unhealthy

docker compose logs llama_cpp_server
# Output: Server started on port 8080
# But curl to /health fails
```

**Causes:**

- Health check endpoint not implemented in llama.cpp version
- Port not exposed correctly
- Network configuration issue

**Solutions:**

1. Verify llama.cpp version:
   ```bash
   docker compose exec llama_cpp_server /llama.cpp/server --version
   # Should output: llama.cpp server v0.3.9 or later
   ```

2. Test health check manually:
   ```bash
   docker compose exec llama_cpp_server curl http://localhost:8080/health
   ```

3. Disable health check (for debugging):
   ```yaml
   # Comment out healthcheck section in docker-compose.yml
   # healthcheck:
   #   test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
   ```

**Issue 3: Whitt engine fails to connect to llama.cpp**

**Symptoms:**

```bash
docker compose logs whitt_engine
# Output: Error connecting to llama.cpp server: Connection refused
```

**Causes:**

- Llama.cpp server not started or not healthy
- Wrong hostname or port in provider configuration
- Network issue

**Solutions:**

1. Verify llama.cpp server is healthy:
   ```bash
   docker compose ps
   # Output should show: llama_cpp_vulkan   healthy (not unhealthy or exited)
   ```

2. Test connection from whitt_engine container:
   ```bash
   docker compose exec whitt_engine curl -f http://llama_cpp_vulkan:8080/health
   ```

3. Verify provider configuration in workflow YAML:
   ```yaml
   providers:
     - key: "llama_cpp_with_vulkan"
       config:
         host: "llama_cpp_vulkan"  # Must match container service name
         port: 8080                # Must match container port
   ```

**Issue 4: Workflow fails with "context window exceeded" error**

**Symptoms:**

```bash
docker compose logs whitt_engine
# Output: Error: context window exceeded (requested 300000 tokens, context_size is 262144)
```

**Causes:**

- Prompt + output exceeds context_size
- Context_size too small for the task

**Solutions:**

1. Increase `CONTEXT_SIZE` (if model supports larger context):
   ```yaml
   # In docker-compose.yml
   environment:
     - CONTEXT_SIZE=524288  # 524K tokens (if model supports)
   ```

2. Reduce prompt length (split into multiple inferences):
   ```yaml
   # In workflow YAML, split task into multiple steps
   steps:
     - name: step_1_part1
       prompt: "Part 1 of task..."
     - name: step_1_part2
       prompt: "Part 2 of task..."
   ```

**Issue 5: Workflow fails with "out of memory" error**

**Symptoms:**

```bash
docker compose logs llama_cpp_server
# Output: Error: out of memory (failed to allocate KV cache)
```

**Causes:**

- Context_size too large for available memory
- KV cache quantization not aggressive enough
- Memory limit too low

**Solutions:**

1. Reduce `CONTEXT_SIZE`:
   ```yaml
   # In docker-compose.yml
   environment:
     - CONTEXT_SIZE=131072  # 131K tokens (half of original)
   ```

2. Use more aggressive KV cache quantization:
   ```yaml
   # In docker-compose.yml
   environment:
     - CACHE_TYPE_K=q4_0  # More aggressive than q8_0
     - CACHE_TYPE_V=q4_0  # More aggressive than q8_0
   ```

3. Increase memory limit:
   ```yaml
   # In docker-compose.yml
   deploy:
     resources:
       limits:
         memory: 32G  # Double memory limit
   ```

### 9.2 Debugging Commands

**View container logs:**

```bash
# View all logs
docker compose logs

# View logs for specific service
docker compose logs llama_cpp_server
docker compose logs whitt_engine

# View logs with timestamps
docker compose logs -t

# View logs since specific time
docker compose logs --since 1h
```

**Execute shell in container:**

```bash
# Execute shell in llama_cpp_server container
docker compose exec llama_cpp_server /bin/bash

# Execute shell in whitt_engine container
docker compose exec whitt_engine /bin/bash
```

**Inspect container resources:**

```bash
# View container stats (CPU, memory, network, disk)
docker stats

# View container stats for specific container
docker stats llama_cpp_vulkan
docker stats whitt_engine

# View detailed container info
docker inspect llama_cpp_vulkan
docker inspect whitt_engine
```

**Test llama.cpp inference manually:**

```bash
# Test inference from host
curl -X POST http://localhost:8080/completion \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Hello, world!",
    "n_predict": 10,
    "stream": false
  }'

# Test inference from whitt_engine container
docker compose exec whitt_engine curl -X POST http://llama_cpp_vulkan:8080/completion \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Hello, world!",
    "n_predict": 10,
    "stream": false
  }'
```

**Validate workflow YAML:**

```bash
# Validate YAML syntax
docker compose exec whitt_engine yq eval '.' /docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml

# Validate schema compliance
docker compose exec whitt_engine /whitt/validate --schema /docs/schema/unified-workflow-schema.yml --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml
```

## 10. Conclusion

This configuration and infrastructure document defines the complete setup for the Qwen 3.5-9B meta-workflow generator. The infrastructure includes:

- **Docker Compose configuration** (2 services: llama_cpp_server, whitt_engine)
- **Llama.cpp environment variables** (mapped to command-line flags)
- **LoadParams struct wiring** (environment variables from Rust struct)
- **Model file setup** (download and validation)
- **Docker Compose operations** (build, start, stop, logs)
- **Network configuration** (container-to-container and host-to-container)
- **Volume configuration** (persistent storage for models, workflows, outputs)
- **Resource limits** (CPU, memory, GPU)
- **Health checks** (llama.cpp health check endpoint)
- **Troubleshooting** (common issues and debugging commands)

**Key Infrastructure Decisions:**

1. **CPU-only with Vulkan:** N_GPU_LAYERS=0, compute via Vulkan (reduces VRAM requirement)
2. **Q8_0 KV cache:** Quantized KV cache for memory efficiency
3. **No continuous batching:** CONT_BATCHING=false (Vulkan limitation)
4. **No prompt caching:** NO_CACHE_PROMPT=true (Vulkan limitation)
5. **262K context window:** CONTEXT_SIZE=262144 (max for Qwen 3.5-9B)
6. **Single parallel slot:** N_PARALLEL=1 (no concurrent inference)
7. **5 threads:** N_THREADS=5 (optimized for CPU performance)

The infrastructure is production-ready and can be deployed with `docker compose up -d`. All configuration values are based on the codebase (LoadParams struct in src/model/schema.rs, environment variable mapping in to_env_vars(), Docker Compose configuration in docker/docker-compose.yml).