# Meta-Workflow Parity — Status & Index

**Last updated:** 2026-08-03
**Active plan:** `00-COMPREHENSIVE-PLAN.md` (v2.2)
**Master plan (v1):** archived — see `archive/00-MASTER-PLAN-superseded.md`

---

## Current State

| Item | Status | Evidence |
|------|--------|----------|
| Engine reliability | ✅ PASS | cargo test: 562 lib + 202 integration = 764 total, 0 failures, 0 clippy errors |
| SW1-SW5 LLM generator | ✅ WORKS | Avg 49.29/50 honest validator score |
| Deterministic fallback (`generate-minimal.py`) | ✅ REMOVED 2026-08-03 | Bypass per promise gate — deleted; SW1-SW5 LLM path is canonical |
| Framework YAML-ization (chunks 6-10 + GAP-1..7) | ✅ DONE | (B) resource gov; (A+C) retry+timing; (F) output_root; (G) hook triggers incl during_step_streaming; (E) refusal_detected as GWT predicate; (D) require_step_prompt; (H) iterate_values config + on_requires_failed GWT demonstrated. |
| Framework YAML-ization (chunks I, J — UN-DEFERRED + DONE) | ✅ DONE | (I) ModelSelector production wiring: 3 fields on BenchmarkWorkflowConfig + ModelSelectionConfig struct + dispatch + 2 tests. (J) DetailGenerator template-driven: render_detail() pub fn + DEFAULT_DETAIL_TEMPLATE const + workspace.detail_template field + 11 tests. |
| Live system tested | ✅ PARTIAL | P08 + P12 + P13 fully through SW1-SW5 path |
| Side-by-side opencode comparison | ✅ P08 only | SW1-SW5 SURPASSES opencode-glm-5.2 on P08 |
| Prompts ≥45/50 honest validator | ✅ 7/7 of tested | P10, P11, P15 not yet passing |
| Promise gate criteria met | ⚠️ PENDING | Live demo DEFERRED per user direction (2026-08-03). Cleanup + framework work ships now; SW1-SW5 live parity test runs in future session. Criteria: 3+ prompts SW1-SW5 ✓; >1 SW1-SW5 beats opencode ✗ (only P08 currently); user explicit confirmation ✗ |

## Architectural Decisions (user direction 2026-08-03)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Sync vs parallel (SW1-SW5 sequential cost) | **SYNCHRONOUS** | Honor AGENTS.md hard rule. ~5hr wall clock per cycle. Predictable, debuggable. |
| Per-cycle cache (prompt→workflow memoization) | **DEFER** | Document as known limitation. Revisit if cycle count grows. |
| Validator authority (Goodhart's Law) | **CROSS-CHECK + MINIMAL HUMAN SAMPLING** | Added C9 semantic alignment check (heuristic, <1s per check) + N=2 human sampling protocol per cycle. LLM-as-judge rejected for time cost. See `human-sampling-protocol.md`. |

## Validator Cross-Check Implementation

**Weaknesses identified in `parity-check.sh`** (pre-cross-check):
1. C2 "well-formed" = shallow grep (gameable via comments)
2. C4 "substantive" = byte/line count (lorem ipsum passes)
3. C5 refusals = fixed 5-pattern regex (paraphrase evasion)
4. **No semantic alignment check** (critical Goodhart vulnerability)

**C9 semantic-check.py coverage** (added):
- Keyword overlap (≥30% content words from prompt in deliverable)
- Structure match (prompt asks for list/code/summary → deliverable has matching shape)
- Broader refusal patterns (paraphrase evasion)
- HARD_CAP: keyword_overlap < 0.10 = max score 3 (kills off-topic garbage)

**Threshold adjustment**: 40/50 → 50/60 to maintain ~83% pass rate with new C9 criterion.

**Human sampling**: N=2 random prompts per cycle from PASS set. Q1 veto (objective alignment) overrides validator PASS. See `human-sampling-protocol.md` for procedure + review template.

## Outstanding Work (per Comprehensive Plan exit criteria)

1. ~~Remove `generate-minimal.py` (deterministic bypass)~~ ✅ DONE 2026-08-03
2. Run P10/P11/P15 through fixed SW1-SW5 LLM generator
3. Side-by-side comparison on ≥3 prompts (only P08 currently)
4. Live system E2E test on cleanup-complete state
5. Decide: ship as-is OR iterate one more cycle

## Doc Map

### Active (read these)

| Doc | Purpose |
|-----|---------|
| `00-COMPREHENSIVE-PLAN.md` | Master plan, revision history, exit criteria |
| `01-BASELINE-METHODOLOGY.md` | How baselines established |
| `02-VALIDATION-CRITERIA.md` | Strict pass/fail criteria |
| `03-GAP-ANALYSIS.md` | Current state gaps |
| `04-ITERATION-STRATEGY.md` | Cycle plan |
| `05-RESOURCE-CONSTRAINTS.md` | Hardware/safety limits |
| `06-EVALUATION-FRAMEWORK.md` | Critical evaluation criteria |
| `07-WEB-RESEARCH-LOG.md` | Assumption testing log |
| `08-CYCLE-4-PLAN.md` | Cycle-4 work plan |
| `09-CYCLE-5-ITERATION-PLAN.md` | Cycle-5 plan |
| `cycle-1-final-report.md` | Cycle-1 outcome |
| `cycle-2-final-report.md` | Cycle-2 outcome |
| `cycle-3-final-report.md` | Cycle-3 outcome (claims 9/11 — partially fraudulent per cycle-4) |
| `cycle-4-final-report.md` | Cycle-4 outcome (honest validator; SW1-SW5 works on 7/7 tested) |
| `cycle-4-opencode-comparison.md` | P08 side-by-side comparison |
| `STATUS.md` (this file) | Current state summary — single source of truth |

### Archived (`archive/`)

Superseded, progress tracking, amendments, historical analysis. Not authoritative.

- `00-MASTER-PLAN-superseded.md` — original v1 master plan (pre-Momus review)
- `cycle-0-phase2-audit.md`
- `cycle-1-progress.md`, `cycle-2-progress.md`, `cycle-2-design.md`
- `cycle-3-plan.md`, `cycle-3-final-report-amendment.md`, `cycle-3-final-report-amendment-2.md`
- `PROGRESS-REPORT.md`, `SESSION-PROGRESS-20260628.md`, `SPEEDUP-ANALYSIS.md`
- `promise-gate-verification.md`

### Cycle result data (`cycle-N-results/`, `comparisons/`, `baselines/`)

Raw per-prompt outputs + comparison analyses. Kept as evidence trail.

## Navigation Order for New Reader

1. `STATUS.md` (this file)
2. `00-COMPREHENSIVE-PLAN.md`
3. `cycle-4-final-report.md` (latest verified outcome)
4. `02-VALIDATION-CRITERIA.md` (how PASS decided)
5. Skip archive unless historical context needed
