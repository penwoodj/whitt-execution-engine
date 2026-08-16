# REA v1 — Experiment Plan

Status: scaffolding complete; live testing PENDING (machine busy, user directive).

## Objective

Prompt + non-thinking small-model draft in → big-model-quality answer out,
via bounded multi-lens cascade. Success = deterministic gates, not vibes.

## Hypotheses (GUIDELINES.md)

- H1 decomposition unlocks 1.7B/4B on tasks they fail end-to-end
- H2 factored sub-answers (no draft visibility) beat draft-conditioned ones
- H3 external deterministic gates mandatory (intrinsic self-correction fails)
- H4 ratchet + hard-fail gate never degrade below input
- H5 bounded escalation (memory + persona) recovers r1 failures ≤2 rounds

## Phases

| Phase | What | Gate |
|---|---|---|
| P0 | Scaffolding (this repo state) | validator + lint + dry-run 6/6 green — DONE |
| P1 | Smoke: 1 live case (rea-001) end-to-end on Docker | run completes ≤300s, select-best.json written, no Vulkan errors |
| P2 | Suite: all 6 cases, sequential, RAM-gated | 6 run dirs + metrics.json each |
| P3 | Analysis vs success gates G1-G6 (GUIDELINES.md) | table below filled |
| P4 | Iterate per ablation order (below), max 3 cycles | re-run affected cases |

## Success Gates

- G1 case lint clean (P0, holds)
- G2 workflow validates (P0, holds)
- G3 ≥5/6 finals pass all deterministic checks
- G4 ≤180s median, ≤300s cap per case
- G5 ratchet holds: no shipped final worse than draft (audit select-best.json candidates)
- G6 ≥1 case where enhanced passes checks the draft failed (mode selected_rN, input failed)

## Suite results (fill at P3)

| case | difficulty | mode | passed | rounds | seconds |
|---|---|---|---|---|---|
| rea-001 | easy | | | | |
| rea-002 | easy | | | | |
| rea-003 | medium | | | | |
| rea-004 | medium | | | | |
| rea-005 | hard | | | | |
| rea-006 | hard | | | | |

## Ablation order (P4, if gates fail)

1. Drop round 2 (persona/escalation) — tests H5 in isolation
2. Drop decomposer+solve (direct synthesize) — tests H1/H2 contribution
3. Drop verify-planner step (already observability-only in v1)

Each ablation: edit workflows/rea-v1.yml copy → validate-rea.py → rerun
affected cases → compare against P2 table. Revert if worse.

## Hard bounds (SAFETY.md)

≤6 LLM inferences, ≤500 tok out each, sequential only, 300s cap, RAM
watchdog, preflight, unload-all-first, no 9B, no network in steps.
