# 10-execute-observe — Experiment Design

## Atom
**execute-and-observe + edit** — run tool/shell, observe output, classify
failure, retry with mapped recovery. The ground-truth loop.

## Hypotheses (pre-registered)
- **H1 (fault recovery):** injected tool faults (5 classes) recover ≥60%
  at the retry rung with class-mapped recovery — spoof design: fault
  class rotates by hash (5 classes + 1/6 clean), fault win = min(2, n-1)
  landing after observe→retry.
- **H2 (class value):** recovery rate with class-mapped retry beats
  blind retry by ≥15pp (v2 ablation A-2); v1 proves the machinery +
  measures per-class recovery for the mapping table.
- **H3 (clean pass-through):** clean cases (no fault) never loop —
  single execute→observe→solve pass, no spurious retries ≥95%.

## Cases
eo-01..eo-36 = 6 engines × 2 variants × 3 idx. FAULT_PLAN: {cid:
execute: <class>} — class = FAULT_CLASSES[hsh%5] for 5/6 of cases,
1/6 clean.

## Workflow (v1)
- H lane: execute(fmt, gather style) → observe(cot) → solve(cot) →
  extract → retry → extract2 → judge
- L lane: solve → extract → judge
- CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: aggregate fault recovery ≥60% AND clean loop rate ≤5% AND
  per-class recovery table populated (timeout/unreachable/garble high;
  delta hardest per S40).
- REFUTED: recovery <30% (retry rung doesn't help) or clean loops >15%
  (observation classification too noisy — false-positive trap).

## Ablations
- A-1: retry removed → fault cases all fail (faults are sticky by design).
- A-2: blind retry (class signal dropped from retry prompt) — isolates
  the MAPPING from the RETRY.

## Live variant plan
v1-live.yml ready. classify_observation maps observation→class
(fas_lib); per-class recovery matrix is the deliverable for the
generator's fault-handling table.
