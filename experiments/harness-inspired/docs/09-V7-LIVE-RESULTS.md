# V7 LIVE Results — 10/10

> 2026-08-23. Full LLM suite, Docker llama.cpp router, hops-routed workers
> (4B light / 9B heavy + 9B escalation), Hermes-7B cross-family judge.

## Final: 10/10 PASS, 756s (12.6 min) total

| Case  | Verdict | Depth | Time  | Judge              |
| ----- | ------- | ----- | ----- | ------------------ |
| ha-01 | PASS    | 0     | 52.0s | pass               |
| ha-02 | PASS    | 0     | 55.9s | pass               |
| ha-03 | PASS    | 0     | 60.2s | pass               |
| ha-04 | PASS    | 0     | 83.8s | pass               |
| ha-05 | PASS    | 0     | 88.7s | pass               |
| ha-06 | PASS    | 0     | 101.1s| pass               |
| ha-07 | PASS    | 0     | 55.3s | pass               |
| ha-08 | PASS    | 0     | 55.5s | **fail** (det overrides) |
| ha-09 | PASS    | 0     | 100.1s| pass               |
| ha-10 | PASS    | 2     | 103.4s| pass               |

9/10 wins at depth 0 (solve sufficed). ha-10 recovered at depth 2 — full
cascade exercised end-to-end: check→fix_1→fix_2→final→judge→audit.
ha-08 judge=fail but deterministic final=pass overrides (two-lane design:
det is primary). Judge disagreement rate: 1/10 — cross-family blind judge
confirms det verdict in 9/10.

## Version ladder (all live, same 10 cases)

| Version | Config | Pass |
|---------|--------|------|
| v3.0 | 4B, JSON-only, no table | 2/10 |
| v3.1 | 4B + CoT + extraction | 3/10 |
| v3.2 | 9B | 5/10 |
| v3.4 | 9B + enumeration + max_words fix | 8/10 |
| **v7** | **firing-table decomposition + hops routing + leak-safe cascade** | **10/10** |

v7 fixes v3's two unsolvables: ha-08 (end_fail-line inference) and ha-06
(verdict-matrix overload) — both depth-0 via per-entity decomposition.
The 5 heavy cases failed wholesale in v3 now decompose to per-entity rows
the 4B/9B models can each handle.

## Iteration ladder this session (post zero-LLM verification)

| Run | Result | Root cause found | Fix applied |
|-----|--------|------------------|-------------|
| smoke 1 | instant fail | duplicate `when:` key (gen bug, engine stricter than validator) | helper returns body only |
| smoke 2 | 0.1s fail | stale spoof artifacts | glob artifact.* too |
| smoke 3 | degenerate "A B C D" | case prompt's embedded "Output ONLY this JSON" overrode table contract | regex-strip at embed |
| smoke 4 | bare rows, 0/4 checks | table_lib ROW_RE required literal ENTITY prefix | prefix optional + explicit_ids |
| run A | B/C=2 (wrong) | entity question presumed re-runs | resume-run framing + rule restatement |
| run B | fix_2 ghost ids | fix prompts lacked Entities list | ents block in both fix prompts |
| run C | dict values as prose | no value-format contract | per-type format hints (schema only) |
| run D | PASS/FAIL vs allowed/VIOLATION | vocabulary mismatch | binary-choice directive |
| run E | depth2_group=5 | d+1 rule misapplied | worked-example restatement |

Pattern: every failure = information gap between case intent and model
understanding, closed by restatement at point-of-application — NOT by
leaking answers. Rule-restatement is the cheapest scaffold that works.

## Harness features proven live

- Firing-table decomposition: 4-7 entity rows per case, per-entity checks
- Hops routing: light→4B, heavy→9B (solve), escalation→9B (fix_2 always)
- Leak-safe cascade: feedback = id + observed + question, never expected
- Deterministic aggregate: hook-only step computes final JSON from rows
- Two-lane verdict: det final-check overrides judge (ha-08 case)
- Snapshots: artifact.{solve,fix_1,fix_2}.txt per stage
- Meter: tokens_est per stage in trace
- Leak audit: exit 0 all runs (feedback clean)
- Spoof battery remained green throughout (80/80) — structure intact

## Timing

- Total 756s / 10 cases = 75.6s avg (v3: 33.6s — 2.2x slower but 10/10 vs 8/10)
- Depth-0 case: 52-101s (solve + aggregate + judge)
- Depth-2 case (ha-10): 103.4s — cascade adds ~50% over depth-0
- Model loads: router warm after first case; zombie gate never tripped
  (conditional restart saved the 15s/case tax — 0 restarts this run)

## Unit tests

139 passing (2 new: bare-row parse, explicit_ids). test_routing_light_heavy
updated for escalation semantics + new test_fix2_escalates_to_heavy.
