# 04-intent-classify — Research & Reasoning

## Why this atom exists
The primary objective's simplest failure mode: paying full-chain cost on
trivial prompts. The generator must learn to skip excessive validation on
simple inputs — that learning is a routing decision made BEFORE any
expensive stage. This experiment isolates the router as its own atom.

## Sources
- **STEER (AAAI):** routes on small-model logit confidence (GMM-calibrated)
  — +20% accuracy at 48% less FLOPs, no external router model.
- **GlimpRouter (arXiv 2601.05110):** first-token entropy routing,
  training-free, +10.7% accuracy −25.9% latency. Our conf_route is this
  pattern (entropy threshold 0.8 on first-token logprobs).
- **S46 Hierarchical Waterfall (MS ISE):** routing-first, stage-gated eval —
  route decision evaluated separately from downstream stages. Our confusion
  matrix design follows this.
- **S47 RAGRouter-Bench (2602.00296):** cannot-answer label is a first-class
  routing outcome — our DECLINE/DECOY structure tests decline discipline,
  not just lane choice.
- **S48 MetaRoute-Bench (2608.00107):** acceptance criteria BEFORE testing;
  4-stage rollout replay→shadow→low-risk-live→randomized. Our spoof battery
  = replay stage; live v1 = low-risk-live.
- **S49 (2608.14641):** route stability + Pareto frontier — router value is
  accuracy-per-token, not accuracy alone.
- **E6 difficulty-budget routing (fusion, proven):** variant-level
  difficulty exists and is exploitable.
- **RCA boundary (ACM 2026):** structure must remove responsibility from
  small models — the router is deterministic code, the model never decides
  its own lane.

## Reasoning chain
1. Lane mis-assignment cost is asymmetric: H-on-L wastes tokens (~2.5×),
   L-on-H loses accuracy (cliff). So gate threshold should err HEAVY.
2. Entropy on first token of a cheap probe = GlimpRouter-cheap signal;
   GWT token gates are the deterministic fallback when logprobs absent.
3. Decline is a third outcome with its own failure mode (coverage loss).
   Decoys quantify over-declining; a gate that declines solvable cases
   is refuse-locked, worse than no gate.
4. Cues in v1 are surface-engineered (H/L notes) to make truth knowable
   deterministically; v2 replaces cues with real complexity signals
   (hops, digit count) once gate machinery is proven.

## What would change our mind
- If entropy signal can't separate lanes ≥70% even on cued cases, drop the
  probe path, keep GWT-only routing (keyword/structure gates).
- If L-lane accuracy cliffs on mid-difficulty cases, lanes need a third
  MIDDLE tier rather than threshold tuning.

## Relation to opencode parity
opencode's human decides effort implicitly per prompt. A deterministic
router matching that instinct at zero marginal cost is core to "semi-
efficient full replacement".
