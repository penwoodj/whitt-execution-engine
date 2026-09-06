# 11-verify-blind — Experiment Design

## Atom
**verify-blind + two-lane det-check** — independent verifier (different
model, no generator context) reviews against contract; det checks always
override.

## Hypotheses (pre-registered)
- **H1 (catch rate):** planted answer errors (delta fault at extract)
  that pass det packaging are caught by blind verify ≥50% — spoof
  design: 2/3 of H cases planted, win = min(2, n-1) landing after
  verify→extract2.
- **H2 (no false-alarm regression):** clean cases flagged fail by verify
  ≤10% — the verifier must not strangle the pipeline (Leni bound:
  firewall FPR ≤3.6% is the gold standard).
- **H3 (judge independence):** judge_disagree events (blind judge
  contradicts det pass) recorded but never load-bearing; disagreement
  rate tracked as judge calibration data, final verdict = det always.

## Cases
vb-01..vb-36 = 6 engines × 2 variants × 3 idx. Planted = hsh%3!=2 on H
(2/3). FAULT_PLAN: {cid: extract: 'delta'} — planted error enters the
packaged answer stream.

## Workflow (v1)
- H lane: solve(cot) → extract → verify(think 3000, backward-check) →
  extract2 → judge(think, blind)
- L lane: solve → extract → judge
- EXTRA_SCENARIO: judge_disagree = planted and hsh%4==0.
- CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: planted catch ≥50% AND false alarms ≤10%.
- REFUTED: catch <25% (blind verify adds nothing over det) or false
  alarms >20% (verifier too aggressive — threshold retrains).

## Ablations
- A-1: verify removed → planted errors flow to judge only; catch delta
  = verify's value.
- A-2: verify staffed by GENERATOR model (fmt) — Leni's specialist-swap:
  expected catch collapse; isolates independence from extra-budget.

## Live variant plan
v1-live.yml ready. Verify sees digest-contract + answer-under-review
only (never solve internals). Deliverable: catch/FPR matrix + judge
calibration curve (agreement by planted status).
