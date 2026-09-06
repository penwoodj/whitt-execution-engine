# Testing Methodology Sources — S37-S49

Annotated sources for HOW to test atoms. Principles sources (S1-S36) live in
`experiments/principle-fusion/docs/web-research/FUSION-SOURCES.md`. This doc
covers evaluation-methodology findings and how each shapes a sub-experiment.

## S37 — Layer-Isolated Evaluation (arXiv 2606.11686)
Production agent decomposed into fixed layers; each exercised by its own
assertion slice in deterministic no-LLM "pure" mode. 238 cases / 23 slices,
~10ms/case, CI-locked baselines. **Regression injection:** degrading one layer
moves aggregate pass-rate only −1.7 to −5.9pp (dashboard noise) while the
matching slice craters −25 to −91pp; near-flat off-diagonal. **Coverage
honesty:** uncovered slice reports `rate: null`, never 1.0 — green aggregate
cannot launder unexercised layers.
**Adopted:** per-atom slice design of this whole suite; our spoof batteries are
"pure mode"; per-experiment pass/fail = per-slice gates. Coverage-honesty rule
applied to test_hooks (untested hook paths must fail tests, not silently pass).

## S38 — ToolMisuseBench (arXiv 2604.01508)
Offline deterministic benchmark for tool misuse + recovery under explicit step/
call/retry budgets. Fault taxonomy: schema drift, rate limit, timeout, auth,
adversarial error rewriting. Replayable seeded episodes; metrics: success,
invalid-call rate, policy violations, recovery quality, budgeted success at
caps {4,8,16,32} (AUC). Failure attribution decomposed: call construction vs
policy vs recovery.
**Adopted:** 10-execute-observe fault classes + budgeted-success metric; replayable
spoof scenarios are our seeded fault plans.

## S39 — ToolBench-X (arXiv 2606.25819)
Tool-use under 5 structured hazards: specification drift, invocation error,
execution failure, output drift, cross-source conflict. **Every injected instance
stays recoverable** (≥1 valid recovery path: retry, fallback, verification,
cross-check). All models <60% under hazards; failures driven by hazard diagnosis
+ recovery, not tool-call volume; recovery hints >> test-time scaling.
**Adopted:** recoverability constraint on every fault case (truth must remain
reachable); 08-replan hazard vocabulary.

## S40 — ChaosLLM (ISSRE 2025)
Fault injection between LLM and tools: unreachable, slow, hang, incorrect
response (delta-perturbator = subtle drift; garble = obvious). Slow faults had
NO effect (ReAct just waits); incorrect-response faults hardest. Metrics: TSR,
hallucination rate (SILENT-DIFF), timeout ratio.
**Adopted:** 10-execute-observe includes subtle-delta observations (plausible but
wrong tool output) — the silent-failure class our det checks must catch.

## S41 — Schema-First Tool APIs (arXiv 2603.13404)
Controlled study: free-form docs vs JSON Schema vs schema+structured-diagnostics
under budgets {3,5,8,12}. Schemas cut interface misuse, NOT semantic misuse;
recovery-conditioned analysis (success given ≥1 invalid call); diagnostic
granularity ablation (generic vs fields vs full hints).
**Adopted:** 01-typed-envelope three-condition design (prose contract / typed
schema / typed+error-class diagnostics) is a direct replication at stage-I/O level.

## S42 — Structural Testing of LLM Agents (arXiv 2601.18827)
Traces (OpenTelemetry) + mocking + assertions = test pyramid for agents: unit
(deterministic components), integration (mock the LLM, assert tool selection),
acceptance (E2E). Mocking enforces reproducible LLM behavior.
**Adopted:** our spoof-emit is exactly this mocking layer; ledger JSONL = traces;
test_hooks asserts hook behavior, spoof asserts integration.

## S43 — ToolFailBench (arXiv 2607.04686)
Failure labels: Tool-Skip, Result-Ignore, Output-Fabrication, Unnecessary-Tool-Use.
Tool-required tasks (answer unguessable without tool) + control tasks (tool
attached but should NOT be used). Best model only 86.33% clean tool-use.
**Adopted:** 05-gather and 10-execute-observe case design: tool-required facts
(unguessable, per SeedRG discipline) + no-tool control cases (atom must skip).

