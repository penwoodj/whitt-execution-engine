# Task 09: Execution Modes

**Goal:** Implement execution mode strategies: serial, parallel, and hybrid execution with step coordination.

**Files:**
- Create: `src/execution_mode/mod.rs`
- Create: `src/execution_mode/serial.rs`
- Create: `src/execution_mode/parallel.rs`
- Create: `src/execution_mode/hybrid.rs`
- Create: `tests/unit/execution_mode_test.rs`

---

## Rust Definitions

### `src/execution_mode/serial.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use async_trait::async_trait;

/// Serial execution strategy (steps execute one after another)
pub struct SerialExecutor {
    step_executor: Box<dyn StepExecutor>,
}

impl SerialExecutor {
    pub fn new(step_executor: Box<dyn StepExecutor>) -> Self {
        Self { step_executor }
    }

    /// Execute steps serially
    pub async fn execute_serial(
        &self,
        steps: &[crate::ir::Step],
        context: &mut ExecutionContext,
    ) -> Result<Vec<StepOutput>, String> {
        let mut outputs = Vec::new();

        for step in steps {
            let output = self.step_executor.execute_step(step, context).await?;

            if !output.success {
                return Err(output.error.unwrap_or_else(|| "Step failed".to_string()));
            }

            outputs.push(output);
        }

        Ok(outputs)
    }
}
```

### `src/execution_mode/parallel.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::control_flow::parallel::ParallelGroup;

/// Parallel execution strategy (independent steps execute concurrently)
pub struct ParallelExecutionStrategy {
    step_executor: Box<dyn StepExecutor>,
    parallel_executor: crate::control_flow::parallel::ParallelExecutor,
}

impl ParallelExecutionStrategy {
    pub fn new(step_executor: Box<dyn StepExecutor>) -> Self {
        let parallel_executor = crate::control_flow::parallel::ParallelExecutor::new(
            step_executor.clone_box(),
        );

        Self {
            step_executor,
            parallel_executor,
        }
    }

    /// Execute steps in parallel
    pub async fn execute_parallel(
        &self,
        group: &ParallelGroup,
        context: &ExecutionContext,
    ) -> Result<Vec<StepOutput>, String> {
        let result = self.parallel_executor.execute_parallel(group, context).await
            .map_err(|e| e.to_string())?;

        Ok(result.outputs)
    }
}

// Helper trait for Box<dyn StepExecutor>
trait StepExecutorClone: StepExecutor {
    fn clone_box(&self) -> Box<dyn StepExecutor>;
}

impl<T> StepExecutorClone for T
where
    T: StepExecutor + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn StepExecutor> {
        Box::new(self.clone())
    }
}
```

### `src/execution_mode/hybrid.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::control_flow::parallel::ParallelGroup;

/// Hybrid execution strategy (mix of serial and parallel based on dependencies)
pub struct HybridExecutor {
    step_executor: Box<dyn StepExecutor>,
    serial_executor: crate::execution_mode::serial::SerialExecutor,
    parallel_strategy: crate::execution_mode::parallel::ParallelExecutionStrategy,
}

impl HybridExecutor {
    pub fn new(step_executor: Box<dyn StepExecutor>) -> Self {
        let serial_executor = crate::execution_mode::serial::SerialExecutor::new(
            step_executor.clone_box(),
        );

        let parallel_strategy = crate::execution_mode::parallel::ParallelExecutionStrategy::new(
            step_executor.clone_box(),
        );

        Self {
            step_executor,
            serial_executor,
            parallel_strategy,
        }
    }

    /// Execute workflow with hybrid strategy
    pub async fn execute_hybrid(
        &self,
        steps: &[crate::ir::Step],
        parallel_groups: &[ParallelGroup],
        context: &mut ExecutionContext,
    ) -> Result<Vec<StepOutput>, String> {
        // TODO: Analyze dependencies to determine execution strategy
        // For now, execute serially as fallback
        self.serial_executor.execute_serial(steps, context).await
    }

