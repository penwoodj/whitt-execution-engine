# Reasoning Enhancer Atom v1.1 — Live Results

**Date:** 2026-08-14 · **Workflow:** `workflows/rea-v1.yml` (v1.1) · **Final suite: 6/6 PASS**

## Revamp applied pre-live (v8 findings, SUMMARY-v8.md)

1. Hybrid rubric prompts (LENS/RECOGNIZE/<ROLE> RULES + context + hard output
   rules last) — v8 terse-only regressed 25/27.
2. `check_lib` number-normalization port (three≡3, third≡3rd).

## Final suite (6 cases, clean run, identical build)

| case | domain | dur | rounds | mode | input→ship subchecks | pass |
|---|---|---|---|---|---|---|
| rea-001 | easy/arithmetic | 24s | 2 | selected_r2 | 6/8 → 8/8 | ✅ |
| rea-002 | easy/formatting | 14s | 1 | selected_r1 | 6/8 → 8/8 | ✅ |
| rea-003 | medium/constraint-arith | 16s | 1 | selected_r1 | 6/8 → 8/8 | ✅ |
| rea-004 | medium/multistep-arith | 10s | 1 | selected_r1 | 7/8 → 8/8 | ✅ |
| rea-005 | hard/planning | 15s | 1 | selected_r1 | 5/8 → 8/8 | ✅ |
| rea-006 | hard/structured-yaml | 13s | 1 | selected_r1 | 8/9 → 9/9 | ✅ |

Total 92s wall (median 14.5s, cap 300s). 5/6 first-round pass; rea-001 needed
round 2 (escalation memory fixed r1's open failures live). Zero leaks, zero
degenerate ships, zero hard-fail ships. G5 ratchet: HOLDS (every ship ≥ input).

## Gates

- G1 case lint 6/6 ✅ · G2 workflow validates ✅ · G3 finals 6/6 ≥ 5/6 ✅
- G4 median 14.5s ≤ 180s ✅ · G5 ratchet holds ✅ · G6 6/6 enhanced-passes-where-draft-failed ✅

## Live-run bug log (all fixed same session; full detail REVIEW-CYCLES.md)

- **B6 exit-code inversion** — `check-answer.py` exited 0 always → gwt
  early-exit fired on FAILED checks → round 2 never ran. Fix: exit 0 iff
  passed (v8 check-angle semantics). Dry-run upgraded to assert the contract.
- **B7 stdout redirect killed prompt context (ROOT CAUSE of run-1/2 garbage)**
  — emit shells redirected stdout to log files → `{{bookmarks.shell_output.stdout}}`
  = EMPTY → models saw only static YAML rubric, no task → total fabrication
  (r1 "latency/GDPR", decomposer echoed rubric nouns as fake JSON
  `{"lenses":[...]}`). Fix: emit shells print to stdout + `--out` file for
  observability; redirect ONLY exit-signal shells (v8 pattern). This one bug
  explained every garbage output in runs 1-2.
- **B8 sub-question validator too lenient** — JSON dict without `subquestions`
  fell through to line-parse; JSON syntax fragments "validated". Fix: dict
  without key = invalid; skip `{["`-leading lines.
- **B9 solver arithmetic slip** — 4B guessed 42−31−5=8 (expression in source).
  Fix: SOLVE RULES "show derived numbers as inline arithmetic" (DialCoT:
  sequential explicit computation for <10B). rea-001 now derives correctly.
- **B10 verifier calibration (case-003)** — min_words 20 rejected a perfect
  19-word allocation (v8 finding #7 repeat). Fix: terse-format cases use
  contains_required phrase ("full budget") not word counts.
- **B11 sum-check prose trap (case-003)** — restating the $1200 total in
  prose double-counts in numbers_must_sum_to. Fix: prompt "never restate the
  total budget figure" (check unchanged — target-removal heuristic would
  falsely pass the draft; contains_required still catches wrong items).

## Ops events

- Preflight aborted 3 runs mid-suite (swap used crossed 10GB gate from
  accumulated cold pages across 9 model load/unload cycles; vmstat so=0 =
  no active thrash). Remedy: docker restart whitt-llama-server → suite
  completed. Safety system worked as designed.

## v8 mechanism transfer — confirmed working live

- Deterministic-first gate, escalation memory (rea-001 r1→r2 live fix),
  ratchet select-best, hard-fail classes, hybrid rubric prompts, verifier
  number-normalization, early-exit cascade (5/6 never needed round 2).
- REA additions holding: factored sub-solve (solver never sees draft),
  inline-arithmetic rule, anti-meta output rule, sub-question JSON contract
  w/ rejection routing.
