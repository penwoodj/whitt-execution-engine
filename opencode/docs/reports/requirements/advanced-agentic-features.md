# Advanced Agentic Features

## Overview

Advanced agentic features enable workflows to spawn multiple parallel agents, aggregate their results, and handle loop termination edge cases. These features move beyond single-step execution to complex, multi-agent workflows.

---

## Multi-Agent Spawning

### Why Parallel Agents?

**Use Cases:**
1. **Independent analysis**: Multiple agents analyze different aspects concurrently (code review, security scan, test generation)
2. **Divide-and-conquer**: Split large task into independent subtasks
3. **Redundancy**: Run same task with different models/prompts for quality
4. **Time-critical**: Complete workflow faster than serial execution

**Performance Benefits:**
- **Throughput**: N parallel agents = N× throughput (linear scaling)
- **Latency**: Overall workflow time = max(all agent times), not sum(all agent times)
- **Resource utilization**: Keep GPU/CPU busy during I/O waits

### Unified Schema Support

The unified schema v2.0 already supports multi-agent workflows via `parallel_group` and `sub_workflow`:

```yaml
steps:
  - name: analyze_code
    tools: [llm_generate]
    parallel_group: analysis_group  # Spawn 4 parallel agents
    parallel:
      - sub_workflow: security_scan
      - sub_workflow: performance_review
      - sub_workflow: test_generation
      - sub_workflow: doc_update
```

### Implementation: Parallel Agent Spawning

**Workflow Engine Responsibilities:**

1. **Parse parallel_group**: Identify steps in parallel execution group
2. **Spawn agents**: For each parallel sub-workflow, create independent execution context
3. **Track completion**: Monitor each agent independently
4. **Aggregate results**: Collect outputs when all agents complete
5. **Error handling**: Continue on failures, collect partial results

**Rust Implementation:**
```rust
use tokio::task::JoinSet;

pub struct ParallelAgentGroup {
    agents: Vec<AgentContext>,
}

impl ParallelAgentGroup {
    pub async fn execute(&self) -> Result<AggregatedOutput> {
        // 1. Spawn all agents concurrently
        let tasks: Vec<_> = self.agents.iter()
            .map(|agent| tokio::spawn(agent.execute()))
            .collect();

        // 2. Wait for all to complete (or fail)
        let results = JoinSet::new(tasks).await;

        // 3. Aggregate results
        let mut aggregated = AggregatedOutput::default();
        for result in results {
            match result {
                Ok(output) => aggregated.add(output),
                Err(e) => aggregated.add_error(e),
            }
        }

        Ok(aggregated)
    }
}
```

**Result Aggregation Strategies:**

| Strategy | Description | Use Case |
|----------|-------------|----------|
| **Merge all** | Combine all outputs into single result | Code review findings |
| **Vote/Majority** | Most common answer wins | Classification tasks |
| **Collect all** | Keep all outputs separate | Multiple analysis results |
| **First successful** | Use first output that succeeds | Any task valid |
| **Weighted score** | Score outputs, pick best | Quality evaluation |

**Example Aggregation:**
```yaml
steps:
  - name: aggregate_results
    depends_on: [analyze_code]
    parallel_group: analysis_group
    parallel:
      - sub_workflow: security_scan
      - sub_workflow: performance_review
      - sub_workflow: test_generation
      - sub_workflow: doc_update
    then:
      aggregate_strategy: "merge_all"  # merge_all, vote_first, collect_all
      output_field: "analysis_report"  # Field name containing all parallel outputs
```

**Unified Schema Fields:**
```yaml
parallel:
  parallel_group: analysis_group
  parallel_group_timeout_secs: 600  # Timeout for entire group
  max_parallel_workflows: 4           # Maximum concurrent sub-workflows
  aggregate_strategy: "merge_all"    # merge_all, vote_first, collect_all, first_success
```

### Error Handling for Parallel Agents

**Policies:**

1. **Continue on partial failure**: Don't fail entire workflow if one agent fails
2. **Collect errors**: Aggregate error messages from failed agents
3. **Retry policy**: Failed sub-workflows can be retried independently
4. **Timeout handling**: All agents stopped if group timeout exceeds

