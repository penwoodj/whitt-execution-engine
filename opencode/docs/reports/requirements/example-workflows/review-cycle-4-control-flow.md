# Critical Review Cycle 4: Control Flow

**Date**: 2026-03-08
**Scope**: Control Flow (loops, retry, validation, execution modes)
**Reviewer**: Sisyphus

---

## Review Process

Reviewed loop types, retry logic, validation criteria, and execution mode consistency.

---

## Findings

### 1. Loop Type Coverage

**Coverage**: Partial - validation loops shown, but missing other loop types

**Present**:
- ✅ Validation-based loops (exact) - ex01, ex02
- ✅ Validation-based loops (abstract) - ex01, ex03
- ✅ Convergence loops - ex05

**Missing**:
- ❌ Count-based loops (execute exactly N times)
- ❌ Time-based loops (execute for duration)
- ❌ Infinite loops (continue until manual termination)
- ❌ Nested loops (loop within loop)

**Example Gap**: No example shows:
```yaml
loops:
  - type: count
    iterations: 10

  - type: time
    duration_secs: 300

  - type: infinite
    stop_on: manual_termination

  - type: nested
    outer:
      type: validation
      max_iterations: 5
    inner:
      type: count
      iterations: 3
```

---

### 2. Loop Integration with Pipeline

**Issue**: Loop syntax not integrated with pipeline steps

**Current Examples**:
- ex01: `validation_loop` at top level, references `step_3`
- ex05: `convergence_loop` at top level, defines steps within loop
- **Problem**: Inconsistent approach - some loops reference steps, others define steps

**Question**: When are steps inside the loop vs outside?

**Recommendation**: Consistent loop-pipeline integration:
```yaml
# Option A: Loop as wrapper around pipeline steps
# Note (v2.0): pipeline: replaced with agentic_workflow: in unified schema
pipeline:
  - step: analyze
    loop:
      id: retry_loop
      type: count
      iterations: 3

# Option B: Steps specify loop behavior
pipeline:
  - step: optimize
    repeat:
      type: convergence
      threshold: 0.01

# Option C: Loops defined separately, explicitly assign steps
loops:
  - id: optimization_loop
    type: convergence
    steps: [optimize_1, optimize_2]
```

---

### 3. Loop Control Parameters

**Issue**: Incomplete loop control specification

**Current Examples**:
- ex01: `max_iterations: 5`, `stop_condition`
- ex05: `threshold`, `metric`, `max_iterations`

**Missing Control Features**:
- ❌ `break_on`: Break loop on specific condition
- ❌ `continue_on`: Continue despite condition
- ❌ `timeout`: Maximum loop duration
- ❌ `continue_after_failure`: Continue even if step fails
- ❌ `save_iteration_results`: Store results from each iteration

**Example Gap**: No example shows:
```yaml
loops:
  - type: validation
    max_iterations: 10
    break_on:
      - error_severity: critical
      - user_cancellation: true
    continue_on:
      - error_type: timeout
    timeout_secs: 300
    save_iteration_results:
      - path: ./workspace/iterations/
      format: json
```

---

### 4. Retry Logic Consistency

**Coverage**: Good - default and overridable retry shown

**Issues**:

1. **Retry Scope Inconsistent**
   - ex01: `retry.default`
   - ex02: `retry.default` + `retry.step_specific`
   - ex06: `retry.rag_specific`
   - **Problem**: Different scoping levels (step vs rag vs tool)

2. **Backoff Strategy**
   - ex01: `backoff_strategy: exponential`
   - ex02: `backoff_strategy: linear`
   - **Question**: What does "linear" mean? Fixed delay? Incremental?

3. **Escalation Logic**
   - ex02: `on_failure: escalate_model`, `escalate_to: fallback_models`
   - **Gap**: No escalation strategy defined (sequential, random, performance-based)

**Recommendation**: Unified retry schema:
```yaml
retry:
  global:
    max_attempts: 3
    backoff:
      strategy: exponential  # exponential, linear, fixed, fibonacci
      base_delay_ms: 1000
      max_delay_ms: 10000
      jitter: true
    on_final_failure:
      action: escalate  # escalate, stop, continue, fallback
      escalation_strategy:
        type: performance_based  # sequential, random, performance_based
        fallback_chain: [model_b, model_c]

  step_override:
    step_1:
      max_attempts: 5
      backoff:
        strategy: linear
        base_delay_ms: 500

  error_specific:
    - error_type: timeout
      max_attempts: 5
    - error_type: rate_limit
      max_attempts: 10
      backoff:
        strategy: exponential
        base_delay_ms: 60000  # 1 minute
```

---

### 5. Validation Criteria Specification

**Coverage**: Good - both exact and abstract validation shown

**Issues**:

1. **Abstract Validation Criteria Unclear**
   - ex01: `abstract_criteria: improvement_detected`
   - ex03: `abstract_criteria: quality_threshold_met`
   - **Problem**: What defines "improvement" or "quality"?
   - **Gap**: No way to define custom abstract criteria

