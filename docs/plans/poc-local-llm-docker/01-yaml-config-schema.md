# Phase 01: YAML Config Schema + Global CLI

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Design unified YAML configuration system for llama.cpp with Rust validation, CLI parsing, and environment variable translation.

**Tech Stack:** Rust, serde, serde-saphyr v0.0.24, clap, garde, anyhow

**llama-cpp-2 Configuration:**
- **Version:** Use `llama-cpp-2 = { version = "0.1.138", features = ["vulkan", "sampler"] }` in Cargo.toml
- **Latest version:** 0.1.144 (April 19, 2026)
- **Note:** No 0.2.x series exists; 0.1.138 is compatible
- **Sampler feature:** The "sampler" feature provides cleaner Rust sampling API
- **Usage in this PoC:** llama-cpp-2 is NOT used in Phase 05 (HTTP client approach). It's reserved for future embedded mode.

**Dependencies:**
- Phase 01 is independent (no dependencies on other phases)
- Output used by Phase 03 (config injection) and Phase 05 (Rust client)

---

## Research Foundation: llama.cpp Configuration Model

### Core Constraint

**NO native YAML/JSON/TOML configuration file support.** All configuration is via:
- CLI arguments (e.g., `-m model.gguf -c 4096 -ngl 999`)
- Environment variables with `LLAMA_ARG_*` prefix (undocumented convention in codebase)

**Example:**
```bash
# CLI arguments
llama-server -m /models/model.gguf -c 4096 -ngl 999 --temp 0.80

# Environment variable equivalent
LLAMA_ARG_MODEL_PATH=/models/model.gguf
LLAMA_ARG_CTX_SIZE=4096
LLAMA_ARG_N_GPU_LAYERS=999
LLAMA_ARG_TEMP=0.80
```

### Environment Variable Pattern

`LLAMA_ARG_*` prefix maps to CLI flags:
- Converts kebab-case to snake_case: `--ctx-size` → `LLAMA_ARG_CTX_SIZE`
- Converts short flags: `-c` → `LLAMA_ARG_CTX_SIZE`
- All boolean flags use `"1"` for true, `"0"` for false

### Upstream CLI Defaults (llama-server)

**Server Network (from llama.cpp source):**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--host` | `LLAMA_ARG_HOST` | `127.0.0.1` |
| `--port` | `LLAMA_ARG_PORT` | `8080` |
| `--timeout` | `LLAMA_ARG_TIMEOUT` | `600` |

**Model and Context:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `-m, --model` | `LLAMA_ARG_MODEL_PATH` | Required |
| `-c, --ctx-size` | `LLAMA_ARG_CTX_SIZE` | `2048` (use `0` for model default) |
| `-t, --threads` | `LLAMA_ARG_N_THREADS` | `-1` (auto-detect) |

**GPU Offloading:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `-ngl, --gpu-layers` | `LLAMA_ARG_N_GPU_LAYERS` | Auto (all possible) |
| `--split-mode` | `LLAMA_ARG_SPLIT_MODE` | `none` |
| `--main-gpu` | `LLAMA_ARG_MAIN_GPU` | `0` |

**Sampling Parameters:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--temp` | `LLAMA_ARG_TEMP` | `0.80` |
| `--top-k` | `LLAMA_ARG_TOP_K` | `40` |
| `--top-p` | `LLAMA_ARG_TOP_P` | `0.95` |
| `--min-p` | `LLAMA_ARG_MIN_P` | `0.05` |
| `--repeat-penalty` | `LLAMA_ARG_REPEAT_PENALTY` | `1.00` |
| `-n, --n-predict` | `LLAMA_ARG_N_PREDICT` | `-1` (infinity) |

**Concurrency:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--parallel` | `LLAMA_ARG_PARALLEL` | `false` (set to `"1"` to enable) |
| `--cont-batching` | (implicit with --parallel) | Enabled with parallel |

**Batching Configuration:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `-b, --batch-size` | `LLAMA_ARG_BATCH_SIZE` | `2048` |
| `-ub, --ubatch-size` | `LLAMA_ARG_UBATCH_SIZE` | `512` |

**Constraint:** Embeddings mode requires `n_batch <= n_ubatch` (code enforcement in llama.cpp).

**KV Cache Settings:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--cache-type-k` | `LLAMA_ARG_CACHE_TYPE_K` | `f16` |
| `--cache-type-v` | `LLAMA_ARG_CACHE_TYPE_V` | `f16` |
| `--cache-ram` | `LLAMA_ARG_CACHE_RAM` | `8192` MiB |
| `--kv-unified` | (implicit) | Auto-enabled |

**Cache Type Options:**
- `f32`: Full precision (32-bit) - Slowest, most accurate
- `f16`: Half precision (16-bit) - Recommended, 2x faster than f32
- `q8_0`: 8-bit quantized - Faster, minor quality loss
- `q4_0`: 4-bit quantized - Fastest, noticeable quality loss

**Memory Management:**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--mmap` | `LLAMA_ARG_USE_MMAP` | `true` |
| `--mmap-load` | `LLAMA_ARG_MMAP_LOAD` | `true` |

**Performance Impact:** Faster model load, may cause pageouts under memory pressure.

**Speculative Decoding (Experimental):**
| CLI Flag | Env Variable | Default |
|----------|-------------|---------|
| `--draft-n` | `LLAMA_ARG_DRAFT_N` | `16` |
| `--draft-min` | `LLAMA_ARG_DRAFT_MIN` | `0` |
| `--draft-p-min` | `LLAMA_ARG_DRAFT_P_MIN` | `0.75` |
| `--spec-type` | `LLAMA_ARG_SPEC_TYPE` | `ngram-cache` |

**Known Limitation:** Not fully implemented for multi-sample batching (TODO in codebase).

**Metrics Endpoints:**
| CLI Flag | Effect |
|----------|--------|
| `--metrics` | Enable `/metrics` endpoint (Prometheus) |
| `--slots` | Enable `/slots` endpoint (slot status) |

### Vulkan Environment Variables

**Device Selection:**
| Variable | Purpose | Default |
|----------|---------|---------|
| `GGML_VK_VISIBLE_DEVICES` | GPU indices to use | `0` (comma-separated) |
| `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` | Max GPU allocation (bytes) | `0` (auto) |
| `GGML_VK_DISABLE_COOPMAT` | Disable cooperative matrices | `0` |
| `GGML_VK_ALLOW_GRAPHICS_QUEUE` | Allow graphics queue (AMD) | `0` |
| `GGML_VK_DISABLE_DEBUG` | Disable Vulkan debug output | `1` |

**Critical Gotcha (Vulkan ICD):**
Issue #1392: NVIDIA ICD files mounted at `/etc/vulkan/icd.d/` but applications expect `/usr/share/vulkan/icd.d/`.

**Workaround (add to Dockerfile in Phase 02):**
```dockerfile
RUN ln -s /etc/vulkan/icd.d /usr/share/vulkan/icd.d
```

### HuggingFace Token Handling

**Environment Variable:**
```bash
HUGGING_FACE_HUB_TOKEN=hf_xxx
```

**CRITICAL SECURITY CONSTRAINT:**
- **DO NOT USE BUILD ARGUMENTS** in Dockerfile (visible in docker history)
- **USE ENVIRONMENT VARIABLES** at runtime (dev) or Docker secrets (production)

**Examples:**
```bash
# Development: Pass as env var
docker run -e HUGGING_FACE_HUB_TOKEN=hf_xxx ghcr.io/ggml-org/llama.cpp:server-vulkan

