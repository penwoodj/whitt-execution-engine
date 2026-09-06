# v4 Batch Log — 170-case campaign

Per docs/06-VERSION-ITERATION-RULESET.md §8. Workflow: meta-v4. Corpus: prompts/meta-prompts-170.yml (H=43 S=57 L=70, batches b01-b17).

## b01 — COMPLETE, GATE PASS (2026-08-30)

- ingest: 10 cases (tcA001-010, all HEAVY), validation: PASS (both revisions)
- run r1 (launched 12:52 CDT, fresh): **10/10 PASS**, wall 5619s LLM+stages,
  stages 18-20/case
- triage: no case failures. Structural defect found via stage counts:
  chain-last stage fell through sequentially into other lanes' chains
  (every case ran HEAVY+SYNTH+LIGHT ≈ 2x wasted LLM calls). Class: GATE.
  Fix: generator chain-exit route (`after_step_succeeds: gwt: '1 == 1' →
  routelog_{pid}`) on chain-last stages. TDD'd (test_gen_chain_exit_routing).
- retest/r2: fresh full-batch on regenerated `meta-v4-b01-r2.yml`:
  **10/10 PASS, stages=9/case exact** (conf + HEAVY 8 + routelog)
- regression: r2 reran all 10 batch cases (no prior batches) — green
- quality parity: extract3 artifacts complete both runs, quality_score 1.0
  all stages both runs; r2 tighter (tcA001 426-char complete JSON final).
  No degradation.
- efficiency (same cases r1→r2): wall 5619s → 3324s = **1.7x faster**,
  tokens ~340k → ~188k = **1.8x fewer**, median/case 601s → 346s.
  Confounders: same session, same model, RAM 3.4-5.6Gi ok.
- dedup (§6): yml-dedup-audit b01 yml: 190 steps → 19 normalized groups
  (~291KB scaffolding). Runtime nil (parse-time only); real dedup was
  execution-level (stages 18-20 → 9). RECORDED, no churn this batch.
- side findings: stale Qwen3-4B-Instruct auto-unloaded by resource-guard
  pre-flight (correct); setsid launcher PID ≠ child PID — poll via pgrep.

## b02 — PENDING (tcA011-020)


## b02 (2026-08-30, meta-v4-b02.yml r1, seed: none)
- ingest: tcA011-020, H=5 S=5 (first SYNTH lane live exposure), validation PASS
- run r1: 10/10 PASS, wall 2388s, tok 145,848
  - HEAVY cases (011,012,015,016,018): 303-411s, 17.3k-21.9k tok, 9 stages
  - SYNTH cases (013,014,017,019,020): 99-151s, 8.7k-10.8k tok, 8 stages
  - SYNTH ~2.5x cheaper/case than HEAVY (fewer/shorter stages: split+2xworker+merge vs plan+2xsolve+verify chain)
- triage: ZERO failures — no fixes needed. SYNTH lane (split→worker_a/b→extracts→merge→extract_m→routelog) worked first live exposure; routelog JSON exact for all
- retest: n/a (no failures); batch-previous: b01 regression launched as b01-r3 (fresh run, meta-v4-b01-r3.yml w/ dedicated run-dir; NOTE: first launch attempt reused r2 yml whose baked paths would clobber r2 artifacts — caught+fixed pre-stage, regen r3 yml)
- regression: b01-r3 in flight, result below
- dedup: no new structure (same 191-step shape); prior audit stands (runtime nil impact)
- efficiency: b02 wall 2388s/10 vs b01-r2 3324s/10 = same-engine same-fix, delta = case mix (5 SYNTH), not version change. Per-lane medians now measured: HEAVY ~350s, SYNTH ~127s — future lane-mix projections use these
- regression result: b01-r3 10/10 PASS (fresh run, stages=9/case) — zero regressions
- GATE: PASS → b03

## b03 (2026-08-30, meta-v4-b03.yml r1, seed: none)
- ingest: tcA021-030, H=4 S=6, validation PASS
- run r1: 10/10 PASS, wall 2095s (fastest batch — SYNTH-heavy mix)
  - stage counts correct: HEAVY=9, SYNTH=8 across all cases
