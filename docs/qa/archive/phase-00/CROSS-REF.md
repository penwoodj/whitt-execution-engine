# Phase 00: Cross-References

## Plan Files
- **[Plan](../../plans/00-foundation/plan.md)** - Foundation implementation plan
- **[Tasks](../../plans/00-foundation/tasks/)** - Detailed task implementation guides

## Schema Sections

Phase 0 implements Sections 1, 2, 13-18 of the unified workflow schema:

| Section | Schema Ref | Implementation | Status |
|----------|-------------|---------------|--------|
| Section 1: Workflow Identification | Lines 14-20 (docs/schema/unified-workflow-schema.yml) | src/schema/identification.rs | 🔵 DEFERRED |
| Section 2: Model Configuration | Lines 27-57 | src/schema/model.rs | 🔵 DEFERRED |
| Section 13: Workspace Configuration | Lines 503-598 | src/schema/workspace.rs | 🔵 DEFERRED |
| Section 14: Features Demonstrated | Lines 64-158 | src/schema/features.rs | 🔵 DEFERRED |
| Section 15: Variable Interpolation | Lines 726-740 | src/interpolation.rs | 🔵 DEFERRED |
| Section 16: Default Behavior | Lines 740-805 | src/defaults.rs | 🔵 DEFERRED |
| Section 17: Scope & Inheritance | Lines 726-740 | src/policy.rs | 🔵 DEFERRED |
| Section 18: Numeric Thresholds | Lines 74-88 | src/validation/thresholds.rs | 🔵 DEFERRED |

## Related QA

### Extended POC QA
- **[QA Areas](../extended-poc/QA-AREAS-EXTENDED-POC.md)** - Areas 1-18 (provider config, parsing, backends, etc.)
- **[Test Procedures](../extended-poc/QA-TEST-PROCEDURES-EXTENDED-POC.md)** - EPOC-001 through EPOC-087 test procedures

### POC Docker QA
- **[QA Areas](../poc-local-llm-docker/QA-AREAS.md)** - Areas 1-10 (server management, model management, chat, etc.)
- **[Status](../poc-local-llm-docker/QA-AREAS.md)** - CONFIRMED WORKING status for all areas

## Validation Criteria

### Framework
- **[Framework](../../plans/validation-criteria/framework.md)** - 7-layer verification system (Unit, Integration, Property, E2E, System Log, Live CLI, Benchmark)

### Phase-Specific Criteria
- **Phase-00 Criteria](../../plans/validation-criteria/phase-00-criteria.md)** - Foundation phase measurable validation criteria *(if exists)*

## Architecture Decisions

### ADRs
- **ADR-0001: Compiler Approach** - Selected compiler approach (8.7/10 score) over transpiler (3.8/10)
- **[Full ADR list](../../roadmap/)** - All architectural decisions

## Source Code Map

### Phase 0 Module Structure
```
src/
├── lib.rs                      # Root module exports
├── error.rs                    # Error types (151 lines)
├── schema/                     # Schema modules
│   ├── mod.rs
│   ├── identification.rs          # Section 1 (Lines 14-20)
│   ├── model.rs                 # Section 2 (Lines 27-57)
│   ├── workspace.rs            # Section 13 (Lines 503-598)
│   ├── features.rs              # Section 14 (Lines 64-158)
│   └── policy.rs                # Section 17 (Lines 726-740)
├── parser.rs                   # YAML parsing (serde-saphyr)
├── interpolation.rs              # Variable interpolation
├── ir/                         # WorkflowIR internal representation
│   ├── mod.rs
│   ├── types.rs
│   └── compilation.rs
├── validation/                  # Validation modules
│   ├── dag.rs                   # DAG validation
│   ├── thresholds.rs           # Section 18 (Lines 74-88)
│   └── policy.rs               # Policy compilation
└── storage/                    # Persistence layer
    ├── sled.rs                  # sled KV operations
    └── workspace.rs             # Workspace management
```

## Task Mapping to QA Areas

| Task | QA Area | Dependencies | Est. Time |
|-------|-----------|-------------|------------|
| Task 00: Cargo Project Setup | QA-00-01 | None | 2 hours |
| Task 01: Error Module | QA-00-02 | Task 00 | 3 hours |
| Task 02: Schema Types | QA-00-03 | Task 01 | 8 hours |
| Task 03: YAML Parser | QA-00-04, QA-00-06, QA-00-07 | Task 02 | 6 hours |
| Task 04: Variable Interpolation | QA-00-05 | Task 02 | 5 hours |
| Task 05: Workflow IR | QA-00-06 | Task 02 | 6 hours |
| Task 06: IR Compiler | QA-00-07 | Tasks 03, 04, 05 | 8 hours |
| Task 07: DAG Validator | QA-00-08 | Task 06 | 4 hours |
| Task 08: Policy Compiler | QA-00-09 | Task 06 | 5 hours |
| Task 09: Local Storage | QA-00-10 | Task 06 | 6 hours |
| Task 10: Workspace Management | QA-00-11 | Task 09 | 4 hours |
| Task 11: Defaults & Inheritance | QA-00-09 | Task 08 | 5 hours |
| Task 12: Threshold Validation | QA-00-12 | Task 02 | 4 hours |

## Dependencies

### Prerequisites
- Rust 1.75+ installed
- Cargo workspace initialized
- CI/CD pipeline configured

### Phase 1 Dependencies
Phase 0 must be complete before starting Phase 1 (Core Execution Engine):

| Phase 1 Component | Phase 0 Dependency |
|-------------------|---------------------|
| ChatSession Manager | Phase 0 Workflow IR, Storage |
| Queue State Machine | Phase 0 Storage |
| Scheduler | Phase 0 IR, Storage |
| Executor | Phase 0 IR, Schema Types |
| Loop Runner | Phase 0 Schema Types |
| Branch Evaluator | Phase 0 Schema Types, DAG Validator |
| Execution Modes | Phase 0 Schema Types, Workflow IR |
| Observability | Phase 0 Schema Types |
| Retry & Error Handling | Phase 0 Error Module, IR Compiler |
| Human Gating | Phase 0 Schema Types |

## Next Steps

Proceed to [Phase 1 (Core Execution Engine)](../../phase-01/) QA documentation.

---

**Document Version:** 1.0.0
**Last Updated:** 2026-04-26
**Status:** Ready for QA Execution
