# 12-shape-library — Research & Reasoning

## Why this atom exists
The meta-workflow generator (SW1-5) pays full generation cost per
prompt. Shape-library = plan reuse: hash-match the query shape to a
proven composition, fall through to generation only on miss. This is
the atom that makes the generator semi-efficient — the "skip excessive
validation on simple prompts" requirement's structural version.

## Sources
- **Agent Primitives (arXiv 2602.03695):** Knowledge Pool (45 saved
  structures) guides an Organizer to compose rather than generate;
  +12-16.5% accuracy at 3-4× lower tokens vs text MAS — the pool IS
  the product.
- **Nexus plan library (tensegrity.blog):** saved query-shapes matched
  first, dynamic planning fallback; operators compose into DAGs; same
  composition answers new corpora — exactly our match→short-chain.
- **AFlow / MAS-GPT (via Agent Primitives):** workflow generation from
  retrieved structures beats from-scratch — retrieval beats generation
  in workflow space too.
- **S45 SeedRG (2605.08838):** leakage-free generation discipline —
  registry entries store shape ids + composition, never case truths;
  wrong-reuse traps verify the boundary holds.
- **S49 (2608.14641):** route stability metric — repeat-hit stability
  is the library's health check.
- **RouteGoT (2603.05818):** −79.1% tokens from routing — shape-match
  is routing taken to its cache extreme.
- **CCI (2605.05716):** best proper subset ≥ all-in — the LIBRARY is
  how subset-selection knowledge persists; entries encode which atoms
  each shape actually needs.

## Reasoning chain
1. Novel shapes need full scaffold (plan+verify) — generation path.
2. Repeat shapes already know their minimal chain — match stage emits
   shape id, L-lane runs; plan+verify skipped entirely.
3. Discrimination requirement: engine alone is too coarse (variant
   flips truths); registry key = engine:variant minimum, digit-hash
   safer. H3 traps enforce.
4. Graduation loop closes: each proven atom (01-11) registers its
   composition for its shape class — the library grows from experiment
   results, becoming the generator's fast path.

## What would change our mind
- If repeat accuracy <90% of novel even with correct discrimination,
  reuse carries risk — restrict library to L-lane shapes only.
- If novel-chain cost is already cheap (plan+verify <15% of total),
  library ROI too thin — measure first, then decide.

## Relation to opencode parity
opencode regenerates approach per session. Shape-library amortizes —
the compounding advantage that makes local replacement eventually
CHEAPER than flagship per-prompt, not just equal.