# Production: Use Docker secrets
echo "hf_xxx" | docker secret create hf_token -
docker service create \
  --secret source=hf_token,target=hf_token \
  -e HUGGING_FACE_HUB_TOKEN_FILE=/run/secrets/hf_token \
  ghcr.io/ggml-org/llama.cpp:server-vulkan
```

### Model Loading and Formats

**Supported Formats:**
- GGUF (primary, fully supported)
- GGML (deprecated, legacy support)

**Model Search Path (from llama.cpp source):**
1. Explicit path via `-m` flag
2. Working directory
3. `~/.cache/llama.cpp/` (default cache location)

**Recommended Quantization: Q4_K_M**
- Size: ~4.2GB (7B parameter model)
- Quality: 96% of full precision (evaluated on MMLU)
- Speed: 1.5x faster than Q8_0
- VRAM: ~5.5GB

**Fast Small Models:**
- Llama 3.2 1B: 226 TPS, <1GB VRAM
- Qwen 2.5 1.5B: 180 TPS, ~1.5GB VRAM
- Gemma 3 1B: 200 TPS, <1GB VRAM
- Gemma 3 4B: 80 TPS, ~3GB VRAM

### AMD RX 580 (Polaris) GPU Profile

**Confirmed Working:** llama.cpp Vulkan with RADV driver
**Performance:** ~39 tok/s (7B Q4_K_M), ~226 tok/s (1B)

**Required Settings:**
```yaml
vulkan:
  visible_devices: "0"
  flash_attention: false  # CRITICAL: Disable for Polaris (llama.cpp issue #20465)
  disable_debug: true
```

**Critical Requirements:**
- **Driver:** MUST use RADV (NOT AMDVLK — 2GB allocation limit, llama.cpp issue #15054)
- **Flash Attention:** Disable (`-fa 0`) due to bug on Polaris
- **Memory Allocation:** `GGML_VK_FORCE_MAX_ALLOCATION_SIZE=2147483646` (set in Dockerfile)
- **Vulkan Version:** 1.4.335 on RX 580 Polaris10

**Recommended Config:**
```yaml
hardware:
  gpu_layers: 99  # Near-max offload for Polaris
vulkan:
  visible_devices: "0"
  flash_attention: false
  disable_debug: true
```


---

## Task 1: Design YAML Schema

### Schema Overview

This schema provides a translation layer: YAML config → `LLAMA_ARG_*` env vars → llama.cpp CLI flags.

### Complete YAML Schema

```yaml
# Complete example config.yml for PoC
model:
  # Local path OR HuggingFace download spec
  path: /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf
  # Optional: HuggingFace download (if path doesn't exist)
  huggingface:
    repo: meta-llama/Llama-3.2-1B-Instruct
    filename: Llama-3.2-1B-Instruct.Q4_K_M.gguf
    branch: main
    # SHA256 checksum for validation (optional, recommended for security)
    sha256: abc123def456...
  # Quantization format (for validation only)
  # Options: Q4_0, Q4_K, Q5_0, Q5_K, Q6_K, Q8_0, F16, F32
  quantization: Q4_K_M
  # Expected parameter count (for validation)
  parameter_count: 1000000000  # 1B parameters

context:
  # Token context window (default from llama.cpp: 2048)
  # Use 0 to get model default
  size: 4096  # 512-32768 (model-dependent)
  # Batch size (tokens per request)
  # Default from llama.cpp: 2048
  batch_size: 512  # 1-4096
  # Maximum context per slot (parallel processing)
  # CONSTRAINT: Must be <= context.size
  max_context_per_slot: 2048  # 1-context.size
  # Micro-batch size (default from llama.cpp: 512)
  # CONSTRAINT: For embeddings: batch_size <= ubatch_size
  ubatch_size: 512  # 1-4096

hardware:
  # CPU threads (default from llama.cpp: -1 = auto-detect)
  threads: 4  # 0-64
  # GPU layers to offload (default from llama.cpp: auto/all possible)
  # 0 = CPU only, 999 = all layers
  gpu_layers: 999  # 0-999
  # CPU memory map size (GB)
  mmap_size: 8  # 1-64
  # Use mmap for model loading (default from llama.cpp: true)
  use_mmap: true
  # Lock memory (prevent swap)
  lock_memory: false

sampling:
  # Sampling temperature (default from llama.cpp: 0.80)
  # 0.0 = greedy, 2.0 = very random
  temperature: 0.7  # 0.0-2.0
  # Top-P sampling (default from llama.cpp: 0.95)
  top_p: 0.95  # 0.0-1.0
  # Top-K sampling (default from llama.cpp: 40)
  # 0 = disabled
  top_k: 40  # 0-1000
  # Min-P sampling (default from llama.cpp: 0.05)
  min_p: 0.05  # 0.0-1.0
  # Typical-P sampling
  typical_p: 1.0  # 0.0-2.0
  # Repeat penalty (default from llama.cpp: 1.00)
  repeat_penalty: 1.1  # 0.0-2.0
  # Presence penalty
  presence_penalty: 0.0  # -2.0-2.0
  # Frequency penalty
  frequency_penalty: 0.0  # -2.0-2.0
  # Last N tokens to penalize repeats
  repeat_last_n: 64  # 0-2048
  # Seed (0 = random)
  seed: 42  # 0-4294967295
  # Maximum tokens to generate (default from llama.cpp: -1 = infinity)
  max_tokens: 512  # 1-context.size

server:
  # HTTP server host (default from llama.cpp: 127.0.0.1)
  # Use 0.0.0.0 to bind to all interfaces (development only)
  # SECURITY: Use 127.0.0.1 for production
  host: 127.0.0.1  # 0.0.0.0, 127.0.0.1
  # HTTP server port (default from llama.cpp: 8080)
  port: 8080  # 1-65535
  # Enable parallel processing / continuous batching
  # Set to "1" env var to enable
  parallel: true
  # Request timeout seconds (default from llama.cpp: 600)
  timeout: 300  # 1-3600
  # Maximum slots (parallel requests)
  # Controls concurrent request capacity
  max_slots: 8  # 1-32
  # Enable metrics endpoint (/metrics)
  metrics: true
  # Enable slots endpoint (/slots)
  slots_endpoint: true
  # CORS origins (comma-separated or "*")
  # SECURITY: Do NOT use "*" in production. Specify allowed origins.
  cors_origins: "http://localhost:8080"
  # API key for authentication (optional, recommended for production)
  # Pass via environment variable: --api-key
  api_key: null  # Set to string to enable
  # Access log file (optional)
  access_log: /var/log/llama-server/access.log

