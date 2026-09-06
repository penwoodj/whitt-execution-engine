# Top-Level Research — Why These Atoms, Why This Shape

## Scope decision (research-driven)
Phase-1 atom list (12 experiments) vs external literature gap analysis
produced TWO additions and TWO explicit folds:

- **13-synthesis ADDED** — multi-part prompts are the modal opencode
  request; ParaManager proves our exact fmt model can orchestrate when
  orchestration is scaffold-owned; pattern catalogs supply failure
  modes to design against (over-spawning, premature termination,
  blind-merge).
- **14-budget-inflation ADDED** — 2026 produced a complete budget-aware
  family (S54-S58 + RouteGoT scheduler); none of it needs training to
  implement as deterministic gates; opencode has NO budget surface, so
  this atom is a strict deployment advantage for the replacement.
- **edit-verify FOLDED into 10-execute-observe** — at spoof level the
  edit loop is identical machinery (fault injection on the mutation
  stage + observation classification); a separate experiment would
  duplicate infrastructure without a distinct hypothesis.
- **session-memory DEFERRED** — cross-prompt state is already carried
  by 03-cache-addressing (content-keyed artifacts) + 12-shape-library
  (composition reuse); a dedicated memory atom becomes interesting
  only after live data shows cross-session recurrence the cache misses.

## Source index for the top level (S50-S58, new this phase)
- S50 ParaManager 2604.17009 — 4B orchestrator, frozen summarizer,
  planning/solving decoupling
- S51 Parallel-Synthesis 2606.14672 — text merge ≈ cache merge (7/9)
- S52 Conductor 2512.04388 — (subtask, agent, access-list) triples
- S53 Orchestrator-worker catalogs 2026 — isolation, synthesis-as-
  reasoning, 1/6 multi-agent win rate, effort scaling
- S54 BAAR 2602.21227 — budget-aware per-step routing, fail-cheap
- S55 BAVT 2603.12634 — budget-conditioned selection, forced generation
- S56 TAB 2604.05164 — turn-adaptive budgets, 35-40% savings
- S57 InflationAgent 2608.13571 — inflation 4.25×, CBE signal,
  fresh-escalation −34.8pp
- S58 BAGEN 2606.00198 — over-optimism, early-stop 28-64% savings

## Top-level design reasoning

1. **Conf gate before anything expensive.** Three-way lane split
   (LIGHT/HEAVY/SYNTH) because the cost asymmetry is 4-6× and the
   multi-part axis is orthogonal to difficulty (S47 cannot-answer is
   the DECLINE case, folded into lane semantics for v1).
2. **route_to lane heads, not fall-through.** The engine supports it,
   unit tests verified GWT multi-target, but no E2E had exercised it.
   The top-level workflow is now that E2E (dry-run proof: 61 hops, no
   fall-throughs — the bug existed in v0 of the simulator and was
   caught exactly because the lane-skip pattern was observable).
3. **Routelog as the per-prompt contract.** Every prompt's final
   artifact is its route record {lane, route_ok, shape} — checkable
   deterministically, independent of content quality (content gates
   live inside the lane chains at extract stages). This mirrors the
   two-lane discipline: control-flow truth is det-checked, content
   truth is the lane's own json_exact.
4. **Spoof gates are the LLM.** dry-meta-run.py executes every hook
   script for real and simulates only the model call (which the spoof
   gate has already pre-written). This is the same win-index discipline
   as REA+/fusion batteries — the machinery is proven before any model
   is loaded.
5. **Scaffold ≠ win.** New invariant discovered while building this:
   unchecked scaffold stages must not write winning checks, or
   skip-if-won short-circuits the real answer stages (spoof-emit now
   writes passed=None for scaffolds). This is exactly the class of
   integration bug the dry runner exists to catch.

## Experimentation documentation (how to iterate)
Per 03-ORCHESTRATION loop, applied to the top level:
GENERATE (gen-meta-system.py) → UNITTEST (test_meta.py) → VALIDATE
(validate-workflow.py 9/9) → DRY-RUN (dry-meta-run.py) → INSPECT
(routelog checks, hops, lane distribution) → GATE (all-pass or fix).
Live phase replaces DRY-RUN with `whitt benchmark` on meta-v1-live.yml
after sub-experiment atoms graduate.

## Parity bar (unchanged)
Within 10pp of max-budget control at ≤40% tokens (00-SYSTEM-DESIGN).
The top-level adds a second axis: route correctness ≥85% (04-intent-
classify H1) and routelog completeness 100% (deterministic, must be
exactly 8/8 in dry run — currently true).