**Example:**
```rust
pub async fn execute_parallel_with_retry(
    group: ParallelGroup,
) -> Result<WorkflowOutput> {
    let results = group.execute().await?;

    // Check results
    let successful = results.iter().filter(|r| r.is_ok()).count();
    let total = results.len();

    if successful < total {
        warn!(
            "Partial success: {}/{} agents completed",
            successful, total
        );

        // Continue with partial results?
        if group.continue_on_partial_failure {
            return Ok(WorkflowOutput::Partial(results));
        } else {
            return Err(Error::ParallelAgentsFailed {
                successful,
                failed: total - successful,
            });
        }
    }

    Ok(WorkflowOutput::Complete(results))
}
```

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

**Detection:**
```rust
pub fn detect_no_progress(
    outputs: &[LoopOutput],
    threshold: usize,
) -> bool {
    if outputs.len() < threshold {
        return false; // Not enough history yet
    }

    // Check last N iterations for meaningful change
    let recent = &outputs[outputs.len() - threshold..];
    let first = &recent[0];

    for output in recent.iter().skip(1) {
        if !meaningfully_different(first, output) {
            return true; // No progress detected
        }
    }

    false
}

fn meaningfully_different(a: &LoopOutput, b: &LoopOutput) -> bool {
    // Define what constitutes "meaningful" change
    a.issues_found != b.issues_found
        || a.files_processed != b.files_processed
        || a.issues_fixed.abs_diff(b.issues_fixed) > 5
}
```

**Resolution:**
```yaml
steps:
  - name: fix_issues_loop
    loop:
      while:
        condition: "{{step.analyze_code.output.issues.length > 0}}"
        min_progress_iterations: 5
        on_no_progress: fail    # Stop if no progress for 5 iterations
```

### Edge Case 2: Consecutive Failure Handling

**Problem**: Loop keeps retrying but each iteration fails.

**Detection:**
```rust
pub fn detect_consecutive_failures(
    history: &[LoopResult],
    threshold: usize,
) -> bool {
    if history.len() < threshold {
        return false;
    }

    // Check last N results
    let recent = &history[history.len() - threshold..];

    // If all failed, consecutive failure detected
    recent.iter().all(|r| r.is_err())
}
```

**Resolution:**
```yaml
steps:
  - name: fix_issues_loop
    loop:
      while:
        condition: "{{step.analyze_code.output.issues.length > 0}}"
        on_consecutive_failures: 3       # Stop after 3 consecutive failures
        then:
          analyze_failure_reason  # Diagnostic step before exiting
```

### Edge Case 3: Unbounded Growth

**Problem**: Loop accumulates data without limit (e.g., building list, growing buffer).

**Detection:**
```rust
pub fn enforce_growth_limit<T>(
    collection: &mut Vec<T>,
    max_size: usize,
) -> Result<()> {
    if collection.len() >= max_size {
        return Err(Error::GrowthLimitExceeded {
            current: collection.len(),
            limit: max_size,
        });
    }

    Ok(())
}
```

**Resolution:**
```yaml
steps:
  - name: collect_results
    loop:
      while:
        condition: "{{step.search.output.has_more}}"
        max_iterations: 1000
        max_collection_size: 10000  # Prevent unbounded growth
        on_growth_limit_exceeded: fail
```

### Edge Case 4: Exit on Empty/Null Condition

**Problem**: Loop condition becomes false/falsey (empty list, null result).

**Detection:**
```rust
pub fn check_empty_condition(condition: &LoopCondition) -> bool {
    match condition {
        LoopCondition::HasMore(items) => !items.is_empty(),
        LoopCondition::ContinueWhile(expr) => !eval_truthiness(expr)?,
        LoopCondition::NotEmpty(value) => !is_empty(value),
    }
}
```

**Resolution:**
```yaml
steps:
  - name: process_results
    loop:
      while:
        condition: "{{step.search.output.next_item}}"
        on_empty_iterations: skip    # Skip iteration if no more items
        on_empty_iterations: fail    # Or fail the loop
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

### Complete Example: Multi-Agent Loop with Termination

```yaml
name: "Multi-Agent Parallel Analysis with Loop"

