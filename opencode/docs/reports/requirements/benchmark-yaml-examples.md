# Complete YAML Benchmark Examples

## Overview

Three benchmark workflows demonstrating scaling from 5 to 100 models. Each workflow benchmarks all models end-to-end (discovery → loading → 8 prompts → aggregation → ranking → report), demonstrating fault tolerance and comprehensive data collection.

---

## Benchmark 1: Small-Scale Validation (5 Models)

**Purpose**: Quick validation of benchmark system with representative models across providers.

**Models**: 5 total
- LM Studio: `llama-2-7b` (Q4_K_M)
- Ollama: `llama-3.1-8b` (Q4_K_M)
- llama.cpp: `llama-2-1b` (Q4_K), `llama-7b` (Q4_K)
- OpenAI: `gpt-4o-mini` (via proxy)

**Estimated Duration**: 30 minutes

### Workflow: benchmark-small-scale.yml

```yaml
name: "Small-Scale Benchmark Validation"

models:
  - id: "llama-2-7b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  - id: "llama-3.1-8b-ollama"
    provider: "ollama"
    host: "localhost"
    port: 11434
    quantization: "Q4_K_M"

  - id: "llama-2-1b-llamacpp"
    provider: "llamacpp"
    host: "localhost"
    port: 8080
    quantization: "Q4_K"

  - id: "llama-7b-llamacpp"
    provider: "llamacpp"
    host: "localhost"
    port: 8080
    quantization: "Q4_K"

  - id: "gpt-4o-mini-proxy"
    provider: "openai"
    host: "proxy.example.com"
    port: 443
    quantization: "unquantized"

workflow:
  type: "benchmark"
  prompts_per_model: 8
  temperature: 0.7
  max_tokens: 2048
  output:
    save_to: "./benchmark-results/small-scale"
    format: "json"

steps:
  - name: discover_models
    tools: [model_discovery]
    timeout_secs: 120

  - name: load_all_models
    depends_on: [discover_models]
    tools: [model_loader]
    parallel:
      - target: all_models
    timeout_secs: 300

  - name: run_prompts
    depends_on: [load_all_models]
    tools: [llm_generate]
    parallel:
      - target: all_models
      parallel_group: prompt_group
    timeout_secs: 1800  # 8 prompts × 5 models × 45s each

  - name: aggregate_results
    depends_on: [run_prompts]
    tools: [aggregator]
    timeout_secs: 60

  - name: rank_models
    depends_on: [aggregate_results]
    tools: [ranker]
    timeout_secs: 30

  - name: generate_report
    depends_on: [rank_models]
    tools: [report_generator]
    timeout_secs: 120
```

### Expected Output

**Structure:**
```
./benchmark-results/small-scale/
├── model-results.json              # Per-model scores, timing, metrics
├── ranking.json                    # Overall ranking, category winners
├── comparison-table.md              # Side-by-side model comparison
├── summary.json                    # Statistics (mean, median, std dev)
└── detail/
    ├── llama-2-7b-lmstudio/
    │   ├── prompt-1.md
    │   ├── prompt-2.md
    │   └── ...
    ├── llama-3.1-8b-ollama/
    │   └── ...
    └── ...
```

**Success Criteria:**
- All 5 models load successfully
- All 40 prompts executed (8 per model)
- Results aggregated without errors
- Rankings generated with tie-breaking logic
- Total time < 45 minutes (including overhead)

---

## Benchmark 2: Medium-Scale Scalability Test (20 Models)

**Purpose**: Validate scalability of benchmark system with larger model set, testing parallel execution and resource management.

**Models**: 20 total
- LM Studio: 5 models (various sizes: 2B, 7B, 13B, 34B)
- Ollama: 5 models (2B-8B range, Q4/Q8 quantization)
- llama.cpp: 5 models (1B-7B range, mixed quantization)
- OpenAI: 3 models (gpt-4o-mini, gpt-4o, gpt-3.5-turbo)
- Custom: 2 models (user-provided GGUF files)

**Estimated Duration**: 2.5 hours

### Workflow: benchmark-medium-scale.yml

