# Task 05: Loop Runner

**Goal:** Implement loop runner for all 6 loop types: count, foreach, while, validation, retry, infinite loops.

**Files:**
- Create: `src/control_flow/mod.rs`
- Create: `src/control_flow/loop.rs`
- Create: `tests/unit/loop_runner_test.rs`

---

## Rust Definitions

### `src/control_flow/loop.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::ir::LoopConfig;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Loop types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopType {
    Count,
    Foreach,
    While,
    Validation,
    Retry,
    Infinite,
}

/// Loop execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopResult {
    pub iterations: u32,
    pub outputs: Vec<StepOutput>,
    pub completed: bool,
    pub error: Option<String>,
}

impl LoopResult {
    pub fn success(iterations: u32, outputs: Vec<StepOutput>) -> Self {
        Self {
            iterations,
            outputs,
            completed: true,
            error: None,
        }
    }

    pub fn failure(iterations: u32, outputs: Vec<StepOutput>, error: String) -> Self {
        Self {
            iterations,
            outputs,
            completed: false,
            error: Some(error),
        }
    }
}

/// Loop execution error
#[derive(Debug, Error)]
pub enum LoopError {
    #[error("Invalid loop configuration: {0}")]
    InvalidConfig(String),

    #[error("Loop exceeded maximum iterations: {0}")]
    MaxIterationsExceeded(u32),

    #[error("Loop condition error: {0}")]
    ConditionError(String),

    #[error("Step execution error: {0}")]
    StepExecutionError(String),
}

/// Loop runner
pub struct LoopRunner {
    step_executor: Box<dyn StepExecutor>,
    max_iterations: u32,
}

impl LoopRunner {
    pub fn new(step_executor: Box<dyn StepExecutor>, max_iterations: u32) -> Self {
        Self {
            step_executor,
            max_iterations,
        }
    }

    /// Execute a loop
    pub async fn execute_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        match loop_config.loop_type.as_str() {
            "count" => self.execute_count_loop(loop_config, context).await,
            "foreach" => self.execute_foreach_loop(loop_config, context).await,
            "while" => self.execute_while_loop(loop_config, context).await,
            "validation" => self.execute_validation_loop(loop_config, context).await,
            "retry" => self.execute_retry_loop(loop_config, context).await,
            "infinite" => self.execute_infinite_loop(loop_config, context).await,
            _ => Err(LoopError::InvalidConfig(format!("Unknown loop type: {}", loop_config.loop_type))),
        }
    }

    /// Execute count loop
    async fn execute_count_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let count = loop_config
            .count
            .ok_or_else(|| LoopError::InvalidConfig("Missing count".to_string()))?;

        let mut outputs = Vec::new();

        for i in 0..count {
            if i >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            // Set loop index variable
            context.set_variable("loop_index".to_string(), serde_json::json!(i));

            // Execute step
            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output);
        }

        Ok(LoopResult::success(count, outputs))
    }

    /// Execute foreach loop
    async fn execute_foreach_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let items = loop_config
            .items
            .as_ref()
            .ok_or_else(|| LoopError::InvalidConfig("Missing items".to_string()))?;

        let mut outputs = Vec::new();

        for (i, item) in items.iter().enumerate() {
            if i as u32 >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            // Set loop item and index variables
            context.set_variable("loop_item".to_string(), item.clone());
            context.set_variable("loop_index".to_string(), serde_json::json!(i));

            // Execute step
            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output);
        }

        Ok(LoopResult::success(items.len() as u32, outputs))
    }

    /// Execute while loop
    async fn execute_while_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let mut iteration = 0;
        let mut outputs = Vec::new();

        loop {
            if iteration >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            // Evaluate condition
            let should_continue = self
                .evaluate_condition(loop_config, context)
                .await?;

            if !should_continue {
                break;
            }

            context.set_variable("loop_iteration".to_string(), serde_json::json!(iteration));

            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output);
            iteration += 1;
        }

        Ok(LoopResult::success(iteration, outputs))
    }

    /// Execute validation loop
    async fn execute_validation_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let mut iteration = 0;
        let mut outputs = Vec::new();

        loop {
            if iteration >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            context.set_variable("loop_iteration".to_string(), serde_json::json!(iteration));

            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output.clone());

            // Check if validation passed
            if output.success {
                // Run validation rule
                let validated = self
                    .validate_output(loop_config, &output, context)
                    .await?;

                if validated {
                    return Ok(LoopResult::success(iteration + 1, outputs));
                }
            }

            iteration += 1;
        }
    }

    /// Execute retry loop
    async fn execute_retry_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let max_retries = loop_config
            .max_retries
            .ok_or_else(|| LoopError::InvalidConfig("Missing max_retries".to_string()))?;

        let mut iteration = 0;
        let mut outputs = Vec::new();

        loop {
            if iteration >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            context.set_variable("loop_iteration".to_string(), serde_json::json!(iteration));

            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output.clone());

            // If step succeeded, return
            if output.success {
                return Ok(LoopResult::success(iteration + 1, outputs));
            }

            // If max retries reached, fail
            if iteration >= max_retries {
                return Ok(LoopResult::failure(
                    iteration + 1,
                    outputs,
                    output.error.unwrap_or_else(|| "Step failed".to_string()),
                ));
            }

            iteration += 1;

            // Apply backoff if configured
            if let Some(backoff_ms) = loop_config.backoff_ms {
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms as u64)).await;
            }
        }
    }

    /// Execute infinite loop (with max iterations as safety)
    async fn execute_infinite_loop(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<LoopResult, LoopError> {
        let mut iteration = 0;
        let mut outputs = Vec::new();

        loop {
            if iteration >= self.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.max_iterations));
            }

            context.set_variable("loop_iteration".to_string(), serde_json::json!(iteration));

            let output = self
                .execute_step_with_config(loop_config, context)
                .await?;

            outputs.push(output);
            iteration += 1;
        }
    }

    /// Execute step with loop configuration
    async fn execute_step_with_config(
        &self,
        loop_config: &LoopConfig,
        context: &mut ExecutionContext,
    ) -> Result<StepOutput, LoopError> {
        // TODO: Get step from loop_config
        // This will be implemented when we have the full IR structure
        Ok(StepOutput::success(serde_json::json!({})))
    }

    /// Evaluate loop condition
    async fn evaluate_condition(
        &self,
        loop_config: &LoopConfig,
        context: &ExecutionContext,
    ) -> Result<bool, LoopError> {
        // TODO: Implement condition evaluation
        // This will use the interpolation logic from Phase 0
        Ok(true)
    }

    /// Validate step output
    async fn validate_output(
        &self,
        loop_config: &LoopConfig,
        output: &StepOutput,
        context: &ExecutionContext,
    ) -> Result<bool, LoopError> {
        // TODO: Implement validation logic
        // This will use the validation configuration from the schema
        Ok(true)
    }
}
```

### `src/control_flow/mod.rs`

```rust
pub mod loop_;

pub use loop_::{LoopError, LoopResult, LoopRunner, LoopType};
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement loop.rs**

- [ ] **Step 3: Implement mod.rs**

- [ ] **Step 4: Add control_flow to lib.rs**

- [ ] **Step 5: Run tests**

- [ ] **Step 6: Commit**

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
