# llama.cpp Server Research Reference

## Docker Version Requirements

**Docker Engine:** 29.3.0+ recommended (fixes MIG bug, AMD CDI support)
**Docker Compose:** 2.0+ required

**NVIDIA Container Toolkit:**
- **Version:** 1.17.4+ REQUIRED (CVE-2025-23266/23359 fixes)
- **CVE Severity:** CVSS 9.0 (container escape vulnerability)
- **Installation:**
  ```bash
  distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
  curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
  curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | \
    sudo tee /etc/apt/sources.list.d/nvidia-docker.list

  sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
  sudo systemctl restart docker

  # Verify version (must be 1.17.4+)
  nvidia-container-cli --version
  ```

**AMD Container Toolkit:**
- **Version:** 1.2.0+ for --gpus support (requires Docker 25.0+)
- **Installation:**
  ```bash
  # Install AMD Container Toolkit (requires Docker 25.0+)
  curl -fsSL https://repo.radeon.com/rocm/rocm.gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/rocm-archive-keyring.gpg
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/rocm-archive-keyring.gpg] https://repo.radeon.com/rocm/apt/$(. /etc/os-release; echo $VERSION_CODENAME) main" | \
    sudo tee /etc/apt/sources.list.d/rocm.list

  sudo apt-get update && sudo apt-get install -y amdgpu-container-toolkit
  sudo systemctl restart docker

  # Verify version (must be 1.2.0+)
  amdgpu-container-cli --version
  ```

**GPU Driver Requirements:**
- NVIDIA 570.123.10+ (for CUDA workloads)
- AMD AMDGPU 6.4.x (for ROCm/Vulkan)
- Intel NEO 26.09+ (for oneAPI/Vulkan)

## Official Docker Images

**Primary Image:**
- `ghcr.io/ggml-org/llama.cpp:server-vulkan`
- Base: Ubuntu 26.04
- Build configuration: `cmake -DGGML_VULKAN=ON`

**Architecture:**
- Single-threaded context management
- Multi-threaded HTTP workers (default 4, controlled by `--parallel`)
- Queue-based task submission with automatic scheduling

## Configuration Model

### Core Constraint
**NO native YAML/JSON/TOML configuration file support.** All configuration via CLI arguments and environment variables.

### Environment Variable Pattern
`LLAMA_ARG_*` prefix maps to CLI flags (undocumented convention in codebase).

Example: `LLAMA_ARG_CTX_SIZE=4096` equals `--ctx-size 4096`.

## Key CLI Flags

### Model and Context
```bash
-m, --model <path>              # Model path (GGUF format required)
-c, --ctx-size <n>              # Context size, default 2048
-t, --threads <n>               # Thread count, default -1 (auto-detect)
```

### GPU Offloading
```bash
-ngl, --gpu-layers <n>          # GPU layers, default auto (all possible)
--split-mode <mode>             # Split mode: none, layer, row (default none)
--main-gpu <n>                  # Main GPU index, default 0
```

### Sampling Parameters
```bash
--temp <float>                  # Temperature, default 0.80
--top-k <n>                     # Top-K sampling, default 40
--top-p <float>                 # Top-P sampling, default 0.95
--min-p <float>                 # Min-P sampling, default 0.05
--repeat-penalty <float>        # Repetition penalty, default 1.00
--n-predict <n>                 # Max tokens to predict, default -1 (infinity)
```

## Server Configuration

### Network
```bash
--host <addr>                   # Bind address, default 127.0.0.1
--port <port>                   # HTTP port, default 8080
--timeout <sec>                  # Request timeout, default 600
--api-key <key>                # API key for authentication (production)
```

### API Key Authentication
```bash
--api-key "your-secret-api-key"
```
**Usage:**
- Set via command line flag
- Pass via environment variable: `LLAMA_ARG_API_KEY=your-secret-api-key`
- Requires `Authorization: Bearer <key>` header in requests
**Best Practices:**
- Use strong, randomly generated keys (minimum 32 characters)
- Rotate keys regularly
- Never log API keys

### Concurrency
```bash
--parallel <n>                  # Parallel requests, default -1 (auto=4)
--cont-batching                # Continuous batching (implicit with --parallel)
--slot-save-path <path>         # Slot persistence path (experimental)
```

**Note:** `--cont-batching` is implicit when `--parallel` is enabled. No separate flag needed.

