# REA — Safety (Hard Rules)

**Inherits:** experiments/atomic-reasoning/SAFETY.md in full. This file lists
only deltas + REA-specific rules. Read BOTH before any live run.

## Machine status constraint (this scaffolding phase)

Another experiment is using the machine NOW. Therefore during scaffolding:
**NO** cargo build/test, **NO** whitt benchmark runs, **NO** docker contact,
**NO** model loads, **NO** curl to :8080. All verification is pure-Python
self-tests + dry-run simulation. Live commands live in RUNBOOK.md only.

## Deltas vs atomic-reasoning SAFETY.md

| # | Rule | Delta |
|---|------|-------|
| R1 | `WHITT_MAX_CONCURRENT_INFERENCES=1` ALWAYS | unchanged, non-negotiable |
| R2 | preflight before every run | reuse `experiments/atomic-reasoning/scripts/preflight.sh` verbatim |
| R3 | ram-watchdog kills if RAM < 2GB | reuse `ram-watchdog.sh`, threshold unchanged |
| R4 | timeout 180s | REA raises to 300s hard cap (5 LLM steps worst case; median target stays 180s) — run-enhancer.sh enforces both |
| R5 | cooldown 60s between runs | unchanged, RAM-gated at ≥3072MB free |
| R6 | single model loaded | REA loads TWO models across the workflow (1.7B + 4B) but engine `unload_unused: true` swaps them sequentially — still only ONE in VRAM at any instant. run-enhancer.sh verifies via `/v1/models` before AND between phases |
| R7 | 5 consecutive runs → 120s cooldown | unchanged |
| R8 | 9B+ → gpu_layers=99 | REA uses NO 9B — rule moot but kept |

## REA-specific hard rules

- **R9 — Sequential steps only.** The workflow YAML declares no parallel
  branches (no route_to fan-out). Diversity comes from lenses, not
  concurrency.
- **R10 — Watchdog stack.** Outer `timeout 300` (runner) + inner per-case
  budget (workflow-level). Two layers, either may kill; both log.
- **R11 — Output confinement.** All artifacts under the per-run output dir
  (`__OUTPUT_DIR__` placeholder → run dir). No writes to repo root. save_to
  entries all `$`-prefixed variables or absolute output-dir paths.
- **R12 — No network.** Scripts touch no network. The only network actor is
  the engine talking to localhost:8080. dry-run.py must work with networking
  disabled.
- **R13 — Case lint is a gate.** A case whose draft passes its own checks, or
  whose reference fix fails them, is a broken case — never weaken checks to
  fit. (v8 learning 7.)
- **R14 — Ratchet invariant.** select-best.py must never write a
  final-answer.txt that passes FEWER subchecks than the input draft, unless
  the draft itself has hard-fail classes (then no final at all).

## Violation response

Any watchdog kill, OOM, or Vulkan error in logs → stop, `docker restart
whitt-llama-server`, wait for health, re-run preflight, only then continue.
Never chain runs after an unexplained kill.
