# 09-sample-diverse — Research & Reasoning

## Why this atom exists
Fusion live data: same-model wait-retry converted 0 stuck cases;
cross-model sample converted the stubborn band. External literature now
strongly confirms diversity-over-iteration at small scale. This atom
isolates that claim and — via A-2 — separates diversity from budget.

## Sources
- **Sample More, Reflect Less (arXiv 2607.28576):** 0/36 reflect methods
  beat cost-matched sampling; best-of-N self-pick loses to majority vote
  below 7B. Cross-model sampling is the strongest diversity lever.
- **Agent Primitives (2602.03695):** Voting/Selection primitive — their
  +12-16.5% comes partly from multi-attempt aggregation; our sample2+verify
  is attempt-then-verify (cheaper than vote at 2 attempts).
- **RouteGoT (2603.05818):** strong models for synthesis nodes only,
  +8.1pp at −79.1% tokens — deep model reserved for ONE rung, not the
  whole chain (our L lane never loads deep).
- **Leni (2607.17044):** executor pool per step type; frontier reasoning
  only for multi-hop — 27B at the stuck rung mirrors this staffing.
- **TRIM (ICLR 2026):** route only derail-likely steps to big model,
  5× efficiency — stuck-band detection (check-fail after wait) is our
  derail signal; cheaper than process-reward models.
- **E9/verify (fusion, proven):** cheap-verify-run-twice — H3 checks the
  verify lane still binds on diverse outputs.
- **P2 (fusion, proven):** sample-more-reflect-less at unit level.

## Reasoning chain
1. Same-model retry explores the same basin (temp 0!) — deterministic
   repeat. Cross-model changes the prior, not just the seed.
2. Cost asymmetry: deep model tariff (~load + slower tokens) must be
   paid ONLY on the stuck band — the check-gate before sample2 enforces
   this (skip-if-won routes non-stuck cases around it).
3. A-2 is the falsifier that makes this an experiment rather than a
   demo: fmt-again at rung 4 with same budget. Diversity claim survives
   only if deep beats fmt-4th.

## What would change our mind
- If A-2 (4th fmt attempt) matches deep conversion, the atom collapses
  into budget forcing (E1) — cheaper implementation, same effect.
- If verify catches <50% of wrong diverse samples, diversity without
  verification is dangerous — composition rule broken, fix verify first.

## Relation to opencode parity
opencode's flagship model IS the deep rung always. Matching it with
4B-by-default + 27B-on-stuck is the semi-efficient replacement thesis
in miniature.
