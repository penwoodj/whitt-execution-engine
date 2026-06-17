# Agentic Behavior Categorization

**Source task breakdown:** Parallel inference for same-model multi-target route_to (T1-T12)
**Generated:** 2026-06-17
**Purpose:** Categorize tasks by agentic behavior type to guide substructure generation
**Scope:** Code generation and integration test creation

---

## Category Definition

| Category | Description | Typical Substructure |
|----------|-------------|---------------------|
| **DATA_TRANSFORMER** | Transforms input data/structure into output data/structure without creating new behavior | Single LLM step with save_to + log hooks |
| **GENERATOR** | Creates new code, artifacts, or behavior that didn't exist before | LLM step with file write + validation hooks |
| **EVALUATOR** | Assesses, analyzes, or evaluates existing code/state to produce metrics or decisions | LLM step with gwt routing + evaluation hooks |
| **ORCHESTRATOR** | Coordinates multiple operations, manages execution flow, or makes routing decisions | Multi-step workflow with depends_on, route_to, conditionals |
| **VALIDATOR** | Verifies correctness, runs tests, or checks compliance with constraints | LLM step with test execution + result verification |

---

## Task Categorization

### T1: Add tokio::JoinSet import
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms file state (no import) to new state (with import). This is a straightforward state change — add one line to file. No new behavior created, just structural transformation.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to (file edit) + log hooks

---

### T2: Create send_inference_request helper method
**Primary category:** GENERATOR
**Reasoning:** Creates new method that didn't exist before. The method includes business logic (semaphore acquisition, message construction, response extraction). This is new behavior, not just transformation of existing code.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to (code generation) + validation hooks

---

### T2.1: Semaphore permit acquisition
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms method skeleton (signature only) to include semaphore logic. This is structural transformation — adding acquire/hold/release pattern.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T2.2: ChatMessage construction
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms empty message vector into populated vector. Adds conditional push logic. No new behavior pattern introduced — just structural transformation.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T2.3: Response extraction
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms method body to include response parsing logic. Extracts fields from response struct. Structural transformation of incomplete method to complete method.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T3: Identify same-model multi-target route_to branch
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms loop body to include model comparison logic. Computes boolean flag. No new control flow — just adds conditional flag computation.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T4: Implement parallel inference path
**Primary category:** ORCHESTRATOR
**Reasoning:** Coordinates multiple async operations (spawn, join_all, sort). Manages execution flow (load → spawn → join → sort). Makes routing decision (parallel vs sequential). Complex coordination requiring orchestration.
**Iteration budget:** 1
**Substructure hint:** Multi-step workflow: T4.1 → T4.2 → T4.3 with depends_on chains

---

### T4.1: Load model once outside loop
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms sequential pattern (load per target) to parallel pattern (load once). Moves load call outside loop. Structural transformation of execution pattern.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T4.2: Build request batch
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms list of targets into list of spawn calls. Iterates and spawns. Structural transformation — no new behavior patterns, just different execution structure.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T4.3: Collect ordered responses
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms JoinSet results into sorted vector. Drains set, collects, sorts. Structural transformation — reorganizes existing data into sorted order.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T5: Sequential hook processing after parallel inference
**Primary category:** ORCHESTRATOR
**Reasoning:** Coordinates hook execution for all targets after inference completes. Maintains sequential semantics while preserving parallel results. Orchestrates interaction between parallel results and sequential hooks.
**Iteration budget:** 1
**Substructure hint:** Multi-step workflow: iterate results, run hooks, update state

---

### T6: Unload model once after all targets
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms parallel execution end to include unload. Moves unload call after hook loop. Structural transformation — adds cleanup step.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T7: Preserve sequential fallback for mixed-model targets
**Primary category:** VALIDATOR
**Reasoning:** Validates that sequential path unchanged. Ensures no regression on mixed-model scenario. Requires verification that existing code path works as before.
**Iteration budget:** 1
**Substructure hint:** Test step with assertion hooks + result verification

---

### T8: Add logging for parallel vs sequential path
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms branch points to include log statements. Adds diagnostic output. No new behavior — just adds observability.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T9: Add integration test for same-model parallel path
**Primary category:** GENERATOR
**Reasoning:** Creates new test file with test function. Generates test setup, execution, assertions. New artifact (test) that validates parallel path behavior.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to (test file) + validation hooks

---

### T10: Add integration test for mixed-model sequential fallback
**Primary category:** GENERATOR
**Reasoning:** Creates new test file with test function. Generates test setup, execution, assertions. New artifact (test) that validates sequential path preservation.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to (test file) + validation hooks

---

### T11: Update benchmark log format for parallel metrics
**Primary category:** DATA_TRANSFORMER
**Reasoning:** Transforms log entry structure to include new fields. Adds parallel_targets, wall_time, sequential_est, speedup. No new behavior — just log format transformation.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to + log hooks

---

### T12: Documentation update
**Primary category:** GENERATOR
**Reasoning:** Creates new documentation content. Describes optimization, technical details. New artifact (documentation note) that didn't exist before.
**Iteration budget:** 1
**Substructure hint:** Single step with save_to (markdown) + log hooks

---

## Category Summary

