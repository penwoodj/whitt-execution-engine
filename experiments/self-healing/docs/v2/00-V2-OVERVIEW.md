# Self-Healing v2 — Overview

Status: DESIGN ONLY. No workflow execution, no LLM calls (user directive 2026-08-24).
Supersedes: sh-v1 (Phase S green, see ../06-TRACKING.md).

## What v2 adds over v1

| Area | v1 | v2 |
|------|----|----|
| Cases | 6 probe, single domain (cache-sweep JSON) | 100 cases, 10 domains × 5 archetypes × 2 complexity tiers, 1k–1.5k word prompts |
| Model specialization | declared, never exercised | domain→worker routing + per-failure-class model swap, script-driven |
| LLM call economy | n/a (zero-LLM spoof) | explicit call-budget governor; clean case = 1 LLM call, hard cap 6 |
| Conditional LLM steps | all steps spoofed | every LLM step behind GWT gate flags (`need_replan`, `need_judge`, `budget_ok`) |
| Heal generation | script templates only | F1/F2 = script templates (0 LLM); F3/F4 = LLM replan on heavy model |
| Loops | unrolled ×3 | unrolled ×3 core (proven) + `iterate_values` where sweep semantics needed (proven by 3/5/15/50-model benchmarks) |
| Judge | none (deterministic outcome) | deterministic fast-accept; Hermes judge LLM only in gray zone θ ≤ R < θ+ε |

## Design constraints honored

1. Engine-proven mechanics only: GWT routing on `bookmarks.shell_output.exit_code`, `skip_step` pre-inference, shell hooks w/ `working_dir`, unrolled chains, `iterate_values` expansion.
2. Schema-native keys only (`docs/schema/unified-workflow-schema.yml`); no new keys.
3. `LoopConfig` validation/count loops: schema-valid but ZERO live experiment evidence → NOT used in executable path; documented as DEFERRED.
4. Model safety rules (AGENTS.md): never 2+ models loaded; v2 workflows declare per-step `generative_entity` swaps, load/unload governed by `unload_unused: true` in Phase L. Phase S remains zero-LLM spoof.

## LLM call budget accounting (per case)

| Call site | Script or LLM | Count |
|-----------|--------------|-------|
| Worker attempt (specialized model) | LLM | ≤3 |
| Replan heal (F3/F4, heavy model) | LLM | ≤2 |
| Judge (gray zone only, Hermes) | LLM | ≤1 |
| Detect / triage / gates / route / budget / report / F1+F2 heals / outcome / end | SCRIPT | 0 |

Expected totals: clean=1, F1=2, F2=2, F3=3, F4=4 (w/ judge only in gray zone), persist-final_fail=6.

## Deliverable map

- `../01-CASE-MATRIX-100.md` — 100-case coverage design + full matrix
- `../02-WORKFLOW-V2-DESIGN.md` — step graph, gates, specialization map, loop strategy
- `../../cases/v2/flagship/case-*.yml` — 10 hand-written flagship cases (iteration set)
- `../../cases/v2/matrix/` — 90 composed cases (via `gen-cases-v2.py`, deterministic, no LLM)
- `../../scripts/gen-cases-v2.py`, `../../scripts/gen-v2.py` — composers
- `../../scripts/*_v2.py` (route/budget/gates/triage/detect/agent_run/judge/report) — v2 script kit (spoof path Phase S; real models Phase L)
- `../../workflows/generated-v2/sh-v2-<case>.yml` — per-case workflows

## Phase plan

- **Phase S2 (next session, user-gated run):** emit 90 matrix cases, validate all 100 workflows statically, run 10 flagship live zero-LLM, then full 100 sweep via driver script.
- **Phase L (user-gated):** real models, per 03-MODEL-SCOPE.md hardware rules.
