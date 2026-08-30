# Source: Total Uncertainty = Aleatoric + Epistemic for Agent Reliability

- **URL:** https://arxiv.org/abs/2604.17112
- **Retrieved:** 2026-08-24 (websearch)

## Method Mechanics

- **AU (aleatoric):** intra-model self-consistency across 10 samples/model
- **EU (epistemic):** inter-model semantic disagreement across 5 models (7–9B)
- **TU = AU + EU** — combined uncertainty for abstention decisions
- TU threshold τ* selected on validation set (SailorFog-QA)

## Key Numbers

- +15–20% AUROC improvement over AU alone
- Mistake detection AUROC: 18.2% baseline → 36.2% with TU (GAIA subset)
- Confident errors = low AU (model consistent but wrong) — caught only by EU

## What We Borrow

1. **"Confident errors" concept** — F1 hallucination may show HIGH consistency (model confidently wrong). C component alone cannot catch F1 → S component (semantic/schema validation) and hallucination markers must carry F1 detection. Classify priority reflects this: hallucination marker → F1 even when C high
2. **Multi-model disagreement as signal** — v2+: cross-model check (Qwen3-4B vs Ministral-3B answers diverge → EU flag). Our workflow already has 5 distinct models mapped to roles; a future detect v2 could compare worker outputs across 2 models. v1: spoofed in case data
3. **Threshold-on-validation discipline** — θ and class thresholds recorded per-run in report.json for later calibration analysis

## Divergence

Full AU+EU needs 10 samples × 5 models = 50 inferences/attempt. Hardware-forbidden. v1 = deterministic spoof; live v1 = single-sample + scripted signals only.
