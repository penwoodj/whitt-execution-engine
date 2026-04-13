# Critical Review Cycle 3: Model Management

**Date**: 2026-03-08
**Scope**: Model Management
**Reviewer**: Sisyphus

---

## Review Process

Reviewed model loading, unloading, auto-routing, and provider configuration across all examples.

---

## Findings

### 1. Model Provider Configuration

**Coverage**: Good - multiple providers shown (LM Studio, llama.cpp, Ollama)

**Issues**:

1. **Backend Configuration Inconsistent**
   - ex01: `backend: vulkan` with `gpu_layers: 32`
   - ex02: `backend: vulkan` with `gpu_layers: 32`
   - Missing: What about CPU-only backends? Metal? CUDA?
   - **Gap**: No examples show how to specify CPU backend for CPU-only models

2. **Provider URI Format Inconsistent**
   - ex01: `provider: lmstudio`, `model: "llama-3.2-3b-instruct"`
   - ex02: `provider: llamacpp`, `model: "/models/code-specialist.gguf"`
   - Missing: URI format like `lmstudio://` or `llamacpp://`
   - **Gap**: Unclear if `provider:` and `model:` should be combined into single URI

3. **Model Download/Installation**
   - **Missing**: No examples show automatic model installation
   - **Gap**: Workflow requirements mention model installation automation (Phase 2.5), but no schema examples

**Recommendation**: Unified model configuration:
```yaml
model:
  uri: lmstudio://llama-3.2-3b-instruct  # Unified URI
  backend:
    type: vulkan  # vulkan, cuda, metal, cpu
    gpu_layers: 32
    max_memory_mb: 4096

  download:
    auto_install: true
    source_url: https://example.com/models/...
    verify_checksum: true
```

---

### 2. Model Loading Strategy

**Coverage**: Mixed - shows serial and parallel modes

**Issues**:

1. **Serial Execution Unclear**
   - ex02: `execution.mode: serial`, `memory.load_unload_strategy: one_at_a_time`
   - **Question**: Are models pre-loaded then swapped? Or loaded on-demand?
   - **Gap**: No explicit model lifecycle (load → execute → unload → load next)

2. **Parallel Execution Memory Tracking**
   - ex01: `execution.memory.parallel_models: true`, `max_allocated_memory_mb: 4096`
   - **Question**: How does transpiler ensure memory limit is respected?
   - **Gap**: No model size specification or memory estimation

3. **Persistent Models**
   - **Gap**: No examples show keeping specific models in memory throughout workflow
   - **Use Case**: Embedding model should stay loaded while task models swap

**Recommendation**: Explicit model lifecycle:
```yaml
execution:
  mode: hybrid

  model_lifecycle:
    persistent_models:
      - uri: lmstudio://all-MiniLM-L6-v2
        reason: embedding_model_used_throughout

    swap_models:
      - uri: llamacpp://code-specialist
        load_when: step_1_start
        unload_when: step_1_complete
      - uri: lmstudio://deepseek-coder
        load_when: step_2_start
        unload_when: step_2_complete

    memory_tracking:
      estimate_from: model_size  # model_size, manual, dynamic
      max_allocated_mb: 8192
      gc_when_exceeded: true
```

---

### 3. Auto Model Routing

**Coverage**: Partial - ex02 shows failure history and priority adjustment

**Issues**:

1. **Routing Rules Not Explicit**
   - ex02: `priority_adjustment` per model
   - **Gap**: No explicit routing rules (when to use which model)
   - **Question**: How does transpiler decide which model to use for a step?

2. **Task Category Mapping**
   - ex02: `task_categories: [code_analysis, code_generation, code_review]`
   - **Gap**: No mapping of which task category each step belongs to
   - **Missing**: How does step specify its task category?

3. **Fallback Model Configuration**
   - ex02: `fallback_order` lists models
   - **Gap**: No per-model configuration for fallback (different backend, parameters)
   - **Question**: Does fallback inherit original model's settings?

**Recommendation**: Explicit routing configuration:
```yaml
model_routing:
  enabled: true
  # Note (v2.0): In v2.0, omit enabled: (presence=enabled by default) or use disabled: true
  strategy: task_category_based

  routing_rules:
    - task_category: code_analysis
      primary_model: llamacpp://code-specialist
      fallback_chain:
        - lmstudio://deepseek-coder
        - ollama://llama3.2

    - task_category: code_generation
      primary_model: lmstudio://deepseek-coder
      fallback_chain:
        - llamacpp://code-specialist
        - ollama://llama3.2-70b

  failure_tracking:
    max_history_entries: 100
    adjustment:
      on_failure: -0.1
      on_success: +0.01
      min_priority: 0.0
      max_priority: 1.0

  step_assignment:
    explicit:
      step_1:
        task_category: code_analysis
        preferred_model: code-specialist
    automatic:
      task_category_inference: true  # Infer from step type
```

