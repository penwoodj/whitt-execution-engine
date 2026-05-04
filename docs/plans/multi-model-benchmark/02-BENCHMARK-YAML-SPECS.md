# Benchmark YAML Specifications

## Overview

Generate 4 YAML benchmark workflow files conforming to `docs/schema/unified-workflow-schema.yml` (v2.0.0+). Each file defines a serial workflow that sequentially loads, benchmarks, and unloads N models.

## YAML Template Structure

All benchmark YAML files follow this template:

```yaml
workflow_id: benchmark_N_models
name: "N-Model Benchmark Suite"
description: "Sequential benchmark of N diverse models from external drive"
min_schema_version: "2.0.0"

models:
  primary:
    provider: lmstudio
    model: "${benchmark.current_model}"

execution:
  mode: serial
  memory:
    max_allocated_memory_mb: 8192
    model_memory_mb: 6144        # 6GB max per model (8GB VRAM - 2GB headroom)
    unload_unused: true

logging:
  global:
    level: info
    detail: medium
    output_type: chat
    format: json
    console: true
  performance_metrics:
    level: debug
    detail: very_high

benchmark:
  prompts:
    - "Explain the concept of recursion in programming, with a practical example."
    - "Write a Rust function that finds the longest increasing subsequence in a vector."
    - "Analyze the tradeoffs between microservices and monolithic architectures."
  max_tokens: 128
  temperature: 0.7
  top_p: 0.9

agentic_workflow:
  - step: discover_models
    id: discover
    type: model_discovery
    input:
      models_dir: "/run/media/jon/data/models"
      filter:
        max_size_bytes: 6442450944  # 6GB
        file_extension: ".gguf"
    output:
      save_to: discovered_models

  - step: benchmark_loop
    id: bench_loop
    loop:
      count:
        max_iterations: N    # 3, 5, 15, or 50
        iteration_variable: current_model
    input:
      model_path: "{{loop.current_model}}"
      prompts: "${benchmark.prompts}"
      max_tokens: "${benchmark.max_tokens}"
    when:
      before_step_starts:
        log:
          to_file_path: "./workspace/logs/benchmark.log"
          event_fields: [step_name, loop_iteration, current_model]
      after_step_succeeds:
        append_to:
          - "./workspace/output/benchmark_results.yaml"
          - benchmark_collection
      after_loop_iteration_fails:
        log:
          to_file_path: "./workspace/logs/benchmark-errors.log"
          event_fields: [iteration, current_model, error_message]
    output:
      save_to: benchmark_results

  - step: generate_report
    id: report
    input:
      results: "${step.bench_loop.output}"
    output:
      save_to:
        - final_report
        - "./workspace/output/benchmark_report.json"
```

## Model Selection Criteria

### Selection Strategy

Models are selected for **diversity** across 4 dimensions:

1. **Size tier** (VRAM fit)
2. **Architecture** (Qwen, Llama, Mistral, Gemma, Phi, Falcon, etc.)
3. **Specialization** (code, reasoning, chat, instruct, thinking)
4. **Author diversity** (different organizations)

### Priority Rules

1. Models already on data drive (no download needed)
2. Q4_K_M quantization preferred (good quality/size ratio)
3. Instruct/chat variants preferred over base
4. One model per architecture per size tier (avoid duplicates)

## File Specifications

### benchmark-3-models.yml

**Purpose**: Quick validation (15-30 minutes total)

| Model | Size | Architecture | Specialization |
|-------|------|-------------|----------------|
| Qwen3-4B-Instruct-2507-Q4_K_M | ~2.5GB | Qwen3 | Reasoning/chat |
| Phi-4-mini-instruct-Q4_K_M | ~2.7GB | Phi | Code/reasoning |
| Llama-3.2-3B-Instruct-Q4_K_M | ~2.0GB | Llama | General chat |

**Verification**: Run 3 prompts × 128 tokens × 3 models = ~9 inference calls

### benchmark-5-models.yml

**Purpose**: Architecture diversity test (30-60 minutes)