    /// Determine if steps can be executed in parallel
    fn can_execute_parallel(&self, steps: &[crate::ir::Step]) -> bool {
        // Check if steps have no dependencies between them
        true // Placeholder - implement dependency analysis
    }
}
```

### `src/execution_mode/mod.rs`

```rust
pub mod hybrid;
pub mod parallel;
pub mod serial;

pub use hybrid::HybridExecutor;
pub use parallel::ParallelExecutionStrategy;
pub use serial::SerialExecutor;
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement serial.rs**

- [ ] **Step 3: Implement parallel.rs**

- [ ] **Step 4: Implement hybrid.rs**

- [ ] **Step 5: Implement mod.rs**

- [ ] **Step 6: Add execution_mode to lib.rs**

- [ ] **Step 7: Run tests**

- [ ] **Step 8: Commit**

---

## Additional Edge Case Tests

### Step 9: Empty step list test

```rust
#[test]
fn test_empty_step_list_serial() {
    let executor = SerialExecutor::new(Box::new(MockStepExecutor::new()));
    let steps: Vec<Step> = vec![];
    let mut context = ExecutionContext::new();

    let result = executor.execute_serial(&steps, &mut context).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_empty_step_list_parallel() {
    let executor = ParallelExecutionStrategy::new(Box::new(MockStepExecutor::new()));
    let group = ParallelGroup {
        steps: vec![],
        max_parallel: 4,
    };
    let context = ExecutionContext::new();

    let result = executor.execute_parallel(&group, &context).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
```

- [ ] **Step 9a:** Implement empty step tests for all modes
- [ ] **Step 9b:** Run tests to verify graceful handling
- [ ] **Step 9c:** Commit: `test: add empty step list edge cases`

### Step 10: Single step test

```rust
#[test]
fn test_single_step_serial() {
    let executor = SerialExecutor::new(Box::new(MockStepExecutor::new()));
    let steps = vec![create_mock_step("step_1")];
    let mut context = ExecutionContext::new();

    let result = executor.execute_serial(&steps, &mut context).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn test_single_step_parallel() {
    let executor = ParallelExecutionStrategy::new(Box::new(MockStepExecutor::new()));
    let group = ParallelGroup {
        steps: vec![create_mock_step("step_1")],
        max_parallel: 4,
    };
    let context = ExecutionContext::new();

    let result = executor.execute_parallel(&group, &context).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}
```

- [ ] **Step 10a:** Implement single step tests
- [ ] **Step 10b:** Run tests to verify single step behavior
- [ ] **Step 10c:** Commit: `test: add single step edge case`

### Step 11: Circular dependencies in hybrid mode

```rust
#[test]
fn test_circular_dependencies_hybrid() {
    let executor = HybridExecutor::new(Box::new(MockStepExecutor::new()));

    // Create steps with circular dependencies
    let step1 = create_mock_step("step_1");
    let step2 = create_mock_step("step_2");
    let step3 = create_mock_step("step_3");

    // step1 depends on step3, step3 depends on step2, step2 depends on step1
    let steps = vec![step1, step2, step3];
    let mut context = ExecutionContext::new();

    let result = executor.execute_hybrid(&steps, &[], &mut context).await;

    // Should detect circular dependency and fail
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Circular dependency"));
}
```

- [ ] **Step 11a:** Implement circular dependency detection in hybrid
- [ ] **Step 11b:** Add test for circular dependency handling
- [ ] **Step 11c:** Run test to verify detection
- [ ] **Step 11d:** Commit: `test: add circular dependency handling`

---

## Integration Tests

### Step 12: Serial→Parallel switch mid-workflow

```rust
#[test]
fn test_serial_to_parallel_switch() {
    let mut context = ExecutionContext::new();
    let executor = HybridExecutor::new(Box::new(MockStepExecutor::new()));

    // First phase: serial execution
    let serial_steps = vec![create_mock_step("serial_1"), create_mock_step("serial_2")];
    let result = executor.execute_hybrid(&serial_steps, &[], &mut context).await;
    assert!(result.is_ok());

    // Second phase: parallel execution
    let parallel_group = ParallelGroup {
        steps: vec![create_mock_step("parallel_1"), create_mock_step("parallel_2")],
        max_parallel: 2,
    };
    let result = executor.parallel_strategy.execute_parallel(&parallel_group, &context).await;
    assert!(result.is_ok());
}
```

