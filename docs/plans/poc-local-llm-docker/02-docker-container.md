# Phase 02: Docker Container Build + Auto-Install

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build optimized Docker container for llama.cpp with automatic model download, configuration loading, and health monitoring.

**Tech Stack:** Docker, Bash, llama.cpp (official image), huggingface-cli

**Dependencies:**
- Phase 02 is independent (no dependencies on other phases)
- Output used by Phase 03 (docker-compose) and Phase 04 (HTTP interface)

---

## Research Foundation: Official Docker Images

### Primary Image

**Image:** `ghcr.io/ggml-org/llama.cpp:server-vulkan`

**Base OS:** Ubuntu 26.04

**Build Configuration:**
```dockerfile
FROM ubuntu:26.04
RUN cmake -DGGML_VULKAN=ON ...
```

**Architecture:**
- Single-threaded context management
- Multi-threaded HTTP workers (default 4, controlled by `--parallel`)
- Queue-based task submission with automatic scheduling

**Includes:**
- llama-server binary
- llama-cli binary
- No model included (download on first run)

**Image Size Target:** <500MB final image

---

## Task 1: Design Multi-Stage Dockerfile

### Dockerfile Overview

**Strategy:** Multi-stage build to minimize final image size
- **Stage 1 (builder):** Install build tools, compile llama.cpp
- **Stage 2 (runtime):** Copy binaries, install runtime deps only

### Complete Dockerfile

Create: `docker/Dockerfile`

```dockerfile
# Stage 1: Base image with llama.cpp and Vulkan support
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan AS base

# Build arguments
ARG LLAMA_CPP_VERSION=b4450
ARG MODEL_REPO=""
ARG QUANTIZATION=Q4_K_M
# CRITICAL: DO NOT USE HF_TOKEN AS BUILD ARG (visible in docker history)
# HF_TOKEN must be passed as environment variable at runtime

# Metadata
LABEL maintainer="whitt-execution-engine"
LABEL description="Local LLM server with llama.cpp and Vulkan GPU support"
LABEL version="0.1.0"
LABEL llama.cpp.version="${LLAMA_CPP_VERSION}"

# Stage 2: Runtime environment
FROM ubuntu:26.04

# Environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Vulkan runtime (required for llama.cpp Vulkan support)
    libvulkan1 \
    mesa-vulkan-drivers \
    libglvnd0 \
    # OpenGL libraries
    libgl1 \
    libglx0 \
    libegl1 \
    libgles2 \
    # Python and pip for huggingface-cli
    python3 \
    python3-pip \
    # HTTP client for healthcheck
    curl \
    # Process management (for graceful shutdown)
    tini \
    # Utilities
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install huggingface-cli
RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Copy llama-server and llama-cli from base image
COPY --from=base /llama-server /usr/local/bin/llama-server
COPY --from=base /llama-cli /usr/local/bin/llama-cli

# Create directories
RUN mkdir -p /models \
    /config \
    /var/log/llama-server \
    /var/run/llama-server

# CRITICAL: Vulkan ICD loader symlink (fix for Issue #1392)
# NVIDIA ICD files mounted at /etc/vulkan/icd.d/ but applications expect /usr/share/vulkan/icd.d/
RUN ln -s /etc/vulkan/icd.d /usr/share/vulkan/icd.d

# Set working directory
WORKDIR /app

# Copy entrypoint script
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Expose HTTP server port (default from llama.cpp: 8080)
EXPOSE 8080

# Health check
# CRITICAL: curl must be in RUNTIME layer, not build layer
# WORKAROUND: Use /props instead of /health to avoid queueing under load (Issue #20684)
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

# Set up volumes
VOLUME ["/models", "/config", "/var/log/llama-server"]

# Entry point with tini for signal handling
ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

### Dockerfile Build Args

| Argument | Purpose | Default |
|----------|---------|---------|
| `LLAMA_CPP_VERSION` | llama.cpp commit tag | `b4450` |
| `MODEL_REPO` | Default HuggingFace repo | (empty) |
| `QUANTIZATION` | Default quantization format | `Q4_K_M` |

**CRITICAL:** `HF_TOKEN` is NOT a build arg (security constraint)

### Image Size Target

- **Target:** <500MB final image
- **Base image:** ~200MB (Ubuntu + llama.cpp)
- **Runtime deps:** ~150MB (Vulkan, Python, huggingface-cli, curl, tini)
- **Models:** NOT included (downloaded at runtime)

---

## Task 2: Implement Entrypoint Script

### Entrypoint Overview

**Responsibilities:**
1. Download model from HuggingFace (if not present)
2. Parse YAML config and translate to env vars
3. Validate Vulkan GPU availability
4. Start llama-server with appropriate flags
5. Handle graceful shutdown (SIGTERM)

### Complete Entrypoint Script

Create: `docker/entrypoint.sh`

```bash
#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to download model from HuggingFace
download_model() {
    local model_path="$1"
    local repo="$2"
    local filename="$3"
    local branch="${4:-main}"

    # If model already exists, skip download
    if [ -f "$model_path" ]; then
        log_info "Model already exists: $model_path"
        return 0
    fi

    log_info "Downloading model: $repo/$filename"
    log_info "This may take a while depending on your network speed..."

    # Create directory for model
    mkdir -p "$(dirname "$model_path")"

    # Use huggingface-cli for download (supports resume)
    if command -v huggingface-cli &> /dev/null; then
        # Check for HF_TOKEN in environment
        if [ -n "$HUGGING_FACE_HUB_TOKEN" ]; then
            export HUGGING_FACE_HUB_TOKEN
        elif [ -n "$HF_TOKEN" ]; then
            # Support legacy HF_TOKEN env var
            export HUGGING_FACE_HUB_TOKEN="$HF_TOKEN"
        fi

        huggingface-cli download \
            --repo-type model \
            --local-dir "$(dirname "$model_path")" \
            --local-dir-use-symlinks False \
            --resume-download \
            "$repo" \
            "$filename" \
            --revision "$branch"

        log_info "Model downloaded successfully: $model_path"
    else
        log_error "huggingface-cli not found. Cannot download model."
        exit 1
    fi
}

