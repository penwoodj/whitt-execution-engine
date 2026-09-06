# Correction Atom v7 — Full 27-Case Suite Results

**Date:** 2026-08-14 · **Workflow:** `workflows/correction-atom-v7.yml` · 27 live runs (15 old + 12 new)

## New cases 016-027 (12, realistic defect classes)

Truncation (016), hallucinated citations (017), unit errors (018), prose-when-bullets (019), scope creep (020), self-contradiction (021), YAML indentation (022), quote alteration (023), hedging/meta (024), number drift (025), table format (026), prompt echo leak (027).

All 12 validated before running: broken_output fails its own checks; hand-written reference fix passes all checks (incl. grounding + leak).

Verifier fix during validation: `check_degenerate` was table-hostile — markdown separator row counted as a 29-char "word" (long_token), and yes/no cell repetition as a repetition loop. Fixed: long_token counts alnum chars only; unique-ratio check skipped when ≥3 table lines present (026).

## Aggregate

| Suite | Pass | Avg wall | Leaks |
|---|---|---|---|
| Old 001-015 | 13/15 | 14.7s | 0 |
| New 016-027 | 10/12 | 11.1s | 0 |
| **Total** | **23/27** | **13.1s** | **0** |

Winning-angle distribution (23 passes): deterministic ×4, 1.7B ×9, 4B-req ×9, 4B-consistency ×1, **9B ×0**.

## The 4 failures — none are model-capability failures

| Case | Output content | Failed check | Root cause class |
|---|---|---|---|
| 006 | Correct Python (str/int hints + guard present) | word_count 12 < min 15 | Verifier miscalibration: min_words set for prose, applied to terse correct code |
| 010 | Correct guard pattern | word_count 9 < min 10 | Same |
| 021 | Semantically perfect ("After three retries, manually reviewed") | contains_required literal "3rd failed retry" | Verifier literalism: no number-word normalization (three ≡ 3rd) |
| 027 | Clean 3-bullet timeline, leak stripped | contains_required "14:15" | Case over-constrained: 3-bullet budget + 4 events + 2 mandatory timestamps forces merging — unguessable from spec |

Semantic scores of failing outputs: 85/85/100/100 — content-correct, checker-rejected.

## Key structural finding

**Qwen3-5-9B (angle 4, hallucination hunter) won 0 of 27 cases** — including case 017, whose primary defect IS hallucinated citations (won by 4B requirements tracer instead). The 9B stage costs ~20s load+infer each time it's reached and has never produced a selected output across 35 total v5+v6+v7 runs. Largest single v8 lever.

Per-run data: `benchmarks/comparison-v7-27cases.csv`.
