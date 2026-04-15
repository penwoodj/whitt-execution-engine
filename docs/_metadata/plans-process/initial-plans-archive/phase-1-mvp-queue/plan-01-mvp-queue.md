# Plan-01: MVP Queue & Scheduler

**Plan ID**: plan-01
**Phase**: Phase 1 - MVP Queue & Scheduler
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0002 (MVP Queue and Scheduler Safety)
**Related Research Plan**: research-plan-02-mvp-queue-cli.yml
**Estimated Time**: 6-8 weeks
**Dependencies**: plan-00 (Foundation)

---

## Overview

This plan implements the MVP queue and scheduler system for managing chat sessions as scoped executable work containers. The queue provides title-based organization, live execution meaning, reprioritization, pause/resume, and human-gated safety controls.

**Key Deliverables**:
- ChatSession work containers
- Queue state machine
- Persistent job queue storage
- Scheduler with prioritization, cancellation, retry
- Human-gated safety (confirmations, staged diffs)
- CLI control surface for queue management
- Multiple execution modes (serial, parallel, hybrid)

---

## Code Review

### ADR-0002 Summary

**Decision**: Adopt a ChatSession-based queue with human-gated safety controls and CLI control surface.

**Key Requirements**:
1. **Chat as Work Container**: Each chat is a scoped executable work container
2. **Queue UX with Live Meaning**: Titles and real-time execution status
3. **Policy Sliders**: Runtime, review, rigor, scope, concurrency, efficiency, logging, context-budget
4. **Human Gating**: Structured clarification and confirmation for dangerous operations
5. **Declared Effects**: Previews, diffs before execution
6. **Queue Semantics**: Worker pools, persistence, cancellation, result retrieval
7. **Staged Diffs**: Preview, confirm, then execute
8. **CLI/TUI-First**: CLI interface before richer UI

---

## Web Research

### Research Area 1: Scheduler State Machine Patterns

**Research Question**: What state machine patterns best model queue lifecycle?