# Function to validate YAML config
validate_config() {
    local config_path="$1"

    if [ ! -f "$config_path" ]; then
        log_warn "Config file not found: $config_path. Using defaults."
        return 0
    fi

    log_info "Validating config: $config_path"

    # Check if yq is available (optional)
    if command -v yq &> /dev/null; then
        # Validate YAML syntax
        if ! yq eval '.' "$config_path" > /dev/null 2>&1; then
            log_error "Invalid YAML syntax in: $config_path"
            exit 1
        fi

        log_info "Config is valid"
    else
        log_warn "yq not found. Skipping detailed validation."
    fi
}

# Function to translate YAML config to environment variables
translate_config() {
    local config_path="$1"

    if [ ! -f "$config_path" ]; then
        log_warn "Config file not found. Using defaults."
        return 0
    fi

    log_info "Translating config to environment variables..."

    # Helper function to get YAML value
    get_yaml_value() {
        local path="$1"
        local default="$2"

        if command -v yq &> /dev/null; then
            value=$(yq eval "$path" "$config_path" 2>/dev/null || echo "$default")
            echo "$value"
        else
            # Fallback: use grep (less reliable)
            echo "$default"
        fi
    }

    # Model path (required)
    model_path=$(get_yaml_value ".model.path" "/models/model.gguf")
    export LLAMA_ARG_MODEL_PATH="$model_path"

    # Context size (default from llama.cpp: 2048)
    ctx_size=$(get_yaml_value ".context.size" "2048")
    export LLAMA_ARG_CTX_SIZE="$ctx_size"

    # Batch size (default from llama.cpp: 2048)
    batch_size=$(get_yaml_value ".context.batch_size" "2048")
    export LLAMA_ARG_BATCH_SIZE="$batch_size"

    # Micro-batch size (default from llama.cpp: 512)
    ubatch_size=$(get_yaml_value ".context.ubatch_size" "512")
    export LLAMA_ARG_UBATCH_SIZE="$ubatch_size"

    # CPU threads (default from llama.cpp: -1 = auto)
    n_threads=$(get_yaml_value ".hardware.threads" "4")
    if [ "$n_threads" -gt 0 ]; then
        export LLAMA_ARG_N_THREADS="$n_threads"
    fi

    # GPU layers (default from llama.cpp: auto)
    n_gpu_layers=$(get_yaml_value ".hardware.gpu_layers" "999")
    export LLAMA_ARG_N_GPU_LAYERS="$n_gpu_layers"

    # Use mmap (default from llama.cpp: true)
    use_mmap=$(get_yaml_value ".hardware.use_mmap" "true")
    if [ "$use_mmap" = "false" ]; then
        export LLAMA_ARG_USE_MMAP="0"
    fi

    # Lock memory
    lock_memory=$(get_yaml_value ".hardware.lock_memory" "false")
    if [ "$lock_memory" = "true" ]; then
        export LLAMA_ARG_LOCK_MEMORY="1"
    fi

    # Sampling temperature (default from llama.cpp: 0.80)
    temp=$(get_yaml_value ".sampling.temperature" "0.80")
    export LLAMA_ARG_TEMP="$temp"

    # Top-P (default from llama.cpp: 0.95)
    top_p=$(get_yaml_value ".sampling.top_p" "0.95")
    export LLAMA_ARG_TOP_P="$top_p"

    # Top-K (default from llama.cpp: 40)
    top_k=$(get_yaml_value ".sampling.top_k" "40")
    if [ "$top_k" -gt 0 ]; then
        export LLAMA_ARG_TOP_K="$top_k"
    fi

    # Min-P (default from llama.cpp: 0.05)
    min_p=$(get_yaml_value ".sampling.min_p" "0.05")
    if [ "$min_p" != "null" ] && [ "$min_p" != "" ]; then
        export LLAMA_ARG_MIN_P="$min_p"
    fi

    # Typical-P
    typical_p=$(get_yaml_value ".sampling.typical_p" "1.0")
    if [ "$typical_p" != "1.0" ]; then
        export LLAMA_ARG_TYPICAL_P="$typical_p"
    fi

    # Repeat penalty (default from llama.cpp: 1.00)
    repeat_penalty=$(get_yaml_value ".sampling.repeat_penalty" "1.00")
    if [ "$repeat_penalty" != "1.0" ]; then
        export LLAMA_ARG_REPEAT_PENALTY="$repeat_penalty"
    fi

    # Presence penalty
    presence_penalty=$(get_yaml_value ".sampling.presence_penalty" "null")
    if [ "$presence_penalty" != "null" ] && [ "$presence_penalty" != "" ]; then
        export LLAMA_ARG_PRESENCE_PENALTY="$presence_penalty"
    fi

    # Frequency penalty
    frequency_penalty=$(get_yaml_value ".sampling.frequency_penalty" "null")
    if [ "$frequency_penalty" != "null" ] && [ "$frequency_penalty" != "" ]; then
        export LLAMA_ARG_FREQUENCY_PENALTY="$frequency_penalty"
    fi

    # Repeat last N
    repeat_last_n=$(get_yaml_value ".sampling.repeat_last_n" "64")
    if [ "$repeat_last_n" -gt 0 ]; then
        export LLAMA_ARG_REPEAT_LAST_N="$repeat_last_n"
    fi

    # Seed
    seed=$(get_yaml_value ".sampling.seed" "0")
    if [ "$seed" -gt 0 ]; then
        export LLAMA_ARG_SEED="$seed"
    fi

    # Max tokens
    n_predict=$(get_yaml_value ".sampling.max_tokens" "512")
    export LLAMA_ARG_N_PREDICT="$n_predict"

    # Server host (default from llama.cpp: 127.0.0.1)
    host=$(get_yaml_value ".server.host" "0.0.0.0")
    export LLAMA_ARG_HOST="$host"

    # Server port (default from llama.cpp: 8080)
    port=$(get_yaml_value ".server.port" "8080")
    export LLAMA_ARG_PORT="$port"

    # Parallel processing
    parallel=$(get_yaml_value ".server.parallel" "false")
    if [ "$parallel" = "true" ]; then
        export LLAMA_ARG_PARALLEL="1"
    fi

    # Timeout (default from llama.cpp: 600)
    timeout=$(get_yaml_value ".server.timeout" "600")
    export LLAMA_ARG_TIMEOUT="$timeout"

    # Max slots
    max_slots=$(get_yaml_value ".server.max_slots" "8")
    if [ "$max_slots" -gt 0 ]; then
        export LLAMA_ARG_N_SLOT="$max_slots"
    fi

    # Metrics endpoint
    metrics=$(get_yaml_value ".server.metrics" "true")
    if [ "$metrics" = "true" ]; then
        export LLAMA_ARG_METRICS="1"
    fi

    # Slots endpoint
    slots_endpoint=$(get_yaml_value ".server.slots_endpoint" "true")
    if [ "$slots_endpoint" = "true" ]; then
        export LLAMA_ARG_SLOTS_ENDPOINT="1"
    fi

    # Cache type
    cache_type_k=$(get_yaml_value ".cache.cache_type_k" "f16")
    export LLAMA_ARG_CACHE_TYPE_K="$cache_type_k"

    cache_type_v=$(get_yaml_value ".cache.cache_type_v" "f16")
    export LLAMA_ARG_CACHE_TYPE_V="$cache_type_v"

    # Log level (0=trace, 1=debug, 2=info, 3=warn, 4=error)
    log_level=$(get_yaml_value ".features.log_level" "info")
    case "$log_level" in
        trace) export LLAMA_ARG_LOG_LEVEL="0" ;;
        debug) export LLAMA_ARG_LOG_LEVEL="1" ;;
        info) export LLAMA_ARG_LOG_LEVEL="2" ;;
        warn) export LLAMA_ARG_LOG_LEVEL="3" ;;
        error) export LLAMA_ARG_LOG_LEVEL="4" ;;
    esac

    # Profiling
    profiling=$(get_yaml_value ".features.profiling" "false")
    if [ "$profiling" = "true" ]; then
        export LLAMA_ARG_PROFILING="1"
    fi

    # Verbose
    verbose=$(get_yaml_value ".features.verbose" "false")
    if [ "$verbose" = "true" ]; then
        export LLAMA_ARG_VERBOSE="1"
    fi

    # Vulkan settings (GGML_VK_* prefix, not LLAMA_ARG_*)
    visible_devices=$(get_yaml_value ".vulkan.visible_devices" "0")
    export GGML_VK_VISIBLE_DEVICES="$visible_devices"

    force_max_allocation=$(get_yaml_value ".vulkan.force_max_allocation" "0")
    if [ "$force_max_allocation" != "0" ]; then
        # Convert MB to bytes
        export GGML_VK_FORCE_MAX_ALLOCATION_SIZE="$((force_max_allocation * 1024 * 1024))"
    fi

    disable_debug=$(get_yaml_value ".vulkan.disable_debug" "true")
    if [ "$disable_debug" = "true" ]; then
        export GGML_VK_DISABLE_DEBUG="1"
    fi

    enable_validation=$(get_yaml_value ".vulkan.enable_validation" "false")
    if [ "$enable_validation" = "true" ]; then
        export GGML_VK_ENABLE_VALIDATION="1"
    fi

    log_info "Config translation complete"
}