2. **Exact Validation Limited**
   - ex02: `contains_field: "quality_assessment"`, `all_priorities_valid: true`
   - **Gap**: Limited validation types shown
   - **Missing**: Schema validation, format validation, range validation, regex validation

**Recommendation**: Enhanced validation schema:
```yaml
validation:
  abstract_criteria:
    - name: improvement_detected
      definition: |
        Compare current iteration with previous.
        Improvement if:
        - Quality score increase > threshold
        - Error count decrease
        - Performance metric improvement
      tolerance: 0.05

    - name: quality_threshold_met
      definition: |
        Code quality metrics exceed threshold:
        - Code coverage > 80%
        - Cyclomatic complexity < 10
        - No critical issues
      threshold: 0.85

  exact_criteria:
    - type: schema_validation
      schema_path: /schemas/output_schema.json

    - type: format_validation
      format: json
      required_fields: [id, name, value]

    - type: range_validation
      field: score
      min: 0.0
      max: 1.0

    - type: regex_validation
      field: url
      pattern: "^https?://.*"

    - type: custom_function
      function: python:./workspace/validators/check_output.py
      args:
        - output: "${result}"
        - rules: "${validation_rules}"
```

---

### 6. Execution Mode Coherence

**Issue**: Execution modes not fully specified

**Current Examples**:
- ex01: `execution.mode: parallel`
- ex02: `execution.mode: serial`
- ex03: `execution.mode: dynamic_parallelization`
- **Gap**: What is `dynamic_parallelization` vs `parallel`? Difference unclear

**Missing**:
- ❌ `hybrid` mode specification
- ❌ How to specify which steps run in parallel vs serial within same workflow

**Recommendation**: Clarify execution modes:
```yaml
execution:
  # Mode: Overall execution strategy
  mode: hybrid  # parallel, serial, hybrid, custom

  # Parallel: All steps run concurrently (if no dependencies)
  parallel:
    max_concurrent_steps: 4
    dependency_aware: true

  # Serial: Steps run sequentially
  serial:
    fail_fast: true  # Stop on first error or continue

  # Hybrid: Mix of parallel and serial
  hybrid:
    parallel_groups:
      - group_1: [analyze_a, analyze_b]
      group_2: [synthesize]
    serial_between_groups: true

  # Custom: User-defined execution order
  custom:
    order:
      - step: analyze_a
        parallel_with: analyze_b
      - step: synthesize
        depends_on: [analyze_a, analyze_b]
```

---

### 7. Loop Termination Handling

**Gap**: No examples show what happens after loop terminates

**Questions**:
- Does workflow continue to next step after loop?
- Are loop results saved?
- What if loop terminates with error?

**Recommendation**: Add loop termination handling:
```yaml
loops:
  - type: validation
    max_iterations: 5

    on_termination:
      success:
        continue_workflow: true
        save_results:
          - last_iteration: true
          - all_iterations: true
          - path: ./workspace/loop_results/

      failure:
        continue_workflow: false  # or continue with warning
        error_handling: use_fallback_strategy
        notify_user: true
```

---

## Critical Gaps

### 1. Loop Nesting Support

**Missing**: Schema doesn't show how to define nested loops

**Use Case**: Optimize with validation, then repeat optimization process

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
loops:
  - id: outer_optimization
    type: convergence
    inner_loop:
      id: validation
      type: exact_validation
```

---

### 2. Dynamic Loop Parameters

**Missing**: Can loop parameters be computed at runtime?

**Use Case**: Loop until a condition computed during execution is met

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
loops:
  - type: validation
    max_iterations: "${workflow.config.max_iterations}"
    condition: "${current.score} >= ${workflow.config.target_score}"
```

---

### 3. Loop State Persistence

**Missing**: How is loop state passed between iterations?

**Use Case**: Each iteration builds on previous results

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
loops:
  - type: count
    iterations: 10
    state:
      accumulate_results: true
      pass_to_next_iteration:
        - best_score
        - optimization_history
```

---

## Summary of Control Flow Gaps

1. ❌ Count-based loops missing
2. ❌ Time-based loops missing
3. ❌ Infinite loops missing
4. ❌ Nested loops missing
5. ❌ Loop integration with pipeline inconsistent
6. ❌ Loop control parameters incomplete
7. ❌ Retry scope inconsistent
8. ❌ Backoff strategy unclear
9. ❌ Escalation logic undefined
10. ❌ Abstract validation criteria undefined
11. ❌ Exact validation types limited
12. ❌ Execution modes not fully specified
13. ❌ Loop termination handling missing
14. ❌ Dynamic loop parameters missing
15. ❌ Loop state persistence undefined

**Total Gaps**: 15

---

## Action Items

### High Priority
1. Add count-based loop examples
2. Add time-based loop examples
3. Add infinite loop examples
4. Standardize loop-pipeline integration

### Medium Priority
5. Define abstract validation criteria specification
6. Expand exact validation types
7. Clarify execution modes (parallel, serial, hybrid)
8. Add loop termination handling

### Low Priority
9. Add nested loop examples
10. Add dynamic loop parameter examples
11. Add loop state persistence examples

---

## Next Steps

Proceed to Review Cycle 5: Tool and Permission Model
