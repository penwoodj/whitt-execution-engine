# Correction Atom Experiment — Final Summary

**Branch:** `opencode/nimble-harbor`
**Date:** 2026-08-13
**Workflow:** `experiments/correction-atom/workflows/correction-atom-v3.yml`
**Model:** Ministral-3-3B-Instruct-2512-Q4_K_M (CPU only)

## Result: 10/10 PASS

| Case | Issue Type           | Wall (s) | Best Angle | Score | Pass |
| ---- | -------------------- | -------- | ---------- | ----- | ---- |
| 001  | format_drift         | 49       | 1          | 100   | ✓    |
| 002  | format_drift         | 39       | 1          | 100   | ✓    |
| 003  | factual_error        | 33       | 1          | 100   | ✓    |
| 004  | factual_error        | 28       | 1          | 100   | ✓    |
| 005  | missed_requirement   | 56       | 1          | 100   | ✓    |
| 006  | missed_requirement   | 44       | 1          | 100   | ✓    |
| 007  | hallucinated_content | 20       | 1          | 90    | ✓    |
| 008  | hallucinated_content | 28       | 1          | 100   | ✓    |
| 009  | refine_ignored_fix   | 58       | 1          | 90    | ✓    |
| 010  | refine_ignored_fix   | 32       | 1          | 100   | ✓    |

**Aggregate:** 10/10 pass (100%), avg wall 38.7s, total wall 387s, avg score 98.

## Architecture

```
Case YAML (task_spec + broken_output + auxiliary + checks)
    │
    ▼
Bootstrap (shell emits context block)
    │
    ▼
Angle 1: FORMAT AUDITOR (persona + lens + recognize + think + fix)
    │ deterministic check
    ▼
Angle 2: FACT CHECKER (sees Angle 1 output)
    │ deterministic check
    ▼
Angle 3: REQUIREMENTS TRACER (sees Angle 2 output)
    │ deterministic check
    ▼
Angle 4: HALLUCINATION HUNTER (sees Angle 3 output)
    │ deterministic check
    ▼
Angle 5: CONSISTENCY AUDITOR (sees Angle 4 output)
    │ deterministic check
    ▼
Analyzer (scripts/analyze.py)
    │ best-angle selection: FIRST angle whose deterministic check passes
    │   → its text becomes corrected_text
    ▼
analyze.json: {best_angle, score, case_passed, per-angle results}
```

## Key Findings

### 1. Single angle (format auditor) solved all 10 cases
Angle 1 alone produced output passing deterministic checks on every case. This contradicts the initial hypothesis that 5 angles compound fixes for harder cases.

**Why:** Our 5 issue types all manifest as surface-structure defects:
- Format drift → leaked prompt text, wrong bullet count
- Factual error → wrong number/word visible in text
- Missed requirement → missing line/field
- Hallucinated content → invented taxonomy/categories
- Refine-ignored-fix → original problem still present

All 5 are surface-visible to a strict format auditor with checklist.

### 2. Multi-angle cascade is harmful on small models
Angles 2-5 consistently over-mutated correct output from angle 1:
- Fact checker invented errors that didn't exist (claimed source says "four" when actual is "three")
- Requirements tracer reduced bullet count
- Hallucination hunter converted bullets to paragraphs
- Consistency auditor added meta-commentary

Even with HARD CONSTRAINTS ("PRESERVE format EXACTLY", temp 0.1), Ministral-3B did not respect preservation rules. Each subsequent angle propagated false fixes.

**Mitigation:** Analyzer-side best-angle selection. First angle passing deterministic check wins. This preserved the multi-angle workflow architecture while getting correct results.

### 3. Deterministic checks carry the load
The semantic_score heuristic (0-100) added little signal beyond deterministic pass/fail. Cases scoring 90 vs 100 had identical structural correctness; the 10-point difference came from soft heuristics (forbidden phrase proximity, word count slack) that didn't affect actual correctness.

**Implication:** For known task types with structured outputs, deterministic checks alone suffice. Reserve semantic scoring for unstructured/free-text cases.

### 4. Time cost analysis
- Single-angle would solve all 10 cases in ~7s avg (1 inference)
- Full 5-angle workflow: 38.7s avg (5 inferences + shell checks)
- **5.5× time premium** for the multi-angle architecture
- Premium buys: defense in depth (if angle 1 fails on harder cases, angles 2-5 might catch it)
- Premium costs: 5.5× latency on every case, including easy ones

