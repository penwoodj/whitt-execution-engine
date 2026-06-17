<!--
Source Session: ses_237b25c23ffenPrlbYu6fi64en
Part ID: prt_dc84da3df002upqhhxQ4g5t7Hl
Character Count: 11189
Extracted: 2026-06-17T07:50:39Z
-->

TASK: Implement the Agent ReAct Layer for a Rust workflow engine. Create 7 files.

## CONTEXT
Rust project (whitt-execution-engine) using serde, tokio, async-trait, thiserror, anyhow, tracing. The agent layer implements a ReAct (Reason-Act) loop with tools, retry, streaming, and persistence.

Working directory: /home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/playful-star

## YAML SCHEMA (agentic_workflow section, lines 196-497)
The agentic_workflow section defines:
- inputs (hardcoded_values, user_inputs)
- retry (step-level with backoff: exponential/linear/fixed, initial_delay, max_delay, multiplier)
- when (default hooks: before_step, after_step, on_error with then clauses)
- steps: agent steps (generative_entity + prompt + model), tool steps, control flow

## FILES TO CREATE

### 1. src/agent/mod.rs
```rust
pub mod tools;
pub mod react;
pub mod executor;
pub mod streaming;
pub mod persistence;
pub mod sandbox;
pub use tools::{Tool, ToolCall, ToolResult, ToolRegistry};
pub use react::ReactAgent;
pub use executor::StepExecutor;
pub use streaming::StreamingResponse;
pub use persistence::WorkflowPersistence;
pub use sandbox::ToolSandbox;
```

### 2. src/agent/tools.rs
Define 6 tools for the ReAct agent:

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub output: String,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}
```

ToolRegistry methods:
- `new() -> Self`
- `register(&mut self, tool: Box<dyn Tool>)`
- `get(&self, name: &str) -> Option<&dyn Tool>`
- `list(&self) -> Vec<(&str, &str)>` — (name, description) pairs
- `execute(&self, call: ToolCall) -> Result<ToolResult, anyhow::Error>`

Implement 6 concrete tools (each as a struct implementing Tool):
1. **ModelListTool** — returns list of available models (holds ref to ModelRegistry)
   - name: "model_list", no required args
   - returns JSON array of model names with their lifecycle state
2. **ModelLoadTool** — loads a model by name
   - name: "model_load", args: model_name (String)
   - calls backend.load_model(), updates registry state to Active
3. **ModelUnloadTool** — unloads a model by name
   - name: "model_unload", args: model_name (String)
   - calls backend.unload_model(), updates registry state to Unloaded
4. **ChatTool** — sends chat message to loaded model
   - name: "chat", args: model_name (String), message (String), temperature (Option<f32>)
   - calls backend.chat(), returns response content
5. **FileReadTool** — reads a file from workspace
   - name: "file_read", args: path (String)
   - reads file content, enforces allowed_paths restrictions
   - Returns ToolResult with file content or error
6. **FinalAnswerTool** — signals completion of ReAct loop
   - name: "final_answer", args: answer (String)
   - returns the final answer, sets a flag that breaks the loop

IMPORTANT: The tools that reference ModelRegistry or LlmBackend should accept them via constructor parameters. Use `Arc<Mutex<>>` or `Arc<RwLock<>>` for shared state.

### 3. src/agent/react.rs
ReAct agent with tool loop:

```rust
use crate::agent::tools::ToolRegistry;
use crate::backend::llm_backend::{LlmBackend, ChatMessage, ChatResponse, LlmError};
use std::sync::Arc;

pub struct ReactAgent {
    backend: Arc<dyn LlmBackend>,
    tools: Arc<ToolRegistry>,
    model: String,
    max_iterations: usize,
    system_prompt: String,
}

pub struct AgentResponse {
    pub final_answer: String,
    pub tool_calls: Vec<crate::agent::tools::ToolCall>,
    pub tool_results: Vec<crate::agent::tools::ToolResult>,
    pub iterations: usize,
    pub total_tokens: u64,
}
```

Methods:
- `new(backend: Arc<dyn LlmBackend>, tools: Arc<ToolRegistry>, model: String) -> Self`
- `with_system_prompt(mut self, prompt: String) -> Self`
- `with_max_iterations(mut self, max: usize) -> Self`
- `async run(&self, user_message: String) -> Result<AgentResponse, anyhow::Error>` — main ReAct loop:
  1. Build messages: system prompt + tool definitions + user message
  2. Send to LLM via backend.chat()
  3. Parse response for tool calls (JSON format: {"tool": "name", "arguments": {...}})
  4. If tool call found: execute via ToolRegistry, append result to messages, loop
  5. If FinalAnswerTool called: return AgentResponse
  6. If max_iterations reached: return best effort
  7. Use tracing::debug! for every step

### 4. src/agent/executor.rs
Step executor with retry logic:

```rust
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    Exponential { multiplier: f64 },
    Linear { increment_secs: u64 },
    Fixed,
}

