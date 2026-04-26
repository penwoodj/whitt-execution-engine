# QA Areas — Extended POC

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Date**: 2026-04-26
**Status**: 🟢 COMPLETE — 19/20 PASS, 1 PARTIAL (commit fa5e6f3)

---

## Summary Table

| # | QA Area | Schema Ref | Status | Priority |
|---|---------|------------|--------|----------|
| 1 | Provider Config Parsing (serde-saphyr) | Lines 27-57 (providers) | ✅ PASS | P0 |
| 2 | Provider Config Validation (garde) | Lines 27-57 (providers) | ✅ PASS | P0 |
| 3 | LlmBackend Trait | Lines 27-57 (providers) | ✅ PASS | P0 |
| 4 | LlamaCppVulkanBackend Implementation | Lines 56-57 (llama_cpp_with_vulkan) | ✅ PASS | P0 |
| 5 | Config Resolution Hierarchy | Lines 503-598 (workflow_execution_strategy) | ✅ PASS (Fixed) | P0 |
| 6 | Model Schema Parsing | Lines 64-158 (models) | ✅ PASS | P0 |
| 7 | Model Registry Lifecycle | Lines 64-158 (models) | ✅ PASS (Fixed) | P0 |
| 8 | Resource Management | Lines 74-88 (ram_allocation, max_allowed, min_allowed) | ✅ PASS | P1 |
| 9 | Template Interpolation (Minijinja) | Lines 726-740 (variable_interpolation) | ✅ PASS | P0 |
| 10 | ReAct Agent Tool Loop | Lines 196-497 (agentic_workflow steps) | ✅ PASS (Fixed) | P0 |
| 11 | Tool Definitions (6 tools) | Lines 606-676 (tool_permissions) | ✅ PASS | P0 |
| 12 | Step Executor with Retry | Lines 245-268 (retry configuration) | ✅ PASS | P0 |
| 13 | SSE Streaming | Lines 196-497 (agentic_workflow execution) | ✅ PASS (Fixed) | P1 |
| 14 | Workflow Persistence (Treadle) | Lines 568-583 (checkpointing) | ✅ PASS (Fixed) | P1 |
| 15 | Tool Sandboxing (Landlock/namespace) | Lines 606-676 (tool_permissions restrictions) | ⚠️ PARTIAL | P2 |
| 16 | Mock Testing (HTTP mock server) | N/A (implementation detail) | ✅ PASS (Fixed) | P1 |
| 17 | Schema Version Validation | Lines 14-20, 803-805 (schema_version, min_schema_version) | ✅ PASS (Fixed) | P0 |
| 18 | End-to-End Integration | Lines 14-805 (full schema) | ✅ PASS (Fixed) | P0 |
| 19 | CLI Integration | Lines 196-497 (agentic_workflow) | ✅ PASS (Fixed) | P0 |
| 20 | Build Hygiene (warnings/errors/clippy) | N/A (code quality) | ✅ PASS | P0 |

---

## Area Details

### Area 1: Provider Config Parsing (serde-saphyr)

**Schema Ref**: Lines 27-57 (providers section)
**Files**: `src/config/provider.rs`
**Criteria**:
- Parse `providers` section from unified YAML
- All structs deserialize correctly: ProviderConfig, LlmStudioProvider, OllamaProvider, LlamaCppVulkanProvider, ConnectionConfig, HostingConfig, GpuAllocation, CpuFallback, RequestsConfig, RetryConfig
- Missing optional fields use correct defaults (e.g., host=localhost, port varies by provider)
- serde-saphyr handles merge keys correctly
- No hardcoded defaults in implementation — all from YAML or struct default functions
**Test Commands**: See `QA-TEST-PROCEDURES-EXTENDED-POC.md` EPOC-001 through EPOC-005

### Area 2: Provider Config Validation (garde)

**Schema Ref**: Lines 27-57 (providers section)
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

**Schema Ref**: Lines 27-57 (providers section - backend interface requirements)
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

**Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
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

**Schema Ref**: Lines 503-598 (workflow_execution_strategy) + Lines 27-57 (providers) + Lines 64-158 (models) + Lines 196-497 (agentic_workflow)
**Criteria**:
- Resolution order: providers section → per-model overrides → step-level overrides → defaults
- CLI args still override everything (backward compat from POC 1)
- Missing provider section uses sensible defaults
- Override chain produces correct final values
**Test Commands**: See EPOC-019 through EPOC-021

### Area 6: Model Schema Parsing

**Schema Ref**: Lines 64-158 (models section)
**Files**: `src/model/schema.rs`
**Criteria**:
- Parse `models` section from unified YAML
- ModelsConfig: global_config_path, default_router, models HashMap
- ModelSpec: name, host, ram_allocation, max_allowed, min_allowed, model_memory, execution, thinking
- ResourceLimit: percentage ("13%") and absolute ("3.7GB")
- All enum variants parse correctly (CacheSize, KvCacheQuantization, AttentionContext, RouterStrategy)
- ThinkingConfig optional (presence = enabled)
**Test Commands**: See EPOC-022 through EPOC-026

### Area 7: Model Registry Lifecycle

