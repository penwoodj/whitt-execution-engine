# Whitt Extended POC Master Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement extended POC that validates unified-workflow-schema.yml with provider-based configuration, model schema registry, and basic ReAct agent using Rig framework.

**Architecture:** Three-layer refactoring: provider config layer → model schema layer → agent execution layer. Parse unified YAML, map to Rust structs, execute via Rig with llama.cpp Vulkan backend.

**Tech Stack:** serde-saphyr (YAML), Rig (agent), Treadle (state persistence), Minijinja (templates), tokio (runtime), garde (validation), Sandlock (sandboxing), VidaiMock (testing).

---

## Scope Boundaries

### IN Scope
- Only `llama_cpp_with_vulkan` provider type
- Model configuration under `providers` and `models` keys
- Basic LLM agent steps (generative_entity + prompt pattern)
- Streaming output support (SSE)
- Max turns, retry logic, model overrides at step level
- Template interpolation at parse time (${models.model_name})

### OUT of Scope
- Hooks (when:, before_step_starts:, etc.)
- User input (ask/trust/block permissions)
- Loops, sub-workflows, code generation
- RAG, guardrails, tool permissions system
- Hardcoded values in code — all from YAML

---

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    00-master-plan.md                               │
│                    (This document)                                  │
└──────────┬──────────────────┬──────────────────┬──────────────────┘
           │                  │                  │
           ▼                  ▼                  ▼
    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │01-provider-  │  │02-model-     │  │03-agent-     │
    │config.md     │  │schema.md     │  │react.md      │
    └──────┬───────┘  └──────┬───────┘  └──────┬───────┘
           │                  │                  │
           └──────────────────┴──────────────────┘
                          │
                          ▼
              ┌──────────────────────┐
              │ Extended POC Done  │
              └──────────────────────┘