```yaml
name: "Medium-Scale Scalability Test"

models:
  # LM Studio models
  - id: "llama-2-7b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  - id: "llama-7b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  - id: "llama-13b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  - id: "llama-34b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  - id: "llama-70b-lmstudio"
    provider: "lmstudio"
    host: "localhost"
    port: 1234
    quantization: "Q4_K_M"

  # Ollama models
  - id: "llama-3b-ollama"
    provider: "ollama"
    host: "localhost"
    port: 11434
    quantization: "Q4_K"

  - id: "llama-7b-ollama-q8"
    provider: "ollama"
    host: "localhost"
    port: 11434
    quantization: "Q8_K"

  # llama.cpp models
  - id: "llama-1b-llamacpp"
    provider: "llamacpp"
    host: "localhost"
    port: 8080
    quantization: "Q4_K"

  - id: "llama-3b-llamacpp"
    provider: "llamacpp"
    host: "localhost"
    port: 8080
    quantization: "Q4_K"

  # OpenAI models (for comparison)
  - id: "gpt-4o-mini-proxy"
    provider: "openai"
    host: "proxy.example.com"
    port: 443
    quantization: "unquantized"

workflow:
  type: "benchmark"
  prompts_per_model: 8
  temperature: 0.7
  max_tokens: 2048
  output:
    save_to: "./benchmark-results/medium-scale"
    format: "json"

  # Concurrency via existing workflow_execution_strategy.parallel fields
  # and provider-level hosting.requests.max_concurrent_requests

steps:
  - name: discover_models
    tools: [model_discovery]
    timeout_secs: 120

  - name: load_first_batch
    depends_on: [discover_models]
    tools: [model_loader]
    parallel:
      - target: batch_1
    when:
      - condition: "{{step.discover_models.models_count >= 8}}"

  - name: run_prompts_batch_1
    depends_on: [load_first_batch]
    tools: [llm_generate]
    parallel:
      - target: batch_1
      parallel_group: prompt_group
    timeout_secs: 1800

  - name: load_second_batch
    depends_on: [run_prompts_batch_1]
    tools: [model_loader]
    parallel:
      - target: batch_2
    when:
      - condition: "{{step.run_prompts_batch_1.completed}}"

  - name: run_prompts_batch_2
    depends_on: [load_second_batch]
    tools: [llm_generate]
    parallel:
      - target: batch_2
      parallel_group: prompt_group
    timeout_secs: 1800

  - name: aggregate_results
    depends_on: [run_prompts_batch_2]
    tools: [aggregator]
    timeout_secs: 60

  - name: rank_models
    depends_on: [aggregate_results]
    tools: [ranker]
    timeout_secs: 30

  - name: generate_report
    depends_on: [rank_models]
    tools: [report_generator]
    timeout_secs: 180
```

### Expected Output

**Structure:**
```
./benchmark-results/medium-scale/
├── model-results.json              # All 20 models with full metrics
├── ranking.json                    # Tiered rankings (S/A/B/C/D/F)
├── comparison-table.md              # 20×20 matrix (models × prompts)
├── summary.json                    # Per-tier statistics
└── detail/
    ├── batch-1/                    # First 8 models
    │   ├── llama-2-7b-lmstudio/
    │   └── ...
    └── batch-2/                    # Second 8 models
        └── ...
```

**Success Criteria:**
- All 20 models tested across 160 prompts (8 per model)
- Concurrency limits respected (4 models, 16 requests simultaneously)
- Memory budget not exceeded
- Results aggregated and ranked
- Total time < 3 hours

---

## Benchmark 3: Production-Grade Full Benchmark (100 Models)

**Purpose**: Full production-grade benchmark testing system fault tolerance, recovery, and comprehensive data collection at scale.

**Models**: 100 total
- Local models: 80 (various sizes, quantizations, providers)
  - LM Studio: 30 models (2B-70B range)
  - Ollama: 20 models (1B-8B, Q2/Q4/Q8 quantization)
  - llama.cpp: 20 models (1B-13B range, mixed quantization)
  - Custom GGUF: 10 models (user-provided)
- Cloud models (baseline): 20
  - OpenAI: 5 models (gpt-4o-mini, gpt-4o, gpt-3.5-turbo, gpt-4-turbo, claude-3-opus)
  - Anthropic: 5 models (claude-3-haiku, claude-3-sonnet, claude-3.5-sonnet)
  - Google: 5 models (gemini-1.5-pro, gemini-1.5-flash, gemini-pro, gemini-1.0-pro)
  - DeepSeek: 5 models (deepseek-chat, deepseek-coder)

**Estimated Duration**: 8 hours

