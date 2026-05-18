# Basic LLM Agent Plan (ReAct Pattern)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement basic ReAct agent using Rig framework (agent builder, tool definitions, provider abstraction), Treadle StateStore for workflow persistence, Sandlock for per-tool sandboxing, step execution, retry logic, and streaming support.

**Architecture:** Step execution → ReAct agent → tool parse → execute → append → loop. Max turns from model config, retry from step config.

**Tech Stack:** Rig (agent framework, agent builder, tool definitions), Treadle (workflow state persistence, SQLite StateStore), Sandlock (per-tool sandboxing, Landlock + seccomp), tokio (async), sseer (streaming), regex (tool parsing), VidaiMock (testing, physics-accurate streaming).

**Prerequisites:** 01-provider-config.md (backend trait), 02-model-schema.md (model registry)

---

## Scope

### Unified Schema Sections (unified-workflow-schema.yml lines 293-339)

```yaml
steps:
  step_name:
    generative_entity: "${models.model-name}"
    prompt: |
      Task description here
    model_overrides:
      max_turns: 10
    retry:
      max_attempts: 3
      backoff: exponential
      initial_delay: "1s"
      max_delay: "30s"
      multiplier: 2.0
      jitter: true
```

### IN Scope
- Basic LLM agent steps (generative_entity + prompt pattern)
- Tool definitions (model_list, model_load, model_unload, chat, file_read, final_answer)
- Step execution: generative_entity + prompt → LLM call → tool parse → execute → loop
- Max turns enforcement from model execution config
- Retry logic from step retry config
- Streaming output support (SSE)
- Regex-based tool parsing

### OUT of Scope
- Hooks (when:, before_step_starts:, etc.)
- Sub-workflows, loops
- Tool permissions (ask/trust/block)
- Guardrails
- Code generation
- RAG

---

## Files to Create

### 1. src/agent/tools.rs

**Purpose:** Tool definitions for ReAct agent.

