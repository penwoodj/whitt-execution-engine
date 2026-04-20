# Plan-04: Quality Loops, Artifact Workflows, and Benchmark-Driven File-Type Expansion

**Plan ID**: plan-04
**Phase**: Phase 4 - Quality Loops
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0005 (Quality loops, artifact workflows, and benchmark-driven file-type expansion)
**Related Research Plan**: research-plan-03-quality-memory-search.yml
**Estimated Time**: 8-10 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue), plan-02 (CLI & Backends), plan-03 (Glyphnova UI)

---

## Overview

This plan implements quality loops, benchmark-driven file-type expansion, and artifact workflows for improving output quality across file types through local agentic flows, testing, and benchmarking.

**Key Features**:
- **Generate-Verify-Repair Loops**: Runtime semantics (not optional prompt style) for systematic quality improvement
- **Benchmark Suites**: Stored artifacts tied to workflow version, policy snapshot, backend, and file type
- **Workflow Specs as Artifacts**: Editable artifacts transformable by later workflows under review controls
- **Observability**: Produces summaries, reports, graphs, and benchmark outputs
- **File-Type Capability Matrix**: Track which file types are supported at what quality levels
- **Verifier Interfaces**: Pluggable verification for different artifact types
- **Repair Loops**: Automated repair based on verification failures
- **Report Generation**: Structured, queryable reports with actionable insights

**Key Deliverables**:
- Verifier interface framework
- Generate-verify-repair loop runtime
- Benchmark harness and storage
- File-type capability matrix
- Artifactized workflow library
- Report generation system
- Quality metrics dashboard integration

---

## Code Review

### ADR-0005 Summary

**Decision**: Make review, repair, benchmark, and artifact-driven iteration first-class capabilities.

**Key Requirements**:
1. **Generate-Verify-Repair Loops**: Runtime semantics (not optional prompt style)
2. **Benchmark Suites**: Stored artifacts tied to workflow version, policy snapshot, backend, and file type
3. **Workflow Specs as Artifacts**: Editable artifacts transformable by later workflows under review controls
4. **Observability**: Produces summaries, reports, graphs, and benchmark outputs

**Scope**:
- **Included**: Verifier interfaces, repair loops, benchmark harness design, file-type capability matrix, artifactized workflow libraries, report generation
- **Excluded**: Memory retrieval internals, web scraping details, fully autonomous scheduling

**Positive Consequences**:
- Systematic quality improvement
- Measurable progress per file type
- Strong inputs for lunch-and-learn checkpoints

**Negative Consequences**:
- Benchmark curation can be expensive
- More artifacts to manage and prune

### Requirements Review

From `requirements.md` and `schema-consolidated-report.md`:

**R04**: Depth-over-speed and capability-normalized outcomes
**R21**: Artifact semantics - workflows are inspectable, editable artifacts
**R25**: Generate to verify to repair review loop semantics
**R30**: Observability - logging levels, summaries, graphs, reports, and provenance

**Validation Focus for v0.1.0**:
- Generate-verify-repair loops execute as runtime semantics
- Benchmark suites store with complete metadata
- Observability outputs are structured and queryable
- File-type benchmarks track quality metrics per language
- Reports include actionable insights for improvement

### Research Plan Review

**research-plan-03-quality-memory-search.yml** is **COMPLETED** with 3 research domains (plan-04 uses domains 1 and 2):

1. **Quality Loops**: What verifier interfaces and repair loop semantics best support generate-verify-repair for different artifact types?
2. **Benchmark Strategy**: What benchmark design and curation strategy best measures quality across file types while remaining maintainable?

**Key Findings**:
- Report outputs: `quality-memory-search-research-report.md`, `benchmark-strategies.csv`, `quality-loop-patterns.csv`
- Quality gates met: Recommendations cover all artifact types with verifiable metrics

---

## Web Research

### Research Area 1: Verification Interface Patterns

**Research Question**: What verification interface patterns exist for code, documentation, and other artifact types?

