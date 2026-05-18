# QA Test Procedures — Extended POC

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Date**: 2026-04-26
**Status**: 🔴 NOT STARTED — Pre-implementation test procedure definition

> These test procedures define how to verify each QA area from `QA-AREAS-EXTENDED-POC.md`.
> Each test has a unique ID (EPOC-NNN), command, expected result, and acceptance criteria.
> All tests validate against the unified workflow schema (Lines 14-805).

---

## Section 1: Provider Config Parsing (serde-saphyr)

### EPOC-001: Parse Minimal Provider Config

**Area**: Area 1
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_minimal_provider_config -- --nocapture
```
**Setup YAML**:
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
      connection_timeout_secs: 30
    hosting:
      max_concurrent_models: 1
    requests:
      max_concurrent_requests: 1
      request_timeout_secs: 600
```
**Expected**: ProviderConfig parses successfully. `llama_cpp_with_vulkan` field is Some. ConnectionConfig defaults: host=localhost, port=8080.
**Pass Criteria**: ✅ Test passes, no panic, correct field values.

---

### EPOC-002: Parse Full Provider Config with All Fields

**Area**: Area 1
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_full_provider_config -- --nocapture
```
**Setup YAML**:
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      host: 192.168.1.100
      port: 9090
      connection_timeout_secs: 60
    hosting:
      max_concurrent_models: 2
      model_offload_timeout_secs: 120
      gpu_allocation:
        vram_per_model_mb: 8192
      cpu_fallback:
        cpu_cores_per_model: 8
    requests:
      max_concurrent_requests: 4
      request_timeout_secs: 1200
      queue_timeout_secs: 600
      rate_limit_per_minute: 120
      retry:
        max_retries: 5
        backoff: exponential
        initial_delay: "2s"
        max_delay: "60s"
        multiplier: 3.0
        jitter: false
```
**Expected**: All fields populated correctly. Retry config parsed with exponential backoff, multiplier=3.0, jitter=false.
**Pass Criteria**: ✅ All field values match YAML exactly.

---

### EPOC-003: Parse Provider Config with Missing Optional Fields

**Area**: Area 1
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_minimal_provider_defaults -- --nocapture
```
**Setup YAML**:
```yaml
providers:
  llama_cpp_with_vulkan:
    config: {}
```
**Expected**: All defaults applied:
- host=localhost, port=8080, connection_timeout_secs=30
- max_concurrent_models=1, model_offload_timeout_secs=60
- gpu_allocation=None, cpu_fallback=None
- max_concurrent_requests=1, request_timeout_secs=600
- queue_timeout_secs=300, rate_limit_per_minute=60
- retry=None
**Pass Criteria**: ✅ All defaults match expected values.

---

### EPOC-004: Parse Retry Config with All Backoff Strategies

**Area**: Area 1
**Schema Ref**: Lines 45-51 (retry configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_retry_config_with_backoff -- --nocapture
```
**Setup**: Three YAML variants — `backoff: exponential`, `backoff: linear`, `backoff: fixed`
**Expected**: BackoffStrategy enum parses correctly for all 3 variants.
**Pass Criteria**: ✅ Each variant matches expected enum value.

---

### EPOC-005: serde-saphyr Handles Merge Keys

**Area**: Area 1
**Schema Ref**: Lines 27-57 (providers section - supports merge keys)
**Test Type**: Unit
**Command**:
```bash
cargo test serde_saphyr_merge_keys -- --nocapture
```
**Setup YAML with merge key (`<<: *default`)**:
```yaml
defaults: &defaults
  host: localhost
  port: 8080

providers:
  llama_cpp_with_vulkan:
    config:
      <<: *defaults
      connection_timeout_secs: 60
```
**Expected**: Merged config has host=localhost, port=8080 from anchor + connection_timeout_secs=60 override.
**Pass Criteria**: ✅ Merge key resolved correctly.

---

## Section 2: Provider Config Validation (garde)

### EPOC-006: Garde Rejects Invalid Port (>65535)

**Area**: Area 2
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test garde_validation_rejects_invalid_port -- --nocapture
```
**Setup YAML**:
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      port: 99999
```
**Expected**: Validation error: port exceeds 65535 range.
**Pass Criteria**: ✅ `Result::Err` returned with descriptive message.

---

### EPOC-007: Garde Rejects Negative Timeout

**Area**: Area 2
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test garde_validation_rejects_negative_timeout -- --nocapture
```
**Setup YAML**:
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      connection_timeout_secs: 0
```
**Expected**: Validation error: connection_timeout_secs must be >= 1.
**Pass Criteria**: ✅ `Result::Err` returned.

---

### EPOC-008: Garde Validates All Range Constraints