retry_config:
  # Retry configuration for model downloads and API requests
  # Request timeout (seconds)
  timeout_seconds: 300  # 1-3600
  # Maximum retry attempts
  max_retries: 3  # 0-10
  # Delay between retries (seconds)
  retry_delay_seconds: 5  # 1-60

cache:
  # KV cache type (key)
  # Options: f32, f16, q8_0, q4_0
  # Recommended: f16 (2x faster than f32)
  cache_type_k: f16
  # KV cache type (value)
  cache_type_v: f16
  # KV cache size (GB, 0 = auto)
  # Default from llama.cpp: 8192 MiB
  kv_cache_size: 4  # 0-64

docker:
  # Docker resource limits
  # Memory limit (GB, 0 = unlimited)
  memory_limit: 8  # 0-64
  # Shared memory size (GB, recommended for GPU workloads)
  # Default: 8GB for Vulkan operations
  shm_size: 8  # 0-32
  # CPU count (0 = unlimited, use host CPUs)
  cpu_count: 4  # 0-64

features:
  # Log verbosity
  # Maps to llama.cpp log level: 0=trace, 1=debug, 2=info, 3=warn, 4=error
  log_level: info  # trace, debug, info, warn, error
  # Enable profiling
  profiling: false
  # Print system info on startup
  print_system_info: true
  # Verbose output
  verbose: false
  # Enable colored output
  color: true

vulkan:
  # Visible GPUs (comma-separated)
  # Default: "0"
  visible_devices: "0"  # "0", "0,1", "0,1,2"
  # Force max GPU allocation (MB, 0 = auto)
  force_max_allocation: 0  # 0-65536
  # Disable Vulkan debug output (default from llama.cpp: true)
  disable_debug: true
  # Enable validation layers (debug)
  enable_validation: false
