# Transpiler Requirements Index

**Version**: 4.0  
**Date**: 2026-03-04  
**Repository**: https://github.com/penwoodj/yaml-to-rust-agentsdk  
**Last Updated**: 2026-03-07

---

## Overview

This index organizes the transpiler requirements into logical groupings with implementation priority, complexity assessment, and ADR traceability. Requirements are ordered by implementation phase to enable progressive delivery from foundation to advanced features.

---

## Requirements Groupings

### Phase 1: Foundation (High Priority, Medium Complexity)

**Category**: Core transpiler architecture and schema foundation

**Related ADRs**:
- [ADR-0001: Foundation Compiler Contract](../roadmap/adr-0001-foundation-compiler-contract.yml) - Schema system, IR compilation, policy compilation
- [ADR-0002: MVP Queue and Scheduler](../roadmap/adr-0002-mvp-queue-scheduler-safety.yml) - ChatSession, queue lifecycle, human gating

**Requirements Documents**:
- [01-core-functionality.md](./01-core-functionality.md) - Primary purpose and key attributes
- [02-yaml-schema.md](./02-yaml-schema.md) - Schema characteristics and elements
- [04-tech-stack.md](./04-tech-stack.md) - Selected libraries and rationale
- [07-project-structure.md](./07-project-structure.md) - Repository layout and branching

**Implementation Order**: 1-4  
**Dependencies**: None (foundation phase)  
**Complexity**: Medium (requires schema design, IR architecture, tech stack integration)  
**Risk**: Low (well-understood patterns from ADR research)

**Key Requirements**:
- R15: Schema-based YAML DSL for agentic behavior
- R16: Prompt → YAML workflow generation planning layer
- R19: Explicit DAG/state-machine semantics, composability, determinism
- R22: YAML → Rust AgentSDK execution target
- R01: System identity (compiler-centered local agentic orchestration)
- R03: Local-first operation
- R28: Per-chat workspace and .glyphnova/ system-of-record
- R31: Auditable local artifacts, versioned runs, hashes, privacy-preserving defaults

---

### Phase 2: MVP Execution (High Priority, High Complexity)

**Category**: Queue, scheduler, human gating, and staged operations

**Related ADRs**:
- [ADR-0002: MVP Queue and Scheduler](../roadmap/adr-0002-mvp-queue-scheduler-safety.yml) - Queue semantics, human gating, staged diffs
- [ADR-0003: CLI and Backends](../roadmap/adr-0003-cli-backends-networking-boundary.yml) - CLI surface, provider abstraction

**Requirements Documents**:
- [03-agentic-capabilities.md](./03-agentic-capabilities.md) - Tools, permissions, loops, multi-agent spawning
- [05-llm-providers.md](./05-llm-providers.md) - Provider support and hardware acceleration
- [08-readme-content.md](./08-readme-content.md) - Documentation and examples

**Implementation Order**: 5-8  
**Dependencies**: Phase 1 (foundation)  
**Complexity**: High (state machine complexity, human gating logic, permission system)  
**Risk**: Medium (requires careful safety design)

**Key Requirements**:
- R07: Chat as scoped executable work container
- R08: Queue UX with titles and live execution meaning
- R12: Policy sliders (runtime, review, rigor, scope, concurrency, efficiency, logging, context-budget)
- R13: Human gating (structured clarification and confirmation)
- R20: Declared effects, previews, and diff-first behavior
- R24: Queue semantics, worker pools, persistence, cancellation, result retrieval
- R29: Create/edit/delete with staged diffs and confirmation
- R05: CLI/TUI-first path then richer desktop UI

---

### Phase 3: Backends and Providers (High Priority, Medium Complexity)

**Category**: Model provider abstraction and hardware acceleration

**Related ADRs**:
- [ADR-0003: CLI and Backends](../roadmap/adr-0003-cli-backends-networking-boundary.yml) - Provider abstraction, packaging, networking boundary
- [ADR-0004: Glyphnova UI](../roadmap/adr-0004-glyphnova-ui-control-plane.yml) - Desktop shell over runtime

