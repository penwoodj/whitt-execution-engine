# Whitt Execution Engine — User Tutorial

## Overview

Whitt is a CLI tool for running local LLMs with GPU offloading (Vulkan/CUDA), managing models, and executing autonomous agents. It wraps llama.cpp in Docker with GPU passthrough and provides a unified interface for chat, agents, and workflows.

## Quick Start (5 minutes)

### 1. Prerequisites

- Docker + Docker Compose
- Vulkan-capable GPU (AMD or NVIDIA) with drivers installed
- Rust toolchain (for building)

### 2. Build

```bash
cargo build --release --all-features
```

The binary is at `target/release/whitt`. Optionally install it:

```bash
cp target/release/whitt ~/.local/bin/
```

### 3. Start the Server

```bash
whitt server start
```

This launches a llama.cpp server in Docker with Vulkan GPU offloading. On first run, it pulls the Docker image (~2GB).

Verify it's running:

```bash
whitt server status
```

You should see:
```
Server: http://localhost:8080
Health: ok
```

### 4. Download a Model

```bash
whitt download Qwen/Qwen2.5-0.5B-Instruct-GGUF --file qwen2.5-0.5b-instruct-q4_k_m.gguf
```

This downloads a small (~400MB) model to `./models/`. For a larger model:

```bash
whitt download Qwen/Qwen2.5-7B-Instruct-GGUF --file qwen2.5-7b-instruct-q4_k_m.gguf
```

### 5. Load a Model

```bash
whitt model load Qwen2.5-0.5B-Instruct-Q4_K_M
```

List available models:

```bash
whitt model list
```

### 6. Chat

One-shot:

```bash
whitt chat "Explain Rust ownership in 3 sentences"
```

Interactive REPL (no prompt argument):

```bash
whitt chat
```

With options:

```bash
whitt chat "Write a haiku about debugging" --temperature 0.9 --max-tokens 100
```

### 7. Agent Mode

The ReAct agent autonomously uses tools (model list, model load, chat) to complete tasks:

```bash
whitt agent "What models are available and which ones are loaded?"
```

With more steps:

```bash
whitt agent "Load the smallest model and ask it to explain quantum computing" --max-steps 15
```

---

## Configuration Files

### `config.yml` — Global Settings

Location: Project root `config.yml`. Controls server defaults.

```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 512

hardware:
  threads: 4
  gpu_layers: 99        # 99 = all layers on GPU
  use_mmap: true
  flash_attn: true

cache:
  cache_type_k: f16     # Vulkan-safe (NOT quantized)
  cache_type_v: f16

server:
  host: 127.0.0.1
  port: 8080
  cache_prompt: false   # Must be false for Vulkan stability
  cont_batching: false  # Must be false for Vulkan stability

sampling:
  temperature: 0.7
  top_p: 0.95
  top_k: 40
  repeat_penalty: 1.1
```

**Vulkan-specific constraints:**
- `cache_prompt: false` — Prompt caching crashes Vulkan backend (GGML_ASSERT)
- `cont_batching: false` — Continuous batching triggers KV cache serialization crash
- `cache_type_k/v: f16` — Quantized KV cache (q8_0, q4_0) is unstable with Vulkan

### Per-Model Configs

Location: `configs/models/<ModelName>.yml`. Override global settings per model.

```yaml
model:
  path: /models/Qwen2.5-7B-Instruct-1M-Q4_K_M.gguf
  quantization: Q4_K_M
  parameter_count: 7000000000

context:
  size: 8192
  batch_size: 512
  ubatch_size: 512

hardware:
  gpu_layers: 99
  flash_attn: true

cache:
  cache_type_k: f16
  cache_type_v: f16

sampling:
  temperature: 0.7
  max_tokens: 4096
```

### `docker-compose.yml` — Docker Configuration

Volume mounts:
- `./config.yml` → `/config/config.yml` (server config)
- `./configs/models` → `/config/models` (per-model overrides)
- `./docker/entrypoint.sh` → `/entrypoint.sh` (server startup script)
- `./models` → `/models` (GGUF model files)
- Vulkan ICD: `/usr/share/vulkan/icd.d` (GPU driver)

GPU passthrough:
- AMD: `/dev/dri` + `/dev/kfd`
- NVIDIA: Uses `docker-compose.nvidia.yml` overlay

---

## All Commands Reference

### Server

| Command | Description |
|---------|-------------|
| `whitt server start` | Start llama.cpp Docker container with GPU |
| `whitt server stop` | Stop the server |
| `whitt server status` | Health check + loaded models |
| `whitt server gpu` | Detect GPU type |
| `whitt server logs` | Follow server logs |

