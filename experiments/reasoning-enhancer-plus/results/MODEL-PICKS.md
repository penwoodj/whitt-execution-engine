# MODEL-PICKS — stage-1 gate artifact

5 specialists from live probe results.

| category | model | probe score | wall_s |
|---|---|---|---|
| FMT | Qwen3-4B-Instruct-2507-Q4_K_M | 1.00 | 26.6 |
| DRV | Qwen3-4B-Thinking-2507-Q4_K_M | 0.75 | 442.8 |
| LOG | Qwen3-4B-Thinking-2507-Q4_K_M | 1.00 | 442.8 |
| PLN | Qwen3-4B-Instruct-2507-Q4_K_M | 0.75 | 26.6 |
| AUD | Falcon-H1-7B-Instruct-Q4_K_M | 0.25 | 48.9 |

Distinct models: 3

STAGE-2 gate: OPEN (case authoring permitted)

## Substitutes / runner-ups (live-proven, category-redundant)

| model | role | evidence |
|---|---|---|
| jan-nano-128k-Q6_K | FMT backup (1.00, 36.3s) | tied FMT score, lost wall tiebreak |
| Hermes-2-Pro-Mistral-7B.Q4_K_M | FMT backup (1.00, 54.0s) | tied FMT score, lost wall tiebreak |
| Mistral-7B-Instruct-v0.3-Q4_K_M | PLN backup (0.75, 45.6s) | tied PLN score, lost wall tiebreak |
| Qwen3-5-9B-Q4_K_M | DRV backup (0.25) + overcontext reference | only other DRV>0 model |

## Caveats (honest)

1. **AUD winner is weak** — Falcon-H1 0.25. No model exceeds 0.25 on AUD.
   Either battery AUD cases are miscalibrated (too strict) or the whole
   4B-11B pool genuinely fails audit tasks. Stage-2 must recalibrate before
   trusting AUD signal (tracked in 07-TRACKING).
2. **Bigger != better** — Qwen3-5-9B total 0.35 < both 4B Qwens (0.55/0.50).
   The 9B is NOT the strongest solver on this battery. Stage-2 gap-band
   "bigger-model-passes" anchor = category winners, NOT raw parameter count.
3. **Excluded models** — Phi-4-mini-instruct + Phi-4-mini-reasoning:
   model-specific Vulkan GGML_ASSERT crash on load (documented 07-TRACKING).
   Yi-6B-Airo (roleplay FT, echoes prompts) and LWM (im_start token spam)
   scored genuine 0/20 — incompatible, not artifacts.
4. Pool exhausted at 11 usable models; 5th specialist slot cannot be filled
   by a distinct winner without violating ≤2-categories/model scoring rule.
