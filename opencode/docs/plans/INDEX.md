# AgentSDK Execution Engine — Master Plan Suite

**Version**: 2.0
**Created**: 2026-04-06
**Status**: Planning Phase
**Supersedes**: `initial-plans/` (v1.0, all plans 0% complete)

---

## Purpose

This Plan File Suite provides an **extremely extensive, well-organized implementation roadmap** for the AgentSDK Execution Engine — a native Rust runtime that executes YAML-defined agentic workflows against local LLM backends (LM Studio, Ollama, llama.cpp). Every requirement from the 1705-line unified schema, 53 example workflows, 9 ADRs, and extensive research is covered with incremental validation criteria to prevent goal drift.

---

## What Changed from v1.0

| Aspect | v1.0 (initial-plans/) | v2.0 (plans/) |
|--------|----------------------|----------------|
| Phase count | 8 | 10 (added Deep Research + Final Validation) |
| Validation | 4 layers, loose criteria | 7 layers, measurable criteria per task |
| Mock strategies | Missing | Required for every external dependency |
| Struct/trait definitions | Absent | Inline definitions in every plan |
| Schema→Phase mapping | Missing | Complete 19-domain→phase matrix |
| Requirements traceability | None | Full cross-phase matrix |
| Research integration | Not started | Deep research plan suite |
| Final validation | Missing | 5→120 model benchmarking, workflow generation |
| OpenCode tool specs | Missing | Tool usage specified per task |
| Anti-goal-drift | Missing | Incremental validation criteria suite |

---

## Plan Suite Structure

```
opencode/docs/plans/
├── INDEX.md                          # THIS FILE — Master navigation
├── ARCHITECTURE.md                   # System architecture and design decisions
│
├── deep-research/                    # DEEP RESEARCH PLAN SUITE
│   ├── 00-research-master-plan.md    # Research orchestration and integration
│   ├── 01-yaml-schema-research.md    # Schema validation and compilation research
│   ├── 02-rust-ecosystem-research.md # Crate evaluation and selection research
│   ├── 03-llm-backend-research.md    # Backend API protocol research
│   ├── 04-agent-patterns-research.md # Agent orchestration pattern research
│   ├── 05-testing-strategy-research.md # Testing and validation research
│   └── 06-performance-research.md    # Performance optimization research
│
├── 00-foundation/                    # PHASE 0: FOUNDATION
│   ├── plan.md                       # Master plan for Phase 0
│   ├── tasks/                        # Granular task breakdowns
│   │   ├── 00-cargo-project-setup.md
│   │   ├── 01-error-module.md
│   │   ├── 02-schema-types.md
│   │   ├── 03-yaml-parser.md
│   │   ├── 04-workflow-spec.md
│   │   ├── 05-variable-interpolation.md
│   │   ├── 06-workflow-ir.md
│   │   ├── 07-ir-compiler.md
│   │   ├── 08-dag-validator.md
│   │   ├── 09-policy-compiler.md
│   │   ├── 10-local-storage.md
│   │   ├── 11-workspace-management.md
│   │   └── 12-defaults-scope-inheritance.md
│   ├── validation/                   # Phase 0 validation criteria
│   │   ├── checkpoint-criteria.md
│   │   └── acceptance-criteria.md
│   └── tests/                        # Test specifications
│       ├── unit-tests.md
│       ├── integration-tests.md
│       └── property-tests.md
│
├── 01-mvp-queue/                     # PHASE 1: MVP QUEUE & SCHEDULER
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 02-cli-backends/                  # PHASE 2: CLI & LLM BACKENDS
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 03-glyphnova-ui/                  # PHASE 3: GLYPHNOVA UI
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 04-quality-loops/                 # PHASE 4: QUALITY LOOPS & BENCHMARKS
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 05-memory-search/                 # PHASE 5: MEMORY & SEARCH
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 06-automation/                    # PHASE 6: AUTOMATION
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 07-autonomy-metrics/              # PHASE 7: AUTONOMY & METRICS
│   ├── plan.md
│   ├── tasks/
│   ├── validation/
│   └── tests/
│
├── 08-final-validation/              # PHASE 8: FINAL VALIDATION
│   ├── plan.md                       # Comprehensive final validation plan
│   ├── 00-system-log-verification.md # System log verification
│   ├── 01-unit-test-verification.md  # Unit test deep verification
│   ├── 02-integration-test-verification.md
│   ├── 03-cli-live-testing.md        # Live CLI testing
│   ├── 04-workflow-execution-tests.md # Example workflow execution tests
│   ├── 05-benchmark-5-model.md       # 5-model benchmarking baseline
│   ├── 06-benchmark-20-model.md      # 20-model scaling test
│   ├── 07-benchmark-50-model.md      # 50-model scaling test
│   ├── 08-benchmark-120-model.md     # 120-model single workflow test
│   ├── 09-workflow-generation-test.md # Workflow-generating-workflow tests
│   ├── 10-cross-phase-regression.md  # Cross-phase regression verification
│   └── 11-final-signoff.md           # Final acceptance signoff
│
├── traceability/                     # CROSS-PHASE REQUIREMENTS TRACEABILITY
│   ├── schema-to-phase-matrix.md     # 19 schema domains → phase ownership
│   ├── adr-to-phase-matrix.md        # 47 ADR constraints → phase ownership
│   ├── workflow-to-phase-matrix.md   # 53 workflows → phase testing ownership
│   └── requirements-traceability.md  # Every requirement R01-R31 traced
│
└── validation-criteria/              # INCREMENTAL VALIDATION CRITERIA SUITE
    ├── framework.md                  # Validation framework definition
    ├── phase-00-criteria.md          # Phase 0 measurable criteria
    ├── phase-01-criteria.md          # Phase 1 measurable criteria
    ├── phase-02-criteria.md          # Phase 2 measurable criteria
    ├── phase-03-criteria.md          # Phase 3 measurable criteria
    ├── phase-04-criteria.md          # Phase 4 measurable criteria
    ├── phase-05-criteria.md          # Phase 5 measurable criteria
    ├── phase-06-criteria.md          # Phase 6 measurable criteria
    ├── phase-07-criteria.md          # Phase 7 measurable criteria
    ├── phase-08-criteria.md          # Phase 8 measurable criteria
    ├── anti-goal-drift-checklist.md  # Goal drift detection checklist
    └── cumulative-progress.md        # Cumulative progress tracker
```

