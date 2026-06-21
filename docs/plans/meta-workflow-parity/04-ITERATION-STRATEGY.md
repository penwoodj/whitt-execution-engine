# 04 - ITERATION STRATEGY

## OVERALL APPROACH

Two-track parallel work:

**Track A: Engine Tooling (Option A from 03-GAP-ANALYSIS)**
- Implement missing tools: ShellTool, FileWriteTool, GrepTool
- Wire tool access into workflow steps (not just ReAct)
- Long-term enabler — needed for true parity on code prompts

**Track B: Workflow Generation Fixes (Option B from 03-GAP-ANALYSIS)**
- Rewrite SW4/SW5 prompt templates
- Replace "Read X" with shell-hook-loaded bookmarks
- Short-term improvement — hours not days

## PHASE 1: ESTABLISH OPENCODE BASELINE (DO FIRST)

For each of 11 prompts:
1. OpenCode (me) attempts prompt directly
2. Save artifacts to `docs/plans/meta-workflow-parity/baselines/opencode/prompt-{N}/`
3. Self-evaluate per 5 dimensions (completeness, correctness, depth, verification, honesty)

**Stop baseline phase:** All 11 saved OR timeout at 2 hours total.

## PHASE 2: CURRENT STATE AUDIT

For each of 11 prompts (use existing SW5 outputs from b3 run):
1. Read generated workflow.yml
2. Execute workflow via whitt benchmark
3. Capture all output files
4. Check for refusal indicators
5. Score per 02-VALIDATION-CRITERIA

**Stop audit phase:** All 11 scored OR consistent pattern identified.

## PHASE 3: ROOT CAUSE ITERATION

Per gap found, choose path:
- Engine fix (Track A) — if blocks ALL prompts
- Template fix (Track B) — if blocks specific prompt types
- Declare limitation — if outside scope

## PHASE 4: IMPLEMENTATION CYCLES

**Cycle 1 (minimum viable):**
- Implement Track B (shell hook template rewrite)
- Re-run all 11 prompts
- Evaluate

**Cycle 2 (if Cycle 1 insufficient):**
- Implement Track A partial (ShellTool only — biggest bang)
- Re-run failing prompts
- Evaluate

**Cycle 3 (last resort):**
- Implement remaining Track A tools (FileWriteTool, GrepTool)
- Re-run all prompts
- Final evaluation

## ITERATION RULES

1. **One change per cycle** — change engine OR templates, not both
2. **Full re-run per cycle** — all 11 prompts, not just failures
3. **Honest scoring** — refusal in output = HARD FAIL
4. **3 cycle limit** — if Cycle 3 fails, escalate to user

## TIME BUDGETS

| Phase | Est. Time | Actual |
|-------|-----------|--------|
| Phase 1 (baselines) | 2h | TBD |
| Phase 2 (audit) | 1h | TBD |
| Phase 3 (root cause) | 30min | TBD |
| Cycle 1 (template fix) | 4h | TBD |
| Cycle 1 eval | 1h | TBD |
| Cycle 2 (engine shell tool) | 6h | TBD |
| Cycle 2 eval | 1h | TBD |
| Cycle 3 (full engine tools) | 8h | TBD |
| Cycle 3 eval | 1h | TBD |
| **Total max** | **24.5h** | |

Realistic: Cycle 1 likely gets to 5-7/11 parity. Cycle 2 likely needed for code prompts.

## DEPENDENCIES

- Docker container stable across multi-prompt runs
- Whitt binary rebuild between engine changes
- Plan suite kept in sync with findings (write to plan after each cycle)
