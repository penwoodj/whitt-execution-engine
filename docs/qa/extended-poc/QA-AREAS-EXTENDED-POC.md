# QA Areas — Extended POC (Provider Config + Model Schema + Agent React)

**Plan Suite**: `docs/plans/extended-poc/`
**Date**: 2026-04-26
**Status**: 🔴 NOT STARTED — Pre-implementation QA criteria definition

---

## Summary Table

| # | QA Area | Plan Ref | Status | Priority |
|---|---------|----------|--------|----------|
| 1 | Provider Config Parsing (serde-saphyr) | 01-provider-config | 🔴 Pending | P0 |
| 2 | Provider Config Validation (garde) | 01-provider-config | 🔴 Pending | P0 |
| 3 | LlmBackend Trait | 01-provider-config | 🔴 Pending | P0 |
| 4 | LlamaCppVulkanBackend Implementation | 01-provider-config | 🔴 Pending | P0 |
| 5 | Config Resolution Hierarchy | 01-provider-config | 🔴 Pending | P0 |
| 6 | Model Schema Parsing | 02-model-schema | 🔴 Pending | P0 |
| 7 | Model Registry Lifecycle | 02-model-schema | 🔴 Pending | P0 |
| 8 | Resource Management | 02-model-schema | 🔴 Pending | P1 |
| 9 | Template Interpolation (Minijinja) | 02-model-schema | 🔴 Pending | P0 |
| 10 | ReAct Agent Tool Loop | 03-agent-react | 🔴 Pending | P0 |
| 11 | Tool Definitions (6 tools) | 03-agent-react | 🔴 Pending | P0 |
| 12 | Step Executor with Retry | 03-agent-react | 🔴 Pending | P0 |
| 13 | SSE Streaming | 03-agent-react | 🔴 Pending | P1 |
| 14 | Workflow Persistence (Treadle) | 03-agent-react | 🔴 Pending | P1 |
| 15 | Tool Sandboxing (Sandlock) | 03-agent-react | 🔴 Pending | P2 |
| 16 | Mock Testing (VidaiMock) | 03-agent-react | 🔴 Pending | P1 |
| 17 | Schema Version Validation | 00-master-plan | 🔴 Pending | P0 |
| 18 | End-to-End Integration | 00-master-plan | 🔴 Pending | P0 |
| 19 | CLI Integration | 03-agent-react | 🔴 Pending | P0 |
| 20 | Build Hygiene (warnings/errors/clippy) | All | 🔴 Pending | P0 |

---

## Area Details

### Area 1: Provider Config Parsing (serde-saphyr)

**Plan**: 01-provider-config.md
**Files**: `src/config/provider.rs`
**Criteria**:
- Parse `providers.llama_cpp_with_vulkan` section from unified YAML
- All structs deserialize correctly: ProviderConfig, LlamaCppVulkanProvider, LlamaCppConfig, ConnectionConfig, HostingConfig, GpuAllocation, CpuFallback, RequestsConfig, RetryConfig
- Missing optional fields use correct defaults (e.g., host=localhost, port=8080)
- serde-saphyr handles merge keys correctly
- No hardcoded defaults in implementation — all from YAML or struct default functions
**Test Commands**: See `QA-TEST-PROCEDURES-EXTENDED-POC.md` EPOC-001 through EPOC-005

### Area 2: Provider Config Validation (garde)

**Plan**: 01-provider-config.md
**Files**: `src/config/provider.rs`
**Criteria**:
- `garde::Validate` annotations enforce range constraints
- Port must be 1-65535
- connection_timeout_secs must be >= 1
- max_concurrent_models must be >= 1
- request_timeout_secs must be >= 1
- max_retries must be >= 0
- multiplier must be >= 1.0
- Invalid configs rejected with clear error messages at load time, not runtime
**Test Commands**: See EPOC-006 through EPOC-008

### Area 3: LlmBackend Trait

**Plan**: 01-provider-config.md
**Files**: `src/backend/llm_backend.rs`
**Criteria**:
- Trait defined with all 8 methods: chat, chat_stream, list_models, health_check, load_model, unload_model, capabilities, base_url
- LlmError enum covers: Connection, Timeout, Parse, Model, RateLimited, Internal
- BackendCapabilities struct: streaming, tools, function_calling
- HealthStatus enum: Healthy, Degraded, Unhealthy
- Trait is object-safe (can use `dyn LlmBackend`)
- async_trait applied correctly
**Test Commands**: See EPOC-009 through EPOC-011

### Area 4: LlamaCppVulkanBackend Implementation

**Plan**: 01-provider-config.md
**Files**: `src/backend/llama_vulkan.rs`
**Criteria**:
- Implements LlmBackend trait fully
- chat() sends POST /v1/chat/completions
- chat_stream() sends POST /v1/chat/completions with stream=true, returns SSE stream
- list_models() sends GET /v1/models
- health_check() sends GET /health
- load_model() sends POST /v1/models/load
- unload_model() sends POST /v1/models/unload
- Retry delay calculation: exponential, linear, fixed backoff with jitter
- Duration parsing: "1s", "30s" etc.
- All timeouts from config, not hardcoded
**Test Commands**: See EPOC-012 through EPOC-018

