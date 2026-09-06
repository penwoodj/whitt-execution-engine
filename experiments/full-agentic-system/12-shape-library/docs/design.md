# 12-shape-library — Experiment Design

## Atom
**shape-library + synthesis + assemble-validate** — saved query-shape →
proven subworkflow composition, hash-matched before dynamic generation.

## Hypotheses (pre-registered)
- **H1 (reuse quality):** repeat-shape cases (shape seen before) produce
  pass rates ≥90% of novel-shape cases while skipping plan+verify stages
  — spoof design: novel = first occurrence (win min(n-1,3) full chain),
  repeat = short chain via match stage.
- **H2 (generation economy):** repeat cases consume ≤30% of novel-case
  token budget (match stage replaces plan+verify).
- **H3 (trap resistance):** shape-similar-but-different cases (same
  engine, different variant/idx → different truth) do NOT get the
  sibling's cached answer (wrong-reuse = 0; the match stage must
  discriminate variant, not just engine).

## Cases
sl-01..sl-36 = 6 engines × 2 variants × 3 idx. Shape = engine:variant.
Novel = first occurrence of shape; repeats thereafter. Same-engine
different-variant pairs are the H3 traps.

## Workflow (v1)
- H lane (novel): plan → solve → extract → verify → extract2 → judge
- L lane (repeat): match(unchecked, emits shape id) → solve → extract →
  judge
- SCENARIO: novel win = min(n-1, 3); repeat win = idx_in_shape % 2.
- UNCHECKED: match. CHECK_HOOK_STAGES: extract, extract2.

## Pass/fail criteria
- PROVEN: repeat pass ≥ 0.9 × novel pass AND repeat cost ≤0.3 × novel
  AND wrong-reuse = 0.
- REFUTED: repeat accuracy cliff (>10pp below novel) or any wrong-
  reuse (match key too coarse — add variant+digit-hash to shape id).

## Ablations
- A-1: match removed (all cases novel chain) — the economy baseline;
  H2's denominator.
- A-2: match on engine only (ignore variant) — expected: wrong-reuse
  on variant traps; proves discrimination requirement.

## Live variant plan
v1-live.yml ready. Shape registry = fixtures + scenario; deliverable:
reuse quality/economy curve + registry key spec for the meta-generator
(SW1-5's replacement for de-novo generation).