# Function to validate Vulkan GPU
validate_vulkan() {
    log_info "Validating Vulkan GPU..."

    if [ -z "$GGML_VK_VISIBLE_DEVICES" ]; then
        export GGML_VK_VISIBLE_DEVICES="0"
    fi

    # Try to run llama-cli to check Vulkan
    if command -v llama-cli &> /dev/null; then
        if ! llama-cli --help 2>&1 | grep -q "vulkan"; then
            log_warn "Vulkan support not found in llama-cli. Running in CPU-only mode."
            export LLAMA_ARG_N_GPU_LAYERS="0"
        else
            log_info "Vulkan support detected. GPU: $GGML_VK_VISIBLE_DEVICES"
        fi
    else
        log_warn "llama-cli not found. Skipping Vulkan validation."
    fi
}

# Function to start llama-server
start_server() {
    log_info "Starting llama-server..."

    # Construct command arguments
    local server_args=""

    # Model path (required)
    if [ -z "$LLAMA_ARG_MODEL_PATH" ]; then
        log_error "Model path not set. Set LLAMA_ARG_MODEL_PATH or provide config."
        exit 1
    fi
    server_args="$server_args -m $LLAMA_ARG_MODEL_PATH"

    # Context size
    if [ -n "$LLAMA_ARG_CTX_SIZE" ]; then
        server_args="$server_args -c $LLAMA_ARG_CTX_SIZE"
    fi

    # Batch size
    if [ -n "$LLAMA_ARG_BATCH_SIZE" ]; then
        server_args="$server_args -b $LLAMA_ARG_BATCH_SIZE"
    fi

    # Micro-batch size
    if [ -n "$LLAMA_ARG_UBATCH_SIZE" ]; then
        server_args="$server_args -ub $LLAMA_ARG_UBATCH_SIZE"
    fi

    # CPU threads
    if [ -n "$LLAMA_ARG_N_THREADS" ] && [ "$LLAMA_ARG_N_THREADS" != "0" ]; then
        server_args="$server_args -t $LLAMA_ARG_N_THREADS"
    fi

    # GPU layers
    if [ -n "$LLAMA_ARG_N_GPU_LAYERS" ] && [ "$LLAMA_ARG_N_GPU_LAYERS" != "0" ]; then
        server_args="$server_args -ngl $LLAMA_ARG_N_GPU_LAYERS"
    fi

    # Sampling parameters
    if [ -n "$LLAMA_ARG_TEMP" ]; then
        server_args="$server_args --temp $LLAMA_ARG_TEMP"
    fi

    if [ -n "$LLAMA_ARG_TOP_P" ]; then
        server_args="$server_args --top-p $LLAMA_ARG_TOP_P"
    fi

    if [ -n "$LLAMA_ARG_TOP_K" ]; then
        server_args="$server_args --top-k $LLAMA_ARG_TOP_K"
    fi

    if [ -n "$LLAMA_ARG_MIN_P" ]; then
        server_args="$server_args --min-p $LLAMA_ARG_MIN_P"
    fi

    if [ -n "$LLAMA_ARG_TYPICAL_P" ]; then
        server_args="$server_args --typical-p $LLAMA_ARG_TYPICAL_P"
    fi

    if [ -n "$LLAMA_ARG_REPEAT_PENALTY" ]; then
        server_args="$server_args --repeat-penalty $LLAMA_ARG_REPEAT_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_PRESENCE_PENALTY" ]; then
        server_args="$server_args --presence-penalty $LLAMA_ARG_PRESENCE_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_FREQUENCY_PENALTY" ]; then
        server_args="$server_args --frequency-penalty $LLAMA_ARG_FREQUENCY_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_REPEAT_LAST_N" ]; then
        server_args="$server_args --repeat-last-n $LLAMA_ARG_REPEAT_LAST_N"
    fi

    if [ -n "$LLAMA_ARG_SEED" ]; then
        server_args="$server_args --seed $LLAMA_ARG_SEED"
    fi

    if [ -n "$LLAMA_ARG_N_PREDICT" ]; then
        server_args="$server_args -n $LLAMA_ARG_N_PREDICT"
    fi

    # Server parameters
    if [ -n "$LLAMA_ARG_HOST" ]; then
        server_args="$server_args --host $LLAMA_ARG_HOST"
    fi

    if [ -n "$LLAMA_ARG_PORT" ]; then
        server_args="$server_args --port $LLAMA_ARG_PORT"
    fi

    if [ -n "$LLAMA_ARG_PARALLEL" ]; then
        server_args="$server_args --parallel"
    fi

    if [ -n "$LLAMA_ARG_TIMEOUT" ]; then
        server_args="$server_args --timeout $LLAMA_ARG_TIMEOUT"
    fi

    if [ -n "$LLAMA_ARG_N_SLOT" ]; then
        server_args="$server_args -n-slot $LLAMA_ARG_N_SLOT"
    fi

    # Cache parameters
    if [ -n "$LLAMA_ARG_CACHE_TYPE_K" ]; then
        server_args="$server_args --cache-type-k $LLAMA_ARG_CACHE_TYPE_K"
    fi

    if [ -n "$LLAMA_ARG_CACHE_TYPE_V" ]; then
        server_args="$server_args --cache-type-v $LLAMA_ARG_CACHE_TYPE_V"
    fi

    # Log level
    if [ -n "$LLAMA_ARG_LOG_LEVEL" ]; then
        server_args="$server_args --log-level $LLAMA_ARG_LOG_LEVEL"
    fi

    # Metrics
    if [ -n "$LLAMA_ARG_METRICS" ]; then
        server_args="$server_args --metrics"
    fi

    # Slots endpoint
    if [ -n "$LLAMA_ARG_SLOTS_ENDPOINT" ]; then
        server_args="$server_args --slots"
    fi

    # Profiling
    if [ -n "$LLAMA_ARG_PROFILING" ]; then
        server_args="$server_args --profiling"
    fi

    # Verbose
    if [ -n "$LLAMA_ARG_VERBOSE" ]; then
        server_args="$server_args --verbose"
    fi

    log_info "Server arguments: $server_args"

    # Start server with tini (already in ENTRYPOINT)
    exec /usr/local/bin/llama-server $server_args
}

