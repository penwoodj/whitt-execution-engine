# Correction Atom v8 — Memory-Escalation Cascade: Results

**Date:** 2026-08-14/15 · **Workflow:** `workflows/correction-atom-v8.yml` · **Final suite:** 27/27 PASS, 29 live runs total (2 prompt-iteration rounds + 1 case-fix rerun)

## What v8 shipped (all 6 proposals from 27-case analysis)

1. **9B stage deleted** — 0/27 wins, 0/35 historical. Cascade: det → 1.7B fact → 4B requirements → 4B consistency.
2. **Escalation memory** — `emit-state.py` aggregates still-open failures across angles (latest-wins per check), injects with fix hints.
3. **Prompt structure = hybrid** (iteration found mid-suite): v7-style lens rubric (RECOGNIZE/FIX RULES) + memory section + hard output rules last. Terse-only prompts regressed 011/013 (1.7B/4B need the rubric); rubric+memory fixed both — 011 dropped 17s→4s.
4. **Hard-fail gate** in `select-best.py` — best-partial with leak/degenerate/forbidden/yaml failures → no final shipped (`hard_fail_input_fallback`). Never fired in final suite (all cases found a passing angle) — pure safety net.
5. **Verifier upgrades** — `contains_required` number-normalizes (three≡3, third≡3rd); long_token 25→35 alnum chars (model filenames legit).
6. **Case lint** — `test-cases.py` + `fixtures/reference-fixes.yml` (27 reference fixes). Every case: broken fails own checks, reference passes.

## Case-bugfixes made during v8 (authoring errors, not model errors)

- 006/010: `min_words` calibrated for prose applied to terse code — removed
- 021: `contains_required ["3rd failed retry"]` over-literal — relaxed to ["timeout","manual"] (forbiddens carry the case)
- 027: 3-bullet budget vs 4 timeline events — made 4 bullets
- 010: `contains_any` rejected correct ternary guard `if items else` — added
- 012 reference fix: model-filename token tripped long_token — threshold raised

## Headline: v7 → v8 (27 cases)

| Metric | v7 | v8 |
|---|---|---|
| Pass | 23/27 | **27/27** |
| Total wall | 353s | **187s (47% faster)** |
| Avg wall | 13.1s | **6.9s** |
| Slowest case | 044s (013) | 14s (015) |
| 9B loads | whenever reached | **0** |
| Leaks shipped | 0 | 0 (case-027 det pass-through caught by verifier, fixed at angle 2) |

Winning angles: det ×4, 1.7B ×4, 4B-req ×19. Angle-5 (consistency) never needed — cascade never exhausted in final suite.

## Iteration honesty notes

- First v8 prompt round (terse ROLE-only) went 25/27: 011/013 regressed — small models need the lens rubric. Hybrid restored, both cases fixed faster than v7.
- Second round: 010's correct ternary output exposed last verifier literalism; case fixed.
- Case-027 `after-angle-1.txt` contains leak markers BY DESIGN (broken output IS a prompt echo; deterministic pass-through can't understand it). Verifier flags it, angle 2 fixes it, final is clean. Leak counter reports the intermediate, not the shipped output.
- Stochastic watch: 013 passed first-try at angle 3 in 10s (v6 needed rerun, v7 needed angle 5). Memory + rubric appears to stabilize the historically flaky case, but n=1 run.

## Test coverage (cumulative 91 tests)

- `test_v8_features.py` — 16 (normalization, memory, gate)
- `test-cases.py` — 27-case lint
- Prior suites still green: format-fix 17, leak 20, degenerate 11, select-best 11, retry-feedback 15
