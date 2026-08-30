# REA+ — Reasoning Enhancer Plus: Scope

**Status:** NORTH STAR doc. Supersedes ad-hoc iteration. Created 2026-08-16.
**Home:** `experiments/reasoning-enhancer-plus/`

## One-line objective

Prove (or disprove) that a specialist cascade of five 4B–11B models,
driven by the REA verifier-scaffold, passes 30 realistic benchmark-grade
reasoning cases that the previous best small-model workflow
(REA v1.4.3, 1.7B+4B) demonstrably fails — with multi-dimensional
quality metrics, one metric targeted per iteration, TDD discipline.

## Why (gap analysis from REA v1.4)

REA v1.4 proved: scaffold+verifier beats bare model on FORMAT-exact
deliverables (50/50). Critique found gaps:

1. Discriminating power was 68% formatting (32/58 subchecks), not reasoning
2. Struggle-suite cognition probes (s01–s30) authored but NEVER run
3. No ablations (retry-only? rubric-only?), n=1 everywhere
4. Ground truth self-calibrated (b39 total_pages bug: 33 vs correct 34)
5. Zero tool-use / multi-turn / environment-feedback axes
6. Answers often spelled out in auxiliary material

REA+ closes 1,3,4,6 by design. Axes 5 (tools) stay OUT of scope (engine
benchmark path has no tool loop yet) — documented as non-goal, not silently
skipped.

## Two stages, hard gate between

```
STAGE 1: MODEL SELECTION (prerequisite, quick)
  pool: 10 installed models, 4B–11B (see 03-MODEL-SELECTION.md)
  5 categories: FMT, DRV, LOG, PLN, AUD
  20-item probe battery, 1 load/model, ~6 min/model
  OUTPUT: 5 picked specialist models -> results/MODEL-PICKS.md

  ═══════ HARD GATE ═══════
  NO stage-2 case design until MODEL-PICKS.md exists.
  Reasons: case difficulty must sit in the gap between
  "REA v1.4.3 small cascade fails" and "picked big specialists pass"
  — unknowable until picks are live-proven.
  ═════════════════════════

STAGE 2: 30-CASE SUITE + ITERATION
  30 cases, external ground truth (public-bench archetypes)
  dual validation: REA v1.4.3 must FAIL all 30; picked big models
  (direct or light scaffold) must PASS — else case re-authored
  iterate workflow versions vP0..vP8, ONE metric at a time (05-TDD-ROADMAP)
```

## Non-goals

- Tool-use/multi-turn agentic loops (engine limitation, tracked upstream)
- Beating 9B+ class frontier models — target is the documented gap band
- New Rust engine code (unless an engine bug blocks a gate — then minimal
  fix via TDD per AGENTS.md)

## Exit criteria (all must hold)

1. MODEL-PICKS.md: 5 specialists, live-probed, ≥3 probe items/category pass
2. 30 cases: lint-clean, externally-derived truth, dual-validated
   (old-workflow fail + big-model pass), 24 train / 6 held-out split
3. vP-series iterations each target ONE metric with red→green evidence
4. Final suite: ≥24/30 train pass, held-out reported separately, n=3 seeds
5. Ablation row exists: retry-only and rubric-only vs full scaffold
6. 07-TRACKING.md complete: every run has YAML+log+artifact refs
7. Zero ratchet violations / hard-fail ships (carried from REA v1.4)

## Honesty rules (hard)

- Ground truth derived independently, never from model output
- Calibrating a check to make output pass = BUG, logged as such
- n=1 claims labeled n=1; variance reported when n≥3
- Walls from watchdog-killed runs never reported as clean timings