**Area**: Area 2
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test garde_validates_all_ranges -- --nocapture
```
**Setup**: YAML with multiplier=0.5 (below min 1.0), max_concurrent_models=0, vram_per_model_mb=0
**Expected**: All invalid values rejected.
**Pass Criteria**: ✅ Multiple validation errors reported.

---

## Section 3: LlmBackend Trait

### EPOC-009: LlmBackend Trait Is Object-Safe

**Area**: Area 3
**Schema Ref**: Lines 27-57 (providers section - backend interface)
**Test Type**: Unit (compile-time)
**Command**:
```bash
cargo check --lib 2>&1 | grep -i "object.*safe\|dyn.*LlmBackend"
```
**Expected**: No errors. Trait compiles and can be used as `dyn LlmBackend`.
**Pass Criteria**: ✅ `cargo check` passes.

---

### EPOC-010: LlmError Enum Covers All Variants

**Area**: Area 3
**Schema Ref**: Lines 27-57 (providers section - error handling)
**Test Type**: Unit
**Command**:
```bash
cargo test llm_error_variants -- --nocapture
```
**Expected**: All 6 variants constructible: Connection, Timeout, Parse, Model, RateLimited, Internal.
**Pass Criteria**: ✅ Each variant displays correct error message.

---

### EPOC-011: BackendCapabilities and HealthStatus

**Area**: Area 3
**Schema Ref**: Lines 27-57 (providers section)
**Test Type**: Unit
**Command**:
```bash
cargo test backend_types -- --nocapture
```
**Expected**: BackendCapabilities has streaming/tools/function_calling. HealthStatus has Healthy/Degraded/Unhealthy.
**Pass Criteria**: ✅ All fields and variants accessible.

---

## Section 4: LlamaCppVulkanBackend Implementation

### EPOC-012: Backend Constructs from Config

**Area**: Area 4
**Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
**Test Type**: Unit
**Command**:
```bash
cargo test llama_vulkan_backend_new -- --nocapture
```
**Expected**: LlamaCppVulkanBackend::new(config, model) creates instance with correct base_url, timeout.
**Pass Criteria**: ✅ base_url = "http://localhost:8080", timeout = 600s.

---

### EPOC-013: Backend Chat Completion (Live)

**Area**: Area 4
**Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
**Test Type**: Integration (requires Docker)
**Command**:
```bash
# Ensure server running:
docker ps | grep whitt-llama-server
# Test:
cargo test llama_vulkan_chat_integration -- --nocapture --ignored
```
**Expected**: ChatCompletionResponse returned with content.
**Pass Criteria**: ✅ Response has choices[0].message.content non-empty.

---

### EPOC-014: Backend Health Check (Live)

**Area**: Area 4
**Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
**Test Type**: Integration (requires Docker)
**Command**:
```bash
cargo test llama_vulkan_health_check -- --nocapture --ignored
```
**Expected**: health_check() returns HealthStatus::Healthy when server running.
**Pass Criteria**: ✅ Returns Healthy.

---

### EPOC-015: Backend List Models (Live)

**Area**: Area 4
**Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
**Test Type**: Integration (requires Docker)
**Command**:
```bash
cargo test llama_vulkan_list_models -- --nocapture --ignored
```
**Expected**: list_models() returns non-empty Vec of model IDs.
**Pass Criteria**: ✅ Vec contains expected model names.

---

### EPOC-016: Retry Delay Calculation — Exponential

**Area**: Area 4
**Schema Ref**: Lines 45-51 (retry configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test retry_delay_exponential -- --nocapture
```
**Expected**: attempt=1: 1s, attempt=2: 2s, attempt=3: 4s (multiplier=2.0)
**Pass Criteria**: ✅ Delays match exponential formula.

---

### EPOC-017: Retry Delay with Jitter

