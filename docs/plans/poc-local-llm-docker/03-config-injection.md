# Phase 03: Config Injection into Docker

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement configuration injection system that mounts YAML config into container, translates to environment variables, and manages model switching without container rebuild.

**Tech Stack:** Docker Compose, Bash, Environment Variables

**Dependencies:**
- Phase 03 depends on Phase 01 (YAML schema)
- Phase 03 depends on Phase 02 (Docker container)
- Phase 03 is used by Phase 05 (Rust client)

---

## Research Foundation: Docker Compose and GPU Passthrough

### GPU Vendor Configuration

**AMD GPUs:**
```bash
--device=/dev/dri:/dev/dri
--group-add video
--device=/dev/kfd
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
  - `nvidia-container-toolkit` 1.17.4+ installed on host (REQUIRED for CVE-2025-23266/23359 fixes)
  - `nvidia-docker` version 2.0+
  - NVIDIA driver >= 570.123.10+
- **Toolkit Installation:**
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

**AMD GPUs:**
```bash
--gpus all  # Requires AMD Container Toolkit 1.2.0+ (Docker 25.0+)
```
- **Prerequisites:**
  - `amdgpu-container-toolkit` 1.2.0+ installed on host
  - Docker 25.0+ required for --gpus support
  - ROCm drivers installed on host
- **Toolkit Installation:**
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

**Intel GPUs:**
```bash
--device=/dev/dri
--group-add video
--group-add render
```
- **Recommendation:** SYCL preferred over Vulkan for Intel GPUs (better performance)
- **Prerequisites:** Intel oneAPI Base Toolkit, Intel GPU drivers (latest)

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
**Note:** Indices match GPU order from `vulkaninfo`

### Vulkan Environment Variables for Docker Compose

**Device Selection:**
- `GGML_VK_VISIBLE_DEVICES=0,1,2`: GPU indices to use

**Memory Management:**
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE=1073741824`: Max allocation in bytes (1GB example)
- `GGML_VK_DISABLE_COOPMAT=1`: Disable cooperative matrices

**AMD-Specific:**
- `GGML_VK_ALLOW_GRAPHICS_QUEUE=1`: Allow graphics queue usage (AMD)

**Debugging:**
- `VK_LAYER_PATH=/path/to/layers`
- `VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation`

### HuggingFace Token Security

**Environment Variable (Development):**
```bash
-e HUGGING_FACE_HUB_TOKEN=hf_xxx
```

**Docker Secrets (Production):**
```bash
echo "hf_xxx" | docker secret create hf_token -

docker service create \
    --secret source=hf_token,target=hf_token \
    -e HUGGING_FACE_HUB_TOKEN_FILE=/run/secrets/hf_token \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**CRITICAL:** NEVER use build arguments for HF_TOKEN (visible in docker history)

---

## Task 1: Design Docker Compose File

### Docker Compose Overview

**Purpose:** Define complete stack with:
- LLM server service
- Config volume mount
- Model volume mount
- GPU passthrough (AMD or NVIDIA)
- Network configuration
- Environment variable overrides

### Complete docker-compose.yml

Create: `docker-compose.yml`

```yaml
version: '3.9'

