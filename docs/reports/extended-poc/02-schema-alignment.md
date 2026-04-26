# Schema Alignment Report

This report maps the unified-workflow-schema.yml to the extended POC implementation, identifying what we implement, what we defer, and the gaps to full Phase 2 compliance.

---

## Schema Field → Implementation Mapping

### High-Level Summary

| Schema Section | Implementation Status | Plan File | Notes |
|----------------|----------------------|------------|--------|
| **Providers** (llama_cpp_with_vulkan only) | ✅ FULL | 01-provider-config.md | Lines 27-51 |
| **Models** (definitions, execution, thinking) | ✅ FULL | 02-model-schema.md | Lines 64-118 |
| **Models.tools** | ❌ OUT | — | Lines 109-117, Phase 2 |
| **Models.guardrails** | ❌ OUT | — | Lines 119-141, Phase 2 |
| **Steps** (basic generative_entity + prompt) | 🟡 PARTIAL | 03-agent-react.md | Lines 293-339 |
| **Steps.tool** (tool key present) | ❌ OUT | — | Lines 344-363, Phase 2 |
| **Steps.loop** | ❌ OUT | — | Lines 411-452, Phase 2 |
| **Steps.sub_workflow** | ❌ OUT | — | Lines 393-410, Phase 2 |
| **Retry** (step-level) | ✅ FULL | 03-agent-react.md | Lines 249-268 |
| **Hooks** | ❌ OUT | — | Lines 273-341, Phase 2 |
| **Tool Permissions** | ❌ OUT | — | Lines 606-676, Phase 2 |
| **Sub-workflows** | ❌ OUT | — | Lines 165-191, Phase 2 |
| **Memory.rag** | ❌ OUT | — | Lines 682-696, Phase 2 |

---

## Detailed Mapping: Providers Section

### Schema Lines 27-51

