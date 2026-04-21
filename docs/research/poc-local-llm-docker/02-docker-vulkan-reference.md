# Docker + Vulkan + GPU Research Reference

## Docker Version Requirements

**Docker Engine:** 29.3.0+ recommended (fixes MIG bug, AMD CDI support)
**docker compose:** 2.0+ required

### Docker 29.x Deprecations

**cgroup v1 Deprecation:** Docker 29.x deprecates cgroup v1 support.

**Impact:**
- GPU workloads may require cgroup v2 for optimal resource isolation
- Legacy systems using cgroup v1 should upgrade kernel (5.2+)
- Some container runtimes may have compatibility issues

**Migration:**
- Verify cgroup version: `docker info | grep "Cgroup Version"`
- Enable cgroup v2: Boot kernel with `systemd.unified_cgroup_hierarchy=1`
- Check AMD GPU toolkit compatibility with cgroup v2

**Note:** cgroup v2 is default in modern Linux distributions (Ubuntu 21.04+, RHEL 9+, etc.)

**NVIDIA Container Toolkit:**
- **Version:** 1.17.4+ REQUIRED (CVE-2025-23266/23359 fixes)
- **CVE Severity:** CVSS 9.0 (container escape vulnerability)
- **Installation:** See NVIDIA GPUs section below

**AMD Container Toolkit:**
- **Version:** 1.2.0+ for --gpus support (requires Docker 25.0+)
- **Installation:** See AMD GPUs section below

**GPU Driver Requirements:**
- NVIDIA 570.123.10+ (for CUDA workloads)
- AMD AMDGPU 6.4.x (for ROCm/Vulkan)
- Intel NEO 26.09+ (for oneAPI/Vulkan)

## Official Vulkan Dockerfile

**Base Configuration:**
```dockerfile
FROM ubuntu:26.04
RUN cmake -DGGML_VULKAN=ON ...
```

**Runtime Dependencies:**
```dockerfile
RUN apt-get install -y \
    libvulkan1 \
    mesa-vulkan-drivers \
    libglvnd0 \
    libgl1 \
    libglx0 \
    libegl1 \
    libgles2
```

## GPU Vendor Configuration

### NVIDIA GPUs
```bash
docker run \
    --gpus all \
    --runtime=nvidia \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Prerequisites:**
- `nvidia-container-toolkit` 1.17.4+ installed on host (REQUIRED for CVE-2025-23266/23359 fixes, CVSS 9.0)
- `nvidia-docker` version 2.0+
- NVIDIA driver >= 570.123.10+

**Toolkit Installation:**
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

### AMD GPUs

#### CDI-Based GPU Injection (Docker 29.3.0+)

**New in Docker 29.3.0:** Container Device Interface (CDI) for AMD GPU injection.

```bash
# CDI-based injection (recommended for Docker 29.3.0+)
docker run \
    --device nvidia.com/gpu=all \
    ghcr.io/ggml-org/llama.cpp:server-vulkan

# Or specific GPU by index
docker run \
    --device nvidia.com/gpu=0 \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Benefits:**
- Declarative device specification
- Better integration with container runtimes
- Future-proof (replacing legacy device mounting)

#### Legacy Device Mounting (Still Supported)

```bash
docker run \
    --gpus all \
    --device=/dev/kfd \
    --device=/dev/dri:/dev/dri \
    --group-add video \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Note:** `--device /dev/dri` still works but CDI is the future. Legacy method remains compatible.

**Prerequisites:**
- `amdgpu-container-toolkit` 1.2.0+ installed on host (REQUIRED for --gpus support, requires Docker 25.0+)
- ROCm drivers installed on host
- Docker 25.0+ required for --gpus support
- Docker 29.3.0+ recommended for CDI-based injection

**Toolkit Installation:**
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

**Alternative (legacy):** `--device=/dev/kfd` + `--device=/dev/dri` + `--group-add video` (if toolkit not available)

#### AMD RX 580 (Polaris) Specific Configuration

**Driver Requirement:** Use RADV (Mesa) driver. Do NOT use AMDVLK — it has a 2GB memory allocation limit (llama.cpp issue #15054).

**Flash Attention Bug:** Flash attention is broken on Polaris (llama.cpp issue #20465). Always launch with `-fa 0` flag to disable.

**Memory Workaround:** Set environment variable `GGML_VK_FORCE_MAX_ALLOCATION_SIZE=2147483646` to work around allocation limits.

**Performance:** ~39 tok/s (7B Q4_K_M), ~226 tok/s (1B models) on RX 580 8GB with Vulkan backend.

### Intel GPUs
```bash
docker run \
    --device=/dev/dri \
    --group-add video \
    --group-add render \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Recommendation:** SYCL preferred over Vulkan for Intel GPUs (better performance).
