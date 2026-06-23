# CYCLE 3 — EXEC HARNESS + GENERATOR FIX PLAN

**Created:** 2026-06-22
**Status:** DRAFT (awaiting self-audit)
**Mode:** Caveman (token-efficient)
**Predecessor:** cycle-2-final-report.md (retracted — claims invalid)

## BRUTAL HONESTY SECTION

Cycle 2 claim "11/11 PASS" = FRAUD. Real state:
- Generated workflows have BROKEN YAML INDENT (steps at 14 spaces, should be 4)
- Duplicate `prompt:` keys everywhere (3-25 per file)
- `whitt benchmark --workflow X` falls back to benchmark-only mode when YAML invalid
- NO workflow steps actually executed in "exec phase"
- Output files = benchmark boilerplate, not workflow save_to targets
- Scoring script gave 5/5 for boilerplate existence → false positives

## OBJECTIVE (PRIMARY, MUST COMPLETE)

Meta-workflow-v6 generator + execution engine MUST produce workflows that:
1. Parse as valid YAML (no duplicate keys, correct indent)
2. Execute steps when fed to `whitt benchmark --workflow X`
3. Each step's `save_to` writes real output files
4. Output content accomplishes prompt objective (no refusals)
5. Quality ≥ opencode baseline on same prompt with max tool usage

## SUB-OBJECTIVES (DECOMPOSED)

### SO-1: Workflow YAML Structure Validity

**Goal:** All generated SW5 workflows pass strict YAML schema validation.