### Concurrency
```bash
--parallel <n>                  # Parallel requests, default -1 (auto=4)
--cont-batching                 # Continuous batching enabled by default
--slot-save-path <path>         # Slot persistence path (experimental)
```

## Batching Configuration

```bash
-b, --batch-size <n>            # Global batch size, default 2048
-ub, --ubatch-size <n>          # Micro-batch size, default 512
```

**Constraint:** Embeddings mode requires `n_batch <= n_ubatch` (code enforcement).

## KV Cache Settings

```bash
--cache-type-k <type>           # Key cache type: f16 (default), q8_0, q4_0
--cache-type-v <type>           # Value cache type: f16 (default), q8_0, q4_0
--cache-ram <MiB>               # KV cache RAM limit, default 8192 MiB
--kv-unified                    # Unified KV cache (auto-enabled)
```

## Speculative Decoding

**Status: Experimental with known TODOs.**

```bash
--draft-n <n>                   # Draft tokens, default 16
--draft-min <n>                 # Min draft tokens, default 0
--draft-p-min <float>           # Min accept probability, default 0.75
--spec-type <type>              # Speculative type: ngram-cache (default)
```

**Known Limitation:** Not fully implemented for multi-sample batching (TODO in codebase).

## Memory Management

### mmap Support
```bash
--mmap                          # Enable memory mapping (default: true)
--mmap-load                     # Full mmap load (default: true)
```

**Performance Impact:** Faster model load, may cause pageouts under memory pressure.

## API Endpoints

### OpenAI-Compatible
- `/v1/chat/completions` - Chat completions with template processing
- `/v1/completions` - Standard completions
- `/v1/embeddings` - Embeddings generation

### Native (Lower Latency)
- `/completion` - Raw completion endpoint (no chat template overhead)
- `/health` - Health check (gets queued under load - see Issue #20684)
- `/props` - Server properties (recommended for health checks under load)
- `/slots` - Active slots status
- `/models` - Model listing
- `/models/unload` - Model unload endpoint

### Health Check Alternative (Recommended for Production)

**Issue #20684:** `/health` endpoint gets queued with other requests under high load.
**Workaround:** Use `/props` endpoint for health checks under load.

```bash
# Recommended health check (bypasses queue)
curl http://localhost:8080/props

# Legacy health check (may queue under load)
curl http://localhost:8080/health
```

### Streaming Format

**OpenAI /v1/chat/completions stream:**
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

**Native /completion stream:**
```json
{
  "content": "Hello",
  "tokens": [1234],
  "stop": false
}
```

## Prometheus Metrics

### Available Metrics
- `llamacpp:prompt_tokens_seconds` - Prompt processing time
- `llamacpp:predicted_tokens_seconds` - Token generation time
- `llamacpp:kv_cache_usage_ratio` - KV cache utilization

### Endpoint
- `/metrics` - Prometheus scraping endpoint

## Known Issues

### Health Check Under Load
**Issue #20684:** `/health` endpoint gets queued with other requests under high load.
**PR #20799:** Adds bypass mechanism to check server status without queueing.

### Streaming Chunk Splitting
**PR #9519:** Tested and verified SSE events may split JSON across events.
**Handling required:** Buffer incomplete JSON until parseable.

## Performance Characteristics

### OpenAI vs Native Endpoints
- `/v1/chat/completions`: Adds chat template processing overhead (10-30ms)
- `/completion`: Raw completion with minimal overhead (2-5ms)

### Latency Breakdown
- Model load: 200-1000ms (model-dependent)
- First token: 50-100ms after model load
- Subsequent tokens: 5-20ms per token (hardware-dependent)

## Model Loading

### Supported Formats
- GGUF (primary, fully supported)
- GGML (deprecated, legacy support)

### Model Search Path
1. Explicit path via `-m` flag
2. Working directory
3. `~/.cache/llama.cpp/` (default cache location)

## References

- **Repository:** https://github.com/ggerganov/llama.cpp
- **Docker Images:** https://github.com/ggml-org/llama.cpp/pkgs/container/llama.cpp
- **Issue #20684:** Health check queuing under load
- **PR #20799:** Health check bypass implementation
- **PR #9519:** SSE chunk splitting test verification
- **Vulkan Support:** https://github.com/ggerganov/llama.cpp#vulkan