**Area**: Area 4
**Schema Ref**: Lines 45-51 (retry configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test retry_delay_jitter -- --nocapture
```
**Expected**: With jitter=true, delay is base + random(0..base/10).
**Pass Criteria**: ✅ Delay within expected range (base to base + base/10).

---

### EPOC-018: Duration Parsing

**Area**: Area 4
**Schema Ref**: Lines 45-51 (retry configuration - duration strings)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_duration -- --nocapture
```
**Expected**: "1s" → 1s, "30s" → 30s, "1m" → error or 60s (depending on implementation).
**Pass Criteria**: ✅ Valid durations parsed, invalid rejected.

---

## Section 5: Config Resolution Hierarchy

### EPOC-019: Provider Defaults → Per-Model Override

**Area**: Area 5
**Schema Ref**: Lines 27-57 (providers) + Lines 64-158 (models)
**Test Type**: Unit
**Command**:
```bash
cargo test config_resolution_provider_to_model -- --nocapture
```
**Expected**: Provider sets request_timeout_secs=600, per-model overrides to 1200 → final=1200.
**Pass Criteria**: ✅ Override wins.

---

### EPOC-020: Step-Level Override Wins Over Model Config

**Area**: Area 5
**Schema Ref**: Lines 64-158 (models) + Lines 196-497 (agentic_workflow steps)
**Test Type**: Unit
**Command**:
```bash
cargo test config_resolution_step_override -- --nocapture
```
**Expected**: Model sets max_turns=10, step overrides max_turns=5 → final=5.
**Pass Criteria**: ✅ Step-level override wins.

---

### EPOC-021: Full Resolution Chain

**Area**: Area 5
**Schema Ref**: Lines 27-57 (providers) + Lines 64-158 (models) + Lines 196-497 (agentic_workflow)
**Test Type**: Unit
**Command**:
```bash
cargo test config_resolution_full_chain -- --nocapture
```
**Expected**: providers base → model override → step override → CLI flag → final values.
**Pass Criteria**: ✅ Each level overrides previous correctly.

---

## Section 6: Model Schema Parsing

### EPOC-022: Parse ModelSpec with All Fields

**Area**: Area 6
**Schema Ref**: Lines 68-158 (model specification)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_model_spec -- --nocapture
```
**Expected**: Full ModelSpec parsed with name, host, ram_allocation, max_allowed, min_allowed, model_memory, execution, thinking.
**Pass Criteria**: ✅ All fields populated.

---

### EPOC-023: Parse ResourceLimit Variants

**Area**: Area 6
**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed, min_allowed)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_resource_limit -- --nocapture
```
**Expected**: "13%" → Percentage, "3.7GB" → Absolute, "512MB" → Absolute.
**Pass Criteria**: ✅ Correct variant for each format.

---

### EPOC-024: Parse ExecutionConfig with Timeouts

**Area**: Area 6
**Schema Ref**: Lines 95-102 (execution configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_execution_config -- --nocapture
```
**Expected**: timeout.load_into_memory="45s", max_turns=10, stop_on_tool_failure=false, accumulate_tool_results=true.
**Pass Criteria**: ✅ All fields correct.

---

### EPOC-025: ThinkingConfig Optional

**Area**: Area 6
**Schema Ref**: Lines 104-107 (thinking configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test thinking_config_optional -- --nocapture
```
**Expected**: ModelSpec without thinking field → thinking=None. With thinking → budget_tokens=4096, capture_in_output=true.
**Pass Criteria**: ✅ Optional handling correct.

---

### EPOC-026: RouterStrategy Variants

**Area**: Area 6
**Schema Ref**: Line 66 (default_router)
**Test Type**: Unit
**Command**:
```bash
cargo test router_strategy_variants -- --nocapture
```
**Expected**: "automatic" → RouterStrategy::Automatic, "manual" → RouterStrategy::Manual.
**Pass Criteria**: ✅ Both variants parse.

---

## Section 7: Model Registry Lifecycle

### EPOC-027: Register Models from Config

**Area**: Area 7
**Schema Ref**: Lines 64-158 (models section)
**Test Type**: Unit
**Command**:
```bash
cargo test model_registry_new -- --nocapture
```
**Expected**: ModelRegistry created from ModelsConfig. All models start as ModelState::Unloaded.
**Pass Criteria**: ✅ list_models() returns all model IDs.

---

### EPOC-028: Load Model State Transition

**Area**: Area 7
**Schema Ref**: Lines 64-158 (models section)
**Test Type**: Unit
**Command**:
```bash
cargo test model_registry_load -- --nocapture
```
**Expected**: Unloaded → Loading → Loaded. Double load returns early (already loaded).
**Pass Criteria**: ✅ State transitions correct, no errors.

---

### EPOC-029: Unload Model State Transition

**Area**: Area 7
**Schema Ref**: Lines 64-158 (models section)
**Test Type**: Unit
**Command**:
```bash
cargo test model_registry_unload -- --nocapture
```
**Expected**: Loaded → Unloading → Unloaded. Double unload returns early (already unloaded).
**Pass Criteria**: ✅ State transitions correct.

---

### EPOC-030: Load Unknown Model

**Area**: Area 7
**Schema Ref**: Lines 64-158 (models section)
**Test Type**: Unit
**Command**:
```bash
cargo test model_registry_load_unknown -- --nocapture
```
**Expected**: load_model("nonexistent") returns error: "Model nonexistent not found".
**Pass Criteria**: ✅ Error returned with model name.

---

### EPOC-031: Concurrent Access (RwLock)

**Area**: Area 7
**Schema Ref**: Lines 64-158 (models section)
**Test Type**: Unit
**Command**:
```bash
cargo test model_registry_concurrent -- --nocapture
```
**Expected**: Multiple concurrent load/unload operations don't deadlock or panic.
**Pass Criteria**: ✅ All operations complete.

---

## Section 8: Resource Management

### EPOC-032: Parse Percentage ResourceLimit

**Area**: Area 8
**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed, min_allowed)
**Test Type**: Unit
**Command**:
```bash
cargo test resource_parse_percentage -- --nocapture
```
**Expected**: "13%" parsed to numeric value based on total resources.
**Pass Criteria**: ✅ Correct percentage calculation.

---

### EPOC-033: Parse Absolute ResourceLimit (GB/MB)

**Area**: Area 8
**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed, min_allowed)
**Test Type**: Unit
**Command**:
```bash
cargo test resource_parse_absolute -- --nocapture
```
**Expected**: "3.7GB" → 3788 MB, "512MB" → 512 MB.
**Pass Criteria**: ✅ Correct absolute values.

