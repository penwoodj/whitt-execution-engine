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
