# Model Aggregation Report — Stress Test

**Generated:** 2026-07-02T23:01:31-05:00
**Total unique (model, config) pairs:** 22
**Source batches:** 6

## Methodology

- Each model runs the `model-stress-test.yml` workflow repeatedly within a 180s time budget.
- Per-iteration outputs land in `iter-N/`.
- Quality scores from step_02 (long-form code gen) and step_03 (multi-perspective eval) are tracked.
- `best_sN_quality` = highest quality_score seen across iterations for step N.
- Verdict: PASS requires best_s2_quality > 0.5 AND refusals < iterations.

## Ranking (sorted by best_s3 desc, then s2 desc)

| Rank | Model | Config | Iters | best_s2 | best_s3 | tokens | bytes | refusals | verdict |
|------|-------|--------|-------|---------|---------|--------|-------|----------|---------|
| 1 | `Ministral-3-3B-Instruct-2512-Q4_K_M` | gpu0 | 3 | 0.337 | **1.000** | 18554 | 22409 | 0 | FAIL |
| 2 | `Qwen3-5-9B-Q4_K_M` | gpu99 | 2 | 0.302 | **1.000** | 7649 | 15504 | 0 | FAIL |
| 3 | `Qwen3-5-9B-Q4_K_M` | gpu0 | 1 | 0.298 | **1.000** | 2746 | 8290 | 0 | PASS |
| 4 | `Falcon3-3B-Instruct-q8_0` | gpu0 | 3 | 0.243 | **0.871** | 13666 | 20173 | 0 | FAIL |
| 5 | `Falcon-H1-7B-Instruct-Q4_K_M` | gpu0 | 1 | 0.244 | **0.866** | 2297 | 6017 | 0 | PASS |
| 6 | `LFM2-2.6B-SDG-q8` | gpu0 | 4 | 0.228 | **0.863** | 15144 | 19447 | 0 | FAIL |
| 7 | `Qwen3-1.7B-abliterated-q4_k_m` | gpu0 | 5 | 0.185 | **0.767** | 18367 | 33286 | 0 | FAIL |
| 8 | `Falcon-H1-7B-Instruct-Q4_K_M` | gpu99 | 2 | 0.199 | **0.731** | 6364 | 8105 | 0 | FAIL |
| 9 | `llama-3.2-1b-instruct-q8_0` | gpu0 | 7 | 0.209 | **0.726** | 24927 | 42627 | 0 | FAIL |
| 10 | `LFM2.5-1.2B-Instruct-Q8_0` | gpu0 | 8 | 0.207 | **0.717** | 28573 | 34755 | 0 | FAIL |
| 11 | `Llama-3.2-3B-Instruct-Q4_K_S` | gpu0 | 5 | 0.165 | **0.624** | 16118 | 24840 | 0 | FAIL |
| 12 | `granite-4.0-h-tiny-Q4_K_M` | gpu0 | 4 | 0.166 | **0.615** | 13383 | 24470 | 0 | FAIL |
| 13 | `Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M` | gpu0 | 2 | 1.000 | **0.335** | 8675 | 21131 | 0 | PASS |
| 14 | `Instella-3B-Q8` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 15 | `Jan-v3-4b-base-instruct-Q4_K_M` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 16 | `Phi-4-mini-reasoning-Q4_K_M` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 17 | `Hermes-2-Pro-Mistral-7B.Q4_K_M` | gpu99 | 1 | 0.000 | **0.000** | 1831 | 1182 | 0 | FAIL |
| 18 | `mistral-7b-instruct-v0.2.Q4_K_S` | gpu99 | 1 | 0.000 | **0.000** | 1882 | 1503 | 0 | FAIL |
| 19 | `Qwen3-4B-Instruct-2507-Q4_K_M` | gpu99 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 20 | `jan-nano-128k-Q6_K` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 21 | `Phi-4-mini-instruct-Q4_K_M` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |
| 22 | `LWM-Text-Chat-1M-Q4_K_M` | gpu0 | 1 | 0.000 | **0.000** | 0 | 0 | 0 | FAIL |

## Top 5 Contenders

### #1 `Ministral-3-3B-Instruct-2512-Q4_K_M` (gpu0)
- best_s2_quality: **0.337**
- best_s3_quality: **1.000**
- iterations in 180s: 3
- total tokens: 18554
- total bytes: 22409
- refusals detected: 0
- verdict: FAIL
- source batch: `stress-20260702-213216`

### #2 `Qwen3-5-9B-Q4_K_M` (gpu99)
- best_s2_quality: **0.302**
- best_s3_quality: **1.000**
- iterations in 180s: 2
- total tokens: 7649
- total bytes: 15504
- refusals detected: 0
- verdict: FAIL
- source batch: `stress-20260702-221321`

### #3 `Qwen3-5-9B-Q4_K_M` (gpu0)
- best_s2_quality: **0.298**
- best_s3_quality: **1.000**
- iterations in 180s: 1
- total tokens: 2746
- total bytes: 8290
- refusals detected: 0
- verdict: PASS
- source batch: `stress-20260702-172333`

### #4 `Falcon3-3B-Instruct-q8_0` (gpu0)
- best_s2_quality: **0.243**
- best_s3_quality: **0.871**
- iterations in 180s: 3
- total tokens: 13666
- total bytes: 20173
- refusals detected: 0
- verdict: FAIL
- source batch: `stress-20260702-214940`

### #5 `Falcon-H1-7B-Instruct-Q4_K_M` (gpu0)
- best_s2_quality: **0.244**
- best_s3_quality: **0.866**
- iterations in 180s: 1
- total tokens: 2297
- total bytes: 6017
- refusals detected: 0
- verdict: PASS
- source batch: `stress-20260702-172957`

## Insights

- **s3 quality = 1.0:** 3 models — Ministral-3-3B-Instruct-2512-Q4_K_M, Qwen3-5-9B-Q4_K_M, Qwen3-5-9B-Q4_K_M
- **s2 quality >= 0.4:** 1 models
- **s2 quality >= 0.3:** 3 models — Ministral-3-3B-Instruct-2512-Q4_K_M, Qwen3-5-9B-Q4_K_M, Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M
- **zero refusals (and ran):** 22 models

- **most iterations in 180s:** `LFM2.5-1.2B-Instruct-Q8_0` with 8


## Per-Config Summary

| config | models tested | avg best_s3 | avg best_s2 | avg iters |
|--------|---------------|-------------|-------------|-----------|
| gpu0 | 17 | 0.493 | 0.193 | 2.9 |
| gpu99 | 5 | 0.346 | 0.100 | 1.4 |

## Recommendation

**Top overall:** `Ministral-3-3B-Instruct-2512-Q4_K_M` (s3=1.000, s2=0.337, iters=3, config=gpu0)

For SW3 categorization (small steps, many calls): prefer CPU for <=4GB models (avoids Vulkan overhead).
For SW2 generation (large single-shot outputs): prefer GPU=99 for >=7B models (Vulkan helps with sustained throughput).

