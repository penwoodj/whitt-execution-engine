# System of Record (SoT) - Implementation Plans

**Version**: 1.0
**Created**: 2026-03-26
**Status**: In Progress

---

## Overview

This is the master System of Record (SoT) for all implementation plans in the initial-plans directory. It tracks the complete execution status of every plan file, verification checkpoints, and overall progress toward production-ready implementation.

---

## Plan Index

| Plan ID | Name | Phase | Status | Progress | Completion Date |
|---------|-------|-------|----------|-----------------|
| **plan-00** | Foundation Compiler Contract | Phase 0 | Not Started | - |
| **plan-01** | MVP Queue & Scheduler | Phase 1 | Not Started | - |
| **plan-02** | CLI & Backends | Phase 2 | Not Started | - |
| **plan-03** | Glyphnova UI | Phase 3 | Not Started | - |
| **plan-04** | Quality Loops & Benchmarks | Phase 4 | Not Started | - |
| **plan-05** | Memory & Search | Phase 5 | Not Started | - |
| **plan-06** | Automation (Cron & Git) | Phase 6 | Not Started | - |
| **plan-07** | Autonomy & Metrics | Phase 7 | Not Started | - |

---

## Progress Tracking

### Overall Progress
- **Total Plans**: 8
- **Plans Started**: 0
- **Plans Completed**: 0
- **Overall Completion**: 0%

### Phase Progress
- **Phase 0 (Foundation)**: 0%
- **Phase 1 (MVP Queue)**: 0%
- **Phase 2 (CLI & Backends)**: 0%
- **Phase 3 (Glyphnova UI)**: 0%
- **Phase 4 (Quality Loops)**: 0%
- **Phase 5 (Memory & Search)**: 0%
- **Phase 6 (Automation)**: 0%
- **Phase 7 (Autonomy & Metrics)**: 0%

---

## Verification Tracking

Each plan includes 4 verification layers:
1. **Unit Tests**: Individual component testing
2. **Integration Tests**: Multi-component interaction testing
3. **Property-Based Tests**: Fuzzing and edge case testing (1000 iterations each)
4. **End-to-End Tests**: Full workflow execution testing

### Verification Status Matrix

| Plan ID | Unit Tests | Integration Tests | Property Tests | E2E Tests |
|---------|-------------|-------------------|----------------|--------------|
| **plan-00** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-01** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-02** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-03** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-04** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-05** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-06** | ⬜ | ⬜ | ⬜ | ⬜ |
| **plan-07** | ⬜ | ⬜ | ⬜ | ⬜ |

Legend:
- ⬜ Not Started
- ⬛ In Progress
- ✅ Completed

---

## Dependencies & Blocking

| Plan ID | Depends On | Unblocked By | Status |
|---------|------------|---------------|--------|
| **plan-01** | plan-00 | - | Blocked |
| **plan-02** | plan-00, plan-01 | - | Blocked |
| **plan-03** | plan-00, plan-01, plan-02 | - | Blocked |
| **plan-04** | plan-00, plan-01, plan-02, plan-03 | - | Blocked |
| **plan-05** | plan-04 | - | Blocked |
| **plan-06** | plan-01, plan-02, plan-03, plan-04, plan-05 | - | Blocked |
| **plan-07** | plan-06 | - | Blocked |

---

## Critical Path

The critical execution path is:
```
plan-00 (Foundation)
  ↓
plan-01 (MVP Queue)
  ↓
plan-02 (CLI & Backends)
  ↓
plan-03 (Glyphnova UI) ←─┐
  ↓                      │
plan-04 (Quality Loops) ────┘
  ↓
plan-05 (Memory & Search)
  ↓
plan-06 (Automation)
  ↓
plan-07 (Autonomy & Metrics)
```

**Parallelization Opportunities**:
- plan-03 (UI) can start in parallel with plan-04 (Quality Loops)
- plan-07 (Autonomy) can start UI design in parallel with plan-05 and plan-06

---

## Milestones