- [ ] **Step 12a:** Implement mid-workflow mode switching
- [ ] **Step 12b:** Add integration test
- [ ] **Step 12c:** Run test to verify seamless switching
- [ ] **Step 12d:** Commit: `test: add serial-parallel switch integration`

### Step 13: Resource exhaustion handling

```rust
#[test]
fn test_resource_exhaustion_parallel() {
    let executor = ParallelExecutionStrategy::new(Box::new(MockStepExecutor::new()));

    // Create many steps that exceed memory limits
    let steps: Vec<Step> = (0..100).map(|i| create_memory_intensive_step(format!("step_{}", i))).collect();

    let group = ParallelGroup {
        steps,
        max_parallel: 10,
    };
    let context = ExecutionContext::new();

    let result = executor.execute_parallel(&group, &context).await;

    // Should handle exhaustion gracefully
    assert!(result.is_err() || result.unwrap().len() < 100);
}
```

- [ ] **Step 13a:** Implement resource limit checks
- [ ] **Step 13b:** Add exhaustion handling test
- [ ] **Step 13c:** Run test to verify graceful degradation
- [ ] **Step 13d:** Commit: `test: add resource exhaustion handling`

---

## Mock Strategies for Execution Mode Testing

### Step 14: Mock step executor

```rust
pub struct MockStepExecutor {
    execution_time_ms: u64,
    failure_rate: f64,
}

impl MockStepExecutor {
    pub fn new() -> Self {
        Self {
            execution_time_ms: 10,
            failure_rate: 0.0,
        }
    }

    pub fn with_delay(mut self, ms: u64) -> Self {
        self.execution_time_ms = ms;
        self
    }

    pub fn with_failure_rate(mut self, rate: f64) -> Self {
        self.failure_rate = rate;
        self
    }
}

#[async_trait]
impl StepExecutor for MockStepExecutor {
    async fn execute_step(
        &self,
        step: &Step,
        context: &mut ExecutionContext,
    ) -> Result<StepOutput, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.execution_time_ms)).await;

        if rand::random::<f64>() < self.failure_rate {
            return Ok(StepOutput {
                success: false,
                error: Some("Simulated failure".to_string()),
                data: None,
            });
        }

        Ok(StepOutput {
            success: true,
            error: None,
            data: Some(serde_json::json!({"step_id": step.id})),
        })
    }
}
```

- [ ] **Step 14a:** Implement mock step executor
- [ ] **Step 14b:** Add configuration options (delay, failure rate)
- [ ] **Step 14c:** Unit tests for mock executor
- [ ] **Step 14d:** Commit: `test: add mock step executor`

---

## Performance Validation

### Step 15: Throughput comparison serial vs parallel vs hybrid

```rust
#[test]
fn test_throughput_comparison() {
    let steps: Vec<Step> = (0..50).map(|i| create_mock_step(format!("step_{}", i))).collect();
    let mut context = ExecutionContext::new();

    // Serial execution
    let serial_executor = SerialExecutor::new(Box::new(MockStepExecutor::new()));
    let start = std::time::Instant::now();
    let _ = serial_executor.execute_serial(&steps, &mut context).await;
    let serial_duration = start.elapsed();

    // Parallel execution
    let parallel_executor = ParallelExecutionStrategy::new(Box::new(MockStepExecutor::new()));
    let group = ParallelGroup {
        steps: steps.clone(),
        max_parallel: 10,
    };
    let start = std::time::Instant::now();
    let _ = parallel_executor.execute_parallel(&group, &context).await;
    let parallel_duration = start.elapsed();

    // Parallel should be significantly faster than serial
    assert!(parallel_duration.as_millis() < serial_duration.as_millis(),
            "Parallel ({:?}) should be faster than serial ({:?})",
            parallel_duration, serial_duration);

    // Verify speedup is reasonable (at least 2x for 10-way parallelism)
    let speedup = serial_duration.as_millis() as f64 / parallel_duration.as_millis() as f64;
    assert!(speedup > 2.0, "Speedup should be >2x, got {}", speedup);
}
```

