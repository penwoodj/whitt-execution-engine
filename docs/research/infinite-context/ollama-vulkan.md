# Ollama & llama.cpp with Vulkan

## Executive Summary

**Vulkan Support Status**: Production-ready in llama.cpp, merged in Ollama
**Purpose**: GPU acceleration for AMD, Intel, and NVIDIA GPUs
**Performance**: 25 tok/s (4B models) to 15 tok/s (9B models)
**Context Window**: Tested up to 16,384 tokens with Vulkan backend

## Overview

Vulkan is a cross-platform graphics API that provides:
- **Vendor-agnostic GPU acceleration**: Works on AMD, Intel, NVIDIA
- **Cross-platform**: Linux, macOS, Windows (with limitations)
- **Memory efficiency**: Better VRAM utilization than some vendor backends
- **Open source**: No vendor lock-in

## Implementation Status

### llama.cpp Vulkan Backend

**Repository**: [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp)
**Status**: Fully integrated, production-ready
**Key PRs**:
- **[#2059: Vulkan Implementation](https://github.com/ggml-org/llama.cpp/pull/2059)** - Merged Jan 2024
  - 156 commits
  - 69K additions
  - Initial implementation with matmul kernels
- **[#7961: Multiple Contexts](https://github.com/ggml-org/llama.cpp/pull/7961)** - Merged Jun 2024
  - Fixed memory overlap issues
  - Support for multiple concurrent contexts
- **[#7084: Bugfixes and Improvements](https://github.com/ggml-org/llama.cpp/pull/7084)** - Merged May 2024
  - Fixed MoE model support
  - Performance optimizations
  - MMV shader improvements

### Ollama Vulkan Integration

**Repository**: [ollama/ollama](https://github.com/ollama/ollama)
**Status**: Merged in main branch, not in binary releases
**Key PRs**:
- **[#2396: llama.cpp now supports Vulkan](https://github.com/ollama/ollama/issues/2396)** - Request Feb 2024
- **[#2578: First attempt at Vulkan](https://github.com/ollama/ollama/pull/2578)** - Closed (WIP)
- **[#5059: Add Vulkan support to ollama](https://github.com/ollama/ollama/pull/5059)** - Merged Jun 2024
  - Memory monitoring implementation
  - Proper GPU budget handling

## Building with Vulkan

### llama.cpp Compilation

```bash
# Clone repository
git clone https://github.com/ggml-org/llama.cpp.git
cd llama.cpp

# Enable Vulkan backend
cmake -B build \
    -DLLAMA_VULKAN=ON \
    -DCMAKE_BUILD_TYPE=Release

# Build
cmake --build build --target llama-cli

# Result: llama-cli with Vulkan support
```

### Ollama Compilation

```bash
# Clone repository
git clone https://github.com/ollama/ollama.git
cd ollama

# Build with Vulkan support (requires local build)
OLLAMA_SKIP_PATCHING=true ./gen_linux.sh
make OLLAMA_CUSTOM_CPU_DEFS="-DLLAMA_VULKAN=1"

# Note: Vulkan support requires compiling llama.cpp submodule with GGML_VULKAN=ON flag
```

### RAMalama (Container-based)

**Repository**: [containers/ramalama](https://github.com/containers/ramalama)
**Status**: Defaults to Vulkan for AMD/Intel on Linux/macOS
**Key PR**: [#2541: Default to llama.cpp vulkan backend](https://github.com/containers/ramalama/pull/2541)

```bash
# Pull image with Vulkan support
docker pull ghcr.io/ramalama/ramalama:latest

# Run with Vulkan backend (auto-selected for AMD/Intel)
docker run --device=/dev/dri ramalama:latest \
    --backend=vulkan  # Or auto for default
```

## Performance Benchmarks

### llama.cpp Vulkan Performance

#### System Specifications
```
CPU: Intel i3-10100
GPU: AMD RX 580 2048SP 8GB
RAM: 24GB DDR4 2666MHz
```

#### Performance Results

| Model | Quantization | Context Size | Batch Size | Throughput |
|-------|--------------|--------------|-------------|------------|
| Qwen 3.5 4B | Q4_K | 16,384 | 8192 | ~25 tok/s |
| Qwen 3.5 9B | Q4_K | 16,384 | 8192 | ~15 tok/s |

#### Command Line
```bash
./llama-cli \
    --model qwen3.5-4b-instruct-q4_k_m.gguf \
    --device Vulkan0,Vulkan1 \
    --main-gpu 0 \
    --ctx-size 16384 \
    --batch-size 8192 \
    --ubatch-size 256 \
    --ctk q8_0 \
    --ctv q8_0 \
    --n-gpu-layers 33 \
    --parallel 1
```

### Backend Comparison

| Backend | GPU | Performance | Notes |
|---------|-----|-------------|-------|
| Vulkan | AMD RX 580 | 25 tok/s (4B) | Good, stable |
| Vulkan | Intel Arc 130V | 15-20 tok/s | Maturing |
| ROCm | AMD RX 580 | 22 tok/s (4B) | Vendor-specific |
| CUDA | NVIDIA RTX 4090 | 35+ tok/s | Most mature |
| SYCL | Intel Arc | 12-15 tok/s | Experimental |

## Vulkan Backend Configuration

### Device Selection

```bash
# List available Vulkan devices
./llama-cli --list-devices
# Output:
# ggml_vulkan: Found 2 Vulkan devices:
# Vulkan0: NVIDIA RTX 4090 | uma: 0 | fp16: 1
# Vulkan1: AMD RX 580 | uma: 1 | fp16: 1

# Use specific device
./llama-cli --model model.gguf --device Vulkan0

# Use multiple devices
./llama-cli --model model.gguf --device Vulkan0,Vulkan1
```

### GPU Layer Offloading

```bash
# Offload N layers to GPU
./llama-cli \
    --model model.gguf \
    --n-gpu-layers 33  # Offload 33 of 33 layers (for 7B model)

# Test optimal layer count
for layers in 10 20 30 33; do
    time ./llama-cli --model model.gguf --n-gpu-layers $layers
```

### Context Size Optimization

```bash
# Set context size (must fit in VRAM)
./llama-cli \
    --ctx-size 8192  # 8K context
# or
    --ctx-size 16384 # 16K context (if VRAM permits)

# Check VRAM usage
./llama-cli --model model.gguf --device Vulkan0
# Output: ggml_vulkan: buffer size = 4403.49 MiB
```

### Batch Size Tuning

```bash
# Larger batch sizes = better GPU utilization
# but higher memory usage
./llama-cli \
    --batch-size 8192 \  # For throughput
    --ubatch-size 256 \  # Micro-batch size
    --ctx-size 16384
```

## Known Issues

### Context Size Ignored (Intel GPUs)
**Issue**: [#13573](https://github.com/ollama/ollama/issues/13573)
**Symptom**: Context size settings ignored on Intel GPUs
**Status**: Open, affects Intel GPU users
**Workaround**: Use specific command-line flags

### Multiple Contexts Memory Overlap
**Issue**: [#7575](https://github.com/ggml-org/llama.cpp/issues/7575)
**Symptom**: Multiple contexts share memory inappropriately
**Status**: Fixed in PR #7961
**Resolution**: Use latest llama.cpp version

### Model Scheduling (Ollama)
**Symptom**: Model scheduling not fully implemented
**Status**: In development
**Impact**: Can't efficiently switch between models

## Best Practices

### 1. Choose Right Backend

```bash
# Automatic selection (recommended)
./llama-cli --model model.gguf --backend auto
# Auto picks:
# - Vulkan for AMD/Intel (Linux/macOS)
# - CUDA for NVIDIA
# - ROCm for AMD (Windows)
# - SYCL for Intel (Windows)
```

### 2. Optimize GPU Layer Offloading

```bash
# Start with all CPU
./llama-cli --n-gpu-layers 0
# Increase gradually and monitor VRAM
for i in 10 20 30; do
    ./llama-cli --n-gpu-layers $i
    nvidia-smi  # Check VRAM usage
# Find max layers that fit
```

### 3. Use Appropriate Quantization

```bash
# Q4_K: Best compression, slightly slower
./llama-cli --model q4_k.gguf

# Q8_0: Good balance
./llama-cli --model q8_0.gguf

# F16: Best quality, 2× VRAM usage
./llama-cli --model f16.gguf
```

### 4. Monitor Performance

```bash
# Enable performance counters
./llama-cli --model model.gguf --perf

# Output includes:
# - Tokens per second
# - VRAM usage
# - CPU usage
# - Cache hits
```

### 5. Context Size Recommendations

| GPU VRAM | Model Size | Recommended Context |
|------------|------------|---------------------|
| 8GB | 7B Q4_K | 8K - 16K |
| 12GB | 7B Q4_K | 16K - 32K |
| 16GB | 13B Q4_K | 32K - 64K |
| 24GB | 34B Q4_K | 64K - 128K |

## Integration with RAG

### Vector Database + Ollama

```python
from ollama import ollama
import chromadb

# Initialize
client = chromadb.PersistentClient(path="./chroma_db")
ollama_client = ollama.Client()

# Generate embeddings
def get_embedding(text):
    response = ollama_client.embeddings(
        model="mxbai-embed-large",
        prompt=text
    )
    return response["embedding"]

# Index documents
documents = load_documents("docs/")
for doc in documents:
    embedding = get_embedding(doc)
    client.add_collection(
        name="docs",
        embeddings=[embedding],
        documents=[doc],
        metadatas=[{"source": doc.source}]
    )

# Query and generate
def query(question):
    # Retrieve relevant context
    query_embedding = get_embedding(question)
    results = client.query(
        collection_name="docs",
        query_embeddings=[query_embedding],
        n_results=5
    )

    # Generate with retrieved context
    context = "\n\n".join([r.doc for r in results["documents"][0]])
    response = ollama_client.generate(
        model="llama3.2",
        prompt=f"Context:\n{context}\n\n\nQuestion: {question}"
    )

    return response["response"]
```

### Multi-GPU RAG

```bash
# Run embedding model on one GPU
OLLAMA_VISIBLE_DEVICES=Vulkan0 ollama run mxbai-embed-large &

# Run generation model on another GPU
OLLAMA_VISIBLE_DEVICES=Vulkan1 ollama run llama3.2
```

## Troubleshooting

### Vulkan Not Found

```bash
# Error: "Could not find Vulkan"
# Solution: Install Vulkan SDK
wget https://sdk.lunargraphics.com/sdk/download/latest/linux/vulkan-sdk.tar.xz
tar -xf vulkan-sdk.tar.xz
source vulkan-sdk/env/setup-env.sh

# Or install via package manager
# Ubuntu
sudo apt install libvulkan-dev
# macOS
brew install vulkan-headers
```

### Out of Memory

```bash
# Error: "Out of memory"
# Solution: Reduce batch size or context
./llama-cli \
    --model model.gguf \
    --batch-size 4096 \  # Reduce from 8192
    --ctx-size 8192 \  # Reduce from 16384
```

### Poor Performance

```bash
# Check: Are you using right backend?
./llama-cli --list-devices

# Check: GPU layer count
./llama-cli --model model.gguf --n-gpu-layers 33

# Check: Context size too large?
./llama-cli --model model.gguf --ctx-size 8192

# Check: Using slow quantization?
# Q4_K is faster than Q8_0, Q8_0 is faster than F16
```

## Performance Tuning Guide

### Optimization Checklist

- [ ] Use Vulkan for AMD/Intel GPUs
- [ ] Offload appropriate layers to GPU
- [ ] Choose optimal batch size
- [ ] Use Q4_K quantization for speed
- [ ] Set appropriate context size
- [ ] Enable caching (--cache-type-k)
- [ ] Use compiled CUDA cores (--flash-attn equivalent)
- [ ] Monitor VRAM usage
- [ ] Profile with --perf flag

### Tuning Commands

```bash
# Performance comparison
time ./llama-cli --model model.gguf --backend vulkan
time ./llama-cli --model model.gguf --backend rocm  # AMD
time ./llama-cli --model model.gguf --backend cuda   # NVIDIA

# Batch size sweep
for bs in 4096 8192 16384; do
    echo "Testing batch size: $bs"
    time ./llama-cli --model model.gguf --batch-size $bs

# Context size sweep
for ctx in 4096 8192 16384; do
    echo "Testing context: $ctx"
    time ./llama-cli --model model.gguf --ctx-size $ctx
```

## Applications

### Ideal Use Cases
- **Local RAG systems**: Fast embeddings and generation
- **Document analysis**: Process large documents locally
- **Code completion**: LSP integration with local models
- **Privacy-sensitive work**: No data leaves device
- **Edge deployment**: ARM/AMD GPUs without NVIDIA

### Hardware Compatibility

| GPU Vendor | Support Level | Performance | Notes |
|-------------|--------------|-------------|-------|
| NVIDIA | Excellent | 35+ tok/s | CUDA backend preferred |
| AMD | Good | 20-25 tok/s | Vulkan mature |
| Intel | Maturing | 12-20 tok/s | Vulkan improving |
| ARM (Apple) | Limited | 5-10 tok/s | Metal backend preferred |

## References

- [llama.cpp Repository](https://github.com/ggml-org/llama.cpp)
- [Ollama Repository](https://github.com/ollama/ollama)
- [RAMalama](https://github.com/containers/ramalama)
- [Vulkan Specification](https://www.vulkan.org/)
- [Discussion: Local RAG](https://github.com/ggml-org/llama.cpp/discussions/3518)

---

*Last updated: April 13, 2026*
