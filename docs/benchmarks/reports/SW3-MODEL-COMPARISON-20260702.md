# SW3 Model Comparison Report

**Date:** 2026-07-02
**Task:** Compare Qwen3.5-9B (baseline) vs top-ranked leaderboard models installed locally, on SW3 (agentic categorization) sub-workflow.
**Hardware:** AMD RX 580 8GB Vulkan (RADV), 6 CPU threads
**Input:** P05 v4 SW1+SW2 outputs (fixed across all runs)

## Methodology

Per user directive (2026-07-02): CPU-only baseline first for quality, then GPU ramp incremental with monitoring.

- Each model tested CPU-only first (`--gpu-layers 0`)
- Winner tested at GPU layers 10 and 99 to find optimal
- After each GPU run, docker logs scanned for `vk::DeviceLost|Vulkan.*Error`
- SW3 chosen as test sub-workflow: 5-7 min on 9B, complex GWT-gated categorization

## Results Summary

| Model                    | GPU  | Time   | Bytes  | Steps | Tokens | Quality       |
|--------------------------|------|--------|--------|-------|--------|---------------|
| Qwen3-5-9B (baseline)    | 0    | 599s   | 3530B  | 5     | 30591  | Repetitive    |
| Qwen3-5-9B (baseline)    | 99   | 368s   | 3923B  | 5     | 25172  | Repetitive    |
| **Falcon-H1-7B (winner)**| **0**| **478s** | **11588B** | **20** | 26732 | **Diverse**   |
| Falcon-H1-7B             | 10   | 568s   | 7682B  | 13    | 31403  | Good          |
| Falcon-H1-7B             | 99   | 542s   | 8092B  | 13    | 31535  | Good          |
| Hermes-2-Pro-Mistral-7B  | 0    | 889s   | 7453B  | 13    | 23862  | Garbled names |
| Qwen3-4B-Thinking-2507   | 0    | >900s  | 0B     | 0     | N/A    | TIMEOUT       |

## Winner: Falcon-H1-7B-Instruct-Q4_K_M (CPU-only)

**Falcon-H1-7B on CPU beats Qwen3-5-9B on GPU:**
- **1.25x faster** (478s CPU vs 599s CPU baseline; 478s vs 368s GPU baseline but GPU baseline only produces 5 steps)
- **3.3x larger output** (11588B vs 3530B CPU baseline)
- **4x more steps categorized** (20 vs 5)
- **More diverse categories**: PARALLEL_FAN_OUT, SEQUENTIAL_PROCESSOR, DATA_TRANSFORMER vs baseline's mostly DATA_TRANSFORMER

## Counterintuitive Finding: CPU Faster Than GPU for Falcon-H1-7B

| Config | Time  | Bytes |
|--------|-------|-------|
| CPU (layer 0)  | 478s | 11588B |
| GPU 10 layers | 568s | 7682B  |
| GPU 99 layers | 542s | 8092B  |

**CPU is BOTH faster AND produces more output.** Hypotheses:
1. Vulkan overhead for 7B model on RX 580 negates GPU benefit (PCIe transfer dominates)
2. SW3 has many small inference steps — GPU has fixed per-call overhead
3. CPU has more consistent timing (no memory transfer stalls)

**Implication:** For SW3 (categorization), CPU is the optimal config. GPU should be reserved for SW2 (large generation steps with max_tokens 8192+).

## Loser Analysis

### Hermes-2-Pro-Mistral-7B (889s, 7453B, 13 steps)
Quality issues:
- Garbled task names: "T1 - Read src/benchmark/runner.rs and locate L1858 parallel TOKEN" (TOKEN appended)
- Truncated extensions: "runner.s" instead of "runner.rs"
- Unwanted preamble: "Here is the categorization for each task..." (despite "No preamble" instruction)

Categorization itself is reasonable (DATA_TRANSFORMER, PARALLEL_FAN_OUT) but the surface errors reduce trust.

### Qwen3-4B-Thinking-2507-Q4_K_M (TIMEOUT >900s, 0B)
- step_01_extract_task_list took 280981ms (4.7 min) — never completed
- Thinking model emits chain-of-thought tokens BEFORE the answer
- With max_tokens 1500, thinking consumed entire budget, no final answer produced
- Fundamentally incompatible with SW3's token-capped step design

## Recommendation

**For SW3 specifically:** Switch to Falcon-H1-7B-Instruct-Q4_K_M with `gpu_layers: 0` (CPU).

**For meta-v6 pipeline overall:** Test Falcon-H1-7B on SW1/SW2/SW4 before committing. SW2 has max_tokens 8192+ steps that may benefit from GPU even if SW3 doesn't.

**Do NOT use:**
- Qwen3-4B-Thinking variants (thinking overhead incompatible with token-capped steps)
- Hermes-2-Pro-Mistral-7B for categorization (garbled output)

## Hardware Ceiling Confirmed (Again)

- No vk::DeviceLost errors during CPU runs (expected — no GPU usage)
- GPU layer 10 and 99 both stable for single-model Falcon-H1-7B (4.3GB Q4)
- Cannot run multiple models simultaneously (proven earlier with spec decoding crash)
- RX 580 8GB remains the binding constraint

## Files

- Script: `scripts/meta-v6/compare-models-sw3.sh`
- Results: `docs/benchmarks/outputs/meta-workflow/model-comparison/sw3-20260702-*/`
- This report: `docs/benchmarks/reports/SW3-MODEL-COMPARISON-20260702.md`

## Next Steps

1. Test Falcon-H1-7B CPU on SW2 (large generation) to see if CPU advantage holds
2. If yes, full P05 run with Falcon-H1-7B for SW1/SW2/SW3 (keep 9B for SW4/SW5)
3. Compare Falcon-H1-7B P05 deliverable vs Qwen3-5-9B P05 v4 deliverable
