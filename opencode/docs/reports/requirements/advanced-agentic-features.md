# Advanced Agentic Features

## Overview

Advanced agentic features enable workflows to handle complex scenarios like loop termination, timeout management, and state tracking.

---

## Parallel Execution: MOVED TO AGENT-QUEUE

**Status:** Parallel execution features (multi-agent spawning, result aggregation, parallel groups) have been moved to the agent-queue project. The remaining content in this file focuses on loop termination and timespan management for serial execution.

---

## Loop Termination Edge Cases

### Why Loop Termination Matters?

Loops in agentic workflows can terminate for multiple reasons:
1. **Goal achieved**: Loop should stop when objective is met
2. **No progress**: Loop should exit if iterations aren't making progress
3. **Error accumulation**: Too many consecutive failures should stop loop
4. **Resource exhaustion**: Stop before OOM or timeout
5. **External signal**: User cancels workflow

**Dangers of Incorrect Termination:**
- **Infinite loops**: Missing stop condition runs forever
- **Premature exit**: Stop too early, incomplete solution
- **Stuck loops**: No progress but not stopped, wasting resources
- **Unbounded growth**: Growing data structures cause OOM

### Unified Schema Loop Controls

The unified schema v2.0 provides loop constructs with termination controls:

```yaml
steps:
  - name: fix_issues_loop
    loop:
      while:
        condition: "{{step.analyze_code.output.issues.length > 0}}"
        max_iterations: 100
        on_empty_iterations: fail            # What to do if condition false
        on_max_iterations: fail
        on_consecutive_failures: 3         # Stop after 3 failures in a row
        min_iterations: 1                # Run at least once
        min_progress_iterations: 5         # Require progress every 5 iterations
```

### Termination Conditions

| Condition | Schema Field | Trigger | Example |
|-----------|---------------|---------|---------|
| **Goal achieved** | `while.condition = false` | Loop exits when condition met |
| **Max iterations** | `max_iterations: N` | Hard limit, prevent infinite loops |
| **Consecutive failures** | `on_consecutive_failures: N` | Stop after N failures in row |
| **Min iterations** | `min_iterations: N` | Ensure at least N runs |
| **Progress required** | `min_progress_iterations: N` | Require change every N iterations |
| **No progress** | `on_empty_iterations: fail/skip/continue` | What to do if condition false |

### Edge Case 1: No Progress Detection

**Problem**: Loop condition always true, but outputs aren't changing.

**Solution**: Track iteration outputs and compare for changes:

```rust
pub struct LoopState {
    iterations: Vec<IterationResult>,
    min_progress_iterations: usize,
}

impl LoopState {
    pub fn add_result(&mut self, result: IterationResult) {
        self.iterations.push(result);
    }

    pub fn check_progress(&self) -> bool {
        if self.iterations.len() < self.min_progress_iterations {
            return true;  // Not enough iterations yet
        }

        let last_n = &self.iterations[self.iterations.len() - self.min_progress_iterations..];
        !last_n.windows(2).all(|w| w[0].output == w[1].output)
    }
}
```

### Edge Case 2: Consecutive Failures

**Problem**: Loop retries but never succeeds, wasting resources.

**Solution**: Track consecutive failures and terminate:

```rust
impl LoopState {
    pub fn check_consecutive_failures(&self, limit: usize) -> bool {
        let failures = self.iterations
            .iter()
            .rev()
            .take(limit)
            .filter(|r| !r.success)
            .count();

        failures >= limit
    }
}
```

### Edge Case 3: No Progress After Max Attempts

**Problem**: Condition true, output changes, but quality doesn't improve.

**Solution**: Add quality threshold check:

```yaml
steps:
  - name: refine_code_loop
    loop:
      while:
        condition: "{{step.quality_score < 0.95}}"
        max_iterations: 100
        on_consecutive_failures: 3
        min_progress_iterations: 5
        on_max_iterations: skip
        min_quality_improvement: 0.01  # Require 1% improvement
```

---

## Timespan Management

### Timespan in Unified Schema

Timespans (duration constraints) are managed through hooks, not as separate loop constructs:

```yaml
steps:
  - name: analyze_code
    tools: [llm_generate]
    timeout_secs: 300
    when:
      - condition: "start_time < {{now}} - 86400"  # Run within 24 hours
        fail_if_timeout: true
```

**Why Hooks Instead of Timespan Field?**

1. **Flexibility**: Hooks can evaluate complex expressions, not just time
2. **Integration**: Hooks integrate with workflow state, conditions, errors
3. **Granularity**: Fine-grained control (per iteration, per step, overall)
4. **Reusability**: Same hook mechanism used for timeouts, rate limits, etc.

### Timespan Hook Examples

**Per-iteration timeout:**
```yaml
steps:
  - name: process_item
    loop:
      while:
        condition: "{{items}}"
        when:
          - condition: "{{loop.iteration_duration_secs < 300}}"
            fail_if_timeout: true
```

**Workflow-wide timeout:**
```yaml
steps:
  - name: analyze_code
    tools: [llm_generate]
    when:
      - condition: "{{workflow.start_time < {{now}} - 3600}}"
            fail_if_timeout: true
            fail_action: "cancel_workflow"  # or "continue", "skip_next"
```

**Cumulative time budget:**
```yaml
steps:
  - name: analyze_code
    tools: [llm_generate]
    when:
      - condition: "{{workflow.cumulative_time_secs < 7200}}"
            fail_if_timeout: true
            fail_action: "fail_workflow"
```

---

## Unified Schema Integration

### Complete Example: Serial Loop with Termination

```yaml
name: "Serial Code Analysis with Loop"

workflow:
  type: "serial"

steps:
  - name: discover_issues
    tools: [llm_generate, search_files]
    timeout_secs: 300
    when:
      route_to: analyze_code

  - name: analyze_code
    tools: [llm_generate]
    when:
      route_to: fix_issues_loop

  - name: fix_issues_loop
    loop:
      while:
        condition: "{{step.analyze_code.output.issues_found.length > 0}}"
        max_iterations: 100
        on_consecutive_failures: 3
        min_progress_iterations: 5
        when:
          - condition: "{{loop.iteration_duration_secs < 300}}"
            fail_if_timeout: true
        route_to: apply_fixes

  - name: generate_report
    depends_on: [fix_issues_loop]
    tools: [template_engine]
```

### Schema Fields Summary

| Field | Type | Purpose |
|-------|------|---------|
| `while` | object | Loop construct |
| `while.condition` | string/expression | Loop continuation condition |
| `max_iterations` | number | Hard iteration limit |
| `on_max_iterations` | string | Action: fail, skip, continue |
| `on_consecutive_failures` | number | Stop after N consecutive failures |
| `min_iterations` | number | Minimum iterations before checks apply |
| `min_progress_iterations` | number | Require progress every N iterations |
| `on_empty_iterations` | string | Action: fail, skip, continue (when condition false) |
| `when.condition` | string/expression | Hook for timeouts, rate limits, etc. |

---

## Related Documents

- [Unified Workflow Schema](./unifying-schema/unified-workflow-schema.yml) — Source of truth
- [Benchmark YAML Examples](./benchmark-yaml-examples.md) — Serial workflow examples
- [Configuration Defaults](./configuration-defaults.md) — Default values
- [Parallel Execution](../agent-queue/docs/parallel-execution.md) — Moved to agent-queue project
