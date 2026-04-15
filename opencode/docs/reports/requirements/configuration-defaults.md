# Configuration Defaults

## Overview

Default configurations for AutoAgents SDK derived from unified schema requirements, manual workflow patterns, and backend specifications. These defaults provide reasonable out-of-the-box behavior while remaining configurable.

---

## Unified Schema Defaults

### Root-Level Defaults

These defaults apply to all workflows unless overridden in YAML.

```yaml
# Default values (can be overridden)
name: "workflow_name"
version: "1.0"
description: "Default workflow description"
tags: []

workflow:
  type: "sequential"  # sequential only — parallel moved to agent-queue
  # concurrency: → moved to agent-queue project

  execution:
    timeout_secs: 300        # 5 minutes default
    retry_max_attempts: 3
    retry_backoff_secs: [5, 15, 60]  # Exponential backoff

  output:
    save_to: "./workspace/output"
    format: "json"  # json | yaml | markdown

  logging:
    level: "info"  # debug | info | warn | error
    scopes:
      - name: "workflow"
        format: "structured"  # structured | minimal | silent
        output_type: "file"
        log_dir: "./workspace/logs"

  memory:
    strategy: "auto"  # auto | conservative | aggressive
    unload_after_secs: 300  # Unload models after 5 minutes idle

  state_management:
    checkpoint_interval_secs: 60  # Save state every minute
    state_location: "./workspace/state"
```

**Rationale:**
- **5-minute timeout**: Balanced for interactive vs batch workflows
- **3 retry attempts**: Handles transient failures without infinite loops
- **JSON output**: Machine-readable, easy to parse, smaller than YAML
- **Info logging**: Balanced verbosity (not too noisy, not too quiet)
- **Auto memory strategy**: Let system decide based on available RAM
- **1-minute checkpoint**: Protects against crashes without excessive I/O

---

## Step-Level Defaults

### Simple Step Defaults

```yaml
steps:
  - name: "generate_text"
    type: "simple"
    tools: [llm_generate]
    timeout_secs: 60
    retries: 1
    on_error: "continue"  # continue | fail | skip_step
```

**Fields:**
- `timeout_secs`: Default 60 seconds (reasonable for most LLM operations)
- `retries`: Default 1 (retry once on transient failure)
- `on_error`: Default "continue" (continue workflow on non-critical errors)

### Loop Step Defaults

```yaml
steps:
  - name: "process_items"
    type: "loop"
    max_iterations: 100
    on_empty_iterations: "fail"  # fail | skip | continue
    on_max_iterations: "continue"
    on_consecutive_failures: 3
    timeout_secs: 300
```

**Fields:**
- `max_iterations`: Default 100 (prevent infinite loops)
- `on_empty_iterations`: Default "fail" (fail if loop produces no output)
- `on_max_iterations`: Default "continue" (save state and continue)
- `on_consecutive_failures`: Default 3 (stop after 3 failures in a row)
- `timeout_secs`: Default 300 seconds (5 minutes per loop)

### Parallel Step Defaults

