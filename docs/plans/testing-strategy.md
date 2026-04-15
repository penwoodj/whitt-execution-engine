# Testing Strategy Plan

**Created**: 2026-04-15
**Status**: PLANNING
**Depends on**: Phase 0 (foundation), Phase 1 (MVP queue)

---

## Purpose

This document defines the multi-layer testing strategy for yaml-to-rust-agentsdk. Each layer is a **requirement**, not optional. Tests must be implemented incrementally and verified continuously — past tests run in batches on every change to prevent regression.

---

## Multi-Layer Testing Strategy

### Layer 1: Unit Tests — `#[cfg(test)]` co-located

| Attribute | Detail |
|-----------|--------|
| **Scope** | Per-crate, per-function |
| **Location** | Co-located in `src/**/*.rs` |
| **Tooling** | Built-in `cargo test` |
| **Coverage target** | ≥80% line coverage per crate |
| **Run frequency** | Every commit |

**Requirements**:
- Every public function has at least one test
- Error paths tested (parse failures, invalid input, edge cases)
- Default value functions tested
- Serialization round-trips tested (YAML → Struct → YAML)

### Layer 2: Integration Tests — 52 Existing YAML Workflows

| Attribute | Detail |
|-----------|--------|
| **Scope** | Parse each workflow, validate against schema, verify zero parse errors |
| **Location** | `tests/integration/` |
| **Input** | All 49 YAML files in `docs/workflows/examples/requirements-oriented-auto/` + chatgpt + manual |
| **Tooling** | `cargo test --test integration` |
| **Run frequency** | Every PR |

**Requirements**:
- Every example workflow YAML parses without error
- Every parsed workflow produces a valid IR (intermediate representation)
- Workflow validation catches intentional errors in negative-test YAMLs
- Test batch runs as single command, reports pass/fail per workflow

### Layer 3: Property-Based Tests — `proptest`

| Attribute | Detail |
|-----------|--------|
| **Scope** | Generate random valid YAML, parser never panics |
| **Location** | `tests/property/` |
| **Tooling** | `proptest` crate |
| **Strategy** | Generate arbitrary valid workflow structures, verify parser handles them |
| **Run frequency** | Every PR |

**Requirements**:
- Random valid YAML always parses (no panics, no unwrap failures)
- Random invalid YAML returns Err (never panics)
- Property: `parse(serialize(parse(yaml))) == parse(yaml)` (round-trip)
- Property: Schema validation accepts all generated valid workflows
- Minimum 10,000 generated cases per property

### Layer 4: Snapshot Tests — `insta` / `cargo-insta`

| Attribute | Detail |
|-----------|--------|
| **Scope** | Transpiler output captured, fail CI on unexpected changes |
| **Location** | `tests/snapshots/` |
| **Tooling** | `insta` crate |
| **Strategy** | Snapshot IR output, Rust code output, and validation output |
| **Run frequency** | Every PR |

**Requirements**:
- IR generation snapshots for each workflow type
- Generated Rust code snapshots
- Validation error message snapshots
- Intentional changes require explicit `cargo insta approve`
- Snapshots stored in git for diff review

### Layer 5: Benchmark Tests — `criterion`

| Attribute | Detail |
|-----------|--------|
| **Scope** | 3 benchmark YAMLs against mock providers |
| **Location** | `benches/` |
| **Tooling** | `criterion` crate |
| **Metrics** | Parse time, IR compilation time, execution throughput |
| **Run frequency** | Weekly + before releases |

**Requirements**:
- Benchmark: parse 52-workflow corpus (throughput)
- Benchmark: compile single complex workflow to IR (latency)
- Benchmark: end-to-end mock execution (latency)
- Regression threshold: 10% degradation fails CI
- Results tracked over time for trend analysis

### Layer 6: Mock Providers — `mockall`

| Attribute | Detail |
|-----------|--------|
| **Scope** | Provider trait mocking for CI without GPU/API keys |
| **Location** | `tests/mocks/` |
| **Tooling** | `mockall` crate |
| **Strategy** | Mock all LLM provider traits with configurable responses |
| **Run frequency** | Every PR |

**Requirements**:
- MockProvider implementing full Provider trait
- Configurable responses: success, timeout, rate limit, error
- Simulated streaming responses for step execution tests
- No network access required for full test suite
- Mock responses from fixture files matching real provider formats

---

## Incremental Implementation Plan

### Phase 0 (Foundation) — Test Infrastructure

| Task | Deliverable | Depends on |
|------|-------------|------------|
| T0.1 | Add test dependencies (proptest, insta, criterion, mockall) to Cargo.toml | Cargo.toml |
| T0.2 | Create `tests/` directory structure (integration/, property/, snapshots/, mocks/) | T0.1 |
| T0.3 | Implement MockProvider with mockall | Provider trait |
| T0.4 | Create test fixture loader (reads YAML from docs/workflows/examples/) | T0.2 |
| T0.5 | Set up CI test pipeline (cargo test + cargo insta test + proptest) | T0.1-T0.4 |

### Phase 1 (MVP) — Core Tests

| Task | Deliverable | Depends on |
|------|-------------|------------|
| T1.1 | Unit tests for YAML parser (schema types, variable interpolation) | Parser module |
| T1.2 | Integration test: parse all 49 requirements-oriented workflows | T0.4, Parser |
| T1.3 | Property tests: random valid YAML never panics parser | T0.1, Parser |
| T1.4 | Snapshot tests: IR output for 5 representative workflows | IR compiler |
| T1.5 | Mock provider: simulated LLM responses for step execution | T0.3 |

### Phase 2 (CLI + Backends) — Extended Tests

| Task | Deliverable | Depends on |
|------|-------------|------------|
| T2.1 | Integration tests: parse chatgpt + manual brainstorming YAMLs | T1.2 |
| T2.2 | Backend mock strategies (LM Studio, Ollama, llama.cpp) | T0.3 |
| T2.3 | End-to-end mock workflow execution tests | T1.5, Scheduler |
| T2.4 | Benchmark: parse corpus + compile IR + mock execution | T1.2, T1.4 |

### Phase 3+ — Continuous Regression

| Task | Deliverable | Depends on |
|------|-------------|------------|
| T3.1 | Nightly full regression suite (all layers) | T1.x, T2.x |
| T3.2 | Coverage tracking with `cargo-tarpaulin` | All tests |
| T3.3 | Performance regression dashboard | T2.4 |

---

## Continuous Batch Testing Protocol

### Rule: No Regressive Changes Ship

After every code change:

1. **Fast check** (local, <30s): `cargo test --lib` — unit tests only
2. **Standard check** (CI, <5min): `cargo test` — unit + integration + property
3. **Full check** (CI, <15min): `cargo test` + `cargo insta test` + `cargo bench`

### Batch Regression

Every PR runs all existing tests plus:
- All 49 workflow parse tests
- All property tests (10K cases)
- All snapshot tests
- Mock provider end-to-end tests

**If any test fails**: PR is blocked. Fix or intentionally update snapshot.

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Unit test coverage | ≥80% per crate |
| Integration: workflow parse rate | 100% of 49 workflows |
| Property: no-panic cases | 10,000+ per property |
| Snapshot stability | Changes require explicit approval |
| Benchmark regression | <10% degradation |
| Mock provider: CI passes | Without GPU/API keys |
