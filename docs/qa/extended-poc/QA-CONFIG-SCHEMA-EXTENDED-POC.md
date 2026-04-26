# QA Config Schema — Extended POC

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Date**: 2026-04-26
**Status**: 🔴 NOT STARTED — Schema definition for QA testing

---

## Unified Workflow YAML Schema (v2.0)

This document defines the YAML schema structure that the extended POC must parse and validate. All structs map to unified-workflow-schema.yml (Lines 14-805).

### Top-Level Structure

```yaml
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config: { ... }
    hosting: { ... }
    requests: { ... }

models:
  global_config_path: "./workspace/model-config.yml"
  default_router: automatic
  "model-id":
    name: "..."
    host: { ... }
    # ...

steps:
  step-name:
    generative_entity: "${models.model-id}"
    prompt: |
      ...
    model_overrides: { ... }
    retry: { ... }
```

---

## Provider Config Schema

### ProviderConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| schema_version | String | No | "2.0.0" | Semver format |
| llama_cpp_with_vulkan | LlamaCppVulkanProvider | No | None | - |

### LlamaCppVulkanProvider

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| config | LlamaCppConfig | No | (defaults) | - |
| config_file | String | No | None | Valid path (not in POC) |

### ConnectionConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| host | String | No | "localhost" | - |
| port | u16 | No | 8080 | garde(range 1..=65535) |
| connection_timeout_secs | u64 | No | 30 | garde(range min=1) |

### HostingConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| max_concurrent_models | usize | No | 1 | garde(range min=1) |
| model_offload_timeout_secs | u64 | No | 60 | garde(range min=1) |
| gpu_allocation | GpuAllocation | No | None | - |
| cpu_fallback | CpuFallback | No | None | - |

### GpuAllocation

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| vram_per_model_mb | usize | No | 4096 | garde(range min=1) |

### CpuFallback

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| cpu_cores_per_model | usize | No | 4 | garde(range min=1) |

### RequestsConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| max_concurrent_requests | usize | No | 1 | garde(range min=1) |
| request_timeout_secs | u64 | No | 600 | garde(range min=1) |
| queue_timeout_secs | u64 | No | 300 | garde(range min=1) |
| rate_limit_per_minute | usize | No | 60 | garde(range min=1) |
| retry | RetryConfig | No | None | - |

### RetryConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| max_retries | u32 | No | 3 | garde(range min=0) |
| backoff | BackoffStrategy | No | Exponential | Enum: exponential, linear, fixed |
| initial_delay | String | No | "1s" | Duration format |
| max_delay | String | No | "30s" | Duration format |
| multiplier | f64 | No | 2.0 | garde(range min=1.0) |
| jitter | bool | No | true | - |

---

## Model Schema

### ModelsConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| schema_version | String | No | "2.0.0" | Semver |
| global_config_path | String | No | None | Valid path |
| default_router | RouterStrategy | No | Automatic | Enum: automatic, manual |
| models | HashMap<String, ModelSpec> | No | {} | - |

### ModelSpec

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| name | String | Yes | - | Non-empty |
| host | ModelHost | Yes | - | - |
| ram_allocation | RamAllocation | No | None | - |
| max_allowed | MaxAllowed | No | (defaults) | - |
| min_allowed | MinAllowed | No | (defaults) | - |
| model_memory | ModelMemory | No | (defaults) | - |
| execution | ExecutionConfig | No | (defaults) | - |
| thinking | ThinkingConfig | No | None | - |

### ResourceLimit (enum)

| Variant | Format | Example |
|---------|--------|---------|
| Percentage | "{N}%" | "13%", "74%" |
| Absolute | "{N}GB" or "{N}MB" | "3.7GB", "512MB" |

### ExecutionConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| timeout | TimeoutConfig | No | (defaults) | - |
| max_turns | usize | No | 10 | Positive |
| stop_on_tool_failure | bool | No | false | - |
| accumulate_tool_results | bool | No | true | - |

### ThinkingConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| budget_tokens | usize | No | 4096 | 0-16384 |
| capture_in_output | bool | No | false | - |
| capture_in_events | bool | No | false | - |

---

## Step Schema

### StepConfig

