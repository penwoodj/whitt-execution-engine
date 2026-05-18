# Extended POC Implementation Guide

This guide walks through the implementation of the extended POC phase-by-phase, including dependency ordering, migration path, testing strategy, and key code patterns.

---

## Phase 1: Provider Configuration

### Objective

Map the unified schema `providers` section (lines 27-51) to Rust structs with serde-saphyr parsing, garde validation, and LlmBackend trait.

### Files to Create

| File | Purpose | Lines |
|------|---------|--------|
| `src/config/provider.rs` | Provider config structs | 280+ |
| `src/backend/llm_backend.rs` | LlmBackend trait definition | 70+ |
| `src/backend/llama_vulkan.rs` | LlamaCppVulkanBackend impl | 280+ |
| `tests/provider_config_test.rs` | Unit tests for parsing | 80+ |

### Files to Modify

| File | Changes |
|------|----------|
| `src/config/mod.rs` | Export provider types, keep backward compat |
| `src/client/mod.rs` | Export LlmBackend trait |
| `Cargo.toml` | Add: garde, async-trait, futures, sseer, fastrand |

### Implementation Steps

#### Step 1: Create Provider Config Structs

**File**: `src/config/provider.rs`

```rust
// Core structs (simplified signatures):
pub struct ProviderConfig {
    #[serde(rename = "llama_cpp_with_vulkan")]
    pub llama_cpp_with_vulkan: Option<LlamaCppVulkanProvider>,
}

pub struct LlamaCppVulkanProvider {
    pub config: Option<LlamaCppConfig>,
    pub config_file: Option<String>,  // POC: inline only
}

pub struct LlamaCppConfig {
    pub config: ConnectionConfig,      // host, port, timeout
    pub hosting: HostingConfig,        // concurrency, GPU/CPU
    pub requests: RequestsConfig,      // timeouts, rate limiting, retry
}

pub struct RetryConfig {
    pub max_retries: u32,
    pub backoff: BackoffStrategy,     // Exponential, Linear, Fixed
    pub initial_delay: String,
    pub max_delay: String,
    pub multiplier: f64,
    pub jitter: bool,
}
```

**Key Patterns**:
- Use `#[serde(default)]` for optional nested configs
- Use `#[garde(range(min=..., max=...))]` for validation
- Provide `default_*()` functions matching unified schema defaults

#### Step 2: Define LlmBackend Trait

**File**: `src/backend/llm_backend.rs`

```rust
#[async_trait]
pub trait LlmBackend: Send + Sync {
    async fn chat(&self, request: ChatCompletionRequest)
        -> Result<ChatCompletionResponse, LlmError>;

    async fn chat_stream(&self, request: ChatCompletionRequest)
        -> Result<Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>, LlmError>;

    async fn list_models(&self) -> Result<Vec<String>, LlmError>;
    async fn health_check(&self) -> Result<HealthStatus, LlmError>;
    async fn load_model(&self, model_id: &str) -> Result<(), LlmError>;
    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError>;
    fn capabilities(&self) -> BackendCapabilities;
    fn base_url(&self) -> &str;
}
```

**Key Patterns**:
- Use `Pin<Box<dyn Stream>>` for async streaming
- Define error enum `LlmError` (Connection, Timeout, Parse, Model, RateLimited, Internal)
- Define `BackendCapabilities` struct (streaming, tools, function_calling)
- Define `HealthStatus` enum (Healthy, Degraded, Unhealthy)

#### Step 3: Implement LlamaCppVulkanBackend

**File**: `src/backend/llama_vulkan.rs`

```rust
pub struct LlamaCppVulkanBackend {
    base_url: String,
    client: Client,
    timeout: Duration,
    model: String,
}

impl LlamaCppVulkanBackend {
    pub fn new(config: &LlamaCppConfig, model: String) -> Self {
        // Build HTTP client from config.host, config.port, timeout
    }

    fn calculate_delay(retry_config: &RetryConfig, attempt: u32) -> Duration {
        // Exponential: base * multiplier^(attempt-1)
        // Linear: base * attempt
        // Fixed: base
        // Add jitter if enabled
    }
}

#[async_trait]
impl LlmBackend for LlamaCppVulkanBackend {
    // Implement all trait methods using reqwest HTTP client
    // Use SSE parsing for chat_stream
}
```

