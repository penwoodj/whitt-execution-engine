# Score Gap Analysis — All 11 Prompts (2026-06-28)

## Summary

| Metric | Value |
|--------|-------|
| Prompts with deliverables | 9/11 (P06 + P15 missing) |
| Prompts PASS (≥40/50) | 9/11 |
| Perfect scores (50/50) | 2/11 (P05, P11) |
| Average score | 46.1/50 (excluding failures) |
| Most common gap | C7 execution evidence (6 prompts lose 5pts) |

## Per-Prompt Score Breakdown

| Prompt | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | Total | Verdict |
|--------|----|----|----|----|----|----|----|----|----|---------|
| P05 | 5 | 5 | 10 | 10 | 10 | 5 | 5 | 0 | **50** | ✅ PASS |
| P06 | — | — | — | — | — | — | — | — | **?** | ❌ NO .MD DELIV |
| P07 | 5 | 5 | 10 | 10 | 10 | 5 | **0** | 0 | **45** | ✅ PASS |
| P08 | — | — | — | — | — | — | — | — | **?** | ❌ NO WF |
| P09 | 5 | 5 | 10 | 10 | 10 | 5 | **0** | 0 | **45** | ✅ PASS |
| P10 | 5 | 5 | 10 | 10 | 10 | 5 | **0** | 0 | **45** | ✅ PASS |
| P11 | 5 | 5 | 10 | 10 | 10 | 5 | 5 | 0 | **50** | ✅ PASS |
| P12 | 5 | 5 | 10 | 10 | 10 | 5 | **0** | 0 | **45** | ✅ PASS |
| P13 | 5 | 5 | 10 | 10 | 10 | **0** | **0** | 0 | **40** | ✅ PASS |
| P14 | 5 | 5 | 10 | 10 | 10 | 5 | **0** | 0 | **45** | ✅ PASS |
| P15 | — | — | — | — | — | — | — | — | **0** | ❌ NO DELIV |

## Gap Analysis by Criterion

### C7: Execution Evidence (5pts each) — MOST COMMON GAP

**Affected:** P07, P09, P10, P12, P13, P14 (6 prompts × 5pts = 30pts total lost)

**Root cause:** Old pipeline runs (pre-cycle-4) stored benchmark.log in `exec/` or `exec-v2/`, but parity-check.sh looks for `exec-final/`. New runs (P05, P11) have `exec-final/` and score full C7 points.

**Fix:** Two options:
1. Re-execute all 6 prompts with current pipeline (produces exec-final/ dir)
2. Update parity-check.sh to search multiple exec dirs (`exec*/`, `logs/`)

**Impact if fixed:** Average score rises from 46.1 → 50.0 (all would score 50/50 except P13).

---

### C6: Meta-commentary (5pts) — QUALITY ISSUE

**Affected:** P13 (3 meta patterns detected)

**Root cause:** SW4 LLM emits meta-commentary in deliverable text (e.g., "Here is the implementation...", "This document describes..."). The synthesis step copies these patterns into the final deliverable.

**Fix:** Add to SW4 prompt: "Do NOT include meta-commentary, introductions, or self-referential text. Start with the actual content."

**Impact if fixed:** P13 rises from 40 → 45.

---

### C8: Fences Stripped (bonus) — NEVER EARNED

**Affected:** ALL prompts (0/0 bonus)

**Root cause:** Deliverables contain markdown code fences (```). C8 awards bonus points if fences are stripped from the final output. Currently no post-processing strips them.

**Note:** This is a BONUS criterion (max score still 50/50 without it). Not a real gap.

---

### Missing Deliverables: P06, P08, P15

**P06:** Has `report.html` (61404 bytes) but parity-check only looks for `.md` files. The deliverable IS substantive — just wrong format for the checker.

**P08:** From "min" pipeline (generate-minimal.py) which doesn't produce meta/ dir with workflow YAML. Needs full SW1-SW5 re-run.

**P15:** Complete failure — SW4 emitted wrong file paths (config.yaml, tools.rs at repo root instead of src/config/mod.rs, src/agent/tools.rs). Shell hooks failed → 16 steps skipped → no synthesis output.

## Root Cause Priority

1. **P15 SW4 file path accuracy** — CRITICAL: SW4 LLM doesn't know actual source file locations
2. **C7 exec evidence location** — HIGH: 30 points lost across 6 prompts
3. **C6 meta-commentary** — MEDIUM: 5 points lost on P13
4. **P06 format mismatch** — LOW: checker limitation, deliverable exists
5. **P08 missing pipeline** — LOW: needs re-run

## Recommended Actions

1. **Fix SW4 prompt** to include actual source file map (config.yaml → src/config/mod.rs, tools.rs → src/agent/tools.rs)
2. **Re-execute P07/P09/P10/P12/P13/P14** with current pipeline to produce exec-final/
3. **Add anti-meta-commentary rule** to SW4 prompt
4. **Update parity-check.sh** to search multiple deliverable formats (.md, .html, .pdf)
5. **Re-run P08** through full SW1-SW5 pipeline