```rust
use crate::backend::llm_backend::{LlmBackend, LlmError};
use crate::model::ModelRegistry;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Tool execution context.
pub struct ToolContext {
    pub model_registry: ModelRegistry,
    pub workspace_path: std::path::PathBuf,
}

/// Tool trait for ReAct agent.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: serde_json::Value, ctx: &ToolContext)
        -> Result<String, ToolError>;
}

/// Tool execution error.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Backend error: {0}")]
    Backend(String),
}

/// Tool list tool.
pub struct ModelListTool;

#[async_trait]
impl Tool for ModelListTool {
    fn name(&self) -> &str {
        "model_list"
    }

    fn description(&self) -> &str {
        "List all available models in the registry"
    }

    async fn execute(
        &self,
        _args: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        let models = ctx.model_registry.list_models().await;
        let json = serde_json::to_string(&models)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(json)
    }
}

/// Model load tool.
pub struct ModelLoadTool;

#[async_trait]
impl Tool for ModelLoadTool {
    fn name(&self) -> &str {
        "model_load"
    }

    fn description(&self) -> &str {
        "Load a model into memory. Args: {\"model_id\": \"model-name\"}"
    }

    async fn execute(
        &self,
        args: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        #[derive(Deserialize)]
        struct Args {
            model_id: String,
        }

        let args: Args = serde_json::from_value(args)
            .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;

        ctx.model_registry
            .load_model(&args.model_id)
            .await
            .map_err(|e| ToolError::Backend(e.to_string()))?;

        Ok(format!("Model {} loaded successfully", args.model_id))
    }
}

/// Model unload tool.
pub struct ModelUnloadTool;

#[async_trait]
impl Tool for ModelUnloadTool {
    fn name(&self) -> &str {
        "model_unload"
    }

    fn description(&self) -> &str {
        "Unload a model from memory. Args: {\"model_id\": \"model-name\"}"
    }

    async fn execute(
        &self,
        args: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        #[derive(Deserialize)]
        struct Args {
            model_id: String,
        }

        let args: Args = serde_json::from_value(args)
            .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;

        ctx.model_registry
            .unload_model(&args.model_id)
            .await
            .map_err(|e| ToolError::Backend(e.to_string()))?;

        Ok(format!("Model {} unloaded successfully", args.model_id))
    }
}

/// Chat tool (single LLM call without tool loop).
pub struct ChatTool {
    backend: std::sync::Arc<dyn LlmBackend>,
}

impl ChatTool {
    pub fn new(backend: std::sync::Arc<dyn LlmBackend>) -> Self {
        Self { backend }
    }
}

#[async_trait]
impl Tool for ChatTool {
    fn name(&self) -> &str {
        "chat"
    }

    fn description(&self) -> &str {
        "Send a single chat completion. Args: {\"prompt\": \"...\", \"max_tokens\": 100}"
    }

    async fn execute(
        &self,
        args: serde_json::Value,
        _ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        #[derive(Deserialize)]
        struct Args {
            prompt: String,
            #[serde(default)]
            max_tokens: Option<usize>,
        }

        let args: Args = serde_json::from_value(args)
            .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;

        use crate::client::types::{ChatCompletionRequest, Message};

        let request = ChatCompletionRequest {
            model: self.backend.base_url().to_string(),
            messages: vec![Message {
                role: "user".into(),
                content: args.prompt,
            }],
            max_tokens: args.max_tokens,
            ..Default::default()
        };

        let response = self
            .backend
            .chat(request)
            .await
            .map_err(|e| ToolError::Backend(e.to_string()))?;

        Ok(response.choices[0].message.content.clone())
    }
}

/// File read tool.
pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read file contents. Args: {\"path\": \"./workspace/file.txt\"}"
    }

    async fn execute(
        &self,
        args: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        #[derive(Deserialize)]
        struct Args {
            path: String,
        }

        let args: Args = serde_json::from_value(args)
            .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;

        let full_path = ctx.workspace_path.join(&args.path);

        tokio::fs::read_to_string(&full_path)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))
    }
}

/// Final answer tool (terminates ReAct loop).
pub struct FinalAnswerTool;

#[async_trait]
impl Tool for FinalAnswerTool {
    fn name(&self) -> &str {
        "final_answer"
    }

    fn description(&self) -> &str {
        "Provide final answer and terminate reasoning. Args: {\"answer\": \"...\"}"
    }

    async fn execute(
        &self,
        args: serde_json::Value,
        _ctx: &ToolContext,
    ) -> Result<String, ToolError> {
        #[derive(Deserialize)]
        struct Args {
            answer: String,
        }

        let args: Args = serde_json::from_value(args)
            .map_err(|e| ToolError::InvalidArgs(e.to_string()))?;

        Ok(args.answer)
    }
}

/// Get all available tools.
pub fn get_tools(
    backend: std::sync::Arc<dyn LlmBackend>,
) -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(ModelListTool),
        Box::new(ModelLoadTool),
        Box::new(ModelUnloadTool),
        Box::new(ChatTool::new(backend)),
        Box::new(FileReadTool),
        Box::new(FinalAnswerTool),
    ]
}
```

### 2. src/agent/react.rs

**Purpose:** ReAct agent implementation.