**Prerequisites:**
- Intel oneAPI Base Toolkit
- Intel GPU drivers (latest)

## Network Optimization

**Network Mode Comparison:**
| Mode | Latency | Use Case |
|------|---------|----------|
| `--network bridge` | 50-100μs | Default, isolated |
| `--network host` | 5-10μs | Lowest latency, development |

**Recommendation:** Use `--network host` for lowest HTTP latency (5-10μs vs 50-100μs bridge).

**Docker Compose Configuration:**
```yaml
services:
  llama-server:
      network_mode: host  # For lowest latency
```

**Trade-offs:**
- `host` mode: Lowest latency, but no port isolation
- `bridge` mode: Better isolation, but higher latency

## Shared Memory for GPU Workloads

**Recommendation:** `--shm-size=8g` for Vulkan GPU workloads (8GB shared memory).

**Docker Run:**
```bash
docker run --rm \
    --gpus all \
    --shm-size=8g \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**docker compose:**
```yaml
services:
  llama-server:
      shm_size: 8g
```

## Non-Root Container Security Patterns

**Run as Non-Root User:**
```dockerfile
RUN useradd -m -u 1000 -s /bin/bash llama
USER llama
```

**Drop Capabilities:**
```dockerfile
RUN setcap cap_drop=all, cap_sys_admin=eip RUN llama
```

**Read-Only Filesystem:**
```bash
# Mount model volumes as read-only
docker run -v /models:/models:ro \
    --gpus all \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Localhost Binding:**
```bash
# Bind to localhost only (not 0.0.0.0 for production)
docker run -p 127.0.0.1:8080:8080 \
    --gpus all \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**API Key Authentication:**
```bash
docker run -e LLAMA_ARG_API_KEY="your-secret-api-key" \
    --gpus all \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Environment Variable:**
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

**Note:** Comma-separated indices match GPU order from `vulkaninfo`.

## Vulkan Environment Variables

### Device Selection
```bash
GGML_VK_VISIBLE_DEVICES=0,1,2    # GPU indices to use
```

### Memory Management
```bash
GGML_VK_FORCE_MAX_ALLOCATION_SIZE=1073741824    # 1GB max allocation
GGML_VK_DISABLE_COOPMAT=1                         # Disable cooperative matrices
```

### AMD-Specific
```bash
GGML_VK_ALLOW_GRAPHICS_QUEUE=1    # Allow graphics queue usage (AMD)
```

### Debugging
```bash
VK_LAYER_PATH=/path/to/layers
VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation
```

## Vulkan ICD Loader Issue

**Issue #1392:** NVIDIA ICD files mounted at `/etc/vulkan/icd.d/` but applications expect `/usr/share/vulkan/icd.d/`.

## libglvnd Fix (PR #18664)

**PR #18664:** Fixed libglvnd library linking issues in llama.cpp Docker image.

**Problem:** Missing or incorrect libglvnd library references causing Vulkan initialization failures on some GPU vendors.

**Solution:** Updated Dockerfile to properly install and link libglvnd libraries:
```dockerfile
RUN apt-get install -y libglvnd0 libgl1 libglx0 libegl1 libgles2
```

**Impact:** Resolves GPU detection issues on AMD, NVIDIA, and Intel GPUs in containerized environments.