services:
  llama-server:
    image: whitt-execution-engine/llama-server:latest
    container_name: whitt-llama-server
    restart: unless-stopped

    # Config volume mount
    volumes:
      - ./config.yml:/config/config.yml:ro
      - ./models:/models:ro  # Read-only for security
      - ./logs:/var/log/llama-server

    # Port mapping (default from llama.cpp: 8080)
    # SECURITY: Bind to 127.0.0.1 for production, 0.0.0.0 only for development
    ports:
      - "127.0.0.1:8080:8080"

    # Network mode
    # host mode for lowest latency (5-10μs vs 50-100μs bridge)
    network_mode: host

    # Shared memory for GPU workloads (8GB recommended)
    shm_size: 8g

    # Run as non-root user (uid 1000 from Dockerfile)
    user: "1000:1000"

    # Environment variable overrides (optional)
    # These override config file settings if set
    environment:
      # Override model path if needed
      # LLAMA_ARG_MODEL_PATH: "/models/custom-model.gguf"

      # Override context size
      # LLAMA_ARG_CTX_SIZE: "8192"

      # Override GPU layers
      # LLAMA_ARG_N_GPU_LAYERS: "999"

      # Override sampling temperature
      # LLAMA_ARG_TEMP: "0.8"

      # Override server port
      # LLAMA_ARG_PORT: "8080"

      # HuggingFace token for private models
      # NEVER use build args - use env vars or secrets
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}

      # Vulkan GPU selection
      # GGML_VK_VISIBLE_DEVICES: "0"

      # Force max GPU allocation (MB)
      # GGML_VK_FORCE_MAX_ALLOCATION_SIZE: "0"

      # Disable Vulkan debug (default: true)
      # GGML_VK_DISABLE_DEBUG: "1"

    # GPU passthrough - NVIDIA (uncomment for NVIDIA GPUs)
    # deploy:
    #   resources:
    #     reservations:
    #       devices:
    #         - driver: nvidia
    #           count: all
    #           capabilities: [gpu]

    # GPU passthrough - AMD (uncomment for AMD GPUs)
    # devices:
    #   - /dev/dri:/dev/dri
    #   - /dev/kfd:/dev/kfd
    # group_add:
    #   - video

    # Network mode (bridge is default)
    # network_mode: bridge

    # Health check
    # Note: Uses /props instead of /health to avoid queueing under load (Issue #20684)
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 60s

    # Logging
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

    # Resource limits (optional)
    # mem_limit: 8g
    # mem_reservation: 4g
    # cpus: '4.0'
    # pids_limit: 1000

    # Dependency on model volume (optional)
    # depends_on:
    #   - model-loader

  # Optional: Model loader service (for pre-fetching models)
  # model-loader:
  #   image: curlimages/curl:latest
  #   container_name: whitt-model-loader
  #   volumes:
  #     - ./models:/models
  #   environment:
  #     MODEL_REPO: "meta-llama/Llama-3.2-1B-Instruct"
  #     MODEL_FILE: "Llama-3.2-1B-Instruct.Q4_K_M.gguf"
  #     HF_TOKEN: ${HF_TOKEN}
  #   command: >
  #     sh -c "
  #       if [ ! -f /models/$$MODEL_FILE ]; then
  #         echo 'Downloading model...'
  #         curl -L -H 'Authorization: Bearer $$HF_TOKEN' \
  #           https://huggingface.co/$$MODEL_REPO/resolve/main/$$MODEL_FILE \
  #           -o /models/$$MODEL_FILE
  #       else
  #         echo 'Model already exists'
  #       fi
  #     "

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

### Docker Compose Override Files

Create: `docker-compose.amd.yml` (AMD GPU configuration)

```yaml
version: '3.9'

  services:
    llama-server:
      # AMD GPU passthrough (requires AMD Container Toolkit 1.2.0+ and Docker 25.0+)
      deploy:
        resources:
          reservations:
            devices:
              - driver: amdgpu
                count: all
                capabilities: [gpu]
      environment:
        # Vulkan GPU selection
        GGML_VK_VISIBLE_DEVICES: "0"
        # Allow graphics queue (AMD-specific)
        GGML_VK_ALLOW_GRAPHICS_QUEUE: "1"
        # Force max GPU allocation (optional)
        GGML_VK_FORCE_MAX_ALLOCATION_SIZE: "0"
```

Create: `docker-compose.nvidia.yml` (NVIDIA GPU configuration)

```yaml
version: '3.9'

  services:
    llama-server:
      # NVIDIA GPU passthrough (requires NVIDIA Container Toolkit 1.17.4+)
      # CRITICAL: Version 1.17.4+ REQUIRED for CVE-2025-23266/23359 fixes (CVSS 9.0)
      deploy:
        resources:
          reservations:
            devices:
              - driver: nvidia
                count: all
                capabilities: [gpu]
      environment:
        # CUDA GPU selection
        CUDA_VISIBLE_DEVICES: "0"
```