### 5. Small model + narrow scope + checklist prompt = effective
Ministral-3B (3B params, CPU only, 10 tok/s) produced correct fixes on all 10 cases when given:
- Narrow scope (one issue type per pass)
- Explicit checklist (3 bullets of what to look for)
- Hard rules ("PRESERVE format EXACTLY", "Emit corrected text only")
- Source of truth (task spec + auxiliary)

This confirms the core hypothesis: small models can fix small-model mistakes when the framework enforces narrow scope per pass.

## Iteration History

| Version | Change                                   | Case 001 Result        |
| ------- | ---------------------------------------- | ---------------------- |
| v1      | 5 angles mutate freely                   | FAIL — cascade errors  |
| v2      | HARD CONSTRAINTS on angles 2-5           | FAIL — fact hallucination |
| v3      | Echo input default + deterministic gates | PASS via angle 1       |
| v3 + analyzer | Best-angle selection                | 10/10 PASS              |

## Files

```
experiments/correction-atom/
├── README.md (this file, well — SUMMARY.md)
├── benchmarks/
│   ├── SUMMARY.md           # This file
│   └── comparison.csv       # 10 cases × 6 fields
├── cases/
│   ├── case-001.yml         # format_drift: prompt-leaked summary
│   ├── case-002.yml         # format_drift: ORIGINAL_DRAFT prefix
│   ├── case-003.yml         # factual_error: T1/T5 wrong categories
│   ├── case-004.yml         # factual_error: TRANSFORM vs EXECUTE
│   ├── case-005.yml         # missed_requirement: YAML missing description
│   ├── case-006.yml         # missed_requirement: YAML preserved broken
│   ├── case-007.yml         # hallucinated_content: O(n²) preserved + try/except
│   ├── case-008.yml         # hallucinated_content: invented 8 criteria
│   ├── case-009.yml         # refine_ignored_fix: weasel words added
│   └── case-010.yml         # refine_ignored_fix: critique ignored
├── results/
│   ├── exp-001-case-001/         # initial v1 run (failed)
│   ├── exp-002-case-001-v2/      # v2 run (failed)
│   ├── exp-003-case-001-v3/      # v3 run (passed)
│   ├── exp-004-case-005-v3/      # case 005 standalone v3
│   ├── exp-005-case-003-v3/      # case 003 standalone v3
│   ├── exp-006-case-002-v3/      # case 002 standalone v3
│   └── exp-batch-v3-case-{003..010}/  # batch runs
├── scripts/
│   ├── emit-context.py       # Format case YAML → prompt context block
│   ├── check-angle.py        # Deterministic per-angle checks (bullets/words/forbidden/required/yaml)
│   ├── build-changelog.py    # Diff summary across angles
│   ├── analyze.py            # Best-angle selection + semantic score
│   └── run-correction.sh     # Safe runner (preflight + watchdog + cooldown)
└── workflows/
    ├── correction-atom-v1.yml  # Initial (failed)
    ├── correction-atom-v2.yml  # Hard constraints (failed)
    └── correction-atom-v3.yml  # Production (working)
```

## Success Gates

| Gate | Criterion                                         | Result                  |
| ---- | ------------------------------------------------- | ----------------------- |
| G1   | 10/10 cases pass deterministic checks             | ✅ PASS                 |
| G2   | Workflow bounded (no infinite loops)              | ✅ PASS (≤5 angles, ≤300s) |
| G3   | Deterministic + semantic analyzer operational     | ✅ PASS                 |
| G4   | All cases complete under 60s wall clock           | ✅ PASS (avg 38.7s)     |
| G5   | Crash safeguards active (preflight, watchdog)     | ✅ PASS                 |

## Recommendation

**Adopt angle 1 (FORMAT AUDITOR) + analyzer best-angle selection as the production correction atom.** Run angles 2-5 only when angle 1 fails deterministic check (currently 0/10 cases need this). This gives:
- ~7s wall clock on easy cases (1 inference)
- ~40s wall clock on hard cases (5 inferences, fallback cascade)
- Defense in depth without time penalty on common path