> **Note**: Parallel execution features (`parallel_group`, `max_parallel_workflows`, `parallel_group_timeout_secs`) have moved to the [agent-queue](https://github.com/penwoodj/agent-queue) project. This section describes historical schema defaults.

```yaml
steps:
  - name: "parallel_analysis"
    type: "parallel"
    parallel_group: "analysis_group"
    parallel_group_timeout_secs: 600
    aggregate_strategy: "merge_all"  # merge_all | vote_first | collect_all | first_success
    max_parallel_workflows: 4
```

**Fields:**
- `parallel_group_timeout_secs`: Default 600 seconds (10 minutes for entire parallel group)
- `aggregate_strategy`: Default "merge_all" (combine all parallel outputs)
- `max_parallel_workflows`: Default 4 (max concurrent sub-workflows)

---

## Backend Defaults

### LM Studio Backend

```yaml
models:
  - id: "llama-3-8b"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
```

**Default Configuration:**
```yaml
lmstudio:
  host: "localhost"
  port: 1234
  timeout_secs: 120
  max_retries: 3
  streaming: true
  model_load_timeout_secs: 60
```

**Rationale:**
- **Localhost + port 1234**: Default LM Studio configuration
- **120s timeout**: 2 minutes allows complex prompts without hanging
- **Streaming enabled**: Default true (interactive workflows need streaming)
- **60s model load timeout**: Balance between fast startup and allowing large model load

### Ollama Backend

```yaml
models:
  - id: "llama-3.1-8b"
    provider: "ollama"
    host: "localhost"
    port: 11434
```

**Default Configuration:**
```yaml
ollama:
  host: "localhost"
  port: 11434
  timeout_secs: 60
  max_retries: 2
  streaming: true
  num_ctx: 4096  # Default context window size
  num_thread: 8
```

**Rationale:**
- **Port 11434**: Default Ollama HTTP server port
- **60s timeout**: 1 minute sufficient for typical operations
- **2 retries**: Ollama generally stable
- **num_ctx: 4096**: 4K tokens (balanced for 8B model)
- **num_thread: 8**: 1 thread per GB VRAM (heuristic)

### llama.cpp Backend

```yaml
models:
  - id: "llama-7b"
    provider: "llamacpp"
    host: "localhost"
    port: 8080
```

**Default Configuration:**
```yaml
llamacpp:
  host: "localhost"
  port: 8080
  timeout_secs: 60
  max_retries: 3
  streaming: true
  num_ctx: 4096
  n_batch: 512
  n_ubatch: 32
  n_gpu_layers: -1  # -1 = use all layers if GPU memory allows
  context_window: 0  # 0 = use model default
```

**Rationale:**
- **Port 8080**: Default llama.cpp server port
- **60s timeout**: 1 minute for most generations
- **3 retries**: llama.cpp may have transient memory issues
- **num_ctx: 4096**: 4K tokens context window
- **n_batch: 512**: Batch size for throughput vs. latency balance
- **n_ubatch: 32**: Micro-batch size for parallel token generation
- **n_gpu_layers: -1**: Use all GPU layers (let GPU memory be limit)
- **context_window: 0**: Use model's default context size

---

## Tool Defaults

### Built-in Tools

```yaml
tools:
  - name: "file_read"
    type: "built_in"
    timeout_secs: 30
    max_file_size_mb: 100

  - name: "file_write"
    type: "built_in"
    timeout_secs: 30
    create_directories: true

  - name: "shell"
    type: "built_in"
    timeout_secs: 60
    allowed_commands: []
    sandbox: true

  - name: "web_fetch"
    type: "built_in"
    timeout_secs: 60
    max_response_size_mb: 10
    follow_redirects: 5
```

**Built-in Tool Defaults:**
- **30s timeout for file operations**: Fast file access
- **60s timeout for shell commands**: Allow longer-running scripts
- **100MB max file size**: Prevent reading massive files accidentally
- **10MB max response for web**: Prevent downloading huge responses
- **5 redirect limit**: Prevent redirect loops

### Custom Tool Defaults

```yaml
tools:
  - name: "custom_api"
    type: "custom"
    executable: "/path/to/tool"
    timeout_secs: 300
    env_vars:
      - name: "API_KEY"
        required: true
      - name: "ENDPOINT"
        required: true
```

**Custom Tool Defaults:**
- **5-minute timeout**: Allow external tools time to complete
- **Required env vars**: List must be provided or error raised
- **Executable path**: Must exist and be executable

---

## CLI Defaults

### Default Command Behavior

```yaml
cli:
  output_format: "structured"  # structured | minimal | silent | json
  colors: true
  progress: true
  confirm: false
  verbose: false
  editor: "vi"  # or "vim", "nano", "code"
  pager: "less"  # or "more", "cat"
```

**Rationale:**
- **Structured output**: Rich output with metadata, timestamps, formatting
- **Colors enabled**: Better readability (can be disabled)
- **Progress enabled**: Show spinners/progress bars during long operations
- **No confirm**: Don't prompt for destructive operations (faster automation)
- **Not verbose**: Don't log debug info by default (use --verbose to enable)
- **vi editor**: Default editor (familiar to many developers)

### Global CLI Flags

```bash
# Default behavior
agent-cli run workflow.yml

# With overrides
agent-cli run workflow.yml --timeout 600 --retries 5 --no-streaming

# Verbose mode
agent-cli run workflow.yml --verbose --debug

# Silent mode
agent-cli run workflow.yml --silent --no-progress
```

**Flag Descriptions:**
- `--timeout SECS`: Override default step timeout
- `--retries N`: Override default retry count
- `--no-streaming`: Disable streaming for non-interactive workflows
- `--verbose`: Enable debug logging
- `--debug`: Enable detailed diagnostics
- `--silent`: Suppress all non-error output
- `--no-progress`: Disable progress bars

---

## Environment Variable Defaults

### Default Environment Variables

```bash
# Agent SDK environment
export AGENT_SDK_HOME="$HOME/.agent-sdk"
export AGENT_SDK_LOG_LEVEL="info"
export AGENT_SDK_MAX_PARALLEL=4
export AGENT_SDK_MEMORY_LIMIT_GB=16
export AGENT_SDK_STATE_DIR="$HOME/.agent-sdk/state"

# Backend-specific
export LLAMACPP_HOST="localhost"
export LLAMACPP_PORT="8080"
export OLLAMA_HOST="localhost"
export OLLAMA_PORT="11434"
export LMSTUDIO_HOST="localhost"
export LMSTUDIO_PORT="1234"
```

**Variable Descriptions:**
- `AGENT_SDK_HOME`: SDK installation directory
- `AGENT_SDK_LOG_LEVEL`: Logging threshold (debug, info, warn, error)
- `AGENT_SDK_MAX_PARALLEL`: Max concurrent workflows (global limit)
- `AGENT_SDK_MEMORY_LIMIT_GB`: System memory budget
- `AGENT_SDK_STATE_DIR`: State storage location

---

## Runtime Defaults

### Execution Engine Defaults

```yaml
runtime:
  scheduler: "simple"  # simple | priority | fair
  state_backend: "memory"  # memory | sled | postgresql
  checkpoint_enabled: true
  checkpoint_interval_secs: 60
  metrics_enabled: true
  metrics_interval_secs: 10

  memory:
    model_cache_size_mb: 1024  # 1GB cache for model weights
    kv_cache_size_mb: 512      # 512MB cache for activations
    workflow_state_size_mb: 256   # 256MB for workflow state
    max_workflow_memory_mb: 4096  # 4GB per workflow
```

**Rationale:**
- **Simple scheduler**: First-come-first-served (good for interactive)
- **Memory state backend**: Faster than disk, no persistence overhead
- **1-minute checkpoints**: Protect against crashes without excessive I/O
- **Metrics enabled**: Default on for observability
- **10s metrics interval**: Frequent enough without overwhelming storage

### Concurrency Defaults

```yaml
concurrency:
  workflow_level: 4
  model_level: 1
  step_level: 16

  limits:
    max_total_requests: 64
    max_requests_per_model: 8
    max_total_loaded_models: 2
```

**Rationale:**
- **4 workflows max**: Prevent system overload
- **1 model loaded at once**: Respect GPU memory limits
- **16 parallel steps**: Reasonable parallelism within workflow
- **8 requests per model**: Prevent rate limiting from local providers
- **64 total requests**: Global cap for system stability

---

## Validation Defaults

### Schema Validation Defaults

```yaml
validation:
  strict_mode: false
  require_descriptions: true
  require_tags: false
  require_examples: false
  warn_on_deprecated: true
  fail_on_unknown_fields: true
```

**Rationale:**
- **Not strict mode**: Allow workflows with optional/deprecated fields (backward compatible)
- **Require descriptions**: Best practice, documentation quality
- **Tags optional**: Don't require tags (reduces friction)
- **Examples optional**: Don't require examples (simple workflows)
- **Warn on deprecated**: Inform users without blocking
- **Fail on unknown fields**: Strict schema enforcement (catch typos)

### Linting Defaults

```yaml
linting:
  enabled: true
  warn_on_timeout: true
  warn_on_ambiguous_names: true
  warn_on_large_files: true

  rules:
    timeout_warning_threshold_secs: 120
    name_ambiguity_threshold: 5
    large_file_threshold_mb: 10
```

**Rationale:**
- **Linting enabled**: Improve workflow quality
- **Warn on timeout >2min**: Alert user to potentially hung operations
- **Warn on ambiguous names**: Prevent confusion (5+ steps with similar names)
- **Warn on large files**: Alert if files >10MB in workflow
- **120s warning threshold**: Reasonable threshold (most operations complete faster)

---

## Default Values Summary

| Category | Default | Override Via | Description |
|-----------|---------|--------------|-------------|
| **Workflow timeout** | 300s (5min) | `workflow.execution.timeout_secs` | Per-operation timeout |
| **Retry attempts** | 3 | `workflow.execution.retry_max_attempts` | Transient failure recovery |
| **Backoff sequence** | [5, 15, 60]s | `workflow.execution.retry_backoff_secs` | Exponential retry delay |
| **Max iterations (loop)** | 100 | `steps.type.loop.max_iterations` | Prevent infinite loops |
| **Context size** | 4096 tokens | `models.*.num_ctx` | Balanced for 8B models |
| **Memory strategy** | auto | `workflow.memory.strategy` | System chooses conservative/aggressive |
| **Checkpoint interval** | 60s | `workflow.state_management.checkpoint_interval_secs` | Crash recovery |
| **Log level** | info | `cli.log_level` | Balanced verbosity |
| **Parallel workflows** | 4 | `concurrency.workflow_level` | System stability |
| **Parallel steps** | 16 | `concurrency.step_level` | Workflow parallelism |

---

## Configuration File Format

### Default Config File

**Location**: `$HOME/.agent-sdk/config.yml`

```yaml
# AutoAgents SDK Configuration

version: 1

defaults:
  workflow:
    type: "sequential"
    timeout_secs: 300
    retries: 3
    # concurrency: → moved to agent-queue project

  backends:
    lmstudio:
      host: "localhost"
      port: 1234
      timeout_secs: 120

    ollama:
      host: "localhost"
      port: 11434
      timeout_secs: 60

    llamacpp:
      host: "localhost"
      port: 8080
      timeout_secs: 60
      num_ctx: 4096
      n_batch: 512
      streaming: true

  cli:
    output_format: "structured"
    colors: true
    progress: true

  logging:
    level: "info"
    scopes:
      - name: "workflow"
        format: "structured"
        output_type: "file"
        log_dir: "./workspace/logs"

  memory:
    strategy: "auto"
    model_cache_size_mb: 1024
    unload_after_secs: 300

  validation:
    strict_mode: false
    fail_on_unknown_fields: true

  linting:
    enabled: true
    warn_on_timeout: true
```

### Workspace Configuration

**Location**: `./workspace/config.yml` (project-specific override)

```yaml
# Project-specific defaults override global config
defaults:
  workflow:
    timeout_secs: 600  # Longer timeout for this project
    memory:
      strategy: "aggressive"  # Use more memory for performance

  backends:
    llamacpp:
      n_gpu_layers: 30  # Limit to 30 layers to fit in GPU memory
```

---

## Implementation Notes

### Default Resolution Order

1. **Explicit YAML value** (if present)
2. **Environment variable** (if set)
3. **Project config file** (`./workspace/config.yml`)
4. **Global config file** (`$HOME/.agent-sdk/config.yml`)
5. **Built-in default** (last resort)

### Configuration Validation

1. **Validate at load time**: Ensure all required fields present, types correct
2. **Apply constraints**: Ensure values within acceptable ranges
3. **Warn on deprecated**: Inform users of outdated configs
4. **Override warnings**: Log when project config overrides global config

### Schema Integration

All default values must be compatible with unified schema v2.0:

| Schema Section | Compatible Field | Default Value | Type |
|---------------|-----------------|---------------|------|
| `workflow.type` | ✅ | "sequential" | string |
| `workflow.execution.timeout_secs` | ✅ | 300 | number |
| `workflow.execution.retry_max_attempts` | ✅ | 3 | number |
| `workflow.memory.strategy` | ✅ | "auto" | string |
| `models.*.num_ctx` | ✅ | 4096 | number |
| `models.*.n_batch` | ✅ (llamacpp) | 512 | number |
| `concurrency.workflow_level` | ✅ | 4 | number |
| `logging.level` | ✅ | "info" | string |

---

## Migration Guide

### Upgrading from v1 Configs

**v1 configuration files** (legacy):
```yaml
# Old format
timeout: 300
provider: "llamacpp"
```

**Migration to v2:**
1. Add `models:` section with provider, host, port
2. Rename `timeout` → `workflow.execution.timeout_secs`
3. Add `backends:` section with backend-specific configs
4. Add `logging:` section (previously implicit)

**Tooling**: Migration script provided to convert v1 → v2 configs

```bash
agent-cli migrate-config --from v1 --to v2
```

---

## Next Steps

1. Implement configuration file parser with default resolution order
2. Add configuration validation logic
3. Integrate with unified schema (ensure defaults are schema-valid)
4. Implement migration tool for v1 → v2 configs
5. Add CLI commands for config inspection and editing
6. Document all defaults with rationale in code comments