---

## Task 2: Implement Config Override Hierarchy

### Override Hierarchy Logic

**Priority (highest to lowest):**
1. Docker Compose `environment:` section (explicit env vars)
2. YAML config file (`config.yml`)
3. Default values in entrypoint script
4. llama.cpp internal defaults

### Implementation in Entrypoint

Modify: `docker/entrypoint.sh` (add to existing script after translate_config)

```bash
# Function to apply environment variable overrides
apply_overrides() {
    log_info "Applying environment variable overrides..."

    # Priority: Docker env vars > YAML config > defaults

    # Model path
    if [ -n "$LLAMA_ARG_MODEL_PATH" ]; then
        log_info "Overriding model path: $LLAMA_ARG_MODEL_PATH"
    fi

    # Context size
    if [ -n "$LLAMA_ARG_CTX_SIZE" ]; then
        log_info "Overriding context size: $LLAMA_ARG_CTX_SIZE"
    fi

    # Batch size
    if [ -n "$LLAMA_ARG_BATCH_SIZE" ]; then
        log_info "Overriding batch size: $LLAMA_ARG_BATCH_SIZE"
    fi

    # Micro-batch size
    if [ -n "$LLAMA_ARG_UBATCH_SIZE" ]; then
        log_info "Overriding ubatch size: $LLAMA_ARG_UBATCH_SIZE"
    fi

    # GPU layers
    if [ -n "$LLAMA_ARG_N_GPU_LAYERS" ]; then
        log_info "Overriding GPU layers: $LLAMA_ARG_N_GPU_LAYERS"
    fi

    # Temperature
    if [ -n "$LLAMA_ARG_TEMP" ]; then
        log_info "Overriding temperature: $LLAMA_ARG_TEMP"
    fi

    # Top-P
    if [ -n "$LLAMA_ARG_TOP_P" ]; then
        log_info "Overriding top-p: $LLAMA_ARG_TOP_P"
    fi

    # Top-K
    if [ -n "$LLAMA_ARG_TOP_K" ]; then
        log_info "Overriding top-k: $LLAMA_ARG_TOP_K"
    fi

    # Server port
    if [ -n "$LLAMA_ARG_PORT" ]; then
        log_info "Overriding server port: $LLAMA_ARG_PORT"
    fi

    # Parallel processing
    if [ -n "$LLAMA_ARG_PARALLEL" ]; then
        log_info "Overriding parallel: $LLAMA_ARG_PARALLEL"
    fi

    # Timeout
    if [ -n "$LLAMA_ARG_TIMEOUT" ]; then
        log_info "Overriding timeout: $LLAMA_ARG_TIMEOUT"
    fi

    # Max slots
    if [ -n "$LLAMA_ARG_N_SLOT" ]; then
        log_info "Overriding max slots: $LLAMA_ARG_N_SLOT"
    fi

    # Metrics endpoint
    if [ -n "$LLAMA_ARG_METRICS" ]; then
        log_info "Overriding metrics: $LLAMA_ARG_METRICS"
    fi

    # Slots endpoint
    if [ -n "$LLAMA_ARG_SLOTS_ENDPOINT" ]; then
        log_info "Overriding slots endpoint: $LLAMA_ARG_SLOTS_ENDPOINT"
    fi

    # Cache type
    if [ -n "$LLAMA_ARG_CACHE_TYPE_K" ]; then
        log_info "Overriding cache type k: $LLAMA_ARG_CACHE_TYPE_K"
    fi

    if [ -n "$LLAMA_ARG_CACHE_TYPE_V" ]; then
        log_info "Overriding cache type v: $LLAMA_ARG_CACHE_TYPE_V"
    fi

    # Log level
    if [ -n "$LLAMA_ARG_LOG_LEVEL" ]; then
        log_info "Overriding log level: $LLAMA_ARG_LOG_LEVEL"
    fi

    # Vulkan settings
    if [ -n "$GGML_VK_VISIBLE_DEVICES" ]; then
        log_info "Overriding Vulkan visible devices: $GGML_VK_VISIBLE_DEVICES"
    fi

    if [ -n "$GGML_VK_FORCE_MAX_ALLOCATION_SIZE" ]; then
        log_info "Overriding Vulkan max allocation: $GGML_VK_FORCE_MAX_ALLOCATION_SIZE"
    fi

    if [ -n "$GGML_VK_DISABLE_DEBUG" ]; then
        log_info "Overriding Vulkan debug: $GGML_VK_DISABLE_DEBUG"
    fi

    log_info "Override application complete"
}

# Add to main() function after translate_config, before start_server
# apply_overrides
```

