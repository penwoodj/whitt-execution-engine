# 00 — Scope

## In scope (this phase, zero LLM)

1. Deep web research consolidating every principle old + new with
   external anchors (`web-research/FUSION-SOURCES.md`, 36 sources).
2. Case suite: 12 cases, 1000-1150 words each, user prompt voice,
   computed truths, `needs:` principle metadata.
3. Workflow YAMLs: `fusion-v1-spoof.yml` + `fusion-v1-live.yml`
   (validator-PASS, engine-native, identical structure).
4. Unit tests on every script the workflow calls (zero model loads).
5. Live A/B + ablation PROCEDURE documented (`04-VALIDATION-PLAN.md`).

## Out of scope (hard gates)

- Any model load/unload/call (user directive 2026-08-24).
- Any `whitt` engine execution (spoof or live).
- Any schema change (`docs/schema/unified-workflow-schema.yml` frozen).
- Any modification to prior experiment folders.

## Naming

- Case IDs: `fu-01` .. `fu-12` (files `case-fu-NN.yml`).
- Workflow: `fusion-v1` (spoof/live toggle, same as REA+ convention).
- Scripts prefixed `fusion-` / `gen-fusion-` to avoid collisions.

## Success criteria for THIS phase

| # | Criterion | Evidence |
|---|---|---|
| Z1 | 12/12 truths self-verify vs check_lib | test_hooks.py green |
| Z2 | 12/12 prompts ≥1000 words | test_hooks.py green |
| Z3 | digest keeps every digit/trap token, strips expansion | unit test |
| Z4 | harvester echoes only check-passing JSON | unit test |
| Z5 | conf router thresholds correctly on fixture logprobs | unit test |
| Z6 | both YAMLs validator-PASS | validate-workflow.py output |
| Z7 | spoof scenario win-depths deterministic + exact | unit test |
| Z8 | every principle has stage+case+check mapping | docs/01 matrix complete |
| Z9 | zero LLM activity | no server calls anywhere in scripts |