**Requirements Documents**:
- [05-llm-providers.md](./05-llm-providers.md) - Provider support and hardware acceleration
- [04-tech-stack.md](./04-tech-stack.md) - llama.cpp bindings, Vulkan support

**Implementation Order**: 9-11  
**Dependencies**: Phase 1, Phase 2 (MVP queue)  
**Complexity**: Medium (provider abstraction, hardware integration)  
**Risk**: Medium (hardware compatibility issues, driver dependencies)

**Key Requirements**:
- R32: Model provider abstraction for multiple local runners
- R33: Load/unload models, multiple instances, multiple models in parallel
- R23: Packaging choice (standalone vs backend crate vs inlining)
- R27: Progressive results and partial answers
- R18: Tools and custom Rust tools as first-class nodes

---

### Phase 4: Advanced Agentic Features (Medium Priority, High Complexity)

**Category**: Loops, validation, multi-agent spawning, benchmarking

**Related ADRs**:
- [ADR-0005: Quality Loops and Benchmarks](../roadmap/adr-0005-quality-loops-benchmarks-artifact-workflows.yml) - Generate-verify-repair, benchmark suites
- [ADR-0006: Memory and Search](../roadmap/adr-0006-memory-search-scraping.yml) - Local memory, retrieval, scraping

**Requirements Documents**:
- [03-agentic-capabilities.md](./03-agentic-capabilities.md) - Loop control, validation, multi-agent spawning
- [12-advanced-features.md](./12-advanced-features.md) - Detailed loop and multi-agent specifications

**Implementation Order**: 12-16  
**Dependencies**: Phase 1, Phase 2, Phase 3  
**Complexity**: High (complex loop logic, aggregation strategies, validation criteria)  
**Risk**: Medium (loop termination correctness, resource exhaustion)

**Key Requirements**:
- R25: Generate → verify → repair review loop semantics
- R04: Depth-over-speed and capability-normalized outcomes
- R21: Artifact semantics (workflows are inspectable, editable artifacts)
- R30: Observability (logging levels, summaries, graphs, reports, provenance)

---

### Phase 5: Benchmarking and Observability (Medium Priority, Medium Complexity)

**Category**: Performance measurement, model sweep workflows, metrics collection

**Related ADRs**:
- [ADR-0005: Quality Loops and Benchmarks](../roadmap/adr-0005-quality-loops-benchmarks-artifact-workflows.yml) - Benchmark harness design
- [ADR-0008: Autonomy and Metrics](../roadmap/adr-0008-autonomy-and-metrics.yml) - Objective measurements

**Requirements Documents**:
- [12-advanced-features.md](./12-advanced-features.md) - Model sweep workflows, benchmarking metrics
- [15-configuration-defaults.md](./15-configuration-defaults.md) - Profiling and metrics configuration

**Implementation Order**: 17-20  
**Dependencies**: Phase 1, Phase 2, Phase 3, Phase 4  
**Complexity**: Medium (metrics collection, aggregation logic)  
**Risk**: Low (observability is well-understood)

**Key Requirements**:
- R04: Depth-over-speed and capability-normalized outcomes
- R30: Observability with logging levels, summaries, graphs, reports, provenance

---

### Phase 6: Automation and Scheduling (Low Priority, Medium Complexity)

**Category**: Cron scheduling, git experiments, autonomous loops

**Related ADRs**:
- [ADR-0007: Cron and Git Refinement](../roadmap/adr-0007-cron-git-refinement.yml) - Scheduled workflows, git experiments
- [ADR-0008: Autonomy and Metrics](../roadmap/adr-0008-autonomy-and-metrics.yml) - Autonomous loops, metrics

**Requirements Documents**:
- [12-advanced-features.md](./12-advanced-features.md) - Loop control (deferred features)
- [15-configuration-defaults.md](./15-configuration-defaults.md) - Default loop configuration

**Implementation Order**: 21-24  
**Dependencies**: Phase 1, Phase 2, Phase 3, Phase 4, Phase 5  
**Complexity**: Medium (scheduling, git automation)  
**Risk**: Medium (operational complexity, storage growth)

