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

# AMD Polaris GPU workarounds (RX 570/580/590)
# IMPORTANT: Use RADV driver, NOT AMDVLK (2GB allocation limit, llama.cpp issue #15054)
# Issue #5441: Memory allocation limit on Polaris GPUs
ENV GGML_VK_FORCE_MAX_ALLOCATION_SIZE=2147483646
# Issue #20465: Flash attention may cause garbled output on Polaris
# Set in entrypoint: --flash-attn 0

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libvulkan1 \
    mesa-vulkan-drivers \
    libglvnd0 \
    libgl1 \
    libglx0 \
    libegl1 \
    libgles2 \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install huggingface-hub CLI
RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

COPY --from=base /app/llama-server /usr/local/bin/llama-server

# Create directories
RUN mkdir -p /models \
    /config \
    /var/log/llama-server \
    /var/run/llama-server

# CRITICAL: Vulkan ICD loader symlink (fix for Issue #1392)
RUN ln -s /etc/vulkan/icd.d /usr/share/vulkan/icd.d

# Set working directory
WORKDIR /app

# Copy entrypoint script
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Expose HTTP server port
EXPOSE 8080

# Health check - use /props instead of /health to avoid queueing under load (Issue #20684)
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

# Set up volumes
VOLUME ["/models", "/config", "/var/log/llama-server"]

# Run as non-root user for security
RUN useradd -m -u 1000 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

# Entry point with tini for signal handling
ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
