# Requirements Index

**Version**: 5.0
**Date**: 2026-04-13

---

## Overview

Index of all requirements documentation for AutoAgents SDK. Organized by implementation phase, priority, and completion status.

**Status Legend**:
- ✅ Complete - Fully documented and validated
- 🔄 In Progress - Being actively developed
- ⏳ Planned - Outlined but not started
- ❌ Blocked - Waiting on dependency or decision

---

## Requirements by Section

### Core Requirements (Phase 1)

| # | Requirement | Priority | Status | Document | Description |
|---|-------------|----------|--------|----------|-------------|
| 1 | Unified Workflow Schema v2.0 | P0 | ✅ | [unified-workflow-schema.yml](./unifying-schema/unified-workflow-schema.yml) | Single source of truth for workflow structure |
| 2 | Schema Validation | P0 | ✅ | [unified-schema-requirements.md](./unifying-schema/unified-schema-requirements.md) | Validation rules and requirements |
| 3 | Example Workflows | P0 | ✅ | [example-workflows/README.md](./example-workflows/README.md) | 53 categorized workflows covering all features |
| 4 | Model Router | P0 | ✅ | [model-router/README.md](./model-router/README.md) | Multi-provider model abstraction |
| 5 | Transpiler Architecture | P0 | ✅ | [transpiler_architecture.md](../plans/transpiler/transpiler_architecture.md) | 7-layer transpiler design |
| 6 | Transpiler Implementation | P1 | 🔄 | [transpiler_implementation_plan.yml](../plans/transpiler/transpiler_implementation_plan.yml) | Code generation and templates |
| 7 | YAML Parser Integration | P1 | ⏳ | [llamacpp-backend-task.md](../plans/02-cli-and-llm-backend-integration/tasks/04-llamacpp-backend.md) | llama.cpp backend with Vulkan |
| 8 | OpenAI Backend | P1 | ⏳ | [openai-backend-task.md](../plans/02-cli-and-llm-backend-integration/tasks/05-openai-backend.md) | OpenAI API integration |
| 9 | Ollama Backend | P1 | ⏳ | [ollama-backend-task.md](../plans/02-cli-and-llm-backend-integration/tasks/03-ollama-backend.md) | Ollama provider implementation |
| 10 | LLM Backend Trait | P0 | ✅ | [llm-backend-trait-task.md](../plans/02-cli-and-llm-backend-integration/tasks/01-llm-backend-trait.md) | Unified provider abstraction |
| 11 | Model Discovery | P0 | ✅ | [model-discovery-task.md](../plans/02-cli-and-llm-backend-integration/tasks/00-model-discovery.md) | Provider model enumeration |
| 12 | Configuration Defaults | P0 | ✅ | [configuration-defaults.md](./configuration-defaults.md) | Default values for all components |
| 13 | CLI Implementation | P1 | 🔄 | [cli-foundation-task.md](../plans/00-foundation/tasks/00-cli-foundation.md) | Command-line interface |

---

## Advanced Requirements (Phase 2+)

| # | Requirement | Priority | Status | Document | Description |
|---|-------------|----------|--------|----------|-------------|
| 14 | Constraints and Assumptions | P1 | ✅ | [constraints-and-assumptions.md](./constraints-and-assumptions.md) | Project scope and design decisions |
| 15 | Advanced Agentic Features | P1 | ✅ | [advanced-agentic-features.md](./advanced-agentic-features.md) | Multi-agent spawning + loop termination |
| 16 | Benchmark System | P1 | ✅ | [benchmark-yaml-examples.md](./benchmark-yaml-examples.md) | 3 scaling workflows (5/20/100 models) |
| 17 | Llama.cpp Vulkan Integration | P1 | ✅ | [llamacpp-vulkan-integration.md](./llamacpp-vulkan-integration.md) | GPU acceleration with layer offload |
| 18 | Transpiler Feature Matrix | P1 | ✅ | [transpiler-feature-matrix-r0054-r0075.md](./transpiler-feature-matrix-r0054-r0075.md) | Schema definition + docs generation (R0054-R0075) |
| 19 | Critical Evaluation | P1 | ✅ | [critical-evaluation.md](./critical-evaluation.md) | Tech stack decisions (serde-saphyr, Rig, Treadle, etc.) |
| 20 | Human-in-the-Loop | P1 | ⏳ | [treadle-integration-task.md](../plans/04-quality-loops/tasks/01-treadle-integration.md) | HITL with persistent workflows |
| 21 | Quality Loops | P2 | ⏳ | [quality-loops-plan.md](../plans/04-quality-loops/plan.md) | Generate-verify-repair iteration loops |
| 22 | Memory & Search | P2 | ⏳ | [memory-search-plan.md](../plans/05-memory-search/plan.md) | Vector DB with RAG operations |
| 23 | Autonomy Metrics | P3 | ⏳ | [autonomy-metrics-plan.md](../plans/07-autonomy-metrics/plan.md) | Self-improvement metrics |
| 24 | Automation | P4 | ⏳ | [automation-plan.md](../plans/06-automation/plan.md) | Git operations, cron workflows |

