# 03-cache-addressing — Experiment Design

## Atom
**content-addressed cache** — stage results keyed by hash(stage, input);
repeat/overlapping work skips recompute (generalization of skip-if-won).

## Hypotheses (pre-registered)
- **H1 (skip rate):** on paired cases (same shape, sibling params), the
  cache stage short-circuits ≥60% of sibling recompute where truth is
  shared; where sibling_truth_differs, cache correctly does NOT short-
  circuit (zero wrong-reuse).
- **H2 (correctness):** wrong-reuse rate = 0. Any single wrong-reuse
  (sibling truth applied to case with different truth) is a hard FAIL —
  the atom's entire value is trust.
- **H3 (latency):** cached pairs complete in ≤50% of stage time vs
  non-cached (ledger timestamps).

## Cases
ca-01..ca-72 = 36 pairs. Each pair = same engine, adjacent idx (shared
shape, disjoint digits). EXTRA_SCENARIO carries pair_id, member, and
sibling_truth_differs flag. Half the pairs share truth (same variant
params modulo idx shift → engine yields identical truth), half differ.

## Workflow (v1)
- Cache stage first (unchecked, emits cached/PASS/SKIP via spoof; in live
  emits content_key comparison) → H chain solve→extract→judge / L chain.
- SCENARIO %2: alternating member A/B win-depths exercise both paths.

## Pass/fail criteria
- PROVEN: skip ≥60% on share-truth pairs AND 0 wrong-reuse on differ pairs.
- REFUTED: any wrong-reuse (hash key too coarse — must add engine+variant+
  params to key), or skip <30% (key too fine).

## Ablations
- A-1: cache removed — expected: all pairs full-compute, latency ~2×.
- A-2: key = stage only (ignores input) — expected: wrong-reuse on differ
  pairs (this is the failure mode H2 guards; spoof scenario demonstrates
  the trap deterministically).

## Live variant plan
v1-live.yml ready. content_key(stage,input)=sha256[:16] in fas_lib.
Analysis: per-pair skip vs truth-differs matrix; ledger duration deltas.