### Area 5: Config Resolution Hierarchy

**Plan**: 01-provider-config.md
**Criteria**:
- Resolution order: providers section → per-model overrides → step-level overrides → defaults
- CLI args still override everything (backward compat from POC 1)
- Missing provider section uses sensible defaults
- Override chain produces correct final values
**Test Commands**: See EPOC-019 through EPOC-021

### Area 6: Model Schema Parsing

**Plan**: 02-model-schema.md
**Files**: `src/model/schema.rs`
**Criteria**:
- Parse `models` section from unified YAML
- ModelsConfig: global_config_path, default_router, models HashMap
- ModelSpec: name, host, ram_allocation, max_allowed, min_allowed, model_memory, execution, thinking
- ResourceLimit: percentage ("13%") and absolute ("3.7GB")
- All enum variants parse correctly (CacheSize, KvQuantization, AttentionContext, RouterStrategy)
- ThinkingConfig optional (presence = enabled)
**Test Commands**: See EPOC-022 through EPOC-026

### Area 7: Model Registry Lifecycle

**Plan**: 02-model-schema.md
**Files**: `src/model/registry.rs`
**Criteria**:
- ModelState: Unloaded → Loading → Loaded → Unloading → Error
- load_model() transitions state correctly, prevents double-load
- unload_model() transitions state correctly, releases resources
- health_check() delegates to backend or returns Unhealthy if no backend
- get_spec() returns ModelSpec for registered model
- list_models() returns all registered model IDs
- Concurrent access safe (RwLock)
**Test Commands**: See EPOC-027 through EPOC-031

### Area 8: Resource Management

**Plan**: 02-model-schema.md
**Files**: `src/model/resource.rs`
**Criteria**:
- ResourceManager tracks RAM, VRAM, CPU, GPU allocations
- check_resources() rejects load if insufficient RAM or VRAM
- allocate_resources() records allocation for model
- release_resources() removes allocation on unload
- Percentage parsing: "13%" → value
- Absolute parsing: "3.7GB" → 3788 MB, "512MB" → 512 MB
- Unknown unit formats rejected with clear error
**Test Commands**: See EPOC-032 through EPOC-036

### Area 9: Template Interpolation (Minijinja)

**Plan**: 02-model-schema.md
**Files**: `src/model/interpolation.rs`
**Criteria**:
- `${models.model_name}` resolved at parse time (not runtime)
- TemplateInterpolator converts `${...}` to `{{...}}` for Minijinja
- parse_var_ref extracts model name from `${models.primary-analyzer}`
- Unknown variable references produce clear errors
- Does NOT support `{{step.name.output}}` (runtime, deferred to Phase 2)
**Test Commands**: See EPOC-037 through EPOC-040

### Area 10: ReAct Agent Tool Loop

**Plan**: 03-agent-react.md
**Files**: `src/agent/react.rs`
**Criteria**:
- execute_step() runs ReAct loop: LLM call → parse tool call → execute → append → loop
- Max turns enforced (returns error when exceeded)
- final_answer tool terminates loop
- No tool call = treat as final answer
- turn_count tracked correctly
- ReactAgentState: messages, turn_count, completed
**Test Commands**: See EPOC-041 through EPOC-045

### Area 11: Tool Definitions (6 tools)

**Plan**: 03-agent-react.md
**Files**: `src/agent/tools.rs`
**Criteria**:
- model_list: returns JSON array of model IDs
- model_load: loads model by ID, returns success message
- model_unload: unloads model by ID, returns success message
- chat: sends ChatCompletionRequest, returns response content
- file_read: reads file from workspace_path, returns contents
- final_answer: returns answer string, terminates ReAct loop
- Each tool has name() and description() for LLM context
- Invalid args produce ToolError::InvalidArgs with clear message
**Test Commands**: See EPOC-046 through EPOC-052

### Area 12: Step Executor with Retry

**Plan**: 03-agent-react.md
**Files**: `src/agent/executor.rs`
**Criteria**:
- execute_step() runs with retry logic from RetryConfig
- Backoff strategies: Exponential (2^n * base), Linear (n * base), Fixed (base)
- Jitter adds random delay (0-10% of calculated delay)
- Max delay enforced (never exceeds max_delay)
- StepResult: success, output, attempts count
- Failed after all retries → StepResult.success = false
- No retry config → default (3 retries, exponential, jitter=true)
**Test Commands**: See EPOC-053 through EPOC-058

### Area 13: SSE Streaming

**Plan**: 03-agent-react.md
**Files**: `src/agent/streaming.rs`
**Criteria**:
- StreamingResponse wraps backend chat_stream()
- StreamChunk: text + done flag
- collect() consumes stream into string
- Done chunk (empty text) terminates stream
- Implements futures::Stream trait
**Test Commands**: See EPOC-059 through EPOC-061