### Environment Variable Reference

| Variable | Purpose | Source |
|----------|---------|--------|
| `LLAMA_ARG_MODEL_PATH` | Model file path | Docker env > YAML |
| `LLAMA_ARG_CTX_SIZE` | Context size | Docker env > YAML |
| `LLAMA_ARG_BATCH_SIZE` | Batch size | Docker env > YAML |
| `LLAMA_ARG_UBATCH_SIZE` | Micro-batch size | Docker env > YAML |
| `LLAMA_ARG_N_THREADS` | CPU threads | Docker env > YAML |
| `LLAMA_ARG_N_GPU_LAYERS` | GPU layers | Docker env > YAML |
| `LLAMA_ARG_TEMP` | Temperature | Docker env > YAML |
| `LLAMA_ARG_TOP_P` | Top-P | Docker env > YAML |
| `LLAMA_ARG_TOP_K` | Top-K | Docker env > YAML |
| `LLAMA_ARG_HOST` | Server host | Docker env > YAML |
| `LLAMA_ARG_PORT` | Server port | Docker env > YAML |
| `LLAMA_ARG_PARALLEL` | Parallel processing | Docker env > YAML |
| `LLAMA_ARG_TIMEOUT` | Timeout | Docker env > YAML |
| `LLAMA_ARG_N_SLOT` | Max slots | Docker env > YAML |
| `LLAMA_ARG_METRICS` | Enable metrics | Docker env > YAML |
| `LLAMA_ARG_SLOTS_ENDPOINT` | Enable slots | Docker env > YAML |
| `GGML_VK_VISIBLE_DEVICES` | Vulkan GPU | Docker env > YAML |
| `HF_TOKEN` | HuggingFace token | Docker env only |
| `HUGGING_FACE_HUB_TOKEN` | HuggingFace token (long name) | Docker env only |

---

## Task 3: Implement Multi-Model Support

### Model Switching Strategy

**Approach:** Change model by modifying `config.yml` and restarting container (no rebuild needed)

### Model Config Templates

Create: `configs/models/` directory with example configs

Create: `configs/models/llama-3.2-1b.yml`

```yaml
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
  mmap_size: 4
  use_mmap: true
  sampling:
    temperature: 0.7
    top_p: 0.95
    top_k: 40
    repeat_penalty: 1.1
    repeat_last_n: 64
    max_tokens: 512
  server:
    host: 127.0.0.1  # Secure default
    port: 8080
    parallel: true
    timeout: 600
    max_slots: 8
    metrics: true
    slots_endpoint: true
cache:
  cache_type_k: f16
  cache_type_v: f16
  kv_cache_size: 2
features:
  log_level: info
  verbose: false
  print_system_info: true
  profiling: false
  color: true
vulkan:
  visible_devices: "0"
  disable_debug: true
  enable_validation: false
```

Create: `configs/models/llama-3.1-8b.yml`