---

### EPOC-034: Reject Insufficient RAM

**Area**: Area 8
**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed) + Lines 517-526 (memory management)
**Test Type**: Unit
**Command**:
```bash
cargo test resource_check_insufficient_ram -- --nocapture
```
**Expected**: check_resources() returns error when model requires more RAM than available.
**Pass Criteria**: ✅ Error message includes required vs available amounts.

---

### EPOC-035: Reject Insufficient VRAM

**Area**: Area 8
**Schema Ref**: Lines 74-88 (ram_allocation, max_allowed) + Lines 517-526 (memory management)
**Test Type**: Unit
**Command**:
```bash
cargo test resource_check_insufficient_vram -- --nocapture
```
**Expected**: check_resources() returns error when model requires more VRAM than available.
**Pass Criteria**: ✅ VRAM check works.

---

### EPOC-036: Release Resources After Unload

**Area**: Area 8
**Schema Ref**: Lines 517-526 (memory management)
**Test Type**: Unit
**Command**:
```bash
cargo test resource_release -- --nocapture
```
**Expected**: After unload, resources freed. Next model can load.
**Pass Criteria**: ✅ Allocation tracking correct.

---

## Section 9: Template Interpolation (Minijinja)

### EPOC-037: Interpolate Model Reference

**Area**: Area 9
**Schema Ref**: Lines 726-740 (variable_interpolation syntax)
**Test Type**: Unit
**Command**:
```bash
cargo test interpolate_model_ref -- --nocapture
```
**Expected**: `"Model: ${models.primary-analyzer}"` → `"Model: llama-3.2-3b-instruct"` (with vars map).
**Pass Criteria**: ✅ Variable substituted.

---

### EPOC-038: Parse Variable Reference

**Area**: Area 9
**Schema Ref**: Lines 726-740 (variable_interpolation syntax)
**Test Type**: Unit
**Command**:
```bash
cargo test parse_var_ref -- --nocapture
```
**Expected**: `"${models.primary-analyzer}"` → `Some(("models", "primary-analyzer"))`.
**Pass Criteria**: ✅ Extracted correctly.

---

### EPOC-039: Unknown Variable Produces Error

**Area**: Area 9
**Schema Ref**: Lines 726-740 (variable_interpolation syntax)
**Test Type**: Unit
**Command**:
```bash
cargo test interpolate_unknown_var -- --nocapture
```
**Expected**: `"${models.nonexistent}"` with empty vars → Minijinja error.
**Pass Criteria**: ✅ Error returned, not silently empty.

---

### EPOC-040: Runtime Syntax Not Supported

**Area**: Area 9
**Schema Ref**: Lines 726-740 (variable_interpolation - ${} vs {{}})
**Test Type**: Unit
**Command**:
```bash
cargo test interpolate_runtime_not_supported -- --nocapture
```
**Expected**: `"{{step.name.output}}"` treated as literal text (no runtime interpolation in POC).
**Pass Criteria**: ✅ Returns as-is or produces clear "not supported" error.

---

## Section 10: ReAct Agent Tool Loop

### EPOC-041: Single Turn — No Tool Call

**Area**: Area 10
**Schema Ref**: Lines 196-497 (agentic_workflow steps - generative_agent pattern)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test react_agent_single_turn -- --nocapture
```
**Expected**: Agent calls LLM, gets response with no tool call, returns response as final answer.
**Pass Criteria**: ✅ completed=true, turn_count=1.

---

### EPOC-042: Multi-Turn with Tool Call

**Area**: Area 10
**Schema Ref**: Lines 196-497 (agentic_workflow steps - generative_agent pattern)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test react_agent_multi_turn -- --nocapture
```
**Expected**: Agent calls LLM → gets tool call → executes tool → appends result → calls LLM again → returns final answer.
**Pass Criteria**: ✅ turn_count >= 2, completed=true.

---

### EPOC-043: Max Turns Enforced

**Area**: Area 10
**Schema Ref**: Lines 95-102 (execution.max_turns)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test react_agent_max_turns -- --nocapture
```
**Expected**: With max_turns=3 and infinite tool loop, agent stops at turn 3 with error.
**Pass Criteria**: ✅ Error: "Max turns (3) exceeded".

---

### EPOC-044: final_answer Terminates Loop

**Area**: Area 10
**Schema Ref**: Lines 196-497 (agentic_workflow steps)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test react_agent_final_answer -- --nocapture
```
**Expected**: Agent calls final_answer tool → loop terminates immediately.
**Pass Criteria**: ✅ completed=true after final_answer, no further turns.

