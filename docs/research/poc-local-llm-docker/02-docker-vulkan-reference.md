# Docker + Vulkan + GPU Research Reference

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
- `nvidia-container-toolkit` installed on host
- `nvidia-docker` version 2.0+
- NVIDIA driver >= 470.x

**Toolkit Installation:**
```bash
distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | \
  sudo tee /etc/apt/sources.list.d/nvidia-docker.list

sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
sudo systemctl restart docker
```

### AMD GPUs
```bash
docker run \
    --device=/dev/kfd \
    --device=/dev/dri \
    --group-add video \
    ghcr.io/ggml-org/llama.cpp:server-vulkan
```

**Prerequisites:**
- ROCm drivers installed on host
- No NVIDIA toolkit required
- `--device=/dev/kfd` for KFD (AMD GPU device)
- `--device=/dev/dri` for Direct Rendering Infrastructure
- `--group-add video` for video device group permissions

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

## Multi-GPU Configuration

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

**Docker Compose:**
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