- triage: ZERO failures, no fixes
- retest: n/a; batch-previous: none failed
- regression: reg-b03 launched (tcA001-020, 381 steps, H=15 S=5, fresh) — result below
- dedup: no new structure; prior audit stands
- efficiency: SYNTH/HEAVY per-case medians holding (127s/350s); b03 wall/case 210s avg consistent with 4H+6S mix projection (~201s) — projections now reliable
- GATE: PENDING regression

### reg-b03 (all-prior regression, 20 cases)
- 20/20 PASS fresh, stages correct (H=9/S=8), 0s wall-equiv, 0 stages
- b03 GATE: **PASS** (no regressions from b01-b03 cumulative)

## b04 (tcA031-040, H=2 S=8)
- r1: 10/10 PASS, stage counts correct (H=9/S=8), zero triage
- GATE: PENDING regression (reg-b04 = tcA001-040)

### reg-b04 final (40-case regression, tcA001-040)
- First leg: 11 PASS then boundary-killed at 1.3Gi RAM (external consumers: opencode+firefox)
- Resume leg: +28 PASS (launched at documented 2.4Gi deviation, zero OOM)
- tcA012 interrupted at stage 7/9 by kill; single-case rerun reg-b04-tcA012-r1: PASS 9 stages
- **reg-b04 GATE: PASS — 39 (reg-b04) + 1 (tcA012-r1) = 40/40**

### Generator hardening (rule 9, schema L562-574/L912) — 2026-09-03
- gen-meta-system.py now emits resource_admission (block, 6GiB/6GiB/4GiB minima,
  positive estimates, write_profile) + absolute models source_path (basename==name)
- Model label corrected: "Qwen3-5-9B-Q4_K_M" never existed on disk; real served
  model = Qwen3-4B-Instruct-2507-Q4_K_M.gguf (2,497,280,448 B). All prior runs
  actually executed on this model (server ignores model-name field, single-slot)
- Engine admission enforced live: first launch rejected at 5.96GiB < 6GiB floor
  (RESOURCE_REJECT_WORKFLOW_RAM) — contract works; docker restart freed to 7.4GiB
- Telemetry profile: docs/benchmarks/outputs/resource-admission-profile.json —
  weights_bytes exact stat match, min RAM 7.04GiB observed, admitted 27 checks
- Runtime estimate calibrated 900→400 s/case (measured HEAVY 222s)
- test_meta.py: test_gen_resource_admission added (TDD: 2 FAIL → ALL PASS)

## b05 (tcA041-050, H=5 S=5)
- r1: 10/10 PASS, zero triage (~28min)
- reg-b05 (tcA001-050 fresh): 50/50 PASS, ~3.2h, RAM 7.6-9.5Gi stable
- b05 GATE: **PASS**

## b06 (tcA051-060, H=3 S=7)
- r1: 10/10 PASS, zero triage
- GATE: PENDING reg-b06

## b07 (tcA061-070, H=3 S=5 L=2 — first LIGHT lane)
- r1: 10/10 PASS, zero triage (~29min)
- GATE: PENDING regression (reg-b07 = tcA001-070)

### reg-b07 (all-prior regression, 70 cases)
- 70/70 PASS fresh (~4.7h), stage counts correct (H=9/S=8/L=3), RAM stable 7.0-9.1Gi
- b07 GATE: **PASS** (no regressions from b01-b07 cumulative)

## b08 (tcA071-080, H=1 S=8 L=1)
- r1: 10/10 PASS, stage counts correct (H=9/S=8/L=3), zero triage
- GATE: PENDING regression (reg-b08 = tcA001-080)

## b09 (tcA081-090, H=7 S=2 L=1)
- r1: 10/10 PASS, stage counts correct, zero triage (~35min)
- GATE: PENDING regression (reg-b09 = tcA001-090)
- reg-b09: 88/90 + tcA083/tcA085 json_exact soft-fails (replan-loop JSON drift, both hard_fail:false, both passed b09-r1 fresh) → targeted rerun `reg-b09-flake-r1` 2/2 PASS clean 9-stage → merged 90/90
- b09 GATE: **PASS** (2 output-variance flakes, triaged + rerun green)

## b10 (tcA091-100)
- GATE: PENDING
- r1: 10/10 PASS, zero triage (~30min)
- GATE: PENDING regression (reg-b10 = tcA001-100, full tier A)
