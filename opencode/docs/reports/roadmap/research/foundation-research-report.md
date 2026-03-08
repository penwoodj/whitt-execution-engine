# Foundation Research Report

**Plan ID**: research-plan-01-foundation
**Status**: Complete
**Date**: 2026-03-07
**Supports**: ADR-0001, ADR-0000

---

## Executive Summary

This research identifies canonical patterns for a local-first, compiler-centered orchestration framework in Rust, with emphasis on YAML DSL, internal representation, storage architecture, and ADR governance. All recommendations are grounded in 3+ distinct sources and include concrete tradeoffs.

---

## 1. YAML DSL Schema and Validation Patterns

### Key Finding: Typed Rust Structs + Schema Generation

**Recommendation**: Adopt a **typed-first approach** where Rust structs serve as the source of truth, with schema generation as a validation layer rather than ad-hoc YAML parsing.

**Evidence Sources**:
1. **yaml-serde** (Official fork) - Maintained Rust-YAML serialization library with strong type guarantees
2. **schemars** - Schema generation framework for Rust types with validation attributes
3. **JSON Schema.org** - Industry-standard schema format with broad tooling support

**Pattern**:
```rust
// Rust source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
#[schemars(deny_unknown_fields)]
struct WorkflowSpec {
    pub version: String,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub policy: PolicyConfig,
}

// Schema generation
let schema = schemars::schema_for!(WorkflowSpec);
```

**Tradeoffs**:
| Approach | Pros | Cons |
|----------|------|-------|
| Typed structs + schemars | Compile-time safety, IDE autocomplete, easy refactoring | Requires explicit schema generation step, less flexible than pure YAML constraints |
| Hand-rolled validation | Maximum flexibility for custom rules | No compile-time guarantees, maintenance burden |

**ADR Alignment**: ADR-0001 explicitly states "prefer typed Rust structs plus schema generation over ad hoc YAML maps"

---

## 2. Internal Representation (IR) Design

### Key Finding: Multi-Stage IR with Strong Typing

**Recommendation**: Implement a **multi-stage IR** following Rust compiler patterns (HIR → MIR) with explicit DAG/state-machine semantics.

**Evidence Sources**:
1. **Rust Compiler Development Guide** - Documents HIR (High-Level IR), THIR (Typed HIR), and MIR (Mid-Level IR)
2. **Sway Compiler** - SSA-based IR for smart contracts, demonstrating ECS architecture
3. **MLIR** - SSA form dialect for progressive optimization

**Pattern**:
```rust
// Multi-stage IR design
pub enum WorkflowIRStage {
    HIR,    // High-Level IR after YAML parsing
    THIR,   // Typed IR with type inference
    MIR,     // Mid-Level IR for scheduling
}

// Typed internal representation
#[derive(Debug, Clone)]
pub struct WorkflowIR {
    pub hir: WorkflowHIR,
    pub thir: WorkflowTHIR,
    pub mir: WorkflowMIR,
    pub dag: DirectedAcyclicGraph<IRNode>,
}
```

**Tradeoffs**:
| IR Design | Benefits | Costs |
|-----------|---------|-------|
| HIR → THIR → MIR | Clear separation of concerns, type safety, optimization opportunities | Multiple IR representations to maintain, transformation complexity |
| Single-pass IR | Simpler design, less transformation overhead | Harder to reason about and optimize |

**ADR Alignment**: ADR-0001 defines "canonical execution unit is a typed WorkflowSpec and compiled WorkflowIR" with "explicit DAG/state-machine semantics"

---

## 3. Local Artifact Layout

### Key Finding: Content-Addressed Storage with Replayability

**Recommendation**: Implement a **local-first artifact layout** rooted in `.glyphnova/` with content-addressed storage, per-chat workspaces, and explicit provenance tracking.

**Evidence Sources**:
1. **AWS DevOps Guidance** - Emphasizes frequent small commits for replayable checkpoints
2. **Julia Pkg.jl Artifacts** - Demonstrates content-addressable storage with dual verification (sha256 + git-tree-sha1)
3. **lakeFS** - Git-like versioning separating immutable data from mutable metadata

**Recommended Layout**:
```
.glyphnova/
├── per-chat-workspaces/
│   ├── {chat-id}/
│   │   ├── workflow_spec.yaml
│   │   ├── policy_snapshot.yaml
│   │   ├── artifacts/
│   │   ├── logs/
│   │   └── provenance/
├── cache/                    # Content-addressed blobs (SHA256 directories)
├── events/                    # Append-only event log
├── metadata/                  # Artifact bindings
│   ├── artifacts.toml         # Name → hash mappings
│   └── overrides.toml         # Version overrides
├── checkpoints/               # Snapshots/replay points
└── plans/                     # Research outputs and ADR links
```

