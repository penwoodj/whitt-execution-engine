# Engine Bug Inventory — Harvested from Experiments

**Date:** 2026-08-24
**Source:** experiments/{atomic-reasoning, correction-atom, harness-inspired, reasoning-enhancer, reasoning-enhancer-plus}/ + WORKFLOW_RELIABILITY_TRACKING.md + docs/qa/
**Method:** 3 parallel explore agents + manual tracking-doc read; cross-referenced; script-only/model-quality/design-accepted items filtered out.
**Severity order:** silent-wrong-behavior > clean-but-wrong-exit > loud-crash > ergonomics.

## Status Legend

| Status | Meaning |
|--------|---------|
| OPEN | not fixed in src/ |
| FIXED+TESTED | fix in src/, regression test green |
| FIXED-UNTESTED | treat as OPEN per user rule |

## Inventory

### SILENT-WRONG-BEHAVIOR

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| A | Missing `--workflow` file silently falls back to discovery benchmark (flan-t5 loaded by accident). Mechanism: `runner.rs:571-577` warn+`Ok(None)` on read err; `runner.rs:838-839` `.ok()?`; CLI `whitt.rs:370-372` discovery fallback. | reasoninging-enhancer-plus/docs/07-TRACKING.md:331-332,684; bit v14c + overcontext run | **FIXED+TESTED** 2026-08-24 |
| B | GWT lexer swallows brace-template `{{...}}` errors as quiet WARN "treating as false" — hid generator bug two versions. | reasoning-enhancer/REVIEW-CYCLES.md:143; SUMMARY-overcontext.md:81-82,92-94 | **FIXED+TESTED** 2026-08-24 |
| C | `/tmp` hardcoded in preflight df check (`df -B1 /tmp`), ignores TMPDIR. Shim: scripts/shims/df fakes 77GB. | SUMMARY-overcontext.md:88-91 (test drafted, reverted on build break — now buildable) | **FIXED+TESTED** 2026-08-24 |
| D | YAML `timing.min_tmp_space_mb` never reaches preflight (config built pre-parse; only CLI flag works). Note: runner.rs:2342-2345 DOES apply it post-load — CLI-config-built-pre-parse is the gap. | 07-TRACKING.md:284-286 | **FIXED+TESTED** 2026-08-24 (run() now loads workflow + applies timing overrides BEFORE preflight) |
| E | Loop-cap exhaustion exits 0 silently ("workflow loop exceeded N iterations" log only). Cap-scaling fixed; exit-code aspect unverified. | reasoning-enhancer/results/SUMMARY-benchmark.md:69-71 | **FIXED+TESTED** 2026-08-24 (exit-code aspect; cap-scaling landed earlier) |
| G | Relative `--out-dir` breaks engine shell hooks — no error, absolute required. | 07-TRACKING.md:109 | **FIXED+TESTED** 2026-08-24 (engine absolutizes output_dir at run() start; hooks get absolute WHITT_OUTPUT_DIR/save_to base) |
| H | Engine creates literal `${OUTPUT_DIR}` dir when `__OUTPUT_DIR__` substitution misses. Workaround: run-atom.sh:149-152 moves it post-run. | atomic-reasoning/scripts/run-atom.sh:149-152 | **FIXED+TESTED** 2026-08-24 (save_to/append_to/bookmark Fail on unresolved `${...}` placeholder paths) |
| I | VRAM misreport 8192.0 GB vs 8GB → auto-detect max_concurrent_inferences=4 → OOM crash. Workaround: WHITT_MAX_CONCURRENT_INFERENCES=1. | atomic-reasoning/SAFETY.md:5-9,88; run-atom.sh:81 | **FIXED+TESTED** 2026-08-24 (unit bug: sysfs bytes divided as KB; env override restored to committed src) |
| J | benchmark_report.json inaccurate under early exit ("3/5 angles" when 1 ran). | correction-atom/results/SUMMARY-v7.md:15-16 | **FIXED+TESTED** 2026-08-24 (entries carry `step_id`; suite carries `planned_steps`) |
| K | Engine-managed model swap `unload_unused: true` fails "Model not found". Workaround: shell-hook curl swap. | atomic-reasoning/benchmarks/SUMMARY.md:125-126 | **FIXED+TESTED** 2026-08-24 (root cause = fragile model resolution, not the flag; see Fix Log) |
| L | GWT route_to sometimes doesn't fire (m0477). Workaround: always-runs + feedback injection. Likely same root cause as B (eval error → false). | correction-atom/workflows/correction-atom-v4.yml:121 | **FIXED+TESTED** 2026-08-24 (same root cause as B: eval error previously quiet-false) |

### CLEAN-BUT-WRONG-EXIT

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| M | Model load/unload race — `/v1/models/unload` returns before unload completes; scripts need sleep 3/15. | atomic-reasoning/multi-model/atom-v1.yml:55,94 | **FIXED+TESTED** 2026-08-24 (backend path verifies; sleeps were raw-curl shell-hook only — see Fix Log) |
| N | Model name case sensitivity — server requires exact filenames, engine error unclear. | 07-TRACKING.md:61-63 | **PARTIAL** 2026-08-24 (resolution layer now case/separator tolerant — Issue K fix; server still needs exact id, engine passes resolved filename) |

### LOUD-CRASH

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| O | Duplicate log lines every step 2x — manual delete of dup `info!` at runner.rs:1975. Verify fixed; add regression if untested. | correction-atom/results/SUMMARY-v7.md:14 | **FIXED+TESTED** 2026-08-24 (dup existed in src: call-site + executor emissions; call-site deleted, source-count regression added) |
| P | Async-load trap: POST /models/load returns success before model loaded. Engine should poll/verify. | 07-TRACKING.md:22-26 | **FIXED+TESTED** 2026-08-24 (backend `load_model`/`unload_model` now poll status; benchmark client already polled) |

### ERGONOMICS

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| Q | whitt bin requires `client` feature — plain `cargo build --release` silently skips (stale-binary trap). | 07-TRACKING.md:253-254 | **FIXED+TESTED** 2026-08-24 (`default = ["client"]` — plain build produces binaries; `--no-default-features` opt-out preserved) |
| R | Schema doc `then: {route_to: X}` wrong vs Rust untagged enum (step.rs:255). | SUMMARY-v6.md:34; 07-TRACKING.md:226-228 | **FIXED+TESTED** 2026-08-24 (both forms accepted: custom Deserialize normalizes verbose `{route_to: ...}` → Single/Multiple; schema doc note added) |
| S | Foreign zombie llama-server procs trip engine preflight mid-run. Threshold env landed (WHITT_ZOMBIE_MAX); script guard run-probe-model.sh:32-35. | 07-TRACKING.md:58-59 | PARTIAL |

## Second-Wave Inventory (audit 2026-08-24, post-15-fix)