# Graceful shutdown handler
shutdown_handler() {
    log_info "Received shutdown signal. Stopping server gracefully..."

    # Unload model via API if available
    if [ -n "$LLAMA_ARG_HOST" ] && [ -n "$LLAMA_ARG_PORT" ]; then
        local server_url="http://$LLAMA_ARG_HOST:$LLAMA_ARG_PORT"
        curl -X POST "$server_url/models/unload" 2>/dev/null || true
    fi

    log_info "Server stopped. Exiting."
    exit 0
}

# Register signal handlers
trap shutdown_handler SIGTERM SIGINT

# Main entry point
main() {
    log_info "Whitt LLM Server Entrypoint"
    log_info "================================"

    # Config path
    local config_path="/config/config.yml"

    # If no config provided, try to use default
    if [ ! -f "$config_path" ] && [ ! -f "/config/config.yaml" ]; then
        log_warn "No config file found in /config. Using defaults."
    elif [ ! -f "$config_path" ] && [ -f "/config/config.yaml" ]; then
        config_path="/config/config.yaml"
    fi

    # Validate config
    validate_config "$config_path"

    # Translate config to env vars
    translate_config "$config_path"

    # Download model if specified in config
    if command -v yq &> /dev/null; then
        repo=$(yq eval '.model.huggingface.repo // ""' "$config_path" 2>/dev/null || echo "")
        filename=$(yq eval '.model.huggingface.filename // ""' "$config_path" 2>/dev/null || echo "")
        branch=$(yq eval '.model.huggingface.branch // "main"' "$config_path" 2>/dev/null || echo "main")

        if [ -n "$repo" ] && [ -n "$filename" ]; then
            download_model "$LLAMA_ARG_MODEL_PATH" "$repo" "$filename" "$branch"
        fi
    fi

    # Check if model exists
    if [ ! -f "$LLAMA_ARG_MODEL_PATH" ]; then
        log_error "Model not found: $LLAMA_ARG_MODEL_PATH"
        log_error "Please provide a valid model path or configure HuggingFace download."
        exit 1
    fi

    # Validate Vulkan GPU
    validate_vulkan

    # Start server
    start_server
}

