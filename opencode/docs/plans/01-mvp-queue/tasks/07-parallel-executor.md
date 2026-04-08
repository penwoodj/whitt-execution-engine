# Task 07: Parallel Executor

**Goal:** Implement parallel executor for parallel groups with concurrency limits and resource constraints.

**Files:**
- Create: `src/control_flow/parallel.rs`
- Create: `tests/unit/parallel_executor_test.rs`

---

## Rust Definitions

### `src/control_flow/parallel.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Semaphore;

/// Parallel group configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelGroup {
    pub group_id: String,
    pub step_ids: Vec<String>,
    pub max_concurrency: usize,
    pub fail_fast: bool,
    pub resource_constraints: Option<ResourceConstraints>,
}

/// Resource constraints for parallel execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_memory_mb: Option<usize>,
    pub max_cpu_percent: Option<usize>,
}

/// Parallel execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelResult {
    pub outputs: Vec<StepOutput>,
    pub completed: u32,
    pub failed: u32,
    pub cancelled: bool,
}

impl ParallelResult {
    pub fn success(outputs: Vec<StepOutput>) -> Self {
        let completed = outputs.iter().filter(|o| o.success).count() as u32;
        let failed = outputs.iter().filter(|o| !o.success).count() as u32;

        Self {
            outputs,
            completed,
            failed,
            cancelled: false,
        }
    }

    pub fn cancelled(outputs: Vec<StepOutput>, completed: u32, failed: u32) -> Self {
        Self {
            outputs,
            completed,
            failed,
            cancelled: true,
        }
    }
}

/// Parallel execution error
#[derive(Debug, Error)]
pub enum ParallelError {
    #[error("No steps in parallel group")]
    EmptyGroup,

    #[error("Invalid concurrency limit: {0}")]
    InvalidConcurrency(usize),

    #[error("Step execution error: {0}")]
    StepError(String),

    #[error("Group cancelled due to fail_fast")]
    GroupCancelled,
}

/// Parallel executor
pub struct ParallelExecutor {
    step_executor: Box<dyn StepExecutor>,
    semaphore: Option<Semaphore>,
}

impl ParallelExecutor {
    pub fn new(step_executor: Box<dyn StepExecutor>) -> Self {
        Self {
            step_executor,
            semaphore: None,
        }
    }

    /// Execute parallel group
    pub async fn execute_parallel(
        &self,
        group: &ParallelGroup,
        context: &ExecutionContext,
    ) -> Result<ParallelResult, ParallelError> {
        if group.step_ids.is_empty() {
            return Err(ParallelError::EmptyGroup);
        }

        let max_concurrency = if group.max_concurrency == 0 {
            group.step_ids.len()
        } else {
            group.max_concurrency
        };

        if max_concurrency > group.step_ids.len() {
            return Err(ParallelError::InvalidConcurrency(max_concurrency));
        }

        // Create semaphore for concurrency control
        let semaphore = Arc::new(Semaphore::new(max_concurrency));

        // Spawn tasks for each step
        let mut tasks = Vec::new();
        let step_executor = &self.step_executor;

        for step_id in &group.step_ids {
            let semaphore = Arc::clone(&semaphore);
            let context = context.clone();
            let step_id = step_id.clone();

            let task = tokio::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore.acquire().await.unwrap();

                // TODO: Execute step with step_id
                // This will be implemented when we have the full IR structure
                StepOutput::success(serde_json::json!({"step_id": step_id}))
            });

            tasks.push(task);
        }

        // Collect results
        let mut outputs = Vec::new();
        let mut failed = 0u32;

        for task in tasks {
            match task.await {
                Ok(output) => {
                    if !output.success {
                        failed += 1;

                        // Check fail_fast
                        if group.fail_fast {
                            // Cancel remaining tasks
                            return Ok(ParallelResult::cancelled(outputs, (outputs.len() as u32) - failed, failed));
                        }
                    }
                    outputs.push(output);
                }
                Err(e) => {
                    failed += 1;
                    outputs.push(StepOutput::failure(format!("Task panicked: {}", e)));

                    if group.fail_fast {
                        return Ok(ParallelResult::cancelled(outputs, (outputs.len() as u32) - failed, failed));
                    }
                }
            }
        }

        Ok(ParallelResult::success(outputs))
    }

    /// Set semaphore for external concurrency control
    pub fn set_semaphore(&mut self, semaphore: Semaphore) {
        self.semaphore = Some(semaphore);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStepExecutor;

    #[async_trait::async_trait]
    impl StepExecutor for MockStepExecutor {
        async fn execute_step(&self, _step: &crate::ir::Step, _context: &ExecutionContext) -> Result<StepOutput, String> {
            Ok(StepOutput::success(serde_json::json!({})))
        }
    }

    #[tokio::test]
    async fn test_parallel_executor_empty_group() {
        let executor = ParallelExecutor::new(Box::new(MockStepExecutor));
        let group = ParallelGroup {
            group_id: "test".to_string(),
            step_ids: vec![],
            max_concurrency: 2,
            fail_fast: false,
            resource_constraints: None,
        };

        let context = ExecutionContext::new("job-1".to_string(), "session-1".to_string());

        let result = executor.execute_parallel(&group, &context).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParallelError::EmptyGroup));
    }

    #[tokio::test]
    async fn test_parallel_executor_invalid_concurrency() {
        let executor = ParallelExecutor::new(Box::new(MockStepExecutor));
        let group = ParallelGroup {
            group_id: "test".to_string(),
            step_ids: vec!["step-1".to_string()],
            max_concurrency: 10,
            fail_fast: false,
            resource_constraints: None,
        };

        let context = ExecutionContext::new("job-1".to_string(), "session-1".to_string());

        let result = executor.execute_parallel(&group, &context).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParallelError::InvalidConcurrency(10)));
    }
}
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement parallel.rs**

- [ ] **Step 3: Add to control_flow/mod.rs**

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**