**Key Patterns**:
- Use `reqwest::Client::builder().timeout().build()` for HTTP client
- Use `sseer::Event::parse()` for SSE stream parsing
- Implement retry delay calculation with jitter (fastrand)
- Map HTTP status codes to `LlmError` variants

#### Step 4: Update Config Loader

**File**: `src/config/mod.rs`

```rust
impl ConfigLoader {
    pub fn load_provider(yaml: &str) -> anyhow::Result<ProviderConfig> {
        let provider: ProviderConfig = serde_saphyr::from_str(yaml)?;
        Ok(provider)
    }
}
```

**Key Patterns**:
- Keep `LlamaConfig` deprecated but available for migration
- Export both `ProviderConfig` and `LlamaConfig` from `mod.rs`

### Testing Strategy

**Unit Tests** (`tests/provider_config_test.rs`):
- Parse minimal provider config (host, port, timeout only)
- Parse full provider config with retry, GPU, CPU fallback
- Garde validation rejects invalid port (> 65535)
- Garde validation rejects negative timeout
- Backoff strategy enum parsing (exponential, linear, fixed)

**Integration Tests**:
- Load unified YAML with providers section
- Verify config resolution order (providers → per-model → defaults)
- Test LlamaCppVulkanBackend against mock llama.cpp server

---

## Phase 2: Model Schema

### Objective

Implement model schema layer with registry, resource management, and parse-time interpolation (${models.model_name}).

### Prerequisites

**Phase 1 (Provider Config) must be complete** — ModelSpec references provider types.

### Files to Create

| File | Purpose | Lines |
|------|---------|--------|
| `src/model/schema.rs` | Model schema structs | 370+ |
| `src/model/registry.rs` | ModelRegistry with lifecycle | 160+ |
| `src/model/resource.rs` | ResourceManager (RAM, VRAM, CPU, GPU) | 150+ |
| `src/model/interpolation.rs` | TemplateInterpolator with Minijinja | 70+ |
| `tests/model_schema_test.rs` | Unit tests for parsing and interpolation | 70+ |

### Files to Modify

| File | Changes |
|------|----------|
| `src/config/unified.rs` | Export ModelsConfig, ModelSpec |
| `Cargo.toml` | Add: minijinja |

### Implementation Steps

#### Step 1: Create Model Schema Structs

**File**: `src/model/schema.rs`

```rust
pub struct ModelsConfig {
    pub global_config_path: Option<String>,
    pub default_router: RouterStrategy,  // Automatic, Manual
    pub models: HashMap<String, ModelSpec>,
}

pub struct ModelSpec {
    pub name: String,
    pub host: ModelHost,
    pub ram_allocation: Option<RamAllocation>,  // presence = configured
    pub max_allowed: MaxAllowed,
    pub min_allowed: MinAllowed,
    pub model_memory: ModelMemory,
    pub execution: ExecutionConfig,
    pub thinking: Option<ThinkingConfig>,  // presence = enabled
}

pub struct MaxAllowed {
    pub ram: ResourceLimit,      // Percentage("13%") or Absolute("8GB")
    pub vram: Option<ResourceLimit>,
    pub cpu: Option<ResourceLimit>,
    pub gpu: Option<ResourceLimit>,
    pub attention_tokens: Option<usize>,
    pub concurrent_requests: Option<usize>,
}

pub struct ExecutionConfig {
    pub timeout: TimeoutConfig,
    pub max_turns: usize,
    pub stop_on_tool_failure: bool,
    pub accumulate_tool_results: bool,
}
```

**Key Patterns**:
- Use `#[serde(untagged)]` for `ResourceLimit` enum (handles both % and GB/MB)
- Use `Option<>` for presence-based features (ram_allocation, thinking)
- Provide default functions matching unified schema defaults

#### Step 2: Implement ModelRegistry

**File**: `src/model/registry.rs`