---

## Phase Dependency Graph

```
                    ┌─────────────────────┐
                    │   DEEP RESEARCH      │
                    │   (runs throughout)  │
                    └──────────┬──────────┘
                               │ feeds documentation into all phases
            ┌──────────────────┼──────────────────┐
            ▼                  ▼                  ▼
    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │   PHASE 00   │  │   PHASE 00   │  │   PHASE 00   │
    │  Foundation  │  │  Foundation  │  │  Foundation  │
    └──────┬───────┘  └──────────────┘  └──────────────┘
           │
           ▼
    ┌──────────────┐
    │   PHASE 01   │
    │  MVP Queue   │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │   PHASE 02   │──── Can parallel with ────┐
    │ CLI/Backends │                            │
    └──────┬───────┘                            ▼
           │                              ┌──────────────┐
           ▼                              │   PHASE 03   │
    ┌──────────────┐                       │ Glyphnova UI │
    │   PHASE 04   │◄──────────────────────└──────┬───────┘
    │ Quality Loops│                              │
    └──────┬───────┘                              │
           │                                      │
           ▼                                      │
    ┌──────────────┐                              │
    │   PHASE 05   │                              │
    │Memory/Search │                              │
    └──────┬───────┘                              │
           │                                      │
           ▼                                      │
    ┌──────────────┐                              │
    │   PHASE 06   │◄─────────────────────────────┘
    │  Automation  │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │   PHASE 07   │
    │Autonomy/Metr │
    └──────┬───────┘
           │
           ▼
    ┌──────────────────────────────────────────┐
    │           PHASE 08: FINAL VALIDATION     │
    │  5→120 model benchmarking                │
    │  Workflow generation testing             │
    │  System log verification                 │
    │  Cross-phase regression                  │
    │  Final signoff                           │
    └──────────────────────────────────────────┘
```

### Critical Path (Sequential)
```
PHASE 00 → PHASE 01 → PHASE 02 → PHASE 04 → PHASE 05 → PHASE 06 → PHASE 07 → PHASE 08
```

### Parallelizable After Dependencies Met
- **PHASE 03** can start in parallel with PHASE 04 (after PHASE 02)
- **Deep Research** runs continuously throughout all phases

---

## Phase Summary Table

| Phase | Name | Duration | ADR | Schema Domains | Key Deliverables |
|-------|------|----------|-----|---------------|-----------------|
| Research | Deep Research | Continuous | All | All | Research docs integrated into every phase |
| 00 | Foundation | 6-8 weeks | ADR-0001 | 1,2,13-18 | WorkflowSpec, WorkflowIR, parser, storage |
| 01 | MVP Queue | 8-10 weeks | ADR-0002 | 3,4,5(partial),7,8,9,10(partial) | Scheduler, queue, human gating, CLI proto |
| 02 | CLI & Backends | 8-10 weeks | ADR-0003 | 6,10,11,12,19 | LLM backends, tool permissions, codegen |
| 03 | Glyphnova UI | 10-12 weeks | ADR-0004 | (UI layer) | Desktop shell, visualization, shared API |
| 04 | Quality Loops | 10-12 weeks | ADR-0005 | 5,8(perf) | Verify-repair, benchmarks, reports |
| 05 | Memory & Search | 10-12 weeks | ADR-0006 | 12(adv) | Local memory, hybrid search, scraping |
| 06 | Automation | 8-10 weeks | ADR-0007 | (automation) | Cron, git experiments, merge proposals |
| 07 | Autonomy | 10-12 weeks | ADR-0008 | (autonomy) | Bounded loops, metrics, dashboards |
| 08 | Final Validation | 4-6 weeks | All | All | Benchmarking (5→120), workflow gen, signoff |

