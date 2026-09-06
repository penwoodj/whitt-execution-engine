# v6 Zero-LLM Verification Report

> 2026-08-23. Constraint honored: ZERO LLM calls, ZERO model load/unload.
> Docker server left untouched (health-endpoint only, idle throughout).

## Verification Evidence

### 1. Spoof Battery — 40/40 GREEN (`tests/test_v6_spoof.py`)
10 cases × 4 scenarios (wind0/wind1/wind2/lose) through the REAL engine
(`whitt benchmark`), real hooks, real GWT routing, real gate scripts:
- Gate sequences EXACT per scenario (check→final→judge wins; check→fix→…→end_fail loses)
- Final JSON == case json_exact on all win scenarios (deterministic aggregate)
- judge/end_fail mutually exclusive (no wasted judge touch — v3 bug dead)
- No inference markers in engine output (zero-LLM confirmed per run)
- ~0.8s per engine run (vs 23-61s live) — structure-only, per REA+ R5

### 2. Unit Tests — 60/60 (`tests/test_v6_scripts.py` + prior suite 97)
New coverage: table_lib parse (objects/floats/garbage/CoT-noise),
check-table (missing/wrong/leak-safe observed-only/floats/only-filter),
aggregate (ALL 10 cases reproduce json_exact from correct entities — the
linchpin assertion), failflag (pass/fail/missing/final-check), 
table-feedback (leak-safety), spoof-write.

### 3. Structural — 10/10 live YAMLs + 10/10 spoof YAMLs validate PASS
`python3 scripts/meta-v6/validate-workflow.py` green on all 20.

### 4. Zero-LLM mechanism (proven, not assumed)
- spoof-write writes canned stage text where inference would
- REAL check-table/aggregate/check-deterministic run on it
- RouteTo/SkipStep in before_step_starts prevent own-step inference
  (runner.rs:2021-2047 verified)
- Aggregate step NEVER infers (hook-only step, skip_step) — live AND spoof

## The v6 Workflow (one shape, all 10 cases)

s00_screen → s01_solve(TABLE) → s02_fix_1(cells) → s03_fix_2(from-scratch)
→ s04_aggregate(DETERMINISTIC, hook-only) → s05_judge(blind) 
→ s06_end_gate(failflag: routes only on real failure) → s07_fail_trace → s_end

Research principles applied:
- P3 decomposition: per-entity table rows (ha-06's 6 cases → 6 rows)
- P2 deterministic offloading: aggregate.py does ALL composition math (r=0.72)
- ha-08 fix: `end_fail_line` is a REQUIRED cell — omission caught by gate
- F8 leak-safe: feedback = cell id + observed + question, never expected
- P19 role isolation: fix_2 = history-free rebuild
- F9 repair cap 2; early-exit at any pass (win-depth 0 dominant in v3)
- GWT exhaustive routes (==0/!=0) — no fall-through leakage (3 bugs found
  and killed by the battery itself: single-route fall-through, s07 phantom
  trace, float typed_equal)

## Load-Time Engineering

- Conditional restart: zombie-count.sh gate, restart only when >3 procs
  (v3 paid 15s × 9 = 135s unconditional; v6 pays ~0 when router healthy)
- Same warm 9B across all cases (no model swaps)
- Aggregate + judge prompts tiny (128 tok); only solve infers big

## Expected Live Outcomes (hypotheses — NOT yet run, needs LLM window)

| Case | v3 | v6 expected | mechanism |
|------|----|-------------|-----------|
| ha-06 | FAIL | PASS | 6 trivial row classifications + det sum |
| ha-08 | FAIL 5/5 | PASS | end_fail row forced by table schema |
| other 8 | PASS | PASS | table ⊇ enumeration prompt that already won |

If model still misses a ROW, per-cell feedback names it — repair-friendly
(R3 FORMAT-class). If rows right → aggregate is deterministic → PASS
guaranteed by construction (verified in unit test).

## Files

- `scripts/gen-v6.py` — generator (live + spoof + scenarios)
- `scripts/check-table.py`, `table-feedback.py`, `aggregate.py`,
  `failflag.py`, `spoof-write.py`, `table_lib.py`, `zombie-count.sh`
- `workflows/v6-ha-{01..10}.yml` (live) + `v6-ha-*-spoof.yml`
- `cases/ha-*.yml` — now carry `v6:` entity/aggregate truth blocks
- `fixtures/spoof-v6/` — 120 canned stage artifacts (generated, typo-proof)
- `tests/test_v6_spoof.py` (40 engine runs), `tests/test_v6_scripts.py` (20)
- `scripts/run-v6-suite.py` — live runner w/ conditional restart

## Next (requires LLM permission)

`python3 experiments/harness-inspired/scripts/run-v6-suite.py` — one
command, ~10 cases, expected 10/10 or 9/10 + repair-friendly depth-1 wins.