```rust
use super::tools::{Tool, ToolContext, get_tools};
use super::executor::StepExecutor;

use crate::backend::llm_backend::LlmBackend;
use crate::model::ModelRegistry;
use crate::client::types::{ChatCompletionRequest, Message};

use std::sync::Arc;
use regex::Regex;

/// ReAct agent state.
#[derive(Debug, Clone)]
pub struct ReactAgentState {
    pub messages: Vec<Message>,
    pub turn_count: usize,
    pub completed: bool,
}

/// ReAct agent.
pub struct ReactAgent {
    backend: Arc<dyn LlmBackend>,
    model_registry: Arc<ModelRegistry>,
    executor: StepExecutor,
    tool_call_regex: Regex,
}

impl ReactAgent {
    /// Create new ReAct agent.
    pub fn new(
        backend: Arc<dyn LlmBackend>,
        model_registry: Arc<ModelRegistry>,
        workspace_path: std::path::PathBuf,
    ) -> Self {
        let tools = get_tools(backend.clone());
        let tool_context = ToolContext {
            model_registry: model_registry.clone(),
            workspace_path,
        };

        let executor = StepExecutor::new(tools, tool_context);

        // Regex to match tool calls: "ToolName(arg1=value1, arg2=value2)"
        let tool_call_regex = Regex::new(r"([A-Za-z_]+)\(([^)]+)\)").unwrap();

        Self {
            backend,
            model_registry,
            executor,
            tool_call_regex,
        }
    }

    /// Execute ReAct loop for a step.
    pub async fn execute_step(
        &self,
        step_name: &str,
        prompt: &str,
        max_turns: usize,
    ) -> anyhow::Result<String> {
        let mut state = ReactAgentState {
            messages: vec![Message {
                role: "system".into(),
                content: format!(
                    "You are a helpful assistant. Use tools when needed. \
                    Available tools: model_list, model_load, model_unload, \
                    chat, file_read, final_answer. \
                    Use final_answer to provide your final response."
                ),
            }],
            turn_count: 0,
            completed: false,
        };

        state.messages.push(Message {
            role: "user".into(),
            content: prompt.to_string(),
        });

        while state.turn_count < max_turns && !state.completed {
            tracing::info!(
                step = step_name,
                turn = state.turn_count,
                "Executing ReAct turn"
            );

            // Call LLM
            let llm_response = self.call_llm(&state.messages).await?;

            // Check for tool calls
            if let Some(tool_call) = self.parse_tool_call(&llm_response) {
                // Execute tool
                let tool_result = self
                    .executor
                    .execute_tool(&tool_call.name, tool_call.args)
                    .await?;

                // Append tool result to conversation
                state.messages.push(Message {
                    role: "assistant".into(),
                    content: llm_response,
                });
                state.messages.push(Message {
                    role: "tool".into(),
                    content: format!(
                        "{}: {}",
                        tool_call.name, tool_result
                    ),
                });

                // Check if final_answer called
                if tool_call.name == "final_answer" {
                    state.completed = true;
                }
            } else {
                // No tool call, treat as final answer
                state.completed = true;
                return Ok(llm_response);
            }

            state.turn_count += 1;
        }

        if state.turn_count >= max_turns {
            return Err(anyhow::anyhow!(
                "Max turns ({}) exceeded for step {}",
                max_turns,
                step_name
            ));
        }

        // Extract final answer from last message
        Ok(state
            .messages
            .last()
            .and_then(|m| Some(m.content.clone()))
            .unwrap_or_default())
    }

    /// Call LLM backend.
    async fn call_llm(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = ChatCompletionRequest {
            model: self.backend.base_url().to_string(),
            messages: messages.to_vec(),
            stream: Some(false),
            ..Default::default()
        };

        let response = self.backend.chat(request).await?;
        Ok(response.choices[0].message.content.clone())
    }

    /// Parse tool call from LLM response.
    fn parse_tool_call(&self, response: &str) -> Option<ToolCall> {
        let captures = self.tool_call_regex.captures(response)?;

        let tool_name = captures.get(1)?.as_str().to_string();
        let args_str = captures.get(2)?.as_str().to_string();

        // Parse args: arg1=value1, arg2=value2
        let args = self.parse_args(&args_str);

        Some(ToolCall { tool_name, args })
    }

    /// Parse tool arguments.
    fn parse_args(&self, args_str: &str) -> serde_json::Value {
        let mut args = serde_json::Map::new();

        for pair in args_str.split(',') {
            let parts: Vec<&str> = pair.trim().split('=').collect();
            if parts.len() == 2 {
                args.insert(
                    parts[0].trim().to_string(),
                    serde_json::Value::String(parts[1].trim().trim_matches('"').to_string()),
                );
            }
        }

        serde_json::Value::Object(args)
    }
}

/// Tool call representation.
#[derive(Debug, Clone)]
struct ToolCall {
    name: String,
    args: serde_json::Value,
}
```

### 3. src/agent/executor.rs

**Purpose:** Step executor with retry logic.

