# v4/v5/v6 Design — Zero-LLM Verified Technique Variants

> 2026-08-23. Constraint: design + verify WITHOUT LLM calls or model load/unload.
> Spoof pattern (REA+ M1/M2/R5): before_step_starts shell writes canned stage
> output → REAL gate scripts run on it → GWT routes on real exit codes →
> skip_step kills inference. Engine, hooks, routing, scripts, traces = LIVE.
> Inference = scripted. Spoof never counts toward quality (R5).

## Failure → Technique Map

| Failure | Root | Research fix | Version |
|---------|------|--------------|---------|
| ha-08: end_fail rule never applied | rule-to-state inference gap | P3 decomposition: per-rule classification steps | v4 |
| ha-06: 6×3 working-memory overload | too many entities in one context | P3: one entity per call + P2 deterministic aggregation | v4 |
| ha-05/ha-01 value drops (fixed but fragile) | component omission | enumeration generalization → FIRING TABLE with completeness gate | v5 |
| fix feedback leaks / too vague | F8 | per-cell feedback: cell id + observed, never expected | v5 |
| 15s/case restart tax | unconditional restart | conditional restart on zombie count | v6 |
| wasted judge touch after end_fail | topological visitation | skip_remaining on end_fail + explicit depends_on | v6 |
| silent gate crashes | fail_on_error swallows | gate scripts exit-code logged; spoof battery asserts traces | v6 |

## v4 — DECOMP (P3 + P2)

Workflow: for each entity E in case `decomposition.entities`:
SOLVE_E (tiny classify prompt, hops 1-2, max_tokens ~200) → CHECK_E
(check-sub.py vs `decomposition.expected[E]`, leak-safe) → optional FIX_E
(per-entity feedback). Then AGGREGATE (aggregate.py: safe-eval case
`aggregation` expr over entity answers) → json_exact → JUDGE.

- ha-06: 6 sub-calls, each "case N: lane + final + judge_calls" — trivial per call
- ha-08: 3 sub-calls: check_lines / end_fail_line(fires?) / judge_lines
- Load: same warm model, N small calls, NO restarts. Total tokens ≤ single mega-solve.

## v5 — FIRING-TABLE (enumeration generalized)

Workflow: SOLVE emits RULE×ENTITY table (markdown rows) then final JSON.
New gate `check-table.py` BEFORE json_exact:
1. completeness: every case-declared `table.cells` key present
2. correctness: cell values match `table.expected` (values in checker, never in feedback)
Fail → feedback names missing/wrong CELLS + observed values (F8). Fix repairs cells only.
- ha-08: `end_fail_line` cell REQUIRED by schema — omission caught pre-verdict
- ha-06: 6 rows × lane/calls forced before totals
- Single solve call — same load profile as v3.

## v6 — UNIFIED (the one workflow)

v5 table prompt + v4 aggregate branch: entities ALWAYS declared (every case);
SOLVE emits per-entity table rows; aggregate.py builds final JSON from
VERIFIED rows only; json_exact final truth; judge blind.
Infra:
- end_fail: trace-append THEN skip_remaining (no judge touch, ledger complete)
- explicit depends_on chain (no implicit topological surprises)
- runner: zombie-count.sh gate → restart ONLY when count > 3
- all gate chains asserted by spoof battery before any live run

## Verification Ladder (this session, zero LLM)

1. Unit: check-table.py, aggregate.py, check-sub.py, spoof-write.py
2. Spoof battery: 10 cases × scenarios {win-d0, win-d1, win-d2, lose,
   table-incomplete} — full `whitt benchmark` runs, real hooks+routing,
   skip_step everywhere. Assert: artifacts written, routing log lines match
   scenario, trace order correct, collect.py verdict matches, end_fail
   skips judge.
3. Structural: validate-workflow.py PASS on every generated YAML (spoof +
   live variants)
4. Live-ready: live YAMLs generated + validated, NOT executed (awaiting
   LLM-permission window)

## Expected Outcomes (hypotheses, live-test pending)

| Case | v3 | v4 | v5 | v6 | Mechanism |
|------|----|----|----|----|-----------|
| ha-06 | FAIL(var) | PASS | PASS | PASS | per-entity classify + det sum |
| ha-08 | FAIL(5/5) | PASS | PASS | PASS | end_fail cell forced |
| ha-01..05,07,09,10 | PASS | PASS | PASS | PASS | no regression: table ⊇ enumeration prompt |
| Suite wall | ~336s+150s restarts | — | — | ~200s est | no restarts, smaller calls |