## S44 — Rigorous Agentic Benchmarks (arXiv 2507.02825, NeurIPS 2025)
Checklists: task validity (T.4 state cleanup, T.5 ground-truth isolation, T.6
frozen env, T.7-8 truth verification, T.9 oracle solver, T.10 outlier inspection
— consistent easy-task failure = impossible task; hard-only success = shortcut);
outcome validity (O.b no listing-all-answers, O.c judge validation, O.d test-case
quality, O.h no guess-success, O.i metric-process correlation). SWE-Lancer lesson:
agents overwrite protected tests → perfect score without solving.
**Adopted:** generator self-checks = T.7-9 (oracle = rule simulation); truth files
never referenced by hook inputs; answer spaces sized so guessing < 5%.

## S45 — SeedRG (arXiv 2605.08838)
Leakage-free RAG benchmark gen: type-constrained entity replacement + reasoning-
graph consistency check + closed-book leakage filter (N=3; any correct answer →
reject). Direct generation: 53-70% factual inconsistency; SeedRG: 0%.
**Adopted:** 05-gather corpora use invented entities (per-engine fictional
systems) — retrieval-dependent by construction; N8-disjoint digit discipline
carried over from fusion for same reason.

## S46 — Hierarchical Waterfall Evaluation (Microsoft ISE)
Stage-gated: routing accuracy FIRST; retrieval evaluated only for correctly
routed; generation last. Per-class diagnostics + partial-match breakdown;
evaluation data versioned like production code.
**Adopted:** 04-intent-classify is the gatekeeper experiment; its cases feed
05/06 case routing; per-variant (H/L) diagnostics everywhere.

## S47 — RAGRouter-Bench (arXiv 2602.00296)
Query-corpus compatibility routing; unified effectiveness-efficiency protocol
(quality metrics + token cost both reported); explicit "cannot answer" label so
router can decline instead of forcing a route.
**Adopted:** 04 includes DECLINE lane (neither LIGHT nor HEAVY — insufficient
info); every efficiency claim must report tokens (fingerprint ledger), not just
accuracy.

## S48 — MetaRoute-Bench (arXiv 2608.00107)
Meta-decision policies (direct/decompose/tool/code/delegate/verify/recover)
under shared seeded executor; policy isolation, paired seeds, trace completeness.
Compositional task-aware policy 79.4% vs static 76.7% vs direct 52.9%. 4-stage
production eval: offline replay → shadow → low-risk live → randomized.
**Acceptance criteria BEFORE testing** (success floor, p95 latency, cost-per-
success, unrecovered-failure ceiling); slice results by workload — "a global
average can hide a policy beneficial for research but harmful for documents."
**Adopted:** orchestration loop (03-ORCHESTRATION.md) uses replay→shadow→live
staging; every design.md pre-registers acceptance numbers; DECLINE/unverifiable
lane concept.

## S49 — Common-Interface Router Eval (arXiv 2608.14641)
Four routers × four benchmarks behind one interface; report Pareto frontier +
cross-benchmark rank variance, not a leaderboard number. Route-stability across
repeated trials as a metric.
**Adopted:** all routing experiments report quality+tokens Pareto; 04 measures
route stability (same lane across seeds) not just accuracy.

## How the sources map to experiments

| Experiment | Primary sources | Design element borrowed |
|---|---|---|
| 01 typed-envelope | S41, S37, S43 | 3-condition interface study; slice isolation; error taxonomy |
| 02 digest | S44 (O.i), fusion E10 | info-preservation oracle; token accounting |
| 03 cache-addressing | S48 paired seeds, S37 coverage-honesty | wrong-reuse detection; skip accounting |
| 04 intent-classify | S46, S47, S48, S49 | waterfall gating; DECLINE lane; stability metric |
| 05 gather | S45, S43, agentic-RAG ablation | leakage-free corpora; tool-required vs control; 2-iter depth |
| 06 plan | S48, PlanCompiler (S-ref fusion) | decompose-first ordering; plan-quality oracle |
| 07 solve | S1 (sample-more), S44 | control group; guess-resistance |
| 08 replan | S39, TDP, Leni | recoverable hazards; scoped-replan cost bound |
| 09 sample-diverse | S1 | cost-matched comparison design |
| 10 execute-observe | S38, S39, S40, S43 | fault taxonomy; budgets; delta observations |
| 11 verify-blind | Leni, Rulers/RubricForge (fusion) | independent observer; planted-error detection |
| 12 shape-library | Agent Primitives, Nexus, S48 | knowledge-pool reuse; quality-retention bound |