### Model Management

| Command | Description |
|---------|-------------|
| `whitt model list` | List all models + status |
| `whitt model load <name>` | Load model into GPU memory |
| `whitt model unload <name>` | Free GPU memory |
| `whitt model swap <name>` | Unload current, load new |

### Chat

| Command | Description |
|---------|-------------|
| `whitt chat "prompt"` | One-shot response |
| `whitt chat` | Interactive REPL |
| `whitt chat "..." --system "You are..."` | Custom system prompt |
| `whitt chat "..." --temperature 0.9` | Override sampling |
| `whitt chat "..." --save convo.json` | Save conversation |

### Agent

| Command | Description |
|---------|-------------|
| `whitt agent "task"` | Run ReAct agent (default 10 steps) |
| `whitt agent "task" --max-steps 20` | More steps for complex tasks |

### Workflow

```bash
whitt workflow my-workflow.yml           # Load and validate
whitt workflow my-workflow.yml --show-config  # Show resolved config
```

### Download

```bash
whitt download Qwen/Qwen2.5-0.5B-Instruct-GGUF
whitt download Qwen/Qwen2.5-7B-Instruct-GGUF --file specific-file.gguf -o ./models
```

### Benchmark

```bash
whitt benchmark --max-tokens 200 --concurrent 4
```

---

## Architecture

```
┌──────────────────────────────────────────────────┐
│                   whitt CLI                       │
│  (chat, agent, model, server, workflow, benchmark)│
└─────────────────────┬────────────────────────────┘
                      │ HTTP (localhost:8080)
┌─────────────────────▼────────────────────────────┐
│           llama.cpp server (Docker)               │
│  ┌───────────────────────────────────────────┐    │
│  │  Vulkan GPU offloading (99 layers)        │    │
│  │  Flash Attention, f16 KV cache            │    │
│  └───────────────────────────────────────────┘    │
└─────────────────────┬────────────────────────────┘
                      │ GPU passthrough
┌─────────────────────▼────────────────────────────┐
│              AMD / NVIDIA GPU                     │
│         (Vulkan compute shaders)                  │
└──────────────────────────────────────────────────┘
```

## File Layout

```
playful-star/
├── config.yml                    # Global server config
├── docker-compose.yml            # AMD GPU
├── docker-compose.nvidia.yml     # NVIDIA GPU overlay
├── docker/
│   ├── Dockerfile
│   └── entrypoint.sh             # llama.cpp startup script
├── configs/models/
│   ├── Qwen2.5-0.5B-Instruct-Q4_K_M.yml
│   └── Qwen2.5-7B-Instruct-1M-Q4_K_M.yml
├── models/                       # GGUF files (gitignored)
├── src/
│   ├── bin/whitt.rs              # CLI entry point
│   ├── backend/                  # LLM backend trait + implementations
│   │   ├── llm_backend.rs        # LlmBackend trait
│   │   ├── llama_vulkan.rs       # Vulkan backend
│   │   └── mock_backend.rs       # Testing mock
│   ├── agent/
│   │   ├── react.rs              # ReAct agent loop
│   │   ├── tools.rs              # Tool registry + ToolExecutor
│   │   ├── sandbox.rs            # Path-based sandboxing
│   │   └── persistence.rs        # Workflow state persistence
│   ├── client/
│   │   ├── http_client.rs        # reqwest HTTP client
│   │   ├── types.rs              # Request/Response types
│   │   └── model_download.rs     # HuggingFace download
│   ├── config/
│   │   ├── mod.rs                # Config loading
│   │   └── unified.rs            # Unified YAML workflow config
│   └── model/
│       └── registry.rs           # Model lifecycle management
└── tests/
    ├── e2e_integration.rs        # E2E pipeline tests
    ├── agent_resilience.rs       # Error recovery tests
    ├── user_flows.rs             # User workflow tests
    └── ...
```

## Troubleshooting

### "could not select device driver amdgpu"
Use the base docker-compose instead of the AMD variant:
```bash
docker compose -f docker-compose.yml up -d
```

### Agent crashes with "500 Internal Server Error"
Check server logs:
```bash
whitt server logs
```
If you see `GGML_ASSERT(tensor->data != NULL)`, ensure:
- `cache_prompt: false` in config.yml
- `cont_batching: false` in config.yml
- `cache_type_k: f16` and `cache_type_v: f16` (not quantized)

### Model not found
Models must be in `./models/` directory as `.gguf` files. The filename (minus `.gguf`) becomes the model ID.

### Slow first load
First model load includes GPU shader compilation (~25s for 7B). Subsequent loads are faster.