```yaml
model:
  path: /models/Llama-3.1-8B-Instruct.Q4_K_M.gguf
  huggingface:
    repo: meta-llama/Meta-Llama-3.1-8B-Instruct
    filename: Meta-Llama-3.1-8B-Instruct.Q4_K_M.gguf
    branch: main
  quantization: Q4_K_M
  parameter_count: 8000000000
context:
  size: 8192
  batch_size: 2048
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
    repeat_last_n: 64
    max_tokens: 1024
  server:
    host: 127.0.0.1  # Secure default
    port: 8080
    parallel: true
    timeout: 600
    max_slots: 8
    metrics: true
    slots_endpoint: true
cache:
  cache_type_k: f16
  cache_type_v: f16
  kv_cache_size: 4
features:
  log_level: info
  verbose: false
  print_system_info: true
  profiling: false
  color: true
vulkan:
  visible_devices: "0"
  disable_debug: true
  enable_validation: false
```

### Model Switch Script

Create: `scripts/switch-model.sh`

```bash
#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse arguments
MODEL_CONFIG="$1"

if [ -z "$MODEL_CONFIG" ]; then
    log_error "Usage: $0 <model-config-path>"
    log_error "Example: $0 configs/models/llama-3.2-1b.yml"
    exit 1
fi

if [ ! -f "$MODEL_CONFIG" ]; then
    log_error "Model config not found: $MODEL_CONFIG"
    exit 1
fi

log_info "Switching to model: $MODEL_CONFIG"

# Backup current config
if [ -f "config.yml" ]; then
    log_info "Backing up current config to config.yml.backup"
    cp config.yml config.yml.backup
fi

# Copy new config
log_info "Copying new config to config.yml"
cp "$MODEL_CONFIG" config.yml

# Verify checksum if specified in new config
if command -v yq &> /dev/null; then
    model_path=$(yq eval '.model.path' config.yml 2>/dev/null)
    expected_sha256=$(yq eval '.model.huggingface.sha256 // ""' "$MODEL_CONFIG" 2>/dev/null || echo "")

    if [ -n "$expected_sha256" ] && [ -f "$model_path" ]; then
        log_info "Verifying model checksum..."
        actual_sha256=$(sha256sum "$model_path" | awk '{print $1}')
        if [ "$actual_sha256" != "$expected_sha256" ]; then
            log_error "Checksum verification failed!"
            log_error "Expected: $expected_sha256"
            log_error "Actual: $actual_sha256"
            log_error "Model file may be corrupted. Re-download required."
            exit 1
        else
            log_info "Checksum verified successfully"
        fi
    fi
fi

# Restart container
log_info "Restarting container..."
docker compose restart

# Wait for container to be healthy
log_info "Waiting for container to be healthy..."
MAX_WAIT=60
WAIT_TIME=0

while [ $WAIT_TIME -lt $MAX_WAIT ]; do
    STATUS=$(docker compose ps --format "{{.Health}}" | head -n1)
    if [ "$STATUS" = "healthy" ]; then
        log_info "Container is healthy"
        break
    fi
    sleep 2
    WAIT_TIME=$((WAIT_TIME + 2))
done

if [ $WAIT_TIME -ge $MAX_WAIT ]; then
    log_warn "Container did not become healthy within $MAX_WAIT seconds"
    log_warn "Check logs: docker compose logs"
fi

# Show current model
log_info "Current model:"
if command -v yq &> /dev/null; then
    yq eval '.model.path' config.yml
else
    grep -A 2 "^model:" config.yml | grep "path:" | awk '{print $2}'
fi

log_info "Model switch complete"
```

---

## Task 4: GPU Detection Script

### GPU Detection Logic

**Purpose:** Automatically detect GPU type (AMD vs NVIDIA) and apply appropriate configuration

### Complete GPU Detection Script

Create: `scripts/detect-gpu.sh`