```

### Schema Validation Rules

**Type Constraints:**
- All numeric values must be within specified ranges
- Boolean values only where specified
- String enums must match allowed values

**Cross-Field Validation (CRITICAL):**
- `context.max_context_per_slot` ≤ `context.size`
- `context.batch_size` ≤ `context.size`
- `sampling.max_tokens` ≤ `context.size`
- **Embeddings constraint:** `context.batch_size` ≤ `context.ubatch_size`
- If `model.huggingface` provided, `model.path` should be `/models/{filename}`

**Required Fields:**
- `model.path` (required)
- `context.size` (optional, defaults to 2048)
- `hardware.threads` (optional, defaults to 4)
- `sampling.temperature` (optional, defaults to 0.80)

**Optional Fields:**
All other fields have defaults (documented above).

---

## Task 2: Define Rust Structs

### File Structure

Create: `src/config/mod.rs`

### Struct Definitions

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use std::path::PathBuf;

/// Complete llama.cpp configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LlamaConfig {
    /// Model configuration
    #[garde(skip)]
    pub model: ModelConfig,

    /// Context window configuration
    #[garde(skip)]
    pub context: ContextConfig,

    /// Hardware configuration
    #[garde(skip)]
    pub hardware: HardwareConfig,

    /// Sampling parameters
    #[garde(skip)]
    pub sampling: SamplingConfig,

    /// Server configuration
    #[garde(skip)]
    pub server: ServerConfig,

    /// KV cache configuration
    #[garde(skip)]
    pub cache: CacheConfig,

    /// Retry configuration
    #[garde(skip)]
    pub retry_config: RetryConfig,

    /// Docker resource limits
    #[garde(skip)]
    pub docker: DockerConfig,

    /// Feature flags
    #[garde(skip)]
    pub features: FeaturesConfig,

    /// Vulkan-specific configuration
    #[garde(skip)]
    pub vulkan: VulkanConfig,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ModelConfig {
    /// Local path to GGUF model file
    #[garde(nonempty)]
    pub path: PathBuf,

    /// HuggingFace download specification (optional)
    #[serde(default)]
    pub huggingface: Option<HuggingFaceConfig>,

    /// Quantization format (for validation)
    #[serde(default = "default_quantization")]
    pub quantization: String,

    /// Expected parameter count (for validation)
    #[serde(default)]
    pub parameter_count: Option<u64>,
}

/// HuggingFace download configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HuggingFaceConfig {
    /// HuggingFace repository (org/model)
    #[garde(nonempty)]
    pub repo: String,

    /// Filename in repository
    #[garde(nonempty)]
    pub filename: String,

    /// Git branch (default: main)
    #[serde(default = "default_branch")]
    pub branch: String,

    /// SHA256 checksum for validation (optional, recommended for security)
    #[serde(default)]
    pub sha256: Option<String>,
}

/// Context window configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ContextConfig {
    /// Token context size (default from llama.cpp: 2048)
    #[serde(default = "default_ctx_size")]
    #[garde(range(min = 512, max = 32768))]
    pub size: usize,

    /// Batch size (tokens per request, default from llama.cpp: 2048)
    #[serde(default = "default_batch_size")]
    #[garde(range(min = 1, max = 4096))]
    pub batch_size: usize,

    /// Maximum context per slot
    #[serde(default)]
    #[garde(range(min = 1))]
    pub max_context_per_slot: Option<usize>,

    /// Micro-batch size (default from llama.cpp: 512)
    /// CONSTRAINT: For embeddings: batch_size <= ubatch_size
    #[serde(default = "default_ubatch_size")]
    #[garde(range(min = 1, max = 4096))]
    pub ubatch_size: usize,
}

/// Hardware configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HardwareConfig {
    /// CPU threads (default from llama.cpp: -1 = auto-detect)
    #[serde(default = "default_threads")]
    #[garde(range(min = 0, max = 64))]
    pub threads: usize,

    /// GPU layers to offload (default from llama.cpp: auto)
    /// 0 = CPU only, 999 = all layers
    #[serde(default = "default_gpu_layers")]
    #[garde(range(min = 0, max = 999))]
    pub gpu_layers: usize,

    /// CPU memory map size (GB)
    #[serde(default = "default_mmap_size")]
    #[garde(range(min = 1, max = 64))]
    pub mmap_size: usize,

    /// Use mmap for model loading (default from llama.cpp: true)
    #[serde(default = "default_use_mmap")]
    pub use_mmap: bool,

    /// Lock memory (prevent swap)
    #[serde(default)]
    pub lock_memory: bool,
}

/// Sampling parameters
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SamplingConfig {
    /// Sampling temperature (default from llama.cpp: 0.80)
    #[serde(default = "default_temperature")]
    #[garde(range(min = 0.0, max = 2.0))]
    pub temperature: f32,

    /// Top-P sampling (default from llama.cpp: 0.95)
    #[serde(default = "default_top_p")]
    #[garde(range(min = 0.0, max = 1.0))]
    pub top_p: f32,

    /// Top-K sampling (default from llama.cpp: 40)
    #[serde(default = "default_top_k")]
    #[garde(range(min = 0, max = 1000))]
    pub top_k: usize,

    /// Min-P sampling (default from llama.cpp: 0.05)
    #[serde(default)]
    #[garde(range(min = 0.0, max = 1.0))]
    pub min_p: Option<f32>,

    /// Typical-P sampling
    #[serde(default = "default_typical_p")]
    #[garde(range(min = 0.0, max = 2.0))]
    pub typical_p: f32,

    /// Repeat penalty (default from llama.cpp: 1.00)
    #[serde(default = "default_repeat_penalty")]
    #[garde(range(min = 0.0, max = 2.0))]
    pub repeat_penalty: f32,

    /// Presence penalty
    #[serde(default)]
    #[garde(range(min = -2.0, max = 2.0))]
    pub presence_penalty: Option<f32>,

    /// Frequency penalty
    #[serde(default)]
    #[garde(range(min = -2.0, max = 2.0))]
    pub frequency_penalty: Option<f32>,

    /// Last N tokens to penalize repeats
    #[serde(default = "default_repeat_last_n")]
    #[garde(range(min = 0, max = 2048))]
    pub repeat_last_n: usize,

    /// Seed (0 = random)
    #[serde(default = "default_seed")]
    #[garde(range(min = 0, max = 4294967295))]
    pub seed: u32,

    /// Maximum tokens to generate (default from llama.cpp: -1)
    #[serde(default = "default_max_tokens")]
    #[garde(range(min = 1))]
    pub max_tokens: usize,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ServerConfig {
    /// HTTP server host (default from llama.cpp: 127.0.0.1)
    /// SECURITY: Use 127.0.0.1 for production, 0.0.0.0 only for development
    #[serde(default = "default_host")]
    pub host: String,

    /// HTTP server port (default from llama.cpp: 8080)
    #[serde(default = "default_port")]
    #[garde(range(min = 1, max = 65535))]
    pub port: u16,

    /// Enable parallel processing
    #[serde(default = "default_parallel")]
    pub parallel: bool,

    /// Request timeout (seconds, default from llama.cpp: 600)
    #[serde(default = "default_timeout")]
    #[garde(range(min = 1, max = 3600))]
    pub timeout: u64,

    /// Maximum slots (parallel requests)
    #[serde(default = "default_max_slots")]
    #[garde(range(min = 1, max = 32))]
    pub max_slots: usize,

    /// Enable metrics endpoint
    #[serde(default = "default_metrics")]
    pub metrics: bool,

    /// Enable slots endpoint
    #[serde(default = "default_slots_endpoint")]
    pub slots_endpoint: bool,

    /// CORS origins
    /// SECURITY: Do NOT use "*" in production. Specify allowed origins.
    #[serde(default = "default_cors_origins")]
    pub cors_origins: String,

    /// API key for authentication (optional, recommended for production)
    #[serde(default)]
    pub api_key: Option<String>,

    /// Access log file
    #[serde(default)]
    pub access_log: Option<PathBuf>,
}

/// KV cache configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CacheConfig {
    /// KV cache type (key)
    /// Recommended: f16 (2x faster than f32)
    #[serde(default = "default_cache_type")]
    pub cache_type_k: CacheType,

    /// KV cache type (value)
    #[serde(default = "default_cache_type")]
    pub cache_type_v: CacheType,

    /// KV cache size (GB, 0 = auto)
    /// Default from llama.cpp: 8192 MiB
    #[serde(default)]
    #[garde(range(min = 0, max = 64))]
    pub kv_cache_size: Option<usize>,
}

/// Retry configuration for model downloads and API requests
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RetryConfig {
    /// Request timeout (seconds)
    #[serde(default = "default_timeout_seconds")]
    #[garde(range(min = 1, max = 3600))]
    pub timeout_seconds: u64,

    /// Maximum retry attempts
    #[serde(default = "default_max_retries")]
    #[garde(range(min = 0, max = 10))]
    pub max_retries: u32,

    /// Delay between retries (seconds)
    #[serde(default = "default_retry_delay_seconds")]
    #[garde(range(min = 1, max = 60))]
    pub retry_delay_seconds: u64,
}

/// Docker resource limits
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DockerConfig {
    /// Memory limit (GB, 0 = unlimited)
    #[serde(default)]
    #[garde(range(min = 0, max = 64))]
    pub memory_limit: Option<usize>,

    /// Shared memory size (GB, recommended for GPU workloads)
    /// Default: 8GB for Vulkan operations
    #[serde(default = "default_shm_size")]
    #[garde(range(min = 0, max = 32))]
    pub shm_size: usize,

    /// CPU count (0 = unlimited, use host CPUs)
    #[serde(default)]
    #[garde(range(min = 0, max = 64))]
    pub cpu_count: Option<usize>,
}

/// Feature flags
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeaturesConfig {
    /// Log verbosity (0=trace, 1=debug, 2=info, 3=warn, 4=error)
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,

    /// Enable profiling
    #[serde(default)]
    pub profiling: bool,

    /// Print system info
    #[serde(default = "default_print_system_info")]
    pub print_system_info: bool,

    /// Verbose output
    #[serde(default)]
    pub verbose: bool,

    /// Colored output
    #[serde(default = "default_color")]
    pub color: bool,
}

/// Vulkan-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VulkanConfig {
    /// Visible GPUs (default: "0")
    #[serde(default = "default_visible_devices")]
    pub visible_devices: String,

    /// Force max GPU allocation (MB)
    #[serde(default)]
    #[garde(range(min = 0, max = 65536))]
    pub force_max_allocation: Option<usize>,

    /// Disable Vulkan debug (default from llama.cpp: true)
    #[serde(default = "default_disable_debug")]
    pub disable_debug: bool,

    /// Enable validation layers (debug)
    #[serde(default)]
    pub enable_validation: bool,

    /// Enable flash attention. Default: false.
    /// NOTE: Disable (false) for AMD Polaris GPUs due to llama.cpp issue #20465.
    #[serde(default)]
    #[garde(range(min = 0, max = 1))]
    pub flash_attention: Option<bool>,
}

// Enums

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheType {
    F32,
    F16,
    Q8_0,
    Q4_0,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// Defaults (matching llama.cpp upstream)

fn default_quantization() -> String {
    "Q4_K_M".to_string()
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_ctx_size() -> usize {
    2048  // llama.cpp default
}

fn default_batch_size() -> usize {
    2048  // llama.cpp default
}

fn default_ubatch_size() -> usize {
    512  // llama.cpp default
}

fn default_threads() -> usize {
    4  // Practical default (llama.cpp uses -1 for auto)
}

fn default_gpu_layers() -> usize {
    999  // All layers
}

fn default_mmap_size() -> usize {
    8
}

fn default_use_mmap() -> bool {
    true  // llama.cpp default
}

fn default_temperature() -> f32 {
    0.80  // llama.cpp default
}

fn default_top_p() -> f32 {
    0.95  // llama.cpp default
}

fn default_top_k() -> usize {
    40  // llama.cpp default
}

fn default_typical_p() -> f32 {
    1.0
}

fn default_repeat_penalty() -> f32 {
    1.00  // llama.cpp default
}

fn default_repeat_last_n() -> usize {
    64
}

fn default_seed() -> u32 {
    0  // Random (llama.cpp uses 0 for random)
}

fn default_max_tokens() -> usize {
    512
}

fn default_host() -> String {
    "127.0.0.1".to_string()  // llama.cpp default
}

fn default_port() -> u16 {
    8080  // llama.cpp default
}

fn default_parallel() -> bool {
    false  // llama.cpp default (set to true for performance)
}

fn default_timeout() -> u64 {
    600  // llama.cpp default (10 minutes)
}

fn default_max_slots() -> usize {
    8
}

fn default_metrics() -> bool {
    true
}

fn default_slots_endpoint() -> bool {
    true
}

fn default_cache_type() -> CacheType {
    CacheType::F16  // Recommended (2x faster than f32)
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

fn default_print_system_info() -> bool {
    true
}

fn default_color() -> bool {
    true
}

fn default_visible_devices() -> String {
    "0".to_string()  // llama.cpp default
}

fn default_disable_debug() -> bool {
    true  // llama.cpp default
}

fn default_cors_origins() -> String {
    "http://localhost:8080".to_string()  // Secure default (not "*")
}

fn default_timeout_seconds() -> u64 {
    300  // 5 minutes
}

fn default_max_retries() -> u32 {
    3
}

fn default_retry_delay_seconds() -> u64 {
    5
}

fn default_shm_size() -> usize {
    8  // 8GB for GPU workloads
}
```