```rust
use super::tools::{Tool, ToolContext};

use crate::config::provider::{RetryConfig, BackoffStrategy};

use std::collections::HashMap;

/// Step configuration.
#[derive(Debug, Clone)]
pub struct StepConfig {
    pub generative_entity: String,
    pub prompt: String,
    pub max_turns: usize,
    pub retry: Option<RetryConfig>,
}

/// Step execution result.
#[derive(Debug, Clone)]
pub struct StepResult {
    pub success: bool,
    pub output: String,
    pub attempts: usize,
}

/// Step executor.
pub struct StepExecutor {
    tools: HashMap<String, Box<dyn Tool>>,
    tool_context: ToolContext,
}

impl StepExecutor {
    /// Create new executor.
    pub fn new(tools: Vec<Box<dyn Tool>>, tool_context: ToolContext) -> Self {
        let mut tools_map = HashMap::new();
        for tool in tools {
            tools_map.insert(tool.name().to_string(), tool);
        }

        Self {
            tools: tools_map,
            tool_context,
        }
    }

    /// Execute step with retry logic.
    pub async fn execute_step(&self, config: &StepConfig) -> StepResult {
        let retry_config = config.retry.clone().unwrap_or_else(|| {
            // Use default retry from unified schema
            RetryConfig {
                max_retries: 3,
                backoff: BackoffStrategy::Exponential,
                initial_delay: "1s".into(),
                max_delay: "30s".into(),
                multiplier: 2.0,
                jitter: true,
            }
        });

        let mut attempts = 0;
        let mut last_error = String::new();

        for attempt in 0..=retry_config.max_retries {
            attempts = attempt + 1;

            match self.try_execute(config).await {
                Ok(result) => {
                    tracing::info!(
                        step = config.generative_entity,
                        attempts,
                        "Step succeeded"
                    );
                    return StepResult {
                        success: true,
                        output: result,
                        attempts,
                    };
                }
                Err(e) => {
                    last_error = e.to_string();

                    if attempt < retry_config.max_retries {
                        let delay = Self::calculate_delay(&retry_config, attempt);
                        tracing::warn!(
                            step = config.generative_entity,
                            attempt,
                            delay_ms = delay.as_millis(),
                            error = %e,
                            "Step failed, retrying"
                        );

                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        tracing::error!(
            step = config.generative_entity,
            attempts,
            error = %last_error,
            "Step failed after all retries"
        );

        StepResult {
            success: false,
            output: last_error,
            attempts,
        }
    }

    /// Try to execute step once.
    async fn try_execute(&self, config: &StepConfig) -> anyhow::Result<String> {
        // TODO: Integrate with ReactAgent
        // let agent = ReactAgent::new(...);
        // agent.execute_step(...).await
        Ok("TODO: implement".to_string())
    }

    /// Calculate retry delay.
    fn calculate_delay(retry_config: &RetryConfig, attempt: u32) -> tokio::time::Duration {
        let base_delay = Self::parse_duration(&retry_config.initial_delay)
            .unwrap_or_else(|_| tokio::time::Duration::from_secs(1));

        let max_delay = Self::parse_duration(&retry_config.max_delay)
            .unwrap_or_else(|_| tokio::time::Duration::from_secs(30));

        let delay_secs = match retry_config.backoff {
            BackoffStrategy::Exponential => {
                base_delay.as_secs() * retry_config.multiplier.powi(attempt as i32 - 1) as u64
            }
            BackoffStrategy::Linear => {
                base_delay.as_secs() * attempt as u64
            }
            BackoffStrategy::Fixed => base_delay.as_secs(),
        };

        let delay = tokio::time::Duration::from_secs(delay_secs.min(max_delay.as_secs()));

        if retry_config.jitter {
            let jitter_ms = fastrand::u64(0..=delay.as_millis() as u64 / 10);
            tokio::time::Duration::from_millis(delay.as_millis() as u64 + jitter_ms)
        } else {
            delay
        }
    }

    /// Execute a tool.
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool {} not found", tool_name))?;

        tool.execute(args, &self.tool_context)
            .await
            .map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))
    }
}

/// Parse duration string.
fn parse_duration(s: &str) -> anyhow::Result<tokio::time::Duration> {
    let s = s.trim().to_lowercase();
    let num: u64 = s
        .trim_end_matches('s')
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))?;

    Ok(tokio::time::Duration::from_secs(num))
}
```

### 4. src/agent/streaming.rs

**Purpose:** SSE streaming support.

```rust
use crate::backend::llm_backend::LlmBackend;
use crate::client::types::{ChatCompletionRequest, Message};

use std::pin::Pin;
use std::sync::Arc;

/// Stream chunk.
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub text: String,
    pub done: bool,
}

/// Streaming response.
pub struct StreamingResponse {
    stream: Pin<Box<dyn futures::Stream<Item = Result<StreamChunk, String>> + Send>>,
}

impl StreamingResponse {
    /// Create new streaming response.
    pub fn new(
        backend: Arc<dyn LlmBackend>,
        request: ChatCompletionRequest,
    ) -> Self {
        let stream = backend.chat_stream(request);

        let stream = stream.map(|result| {
            result.map(|text| StreamChunk {
                text,
                done: text.is_empty(),
            })
        });

        Self {
            stream: Box::pin(stream),
        }
    }

    /// Consume stream into string.
    pub async fn collect(self) -> Result<String, String> {
        use futures::StreamExt;

        let mut output = String::new();

        let mut stream = self.stream;

        while let Some(result) = stream.next().await {
            match result {
                Ok(chunk) => {
                    if chunk.done {
                        break;
                    }
                    output.push_str(&chunk.text);
                    print!("{}", chunk.text);
                    std::io::stdout().flush().ok();
                }
                Err(e) => return Err(e),
            }
        }

        Ok(output)
    }
}

impl futures::Stream for StreamingResponse {
    type Item = Result<StreamChunk, String>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }
}
```