# Run main function
main "$@"
```

---

## Task 3: Define .dockerignore

### .dockerignore Contents

Create: `docker/.dockerignore`

```
# Git files
.git
.gitignore
.gitattributes

# Documentation
*.md
docs/
README*

# Test files
tests/
*.test.*
test_*
*_test.*

# CI/CD
.github/
.gitlab-ci.yml
.travis.yml

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# Build artifacts
target/
*.o
*.a
*.so
*.dylib
*.dll

# Python
__pycache__/
*.py[cod]
*$py.class
venv/
env/
.venv

# Node
node_modules/
npm-debug.log

# OS files
.DS_Store
Thumbs.db
*.log

# Config files (not needed in image)
config.yml
config.yaml
.env
.env.*

# Model files (downloaded at runtime)
*.gguf
*.ggml
models/
```

---

## Task 4: Define Build Script

### Build Script Overview

**Purpose:** Automate Docker build with version tagging and multi-arch support

### Complete Build Script

Create: `docker/build.sh`

```bash
#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Default values
LLAMA_CPP_VERSION="${LLAMA_CPP_VERSION:-b4450}"
IMAGE_NAME="${IMAGE_NAME:-whitt-execution-engine/llama-server}"
TAG="${TAG:-latest}"
PUSH="${PUSH:-false}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            LLAMA_CPP_VERSION="$2"
            shift 2
            ;;
        --tag)
            TAG="$2"
            shift 2
            ;;
        --name)
            IMAGE_NAME="$2"
            shift 2
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --version VERSION    llama.cpp version (default: b4450)"
            echo "  --tag TAG            Image tag (default: latest)"
            echo "  --name NAME          Image name (default: whitt-execution-engine/llama-server)"
            echo "  --push               Push image to registry"
            echo "  --help               Show this help message"
            exit 0
            ;;
        *)
            log_warn "Unknown option: $1"
            shift
            ;;
    esac
