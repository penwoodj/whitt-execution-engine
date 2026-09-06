# V12 Spoof Verification — Zero-LLM Readiness

> 2026-08-24. v12 hard-suite campaign: 100 hd (extra-hard) + 100 hc (prior suite)
> spoofed end-to-end through the real engine. Zero model loads, zero LLM calls.

## Scope

Prove, without loading a single model, that the next experiment version
(v12 workflow + hook scripts) is mechanically ready to pass:

1. all 100 previous hc cases (regression guarantee)
2. all 100 new hd cases modeled off v10's failure classes (11-HD-DESIGN.md)

## v12 Deltas vs v11 (what spoof had to verify)

| Delta | Why | Spoof proof |
| --- | --- | --- |
| fix_3 rescue stage on ALL cases (light included) | v10: light cases failed arithmetic through fix_2 with nowhere to go | wind3: `check:fail fix_1:fail fix_2:fail fix_3:pass final:pass` on hd-01, hd-37, hc-03, hc-24, hc-99 |
| hd per-entity method hints (hd_hints) | unit/method reminders next to each question | unit tests: hints in hd workflows, absent in hc |
| wind3 spoof scenario | exercises the new fix_3 recovery path | 5/5 cases above |
| isolated output dirs (results/v12-*) | other session's live v11 campaign owns shared dir | ymls reference v12-spoof-output only |
| spoof fixtures in fixtures/spoof-v12/ | same | 7400 fixture files, no spoof-v7 refs |

## Results

### Battery (wind0 — depth-0 pass path)

```
ran 200 workflows (100 hd + 100 hc), sequential whitt benchmark
finals: 200 cases, 200 pass, 0 fail
hd: 100/100   hc: 100/100
audits: all pass   judges: all pass
wall: ~2.5 min total, ~0.8s per case
llama-server procs before: 2, after: 2   ← ZERO model loads
```

### Scenario spot checks

| Scenario | Chain observed (from ha-trace.jsonl) | Cases |
| --- | --- | --- |
| wind1 | check:fail fix_1:pass final:pass audit:pass | hd-02 |
| wind2 | check:fail fix_1:fail fix_2:pass final:pass audit:pass | hd-03 |
| wind3 | check:fail fix_1:fail fix_2:fail fix_3:pass final:pass | hd-01 hd-37 hc-03 hc-24 hc-99 |
| lose | all four fail → end_fail:fail audit:pass (leak-safe) | hd-50 hc-06 |

### Deterministic case verification (zero LLM)

- `verify-hd.py`: 100 cases, **0 problems, 320/320 contract keys** recomputed
  from prompt text alone, 0 manual-review entries.
- `test_hd_cases.py`: 20/20 (word bounds, derived-cell token-boundary check,
  aggregate parity, boundary-mark variation, marker wording, domain disjointness).

### Unit / integration tests

- New: test_v12_scripts.py (12) + test_v12_spoof.py (9) — 21/21.
- Full experiment dir: **205 passed** (no regressions in ha/hb/hc/v6/v7 suites).

## Bugs Found During Spoof Iteration (fixed)

1. **spoof-v7 path leak**: g7.spoof_gate_chain hardcodes its own fixture dir
   into hook text; v12 ymls pointed at spoof-v7 while fixtures landed in
   spoof-v12 → every gate read a missing file (0/0 checks).
   Fix: `spoof_gate_chain_v12()` string-rewrites the path; gen-v7 untouched.
2. **lose scenario fix_3 semantics**: initial write_scenarios_v12 wrote
   fix_3=good for lose (v11 habit where fix_3 rarely existed) — lose could not
   lose. Fix: lose → fix_3 bad; re-verified true end_fail path.
3. **Relative --workflow footgun (engine ticket, known)**: relative path
   triggered the silent discovery-benchmark fallback — rc=0, no workflow run,
   stray 3-model benchmark artifacts in the shared output dir. Fix in runner:
   absolute --workflow + absolute --output-dir. Engine hard-error fix remains
   an open ticket (docs/plans backlog).
4. **Zombie preflight collision**: other session's live campaign holds 2-3
   llama-server procs > default threshold 2. Fix: `WHITT_ZOMBIE_MAX=5` in
   run-v12-spoof.sh (spoof runs load nothing; count asserted unchanged
   before/after).

## Zero-LLM Evidence

- Every generative step is `skip_step`-hooked before inference; engine logs
  show "Routed by hook" / "Skipped by hook" only.
- llama-server process count identical before/after 200-run battery.
- No `/models/load` requests: battery wall ~0.8s/case (a real load is ~30s+).
- All stage artifacts come from `spoof-write.py` canned fixtures feeding the
  REAL gate scripts (check-table, aggregate, check-deterministic, leak-audit).

## Readiness Verdict

**READY for live campaign.** Mechanics proven on all five scenario paths;
all 200 cases structurally pass; deterministic truths verified independently;
no shared-state interference with the running v11 campaign.

Live run NOT executed — per instruction, spoof/unit only until user says go.