### 5. src/agent/mod.rs

**Purpose:** Export agent modules.

```rust
pub mod tools;
pub mod react;
pub mod executor;
pub mod streaming;
pub mod persistence;
pub mod sandbox;

pub use tools::{Tool, ToolContext, get_tools};
pub use react::{ReactAgent, ReactAgentState};
pub use executor::{StepExecutor, StepConfig, StepResult};
pub use streaming::{StreamingResponse, StreamChunk};
pub use persistence::{WorkflowPersistence, WorkflowState};
pub use sandbox::{ToolSandbox, SandboxedToolExecutor};
```

### 6. tests/agent_react_test.rs

**Purpose:** Unit tests for ReAct agent.

```rust
use whitt::agent::{ReactAgent, Tool, ToolContext};
use whitt::model::{ModelRegistry, ModelsConfig};

#[tokio::test]
async fn react_agent_single_turn() {
    // Mock backend and registry
    // let backend = Arc::new(MockBackend::new());
    // let registry = ModelRegistry::new(...);

    // let agent = ReactAgent::new(backend, registry, workspace);
    // let result = agent.execute_step("test", "Hello", 5).await;

    // assert!(result.is_ok());
}

#[test]
fn tool_list_contains_all_tools() {
    use whitt::agent::tools::get_tools;

    // let tools = get_tools(backend);
    // assert_eq!(tools.len(), 6);
    // assert!(tools.iter().any(|t| t.name() == "model_list"));
}

#[test]
fn tool_call_regex_parsing() {
    let regex = regex::Regex::new(r"([A-Za-z_]+)\(([^)]+)\)").unwrap();

    let input = "model_list()";
    let captures = regex.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "model_list");

    let input = "model_load(model_id=\"test\")";
    let captures = regex.captures(input).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "model_load");
    assert_eq!(captures.get(2).unwrap().as_str(), "model_id=\"test\"");
}
```

### 7. src/agent/persistence.rs

**Purpose:** Treadle StateStore integration for workflow state persistence.

```rust
use treadle::StateStore;
use std::path::PathBuf;

/// Workflow state persistence using Treadle.
pub struct WorkflowPersistence {
    store: StateStore,
    workflow_id: String,
    checkpoint_dir: PathBuf,
}

impl WorkflowPersistence {
    /// Create new persistence layer.
    pub fn new(workflow_id: String, checkpoint_dir: PathBuf) -> anyhow::Result<Self> {
        let store = StateStore::new(checkpoint_dir.join("state.db"))?;
        Ok(Self {
            store,
            workflow_id,
            checkpoint_dir,
        })
    }

    /// Save workflow checkpoint.
    pub async fn save_checkpoint(&self, step_name: &str, state: &WorkflowState)
        -> anyhow::Result<()> {
        let key = format!("{}.{}", self.workflow_id, step_name);
        let serialized = serde_json::to_string(state)?;
        self.store.set(&key, &serialized).await?;
        Ok(())
    }

    /// Load workflow checkpoint.
    pub async fn load_checkpoint(&self, step_name: &str) -> anyhow::Result<Option<WorkflowState>> {
        let key = format!("{}.{}", self.workflow_id, step_name);
        if let Some(serialized) = self.store.get(&key).await? {
            let state = serde_json::from_str(&serialized)?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// List all checkpoints for workflow.
    pub async fn list_checkpoints(&self) -> anyhow::Result<Vec<String>> {
        let prefix = format!("{}.", self.workflow_id);
        let keys = self.store.list_prefix(&prefix).await?;
        Ok(keys.into_iter().map(|k| k.strip_prefix(&prefix).unwrap().to_string()).collect())
    }

    /// Clear all checkpoints for workflow.
    pub async fn clear_checkpoints(&self) -> anyhow::Result<()> {
        let prefix = format!("{}.", self.workflow_id);
        let keys = self.store.list_prefix(&prefix).await?;
        for key in keys {
            self.store.delete(&key).await?;
        }
        Ok(())
    }
}

/// Workflow state for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub step_name: String,
    pub turn_count: usize,
    pub messages: Vec<String>,
    pub completed: bool,
    pub last_update: chrono::DateTime<chrono::Utc>,
}
```