```bash
#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect GPU type
detect_gpu() {
    log_info "Detecting GPU type..."

    # Check for NVIDIA
    if command -v nvidia-smi &> /dev/null; then
        log_info "NVIDIA GPU detected"
        nvidia-smi --query-gpu=name,driver_version,memory.total --format=csv,noheader

        # Check NVIDIA Container Toolkit version (CRITICAL for security)
        if command -v nvidia-container-cli &> /dev/null; then
            local nvidia_ctk_version=$(nvidia-container-cli --version 2>/dev/null | head -n1 || echo "unknown")
            log_info "NVIDIA Container Toolkit: $nvidia_ctk_version"

            # Check if version is 1.17.4+ (CVE-2025-23266/23359 fix)
            local major_minor=$(echo "$nvidia_ctk_version" | grep -oP '^\d+\.\d+' || echo "0.0")
            if [ "$(echo "$major_minor" | cut -d. -f1)" -lt 1 ] || [ "$(echo "$major_minor" | cut -d. -f1)" -eq 1 ] && [ "$(echo "$major_minor" | cut -d. -f2)" -lt 17 ]; then
                log_error "NVIDIA Container Toolkit version is too old: $nvidia_ctk_version"
                log_error "Version 1.17.4+ REQUIRED for CVE-2025-23266/23359 fixes (CVSS 9.0)"
                log_error "Update: sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit"
                exit 1
            fi
        else
            log_warn "nvidia-container-cli not found. Cannot verify toolkit version."
        fi

        echo "nvidia"
        return 0
    fi

    # Check for AMD (via /dev/kfd)
    if [ -e /dev/kfd ]; then
        log_info "AMD GPU detected"

        # Check AMD Container Toolkit version (required for --gpus support)
        if command -v amdgpu-container-cli &> /dev/null; then
            local amdgpu_ctk_version=$(amdgpu-container-cli --version 2>/dev/null | head -n1 || echo "unknown")
            log_info "AMD Container Toolkit: $amdgpu_ctk_version"

            # Check if version is 1.2.0+ (required for --gpus support)
            local major_minor=$(echo "$amdgpu_ctk_version" | grep -oP '^\d+\.\d+' || echo "0.0")
            if [ "$(echo "$major_minor" | cut -d. -f1)" -lt 1 ] || [ "$(echo "$major_minor" | cut -d. -f1)" -eq 1 ] && [ "$(echo "$major_minor" | cut -d. -f2)" -lt 2 ]; then
                log_warn "AMD Container Toolkit version is too old: $amdgpu_ctk_version"
                log_warn "Version 1.2.0+ required for --gpus support"
                log_warn "Update: sudo apt-get update && sudo apt-get install -y amdgpu-container-toolkit"
                log_warn "Will use legacy device passthrough (--device=/dev/dri, --device=/dev/kfd)"
            fi
        else
            log_warn "amdgpu-container-cli not found. Using legacy device passthrough."
        fi

        # Try to get GPU info (if available)
        if command -v rocminfo &> /dev/null; then
            rocminfo | grep "Name:" | head -n1
        else
            echo "AMD GPU (details unavailable)"
        fi
        echo "amd"
        return 0
    fi

    # Check for AMD (via lspci)
    if command -v lspci &> /dev/null; then
        if lspci | grep -i "AMD" | grep -i "VGA" > /dev/null; then
            log_info "AMD GPU detected (via lspci)"
            lspci | grep -i "AMD" | grep -i "VGA"
            echo "amd"
            return 0
        fi
    fi

    log_warn "No GPU detected. Running in CPU-only mode."
    echo "cpu"
    return 0
}

# Main
GPU_TYPE=$(detect_gpu)

log_info "Recommended compose file:"
case "$GPU_TYPE" in
    nvidia)
        echo "  docker-compose.yml -f docker-compose.nvidia.yml"
        echo "  or"
        echo "  docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d"
        ;;
    amd)
        echo "  docker-compose.yml -f docker-compose.amd.yml"
        echo "  or"
        echo "  docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d"
        ;;
    cpu)
        echo "  docker-compose.yml (no GPU passthrough)"
        echo "  or"
        echo "  docker compose up -d"
        ;;
esac
```

---

## Task 5: Docker Compose Management Scripts

### Start Script

Create: `scripts/start.sh`

