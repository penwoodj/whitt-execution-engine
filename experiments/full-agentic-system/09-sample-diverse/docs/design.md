# 09-sample-diverse — Experiment Design

## Atom
**sample-diverse** — cross-model resample when same-model retry is
exhausted: 27B (deep) re-attempts after 4B wait-retry fails.

## Hypotheses (pre-registered)
- **H1 (conversion):** stuck cases (same-model wait fails) convert ≥30%
  at the cross-model rung — spoof design: 60% of H cases stuck, win at
  min(3, n-1) landing extract3 (after sample2).
- **H2 (rung efficiency):** cross-model at position 4 beats same-model
  retry-4 (would need v2 ablation A-2); v1 establishes conversion exists
  at all beyond wait (0-conversion in fusion live data for same-model).
- **H3 (verify survives):** post-diverse verification (verify → extract4)
  catches wrong-but-plausible diverse samples ≥ as well as it catches
  4B errors — diversity must not poison the verify lane.

## Cases
sd-01..sd-36 = 6 engines × 2 variants × 3 idx. Stuck = hsh%5<3 on H
cases (deterministic 60%).

## Workflow (v1)
- H lane: solve(cot) → extract → wait(fmt) → extract2 → sample2(deep,
  1200, cot) → extract3 → verify(think) → extract4 → judge
- L lane: solve → extract → judge
- SCENARIO: stuck win=min(3, n-1) — passes only after the deep rung.
- CHECK_HOOK_STAGES: extract, extract2, extract3, extract4.

## Pass/fail criteria
- PROVEN: stuck-band conversion ≥30% with clean verify lane (wrong
  diverse samples caught).
- REFUTED: conversion <10% (27B adds no diversity value at these tasks)
  or verify poisoned (diverse wrong passes verify >15%).

## Ablations
- A-1: sample2 removed → stuck band should collapse to ~0 (fusion live
  evidence: same-model retry converted 0).
- A-2: sample2 → fmt again (same-model 4th attempt) — isolates the
  DIVERSITY claim from the EXTRA-ATTEMPT claim. Critical: without A-2,
  conversion could be pure budget, not diversity.

## Live variant plan
v1-live.yml ready. Model-load tariffs real here (fmt↔deep swap) —
stage-major ordering minimizes swaps; ledger timestamps quantify the
tariff vs conversion trade.