```rust
pub enum ModelState {
    Unloaded, Loading, Loaded, Unloading, Error,
}

pub struct ModelRegistry {
    models: Arc<RwLock<HashMap<String, ModelEntry>>>,
    resource_manager: ResourceManager,
}

impl ModelRegistry {
    pub fn new(config: ModelsConfig, resource_manager: ResourceManager) -> Self {
        // Initialize all models in Unloaded state
    }

    pub async fn load_model(&self, model_id: &str) -> anyhow::Result<()> {
        // 1. Check state ( Loaded = return, Loading = error )
        // 2. Set state to Loading
        // 3. Check resource availability via ResourceManager
        // 4. Call backend.load_model()
        // 5. Set state to Loaded
    }

    pub async fn unload_model(&self, model_id: &str) -> anyhow::Result<()> {
        // 1. Check state ( Unloaded = return, Unloading = error )
        // 2. Set state to Unloading
        // 3. Call backend.unload_model()
        // 4. Release resources via ResourceManager
        // 5. Set state to Unloaded
    }

    pub async fn health_check(&self, model_id: &str) -> anyhow::Result<HealthStatus> {
        // Call backend.health_check() if backend exists
    }

    pub async fn get_spec(&self, model_id: &str) -> Option<ModelSpec>;
    pub async fn list_models(&self) -> Vec<String>;
}
```

**Key Patterns**:
- Use `Arc<RwLock<HashMap>>` for thread-safe model registry
- Release lock before async operations (backend calls)
- Track state transitions: Unloaded → Loading → Loaded → Unloading → Unloaded
- Return early if already in desired state

#### Step 3: Implement ResourceManager

**File**: `src/model/resource.rs`

```rust
pub struct ResourceManager {
    allocations: Arc<RwLock<HashMap<String, ResourceAllocation>>>,
    total_resources: TotalResources,
}

impl ResourceManager {
    pub fn new(total_resources: TotalResources) -> Self;

    fn parse_limit(limit: &ResourceLimit) -> anyhow::Result<usize> {
        // Parse "13%" → (13/100) * 100 = 13MB
        // Parse "3.7GB" → 3.7 * 1024 = 3788MB
        // Parse "8GB" → 8 * 1024 = 8192MB
    }

    pub async fn check_resources(&self, max_allowed: &MaxAllowed) -> anyhow::Result<()> {
        // Sum current allocations
        // Check if required RAM/VRAM fits within total
        // Return error if insufficient
    }

    pub async fn allocate_resources(&self, model_id: String, max_allowed: &MaxAllowed);
    pub async fn release_resources(&self, model_id: &str);
}
```

**Key Patterns**:
- Parse both percentage ("13%") and absolute ("8GB") resource limits
- Use 100MB base for percentage calculations (configurable)
- Track per-model allocations (ram_mb, vram_mb, cpu_percent, gpu_percent)

#### Step 4: Implement TemplateInterpolator

**File**: `src/model/interpolation.rs`

```rust
pub struct TemplateInterpolator {
    env: Environment<'static>,
}

impl TemplateInterpolator {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_debug(false);
        Self { env }
    }

    pub fn interpolate(
        &self,
        template: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String, Error> {
        // Convert ${models.model_name} to {{models.model_name}} for Minijinja
        let template = template.replace("${", "{{").replace("}", "}}");
        self.env.render_str(&template, variables)
    }

    pub fn parse_var_ref(s: &str) -> Option<(String, String)> {
        // Extract "${models.model_name}" pattern
        if s.starts_with("${models.") && s.ends_with("}") {
            let model_name = &s["${models.".len()..s.len() - 1];
            Some(("models".into(), model_name.into()))
        } else {
            None
        }
    }
}
```

**Key Patterns**:
- Use Minijinja for Jinja2-like template syntax
- Convert `${...}` to `{{...}}` syntax for Minijinja compatibility
- Parse-time resolution only (runtime `{{step.X}}` deferred to Phase 2)

### Testing Strategy

**Unit Tests** (`tests/model_schema_test.rs`):
- Parse model spec with all fields
- Parse model spec with minimal required fields
- Template interpolation with variables
- Resource limit parsing (percentage vs absolute)
- Model override resolution (deferred to Phase 3)

**Integration Tests**:
- Load unified YAML with models section
- Initialize ModelRegistry from config
- Load/unload models via registry
- Verify resource allocation tracking

---

## Phase 3: Agent ReAct