---

### EPOC-045: Tool Result Appended to Messages

**Area**: Area 10
**Schema Ref**: Lines 196-497 (agentic_workflow steps)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test react_agent_message_history -- --nocapture
```
**Expected**: After tool execution, messages include: system prompt, user prompt, assistant response, tool result.
**Pass Criteria**: ✅ Message history grows correctly.

---

## Section 11: Tool Definitions

### EPOC-046: model_list Tool

**Area**: Area 11
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_model_list -- --nocapture
```
**Expected**: Returns JSON array of model IDs from registry.
**Pass Criteria**: ✅ Valid JSON, contains registered models.

---

### EPOC-047: model_load Tool

**Area**: Area 11
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_model_load -- --nocapture
```
**Expected**: Loads model by ID, returns "Model X loaded successfully".
**Pass Criteria**: ✅ Model transitions to Loaded state.

---

### EPOC-048: model_unload Tool

**Area**: Area 11
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_model_unload -- --nocapture
```
**Expected**: Unloads model by ID, returns "Model X unloaded successfully".
**Pass Criteria**: ✅ Model transitions to Unloaded state.

---

### EPOC-049: chat Tool

**Area**: Area 11
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test tool_chat -- --nocapture
```
**Expected**: Sends ChatCompletionRequest, returns response content.
**Pass Criteria**: ✅ Returns non-empty string.

---

### EPOC-050: file_read Tool

**Area**: Area 11
**Schema Ref**: Lines 608-621 (file_operations)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_file_read -- --nocapture
```
**Expected**: Reads file from workspace_path, returns contents.
**Pass Criteria**: ✅ Returns file contents as string.

---

### EPOC-051: file_read Tool — Path Traversal Blocked

**Area**: Area 11
**Schema Ref**: Lines 608-621 (file_operations - allowed_paths, forbidden_paths)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_file_read_traversal -- --nocapture
```
**Expected**: `path: "../../../etc/passwd"` rejected or reads only within workspace.
**Pass Criteria**: ✅ Cannot escape workspace boundary.

---

### EPOC-052: final_answer Tool

**Area**: Area 11
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit
**Command**:
```bash
cargo test tool_final_answer -- --nocapture
```
**Expected**: Returns answer string unchanged.
**Pass Criteria**: ✅ Output matches input.

---

## Section 12: Step Executor with Retry

### EPOC-053: Execute Step — Success on First Try

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration) + Lines 196-497 (agentic_workflow steps)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_success_first_try -- --nocapture
```
**Expected**: StepResult.success=true, attempts=1.
**Pass Criteria**: ✅ No retries needed.

---

### EPOC-054: Execute Step — Retry and Succeed

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_retry_success -- --nocapture
```
**Expected**: First 2 attempts fail, 3rd succeeds. StepResult.success=true, attempts=3.
**Pass Criteria**: ✅ Retry count correct.

---

### EPOC-055: Execute Step — All Retries Exhausted

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_all_retries_exhausted -- --nocapture
```
**Expected**: All attempts fail. StepResult.success=false, attempts=max_retries+1.
**Pass Criteria**: ✅ Failure recorded with last error.

---

### EPOC-056: Exponential Backoff Delays

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration - backoff strategies)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_exponential_backoff -- --nocapture
```
**Expected**: Delay between retries: 1s, 2s, 4s (base=1s, multiplier=2.0).
**Pass Criteria**: ✅ Delays increase exponentially.

---

### EPOC-057: Linear and Fixed Backoff

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration - backoff strategies)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_linear_fixed_backoff -- --nocapture
```
**Expected**: Linear: 1s, 2s, 3s. Fixed: 1s, 1s, 1s.
**Pass Criteria**: ✅ Each strategy produces correct delays.

---

### EPOC-058: Default Retry Config

**Area**: Area 12
**Schema Ref**: Lines 245-268 (retry configuration - step defaults)
**Test Type**: Unit
**Command**:
```bash
cargo test executor_default_retry -- --nocapture
```
**Expected**: StepConfig with retry=None uses default: max_retries=3, exponential, jitter=true.
**Pass Criteria**: ✅ Default applied correctly.

---

## Section 13: SSE Streaming

### EPOC-059: Streaming Collect to String

**Area**: Area 13
**Schema Ref**: Lines 196-497 (agentic_workflow execution - streaming)
**Test Type**: Unit (with mock backend)
**Command**:
```bash
cargo test streaming_collect -- --nocapture
```
**Expected**: StreamingResponse.collect() returns complete text.
**Pass Criteria**: ✅ Full text assembled from chunks.

---

### EPOC-060: Streaming Done Chunk Terminates