### SILENT-WRONG-BEHAVIOR

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| T | Discovery fallback alive via workflow SHAPE errors: unreadable path, invalid YAML, missing `agentic_workflow`, non-array/object `steps:`, empty steps — all silently → None → discovery loop loads random model (Issue A only killed missing-FILE door). | Mechanism: runner.rs `load_workflow_steps` `.ok()?` doors + `!workflow_ran` branch; same evidence class as 07-TRACKING.md:331-332 | **FIXED+TESTED** 2026-08-24 (all shape doors → hard Err "refusing to fall back") |
| AE | Map-format workflows pick "first" step via `values().next()` on BTreeMap = ALPHABETICAL, not YAML order → CLI-default max_tokens/temperature sampled from arbitrary step. | runner.rs `load_workflow_config` 657-663 | **FIXED+TESTED** 2026-08-24 (`serde_json` preserve_order — Map keeps YAML document order) |
| Z | ~~GWT `when:` clause dead~~ REFRAMED on investigation: schema-promised `requires: [{step: X, condition: expr}]` conditional dependencies (unified-workflow-schema.yml:411-414) silently DROPPED at parse — `condition:` never extracted, deps ran unconditionally. (GwtClause `when:` field = prose BDD naming per schema:396-397 — ignored by design, CLEARED.) | schema:411-414 vs runner.rs extract_dependency_names | **FIXED+TESTED** 2026-08-24 (conditions captured + evaluated via GWT; malformed expr → loud bail) |
| AA | `WHITT_STREAM=1` in workflow YAMLs but no code reads it — silent no-op knob. | experiments YAMLs; grep clean | **CLEARED** 2026-08-24 — repo-wide grep (src + experiments + docs, target/ excluded) finds ZERO occurrences of `WHITT_STREAM` anywhere. Audit-agent claim unsubstantiated; no such knob exists. Not a bug. |
| AB | Engine writes hardcoded `outputs/output/` regardless of YAML `save_to: outputs/json/` — artifact split. | RESOLVED Phase-1 note in AGENTS.md | **CLEARED** 2026-08-24 — historical, already fixed: phase-07 QA-FINDINGS-HOOK-CLEANUP.md:44 "YAML output paths corrected from outputs/json/ to outputs/output/"; all current YAMLs use `./docs/benchmarks/outputs/output/`; resolve_save_path (actions.rs:319) passes multi-segment relative paths through unchanged (author intent honored). No live defect. |
| AC | Schema-declared per-step `retry.max_attempts` (StepRetryConfig) parsed by workflow serde but never consulted by benchmark path — global inference_max_attempts + fixed 30s always applied. (`retry_on` itself = audit mislabel; no such literal exists.) | step.rs:403-427; schema:273-276 | **FIXED+TESTED** 2026-08-24 (parse step.retry.max_attempts → run_model_inference effective attempts; backoff/delays still global — documented limitation) |
| AD | Hooks fire AFTER benchmark metrics recorded — metrics can't reflect hook effects. | runner.rs after_step_succeeds Fail arm | **FIXED+TESTED** 2026-08-24 (audit's line refs pointed at deprecated fns; real bug = `HookResult::Fail` on after_step_succeeds only warned, step recorded SUCCESS; now sets model_result.error w/ hook reason) |
| F2 | `on_requires_failed` / `after_loop_iteration_fails` hooks fire but `execute_hooks_for_trigger` errors swallowed via `let _ =`. | runner.rs (sites were 2782/2849) | **FIXED+TESTED** 2026-08-24 (both sites now match + error!-log; source-count regression test) |

### CLEAN-BUT-WRONG-EXIT / LOUD-CRASH / DESIGN GAPS

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| U | `during_step_streaming` trigger dead (stream:false hardcoded). | gap analysis | **CLEARED (stale)** 2026-08-24 — streaming path EXISTS for workflows: run_model_inference_streaming (runner.rs ~3804) fires fire_during_streaming_hook per chunk (3856); selected when streaming_enabled && has_during_streaming_hook (execute_workflow_step 2314). stream:false remains only in benchmark_single_model (discovery mode, no steps). Audit claim stale. Default off; enable via YAML `workflow_execution_strategy.streaming` (Issue D wiring) or CLI. |
| V | `iterate_values` action zero readers — passthrough stub. | actions.rs ~577 | **CLEARED (misread)** 2026-08-24 — the HOOK action variant is an unused passthrough, but iterate_values the FEATURE is implemented runner-side (extract_iterate_values drives loops; 8 test_iterate_values_*_pipeline tests green). Hook-action variant predates runner impl; no workflow uses it. Low-priority cleanup at most. |
| W | `notify` = try_send into channel nobody receives — write-only. | actions.rs notify_tx | **DONE (docs)** 2026-08-24 — behavior documented: embedder channel if wired via with_notify(), else JSONL append to outputs/notifications.jsonl (verified live, 59 entries). Schema reference updated. |
| X | No wall-clock timeout for whole workflow run. | run-atom.sh `timeout` wrapper | **DESIGN PROPOSAL WRITTEN** 2026-08-24 (per user decision: proposal only, no impl — see Fix Log section: CLI `--max-wall-time` first, YAML key gated on schema approval, between-steps checking, non-zero exit + Y-synergy) |
| Y | No checkpoint/resume in benchmark runner — watchdog kill = total loss. | REVIEW-CYCLES.md OC-4 | **FIXED+TESTED** 2026-08-24 (minimal impl per user decision: record_step_output appends {step_id, output} to <output_dir>/checkpoint.jsonl at all 8 completion sites; `--resume` restores map + skips checkpointed steps via snapshot HashSet — runtime revisits NOT skipped; checkpoint IO failures logged, never fatal) |

### ERGONOMICS (second wave)

| # | Issue | Evidence | Status |
|---|-------|----------|--------|
| AF | README quickstart `whitt workflow --workflow file.yml` — `workflow` subcommand takes a POSITIONAL file, no `--workflow` flag (that is a `benchmark` flag). Copy-paste → clap "unexpected argument". | README:21 vs whitt.rs:207-215 | **FIXED** 2026-08-24 (README corrected: positional for `workflow`, `--workflow` example for `benchmark`; live-proven both shapes) |
| AG | atom-v1.yml raw-curl swap sleeps obsolete for engine-managed path. | multi-model/atom-v1.yml:55,94 | **DONE** 2026-08-24 (both sites annotated: sleeps REQUIRED for raw-curl bypass, engine path polls since Issue M/P; single-model variant has no sleeps) |

### LOW (parking lot)

- Topo-sort cycle silently drops steps (warn-only) — runner.rs ~2632-2635. **FIXED+TESTED 2026-08-24**: cycle_error() → hard bail (given_cycle_when_cycle_error_computed_then_some).
- Array steps missing `step:` key vanish via `filter_map` — runner.rs ~894. **FIXED+TESTED 2026-08-24**: hard Err naming entry index (given_array_step_missing_name_when_steps_loaded_then_hard_error).
- Registry `set_state` result ignored in tools.rs 194/207/275. **FIXED+TESTED 2026-08-24**: propagated + warn!-logged (given_tools_source_when_set_state_discards_counted_then_zero).


## FIXED+TESTED (verified green 2026-08-24)

| Issue | Test | Evidence |
|-------|------|----------|
| 100-step cap scaling (linear workflows) | given_linear_workflow_exceeding_100_steps_when_iteration_cap_computed_then_allows_full_run; given_no_loop_steps...floor_is_100; given_loop_max_iterations...override_and_scale | SUMMARY-benchmark.md:69-71 (cap aspect) |
| Router-mode zombie false positive | given_router_mode_zombie_command_when_built_then_excludes_router_process | 07-TRACKING.md:372-378 |

## Fix Log

### 2026-08-24 — Issue A: missing --workflow file silent discovery fallback

- **Original evidence:** `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:331-332,684`: "missing --workflow file silently falls back to discovery benchmark (flan-t5 loaded by accident) — should hard-error." Bit v14c + overcontext run.
- **Root cause (investigated):** Two swallow points. (1) `src/benchmark/runner.rs` `load_workflow_config()`: `fs::read_to_string` error → `warn!` + `return Ok(None)` — engine then treats run as "no workflow" and proceeds to model discovery (loads whatever is in models_dir, e.g. flan-t5). Invalid YAML already hard-errored ("cycle-3 hardening") but the read-error path was missed. (2) `load_workflow_steps()` `.ok()?` silently returns None (shadowed by fix 1 in run() ordering). Preflight runs BEFORE config load in `run()`, so even a hard error there came too late and after Docker/network contact.
- **Fix (minimal, two layers of same bug):**
  - `runner.rs` `load_workflow_config()`: read error → `Err(anyhow!("cannot read --workflow file ... refusing to fall back to discovery benchmark"))`.
  - `src/bin/whitt.rs` `Commands::Benchmark` arm: early `Path::exists()` check → `anyhow::bail!("--workflow file not found: ... no fallback to discovery benchmark")` before preflight/network.
