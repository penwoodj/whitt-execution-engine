# Environment Variables

This document describes all environment variables supported by the Whitt Execution Engine framework.

## Overview

Environment variables can be configured through:
1. **.env file** in the project root (recommended for development)
2. **System environment variables** (recommended for production)
3. **Command-line arguments** (highest precedence)

**Precedence**: CLI arguments > System environment > .env file > defaults

## Security Best Practices

- **Never commit** `.env` files to version control
- Use `.env.example` as a template (already included in repo)
- Rotate API keys regularly
- Use minimal required permissions
- Store API keys in secure vaults for production deployments
- Limit tool execution with `ALLOWED_*` variables

## Variable Reference

### OpenAI-Compatible API Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `OPENAI_API_KEY` | Yes (for openai provider) | OpenAI API key for authentication | `sk-proj-abc123...` |
| `CUSTOM_SERVER_KEY` | Yes (for custom provider) | API key for custom/openai-compatible servers | `Bearer sk-xyz789...` |
| `CUSTOM_SERVER_URL` | Yes (for custom provider) | URL of custom API server | `http://localhost:11434/v1` |

#### Getting API Keys

- **OpenAI**: https://platform.openai.com/api-keys
- **Custom Server**: Provided by your server administrator

#### Usage Example

```bash
# Using OpenAI
export OPENAI_API_KEY=sk-proj-abc123...
whitt-execution-engine run workflow.yml

# Using Custom Server (Ollama)
export CUSTOM_SERVER_URL=http://localhost:11434/v1
export CUSTOM_SERVER_KEY=bearer-token-here
whitt-execution-engine run workflow.yml
```

---

### Ollama Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `OLLAMA_HOST` | Yes (for ollama provider) | Ollama server URL | `http://localhost:11434` |
| `OLLAMA_MODEL` | No (if in workflow) | Default model to use | `llama3.2`, `mistral`, `codellama` |

#### Setup Ollama

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull llama3.2

# Start Ollama server
ollama serve
```

#### Usage Example

```bash
# Set Ollama host
export OLLAMA_HOST=http://localhost:11434

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### LM Studio Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `LMSTUDIO_HOST` | Yes (for lmstudio provider) | LM Studio server URL | `http://localhost:1234/v1` |
| `LMSTUDIO_MODEL` | No (if in workflow) | Default model to use | `llama-3.2-3b-instruct` |

#### Setup LM Studio

1. Download LM Studio from https://lmstudio.ai/
2. Install and launch LM Studio
3. Download a model (e.g., Llama 3.2 3B Instruct)
4. Start the server (Server tab → Start Server)
5. Note the server URL (default: `http://localhost:1234/v1`)

#### Usage Example

```bash
# Set LM Studio host
export LMSTUDIO_HOST=http://localhost:1234/v1

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### llama.cpp Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `LLAMACPP_MODEL_PATH` | Yes (for llamacpp provider) | Path to GGUF model file | `./models/llama-3.2-3b.Q4_K_M.gguf` |
| `LLAMACPP_BACKEND` | No | Backend for inference | `vulkan`, `cuda`, `cpu`, `metal` |
| `LLAMACPP_CONTEXT_SIZE` | No | Context window size (tokens) | `4096` |
| `LLAMACPP_GPU_LAYERS` | No | Percentage of layers to offload to GPU | `99` |

#### Supported Backends

- **vulkan**: Cross-platform GPU acceleration (default)
- **cuda**: NVIDIA GPU acceleration (Linux/Windows)
- **cpu**: CPU-only (slower, no GPU required)
- **metal**: Apple Silicon GPU acceleration (macOS)

#### Usage Example

```bash
# Set llama.cpp configuration
export LLAMACPP_MODEL_PATH=./models/llama-3.2-3b.Q4_K_M.gguf
export LLAMACPP_BACKEND=vulkan
export LLAMACPP_CONTEXT_SIZE=4096
export LLAMACPP_GPU_LAYERS=99

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### Jina AI Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `JINAAI_API_KEY` | Yes (for jinaai provider) | Jina AI API key for embeddings | `jina-abc123...` |

#### Getting Jina AI API Key

1. Visit https://jina.ai/
2. Sign up for an account
3. Generate an API key
4. Set the environment variable