- [ ] **Step 15a:** Implement throughput comparison test
- [ ] **Step 15b:** Run benchmarks for all three modes
- [ ] **Step 15c:** Document performance characteristics
- [ ] **Step 15d:** Commit: `test: add throughput comparison benchmarks`

---

## Error Recovery

### Step 16: Step failure in parallel mode

```rust
#[test]
fn test_step_failure_parallel() {
    let executor = ParallelExecutionStrategy::new(
        Box::new(MockStepExecutor::new().with_failure_rate(0.2))
    );

    let steps: Vec<Step> = (0..10).map(|i| create_mock_step(format!("step_{}", i))).collect();
    let group = ParallelGroup {
        steps,
        max_parallel: 5,
    };
    let context = ExecutionContext::new();

    let result = executor.execute_parallel(&group, &context).await;

    // Should handle partial failures
    assert!(result.is_ok());
    let outputs = result.unwrap();
    assert!(outputs.len() > 0);
}
```

- [ ] **Step 16a:** Implement partial failure handling
- [ ] **Step 16b:** Add test for parallel mode failures
- [ ] **Step 16c:** Run test to verify graceful degradation
- [ ] **Step 16d:** Commit: `test: add parallel mode failure recovery`

### Step 17: Partial completion handling

```rust
#[test]
fn test_partial_completion() {
    let executor = HybridExecutor::new(Box::new(MockStepExecutor::new()));

    // First group: complete successfully
    let group1_steps = vec![create_mock_step("g1_s1"), create_mock_step("g1_s2")];
    let mut context = ExecutionContext::new();
    let result1 = executor.execute_hybrid(&group1_steps, &[], &mut context).await;
    assert!(result1.is_ok());

    // Second group: partial failure
    let group2_steps = vec![create_mock_step("g2_s1"), create_failing_step("g2_s2")];
    let result2 = executor.execute_hybrid(&group2_steps, &[], &mut context).await;
    assert!(result2.is_err());

    // Third group: should continue despite previous failure
    let group3_steps = vec![create_mock_step("g3_s1")];
    let result3 = executor.execute_hybrid(&group3_steps, &[], &mut context).await;
    assert!(result3.is_ok());
}
```

- [ ] **Step 17a:** Implement partial completion logic
- [ ] **Step 17b:** Add multi-group test
- [ ] **Step 17c:** Run test to verify continuation
- [ ] **Step 17d:** Commit: `test: add partial completion handling`

---

## Verification

### Final verification tests

```bash
# Run all execution mode tests
cargo test execution_mode_test

# Expected output:
# test test_empty_step_list_serial ... ok
# test test_empty_step_list_parallel ... ok
# test test_single_step_serial ... ok
# test test_single_step_parallel ... ok
# test test_circular_dependencies_hybrid ... ok
# test test_serial_to_parallel_switch ... ok
# test test_resource_exhaustion_parallel ... ok
# test test_throughput_comparison ... ok
# test test_step_failure_parallel ... ok
# test test_partial_completion ... ok
# test result: ok. 10 passed in X.XXs

# Run performance benchmarks
cargo test --release -- --ignored --test-threads=1 test_throughput_comparison

# Expected output:
# Serial execution time: XXXms
# Parallel execution time: YYYms
# Speedup: Z.XXx
```

**Checkpoint Criteria:**
- ✅ Serial execution works correctly
- ✅ Parallel execution respects concurrency limits
- ✅ Hybrid execution handles mode switching
- ✅ Edge cases handled (empty, single step, circular dependencies)
- ✅ Integration tests pass (mode switching, resource exhaustion)
- ✅ Mock strategies work for testing
- ✅ Performance benchmarks show parallel speedup
- ✅ Error recovery works (partial failures, partial completion)
- ✅ All tests pass with expected output
