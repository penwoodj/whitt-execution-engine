# Self-Healing Experiment — Tracking

## Iteration Log

| # | Date | Change | Validation | Live run | Zero-LLM gate | Outcome / next |
|---|------|--------|-----------|----------|---------------|----------------|
| 1 | 2026-08-24 | v1 design: research (13 docs), experiment docs, case suite, spoof scripts, gen-v1.py | 6/6 pass | pending | pending | — |
| 2 | 2026-08-24 | standalone script-pipeline test (bash, no engine) | — | n/a | n/a | 5 bugs found+fixed (F1 marker match, R leniency, outcome vocab, end_event choices, gen None-routes); 6/6 oracle |
| 3 | 2026-08-24 | first live runs (whitt benchmark, WHITT_ZOMBIE_MAX=8) | 6/6 pass | 6/6 exit 0 | 0 events ×6 | clean-01 green; 5 cases oracle FAIL — stale-trace pollution + report vocab `=="pass"` bugs; fixed (end_event reset_run, report vocab map); rerun clean-01 green |
| 4 | 2026-08-24 | rerun 5 cases | — | 6/6 exit 0 | 0 events ×6 | 5/5 oracle FAIL — gen-v1 numbering bug: `dno=2n` emitted `step_03_attempt_2`/`step_05_classify_2`, heal targets pointed at nonexistent `step_07_attempt_2`; engine silently falls through to next sorted step on bad target |
| 5 | 2026-08-24 | gen-v1: explicit `chain_nums` table + route-target existence assert; regen 6 | 6/6 pass | 6/6 exit 0 | 0 events ×6 | **6/6 ORACLE PASS**; suite TSR 5/6, FDA 8/8, RSR 4/5 — all targets hit |
| 6 | 2026-08-24 | v2 design: 100 cases (10 flagship 1k-1.2k words + 90 matrix via gen-cases-v2), 46-step graph, 12-script kit, gen-v2.py | 10/10 + 90 emitted | not run (user: write only) | n/a | Phase L authorized 2026-08-29 |
| 7 | 2026-08-29 | ENGINE HARDENING (storybook machine crash, user directive): crash-debug skill + triage; resource_guard.rs TDD t01-t37 (8GB→1 concurrency, KV math, admit_load, hysteresis, gate_action, vk scan, step_gate, guard_inference); runner E1 between-step gate + E2 in-flight cancel (non-retryable RESOURCE_CRITICAL) + L5 preflight crash-signature scan (--since StartedAt); detect chain delegated | 634 tests green | binary rebuilt (CPATH + CMAKE_POLICY_VERSION_MINIMUM=3.5) | L5 fired live on stale crash logs ✓ | smoke run7: full chain, 3 real calls; L5 validated |
| 8 | 2026-08-29 | optional-keys fix (detect_v2 honors expected_schema.optional; flg-01 + matrix cases) + live-script import fixes (parents[1]) + worker_prompt no-heal fallback | script-level TDD red→green | run8 diagnosed, run9 **flg-01 FIRST GREEN LIVE RUN: oracle pass, accept@1, 1 llm_call** | real LLM (Qwen3-4B) | batch 1 launched (9 flagships) |

## Status

- [x] Phase S: structural validation green (6/6 workflows)
- [x] Phase S: live zero-LLM runs green (6/6 cases, report.py oracle 0-exit)
- [x] Phase S: zero-LLM proof gate green (no load/unload/inference events)
- [x] Phase S: suite metrics = expected (TSR 5/6, FDA 8/8, RSR 4/5)
- [ ] Phase L: BLOCKED — awaiting explicit user authorization

## Phase S Final Evidence (2026-08-24)

- Workflows: `workflows/generated/sh-v1-{clean-01,f1-01,f2-01,f3-01,f4-01,f1-persist-01}.yml`
- Logs: `results/whitt-logs/<case>-run2.log`
- Per-case reports: `results/sh-v1/<case>/report.json` (all `oracle_pass: true`)
- Suite: `python3 scripts/report.py --suite results/sh-v1` → cases 6, TSR 0.8333, FDA 1.0 (8/8), RSR 0.8 (4/5), oracle_all_pass true
- Verdicts: clean accept@1 · f1/f2/f3 accept@2 · f4 accept@3 (2 heals) · f1-persist final_fail@3 — exactly per case specs
- Zero-LLM grep (`Prompt tokens|eval count|tokens/s|llama_|loading model|unloading model|\[infer|total duration`): 0 matches ×6 logs; all steps 'routed by hook'/'skipped by hook'
- Engine quirk documented: RouteTo nonexistent target logs "routed to" but silently falls through to next sorted step — gen-v1 assert now guards this class

## Decisions Register

| Decision | Rationale | Source |
|----------|-----------|--------|
| Unroll heal chain (3 attempts explicit steps) | engine forbids cycles; v9 fix_1/2/3 precedent | engine constraint + harness v9 |
| Exit-code classification contract (0/1/2/3/4) | GWT reads bookmarks.shell_output.exit_code natively | v9 proven pattern |
| Detectors deterministic-first | weak self-judge on 4B–9B; Guardrails/CRITIC evidence | docs 04, 11 |
| ω = 0.35/0.35/0.30, θ = 0.65 | paper values where given; weights initial | doc 01 |
| MAX_ATTEMPTS = 3 | k=3 convergence across Reflexion/Self-Refine/CRITIC | docs 02–04 |
| Heal scripts write heal record; attempts read history | Reflexion episodic memory, bounded | doc 02 |
| WHITT_ZOMBIE_MAX=8 for live runs | preflight zombie count self-matches greps; rea+ precedent | rea+ 07-TRACKING + safe-launch.sh |
| `end_event --status init` resets run state | stale trace from prior runs corrupted report oracle | iteration 3 bug |
| Step numbering strictly increasing per chain order | engine iterates sorted-by-name; interleaved rounds + bad targets → silent fallthrough | iteration 4 bug |
| end_pass/end_fail = ORACLE verdict, not case outcome | f1-persist final_fail is a PASS of the oracle (expected == actual) | design, iteration 5 |
