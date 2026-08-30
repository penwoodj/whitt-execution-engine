# 04-intent-classify — Experiment Design

## Atom
**intent-classify + lane-router** — deterministic pre-gate routes each case
to LIGHT or HEAVY lane before any expensive stage runs.

## Hypotheses (pre-registered)
- **H1 (routing accuracy):** pre-gate assigns the designed lane (H cue /
  L cue cases) ≥85% correctly. Cues are engineered surface signals; the
  gate is entropy/GWT-based (fas_lib.conf_route + fusion_conf pattern).
- **H2 (efficiency):** L-lane cases complete at ≤40% of H-lane token cost
  with no accuracy loss on L-design cases.
- **H3 (decline discipline):** DECOY cases (table-absent-but-events-suffice)
  are NOT declined — the gate declines only when solvability is truly
  absent. Decline on decays = over-conservative gate (kills coverage).

## Cases
ic-01..ic-36 = 6 engines × 2 variants × 3 idx. Variant = lane label (H
variant carries HEAVY cue note, L variant LIGHT cue note). idx%5==4 →
DECOY: contract references a table that isn't in the prompt, but events
alone suffice to answer (truth computed from events).

## Workflow (v1)
- H lane: plan→solve→extract→verify→extract2→judge (full scaffold)
- L lane: solve→extract→judge
- SCENARIO buckets [4,3,2,1] spread win-depths.
- CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: routing ≥85% AND L-cost ≤40% H-cost AND decoy decline rate ≤5%
  AND L-case accuracy ≥ H-case accuracy − 3pp (no lane mis-assignment harm).
- REFUTED: routing <70% or any wrong-lane accuracy cliff >10pp.

## Ablations
- A-1: gate removed, all cases H — expected: accuracy flat, cost ~2.5×
  (the tax the gate exists to remove).
- A-2: gate inverted — expected: accuracy collapse on H-design cases
  routed L (upper bound on gate value).

## Live variant plan
v1-live.yml ready. Conf probe files per case (fusion_conf.py pattern);
analysis: confusion matrix cue-lane × routed-lane, cost per lane from
fingerprint ledger, decoy outcomes.