| Schema Field | Rust Struct | Implementation Status |
|-------------|-------------|----------------------|
| `providers.llama_cpp_with_vulkan` | `ProviderConfig` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config` | `LlamaCppConfig` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.config` | `ConnectionConfig` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.host` | `ConnectionConfig.host` | ✅ FULL (default: "localhost") |
| `providers.llama_cpp_with_vulkan.config.port` | `ConnectionConfig.port` | ✅ FULL (default: 8080) |
| `providers.llama_cpp_with_vulkan.config.connection_timeout_secs` | `ConnectionConfig.connection_timeout_secs` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.hosting` | `HostingConfig` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.hosting.max_concurrent_models` | `HostingConfig.max_concurrent_models` | ✅ FULL (default: 1) |
| `providers.llama_cpp_with_vulkan.config.hosting.model_offload_timeout_secs` | `HostingConfig.model_offload_timeout_secs` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.hosting.gpu_allocation` | `Option<GpuAllocation>` | ✅ FULL (presence = enabled) |
| `providers.llama_cpp_with_vulkan.config.hosting.gpu_allocation.vram_per_model_mb` | `GpuAllocation.vram_per_model_mb` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.hosting.cpu_fallback` | `Option<CpuFallback>` | ✅ FULL (presence = enabled) |
| `providers.llama_cpp_with_vulkan.config.hosting.cpu_fallback.cpu_cores_per_model` | `CpuFallback.cpu_cores_per_model` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.requests` | `RequestsConfig` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.requests.max_concurrent_requests` | `RequestsConfig.max_concurrent_requests` | ✅ FULL (default: 1) |
| `providers.llama_cpp_with_vulkan.config.requests.request_timeout_secs` | `RequestsConfig.request_timeout_secs` | ✅ FULL (default: 600) |
| `providers.llama_cpp_with_vulkan.config.requests.queue_timeout_secs` | `RequestsConfig.queue_timeout_secs` | ✅ FULL (default: 300) |
| `providers.llama_cpp_with_vulkan.config.requests.rate_limit_per_minute` | `RequestsConfig.rate_limit_per_minute` | ✅ FULL (default: 60) |
| `providers.llama_cpp_with_vulkan.config.requests.retry` | `Option<RetryConfig>` | ✅ FULL (presence = enabled) |
| `providers.llama_cpp_with_vulkan.config.requests.retry.max_retries` | `RetryConfig.max_retries` | ✅ FULL |
| `providers.llama_cpp_with_vulkan.config.requests.retry.backoff` | `RetryConfig.backoff` | ✅ FULL (Exponential/Linear/Fixed) |
| `providers.llama_cpp_with_vulkan.config.requests.retry.initial_delay` | `RetryConfig.initial_delay` | ✅ FULL (default: "1s") |
| `providers.llama_cpp_with_vulkan.config.requests.retry.max_delay` | `RetryConfig.max_delay` | ✅ FULL (default: "30s") |
| `providers.llama_cpp_with_vulkan.config.requests.retry.multiplier` | `RetryConfig.multiplier` | ✅ FULL (default: 2.0) |
| `providers.llama_cpp_with_vulkan.config.requests.retry.jitter` | `RetryConfig.jitter` | ✅ FULL (default: true) |

### Deferred Provider Features

| Schema Feature | Lines | Deferred To |
|----------------|--------|--------------|
| `providers.lmstudio` | 28-33 | Phase 3 |
| `providers.ollama` | 53-54 | Phase 3 |
| `providers.llama_cpp_with_vulkan.config_file` | 57 | Phase 3 (POC: inline only) |

---

## Detailed Mapping: Models Section

### Schema Lines 64-118

| Schema Field | Rust Struct | Implementation Status |
|-------------|-------------|----------------------|
| `models` | `ModelsConfig` | ✅ FULL |
| `models.global_config_path` | `ModelsConfig.global_config_path` | ✅ FULL (optional) |
| `models.default_router` | `ModelsConfig.default_router` | ✅ FULL (default: Automatic) |
| `models.{model_name}` | `HashMap<String, ModelSpec>` | ✅ FULL |
| `models.{model_name}.name` | `ModelSpec.name` | ✅ FULL |
| `models.{model_name}.host` | `ModelSpec.host` | ✅ FULL |
| `models.{model_name}.host.type` | `ModelHost.provider_type` | ✅ FULL (llama_cpp_with_vulkan only) |
| `models.{model_name}.host.connection_settings` | `ModelHost.connection_settings` | ✅ FULL (serde_json::Value) |
| `models.{model_name}.ram_allocation` | `Option<RamAllocation>` | ✅ FULL (presence = enabled) |
| `models.{model_name}.ram_allocation.strategy` | `RamAllocation.allocation_strategy` | ✅ FULL (Static/Dynamic) |
| `models.{model_name}.max_allowed` | `MaxAllowed` | ✅ FULL |
| `models.{model_name}.max_allowed.ram` | `MaxAllowed.ram` | ✅ FULL (ResourceLimit) |
| `models.{model_name}.max_allowed.vram` | `MaxAllowed.vram` | ✅ FULL (optional) |
| `models.{model_name}.max_allowed.cpu` | `MaxAllowed.cpu` | ✅ FULL (optional) |
| `models.{model_name}.max_allowed.gpu` | `MaxAllowed.gpu` | ✅ FULL (optional) |
| `models.{model_name}.max_allowed.attention_tokens` | `MaxAllowed.attention_tokens` | ✅ FULL (optional) |
| `models.{model_name}.max_allowed.concurrent_requests` | `MaxAllowed.concurrent_requests` | ✅ FULL (optional) |
| `models.{model_name}.min_allowed` | `MinAllowed` | ✅ FULL |
| `models.{model_name}.min_allowed.ram` | `MinAllowed.ram` | ✅ FULL |
| `models.{model_name}.min_allowed.vram` | `MinAllowed.vram` | ✅ FULL (optional) |
| `models.{model_name}.min_allowed.cpu` | `MinAllowed.cpu` | ✅ FULL (optional) |
| `models.{model_name}.min_allowed.gpu` | `MinAllowed.gpu` | ✅ FULL (optional) |
| `models.{model_name}.min_allowed.attention_tokens` | `MinAllowed.attention_tokens` | ✅ FULL (optional) |
| `models.{model_name}.model_memory` | `ModelMemory` | ✅ FULL |
| `models.{model_name}.model_memory.cache_size` | `ModelMemory.cache_size` | ✅ FULL (default: Min) |
| `models.{model_name}.model_memory.kv_cache_quantization` | `ModelMemory.kv_cache_quantization` | ✅ FULL (default: Auto) |
| `models.{model_name}.model_memory.attention_context` | `ModelMemory.attention_context` | ✅ FULL (default: Auto) |
| `models.{model_name}.execution` | `ExecutionConfig` | ✅ FULL |
| `models.{model_name}.execution.timeout` | `TimeoutConfig` | ✅ FULL |
| `models.{model_name}.execution.timeout.load_into_memory` | `TimeoutConfig.load_into_memory` | ✅ FULL (default: "45s") |
| `models.{model_name}.execution.timeout.time_to_first_response` | `TimeoutConfig.ttfb_timeout` | ✅ FULL (default: "1m") |
| `models.{model_name}.execution.timeout.total_time_to_response` | `TimeoutConfig.total_timeout` | ✅ FULL (default: "4h") |
| `models.{model_name}.execution.max_turns` | `ExecutionConfig.max_turns` | ✅ FULL (default: 10) |
| `models.{model_name}.execution.stop_on_tool_failure` | `ExecutionConfig.stop_on_tool_failure` | ✅ FULL (default: false) |
| `models.{model_name}.execution.accumulate_tool_results` | `ExecutionConfig.accumulate_tool_results` | ✅ FULL (default: true) |
| `models.{model_name}.thinking` | `Option<ThinkingConfig>` | ✅ FULL (presence = enabled) |
| `models.{model_name}.thinking.budget_tokens` | `ThinkingConfig.budget_tokens` | ✅ FULL (default: 4096) |
| `models.{model_name}.thinking.capture_in_output` | `ThinkingConfig.capture_in_output` | ✅ FULL (default: true) |
| `models.{model_name}.thinking.capture_in_events` | `ThinkingConfig.capture_in_events` | ✅ FULL (default: true) |

### Deferred Model Features

| Schema Feature | Lines | Deferred To |
|----------------|--------|--------------|
| `models.{model_name}.tools` | 109-117 | Phase 2 |
| `models.{model_name}.tools.default_permissions` | 110-114 | Phase 2 |
| `models.{model_name}.tools.allowed_tools` | 115 | Phase 2 |
| `models.{model_name}.tools.forbidden_tools` | 116 | Phase 2 |
| `models.{model_name}.tools.custom_tools` | 117 | Phase 2 |
| `models.{model_name}.guardrails` | 119-141 | Phase 2 |
| `models.{model_name}.guardrails.enforcement_policy` | 120 | Phase 2 |
| `models.{model_name}.guardrails.input` | 121-132 | Phase 2 |
| `models.{model_name}.guardrails.output` | 133-141 | Phase 2 |

---

## Detailed Mapping: Steps Section

### Schema Lines 293-339 (Basic Agent Steps)

| Schema Field | Rust Struct | Implementation Status |
|-------------|-------------|----------------------|
| `steps` | `HashMap<String, StepConfig>` | 🟡 PARTIAL |
| `steps.{step_name}` | `StepConfig` | 🟡 PARTIAL |
| `steps.{step_name}.generative_entity` | `StepConfig.generative_entity` | ✅ FULL (references `${models.X}`) |
| `steps.{step_name}.prompt` | `StepConfig.prompt` | ✅ FULL |
| `steps.{step_name}.model_overrides` | `StepConfig.model_overrides` | 🟡 PARTIAL (max_turns only) |
| `steps.{step_name}.model_overrides.max_turns` | `model_overrides.max_turns` | ✅ FULL |
| `steps.{step_name}.retry` | `StepConfig.retry` | ✅ FULL |
| `steps.{step_name}.retry.max_attempts` | `RetryConfig.max_retries` | ✅ FULL |
| `steps.{step_name}.retry.backoff` | `RetryConfig.backoff` | ✅ FULL |
| `steps.{step_name}.retry.initial_delay` | `RetryConfig.initial_delay` | ✅ FULL |
| `steps.{step_name}.retry.max_delay` | `RetryConfig.max_delay` | ✅ FULL |
| `steps.{step_name}.retry.multiplier` | `RetryConfig.multiplier` | ✅ FULL |
| `steps.{step_name}.retry.jitter` | `RetryConfig.jitter` | ✅ FULL |

### Deferred Step Features

| Schema Feature | Lines | Deferred To |
|----------------|--------|--------------|
| `steps.{step_name}.tool` | 344 | Phase 2 |
| `steps.{step_name}.depends_on` | 312 | Phase 2 |
| `steps.{step_name}.when` | 314-341 | Phase 2 (all hooks) |
| `steps.{step_name}.sub_workflow` | 395 | Phase 2 |
| `steps.{step_name}.loop` | 413-437 | Phase 2 |
| `steps.{step_name}.user_input` | 458-461 | Phase 3 |

---

## Variable Interpolation Coverage

### Schema Lines 726-739

| Syntax | Example | Implementation Status |
|--------|---------|----------------------|
| `${models.model_name}` | `${models.primary-analyzer}` | ✅ FULL — Parse-time resolution |
| `${workspace.path_var}` | `${workspace.root_path}` | ❌ OUT — Not in POC scope |
| `${workflow.field_name}` | `${workflow.hardcoded_values.thresholds}` | ❌ OUT — Not in POC scope |
| `{{step.step_name.output}}` | `{{step.analyze_code.output}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |
| `{{sub_workflow.name.output}}` | `{{sub_workflow.validate_workflow.output}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |
| `{{inputs.field_name}}` | `{{inputs.workspace_path}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |
| `{{loop.iteration_variable}}` | `{{loop.current_file}}` | ❌ DEFERRED — Loop support (Phase 2) |
| `{{now}}` | `{{now}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |
| `{{workflow_id}}` | `{{workflow_id}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |
| `{{run.number}}` | `{{run.number}}` | ❌ DEFERRED — Runtime interpolation (Phase 2) |

### POC Interpolation Strategy

**Parse-time resolution only** (Phase 1):
- TemplateInterpolator in `src/model/interpolation.rs`
- Converts `${models.X}` to actual model ID at workflow load time
- Uses Minijinja for Jinja2-compatible syntax
- Example: `"${models.primary-analyzer}"` → `"llama-3.2-3b-instruct"`

**Runtime interpolation deferred** (Phase 2):
- No support for `{{step.X.output}}`, `{{inputs.X}}`, etc.
- These require step execution context and variable binding
- Will be added in Phase 2 with full variable binding system

---

## Config Merge/Resolution Strategy

### Schema Default Behavior

From schema lines 751-762:
- **L1 (workflow top-level)**: providers, models, sub_workflows, workspace, memory, tool_permissions
- **L2 (agentic_workflow defaults)**: retry.step:, when (default hooks)
- **L3 (step-level)**: retry, when, requires, depends_on, parallel_group, prompt, tool

### Resolution Rules

| Level | Config Type | Behavior |
|-------|--------------|----------|
| **Retry** | L2 → L3 | Step-level retry **completely replaces** workflow defaults (not merged) |
| **Hooks** | L2 → L3 | Default hooks **merge** with step hooks (additive) |
| **Models** | L1 → L3 | Models defined at L1, referenced at L3 via `${models.X}` |
| **Sub-workflows** | L1 → L3 | Sub-workflows defined at L1, referenced at L3 via `sub_workflow:` key |
| **Tool Permissions** | L1 → L3 | Global baseline at L1, per-step can only **restrict**, not expand |

### POC Implementation

**What we implement**:
1. **Provider → Model → Step resolution**:
   - Base config from `providers.llama_cpp_with_vulkan`
   - Per-model overrides in `models.{model_name}.host.connection_settings`
   - Step-level overrides in `steps.{step_name}.model_overrides`

2. **Retry defaults**:
   - Workflow-level retry (from schema defaults)
   - Step-level retry (completely replaces workflow defaults)

**What we defer**:
- Hook merging (L2 + L3) — Phase 2
- Tool permission enforcement — Phase 2
- Sub-workflow resolution — Phase 2

### POC Resolution Order

```
1. Providers (base defaults)
   ↓
2. Models (per-model overrides on provider defaults)
   ↓
3. Steps (step-level overrides on model/provider defaults)
```

---

## Gap Analysis: POC → Phase 2 Compliance

### Missing Features for Full Phase 2

| Feature | Complexity | Estimated Effort | Priority |
|---------|-------------|-------------------|----------|
| **Hooks (when:) lifecycle** | High | 5-7 days | Critical |
| **Runtime interpolation ({{...}})** | High | 4-6 days | Critical |
| **Sub-workflows** | Medium | 4-5 days | High |
| **Loops (validation + count)** | Medium | 5-6 days | High |
| **Tool permissions** | Medium | 3-4 days | Medium |
| **Guardrails (input/output)** | High | 6-8 days | Medium |
| **Models.tools** | Low | 2-3 days | Medium |
| **Other providers (lmstudio, ollama)** | Low | 3-4 days | Low |

### Architecture Gaps

| Gap | Impact | Solution |
|------|--------|----------|
| **No variable binding** | Cannot reference step outputs | Implement `VariableResolver` in Phase 2 |
| **No hook system** | Cannot execute pre/post step logic | Implement `HookRegistry` in Phase 2 |
| **No loop control** | Cannot iterate or converge | Implement `LoopExecutor` in Phase 2 |
| **No sub-workflow resolution** | Cannot nest workflows | Implement `SubWorkflowResolver` in Phase 2 |
| **No tool permission checks** | Cannot enforce allow/deny lists | Implement `PermissionChecker` in Phase 2 |

### Data Flow Gaps

| Current POC | Phase 2 Required |
|-------------|------------------|
| Step execution isolated | Step outputs stored as variables |
| No dependency tracking | `depends_on` and `requires` resolution |
| No parallel execution | Parallel group scheduling |
| No event propagation | Bidirectional events for hooks |
| No checkpointing | State persistence and resume |

---

## Implementation Status Summary

### Overall Coverage

| Schema Section | Lines | POC Coverage | Phase 2 Target |
|----------------|--------|---------------|-----------------|
| Providers | 27-58 | 50% (1 of 3 providers) | 100% |
| Models | 64-158 | 70% (no tools/guardrails) | 100% |
| Steps | 293-497 | 20% (basic agent only) | 100% |
| Retry | 249-268 | 100% | 100% |
| Hooks | 270-341 | 0% | 100% |
| Tool Permissions | 606-676 | 0% | 100% |
| Sub-workflows | 165-191 | 0% | 100% |
| Memory.rag | 682-696 | 0% | 100% |

**POC Overall Schema Coverage: ~35%**

### What's Next After POC

1. **Phase 2 (Quality Loops)** — Add hooks, loops, sub-workflows, guardrails
2. **Phase 3 (CLI & Backends)** — Add lmstudio, ollama providers, tool permissions
3. **Phase 4 (Memory Search)** — Add RAG implementation
4. **Phase 5-7** — Advanced features: automation, metrics, autonomy

---

## Related Documentation

- **Plan Files**:
  - `00-master-plan.md` — Overall scope and architecture
  - `01-provider-config.md` — Provider config implementation details
  - `02-model-schema.md` — Model schema implementation details
  - `03-agent-react.md` — Agent ReAct implementation details

- **Schema Reference**: `docs/schema/unified-workflow-schema.yml`

- **Architecture Decisions**: `docs/roadmap/ADR-0001.md` (Foundation), `ADR-0002.md` (MVP Queue)
