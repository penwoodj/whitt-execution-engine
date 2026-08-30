# Principle-Fusion — Web Research Sources (2026-08-24)

Annotated bibliography behind this experiment. Every principle in
`docs/01-PRINCIPLES-MAP.md` cites at least one entry here plus our own
live evidence from the five prior experiment folders. Grouped by
principle cluster. "OURS" = finding proven live in this repo's
experiments (atomic-reasoning, correction-atom, harness-inspired,
reasoning-enhancer, reasoning-enhancer-plus).

## A. Sample more, reflect less (P2/P3)

**[S1] "Sample More, Reflect Less: Self-Refine and Reflexion Lose to
Repeated Sampling at Equal Token Cost, from 1.5B to 7B"**
arXiv 2607.28576 (Jul 2026). Designed experiment: 7 methods, Qwen2.5
at 1.5B/3B/7B, GSM8K + MATH-500, 150 paired questions per setting,
Holm-corrected bootstrap. 0/36 comparisons beat cost-matched repeated
sampling; all 18 self-inspection comparisons negative. Best-of-N
self-pick loses to majority-count on the SAME samples by 8.0-11.3pp at
1.5B (converges at 7B); rewriting methods never recover (-3.6 to
-10.1pp at 7B). Reflexion on Qwen2.5-1.5B judged itself correct on
100% of questions — retry never fired, method silently collapsed to
single CoT. Actionable: retries = fresh samples with hints, never
self-critique; external verification mandatory below 7B.
OURS: same-model "wait" retry won 0 cases live; cross-model sample
(Bonsai) fixed what 4B could not (v11 live A/B).

## B. Plan once, dispatch deterministically (P4) + harness > model

**[S2] "Plan Once, Then Act: When the ReAct Loop Is the Wrong Harness
for Small Local Models"** (EdgeVox blog, Jun 2026). Sub-7B Q4 models
degrade past ~6 hops; planned dispatcher = exactly 2 LLM calls vs N+1;
executor is a Python loop that "cannot get discouraged, cannot declare
early victory". Decision rule: small/quantized → plan-once.
**[S3] ReWOO** arXiv 2305.18323 — 5x token savings, planning role
distills to 7B. **[S4] LLMCompiler** arXiv 2312.04511 (ICML 2024) —
3.7x latency, 6.7x cost, +9% accuracy vs ReAct.
**[S5] Harness-Bench** arXiv 2605.27922 — 106 tasks x 6 harnesses x 8
models: 23.8-pt spread between best/worst harness on identical
model+tasks; weaker backends swing hardest with execution layer.
"Report capability at model-harness configuration level."
**[S6] PlanCompiler pattern** (tianpan.co, May 2026) — plan/execute
separation: 92.67% vs 62% success at 1/8 cost, same model; typed plan
registry removes per-step tool-selection errors.
OURS: plandispatch live 19/20 (only failure = variable confusion);
stage-major native v7 = driver parity at 67/73.

## C. Structure boundary: remove responsibility, never add it

**[S7] RCA reasoning-failure study** (ACM 2026, 48k scenarios): smaller
models DEGRADE with workflow complexity — Llama 3.2 location-accuracy
0.31 straight-shot → 0.23 ReAct → 0.05 Plan-and-Execute; below-random
rates rise with structure. Structure that loads procedural overhead
onto the model amplifies compounding missteps. Resolution against
[S2]-[S6] and OURS: our engine owns procedure (gates, routing, checks,
digest); model does only localized narrow-scope inference between
deterministic boundaries. This is the sharpest single design rule:
**structure must remove responsibility from small models, never add
it.**

## D. Reason free, constrain late (P7 upgraded)

**[S8] "The Constraint Tax"** arXiv 2605.26128 — sub-3B models: hard
schema decoding raised validity 61.5→100% but LOWERED answer accuracy
19.7→11.0%; wrong-but-valid-schema rose 49.5→88.9%. Calendar tool-call
analogue: prompt-only 91.5% executable vs hard schema 48.0%. Pattern:
"reason free, constrain late" — delayed deterministic packaging keeps
the semantic result.
**[S9] XGrammar-2** arXiv 2601.04426 — constrained Llama-3.2-3B beats
unconstrained Llama-3.1-70B on BFCL tool-calling; near-zero latency
via tag-dispatch grammars. Constraint belongs on final form, not on
reasoning.
**[S10] CRANE** (ICML 2025) — alternating unconstrained reasoning
windows with constrained output blocks: +10pp over pure constrained.
OURS: extract-harvester (solve unconstrained → deterministic extract →
check-before-echo) independently derives [S8]'s pattern from the
output side; v12.3 any-pass gate fix + harvest-before-verify made it
the universal winner (all technique wins landed in extract).

