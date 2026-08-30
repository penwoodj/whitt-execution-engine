# Full Agentic System — Atom Proof Suite

**Parent goal:** replace opencode (flagship-model agent) with locally-executed
workflow YAMLs for supported task classes. Prerequisite: a library of proven
**subworkflow atoms** — minimal, composable workflow units — each validated
against a hypothesis with deterministic, zero-LLM test infrastructure before
any live model run.

**This folder:** 14 sub-experiments (one per atom family) + a stitched
**top-level meta system** (v1 dry-run proven), each sub-experiment with:
- `docs/design.md` — hypothesis, engine rules, truth derivation, pass/fail criteria
- `docs/research.md` — testing-methodology research + reasoning (sources S37-S58 + FUSION-SOURCES S1-S36)
- `cases/` — 30-150 generated cases, 1k-3k words each, computed truths
- `fixtures/` — truth fixtures w/ derivation comments
- `workflows/` — v1 spoof workflow YAML (+ live variant)
- shared `../scripts/` — generic spoof/stage/check/generate infra driven by per-experiment `config.py`

**Top level:** `top-level/workflows/meta-v1-spoof.yml` (153 steps) routes
natural prompts (`prompts/meta-prompts.yml`, styled on real opencode
sessions) through a conf gate → LIGHT/HEAVY/SYNTH lane heads (GWT
route_to) → per-lane atom chains → checked routelog. The engine
simulator `scripts/dry-meta-run.py` executes it end-to-end with zero
LLM calls: **8/8 prompts route correctly, 8/8 routelog checks pass.**
See `docs/04-TOP-LEVEL-DESIGN.md` + `docs/05-TOP-LEVEL-RESEARCH.md`.

**Zero LLM policy (this phase):** no model calls, no loads/unloads. Cases generated
by Python engines, hooks spoofed via scenario scripts (win-index discipline),
workflows validated by `scripts/meta-v6/validate-workflow.py` dry runs, hook
scripts proven by unit tests, full-system control flow proven by the dry
meta-system run. Live runs are a later, user-gated phase.

## Atom → Experiment Coverage Map

| # | Folder | Atom(s) proven | Status in fusion | Hypothesis (short) |
|---|--------|----------------|------------------|--------------------|
| 01 | typed-envelope | typed-envelope | partial | Typed I/O contracts (schema + error-class) between stages cut FORMAT failures to ~0 without hurting content |
| 02 | digest | digest | proven (unit) | Digest (core+contract, worked/expansion cut) preserves solvability at ≤30% tokens across task types |
| 03 | cache-addressing | content-addressed cache | partial (skip-if-won) | Hash-addressed stage results skip ≥60% of recompute on repeat/overlap with zero wrong-reuse |
| 04 | intent-classify | lane-router + intent-classify | partial (N3) | Deterministic pre-gate (entropy/GWT) routes ≥85% of cases to correct LIGHT/HEAVY lane |
| 05 | gather | gather (read/search evidence) | unproven | Gate-driven 2-iteration evidence accumulation beats single-pass and 5-iteration on local models |
| 06 | plan | plan | proven (unit) | Plan-once scaffold raises multi-dep task success ≥15pp over direct solve |
| 07 | solve | solve (baseline control) | proven | Narrow-scope CoT solve is the right base unit; establishes control-group baselines |
| 08 | replan | replan + bounded-retry-escalate | unproven | Scoped replan on check-failure recovers ≥50% of near-miss cases at <40% token cost of full re-solve |
| 09 | sample-diverse | sample-diverse | proven (unit) | Cross-model resample converts ≥30% of same-model-stuck cases |
| 10 | execute-observe | execute-and-observe + edit | unproven | Shell/edit atom with observation→failure-class mapping recovers ≥60% of injected tool faults |
| 11 | verify-blind | verify-blind + det-check (two-lane) | proven (unit) | Independent-lane verification catches ≥50% of planted answer errors det checks miss |
| 12 | shape-library | shape-library + synthesis + assemble-validate | unproven | Hash-matched shape reuse produces ≥90% of from-scratch quality at ≤30% generation cost |
| 13 | synthesis | orchestrator-workers merge | unproven | Conflict-aware merge of isolated worker outputs beats direct solve on multi-part prompts |
| 14 | budget-inflation | budget-control | unproven | Pre-execution inflation probe + fresh escalation recovers ≥50% of stuck cases; thin budgets abort clean |

Atoms already unit-proven in `experiments/principle-fusion/` get workflow-level
regression here; unproven atoms (gather, replan, execute-observe, shape-library,
synthesis, budget) get first-ever proof infrastructure. Edit-verify is folded
into 10; session-memory deferred (covered by 03+12 composition).

## Directory Layout

```
full-agentic-system/
├── README.md                 ← you are here
├── prompts/meta-prompts.yml  # top-level natural prompts (opencode style)
├── top-level/                # stitched meta system: workflows, cases,
│                             #   fixtures, dry-run artifacts
├── docs/
│   ├── 00-SYSTEM-DESIGN.md   # target system architecture from atoms
│   ├── 01-EXPERIMENT-MATRIX.md # per-experiment hypotheses, criteria, ablations
│   ├── 02-METHODOLOGY-SOURCES.md # S37-S49 annotated testing-methodology sources
│   ├── 03-ORCHESTRATION.md   # iteration protocol: experiment-by-experiment loop
│   ├── 04-TOP-LEVEL-DESIGN.md # stitched control flow + dry-run evidence
│   └── 05-TOP-LEVEL-RESEARCH.md # scope decisions, S50-S58, design reasoning
├── scripts/                  # shared zero-LLM infra (generic across experiments)
│   ├── fas_lib.py / fas_engines.py / fas_cases.py
│   ├── spoof-emit.py / stage-emit.py / check-runner.py
│   ├── gen-cases.py / gen-workflow.py
│   ├── gen-meta-system.py / meta-conf.py / dry-meta-run.py
│   └── test_hooks.py / test_meta.py
├── cases-shared/             # cross-experiment corpus material (corpora for gather)
└── 01..14-*/                 # sub-experiments (see map above)
```

## Success Criteria (suite level)

Z1. Every atom family has a sub-experiment with computed-truth cases (no typed truths).
Z2. Every sub-experiment: 30-150 cases, each 1000-3000 words, self-verified at generation.
Z3. Every v1 spoof workflow passes `validate-workflow.py` 9/9 (28/28 across 14 experiments).
Z4. Every hook script path covered by `test_hooks.py`/`test_meta.py` — all green.
Z5. Spoof batteries demonstrate gate/routing/skip machinery end-to-end in
    workflow form: top-level dry run 8/8 routelog checks pass, lane routing
    correct for all prompts.
Z6. Per-experiment pass/fail criteria pre-registered in `design.md` BEFORE any live run.
Z7. No repo-root pollution; all artifacts under this folder.
Z8. Live-run readiness: each design.md names the exact live workflow variant + model
    plan (fmt/deep/think) so the user-gated live phase needs zero new design.

## Constraints Honored

- No LLM calls, no model load/unload (user directive 2026-08-24)
- Synchronous direct tools only (AGENTS.md)
- Workflow YAMLs under experiment folder (fusion convention)
- Schema discipline: v10-identical headers, only schema-valid step fields
