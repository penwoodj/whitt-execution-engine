# 02-digest — Experiment Design

## Atom
**digest** — downstream stages see core facts + output contract only
(~150-200 words), not the full 1k+ prompt with worked example + expansion.

## Hypotheses (pre-registered)
- **H1 (solvability preserved):** cases routed through digest-only retry
  stages (wait, extract2) pass at ≥90% of full-context retry rate. Fusion
  unit-proved digest integrity; this tests it at workflow scale with
  fact-position pressure.
- **H2 (token economy):** digest retry path uses ≤35% of the tokens of a
  full-prompt retry path (measured via fingerprint ledger tokens_approx).
- **H3 (position robustness):** EML probe note (early/middle/late fact
  placement in core) — digest keeps all task digits by construction
  (verified at generation), so live position effects should be ≈0. Any
  position-correlated failures implicate the model, not the digest.

## Cases
dg-01..dg-36 = 6 engines × 2 variants × 3 idx. idx rotates fact-position
emphasis in the probe note. Truths from engine simulation; digest verified
to retain contract + all task digits + ≤400 words (gen-cases asserts).

## Workflow (v1)
- H lane: solve(fmt,cot) → extract(fmt) → wait(fmt) → extract2(fmt) → judge(think)
- L lane: solve(fmt,cot) → extract(fmt) → judge(think)
- SCENARIO buckets [5,2,1]: most win at solve; 1/3 need digest-based retry.
- CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: digest-retry pass rate ≥ 0.9 × full-retry control AND token
  ratio ≤ 0.35.
- REFUTED: digest retry loses >10pp or any engine shows systematic digest
  failure (would mean slicing loses engine-critical info — fix digest, not atom).

## Ablations
- A-1: no digest (all retry stages get full prompt) — expected: token cost
  ~3×, no accuracy gain (fusion E10 evidence).
- A-2: digest without contract tail — expected: format failures spike
  (contract is the load-bearing slice).

## Live variant plan
v1-live.yml ready; fmt solve/extract, think judge. Compare per-stage
fingerprint tokens_approx digest vs full; win-depth distributions; failure
taxa by fact-position bucket.