**Key Requirements**:
- R17: Multiple workflow archetypes and scheduling modes
- R24: Queue semantics (extension for scheduled workflows)
- R26: Long-running behavior (background, semi-endless, endless, resumable)
- R31: Auditable local artifacts (extension for experiment tracking)

---

### Phase 7: UI and Desktop Shell (Low Priority, High Complexity)

**Category**: Desktop UI, multi-zoom navigation, visualization

**Related ADRs**:
- [ADR-0004: Glyphnova UI](../roadmap/adr-0004-glyphnova-ui-control-plane.yml) - Desktop shell, queue visualization

**Requirements Documents**:
- [08-readme-content.md](./08-readme-content.md) - UI-related documentation
- [15-configuration-defaults.md](./15-configuration-defaults.md) - UI defaults

**Implementation Order**: 25-28  
**Dependencies**: Phase 1, Phase 2, Phase 3  
**Complexity**: High (UI architecture, state management, visualization)  
**Risk**: High (UI delivery waits on backend readiness)

**Key Requirements**:
- R05: CLI/TUI-first path then richer desktop UI
- R08: Queue UX (extension for UI visualization)
- R10: Multi-zoom navigation (nested topics, summaries, graphs)
- R11: Context safety (visible scope, explicit context-switch confirmation)

---

## Implementation Priority Matrix

| Priority | Phase | Complexity | Risk | Dependencies | Time Estimate |
|----------|-------|------------|------|--------------|---------------|
| **Critical** | Phase 1: Foundation | Medium | Low | None | 4-6 weeks |
| **Critical** | Phase 2: MVP Execution | High | Medium | Phase 1 | 6-8 weeks |
| **High** | Phase 3: Backends | Medium | Medium | Phase 1, 2 | 3-4 weeks |
| **Medium** | Phase 4: Advanced Features | High | Medium | Phase 1, 2, 3 | 4-6 weeks |
| **Medium** | Phase 5: Benchmarking | Medium | Low | Phase 1, 2, 3, 4 | 2-3 weeks |
| **Low** | Phase 6: Automation | Medium | Medium | Phase 1-5 | 3-4 weeks |
| **Low** | Phase 7: UI | High | High | Phase 1, 2, 3 | 6-8 weeks |

**Total Estimated Time**: 28-39 weeks (7-10 months) for full implementation

---

## Complexity Assessment

### Low Complexity (Well-Understood Patterns)
- Tech stack selection (research complete)
- File operations and tool implementations
- CLI interface design
- Observability and logging

### Medium Complexity (Requires Design Decisions)
- YAML schema design and validation
- IR compilation pipeline
- Provider abstraction layer
- Queue state machine
- Permission system
- Scheduling and automation

### High Complexity (Novel Problems)
- Human gating and safety controls
- Staged effects with preview and approval
- Loop control with validation criteria
- Multi-agent spawning and aggregation
- Desktop UI architecture
- Autonomous loop contracts

---

## Risk Assessment

### Low Risk
- **Foundation phase**: Well-understood compiler patterns, research complete
- **Benchmarking**: Observability is standard, proven approaches exist

### Medium Risk
- **MVP queue**: State machine complexity, human gating edge cases
- **Backends**: Hardware compatibility (Vulkan, CUDA, Metal), driver issues
- **Advanced features**: Loop termination correctness, resource exhaustion
- **Automation**: Operational complexity, storage growth management

### High Risk
- **UI phase**: Backend dependency, state synchronization, performance

---

## Requirements Traceability Matrix

