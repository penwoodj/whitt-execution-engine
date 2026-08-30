# 10-execute-observe — Research & Reasoning

## Why this atom exists
The engine-blocker finding: schema declares shell_exec/file_write/grep
but tools.rs never implemented them. Before implementing tools in Rust,
prove the LOOP (execute → observe → classify → mapped retry) with spoofed
faults — the loop design is testable without the tools existing.

## Sources
- **Self-healing orchestrator (arXiv 2606.01416):** failure-class-mapped
  recovery under budgets: 98.8% vs 94.5% retry-only, 93.8% full-replanning;
  silent failures → 0 with verifier guidance. Direct template; H2 is
  their class-vs-blind delta.
- **ChaosLLM (ISSRE 2025):** fault classes unreachable/slow/hang/
  incorrect; SLOW faults had NO effect (don't over-engineer for them);
  INCORRECT (our delta) hardest — our per-class table will replicate.
- **ToolBench-X (2606.25819):** 5 hazard types, all injected instances
  RECOVERABLE by design — faults must be survivable or you're testing
  abort logic, not recovery. FAULT_BODIES built accordingly.
- **ToolMisuseBench (2604.01508):** recovery budgets {4,8,16,32} — our
  v1 budget = 1 retry; v2 sweeps if recovery <60%.
- **ToolFailBench (2607.04686):** Result-Ignore and Output-Fabrication
  are the model-side failure modes — observe(cot) stage exists to force
  the model to READ the observation before solving (anti-Result-Ignore).
- **S40/S41 pair:** observation must be classified by DETERMINISTIC code
  (classify_observation), not model judgment — schema cuts interface
  misuse, not semantic; the class mapping table is scaffold-owned.
- **Coding-agent taxonomy (2604.03515):** read/search/edit/execute +
  generate-test-repair loop primitive — our chain is that loop in
  workflow form.

## Reasoning chain
1. Spoof the tool call: execute stage emits FAULT_BODIES observation via
   --fault class; deterministic, reproducible, zero-dependency.
2. observe stage (cot) reads observation aloud → forces grounding.
3. classify_observation maps text→class by regex (timeout phrases,
   unreachable markers, \ufffd garble, delta near-miss, schema keywords).
4. retry carries the class into its prompt (mapped recovery) — A-2
   removes it to isolate mapping value.
5. Clean 1/6 band measures false-positive looping (H3) — recovery
   machinery must not tax the happy path.

## What would change our mind
- If garble recovery ≈ delta recovery (easy≈hard classes collapse),
  class mapping is decoration — drop to single generic retry.
- If clean loops >15%, classify_observation over-fires; tighten regexes
  before any Rust implementation.

## Relation to opencode parity
opencode lives in this loop. The spoofed version de-risks the design so
the Rust tool implementation (later, user-gated) lands into a proven
loop shape, not a guess.