pub struct RetryConfig {
    pub max_retries: u32,
    pub backoff: BackoffStrategy,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

pub struct StepExecutor {
    retry_config: RetryConfig,
}
```

Methods:
- `new(config: RetryConfig) -> Self`
- `async execute_step<F, T>(&self, step_fn: F) -> Result<T, anyhow::Error>` where F: Fn() -> Future<Output = Result<T, anyhow::Error>>
  — execute step_fn with retry logic:
  1. Try step_fn
  2. If fails and retries remaining: calculate delay using backoff strategy
  3. Exponential: delay * multiplier^attempt + jitter (fastrand)
  4. Linear: initial + increment * attempt + jitter
  5. Fixed: initial_delay + jitter
  6. tokio::time::sleep(delay)
  7. Retry up to max_retries
  8. tracing::debug! for each retry attempt
- `parse_backoff_strategy(strategy: &str, multiplier: Option<f64>) -> BackoffStrategy`
- `parse_duration(s: &str) -> Duration` — parse "1s", "500ms", "2m" strings

### 5. src/agent/streaming.rs
SSE streaming response handler:

```rust
use futures::Stream;
use std::pin::Pin;

pub struct StreamingResponse {
    model: String,
    content: String,
    is_complete: bool,
}

pub type SSEStream = Pin<Box<dyn Stream<Item = Result<StreamEvent, anyhow::Error>> + Send>>;

#[derive(Debug, Clone)]
pub enum StreamEvent {
    Token(String),
    ToolCallStart { name: String },
    ToolCallEnd { name: String, result: String },
    Complete { total_tokens: u64 },
    Error(String),
}

impl StreamingResponse {
    pub fn new(model: String) -> Self
    pub fn process_sse_line(&mut self, line: &str) -> Option<StreamEvent>
    pub fn content(&self) -> &str
    pub fn is_complete(&self) -> bool
}

pub fn parse_sse_stream(raw: &str) -> Vec<StreamEvent> — parse raw SSE text into events
```

Gate SSE-specific code behind `#[cfg(feature = "client")]`. For non-client builds, provide a simple parse_sse_line function that works with string input.

### 6. src/agent/persistence.rs
Workflow state persistence using a simple JSON-based store (since treadle may not have the exact API we need):

```rust
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub variables: HashMap<String, serde_json::Value>,
    pub checkpoints: Vec<Checkpoint>,
    pub status: WorkflowStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub step_name: String,
    pub timestamp: String,
    pub state: HashMap<String, serde_json::Value>,
}

pub struct WorkflowPersistence {
    storage_path: PathBuf,
}
```

Methods:
- `new(storage_path: PathBuf) -> Self`
- `save(&self, state: &WorkflowState) -> Result<(), anyhow::Error>` — serialize to JSON, write to `{storage_path}/{workflow_id}.json`
- `load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error>` — read and deserialize
- `create_checkpoint(&self, state: &mut WorkflowState, step_name: &str) -> Result<(), anyhow::Error>` — snapshot current state
- `list_workflows(&self) -> Result<Vec<String>, anyhow::Error>` — list workflow IDs in storage
- `delete(&self, workflow_id: &str) -> Result<(), anyhow::Error>` — remove workflow file

### 7. src/agent/sandbox.rs
Simple sandbox for tool execution (sandlock crate doesn't exist, so implement a path-based sandbox):

```rust
use std::path::{Path, PathBuf};

pub struct ToolSandbox {
    allowed_paths: Vec<PathBuf>,
    forbidden_paths: Vec<PathBuf>,
    allowed_patterns: Vec<String>,
    max_file_size_bytes: u64,
}
```

Methods:
- `new(config: SandboxConfig) -> Self` where SandboxConfig has allowed_paths, forbidden_paths, etc.
- `is_path_allowed(&self, path: &Path) -> Result<bool, anyhow::Error>` — check against allowed/forbidden lists
- `validate_file_access(&self, path: &Path, operation: FileOperation) -> Result<(), anyhow::Error>` — validate with operation type
- `sanitize_path(&self, path: &Path) -> Result<PathBuf, anyhow::Error>` — canonicalize and validate path

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileOperation { Read, Write, Delete }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub allowed_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub allowed_patterns: Vec<String>,
    pub max_file_size_mb: u64,
}
```

## MANDATORY REQUIREMENTS
1. Every struct derives Debug, Clone, Serialize, Deserialize where applicable
2. Use async_trait for async trait methods
3. Use tracing::debug! extensively for every operation
4. Use Arc for shared state across tools/agent
5. No `unwrap()` in non-test code — use `?` or explicit error handling
6. No `todo!()` macros — fully implement everything
7. Tool implementations must be complete, not stubs
8. ReAct loop must handle: tool calls, max iterations, errors, final answer
9. Retry logic must use fastrand for jitter
10. Persistence must use JSON file storage (not treadle directly)
11. Sandbox must be a real path-based validator, not a stub

## MUST NOT DO
1. Do NOT modify any existing files
2. Do NOT add dependencies to Cargo.toml
3. Do NOT reference types from other new modules that haven't been created yet (use placeholder imports if needed, mark with TODO comment)
4. Do NOT create test files
5. Do NOT use rig-core — implement the ReAct loop directly

## EXPECTED OUTCOME
7 files:
- src/agent/mod.rs (~12 lines)
- src/agent/tools.rs (~250 lines)
- src/agent/react.rs (~200 lines)
- src/agent/executor.rs (~120 lines)
- src/agent/streaming.rs (~120 lines)
- src/agent/persistence.rs (~130 lines)
- src/agent/sandbox.rs (~80 lines)

After creating all files, run `cargo check --all-features` to verify. Fix any compilation errors.
<!-- OMO_INTERNAL_INITIATOR -->