**Key Principles**:
1. **Immutability**: Content addressed by cryptographic hash → never changes
2. **Append-only**: Events only added, never modified (enables perfect replay)
3. **Separation**: Metadata separate from data (enables efficient queries)
4. **Replayability**: Any state reproducible from event stream or checkpoint
5. **Auditability**: Hash chains and commit logs prove integrity

**ADR Alignment**: ADR-0001 states "Every chat is a scoped executable work container with a local system-of-record rooted in a per-chat workspace and `.glyphnova/`"

---

## 4. ADR Governance

### Key Finding: Lightweight ADR Process for Rapid Evolution

**Recommendation**: Adopt a **lightweight ADR governance model** adapted from MADR (Markdown ADR) practices, with explicit lifecycle management, amendment protocols, and research traceability.

**Evidence Sources**:
1. **adr.github.io** - Official ADR standards and templates
2. **GOV.UK ADR Framework** - Multi-level governance with escalation criteria
3. **Real-world examples (k3s, Tor)** - Status evolution and amendment patterns

**Recommended Governance Model**:

**Status Lifecycle**:
```
Proposed → Accepted → Superseded by ADR-XXX → Archived
```

**Amendment Protocol**:
1. Draft ADR (single author, async review)
2. 45-min sync review for contentious decisions
3. Update status to "Accepted" with decision rationale
4. For superseded decisions: Create new ADR, update old status to "Superseded by ADR-XXX"

**Required Metadata** (per ADR):
- ADR ID and Title
- Date created and last modified
- Status (Proposed/Accepted/Superseded/Archived)
- Author(s) and Owner
- Decision makers (Responsible, Accountable, Consulted, Informed)
- Related ADRs (bidirectional links)
- Evidence/Artifacts references (research plans, validation results, benchmark outputs)
- Risk assessment and trade-offs
- Open questions and acceptance criteria

**Traceability Requirements**:
- Every ADR with research plan linkage must include explicit reference to research plan file and relevant evidence gates
- Cross-reference map at ADR-0000 listing ADR IDs → Research Plan IDs → Evidence Gates → Validation Artifacts
- Validation criteria and benchmark outputs must reference the ADR IDs they satisfy

**ADR Alignment**: Current ADR suite lacks formal amendment workflow, lifecycle management, and traceability artifacts. Recommendations would significantly improve governance for a rapidly evolving compiler-runtime product.

---

## 5. Synthesis and Recommendations

### Recommended Stack for v0.1.0 Foundation

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **YAML DSL** | serde_yaml + schemars + JSON Schema | Industry standards, strong typing, broad tooling |
| **IR Design** | HIR → THIR → MIR pattern | Rust compiler best practices, optimization opportunities |
| **Schema Validation** | schemars with validation attributes | Compile-time safety, declarative constraints |
| **Storage** | Content-addressed + SQLite | Replayability, auditability, zero external dependencies |
| **Governance** | MADR lightweight process + evidence linkage | Rapid iteration, traceable decisions |

### Quality Gates

- [ ] Schema generator produces valid JSON Schema v2020-12
- [ ] IR transformations preserve type information and graph structure
- [ ] Artifact layout supports replay from any checkpoint
- [ ] ADR template includes all required metadata fields
- [ ] Research plan references are explicit and bidirectional

### Open Questions for ADR-0001

1. Should schema generation support custom validation rules beyond what schemars provides?
2. What specific DAG representation format should be used (DOT, custom binary, Rust enum)?
3. How should IR persistence relate to artifact layout checkpointing?

---

## References

1. **yaml-serde** - https://github.com/yaml/yaml-serde
2. **schemars** - https://graham.cool/schemars/
3. **Rust Compiler Guide** - https://doc.rust-lang.org/nightly/nightly-rustc/dev-guide/
4. **ADR GitHub Organization** - https://adr.github.io/
5. **AWS DevOps Guidance** - https://docs.aws.amazon.com/wellarchitected/latest/devops-guidance/
6. **Julia Pkg.jl Artifacts** - https://julialang.github.io/Pkg.jl/dev/artifacts/
7. **lakeFS** - https://docs.lakefs.io/v1.76/understand/data-structure/
8. **GOV.UK Framework** - https://www.gov.uk/government/publications/architectural-decision-record-framework/

---

**End of Report**