workflow:
  type: "parallel"  # parallel_group, serial, hybrid

steps:
  - name: discover_issues
    tools: [llm_generate, search_files]
    timeout_secs: 300
    then:
      analyze_code

  - name: analyze_code
    tools: [llm_generate]
    parallel_group: analysis_group
    parallel_group_timeout_secs: 600
    parallel:
      - sub_workflow: security_scan
      - sub_workflow: performance_review
      - sub_workflow: test_generation
      - sub_workflow: doc_update
    then:
      aggregate_results

  - name: aggregate_results
    depends_on: [analyze_code]
    aggregate_strategy: "merge_all"
    output_field: "analysis_report"

  - name: fix_issues_loop
    depends_on: [aggregate_results]
    loop:
      while:
        condition: "{{step.aggregate_results.output.issues_found.length > 0}}"
        max_iterations: 100
        on_consecutive_failures: 3
        min_progress_iterations: 5
        when:
          - condition: "{{loop.iteration_duration_secs < 300}}"
            fail_if_timeout: true
        then:
          apply_fixes
          reanalyze

  - name: reanalyze
    depends_on: [fix_issues_loop]
    tools: [llm_generate]
    when:
      - condition: "{{step.fix_issues_loop.output.fixed.length > 0}}"
        skip_loop: false  # Continue loop
      else:
        skip_loop: true   # Exit loop

  - name: generate_report
    depends_on: [reanalyze]
    tools: [template_engine]
```

### Schema Fields Summary

| Field | Type | Purpose |
|-------|------|---------|
| `parallel_group` | string | Group parallel steps together |
| `parallel_group_timeout_secs` | number | Timeout for entire parallel group |
| `max_parallel_workflows` | number | Maximum concurrent sub-workflows |
| `aggregate_strategy` | enum | How to merge parallel outputs |
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

## Implementation Notes

### Treadle Integration

Treadle v0.2.0 provides native support for:
- **Stateful loops**: Per-iteration state tracking
- **Consecutive failure detection**: Built-in to `StageOutcome::NeedsReview`
- **Progress monitoring**: Track iteration results over time

### Rig Integration

Rig's agent framework provides:
- **Parallel spawning**: Built-in multi-agent support
- **Result aggregation**: Collect outputs from parallel agents
- **Error handling**: Continue on partial failures, retry policies

### Workflow Engine Responsibilities

1. **Parse parallel_group**: Identify steps in parallel execution group
2. **Track loop iterations**: Store iteration state (iteration number, outputs, metadata)
3. **Evaluate termination conditions**: Check max_iterations, consecutive_failures, no_progress
4. **Execute hooks**: Run when hooks for timeouts and conditions
5. **Aggregate results**: Combine parallel outputs based on strategy

---

## Testing

### Multi-Agent Tests

```rust
#[tokio::test]
async fn test_parallel_agent_execution() {
    let workflow = Workflow {
        parallel_group: "test_group",
        agents: vec![
            agent_mock("agent1"),
            agent_mock("agent2"),
            agent_mock("agent3"),
        ],
    };

    let result = workflow.execute().await.unwrap();

    assert_eq!(result.agents_completed, 3);
    assert!(result.aggregated.is_some());
}
```

### Loop Termination Tests

```rust
#[tokio::test]
async fn test_consecutive_failure_detection() {
    let mut loop_state = LoopState::new();

    // Simulate 3 consecutive failures
    for i in 0..3 {
        let result = simulate_failing_iteration().await;
        loop_state.add_result(result);
    }

    assert!(loop_state.should_terminate());
}

#[tokio::test]
async fn test_no_progress_detection() {
    let mut loop_state = LoopState::new();

    // Simulate no progress for 10 iterations
    for i in 0..10 {
        let result = simulate_no_progress().await;
        loop_state.add_result(result);
    }

    assert!(loop_state.should_terminate());
}
```

---

## Next Steps

1. Implement parallel group parsing in workflow engine
2. Add termination condition evaluation logic
3. Integrate with Treadle's loop state tracking
4. Add hooks for timeout and condition evaluation
5. Implement result aggregation strategies
6. Add tests for loop termination edge cases
7. Add tests for parallel agent spawning and aggregation
