# GitHub Actions CI/CD Improvements Plan

**Created**: 2026-04-15
**Status**: PLANNING
**Depends on**: 100-model benchmark run complete
**Roadmap reference**: To be added to roadmap after benchmark milestone

---

## Purpose

Define GitHub Actions improvements to implement after the 100-model benchmark run establishes baseline performance. These improvements ensure CI/CD catches regressions before they ship.

---

## Current State

- `.github/` exists but may have minimal CI configuration
- No automated test pipeline
- No benchmark regression detection
- No snapshot test integration

---

## Target State: Full CI Pipeline

### Workflow 1: Fast Check (PR Gate)

**Trigger**: Every PR, every push to `initial-creation`
**Timeout**: 5 minutes
**Steps**:
1. `cargo fmt --check` — formatting
2. `cargo clippy -- -D warnings` — linting
3. `cargo test --lib` — unit tests only
4. `cargo test --test integration` — workflow parse tests

### Workflow 2: Full Check (Merge Gate)

**Trigger**: Push to main/merge to main
**Timeout**: 15 minutes
**Steps**:
1. All fast check steps
2. `cargo test` — all tests including property-based
3. `cargo insta test` — snapshot tests
4. `cargo tarpaulin --out Xml` — coverage report
5. Coverage badge update

### Workflow 3: Benchmark Check (Weekly + Release)

**Trigger**: Weekly cron + tags
**Timeout**: 30 minutes
**Steps**:
1. `cargo bench` — criterion benchmarks
2. Compare against previous results
3. Fail if >10% regression
4. Upload results to GitHub Actions artifacts

### Workflow 4: Nightly Full Regression

**Trigger**: Daily cron at 02:00 UTC
**Timeout**: 60 minutes
**Steps**:
1. Full test suite with extended proptest cases (100K)
2. All snapshot tests
3. All benchmarks
4. Coverage report + upload
5. Performance trend tracking

---

## Implementation Steps

### Step 1: Create `.github/workflows/ci.yml`

```yaml
name: CI
on:
  push:
    branches: [main, initial-creation]
  pull_request:
    branches: [main]

jobs:
  fast-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test --lib
      - run: cargo test --test integration
```

### Step 2: Add test dependencies to Cargo.toml

```toml
[dev-dependencies]
proptest = "1.4"
insta = { version = "1.34", features = ["yaml"] }
mockall = "0.12"
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "workflow_benchmark"
harness = false
```

### Step 3: Create benchmark file `benches/workflow_benchmark.rs`

Baseline benchmarks against 3 representative workflows:
- Simple: `01-basic-model-selection-providers.yaml`
- Medium: `04-hybrid-step-workflows.yaml`
- Complex: `01-complex-orchestration-sub-agents.yaml`

### Step 4: Add snapshot test infrastructure

Create `tests/snapshots/` directory with initial snapshots for:
- IR output of each workflow type
- Validation error messages
- Generated Rust code (when transpiler is functional)

### Step 5: Configure coverage reporting

```yaml
- name: Generate coverage
  run: |
    cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml --output-dir ./coverage
- name: Upload coverage
  uses: codecov/codecov-action@v4
  with:
    files: ./coverage/cobertura.xml
```

---

## Prerequisites

- [ ] 100-model benchmark run complete (establishes baseline)
- [ ] Test dependencies added to Cargo.toml
- [ ] Mock provider implemented
- [ ] Integration test framework reads from docs/workflows/examples/

---

## Roadmap Integration

After 100-model benchmark:
1. Add this plan reference to `docs/roadmap/adr-0000-roadmap-index.yml`
2. Create ADR-0009 for CI/CD strategy
3. Link from Phase 2 (CLI + Backends) validation criteria

---

## Success Criteria

| Metric | Target |
|--------|--------|
| CI fast check | <5 min |
| Full check | <15 min |
| Benchmark regression detection | 10% threshold |
| Coverage visibility | Codecov badge in README |
| Nightly regression | 100% test pass rate |
