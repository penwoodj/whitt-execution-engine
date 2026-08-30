# 04 — Validation Plan

## Phase A — zero-LLM (DONE this session)

`python3 experiments/principle-fusion/scripts/test_hooks.py` — must be
green. Covers: truth self-verification (12/12), word counts (12/12),
digest slicing invariants, harvester check-before-echo, conf router,
spoof win-depth determinism, generator YAML structure (quoted tokens,
save_to shape, L-pruning, model map), leak-safety of hint blocks,
worked-example number disjointness, det-over-judge collector logic.
Plus `validate-workflow.py` on both YAMLs.

## Phase B — spoof engine battery (NEXT session, engine allowed)

1. `whitt benchmark --workflow workflows/fusion-v1-spoof.yml`
2. `collect-fusion.py` → summary.json
3. Assertions: 12/12 det pass; win-depth distribution matches
   scenario.json exactly; unchecked stages never produced PASS tokens;
   judge lane disagreement count matches scenario; ledgers complete.
4. Any failure = engine or YAML bug → fix, never tune scenario.

## Phase C — live-gate seeded run (1 real model, minimal)

fmt model only, live YAML, first H case. Verify: gate wiring, ans/check
artifacts, digest in fingerprint, harvester behavior on real 4B output.
Abort criteria: refusal text, format loops >2 stages, missing markers.

## Phase D — full live A/B (user-gated)

- Fusion live run (safe-launch.sh, 12 cases, stage-major).
- Baseline: 9B single-shot on same 12 cases (same digests).
- Compare: det pass rate, total tokens, wall time.
- Success: fusion ≥ baseline accuracy; predicted ablation failures.

## Phase E — ablation matrix (user-gated)

Variants (edit SHAPES in gen-fusion-workflow.py, regenerate):
no-plan · no-replan · no-wait · no-sample2 · no-verify · no-digest ·
no-harvester (direct-constrain solve). Prediction: for each removed
principle X, cases with X in `needs` fail; others unaffected (±1
noise). A principle whose removal changes nothing = candidate cut
(simplification win, documented either way).

## Evidence rules

- Every claim cites a ledger line or check file.
- Win-depth tables from collect-fusion.py only.
- No post-hoc scenario edits — regenerate + re-run.
- Track in results/ per-run subdirs; WORKFLOW_RELIABILITY_TRACKING.md
  entry per live iteration (repo protocol).
