# QA Findings: Speedup Optimization Iteration 1

**Date:** 2026-07-02
**Phase:** Ralph Loop Phase 1+2 (SPEEDUP-ANALYSIS.md implementation)
**Hardware:** AMD RX 580 8GB Vulkan (RADV), 6 CPU threads
**Baseline:** P05 cycle-3 run (52 min total, 5/11 refusals = 45% refusal rate)

## Iteration Summary

### Phase 1 Optimizations Attempted (5)

| # | Optimization | Status | Outcome |
|---|---|---|---|
| 1.1 | Anti-refusal prompt template | ✅ IMPLEMENTED | SW4 line 374 example rewritten, build-workflow.py synthesis step anti-refusal framing added. Expected to cut refusals 45% → 5%. |
| 1.2 | Speculative decoding (B3) | ❌ CRASHED | vk::DeviceLostError on RX 580 8GB (VRAM exceeded). Infrastructure kept for future 16GB GPU. |
| 1.3 | Qwen3-4B for SW1-SW3 (A1) | ❌ REVERTED | 4B per-token faster (30.6 tok/s vs 16.9 tok/s) but generates 5x more tokens (verbose). Net 20-30% slower wall-clock. |
| 1.4 | Skip Docker restart (C1) | ✅ IMPLEMENTED | `SKIP_DOCKER_RESTART=1` env var skips restart between phases. Saves ~25s/prompt. |
| 1.5 | Honest quality_score | ✅ IMPLEMENTED | Expanded refusal patterns 8→19, removed 2000-byte cap, logs matched pattern. |

### Phase 1 Net Effect

- **Speed:** Phase 1.4 saves ~25s/prompt. Phases 1.1+1.5 reduce wasted generation cycles (refusals now properly detected → no false 50/50 scores). Phase 1.3 would have helped but failed live.
- **Quality:** Phase 1.1 (anti-refusal) is the BIG quality win. Phase 1.5 prevents hidden refusals from scoring false PASS.
- **Hardware ceiling hit:** Spec decoding (1.2) and 4B model (1.3) both blocked by RX 580 8GB constraints. No path to 2x raw inference speedup on this hardware.

### Live Test: P05 v3 in progress

Started: 02:19 CDT
Configuration: all 9B (reverted), anti-refusal prompts, honest quality_score, SKIP_DOCKER_RESTART=1
Baseline comparison: P05 cycle-3 final (52 min, 45% refusal rate)

#### SW1 inner timings (9B baseline restored)

| Step | Duration | Tokens | Notes |
|------|----------|--------|-------|
| step_00_bootstrap | 9s | 28 | Model load |
| step_01_initial_breakdown | 57s | 916 | Task list generation |
| step_02_expand_high_complexity | ~70s | ~1180 | Subtask expansion |
| step_03_evaluate | 39s | ~600 | GWT criteria check |
| step_04_fix | TBD | TBD | Conditional on step_03 verdict |
| step_05_assemble_final | TBD | TBD | max_tokens 12288 |

(Updated as run progresses)

## What Did NOT Work

### Speculative Decoding (B3)

**Test:** Qwen3-1.7B-abliterated (1.1GB) as draft + Qwen3-5-9B-Q4 (5.3GB) main.
**Result:** `vk::DeviceLostError: vk::Queue::submit: ErrorDeviceLost` during first inference.
**Root cause:** RX 580 8GB VRAM exhausted. Breakdown:
- Main model: 5.3GB
- Draft model: 1.1GB
- Main KV cache (q8_0, 32k context): ~1GB
- Draft KV cache: ~0.5GB
- Vulkan runtime overhead: ~0.5GB
- **Total: ~8.4GB > 8GB**

**Librarian research confirmed Vulkan supports spec decoding** (llama.cpp issues #23126, #24492, #23544) — this is purely a VRAM ceiling issue, not a backend limitation.

**Infrastructure kept:** docker/entrypoint.sh + docker-compose.yml have LLAMA_SPEC_DRAFT_MODEL env vars. Will work on 16GB+ GPU.

### Qwen3-4B for SW1-SW3 (A1)

**Test:** SW1 step_01_initial_breakdown on Qwen3-4B-Instruct-2507 (2.4GB) vs Qwen3-5-9B (5.3GB).
**Result:** 4B 3x SLOWER wall-clock.

| Metric | 9B baseline | 4B test | Delta |
|--------|-------------|---------|-------|
| Wall-clock | 55s | 165s | **+200% slower** |
| Per-token rate | 16.9 tok/s | 30.6 tok/s | **+81% faster** |
| Tokens generated | 916 | 5039 | **+450% more verbose** |
| Tasks output | 11 | 21 | +10 (over-decomposed) |

**Root cause:** Qwen3-4B-Instruct-2507 is more verbose than 9B. Doesn't respect implicit conciseness norms. Per-token speedup erased by token bloat.

**Could fix with:** max_tokens cap (lose quality) or prompt tightening (extra iteration cost). Not worth it on this hardware — 9B is the sweet spot.

## What Worked

### Anti-refusal Template (Phase 1.1)

Identified 8 refusal-triggering patterns across 3 files via explore agent:
- **sw4-yaml-substructure-translation.yml line 374 (CRITICAL):** example used "Read {{step.prior_step.output}}" — taught LLM to emit "Read X" in generated prompts, triggering "I cannot read files" downstream.
- sw4 lines 240-241: "If reading prior step output" → "When using prior step output"
- sw4 file injection template: "Source content from injected file" → "Content provided below (injected via shell). The content above is already in your context — do NOT say you cannot read files."
- build-workflow.py line 362: "READ every prior step output" → "REVIEW every prior step output (content is already in your context above)"
- meta-workflow-v6.yml line 190: "Generated workflow preview:" → "Generated workflow preview (provided below via shell injection):"

**Expected impact:** 45% refusal rate → <10%. Live validation pending P05 v3 completion.

### Honest quality_score (Phase 1.5)

Old engine had 8 refusal patterns + 2000-byte cap. Missed:
- "I cannot directly" (we saw 6471-byte refusal matching this)
- "I cannot read"
- "do not have access"
- "cannot access your local"

Expanded to 19 patterns. Removed length cap. Now logs: `output is a REFUSAL (matched 'I cannot directly', 6471 bytes), setting quality_score=0.0`.

**Impact:** parity-check.sh will now correctly fail workflows with refusals instead of false PASS.

### Skip Docker Restart (Phase 1.4)

`SKIP_DOCKER_RESTART=1` env var added to run-prompt-end-to-end.sh. Saves ~25s per prompt. Across 20-prompt batch: 500s = ~8min saved.

## Open Questions for Phase 2

1. Will P05 v3 actually complete faster than 52min baseline? (Anti-refusal should reduce wasted generation cycles)
2. Will P05 v3 produce ZERO refusals? (If yes, anti-refusal validated)
3. How does P05 v3 deliverable quality compare vs opencode baseline? (No P05 opencode baseline exists; need P14 for head-to-head)

## Next Iteration (Phase 2)

- Phase 2.2: P05 v3 live run completion + measurement
- Phase 2.3: P14 head-to-head vs opencode baseline-14
- Phase 2.4: Update this doc with final numbers

## Commit Trail

- 7c74b63: Phase 1 optimizations (anti-refusal, 4B attempt, no-restart, quality_score)
- 8d5ce43: filter-name regex fix (Qwen3-5-9B → Qwen3-[45])
- 386942c: revert SW1/SW2/SW3 back to qwen35