**Area**: Area 13
**Schema Ref**: Lines 196-497 (agentic_workflow execution - streaming)
**Test Type**: Unit
**Command**:
```bash
cargo test streaming_done_chunk -- --nocapture
```
**Expected**: Empty text chunk → StreamChunk.done=true → stream terminates.
**Pass Criteria**: ✅ No infinite loop on done.

---

### EPOC-061: Stream Implements futures::Stream

**Area**: Area 13
**Schema Ref**: Lines 196-497 (agentic_workflow execution - streaming)
**Test Type**: Compile-time
**Command**:
```bash
cargo check --lib
```
**Expected**: StreamingResponse implements futures::Stream.
**Pass Criteria**: ✅ Compiles without error.

---

## Section 14: Workflow Persistence (Treadle)

### EPOC-062: Save and Load Checkpoint

**Area**: Area 14
**Schema Ref**: Lines 568-583 (checkpointing configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test persistence_save_load -- --nocapture
```
**Expected**: save_checkpoint() stores WorkflowState, load_checkpoint() returns same state.
**Pass Criteria**: ✅ Round-trip preserves all fields.

---

### EPOC-063: List Checkpoints for Workflow

**Area**: Area 14
**Schema Ref**: Lines 568-583 (checkpointing configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test persistence_list_checkpoints -- --nocapture
```
**Expected**: After saving 3 checkpoints, list_checkpoints() returns 3 step names.
**Pass Criteria**: ✅ All step names returned.

---

### EPOC-064: Clear Checkpoints

**Area**: Area 14
**Schema Ref**: Lines 568-583 (checkpointing configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test persistence_clear_checkpoints -- --nocapture
```
**Expected**: clear_checkpoints() removes all. load_checkpoint() returns None.
**Pass Criteria**: ✅ All checkpoints removed.

---

### EPOC-065: Checkpoint Key Format

**Area**: Area 14
**Schema Ref**: Lines 568-583 (checkpointing configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test persistence_key_format -- --nocapture
```
**Expected**: Key = "workflow-123.step-1" (workflow_id.step_name).
**Pass Criteria**: ✅ Correct key format.

---

### EPOC-066: Load Nonexistent Checkpoint

**Area**: Area 14
**Schema Ref**: Lines 568-583 (checkpointing configuration)
**Test Type**: Unit
**Command**:
```bash
cargo test persistence_load_nonexistent -- --nocapture
```
**Expected**: load_checkpoint("nonexistent") returns Ok(None).
**Pass Criteria**: ✅ No error, just None.

---

## Section 15: Tool Sandboxing (Landlock/namespace)

### EPOC-067: Default Sandbox Configuration

**Area**: Area 15
**Schema Ref**: Lines 606-676 (tool_permissions - restrictions)
**Test Type**: Unit
**Command**:
```bash
cargo test sandbox_default_config -- --nocapture
```
**Expected**: ToolSandbox created with Landlock=true or namespace-based, memory=512MB, CPU=30s, network_restricted=true.
**Pass Criteria**: ✅ Config matches defaults.
**Note**: sandlock crate does NOT exist. Uses Landlock LSM or namespace-based approach.

---

### EPOC-068: Filesystem Access Restriction

**Area**: Area 15
**Schema Ref**: Lines 608-621 (file_operations - allowed_paths, forbidden_paths)
**Test Type**: Unit
**Command**:
```bash
cargo test sandbox_filesystem_restriction -- --nocapture
```
**Expected**: Tool with allowed_paths=["./workspace/src"] can read from src/ but not from /etc/.
**Pass Criteria**: ✅ Access restricted to allowed paths.

---

### EPOC-069: Network Access Restriction

**Area**: Area 15
**Schema Ref**: Lines 623-635 (web_operations - allowed_domains, forbidden_domains)
**Test Type**: Unit
**Command**:
```bash
cargo test sandbox_network_restriction -- --nocapture
```
**Expected**: Tool with network_restricted=true cannot make HTTP requests.
**Pass Criteria**: ✅ Network blocked.

---

### EPOC-070: Sandboxed Tool Execution

**Area**: Area 15
**Schema Ref**: Lines 606-676 (tool_permissions)
**Test Type**: Unit
**Command**:
```bash
cargo test sandboxed_tool_execution -- --nocapture
```
**Expected**: SandboxedToolExecutor wraps tool in sandbox, returns result.
**Pass Criteria**: ✅ Tool executes within sandbox constraints.

---

## Section 16: Mock Testing (HTTP mock server)

### EPOC-071: Slow Streaming Scenario

**Area**: Area 16
**Schema Ref**: N/A (implementation detail)
**Test Type**: Integration
**Command**:
```bash
cargo test mock_slow_streaming -- --nocapture --ignored
```
**Expected**: Mock server simulates 5s TTFT, 1 token/sec. Agent waits for response.
**Pass Criteria**: ✅ Agent handles slow streaming without timeout.
**Note**: vidaimock crate does NOT exist. Uses simple HTTP mock server pattern.

---

### EPOC-072: Dropped Connections Scenario

**Area**: Area 16
**Schema Ref**: N/A (implementation detail)
**Test Type**: Integration
**Command**:
```bash
cargo test mock_dropped_connections -- --nocapture --ignored
```
**Expected**: 10% drop rate. Agent retries on connection drop.
**Pass Criteria**: ✅ Agent recovers from drops.
**Note**: vidaimock crate does NOT exist. Uses simple HTTP mock server pattern.

---

### EPOC-073: Rate Limiting Scenario

**Area**: Area 16
**Schema Ref**: N/A (implementation detail)
**Test Type**: Integration
**Command**:
```bash
cargo test mock_rate_limiting -- --nocapture --ignored
```
**Expected**: 60 RPM limit. Rapid requests get 429, agent backs off.
**Pass Criteria**: ✅ Agent respects rate limits.
**Note**: vidaimock crate does NOT exist. Uses simple HTTP mock server pattern.

---

### EPOC-074: Malformed Responses Scenario

**Area**: Area 16
**Schema Ref**: N/A (implementation detail)
**Test Type**: Integration
**Command**:
```bash
cargo test mock_malformed_responses -- --nocapture --ignored
```
**Expected**: 1% malformed SSE chunks. Agent handles parse errors gracefully.
**Pass Criteria**: ✅ No panic on malformed data.
**Note**: vidaimock crate does NOT exist. Uses simple HTTP mock server pattern.

---

## Section 17: Schema Version Validation

### EPOC-075: Accept Compatible Schema Version

**Area**: Area 17
**Schema Ref**: Lines 14-20 (workflow metadata) + Lines 803-805 (schema_version)
**Test Type**: Unit
**Command**:
```bash
cargo test schema_version_compatible -- --nocapture
```
**Expected**: schema_version="2.0.0" accepted. min_schema_version="2.0.0" compatible.
**Pass Criteria**: ✅ No error.

---

### EPOC-076: Reject Incompatible Schema Version

**Area**: Area 17
**Schema Ref**: Lines 14-20 (workflow metadata) + Lines 803-805 (schema_version)
**Test Type**: Unit
**Command**:
```bash
cargo test schema_version_incompatible -- --nocapture
```
**Expected**: schema_version="3.0.0" rejected (unsupported major version).
**Pass Criteria**: ✅ Error with version mismatch details.

---

### EPOC-077: Missing Schema Version Uses Default

**Area**: Area 17
**Schema Ref**: Lines 14-20 (workflow metadata) + Lines 803-805 (schema_version)
**Test Type**: Unit
**Command**:
```bash
cargo test schema_version_default -- --nocapture
```
**Expected**: YAML without schema_version field defaults to "2.0.0".
**Pass Criteria**: ✅ Default applied.

---

## Section 18: End-to-End Integration

### EPOC-078: Full Workflow Execution (Live)

**Area**: Area 18
**Schema Ref**: Lines 14-805 (full unified workflow schema)
**Test Type**: Integration (requires Docker + live server)
**Command**:
```bash
cargo test e2e_workflow_execution -- --nocapture --ignored
```
**Expected**: Load unified YAML → parse providers → parse models → create registry → load model → execute step → get response.
**Pass Criteria**: ✅ End-to-end completes with non-empty output.

---

### EPOC-079: Template Interpolation in Workflow

**Area**: Area 18
**Schema Ref**: Lines 726-740 (variable_interpolation) + Lines 196-497 (agentic_workflow steps)
**Test Type**: Unit
**Command**:
```bash
cargo test e2e_template_interpolation -- --nocapture
```
**Expected**: Step with `generative_entity: "${models.primary-analyzer}"` resolves to actual model ID.
**Pass Criteria**: ✅ Variable substituted before execution.

---

### EPOC-080: Step-Level Model Overrides Applied

**Area**: Area 18
**Schema Ref**: Lines 196-497 (agentic_workflow steps - model_overrides)
**Test Type**: Unit
**Command**:
```bash
cargo test e2e_model_overrides -- --nocapture
```
**Expected**: Step with `model_overrides: { max_turns: 5 }` limits agent turns to 5 (not 10 from model config).
**Pass Criteria**: ✅ Override takes effect.

---

### EPOC-081: No Hardcoded Values

**Area**: Area 18
**Schema Ref**: Lines 14-805 (full schema - all configurable)
**Test Type**: Audit (manual)
**Command**:
```bash
# Search for hardcoded defaults in implementation files:
grep -rn "localhost\|8080\|512\|0.7" src/backend/ src/model/ src/agent/ --include="*.rs" | grep -v "// default\|fn default\|#\[doc\|///"
```
**Expected**: No hardcoded connection params, model params, or timeout values. All from config.
**Pass Criteria**: ✅ Only default functions contain values.

---

## Section 19: CLI Integration

### EPOC-082: Existing Commands Still Work

**Area**: Area 19
**Schema Ref**: Lines 196-497 (agentic_workflow) + Lines 14-20 (workflow metadata)
**Test Type**: Manual (live)
**Command**:
```bash
./target/release/whitt model list
./target/release/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
./target/release/whitt chat "Hello" --no-stream
```
**Expected**: All existing POC 1 commands continue to work. No regression.
**Pass Criteria**: ✅ Commands execute, responses generated.

---

### EPOC-083: Workflow Execution Command

**Area**: Area 19
**Schema Ref**: Lines 196-497 (agentic_workflow)
**Test Type**: Manual (live)
**Command**:
```bash
./target/release/whitt workflow run configs/workflows/test-workflow.yml
```
**Expected**: Loads unified YAML, parses workflow, executes steps.
**Pass Criteria**: ✅ Workflow executes end-to-end.

---

### EPOC-084: Deprecation Warning for Old Config

**Area**: Area 19
**Schema Ref**: N/A (CLI behavior, not schema)
**Test Type**: Manual (live)
**Command**:
```bash
./target/release/whitt chat "Hello" --verbose --no-stream 2>&1 | grep -i deprecat
```
**Expected**: When using old config.yml format, deprecation warning logged.
**Pass Criteria**: ✅ Warning appears for old format.

---

## Section 20: Build Hygiene

### EPOC-085: Clean Build (Release, All Features)

**Area**: Area 20
**Schema Ref**: N/A (code quality, not schema)
**Test Type**: Automated
**Command**:
```bash
cargo build --release --all-features 2>&1 | grep -E "warning|error"
```
**Expected**: 0 warnings, 0 errors.
**Pass Criteria**: ✅ Empty output (clean build).

---

### EPOC-086: Clippy Clean

**Area**: Area 20
**Schema Ref**: N/A (code quality, not schema)
**Test Type**: Automated
**Command**:
```bash
cargo clippy --all-features -- -W clippy::all 2>&1 | grep -E "warning|error"
```
**Expected**: 0 warnings.
**Pass Criteria**: ✅ Empty output.

---

### EPOC-087: Unit Tests Pass

**Area**: Area 20
**Schema Ref**: N/A (code quality, not schema)
**Test Type**: Automated
**Command**:
```bash
cargo test --lib --all-features 2>&1
```
**Expected**: All tests pass. 0 failures.
**Pass Criteria**: ✅ `test result: ok`.

---

## Test Execution Order

**Phase 1: Provider Config (Lines 27-57)**
EPOC-001 → EPOC-008 → EPOC-009 → EPOC-011 → EPOC-012 → EPOC-018 → EPOC-019 → EPOC-021

**Phase 2: Model Schema (Lines 64-158)**
EPOC-022 → EPOC-026 → EPOC-027 → EPOC-031 → EPOC-032 → EPOC-036 → EPOC-037 → EPOC-040

**Phase 3: Agent React (Lines 196-497)**
EPOC-041 → EPOC-045 → EPOC-046 → EPOC-052 → EPOC-053 → EPOC-058 → EPOC-059 → EPOC-061

**Phase 4: Persistence + Sandboxing + Mock**
EPOC-062 → EPOC-066 → EPOC-067 → EPOC-070 → EPOC-071 → EPOC-074

**Phase 5: Integration + CLI + Schema**
EPOC-075 → EPOC-077 → EPOC-078 → EPOC-081 → EPOC-082 → EPOC-084

**Phase 6: Build Hygiene**
EPOC-085 → EPOC-087

---

## Checklist

- [ ] EPOC-001 through EPOC-008: Provider Config Parsing + Validation
- [ ] EPOC-009 through EPOC-018: LlmBackend Trait + Implementation
- [ ] EPOC-019 through EPOC-021: Config Resolution Hierarchy
- [ ] EPOC-022 through EPOC-026: Model Schema Parsing
- [ ] EPOC-027 through EPOC-031: Model Registry Lifecycle
- [ ] EPOC-032 through EPOC-036: Resource Management
- [ ] EPOC-037 through EPOC-040: Template Interpolation
- [ ] EPOC-041 through EPOC-045: ReAct Agent Tool Loop
- [ ] EPOC-046 through EPOC-052: Tool Definitions
- [ ] EPOC-053 through EPOC-058: Step Executor with Retry
- [ ] EPOC-059 through EPOC-061: SSE Streaming
- [ ] EPOC-062 through EPOC-066: Workflow Persistence
- [ ] EPOC-067 through EPOC-070: Tool Sandboxing
- [ ] EPOC-071 through EPOC-074: Mock Testing
- [ ] EPOC-075 through EPOC-077: Schema Version Validation
- [ ] EPOC-078 through EPOC-081: End-to-End Integration
- [ ] EPOC-082 through EPOC-084: CLI Integration
- [ ] EPOC-085 through EPOC-087: Build Hygiene
