# Critical Review Cycle 2: Workflow Coherence

**Date**: 2026-03-08
**Scope**: Workflow Coherence
**Reviewer**: Sisyphus

---

## Review Process

Reviewed workflow examples for logical consistency, contradictions, and gaps in execution flow.

---

## Findings

### 1. Execution Mode Coherence

**Issue**: Inconsistent memory management terminology

**Examples**:
- ex01: `execution.memory.parallel_models: true` (confusing with execution.mode)
- ex02: `execution.mode: serial` + `execution.memory.load_unload_strategy: one_at_a_time`
- ex03: `execution.mode: dynamic_parallelization`

**Problem**: `parallel_models` in memory section is redundant with execution.mode

**Recommendation**: Remove `memory.parallel_models`, use only `execution.mode` (parallel, serial, hybrid)

---

### 2. Model Loading Coherence

**Issue**: Unclear when models are loaded/unloaded

**ex02**:
```yaml
execution:
  mode: serial
  memory:
    load_unload_strategy: one_at_a_time
```

**Question**: Are all models defined in `models:` section loaded at once, then swapped serially? Or only load one model total?

**Ambiguity**: `load_priority` field suggests some models are pre-loaded, but `one_at_a_time` contradicts this.

**Recommendation**: Add explicit model loading phases:
```yaml
execution:
  mode: serial
  loading_strategy:
    initial_load: [step1_model]  # Load only first model
    swap_on_step_change: true
```

---

### 3. Loop Integration Coherence

**Issue**: Loops defined at top level but referenced in pipeline

**ex01**:
```yaml
validation_loop:
  type: validation
  steps:
    - refactored_code_generation: step_3
```

**Problem**: `validation_loop` references step IDs, but doesn't specify which pipeline steps to loop over. Unclear if this loops over specific steps or creates a new loop structure.

**ex02**:
```yaml
validation_loop:
  type: exact_validation
  steps:
    - review_generation: step_3
```

**Problem**: Only references one step. What happens to other steps? Are they outside the loop?

**Recommendation**: Make loop relationship explicit:
```yaml
pipeline:
  - step: generate_refactored_code
    id: step_3
    loop:
      id: validation_loop
      type: validation
      # ... loop config
```

OR

```yaml
pipeline:
  - step: generate_refactored_code
    id: step_3

loops:
  - id: validation_loop
    steps: [step_3]
    # ... loop config
```

---

### 4. Variable Reference Coherence

**Issue**: Mixed variable reference styles

**ex01**:
```yaml
output:
  save_to: code_analysis
```

**ex01** later:
```yaml
input:
  analysis: "${step.step_2.output}"  # Step reference
```

**Problem**: Inconsistent - sometimes use `save_to` variable name directly, sometimes use `step.step_id.output`.

**Question**: When can you reference `save_to` variable directly vs `step.step_id.output`?

**Recommendation**: Define scoping rules:
```yaml
# Global scope: Accessible from any step
workflow_variables:
  - codebase_path

# Step scope: Accessible only from subsequent steps
step_outputs:
  - step_id: step_1
    variable: code_analysis

# Direct reference: Always use step.step_id.output
step.step_1.output  # Never use save_to variable directly in other steps
```

---

### 5. RAG Operations Coherence

**Issue**: RAG operations mixed with LLM steps

**ex06**:
```yaml
- step: add_docs_to_rag
  model: "${model}"  # Why specify model for RAG add?
  input:
    prompt: "Load documentation from ${workflow.docs_path} into RAG"
    rag_operation: add
  rag:
    operation: add
```

**Problem**: RAG add operation doesn't need LLM model (just embedding model). Specifying `model: "${model}"` is misleading.

**Recommendation**: Separate LLM operations from RAG operations:
```yaml
rag:
  embedding_model: lmstudio://all-MiniLM-L6-v2

pipeline:
  - step: add_docs_to_rag
    type: rag_operation  # Not LLM step
    rag:
      operation: add
      # ... config

  - step: query_rag
    type: rag_operation
    rag:
      operation: query
      # ... config

  - step: generate_from_rag
    model: "${model}"  # Only LLM steps need model
    input:
      rag_results: "${step.query_rag.output}"
```

---

### 6. Tool Call Coherence

**Issue**: Inconsistent tool call syntax

**ex03**:
```yaml
- step: run_tests
  model: "${agent}"
  tool:
    type: shell_exec
    command: ["cargo", "test"]
```

**ex04**:
```yaml
- step: run_tests
  model: "${model}"
  tool:
    type: shell_exec
    command: ["cargo", "test"]
```

**Problem**: Same operation (running tests) uses different syntax. Also, why does `tool:` have same indentation as `model:` in some cases?