| Field | Type | Required | Default | Validation |
|-------|------|----------|---------|------------|
| generative_entity | String | Yes | - | Must resolve to model ID |
| prompt | String | Yes | - | Non-empty |
| model_overrides | HashMap | No | {} | Override execution config |
| retry | RetryConfig | No | None | Same as provider retry |

---

## Template Interpolation

### Parse-Time Variables (${...})

| Pattern | Resolves To | Example |
|---------|-------------|---------|
| `${models.model-id}` | Model ID from models section | `${models.primary-analyzer}` → `primary-analyzer` |

### Runtime Variables ({{...}}) — NOT in POC

| Pattern | Resolves To | Example |
|---------|-------------|---------|
| `{{step.name.output}}` | Output from previous step | Deferred to Phase 2 |

---

## Validation Rules Summary

| Rule | Level | Message |
|------|-------|---------|
| port: 1..=65535 | Error | "port must be between 1 and 65535" |
| connection_timeout_secs >= 1 | Error | "connection_timeout_secs must be >= 1" |
| max_concurrent_models >= 1 | Error | "max_concurrent_models must be >= 1" |
| request_timeout_secs >= 1 | Error | "request_timeout_secs must be >= 1" |
| max_retries >= 0 | Error | "max_retries must be >= 0" |
| multiplier >= 1.0 | Error | "multiplier must be >= 1.0" |
| schema_version compatible | Error | "Unsupported schema version: X.X.X" |
| generative_entity resolves | Error | "Model reference not found: ${models.X}" |
| Unknown YAML keys | Warning | "Unknown field ignored: X" |

---

## Test Configurations

### Minimal Valid Config

```yaml
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config: {}

models:
  "test-model":
    name: "Test Model"
    host:
      type: llama_cpp_with_vulkan
      connection_settings: {}

steps:
  test-step:
    generative_entity: "${models.test-model}"
    prompt: "Hello"
```

### Full Config (All Fields)

```yaml
schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
      connection_timeout_secs: 30
    hosting:
      max_concurrent_models: 2
      model_offload_timeout_secs: 120
      gpu_allocation:
        vram_per_model_mb: 8192
      cpu_fallback:
        cpu_cores_per_model: 8
    requests:
      max_concurrent_requests: 4
      request_timeout_secs: 1200
      queue_timeout_secs: 600
      rate_limit_per_minute: 120
      retry:
        max_retries: 5
        backoff: exponential
        initial_delay: "2s"
        max_delay: "60s"
        multiplier: 3.0
        jitter: false

models:
  global_config_path: "./workspace/model-config.yml"
  default_router: automatic

  "primary-analyzer":
    name: "Primary Code Analyzer"
    host:
      type: llama_cpp_with_vulkan
      connection_settings: {}

    ram_allocation:
      strategy: dynamic
    max_allowed:
      ram: 13%
      vram: 3.7GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 150000
      concurrent_requests: 2
    min_allowed:
      ram: 9%
      vram: 2.4GB

    model_memory:
      cache_size: min
      kv_cache_quantization: auto
      attention_context: auto

    execution:
      timeout:
        load_into_memory: 45s
        time_to_first_response: 1m
        total_time_to_response: 4h
      max_turns: 10
      stop_on_tool_failure: false
      accumulate_tool_results: true

    thinking:
      budget_tokens: 4096
      capture_in_output: true
      capture_in_events: true

steps:
  analyze:
    generative_entity: "${models.primary-analyzer}"
    prompt: |
      Analyze the following code for issues:
      {{step.previous.output}}
    model_overrides:
      max_turns: 5
    retry:
      max_attempts: 3
      backoff: exponential
      initial_delay: "1s"
      max_delay: "30s"
      multiplier: 2.0
      jitter: true
```

### Invalid Configs for Validation Testing

```yaml
# Invalid port
providers:
  llama_cpp_with_vulkan:
    config:
      config:
        port: 99999  # > 65535
```

```yaml
# Invalid multiplier
providers:
  llama_cpp_with_vulkan:
    config:
      requests:
        retry:
          multiplier: 0.5  # < 1.0
```

```yaml
# Invalid schema version
schema_version: "3.0.0"  # Unsupported
```