- **Tests (both observed RED before fix, GREEN after):**
  - Unit: `benchmark::runner::tests::given_missing_workflow_file_when_config_loaded_then_hard_error` (RED: returned Ok → panic "must hard-error").
  - Integration (binary spawn, no server needed): `tests/cli_workflow_file.rs::given_benchmark_with_missing_workflow_file_when_run_then_hard_error` (RED: preflight Docker error, no workflow mention; GREEN: exits non-zero naming the file).
- **Verification:** full `cargo test --all-features` — 567 lib + all integration suites, 0 failures. Clippy: 0 new warnings from this change (13 pre-existing in untouched code, documented). LSP: 0 errors on runner.rs, whitt.rs, tests/cli_workflow_file.rs.
- **Failure mode dead:** the exact sequence "wrong/missing path → warn-only → discovery benchmark loads random model" can no longer occur: CLI bails first; runner hard-errors if reached programmatically.
- **Obsolete workarounds:** none existed (user noted "needs engine fix").

### 2026-08-24 — Issues B + L: GWT eval errors quiet-false ("treating as false" WARN)

- **Original evidence:** `experiments/reasoning-enhancer/REVIEW-CYCLES.md:143`: "engine GWT lexer error → 'treating as false' WARN (log line 43-style)". `results/SUMMARY-overcontext.md:81-82`: "generator wrote gwt `given` through Python `.format()` with `{{...}}` → collapsed to single braces `{bookmarks...}` → engine GWT lexer error, treated as false (WARN in logs, unnoticed)"; `:92-94`: "GWT-lexer WARN on brace-templates is silent-ish — a `{{...}}` reaching the lexer unchanged deserves a louder signal (generator-side bug was invisible for two versions)". Issue L: `experiments/correction-atom/workflows/correction-atom-v4.yml:121` "GWT route unreliable (m0477)".
- **Root cause (investigated, TWO swallow layers):**
  1. `src/workflow/hooks/actions.rs:670-678` `evaluate_gwt_condition()`: `gwt::evaluate` Err → `tracing::warn!("...treating as false")` → returns `false`. Single GWT warn in hooks tree; sole caller `execute_gwt()`.
  2. `src/workflow/hooks/gwt.rs` `Parser::parse()` (~398-400): called `parse_or_expr()` with NO trailing-token check — garbage like `invalid condition syntax` parsed the first word as an IdentPath, silently ignored the rest → Ok(false), not even the WARN fired.
- **Fix (minimal):**
  - `actions.rs`: `evaluate_gwt_condition()` → `Result<bool, String>`; `execute_gwt()` Err arm → `tracing::error!`, fires `after_gwt_evaluates` decision="error", returns `HookResult::Fail{reason}` quoting the expression; appends template hint when expression still contains `{{`/`}}` ("unresolved brace template reached the GWT lexer — check template substitution / bookmark names upstream").
  - `gwt.rs`: `Parser::parse()` now errors on trailing tokens (`Unexpected trailing tokens: ...`).
  - Semantics preserved: missing bookmark/field → Ok(false) → Continue (quiet-false for data-driven conditions unchanged); only syntax/eval errors now Fail.
- **Tests (observed RED before fix, GREEN after):**
  - `given_gwt_invalid_condition_when_execute_then_fails_loudly` (rewritten from `..._treats_as_false` which encoded the buggy spec) — RED via trailing-token swallow (returned Continue), fixed by gwt.rs EOF check.
  - `given_gwt_unresolved_brace_template_when_executed_then_fails_with_template_hint` — RED via actions.rs warn-swallow, fixed by Err→Fail.
  - Guards kept green: `given_gwt_clause_matching_when_execute_then_routes_to_target`, `given_gwt_clause_not_matching_when_execute_then_returns_continue`; gwt module suite 63/63 (incl. missing-field quiet-false).
- **Verification:** full `cargo test --all-features` — 568 lib + all integration suites, 0 failures. Clippy: 0 new warnings (13 pre-existing baseline untouched). LSP: 0 errors on actions.rs, gwt.rs.
- **Failure mode dead:** "lexer error → WARN → false" can no longer occur: eval errors now `HookResult::Fail` with quoted expression + template hint, `error!`-logged, and reported via `after_gwt_evaluates` decision="error". Workflows with generator brace-collapse bugs now fail loudly at the step instead of silently mis-routing (Issue L mechanism).
- **Obsolete workarounds:** correction-atom v4 "always-runs + feedback injection" pattern (correction-atom-v4.yml:121) was a defense against quiet mis-routing — no longer required for correctness, but leave experiment files untouched (historical record; scripts may still depend on pattern).

### 2026-08-24 — Issue C: /tmp hardcoded in preflight, ignores TMPDIR

- **Original evidence:** `experiments/reasoning-enhancer/results/SUMMARY-overcontext.md:88-91`: "runner.rs:348 /tmp path hardcoded — should honor `TMPDIR` (TDD'd test drafted, reverted: worktree cargo cannot build llama-cpp-sys under cmake 4.4 + MAIN repo has uncommitted foreign runner.rs change)" — build blocker now resolved, test landed.
- **Root cause (investigated):** three `Path::new("/tmp")` hardcodes in `src/benchmark/runner.rs` — `preflight_check()` (~362), deprecated `check_system_health()` (~426), deprecated `log_resource_state()` (~461). Foreign processes saturating /tmp (live whisper_stream fds; unlink frees nothing) aborted preflight even with TMPDIR pointing at a 77GB-free volume.
- **Fix (minimal):** `BenchmarkRunner::tmp_root()` — TMPDIR (non-empty) wins, else `/tmp`. All three sites now use it; log/error messages report the actual checked path.
- **Test:** `benchmark::runner::tests::given_tmpdir_env_when_tmp_root_resolved_then_env_wins_over_hardcoded_tmp` (RED as compile error — fn absent; GREEN after fix). Covers: TMPDIR set → wins; TMPDIR empty → /tmp; TMPDIR unset → /tmp.
- **Verification:** full `cargo test --all-features` — 569 lib + all integration suites, 0 failures. Clippy: 13 warnings = exact pre-existing baseline (mod.rs unused imports, dead code, casts — untouched). LSP: 0 errors on runner.rs.
- **Failure mode dead:** "TMPDIR set elsewhere + /tmp full → preflight abort" can no longer occur — checks statvfs the TMPDIR volume and names it in messages.
- **Workaround retired:** `experiments/reasoning-enhancer/scripts/shims/df` PATH-shim (faked `df -B1 /tmp` reporting 4GB avail; injected by run-overcontext.py:60) — annotated OBSOLETE in-file, kept for historical-run reproducibility. Modern path: set TMPDIR to the output workspace.

### 2026-08-24 — Issue E: loop-cap exhaustion exits 0 silently

- **Original evidence:** `experiments/reasoning-enhancer/results/SUMMARY-benchmark.md:69-71`: "`runner.rs` workflow loop hard cap: 100 iterations (silent exit 0 after — log shows `workflow loop exceeded 100 iterations`). Batch designs must chunk ≤ ~86 steps." Cap-scaling aspect landed earlier (`max_workflow_iterations()`, floor 100 / 4x steps / loop-override wins); the exit-code aspect remained: warn-only at loop exit.
- **Root cause (investigated):** `src/benchmark/runner.rs` workflow loop condition `while current_index < steps.len() && loop_count < max_loop_iterations`; at loop exit, `if loop_count >= max_loop_iterations { warn!(...) }` — WARN ONLY, then `run()` fell through to completion → `Ok` → exit 0. Work abandoned mid-workflow still reported success.
- **Fix (minimal):** new `BenchmarkRunner::premature_cap_error(loop_count, max_loop_iterations, current_index, steps_len) -> Option<String>` — `Some(msg)` only when cap hit AND steps unexecuted; boundary case (workflow completes exactly at cap, `current_index == steps_len`) stays success. Wired at the old warn site: `error!` + `anyhow::bail!(crate::error::Error::benchmark(msg))` → non-zero exit (propagation proven by Issue A integration test).
- **Tests (observed RED as E0599 compile error before fix, GREEN after):**
  - `given_cap_exhausted_with_steps_unexecuted_when_cap_error_computed_then_some` — (100,100,12,100) → Some, message names cap + 88 unexecuted.
  - `given_workflow_completed_at_cap_when_cap_error_computed_then_none` — (100,100,100,100) boundary → None; (42,100,42,42) normal → None.