**Estimated Total**: 74-100 weeks (18-24 months)

---

## Verification Layers (7 layers, up from 4)

| Layer | Purpose | Tools | Pass Criteria |
|-------|---------|-------|---------------|
| L1: Unit Tests | Isolated component testing | `cargo test --lib`, rstest | 100% pass, 90%+ coverage |
| L2: Integration Tests | Multi-component interaction | `cargo test --test '*'` | 100% pass, API contracts verified |
| L3: Property Tests | Edge cases and invariants | proptest (1000 iterations) | 100% pass, no shrinking failures |
| L4: E2E Tests | Full workflow execution | `cargo test --test '*e2e*'` | 100% pass, all 53 example workflows |
| L5: System Log Verification | Log output correctness | Custom log analyzers | Logs match expected format/content |
| L6: Live CLI Testing | CLI interaction verification | assert_cmd + real backends | All CLI commands produce correct output |
| L7: Benchmark Tests | Performance and scaling | Custom benchmark harness | Meet performance targets |

---

## Anti-Goal-Drift Framework

Every phase plan includes:

1. **Phase Entry Criteria**: Measurable conditions that must be true before starting
2. **Task-Level Acceptance Criteria**: Every task has specific, testable success criteria
3. **Checkpoint Gate Criteria**: Must pass before proceeding to next task group
4. **Phase Exit Criteria**: Must pass all validation layers before phase is "complete"
5. **Cross-Phase Regression Tests**: Tests from prior phases must continue passing
6. **Schema Coverage Audit**: Every schema field owned by the phase must be implemented
7. **ADR Constraint Compliance**: Every ADR constraint must be satisfied

---

## Rust Crate Stack (v2.0)

```toml
[dependencies]
# Core
tokio = { version = "1.51", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# YAML Parsing
yaml_serde = "0.10"  # Modern replacement for deprecated serde_yaml

# HTTP / LLM APIs
reqwest = { version = "0.13", features = ["json", "rustls-tls", "http2", "stream"] }

# CLI
clap = { version = "4.6", features = ["derive", "env"] }

# State / Checkpointing
sled = "0.34"  # Embedded KV store for state persistence

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"] }

# Error Handling
thiserror = "2.0"  # Library errors (typed)
anyhow = "1.0"     # Application errors (untyped)

# Testing
proptest = "1.11"
rstest = "0.24"
assert_cmd = "2.0"

# Utilities
async-trait = "0.1"
futures = "0.3"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
regex = "1.0"
sha2 = "0.10"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
wiremock = "0.6"  # Mock HTTP for backend testing
```

---

## LLM Backend Protocol Summary

| Backend | Base URL | Streaming | Tool Calling | Key Difference |
|---------|----------|-----------|-------------|----------------|
| LM Studio | `localhost:1234/v1` | SSE | OpenAI-compatible | OpenAI format, `lm-studio` API key |
| Ollama | `localhost:11434/api` | NDJSON | Native `tools` array | NDJSON (not SSE), `think` param |
| llama.cpp | `localhost:8080` | SSE | OpenAI-compatible | Pre-loaded model required, `/health` |
| OpenAI | `api.openai.com/v1` | SSE | OpenAI standard | Rate limiting (429), cloud-only |

---

## How to Use This Plan Suite

### For Implementation
1. Start with `deep-research/00-research-master-plan.md` — understand research flow
2. Execute `00-foundation/plan.md` — foundation first, everything depends on it
3. Follow dependency graph for subsequent phases
4. Use `validation-criteria/` files to verify no goal drift
5. Use `traceability/` matrices to verify complete requirement coverage

### For Review
1. Check `validation-criteria/cumulative-progress.md` for current status
2. Run `anti-goal-drift-checklist.md` at any phase boundary
3. Verify `traceability/requirements-traceability.md` for missing coverage

### For Resuming After Context Loss
1. Read `INDEX.md` (this file)
2. Read the current phase's `plan.md`
3. Check `validation-criteria/cumulative-progress.md`
4. Resume from last completed task

---

## References

- **Schema**: `opencode/docs/reports/requirements/unifying-schema/unified-workflow-schema.yml` (801 lines)
- **ADRs**: `opencode/docs/reports/roadmap/adr-0000` through `adr-0008`
- **Example Workflows**: `opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/` (53 files)
- **v1.0 Plans**: `opencode/docs/reports/initial-plans/` (8 plans, all 0% complete)
- **Research**: `opencode/docs/research/next-steps/` (8 research files)
- **Barebones Assessment**: `opencode/docs/reports/BAREBONES_APPROACH_ASSESSMENT.md`
