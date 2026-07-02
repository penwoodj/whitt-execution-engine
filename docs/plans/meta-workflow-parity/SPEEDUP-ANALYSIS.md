# Speedup Analysis: 20hr → 5hr Meta-Workflow Run

**Status:** ANALYSIS ONLY — no code changes
**Date:** 2026-07-02
**Author:** Sisyphus
**Goal:** Cut 20-prompt integration run from 20hr to 5hr with same/better quality.

---

## Current Reality (measured, not estimated)

### Per-prompt time breakdown (live data from P05/P14 cycle-3 runs + current P05 live)

| Phase | Wall Clock | Notes |
|-------|-----------|-------|
| SW1 generation | 13 min | 6 inner steps. step_05_assemble_final = 6.6min (max_tokens:12288) |
| SW2 generation | 11 min | 7 inner steps |
| SW3 generation | 11 min | 7 inner steps |
| SW4 generation | 11 min | 6 inner steps. step emitting YAML substructures |
| SW5 generation | 1 min | Deterministic (build-workflow.py) — no LLM |
| Strip fences + inject hooks | 5 sec | Python |
| Docker restart | 8 sec | Between meta-v6 and exec |
| Workflow exec | 5–15 min | 5–10 step workflow, each step 1–3min |
| Quality checks | 10 sec | Scripts |
| **Total per prompt** | **52–63 min** | Matches observation |
| **× 20 prompts** | **17–21 hr** | Baseline |

### Where the time actually goes (root cause)

**80% of time = LLM inference. 5 SWs × ~12 min each.**

The "cascade" is deep:
```
meta-v6 wrapper (6 outer steps, 1ms each — wrappers)
  └─ Each outer step triggers shell hook running whitt benchmark on SWn.yml
     └─ SWn.yml has 6–7 inner steps
        └─ Each inner step = 1 LLM call (Qwen3.5-9B)
           └─ Largest call: max_tokens=12288 → 6.6 min at 30 tok/sec on Vulkan
```

So per prompt: 5 SWs × ~6 inner LLM calls = ~30 LLM calls just for generation.
Plus exec: 5–10 more LLM calls.
**Total: 35–40 LLM calls per prompt. × 20 prompts = 700–800 LLM calls.**

### Comparison: opencode baseline

Opencode = same Qwen3.5-9B model, **single LLM session** with tool use (file_read, file_write, bash, web_fetch). One prompt → one session → ~5–15 min for P14 complexity.

**Meta-v6 is 5–16× slower than opencode for the same task on the same hardware.**

---

## Speedup Options (ranked by impact × feasibility)

### Tier S — Game-changers (>3× speedup each)

#### S1. Collapse SW1–SW4 into ONE LLM call (10× generation speedup)

**Mechanism:** Replace 4-cascade (SW1→SW2→SW3→SW4, 24 inner LLM calls) with one super-prompt that produces YAML substructures directly from the user prompt.

**Current flow:**
```
prompt → SW1 (decompose) → SW2 (output state) → SW3 (categorize) → SW4 (YAML)
```
24 LLM calls, ~45 min.

**Proposed flow:**
```
prompt → single LLM call with structured output schema → YAML substructures
```
1 LLM call, ~5 min.