### Objective

Implement basic ReAct agent using Rig framework with tool definitions, step execution, retry logic, and streaming support.

### Prerequisites

**Phase 1 (Provider Config) + Phase 2 (Model Schema) must be complete** — Agent depends on LlmBackend trait and ModelRegistry.

### Files to Create

| File | Purpose | Lines |
|------|---------|--------|
| `src/agent/tools.rs` | 6 tool definitions | 320+ |
| `src/agent/react.rs` | ReAct agent implementation | 220+ |
| `src/agent/executor.rs` | Step executor with retry logic | 170+ |
| `src/agent/streaming.rs` | SSE streaming support | 80+ |
| `tests/agent_react_test.rs` | Unit tests for agent | 50+ |

### Files to Modify

| File | Changes |
|------|----------|
| `src/config/unified.rs` | Export StepConfig, StepResult |
| `src/bin/whitt.rs` | Replace old ReAct agent with new implementation |
| `Cargo.toml` | Add: rig-core, regex |

### Implementation Steps

#### Step 1: Create Tool Definitions

**File**: `src/agent/tools.rs`

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: serde_json::Value, ctx: &ToolContext)
        -> Result<String, ToolError>;
}

pub struct ToolContext {
    pub model_registry: ModelRegistry,
    pub workspace_path: std::path::PathBuf,
}

// Tools:
pub struct ModelListTool;         // list_models()
pub struct ModelLoadTool;         // model_load(model_id)
pub struct ModelUnloadTool;       // model_unload(model_id)
pub struct ChatTool {             // chat(prompt, max_tokens)
    backend: Arc<dyn LlmBackend>,
}
pub struct FileReadTool;          // file_read(path)
pub struct FinalAnswerTool;       // final_answer(answer) — terminates loop