done

FULL_IMAGE_NAME="${IMAGE_NAME}:${TAG}"

log_info "Building Docker image..."
log_info "  llama.cpp version: ${LLAMA_CPP_VERSION}"
log_info "  Image name: ${FULL_IMAGE_NAME}"

# Build image
docker build \
    --build-arg LLAMA_CPP_VERSION="${LLAMA_CPP_VERSION}" \
    -t "${FULL_IMAGE_NAME}" \
    -f docker/Dockerfile \
    .

log_info "Build complete: ${FULL_IMAGE_NAME}"

# Display image size
IMAGE_SIZE=$(docker images "${FULL_IMAGE_NAME}" --format "{{.Size}}")
log_info "Image size: ${IMAGE_SIZE}"

# Verify image size < 500MB
SIZE_BYTES=$(docker images "${FULL_IMAGE_NAME}" --format "{{.Size}}")
if [[ "$SIZE_BYTES" =~ .*GB ]]; then
    SIZE_MB=$(echo "$SIZE_BYTES" | sed 's/GB//')
    SIZE_MB_INT=$(echo "$SIZE_MB * 1024" | bc | cut -d'.' -f1)
    if [ "$SIZE_MB_INT" -gt 500 ]; then
        log_warn "Image size exceeds 500MB target: $SIZE_BYTES"
    fi
