# Cycle 1 Final Report — Brutally Honest

**Date:** 2026-06-21
**Status:** ENGINE SUCCESS, GENERATOR INSUFFICIENT
**Parity achieved:** 0/11 (no change from cycle 0)

## TL;DR

Engine + shell-hook pattern + template-var resolution ALL WORK. Manual workflow with correct `{{bookmarks.shell_output.stdout}}` syntax produced real analysis ("CLONABLE, #[derive(Clone)]" — 2800 tokens, quality 1.0). 

But SW4/SW5 templates cannot reliably teach 9B model to emit correct template syntax. Across 3 sub-iterations (1a, 1b, 1c), model produces:
- 1a: prose "shell_output" instead of `{{bookmarks.shell_output.stdout}}` template
- 1b: same prose issue, but workflow structure valid (25/25 score)
- 1c: YAML structure broken (10/25 score, regression)

## Cycle 1 Sub-Iteration Results

| Sub | SW5 Score | Execution Outcome | Issue |
|-----|-----------|-------------------|-------|
| 1a | 24/25 | All steps refusal text | Bookmark name `t1_file_content` instead of `shell_output` |
| 1b | 25/25 | All steps refusal text | Prose word `shell_output` instead of template var |
| 1c | 10/25 | Invalid YAML, didn't execute | Stricter rules confused model into mixed-format output |

## Engine Verification (Manual Test)

**Workflow:** `docs/benchmarks/workflows/manual-test-template-var.yml` — hand-crafted correct syntax.

**Execution result:**
```
[shell] cat ["src/client/http_client.rs"] → exit=0, stdout=11784 bytes
step_t1_analyze_file duration_ms=96586 quality_score=1.0
output: "CLONABLE, #[derive(Clone)]"
```

**Output file:** `./outputs/t1_clone_verdict.txt`:
```
CLONABLE, #[derive(Clone)]
```

**Conclusion:** Engine + shell hooks + template resolution = CORRECTLY FUNCTIONING. Model receives real file content (11KB of Rust code), produces real analysis.

## Root Cause of Generator Insufficiency

Qwen3.5-9B (9B params) cannot reliably follow complex meta-template rules:
- Multiple levels of templating (template generating templates)
- Subtle distinction between "shell_output" as word vs `{{bookmarks.shell_output.stdout}}` as syntax
- Strict YAML formatting + content rules + shell hook patterns = too many constraints

Even with explicit CORRECT/WRONG examples, model produces inconsistent output.

## What Cycle 1 Achieved

1. ✅ Proved engine + shell hooks work end-to-end (manual test)
2. ✅ SW4/SW5 templates improved significantly (added shell hook requirements)
3. ✅ Cycle 1b achieved 25/25 structural score on P14 (perfect workflow shape)
4. ✅ Identified exact bug: template variable emission by 9B model
5. ✅ Documented the path forward (engine-assisted template generation)

## What Cycle 1 Did NOT Achieve

1. ❌ Generated workflows don't reliably include `{{bookmarks.shell_output.stdout}}`
2. ❌ Execution of generated workflows still produces refusal text
3. ❌ Parity on P14 not reached (let alone other 10 prompts)
4. ❌ Template rewrite alone insufficient

## Path Forward — Options for Cycle 2

### Option A: Engine-Assisted Template Injection
In `src/benchmark/runner.rs` `execute_workflow_step()`:
- If step has `before_step_starts` shell hook AND prompt mentions `{{bookmarks.shell_output.stdout}}` literally anywhere:
  - Already works (manual test proved it)
- If step has shell hook but NO template var:
  - **NEW ENGINE FEATURE:** auto-append shell_output content to prompt with clear delimiters
  - Or auto-replace bare "shell_output" word with actual template syntax

**Estimated effort:** 2-4 hours engine work + tests
**Pros:** Solves root cause at engine level. Future workflows benefit.
**Cons:** Engine change. Schema needs update.

### Option B: Post-Process Generated Workflows
Extend `scripts/meta-v6/fix-yaml.py` to:
- Detect prose "shell_output" in prompts
- Replace with `{{bookmarks.shell_output.stdout}}`
- Add as final hook in SW5 step_06_finalize

**Estimated effort:** 1-2 hours
**Pros:** No engine changes. Fast iteration.
**Cons:** Whack-a-mole. Doesn't fix underlying generation issue.

### Option C: Larger Model (Qwen3.5-32B or similar)
Bigger model better at following meta-template rules.

**Estimated effort:** Multi-day (need different quantization, may not fit RX580 8GB)
**Cons:** Hardware limit. Out of scope.

### Option D: Different Generator Architecture
Instead of SW4/SW5 generating free-form YAML, use a templated approach:
- SW4 outputs task-level metadata (JSON)
- SW5 renders via deterministic templates (Jinja2-style)
- Engine or script assembles final workflow

**Estimated effort:** 1-2 days refactor
**Pros:** Deterministic, no LLM unreliability for structure
**Cons:** Major refactor. Loses LLM's flexibility for prompt phrasing.

## Recommendation

**Option A (Engine-Assisted) + Option B (Post-Process) hybrid.**

- A: Engine auto-injects shell_output content when prompt references it but not via template syntax
- B: fix-yaml.py fixes obvious prose "shell_output" mentions in generated YAML

Time: 4-6 hours. Achievable in Cycle 2.

## 11-Prompt Parity Outlook

If Option A+B implemented in Cycle 2:
- Code prompts (P05, P07, P13, P14, P15): Likely achievable (5/5)
- Research/doc prompts (P10, P12): Likely achievable (2/2)
- Diagnostic (P08): Out of scope (no clear artifact)
- Other-repo prompts (P06, P09): Out of scope (different working dirs)
- Duplicate (P11): Same as P10

**Realistic Cycle 2 outcome:** 7/11 parity (matching baseline inventory)

## Honest Final Statement

Cycle 1 did NOT achieve parity. But it achieved something more valuable: **PROOF that the engine works**, and **PRECISE DIAGNOSIS of the gap**. The next cycle has clear, narrow work to do.

The 9B model limitation is real. Future iterations should accept this and either:
1. Move complexity to engine (deterministic) side
2. Use bigger model
3. Accept partial parity on hardest prompts
