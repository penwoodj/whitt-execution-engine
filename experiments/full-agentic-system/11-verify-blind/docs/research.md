# 11-verify-blind — Research & Reasoning

## Why this atom exists
Det checks catch format + exact-truth, but real agentic work has
near-truth errors det can't see (wrong-but-valid). The two-lane verdict
(det overrides) is our established pattern; the open question is the
BLIND lane's catch rate and false-alarm cost at 4B/27B scale.

## Sources
- **Leni (arXiv 2607.17044):** verifier catch c≈0.20, fix r≈0.75, no
  false-alarm regressions; verification contribution small (+1.5pp) but
  positionally decisive (top-of-leaderboard conversions). Specialist-
  swap ablation: verifier=generator kills rescues → our A-2.
- **Rulers (2601.08654):** executable rubrics + evidence gates let small
  judges rival large — verify prompt is rule-by-rule backward check
  (rubric-shaped), not holistic opinion.
- **RubricForge (2608.13564):** false-pass rate = deployment metric;
  induced rubrics halve over-crediting — our H2 false-alarm bound is
  the mirror metric.
- **RuVerBench:** judge reliability 51.6-94.7 by domain/model/prompt —
  judge calibration data (H3) is mandatory, judges are not interchangeable.
- **Preference leakage (2502.01534):** judges favor related students —
  blind lane exists precisely to starve the judge of provenance.
- **P10 cheap-verify-run-twice + E9 (fusion, proven):** verify lane
  already earned its place in fusion; this experiment quantifies the
  catch/FPR frontier.
- **S44 (NeurIPS):** SWE-Lancer test-overwrite lesson — checks
  themselves can be gamed; det-check truths computed by independent
  simulation, never by the model under test.

## Reasoning chain
1. Plant deterministic deltas at extract (post-solve) — errors are
   format-valid, value-wrong: exactly det-invisible class.
2. Verify (think model) gets contract digest + answer only; backward
   check re-derives each value; mismatch → fail verdict → retry path.
3. Two-lane: det PASS is necessary; verify FAIL routes to retry but
   never flips a det FAIL to pass. Judge runs blind AFTER, disagreement
   logged for calibration, never gating.
4. Catch/FPR matrix over planted/clean bands is the atom's product —
   it parameterizes when generator emits the verify stage at all.

## What would change our mind
- If false alarms track planted catch 1:1, the frontier is flat —
  verify adds risk as fast as safety; demote to deep-lane only.
- If A-2 (generator-model verifier) matches think-model catch,
  independence is free — staff verify with fmt and save the model swap.

## Relation to opencode parity
opencode trusts flagship judgment; we can't — two-lane + calibration
data is how a 4B system earns comparable trust deterministically.
