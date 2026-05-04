# QA Plan: Multi-Model Benchmarking & Workflow Loops

## Overview

Verification criteria for all implementation phases. Every phase must pass QA before proceeding to the next.

## QA Areas

### QA-01: Infrastructure (Phase A)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-01-01 | External drive path `/run/media/jon/data/models/` accessible from Docker container | Manual | P0 |
| QA-01-02 | `ModelCandidate` discovery returns correct count (209 models ≤6GB) | Unit | P0 |
| QA-01-03 | Disk space check returns correct free bytes for main drive | Unit | P0 |
| QA-01-04 | Benchmark aborts with clear error if main drive < 5GB free | Unit | P0 |
| QA-01-05 | Cleanup removes any temp files after model unload | Unit | P1 |
| QA-01-06 | `ModelCandidate::fits_in_vram` correctly computes for edge cases (5.9GB fits, 6.1GB doesn't) | Property | P0 |

**Verification**:
```bash
cargo test model_candidate --all-features
cargo test disk_monitor --all-features
```

### QA-02: Multi-Model Benchmark Command (Phase B)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-02-01 | `whitt benchmark --models-dir PATH` discovers all GGUF files in directory | Integration | P0 |
| QA-02-02 | `whitt benchmark --model-list FILE` reads specific model paths from file | Integration | P0 |
| QA-02-03 | Sequential model swap: unload → load → benchmark → unload works for 3 models | Integration | P0 |
| QA-02-04 | `--concurrent` flag removed from CLI (no dead code) | Unit | P1 |
| QA-02-05 | BenchmarkResult contains correct percentile calculations | Unit | P0 |
| QA-02-06 | JSON output is valid JSON with all required fields | Unit | P0 |
| QA-02-07 | CSV output has correct headers and data rows | Unit | P0 |
| QA-02-08 | Console table is aligned and readable | Manual | P2 |
| QA-02-09 | Failed model doesn't abort entire benchmark (continues to next) | Integration | P0 |
| QA-02-10 | Token-per-second calculation is accurate | Unit | P0 |
| QA-02-11 | `--filter-size-max` correctly skips models over size limit | Unit | P1 |
| QA-02-12 | `--filter-name` regex correctly filters model names | Unit | P1 |

**Verification**:
```bash
cargo test benchmark --all-features
# Manual: run 3-model benchmark against live server
whitt benchmark --models-dir /run/media/jon/data/models --filter-size-max 3000000000 --prompts 1
```

### QA-03: GPU vs CPU Comparison (Phase C)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-03-01 | `--compare-gpu-cpu` runs each model twice with different GPU settings | Integration | P0 |
| QA-03-02 | GPU speedup factor calculated and displayed correctly | Unit | P0 |
| QA-03-03 | Server restart between GPU/CPU runs succeeds | Integration | P0 |
| QA-03-04 | Paired results per model show meaningful comparison | Manual | P1 |

**Verification**:
```bash
cargo test gpu_cpu_comparison --all-features
```

### QA-04: Workflow Loop Execution (Phase D)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-04-01 | Count-based loop executes exactly N iterations | Unit | P0 |
| QA-04-02 | Validation loop terminates when criteria met (early exit) | Unit | P0 |
| QA-04-03 | Validation loop terminates at max_iterations if criteria never met | Unit | P0 |
| QA-04-04 | `iteration_variable` accessible in prompt template as `{{loop.current_file}}` | Unit | P0 |
| QA-04-05 | Loop hooks execute at correct lifecycle points | Unit | P0 |
| QA-04-06 | `after_loop_iteration_fails` hook receives error details | Unit | P1 |
| QA-04-07 | Output from iteration N accessible in iteration N+1 | Unit | P0 |
| QA-04-08 | `ComparisonOperator` correctly evaluates >=, <=, ==, !=, >, < | Property | P0 |
| QA-04-09 | Nested loops work (loop inside loop step) | Unit | P1 |
| QA-04-10 | LoopConfig deserializes from YAML without errors | Unit | P0 |
| QA-04-11 | `foreach` loop iterates over provided items list | Unit | P0 |

**Verification**:
```bash
cargo test loop_executor --all-features
cargo test loop_config --all-features
cargo test comparison_operator --all-features
```

### QA-05: Agentic Summarization-Expansion (Phase E)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-05-01 | `oscillate_abstraction` step type parses from YAML | Unit | P0 |
| QA-05-02 | File chunking respects paragraph boundaries | Unit | P0 |
| QA-05-03 | Each chunk oscillates summarize/expand N times | Integration | P0 |
| QA-05-04 | Previous iteration output feeds into next iteration correctly | Unit | P0 |
| QA-05-05 | Overlap between chunks preserves context | Unit | P1 |
| QA-05-06 | Final output preserves all technical substance from original | Manual | P0 |
| QA-05-07 | Token budget estimation is reasonable (within 2x of actual) | Manual | P2 |
| QA-05-08 | `--input-file` CLI flag works with workflow run | Integration | P0 |
| QA-05-09 | `read_file_chunk` tool reads file by offset/length | Unit | P0 |

**Verification**:
```bash
cargo test oscillation --all-features
cargo test chunker --all-features
# Manual: run document refinement workflow against live server
```

### QA-06: Benchmark YAML Suite (Phase F)

| ID | Criteria | Type | Priority |
|----|----------|------|----------|
| QA-06-01 | `benchmark-3-models.yml` parses as `UnifiedConfig` without errors | Unit | P0 |
| QA-06-02 | `benchmark-5-models.yml` parses as `UnifiedConfig` without errors | Unit | P0 |
| QA-06-03 | `benchmark-15-models.yml` parses as `UnifiedConfig` without errors | Unit | P0 |
| QA-06-04 | `benchmark-50-models.yml` parses as `UnifiedConfig` without errors | Unit | P0 |
| QA-06-05 | All YAML files conform to schema version 2.0.0+ | Unit | P0 |
| QA-06-06 | All referenced models exist on external drive | Manual | P0 |
| QA-06-07 | Model selection has correct tier distribution | Unit | P1 |
| QA-06-08 | `whitt benchmark-suite YAML_FILE` command executes | Integration | P0 |
| QA-06-09 | 3-model benchmark suite completes end-to-end | Integration | P0 |

**Verification**:
```bash
cargo test yaml_generator --all-features
cargo test benchmark_suite --all-features
# Manual: validate each YAML file
whitt workflow show docs/workflows/benchmarks/benchmark-3-models.yml
```

## Global Verification (All Phases)

### Build & Test Gate (mandatory before any commit)

```bash
cargo test --all-features          # ALL tests pass
cargo clippy --all-features -- -W clippy::all  # 0 warnings
cargo build --release --all-features            # exit code 0
```

### LSP Diagnostics

Run `lsp_diagnostics` on every modified `.rs` file. Zero errors required.

### Regression Prevention

- All existing 195 tests must continue to pass
- New tests added for each phase
- No test ignored without documented reason

## QA Execution Order

```
QA-01 (Infrastructure) ─── must PASS before QA-02
QA-02 (Benchmarking) ─── must PASS before QA-03
QA-03 (GPU vs CPU) ─── independent, but after QA-02
QA-04 (Workflow Loops) ─── independent, parallel with QA-02
QA-05 (Agentic Workflow) ─── must PASS QA-04 first
QA-06 (YAML Suite) ─── must PASS QA-02 + QA-04 first
```

## Evidence Template

Each QA area produces a findings file:

```markdown
# QA Findings: [Area Name]
Date: [ISO date]
Phase: [A-F]

## Results
| ID | Status | Evidence |
|----|--------|----------|
| QA-01-01 | ✅ PASS | Manual check: `ls /run/media/jon/data/models/` returns 56 directories |
| QA-01-02 | ✅ PASS | `cargo test model_candidate` — 12 tests pass |

## Summary
- Pass: N
- Fail: 0
- Skipped: 0
- Regression: 0
```
