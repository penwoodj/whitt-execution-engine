# Web Research: LLM-as-Judge + Leak-Safety (2024-2026)

> Compiled 2026-08-23. 12 findings from librarian web research.

## Small-Model-as-Judge

### F1. Representation probing beats prompting for small judges
- 0.6B-8B models encode evaluative signal in hidden states; probing mid-upper layers = 0.83 F1 vs 0.56 prompted.
- Needs logit access — NOT available via llama.cpp server. SKIP, noted.
- Src: arxiv.org/html/2601.22588

### F2. SAJA: calibration head > prompt engineering (ACL 2026 industry.45)
- Structured rubric JSON + calibration head, single call, 4.8x cheaper. 9B+SAJA (ρ=0.70) > GPT-4.1 raw (ρ=0.60).
- Needs head training. SKIP, noted.

### F3. Decompose quality into 3-5 BINARY criteria ← ADOPT
- Holistic score noisy; per-criterion binary verdicts cut variance 3-4x. Verifiable criteria 0.71 accuracy vs 0.19 subjective.
- v3 judge prompt: ask binary per-key check ("all keys present? values plausible? exact format?") then aggregate. Src: galileo.ai/blog/tricks-to-improve-llm-as-a-judge

## Judge-Worker Asymmetry

### F4. Self-preference bias: +0.14 mean, family-wide
- Every judge inflates own family. Blind evaluation + consensus baseline mitigates.
- Our mitigation: judge sees ONLY objective + artifact (no model name, no prompt). Same-model judging accepted as limitation; deterministic gate is primary truth, judge secondary. Src: github.com/hankimis/self-preference

### F5. Cross-family panel reduces self-preference 31.5% — SKIP (single local model). Future: 9B judge + 4B worker = family-same but size-different.

### F6. Position/length bias: first-shown wins 63%, length↔verdict +0.98
- Randomize order when comparing; check length-correlation in judge verdicts. Noted for analysis; our judge sees single artifact (no pairwise) = immune.

## Answer Leakage in Repair Loops

### F7. Test feedback (61%) > minimal "wrong, fix" (53%) > compiler (49%)
- Structured failing-check feedback best. Minimal feedback surprisingly strong fallback. Src: arxiv.org/html/2504.06939v1
- v3 fix feedback = failing check names + categories + model's observed values (never expected).

### F8. Leak-safe feedback format: location + observed + admissible alternatives ← KEY
- +44 pts vs location-only. Alternatives contribute +36. Prose ≈ JSON (format flexible).
- TENSION with exact-answer tasks: alternatives = the answer. RESOLUTION: for json_exact we give location (key name) + observed (model's value) + constraint class ("wrong magnitude" etc), NEVER expected value. Src: arxiv.org/html/2607.14167v1

### F9. Repair@k plateaus at 2-3 ← ADOPT
- Gain repair@1→2 big, @3 marginal. Our FIX x2 cap is correct; x3 = waste.

## Deterministic-First Gating

### F10. Three-stage cascade = 95% cost cut ← CONFIRMS design
- Deterministic scanners (30-60% catch) → small classifier → judge only on ambiguous 5-10%.
- Our analog: json_exact check (free) catches most; judge runs only on det-PASS. Src: futureagi.com/blog/deterministic-vs-llm-judge-evals-2026

### F11. Two-tier CI: deterministic BLOCKS, judge TRENDS
- Deterministic = hard gate; judge = dashboard signal. Our analog: det check routes; judge PASS/FAIL logged to trace, does not override det verdict. Src: dreaming.press (agent evals in CI)

## Verdict Aggregation

### F12. Mean aggregation > median/majority on score distributions
- Sample T=0.3-0.7 x N=8, average. For future judge-improvement iterations. Src: arxiv.org/html/2506.13639v1

## v3 Design Consequences

1. Judge prompt: 3 binary sub-checks (keys present? values plausible? exact JSON?) then VERDICT/REASON/SCORE.
2. feedback.py v2: show failing key NAMES + model's observed values, never expected. Category hints ("wrong magnitude").
3. FIX x2 cap (repair@k plateau).
4. Judge = secondary signal in trace; det check = primary verdict. (Already so.)
5. Judge blindness: objective + artifact only (already so — H2 confirmed).