fi

# Push if requested
if [ "$PUSH" = "true" ]; then
    log_info "Pushing image to registry..."
    docker push "${FULL_IMAGE_NAME}"
    log_info "Push complete"
fi
```

---

## Verification Steps

### Step 1: Build Docker Image

**Test:** Build image and verify size

```bash
# Build image
./docker/build.sh --version b4450 --tag v0.1.0

# Expected output:
# [INFO] Building Docker image...
# [INFO]   llama.cpp version: b4450
# [INFO]   Image name: whitt-execution-engine/llama-server:v0.1.0
# ...
# [INFO] Build complete: whitt-execution-engine/llama-server:v0.1.0
# [INFO] Image size: < 500MB

# Verify image exists
docker images whitt-execution-engine/llama-server

# Check image size is < 500MB
docker images whitt-execution-engine/llama-server --format "{{.Size}}"

# Expected: 450-500MB
```

### Step 2: Start Container with Default Config

**Test:** Start container without config (default behavior)

```bash
# Start container
docker run --rm \
    --name llama-server-test \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    whitt-execution-engine/llama-server:v0.1.0

# Expected output:
# [INFO] Whitt LLM Server Entrypoint
# [INFO] ================================
# [INFO] Config file not found in /config. Using defaults.
# [INFO] Validating Vulkan GPU...
# [INFO] Vulkan support detected. GPU: 0
# [INFO] Server arguments: -m /models/model.gguf -c 2048 ...
# [INFO] Starting llama-server...
```

### Step 3: Start Container with Config File

**Test:** Start container with YAML config

```bash
# Create test config
cat > test_config.yml <<EOF
model:
  path: /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf
  huggingface:
    repo: meta-llama/Llama-3.2-1B-Instruct
    filename: Llama-3.2-1B-Instruct.Q4_K_M.gguf
    branch: main
  quantization: Q4_K_M
  parameter_count: 1000000000
context:
  size: 4096
  batch_size: 2048
  ubatch_size: 512
hardware:
  threads: 4
  gpu_layers: 999
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
  parallel: true
  timeout: 600
  max_slots: 8
  metrics: true
cache:
  cache_type_k: f16
  cache_type_v: f16
features:
  log_level: info
vulkan:
  visible_devices: "0"
  disable_debug: true
EOF

# Start container with config
docker run --rm \
    --name llama-server-test \
    -v $(pwd)/test_config.yml:/config/config.yml \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    whitt-execution-engine/llama-server:v0.1.0

# Expected: Model downloads (if not present), server starts with config values
```

### Step 4: Verify Healthcheck

**Test:** Check health endpoint

```bash
# In another terminal, wait for server to start, then:
curl http://localhost:8080/props

# Expected: 200 OK with JSON response
# Verify model properties returned

# Check Docker health status
docker ps --filter name=llama-server-test --format "{{.Status}}"

# Expected: "Up X seconds (healthy)"
```

### Step 5: Verify Model Download

**Test:** Automatic model download from HuggingFace

```bash
# Remove model if exists
rm -f models/Llama-3.2-1B-Instruct.Q4_K_M.gguf

# Start container with config (will trigger download)
docker run --rm \
    --name llama-server-test \
    -v $(pwd)/test_config.yml:/config/config.yml \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    -e HUGGING_FACE_HUB_TOKEN=hf_xxx \
    whitt-execution-engine/llama-server:v0.1.0