- **Verification:** full `cargo test --all-features` — 571 lib + all integration suites, 0 failures. Clippy: 13 warnings = exact pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "loop cap hit → warn → exit 0 with steps unexecuted" can no longer occur — run() bails with an error naming the cap and unexecuted-step count; success reporting (benchmark_report) is skipped on that path.
- **Obsolete workarounds:** none removable — gen-batch-baseline.py chunking (≤86 steps) was a cap-scaling workaround already obsoleted by `max_workflow_iterations()`; leave experiment scripts untouched.

### 2026-08-24 — Issue D: YAML timing.min_tmp_space_mb never reached preflight

- **Original evidence:** `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:284-286`: "engine preflight honors CLI flag — note YAML timing.min_tmp_space_mb did NOT reach preflight [config built pre-parse] — CLI flag is the reliable path".
- **Root cause (investigated):** `run()` ordering — `preflight_check()` (disk gate at runner.rs:373 reads `self.config.min_tmp_space_mb`) ran BEFORE `load_workflow_config()` and the YAML override block (which did set `config.min_tmp_space_mb` at what was ~2385, but only after preflight had consumed the CLI value). Path in YAML: `workflow_execution_strategy.timing.min_tmp_space_mb` (parsed at runner.rs:783-797).
- **Fix (minimal):** extracted the override block into `BenchmarkRunner::apply_workflow_timing_overrides(&mut self, ctx)` (all scalar/timing overrides: inference_max_attempts, refusal_patterns, output_root, cooldown, load_timeout, min_tmp_space, streaming, detail_template, model_filter — logic unchanged) and moved workflow-config load + override application to the top of `run()`, before `preflight_check()`. Side benefits: YAML `output_root` now also applies before `ensure_output_dirs()`/hook-engine output_dir wiring; `--preflight-only` mode now checks the YAML-configured threshold. `load_params` (Docker restart) block untouched — stays after client creation.
- **Test (observed RED as E0599 compile error + schema-validation failure during development, GREEN after):** `given_workflow_yaml_min_tmp_space_when_timing_overrides_applied_then_config_updated` — workflow YAML with `workflow_execution_strategy.timing.min_tmp_space_mb: 2048`, config starts at CLI `1`, after `apply_workflow_timing_overrides(&ctx)` config reads `2048`. (Dev note: first test draft used top-level `timing:` — schema rejected it with allowed-keys list; corrected to nested path, proving validation gate active.)
- **Verification:** full `cargo test --all-features` — 775 tests total (572 lib + all integration suites), 0 failures. Clippy: 13 warnings = exact pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "YAML timing.min_tmp_space_mb silently ignored by the preflight disk gate" can no longer occur — overrides apply before preflight reads the value; `info!` log line "workflow YAML overrides min_tmp_space_mb: N → M" now fires pre-preflight.
- **Obsolete workarounds:** experiments relied on CLI `--min-tmp-space` flag only; no script to retire.

### 2026-08-24 — Issue G: relative --out-dir breaks engine shell hooks

- **Original evidence:** `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:109`: "relative --out-dir breaks engine shell hooks (always absolute)".
- **Root cause (investigated):** `config.output_dir` (CLI `--output-dir` / YAML `output_root`) flowed verbatim — relative values reached `execute_shell()`'s `WHITT_OUTPUT_DIR` env (actions.rs:523-525), save_to base resolution (actions.rs:312), and hook `working_dir` joins, where they resolved against whichever CWD the hook process had. Launch whitt from a different directory → hook outputs land elsewhere or hooks "skip" silently.
- **Fix (minimal):** `BenchmarkRunner::normalized_output_dir()` — relative → `std::path::absolute()` against process CWD (no symlink resolution, no existence requirement); absolute unchanged. Wired in `run()` after YAML overrides, before preflight/`ensure_output_dirs`/hook-engine wiring — single normalization point covers CLI + YAML sources.
- **Test (observed RED as E0599 compile error, GREEN after):** `given_relative_output_dir_when_normalized_then_absolute_against_cwd` — "outputs/run-42" → absolute + cwd-joined; "/tmp/abs-out" unchanged.
- **Verification:** full `cargo test --all-features` — 776 total passed, 0 failures. Clippy: 13 = pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "relative out-dir + hooks writing to wrong place / skipping" can no longer originate from `config.output_dir` — hooks always receive an absolute base.
- **Obsolete workarounds:** scripts' "always absolute --out-dir" convention (documented habit, not a script to retire).

### 2026-08-24 — Issue H: literal `${OUTPUT_DIR}` directory created on placeholder miss

- **Original evidence:** `experiments/atomic-reasoning/scripts/run-atom.sh:149-152`: "# Move literal-${OUTPUT_DIR} outputs (legacy fallback) / LITERAL_DIR=\"${REPO_ROOT}/\${OUTPUT_DIR}\" / if [ -d ... ]; then mv ... ; rmdir" — post-run cleanup of a directory the engine had created with a literal `${OUTPUT_DIR}` name.
- **Root cause (investigated):** engine's only `${...}` support is structural references (`${models.*}` etc., src/model/interpolation.rs) — no path-placeholder substitution. A YAML `save_to`/`append_to` path carrying `${OUTPUT_DIR}/x` passed through `resolve_save_path()` verbatim; `save_to_file()`/`append_to_file()` then ran `fs::create_dir_all(parent)` on the literal placeholder name and wrote files into it. Silent wrong-location output.
- **Fix (minimal):** `unresolved_placeholder_error(path)` helper (detects `${` followed by `}`) + guard before every file-materializing hook write — `execute_save_to` (all 3 arms), `execute_append_to` (all 3 arms), `execute_bookmark` (path variants). Guard returns `HookResult::Fail{reason}` quoting the path; no directory is created.
- **Tests (observed RED — both returned Continue and the literal dir WAS created during red run (then removed) — GREEN after):**
  - `given_save_to_with_unresolved_placeholder_when_executed_then_fails_not_literal_dir` — `${OUTPUT_DIR}/result.json` → Fail with quoted path + "placeholder"; asserts no literal dir exists.
  - `given_append_to_with_unresolved_placeholder_when_executed_then_fails_not_literal_dir` — same for append.
- **Verification:** full `cargo test --all-features` — 778 total passed, 0 failures. Clippy: 13 = pre-existing baseline, 0 new. LSP: 0 errors on actions.rs. Repo root clean (no `${OUTPUT_DIR}` dir).
- **Failure mode dead:** "engine silently creates a directory literally named `${...}` and stashes outputs there" can no longer occur via save_to/append_to/bookmark — placeholder paths Fail loudly at the step.
- **Workaround status:** run-atom.sh:149-152 legacy-fallback block now dead code for engine-created dirs — left in place (harmless `if [ -d ]` no-op; historical runs may still need it).

### 2026-08-24 — Issue J: benchmark_report.json lies under early exit ("3/5 angles")

- **Original evidence:** `experiments/correction-atom/results/SUMMARY-v7.md:15-16`: "metrics.json lied under early exit ('3/5 angles')" — workaround "Reads select-best.json: selected_angle + case_passed + real file count". Real reports (exp-001..006) confirmed: ALL entries carry the IDENTICAL model_id (model filename ×7) — zero step attribution.
- **Root cause (investigated):** `ModelBenchmarkResult` had no step attribution — every workflow result tagged only with the model file name; consumers could not attribute entries to steps/angles nor distinguish executed vs hook-skipped. `BenchmarkSuiteResult` carried `total_models = results.len()` with no planned-steps count, so under early exit (route_to/skip_remaining), consumers mis-derived counts ("3/5 angles" when 1 ran).
- **Fix (minimal, additive — byte-identical JSON in model-discovery mode):**
  - `ModelBenchmarkResult.step_id: Option<String>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` — set to `Some(step_id)` at every workflow-path construction (3 hook arms in `execute_workflow_step`, the fn-end result binding, `model_resolution_failure_result`); `None` at all model-discovery/deprecated paths.
  - `BenchmarkSuiteResult.planned_steps: Option<usize>` (same serde style) — `Some(steps.len())` for workflow runs; suite assembly extracted into `BenchmarkRunner::suite_result_from(results, planned_steps, server_url)`.
