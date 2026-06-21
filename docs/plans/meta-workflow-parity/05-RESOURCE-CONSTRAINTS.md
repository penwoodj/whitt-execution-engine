# 05 - RESOURCE CONSTRAINTS

## HARDWARE

- **CPU:** AMD FX/Ryzen-class (verify in lscpu if needed)
- **RAM:** System memory must not exceed 80% utilization during workflow runs
- **GPU:** AMD RX580 8GB Vulkan/RADV (single slot)
- **Disk:** SSD preferred; outputs gitignored to prevent bloat

## SAFE OPERATING LIMITS

| Resource | Limit | Monitor |
|----------|-------|---------|
| GPU VRAM | 7GB peak (Qwen3.5-9B Q4_K_M needs ~5.5GB) | `nvidia-smi` or `radeontop` (if available) |
| System RAM | 12GB peak | `free -h` |
| CPU cores | 5 max (1 for system, 5 for whitt threads) | `htop` |
| Disk writes | <100GB per cycle | `df -h` |
| Docker container uptime | Restart per prompt (stability) | docker logs |

## DOCKER STABILITY (FROM B3 FINDINGS)

- Server crashes after ~5 prompts under sustained load
- Mitigation: restart per prompt (already standard practice)
- Possible causes: Vulkan/RADV driver leak, llama.cpp memory issue, whitt slot release bug
- NOT user-fixable — work around with restarts

## SPEED EXPECTATIONS

- Single prompt through meta-v6: ~45 min wall clock
- All 11 prompts: ~8-9 hours
- Per-cycle iteration: 1 work day
- 3 cycles max: 3 days worst case

## USER EXPLICIT REQUIREMENTS (FROM PROMPT)

> "I still want things to run as fast as they safely can on my hardware without crashing my machine from overuse of resources"

Implementation:
- gpu_layers=99 (full offload, proven fastest in b3)
- threads=5 (whitt worker threads)
- parallel=1 (single inference slot — GPU constraint)
- q8_0 KV cache (Vulkan-stable)
- Docker restart per prompt (stability)

## MONITORING

During long runs:
```bash
# Check docker alive
docker ps | grep whitt-llama-server

# Check system resources
free -h; echo "---"; df -h | head -5

# Check workflow progress
tail -f docs/benchmarks/outputs/meta-workflow/meta-meta-v6-*/meta-benchmark.log
```

## ABORT CONDITIONS

Hard abort if:
- System RAM >90% sustained 30s
- Docker unresponsive >5min
- Disk full
- Whitt process zombie
- Workflow stuck same step >30min

Recover: kill processes, restart docker, resume from last successful prompt.