### Workflow: benchmark-large-scale.yml

```yaml
name: "Production-Grade Full Benchmark"

models:
  # Include all 100 models here...
  # [Truncated for brevity - workflow would contain 100 model definitions]

workflow:
  type: "benchmark"
  prompts_per_model: 10
  temperature: 0.7
  max_tokens: 4096
  output:
    save_to: "./benchmark-results/large-scale"
    format: "json"

  # Concurrency via existing workflow_execution_strategy.parallel fields
  # and provider-level hosting.requests.max_concurrent_requests

  # Fault tolerance via existing unified schema fields:
  #   agentic_workflow.retry.max_attempts: 3
  #   agentic_workflow.retry.backoff: exponential
  #   agentic_workflow.retry.step.checkpoint_after_retry: true
  #   providers.*.hosting.skip_on_load_failure: true

steps:
  - name: discover_models
    tools: [model_discovery]
    timeout_secs: 180

  - name: load_models_batched
    depends_on: [discover_models]
    tools: [model_loader]
    parallel:
      - target: all_models
      parallel_group: model_load_group
      max_workers: 6
    when:
      - condition: "{{step.discover_models.models_count > 0}}"

  - name: run_prompts_wave_1
    depends_on: [load_models_batched]
    tools: [llm_generate]
    parallel:
      - target: all_models
      parallel_group: prompt_group
      parallel_group_timeout_secs: 3600   # 1 hour per wave
    when:
      - condition: "{{step.load_models_batched.completed}}"

  - name: save_checkpoint_1
    depends_on: [run_prompts_wave_1]
    tools: [checkpoint_saver]
    when:
      - condition: "{{step.run_prompts_wave_1.completed}}"

  - name: run_prompts_wave_2
    depends_on: [save_checkpoint_1]
    tools: [llm_generate]
    parallel:
      - target: all_models
      parallel_group: prompt_group
      parallel_group_timeout_secs: 3600
    when:
      - condition: "{{step.run_prompts_wave_1.completed}}"

  - name: aggregate_results
    depends_on: [run_prompts_wave_2]
    tools: [aggregator]
    timeout_secs: 300

  - name: rank_models
    depends_on: [aggregate_results]
    tools: [ranker]
    timeout_secs: 60

  - name: generate_report
    depends_on: [rank_models]
    tools: [report_generator]
    timeout_secs: 600
```

### Expected Output

**Structure:**
```
./benchmark-results/large-scale/
├── model-results.json                    # All 100 models with full metrics
├── ranking.json                        # Overall rankings
├── tiered-rankings.json               # S/A/B/C/D/F tiers
├── comparison-table.md                  # 100×10 matrix
├── summary.json                        # Per-tier statistics
├── checkpoints/
│   ├── checkpoint-001.json                # After wave 1 (50% of prompts)
│   ├── checkpoint-002.json                # After wave 2 (100% of prompts)
│   └── ...
├── failed-models.json                    # Models that failed to load or run
└── detail/
    ├── wave-1/                           # First 10 prompts
    │   ├── model-1/
    │   │   ├── prompt-1.md
    │   │   └── ...
    │   ├── model-2/
    │   └── ...
    └── wave-2/                           # Second 10 prompts
        └── ...
```

**Success Criteria:**
- All 100 models attempted
- 1,000 prompts executed total (10 per model)
- Checkpoints saved every wave
- Failed models tracked and excluded from rankings
- Results aggregated across all successful models
- Total time < 10 hours (including fault recovery)

---

## Schema Integration

### Unified Schema Fields Required

These benchmark workflows use unified schema v2.0 with specific fields for concurrency and fault tolerance:

```yaml
models:
  - id: string                 # Model identifier (required)
    provider: string          # lmstudio, ollama, llamacpp, openai, etc. (required)
    host: string              # Hostname or IP (required)
    port: number              # Port number (required)
    quantization: string      # Q4_K, Q8_K, etc. (optional)
    # ... other model config fields

workflow:
  type: "benchmark"                    # Workflow type identifier
  prompts_per_model: number              # N prompts per model (required)
  temperature: number (0.0-2.0)         # Generation temperature
  max_tokens: number                     # Max output tokens
  output:
    save_to: string           # Output directory path (required)
    format: string            # "json" or "yaml" (required)

  # Fault tolerance uses EXISTING unified schema fields:
  #   retry.max_attempts + retry.backoff (in agentic_workflow.retry)
  #   retry.step.checkpoint_after_retry: true (in agentic_workflow.retry.step)
  #   providers.*.hosting.skip_on_load_failure: true (NEW single field)
  #   workflow_execution_strategy.timeout.per_operation (timeouts)
  #   when.after_step_fails hooks (error routing)
```