#### Usage Example

```bash
# Set Jina AI API key
export JINAAI_API_KEY=jina-abc123...

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### Workspace Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `WORKSPACE_ROOT` | No | Base directory for workspace operations | `./workspace` |
| `LOGS_DIR` | No | Directory for execution logs | `../workspace/logs` |
| `CHAT_DIR` | No | Directory for conversation history | `../workspace/chat` |
| `OUTPUT_DIR` | No | Directory for workflow output files | `../workspace/output` |
| `STATE_DIR` | No | Directory for execution state/checkpoints | `../workspace/state` |
| `CHECKPOINTS_DIR` | No | Directory for workflow checkpoints | `../workspace/checkpoints` |
| `METRICS_DIR` | No | Directory for performance metrics | `../workspace/metrics` |

#### Workspace Structure

```
workspace/
├── logs/              # Execution logs (JSON format)
├── chat/              # LLM conversation history
├── output/            # Workflow output files
├── state/             # Execution state and checkpoints
├── checkpoints/       # Workflow checkpoints
└── metrics/           # Performance metrics (JSON format)
```

#### Usage Example

```bash
# Custom workspace location
export WORKSPACE_ROOT=/tmp/my-workspace
export LOGS_DIR=/tmp/my-workspace/logs
export OUTPUT_DIR=/tmp/my-workspace/output

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### Execution Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `MAX_PARALLEL_EXECUTIONS` | No | Maximum parallel workflow steps | `3` |
| `DEFAULT_RETRY_STRATEGY` | No | Retry strategy for failed steps | `exponential` |
| `MAX_RETRY_ATTEMPTS` | No | Maximum retry attempts per step | `3` |
| `LOG_LEVEL` | No | Global logging verbosity | `info` |

#### Retry Strategies

- **exponential**: Exponential backoff (1s, 2s, 4s, 8s, 16s)
- **linear**: Linear backoff (1s, 2s, 3s, 4s, 5s)
- **fixed**: Fixed delay between retries
- **none**: No retry (fail immediately)

#### Log Levels

- **trace**: Extremely verbose (all execution details)
- **debug**: Debug information (development)
- **info**: General information (default)
- **warn**: Warnings only
- **error**: Errors only

#### Usage Example

```bash
# Configure execution
export MAX_PARALLEL_EXECUTIONS=5
export DEFAULT_RETRY_STRATEGY=exponential
export MAX_RETRY_ATTEMPTS=5
export LOG_LEVEL=debug

# Run workflow
whitt-execution-engine run workflow.yml
```

---

### Security Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `REQUIRE_TOOL_CONFIRMATION` | No | Require user confirmation for tools | `true` |
| `ALLOWED_SHELL_COMMANDS` | No | Comma-separated allowed shell commands | `cargo,rustc,git` |
| `ALLOWED_FILE_PATHS` | No | Comma-separated allowed file paths | `./src,./config,./output` |

#### Security Features

**Tool Confirmation**: When enabled, the framework will prompt for confirmation before:
- Executing shell commands
- Writing to files
- Deleting files

**Allowed Shell Commands**: Restrict which shell commands can be executed. If empty, all commands are allowed.

**Allowed File Paths**: Restrict which paths can be accessed for file operations. If `.*`, all paths in current directory are allowed.

#### Usage Example

```bash
# Enable security features
export REQUIRE_TOOL_CONFIRMATION=true
export ALLOWED_SHELL_COMMANDS=cargo,rustc,git,ls,cat
export ALLOWED_FILE_PATHS=./src,./config,./output

# Run workflow
whitt-execution-engine run workflow.yml
```

**Warning**: In production environments, always set `REQUIRE_TOOL_CONFIRMATION=false` with proper `ALLOWED_*` restrictions to prevent unauthorized operations.

---

### Model Memory Management

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `MAX_ALLOCATED_MEMORY_MB` | No | Maximum total memory for all models | `16384` |
| `MODEL_MEMORY_MB` | No | Memory allocated per model instance | `4096` |
| `UNLOAD_UNUSED_MODELS` | No | Auto-unload models not in active use | `true` |

#### Memory Allocation

