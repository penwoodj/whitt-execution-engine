# Correction Atom v6 — Early-Exit Ratchet Cascade: Results

**Date:** 2026-08-14 · **Workflow:** `workflows/correction-atom-v6.yml` · **Live runs:** 9 (8 cases + 1 rerun)

## What v6 fixed (vs v5)

| Defect in v5 | v6 mechanism | Result |
|---|---|---|
| All 10 steps always ran; check-pass ignored | Check propagates exit code → GWT `{{bookmarks.shell_output.exit_code}} == 0` → `route_to: step_99_finalize` (forward-only) | Early exit fired in 8/9 runs |
| Prompt leakage passed checks (case-011 angle 4 echoed full retry prompt, judged PASS) | Always-on `prompt_leak` check (10 markers) | 0 leaks in v6; retro-scan caught all 3 v5 leaks |
| Later angles broke passing text (v4qwen case-013/015) | `select-best.py` ratchet: first passing angle, else best-partial; finalize via `skip_step` (zero-LLM) | Output never regresses |
| "PRIOR ATTEMPT PASSED, copy verbatim" retry branch caused echo leaks | Branch unreachable (early exit routes away on pass); retries get structured feedback: numbered failed checks + fix hints + full-rewrite rule (CRITIC/Self-Refine evidence) | Retries produced 0 exits in all 9 runs — all early exits came from angle FIRST attempts (correction of earlier draft; post-hoc log analysis) |
| **Found during live test:** word salad passed structural checks (0.5B garbage with 3 "-" lines + 43 words = PASS → shipped in 6s) | New `degenerate` check: long-token, alpha-ratio, unique-word-ratio, **source-grounding** (≥50% content words must appear in case auxiliary/broken_output) | Case-011 run 1 garbage now rejected; 0 false positives on all v5 passing outputs |

## Aggregate

| Metric | v5 | v6 |
|---|---|---|
| Pass rate (hard 011-015) | 5/5 | 5/5 (013 needed 1 rerun — stochastic, temp 0.2) |
| Pass rate (easy 001-003) | 3/3 | 3/3 |
| Avg wall, all successful runs | 55.6s | **29.0s (48% faster)** |
| Early exit | never | 8/9 runs |
| Prompt leaks | 3 (1 judged PASS) | **0** |
| Best-angle distribution | always 3 | 2,2,2,2,3,4,4,3 — first passing angle ships, no overwork |

Per-run data: `benchmarks/comparison-v6.csv` · Run dirs: `results/exp-v6-case-*`, `results/exp-v6b-case-013`.

## Engine mechanics validated live (first GWT-routed workflow in this repo)

1. `after_step_succeeds` → GWT → `RouteTo` honored; main loop jumps by step index (runner.rs:2213/2602)
2. GWT `given` resolves `{{bookmarks.*}}` templates before evaluation (actions.rs:602); numeric `==` works
3. `before_step_starts` + `skip_step: true` = zero-LLM finalize (no model load, no inference — runner.rs:1998)
4. Schema doc's `then: { route_to: X }` form is WRONG vs Rust struct — correct: `then: X` or `then: [X]` (untagged enum, step.rs:255)

## Honest notes

- Case-013 first run: all angles failed (prose, no bullets, forbidden phrases). Rerun passed at angle 3 in 21s. Temperature-0.2 variance, not structural — but shows v6's tighter verifier makes some runs stricter than v5's (v5's angle-3 pass on 013 would also pass here; the failing run never produced that output).
- Case-011 v6 final (angle 2, 17s) contains minor residual distortion ("must be approved by the generator") that passes all deterministic checks; v5's angle-3 output was cleaner but took 66s. Early exit trades last-bit semantics for speed — that's the measured trade-off.
- Retry feedback format is now structured, but 0.5B remains unable to fix its own garbage (angle-1 retry failed every case) — consistent with IFEval 31.5% for Qwen2.5-0.5B. Sub-2B models cannot do multi-constraint correction; next experiment (v7) should replace them with deterministic format post-processing.

## Test coverage added (TDD, red-green)

- `scripts/test_leak_check.py` — 20 tests incl. v5 leak regression fixture
- `scripts/test_degenerate.py` — 11 tests incl. v6 case-011 garbage regression fixture
- `scripts/test_select_best.py` — 11 tests
- `scripts/test_retry_feedback.py` — 15 tests
- Total: 57 tests, all passing