| Requirement | ADR | Phase | Priority | Implementation Status |
|-------------|-----|-------|----------|----------------------|
| R01: System identity | ADR-0001 | Phase 1 | Critical | Not started |
| R02: OSS ethos | ADR-0000 | Phase 1 | High | Not started |
| R03: Local-first | ADR-0001 | Phase 1 | Critical | Not started |
| R04: Depth-over-speed | ADR-0005, ADR-0008 | Phase 4, 5 | Medium | Not started |
| R05: CLI-first | ADR-0002, ADR-0003 | Phase 2, 3 | Critical | Not started |
| R06: Long-term direction | ADR-0008 | Phase 6 | Low | Not started |
| R07: Chat as work container | ADR-0002 | Phase 2 | Critical | Not started |
| R08: Queue UX | ADR-0002 | Phase 2 | Critical | Not started |
| R09: Reprioritization | ADR-0002 | Phase 2 | Medium | Not started |
| R10: Multi-zoom navigation | ADR-0004 | Phase 7 | Low | Not started |
| R11: Context safety | ADR-0002 | Phase 2 | High | Not started |
| R12: Policy sliders | ADR-0001, ADR-0002 | Phase 1, 2 | Critical | Not started |
| R13: Human gating | ADR-0002 | Phase 2 | Critical | Not started |
| R14: Workflow selection | ADR-0002 | Phase 2 | Medium | Not started |
| R15: Schema-based YAML DSL | ADR-0001 | Phase 1 | Critical | Not started |
| R16: Prompt → YAML | ADR-0001 | Phase 1 | High | Not started |
| R17: Workflow archetypes | ADR-0002, ADR-0007 | Phase 2, 6 | Medium | Not started |
| R18: Tools and custom Rust | ADR-0003 | Phase 3 | High | Not started |
| R19: DAG/state-machine | ADR-0001 | Phase 1 | Critical | Not started |
| R20: Effect declaration | ADR-0002 | Phase 2 | Critical | Not started |
| R21: Artifact semantics | ADR-0005 | Phase 4 | Medium | Not started |
| R22: YAML → Rust target | ADR-0001 | Phase 1 | Critical | Not started |
| R23: Packaging choice | ADR-0003 | Phase 3 | High | Not started |
| R24: Queue semantics | ADR-0002 | Phase 2 | Critical | Not started |
| R25: Generate-verify-repair | ADR-0005 | Phase 4 | Medium | Not started |
| R26: Long-running behavior | ADR-0007, ADR-0008 | Phase 6 | Low | Not started |
| R27: Progressive results | ADR-0003 | Phase 3 | High | Not started |
| R28: Per-chat workspace | ADR-0001 | Phase 1 | Critical | Not started |
| R29: Staged diffs | ADR-0002 | Phase 2 | Critical | Not started |
| R30: Observability | ADR-0005, ADR-0006, ADR-0008 | Phase 4, 5, 6 | Medium | Not started |
| R31: Auditable artifacts | ADR-0001, ADR-0007 | Phase 1, 6 | High | Not started |
| R32: Model provider abstraction | ADR-0003 | Phase 3 | Critical | Not started |
| R33: Model lifecycle | ADR-0003 | Phase 3 | High | Not started |
| R34: Routing strategy | ADR-0003 | Phase 3 | Low | Not started |

---

## Critical Path

The critical path through implementation:

```
Phase 1 (Foundation)
  ↓
Phase 2 (MVP Execution)
  ↓
Phase 3 (Backends)
  ↓
Phase 4 (Advanced Features) ←─┐
  ↓                           │
Phase 5 (Benchmarking) ───────┘
  ↓
Phase 6 (Automation)
  ↓
Phase 7 (UI)
```

**Parallelization Opportunities**:
- Phase 4 and Phase 5 can partially overlap (benchmarking can start while advanced features are in progress)
- Phase 7 can start UI design in parallel with Phase 4-6

---

## Next Steps

1. **Begin Phase 1 implementation** (foundation compiler contract)
   - Implement schema system with JSON Schema and schemars
   - Build WorkflowIR compiler pipeline
   - Establish .glyphnova/ directory structure

2. **Validate ADR research** against actual implementation
   - Confirm schema patterns work in practice
   - Validate IR compilation performance
   - Test local-first storage patterns

3. **Create implementation tracking** for each phase
   - Break down each phase into specific tasks
   - Assign complexity and time estimates
   - Identify blockers and dependencies

---

## References

- **Full Requirements Document**: [../requirements.mdc](../requirements.mdc)
- **Roadmap ADRs**: [../roadmap/](../roadmap/)
- **Research Plans**: [../roadmap/research-plan-*.yml](../roadmap/)
- **Completion Report**: [../roadmap/research/completion-report.yml](../roadmap/research/completion-report.yml)