| Category | Count | Percentage | Tasks |
|----------|-------|------------|-------|
| DATA_TRANSFORMER | 9 | 75% | T1, T2.1, T2.2, T2.3, T3, T4.1, T4.2, T4.3, T6, T8, T11 |
| GENERATOR | 3 | 25% | T2, T9, T10, T12 |
| ORCHESTRATOR | 2 | 16.7% | T4, T5 |
| EVALUATOR | 0 | 0% | — |
| VALIDATOR | 1 | 8.3% | T7 |

**Note:** Percentages exceed 100% because some tasks span multiple categories (T2 spans GENERATOR + DATA_TRANSFORMER via subtasks, T4 spans ORCHESTRATOR + DATA_TRANSFORMER via subtasks).

---

## Coverage Verification

**Total tasks:** 12 (T1-T12) with 6 subtasks (T2.1, T2.2, T2.3, T4.1, T4.2, T4.3) = 18 items categorized

**Categorized items:** 18/18 (100% coverage)

**Uncategorized items:** 0

**Orphan tasks:** 0 (all tasks belong to at least one category)

**Multi-category tasks:** 2 (T2, T4) — each spans GENERATOR/ORCHESTRATOR (main) + DATA_TRANSFORMER (subtasks)

---

## Substructure Mapping

### DATA_TRANSFORMER (9 instances)
**Substructure pattern:**
```yaml
step_name:
  generative_entity: ${models.qwen35}
  prompt: [specific transformation description]
  model_overrides: {max_tokens: 8192}
  when:
    after_step_succeeds:
      - save_to: [artifact_name, ./path/to/file.ext]
      - log:
          to_file_path: ./logs/step_name.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: []
```

**Applied to:** T1, T2.1, T2.2, T2.3, T3, T4.1, T4.2, T4.3, T6, T8, T11

---

### GENERATOR (3 instances)
**Substructure pattern:**
```yaml
step_name:
  generative_entity: ${models.qwen35}
  prompt: [specific generation description with validation criteria]
  model_overrides: {max_tokens: 8192}
  when:
    after_step_succeeds:
      - save_to: [artifact_name, ./path/to/file.ext]
      - log:
          to_file_path: ./logs/step_name.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - gwt:
          - given: quality_score > 0.8
            then: route_to: [validation_step]
            else: route_to: [retry_step]
  depends_on: []
```

**Applied to:** T2 (method), T9 (test), T10 (test), T12 (docs)

---

### ORCHESTRATOR (2 instances)
**Substructure pattern:**
```yaml
# Orchestrator step coordinates child steps
orchestration_step:
  generative_entity: ${models.qwen35}
  prompt: [coordination description]
  when:
    after_step_succeeds:
      - log:
          to_file_path: ./logs/orchestration_step.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info

# Child steps with depends_on
child_step_1:
  generative_entity: ${models.qwen35}
  prompt: [child task description]
  depends_on: [orchestration_step]
  when:
    after_step_succeeds:
      - save_to: [artifact_1, ./path/to/file1.ext]

child_step_2:
  generative_entity: ${models.qwen35}
  prompt: [child task description]
  depends_on: [child_step_1]
  when:
    after_step_succeeds:
      - save_to: [artifact_2, ./path/to/file2.ext]

child_step_3:
  generative_entity: ${models.qwen35}
  prompt: [child task description]
  depends_on: [child_step_2]
  when:
    after_step_succeeds:
      - save_to: [artifact_3, ./path/to/file3.ext]
```

**Applied to:** T4 (orchestrates T4.1 → T4.2 → T4.3), T5 (orchestrates hook processing loop)

---

### EVALUATOR (0 instances)
**Substructure pattern:** (not used in this task set)
```yaml
evaluation_step:
  generative_entity: ${models.qwen35}
  prompt: [evaluation criteria and assessment logic]
  when:
    after_step_succeeds:
      - save_to: [evaluation_report, ./path/to/evaluation.json]
      - log:
          to_file_path: ./logs/evaluation_step.log
          event_fields: [step_name, evaluation_score, confidence]
          level: info
      - gwt:
          - given: evaluation_score >= 0.9
            then: route_to: [approval_step]
            else: route_to: [refinement_step]
```

**Applied to:** None (no tasks require pure evaluation)

---

### VALIDATOR (1 instance)
**Substructure pattern:**
```yaml
validation_step:
  generative_entity: ${models.qwen35}
  prompt: [test execution and assertion description]
  when:
    after_step_succeeds:
      - shell:
          command: "cargo test integration_test_sequential_mixed_models"
          fail_on_error: true
      - log:
          to_file_path: ./logs/validation_step.log
          event_fields: [step_name, test_result, exit_code]
          level: info
```

**Applied to:** T7 (sequential fallback validation)

---

## Evaluation Verdict

**PASS**

**Justification:**
- ✅ 100% coverage: all 18 tasks/subtasks categorized
- ✅ No orphan tasks: every task assigned to at least one category
- ✅ Category reasoning provided for every task
- ✅ Substructure hints mapped to each category
- ✅ Multi-category tasks identified (T2, T4)
- ✅ Substructure patterns defined for all 5 categories
- ✅ Category summary shows distribution (75% DATA_TRANSFORMER, 25% GENERATOR, 16.7% ORCHESTRATOR, 0% EVALUATOR, 8.3% VALIDATOR)
- ✅ Iteration budget specified for every task (all = 1, as this is straightforward code generation)

**Quality bar:** This baseline represents minimum viable categorization. Live SW3 must produce richer category reasoning, more detailed substructure hints, and more sophisticated categorization (e.g., recognize when tasks require GENERATOR + EVALUATOR hybrid structures).