---

### 4. Model Performance Metrics

**Gap**: Complete - No examples show model performance tracking

**Missing Features**:
- Tokens per second
- Latency
- Memory usage
- Quality metrics
- Comparison between models

**Use Case**: Decision making for auto-routing based on performance

**Recommendation**: Add performance tracking:
```yaml
model_metrics:
  enabled: true
  # Note (v2.0): In v2.0, omit enabled: (presence=enabled by default) or use disabled: true
  collect:
    - tokens_per_second
    - latency_ms
    - memory_mb
    - quality_score

  compare:
    - by_task_category: true
    - by_model: true
    - by_backend: true

  routing_decisions:
    log: true
    file: ./workspace/.metrics/routing_decisions.json
```

---

### 5. Model Versioning

**Gap**: Missing - No examples show model version management

**Use Cases**:
- Update to newer model version
- Rollback to previous version
- Compare model versions

**Recommendation**: Add version control:
```yaml
model:
  uri: lmstudio://llama-3.2-3b-instruct
  version:
    current: v1
    available:
      - v2: lmstudio://llama-3.2-7b-instruct
      - v3: lmstudio://llama-3.2-70b-instruct

  upgrade_strategy:
    auto_upgrade: false
    test_before_upgrade: true
    rollback_on_failure: true
```

---

## Critical Gaps

### 1. Model Size Estimation

**Missing**: How does transpiler know model memory usage?

**Problem**: `max_allocated_memory_mb` needs model sizes to enforce limits

**Use Cases**:
- Pre-validate if models fit in memory
- Warn if memory will be exceeded
- Optimize model loading order

**Recommendation**: Add model size specification:
```yaml
model:
  uri: lmstudio://llama-3.2-3b-instruct
  size:
    disk_mb: 6200
    memory_mb: 4096
    estimated: true  # or exact
```

---

### 2. Multi-Model Coordination

**Missing**: How do multiple models communicate?

**Problem**: If step A uses model X and step B uses model Y, how is data passed?

**Use Cases**:
- Parallel execution with different models
- Sub-agent coordination across models
- Result aggregation

**Recommendation**: Add model coordination layer:
```yaml
model_coordination:
  sync_points:
    - point: after_parallel_execution
      wait_for_all: true
      timeout_secs: 60

  result_aggregation:
    - parallel_step_id: step_1
      aggregator: average
      output_var: aggregated_results
```

---

### 3. Model Fallback Details

**Missing**: What happens when all models in fallback chain fail?

**Problem**: No fallback handling beyond the chain

**Use Cases**:
- All models unavailable
- All models fail the task
- Network/infrastructure failure

**Recommendation**: Add ultimate fallback:
```yaml
fallback:
  ultimate:
    strategy: report_failure  # report_failure, retry_later, use_cloud_fallback
    notify_user: true
    save_partial_results: true

  cloud_fallback:
    enabled: true
    # Note (v2.0): In v2.0, omit enabled: (presence=enabled by default) or use disabled: true
    models:
      - openai://gpt-4
    use_when: all_local_failed
    require_confirmation: true
```

---

## Summary of Model Management Gaps

1. ❌ Backend configuration incomplete (CPU, Metal, CUDA missing)
2. ❌ Provider URI format inconsistent
3. ❌ Model installation automation missing
4. ❌ Serial execution model lifecycle unclear
5. ❌ Parallel execution memory tracking undefined
6. ❌ Persistent models not shown
7. ❌ Routing rules not explicit
8. ❌ Task category mapping missing
9. ❌ Performance tracking absent
10. ❌ Model versioning missing
11. ❌ Model size estimation missing
12. ❌ Multi-model coordination undefined
13. ❌ Ultimate fallback strategy missing

**Total Gaps**: 13

---

## Action Items

### High Priority
1. Add explicit model lifecycle configuration
2. Define explicit routing rules with task category mapping
3. Add model performance tracking schema
4. Add model size estimation

### Medium Priority
5. Standardize provider URI format
6. Add model installation automation
7. Add model versioning support

### Low Priority
8. Add multi-model coordination layer
9. Add ultimate fallback strategy
10. Add cloud fallback option

---

## Next Steps

Proceed to Review Cycle 4: Control Flow
