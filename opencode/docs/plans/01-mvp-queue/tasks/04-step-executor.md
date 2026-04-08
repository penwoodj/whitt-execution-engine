# Task 04: Step Executor

**Goal:** Implement step executor that executes workflow steps: agent steps (LLM calls via trait), tool steps, code steps, and sub-workflow steps.

**Files:**
- Create: `src/executor/mod.rs`
- Create: `src/executor/step.rs`
- Create: `src/executor/agent.rs`
- Create: `src/executor/tool.rs`
- Create: `src/executor/code.rs`
- Create: `src/executor/workflow.rs`
- Create: `tests/unit/executor_test.rs`

---

## Rust Definitions

### `src/executor/step.rs`

```rust
use crate::ir::Step as IRStep;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Execution context for a step
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub job_id: String,
    pub session_id: String,
    pub inputs: HashMap<String, serde_json::Value>,
    pub outputs: HashMap<String, serde_json::Value>,
    pub step_variables: HashMap<String, serde_json::Value>,
}

impl ExecutionContext {
    pub fn new(job_id: String, session_id: String) -> Self {
        Self {
            job_id,
            session_id,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            step_variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&serde_json::Value> {
        self.step_variables.get(name)
            .or_else(|| self.outputs.get(name))
            .or_else(|| self.inputs.get(name))
    }

    pub fn set_variable(&mut self, name: String, value: serde_json::Value) {
        self.step_variables.insert(name, value);
    }
}

/// Step execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepOutput {
    pub success: bool,
    pub data: serde_json::Value,
    pub logs: Vec<String>,
    pub error: Option<String>,
}

impl StepOutput {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            logs: vec![],
            error: None,
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            data: serde_json::json!({}),
            logs: vec![],
            error: Some(error),
        }
    }
}

/// Step executor trait
#[async_trait]
pub trait StepExecutor: Send + Sync {
    async fn execute_step(&self, step: &IRStep, context: &ExecutionContext) -> Result<StepOutput, String>;
}
```

### `src/executor/agent.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::ir::Step;
use async_trait::async_trait;

/// Trait for LLM backend (Phase 2 will implement this)
#[async_trait]
pub trait LLMBroker: Send + Sync {
    async fn call_llm(&self, prompt: String, context: serde_json::Value) -> Result<serde_json::Value, String>;
}

/// Agent step executor
pub struct AgentStepExecutor {
    llm_broker: Box<dyn LLMBroker>,
}

impl AgentStepExecutor {
    pub fn new(llm_broker: Box<dyn LLMBroker>) -> Self {
        Self { llm_broker }
    }
}

#[async_trait]
impl StepExecutor for AgentStepExecutor {
    async fn execute_step(&self, step: &Step, context: &ExecutionContext) -> Result<StepOutput, String> {
        if step.step_type != "agent" {
            return Err("Not an agent step".to_string());
        }

        // Extract prompt from step inputs
        let prompt = step
            .inputs
            .get("prompt")
            .and_then(|v| v.as_str())
            .ok_or("Missing prompt input")?
            .to_string();

        // Build LLM context
        let llm_context = serde_json::json!({
            "session_id": context.session_id,
            "inputs": context.inputs,
            "variables": context.step_variables,
        });

        // Call LLM (placeholder - Phase 2 will implement actual LLM backend)
        let result = self.llm_broker.call_llm(prompt, llm_context).await?;

        Ok(StepOutput::success(result))
    }
}

/// Mock LLM broker for testing
pub struct MockLLMBroker;

#[async_trait]
impl LLMBroker for MockLLMBroker {
    async fn call_llm(&self, _prompt: String, _context: serde_json::Value) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "response": "Mock LLM response"
        }))
    }
}
```

### `src/executor/tool.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::ir::Step;
use async_trait::async_trait;

/// Tool step executor
pub struct ToolStepExecutor;