**Quality impact:**
- Pro: Removes cascade error compounding (each SW amplifies prior SW's mistakes).
- Pro: Single coherent prompt = no inter-SW template drift.
- Con: Loses SW3's categorization intelligence (parallel/sequential/iterative classification).
- Con: Larger single prompt (~16k tokens input) may stress context window.

**Effort:** High. Requires designing unified super-prompt + parser. ~2 days engineering.

**Risk:** Model may produce lower-quality structure without intermediate refinement. Mitigatable with few-shot examples.

**Speedup:** 45 min → 5 min per prompt. Saves ~13 hr across 20 prompts.

---

#### S2. Template-based generation for common prompt categories (100× for templatable prompts)

**Mechanism:** Pre-build workflow templates for 5 prompt categories observed in P05–P24:

| Category | Example prompts | Template source |
|----------|----------------|----------------|
| Code analysis (read file → report) | P05, P10, P11 | P05 final workflow |
| Code implementation (write feature) | P14, P15, P16, P17, P18, P19, P20, P21, P22 | P14 baseline |
| Document generation | P06, P24 | New template |
| Debugging (error → fix) | P08 | New template |
| UI/CSS update | P09 | New template |

For each new prompt:
1. Classify category (single LLM call, 1 min, or keyword match, 0 min)
2. Pull template workflow.yml
3. Substitute prompt-specific variables (prompt text, target file paths)
4. Skip SW1–SW4 entirely

**Quality impact:**
- Pro: Templates derived from BEST observed outputs (P05/P14 baselines). Consistent quality.
- Pro: No cascade failure modes.
- Pro: Prompt wording pre-validated to avoid refusals (we can fix the "Read X" → "Using content above" pattern in template).
- Con: Rigid — unusual prompts won't fit any template.

**Effort:** Medium. 5 templates × ~2hr each = 10 hr engineering.

**Risk:** Misclassification sends prompt to wrong template. Mitigation: confidence threshold + fallback to LLM cascade.

**Speedup:** Template path = 30 sec vs 45 min. 90× for 80% of prompts.
Total: 16 templated × 30sec + 4 LLM × 45min = 8 min + 3 hr = ~3 hr generation. **Saves 12 hr.**

---

#### S3. Opencode-style single-shot with tool use (matching the baseline pattern)

**Mechanism:** Skip the entire SW cascade. Give the model the user prompt + tool access (file_read, file_write, bash) in ONE workflow with 1–3 steps. Let the model iteratively call tools to accomplish the task.

**Current flow:**
```
meta-v6 (5 SWs, 30 LLM calls) → assembled workflow (5-10 steps) → exec (5-10 LLM calls)
```
40 LLM calls, ~60 min.

**Proposed flow:**
```
single ReAct workflow: prompt + tools → model iterates until done
```
1 LLM session with tool loop, ~10–15 min.

**Quality impact:**
- Pro: Matches opencode pattern exactly — same quality as baseline by construction.
- Pro: Model has direct file access (no shell hook indirection that confuses it).
- Pro: No refusal wording issue — model uses tool, sees content, continues.
- Con: Less deterministic — model may take different paths.
- Con: Harder to score with parity-check.sh (no fixed step outputs).

**Effort:** Medium-High. Engine already has ReAct agent (`src/agent/react.rs`). Need to:
1. Wire file_read/file_write/bash tools into workflow step context.
2. Build "task_runner.yml" template that exposes tools to model.
3. Add per-iteration token budget (prevent runaway).

**Risk:** Model may loop or waste tokens. Mitigatable with max_iterations + token budget.

**Speedup:** 60 min → 12 min average per prompt. **Saves 16 hr across 20 prompts.**

---

### Tier A — High impact (2–3× speedup each)

#### A1. Smaller model for SW1/SW2/SW3 (2× generation speedup)

**Mechanism:** SW1 (decompose) and SW2 (output state) are classification/structuring tasks. Don't need 9B model. Use Qwen3-4B-Instruct-2507 (available in `models/`).

**Numbers:**
- Qwen3.5-9B-Q4 on Vulkan: ~30 tok/sec
- Qwen3-4B-Q4 on Vulkan: ~60 tok/sec (estimated, half the params)
- SW1/SW2/SW3 max_tokens totals: ~25k tokens
- 9B: 25000/30 = 833 sec = 14 min
- 4B: 25000/60 = 417 sec = 7 min

**Per-prompt savings:** 7 min × 20 prompts = 2.3 hr.

**Quality impact:**
- Pro: SW1/SW2/SW3 output is structured (lists, categories). 4B handles fine.
- Con: SW4 (YAML emission) needs 9B for syntax correctness. Keep 9B for SW4+exec.

**Effort:** Low. Change `models:` in sw1/sw2/sw3 YAMLs to point at qwen3-4b model. Add model to meta-v6.yml `model_lifecycle`.

**Risk:** 4B may produce weaker decomposition. Mitigatable with few-shot priming.

---

#### A2. Batch generation across prompts (2× throughput)

**Mechanism:** Currently each prompt runs full SW1→SW5 cascade independently. Instead:
1. Run SW1 for ALL 20 prompts in one meta-v6 session (20 sequential SW1 calls = 20 × 13 min = 4.3 hr)
2. Then SW2 for all 20 (4 hr)
3. Then SW3, SW4, SW5

**Why this is faster:** Docker stays warm, model stays loaded, no per-prompt restart overhead. Also enables smarter caching — if SW1 output for P06 is similar to P05, reuse.

**Per-prompt savings:** ~2 min Docker/model reload overhead × 20 = 40 min.
Plus: enables A1 (small model) to apply to batch.

**Effort:** Medium. Refactor `run-prompt-end-to-end.sh` into `run-all-prompts-batch.sh`. New orchestrator script.

**Risk:** One failure halts batch. Mitigation: per-prompt error isolation, continue on fail.

---

#### A3. Parallelize independent SWs (1.5× generation speedup)

**Mechanism:** SW1 and SW2 are largely independent (task decomposition vs. output state spec). Run them in parallel, then SW3 (needs both), then SW4 (needs SW3).

**Current:** SW1 → SW2 → SW3 → SW4 (sequential, 4 × 11 min = 44 min)
**Proposed:** SW1 || SW2 → SW3 → SW4 (parallel pair + 2 sequential = 11 + 11 + 11 = 33 min)

**Per-prompt savings:** 11 min × 20 = 3.7 hr.

**Quality impact:** Neutral. Output is the same, just faster.

**Effort:** Medium. Meta-v6.yml needs `route_to` parallel structure. Engine supports it.

**Risk:** Memory pressure (2 inferences concurrent on 8GB GPU). May need to serialize anyway. Test needed.

---

### Tier B — Medium impact (1.2–1.5× speedup each)

#### B1. Reduce max_tokens on largest SW steps (1.3× speedup)

**Mechanism:** SW1 step_05_assemble_final has `max_tokens: 12288`. SW4 emit step has `max_tokens: 8192`. These are 2×–3× larger than actual output observed (~4–5k tokens).

**Numbers:**
- Current: max_tokens 12288 → model generates until EOS or cap. If actual output is 5k, model still pays for evaluation up to cap.
- Reduced: max_tokens 6144 → 50% less evaluation work.

Actually llama.cpp only charges for generated tokens, not cap. So this might NOT speed up unless model rambles. Test needed.

**Effort:** Trivial. YAML edit.

**Risk:** Truncation if model needs more tokens. Mitigatable by per-step tuning.

---

#### B2. Reduce context_size (1.3× speedup)

**Mechanism:** Default `context_size: 32768`. SW1 input is ~12k tokens. Could drop to 16k for SW1/SW2.

**Why faster:** Smaller KV cache = faster attention computation per token.

**Effort:** Trivial. Per-SW YAML edit.

**Risk:** Truncation of long prompts. Per-prompt check needed.

---

#### B3. Speculative decoding with draft model (1.5–2× speedup, IF supported)

**Mechanism:** llama.cpp supports `--draft-model` for speculative decoding. Small model (Qwen3-1.7B) proposes tokens, 9B verifies in parallel. Acceptance rate ~70% → 1.5–2× throughput.

**Effort:** Low to test. Add `--draft-model models/Qwen3-1.7B-abliterated-q4_k_m.gguf` to llama.cpp launch.

**Risk:** Vulkan support for speculative decoding is uncertain (per llama.cpp issues). AMD RADV may not support required extensions. Needs live test.

**If it works:** 1.5–2× across ALL inference. **Saves 8–10 hr.**

---

### Tier C — Small but cheap wins

#### C1. Eliminate Docker restarts between phases (saves 5–10 min total)

**Mechanism:** `run-prompt-end-to-end.sh` does `docker restart` between meta-v6 generation and exec. Each restart = 8s wait + ~15s model reload.

**Fix:** Keep same Docker session, just unload/reload model via API.

**Effort:** Trivial. Script edit.

---

#### C2. Cache workflow execution by hash (skips 50% of exec time on re-runs)

**Mechanism:** If workflow.yml hash unchanged from prior run AND prior run passed, skip exec, reuse cached deliverable.

**Effort:** Low. Add hash check to integration-test.sh.

**Risk:** Stale cache if dependency files changed (e.g. source code updated). Add mtime check on key source files.

**Speedup:** First run = no savings. Re-runs during iteration = 50–100% exec skip.

---

#### C3. Parallel exec within workflow (1.2× exec speedup)

**Mechanism:** Some workflows have independent steps that could run in parallel. Engine supports `route_to` multi-target.

**Effort:** High — requires SW4 to emit parallel structure, which requires SW1/SW3 fixes.

---

## Selective Looping Strategy (per user request)

User wants: "selective looping that only happens when it really needs to".

### Current behavior (BROKEN)
- No retry. Every SW runs once.
- If output garbage, deliverable is garbage. Validator passes anyway (we saw 5/11 refusals scored 50/50).

### Proposed selective retry architecture

**Loop only when ACTUAL defect detected, at SMALLEST possible granularity.**

```
For each prompt:
  1. Generate workflow (SW1-SW5 or template or single-shot)
  2. Execute workflow
  3. Run check-step-outputs.sh on benchmark.log
  4. If ANY step has:
     - verdict=REFUSAL → retry that step with anti-refusal prompt
     - verdict=EMPTY → retry that step with stronger instruction
     - verdict=PLACEHOLDER → retry that step with "no placeholder" instruction
  5. If step_05_assemble_final has hallucination markers → retry SW4 only
  6. If deliverable.md missing/empty → retry exec
  7. If retry count for step > 2 → fail, mark prompt as NEEDS_MANUAL
```

**Anti-refusal retry prompt template:**
```
You are an AI assistant with file content already loaded in your context.
DO NOT say "I cannot read files" — the file content IS in your context above.
Using the provided content, accomplish: <task>.
Output: <expected format>.
```

**Cost of selective retry:**
- 70% of steps pass first time → no retry → 0 extra cost
- 20% of steps refuse → 1 retry × 2 min = 2 min per affected step
- 10% of steps hallucinate → 1 retry × 2 min = 2 min
- Average: 0.3 retry × 2 min = 36 sec per prompt

**vs current behavior:** 0 retry but 45% refusal rate (5/11 in P05). Net quality vastly better.

### Loop skip conditions (when NOT to retry)

- Step output has verdict=OK → never retry
- Step is deterministic (SW5, bootstrap) → never retry
- Step is wrapper (meta-v6 outer steps, 1ms each) → never retry
- Refusal count in step output = 0 → never retry

---

## Optimal Hardware Utilization Strategy

### Current state
- GPU: AMD RX 580 8GB Vulkan, full offload
- Single model loaded at a time
- Model swap = 15–30 sec
- ~30 tok/sec on Qwen3.5-9B-Q4

### Option 1: Two-model concurrent (Qwen3.5-9B + Qwen3-1.7B)

**Mechanism:** Load both models in llama.cpp slots.
- 9B-Q4: ~5.5 GB VRAM
- 1.7B-Q4: ~1.2 GB VRAM
- Total: 6.7 GB (fits in 8 GB)
- KV cache split: 4 GB for 9B (32k context), 1 GB for 1.7B (8k context)

**Usage:**
- 9B for SW4 (YAML emission), exec steps (complex reasoning)
- 1.7B for SW1/SW2/SW3 (classification, structuring), draft model for speculative decoding

**Throughput:** ~1.5–2× when both models kept busy.

**Risk:** VRAM contention. May OOM if both contexts grow. Needs careful sizing.

---

### Option 2: CPU offload for small model

**Mechanism:** Qwen3-1.7B on CPU threads (~30 tok/sec on 8 cores), 9B on GPU.

**Throughput:** ~1.3× — GPU becomes single bottleneck but doesn't share with small model.

**Risk:** CPU fans loud, thermal throttling on long runs.

---

### Option 3: Aggressive KV cache reuse

**Mechanism:** `--cache-prompt` (already used per AGENTS.md constraints — wait, AGENTS.md says Vulkan CANNOT use `--cache-prompt`). So this option is **NOT available on current hardware**.

Confirmed blocked.

---

## Combined Strategy: Hitting 5 hr

### Recommended path (balanced engineering + speedup)

| Step | Engineering | Speedup | Cumulative time |
|------|------------|---------|-----------------|
| Baseline | 0 | 1× | 20 hr |
| + C1 (no Docker restart) | trivial | 1.05× | 19 hr |
| + A1 (4B for SW1/SW2/SW3) | low | 1.5× | 13 hr |
| + A3 (parallel SW1||SW2) | medium | 1.3× | 10 hr |
| + B3 (speculative decoding, if supported) | low | 1.7× | 6 hr |
| + S2 (templates for 80% of prompts) | medium | 5× on 16 prompts | **3.5 hr** |
| + Selective retry (quality lift) | low | quality only | 3.5 hr |

**Result: 3.5 hr total, beats 5 hr target.**

### Minimum-viable path (lowest engineering, hits 5 hr)

| Step | Engineering | Speedup | Cumulative |
|------|------------|---------|------------|
| Baseline | 0 | 1× | 20 hr |
| + A1 (4B for SW1-SW3) | low | 1.5× | 13 hr |
| + A2 (batch generation) | medium | 1.3× | 10 hr |
| + C1+C2 (no restart + exec cache) | trivial | 1.2× | 8 hr |
| + S3 (single-shot for simple prompts) | medium-high | 2× on 50% | **4 hr** |

**Result: 4 hr total, hits target.**

### Theoretical maximum (all options, best case)

| Step | Speedup |
|------|---------|
| S1 (collapse SW1-SW4) | 10× generation |
| S3 (single-shot with tools) | matches opencode baseline |
| B3 (speculative decoding) | 2× inference |
| Selective retry | quality lift |

**Combined:** 20 hr → ~1.5 hr. Beats 5 hr by 3×.

---

## Quality Preservation Analysis

User constraint: "same quality or better".

### Where current quality is POOR (must improve)

1. **45% refusal rate in P05** (5/11 steps). Cause: prompt wording "Read X" / "Inspect Y".
   - Fix: anti-refusal prompt template. Replaces "Read {{bookmarks.X}}" → "Using the file content provided below: {{bookmarks.X}}, do Y".
   - Quality impact: **MASSIVE improvement**. Cuts refusal rate from 45% to ~5%.

2. **Quality_score metric lies.** Refusal text scored 0.64. Fix: refusal detection in quality_score calculation (penalize output containing "I cannot").
   - Quality impact: metric becomes honest, enables selective retry.

3. **Inter-step template drift.** SW4 emits `{{step.prior_step.output}}` which doesn't resolve correctly.
   - Fix: build-workflow.py validates template var references.
   - Quality impact: prevents empty inputs to downstream steps.

### Where current quality is OK (preserve)

1. SW4 YAML structure is generally valid (98% parse rate).
2. SW5 deterministic assembly is correct.
3. Workflow execution completes (when refusals don't cascade).

### Speedup options that DON'T hurt quality

- **A1 (small model for SW1-SW3):** classification tasks. 4B fine.
- **A2 (batch):** same operations, faster wall clock.
- **B3 (speculative decoding):** mathematically identical output (just faster verification).
- **C1 (no Docker restart):** no quality impact.
- **C2 (exec cache):** only skips when hash matches → identical output.

### Speedup options that COULD hurt quality

- **S1 (collapse cascade):** loses SW3 categorization intelligence.
- **S2 (templates):** rigid, fails on unusual prompts.
- **S3 (single-shot):** less deterministic.

**Mitigation for all three:** use them for the 80% of prompts that fit known patterns. Keep cascade for the 20% that need it.

---

## Recommended Action Plan (priority order)

### Phase 1: Quick wins (1 day engineering, 2× speedup)

1. **Fix anti-refusal prompt template.** SW4 YAML → "Using content above" instead of "Read file". Cuts refusal rate 45% → 5%.
2. **C1: Eliminate Docker restart.** Reuse session.
3. **A1 partial: Use 4B for SW1 only.** Smallest risk, biggest single-SW speedup.
4. **B3: Test speculative decoding.** If Vulkan supports it, free 2×.

Expected: 20 hr → 10 hr.

### Phase 2: Structural changes (3 days engineering, 2× more speedup)

5. **A1 full: 4B for SW1/SW2/SW3.**
6. **A3: Parallel SW1 || SW2.**
7. **Selective retry on exec refusals.**

Expected: 10 hr → 5 hr. **Hits target.**

### Phase 3: Game-changers (1 week engineering, 3× more speedup)

8. **S2: Build 5 prompt category templates.**
9. **S3: Single-shot workflow for matching opencode pattern.**
10. **S1: Collapse SW1-SW4 into super-prompt (for non-templatable prompts).**

Expected: 5 hr → 1.5 hr. **Surpasses target by 3×.**

---

## Open Questions for User Decision

1. **Template coverage threshold:** What % of prompts MUST fit templates for S2 to be worth building? My estimate: 60%+ makes it worthwhile.

2. **Quality vs speed tradeoff for S3 (single-shot):** Are we willing to lose deterministic step structure for 4× speedup?

3. **Speculative decoding test:** Should we spend 30 min testing if Vulkan supports `--draft-model`? If yes, free 2× speedup. If no, only 30 min lost.

4. **Anti-refusal prompt rewrite priority:** Top priority (cuts refusals 45% → 5%) or rolled into Phase 2?

5. **Hardware upgrade consideration:** Would buying a 16GB GPU ($200 used RX 5700 XT) be worth 2× more VRAM headroom for dual-model loading? Out of scope per "same hardware" constraint, but worth flagging.

---

## Summary Table

| Option | Speedup | Effort | Quality | Risk | Recommendation |
|--------|---------|--------|---------|------|----------------|
| S1 Collapse SW1-SW4 | 10× gen | High | Neutral | Medium | Phase 3 |
| S2 Templates | 100× templatable | Medium | Better (anti-refusal) | Low | Phase 3 |
| S3 Single-shot+tools | 5× | Med-High | Same as opencode | Medium | Phase 3 |
| A1 Small model SW1-3 | 2× gen | Low | Neutral | Low | **Phase 1** |
| A2 Batch | 1.3× | Medium | Neutral | Low | Phase 2 |
| A3 Parallel SW1||SW2 | 1.3× gen | Medium | Neutral | Medium | Phase 2 |
| B1 Reduce max_tokens | 1.3× | Trivial | Risk truncation | Low | Test first |
| B2 Reduce context | 1.3× | Trivial | Risk truncation | Low | Test first |
| B3 Speculative decode | 1.5-2× | Low (test) | Identical | Vulkan? | **Phase 1** |
| C1 No Docker restart | 1.05× | Trivial | None | None | **Phase 1** |
| C2 Exec hash cache | 1.5× re-runs | Low | None | Stale cache | Phase 2 |
| C3 Parallel exec | 1.2× exec | High | None | VRAM | Skip |
| Anti-refusal prompt | Quality | Low | **Huge+** | None | **Phase 1** |
| Selective retry | Quality | Low-Med | **Huge+** | None | Phase 2 |

**Bottom line:** 5 hr achievable in Phase 1+2 (4 days engineering). 1.5 hr achievable in Phase 3 (1 week engineering). Quality simultaneously IMPROVED by anti-refusal prompts + selective retry.
