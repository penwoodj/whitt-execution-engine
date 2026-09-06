# whitt-crash-debug — Engine Crash Triage & Hardening Protocol

MUST USE when: the machine or execution engine crashes/hangs during a
`.yml` agentic workflow run (local LLM, llama.cpp/Vulkan), when a run dies
with HTTP 500 / `vk::DeviceLost` / OOM symptoms, or after ANY hard freeze
that happened while whitt was executing.

Purpose: turn every crash into (a) collected evidence, (b) a root-cause
hypothesis, (c) a failing regression test, (d) an engine fix — TDD loop
until the crash class is structurally impossible.

## 1. Immediate triage (evidence collection — run FIRST after a crash)

```bash
bash .opencode/skill/whitt-crash-debug/scripts/triage.sh <whitt-run-log> [out-dir]
```
Collects into `<out-dir>/` (default `experiments/crash-reports/<ts>/`):
- kernel OOM/kill events (`journalctl -k`, `dmesg` guarded)
- docker server log tail + `vk::|DeviceLost|Vulkan|OutOfMemory` grep
- container restart count + exit code (was llama-server OOM-killed?)
- llama-server cmdline (ctx size! `-c` — the 100k incident) + slot config
- whitt run log: `RESOURCE_|error|failed|500` greps + last 100 lines
- host memory snapshot (`free -h`, amdgpu `mem_info_vram_*`)
- writes `evidence.md` summary + `hypotheses.md` starter

## 2. Hypothesis table (fill from evidence — one row per candidate)

| ID | Hypothesis | Evidence for | Evidence against | Test to write |
|----|-----------|--------------|------------------|---------------|
| H1 | External app (desktop/Storybook) consumed VRAM/RAM mid-inference → thrash/OOM | journal OOM lines, vram_free trace | timing of load vs crash | watchdog red-state unit test |
| H2 | ctx/KV budget exceeded VRAM (config drift) | server `-c` value, KV math | — | admit_load rejection test |
| H3 | Concurrency >1 on 8GB card | log "max concurrent" line | — | detect_max_concurrent test |
| H4 | Server-side slot error surfaced as HTTP 500, engine kept going | docker log 500s, whitt log | — | after_step_fails propagation test |
| H5 | Unit bug (bytes vs KB) mis-sized resources | log "8192.0 GB VRAM" | — | sensor parser test |

## 3. TDD hardening loop (repo HARD rule: failing test FIRST)

1. RED: write the unit/BDD test encoding the crash scenario
   (mock sensors / injected snapshots — never require real OOM):
   `tests/resource_guard_*.rs` or `#[cfg(test)]` in module.
   RUN IT. Watch it FAIL. A test that passes first is worthless.
2. GREEN: minimal fix in `src/client/resource_guard.rs` (or touch-point).
3. REFACTOR: decouple while green — sensors behind traits, runner takes
   `ResourceWatchdog` handle, no direct /proc reads in step loop.
4. Clean en route: dead code, duplicated parse helpers, misleading logs.
5. Log every decision: `RESOURCE_SAMPLE`, `RESOURCE_PRESSURE`,
   `RESOURCE_CRITICAL`, `RESOURCE_ADMIT`, `RESOURCE_REJECT` (tracing).
6. Re-run FULL suite: `CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo test --lib`
   (cmake 4.x needs the policy env for llama-cpp-sys-2).
7. Rebuild binary: `CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo build --release --bin whitt`.

## 4. Engine protection invariants (regression-protected)

- I1: VRAM reported in correct units (bytes→GB, never 1024× overreport)
- I2: ≤8GB VRAM ⇒ max_concurrent_inferences == 1
- I3: load admitted only if weights+KV+compute+reserve ≤ free VRAM
      (reserve = user-desktop allowance, default 1.5GB)
- I4: ctx size rejected when KV budget alone can't fit (the -c 100000 case)
- I5: RAM floor: MemAvailable − footprint ≥ reserve (default 2.5GB)
- I6: watchdog RED ⇒ no new inference issued (between-step gate)
- I7: watchdog RED during inference ⇒ request cancelled ≤3s + typed
      `RESOURCE_CRITICAL` error flows to after_step_fails hooks
- I8: hysteresis: RED sticky, AMBER needs N sustained samples, GREEN
      requires cooldown — no flapping
- I9: every gate/reject emits one greppable RESOURCE_* log line w/ numbers

## 5. After fixing

- Update `experiments/crash-reports/<ts>/hypotheses.md` verdict column
- Add invariant row above if new class discovered
- Re-run the live adversarial scenario (user opens Storybook mid-run):
  engine should log RESOURCE_CRITICAL, cancel, machine stays interactive
- Commit w/ conventional message referencing the crash report dir

## 6. Live-run safety (standing rules)

- Server ctx sanity: `docker exec whitt-llama-server sh -c 'cat
  /proc/$(pgrep -f llama-server|head -1)/cmdline|tr "\0" " "'` → `-c`
  must be ≤30000 on this box before ANY run
- `free -h` ≥3GB available between cases; else docker restart + 30s
- Watchdog + earlyoom present before long batches
- All crash evidence under `experiments/crash-reports/` — never repo root