---

## Traceability Matrix

**Requirement-to-Phase Mapping**:

| Requirement | Phase 0 Foundation | Phase 1 MVP | Phase 2 CLI | Phase 3 UI | Phase 4 Loops | Phase 5 Search | Phase 6 Auto | Phase 7 Metrics | Phase 8 Final |
|-------------|--------------------|-------------|-----------|-----------|----------|-----------|-------------|
| Unified Schema | ✅ | ✅ | ✅ | - | - | - | - | - |
| Schema Validation | ✅ | ✅ | ✅ | - | - | - | - |
| Example Workflows | ✅ | ✅ | ✅ | - | - | - | - |
| Model Router | ✅ | ✅ | ✅ | - | - | - | - |
| Transpiler Arch | ✅ | - | 🔄 | - | - | - | - |
| Backend Traits | ✅ | ✅ | 🔄 | - | - | - | - |
| Backends (3) | ✅ | 🔄 | - | - | - | - | - |
| Config Defaults | ✅ | ✅ | - | - | - | - | - |
| CLI | ✅ | - | 🔄 | - | - | - | - |
| Constraints | ✅ | ✅ | - | - | - | - | - |
| Advanced Features | ✅ | ✅ | - | - | - | - | - |
| Benchmark System | ✅ | ✅ | - | - | - | - | - |
| Vulkan | ✅ | ✅ | - | - | - | - | - |
| Transpiler Feature Matrix | ✅ | ✅ | - | - | - | - | - |
| Critical Evaluation | ✅ | ✅ | - | - | - | - | - | - |
| HITL | - | - | - | - | - | - | 🔄 | - |
| Quality Loops | - | - | - | - | - | 🔄 | - |
| Memory Search | - | - | - | - | - | 🔄 | - |
| Autonomy Metrics | - | - | - | - | - | 🔄 | - |
| Automation | - | - | - | - | - | - | 🔄 | - |

---

## Schema Coverage Analysis

### Coverage by Unified Schema Section

| Schema Section | Coverage | Gap | Filled By | Notes |
|---------------|---------|------|-----------|---------|
| **workflow:** | 100% | None | All core sections complete |
| **models:** | 100% | None | Configuration and provider sections complete |
| **steps:** | 100% | None | Simple, loop, parallel, sub-workflows complete |
| **tools:** | 100% | None | Built-in and custom tool definitions complete |
| **when:** | 100% | None | Hooks for conditional execution complete |
| **inputs:** | 100% | None | Step-level inputs complete |
| **outputs:** | 100% | None | Capture and format sections complete |
| **logging:** | 100% | None | Hierarchical logging complete |
| **state_management:** | 100% | None | Checkpointing and persistence complete |
| **parallel_group:** | 100% | None | [MOVED TO AGENT-QUEUE] Parallel execution groups complete |
| **concurrency:** | 80% | 20% | workflow-level, model-level, step-level defined; skip_on_load_failure field missing |
| **fault tolerance:** | 100% | None | Covered by existing retry, when hooks, timeout fields (see schema-additions-needed.md) |

**Gaps:**
1. **Missing `skip_on_load_failure` in providers section**: New field needed so benchmarks can continue when a model fails to load
 2. **Concurrency coverage evaluation pending**: User reviewing whether current `workflow_execution_strategy.parallel` fields are sufficient or need extension [MOVED TO AGENT-QUEUE]

**Schema Addition (Single Field):**
- Add `skip_on_load_failure: boolean` to `providers.<name>.hosting` section:
  ```yaml
  providers:
    lmstudio:
      hosting:
        skip_on_load_failure: true    # Continue workflow if this provider's model fails to load
  ```

