# Model Comparison Report: Meta-Workflow Pipeline Testing

**Date:** 2026-07-13
**Models Tested:** Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M, Falcon-H1-7B-Instruct-Q4_K_M, Ministral-3-3B-Instruct-2512-Q4_K_M
**Prompts:** P19 (Docker Health Recovery), P06 (ADR Fragmentation Analysis), P15 (Wiki Link Audit)
**Total Runs:** 9 meta-gen + 9 exec = 18 pipeline executions

---

## Executive Summary

All 9 runs completed the full SW1→SW5 meta-workflow generation + exec pipeline. **All 9 produced 0-byte deliverables** due to a systemic generator issue: the assembled workflows save intermediate state to `./state/*.json` but lack a final deliverable assembly step writing to `outputs/deliverable.md`.

Despite the shared deliverable failure, **quality and speed differ significantly across models**:

| Metric | Yi-6B (6B) | Falcon-H1-7B (7B) | Ministral-3-3B (3B) |
|--------|-----------|-------------------|---------------------|
| **Avg Exec Quality Score** | 0.031 | 0.023 | **0.111** |
| **Total Wall Time (9 runs)** | 117 min | 116 min | **85 min** |
| **Meta-Gen Success** | 3/3 | 3/3 | 3/3 |
| **Deliverable Produced** | 0/3 | 0/3 | 0/3 |
| **Parity Score** | 10/50 FAIL | 10/50 FAIL | 10/50 FAIL |
| **Output Pattern** | Repetition loops, HTML tags | Consistent but low quality | **Coherent, consistent** |

**Key Finding:** Ministral-3-3B — the smallest model — produces the highest quality output (3-5x better quality scores) at the fastest speed (27% faster), despite all models failing the deliverable check.

---

## Test Configuration

### Hardware
- **GPU:** AMD RX 580 8GB (Vulkan/RADV)
- **RAM:** 16GB system, 5GB available during tests
- **Backend:** llama.cpp in Docker with Vulkan, `gpu_layers: 99` (full offload)

### Pipeline Configuration
- **Meta-Workflow:** SW1 (Task Deconstruction) → SW2 (Desired Output State) → SW3 (Agentic Categorization) → SW4 (YAML Substructure Translation) → SW5 (Final Workflow Assembly)
- **Context Size:** 32,768 tokens
- **KV Cache:** Q8_0 quantization
- **Vulkan Flags:** `--no-cache-prompt` (required), no `--cont-batching`
- **Timeouts:** Meta-gen 3600s/prompt, Exec 1800s/prompt, Load 300s

### Prompts
| Prompt | Description | Complexity |
|--------|-------------|------------|
| **P19** | Docker Health Recovery — add retry logic to engine | Medium (code generation) |
| **P06** | ADR Fragmentation Analysis — analyze architecture decision records | High (multi-file analysis) |
| **P15** | Wiki Link Audit — audit bidirectional wiki links | High (cross-reference analysis) |

---

## Results Overview

### Complete Results Table

| Model | Prompt | Config | Steps | Deliverable (B/L) | Refusals | Parity | Duration |
|-------|--------|--------|-------|-------------------|----------|--------|----------|
| Yi-6B | P19 | full | 20 | 0/0 | 0 | 10/50 FAIL | 22 min |
| Yi-6B | P06 | full | 20 | 0/0 | 0 | 10/50 FAIL | 54 min |
| Yi-6B | P15 | full | 20 | 0/0 | 0 | 10/50 FAIL | 41 min |
| Falcon-H1-7B | P19 | full | 20 | 0/0 | 0 | 10/50 FAIL | 38 min |
| Falcon-H1-7B | P06 | full | 20 | 0/0 | 0 | 10/50 FAIL | 44 min |
| Falcon-H1-7B | P15 | full | 20 | 0/0 | 0 | 10/50 FAIL | 33 min |
| Ministral-3-3B | P19 | full | 20 | 0/0 | 0 | 10/50 FAIL | 33 min |
| Ministral-3-3B | P06 | full | 20 | 0/0 | 0 | 10/50 FAIL | 25 min |
| Ministral-3-3B | P15 | full | 20 | 0/0 | 0 | 10/50 FAIL | 27 min |

