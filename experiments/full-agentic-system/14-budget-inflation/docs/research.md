# 14-budget-inflation — Research & Reasoning

## Why this atom exists
Every efficiency principle we've proven (E1/E3/E6, routing, lanes) needs
a unifying control surface: how much total budget does THIS prompt get,
when does it escalate, and when does it stop. Externally, 2026 produced
a whole budget-aware family — our engine can implement their findings
as deterministic gates without training anything.

## Sources
- **S57 InflationAgent (arXiv 2608.13571):** token inflation (true
  workflow cost ÷ single-call cost) reaches 4.25× for 7B on multi-hop;
  CBE (CoT Branching Entropy) pre-execution difficulty signal, AUROC
  0.887, computed from K short local samples at zero API cost — our
  probe stage is CBE-shaped (entropy of short samples = fas_lib
  entropy_from_logprobs on probe output). Fresh-escalation validated:
  forwarding failed chains to a stronger model −34.8pp vs clean prompt.
  Compute sweet spot: small-model retries peak at ~5× then DECLINE
  (context interference).
- **S54 BAAR (2602.21227):** per-step cheap/expensive choice under
  strict per-task budgets; boundary policies (always-small/always-large)
  anchor evaluation; fail-cheap for intractable tasks — our ABORT
  posture.
- **S55 BAVT (2603.12634):** budget-conditioned selection — remaining-
  budget ratio as scaling exponent, exploration→exploitation shift as
  budget depletes; low-budget BAVT beats 4× compute baselines; forced
  generation on exhaustion (never end with nothing).
- **S56 TAB (2604.05164):** multi-turn budgets = sequential allocation
  problem; 35-40% token savings; "fund the crucial harder turns" —
  our reserve posture note operationalizes this in prose.
- **S58 BAGEN (2606.00198):** agents are systematically over-optimistic;
  early-stop saves 28-64% tokens on failed trajectories at 1.6-4.2pp
  success cost — thin-budget ABORT is the workflow-owned version.
- **RouteGoT budget scheduler (fusion S-set):** synthesis reserve
  B_syn = max(B_min, α·B_total) — merge/judge stages must never be
  starved by earlier retries.
- **E1/E3/E6 (fusion, proven):** bounded wait, early-abort cascade,
  difficulty-budget routing — the seeds this atom unifies.

## Reasoning chain
1. Cheap probe (60 tok) + entropy signal → inflation class → posture.
2. Posture changes PATH not effort: thin = no escalation runway;
   reserve = one fresh escalation after clean lane failure.
3. Escalation is FRESH by construction (S57): sample2's stage-emit
   builds from digest + priors, not the failed chain (A-2 falsifies).
4. Truth stays engine-truth: quality must not bend to budget; posture
   is measured in the artifact ledger, not the answer.

## What would change our mind
- If wait-stage conversions match fresh sample2, our fusion same-model-
  retry-zero finding doesn't generalize — budget atom simplifies to
  plain cascading.
- If probe entropy can't separate classes ≥70%, drop probe; budget
  note + engine hash alone pick posture (cruder, still deterministic).

## Relation to opencode parity
opencode has no budget surface at all (flagship = infinite runway).
Deterministic budget control is where a local system is STRICTLY more
deployable than the thing it replaces.