All 3-model models plus:
| Model | Size | Architecture | Specialization |
|-------|------|-------------|----------------|
| Mistral-Nemo-Instruct-2407-Q4_K_M | ~5.1GB | Mistral | General |
| Gemma-3-4b-it-Q4_K_M | ~2.7GB | Gemma | Multimodal/chat |

**Verification**: Run 3 prompts × 128 tokens × 5 models = ~15 inference calls

### benchmark-15-models.yml

**Purpose**: Comprehensive coverage test (2-3 hours)

Expands to 15 models covering all major architectures and size tiers:

| Tier | Models | Architectures |
|------|--------|--------------|
| 0.5-1.5B | 3 models | Qwen, Llama, Phi | 
| 1.5-3B | 4 models | Mistral, Gemma, Qwen, Phi |
| 3-4B | 4 models | Qwen, Llama, Falcon, Granite |
| 4-6B | 4 models | Mistral, Qwen, Llama, CodeGemma |

**Verification**: Run 3 prompts × 128 tokens × 15 models = ~45 inference calls

### benchmark-50-models.yml

**Purpose**: Full benchmark suite (6-10 hours)

50 models selected from 209 candidates on external drive. Full diversity across:
- 10+ distinct architectures
- 4 size tiers (balanced)
- 5+ specializations (code, reasoning, chat, instruct, thinking, math)
- 20+ distinct authors/organizations

**Verification**: Run 3 prompts × 128 tokens × 50 models = ~150 inference calls

## Benchmark Metrics Collected

For each model, collect:

| Metric | Unit | Description |
|--------|------|-------------|
| model_id | string | Model identifier |
| model_path | string | Full path to GGUF file |
| file_size_bytes | bytes | File size on disk |
| load_duration_ms | ms | Time to load model into VRAM |
| prompt_eval_duration_ms | ms | Time to process prompt tokens |
| completion_duration_ms | ms | Time to generate completion |
| total_duration_ms | ms | End-to-end time per prompt |
| prompt_tokens | count | Number of input tokens |
| completion_tokens | count | Number of output tokens |
| tokens_per_second | tokens/s | Throughput (completion_tokens / completion_duration) |
| time_to_first_token_ms | ms | Latency to first output token |
| avg_latency_ms | ms | Average per-token latency |
| p50_latency_ms | ms | 50th percentile per-token latency |
| p95_latency_ms | ms | 95th percentile per-token latency |
| p99_latency_ms | ms | 99th percentile per-token latency |
| vram_usage_bytes | bytes | VRAM consumed during inference |
| error | string? | Error message if model failed |

## Output Format

### JSON (programmatic)
```json
{
  "timestamp": "2026-05-03T12:00:00Z",
  "gpu_info": "AMD Radeon RX 7600 (8GB VRAM, Vulkan)",
  "total_models": 50,
  "successful": 47,
  "failed": 3,
  "results": [...]
}
```

### CSV (spreadsheet)
```
model_id,file_size_gb,load_s,tps,avg_latency_ms,p99_latency_ms,error
qwen3-4b-q4km,2.5,8.2,42.1,23.8,45.2,
phi-4-mini-q4km,2.7,9.1,38.5,26.0,51.3,
```

### Console Table
```
┌─────────────────────┬───────────┬────────┬───────────┬────────────┐
│ Model               │ Size(GB)  │ TPS    │ Avg ms    │ P99 ms     │
├─────────────────────┼───────────┼────────┼───────────┼────────────┤
│ qwen3-4b-q4km       │ 2.5       │ 42.1   │ 23.8      │ 45.2       │
│ phi-4-mini-q4km     │ 2.7       │ 38.5   │ 26.0      │ 51.3       │
└─────────────────────┴───────────┴────────┴───────────┴────────────┘
```

## Validation Criteria

Each generated YAML file must:
1. Parse without errors via `serde_yaml`
2. Deserialize into `UnifiedConfig` struct
3. Conform to schema version 2.0.0+
4. Contain valid loop configuration
5. Reference only models that exist on external drive
6. Pass `cargo test --all-features` after integration
