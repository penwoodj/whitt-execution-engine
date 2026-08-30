# 14-budget-inflation — Experiment Design

## Atom
**budget-control** — pre-execution inflation probe picks CHEAP/ESCALATE
posture; retry exhaustion escalates FRESH (context discarded, S57);
early-abort on infeasible (BAGEN).

## Hypotheses (pre-registered)
- **H1 (inflation routing):** probe+budget stages assign the designed
  posture (LOW→win-at-solve, MID→win-after-wait, HIGH→fresh-escalation)
  ≥70% — artifact-measured via conf/budget outputs vs scenario class.
- **H2 (fresh-escalation value):** stuck cases (solve delta) recover via
  fresh sample2 (new model, no failed-context forwarding) ≥50%; the
  S57 contamination result (−34.8pp for forwarded chains) predicts
  wait-stage same-model retry converts ~0 — our own fusion data agrees.
- **H3 (thin-budget discipline):** thin-note cases (1/5 band) that fail
  cheap lane ABORT rather than escalate — budget posture actually
  changes the path, measurable as no-sample2-runs on thin band.

## Cases
bi-01..bi-36 = 6 engines × 2 variants × 3 idx. Budget-posture note in
expansion (thin vs reserve). Truth = engine truth (quality gate);
routing correctness measured from artifacts (EXTRA_SCENARIO
inflation_class LOW/MID/HIGH + thin flag).

## Workflow (v1)
- H lane: probe(unchecked, 60 tok) → budget(unchecked, plan style) →
  solve → extract → wait → extract2 → sample2(deep, FRESH prompt) →
  extract3 → judge
- L lane: probe → budget → solve(400) → extract → judge
- SCENARIO: h%5<2→0, ==2→min(2,n−1), else min(4,n−1).
- FAULT_PLAN: solve delta on HIGH band (escalation is then forced).
- CHECK_HOOK_STAGES: extract, extract2, extract3.

## Pass/fail criteria
- PROVEN: H1 ≥70% AND stuck recovery ≥50% AND thin-band sample2 runs = 0.
- REFUTED: routing <50%, or fresh escalation doesn't beat wait (H2's
  falsifier: wait-stage conversions ≈ sample2 conversions).

## Ablations
- A-1: probe+budget removed (flat H lane) — posture value = cost delta.
- A-2: sample2 receives forwarded failed context instead of fresh
  prompt — S57's contamination experiment replicated at atom level;
  expected recovery collapse.

## Live variant plan
v1-live.yml ready (504 steps). Deliverables: posture-accuracy matrix,
recovery table (wait vs fresh), thin-band path audit, token ledger per
class (the inflation measurement itself).
