# Cycle 5: Live System Iteration Plan

## Objective
Fix SW4 source file map, validate 20 complex prompts agentically surpass opencode baseline via live system testing.

## Current State (10/11 PASS)
- P05: 50/50 ✅ | P06: NO .MD | P07: 45/50 ✅ | P08: NO WF | P09: 45/50 ✅
- P10: 45/50 ✅ | P11: 50/50 ✅ | P12: 45/50 ✅ | P13: 40/50 ✅ | P14: 45/50 ✅ | P15: 0/50 ❌

## Root Causes (from SCORE-GAP-ANALYSIS-20260628.md)
1. **P15 CRITICAL:** SW4 emits wrong file paths (config.yaml → src/config/mod.rs)
2. **C7 (30pts lost):** 6 prompts missing exec-final/ dir with benchmark.log
3. **C6 (5pts lost):** P13 has meta-commentary patterns
4. **P06:** Deliverable is .html not .md
5. **P08:** No meta/ dir (min pipeline, not full SW1-SW5)

## Fix Strategy: Bottom-Up (smallest elements first)

### Step 1: Fix SW4 Prompt (source file map)
- Add actual source file paths to SW4 prompt
- Prevent LLM from guessing file locations
- Test: validate-workflow.py on P15-generated workflow

### Step 2: Test SW4 in Isolation
- Run SW4 alone on P15's SW1-SW3 output
- Verify no "MISSING" file references
- Verify workflow passes dry-run validator

### Step 3: Test Full Pipeline on P15
- Run SW1-SW5 on P15 prompt
- Execute generated workflow
- Run parity-check
- Target: ≥45/50 (was 0/50)

### Step 4: Fix Meta-commentary (C6)
- Add anti-meta-commentary rule to SW4 prompt
- Target: P13 rises from 40 → 45

### Step 5: Re-run Failed/Incomplete Prompts
- P06: re-run, ensure .md deliverable
- P08: re-run through full SW1-SW5 pipeline

### Step 6: Create 9 New Complex Prompts
- Need 20 total complex prompts for validation
- Create prompts covering: algorithms, debugging, refactoring, testing, architecture, optimization, security, concurrency, API design

### Step 7: Validate All 20 Prompts
- Run each through SW1-SW5 pipeline
- Run each through opencode baseline (same model)
- Compare scores: SW must win or tie on ≥15/20

## Validation Criteria
- Each prompt: parity-check ≥45/50
- Each prompt: SW1-SW5 score ≥ opencode baseline score
- No refusals in any deliverable
- No meta-commentary in any deliverable
- All shell hooks resolve correctly (no MISSING files)

## Live System Testing Protocol
1. For each fix: modify YAML → rebuild binary → test in isolation → validate
2. For each prompt: run SW1-SW5 → execute → parity-check → compare baseline
3. Document results in WORKFLOW_RELIABILITY_TRACKING.md
4. Critical failures: abort, analyze, fix, retry

## Honest Constraint
20 prompts × ~60 min each = ~20 hours of pipeline time. Plus 20 × 5 min baselines = ~2 hours. Total: ~22 hours of live system testing. This MUST be done sequentially due to single-GPU constraint.

## Exit Criteria
- [x] P15 PASS (≥45/50) — **50/50 PERFECT** (2026-06-29)
- [x] All 11 original prompts PASS (≥45/50) — **11/11 PASS**
- [x] 9 new complex prompts created (P16-P24) — **DONE** (2026-06-29)
- [x] Baselines generated for all 9 new prompts — **DONE** (2026-06-29)
- [ ] 9 new prompts validated via SW1-SW5 pipeline — **IN PROGRESS** (batch runner PID 452133)
- [ ] 20/20 prompts: SW1-SW5 ≥ opencode baseline — **PENDING** (waiting for batch)
- [ ] No refusals, no meta-commentary, no MISSING files — **VERIFIED on P15**