### Validation Implementation

```rust
use anyhow::{Context, Result};

impl LlamaConfig {
    /// Validate config with cross-field constraints
    pub fn validate_cross_fields(&self) -> Result<()> {
        // Validate max_context_per_slot
        if let Some(max) = self.context.max_context_per_slot {
            if max > self.context.size {
                anyhow::bail!(
                    "max_context_per_slot ({}) cannot exceed context.size ({})",
                    max,
                    self.context.size
                );
            }
        }

        // Validate batch_size
        if self.context.batch_size > self.context.size {
            anyhow::bail!(
                "batch_size ({}) cannot exceed context.size ({})",
                self.context.batch_size,
                self.context.size
            );
        }

        // Validate ubatch_size (embeddings constraint)
        if self.context.batch_size > self.context.ubatch_size {
            anyhow::bail!(
                "batch_size ({}) cannot exceed ubatch_size ({}) - required for embeddings mode",
                self.context.batch_size,
                self.context.ubatch_size
            );
        }

        // Validate max_tokens
        if self.sampling.max_tokens > self.context.size {
            anyhow::bail!(
                "max_tokens ({}) cannot exceed context.size ({})",
                self.sampling.max_tokens,
                self.context.size
            );
        }

        // Validate HuggingFace path consistency
        if self.model.huggingface.is_some() {
            let path_str = self.model.path.to_string_lossy();
            if !path_str.starts_with("/models/") {
                anyhow::warn!(
                    "model.path ({}) should start with /models/ when using HuggingFace download",
                    path_str
                );
            }
        }

        Ok(())
    }

    /// Load config from file
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: LlamaConfig = serde_saphyr::from_str(&contents)
            .with_context(|| format!("Failed to parse YAML config: {}", path.display()))?;

        // Run garde validation
        config.validate(&garde::Default).with_context(|| {
            format!("Config validation failed: {}", path.display())
        })?;

        // Run cross-field validation
        config.validate_cross_fields()?;

        Ok(config)
    }

    /// Load config from file or search standard locations
    pub fn load() -> Result<Self> {
        // 1. Check explicit path from env var
        if let Ok(path) = std::env::var("LLAMA_CONFIG_PATH") {
            return Self::from_file(path);
        }

        // 2. Check current directory
        if std::path::Path::new("config.yml").exists() {
            return Self::from_file("config.yml");
        }

        // 3. Check ~/.config/llama/
        let xdg_path = dirs::config_dir()
            .context("Failed to determine XDG config directory")?
            .join("llama/config.yml");

        if xdg_path.exists() {
            return Self::from_file(xdg_path);
        }

        anyhow::bail!(
            "Config file not found. Tried: config.yml, $LLAMA_CONFIG_PATH, ~/.config/llama/config.yml"
        );
    }
}
```

---

## Task 3: Implement CLI Parser

### File Structure

Create: `src/cli.rs`

### CLI Definition

```rust
use clap::{Parser, Subcommand};
use crate::config::LlamaConfig;

#[derive(Debug, Parser)]
#[command(name = "whitt")]
#[command(about = "Whitt Execution Engine - Local LLM with Docker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Config file path (default: search standard locations)
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Validate configuration file
    ConfigValidate {
        /// Config file path
        path: String,
    },

    /// Translate YAML config to LLAMA_ARG_* environment variables
    ConfigTranslate {
        /// Config file path
        path: String,
        /// Output format (bash, json)
        #[arg(short, long, default_value = "bash")]
        format: String,
    },

    /// Generate example config file
    ConfigInit {
        /// Output path
        #[arg(short, long, default_value = "config.yml")]
        path: String,
        /// Model preset (1b, 7b, 13b)
        #[arg(short, long, default_value = "1b")]
        preset: String,
    },

    /// Start LLM server with Docker
    Start {
        /// Config file path
        #[arg(short, long)]
        config: Option<String>,

        /// Detach from terminal
        #[arg(short, long)]
        detach: bool,
    },

    /// Run inference (PoC client)
    Infer {
        /// Prompt text
        #[arg(short, long)]
        prompt: String,

        /// Stream output
        #[arg(short, long)]
        stream: bool,

        /// Server URL
        #[arg(short, long, default_value = "http://localhost:8080")]
        url: String,
    },
}

/// Load config from CLI args or standard locations
fn load_config(cli: &Cli) -> Result<LlamaConfig> {
    let path = if let Some(ref config_path) = cli.config {
        config_path.clone()
    } else {
        match &cli.command {
            Commands::ConfigValidate { path } => path.clone(),
            Commands::ConfigTranslate { path, .. } => path.clone(),
            _ => {
                // Try to find config in standard locations
                return LlamaConfig::load();
            }
        }
    };

    LlamaConfig::from_file(&path)
}
```