- **Tests (observed RED as E0599/E0609 compile errors before fix, GREEN after):**
  - `given_hook_skipped_step_when_executed_then_result_attributed_to_step` — hook-skipped step's result now carries `step_id == Some("angle_one")` + error "Skipped by hook" (dead-port client guard: hook arm returns before any client use).
  - `given_results_and_planned_steps_when_suite_assembled_then_counts_honest` — 3 results/5 planned → total 3, successful 2, failed 1, planned Some(5); empty + None → serialized JSON has NO `planned_steps` key (model-mode byte-identical).
- **Verification:** full `cargo test --all-features` — 580 lib + all integration suites, 0 failures. Clippy: 13 = exact pre-existing baseline, 0 new. LSP: 0 errors on mod.rs, runner.rs, detail_generator.rs.
- **Failure mode dead:** "report shows N/M angles after early exit" mis-derivation can no longer occur — entries are per-step attributable (`step_id`), skips are identifiable (`step_id` + error "Skipped by hook"/"Routed by hook"), and the suite states how many steps the workflow planned (`planned_steps`).
- **Obsolete workarounds:** correction-atom's "read select-best.json for real counts" remains valid for its own selection semantics; engine report no longer lies, so the workaround is optional robustness, not a necessity.

### 2026-08-24 — Issue K: engine-managed swap fails "Model not found" (fragile model resolution)

- **Original evidence:** `experiments/atomic-reasoning/benchmarks/SUMMARY.md:125`: "v1.0 | GPU | Engine-managed swap | Model not found" (workaround rows: shell swap hook, then shell+curl router). `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:61-63`: "Model name case sensitivity: server requires exact filenames (Qwen3-5-9B not Qwen3.5-9B)".
- **Root cause (investigated):** the failure text comes from the engine's own `model_resolution_failure_result` (runner.rs:597 "Model '{}' not found on server for workflow step"). Trigger: `resolve_model_file()` matched ONLY by raw case-sensitive substring (`id.contains(name)`) plus an aggressive base-name fallback — YAML names drifting from filenames in case ("ministral-3b" vs "Ministral-3B") or separator spelling ("Qwen3.5-9B" vs "Qwen3-5-9B.gguf") failed BOTH layers → resolution None → step died "Model not found"; when multiple variants existed, the substring loop could also bind an arbitrary sibling first. Secondary finding (investigated, NOT the bug): `workflow_execution_strategy.memory.model_lifecycle.unload_unused` is parsed (execution.rs:117) but has ZERO readers — dead config; the unload-others-then-load behavior it promises already runs unconditionally inside `run_model_inference()`, so the flag is a redundant no-op rather than the failure source.
- **Fix (minimal, layered matcher in `resolve_model_file`):**
  1. exact id (with/without `.gguf`) — deterministic, wins over substring siblings;
  2. normalized equality — case-insensitive, `.`/`-`/`_` unified, `.gguf` stripped — kills the "Qwen3.5-9B vs Qwen3-5-9B" and case-drift failure classes;
  3. legacy substring; 4. legacy base-name fallback — unchanged for backward compatibility.
  Server contract preserved: the RESOLVED filename id is what flows to `/models/load` (exact id at API boundary, per TUTORIAL.md:332).
- **Tests (observed RED before fix — all three panicked on resolution returning None/wrong file — GREEN after):**
  - `given_dot_dash_spelling_drift_when_model_resolved_then_normalized_match`
  - `given_case_mismatch_when_model_resolved_then_case_insensitive_match` (also covers Issue N resolution-layer aspect)
  - `given_exact_name_and_variant_when_model_resolved_then_exact_wins`
- **Verification:** full `cargo test --all-features` — 786 total passed, 0 failures (583 lib). Clippy: 13 = exact pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "valid model in models_dir + spelling/case drift in YAML → step fails 'Model not found'" can no longer occur — exact or normalized match resolves it and the exact filename is passed downstream.
- **Workaround status:** atomic-reasoning shell-hook curl swap (atom-v1.yml:55,94) was a defense against engine resolution/swap failure — engine path now resolves drift-tolerantly; leave experiment files untouched (historical). Issue N remains PARTIAL: engine-side resolution tolerant, but server-side load of a nonexistent id still surfaces the server's own error text (acceptable loudness).

### 2026-08-24 — Issues P + M(backend half): fire-and-forget load/unload ACKs trusted

- **Original evidence:** `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:23-25`: "POST /models/load returns `{"success":true}` in ~13ms = fire-and-forget ACK, NOT load confirmation. Always verify via /models status=loaded (wait-loaded.py)". `experiments/atomic-reasoning/multi-model/atom-v1.yml:55,94`: raw-curl swap hooks with hand-rolled `sleep 3` / `sleep 15`.
- **Root cause (investigated, split by path):**
  - Benchmark client path (`LlamaHttpClient::load_model`/`unload_model`, http_client.rs:146,164): ALREADY polls `wait_for_model_status` — no bug there.
  - Backend path (`LlamaCppVulkanBackend::load_model`/`unload_model`, llama_vulkan.rs): returned `Ok(())` the moment the POST returned HTTP 200 — trusting the fire-and-forget ACK. This is the path the agent tools (`agent/tools.rs:192,273`) use. `unload_model` had the identical gap.
  - The experiment sleeps (M) guard RAW-CURL shell hooks that bypass the engine entirely — script-level pattern; the engine-side gap was the backend trait impl.
- **Fix (minimal):** new `LlamaCppVulkanBackend::wait_until_model_status(base_url, model_id, loaded)` — polls `GET /v1/models` every 250ms up to 240s; loads require `status=loaded`; unloads accept `status=unloaded` OR absence from the list (servers drop unloaded entries; status field is a whitt-server extension → untyped JSON parsing). Both `load_model`/`unload_model` success arms now verify instead of trusting the ACK; timeout → `LlmError::Timeout` naming model + expected status.
- **Tests (observed RED — both panicked "saw 0 polls", proving the ACK-only return — GREEN after):** mock llama-server on a plain `std::thread` TcpListener (blocking accept on the current_thread test runtime would starve the client future):
  - `given_loading_status_when_load_ack_received_then_waits_until_loaded` — server ACKs instantly, reports `loading` for 2 polls then `loaded`; asserts Ok AND ≥3 status polls before return.
  - `given_still_listed_when_unload_ack_received_then_waits_until_gone` — server ACKs, keeps model listed once, then empty list; asserts Ok AND ≥2 polls.
- **Verification:** full `cargo test --all-features` — 788 total passed, 0 failures. Clippy: 13 = exact pre-existing baseline, 0 new. LSP: 0 errors on llama_vulkan.rs.
- **Failure mode dead:** "backend `load_model` returns Ok while model still loading → immediate chat_completion fails / hits previous model" can no longer occur on the backend path; same for unload. Raw-curl shell hooks remain outside engine control (their sleeps are inherent to bypassing the engine).
- **Workaround status:** experiment `wait-loaded.py` + sleep-laden curl hooks remain necessary only for scripts that bypass the engine; engine-mediated loads/unloads are now self-verifying on both client and backend paths.

### 2026-08-24 — Issue O: duplicate step log lines (2× per step)