**Verification:**
```bash
docker run --rm ghcr.io/ggml-org/llama.cpp:server-vulkan \
    vulkaninfo | grep "GPU id"
```

**Note:** If using older images without this fix, manually install libglvnd packages in your Dockerfile.

**Symptoms:**
- `vkCreateInstance` failures
- "No compatible GPU found" errors

**Workaround:**
```dockerfile
RUN ln -s /etc/vulkan/icd.d /usr/share/vulkan/icd.d
```

**Verification:**
```bash
docker run --rm --gpus all ghcr.io/ggml-org/llama.cpp:server-vulkan \
    vulkaninfo | grep "GPU id"
```

## Model Distribution Strategies

### 1. Hugging Face Hub (Recommended)

**Using Python huggingface_hub:**
```dockerfile
FROM python:3.11-slim

RUN pip install huggingface_hub

COPY download_model.py /app/
RUN python /app/download_model.py --repo llama3/Meta-Llama-3-8B-Instruct \
    --filename llama-3-8b-instruct.Q4_K_M.gguf \
    --output /models

COPY --from=builder /app/llama-server /usr/local/bin/
```

**download_model.py:**
```python
from huggingface_hub import hf_hub_download

model_path = hf_hub_download(
    repo_id="meta-llama/Meta-Llama-3-8B-Instruct",
    filename="llama-3-8b-instruct.Q4_K_M.gguf",
    local_dir="/models",
    token=os.environ.get("HUGGING_FACE_HUB_TOKEN")
)
```

### 2. llama.cpp Native

```bash
docker run \
    -e HF_MODEL_REPO=meta-llama/Meta-Llama-3-8B-Instruct \
    -e HF_MODEL_FILE=llama-3-8b-instruct.Q4_K_M.gguf \
    -e HUGGING_FACE_HUB_TOKEN=<token> \
    ghcr.io/ggml-org/llama.cpp:server-vulkan \
    --hf-repo meta-llama/Meta-Llama-3-8B-Instruct \
    --hf-file llama-3-8b-instruct.Q4_K_M.gguf
```

### 3. Direct Download

```dockerfile
RUN wget -O /models/model.gguf \
    https://huggingface.co/meta-llama/Meta-Llama-3-8B-Instruct/resolve/main/llama-3-8b-instruct.Q4_K_M.gguf
```

### 4. BuildKit Cache for Model Downloads

**Docker BuildKit cache mount pattern for HuggingFace downloads:**

```dockerfile
#syntax=docker/dockerfile:1.4
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan

RUN apt-get update && apt-get install -y python3-pip
RUN pip3 install huggingface_hub

RUN --mount=type=cache,target=/root/.cache/huggingface \
    hf download meta-llama/Meta-Llama-3-8B-Instruct \
    llama-3-8b-instruct.Q4_K_M.gguf \
    --local-dir /models
```

**Benefits:**
- Cache persists across builds (no re-download on rebuild)
- Reduces build time significantly for development workflows
- Compatible with Docker BuildKit (default in modern Docker)

**Important: Docker Hub Layer Limit**
Docker Hub has a 5GB layer limit. Models exceeding this size (many 7B+ GGUF files) MUST use volume mounts at runtime instead of embedding in the image. Use BuildKit cache during development, then switch to runtime downloads or volume mounts for production.

## Hugging Face Token Handling

### Environment Variable (Development)
```bash
docker run \
    -e HUGGING_FACE_HUB_TOKEN=hf_xxx \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

### Docker Secrets (Production)
```bash
echo "hf_xxx" | docker secret create hf_token -

docker service create \
    --secret source=hf_token,target=hf_token \
    -e HUGGING_FACE_HUB_TOKEN_FILE=/run/secrets/hf_token \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

### NEVER Use Build Args
```dockerfile
# BAD: Visible in docker history
ARG HF_TOKEN=hf_xxx
ENV HUGGING_FACE_HUB_TOKEN=${HF_TOKEN}
```