### Subcommand Implementations

```rust
impl Commands {
    pub fn execute(&self, cli: &Cli) -> Result<()> {
        match self {
            Commands::ConfigValidate { path } => {
                self.config_validate(path)
            }
            Commands::ConfigTranslate { path, format } => {
                self.config_translate(path, format)
            }
            Commands::ConfigInit { path, preset } => {
                self.config_init(path, preset)
            }
            Commands::Start { config, detach } => {
                self.start(config, *detach)
            }
            Commands::Infer { prompt, stream, url } => {
                self.infer(prompt, *stream, url)
            }
        }
    }

    fn config_validate(&self, path: &str) -> Result<()> {
        println!("Validating config: {}", path);
        let config = LlamaConfig::from_file(path)?;
        println!("✓ Config is valid");
        println!("  Model: {}", config.model.path.display());
        println!("  Context: {} tokens", config.context.size);
        println!("  GPU layers: {}", config.hardware.gpu_layers);
        Ok(())
    }

    fn config_translate(&self, path: &str, format: &str) -> Result<()> {
        let config = LlamaConfig::from_file(path)?;

        match format {
            "bash" => {
                println!("# Export LLAMA_ARG_* and GGML_VK_* environment variables");
                for (key, value) in config.to_env_vars() {
                    println!("export {}=\"{}\"", key, value);
                }
            }
            "json" => {
                let env_vars = config.to_env_vars();
                let json = serde_json::to_string_pretty(&env_vars)?;
                println!("{}", json);
            }
            _ => anyhow::bail!("Invalid format: {}. Use 'bash' or 'json'", format),
        }

        Ok(())
    }

    fn config_init(&self, path: &str, preset: &str) -> Result<()> {
        let config = match preset {
            "1b" => LlamaConfig::example_1b(),
            "7b" => LlamaConfig::example_7b(),
            "13b" => LlamaConfig::example_13b(),
            _ => anyhow::bail!("Invalid preset: {}. Use '1b', '7b', or '13b'", preset),
        };

        let yaml = serde_saphyr::to_string(&config)?;
        std::fs::write(path, yaml)
            .with_context(|| format!("Failed to write config to {}", path))?;

        println!("✓ Config written to: {}", path);
        Ok(())
    }

    fn start(&self, config: &Option<String>, detach: bool) -> Result<()> {
        // Implemented in Phase 05
        println!("Starting LLM server (Phase 05)");
        Ok(())
    }

    fn infer(&self, prompt: &str, stream: bool, url: &str) -> Result<()> {
        // Implemented in Phase 05
        println!("Running inference (Phase 05)");
        Ok(())
    }
}
```

### Example Configs

```rust
impl LlamaConfig {
    pub fn example_1b() -> Self {
        Self {
            model: ModelConfig {
                path: PathBuf::from("/models/Llama-3.2-1B-Instruct.Q4_K_M.gguf"),
                huggingface: Some(HuggingFaceConfig {
                    repo: "meta-llama/Llama-3.2-1B-Instruct".to_string(),
                    filename: "Llama-3.2-1B-Instruct.Q4_K_M.gguf".to_string(),
                    branch: "main".to_string(),
                    sha256: None,  // Set to string for checksum validation
                }),
                quantization: "Q4_K_M".to_string(),
                parameter_count: Some(1_000_000_000),
            },
            context: ContextConfig {
                size: 4096,
                batch_size: 2048,
                ubatch_size: 512,
                max_context_per_slot: None,
            },
            hardware: HardwareConfig {
                threads: 4,
                gpu_layers: 999,
                mmap_size: 4,
                use_mmap: true,
                lock_memory: false,
            },
            sampling: SamplingConfig {
                temperature: 0.7,
                top_p: 0.95,
                top_k: 40,
                min_p: Some(0.05),
                typical_p: 1.0,
                repeat_penalty: 1.1,
                presence_penalty: None,
                frequency_penalty: None,
                repeat_last_n: 64,
                seed: 0,
                max_tokens: 512,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),  // Secure default
                port: 8080,
                parallel: true,
                timeout: 600,
                max_slots: 8,
                metrics: true,
                slots_endpoint: true,
                cors_origins: "http://localhost:8080".to_string(),  // Secure default
                api_key: None,  // Set to string to enable authentication
                access_log: None,
            },
            cache: CacheConfig {
                cache_type_k: CacheType::F16,
                cache_type_v: CacheType::F16,
                kv_cache_size: Some(2),
            },
            retry_config: RetryConfig {
                timeout_seconds: 300,
                max_retries: 3,
                retry_delay_seconds: 5,
            },
            docker: DockerConfig {
                memory_limit: Some(8),
                shm_size: 8,
                cpu_count: Some(4),
            },
            features: FeaturesConfig {
                log_level: LogLevel::Info,
                profiling: false,
                print_system_info: true,
                verbose: false,
                color: true,
            },
            vulkan: VulkanConfig {
                visible_devices: "0".to_string(),
                force_max_allocation: None,
                disable_debug: true,
                enable_validation: false,
            },
        }
    }

    pub fn example_7b() -> Self {
        let mut config = Self::example_1b();
        config.model.path = PathBuf::from("/models/Llama-3.1-8B-Instruct.Q4_K_M.gguf");
        config.model.huggingface = Some(HuggingFaceConfig {
            repo: "meta-llama/Meta-Llama-3.1-8B-Instruct".to_string(),
            filename: "Meta-Llama-3.1-8B-Instruct.Q4_K_M.gguf".to_string(),
            branch: "main".to_string(),
            sha256: None,
        });
        config.model.quantization = "Q4_K_M".to_string();
        config.model.parameter_count = Some(8_000_000_000);
        config.context.size = 8192;
        config.hardware.mmap_size = 8;
        config.hardware.gpu_layers = 999;
        config.cache.kv_cache_size = Some(4);
        config
    }

    pub fn example_13b() -> Self {
        let mut config = Self::example_7b();
        config.model.path = PathBuf::from("/models/Llama-3.1-70B-Instruct.Q4_K_M.gguf");
        config.model.huggingface = Some(HuggingFaceConfig {
            repo: "meta-llama/Meta-Llama-3.1-70B-Instruct".to_string(),
            filename: "Meta-Llama-3.1-70B-Instruct.Q4_K_M.gguf".to_string(),
            branch: "main".to_string(),
            sha256: None,
        });
        config.model.quantization = "Q4_K_M".to_string();
        config.model.parameter_count = Some(70_000_000_000);
        config.context.size = 16384;
        config.hardware.mmap_size = 16;
        config.hardware.gpu_layers = 999;
        config.cache.kv_cache_size = Some(8);
        config
    }
}
```