## E. Routing and cascades by node role / difficulty (P9, E3, E6)

**[S11] RouteGoT** arXiv 2603.05818 — node-adaptive routing inside
graph reasoning: strong models for planning+synthesis (globally
coupled), light models for leaf subtasks (localized): +8.1pp accuracy,
-79.1% tokens vs uniform.
**[S12] TRIM** (ICLR 2026) — step-level routing via process reward
models; only derail-likely steps escalate: 5x cost efficiency, matches
strong model with 80% fewer expensive tokens.
**[S13] STEER** (AAAI 2026) — routes on small-model logit confidence
(GMM-calibrated), no external router: up to +20% accuracy at 48% less
FLOPs. Model-internal confidence as routing signal.
**[S14] GlimpRouter** arXiv 2601.05110 — entropy of the FIRST token of
each reasoning step predicts step difficulty; training-free
probe-then-dispatch: +10.7% accuracy, -25.9% latency.
**[S15] LM Cascades** arXiv 2404.10136 — learned quantile deferral
beats naive sequence-uncertainty aggregation (length bias).
OURS: GWT gate-on-check routing = deterministic-check-driven cascade
(same family, orthogonal signal: we need only output text, works
through any server); variant-level difficulty (not engine-level)
discovered live in agentic-100 (cp02/residency flips).
**[S16] AdaptMI** arXiv 2505.00147 — skill-based in-context examples
HURT SLMs on easy questions (cognitive overload), help on hard ones;
adaptive selection (+up to 6%): apply scaffolds only where the model
is weak. Directly validates difficulty-pruned stage structure.

## F. Budget forcing, bounded (E1)

**[S17] s1: Simple test-time scaling** arXiv 2501.19393 — budget
forcing origin: append "Wait" to extend thinking, end-thinking token
to truncate; AIME24 50→57%.
**[S18] "Wait, Do We Need to Wait?"** (ICLR blog 2026) — bounded
replication: gains are SFT-reasoning-model-specific; often NEGATIVE on
distillation-based models; "Wait" never the best keyword (best is
model-specific); linear-scaling claim fails; instruct models need
explicit think-tag structure. Use sparingly, late, near-miss only.
**[S19] Learned continue-thinking token** (IJCNLP 2025) — RL-trained
single token beats fixed "Wait" (+4.2% vs +1.3% on GSM8K) where budget
forcing helps at all. Future training hook, out of engine scope.
OURS: plandispatch wait-retry won 0/20 live — cut; kept only as final
near-miss escalation in fusion shape.

## G. Retrieve, don't recall; context discipline (P8, E10)

**[S20] "Lost in the Middle"** TACL 2024 — U-shaped position curve;
mid-context info retrieved worse than closed-book.
**[S21] Retrieval meets Long Context** (ICLR 2024) — 4K+retrieval ≈
16K fine-tuned window at 43B/70B; retrieval gains shrink at 7B
zero-shot (small models incorporate retrieved chunks poorly without
scaffolding).
**[S22] ReadAgent** (ICML 2024) — gist memory + episodic lookup:
3.5-20x effective context.
**[S23] Agentic context strategies** (ACL 2026 industry) — tools
transformative: RAG+Tools 46% vs RAG-only 6% (7.7x); GPT-4.1 200K
context = no advantage over smaller context + tools; random routing
14% vs semantic 46%.
**[S24] Progressive disclosure study** arXiv 2607.17598 — one-level
disclosure wins at scale; deeper routing levels never help. "Buys
context, not intelligence."
**[S25] DOS RAG** (EMNLP 2025) — simple structure-preserving
retrieve-then-read matches ReadAgent/RAPTOR under matched budgets.
Source fidelity beats pipeline cleverness.
OURS: chunked-intake 30/30 (9B 0/30) via gate-driven verbatim evidence
accumulation; v14 digest offload (models see ~160-word core regardless
of 1-2k-word case length) fixed context dilution 15/100 → 100/100.

## H. Worked examples and prompt-form stability