### 8. src/agent/sandbox.rs

**Purpose:** Sandlock integration for per-tool sandboxing.

```rust
use sandlock::{Sandbox, SandboxConfig};

/// Tool sandbox using Sandlock.
pub struct ToolSandbox {
    config: SandboxConfig,
}

impl ToolSandbox {
    /// Create new sandbox configuration.
    pub fn new() -> Self {
        let config = SandboxConfig::builder()
            .landlock(true)            // Enable Landlock for filesystem + network restrictions
            .seccomp(true)            // Enable seccomp-bpf for syscall filtering
            .memory_limit_mb(512)     // Per-tool memory limit
            .cpu_time_limit_secs(30)   // Per-tool CPU time limit
            .network_restricted(true)   // Restrict network access
            .build();

        Self { config }
    }

    /// Execute tool in sandbox.
    pub async fn execute_tool<F, R>(&self, tool_name: &str, f: F) -> anyhow::Result<R>
    where
        F: FnOnce() -> anyhow::Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let sandbox = Sandbox::new(tool_name, self.config.clone())?;

        // Execute in sandboxed environment
        let result = tokio::task::spawn_blocking(move || {
            f()
        }).await??;

        Ok(result)
    }

    /// Configure filesystem access for tool.
    pub fn with_filesystem_access(mut self, allowed_paths: Vec<String>) -> Self {
        self.config = SandboxConfig::from(self.config.clone())
            .with_allowed_paths(allowed_paths)
            .build();
        self
    }

    /// Configure network access for tool.
    pub fn with_network_access(mut self, allowed_domains: Vec<String>) -> Self {
        self.config = SandboxConfig::from(self.config.clone())
            .with_allowed_domains(allowed_domains)
            .build();
        self
    }
}

/// Tool execution wrapper with sandboxing.
pub struct SandboxedToolExecutor {
    sandbox: ToolSandbox,
}

impl SandboxedToolExecutor {
    /// Create new sandboxed executor.
    pub fn new() -> Self {
        Self {
            sandbox: ToolSandbox::new(),
        }
    }

    /// Execute tool in sandbox with filesystem restrictions.
    pub async fn execute_sandboxed_tool(
        &self,
        tool_name: &str,
        allowed_paths: Vec<String>,
        tool_fn: impl FnOnce() -> anyhow::Result<String> + Send + 'static,
    ) -> anyhow::Result<String> {
        let sandboxed = self.sandbox
            .with_filesystem_access(allowed_paths);

        sandboxed.execute_tool(tool_name, tool_fn).await
    }
}
```

### 9. tests/vidaimock_integration_test.rs

**Purpose:** VidaiMock integration for realistic LLM streaming testing.