---

## Task 4: Implement Environment Variable Translation

### Translation Logic

```rust
impl LlamaConfig {
    /// Translate config to LLAMA_ARG_* and GGML_VK_* environment variables
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut env_vars = Vec::new();

        // Model
        env_vars.push((
            "LLAMA_ARG_MODEL_PATH".to_string(),
            self.model.path.to_string_lossy().to_string(),
        ));

        // Context
        env_vars.push((
            "LLAMA_ARG_CTX_SIZE".to_string(),
            self.context.size.to_string(),
        ));

        // Batch size
        env_vars.push((
            "LLAMA_ARG_BATCH_SIZE".to_string(),
            self.context.batch_size.to_string(),
        ));

        // Micro-batch size
        env_vars.push((
            "LLAMA_ARG_UBATCH_SIZE".to_string(),
            self.context.ubatch_size.to_string(),
        ));

        // Hardware
        if self.hardware.threads > 0 {
            env_vars.push((
                "LLAMA_ARG_N_THREADS".to_string(),
                self.hardware.threads.to_string(),
            ));
        }
        if self.hardware.gpu_layers > 0 {
            env_vars.push((
                "LLAMA_ARG_N_GPU_LAYERS".to_string(),
                self.hardware.gpu_layers.to_string(),
            ));
        }
        if !self.hardware.use_mmap {
            env_vars.push((
                "LLAMA_ARG_USE_MMAP".to_string(),
                "0".to_string(),
            ));
        }
        if self.hardware.lock_memory {
            env_vars.push((
                "LLAMA_ARG_LOCK_MEMORY".to_string(),
                "1".to_string(),
            ));
        }

        // Sampling
        env_vars.push((
            "LLAMA_ARG_TEMP".to_string(),
            self.sampling.temperature.to_string(),
        ));
        env_vars.push((
            "LLAMA_ARG_TOP_P".to_string(),
            self.sampling.top_p.to_string(),
        ));
        if self.sampling.top_k > 0 {
            env_vars.push((
                "LLAMA_ARG_TOP_K".to_string(),
                self.sampling.top_k.to_string(),
            ));
        }
        if let Some(min_p) = self.sampling.min_p {
            env_vars.push((
                "LLAMA_ARG_MIN_P".to_string(),
                min_p.to_string(),
            ));
        }
        if self.sampling.typical_p != 1.0 {
            env_vars.push((
                "LLAMA_ARG_TYPICAL_P".to_string(),
                self.sampling.typical_p.to_string(),
            ));
        }
        if self.sampling.repeat_penalty != 1.0 {
            env_vars.push((
                "LLAMA_ARG_REPEAT_PENALTY".to_string(),
                self.sampling.repeat_penalty.to_string(),
            ));
        }
        if let Some(pp) = self.sampling.presence_penalty {
            env_vars.push((
                "LLAMA_ARG_PRESENCE_PENALTY".to_string(),
                pp.to_string(),
            ));
        }
        if let Some(fp) = self.sampling.frequency_penalty {
            env_vars.push((
                "LLAMA_ARG_FREQUENCY_PENALTY".to_string(),
                fp.to_string(),
            ));
        }
        if self.sampling.repeat_last_n > 0 {
            env_vars.push((
                "LLAMA_ARG_REPEAT_LAST_N".to_string(),
                self.sampling.repeat_last_n.to_string(),
            ));
        }
        if self.sampling.seed > 0 {
            env_vars.push((
                "LLAMA_ARG_SEED".to_string(),
                self.sampling.seed.to_string(),
            ));
        }
        env_vars.push((
            "LLAMA_ARG_N_PREDICT".to_string(),
            self.sampling.max_tokens.to_string(),
        ));

        // Server
        env_vars.push((
            "LLAMA_ARG_HOST".to_string(),
            self.server.host.clone(),
        ));
        env_vars.push((
            "LLAMA_ARG_PORT".to_string(),
            self.server.port.to_string(),
        ));
        if self.server.parallel {
            env_vars.push((
                "LLAMA_ARG_PARALLEL".to_string(),
                "1".to_string(),
            ));
        }
        env_vars.push((
            "LLAMA_ARG_TIMEOUT".to_string(),
            self.server.timeout.to_string(),
        ));
        if self.server.max_slots > 0 {
            env_vars.push((
                "LLAMA_ARG_N_SLOT".to_string(),
                self.server.max_slots.to_string(),
            ));
        }
        if self.server.metrics {
            env_vars.push((
                "LLAMA_ARG_METRICS".to_string(),
                "1".to_string(),
            ));
        }
        if self.server.slots_endpoint {
            env_vars.push((
                "LLAMA_ARG_SLOTS_ENDPOINT".to_string(),
                "1".to_string(),
            ));
        }

        // Cache
        let cache_type_str = |ct: &CacheType| match ct {
            CacheType::F32 => "f32",
            CacheType::F16 => "f16",
            CacheType::Q8_0 => "q8_0",
            CacheType::Q4_0 => "q4_0",
        };
        env_vars.push((
            "LLAMA_ARG_CACHE_TYPE_K".to_string(),
            cache_type_str(&self.cache.cache_type_k).to_string(),
        ));
        env_vars.push((
            "LLAMA_ARG_CACHE_TYPE_V".to_string(),
            cache_type_str(&self.cache.cache_type_v).to_string(),
        ));

        // Features
        let log_level_num = match self.features.log_level {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        };
        env_vars.push((
            "LLAMA_ARG_LOG_LEVEL".to_string(),
            log_level_num.to_string(),
        ));
        if self.features.profiling {
            env_vars.push((
                "LLAMA_ARG_PROFILING".to_string(),
                "1".to_string(),
            ));
        }
        if self.features.verbose {
            env_vars.push((
                "LLAMA_ARG_VERBOSE".to_string(),
                "1".to_string(),
            ));
        }

        // Vulkan (GGML_VK_* prefix, not LLAMA_ARG_*)
        env_vars.push((
            "GGML_VK_VISIBLE_DEVICES".to_string(),
            self.vulkan.visible_devices.clone(),
        ));
        if let Some(max_alloc) = self.vulkan.force_max_allocation {
            env_vars.push((
                "GGML_VK_FORCE_MAX_ALLOCATION_SIZE".to_string(),
                (max_alloc * 1024 * 1024).to_string(),
            ));
        }
        if self.vulkan.disable_debug {
            env_vars.push((
                "GGML_VK_DISABLE_DEBUG".to_string(),
                "1".to_string(),
            ));
        }
        if self.vulkan.enable_validation {
            env_vars.push((
                "GGML_VK_ENABLE_VALIDATION".to_string(),
                "1".to_string(),
            ));
        }

        env_vars
    }
}
```