### Schema Fields Used (All Existing Except One)

**Existing fields that handle fault tolerance:**
```yaml
# In agentic_workflow.retry:
retry:
  max_attempts: 3
  backoff: exponential
  delay_ms: 5000
  step:
    checkpoint_after_retry: true

# In providers.<name>.hosting:
hosting:
  skip_on_load_failure: true           # NEW: Only genuinely new field

# In workflow_execution_strategy.parallel:
parallel:
  max_models: 3                        # Already exists
  max_threads: 4                        # Already exists
  parallel_group_timeout_secs: 600     # Already exists
```

---

## Performance Characteristics

| Benchmark | Models | Prompts | Total Requests | Est. Duration | Est. Tokens |
|-----------|--------|---------|----------------|----------------|--------------|
| Small (5) | 5 | 8 | 40 | 30 min | ~40K |
| Medium (20) | 20 | 8 | 160 | 2.5 hr | ~160K |
| Large (100) | 100 | 10 | 1,000 | 8 hr | ~400K |

### Scaling Observations

1. **Linear scaling**: Duration scales linearly with model count (not exponential)
2. **Parallelism gains**: Medium benchmark runs 4× faster than sequential due to parallel prompts
3. **Memory bottleneck**: Large benchmark limited by GPU memory, requires staggered loading
4. **Network saturation**: Cloud models (OpenAI/Anthropic) hit rate limits at scale, require backoff
5. **Fault tolerance critical**: Large benchmark expects failures (OOM, timeouts, crashes) - must be robust

---

## Implementation Notes

### Tool Requirements

**Custom Tools Needed:**

1. **model_discovery** - Scan configured providers for available models
2. **model_loader** - Load specific model with memory allocation
3. **llm_generate** - Execute prompt with streaming response capture
4. **aggregator** - Combine individual model results into unified dataset
5. **ranker** - Score and rank models by metrics
6. **report_generator** - Create summary tables, rankings, comparison matrix
7. **checkpoint_saver** - Save workflow state for recovery

### Execution Engine Features

1. **Parallel step groups**: `parallel_group` for concurrent prompts
2. **Parallel sub-workflows**: Sub-workflows for per-model prompt execution
3. **Parallel group timeout**: `parallel_group_timeout_secs` for bounding wave duration
4. **Fault tolerance**: Retry logic, checkpointing, skip-on-failure
5. **Memory management**: Respect `memory_limit_gb`, stagger model loading
6. **Concurrency limits**: Respect `max_parallel_models` and `max_parallel_requests`

### Output Formats

**JSON Structure:**
```json
{
  "benchmark_id": "uuid",
  "timestamp": "ISO8601",
  "workflow": "benchmark-large-scale",
  "models": [
    {
      "model_id": "llama-7b-llamacpp",
      "provider": "llamacpp",
      "loaded": true,
      "prompts_completed": 10,
      "prompts_failed": 0,
      "total_tokens": 18450,
      "avg_time_per_prompt_ms": 4230,
      "quality_score": 0.87,
      "metrics": { ... }
    },
    // ... 99 more models
  ],
  "ranking": [
    {
      "rank": 1,
      "model_id": "llama-13b-lmstudio",
      "overall_score": 0.92,
      "category": "best"
    },
    // ... more rankings
  ],
  "summary": {
    "total_models": 100,
    "successful_models": 95,
    "failed_models": 5,
    "total_prompts": 1000,
    "total_tokens": "1.2M",
    "total_duration_seconds": 28800,
    "avg_tokens_per_second": 41.6
  }
}
```

---

## Next Steps

1. Implement custom tools for benchmark-specific operations
2. Add concurrency fields to unified schema
3. Evaluate whether concurrency fields need extension (see schema-additions-needed.md)
4. Implement parallel group timeout in workflow engine
5. Add checkpointing support to workflow engine
6. Create benchmark result aggregation and ranking logic
7. Implement memory-aware model loading and unloading
8. Add benchmark progress display (current model, prompts, ETA)