```

**Execution Order:** Complete 01-provider-config first → 02-model-schema → 03-agent-react. Each sub-plan has prerequisites.

---

## Technology Decisions

### Why These Crates

| Crate | Purpose | Why Chosen |
|-------|---------|-------------|
| serde-saphyr | YAML parsing | 1.6x faster than serde_yaml, pure Rust (no unsafe), merge keys, garde integration, DoS budgets |
| Rig | Agent orchestration | 6,736★, 20+ providers, battle-tested, OpenAI-compatible types, agent builder, tool definitions |
| Treadle | Workflow state persistence | Persistent, resumable, SQLite StateStore, HITL support, DAG execution |
| Minijinja | Template interpolation | Jinja2 = LLM training familiarity, faster than Tera/Handlebars |
| garde | Validation | Integration with serde-saphyr, compile-time checks, schema validation at load time |
| Sandlock | Tool sandboxing | Landlock + seccomp + resource limits, per-tool sandboxing, Linux 6.12+ |
| VidaiMock | LLM testing | Physics-accurate streaming simulation, zero-config, 50,000+ RPS benchmark mode |
| tokio | Async runtime | Industry standard, 5M+ downloads, mature ecosystem |

### Alternatives Considered and Rejected

- `yaml_serde`: Good for migration compat, but serde-saphyr is faster and has better features
- `serde_yml`: **REJECTED** — RustSec advisories, archived, AI-slop repo
- `serde_yaml`: **REJECTED** — Deprecated, unmaintained
- `dagrs`: **REJECTED** — Archived January 2026
- `Cognis`: Too new (March 2026), untested in production
- `Handlebars`: 4-5x slower than Minijinja

### Schema Versioning Strategy

All YAML workflows MUST include `schema_version` field (from unified-workflow-schema.yml line 804):

```yaml
schema_version: "2.0.0"
min_schema_version: "2.0.0"
```

**Validation at load time:**
1. Parse schema_version field from YAML
2. Verify it matches supported version range
3. Use garde annotations on all structs for compile-time validation
4. Reject workflows with incompatible schema versions before execution

**Version format:** Semver (MAJOR.MINOR.PATCH)
- MAJOR: Breaking changes to schema structure
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

### Workflow Metadata Separation

Following upstream success factor #8, workflow execution properly separates metadata from execution configuration:

**Metadata (workflow-level):**
```rust
pub struct WorkflowMetadata {
    pub workflow_id: String,
    pub name: String,
    pub description: String,
    pub version: String,           // Semver
    pub schema_version: String,    // Schema semver
    pub author: String,
    pub tags: Vec<String>,
}
```

**Execution Config (separate):**
```rust
pub struct WorkflowExecution {
    pub providers: ProviderConfig,
    pub models: ModelsConfig,
    pub steps: HashMap<String, StepConfig>,
    pub retry: Option<WorkflowRetryConfig>,
    pub hooks: Option<WorkflowHooks>,
}
```

**Why this matters:**
- Metadata is human-readable documentation (name, description, tags)
- Execution config is machine-actionable (providers, models, steps)
- Clear separation enables:
  - Schema evolution without breaking execution
  - Metadata indexing without parsing execution logic
  - Execution config reuse across workflows

---

## File Inventory

### Files to Create

**Provider Config:**
- `src/config/provider.rs` — Provider configuration structs (ProviderConfig, LlamaCppVulkanConfig, Hosting, Requests)
- `src/config/unified.rs` — Unified workflow schema structs (WorkflowSpec, Models, Steps)
- `src/backend/llm_backend.rs` — LlmBackend trait definition
- `src/backend/llama_vulkan.rs` — LlamaCppVulkanBackend implementation
- `src/config/loader.rs` — Updated config loader with providers hierarchy

**Model Schema:**
- `src/model/registry.rs` — ModelRegistry with load/unload/health_check
- `src/model/resource.rs` — Resource management structs (RamAllocation, MaxAllowed, MinAllowed)
- `src/model/interpolation.rs` — Template interpolation with Minijinja

**Agent React:**
- `src/agent/react.rs` — ReAct agent implementation using Rig
- `src/agent/tools.rs` — Tool definitions (model_list, model_load, model_unload, chat, file_read, final_answer)
- `src/agent/executor.rs` — Step executor with retry and max turns
- `src/agent/streaming.rs` — SSE streaming support

**Tests:**
- `tests/provider_config_test.rs` — Provider config parsing and validation
- `tests/model_schema_test.rs` — Model registry and interpolation tests
- `tests/agent_react_test.rs` — ReAct agent integration tests

### Files to Modify

**Refactoring:**
- `src/config/mod.rs` — Extract provider config to `provider.rs`, keep backward compat
- `src/client/mod.rs` — Keep OpenAI-compatible types, add backend trait impl
- `src/bin/whitt.rs` — Replace ReAct agent with unified workflow execution

**Deprecate:**
- `src/config/mod.rs:LlamaConfig` — Deprecate flat config, keep for migration path

### Files to Delete

- `src/client/llama_client.rs` — Replace with backend trait impl

---

## Alignment with Unified Schema v2.0

### Schema Sections Mapped

| Schema Section | Plan Document | Status |
|---------------|---------------|---------|
| `providers` (lines 27-51) | 01-provider-config.md | Planned |
| `models` (lines 64-118) | 02-model-schema.md | Planned |
| `steps` (lines 293-339) | 03-agent-react.md | Planned |
| Variable interpolation (lines 726-738) | 02-model-schema.md | Planned |
| Retry config (lines 249-268) | 03-agent-react.md | Planned |

### ADR Alignment

| ADR | Requirement | Plan Coverage |
|-----|-------------|---------------|
| ADR-0001 | Foundation phase | All 3 plans establish foundation |
| ADR-0002 | MVP queue/scheduler | Deferred to Phase 2 (not in POC) |
| ADR-0003 | CLI & backends | Backend abstraction via trait, CLI unchanged |

---

## Success Criteria

### Definition of Done

1. **Provider Config (01):**
   - [ ] Parse `providers.llama_cpp_with_vulkan` section from unified YAML
   - [ ] Validate using garde (range constraints, required fields)
   - [ ] Resolve hierarchy: providers → per-model overrides → defaults
   - [ ] Implement LlmBackend trait with all methods
   - [ ] Implement LlamaCppVulkanBackend (refactor from LlamaHttpClient)
   - [ ] All values as ${variable} references, no hardcoded defaults

2. **Model Schema (02):**
   - [ ] Parse `models` section from unified YAML
   - [ ] Implement ModelRegistry with load/unload/health_check
   - [ ] Resource management (ram_allocation, max_allowed, min_allowed)
   - [ ] Template interpolation (${models.model_name} at parse time)
   - [ ] Model override at step level (model_overrides in steps)

3. **Agent React (03):**
   - [ ] ReAct agent using Rig crate
   - [ ] Tool definitions (model_list, model_load, model_unload, chat, file_read, final_answer)
   - [ ] Step execution: generative_entity + prompt → LLM → tool parse → execute → loop
   - [ ] Max turns from model execution config
   - [ ] Retry logic from step retry config
   - [ ] Streaming output support (SSE)

4. **Integration:**
   - [ ] All 3 sub-plans integrate (provider → model → agent)
   - [ ] End-to-end workflow execution test passes
   - [ ] No hardcoded values in implementation
   - [ ] LSP diagnostics clean on all changed files
   - [ ] Build passes (cargo build)

---

## Sub-Plan Details

### 01-provider-config.md
**Focus:** Provider configuration layer.
**Files:** `src/config/provider.rs`, `src/backend/llm_backend.rs`, `src/backend/llama_vulkan.rs`
**Key Tasks:**
- Map unified schema providers section to Rust structs
- Define LlmBackend trait
- Implement LlamaCppVulkanBackend
- Refactor LlamaHttpClient → backend trait impl
- Config resolution with merge logic

### 02-model-schema.md
**Focus:** Model schema layer.
**Files:** `src/model/registry.rs`, `src/model/resource.rs`, `src/model/interpolation.rs`
**Key Tasks:**
- Parse models section from unified YAML
- Implement ModelRegistry with lifecycle
- Resource management (RAM, VRAM, CPU, GPU)
- Template interpolation with Minijinja
- Model overrides at step level

### 03-agent-react.md
**Focus:** Agent execution layer.
**Files:** `src/agent/react.rs`, `src/agent/tools.rs`, `src/agent/executor.rs`, `src/agent/streaming.rs`
**Key Tasks:**
- ReAct agent using Rig
- Tool definitions (6 tools)
- Step executor with retry
- Max turns enforcement
- SSE streaming support

---

## Implementation Notes

### Configuration Resolution Order

From ADR-0001 requirements:
1. Providers section (base config)
2. Per-model overrides (`models.{model_name}`.host.connection_settings)
3. Step-level overrides (`steps.{step_name}.model_overrides`)

### Variable Interpolation

Two types (per schema lines 726-738):
- `${models.model_name}` — Parse-time structural references (resolved in loader)
- `{{step.name.output}}` — Runtime template values (resolved in executor)

**POC Scope:** Only `${models.model_name}` implemented (parse-time). Runtime interpolation deferred to Phase 2.

### Error Handling

- Provider errors: `anyhow::Result` with context
- Backend errors: `LlmError` enum (connection, timeout, parse)
- Agent errors: `AgentError` enum (tool_failure, max_turns_exceeded)
- Retry on transient errors (429, 503, timeout)

---

## Testing Strategy

### Unit Tests (Per Sub-Plan)

- **01:** Provider config parsing, validation, backend trait methods
- **02:** Model registry, resource management, interpolation
- **03:** Tool execution, ReAct loop, retry logic

### Integration Tests (After All 3)

- Full workflow: load YAML → parse → resolve config → load model → execute step → stream response
- Test: Valid unified YAML with llama.cpp Vulkan provider
- Test: Invalid config rejected with clear error
- Test: Template interpolation resolves correctly
- Test: Max turns enforced
- Test: Retry logic works on transient errors

### Mock Testing

Use `VidaiMock` for realistic streaming simulation (from upstream-success-factors.md).

---

## Execution Handoff

**Plan complete and saved to `docs/plans/extended-poc/00-master-plan.md`. Two execution options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