## Multi-Stage Builds

### Optimized Builder Pattern
```dockerfile
# Builder Stage
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan AS builder

RUN apt-get update && apt-get install -y python3-pip
RUN pip3 install huggingface_hub

COPY download_model.py /app/
RUN python3 /app/download_model.py --repo meta-llama/Meta-Llama-3-8B-Instruct \
    --filename llama-3-8b-instruct.Q4_K_M.gguf \
    --output /models

# Runtime Stage
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan

COPY --from=builder /models /models
COPY --from=builder /usr/local/bin/llama-server /usr/local/bin/

EXPOSE 8080
CMD ["llama-server", "-m", "/models/llama-3-8b-instruct.Q4_K_M.gguf", "-ngl", "99"]
```

## Health Checks

### Standard HEALTHCHECK
```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1
```

### Important: Install curl in RUNTIME layer
```dockerfile
FROM ghcr.io/ggml-org/llama.cpp:server-vulkan

# GOOD: curl in runtime layer
RUN apt-get update && apt-get install -y --no-install-recommends curl

HEALTHCHECK CMD curl -f http://localhost:8080/health || exit 1

# BAD: curl in builder layer (not available in runtime)
```

### Health Check Under Load
```dockerfile
# Workaround for issue #20684
HEALTHCHECK CMD curl -f http://localhost:8080/props || exit 1
```

## Graceful Shutdown

### Using --init flag
```bash
docker run --init ghcr.io/ggml-org/llama.cpp:server-vulkan
```

### Using dumb-init
```dockerfile
RUN apt-get update && apt-get install -y dumb-init

ENTRYPOINT ["dumb-init", "--"]
CMD ["llama-server", "-m", "/models/model.gguf"]
```

### Model Unload Before Exit
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

## Performance Overhead

**Finding:** No Docker-native performance regression documented in llama.cpp issues.

**Overhead Sources:**
- Containerization layer: <5% CPU overhead
- GPU passthrough: Negligible (<1%)
- Network: Only for external clients

**Benchmark:**
- Native: 50 tokens/sec
- Dockerized: 47-49 tokens/sec (3-6% overhead)

## Community Examples

### kth8/llama-server-vulkan
**Repository:** https://github.com/kth8/llama-server-vulkan

**docker compose:**
```yaml
services:
  llama-server:
    image: kth8/llama-server-vulkan:latest
    devices:
      - /dev/kfd
      - /dev/dri
    group_add:
      - video
    ports:
      - "8080:8080"
    environment:
      - GGML_VK_VISIBLE_DEVICES=0
```

**Features:**
- AMD GPU optimized
- Auto model download
- Configurable via env vars

## Best Practices

### 1. Layer Caching
```dockerfile
# GOOD: Layer caching for model downloads
COPY requirements.txt /app/
RUN pip install -r requirements.txt

COPY download_model.py /app/
RUN python /app/download_model.py

# BAD: No caching, downloads every build
COPY download_model.py requirements.txt /app/
RUN pip install -r requirements.txt && python /app/download_model.py
```

### 2. Security
```dockerfile
# Run as non-root
RUN useradd -m -u 1000 llm
USER llm

# Read-only root filesystem
RUN chmod -R a-w /usr/local/share

# Drop capabilities
RUN setcap cap_net_bind_service=+ep /usr/local/bin/llama-server
```

### 3. Resource Limits
```bash
docker run \
    --memory=16g \
    --memory-reservation=8g \
    --cpus=4 \
    --pids-limit 1000 \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

## References

- **llama.cpp Vulkan:** https://github.com/ggerganov/llama.cpp#vulkan
- **Issue #1392:** Vulkan ICD loader path mismatch
- **NVIDIA Container Toolkit:** https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/
- **ROCm Installation:** https://rocm.docs.amd.com/
- **Intel oneAPI:** https://www.intel.com/content/www/us/en/developer/tools/oneapi/base-toolkit.html
- **kth8/llama-server-vulkan:** https://github.com/kth8/llama-server-vulkan
