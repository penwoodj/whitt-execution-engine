# Plan-00: Foundation Compiler Contract

**Plan ID**: plan-00
**Phase**: Phase 0 - Foundation
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0001 (Foundation Compiler Contract and Local-First System-of-Record)
**Related Research Plan**: research-plan-01-foundation.yml
**Estimated Time**: 4-6 weeks
**Dependencies**: None (foundation phase)

---

## Overview

This plan implements the foundation architecture for the YAML to Rust AgentSDK transpiler, establishing:
- Canonical execution unit: typed `WorkflowSpec` and compiled `WorkflowIR`
- YAML DSL authoring surface with schema validation
- Local-first system-of-record under `.glyphnova/`
- Policy compilation into deterministic fields
- Complete artifact provenance and replayability

The foundation phase provides the bedrock upon which all subsequent features are built: queue management, CLI backends, UI, quality loops, memory/search, automation, and autonomy/metrics.

**Key Deliverables**:
- Schema system with JSON Schema and `schemars`
- `WorkflowSpec` Rust structs with validation
- `WorkflowIR` typed internal representation
- Compiler pipeline: YAML → `WorkflowSpec` → `WorkflowIR`
- `.glyphnova/` directory structure and persistence layer
- Policy compilation system
- Reproducibility and privacy defaults

---

## Code Review

### ADR-0001 Summary

**Decision**: Adopt a foundation architecture with canonical execution unit, constrained YAML DSL, and local-first system-of-record.

**Key Requirements**:
1. **System Identity**: Compiler-centered local agentic orchestration framework
2. **Local-First Defaults**: All operations default to local storage
3. **YAML DSL Contract**: Constrained, validated against generated schemas
4. **Prompt → YAML Planning Layer**: Planning layer for prompt-to-workflow generation
5. **Explicit DAG/State-Machine Semantics**: Deterministic execution paths
6. **YAML → Rust Execution Target**: Compile to Rust AgentSDK
7. **Per-Chat Workspace**: `.glyphnova/` system-of-record for each session
8. **Auditable Artifacts**: Versioned runs, hashes, privacy-preserving defaults

**Implementation Notes**:
- Prefer typed Rust structs plus schema generation over ad-hoc YAML maps
- Store workflow, policy, logs, summaries, reports, and hashes under `.glyphnova/`
- Define stable ADR-aware directory layout before code generation begins
- Treat validation criteria for v0.1.0 as artifacts stored beside foundation spec

### Requirements Review

From `requirements.mdc` and `schema-consolidated-report.md`:

**R01**: System identity (compiler-centered local agentic orchestration)
**R03**: Local-first operation
**R15**: Schema-based YAML DSL for agentic behavior
**R16**: Prompt → YAML workflow generation planning layer
**R19**: Explicit DAG/state-machine semantics, composability, determinism
**R22**: YAML → Rust AgentSDK execution target
**R28**: Per-chat workspace and `.glyphnova/` system-of-record
**R31**: Auditable local artifacts, versioned runs, hashes, privacy-preserving defaults

### Research Plan Review

**research-plan-01-foundation.yml** is **COMPLETED** with 4 research domains:

1. **Workflow Schema Design**: What schema and validation patterns best support a YAML-authored but strongly typed workflow DSL in Rust?
2. **Typed IR and Compilation**: What canonical internal representation should be used after YAML parsing?
3. **Local-First Storage**: What local artifact layout best preserves replayability and auditability?
4. **ADR Governance**: What ADR structure best fits a rapidly evolving compiler-runtime product?

**Key Findings**:
- All 4 questions completed with evidence reviews
- Report outputs: `foundation-research-report.md`, `workflow-schema-options.csv`, `artifact-layout-recommendation.csv`
- Quality gates met: All recommendations map to cited sources and tradeoffs

---

## Web Research

### Research Area 1: Schema and Validation Patterns

**Research Question**: What schema and validation patterns best support a YAML-authored but strongly typed workflow DSL in Rust?

