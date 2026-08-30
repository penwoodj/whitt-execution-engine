# Source: Self-Consistency Improves Chain of Thought Reasoning (Wang et al.)

- **URL:** https://arxiv.org/abs/2203.11171
- **Retrieved:** 2026-08-24 (websearch)

## Method Mechanics

Sample N diverse reasoning paths (temperature/top-k), marginalize over final answers via majority vote:

```
p_SC = f* / N     # f* = count of most frequent final answer
```

- **N budget:** 10–40 samples typical; diminishing returns after ~20
- Consistency correlates with accuracy (ρ > 0.7 on GSM8K)
- Gains: GSM8K +17.9%, SVAMP +11.0%, AQuA +12.2%, StrategyQA +6.4%, ARC-c +3.9%

## As Detector (our use)

Low consistency = uncertainty signal: `p < 0.3` → high uncertainty indicator. Self-consistency doubles as confidence proxy WITHOUT requiring calibrated probabilities — works with logit-free local APIs.

## What We Borrow

1. **C component of R score** — primary paper's consistency component implemented as p_SC analogue. Spoof phase: case YAML supplies simulated sample agreement (e.g., F3 cases → agreement 0.33); live phase: N=5 samples (hardware-budget compromise vs paper's 10-40), p_SC over parsed answers
2. **p < 0.3 = uncertainty flag** — feeds F1 detection (confidence degradation signal)
3. **N choice documented** — N=5 justified by RX 580 8GB latency budget (5× inference per attempt is already heavy; 20 = unusable)

## Divergence

Spoof phase computes C from scripted agreement values (deterministic). No sampling. Live phase (user-gated) would run true multi-sample.