```bash
#!/bin/bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

# Detect GPU
GPU_TYPE=$(./scripts/detect-gpu.sh 2>/dev/null || echo "cpu")

log_info "Starting LLM server with $GPU_TYPE GPU..."

# Choose compose files based on GPU
case "$GPU_TYPE" in
    nvidia)
        docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d
        ;;
    amd)
        docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d
        ;;
    *)
        docker compose up -d
        ;;
esac

log_info "Server starting. Check health with: docker compose ps"
log_info "View logs: docker compose logs -f"
```

### Stop Script

Create: `scripts/stop.sh`

```bash
#!/bin/bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_info "Stopping LLM server..."
docker compose down

log_info "Server stopped"
```

### Logs Script

Create: `scripts/logs.sh`

```bash
#!/bin/bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_info "Showing logs (Ctrl+C to exit)..."
docker compose logs -f
```

### Status Script

Create: `scripts/status.sh`

```bash
#!/bin/bash
set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo "=== LLM Server Status ==="
echo ""

# Container status
log_info "Container Status:"
docker compose ps

echo ""

# Health check
HEALTH=$(docker compose ps --format "{{.Health}}" | head -n1)
if [ "$HEALTH" = "healthy" ]; then
    log_info "Health: $HEALTH"
else
    log_warn "Health: $HEALTH"
fi

echo ""

# Port binding
log_info "Port Binding:"
docker compose ps --format "  Port: {{.Ports}}"

echo ""

# GPU usage (if available)
if command -v nvidia-smi &> /dev/null; then
    log_info "NVIDIA GPU Usage:"
    nvidia-smi --query-gpu=index,utilization.gpu,memory.used,memory.total --format=csv,noheader
    echo ""
elif command -v rocminfo &> /dev/null; then
    log_info "AMD GPU Info:"
    rocminfo | grep "Compute Unit:" | head -n1
    echo ""
fi

# Model
log_info "Current Model:"
if [ -f "config.yml" ]; then
    if command -v yq &> /dev/null; then
        yq eval '.model.path' config.yml
    else
        grep -A 2 "^model:" config.yml | grep "path:" | awk '{print $2}'
    fi
else
    echo "  No config.yml found"
fi
```

---

## Verification Steps

### Step 1: Start Server with Default Config

**Test:** Start server using docker-compose

```bash
# Start server
./scripts/start.sh

# Expected output:
# [INFO] Starting LLM server with [amd/nvidia/cpu] GPU...
# [INFO] Server starting. Check health with: docker compose ps
# [INFO] View logs: docker compose logs -f

# Check status
./scripts/status.sh

# Expected:
# === LLM Server Status ===
#
# [INFO] Container Status:
# NAME                  COMMAND                  SERVICE
# whitt-llama-server     "/entrypoint.sh"          llama-server
#
# [INFO] Health: healthy
```

### Step 2: Verify Config Override

**Test:** Override config with environment variable

```bash
# Stop server
./scripts/stop.sh

# Start with environment variable override
LLAMA_ARG_CTX_SIZE=8192 docker compose up -d

# Check logs
docker compose logs | grep "context size"

# Expected: "Overriding context size: 8192"

# Verify via API
curl http://localhost:8080/props

# Expected: JSON with "ctx_size": 8192
```

### Step 3: Verify Model Switching

**Test:** Switch to different model without rebuild

```bash
# Switch to 1B model
./scripts/switch-model.sh configs/models/llama-3.2-1b.yml

# Expected:
# [INFO] Switching to model: configs/models/llama-3.2-1b.yml
# [INFO] Backing up current config to config.yml.backup
# [INFO] Copying new config to config.yml
# [INFO] Restarting container...
# [INFO] Waiting for container to be healthy...
# [INFO] Container is healthy
# [INFO] Current model:
# /models/Llama-3.2-1B-Instruct.Q4_K_M.gguf
# [INFO] Model switch complete

# Switch to 8B model
./scripts/switch-model.sh configs/models/llama-3.1-8b.yml

# Verify model changed
curl http://localhost:8080/props

# Expected: Model info for 8B model
```

### Step 4: Verify GPU Passthrough (AMD)

**Test:** AMD GPU access

