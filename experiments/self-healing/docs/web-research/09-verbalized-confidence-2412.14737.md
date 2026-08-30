# Source: On Verbalized Confidence Scores + BrowseConf

- **URLs:**
  - https://arxiv.org/abs/2412.14737 (Verbalized Confidence)
  - https://arxiv.org/abs/2510.23458 (BrowseConf — confidence-guided test-time scaling)
- **Retrieved:** 2026-08-24 (websearch)

## Verbalized Confidence (2412.14737)

Model self-reports confidence (0–100) alongside answer.

- **Small models (7B):** simple prompt formulations work best (probscore)
- **Large models (72B+):** need few-shot + ranking + advanced descriptions
- Calibration quality: ECE, Brier score, AUROC
- **Overconfidence signature:** confidence > 0.9 with < 70% accuracy = miscalibrated

## BrowseConf (2510.23458)

Confidence-triggered scaling:

```
if C >= τ*: accept
else: generate more rollouts until C >= τ* or max attempts
```

- C < 0.3 → near-zero accuracy (hard abstention zone)
- τ* calibrated on validation set
- 40–60% token savings vs fixed-budget self-consistency

## What We Borrow

1. **F1 confidence-degradation signal** — primary paper's "confidence of last 3 outputs degraded" → spoofed per-attempt confidence in output.json (F1 attempt 1: 0.42, clean: 0.9); detect.py reads it
2. **τ* threshold discipline** — our θ=0.65 reliability threshold = same pattern: accept above, heal below. Threshold explicitly configurable, not hardcoded magic
3. **Small-model guidance** — 4B–9B models = simple confidence extraction only (single line "CONFIDENCE: 0.X" suffix contract); no fancy calibration machinery — matches weak-model findings
4. **Abstention zone** — R < 0.3 hard-fail zone → immediate fail (no heal) could be v2 addition; v1 heals everything below θ

## Caveat

Verbalized confidence from small models is poorly calibrated (overconfident). Detector weighting keeps C component at ω₁=0.35, never sole trigger. Deterministic detectors (schema, tool errors) dominate classification.