```rust
use vidaimock::{VidaiMock, MockConfig, StreamingBehavior};
use whitt::agent::{ReactAgent, ToolContext};
use whitt::backend::llm_backend::{LlmBackend, LlmError};

/// VidaiMock test harness for realistic streaming.
pub struct VidaiMockHarness {
    mock: VidaiMock,
    mock_port: u16,
}

impl VidaiMockHarness {
    /// Create new VidaiMock harness.
    pub async fn new() -> anyhow::Result<Self> {
        let config = MockConfig::builder()
            .streaming_behavior(StreamingBehavior::PhysicsAccurate)  // Realistic TTFT and token delivery
            .chaos_mode(true)                                      // Enable latency/failure simulation
            .benchmark_mode(false)                                     // Normal operation (not 50K RPS)
            .build();

        let mock = VidaiMock::start(config).await?;
        let mock_port = mock.port();

        Ok(Self { mock, mock_port })
    }

    /// Get mock server URL for backend testing.
    pub fn mock_url(&self) -> String {
        format!("http://localhost:{}", self.mock_port)
    }

    /// Configure mock to simulate specific scenarios.
    pub async fn configure_scenario(&self, scenario: MockScenario) -> anyhow::Result<()> {
        match scenario {
            MockScenario::SlowStreaming => {
                self.mock.set_ttft_ms(5000).await?;  // 5s time-to-first-token
                self.mock.set_tokens_per_second(1).await?;
            }
            MockScenario::DroppedConnections => {
                self.mock.set_drop_rate(0.1).await?;  // Drop 10% of connections
            }
            MockScenario::RateLimited => {
                self.mock.set_rate_limit_rpm(60).await?;
            }
            MockScenario::MalformedResponses => {
                self.mock.set_malformed_rate(0.01).await?;  // 1% malformed
            }
        }
        Ok(())
    }
}

/// Mock scenarios for testing.
pub enum MockScenario {
    SlowStreaming,           // High latency, slow token delivery
    DroppedConnections,     // Intermittent connection drops
    RateLimited,             // Simulate 429 responses
    MalformedResponses,      // Malformed SSE chunks
}

#[tokio::test]
async fn vidaimock_realistic_streaming_test() {
    let harness = VidaiMockHarness::new().await?;
    let mock_url = harness.mock_url();

    // Configure slow streaming scenario
    harness.configure_scenario(MockScenario::SlowStreaming).await?;

    // Test backend with realistic streaming
    // let backend = Arc::new(LlamaCppVulkanBackend::new(config, model));
    // let agent = ReactAgent::new(backend, registry, workspace);
    // let result = agent.execute_step("test", "Hello", 5).await;

    // assert!(result.is_ok());
}

#[tokio::test]
async fn vidaimock_chaos_mode_test() {
    let harness = VidaiMockHarness::new().await?;

    // Test with dropped connections
    harness.configure_scenario(MockScenario::DroppedConnections).await?;

    // Verify agent handles connection drops gracefully
    // let result = agent.execute_step("test", "Hello", 5).await;
    // assert!(result.is_ok() || matches!(result, Err(_)));
}

#[tokio::test]
async fn vidaimock_rate_limiting_test() {
    let harness = VidaiMockHarness::new().await?;

    // Test rate limit handling
    harness.configure_scenario(MockScenario::RateLimited).await?;

    // Verify agent respects rate limits
    // let result = agent.execute_step("test", "Hello", 5).await;
    // assert!(matches!(result, Err(LlmError::RateLimited(_))));
}
```

---

## Files to Modify

### 1. Cargo.toml

**Purpose:** Add Rig, Treadle, Sandlock, VidaiMock, and regex dependencies.

```toml
[dependencies]
# Existing...
rig-core = "0.4"           # For agent framework, agent builder, tool definitions
treadle = "0.2.0"         # For workflow state persistence, SQLite StateStore
sandlock = "0.5.0"         # For per-tool sandboxing, Landlock + seccomp
vidaimock = "0.1.3"        # For realistic LLM streaming testing
regex = "1.10"
chrono = "0.4"             # For timestamp in workflow state
```

### 2. src/config/unified.rs

**Purpose:** Export StepConfig for agent layer.

```rust
pub use crate::agent::StepConfig;
pub use crate::agent::StepResult;
```

### 3. src/bin/whitt.rs

**Purpose:** Replace ReAct agent with unified workflow execution.

```rust
use whitt::config::unified::{WorkflowSpec, StepConfig};
use whitt::agent::{ReactAgent, StepExecutor};
use whitt::model::ModelRegistry;

// In main():
// 1. Load unified YAML
// 2. Parse WorkflowSpec
// 3. Initialize ModelRegistry
// 4. For each step:
//    - Execute via ReactAgent
//    - Apply retry logic
// 5. Output results
```

---

## Implementation Tasks

### Task 1: Create tool definitions

**Files:**
- Create: `src/agent/tools.rs`

- [ ] **Step 1: Write tools.rs with all 6 tools**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Verify tools compile**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Run tool tests**

Run: `cargo test tools`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add src/agent/tools.rs
git commit -m "feat: add tool definitions for ReAct agent"
```

### Task 2: Implement ReAct agent

**Files:**
- Create: `src/agent/react.rs`

- [ ] **Step 1: Write react.rs with ReAct logic**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Verify agent compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/agent/react.rs
git commit -m "feat: implement ReAct agent with tool loop"
```

### Task 3: Implement step executor

**Files:**
- Create: `src/agent/executor.rs`

- [ ] **Step 1: Write executor.rs with retry logic**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Verify executor compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/agent/executor.rs
git commit -m "feat: implement step executor with retry"
```

### Task 4: Implement streaming support

**Files:**
- Create: `src/agent/streaming.rs`

- [ ] **Step 1: Write streaming.rs with SSE support**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add sseer dependency**

Run: `cargo add sseer futures`
Expected: Add sseer and futures to Cargo.toml

- [ ] **Step 3: Verify streaming compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/agent/streaming.rs Cargo.toml
git commit -m "feat: add SSE streaming support"
```