### Speed Comparison

| Model | P19 | P06 | P15 | Total | Avg |
|-------|-----|-----|-----|-------|-----|
| Yi-6B | 22 min | 54 min | 41 min | **117 min** | 39 min |
| Falcon-H1-7B | 38 min | 44 min | 33 min | **116 min** | 39 min |
| Ministral-3-3B | 33 min | 25 min | 27 min | **85 min** | 28 min |

Ministral-3-3B is **27% faster** than either competitor.

---

## Model-by-Model Analysis

### 1. Yi-6B-200K-Airo-Claude-Puffin (6B, Q4_K_M, ~3.5GB)

**Stress Test Score:** s2=1.0 (perfect code generation), s3=0.335

#### Meta-Gen Phase
- **SW1-3:** Completed quickly but produced garbage output (HTML tags, template artifacts)
- **SW4:** Hit repetition loops — step_04_fix produced 11,851 tokens of identical text (`<|im_start|>assistant\nAssist with categorization...` repeated 40+ times) over 5 minutes
- **SW5:** Assembled workflow successfully (deterministic template, doesn't require LLM quality)

#### Exec Phase
- **Quality Scores:** Highly variable — mix of false 1.0 (short non-refusal outputs) and very low 0.003-0.04
- **Output Pattern:** HTML tags (`</body>`, `</transform>`), template artifacts, repetition
- **Step Duration:** Steps executed in <1s each (model produced 30-35 token garbage outputs instantly)

#### Key Issues
1. **Repetition loops:** Yi-6B gets stuck repeating the same text with `<|im_start|>assistant` tokens
2. **Template artifacts:** Produces XML/HTML tags instead of real content
3. **False quality scores:** Short non-refusal outputs score 1.0 despite being meaningless
4. **Cannot follow multi-step instructions:** Despite s2=1.0 on stress test (single-shot code gen)

#### Sample Output (P19, step_t1_load_session_metadata)
```
</suggestion>\n\n<replace-with-prompt>Replace with Prompt:\n"+usage+"\n\n> "+prompt+"
```
Quality score: 0.0078 — template artifact, not real content.

---

### 2. Falcon-H1-7B-Instruct (7B, Q4_K_M, ~4.4GB)

**Stress Test Score:** s3=0.866 (4th place overall)

#### Meta-Gen Phase
- **SW1-3:** Completed with real content — proper task categorization format
- **SW4:** Produced real YAML structure with correct categories and budgets
- **SW5:** Assembled workflow successfully

#### Exec Phase
- **Quality Scores:** Consistently low (0.01-0.07) — no false 1.0 scores, no garbage
- **Output Pattern:** Conversational responses, coherent but shallow
- **Step Duration:** Steps took 5-45s each (model produced 100-300 token outputs)

#### Key Issues
1. **Low quality ceiling:** Falcon produces consistent but shallow output — never terrible, never great
2. **No repetition loops:** Unlike Yi, Falcon stays on track
3. **Real categorization:** SW4 output was properly formatted with task categories
4. **Slow inference:** 7B model at ~15 tok/s (vs 30 tok/s for Ministral)

#### Sample Output (P19, SW4 step_01_load_categories)
```
T1 - Read runner.rs for retry logic | CATEGORY: DATA_TRANSFORMER | BUDGET: 1
T2 - Read...
```
Proper format with categories — much better than Yi's HTML tags.

---

### 3. Ministral-3-3B-Instruct-2512 (3B, Q4_K_M, ~2.0GB)

**Stress Test Score:** s3=1.0 (tied for 1st place overall)

#### Meta-Gen Phase
- **SW1-3:** Completed quickly with coherent output
- **SW4:** Produced valid YAML structure
- **SW5:** Assembled workflow successfully
- **Speed:** Fastest meta-gen of all 3 models

#### Exec Phase
- **Quality Scores:** Consistently highest (0.08-0.16) — 3-5x better than competitors
- **Output Pattern:** Coherent, conversational responses that address the prompt
- **Step Duration:** Steps took 3-15s each (model produced 50-200 token outputs at ~30 tok/s)

#### Key Issues
1. **Highest quality:** Quality scores consistently 0.08-0.16 across all steps and prompts
2. **Most consistent:** Lowest variance in quality scores across steps
3. **Fastest inference:** 30 tok/s (2x faster than Falcon, 3x faster than Yi on garbage)
4. **Still below threshold:** Even 0.11 avg quality is insufficient for production use
5. **Conversational, not structured:** Produces prose instead of structured JSON/YAML

#### Sample Output (P19, content_index.json)
```
It looks like you're referring to a concept from deep learning frameworks...
```
Coherent response — addresses the prompt, but conversational rather than structured.

---

## Root Cause Analysis: Why All 9 Runs Failed

### The Deliverable Gap

All 9 runs scored 10/50 on parity check due to the same root cause: **the generated workflows lack a deliverable assembly step**.

#### What Happens
1. SW1-SW5 generates a workflow YAML with ~20 steps
2. Each step has a `save_to` hook writing to `./state/{variable_name}.json`
3. The exec phase runs all 20 steps, producing 15-20 state files
4. **No step assembles these state files into a final `outputs/deliverable.md`**
5. Parity check looks for `outputs/deliverable.md` → not found → 10/50 FAIL

#### State Files Produced (P19 example)
```
state/session_metadata.json      (2,195 bytes)
state/file_manifest.json         (1,740 bytes)
state/directory_manifest.json    (1,220 bytes)
state/content_index.json         (3,425 bytes)
state/directory_structure_map.json (1,740 bytes)
state/overlap_analysis_result.json (956 bytes)
state/index_claims.json          (1,405 bytes)
state/report_counts.json         (1,882 bytes)
... (15+ files total)
```

The workflow **does execute** and **does produce output** — it just doesn't assemble a final deliverable.

#### Why This Is a Generator Issue, Not a Model Issue
- All 3 models produce the same workflow structure (SW5 is deterministic)
- The workflow template lacks a "assemble deliverable" final step
- This is a **template/generator bug**, not a model capability issue
- Fix: Add a final step to the SW5 template that concatenates state files into `outputs/deliverable.md`

#### Additional Issue: State Files Written to Wrong Location
- Save_to paths use `./state/*.json` (relative to CWD = repo root)
- Should use `${output_dir}/state/*.json` or `${WHITT_OUTPUT_DIR}/state/*.json`
- Result: state files pollute repo root instead of staying in exec output dir

---

## Quality Score Comparison

### Average Exec Quality Scores (20 steps per run)

| Model | P19 | P06 | P15 | Overall Avg |
|-------|-----|-----|-----|-------------|
| Yi-6B | 0.014 | 0.024 | 0.057 | **0.031** |
| Falcon-H1-7B | 0.024 | 0.020 | 0.025 | **0.023** |
| Ministral-3-3B | 0.110 | 0.109 | 0.113 | **0.111** |

### Quality Score Distribution

| Model | Min | Max | Median | Std Dev |
|-------|-----|-----|--------|---------|
| Yi-6B | 0.003 | 1.0* | 0.008 | 0.22 (high variance) |
| Falcon-H1-7B | 0.002 | 0.07 | 0.020 | 0.013 (low variance) |
| Ministral-3-3B | 0.080 | 0.158 | 0.109 | 0.018 (low variance) |

*Yi-6B has false 1.0 scores from short non-refusal outputs

### Key Observations
1. **Ministral-3-3B has 3.6x higher quality than Yi-6B** and 4.8x higher than Falcon
2. **Falcon has lowest variance** — consistently mediocre (never terrible, never great)
3. **Yi-6B has highest variance** — mix of garbage (0.003) and false positives (1.0)
4. **Ministral-3-3B has lowest variance** among real scores — consistently the best

---

## SW4 Output Quality Comparison

SW4 (YAML Substructure Translation) is the critical step where the model must produce structured YAML. This is where model differences are most visible.

| Model | SW4 Output Quality | Pattern |
|-------|-------------------|---------|
| Yi-6B | Garbage | HTML tags, repetition loops, `<\|im_start\|>assistant` tokens |
| Falcon-H1-7B | Structured | Proper task categorization with categories and budgets |
| Ministral-3-3B | Coherent | Valid YAML structure with real content |

### Yi-6B SW4 Failure (P06, step_04_fix)
- **Duration:** 298 seconds (5 min)
- **Tokens:** 11,851
- **Content:** Same paragraph repeated 40+ times with chat template tokens
- **Quality Score:** 1.0 (false positive — not a refusal, so scored as valid)

### Falcon-H1-7B SW4 Success (P19, step_01_load_categories)
- **Duration:** 38 seconds
- **Tokens:** ~200
- **Content:** `T1 - Read runner.rs for retry logic | CATEGORY: DATA_TRANSFORMER | BUDGET: 1`
- **Quality Score:** 0.025 (low but real content)

### Ministral-3-3B SW4 (P19)
- **Duration:** ~5 minutes total for all SW4 steps
- **Content:** Valid YAML with proper structure
- **Quality Score:** 0.08-0.16 per step

---

## Meta-Gen Phase Analysis

### SW1-SW5 Completion Rates

| Model | SW1 | SW2 | SW3 | SW4 | SW5 | All Complete |
|-------|-----|-----|-----|-----|-----|---------------|
| Yi-6B | 3/3 | 3/3 | 3/3 | 3/3* | 3/3 | 3/3 |
| Falcon-H1-7B | 3/3 | 3/3 | 3/3 | 3/3 | 3/3 | 3/3 |
| Ministral-3-3B | 3/3 | 3/3 | 3/3 | 3/3 | 3/3 | 3/3 |

*Yi-6B SW4 completed but with garbage output (repetition loops)

### Meta-Gen Duration by SW Stage (P19 average)

| Stage | Yi-6B | Falcon-H1-7B | Ministral-3-3B |
|-------|-------|--------------|----------------|
| SW1 | ~30s | ~30s | ~10s |
| SW2 | ~6 min | ~6 min | ~4 min |
| SW3 | ~30s | ~7 min | ~7 min |
| SW4 | ~16 min* | ~10 min | ~4 min |
| SW5 | ~3 min | ~3 min | ~3 min |
| **Total** | ~25 min | ~26 min | ~18 min |

*Yi-6B SW4 was slow due to 5-minute repetition loop

---

## Conclusions

### 1. Ministral-3-3B Is the Best Alternative Model
- **3.6x higher quality** than Yi-6B, 4.8x higher than Falcon
- **27% faster** than both competitors
- **Most consistent** output quality (lowest variance)
- **Smallest model** (2GB vs 3.5GB vs 4.4GB) — fastest to load, least VRAM

### 2. Yi-6B Cannot Follow Multi-Step Instructions
- Despite s2=1.0 on stress test (single-shot code generation)
- Gets stuck in repetition loops on SW4
- Produces HTML tags and template artifacts
- **Stress test scores do not predict pipeline viability**

### 3. Falcon-H1-7B Is Consistent but Low Quality
- Never produces garbage (unlike Yi)
- Never produces high quality (unlike Ministral)
- Slowest inference (7B at ~15 tok/s)
- SW4 output was properly formatted but shallow

### 4. The Deliverable Gap Is a Generator Bug, Not a Model Issue
- All 3 models "fail" because the workflow template lacks a deliverable step
- The workflow executes correctly (20 steps, 15+ state files produced)
- Fix: Add final assembly step to SW5 template
- This fix would benefit all models equally

### 5. None of the 3 Models Can Replace Qwen3-5-9B
- Qwen3-5-9B achieved 45/50 parity on P05 v4 with anti-refusal template
- Best alternative (Ministral) averages 0.11 quality score — still far below production threshold
- All 3 alternatives produce 0B deliverables due to generator issue
- **Recommendation: Continue using Qwen3-5-9B as primary model**

---

## Recommendations

### Immediate Fixes (Generator)
1. **Add deliverable assembly step** to SW5 template — concatenate state files into `outputs/deliverable.md`
2. **Fix save_to paths** — use `${output_dir}/state/` instead of `./state/` to prevent repo root pollution
3. **Add deliverable validation** to parity-check.sh — check for non-empty deliverable.md

### Model Strategy
1. **Primary model:** Qwen3-5-9B-Q4_K_M (proven 45/50 parity on P05 v4)
2. **Fast/budget alternative:** Ministral-3-3B-Instruct-2512-Q4_K_M (best quality/speed ratio among alternatives)
3. **Do not use:** Yi-6B-200K-Airo-Claude-Puffin (repetition loops, cannot follow multi-step instructions)
4. **Not recommended:** Falcon-H1-7B-Instruct (consistent but too low quality, slow)

### Future Testing
1. Re-run all 9 tests after fixing the deliverable assembly step
2. Test Ministral-3-3B with GPU-incremented variants (currently using gpu_layers=99)
3. Test Qwen3-5-9B on the same 3 prompts for direct comparison
4. Consider testing other top-5 stress test models (Falcon3-3B, LFM2-2.6B)

---

## Appendix A: Raw TSV Results

### Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M
```
prompt  model  config  steps  deliv_bytes  deliv_lines  refusals  parity_score  parity_verdict  duration_sec  sw1_quality
19      Yi-6B  full    20     0            0            0         10/50         FAIL            1344
06      Yi-6B  full    20     0            0            0         10/50         FAIL            3236
15      Yi-6B  full    20     0            0            0         10/50         FAIL            2442
```

### Falcon-H1-7B-Instruct-Q4_K_M
```
prompt  model   config  steps  deliv_bytes  deliv_lines  refusals  parity_score  parity_verdict  duration_sec  sw1_quality
19      Falcon  full    20     0            0            0         10/50         FAIL            2289
06      Falcon  full    20     0            0            0         10/50         FAIL            2657
15      Falcon  full    20     0            0            0         10/50         FAIL            2002
```

### Ministral-3-3B-Instruct-2512-Q4_K_M
```
prompt  model      config  steps  deliv_bytes  deliv_lines  refusals  parity_score  parity_verdict  duration_sec  sw1_quality
19      Ministral  full    20     0            0            0         10/50         FAIL            1963
06      Ministral  full    20     0            0            0         10/50         FAIL            1487
15      Ministral  full    20     0            0            0         10/50         FAIL            1637
```

## Appendix B: File Locations

| Artifact | Location |
|----------|----------|
| TSV Results | `docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests/results-*.tsv` |
| Per-Model Prompt Backups | `docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests/prompt-{N}-{model}/` |
| Batch Logs | `docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests/{model}-batch.log` |
| Gen Logs | `docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests/{model}-P{N}-gen.log` |
| Batch Script | `scripts/meta-v6/test-model-batch.sh` |
| Parity Check | `scripts/meta-v6/parity-check.sh` |

## Appendix C: Model Specifications

| Model | Params | Q4 Size | VRAM (gpu=99) | Tok/s | Context |
|-------|--------|---------|---------------|-------|---------|
| Yi-6B-200K-Airo-Claude-Puffin | 6B | ~3.5GB | ~4.5GB | ~15* | 32K |
| Falcon-H1-7B-Instruct | 7B | ~4.4GB | ~5.5GB | ~15 | 32K |
| Ministral-3-3B-Instruct-2512 | 3B | ~2.0GB | ~3.0GB | ~30 | 32K |
| Qwen3-5-9B (baseline) | 9B | ~5.6GB | ~6.5GB | ~12 | 32K |

*Yi-6B token rate appears higher but most tokens are repetition (garbage)
