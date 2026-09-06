# 08-replan — Experiment Design

## Atom
**replan + bounded-retry-escalate** — scoped re-planning on check-failure:
retry → local patch → scoped replan → escalate. Never global re-solve
from scratch when a near-miss is recoverable.

## Hypotheses (pre-registered)
- **H1 (recovery rate):** scoped replan converts ≥50% of near-miss cases
  (first attempt fails on delta-fault, second attempt with replan note
  passes) — spoof design: 75% of H cases are near-miss, win at
  min(2, n-1) landing after replan+solve2.
- **H2 (cost bound):** replan path ≤40% of full re-solve token cost
  (replan consumes prior attempt + leak-safe hint, not the whole prompt
  re-derived).
- **H3 (hint leak-safety):** recovered answers never match truth on
  non-derivable keys without derivation (no oracle leakage through the
  failure hint) — hint carries check-id + observed + question only (N6).

## Cases
rp-01..rp-36 = 6 engines × 2 variants × 3 idx. FAULT_PLAN: {cid: solve:
'delta'} for 75% of H cases (deterministic hash) — first solve emits
near-miss (spoof), forcing the replan path. SCENARIO near-miss win =
min(2, n-1).

## Workflow (v1)
- H lane: solve(cot) → extract → replan(fmt, replan style, unchecked) →
  solve2(cot) → extract2 → judge
- L lane: solve → extract → judge
- CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: near-miss recovery ≥50% AND cost ratio ≤0.4 AND zero leak
  incidents.
- REFUTED: recovery <25% (replan scaffold doesn't help 4B re-orient) or
  leak found (hint format bug — hard fail, fix immediately).

## Ablations
- A-1: replan removed (retry same plan) — the wait-retry control;
  recovery delta = replan's specific value over resampling.
- A-2: global replan (full prompt re-derive) — expected: cost ~2.5×,
  same recovery → scoped wins on efficiency alone.

## Live variant plan
v1-live.yml ready. Analysis: recovery rate on near-miss band; ledger
token ratio replan-path vs solve-path; hint-forensics on recovered cases.
