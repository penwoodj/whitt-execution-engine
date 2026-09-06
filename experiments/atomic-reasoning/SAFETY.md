# Atomic Reasoning Experiments — Safety Rules

## Why This Document Exists

A previous experiment run crashed the host machine by:
1. Auto-detecting `max_concurrent_inferences: 4` (4 parallel inferences)
2. Each inference allocated its own KV cache (~80MB × context_size)
3. Combined VRAM + RAM pressure exhausted 16GB system memory
4. Swap thrashing → load avg spike → unresponsive system → hard reboot required

**This document defines hard rules that MUST be enforced to prevent recurrence.**

## Hard Rules (NEVER violate)

### R1: Sequential Only — `concurrent=1` ALWAYS
```bash
whitt benchmark --concurrent 1 ...
```
**Never omit this flag.** Default is auto-detect which picks 4 on this hardware. 4× is fatal.

### R2: Pre-flight Check Before Every Run
```bash
bash experiments/atomic-reasoning/scripts/preflight.sh
```
Aborts if any of these fail:
- RAM available < 3 GB
- Swap used ≥ 4 GB
- Load avg (5-min) ≥ 8.0
- Docker container not running
- llama.cpp /health not OK
- Vulkan device-loss in recent logs
- >1 model loaded simultaneously
- /tmp free < 1 GB

### R3: RAM Watchdog Active During Every Run
```bash
bash experiments/atomic-reasoning/scripts/ram-watchdog.sh $PID 2 &
```
Kills the whitt process if RAM available drops below 2 GB. Logs to `/tmp/ram-watchdog-${PID}.log`.

### R4: Hard Timeout On Every Run
```bash
timeout 180 whitt benchmark ...
```
180 seconds max. Anything longer is broken anyway. SIGTERM after 180s, SIGKILL after 183s.

### R5: Cooldown Between Runs (60s minimum)
```bash
# Tracked in experiments/atomic-reasoning/results/.last-run-ts
```
Forces 60s gap between consecutive `run-atom.sh` invocations. Lets VRAM/RAM settle.

### R6: Single Model Loaded At Any Time
- Before each run, unload any non-target models via `/v1/models/unload`
- Never load Qwen3.5-9B + any other model simultaneously (VRAM exceeded)
- Verify loaded count = 1 in preflight

### R7: Consecutive Test Cap
After 5 consecutive `run-atom.sh` calls, force 120s cooldown + free -h check.
If RAM <4GB available after cooldown: `docker restart whitt-llama-server && sleep 60`.

### R8: AGENTS.md Hard Rule — 9B+ Models gpu_layers=99
`config.yml` MUST have `hardware.gpu_layers: 99`. CPU-only loading of 9B models
causes thermal overload after ~30 min sustained use.

## Per-Run Resource Budget

| Resource            | Limit                                  |
| ---                 | ---                                    |
| Wall clock          | 180s (hard `timeout` cap)              |
| Concurrency         | 1 (NEVER higher)                       |
| Max tokens / step   | 500 (set in YAML `max_tokens`)         |
| Context size        | 4096 (set in YAML `context_size`)      |
| Inferences per step | 1 (no parallel sampling)               |
| Total inferences    | ~5 per workflow (4 steps + bootstrap)  |

## What Triggered The Crash (postmortem)

```
[benchmark] auto-detected max concurrent inferences: 4 (based on 8192.0 GB VRAM available)
```

The runner saw 8GB VRAM (RX 580) and assumed 4× parallel safe. It is NOT, because:
- Each Vulkan context has its own KV cache allocation
- 4× 3GB Qwen3-4B context = 12GB+ host RAM (KV cache spills to host)
- Combined with OS + Docker overhead → OOM

The "8192.0 GB VRAM available" was a misreport — actual is 8GB.

## Safeguard Architecture

```
┌────────────────────────────────────────────────────────────┐
│ run-atom.sh (entry point)                                  │
│                                                            │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 1. preflight.sh — abort if any check fails       │    │
│   └──────────────────────────────────────────────────┘    │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 2. cooldown check — wait if <60s since last run  │    │
│   └──────────────────────────────────────────────────┘    │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 3. unload non-target models (1-model invariant)  │    │
│   └──────────────────────────────────────────────────┘    │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 4. timeout 180 whitt benchmark --concurrent 1    │    │
│   │    │                                             │    │
│   │    └── ram-watchdog.sh (background)              │    │
│   │           kills whitt if RAM < 2GB               │    │
│   └──────────────────────────────────────────────────┘    │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 5. metrics capture from benchmark_report.json    │    │
│   └──────────────────────────────────────────────────┘    │
│   ┌──────────────────────────────────────────────────┐    │
│   │ 6. 30s post-run cooldown + preflight recheck     │    │
│   └──────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────┘
```

## Recovery Procedure (if safeguards fail)

If machine hangs again:

1. **Hard reboot** (power cycle if SSH unresponsive)
2. **Wait 60s** after boot for Docker to settle
3. **Check `/var/log/syslog`** for OOM killer:
   ```bash
   grep -i "oom\|killed process" /var/log/syslog | tail -20
   ```
4. **Restart Docker clean**:
   ```bash
   docker compose -f /home/jon/code/whitt-execution-engine/docker/docker-compose.yml down
   docker compose -f /home/jon/code/whitt-execution-engine/docker/docker-compose.yml up -d
   sleep 60
   ```
5. **Verify Vulkan healthy**:
   ```bash
   docker logs whitt-llama-server --tail 50 | grep -i "vulkan\|device.*lost"
   ```
6. **Run preflight** before any further experiments

## Forbidden Actions

- `whitt benchmark` WITHOUT `--concurrent 1`
- Loading Qwen3.5-9B + ANY other model simultaneously
- Setting `gpu_layers` to anything other than 99 for 7B+ models
- Running experiments while load avg > 8.0
- Skipping preflight "just this once"
- Disabling the RAM watchdog to "see what happens"
- Running >5 experiments back-to-back without 120s cooldown