- **Original evidence:** `experiments/correction-atom/results/SUMMARY-v7.md:14`: "Every log line written 2× | Deleted duplicate `info!` at main-loop call site (kept inner log, runner.rs:1975) | Live-verified: 1 line/step." Same doc: source later "restored byte-exact (md5 match)" — i.e. the fix never landed in src/.
- **Root cause (investigated):** TWO emissions of `"[benchmark] executing step {} with model {}"` in `src/benchmark/runner.rs` — one at the run() workflow-loop call site (~2781), one inside `execute_workflow_step` (~2010). The experiment deleted the call-site copy in a main-repo build, verified live, then reverted — so worktree src still had both.
- **Fix (minimal):** deleted the call-site `info!` (kept executor's inner emission, matching the experiment's live-verified choice); left a guard comment at the call site explaining the single-emission contract.
- **Test:** `benchmark::runner::tests::given_workflow_step_execution_when_emission_sites_counted_then_exactly_one` — source-count regression (`include_str!("runner.rs")` + `concat!`-split needle `executing step` + ` {} with model`, asserting exactly 1 match). No live seam exists without a server; source-count pins the invariant. Observed RED at count=2 (dev note: first draft self-matched its own literal — fixed by building the needle with `concat!` so the test's own text cannot match).
- **Verification:** full `cargo test --all-features` — 779 total passed, 0 failures. Clippy: 13 = pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "every step logged twice" cannot regress silently — the source-count test fails on any re-addition of a second emission site.
- **Obsolete workarounds:** none (log-parsing consumers saw duplicate lines; nothing to retire).

### 2026-08-24 — Issue I: VRAM misreport 8192.0 GB → concurrency 4 → OOM; env override never committed

- **Original evidence:** `experiments/atomic-reasoning/SAFETY.md:5-9`: "Auto-detecting `max_concurrent_inferences: 4` (4 parallel inferences). Each inference allocated its own KV cache... Combined VRAM + RAM pressure exhausted 16GB system memory. Swap thrashing → load avg spike → unresponsive system → hard reboot required." `:88`: "The '8192.0 GB VRAM available' was a misreport — actual is 8GB." Workaround `run-atom.sh:81`: `export WHITT_MAX_CONCURRENT_INFERENCES=1`; experiment run.logs show `[benchmark] WHITT_MAX_CONCURRENT_INFERENCES=1 overridden (clamped to 1)`.
- **Root cause (investigated, TWO parts):**
  1. Unit bug: `read_sysfs_vram_amd()` reads amdgpu sysfs `mem_info_vram_total`, which reports **bytes**; `detect_vram_gb()` divided as KB — RX 580 8GiB = 8589934592 bytes → 8589934592/1024/1024 = **8192.0 GB** (exact match to the misreport) → `(8192/2).clamp(1,4)` = 4 concurrent → OOM.
  2. Lost workaround: `WHITT_MAX_CONCURRENT_INFERENCES` appears NOWHERE in committed src (`git log -S` over history: never existed). The override only ever lived in an uncommitted main-repo experiment build — the documented knob was a placebo against committed code (same lost-fix pattern as Issue O).
- **Fix (minimal):** new `vram_bytes_to_gb(bytes)` (÷1024³) used by `detect_vram_gb()`; new `env_concurrency_override()` parsed at the top of `detect_max_concurrent_inferences()` (integers ≥1; logs the experiment-format line `WHITT_MAX_CONCURRENT_INFERENCES=n overridden`); corrected `read_sysfs_vram_amd` doc/var names to bytes.
- **Tests (observed RED as E0599/E0425 compile errors — fns absent — GREEN after):**
  - `given_rx580_vram_bytes_when_converted_then_reports_8gb_not_8192` — 8589934592 → 8.0 GB.
  - `given_env_concurrency_override_when_parsed_then_valid_wins_and_invalid_ignored` — "1"→Some(1); "0"/"abc"/unset→None (saves+restores env).
- **Verification:** full `cargo test --all-features` — 578 lib + all integration suites, 0 failures. Clippy: 13 warnings = exact pre-existing baseline, 0 new. LSP: 0 errors on runner.rs.
- **Failure mode dead:** "8192.0 GB VRAM available" cannot recur — sysfs bytes now convert via ÷1024³ (8.0 GB on the same hardware), pinned by the conversion test. The escape hatch for the 2GB-per-inference heuristic overestimating small-VRAM boxes now actually exists in committed src.
- **Honest scope note:** with correct units, an 8GB card still auto-detects 4 by the existing 2GB-per-inference rule — the heuristic's tuning is a design choice, not the documented bug; the experiment-proven mitigation (env=1) is restored and tested.
- **Workaround status:** run-atom.sh:81 export now effective against committed engine (was placebo); leave script as-is.

### 2026-08-24 — Issue Q: whitt bin silently skipped by plain `cargo build` (stale-binary trap)

- **Original evidence:** `experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:253-254`: "(--all-features; note: whitt bin requires `client` feature — plain `cargo build --release` silently skips it, stale-binary trap)."
- **Root cause (investigated):** `Cargo.toml` had `default = []` while all three `[[bin]]` targets (`whitt`, `poc_client`, `model_chain`) declare `required-features = ["client"]`. A plain `cargo build` / `cargo build --release` therefore compiled the lib and exited 0 WITHOUT building any binary — while an older binary sat in `target/`, so `./target/release/whitt ...` ran stale code with no warning.
- **RED (observed live in worktree):** `touch src/bin/whitt.rs` → `cargo build` → "Finished dev profile in 7m04s" with `target/debug/whitt` mtime UNCHANGED (15:11 before and after; wall clock 15:20). Source changed, plain build succeeded, binary stale — exact trap reproduced.
- **Fix (minimal, one line + comment):** `[features] default = ["client"]`. Plain builds now satisfy the bins' required features. Opt-out preserved: `cargo check --no-default-features` still compiles lib-only (verified).
- **GREEN (observed):** same plain `cargo build` → `Compiling whitt-execution-engine` → fresh `target/debug/whitt` + `poc_client` + `model_chain` (mtime = build time, 15:22).
- **Verification:** full `cargo test --all-features` — 788 passed, 0 failures (feature superset unaffected). Clippy `--all-features -W clippy::all`: 13 warnings = exact pre-existing baseline, 0 new. LSP: no diagnostics on Cargo.toml.
- **Failure mode dead:** "edit whitt.rs → plain `cargo build`/`cargo build --release` exits 0 → run stale binary" can no longer occur — the binaries are default build targets now. The silent part died; there is nothing left to miss.
- **Obsolete workarounds:** the experiment habit "always build with `--all-features`" (07-TRACKING.md:253) is no longer required for the binaries (still fine for sqlite/clipboard features).

### 2026-08-24 — Issue T: discovery fallback via workflow SHAPE errors (second-wave)

- **Original evidence class:** same as Issue A ("missing --workflow file silently falls back to discovery benchmark (flan-t5 loaded by accident)" — 07-TRACKING.md:331-332). Issue A killed the missing-FILE door; audit found the same silent fallback alive via shape errors.
- **Root cause (investigated):** `load_workflow_steps()` had five silent doors returning `None`: unreadable path `.ok()?` (882), YAML parse `.ok()?` (884), missing `agentic_workflow` `?` (885), `steps:` neither array nor object → `return None` (946-947), and empty steps vec (→ `workflow_ran` false). All landed in the `!workflow_ran` discovery branch, loading a random model instead of erroring.
- **RED (watched, 5 tests, E0308 mismatched types ×5):** `given_empty_steps_array_when_steps_loaded_then_hard_error`, `given_scalar_steps_when_steps_loaded_then_hard_error`, `given_unreadable_workflow_path_when_steps_loaded_then_hard_error`, `given_no_agentic_workflow_section_when_steps_loaded_then_hard_error`, `given_invalid_yaml_when_steps_loaded_then_hard_error` — all assert Err + "refusing to fall back".
- **Fix (minimal):** signature → `Result<Option<Vec<WorkflowStep>>>`; every silent door → `anyhow` Err naming file + reason + "refusing to fall back to discovery benchmark"; `workflow_file: None` stays `Ok(None)` (legit discovery mode). `run()` call site gets `?`.
- **GREEN:** 5/5 pass. Full `cargo test --all-features`: 793 total, 0 failures. Clippy: 13 = pre-existing baseline, 0 new. LSP: 0 errors runner.rs.
- **Failure mode dead:** a malformed, empty, or unreadable workflow can no longer silently become a discovery benchmark run — the exact v14c/overcontext accident shape now hard-errors before any model loads.

### 2026-08-24 — Issue AE: map-format "first step" was alphabetically-first key

- **Original evidence:** audit of `load_workflow_config` (runner.rs 657-663): map-format `steps_map.values().next()` — serde_json Map is BTreeMap (sorted keys) without preserve_order, so "first" = alphabetically-first key, not YAML document order. CLI-default sampling (temperature/max_tokens/top_p) could come from an arbitrary step.
- **Root cause:** dependency-level: `serde_json` default Map loses document order at YAML parse time.
- **RED (watched):** `given_map_format_steps_when_first_step_sampled_then_yaml_order_wins` — map steps `z_first` (temperature 0.2, max_tokens 512, first in document) then `a_second` (0.9, 2048). Failed: `temperature must come from z_first (YAML-first), got 0.9 (alphabetical pick?)`.
- **Fix (minimal):** Cargo.toml `serde_json = { version = "1.0", features = ["preserve_order"] }` — Map = IndexMap, document order preserved. Bonus: map-format step iteration + providers/models picks now follow YAML order too.
- **GREEN (observed):** test passes; full `cargo test --all-features` 794 passed 0 failures (591 lib) — no order-dependent test broke under the global Map-ordering change. Clippy 13 = baseline. LSP clean.
- **Failure mode dead:** map-format workflows can no longer sample CLI-defaults from the wrong step — document order is preserved end-to-end.

### 2026-08-24 — Issue Z: `requires: [{step, condition}]` conditions silently dropped

- **Original evidence:** schema documents conditional dependencies (unified-workflow-schema.yml:411-414: `requires: [{step: X, condition: "result.config_loaded == true"}]`) but `extract_dependency_names` (runner.rs:998-1022) captured only the `step` key — the `condition` string was discarded at parse time and the run() check (2696-2715) tested mere output presence. Conditional dependencies were silently unconditional.
- **Root cause:** parse-layer drop; no storage, no evaluation.
- **Fix (minimal):** `WorkflowStep.require_conditions: HashMap<String,String>` captured at parse; `eval_require_condition(output, expr)` parses upstream output as JSON (non-JSON falls back to `{"result": {"output": text}}`) and evaluates via the GWT evaluator — same expression language as hooks. Dep check: missing OR condition-false → on_requires_failed skip path (reason distinguishes unsatisfied); malformed condition → loud `anyhow::bail!` (B/L precedent — never silently satisfied). 9 execution-copy literals + test literals filled.
- **Tests:** `given_conditional_requirement_when_steps_loaded_then_condition_preserved` (parse captures condition); `given_upstream_json_output_when_require_condition_evaluated_then_reflects_fields` (true/false/non-JSON fallback). Both watched RED (E0609/E0599) before fix.
- **Verification:** full suite 796 total 0 fail (592 lib); clippy 13 baseline 0 new; LSP clean.
- **Failure mode dead:** a `condition:` on a requirement now either gates execution or fails loudly — it can no longer be ignored.
- **Reframe note:** GwtClause `when:` (step.rs:224) is never evaluated — but schema:396-397 shows it as prose BDD naming ("when all validation checks passed"), not an executable expression. Ignoring it is by design → CLEARED, not a bug.

### 2026-08-24 — Issue AC: per-step `retry.max_attempts` silently ignored

- **Original shape:** audit reported "`retry_on` ignored" — literal `retry_on` exists nowhere (mislabel). Real finding: schema/serde declare per-step retry config (StepRetryConfig: max_attempts, backoff, initial_delay, max_delay, multiplier, jitter, level, adjustment_strategy, tolerance_adjustment, checkpoint_after_retry — step.rs:403-427) but the benchmark execution path never read any of it; global `inference_max_attempts` (default 3) + fixed 30s wait applied to every step.
- **Root cause:** private runner WorkflowStep had no retry field; parse dropped `retry:`; retry loops read `self.inference_max_attempts` only.
- **Fix (minimal scope — attempts only):** parse `step.retry.max_attempts` → `WorkflowStep.retry_max_attempts: Option<u32>`; new helper `BenchmarkRunner::effective_max_attempts(global, step_override)` (step wins, min-1 clamp); `run_model_inference` gains `step_max_attempts` param (single workflow call site passes `step.retry_max_attempts`); smart-retry loop uses effective value. Discovery path (benchmark_single_model) unchanged. Backoff/delay/jitter fields remain unimplemented — documented limitation, feature work.
- **Tests (watched RED first — E0599/E0609):** `given_step_retry_config_when_steps_loaded_then_max_attempts_parsed` (parse captures Some(1) vs None); `given_step_retry_override_when_effective_attempts_computed_then_step_wins` (override wins / exceeds / absent / zero-clamp).
- **Verification:** full suite 798 total 0 fail; clippy 13 baseline 0 new; LSP clean.
- **Incident note:** intermediate literal-fill pass corrupted 12 test YAML fixtures (inserted Rust struct-field lines inside r#""# YAML blocks) — caught by full suite (33 failures), fully repaired; suite now green. Lesson: compile-clean ≠ green after any bulk source edit.
- **Failure mode dead:** a step declaring `retry: {max_attempts: 1}` can no longer silently retry 3×.


### 2026-08-24 — Issue F2: hook-trigger errors swallowed (`let _ =`)

- **Original shape:** `on_requires_failed` and `after_loop_iteration_fails` fired their hooks via `let _ = self.execute_hooks_for_trigger(...)` — any hook-action error (bad shell, unwritable path, malformed action) vanished silently. All five other triggers already used match arms with `error!` logging.
- **Root cause:** two leftover swallow sites from the original trigger wiring.
- **Fix (minimal):** both converted to `match ... { Ok(_) => {}, Err(e) => error!("[benchmark] <trigger> hook error: {}", e) }` — same contract as the other five triggers. Errors still don't abort the step (hook side-effects are best-effort by design) but are now visible in logs.
- **Test:** `given_hook_trigger_call_sites_when_counted_then_none_swallow_errors` — include_str! source-count of the `let _ = self.execute_hooks_for_trigger` pattern, asserts zero. Watched RED (count=2). GREEN pitfall: assert message initially contained the contiguous pattern and self-matched; shortened message.
- **Verification:** full suite 799 total 0 fail; clippy 13 baseline; LSP clean.
- **Failure mode dead:** a failing hook action on these triggers can no longer disappear without a log line.


### 2026-08-24 — Issue AD: after_step_succeeds hook Fail verdict silently discarded

- **Audit claim vs reality:** the audit cited runner.rs ~1340-1380 (deprecated logging fns — stale evidence). Code-confirmed shape: `Ok(HookResult::Fail { reason })` in the after_step_succeeds match only `warn!`ed; `model_result.error` stayed `None`, so a verifier hook's fail verdict could not affect the recorded result — the step counted as successful in the suite/report.
- **Contract inconsistency:** before_step_starts Fail already failed the step (returns error result); after_step_succeeds Fail did not.
- **Fix (minimal):** Fail arm sets `model_result.error = Some("after_step_succeeds hook failed: {reason}")` — tokens/latency metrics preserved (real measurements), but the step is no longer counted successful against the hook's verdict.
- **Test:** `given_success_hook_fail_verdict_when_step_executes_then_result_marked_failed` — mock llama-server (std TcpListener + thread, per M/P precedent) serving GET /v1/models (status loaded), POST /models/load, POST /v1/chat/completions; step with `after_step_succeeds: [{fail: {message: "verifier rejected output"}}]`; asserts error carries the hook reason. Watched RED (error None). Mock pitfall: load/unload endpoints are `/models/load`, NOT `/v1/models/load` (url() helper prefixes differ); `{}` body parses as ModelLoadResponse success=false → "Load model rejected".
- **Verification:** full suite 800 total 0 fail; clippy 13 baseline; LSP clean.
- **Failure mode dead:** hook fail verdict now visible in benchmark_report.json (error field) and suite counts (successful/failed).


### 2026-08-24 — Issue AF: README quickstart used nonexistent flag

- **Original shape:** README:21 `./target/release/whitt workflow --workflow workflows/example.yml` — the `workflow` subcommand takes a POSITIONAL `workflow_file` (whitt.rs:207-215); `--workflow` exists only on `benchmark` (whitt.rs:186-188). Live-proven: `whitt workflow --workflow /tmp/x.yml` → `error: unexpected argument '--workflow' found`; positional form proceeds to the file check.
- **Fix (docs only):** README quickstart now shows `whitt workflow workflows/example.yml` plus a `benchmark --workflow ...` example distinguishing the two.
- **No src change — no TDD applicable.**

### 2026-08-24 — Issue AG: obsolete sleep annotations

- **Shape:** multi-model/atom-v1.yml raw-curl model-swap hooks carry `sleep 3`/`sleep 15` settle-waits. Engine-managed load/unload now polls server status (Issue M/P fix) and needs no sleeps — but these hooks bypass the engine entirely (raw curl), so the sleeps remain REQUIRED for this historical pattern.
- **Action:** both sites annotated in-file (kept, not deleted — historical-run reproducibility). single-model/atom-v1.yml has no sleeps.


### 2026-08-24 — Issue R: GWT `then: {route_to: X}` verbose form failed serde

- **Original evidence:** schema doc examples (unified-workflow-schema.yml:358-402) show `then: { route_to: X }`; Rust RouteToAction untagged enum accepted only bare string/array — "route_to must be untagged `then: <step>` (the `{route_to: X}` form fails engine serde)" (07-TRACKING.md:226-228). Workflows copied from the doc errored.
- **Resolution (user decision: both code + doc):** custom `Deserialize` for RouteToAction accepts string → Single, array → Multiple, object `{route_to: <string|array>}` → Single/Multiple; unknown objects/other types → loud error naming accepted forms. Serialize kept untagged (byte-identical output). Schema doc gained a NOTE comment at the first verbose example listing both accepted forms.
- **Tests:** `given_gwt_verbose_then_object_json_when_deserialized_then_produces_single`, `given_gwt_verbose_then_object_multiple_json_when_deserialized_then_produces_multiple` (tests/hooks_integration.rs) — RED reproduced the exact experiment error "data did not match any variant of untagged enum RouteToAction". GREEN: 65/65 hooks_integration.
- **Pitfall hit:** dropping `Deserialize` from derive also lost `#[serde(untagged)]` on Serialize → round-trip test caught `{"Single": "step"}` external tagging; fixed by re-adding `#[serde(untagged)]`.

### 2026-08-24 — Issue X: wall-clock workflow timeout — DESIGN PROPOSAL (no implementation, per user decision)

**Problem.** A workflow run has no total-run deadline. Experiments wrapped `timeout "${TIMEOUT_SECS}" whitt benchmark ...` (run-atom.sh:82) because the engine itself never stops a stuck run; a wedged model load or slow inference hangs until an external watchdog kills it (and pre-Y, the kill lost all work).

**Proposal — two layers, CLI first (no schema change needed):**

1. **CLI flag (implementable now, no schema touch):**
   - `whitt benchmark --max-wall-time <SECONDS>` (default: 0 = unlimited).
   - Runner field `max_wall_time: Option<Duration>` + builder, same pattern as `resume`.
   - Deadline = `Instant::now() + max_wall_time` captured at top of `run()`.
   - Checked at two points only (cheap, no tasks/threads):
     a. top of the main workflow while-loop per step iteration;
     b. inside the smart-retry loop before each retry sleep.
   - On expiry: `error!("[benchmark] wall-clock limit of {}s exceeded after {} step(s) — stopping")` + `anyhow::bail!` → non-zero exit. NOT exit 0 (Issue E precedent).
   - Synergy with Y: bail leaves checkpoint.jsonl on disk → `--resume` continues the run later. Experiments can then delete their external `timeout` wrapper + watchdog.

2. **YAML key (needs schema change → user approval before implementing):**
   - `workflow_execution_strategy.timing.max_wall_time_secs` (uint).
   - Applied in `apply_workflow_timing_overrides` (Issue D ordering: before preflight).
   - CLI flag wins over YAML (CLI = explicit operator intent; matches `--min-tmp-space` precedence).
   - Schema line comment: `# schema line NNN` per repo rule.

**Semantics decisions to confirm before implementing layer 2:**
- Hard stop vs. finish-current-step: proposal = check BETWEEN steps (never abort mid-inference — Vulkan mid-load aborts can leave the server wedged, per SAFETY.md device-loss history).
- Interaction with retry: deadline check before retry backoff so a 30s×N retry storm cannot blow the budget silently.
- Report: on expiry, suite result should carry `wall_time_exceeded: true`-style error field (extend Issue J attribution pattern).

**Test plan (when implemented):** unit — `wall_time_error(elapsed, limit)` helper (Some/None boundaries); integration — mock-server workflow (AD pattern) with 1s limit + sleep-inducing step → non-zero exit + checkpoint file exists; regression — unlimited default changes nothing.


### 2026-08-24 — Issue Y: minimal checkpoint/resume (user-approved implementation)

- **Original evidence:** REVIEW-CYCLES.md OC-4 — watchdog kill mid-run = "total loss per kill"; persistence.rs wires the ReAct agent only, benchmark run() had nothing.
- **Fix (minimal, per user decision):** `record_step_output()` inserts into the in-memory step_outputs map AND appends `{"step_id", "output"}` JSON line to `<output_dir>/checkpoint.jsonl` — wired at all 8 step-completion sites (sequential, iterate-values loop, routed, parallel-targets helper). `--resume` CLI flag (whitt.rs benchmark arm) → runner field via `with_resume` → run() pre-populates step_outputs from the checkpoint + snapshot HashSet of restored ids; main-loop skip gate fires only for checkpointed steps (runtime revisits of a step are NOT skipped — snapshot, not map-contains). Checkpoint IO failures warn, never fail the run.
- **Tests:** given_checkpoint_file_when_loaded_then_returns_last_outputs (malformed-line skip, duplicate last-wins, missing-file = empty); given_step_output_when_recorded_then_checkpoint_appended (map + 2 JSON lines). RED watched (E0599 ×2 + E0282) before implementation.
- **Verification:** 597 lib tests 0 fail (incl. new); `whitt benchmark --help` shows `--resume`; full suite/clippy/LSP in final batch below.
- **Failure mode dead:** a watchdog kill now leaves every completed step's output on disk; re-running with `--resume` skips them and continues. The "total loss per kill" mode requires additionally deleting checkpoint.jsonl.

### 2026-08-24 — LOW trio + N/S residuals (same session)

- **topo-cycle:** warn-only → `cycle_error()` + hard bail. Test: given_cycle_when_cycle_error_computed_then_some.
- **keyless array step:** filter_map silent drop → enumerated Result-collect hard Err naming entry index. Test: given_array_step_missing_name_when_steps_loaded_then_hard_error.
- **set_state ignored:** 3 sites propagate + warn!. Test: given_tools_source_when_set_state_discards_counted_then_zero (source-count).
- **N residual (drift hint):** `model_name_drift_hint()` fires warn! at every fuzzy-layer resolution naming requested vs resolved ids + exact-filename note for raw API calls. Test: given_fuzzy_model_resolution_when_drift_hint_computed_then_names_drift.
- **S residual (own-port exclusion):** zombie_check_command excludes own server (`--port {own_server_port(server_url)}`, fallback 8080) in addition to router parent. Tests: given_own_server_port_when_zombie_command_built_then_excludes_live_server + ported router-mode test.