**Recommendation**: Standardize tool call syntax:
```yaml
# Option A: Tool at same level as model
- step: run_tests
  model: "${model}"
  tool_call:
    tool: test_runner
    tool_type: shell_exec
    command: ["cargo", "test"]

# Option B: Tool inside model specification
- step: run_tests
  execution:
    type: tool_call
    tool:
      name: test_runner
      type: shell_exec
      command: ["cargo", "test"]
```

---

### 7. Logging Output Type Coherence

**Issue**: Mixed output type terminology

**ex01**: `output_type: chat`
**ex02**: `output_type: stateless_direct_io`
**ex03**: `output_type: mixed`

**Question**: How does `mixed` work? Does it require additional configuration?

**Problem**: No examples show what `mixed` output actually looks like or how to configure it.

**Recommendation**: Add explicit mixed output configuration:
```yaml
logging:
  output_type: mixed
  mixed_config:
    progress:
      type: chat
      level: info
    debugging:
      type: log
      level: debug
      file: workflow_debug.log
    output_data:
      type: stateless_direct_io
      path: /workspace/output/data.json
```

---

### 8. Retry Logic Coherence

**Issue**: Inconsistent retry configuration levels

**ex01**:
```yaml
retry:
  default:
    max_attempts: 3
```

**ex02**:
```yaml
retry:
  default:
    max_attempts: 3
  step_specific:
    step_1:
      max_attempts: 5
```

**ex06**:
```yaml
retry:
  default:
    max_attempts: 3
  rag_specific:
    add:
      max_attempts: 5
```

**Problem**: Inconsistent scoping (step_specific vs rag_specific). What about tool-specific retry?

**Recommendation**: Unified retry hierarchy:
```yaml
retry:
  global:
    max_attempts: 3
    backoff_strategy: exponential
    delay_ms: 1000
    on_failure: escalate_model

  step_specific:
    step_1:
      max_attempts: 5
      backoff_strategy: linear

  tool_specific:
    web_fetch:
      max_attempts: 10
      backoff_strategy: linear

  operation_specific:
    rag_add:
      max_attempts: 5
    rag_query:
      max_attempts: 3
```

---

## Critical Gaps

### 1. Workflow State Management

**Missing**: How is workflow state passed between steps?

**Example**:
- Step 1 outputs `analysis_result`
- Step 2 reads `analysis_result`
- How is this implemented? Is it in-memory? Written to file? Passed as environment variable?

**Recommendation**: Add explicit state management section:
```yaml
state_management:
  backend: memory  # memory, file, database
  persistence: false
  cleanup_on_completion: true
```

---

### 2. Error Handling Strategy

**Missing**: What happens when a step fails?

**Examples**:
- Step fails (3 retries exhausted)
- Does workflow stop? Continue to next step? Run cleanup step?
- How are partial results handled?

**Recommendation**: Add error handling section:
```yaml
error_handling:
  strategy: stop_on_failure  # stop_on_failure, continue_on_failure, retry_alternate
  cleanup_on_failure:
    enabled: true
    cleanup_steps: [cleanup_temp_files]
  partial_results:
    save: true
    path: /workspace/partial_results/
```

---

### 3. Workflow Execution Order

**Missing**: Are steps executed sequentially or in parallel by default?

**Examples**:
- All pipeline steps shown in sequence
- But no explicit `depends_on` or `parallel_with` specification
- How to run steps in parallel?

**Recommendation**: Add step dependency specification:
```yaml
pipeline:
  - step: analyze_code
    id: step_1
    parallel_with: [fetch_docs]

  - step: fetch_docs
    id: step_2
    parallel_with: [analyze_code]

  - step: synthesize
    id: step_3
    depends_on: [step_1, step_2]
```

---

## Summary of Inconsistencies

1. ❌ Memory management terminology inconsistent
2. ❌ Model loading strategy unclear
3. ❌ Loop integration with pipeline unclear
4. ❌ Variable reference styles inconsistent
5. ❌ RAG operations mixed with LLM steps
6. ❌ Tool call syntax inconsistent
7. ❌ Mixed output type undefined
8. ❌ Retry scoping inconsistent

**Total Inconsistencies**: 8

---

## Action Items

### High Priority
1. Standardize execution mode and memory management terminology
2. Clarify model loading strategy with explicit loading phases
3. Define loop integration with pipeline (make relationship explicit)
4. Standardize variable reference scoping rules
5. Separate RAG operations from LLM steps

### Medium Priority
6. Standardize tool call syntax across all examples
7. Define mixed output type configuration
8. Unify retry scoping (global, step, tool, operation)

### Low Priority
9. Add workflow state management section
10. Add error handling strategy section
11. Add step dependency specification

---

## Next Steps

Proceed to Review Cycle 3: Model Management