pub fn get_tools(backend: Arc<dyn LlmBackend>) -> Vec<Box<dyn Tool>> {
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

**Key Patterns**:
- Use async trait for tool execution
- Tools receive `ToolContext` with registry and workspace path
- Tools return `Result<String, ToolError>` (InvalidArgs, ExecutionFailed, Backend)
- `FinalAnswerTool` signals agent termination

#### Step 2: Implement ReAct Agent

**File**: `src/agent/react.rs`

```rust
pub struct ReactAgent {
    backend: Arc<dyn LlmBackend>,
    model_registry: Arc<ModelRegistry>,
    executor: StepExecutor,
    tool_call_regex: Regex,
}

impl ReactAgent {
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
        let tool_call_regex = Regex::new(r"([A-Za-z_]+)\(([^)]+)\)").unwrap();

        Self { backend, model_registry, executor, tool_call_regex }
    }

    pub async fn execute_step(
        &self,
        step_name: &str,
        prompt: &str,
        max_turns: usize,
    ) -> anyhow::Result<String> {
        let mut state = ReactAgentState {
            messages: vec![system_message()],
            turn_count: 0,
            completed: false,
        };
        state.messages.push(user_message(prompt));

        while state.turn_count < max_turns && !state.completed {
            let llm_response = self.call_llm(&state.messages).await?;

            if let Some(tool_call) = self.parse_tool_call(&llm_response) {
                let tool_result = self.executor.execute_tool(&tool_call.name, tool_call.args).await?;

                state.messages.push(assistant_message(llm_response));
                state.messages.push(tool_message(tool_call.name, tool_result));

                if tool_call.name == "final_answer" {
                    state.completed = true;
                }
            } else {
                state.completed = true;
                return Ok(llm_response);
            }

            state.turn_count += 1;
        }

        Err(anyhow::anyhow!("Max turns ({}) exceeded", max_turns))
    }

    fn parse_tool_call(&self, response: &str) -> Option<ToolCall> {
        let captures = self.tool_call_regex.captures(response)?;
        let tool_name = captures.get(1)?.as_str().to_string();
        let args_str = captures.get(2)?.as_str().to_string();
        Some(ToolCall { tool_name, args: self.parse_args(&args_str) })
    }

    fn parse_args(&self, args_str: &str) -> serde_json::Value {
        // Parse "arg1=value1, arg2=value2" into JSON object
    }
}
```

**Key Patterns**:
- Use regex to parse tool calls from LLM response: `ToolName(arg1=value1, arg2=value2)`
- Track turn count and enforce max_turns from model config
- Append tool results to conversation history for next LLM call
- `final_answer` tool terminates loop

#### Step 3: Implement Step Executor

**File**: `src/agent/executor.rs`

```rust
pub struct StepExecutor {
    tools: HashMap<String, Box<dyn Tool>>,
    tool_context: ToolContext,
}

impl StepExecutor {
    pub fn new(tools: Vec<Box<dyn Tool>>, tool_context: ToolContext) -> Self {
        let mut tools_map = HashMap::new();
        for tool in tools {
            tools_map.insert(tool.name().to_string(), tool);
        }
        Self { tools: tools_map, tool_context }
    }

    pub async fn execute_step(&self, config: &StepConfig) -> StepResult {
        let retry_config = config.retry.clone().unwrap_or_else(|| default_retry());

        let mut attempts = 0;
        for attempt in 0..=retry_config.max_retries {
            attempts = attempt + 1;

            match self.try_execute(config).await {
                Ok(result) => return StepResult { success: true, output: result, attempts },
                Err(e) if attempt < retry_config.max_retries => {
                    let delay = Self::calculate_delay(&retry_config, attempt);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => break,
            }
        }

        StepResult { success: false, output: last_error, attempts }
    }

    pub async fn execute_tool(&self, tool_name: &str, args: serde_json::Value) -> anyhow::Result<String> {
        let tool = self.tools.get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool {} not found", tool_name))?;
        tool.execute(args, &self.tool_context).await
            .map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))
    }

    fn calculate_delay(retry_config: &RetryConfig, attempt: u32) -> Duration {
        // Same delay calculation as LlamaCppVulkanBackend
    }
}
```

**Key Patterns**:
- Execute step with retry loop (up to max_retries)
- Use same backoff calculation as provider config
- Return `StepResult` with success, output, attempts

#### Step 4: Implement Streaming Support

**File**: `src/agent/streaming.rs`

```rust
pub struct StreamingResponse {
    stream: Pin<Box<dyn Stream<Item = Result<StreamChunk, String>> + Send>>,
}

impl StreamingResponse {
    pub fn new(backend: Arc<dyn LlmBackend>, request: ChatCompletionRequest) -> Self {
        let stream = backend.chat_stream(request);
        let stream = stream.map(|result| result.map(|text| StreamChunk { text, done: text.is_empty() }));
        Self { stream: Box::pin(stream) }
    }

    pub async fn collect(self) -> Result<String, String> {
        use futures::StreamExt;
        let mut output = String::new();
        let mut stream = self.stream;

        while let Some(result) = stream.next().await {
            match result {
                Ok(chunk) => {
                    if chunk.done { break; }
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
```

**Key Patterns**:
- Wrap backend's chat_stream in StreamingResponse
- Stream chunks printed to stdout as they arrive
- Collect full response into string for return

### Testing Strategy

**Unit Tests** (`tests/agent_react_test.rs`):
- Tool list contains all 6 tools
- Tool execution with valid args
- Tool execution with invalid args
- Regex parsing of tool calls
- ReAct agent single-turn execution
- ReAct agent multi-turn with tools

**Integration Tests**:
- Load unified YAML with steps section
- Initialize ReactAgent with backend and registry
- Execute step with max_turns enforcement
- Verify retry logic on transient errors
- Test streaming output

---

## Dependency Ordering

```
Phase 1: Provider Config
├─ Must complete FIRST (no dependencies)
└─ Provides: LlmBackend trait, LlamaCppVulkanBackend

Phase 2: Model Schema
├─ Requires: Phase 1 complete (ModelSpec references provider types)
└─ Provides: ModelRegistry, ResourceManager, TemplateInterpolator

Phase 3: Agent ReAct
├─ Requires: Phase 1 + Phase 2 complete (needs LlmBackend + ModelRegistry)
└─ Provides: ReactAgent, StepExecutor, Tools, Streaming

Integration Testing
└─ Requires: All 3 phases complete
```

**Critical Path**: Phase 1 → Phase 2 → Phase 3 → Integration Tests

---

## Migration Path

### From Current POC to Extended POC

#### Step 1: Add New Dependencies

```bash
cargo add garde async-trait futures sseer fastrand minijinja rig-core regex
```

#### Step 2: Implement Provider Config Layer

1. Create `src/config/provider.rs` with all structs
2. Create `src/backend/llm_backend.rs` with trait definition
3. Create `src/backend/llama_vulkan.rs` with backend impl
4. **DO NOT DELETE** `src/client/llama_client.rs` yet
5. Add unit tests
6. Run `cargo test provider_config_test`
7. Commit

#### Step 3: Implement Model Schema Layer

1. Create `src/model/schema.rs` with model structs
2. Create `src/model/registry.rs` with lifecycle management
3. Create `src/model/resource.rs` with resource tracking
4. Create `src/model/interpolation.rs` with Minijinja
5. Update `src/config/unified.rs` to export `ModelsConfig`
6. Add unit tests
7. Run `cargo test model_schema_test`
8. Commit

#### Step 4: Implement Agent ReAct Layer

1. Create `src/agent/tools.rs` with 6 tool definitions
2. Create `src/agent/react.rs` with ReAct agent
3. Create `src/agent/executor.rs` with retry logic
4. Create `src/agent/streaming.rs` with SSE support
5. **DO NOT REPLACE** `src/bin/whitt.rs` yet
6. Add unit tests
7. Run `cargo test agent_react_test`
8. Commit

#### Step 5: Integrate with CLI

1. Update `src/bin/whitt.rs` to use new agent implementation
2. Load unified YAML instead of flat config.yml
3. Initialize `ModelRegistry` from `ModelsConfig`
4. Execute steps via `ReactAgent` and `StepExecutor`
5. Test end-to-end with sample workflow
6. Commit

#### Step 6: Cleanup

1. **Deprecate** `LlamaConfig` (keep for migration)
2. **Delete** `src/client/llama_client.rs` (logic moved to backend impl)
3. Remove old ReAct agent code
4. Run `cargo build` and `cargo test`
5. Final commit

### Breaking Changes

| Change | Impact | Migration Path |
|--------|--------|---------------|
| `config.yml` → unified YAML | Flat config now hierarchical | Provide migration script to convert old config |
| `LlamaConfig` → `ProviderConfig` | Struct fields renamed/changed | Keep `LlamaConfig` deprecated with #[deprecated] attribute |
| ReAct agent API | Tool definitions now use trait | Update any custom tools to implement `Tool` trait |

---

## Key Code Patterns

### Trait Definitions

```rust
// Always use async-trait for async traits
#[async_trait]
pub trait LlmBackend: Send + Sync {
    async fn method(&self) -> Result<Type, Error>;
}

// Always include Send + Sync bounds for shared references
```

### Error Handling

```rust
// Library types: use thiserror
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Connection error: {0}")]
    Connection(String),
    // ...
}

// Application code: use anyhow
pub async fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(serde_saphyr::from_str(&content)?)
}
```

### Config Resolution Order

1. **Providers section** (base defaults)
2. **Per-model overrides** (`models.{model_name}.host.connection_settings`)
3. **Step-level overrides** (`steps.{step_name}.model_overrides`)

Use `serde(default)` for optional fields. Override via explicit merge logic.

### Async Lock Patterns

```rust
// Acquire lock, check, release, then async operation
let mut models = self.models.write().await;
if entry.state == ModelState::Loaded {
    return Ok(());
}
entry.state = ModelState::Loading;
drop(models); // Release lock before async call

// Do async operation
backend.load_model(model_id).await?;

// Re-acquire lock to update state
let mut models = self.models.write().await;
entry.state = ModelState::Loaded;
```

---

## Related Documentation

- **Plan Files**:
  - `01-provider-config.md` — Detailed provider config tasks
  - `02-model-schema.md` — Detailed model schema tasks
  - `03-agent-react.md` — Detailed agent ReAct tasks

- **Schema Reference**: `docs/schema/unified-workflow-schema.yml`

- **Architecture Decisions**:
  - `docs/roadmap/ADR-0003.md` — CLI & Backend Integration
  - `docs/roadmap/ADR-0001.md` — Foundation Phase
