# 05-gather — Research & Reasoning

## Why this atom exists
Real agentic work means the facts aren't in the prompt — they're in files,
logs, docs. Every prior experiment handed the model everything inline.
This atom tests whether the gate-driven buffer pattern (engine owns
procedure, model does narrow extraction) survives when intake becomes a
first-class stage with noise and absence.

## Sources
- **Agentic-RAG ablation on Qwen 7B (arXiv 2606.21553):** 2 retrieval
  iterations capture 95% of 5-iteration gains; fixed hybrid beats adaptive
  routing (heuristic over-fires) → our 2-iteration cap + deterministic
  gate. H1 is this finding at case level.
- **ACL 2026 industry study (fusion S-set):** RAG+Tools 46% vs RAG-only 6%;
  random chunk selection 14% vs semantic routing 46% — intelligence in
  WHERE you look, not how many rounds.
- **Progressive disclosure (2607.17598):** one-level disclosure wins, deeper
  routing levels never help → gather2 is the second level; no gather3.
- **S39 ToolBench-X (2606.25819):** cross-source-conflict hazard class —
  our decoy blocks are benign conflicts (same shape, different numbers);
  contamination measurement = did the answer use decoy digits.
- **S43 ToolFailBench (2607.04686):** Output-Fabrication on missing-tool
  tasks is the small-model killer → H3 not_found honesty is the explicit
  countermeasure test ({not_found:[id]} contract makes honesty checkable).
- **S47 RAGRouter-Bench:** cannot-answer as first-class label — our
  NOT_FOUND truth is the same discipline.
- **P8 retrieve-don't-recall (fusion, proven):** evidence buffer = external
  memory; model reads, doesn't memorize.
- **DOS RAG (EMNLP 2025):** structure-preserving retrieval suffices —
  corpus blocks keep document order, no fancy ranking.

## Reasoning chain
1. Solve can't answer without events; events hidden in aux among decoys →
   gather must select by labeled section, not similarity (deterministic
   grep-ability).
2. Decoys are engineered to be one arithmetic-shift away (plausible but
   wrong) — contamination is measurable by digit forensics on failures.
3. Absence handling is a contract, not a vibe: truth = {not_found:[id]}
   makes "report missing" the only passing move when stripped.
4. Two iterations: first locates, second verifies the located block
   against the checklist. Depth 2 = verification read, not more search.

## What would change our mind
- If depth-2 wins are marginal but depth-1 already passes everything,
   corpus too easy → tighten decoy similarity, not the atom.
- If not_found honesty <70%, absence needs its own style prefix (gather
   style taught to emit not_found explicitly) — prompt fix, not structure.

## Relation to opencode parity
opencode reads files via tools constantly. Gather is the workflow-native
read/grep analog; proving it unlocks repo-scale tasks for generated
workflows.
