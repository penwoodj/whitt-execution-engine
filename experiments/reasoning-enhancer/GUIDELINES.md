# Reasoning Enhancer Atom (REA) — Guidelines

**Status:** Scaffolding complete, live testing pending
**Created:** 2026-08-14
**Ancestry:** atomic-reasoning (north-star principles) → correction-atom v8 (operational patterns) → **REA v1**

## Objective

Smallest workflow that takes a prompt **plus the response from a non-thinking small
model** and, through iterative multi-lens reasoning with external verification,
returns a response at the quality level of a much larger model — in reasonable
time (target ≤3 min/case) on local hardware (RX 580 8GB, Qwen3 1.7B + 4B only).

The input response is *genuinely weak reasoning* (shallow logic, missed steps,
unverified claims) — not the seeded surface defects of correction-atom. The atom
must therefore **generate new reasoning**, not merely patch old output.

## Hypotheses (falsifiable, ordered by evidence strength)

- **H1 — Decomposition unlocks small models.** Sub-question decomposition +
  factored sequential sub-answers beats all-at-once answering below 10B.
  (DialCoT, Least-to-Most — see RESEARCH.md §11.)
- **H2 — Factored verification beats self-review.** Verification questions
  answered WITHOUT the draft visible avoid inheriting the draft's errors.
  (CoVe factored variant — RESEARCH.md §3.)
- **H3 — External verification is mandatory.** Intrinsic self-correction fails
  ≤13B; deterministic checks + tool-verify must gate every stage.
  (Huang ICLR 2024, T1 — RESEARCH.md §4, §5.)
- **H4 — Ratchet + hard-fail gate prevents harm.** Never ship output worse than
  the input draft; selection over {draft, synth, round-2} by checks-passed.
  (correction-atom v8 learning 6.)
- **H5 — Bounded escalation beats unbounded iteration.** Max 2 refine cycles,
  escalation memory carries open failures forward, diverse lenses (not diverse
  sampling alone) supply variation. (Snell 2408.03314, diversity-of-thought
  2410.12853, correction-atom learnings 2/4.)

## Core principles (inherited + extended)

1. **Deterministic-first.** Zero-LLM pass first; early exit if input already
   passes all checks. Deterministic checks carry the load; models fill gaps.
2. **One cognitive mode per pass.** DECOMPOSE / SOLVE / PLAN-VERIFY / SYNTHESIZE
   / JUDGE are separate steps with separate prompts, temperatures, and context
   windows. Model never sees too much at once.
3. **Factored contexts.** Sub-solvers do NOT see the draft. The synthesizer
   sees everything. Judge sees only spec + candidate + reference materials.
4. **External gates between stages.** Every transition passes a deterministic
   check; first failure routes to escalation memory, not silent retry.
5. **Ratchet selection.** Ship best-of {input, round-1, round-2} by
   subchecks-passed; hard-fail classes (leak / degenerate / forbidden /
   unparsable) block shipping entirely.
6. **Small models, narrow scopes.** 1.7B for cheap structural work
   (decomposition), 4B for synthesis/judging. No 9B (v8 lesson: it won 0/62
   attempts and burned ~20s each).
7. **Bounded everything.** ≤8 inferences/case, ≤500 output tokens/pass,
   ≤2048 context/pass, ≤300s watchdog, max 2 refine cycles, sequential only.

## Anti-patterns (evidence-backed — DO NOT)

- Free-form "review and improve your answer" prompts (Huang 2024 — drops accuracy).
- Majority voting on hard cases without external verifier (2608.11403 — hurts
  56-66% of hard problems).
- Long rubric walls in prompts — short focused prompts with hard output rules
  LAST win on 1.7B/4B (correction-atom learning 5).
- Unbounded or >3 refinement rounds (Kaesberg 2025 — performance decays).
- Showing the draft to sub-solvers (CoVe — repeats hallucinations).
- Adding a bigger model to fix quality (v8 lesson 3 — biggest model won nothing).
- Parallel inference on this hardware (crash vector — SAFETY.md R1).

## Success gates (live phase)

| Gate | Criterion |
|------|-----------|
| G1 | Case lint: all cases' draft responses FAIL their own checks; reference fixes PASS |
| G2 | Workflow parses + validates (engine `WorkflowFile::validate()` via live run) |
| G3 | ≥5/6 cases end in a `final_answer` written, no hangs, no leaks |
| G4 | Median wall time ≤180s/case, hard cap 300s watchdog |
| G5 | Ratchet holds: zero cases where shipped output is worse than input draft |
| G6 | At least one case where enhanced output passes checks the draft failed |

Exit criteria for the experiment: G1–G6 across two consecutive full runs, plus a
written comparison vs. single-shot 4B baseline (same prompt, no workflow).

## Iteration protocol

Live iteration follows correction-atom's loop: run case → read logs + check JSON
→ classify failure (verifier bug / prompt bug / case bug / model limit) → fix
ONE thing → re-run failing case only → full run when singles pass. Every
iteration logged in RESULTS.md (created at first live run) with the 8-point
evidence checklist (see RUNBOOK.md).

## File map

```
experiments/reasoning-enhancer/
├── GUIDELINES.md        (this file — north star)
├── RESEARCH.md          (web research synthesis + citations)
├── DESIGN.md            (v1 architecture: steps, lenses, gates, budgets)
├── SAFETY.md            (hard rules — delta vs atomic-reasoning SAFETY.md)
├── PLAN.md              (live-phase plan: baselines, run matrix, metrics)
├── REVIEW-CYCLES.md     (scaffolding review log — 3 scopes, findings+fixes)
├── RUNBOOK.md           (exact commands when machine frees)
├── cases/               (case-001..006.yml — 2 easy / 2 medium / 2 hard)
├── fixtures/            (reference-fixes.yml for case lint)
├── workflows/rea-v1.yml (the atom; placeholder-substituted at runtime)
└── scripts/             (emit/check/judge/select/lint/dry-run/validate/run)
```
