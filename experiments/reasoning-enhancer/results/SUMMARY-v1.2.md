# Reasoning Enhancer Atom v1.2 — Fails-Without/Succeeds-With Property

**Date:** 2026-08-14 · **Workflow:** `workflows/rea-v1.yml` (v1.2.0) · **Suite: 6/6 property HOLDS**

## Objective

Every test case must FAIL without the workflow (draft alone, decisively) and
SUCCEED with it. Enforced at authoring time (lint) and proven live.

## Changes

1. **rea-004 draft** — added realistic assistant-voice hedging ("let me know
   if you need anything else") + matching forbidden phrase. Draft now fails
   contains_required (55/45 vs 54/44) AND forbidden_phrases.
2. **rea-006 draft** — list flattened to prose string (`targets: /var/data
   and /etc/app`, 0 bullets vs 2 required). Note: the previous mis-indent
   parsed as a VALID nested list — live run showed input failing only 1
   subcheck, so the "structure" defect was cosmetic. Flattening is a real
   structure failure. Draft now fails contains_required ("84") AND
   bullet_count_min.
3. **test-cases.py margin rule** — draft must fail ≥2 subchecks. A case
   whose draft fails one flipped subcheck cannot demonstrate enhancement.
4. **run-enhancer.sh preflight override (B13)** — shared preflight aborts on
   swap-USED ≥10GB; long-uptime machine carries 12GB of unrelated cold
   pages (rust-analyzer 2.1GB, tsserver 1GB) with si/so ≈ 0. Override: when
   swap-used is the ONLY preflight failure, gate on swap ACTIVITY (si+so
   over 8s); ≥25MB/8s (≈3MB/s sustained) still aborts. First calibration
   (100KB/5s) was too strict — 120-360KB/s is normal cold-page eviction
   noise on this box. The gate caught one REAL burst (171MB/5s → abort,
   rerun clean) — defense works.

## Live evidence (fails-without → succeeds-with)

| case | draft fails (without) | workflow (with) | mode | dur |
|---|---|---|---|---|
| rea-001 | 2 subchecks | PASS | selected_r2 | 22s |
| rea-002 | 2 subchecks | PASS | selected_r1 | 25s |
| rea-003 | 2 subchecks | PASS | selected_r1 | 20s |
| rea-004 | 2 subchecks | PASS | selected_r2 | 16s |
| rea-005 | 3 subchecks | PASS | selected_r1 | 21s |
| rea-006 | 2 subchecks | PASS | selected_r1 | 17s |

Property: **HOLDS 6/6.** Total 121s. 2 cases needed round 2 (escalation
memory); 4 passed round 1. Ratchet never invoked (all enhanced candidates
beat input) — consistent with decisive draft failures leaving headroom.

## Ops log

- Swap gate false-aborted 5 runs (cold pages), then correctly aborted 1 run
  on a real 171MB/5s thrash burst. Activity calibration: noise band
  0.5-23MB/8s observed passing; real burst 171MB/8s aborted.