#[async_trait]
impl StepExecutor for ToolStepExecutor {
    async fn execute_step(&self, step: &Step, context: &ExecutionContext) -> Result<StepOutput, String> {
        if step.step_type != "tool" {
            return Err("Not a tool step".to_string());
        }

        let tool_name = step
            .inputs
            .get("tool")
            .and_then(|v| v.as_str())
            .ok_or("Missing tool name")?;

        match tool_name {
            "echo" => {
                let message = step
                    .inputs
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                Ok(StepOutput::success(serde_json::json!({
                    "output": message
                })))
            }
            "read_file" => {
                let path = step
                    .inputs
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing path")?;

                let content = std::fs::read_to_string(path)
                    .map_err(|e| format!("Failed to read file: {}", e))?;

                Ok(StepOutput::success(serde_json::json!({
                    "content": content
                })))
            }
            "write_file" => {
                let path = step
                    .inputs
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing path")?;

                let content = step
                    .inputs
                    .get("content")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing content")?;

                std::fs::write(path, content)
                    .map_err(|e| format!("Failed to write file: {}", e))?;

                Ok(StepOutput::success(serde_json::json!({
                    "written": path
                })))
            }
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }
}
```

### `src/executor/code.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::ir::Step;
use async_trait::async_trait;

/// Code step executor
pub struct CodeStepExecutor;

#[async_trait]
impl StepExecutor for CodeStepExecutor {
    async fn execute_step(&self, step: &Step, _context: &ExecutionContext) -> Result<StepOutput, String> {
        if step.step_type != "code" {
            return Err("Not a code step".to_string());
        }

        let language = step
            .inputs
            .get("language")
            .and_then(|v| v.as_str())
            .ok_or("Missing language")?;

        let code = step
            .inputs
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or("Missing code")?;

        match language {
            "python" => {
                // Execute Python code (simplified for MVP)
                // In production, use a sandboxed execution environment
                tracing::info!("Executing Python code: {}", code);

                Ok(StepOutput::success(serde_json::json!({
                    "executed": true,
                    "language": "python"
                })))
            }
            "bash" => {
                // Execute bash command (simplified for MVP)
                // In production, use proper sandboxing
                tracing::info!("Executing bash: {}", code);

                Ok(StepOutput::success(serde_json::json!({
                    "executed": true,
                    "language": "bash"
                })))
            }
            _ => Err(format!("Unsupported language: {}", language)),
        }
    }
}
```

### `src/executor/workflow.rs`

```rust
use crate::executor::step::{ExecutionContext, StepExecutor, StepOutput};
use crate::ir::Step;
use async_trait::async_trait;

/// Sub-workflow step executor
pub struct WorkflowStepExecutor {
    step_executor: Box<dyn StepExecutor>,
}

impl WorkflowStepExecutor {
    pub fn new(step_executor: Box<dyn StepExecutor>) -> Self {
        Self { step_executor }
    }
}

#[async_trait]
impl StepExecutor for WorkflowStepExecutor {
    async fn execute_step(&self, step: &Step, context: &ExecutionContext) -> Result<StepOutput, String> {
        if step.step_type != "workflow" {
            return Err("Not a workflow step".to_string());
        }

        let workflow_id = step
            .inputs
            .get("workflow")
            .and_then(|v| v.as_str())
            .ok_or("Missing workflow ID")?;

        // Load sub-workflow (placeholder - implement in Task 05-07)
        tracing::info!("Executing sub-workflow: {}", workflow_id);

        Ok(StepOutput::success(serde_json::json!({
            "workflow_id": workflow_id,
            "completed": true
        })))
    }
}
```

### `src/executor/mod.rs`

```rust
pub mod agent;
pub mod code;
pub mod step;
pub mod tool;
pub mod workflow;

pub use agent::{AgentStepExecutor, LLMBroker, MockLLMBroker};
pub use code::CodeStepExecutor;
pub use step::{ExecutionContext, StepExecutor, StepOutput};
pub use tool::ToolStepExecutor;
pub use workflow::WorkflowStepExecutor;
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

```rust
// tests/unit/executor_test.rs
use agentsdk::executor::{AgentStepExecutor, ExecutionContext, MockLLMBroker, StepExecutor, StepOutput, ToolStepExecutor};
use agentsdk::ir::Step;
use std::collections::HashMap;

#[tokio::test]
async fn test_tool_executor_echo() {
    let executor = ToolStepExecutor;
    let context = ExecutionContext::new("job-1".to_string(), "session-1".to_string());

    let mut step = Step::new("tool".to_string(), "step-1".to_string());
    step.inputs.insert("tool".to_string(), serde_json::json!("echo"));
    step.inputs.insert("message".to_string(), serde_json::json!("Hello, World!"));

    let result = executor.execute_step(&step, &context).await.unwrap();

    assert!(result.success);
    assert_eq!(result.data["output"], "Hello, World!");
}

#[tokio::test]
async fn test_agent_executor_mock() {
    let llm_broker = Box::new(MockLLMBroker);
    let executor = AgentStepExecutor::new(llm_broker);
    let context = ExecutionContext::new("job-1".to_string(), "session-1".to_string());

    let mut step = Step::new("agent".to_string(), "step-1".to_string());
    step.inputs.insert("prompt".to_string(), serde_json::json!("Test prompt"));

    let result = executor.execute_step(&step, &context).await.unwrap();

    assert!(result.success);
    assert_eq!(result.data["response"], "Mock LLM response");
}
```

- [ ] **Step 2: Run tests (will fail)**

Run: `cargo test executor --lib`
Expected: FAIL (modules don't exist)

- [ ] **Step 3: Implement step.rs**

- [ ] **Step 4: Implement agent.rs**

- [ ] **Step 5: Implement tool.rs**

- [ ] **Step 6: Implement code.rs**

- [ ] **Step 7: Implement workflow.rs**

- [ ] **Step 8: Implement mod.rs**

- [ ] **Step 9: Add executor to lib.rs**

- [ ] **Step 10: Run tests (should pass)**

- [ ] **Step 11: Commit**

---

## Mock Strategy

Use `MockLLMBroker` for agent step tests.
For tool steps, use in-memory file operations or mock file system.
For code steps, use sandboxed execution environment.