**Schema Ref**: Lines 64-158 (models section - lifecycle state)
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

**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed, min_allowed) + Lines 517-526 (memory management)
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

**Schema Ref**: Lines 726-740 (variable_interpolation syntax)
**Files**: `src/model/interpolation.rs`
**Criteria**:
- `${models.model_name}` resolved at parse time (not runtime)
- TemplateInterpolator converts `${...}` to `{{...}}` for Minijinja
- parse_var_ref extracts model name from `${models.primary-analyzer}`
- Unknown variable references produce clear errors
- Does NOT support `{{step.name.output}}` (runtime, deferred to Phase 2)
**Test Commands**: See EPOC-037 through EPOC-040

### Area 10: ReAct Agent Tool Loop

**Schema Ref**: Lines 196-497 (agentic_workflow steps - generative_agent step pattern)
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

**Schema Ref**: Lines 606-676 (tool_permissions - file, web, shell operations)
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

**Schema Ref**: Lines 245-268 (retry configuration in agentic_workflow) + Lines 537-556 (error_handling in workflow_execution_strategy)
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

**Schema Ref**: Lines 196-497 (agentic_workflow execution - streaming responses)
**Files**: `src/agent/streaming.rs`
**Criteria**:
- StreamingResponse wraps backend chat_stream()
- StreamChunk: text + done flag
- collect() consumes stream into string
- Done chunk (empty text) terminates stream
- Implements futures::Stream trait
**Test Commands**: See EPOC-059 through EPOC-061

### Area 14: Workflow Persistence (Treadle)

**Schema Ref**: Lines 568-583 (checkpointing in workflow_execution_strategy)
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

### Area 15: Tool Sandboxing (Landlock/namespace)

**Schema Ref**: Lines 606-676 (tool_permissions - allowed_paths, forbidden_paths, allowed_commands, etc.)
**Files**: `src/agent/sandbox.rs`
**Criteria**:
- ToolSandbox configures sandboxing using Landlock Linux LSM or namespace-based approach
- Default: 512MB memory, 30s CPU time, network restricted
- Filesystem access controlled via allowed_paths and forbidden_paths
- Network access controlled via allowed_domains and forbidden_domains
- Shell command access controlled via allowed_commands and forbidden_commands
- Per-tool sandboxing (not global)
**Note**: sandlock crate does NOT exist. Sandbox implementation will use Landlock LSM or namespace-based approach.
**Test Commands**: See EPOC-067 through EPOC-070

### Area 16: Mock Testing (HTTP mock server)

**Schema Ref**: N/A (implementation detail for testing)
**Files**: `tests/mock_integration_test.rs`
**Criteria**:
- Mock server starts with configurable behavior
- MockScenario::SlowStreaming: 5s TTFT, 1 token/sec
- MockScenario::DroppedConnections: 10% drop rate
- MockScenario::RateLimited: 60 RPM limit
- MockScenario::MalformedResponses: 1% malformed
- Agent handles all scenarios gracefully
**Note**: vidaimock crate does NOT exist. Mock testing will use a simple HTTP mock server pattern.
**Test Commands**: See EPOC-071 through EPOC-074

### Area 17: Schema Version Validation

**Schema Ref**: Lines 14-20 (workflow metadata) + Lines 803-805 (schema_version, min_schema_version)
**Criteria**:
- All YAML files must include `schema_version: "2.0.0"`
- schema_version validated at load time
- Incompatible versions rejected with clear error
- Semver format: MAJOR.MINOR.PATCH
- min_schema_version checked against current supported version
**Test Commands**: See EPOC-075 through EPOC-077

### Area 18: End-to-End Integration

**Schema Ref**: Lines 14-805 (full unified workflow schema)
**Criteria**:
- Full workflow: load YAML → parse → resolve config → load model → execute step → stream response
- Provider config → Model registry → Agent execution chain works
- No hardcoded values in implementation
- Template interpolation resolves before agent execution
- Step-level model overrides applied correctly
**Test Commands**: See EPOC-078 through EPOC-081

### Area 19: CLI Integration

**Schema Ref**: Lines 196-497 (agentic_workflow) + Lines 14-20 (workflow_id, name, description)
**Files**: `src/bin/whitt.rs`
**Criteria**:
- CLI uses new ReactAgent + StepExecutor
- Existing commands still work (backward compat)
- New commands for workflow execution added
- Unified YAML loaded and parsed on startup
- Deprecation warnings for old flat config
**Test Commands**: See EPOC-082 through EPOC-084

### Area 20: Build Hygiene

**Schema Ref**: N/A (code quality, not schema-specific)
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
- New crates: serde-saphyr, garde, rig-core, treadle, minijinja, sseer, regex, chrono, fastrand, async-trait, serde-saphyr, serde-saphyr
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
- Mock server for realistic LLM behavior
- Manual smoke tests against live llama.cpp server

---

## Known Pre-existing Issues (Inherited from POC 1)

1. Vulkan GPU crashes on 2nd request (llama.cpp #20002) — upstream issue
2. Config hot reload not supported — requires container restart
3. Machine-wide config `~/.config/whitt/config.yml` not implemented