**[S26] Algorithmic Prompting** arXiv 2211.09066 — worked examples
with unambiguous step detail: ~10x error reduction (long parity), 9x
(addition), 5x (multiplication) vs best baselines; errors in the
worked examples propagate — they must be correct and DIFFERENT-number
(different-number prevents copy-confusion, per our agentic-26
technique).
**[S27] Arithmetic ICL interpretability** (EMNLP 2025) — FORMAT
CONSISTENCY of in-context examples matters more than their arithmetic
correctness (breaking format "8"→"eight" hurts more than wrong-but-
consistent numbers). Mechanistic grounding for OUR prefix-stability
findings: v14c "no reasoning aloud" prefix flipped values at temp 0;
exact-token answer menus fixed hb-10/11/12.
**[S28] In-context Curriculum Random** (openreview 2023) — simpler CoT
demonstrations beat complex ones for 250M-11B models.
**[S29] LMS3** (PMLR 2025) — demonstration selection + rejection:
efficacy bounded by semantic similarity + inference stability;
demonstrations can be net-negative. Select, don't pile.

## I. Two-lane verdict: deterministic floor, judge ceiling

**[S30] "Deterministic vs LLM-Judge Evals"** (futureagi, Feb 2026) —
det checks catch 30-60% of failures at $0 sub-ms; judge handles
semantic remainder; production pattern = cascade (heuristic first,
judge on ambiguous remainder). Reproducibility: det byte-perfect, judge
drifts with model versions.
**[S31] Preference Leakage** arXiv 2502.01534 — judges biased toward
related student models (same family/inheritance); harder to detect
than egocentric bias; objective questions leak least, subjective most.
Cross-family blind judges required.
**[S32] Rulers** arXiv 2601.08654 — rubrics compiled to immutable
executable specs + evidence-anchored scoring with mechanical evidence
gates (score capped without verifiable quotes): small models rival
large proprietary judges.
**[S33] RubricForge** arXiv 2608.13564 — induced rubrics halve
false-pass rate (0.115 vs 0.173) on tau-bench; "false pass ships a
broken agent, false fail merely costs a retry" — the deployment-
relevant metric for any proxy evaluator.
**[S34] RuVerBench / meta-evaluate judges** arXiv 2606.29920 — judge
reliability spans 51.6-94.7 balanced accuracy by domain/model/prompt;
for mechanically checkable rubrics a deterministic checker beats a
judge outright.
OURS: two-lane verdict (det final overrides LLM judge) proven live:
ha-08, hb-14, hb-18 judge=fail but det=pass → correct PASS; cross-
family judge agreement 9/10.

## J. Iteration methodology (ours alone)

**[S35] OURS — spoof-first verification.** Zero-LLM structural proof
before any live campaign: engine-executed spoof runs, scenario-driven
win-depths, 24/24 script test suites, ledger-driven collection. Catches
authoring bugs (quad-brace leak, ANY-PASS gate, collector rank order)
before GPU time is spent. No external equivalent found in the surveyed
literature; matches the reproducibility gaps others self-report (seed
variance missing in [S11]-[S14] evaluations).

## Principle-to-source index

| Principle | Sources | Our live anchor |
|---|---|---|
| P1 CoT first | S28, S29 | proven prefixes, v14c revert |
| P2 Sample more reflect less | S1 | wait-retry 0 wins, sample2 wins |
| P3 External verification | S1, S34 | spoof battery, det checks |
| P4 Plan once | S2-S6 | plandispatch 19/20 |
| P5 Decompose separable only | S26, S16 | decompose pass 0/8 removed |
| P6 ADaPT try-first | S16 | rescue chains on fail only |
| P7 Constrain late | S8-S10 | extract-harvester |
| P8 Retrieve don't recall | S20-S25 | chunked-intake 30/30 |
| P9 Route by role | S11-S15 | stage-major routing |
| P10 Cheap verify run twice | S1, S13 | extract cheapest lever |
| E1 Budget forcing bounded | S17-S19 | wait kept last-resort |
| E3/E6 Difficulty gating | S12, S16 | variant-level H/L |
| E9 Verify last | S4 | harvest-before-verify |
| E10 Context hygiene | S20, S24, S25 | digest offload |
| N1 Structure removes responsibility | S7 | engine owns procedure |
| N2 Reason free constrain late | S8 | harvester echo-check |
| N3 Confidence pre-gate | S13, S14 | conf hook (live phase) |
| N4 Replan on invalidation | S36 | RandomCommits 63→87% |
| N5 Two-lane verdict | S30-S34 | det overrides judge |
| N6 Leak-safety | S31, S33 | id+observed+question |
| N7 Format-consistent scaffolds | S27 | answer menus, quoted tokens |
| N8 Worked examples diff numbers | S26 | agentic-26 technique |

**[S36] RandomCommits agent-loop study** (randomcommits.com, Mar 2026)
— plan + invalidation-detecting re-plan gate: 63% → 87% vs linear
plan-execution; "cheap plans, frequent cheap verification, occasional
re-planning". Used for the replan stage design (N4).