# Expected output:
# [INFO] Downloading model: meta-llama/Llama-3.2-1B-Instruct/Llama-3.2-1B-Instruct.Q4_K_M.gguf
# [INFO] This may take a while depending on your network speed...
# [INFO] Model downloaded successfully: /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf

# Verify model file exists
ls -lh models/Llama-3.2-1B-Instruct.Q4_K_M.gguf

# Expected: ~600-800MB file
```

### Step 6: Verify Graceful Shutdown

**Test:** SIGTERM handling

```bash
# Start container in background
docker run -d \
    --name llama-server-test \
    -v $(pwd)/test_config.yml:/config/config.yml \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    whitt-execution-engine/llama-server:v0.1.0

# Wait for container to be healthy
sleep 30

# Send SIGTERM
docker stop llama-server-test

# Check logs
docker logs llama-server-test

# Expected:
# [INFO] Received shutdown signal. Stopping server gracefully...
# [INFO] Server stopped. Exiting.

# Verify container stopped
docker ps -a --filter name=llama-server-test --format "{{.Status}}"

# Expected: "Exited (0) X seconds ago"
```

### Step 7: Verify GPU Passthrough (AMD)

**Test:** AMD GPU access

```bash
# Start container with AMD GPU
docker run --rm \
    --name llama-server-test \
    --device /dev/dri:/dev/dri \
    --group-add video \
    --device /dev/kfd \
    -v $(pwd)/test_config.yml:/config/config.yml \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    whitt-execution-engine/llama-server:v0.1.0

# Check logs for GPU detection
docker logs llama-server-test | grep -i "vulkan\|gpu"

# Expected: "Vulkan support detected. GPU: 0"
```

### Step 8: Verify GPU Passthrough (NVIDIA)

**Test:** NVIDIA GPU access

```bash
# Start container with NVIDIA GPU (requires nvidia-container-toolkit)
docker run --rm \
    --name llama-server-test \
    --gpus all \
    --runtime=nvidia \
    -v $(pwd)/test_config.yml:/config/config.yml \
    -v $(pwd)/models:/models \
    -p 8080:8080 \
    whitt-execution-engine/llama-server:v0.1.0

# Check logs for GPU detection
docker logs llama-server-test | grep -i "cuda\|gpu"

# Expected: GPU detection message
```

### Step 9: Verify Vulkan ICD Symlink

**Test:** Check ICD loader path fix

```bash
# Start container (any config)
docker run --rm \
    --name llama-server-test \
    -v $(pwd)/models:/models \
    whitt-execution-engine/llama-server:v0.1.0 \
    ls -la /usr/share/vulkan/icd.d

# Expected: Symlink to /etc/vulkan/icd.d
# Output should show: icd.d -> /etc/vulkan/icd.d
```

---

## Integration with Other Phases

### Phase 01: YAML Config Schema

**Output:** YAML schema definition
**Input:** Entrypoint script parses YAML files defined in Phase 01

### Phase 03: Config Injection

**Output:** Docker image with entrypoint
**Input:** Phase 03 uses image in docker-compose.yml

### Phase 04: HTTP Interface

**Output:** No direct integration
**Input:** No input from Phase 04

### Phase 05: Rust Client

**Output:** Docker image
**Input:** Phase 05 starts container via docker-compose

---

## File Locations

Create:
- `docker/Dockerfile` - Multi-stage Dockerfile
- `docker/entrypoint.sh` - Entrypoint script
- `docker/.dockerignore` - Build context exclusions
- `docker/build.sh` - Build automation script

---

## Success Criteria

- [ ] Docker image builds successfully
- [ ] Image size <500MB
- [ ] Container starts with default config
- [ ] Container starts with custom YAML config
- [ ] Model downloads automatically from HuggingFace
- [ ] Healthcheck passes (/props returns 200)
- [ ] GPU passthrough works (both AMD and NVIDIA)
- [ ] Graceful shutdown on SIGTERM
- [ ] Config translation produces correct env vars
- [ ] Vulkan validation works
- [ ] Vulkan ICD symlink fix in place
- [ ] HF_TOKEN can be passed via env var (not build arg)

---

## Next Steps

After completing Phase 02:
1. Proceed to Phase 03: Config Injection (docker-compose)
2. Use image in Phase 05: Rust Client
3. Reference HTTP interface in Phase 04 for API details