**Recommended Sources**:
- [docs.rs](https://docs.rs) - Rust state machine crates
- [github.com](https://github.com) - Open source queue implementations
- [tokio.rs](https://tokio.rs) - Async scheduling patterns

**Status**: Not Started

---

### Research Area 2: Persistent Job Queues

**Research Question**: What persistent queue patterns work best with local-first storage?

**Recommended Sources**:
- [sqlite.org](https://sqlite.org) - SQLite for persistence
- [redis.io](https://redis.io) - Queue patterns (for reference)
- [tokio.rs](https://tokio.rs) - Async storage

**Status**: Not Started

---

### Research Area 3: CLI Operator Experience

**Research Question**: What CLI patterns best support queue management?

**Recommended Sources**:
- [clap.rs](https://clap.rs) - Argument parsing
- [ratatui.rs](https://ratatui.rs) - Terminal UI
- [github.com](https://github.com) - Open source queue CLI examples

**Status**: Not Started

---

### Research Area 4: Human-in-the-Loop Controls

**Research Question**: What patterns best implement human gating?

**Recommended Sources**:
- [UX design patterns](https://lawsofux.com)
- [Safety systems](https://safetydesignpatterns.com)
- [Open source examples](https://github.com)

**Status**: Not Started

---

### Research Area 5: Diff & Approval Patterns

**Research Question**: What diff and approval patterns work well for staged execution?

**Recommended Sources**:
- [git diff patterns](https://git-scm.com)
- [Approval workflows](https://github.com/features/actions)
- [CLI approval patterns](https://clig.dev)

**Status**: Not Started

---

## Implementation Plan

### Phase 1: ChatSession Containers

**Tasks**:
1. Define `ChatSession` struct with all metadata
2. Implement session lifecycle management
3. Add session ID generation
4. Implement session storage
5. Add session retrieval

**Related Requirements**: R07, R08
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 2: Queue State Machine

**Tasks**:
1. Define queue states (pending, running, paused, completed, cancelled)
2. Implement state transitions
3. Add state validation
4. Implement state persistence
5. Add state change events

**Related Requirements**: R08, R09
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 3: Persistent Storage

**Tasks**:
1. Design queue schema for SQLite
2. Implement storage layer
3. Add queue persistence
4. Implement queue recovery
5. Add data migration support

**Related Requirements**: R24
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 4: Scheduler Core

**Tasks**:
1. Implement worker pool management
2. Add job prioritization
3. Implement job execution loop
4. Add cancellation support
5. Implement retry logic
6. Add result retrieval

**Related Requirements**: R24
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 5: Human Gating

**Tasks**:
1. Implement operation classification
2. Add confirmation prompts
3. Implement diff preview
4. Add staged execution
5. Implement approval tracking

**Related Requirements**: R13, R20, R29
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: CLI Control Surface

**Tasks**:
1. Design CLI command structure
2. Implement queue list command
3. Implement queue operations (pause, resume, cancel)
4. Implement job submission
5. Add interactive TUI mode
6. Implement status display

**Related Requirements**: R05, R08
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 7: Execution Modes

**Tasks**:
1. Implement serial execution mode
2. Implement parallel execution mode
3. Implement hybrid execution mode
4. Add mode switching
5. Implement concurrency control
6. Add mode-specific optimizations

**Related Requirements**: R17
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Command**: `cargo test --lib`

**Coverage Areas**:
- ChatSession lifecycle
- Queue state transitions
- Storage operations
- Scheduling logic
- Human gating
- CLI commands

**Status**: Not Started

---

### Layer 2: Integration Tests

**Command**: `cargo test --test '*'`

**Coverage Areas**:
- Queue to scheduler integration
- Storage to state persistence
- CLI to queue operations
- Human gating to execution

**Status**: Not Started

---

### Layer 3: Property-Based Tests

**Command**: `cargo test` with `proptest`

**Coverage Areas**:
- Queue state machine invariants
- Prioritization properties
- Retry loop convergence

**Status**: Not Started

---

### Layer 4: End-to-End Tests

**Command**: `cargo test --test '*e2e*'`

**Coverage Areas**:
- Full job submission lifecycle
- CLI queue management
- Human gating flow
- Multi-mode execution

**Status**: Not Started

---

## Verification Checkpoints

### Checkpoint 001: Phase 1 Complete (ChatSession)
**Date**: TBD
**Verification Layers**: 1, 2
**Sign-off Criteria**:
- [ ] ChatSession struct defined
- [ ] Lifecycle works
- [ ] Storage works
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 002: Phase 2 Complete (Queue State Machine)
**Date**: TBD
**Verification Layers**: 1, 2, 3
**Sign-off Criteria**:
- [ ] States defined
- [ ] Transitions work
- [ ] Invariants hold
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 003: Phase 3 Complete (Persistent Storage)
**Date**: TBD
**Verification Layers**: 1, 2, 4
**Sign-off Criteria**:
- [ ] Schema designed
- [ ] Storage works
- [ ] Recovery works
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 004: Phase 4 Complete (Scheduler Core)
**Date**: TBD
**Verification Layers**: 1, 2, 3, 4
**Sign-off Criteria**:
- [ ] Workers managed
- [ ] Jobs execute
- [ ] Cancellation works
- [ ] Retry works
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 005: Phase 5 Complete (Human Gating)
**Date**: TBD
**Verification Layers**: 1, 2, 4
**Sign-off Criteria**:
- [ ] Classification works
- [ ] Confirmations work
- [ ] Diffs show
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 006: Phase 6 Complete (CLI Control Surface)
**Date**: TBD
**Verification Layers**: 1, 2, 4
**Sign-off Criteria**:
- [ ] Commands work
- [ ] TUI works
- [ ] Display is clear
- [ ] Tests pass

**Status**: Not Started

---

### Checkpoint 007: Phase 7 Complete (Execution Modes)
**Date**: TBD
**Verification Layers**: 1, 2, 4
**Sign-off Criteria**:
- [ ] All modes work
- [ ] Switching works
- [ ] Concurrency controlled
- [ ] Tests pass

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Phase | Status | Progress |
|-------|--------|----------|
| ChatSession | Not Started | 0% |
| Queue State Machine | Not Started | 0% |
| Persistent Storage | Not Started | 0% |
| Scheduler Core | Not Started | 0% |
| Human Gating | Not Started | 0% |
| CLI Control Surface | Not Started | 0% |
| Execution Modes | Not Started | 0% |
| **Overall** | **Not Started** | **0%** |

---

### Verification Progress

| Layer | Status |
|-------|--------|
| Layer 1: Unit Tests | Not Started |
| Layer 2: Integration Tests | Not Started |
| Layer 3: Property-Based Tests | Not Started |
| Layer 4: End-to-End Tests | Not Started |

---

### Research Progress

| Area | Status |
|------|--------|
| Scheduler State Machine Patterns | Not Started |
| Persistent Job Queues | Not Started |
| CLI Operator Experience | Not Started |
| Human-in-the-Loop Controls | Not Started |
| Diff & Approval Patterns | Not Started |

---

## Dependencies

### Blocks

- **plan-00** (Foundation)

### Unblocks

- **plan-02** (CLI & Backends)
- **plan-03** (Glyphnova UI)
- **plan-04** (Quality Loops)
- **plan-05** (Memory & Search)
- **plan-06** (Automation)
- **plan-07** (Autonomy & Metrics)

---

## Quality Gates

### ADR-Specific Gates

1. Chat as scoped executable work container
2. Queue UX with titles and live execution meaning
3. Policy sliders compile deterministically
4. Human gating with structured clarification
5. Declared effects, previews, diff-first behavior
6. Queue semantics: worker pools, persistence, cancellation, results
7. CLI/TUI-first path

### Critical Review Factors

1. ADR Compliance
2. Requirements Satisfaction
3. Test Coverage
4. Documentation
5. Integration
6. Security
7. OpenCode Compatibility

---

## Next Steps

1. Complete web research in 5 areas
2. Write tests for Phase 1
3. Implement Phase 1
4. Verify Checkpoint 001
5. Continue through Phase 7

---

## Execution Commands

```bash
./implement.sh --start plan-01
./implement.sh --continue plan-01
./implement.sh --verify plan-01
./implement.sh --complete plan-01
```

---

## References

- [ADR-0002](../roadmap/adr-0002-mvp-queue-scheduler-safety.yml)
- [research-plan-02-mvp-queue-cli.yml](../roadmap/research-plan-02-mvp-queue-cli.yml)
- [plan-00](../phase-0-foundation/plan-00-foundation.md)

---

**Last Updated**: 2026-03-27