**Validation:**
- `python3 -c "import yaml; yaml.safe_load(open('X.yml'))"` exits 0
- Custom duplicate-key detector (per Q3 audit script) returns 0 duplicates
- `whitt workflow X.yml --show-config` exits 0 (uses engine's own parser)
- All steps at 4-space indent under `agentic_workflow.steps:` mapping

**Success threshold:** 11/11 prompts produce structurally valid workflows.

### SO-2: Step Execution Verification

**Goal:** `whitt benchmark --workflow X.yml` actually executes X's steps.

**Validation:**
- exec.log contains `executing step step_XX_*` lines (count > 0)
- exec.log contains `step_name=step_XX_* duration_ms=*` lines
- exec/outputs/ contains real save_to target files (NOT benchmark boilerplate)
- Each output file >500 bytes (real content, not empty)

**Success threshold:** 11/11 prompts produce step execution evidence.

### SO-3: Output Content Quality

**Goal:** Workflow outputs contain real task-accomplishing content.

**Validation:**
- Zero refusal patterns ("I cannot access", "As an AI", etc.) in outputs
- Each output file has substantive content (not just headers/boilerplate)
- Sample inspection: 3+ output files per prompt contain task-relevant analysis/code

**Success threshold:** Avg output content quality ≥ opencode baseline (manual comparison).

### SO-4: Opencode Baseline Parity

**Goal:** Meta-v6 outputs ≥ opencode single-shot attempts.

**Validation:**
- For each prompt: generate opencode baseline (manual, save artifacts)
- For each prompt: run meta-v6 → execute workflow → save artifacts
- Side-by-side comparison: content depth, accuracy, completeness
- Score: 0 (worse), 1 (equal), 2 (better) per prompt per criterion

**Success threshold:** 8/11 prompts score ≥1 on all criteria.

### SO-5: Engine Hardening

**Goal:** Engine fails fast + loudly on invalid workflow YAML.

**Validation:**
- Invalid YAML → clear error message + non-zero exit (not silent fallback)
- Source code change in `src/benchmark/runner.rs` workflow load path
- Unit test: invalid YAML → Err, valid YAML → Ok

**Success threshold:** No more silent benchmark-only fallback.

## ROOT CAUSES (CYCLE 0-2 + NEW)

### RC1: Generator YAML indent broken (NEW, CRITICAL)

SW5 produces steps at 14-space indent instead of 4. Each step's `prompt:` becomes duplicate top-level key under `steps:` mapping.

**Fix approach:**
- Option A: Improve SW5 prompt template to enforce correct indent (fragile)
- Option B: Add fix-yaml.py rule that re-indents entire `steps:` block (robust)
- Option C: Engine YAML parser uses `duplicate_keys=policy` (accept) — bad, hides bug

**Recommendation:** B + A (defense in depth)

### RC2: fix-yaml.py rules insufficient

Current 13 rules don't catch duplicate keys or massive indent issues.

**Fix:** Add 2 new rules:
- `fix_step_indent`: re-indent step_name + body to canonical 4/6/8 levels
- `fix_duplicate_top_keys`: detect + remove duplicate top-level keys (keep first)

### RC3: Engine silent fallback

When `whitt benchmark --workflow X` YAML parse fails, it logs WARN + falls back to benchmark-only mode. No non-zero exit. User thinks workflow ran.

**Fix:** Promote parse failure to FATAL. Exit non-zero. Print clear error.

### RC4: No actual opencode baselines (per Q6 audit)

5 assumptions still UNVERIFIED. No actual opencode runs for comparison.

**Fix:** Run all 11 prompts in opencode manually. Save outputs as ground truth.

### RC5: Scoring script gave false positives

`parity-check.sh` C6 criterion just checks "any file >100 bytes exists in exec_dir". Benchmark boilerplate passes.

**Fix:** Tighten C6 to check `exec/outputs/` subdir specifically + content patterns (not benchmark_*.yml).

## PLAN FILE STRUCTURE

Per user: "sub-workflows split up in a way that is easy to debug"

### Test infrastructure split

```
scripts/meta-v6/
├── cycle-3/
│   ├── 01-validate-yaml.sh       # SO-1: strict YAML validation
│   ├── 02-execute-workflow.sh    # SO-2: run + verify steps executed
│   ├── 03-inspect-outputs.sh     # SO-3: content quality check
│   ├── 04-compare-baseline.sh    # SO-4: vs opencode baseline
│   ├── 05-run-opencode-baseline.sh  # Generate baselines
│   └── lib/
│       ├── duplicate-keys.py     # Strict duplicate key detector
│       ├── indent-fixer.py       # Re-indent broken YAML
│       └── content-analyzer.py   # Output quality scorer
├── fix-yaml.py                   # Existing (extend with indent rule)
└── parity-check.sh               # Existing (tighten C6/C7 criteria)
```

### Generator split

Per "split up sub workflows" — SW1-SW5 already split. Keep.

Add per-step debug visibility:
- SW5 step_03_assemble_steps: log raw model output BEFORE save_to
- SW5 step_06_finalize: log fixed YAML + validation result

## ITERATION STRATEGY

### Fast iteration loop (per prompt)

```
1. Run meta-v6 on prompt (~45min generation)
2. Run cycle-3/01-validate-yaml.sh (1s)
3. If invalid: improve fix-yaml.py + retry (1min)
4. Run cycle-3/02-execute-workflow.sh (~5min)
5. Inspect outputs (1min)
6. If failures: improve SW5 prompt + retry from step 1
```

Target: 3 iterations max per prompt.

### Strategic testing order

Test in this order (small → complex):

1. **P14 first** (parallel inference, hand-crafted workflow exists as reference)
2. **P05 second** (similar topic, baseline available)
3. **P11 third** (clone verification, simpler structure)
4. Then remaining prompts

## CRITICAL REVIEW CHECKLIST (SELF-AUDIT)

Before executing plan, verify:

- [ ] Each SO has measurable success threshold?
- [ ] Each RC has concrete fix approach?
- [ ] Iteration loop bounded (max N per prompt)?
- [ ] Test scripts small + composable?
- [ ] Logging sufficient for debugging?
- [ ] Engine hardening explicitly addressed?
- [ ] Opencode baselines generation planned?
- [ ] No silent failure paths remaining?
- [ ] Caveman style maintained?
- [ ] Validation criteria falsifiable (can fail)?

## WEB RESEARCH PAUSES SCHEDULED

Per Q6 fix (research was planned but never done):

1. **Before Cycle 3 start:** Verify whitt execution path for workflows
   - Search: `whitt benchmark workflow execution steps`
   - Test: Does `whitt workflow X.yml` execute or just validate?
2. **After RC1 fix:** Verify YAML indent canonical form
   - Reference `meta-workflow-v6.yml` line 51-60 for correct indent
3. **After RC3 fix:** Verify engine error handling patterns
   - Search Rust idioms for fatal-vs-warn workflow errors

## SUCCESS CRITERIA (OVERALL)

Cycle 3 complete when ALL true:

1. 11/11 prompts → valid YAML workflows (no duplicate keys)
2. 11/11 prompts → workflow steps execute (evidence in logs)
3. 11/11 prompts → save_to writes real output files
4. 8/11 prompts → output quality ≥ opencode baseline
5. Engine hardening: invalid YAML = fatal exit
6. Plan file validated against checklist above
7. Honest limitations documented (no fraud)

## EXIT CONDITIONS

- ✅ All success criteria met → promise DONE
- ❌ After 3 iterations on any single prompt without progress → escalate to user
- ❌ Hardware crash (over-resource use) → pause + ask user

## COMMIT STRATEGY

- One commit per SO completion
- Conventional commits: `fix:`, `feat:`, `test:`, `docs:`
- Co-Authored-By: Claude

## ANTI-FRAUD GUARDRAILS

Per cycle-2 fraud discovery:

1. **Never claim PASS without exec.log evidence** — grep for step events
2. **Never claim "outputs substantive" without reading samples**
3. **Never claim parity without opencode baseline artifacts saved**
4. **Cross-check workflow_id uniqueness per prompt** — no stale copies
5. **Self-audit each claim against Q1-Q10 critical lens**
