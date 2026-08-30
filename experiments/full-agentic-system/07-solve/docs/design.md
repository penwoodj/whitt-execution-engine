# 07-solve — Experiment Design

## Atom
**solve** — narrow-scope CoT inference unit. The control experiment: no
extra scaffold manipulation, pure baseline establishment across engines,
variants, and 4 difficulty idx levels.

## Hypotheses (pre-registered)
- **H1 (baseline curve):** pass rate declines monotonically with idx
  (difficulty ramp) — establishes the difficulty gradient all other
  experiments borrow for their H/L manipulations.
- **H2 (CoT necessity):** cot style (P1 prefix) beats empty style at
  matched tokens (live comparison deferred; spoof cannot test style —
  it tests machinery only).
- **H3 (control stability):** buckets [6,2] — ~75% win at depth 0, 25%
  at depth 1 (wait+retry). Live runs that deviate >15pp from this
  design indicate case-set drift (regeneration bug), not model behavior.

## Cases
sv-01..sv-48 = 6 engines × 2 variants × 4 idx. No extra expansion notes —
the shared scaffold only. This is the reference case set: same engines,
same truths as every other experiment, zero manipulation.

## Workflow (v1)
- H lane: solve(cot) → extract → wait → extract2 → judge
- L lane: solve(cot) → extract → judge
- SCENARIO buckets [6,2]; CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- This atom has no PROVE/REFUTE — it produces the baselines:
  - per-engine × variant × idx pass rates (the difficulty map)
  - FORMAT vs CONTENT failure split at each depth
  - token cost per depth (fingerprint ledger)
- Other experiments' hypotheses are evaluated AGAINST these numbers.

## Ablations
- A-1 (v2, live): style sweep ''/cot/wait prefix at matched budget —
  the P1 regression.
- A-2 (v2, live): temperature 0 vs 0.3 on near-miss band — reproducibility
  floor measurement (S48 paired-seed discipline).

## Live variant plan
v1-live.yml ready. Run FIRST in live phase (orchestration doc Phase A);
its curves are the denominators for 01/02/04/06 analysis.