```bash
# Stop server
./scripts/stop.sh

# Start with AMD config
docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d

# Check logs for GPU detection
docker compose logs | grep -i "vulkan\|gpu"

# Expected: "Vulkan support detected. GPU: 0"

# Check GPU usage
./scripts/status.sh

# Expected: AMD GPU info displayed
```

### Step 5: Verify GPU Passthrough (NVIDIA)

**Test:** NVIDIA GPU access

```bash
# Stop server
./scripts/stop.sh

# Start with NVIDIA config
docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d

# Check logs for GPU detection
docker compose logs | grep -i "cuda\|gpu"

# Expected: GPU detection message

# Check GPU usage
./scripts/status.sh

# Expected: NVIDIA GPU usage displayed
```

### Step 6: Verify Config Volume Mount

**Test:** Config file mounted correctly

```bash
# Check config in container
docker exec whitt-llama-server cat /config/config.yml

# Expected: Content of local config.yml

# Modify local config
echo "context:" >> config.yml
echo "  size: 2048" >> config.yml

# Restart container
docker compose restart

# Verify new config applied
curl http://localhost:8080/props | grep ctx_size

# Expected: 2048
```

### Step 7: Verify Model Volume Persistence

**Test:** Model persists across container restarts

```bash
# Check model exists
ls -lh models/Llama-3.2-1B-Instruct.Q4_K_M.gguf

# Stop and remove container
./scripts/stop.sh
docker compose rm -f

# Start again
./scripts/start.sh

# Verify model still exists
ls -lh models/Llama-3.2-1B-Instruct.Q4_K_M.gguf

# Expected: Same file (not re-downloaded)
```

### Step 8: Verify Multi-GPU Configuration

**Test:** Multiple GPUs

```bash
# Stop server
./scripts/stop.sh

# Modify config to use multiple GPUs
sed -i 's/visible_devices: "0"/visible_devices: "0,1"/' config.yml

# Restart with GPU config
docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d

# Check logs
docker compose logs | grep "GPU: 0,1"

# Expected: Vulkan using both GPUs
```

---

## Integration with Other Phases

### Phase 01: YAML Config Schema

**Output:** YAML schema definitions
**Input:** Phase 03 uses config files defined in Phase 01

### Phase 02: Docker Container

**Output:** Docker image with entrypoint
**Input:** Phase 03 uses image from Phase 02

### Phase 04: HTTP Interface

**Output:** No direct integration
**Input:** No input from Phase 04

### Phase 05: Rust Client

**Output:** Docker compose files and management scripts
**Input:** Phase 05 uses docker-compose to manage container lifecycle

---

## File Locations

Create:
- `docker-compose.yml` - Main compose file
- `docker-compose.amd.yml` - AMD GPU override
- `docker-compose.nvidia.yml` - NVIDIA GPU override
- `configs/models/llama-3.2-1b.yml` - 1B model config
- `configs/models/llama-3.1-8b.yml` - 8B model config
- `scripts/switch-model.sh` - Model switching script
- `scripts/detect-gpu.sh` - GPU detection script
- `scripts/start.sh` - Start server script
- `scripts/stop.sh` - Stop server script
- `scripts/logs.sh` - View logs script
- `scripts/status.sh` - Status check script

---

## Success Criteria

- [ ] Docker compose starts container successfully
- [ ] Config file mounted and read correctly
- [ ] Environment variable overrides work
- [ ] Model switching works without rebuild
- [ ] GPU passthrough works (both AMD and NVIDIA)
- [ ] Container healthcheck passes
- [ ] Model volume persists across restarts
- [ ] Logs volume persists across restarts
- [ ] Management scripts work (start, stop, logs, status)
- [ ] GPU detection identifies correct GPU type
- [ ] Multi-GPU configuration works

---

## Next Steps

After completing Phase 03:
1. Proceed to Phase 05: Rust Client
2. Phase 04 can be completed in parallel (documentation)
3. Use docker-compose files in Phase 05 for container management
