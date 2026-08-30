# 06-plan — Research & Reasoning

## Why this atom exists
Plan-once is the most externally validated principle in the set (P4:
EdgeVox N+1→2 calls, ReWOO 5× tokens, LLMCompiler 3.7×/6.7×/+9%,
PlanCompiler 92.67% vs 62%). This experiment regression-tests it at
workflow level with dependency traps the literature doesn't include.

## Sources
- **PlanCompiler pattern (fusion S-set):** deterministic executor after
  LLM plan — "cannot get discouraged, cannot declare early victory".
  Our engine runs the DAG; plan only lists it.
- **ReWOO / LLMCompiler / EdgeVox (fusion S-set):** token/latency/accuracy
  gains all replicated across labs — the scaffold's baseline credibility.
- **Graph Harness (arXiv 2604.11378):** plan immutable per version;
  replan is escalation, not editing — our replan atom (08) handles the
  mutation case; plan itself is frozen.
- **TDP Task-Decoupled Planning (2026):** scoped sub-goal contexts,
  −82% tokens — dependency note keeps sub-task scoping explicit.
- **RandomCommits (fusion S-set):** 63%→87% with plan-invalidating
  re-plan gate — plan + conditional replan beat re-sampling.
- **RCA boundary (ACM 2026):** Plan-and-Execute CRUSHED small models
  (0.31→0.05) when the model executed its own plan. Our plan stage is
  UNCHECKED scaffold consumed by a deterministic dispatch; the model
  never loops on its own plan. H2's red-herring dep tests whether the
  plan scaffold injects misorderings (the RCA mechanism).
- **S44 (NeurIPS 2507.02825):** pre-registered criteria — H1's 15pp
  threshold declared before any run.

## Reasoning chain
1. Multi-dep tasks (expiry-before-request, breach-consumed, precedence
   chains) fail when solved monolithically — steps blur (our trap data).
2. Plan-once externalizes ordering to a scaffold the solve stage can
   re-read; the engine enforces nothing about order (solve is one call),
   so the lift measures *context shaping*, not control flow.
3. Red-herring dep separates real dependency extraction from keyword
   matching: a plan that honors the herring has pattern-matched, not
   parsed.
4. Verify stage (backward check) closes the loop: each rule tested
   against the answer (P10).

## What would change our mind
- If L-lane (direct solve) already ≥80% on H variants, cases are too
  easy for plan lift — raise engine idx (harder params), not scaffold.
- If red-herring violations concentrate in one engine, the herring is
  ambiguous there — rewrite that engine's herring, keep hypothesis.

## Relation to opencode parity
opencode plans implicitly (todo lists). Plan atom = workflow-native
externalized todo with deterministic dispatch — the P4 evidence says
this is where small models match big ones.