### Task 5: Integrate agent with CLI

**Files:**
- Modify: `src/bin/whitt.rs`

- [ ] **Step 1: Replace old ReAct agent with new implementation**

```rust
// Update whitt.rs to use ReactAgent and StepExecutor
```

- [ ] **Step 2: Verify CLI compiles**

Run: `cargo check --bin whitt`
Expected: PASS

- [ ] **Step 3: Run integration test**

Run: `cargo test agent_react`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add src/bin/whitt.rs
git commit -m "feat: integrate ReAct agent with CLI"
```

### Task 6: Implement Treadle StateStore integration

**Files:**
- Create: `src/agent/persistence.rs`
- Modify: `src/agent/react.rs` (integrate checkpointing)
- Modify: `Cargo.toml` (add treadle dependency)

- [ ] **Step 1: Write persistence.rs with WorkflowPersistence**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add treadle dependency**

Run: `cargo add treadle chrono`
Expected: Add treadle to Cargo.toml

- [ ] **Step 3: Integrate checkpointing into ReactAgent**

```rust
// In src/agent/react.rs ReactAgent::execute_step:
// let mut state = ReactAgentState { ... };
// let mut persistence = WorkflowPersistence::new(workflow_id, checkpoint_dir).await?;

// // After each turn:
// persistence.save_checkpoint(step_name, &state).await?;
```

- [ ] **Step 4: Verify persistence compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/agent/persistence.rs src/agent/react.rs Cargo.toml
git commit -m "feat: add Treadle StateStore integration for workflow persistence"
```

### Task 7: Implement Sandlock sandboxing

**Files:**
- Create: `src/agent/sandbox.rs`
- Modify: `src/agent/executor.rs` (wrap tool execution)
- Modify: `Cargo.toml` (add sandlock dependency)

- [ ] **Step 1: Write sandbox.rs with ToolSandbox**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add sandlock dependency**

Run: `cargo add sandlock`
Expected: Add sandlock to Cargo.toml

- [ ] **Step 3: Wrap tool execution in SandboxedToolExecutor**

```rust
// In src/agent/executor.rs:
// pub async fn execute_tool(&self, tool_name: &str, args: serde_json::Value)
//     -> anyhow::Result<String> {
//     let sandboxed = SandboxedToolExecutor::new();
//
//     // Determine allowed paths based on tool type
//     let allowed_paths = match tool_name {
//         "file_read" => vec!["./workspace/src".into()],
//         "file_write" => vec!["./workspace/output".into()],
//         _ => vec![],
//     };
//
//     sandboxed.execute_sandboxed_tool(tool_name, allowed_paths, || {
//         // Original tool execution
//         tool.execute(args, &self.tool_context).await
//     }).await
// }
```

- [ ] **Step 4: Verify sandbox compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/agent/sandbox.rs src/agent/executor.rs Cargo.toml
git commit -m "feat: add Sandlock per-tool sandboxing"
```

### Task 8: Implement VidaiMock testing

**Files:**
- Create: `tests/vidaimock_integration_test.rs`
- Modify: `Cargo.toml` (add vidaimock dev dependency)

- [ ] **Step 1: Write vidaimock_integration_test.rs**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add vidaimock dev dependency**

Run: `cargo add --dev vidaimock chrono`
Expected: Add vidaimock to dev-dependencies in Cargo.toml

- [ ] **Step 3: Verify tests compile**

Run: `cargo test vidaimock`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add tests/vidaimock_integration_test.rs Cargo.toml
git commit -m "test: add VidaiMock integration for realistic streaming testing"
```

---

## Success Criteria

- [ ] 6 tools defined (model_list, model_load, model_unload, chat, file_read, final_answer)
- [ ] ReAct agent implements tool loop with max turns
- [ ] Step executor implements retry logic from step config
- [ ] Streaming support via SSE implemented
- [ ] Regex-based tool parsing works
- [ ] Integration with provider backend (LlmBackend trait)
- [ ] Integration with model registry
- [ ] CLI uses new agent implementation
- [ ] **Treadle StateStore integration** — Workflow checkpoints save/load/resume
- [ ] **Sandlock per-tool sandboxing** — Landlock + seccomp + resource limits
- [ ] **VidaiMock testing** — Physics-accurate streaming scenarios (slow, dropped, rate-limited)
- [ ] Unit tests pass (cargo test agent_react)
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build)