**Recommended Sources**:
- [docs.rs - criterion](https://docs.rs/criterion) - Rust benchmarking
- [GitHub - code verification](https://github.com) - Search for "code verifier" projects
- [linter projects](https://github.com/golang/lint) - Go lint patterns
- [eslint](https://eslint.org) - JavaScript verification
- [mypy](https://mypy-lang.org) - Python type checking
- [shellcheck](https://www.shellcheck.net) - Shell script verification

**Expected Findings**:
- Verifier interface abstractions
- Common verification criteria (syntax, style, correctness, security)
- Error reporting patterns
- Test generation from verification failures
- Pluggable verifier architectures

**Status**: Not Started

---

### Research Area 2: Benchmark Suite Design

**Research Question**: What benchmark suite design patterns support file-type quality tracking?

**Recommended Sources**:
- [criterion.rs](https://docs.rs/criterion) - Statistical benchmarking
- [BenchmarkDotNet](https://benchmarkdotnet.org) - .NET benchmarking patterns
- [Google Benchmark](https://github.com/google/benchmark) - C++ benchmarking
- [pytest-benchmark](https://pytest-benchmark.readthedocs.io) - Python benchmarks
- [LLM benchmarks](https://huggingface.co/spaces/lmsys/chatbot-arena-leaderboard) - LLM quality benchmarks

**Expected Findings**:
- Benchmark storage and versioning
- Metadata tracking (workflow, policy, backend, file type)
- Quality metrics definitions
- Statistical analysis of benchmark results
- Benchmark suite curation patterns

**Status**: Not Started

---

### Research Area 3: Repair Loop Strategies

**Research Question**: What repair loop strategies best automate artifact improvement based on verification failures?

**Recommended Sources**:
- [GitHub - self-healing](https://github.com) - Search for "self-healing" and "auto-repair"
- [AutoFix literature](https://arxiv.org) - Search for "automated program repair"
- [ChatGPT repair examples](https://github.com) - OpenAI tool use patterns
- [LLM code repair](https://paperswithcode.com) - Automated program repair papers
- [Error-driven repair](https://dl.acm.org) - Error-driven development patterns

**Expected Findings**:
- Repair loop architectures
- Repair strategy patterns (direct fix, regeneration, refinement)
- Verification feedback integration
- Repair success metrics
- Convergence criteria for repair loops

**Status**: Not Started

---

### Research Area 4: File-Type Quality Metrics

**Research Question**: What quality metrics are relevant for different file types (code, docs, configs, etc.)?

**Recommended Sources**:
- [CodeQL](https://codeql.github.com) - Code quality queries
- [SonarQube](https://www.sonarqube.org) - Code quality metrics
- [GitHub Actions](https://github.com/features/actions) - CI quality checks
- [Readability metrics](https://dl.acm.org) - Code readability research
- [Documentation quality](https://www.writethedocs.org) - Documentation best practices

**Expected Findings**:
- Code quality metrics (complexity, duplication, security)
- Documentation quality metrics (completeness, clarity, structure)
- Configuration quality metrics (validity, best practices)
- Test quality metrics (coverage, mutation testing)
- Quality scoring formulas

**Status**: Not Started

---

### Research Area 5: Report Generation and Visualization

**Research Question**: What patterns exist for generating structured, queryable reports with actionable insights?

**Recommended Sources**:
- [Jupyter Notebooks](https://jupyter.org) - Report generation
- [Observable](https://observablehq.com) - Interactive dashboards
- [Grafana](https://grafana.com) - Metrics visualization
- [D3.js](https://d3js.org) - Data visualization
- [Pandoc](https://pandoc.org) - Report format conversion

**Expected Findings**:
- Report structure patterns
- Visualization best practices
- Query interfaces for report data
- Actionable insight generation
- Report format options (HTML, PDF, JSON, Markdown)

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Verifier Interface Framework

**Description**: Define and implement pluggable verifier interface

**Tasks**:
1. Define `Verifier` trait with core methods
2. Define `VerificationResult` struct (success, failures, metrics)
3. Define `VerifierCapabilities` struct (file types, checks)
4. Implement built-in verifiers:
   - Code verifier (syntax, style, security)
   - Documentation verifier (completeness, clarity)
   - Configuration verifier (validity, best practices)
5. Implement verifier registry
6. Add verifier configuration
7. Implement verifier discovery and loading

**Related Requirements**: ADR-0005 (verifier interfaces)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 2: Generate-Verify-Repair Loop Runtime

**Description**: Implement runtime semantics for generate-verify-repair loops

**Tasks**:
1. Define `RepairLoop` configuration
2. Implement generate step (call workflow)
3. Implement verify step (call verifiers)
4. Implement repair step (call LLM with verification failures)
5. Implement convergence criteria (max iterations, quality threshold)
6. Add repair loop state tracking
7. Implement repair strategy selection (fix, regenerate, refine)
8. Add repair loop logging and observability

**Related Requirements**: R25 (generate-verify-repair loop semantics)
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 3: Benchmark Harness and Storage

**Description**: Implement benchmark suite storage and execution

**Tasks**:
1. Define `Benchmark` struct (workflow, policy, backend, file type, metrics)
2. Define `BenchmarkSuite` struct (collection of benchmarks)
3. Implement benchmark storage layer (metadata + results)
4. Implement benchmark execution harness
5. Add benchmark versioning (workflow version, policy snapshot)
6. Implement statistical analysis of benchmark results
7. Add benchmark comparison (before/after, across file types)
8. Implement benchmark curation (add, remove, archive)

**Related Requirements**: ADR-0005 (benchmark harness design)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 4: File-Type Capability Matrix

**Description**: Track which file types are supported at what quality levels

**Tasks**:
1. Define `FileCapability` struct (file type, quality level, verifier status)
2. Define `CapabilityMatrix` struct (collection of capabilities)
3. Implement capability registration (add, update, remove)
4. Implement capability querying (by file type, by quality level)
5. Add capability visualization (matrix table, quality radar charts)
6. Implement capability versioning
7. Add capability gaps analysis (unsupported file types)
8. Implement capability recommendations (next file type to support)

**Related Requirements**: R04 (capability-normalized outcomes)
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 5: Artifactized Workflow Library

**Description**: Treat workflows as editable artifacts transformable by other workflows

**Tasks**:
1. Define `ArtifactWorkflow` struct (workflow spec, metadata, history)
2. Implement workflow artifact storage
3. Add workflow versioning (with transformation history)
4. Implement workflow transformation API
5. Add transformation review controls (approve/reject transformations)
6. Implement workflow search and discovery
7. Add workflow library UI integration
8. Implement workflow export/import

**Related Requirements**: R21 (workflows are inspectable, editable artifacts)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Report Generation System

**Description**: Generate structured, queryable reports with actionable insights

**Tasks**:
1. Define `Report` struct (type, data, metadata, generated_at)
2. Define report types (quality, benchmark, improvement, summary)
3. Implement report data collection (from logs, benchmarks, metrics)
4. Implement report generation (HTML, PDF, JSON, Markdown)
5. Add report query interface (filter by type, date, workflow)
6. Implement report visualization (charts, graphs, tables)
7. Add actionable insight generation (recommendations from report data)
8. Implement report storage and archiving

**Related Requirements**: R30 (observability outputs)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 7: Quality Metrics Dashboard Integration

**Description**: Integrate quality metrics into UI dashboard

**Tasks**:
1. Design quality metrics dashboard layout
2. Implement quality trend visualization (over time)
3. Add file-type quality comparison charts
4. Implement benchmark result visualization
5. Add report browsing and filtering
6. Implement quality improvement tracking (generate-verify-repair loops)
7. Add capability matrix display
8. Implement quality alerts (quality degradation, missing capabilities)

**Related Requirements**: R30 (observability)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- Verifier trait method implementations
- Verification result structures
- Repair loop state machine
- Benchmark storage and retrieval
- Capability matrix operations
- Workflow artifact CRUD
- Report data collection and generation

**Test Framework**: `cargo test --lib`

**Success Criteria**:
- 90%+ code coverage on core modules
- All verifier interfaces tested with sample inputs
- All edge cases tested (empty results, errors, etc.)

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- Generate-verify-repair loop end-to-end
- Benchmark execution with metadata storage
- Capability matrix updates and queries
- Workflow transformation with review controls
- Report generation from real execution data
- Dashboard integration with all data sources

**Test Framework**: `cargo test --test '*'`

**Success Criteria**:
- All verifiers work with workflow output
- Repair loops converge correctly
- Benchmarks store with complete metadata
- Workflows can be transformed and reviewed
- Reports generate correctly from real data
- Dashboard displays accurate metrics

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- Verification result validity (always consistent)
- Repair loop convergence (quality never degrades, bounded iterations)
- Benchmark metadata invariants (never lose workflow/policy reference)
- Capability matrix consistency (no conflicts, all valid)
- Workflow transformation history preservation
- Report query correctness (always returns matching results)

**Test Framework**: `proptest` with 1000 iterations each

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (quality monotonic, metadata complete)
- Properties verified with 1000+ iterations

---

### Layer 4: End-to-End Tests

**Scope**: Full quality loop workflows

**Coverage Areas**:
- Complete generate-verify-repair loop on code file
- Complete generate-verify-repair loop on documentation
- Benchmark suite execution and storage
- Workflow transformation and review
- Report generation and dashboard display
- File-type capability tracking updates
- Quality improvement tracking over multiple iterations

**Test Framework**: `cargo test --test '*e2e*'`

**Success Criteria**:
- Complete quality loops execute successfully
- All file types improve quality over iterations
- Benchmarks store with correct metadata
- Reports display actionable insights
- Dashboard shows accurate quality metrics
- Capability matrix reflects true support status

---

## Verification Checkpoints

### Checkpoint 1: Verifier Interface Working

**Target Date**: Week 2
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] `Verifier` trait is well-defined
- [ ] At least 3 built-in verifiers work (code, docs, config)
- [ ] Verifier registry loads and finds verifiers
- [ ] Verification results are structured correctly
- [ ] Property tests pass on verifier results
- [ ] Integration tests pass with real artifacts

**Status**: Not Started

---

### Checkpoint 2: Generate-Verify-Repair Loop Complete

**Target Date**: Week 4
**Verification Layers**: 1, 2, 3, 4
**Sign-Off Criteria**:
- [ ] Repair loop runtime executes correctly
- [ ] Generate, verify, repair steps work together
- [ ] Convergence criteria are enforced
- [ ] Repair state is tracked accurately
- [ ] Loop terminates correctly (convergence or max iterations)
- [ ] Property tests verify quality monotonicity
- [ ] E2E test: full repair loop on code file

**Status**: Not Started

---

### Checkpoint 3: Benchmark Harness Functional

**Target Date**: Week 5
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Benchmark storage works correctly
- [ ] Benchmark execution harness runs benchmarks
- [ ] Metadata is stored (workflow, policy, backend, file type)
- [ ] Statistical analysis produces meaningful results
- [ ] Benchmark comparison works (before/after, across types)
- [ ] Property tests pass on metadata invariants
- [ ] E2E test: create and run benchmark suite

**Status**: Not Started

---

### Checkpoint 4: File-Type Capability Matrix Tracking

**Target Date**: Week 6
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] Capability matrix stores correctly
- [ ] Capabilities can be registered, updated, removed
- [ ] Queries return correct results
- [ ] Visualization displays matrix correctly
- [ ] Capability gaps are identified
- [ ] Recommendations are generated
- [ ] Integration tests pass with multiple file types

**Status**: Not Started

---

### Checkpoint 5: Artifactized Workflow Library Working

**Target Date**: Week 7
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Workflows can be stored as artifacts
- [ ] Workflow versioning tracks history
- [ ] Transformations can be applied under review controls
- [ ] Workflow search and discovery work
- [ ] Export/import preserves workflow integrity
- [ ] E2E test: create, transform, and approve workflow

**Status**: Not Started

---

### Checkpoint 6: Report Generation Functional

**Target Date**: Week 8
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Report data collection works from all sources
- [ ] Reports generate in all formats (HTML, PDF, JSON, Markdown)
- [ ] Report query interface filters correctly
- [ ] Report visualizations render correctly
- [ ] Actionable insights are generated
- [ ] E2E test: generate quality report and display in dashboard

**Status**: Not Started

---

### Checkpoint 7: Quality Metrics Dashboard Complete

**Target Date**: Week 10
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Dashboard displays quality trends correctly
- [ ] File-type quality comparison charts work
- [ ] Benchmark results are visualized
- [ ] Reports can be browsed and filtered
- [ ] Quality improvement tracking works (repair loop iterations)
- [ ] Capability matrix display matches actual state
- [ ] Quality alerts trigger correctly
- [ ] E2E test: complete quality workflow with dashboard review

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 7 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 7 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | Verifier Interface Framework | Not Started | 0% |
| 2 | Generate-Verify-Repair Loop Runtime | Not Started | 0% |
| 3 | Benchmark Harness and Storage | Not Started | 0% |
| 4 | File-Type Capability Matrix | Not Started | 0% |
| 5 | Artifactized Workflow Library | Not Started | 0% |
| 6 | Report Generation System | Not Started | 0% |
| 7 | Quality Metrics Dashboard Integration | Not Started | 0% |

---

### Verification Layer Progress

| Layer | Description | Tests Written | Tests Passing | Coverage |
|-------|-------------|----------------|---------------|----------|
| 1 | Unit Tests | 0 | 0 | 0% |
| 2 | Integration Tests | 0 | 0 | 0% |
| 3 | Property-Based Tests | 0 | 0 | 0% |
| 4 | End-to-End Tests | 0 | 0 | 0% |

---

### Research Progress

| Research Area | Status | Evidence Collected | Synthesized |
|--------------|--------|-------------------|-------------|
| Verification Interface Patterns | Not Started | 0 | No |
| Benchmark Suite Design | Not Started | 0 | No |
| Repair Loop Strategies | Not Started | 0 | No |
| File-Type Quality Metrics | Not Started | 0 | No |
| Report Generation and Visualization | Not Started | 0 | No |

---

## Dependencies

### Blocks

- plan-00 (Foundation): Needed for WorkflowIR and storage
- plan-01 (MVP Queue): Needed for workflow execution
- plan-02 (CLI & Backends): Needed for execution infrastructure
- plan-03 (Glyphnova UI): Needed for dashboard integration

### Unblocks

- plan-05: Memory & Search (depends on quality metrics and benchmarks)
- plan-06: Automation (depends on quality loops for automation)
- plan-07: Autonomy & Metrics (depends on all quality infrastructure)

### Integration Points

- **plan-00 (Foundation)**: Uses WorkflowIR, storage layer, observability
- **plan-01 (MVP Queue)**: Runs workflows for quality loops
- **plan-02 (CLI & Backends)**: Uses CLI for quality loop triggers
- **plan-03 (Glyphnova UI)**: Displays quality metrics in dashboard

---

## Quality Gates

### ADR-0005 Quality Gates

1. **Generate-Verify-Repair as Runtime Semantics**: Loops execute deterministically with enforced convergence criteria
2. **Benchmark Suite Metadata**: All benchmarks store workflow version, policy snapshot, backend, and file type
3. **Observability Outputs Structured**: Reports are queryable and provide actionable insights
4. **File-Type Quality Tracking**: Quality metrics are tracked per file type and visualizable
5. **Reports Include Insights**: Generated reports provide specific improvement recommendations

### Critical Review Upstream Factors

1. **Verifier Interface Stability**: New verifiers can be added without breaking existing code
2. **Repair Loop Correctness**: Quality never degrades across iterations (monotonic improvement or convergence)
3. **Benchmark Metadata Completeness**: Benchmarks are always stored with required metadata
4. **Report Query Performance**: Reports can be queried in <1s for 10,000+ reports
5. **Quality Metrics Accuracy**: Metrics accurately reflect true quality improvements (not false positives)
6. **Dashboard Real-Time Updates**: Quality metrics update within 100ms of new data
7. **Workflow Transformation Safety**: Transformations require explicit approval before taking effect

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0005 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (Verifier Interface Framework)
5. **Implement Phase 1**: Build verifier framework with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1, 2, 3 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-7
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Proceed**: Mark plan complete and move to plan-05

### Starting Point

Begin with Phase 1 (Verifier Interface Framework):
1. Define `Verifier` trait
2. Define `VerificationResult` struct
3. Write unit tests for trait methods
4. Write property tests for result validity
5. Implement first built-in verifier (code)
6. Verify Checkpoint 1

---

## Execution Commands

### Verify All Tests

```bash
# Run all test layers
cargo test --lib              # Layer 1: Unit tests
cargo test --test '*'           # Layer 2: Integration tests
cargo test --test '*proptest*'  # Layer 3: Property-based tests
cargo test --test '*e2e*'       # Layer 4: End-to-end tests
```

### Verify Single Phase

```bash
# Verify Phase 1 (Verifier Interface Framework)
cargo test --lib verifier
cargo test --test verifier_integration
cargo test --test verifier_proptest

# Verify Phase 2 (Generate-Verify-Repair Loop)
cargo test --lib repair
cargo test --test repair_integration
cargo test --test repair_proptest
cargo test --test repair_e2e
```

### Run Quality Loop

```bash
# Run generate-verify-repair loop on workflow
whitt-execution-engine quality-loop workflow.yml --verifiers code,docs --max-iterations 5

# Run benchmark suite
whitt-execution-engine benchmark suite.yml --output ./benchmarks

# Generate quality report
whitt-execution-engine report quality --type file-type --format html

# View quality metrics dashboard
# (Access via Glyphnova UI)
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-04

# Resume from checkpoint
./implement.sh resume plan-04 cp3

# Check progress
./implement.sh status plan-04

# Generate progress report
./implement.sh report plan-04
```

---

## References

### Related Documents

- **ADR-0005**: [Quality loops, artifact workflows, and benchmark-driven file-type expansion](../roadmap/adr-0003-quality-loops-benchmarks-artifact-workflows.yml)
- **Research Plan 04**: [Quality loops and memory & search research](../roadmap/research-plan-03-quality-memory-search.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Plan 02**: [CLI & Backends](phase-2-cli-backends/plan-02-cli-backends.md)
- **Plan 03**: [Glyphnova UI](phase-3-glyphnova-ui/plan-03-glyphnova-ui.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **criterion**: [docs.rs/criterion](https://docs.rs/criterion) - Rust benchmarking
- **codeql**: [codeql.github.com](https://codeql.github.com) - Code quality queries
- **eslint**: [eslint.org](https://eslint.org) - JavaScript verification
- **mypy**: [mypy-lang.org](https://mypy-lang.org) - Python type checking
- **shellcheck**: [shellcheck.net](https://www.shellcheck.net) - Shell script verification
- **Grafana**: [grafana.com](https://grafana.com) - Metrics visualization
- **D3.js**: [d3js.org](https://d3js.org) - Data visualization

### Tool and Plugin References

- **OpenCode Tools**: File operations, grep search, web browsing
- **bash tool**: Command execution for benchmark runs
- **lsp_diagnostics**: Type checking and lint verification
- **glob tool**: File pattern matching for workflow discovery
- **webapp-testing**: UI testing for dashboard integration

---

**Last Updated**: 2026-03-27
**Status**: Not Started
