# Integration Testing Protocol — Meta-Workflow v6

**Status:** ACTIVE
**Last Updated:** 2026-07-01
**Scope:** Full end-to-end integration testing of meta-workflow-v6 generator + execution engine, against all 20 active test prompts.

## Purpose

This document defines the protocol for validating that the meta-workflow pipeline produces real, working deliverables on all 20 prompts. It enforces the pass criteria agreed with the user on 2026-07-01.

## Pass Criteria (HARD — all three required)

A prompt passes if and only if ALL of:

1. **SW_WINS** — `parity-check.sh` returns `VERDICT: PASS`. Workflow achieves the prompt task.
2. **Shell failure rate < 10%** — Of all `[shell]` actions logged during execution, less than 10% fail.
3. **Zero refusals** — No step output contains refusal patterns (`I cannot`, `As an AI`, `cannot access`, etc.).

Any single failure → verdict FAIL.

## Component Inventory

| Script | Purpose |
|--------|---------|
| `scripts/meta-v6/integration-test.sh` | Main runner. Semi-automated (checkpoint between prompts) or `--no-stop` for fully automated. |
| `scripts/meta-v6/check-step-outputs.sh` | Per-step quality checker. Returns JSON: empty/refusal/placeholder/hallucination counts per step file. |
| `scripts/meta-v6/extract-and-compile.sh` | Extracts Rust and TypeScript code blocks from deliverables, runs `cargo check` and `tsc --noEmit`. |
| `scripts/meta-v6/compare-before-after.sh` | Side-by-side diff of step outputs vs `docs/plans/meta-workflow-parity/cycle-3-results/prompt-XX/`. |
| `scripts/meta-v6/generate-dashboard.py` | Builds interactive HTML dashboard (`dashboard.html`) from `summary.tsv` + per-prompt `result.json`. |
| `tests/sw4_fast_regression.rs` | Permanent fast regression test (runs on every push via `cargo test`). Validates the 3 fixes (commits 7e4dbbe, a2dded1, 35e6abf). |

## Usage

### Full semi-automated run (interactive)

```bash
scripts/meta-v6/integration-test.sh
# Stop between each prompt for inspection/approval
```

### Subset of prompts

```bash
scripts/meta-v6/integration-test.sh 05 06 14
```

### Fully automated (CI mode)

```bash
scripts/meta-v6/integration-test.sh --no-stop
```

### Skip generation, reuse existing workflows

```bash
scripts/meta-v6/integration-test.sh --skip-gen
```

### Standalone quality check on existing exec dir

```bash
scripts/meta-v6/check-step-outputs.sh docs/.../p05-final-.../exec-final
```

### Standalone code compilation check

```bash
scripts/meta-v6/extract-and-compile.sh docs/.../p05-final-.../exec-final
```

## Output Structure

Per run, results land at:
```
docs/benchmarks/outputs/meta-workflow/integration-tests/run-<TS>/
├── summary.tsv                          # All 20 prompts, one row each
├── dashboard.html                       # Interactive HTML report
└── prompt-XX/
    ├── workflow.yml                     # Assembled workflow
    ├── pipeline.log                     # run-prompt-end-to-end.sh output
    ├── exec.log                         # whitt benchmark output
    ├── exec-score.txt                   # parity-check.sh output
    ├── step-quality.json                # check-step-outputs.sh JSON
    ├── compile.json                     # extract-and-compile.sh JSON
    ├── comparison.json                  # compare-before-after.sh JSON
    └── result.json                      # Aggregated pass/fail verdict
```

## summary.tsv Schema

Tab-separated, columns:
1. `prompt` — Prompt number (e.g. `05`)
2. `sw_wins` — `true` / `false` (parity-check.sh verdict)
3. `shell_fail_rate` — Fraction 0.0–1.0
4. `refusals` — Integer count
5. `step_quality` — `true` / `false` (no empty/refusal/placeholder)
6. `rust_compiles` — `true` / `false` / `na`
7. `ts_compiles` — `true` / `false` / `na`
8. `duration_sec` — Wall clock duration
9. `verdict` — `PASS` / `FAIL` / `PIPELINE_FAIL` / `NO_EXEC`

## Dashboard

Open `dashboard.html` in any browser. Features:
- Summary cards: overall pass rate, SW wins count, refusals count, Rust/TS compile rates
- Filter bar: search by prompt name, filter by verdict
- Per-prompt table: click any row to expand drilldown
- Drilldown view: full `result.json`, per-step quality table, compile errors, before/after size diff

## SW4 Permanent Regression Test

`tests/sw4_fast_regression.rs` runs on every push via `cargo test`. Covers:
1. Sample SW4 input → assembled YAML parses
2. Bare `cat step_*.txt` → rewritten to `$WHITT_OUTPUT_DIR/outputs/`
3. `cat ./outputs/step_*.txt` → rewritten to `$WHITT_OUTPUT_DIR/outputs/`
4. Unknown step-level fields (`intent`, `fit`, `description`) stripped
5. Schema-valid fields (`generative_entity`, `prompt`, `depends_on`, `retry`) preserved
6. `$WHITT_OUTPUT_DIR/outputs/` cat pattern still derives `depends_on` correctly

Failure of any of these tests blocks merge — they catch regressions in the 3 core fixes.

## Time Budget

- **Full 20-prompt run:** ~20 hours (60min per prompt: 45min meta-v6 generation + 15min execution + quality checks)
- **Subset of 6 high-failure prompts:** ~6 hours
- **SW4 fast regression test alone:** <5 seconds

## Deferral / Known Limitations

- **External file references** (e.g. `package.json` from another project): out of scope. Workflows that reference files not in the execution repo will still fail shell hooks. This is a model-prompting issue, not an engine bug.
- **Code compilation heuristic:** extracted code blocks are concatenated into a single `lib.rs` for Rust, which may produce spurious errors due to symbol conflicts. The intent is to catch gross syntax errors, not enforce standalone compilation.
- **`during_step_streaming` trigger:** not wired in the runner (architectural). Will not be exercised by integration tests.

## Change Log

- **2026-07-01:** Protocol created. Initial infrastructure built and tested against P05 cycle-3 results.
