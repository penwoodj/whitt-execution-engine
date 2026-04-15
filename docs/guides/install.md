# Installation Guide

This guide walks you through installing the YAML to Rust AgentSDK framework on Linux, macOS, and Windows.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Install Rust](#install-rust)
3. [Install the Framework](#install-the-framework)
4. [Install LLM Providers](#install-llm-providers)
5. [Configure the Framework](#configure-the-framework)
6. [Verify Installation](#verify-installation)
7. [Platform-Specific Issues](#platform-specific-issues)
8. [Uninstall](#uninstall)

---

## Prerequisites

### System Requirements

| Requirement | Minimum | Recommended |
|-------------|-----------|--------------|
| **RAM** | 8 GB | 16 GB+ |
| **Storage** | 2 GB free | 10 GB+ (for models) |
| **CPU** | Any modern 64-bit | 4+ cores |
| **GPU** | Optional | 4 GB+ VRAM (for llama.cpp Vulkan/CUDA) |

### Supported Platforms

- **Linux**: Ubuntu 20.04+, Debian 11+, Arch Linux, Fedora 35+
- **macOS**: macOS 11.0+ (Big Sur) with Apple Silicon (M1/M2/M3) recommended
- **Windows**: Windows 10+ (with WSL2) or Windows 11

### Software Requirements

- **Rust**: 1.70+ (required for framework)
- **Git**: 2.20+ (for cloning repository)
- **C Compiler**: GCC or Clang (for llama.cpp compilation)
- **Vulkan SDK** (optional, for GPU acceleration)
- **Python 3.8+** (optional, for some tools)

---

## Install Rust

### Linux

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source Rust environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### macOS

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source Rust environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Windows (WSL2)

```bash
# Open WSL2 terminal and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source Rust environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

---

## Install the Framework

### Clone the Repository

```bash
# Clone the repository
git clone https://github.com/penwoodj/whitt-execution-engine.git
cd whitt-execution-engine
```

### Build from Source

#### Development Build (Faster, Not Optimized)

```bash
# Build in debug mode
cargo build

# Run from debug build
cargo run -- --help
```

#### Release Build (Optimized, Production-Ready)

```bash
# Build in release mode with optimizations
cargo build --release

# Run from release build
./target/release/whitt-execution-engine --help
```

**Release Build Features**:
- Optimized binary (`opt-level 3`)
- Link-time optimization (LTO)
- Stripped symbols (smaller binary)
- Single codegen unit (better optimization)

### Installation via Cargo (When Published)

```bash
# Install directly from crates.io (when available)
cargo install whitt-execution-engine

# Verify installation
whitt-execution-engine --help
```

---

## Install LLM Providers

Choose one or more LLM providers based on your use case:

### Option 1: LM Studio (Recommended for Beginners)

**Pros**: GUI-based, easy model management, cross-platform
**Cons**: GUI required, not ideal for headless servers

#### Installation

**macOS**:
```bash
# Download LM Studio from https://lmstudio.ai/
# Install the .dmg file
# Launch LM Studio
# Download a model (e.g., Llama 3.2 3B Instruct)
```

**Windows**:
```bash
# Download LM Studio from https://lmstudio.ai/
# Install the .exe file
# Launch LM Studio
# Download a model
```

**Linux**:
```bash
# Download LM Studio AppImage from https://lmstudio.ai/
# Make executable
chmod +x LM-Studio-*.AppImage
# Run
./LM-Studio-*.AppImage
# Download a model
```

#### Start LM Studio Server

1. Open LM Studio
2. Go to the "Server" tab
3. Click "Start Server"
4. Note the server URL (default: `http://localhost:1234/v1`)
5. Download models through the interface

#### Configure Environment Variables

```bash
# Add to .env file
echo "LMSTUDIO_HOST=http://localhost:1234/v1" >> .env
echo "LMSTUDIO_MODEL=llama-3.2-3b-instruct" >> .env
```

---

### Option 2: Ollama (Recommended for Servers)

**Pros**: CLI-based, headless, easy automation
**Cons**: No GUI, model management via CLI

#### Installation

**Linux/macOS**:
```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Verify installation
ollama --version
```

**Windows (WSL2)**:
```bash
# Install Ollama in WSL2
curl -fsSL https://ollama.com/install.sh | sh

# Verify installation
ollama --version
```

#### Download Models

```bash
# Download a model
ollama pull llama3.2

# List available models
ollama list

# Download multiple models
ollama pull mistral
ollama pull codellama
```

#### Start Ollama Server

```bash
# Start Ollama server (runs in foreground)
ollama serve

# Start Ollama in background (Linux/macOS)
nohup ollama serve > /dev/null 2>&1 &

# Start Ollama as a service (systemd)
sudo systemctl enable ollama
sudo systemctl start ollama
```

#### Configure Environment Variables

```bash
# Add to .env file
echo "OLLAMA_HOST=http://localhost:11434" >> .env
echo "OLLAMA_MODEL=llama3.2" >> .env
```

---

### Option 3: llama.cpp (Maximum Performance)

**Pros**: Direct GGUF loading, maximum performance, flexible backends
**Cons**: Requires compilation, manual model management

#### Prerequisites

**Linux**:
```bash
# Install build dependencies
sudo apt-get update
sudo apt-get install -y build-essential git cmake

# Install Vulkan SDK (for GPU acceleration)
wget -qO - https://packages.lunarg.com/lunarg-signing-key-pub.asc | sudo apt-key add -
sudo wget -qO /etc/apt/sources.list.d/lunarg-vulkan-1.2.196.list https://packages.lunarg.com/vulkan/1.2.196/lunarg-vulkan-1.2.196-focal.list
sudo apt-get update
sudo apt-get install -y vulkan-sdk
```

**macOS**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install cmake
brew install cmake
```

**Windows (WSL2)**:
```bash
# Install build dependencies
sudo apt-get update
sudo apt-get install -y build-essential git cmake

# Install Vulkan SDK (optional)
sudo apt-get install -y mesa-vulkan-drivers
```

#### Build llama.cpp

```bash
# Clone llama.cpp repository
git clone https://github.com/ggerganov/llama.cpp.git
cd llama.cpp

# Build with Vulkan backend (Linux)
cmake -B build -DLLAMA_VULKAN=ON
cmake --build build -j

# Build with Metal backend (macOS)
cmake -B build -DLLAMA_METAL=ON
cmake --build build -j

# Build with CPU backend (Windows/WSL2)
cmake -B build -DLLAMA_CUBLAS=OFF
cmake --build build -j
```

#### Download GGUF Models

```bash
# Download quantized models from Hugging Face
# Example: Llama 3.2 3B Instruct (Q4_K_M)
wget https://huggingface.co/MaziyarPanahi/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf -P ./models/
```

#### Configure Environment Variables

```bash
# Add to .env file
echo "LLAMACPP_MODEL_PATH=./models/Llama-3.2-3B-Instruct-Q4_K_M.gguf" >> .env
echo "LLAMACPP_BACKEND=vulkan" >> .env
echo "LLAMACPP_CONTEXT_SIZE=4096" >> .env
```

---

### Option 4: OpenAI (Cloud Fallback)

**Pros**: High-quality models, no local hardware required
**Cons**: Requires API key, costs money, network dependency

#### Get API Key

1. Visit https://platform.openai.com/api-keys
2. Sign up or log in
3. Create an API key
4. Copy the key

#### Configure Environment Variables

```bash
# Add to .env file
echo "OPENAI_API_KEY=sk-proj-your-api-key-here" >> .env
```

---

### Option 5: Jina AI (Embeddings)

**Pros**: High-performance embeddings, good for RAG
**Cons**: Requires API key, specialized use case

#### Get API Key

1. Visit https://jina.ai/
2. Sign up for an account
3. Generate an API key
4. Copy the key

#### Configure Environment Variables

```bash
# Add to .env file
echo "JINAAI_API_KEY=jina-your-api-key-here" >> .env
```

---

## Configure the Framework

### Create Environment File

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env with your configuration
nano .env  # or vim .env
```

### Basic Configuration

```bash
# Set your preferred provider
LMSTUDIO_HOST=http://localhost:1234/v1
LMSTUDIO_MODEL=llama-3.2-3b-instruct

# Set workspace location
WORKSPACE_ROOT=./workspace

# Set log level
LOG_LEVEL=info
```

### Advanced Configuration

See [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) for all available configuration options.

---

## Verify Installation

### 1. Verify Rust Installation

```bash
# Check Rust version
rustc --version

# Expected output: rustc 1.70.0 or higher
```

### 2. Verify Framework Installation

```bash
# Build the framework
cargo build --release

# Run with --help flag
./target/release/whitt-execution-engine --help

# Expected output: Usage information
```

### 3. Verify LLM Provider

**For LM Studio**:
```bash
# Test connection
curl http://localhost:1234/v1/models

# Expected output: JSON with available models
```

**For Ollama**:
```bash
# Test connection
curl http://localhost:11434/api/tags

# Expected output: JSON with available models
```

**For OpenAI**:
```bash
# Test connection (requires OPENAI_API_KEY)
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY"

# Expected output: JSON with available models
```

### 4. Run Test Workflow

Create `test_workflow.yml`:

```yaml
workflow_id: test_installation
name: "Installation Test Workflow"

models:
  primary:
    provider: lmstudio  # Change to your provider
    model: "llama-3.2-3b-instruct"

execution:
  mode: serial

agentic_workflow:
  - step: test
    id: step_1
    model: "${models.primary}"
    input:
      prompt: "Say 'Installation successful!'"
    output:
      save_to: test_output
      format: text
```

Run the test workflow:

```bash
# Execute workflow
cargo run -- run test_workflow.yml

# Check output
cat ./workspace/output/test_output

# Expected output: "Installation successful!"
```

---

## Platform-Specific Issues

### Linux

**Issue**: Permission denied when running cargo
**Solution**: Add user to appropriate groups
```bash
sudo usermod -aG docker $USER
sudo usermod -aG render $USER
```

**Issue**: Vulkan not found
**Solution**: Install Vulkan drivers
```bash
sudo apt-get install -y mesa-vulkan-drivers
```

**Issue**: Out of memory during build
**Solution**: Limit parallel jobs
```bash
CARGO_BUILD_JOBS=2 cargo build --release
```

### macOS

**Issue**: Command line tools not found
**Solution**: Install Xcode Command Line Tools
```bash
xcode-select --install
```

**Issue**: Metal backend fails
**Solution**: Update macOS and Xcode
```bash
softwareupdate --all --install
```

**Issue**: Binary not trusted (Gatekeeper)
**Solution**: Allow binary to run
```bash
xattr -cr ./target/release/whitt-execution-engine
```

### Windows (WSL2)

**Issue**: GPU not accessible in WSL2
**Solution**: Install WSL2 GPU drivers
```bash
# Download from NVIDIA: https://developer.nvidia.com/cuda/wsl2
# Install in Windows, restart WSL2
```

**Issue**: Slow file system performance
**Solution**: Move workspace to Linux FS
```bash
# Create workspace in /tmp (not /mnt/c/)
export WORKSPACE_ROOT=/tmp/workspace
```

---

## Uninstall

### Uninstall the Framework

```bash
# Remove source directory
rm -rf whitt-execution-engine

# Remove binary (if installed via cargo)
cargo uninstall whitt-execution-engine

# Remove workspace (optional)
rm -rf ./workspace
```

### Uninstall Rust

```bash
# Remove Rust toolchain
rustup self uninstall

# Remove .cargo and .rustup directories
rm -rf ~/.cargo ~/.rustup
```

### Uninstall LLM Providers

**LM Studio**: Uninstall through application menu
**Ollama**: `ollama uninstall`
**llama.cpp**: `rm -rf llama.cpp`
**Vulkan SDK**: Use package manager (`apt-get remove vulkan-sdk`)

---

## Next Steps

- [ ] Read [README.md](README.md) for framework overview
- [ ] Review [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) for configuration
- [ ] Explore [example workflows](opencode/docs/reports/requirements/example-workflows/)
- [ ] Run your first workflow
- [ ] Generate Rust code from your workflow

---

## Troubleshooting

### Common Installation Errors

**"cargo: command not found"**
- Solution: Install Rust and source environment (see [Install Rust](#install-rust))

**"error: linker `cc` not found"**
- Solution: Install C compiler (`sudo apt-get install build-essential` on Linux)

**"Vulkan not found"**
- Solution: Install Vulkan SDK (see [llama.cpp](#option-3-llamacpp-maximum-performance))

**"Connection refused"**
- Solution: Ensure LLM provider is running and accessible

**"Out of memory"**
- Solution: Reduce `MODEL_MEMORY_MB` or `MAX_ALLOCATED_MEMORY_MB` in `.env`

### Getting Help

- **Documentation**: [README.md](README.md)
- **Environment Variables**: [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md)
- **Example Workflows**: `opencode/docs/reports/requirements/example-workflows/`
- **Issues**: https://github.com/penwoodj/whitt-execution-engine/issues

---

## System Requirements Summary

| Component | Minimum | Recommended |
|-----------|-----------|--------------|
| **OS** | Linux/macOS/Windows | Ubuntu 22.04+, macOS 12+ |
| **RAM** | 8 GB | 16 GB+ |
| **Storage** | 2 GB | 10 GB+ |
| **CPU** | Any 64-bit | 4+ cores |
| **GPU** | None | 4 GB+ VRAM |
| **Rust** | 1.70+ | 1.75+ |
| **Network** | Required (for providers) | Broadband |

---

## Version Compatibility

| Framework Version | Rust Version | Ollama Version | LM Studio Version |
|-----------------|----------------|------------------|-------------------|
| 0.1.0 | 1.70+ | 0.1.0+ | 0.2.0+ |

---

**Last Updated**: April 2026
**Framework Version**: 0.1.0