**Recommended Sources**:
- [json-schema.org](https://json-schema.org) - JSON Schema specification
- [serde.rs](https://serde.rs) - Rust serialization framework
- [schemars.rs](https://graham.cool/schemars/) - JSON Schema generation from Rust types

**Expected Findings**:
- Best practices for deriving schemas from Rust types
- Validation libraries compatible with YAML
- Schema versioning strategies
- Error reporting patterns

**Status**: Not Started

---

### Research Area 2: Typed IR and Compilation

**Research Question**: What canonical internal representation should be used after YAML parsing?

**Recommended Sources**:
- [docs.rs](https://docs.rs) - Rust crate documentation
- [tokio.rs](https://tokio.rs) - Async runtime patterns
- [rustc.rs](https://doc.rust-lang.org/rustc/) - Compiler internals

**Expected Findings**:
- IR design patterns (AST, SSA, etc.)
- Compilation pipeline best practices
- Type system integration
- Optimization opportunities

**Status**: Not Started

---

### Research Area 3: Local-First Storage

**Research Question**: What local artifact layout best preserves replayability and auditability?

**Recommended Sources**:
- [docs.aws.amazon.com](https://docs.aws.amazon.com) - AWS artifact patterns
- [GitHub artifact patterns](https://github.com)
- [Local-first best practices](https://localfirstweb.dev)

**Expected Findings**:
- Directory layout patterns
- Versioning strategies
- Hashing and provenance
- Privacy-preserving storage

**Status**: Not Started

---

### Research Area 4: ADR Governance

**Research Question**: What ADR structure best fits a rapidly evolving compiler-runtime product?

**Recommended Sources**:
- [ADR template](https://adr.github.io) - Architecture Decision Record template
- [Microsoft ADR patterns](https://learn.microsoft.com)
- [Open source ADR examples](https://github.com)

**Expected Findings**:
- ADR lifecycle management
- Versioning and supersession
- Decision tracking
- Quality gates for evolving designs

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Schema System

**Description**: Implement JSON Schema generation and validation for workflow DSL

**Tasks**:
1. Add `schemars` dependency to Cargo.toml
2. Define `WorkflowSpec` Rust struct with all required fields
3. Derive `JsonSchema` for `WorkflowSpec`
4. Implement `serde-saphyr` YAML parsing with validation
5. Add schema validation step in parser
6. Implement error reporting with helpful messages

**Related Requirements**: R15, R16
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 2: WorkflowIR Compiler

**Description**: Implement typed internal representation and compilation pipeline

**Tasks**:
1. Define `WorkflowIR` enum and structs
2. Implement `compile_workflow_spec()` function
3. Add type checking pass
4. Implement DAG validation
5. Add state-machine validation
6. Implement IR serialization
7. Add IR hash computation

**Related Requirements**: R19, R22
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 3: Policy Compilation

**Description**: Implement deterministic policy field compilation

**Tasks**:
1. Define `Policy` struct with all policy fields
2. Implement `compile_policy()` function
3. Add policy validation
4. Implement runtime policy evaluation
5. Add policy enforcement hooks
6. Document policy field semantics

**Related Requirements**: R12, R11
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 4: Local-First Storage

**Description**: Implement `.glyphnova/` system-of-record

**Tasks**:
1. Define `.glyphnova/` directory structure
2. Implement artifact storage layer
3. Add run ID generation
4. Implement versioning system
5. Add hash computation and verification
6. Implement artifact provenance tracking
7. Add privacy-preserving defaults

**Related Requirements**: R03, R28, R31
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 5: Reproducibility & Privacy

**Description**: Implement reproducibility guarantees and privacy features

**Tasks**:
1. Implement deterministic execution tracking
2. Add seed management for randomness
3. Implement private-by-default settings
4. Add artifact encryption support
5. Implement audit logging
6. Add data retention policies

**Related Requirements**: R31
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Command**: `cargo test --lib`

**Purpose**: Test individual functions, structs, and modules

**Coverage Areas**:
- Schema derivation
- YAML parsing and validation
- IR compilation
- Policy compilation
- Storage operations
- Hash computation
- Privacy defaults

**Test Frameworks**: `cargo test`, `rstest`

**Pass Criteria**:
- All unit tests pass
- 100% coverage of new code
- No warnings

**Status**: Not Started

---

### Layer 2: Integration Tests

**Command**: `cargo test --test '*'`

**Purpose**: Test multi-component interaction

**Coverage Areas**:
- YAML → WorkflowSpec → WorkflowIR pipeline
- Policy integration with workflows
- Storage integration with execution
- Reproducibility guarantees

**Test Frameworks**: `cargo test`

**Pass Criteria**:
- All integration tests pass
- API contracts satisfied
- Data flows verified

**Status**: Not Started

---

### Layer 3: Property-Based Tests

**Command**: `cargo test` with `proptest`

**Purpose**: Test edge cases and invariants

**Coverage Areas**:
- Schema validation invariants
- IR compilation properties
- Policy compilation determinism
- Hash correctness
- Version ordering

**Test Frameworks**: `proptest`

**Pass Criteria**:
- All property tests pass (1000 iterations each)
- No shrinking failures
- Invariants preserved

**Status**: Not Started

---

### Layer 4: End-to-End Tests

**Command**: `cargo test --test '*e2e*'`

**Purpose**: Test full workflow execution

**Coverage Areas**:
- Complete YAML to Rust execution
- `.glyphnova/` artifact creation
- Reproducibility guarantees
- Privacy defaults

**Test Frameworks**: `cargo test`

**Pass Criteria**:
- All e2e tests pass
- Full workflows execute correctly
- Artifacts created as expected

**Status**: Not Started

---

## Verification Checkpoints

### Checkpoint 001: Phase 1 Complete (Schema System)

**Date**: TBD
**Description**: Schema system implemented and tested

**Verification Layers Covered**: 1, 2
**Sign-off Criteria**:
- [ ] Schema derives from `WorkflowSpec`
- [ ] YAML parsing validates against schema
- [ ] Error messages are helpful
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Code reviewed

**Status**: Not Started
**Sign-off By**: TBD

---

### Checkpoint 002: Phase 2 Complete (WorkflowIR Compiler)

**Date**: TBD
**Description**: IR compiler implemented and tested

**Verification Layers Covered**: 1, 2, 3
**Sign-off Criteria**:
- [ ] `WorkflowIR` defines canonical representation
- [ ] Compilation pipeline is deterministic
- [ ] Type checking works
- [ ] DAG validation works
- [ ] State-machine validation works
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All property tests pass
- [ ] Code reviewed

**Status**: Not Started
**Sign-off By**: TBD

---

### Checkpoint 003: Phase 3 Complete (Policy Compilation)

**Date**: TBD
**Description**: Policy compilation implemented

**Verification Layers Covered**: 1, 2
**Sign-off Criteria**:
- [ ] `Policy` struct defined
- [ ] Policy compilation is deterministic
- [ ] Policy validation works
- [ ] Runtime evaluation works
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Code reviewed

**Status**: Not Started
**Sign-off By**: TBD

---

### Checkpoint 004: Phase 4 Complete (Local-First Storage)

**Date**: TBD
**Description**: `.glyphnova/` system-of-record implemented

**Verification Layers Covered**: 1, 2, 3, 4
**Sign-off Criteria**:
- [ ] Directory structure defined and stable
- [ ] Artifact storage works
- [ ] Run ID generation is unique
- [ ] Versioning system works
- [ ] Hash computation is correct
- [ ] Provenance tracking works
- [ ] Privacy defaults enforced
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All property tests pass
- [ ] All e2e tests pass
- [ ] Code reviewed

**Status**: Not Started
**Sign-off By**: TBD

---

### Checkpoint 005: Phase 5 Complete (Reproducibility & Privacy)

**Date**: TBD
**Description**: Reproducibility and privacy features implemented

**Verification Layers Covered**: 1, 2, 4
**Sign-off Criteria**:
- [ ] Execution tracking is deterministic
- [ ] Seed management works
- [ ] Privacy-by-default enforced
- [ ] Encryption support works
- [ ] Audit logging works
- [ ] Data retention policies enforced
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All e2e tests pass
- [ ] Code reviewed

**Status**: Not Started
**Sign-off By**: TBD

---

## Progress Tracking

### Overall Progress

| Component | Status | Progress |
|-----------|--------|----------|
| Schema System | Not Started | 0% |
| WorkflowIR Compiler | Not Started | 0% |
| Policy Compilation | Not Started | 0% |
| Local-First Storage | Not Started | 0% |
| Reproducibility & Privacy | Not Started | 0% |
| **Overall** | **Not Started** | **0%** |

---

### Phase Progress

| Phase | Status | Completion Date |
|-------|--------|----------------|
| Phase 1: Schema System | Not Started | TBD |
| Phase 2: WorkflowIR Compiler | Not Started | TBD |
| Phase 3: Policy Compilation | Not Started | TBD |
| Phase 4: Local-First Storage | Not Started | TBD |
| Phase 5: Reproducibility & Privacy | Not Started | TBD |

---

### Verification Progress

| Layer | Status | Pass Rate |
|-------|--------|-----------|
| Layer 1: Unit Tests | Not Started | N/A |
| Layer 2: Integration Tests | Not Started | N/A |
| Layer 3: Property-Based Tests | Not Started | N/A |
| Layer 4: End-to-End Tests | Not Started | N/A |

---

### Research Progress

| Research Area | Status | Completion Date |
|--------------|--------|----------------|
| Schema and Validation Patterns | Not Started | TBD |
| Typed IR and Compilation | Not Started | TBD |
| Local-First Storage | Not Started | TBD |
| ADR Governance | Not Started | TBD |

---

## Dependencies

### Blocks

None (foundation phase - no dependencies)

### Unblocks

- **plan-01** (MVP Queue & Scheduler)
- **plan-02** (CLI & Backends)
- **plan-03** (Glyphnova UI)
- **plan-04** (Quality Loops & Benchmarks)
- **plan-05** (Memory & Search)
- **plan-06** (Cron & Git Refinement)
- **plan-07** (Autonomy & Metrics)

### Integration Points

**Shared Components**:
- `WorkflowSpec` struct - used by all subsequent plans
- `WorkflowIR` - execution target for all plans
- `.glyphnova/` directory structure - used by all plans

**APIs and Interfaces**:
- `compile_workflow_spec()` - called by all plans
- `validate_workflow()` - called by all plans
- `persist_artifact()` - called by all plans

**Data Structures**:
- Run ID format - shared across plans
- Hash algorithm - shared across plans
- Policy types - shared across plans

---

## Quality Gates

### ADR-Specific Gates (from ADR-0001)

1. **G1: WorkflowSpec defines all required fields with types**
   - Description: All fields in schema have Rust types
   - Success Criteria: `WorkflowSpec` compiles with serde and derives
   - Status: Not Started

2. **G2: WorkflowIR compiles from valid WorkflowSpec deterministically**
   - Description: Same input always produces same IR
   - Success Criteria: Property tests verify determinism
   - Status: Not Started

3. **G3: IR validation passes without errors for valid workflows**
   - Description: Valid workflows pass validation
   - Success Criteria: Integration tests with valid workflows pass
   - Status: Not Started

4. **G4: IR serialization produces hash and provenance metadata**
   - Description: Hash computed and stored
   - Success Criteria: Hash verified in unit tests
   - Status: Not Started

5. **G5: .glyphnova/ directory structure is defined and stable**
   - Description: Directory layout documented and tested
   - Success Criteria: E2E tests create expected structure
   - Status: Not Started

6. **G6: Policy fields are deterministic and compile-time resolvable**
   - Description: Policy values are resolvable at compile time
   - Success Criteria: Property tests verify determinism
   - Status: Not Started

### Critical Review Upstream Factors (5 minimum)

1. **ADR Compliance**
   - Description: Does plan fully implement all ADR-0001 requirements?
   - Success Criteria: All 6 ADR gates pass
   - Status: Not Started

2. **Requirements Satisfaction**
   - Description: Does plan satisfy all related requirements (R01, R03, R15, R16, R19, R22, R28, R31)?
   - Success Criteria: All requirements traced to implementation phases
   - Status: Not Started

3. **Test Coverage**
   - Description: Are all requirements covered by tests across 4 verification layers?
   - Success Criteria: 100% of new code has tests
   - Status: Not Started

4. **Documentation**
   - Description: Is plan well-documented with clear execution instructions?
   - Success Criteria: All sections present and complete
   - Status: Not Started

5. **Integration**
   - Description: Does plan integrate cleanly with other plans?
   - Success Criteria: Shared components and APIs defined
   - Status: Not Started

6. **Performance** (Additional)
   - Description: Are performance requirements met?
   - Success Criteria: No performance regressions
   - Status: Not Started

7. **Security** (Additional)
   - Description: Are security and privacy requirements addressed?
   - Success Criteria: Privacy-by-default enforced
   - Status: Not Started

8. **OpenCode Compatibility** (Additional)
   - Description: Is plan executable in OpenCode with available tools?
   - Success Criteria: Plan renders correctly in OpenCode
   - Status: Not Started

---

## Next Steps

### Execution Workflow

1. **Code Review Phase**: Read and analyze ADR-0001, requirements, and research-plan-01 ✅ (COMPLETED)
2. **Web Research Phase**: Conduct research in 4 areas, update plan with findings
3. **Tests-First Implementation**: Write tests for all 5 phases before implementation
4. **Implementation Phase**: Implement features according to 5 phases
5. **Verification Phase**: Run all 4 verification layers at each checkpoint
6. **Update Plan Phase**: Update plan with verified running code
7. **Review Phase**: Critical review with 8 upstream factors

### Immediate Next Actions

1. Complete web research in 4 areas
2. Write unit tests for Phase 1 (Schema System)
3. Implement Phase 1
4. Run verification layers for Checkpoint 001
5. Continue with Phase 2 through Phase 5

---

## Execution Commands

**Script Usage**:

```bash
# Start plan execution
./implement.sh --start plan-00

# Continue implementation
./implement.sh --continue plan-00

# Run verification layers
./implement.sh --verify plan-00

# Mark plan as complete
./implement.sh --complete plan-00

# Check plan status
./implement.sh --status plan-00

# Run critical review
./implement.sh --review plan-00
```

**Manual Commands**:

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run property-based tests
cargo test --test '*property*'

# Run e2e tests
cargo test --test '*e2e*'
```

---

## References

### Related Documents

- [ADR-0001: Foundation Compiler Contract](../roadmap/adr-0001-foundation-compiler-contract.yml)
- [research-plan-01-foundation.yml](../roadmap/research-plan-01-foundation.yml)
- [requirements.mdc](../requirements/requirements.mdc)
- [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- [serde-saphyr](https://github.com/bourumir-wyngs/serde-saphyr) - YAML parsing
- [schemars](https://graham.cool/schemars/) - JSON Schema generation
- [serde](https://serde.rs) - Serialization framework
- [tokio](https://tokio.rs) - Async runtime
- [proptest](https://altsysrq.github.io/proptest-book/) - Property-based testing

### Related Plans

- [plan-01: MVP Queue & Scheduler](../phase-1-mvp-queue/plan-01-mvp-queue.md) - Depends on this plan
- [plan-02: CLI & Backends](../phase-2-cli-backends/plan-02-cli-backends.md) - Depends on this plan
- [plan-03: Glyphnova UI](../phase-3-glyphnova-ui/plan-03-glyphnova-ui.md) - Depends on this plan
- [plan-04: Quality Loops & Benchmarks](../phase-4-quality-loops/plan-04-quality-loops.md) - Depends on this plan
- [plan-05: Memory & Search](../phase-5-memory-search/plan-05-memory-search.md) - Depends on this plan
- [plan-06: Cron & Git Refinement](../phase-6-automation/plan-06-automation.md) - Depends on this plan
- [plan-07: Autonomy & Metrics](../phase-7-autonomy-metrics/plan-07-autonomy-metrics.md) - Depends on this plan

---

**Last Updated**: 2026-03-27
**Next Review**: TBD