### Area 14: Workflow Persistence (Treadle)

**Plan**: 03-agent-react.md
**Files**: `src/agent/persistence.rs`
**Criteria**:
- WorkflowPersistence wraps Treadle StateStore
- save_checkpoint() serializes WorkflowState to SQLite
- load_checkpoint() deserializes from SQLite
- list_checkpoints() returns all step names for workflow
- clear_checkpoints() removes all for workflow
- WorkflowState: step_name, turn_count, messages, completed, last_update
- Checkpoint key format: `{workflow_id}.{step_name}`
**Test Commands**: See EPOC-062 through EPOC-066

### Area 15: Tool Sandboxing (Sandlock)

**Plan**: 03-agent-react.md
**Files**: `src/agent/sandbox.rs`
**Criteria**:
- ToolSandbox configures Sandlock: Landlock + seccomp + memory/CPU limits
- Default: 512MB memory, 30s CPU time, network restricted
- with_filesystem_access() allows specific paths
- with_network_access() allows specific domains
- SandboxedToolExecutor wraps tool execution
- Per-tool sandboxing (not global)
**Test Commands**: See EPOC-067 through EPOC-070

### Area 16: Mock Testing (VidaiMock)

**Plan**: 03-agent-react.md
**Files**: `tests/vidaimock_integration_test.rs`
**Criteria**:
- VidaiMockHarness starts mock server with configurable behavior
- MockScenario::SlowStreaming: 5s TTFT, 1 token/sec
- MockScenario::DroppedConnections: 10% drop rate
- MockScenario::RateLimited: 60 RPM limit
- MockScenario::MalformedResponses: 1% malformed
- Agent handles all scenarios gracefully
**Test Commands**: See EPOC-071 through EPOC-074

### Area 17: Schema Version Validation

**Plan**: 00-master-plan.md
**Criteria**:
- All YAML files must include `schema_version: "2.0.0"`
- schema_version validated at load time
- Incompatible versions rejected with clear error
- Semver format: MAJOR.MINOR.PATCH
- min_schema_version checked against current supported version
**Test Commands**: See EPOC-075 through EPOC-077

### Area 18: End-to-End Integration

**Plan**: 00-master-plan.md
**Criteria**:
- Full workflow: load YAML → parse → resolve config → load model → execute step → stream response
- Provider config → Model registry → Agent execution chain works
- No hardcoded values in implementation
- Template interpolation resolves before agent execution
- Step-level model overrides applied correctly
**Test Commands**: See EPOC-078 through EPOC-081

### Area 19: CLI Integration

**Plan**: 03-agent-react.md
**Files**: `src/bin/whitt.rs`
**Criteria**:
- CLI uses new ReactAgent + StepExecutor
- Existing commands still work (backward compat)
- New commands for workflow execution added
- Unified YAML loaded and parsed on startup
- Deprecation warnings for old flat config
**Test Commands**: See EPOC-082 through EPOC-084

### Area 20: Build Hygiene

**Plan**: All
**Criteria**:
- `cargo build --release --all-features`: 0 warnings, 0 errors
- `cargo clippy --all-features -- -W clippy::all`: 0 warnings
- `cargo test --lib`: all unit tests pass
- LSP diagnostics: 0 errors on all changed files
- No `as any`, `@ts-ignore` equivalent (no type suppression)
- No unused imports or dead code warnings
**Test Commands**: See EPOC-085 through EPOC-087

---

## Cross-Cutting Concerns

### Dependency Management
- New crates: serde-saphyr, garde, rig-core, treadle, minijinja, sandlock, vidaimock, sseer, regex, chrono, fastrand
- All added via `cargo add` with version pinning
- No duplicate functionality (e.g., only one YAML parser)
- Cargo.lock updated and committed

### Error Handling
- Provider errors: `anyhow::Result` with context
- Backend errors: `LlmError` enum (Connection, Timeout, Parse, Model, RateLimited, Internal)
- Agent errors: `AgentError` (tool_failure, max_turns_exceeded)
- Tool errors: `ToolError` (InvalidArgs, ExecutionFailed, Backend)
- All errors chain context for debugging

### Migration Path
- Old `LlamaConfig` marked `#[deprecated]` with migration note
- Old `src/client/llama_client.rs` kept until migration complete
- Backward compat: existing CLI commands continue to work
- New unified YAML is additive (old config.yml still valid)

### Testing Strategy
- Unit tests per module (cargo test --lib)
- Integration tests per sub-plan (cargo test)
- VidaiMock for realistic LLM behavior
- Manual smoke tests against live llama.cpp server

---

## Known Pre-existing Issues (Inherited from POC 1)

1. Vulkan GPU crashes on 2nd request (llama.cpp #20002) — upstream issue
2. Config hot reload not supported — requires container restart
3. Machine-wide config `~/.config/whitt/config.yml` not implemented