---

## Task 5: Config File Discovery

### Discovery Logic

```rust
impl LlamaConfig {
    /// Discover config file in standard locations
    pub fn discover() -> Option<PathBuf> {
        // Priority order:
        // 1. LLAMA_CONFIG_PATH env var
        // 2. ./config.yml (current directory)
        // 3. ~/.config/llama/config.yml
        // 4. ~/.llama.yml (home directory)

        // 1. Environment variable
        if let Ok(path) = std::env::var("LLAMA_CONFIG_PATH") {
            let path = PathBuf::from(path);
            if path.exists() {
                return Some(path);
            }
        }

        // 2. Current directory
        let current_dir = PathBuf::from("config.yml");
        if current_dir.exists() {
            return Some(current_dir);
        }

        // 3. XDG config directory
        if let Some(xdg_config) = dirs::config_dir() {
            let xdg_path = xdg_config.join("llama/config.yml");
            if xdg_path.exists() {
                return Some(xdg_path);
            }
        }

        // 4. Home directory
        if let Some(home) = dirs::home_dir() {
            let home_path = home.join(".llama.yml");
            if home_path.exists() {
                return Some(home_path);
            }
        }

        None
    }
}
```

---

## Verification Steps

### Step 1: Verify YAML Parsing

**Test:** Create valid config and parse

```bash
# Generate example config
cargo run -- config init --preset 1b

# Validate config
cargo run -- config validate config.yml

# Expected output:
# ✓ Config is valid
#   Model: /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf
#   Context: 4096 tokens
#   GPU layers: 999
```

### Step 2: Verify Invalid Config Rejection

**Test:** Create invalid config and verify rejection

```bash
# Create invalid config (context.size too large)
cat > invalid.yml <<EOF
model:
  path: /models/test.gguf
context:
  size: 100000  # Invalid: exceeds max 32768
hardware:
  threads: 4
sampling:
  temperature: 0.7
server:
  host: 0.0.0.0
  port: 8080
cache:
  cache_type_k: f16
  cache_type_v: f16
features:
  log_level: info
vulkan:
  visible_devices: "0"
EOF

# Try to validate
cargo run -- config validate invalid.yml

# Expected: Error about context.size exceeding maximum
```

### Step 3: Verify Env Var Translation

**Test:** Translate config to env vars

```bash
# Translate to bash format
cargo run -- config translate config.yml --format bash

# Expected: Export statements like:
# export LLAMA_ARG_MODEL_PATH="/models/Llama-3.2-1B-Instruct.Q4_K_M.gguf"
# export LLAMA_ARG_CTX_SIZE="4096"
# export LLAMA_ARG_BATCH_SIZE="2048"
# export GGML_VK_VISIBLE_DEVICES="0"
# ...

# Translate to JSON format
cargo run -- config translate config.yml --format json | jq .

# Expected: JSON object with LLAMA_ARG_* and GGML_VK_* keys
```

### Step 4: Verify Cross-Field Validation

**Test:** Validate constraints between fields

```bash
# Create config with max_context_per_slot > context.size
cat > cross_invalid.yml <<EOF
model:
  path: /models/test.gguf
context:
  size: 2048
  max_context_per_slot: 4096  # Invalid: exceeds context.size
hardware:
  threads: 4
sampling:
  temperature: 0.7
  max_tokens: 1024
server:
  host: 0.0.0.0
  port: 8080
cache:
  cache_type_k: f16
  cache_type_v: f16
features:
  log_level: info
vulkan:
  visible_devices: "0"
EOF

# Try to validate
cargo run -- config validate cross_invalid.yml

# Expected: Error about max_context_per_slot exceeding context.size
```

### Step 5: Verify Embeddings Constraint

**Test:** Validate batch_size <= ubatch_size

```bash
# Create invalid config for embeddings
cat > embeddings_invalid.yml <<EOF
model:
  path: /models/test.gguf
context:
  size: 2048
  batch_size: 1024  # Invalid: exceeds ubatch_size
  ubatch_size: 512
hardware:
  threads: 4
sampling:
  temperature: 0.7
server:
  host: 0.0.0.0
  port: 8080
cache:
  cache_type_k: f16
  cache_type_v: f16
features:
  log_level: info
vulkan:
  visible_devices: "0"
EOF

# Try to validate
cargo run -- config validate embeddings_invalid.yml

# Expected: Error about batch_size exceeding ubatch_size (required for embeddings)
```

### Step 6: Verify Config Discovery

**Test:** Test config discovery in various locations

```bash
# Test 1: Current directory
cargo run -- config validate  # Should find ./config.yml

# Test 2: Environment variable
export LLAMA_CONFIG_PATH=/tmp/test_config.yml
cp config.yml /tmp/test_config.yml
cargo run -- config validate  # Should find /tmp/test_config.yml

# Test 3: XDG config directory
mkdir -p ~/.config/llama
cp config.yml ~/.config/llama/config.yml
unset LLAMA_CONFIG_PATH
rm config.yml
cargo run -- config validate  # Should find ~/.config/llama/config.yml

# Test 4: Not found
rm ~/.config/llama/config.yml
cargo run -- config validate 2>&1

# Expected: Error about config file not found
```

---

## Integration with Other Phases

### Phase 02: Docker Container

**Output:** No direct integration (Phase 02 uses base image)
**Input:** Phase 01 provides config structure for entrypoint to parse

### Phase 03: Config Injection

**Output:** YAML schema and validation
**Input:** Phase 03 mounts config.yml into container, uses translation logic

### Phase 04: HTTP Interface

**Output:** No direct integration (Phase 04 is documentation)
**Input:** No input from Phase 04

### Phase 05: Rust Client

**Output:** Config structs and CLI parser
**Input:** Phase 05 uses config to validate and translate env vars

---

## File Locations

Create/Modify:
- `src/config/mod.rs` - Config structs and validation
- `src/cli.rs` - CLI parser and subcommands
- `config.yml` (generated) - Example config files

---

## Success Criteria

- [ ] YAML schema validates all 8 config sections
- [ ] Rust structs compile with serde and garde
- [ ] CLI accepts subcommands: validate, translate, init
- [ ] Translation produces correct `LLAMA_ARG_*` and `GGML_VK_*` env vars
- [ ] Cross-field validation catches invalid combinations
- [ ] Embeddings constraint (batch_size <= ubatch_size) enforced
- [ ] Config discovery finds files in standard locations
- [ ] Invalid configs are rejected with clear error messages

---

## Next Steps

After completing Phase 01:
1. Proceed to Phase 02: Docker Container Build
2. Use config schema in entrypoint script (Phase 03)
3. Reuse config structs in Rust client (Phase 05)