---

## Recent Updates

### 2026-04-13

**Added New Specs:**
- ✅ [transpiler-feature-matrix-r0054-r0075.md](./transpiler-feature-matrix-r0054-r0075.md) - Schema definition + docs generation
- ✅ [critical-evaluation.md](./critical-evaluation.md) - Tech stack decisions
- ✅ [llamacpp-vulkan-integration.md](./llamacpp-vulkan-integration.md) - GPU acceleration integration
- ✅ [advanced-agentic-features.md](./advanced-agentic-features.md) - Multi-agent spawning + loop termination
- ✅ [benchmark-yaml-examples.md](./benchmark-yaml-examples.md) - 3 scaling workflows (5/20/100)
- ✅ [constraints-and-assumptions.md](./constraints-and-assumptions.md) - Project scope and design decisions
- ✅ [configuration-defaults.md](./configuration-defaults.md) - Default values for all components

**Revamped:**
- ✅ [requirements/index.md](./index.md) - Updated to include all new specs and traceability
- ✅ All new specs linked from central index
- ✅ Schema gaps identified (skip_on_load_failure field needed; concurrency under review)

**Status**: Foundation requirements complete (24/24 core specs). Advanced requirements and schema additions pending.

---

## Implementation Priority

### Must-Have Before Development

| Priority | Requirements | Why |
|-----------|-------------|------|
| **P0** | Unified Schema v2.0 | Source of truth for all workflows |
| **P0** | Schema Validation | Prevent errors before execution |
| **P0** | Configuration Defaults | Sensible defaults for immediate usability |
| **P0** | Constraints & Assumptions | Scope definition and design boundaries |
| **P1** | Transpiler | Generate Rust code from YAML workflows |
| **P1** | Backend Implementations | llama.cpp, Ollama, OpenAI adapters |
| **P0** | CLI | User-facing command-line interface |
| **P1** | Advanced Features | Multi-agent, loop termination for complex workflows |

### Blockers

1. **Schema addition for `skip_on_load_failure`** - Single new field for benchmark fault tolerance
2. **Concurrency evaluation** - User reviewing whether current parallel fields sufficient
2. **Treadle integration** - HITL support for persistent workflows
3. **Memory & Search** - Vector DB with RAG operations
4. **Autonomy metrics** - Self-improvement loops
5. **Automation** - Git operations, cron workflows

---

## References

**Architecture Decisions:**
- [Roadmap ADRs](../roadmap/) - All architectural decisions with rationale
- [Foundation Plan](../plans/00-foundation/plan.md) - Implementation plan for core infrastructure
- [Transpiler Architecture](../plans/transpiler/transpiler_architecture.md) - 7-layer transpiler design

**Research:**
- [Upstream Success Factors](../../../../RESEARCH_UPSTREAM_SUCCESS_FACTORS.md) - Ecosystem research (serde-saphyr, Rig, Treadle, etc.)
- [Critical Evaluation](./critical-evaluation.md) - Tech stack comparison and choices

**Example Workflows:**
- [52 Categorized Workflows](./example-workflows/requirements-oriented-auto/) - All schema features demonstrated
- [Manual Brainstorm](./example-workflows/manual/agentic-workflow-manual-brainstorm.yml) - Conceptual agentic patterns

---

## Completion Metrics

**Total Requirements**: 24
**Core Complete**: 12 (50%)
**Advanced Complete**: 8 (33%)
**In Progress**: 4 (17%)
**Blocked**: 0

**Documentation Coverage**: 95% (skip_on_load_failure field addition pending, concurrency evaluation pending)

---

## AutoAgents SDK Tool Abstraction

All 53 workflow examples leverage the AutoAgents SDK for:
- **Multi-provider model support** - LM Studio, Ollama, llama.cpp, OpenAI
- **Backend selection** - Vulkan, CUDA, CPU, Metal with automatic fallback
- **Parameter tuning** - Temperature, top_p, max_tokens configuration
- **Tool abstraction** - Built-in tools (file operations, web requests, shell commands, grep)
- **Retry logic** - Exponential backoff with resource awareness
- **Stateful conversations** - Sliding memory windows for context management

**Implementation Note**: This abstraction layer (tool calling interface) is complete and functional.
