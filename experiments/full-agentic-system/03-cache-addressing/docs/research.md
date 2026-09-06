# 03-cache-addressing — Research & Reasoning

## Why this atom exists
Agentic sessions repeat work: same subtask shape, same retrieval, same
sub-solution. opencode re-runs everything per turn; a local system with
30s model-load tariffs cannot afford that. Skip-if-won (has_pass) proved
the special case; content addressing generalizes it to overlapping inputs.

## Sources
- **Node-primitive blog (asteromorph, 2026-05):** content-addressed cache
  keyed by hash(config+input) — Nix/Bazel pattern applied to LLM nodes;
  resume + iteration without re-paying completed stages. Direct template.
- **Agent Primitives (arXiv 2602.03695):** Knowledge Pool of 45 prior
  structures — reuse beats regeneration; 3-4× token/latency reduction vs
  text-based MAS.
- **Nexus plan library (tensegrity.blog):** saved query-shapes matched
  before dynamic planning — cache at the plan level; operators reused
  across corpora.
- **S48 MetaRoute-Bench (2608.00107):** paired-seed design, policy
  isolation — our pair structure (same shape, adjacent idx) is the case-
  level analog; acceptance criteria pre-registered before testing.
- **S49 Common-Interface Router Eval (2608.14641):** route stability
  metric — cache hits are the stability extreme; jitter = miss.
- **skip-if-won (fusion/REA+, proven):** mtime-ordered check glob any-pass
  → short-circuit. Works; but keyed on case-id only, not content.

## Reasoning chain
1. Wrong-reuse is catastrophic (silently wrong answer), misses are merely
   costly → key must err fine, verify err hard.
2. Key = hash(stage, full input text): input includes every task digit →
   sibling pairs with different truths hash differently by construction.
3. Share-truth pairs differ only in decoration (padding, worked numbers)
   → need normalization in the key input (engine name + variant + core
   digits, not raw prompt) to reach ≥60% hit. v1 uses raw input; if hit
   rate low, v2 keys on digest(prompt) instead.
4. H2's zero-tolerance is testable deterministically in spoof: differ-pair
   short-circuits are counted as FAIL_ ledger events.

## What would change our mind
- If share-truth hit rate <30% even with digest-keyed inputs, caching at
  case level isn't where reuse lives → move atom up to shape level (12).

## Relation to opencode parity
opencode has no cross-session cache; beating it on repeat workloads is a
structural advantage local systems can own outright.