The framework manages memory automatically:
1. Total memory is limited by `MAX_ALLOCATED_MEMORY_MB`
2. Each model instance gets `MODEL_MEMORY_MB`
3. Unused models are unloaded if `UNLOAD_UNUSED_MODELS=true`
4. Models are reloaded automatically when needed

#### Usage Example

```bash
# Configure memory management
export MAX_ALLOCATED_MEMORY_MB=32768
export MODEL_MEMORY_MB=8192
export UNLOAD_UNUSED_MODELS=true

# Run workflow
whitt-execution-engine run workflow.yml
```

**Tip**: For 16GB RAM systems:
- `MAX_ALLOCATED_MEMORY_MB=16384` (use 90% of RAM)
- `MODEL_MEMORY_MB=4096` (4 3B models or 2 7B models)

---

### Development Configuration

| Variable | Required | Description | Default | Example |
|-----------|-----------|-------------|----------|----------|
| `DEBUG_MODE` | No | Enable debug output and stack traces | `false` |
| `ENABLE_PROFILING` | No | Enable performance profiling | `false` |
| `PROFILES_DIR` | No | Directory for profile data | `../workspace/profiles` |

#### Debug Mode

When enabled (`DEBUG_MODE=true`):
- Additional verbose logging
- Stack traces on errors
- Internal state dumps
- Slower execution (due to overhead)

**Use for**: Development, debugging, troubleshooting

#### Profiling

When enabled (`ENABLE_PROFILING=true`):
- Performance metrics collection
- Execution timing breakdown
- Memory usage tracking
- Profile data saved to `PROFILES_DIR`

**Use for**: Performance optimization, bottleneck identification

#### Usage Example

```bash
# Enable debug mode for development
export DEBUG_MODE=true

# Enable profiling for optimization
export ENABLE_PROFILING=true
export PROFILES_DIR=../workspace/profiles

# Run workflow
whitt-execution-engine run workflow.yml
```

---

## Configuration File (workflow.yml)

Environment variables are referenced in workflow YAML using `${VARIABLE_NAME}` syntax:

```yaml
workflow_id: my_workflow
name: "My Workflow"

models:
  primary:
    provider: lmstudio  # Uses LMSTUDIO_HOST, LMSTUDIO_MODEL
    # Alternative: ollama (uses OLLAMA_HOST, OLLAMA_MODEL)
    # Alternative: llamacpp (uses LLAMACPP_MODEL_PATH)
    # Alternative: openai (uses OPENAI_API_KEY)

execution:
  mode: serial
  memory:
    max_allocated_memory_mb: ${MAX_ALLOCATED_MEMORY_MB:-16384}  # Use env var with default

agentic_workflow:
  - step: analyze
    model: "${models.primary}"
    input:
      prompt: "Analyze codebase"
```

**Default Values**: Use `${VARIABLE_NAME:-default}` to provide defaults.

---

## Troubleshooting

### Common Issues

**Issue**: "API key not found"
- **Solution**: Set `OPENAI_API_KEY`, `CUSTOM_SERVER_KEY`, or `JINAAI_API_KEY`

**Issue**: "Connection refused to localhost:11434"
- **Solution**: Ensure Ollama is running (`ollama serve`)

**Issue**: "LM Studio connection failed"
- **Solution**: Start LM Studio server (Server tab → Start Server)

**Issue**: "Model file not found"
- **Solution**: Set correct `LLAMACPP_MODEL_PATH` (absolute or relative path)

**Issue**: "Permission denied" on file operations
- **Solution**: Add path to `ALLOWED_FILE_PATHS` or set to `./`

**Issue**: "Out of memory"
- **Solution**: Reduce `MODEL_MEMORY_MB` or `MAX_ALLOCATED_MEMORY_MB`

### Debugging

Enable debug mode to troubleshoot:

```bash
export DEBUG_MODE=true
export LOG_LEVEL=trace

whitt-execution-engine run workflow.yml
```

Check logs for detailed information:

```bash
cat ./workspace/logs/workflow.log | jq .
```

---

## Additional Resources

- **Main Documentation**: See [README.md](README.md)
- **Example Workflows**: `opencode/docs/reports/requirements/example-workflows/`
- **Schema Documentation**: `opencode/docs/reports/requirements/schema-consolidated-report.md`
- **Installation Guide**: [INSTALL.md](INSTALL.md)
