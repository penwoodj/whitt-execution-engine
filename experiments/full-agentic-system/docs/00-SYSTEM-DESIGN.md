# System Design — From Atoms to opencode Replacement

Target: a workflow-native agent system on local models (llama.cpp/Vulkan,
fmt=4B-instruct, deep=27B, think=4B-thinking) that executes user prompts
agenticly with opencode-comparable reliability on supported task classes,
at a fraction of the cost. This doc is the composition blueprint the 12
experiments feed.

## Pipeline (target system)

```
user prompt
   │
   ▼
[04 intent-classify]  ── DECLINE ──► ask-user / not-supported
   │ LIGHT / HEAVY
   ▼
[12 shape-library]  ── hit ──► re-bind params ──► [assemble-validate] ──► execute
   │ miss
   ▼
[06 plan] ──► [01 typed-envelope contracts per node]
   │
   ▼ (per node)
[05 gather] (if evidence needed; 2-iter gate-driven)
   ▼
[02 digest] (compress intake; keep contract)
   ▼
[07 solve] (narrow CoT) ──► [extract/det-check]
   │ check-fail
   ▼
[08 replan] (scoped) ──retry──► solve
   │ stuck (2× fail)
   ▼
[09 sample-diverse] (cross-model)
   │
   ▼ (side lane)
[10 execute-observe] (shell/edit nodes; fault-classify + recover)
   │
   ▼
[11 verify-blind] (independent lane, non-gating) + det two-lane verdict
   │
   ▼
[03 cache-addressing] throughout (stage results keyed by input hash)
   │
   ▼
final answer + ledgers
```

## Composition rules (evidence-linked)

1. Engine owns procedure; model owns local inference (RCA resolution).
   Never emit a loop the model must "manage".
2. Every stage I/O = typed envelope (01) — schema + error-class — checked
   at the boundary (S41: cuts interface misuse; does not fix semantics).
3. LIGHT lane = solve+extract+judge only. Simple prompts must not pay the
   HEAVY tax (primary objective; CCI: subsets beat all-in).
4. HEAVY additions are individually gated ablations, not defaults (CCI).
5. Retry ladder: scoped replan before diverse resample before escalate
   (TDP; S39 recovery-path ordering; self-healing budgeted recovery).
6. Deterministic checks are ground truth; blind verify is advisory
   (two-lane; fusion N5).
7. Everything content-addressed (03): identical sub-solutions never
   re-inferred; traps (same prefix, different outcome) never skip.
8. Observability substrate: outcome + fingerprint ledgers on every stage
   (S42; familiar lesson: 80% of value is observability).

## What "replace opencode" means here (scope)

Supported classes (v1): deterministic multi-rule computation, evidence
gathering over local corpora, multi-dependency planning, tool-mediated
fact derivation with recovery, summarization-to-contract. Out of scope:
open-ended interactive dialogue, GUI control, novel codebase architecture
(these stay opencode/user).

Parity bar (from 01-EXPERIMENT-MATRIX): atom-composed pipeline within
10pp of max-budget control on pass rate, at ≤40% tokens. That is the
semi-efficiency contract: local models + structure ≈ flagship + budget,
cheaper.

## Mapping to meta-workflow generator

SW1-SW5 (deconstruct→state→categorize→translate→assemble) become:
- SW1/SW3 → 04-intent-classify + 12-shape-match (classification, reusable)
- SW2 → 01 typed contracts per node (desired state = output schema)
- SW4 → template re-bind from shape library (deterministic)
- SW5 → assemble-validate (already deterministic; keep)

Generator collapse: 5 LLM stages → 1 classifier call + 0 (on shape hit),
per Agent Primitives Knowledge Pool evidence (+12-16pp, 3-4× cheaper).
