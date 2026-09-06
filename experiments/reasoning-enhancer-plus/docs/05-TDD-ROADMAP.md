# REA+ — TDD Roadmap (one metric per version)

Test-first for EVERY version: lint → dry-run → single-case pulse →
6-case slice → 24-case train suite. Full suite only from green slice.
No version skips the ladder. Max 3 fix cycles per version, then escalate
to user (AGENTS.md QA cycle rule).

## Version series

| ver | target | objective (green when...) | red evidence first |
|---|---|---|---|
| vP0 | baseline | old workflow fails 30/30 on new suite; artifact archived | gap-band validation run |
| vP1 | M2 format | format subcheck rate ≥90% on 6-case slice | slice run showing <90% |
| vP2 | M1 accuracy | train accuracy ≥60% (from vP1 baseline) | slice run |
| vP3 | M3 depth | ≥50% of hops:3 cases pass; ≥1 hops:4 passes | per-hop breakdown |
| vP4 | M4 robustness | robust:true pass rate ≥50% | robust-slice run |
| vP5 | M5 priority | prio:true pass rate ≥60% | prio-slice run |
| vP6 | M6 stability | n=3 seeds, σ ≤0.10, no seed <50% | 3 seed runs |
| vP7 | M7 latency | median clean wall ≤60s/case, p90 ≤120s | timing table |
| vP8 | M8 efficiency | ≤8 model loads/suite run, load-share ≤35% | load count from logs |
| vP9 | held-out | held-out 6 reported, ≥50% (honest generalization number) | first held-out run |

Notes:
- Objectives recalibrated ONCE after vP2 with real numbers, then frozen.
  Recalibration logged in 07-TRACKING.md with reason.
- Ablation arms B (retry-only) and C (rubric-only) run once on final
  candidate, before vP9.

## Ladder (every version, no exceptions)

```
1. python3 scripts/lint-cases.py --suite rea-plus     # must be green
2. python3 scripts/dry-run.py (REA pattern)           # static contract
3. rea-live-pulse: 1 case, 1 model, fingerprint check  # smallest live
4. slice: 6 cases (2 hardest + 2 median + 2 easy)      # stratified
5. train suite: 24 cases                               # only from green 4
6. judge.py --all-metrics -> 07-TRACKING.md row        # evidence
```

## Regression ratchet

- Target metric improved AND no other metric regressed >10% relative
  → green, tag version
- Regression → fix before proceeding (may not retitle as "tradeoff")
- Every green appends to results/regression-table.md (the M1..M8 table)

## Debugging protocol (live-system source of truth)

- Failure triage starts at fingerprint.log: case_id+step+sha1(prompt)
  → exact input reproduced in <1min via pulse replay
- No fix without reproduced failure (systematic-debugging)
- Engine suspects: check runner log for `workflow loop exceeded 100
  iterations` silent-exit (known upstream bug) before blaming prompts

## Skill usage

- `rea-model-probe` — stage-1 sweeps + any model-comparison need
- `rea-live-pulse` — ladder steps 3-4, failure replay
- `rea-metrics-judge` — ladder step 6, one-metric focus flag
