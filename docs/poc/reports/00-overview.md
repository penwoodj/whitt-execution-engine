# Extended POC Overview

## Purpose

This POC validates the unified-workflow-schema.yml implementation for **Roadmap Phases 0-2** by replacing the current flat configuration with a provider-based architecture, model schema registry, and basic ReAct agent using the Rig framework.

### Goals

1. **Provider Config Layer** — Map schema providers section to Rust structs with serde-saphyr parsing and garde validation
2. **Model Schema Layer** — Implement model registry with lifecycle management and parse-time interpolation
3. **Agent Execution Layer** — Replace current ReAct agent with Rig-based implementation using structured tools

---

## Scope Summary (1 Page)

### IN Scope

| Area | What We Implement |
|-------|------------------|
| **Providers** | Only `llama_cpp_with_vulkan` type (lines 27-51) |
| **Models** | Model definitions, registry, resource management (lines 64-118) |
| **Steps** | Basic LLM agent steps (generative_entity + prompt) (lines 293-339) |
| **Streaming** | SSE output support |
| **Retry** | Step-level retry with backoff strategies |
| **Interpolation** | Parse-time only: `${models.model_name}` |
| **Tools** | 6 tools: model_list, model_load, model_unload, chat, file_read, final_answer |

### OUT of Scope

| Feature | Deferred To |
|---------|-------------|
| Other providers (lmstudio, ollama) | Phase 3 |
| Hooks (when:, before_step_starts:) | Phase 2 |
| User input (ask/trust/block) | Phase 3 |
| Loops, sub-workflows | Phase 2 |
| Guardrails, tool permissions | Phase 2 |
| Runtime interpolation (`{{step.X.output}}`) | Phase 2 |
| Code generation | Phase 3 |

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                     Unified Workflow YAML                     │
│              (providers, models, steps, retry)                         │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                        serde-saphyr Parser                                  │
│                  (YAML → Rust structs + validation)                      │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
            ┌────────────┴────────────┐
            │                         │
            ▼                         ▼
┌───────────────────────┐  ┌───────────────────────┐
│   Provider Config     │  │    Model Schema       │
│                       │  │                       │
│ • LlamaCppVulkan     │  │ • ModelSpec          │
│   Provider           │  │ • ModelRegistry      │
│ • Connection,       │  │ • ResourceManager   │
│   Hosting, Requests │  │ • TemplateInterpolator│
│ • RetryConfig       │  │ • Lifecycle States   │
└───────────┬───────────┘  └───────────┬───────────┘
            │                          │
            └────────────┬─────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                      LlmBackend Trait                                       │
│  (chat, chat_stream, list_models, health_check, load_model, unload_model)     │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                  LlamaCppVulkanBackend                                    │
│        (HTTP client, timeout, streaming, retry logic)                     │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                   ReAct Agent (Rig Framework)                              │
│  • Tool loop with max turns                                                 │
│  • Regex-based tool parsing                                                 │
│  • Step-level retry enforcement                                              │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                      Tools                                                 │
│  • model_list, model_load, model_unload                                 │
│  • chat (single LLM call)                                                  │
│  • file_read                                                               │
│  • final_answer (terminates loop)                                          │
└────────────────────────────┬────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                    SSE Streaming Output                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Technology Stack

| Crate | Version | Purpose | Rationale |
|-------|---------|----------|------------|
| **serde-saphyr** | Latest | YAML parsing | 1.6x faster than serde_yaml, pure Rust (no unsafe), merge keys, garde integration |
| **Rig** | 0.4 | Agent orchestration | 6,736★, 20+ providers, battle-tested, OpenAI-compatible types |
| **minijinja** | 2.5 | Template interpolation | Jinja2 = LLM training familiarity, 4-5x faster than Handlebars |
| **garde** | Latest | Validation | Integration with serde-saphyr, compile-time checks |
| **tokio** | Latest | Async runtime | Industry standard, 5M+ downloads, mature ecosystem |
| **async-trait** | Latest | Async trait support | Required for LlmBackend trait |
| **regex** | 1.10 | Tool parsing | Battle-tested regex engine |
| **sseer** | Latest | SSE streaming | Server-Sent Events support |
| **thiserror** | Latest | Error handling | Type-safe error enums for libraries |
| **anyhow** | Latest | Error handling | Simple error context for applications |

---

## Schema Alignment

### Schema Sections We Implement

| Schema Section | Lines | Implementation |
|----------------|--------|-----------------|
| `providers` (llama_cpp_with_vulkan only) | 27-51 | **01-provider-config.md** → `ProviderConfig` structs |
| `models` (definitions, execution, thinking) | 64-118 | **02-model-schema.md** → `ModelsConfig`, `ModelSpec`, `ModelRegistry` |
| `steps` (basic generative_entity + prompt) | 293-339 | **03-agent-react.md** → `StepConfig`, `ReactAgent` |
| `retry` (step-level) | 249-268 | **03-agent-react.md** → `RetryConfig`, backoff logic |
| Variable interpolation (parse-time `${models.X}`) | 726-738 | **02-model-schema.md** → `TemplateInterpolator` |

### Schema Sections We Do NOT Implement