| Milestone | Description | Target Date | Status |
|-----------|-------------|--------------|--------|
| **M0** | Foundation Complete (plan-00) | TBD | ⬜ |
| **M1** | MVP Queue & Scheduler (plan-01) | TBD | ⬜ |
| **M2** | CLI & Backends (plan-02) | TBD | ⬜ |
| **M3** | Glyphnova UI (plan-03) | TBD | ⬜ |
| **M4** | Quality Loops & Benchmarks (plan-04) | TBD | ⬜ |
| **M5** | Memory & Search (plan-05) | TBD | ⬜ |
| **M6** | Automation (plan-06) | TBD | ⬜ |
| **M7** | Autonomy & Metrics (plan-07) | TBD | ⬜ |
| **M8** | Full Implementation Complete | TBD | ⬜ |

---

## Quality Gates

Before proceeding from one plan to the next, the following quality gates must pass:

### Foundation Quality Gates (plan-00)
- [ ] WorkflowSpec defines all required fields with types
- [ ] WorkflowIR compiles from valid WorkflowSpec deterministically
- [ ] IR validation passes without errors for valid workflows
- [ ] IR serialization produces hash and provenance metadata
- [ ] `.glyphnova/` directory structure is defined and stable
- [ ] Policy fields are deterministic and compile-time resolvable

### MVP Quality Gates (plan-01)
- [ ] enqueue, cancel, reprioritize, resume operations work correctly
- [ ] safe scope enforcement prevents unauthorized file access
- [ ] approval checkpoints fire when expected
- [ ] staged diff generation produces accurate previews
- [ ] persistence survives process restart
- [ ] scheduler respects priority and concurrency constraints

### Backends Quality Gates (plan-02)
- [ ] CLI exposes all necessary queue and scheduler operations
- [ ] Provider abstraction loads and unloads models correctly
- [ ] Networking actions are opt-in with explicit confirmation
- [ ] Progressive results stream to CLI for long-running workflows
- [ ] Packaging strategy produces distributable artifacts

### UI Quality Gates (plan-03)
- [ ] Desktop shell exposes runtime queue state correctly
- [ ] Queue visualization reflects scheduler states
- [ ] Scope indicators prevent context confusion
- [ ] Drag-and-drop operations call same scheduler APIs
- [ ] Navigation provides multiple abstraction levels

### Quality Loops Quality Gates (plan-04)
- [ ] Generate-verify-repair loops execute as runtime semantics
- [ ] Benchmark suites store with complete metadata
- [ ] Observability outputs are structured and queryable
- [ ] File-type benchmarks track quality metrics per language
- [ ] Reports include actionable insights

### Memory & Search Quality Gates (plan-05)
- [ ] Local memory stores and retrieves workflow artifacts reliably
- [ ] Search indexing supports efficient artifact discovery
- [ ] External search is behind explicit policy gates
- [ ] Scraping respects robots.txt and scope restrictions
- [ ] Provenance records capture timestamps and extraction traces

### Automation Quality Gates (plan-06)
- [ ] Cron jobs execute workflows with proper environment isolation
- [ ] Git experiments run in isolated branches
- [ ] Merge proposals are generated with clear diff recommendations
- [ ] Manual refinement events are captured as auditable artifacts
- [ ] Experiment results are tracked and comparable

### Autonomy & Metrics Quality Gates (plan-07)
- [ ] Autonomous loops respect bounded goals and stop conditions
- [ ] Metrics collection captures all relevant performance signals
- [ ] Human override and pause controls are always functional
- [ ] Dashboards provide real-time visibility
- [ ] Observability integrates with `.glyphnova/` artifact system

---

## Change Log

| Date | Change | Author |
|-------|---------|---------|
| 2026-03-26 | Initial SoT creation | Sisyphus |

---

## Next Steps

1. **Begin plan-00 (Foundation) implementation**
   - Execute code reading review
   - Perform web research
   - Update plan with research findings
   - Start with unit tests
   - Implement features incrementally
   - Verify at all 5 test layers
   - Update plan with verified code

2. **Track progress in this SoT**
   - Update plan status after each verification checkpoint
   - Update completion dates when plans finish
   - Update dependency blocking status

3. **Use implementation script**
   - Run `./implement.sh --plan plan-00` to start
   - Script handles verification checkpoints automatically
   - Progress is tracked and reported

---

## Notes

- All plans follow the same structure: Research → Implement → Verify → Document
- Verification is incremental and batched for each plan
- No plan proceeds to implementation until all research is complete
- Critical path must be respected for sequential dependencies
- Parallelization opportunities should be leveraged where possible
