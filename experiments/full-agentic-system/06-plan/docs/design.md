# 06-plan — Experiment Design

## Atom
**plan** — plan-once scaffold: decomposition with formulas/dependencies
listed before any answering. Engine owns dispatch; plan never executes.

## Hypotheses (pre-registered)
- **H1 (lift):** plan-first raises pass rate ≥15pp over direct solve on
  multi-dependency cases (win-depths designed [4,3,2,1] so most cases
  need the scaffold).
- **H2 (dependency respect):** dependency-chain note (with red-herring
  dep) — answers respect true deps; red-herring violations ≤10%.
- **H3 (no hallucinated order):** plan stages produce step lists whose
  order matches engine rule order (checkable in live via plan-stage
  artifact inspection) ≥80%.

## Cases
pl-01..pl-36 = 6 engines × 2 variants × 3 idx. Dependency-chain note in
expansion enumerates apply-order with one red-herring dependency that
must NOT be honored (engine truth ignores it).

## Workflow (v1)
- H lane: plan(fmt, plan style, unchecked) → solve(cot) → extract →
  verify(think) → extract2 → judge
- L lane: solve → extract → judge (control = no plan)
- SCENARIO buckets [4,3,2,1]; CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: H-lane ≥ L-lane + 15pp AND red-herring violations ≤10%.
- REFUTED: plan adds <5pp (scaffold not load-bearing at these difficulty
  levels) or red-herring violations >25% (plan amplifies misordering —
  RCA structure-hurts pattern).

## Ablations
- A-1: plan removed → L lane becomes whole experiment (this IS the
  in-design control).
- A-2: plan executes + answers (violates plan-once) — expected: accuracy
  drop from premature answering; mirrors P4 evidence.

## Live variant plan
v1-live.yml ready. Plan artifacts inspected for rule-order match; judge
blind lane verifies contract only.