| Schema Section | Lines | Deferred To |
|----------------|--------|--------------|
| `providers` (lmstudio, ollama) | 28-54 | Phase 3 |
| `models.tools` | 109-117 | Phase 2 |
| `models.guardrails` | 119-141 | Phase 2 |
| `sub_workflows` | 165-191 | Phase 2 |
| `agentic_workflow.hooks` | 273-341 | Phase 2 |
| `agentic_workflow.steps.tool` | 344-363 | Phase 2 |
| `agentic_workflow.steps.loop` | 411-452 | Phase 2 |
| `tool_permissions` | 606-676 | Phase 2 |
| `memory.rag` | 682-696 | Phase 2 |
| Runtime interpolation (`{{step.X.output}}`) | 732-739 | Phase 2 |

---

## Migration Path: What We Keep vs Replace

### Keep from Current POC

| Component | Status | Notes |
|-----------|--------|-------|
| **LlamaHttpClient** | Keep (refactor) | Migrate logic into `LlamaCppVulkanBackend` impl |
| **DockerManager** | Keep | No changes required |
| **OpenAI-compatible types** | Keep | `ChatCompletionRequest`, `ChatCompletionResponse`, `Message` |
| **CLI interface structure** | Keep | `src/bin/whitt.rs` entry point |
| **Error handling patterns** | Keep | `anyhow::Result`, `thiserror` for library types |

### Replace from Current POC

| Current | Replacement | Location |
|---------|--------------|-----------|
| `LlamaConfig` (flat) | `ProviderConfig` (hierarchical) | `src/config/provider.rs` |
| Regex-based YAML parsing | serde-saphyr parsing | Config loader |
| Hand-rolled ReAct agent | Rig framework + `ReactAgent` | `src/agent/react.rs` |
| Tool definitions (inline) | Structured `Tool` trait + 6 tools | `src/agent/tools.rs` |
| No streaming | SSE streaming via `sseer` | `src/agent/streaming.rs` |
| No validation | garde annotations on all structs | All config structs |
| No retry logic | Configurable retry with backoff | `RetryConfig`, step executor |

---

## Success Criteria Checklist

### Provider Config (01-provider-config.md)

- [ ] Parse `providers.llama_cpp_with_vulkan` section from unified YAML
- [ ] Validate using garde (range constraints, required fields)
- [ ] Resolve hierarchy: providers → per-model overrides → defaults
- [ ] Implement LlmBackend trait with all methods
- [ ] Implement LlamaCppVulkanBackend (refactor from LlamaHttpClient)
- [ ] All values as ${variable} references, no hardcoded defaults

### Model Schema (02-model-schema.md)

- [ ] Parse `models` section from unified YAML
- [ ] Implement ModelRegistry with load/unload/health_check
- [ ] Resource management (ram_allocation, max_allowed, min_allowed)
- [ ] Template interpolation (${models.model_name} at parse time)
- [ ] Model override at step level (model_overrides in steps)

### Agent React (03-agent-react.md)

- [ ] ReAct agent using Rig crate
- [ ] Tool definitions (model_list, model_load, model_unload, chat, file_read, final_answer)
- [ ] Step execution: generative_entity + prompt → LLM → tool parse → execute → loop
- [ ] Max turns from model execution config
- [ ] Retry logic from step retry config
- [ ] Streaming output support (SSE)

### Integration

- [ ] All 3 sub-plans integrate (provider → model → agent)
- [ ] End-to-end workflow execution test passes
- [ ] No hardcoded values in implementation
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build)

---

## Risk Assessment and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|---------|------------|
| **serde-saphyr learning curve** | Medium | Medium | Start with simple structs, iterate. Use garde annotations early. |
| **Rig API changes** | Low | High | Pin to specific version (0.4). Monitor upstream releases. |
| **Tool parsing regex fragility** | High | Medium | Test extensively with LLM outputs. Add fallback parsing. |
| **Async trait complexity** | Medium | Low | Use proven patterns from examples. Keep trait simple. |
| **SSE streaming issues** | Medium | Medium | Mock llama.cpp server for testing. Handle connection drops. |
| **Resource management race conditions** | Low | High | Use `RwLock` for registry. Test concurrent loads/unloads. |
| **Config merge bugs** | Medium | High | Write unit tests for all merge scenarios. Document resolution order. |
| **Migration breakage** | Low | High | Keep old types deprecated. Add migration guide. |

---

## Estimated Effort per Sub-Plan

| Sub-Plan | Tasks | Estimated Days | Dependencies |
|-----------|--------|----------------|---------------|
| **01-provider-config.md** | 4 tasks × 5 steps each | 4-5 days | None (first) |
| **02-model-schema.md** | 5 tasks × 5 steps each | 4-5 days | Provider config must exist |
| **03-agent-react.md** | 5 tasks × 5 steps each | 5-6 days | Provider + model schemas must exist |
| **Integration Testing** | End-to-end validation | 2-3 days | All 3 sub-plans complete |
| **Total** | — | **15-19 days** | — |

---

## Related Documentation

- **Plan Files**: See `docs/plans/extended-poc/` for detailed implementation tasks
  - `00-master-plan.md` — Overall scope and dependencies
  - `01-provider-config.md` — Provider config implementation
  - `02-model-schema.md` — Model schema implementation
  - `03-agent-react.md` — ReAct agent implementation

- **Schema Reference**: `docs/schema/unified-workflow-schema.yml`
- **Architecture Decisions**: `docs/roadmap/ADR-0001.md` (Foundation), `ADR-0003.md` (CLI & Backends)
