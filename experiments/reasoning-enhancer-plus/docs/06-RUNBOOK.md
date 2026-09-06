# REA+ — Runbook & Ops Safety

## Layout

```
experiments/reasoning-enhancer-plus/
  docs/            # this suite (north star)
  cases/probe/     # 20 stage-1 items
  fixtures/        # probe-fixes.yml (reference answers)
  scripts/         # emit-probe, gen-probe-workflow, run-probe-model,
                   # rank-models, lint-cases, fingerprint
  workflows/       # generated probe-<model>.yml
  results/
    model-probe/<model>/     # per-model run artifacts + summary.json
    model-probe/RANKING.md
    MODEL-PICKS.md           # STAGE-1 GATE ARTIFACT
    rea-plus/                # stage-2 runs later
    regression-table.md
```

## Commands

```bash
# stage 1
python3 scripts/lint-cases.py                          # battery lint
bash scripts/run-probe-model.sh Qwen3-4B-Instruct-2507-Q4_K_M --limit 2   # smoke
bash scripts/sweep-probes.sh                           # full 10-model sweep
python3 scripts/rank-models.py                         # RANKING + PICKS

# skills (anywhere)
rea-model-probe: probe.py --model X [--limit N]
rea-live-pulse:  pulse.py --run-dir D [--case C]       # replay+fingerprint
rea-metrics-judge: judge.py --runs GLOB [--metric M2]  # metric table
```

## Ops safety (RX 580 8GB — HARD)

- 1 model resident max; 7B+ needs gpu_layers 99 (never 1-98, never 0)
- ≤5 consecutive loads → 60s cooldown + free -h ≥3GB avail
- preflight per model: RAM ≥3GB, swap si+so <25MB/8s
- vk::/DeviceLost/GGML_ASSERT in docker logs → restart + settle 30s,
  rerun model; 3 consecutive load failures → STOP, escalate to user
  (driver-state decay suspected; host reboot or amdgpu reload needed)
- watchdog: `timeout 900` per model; rc=143 → rerun (resume-safe)
- NEVER load 9B + anything else simultaneously

## Load-verification trap (HARD lesson 2026-08-16)

POST /models/load returns `{"success":true}` in ~13ms — that is an
async ACCEPT, not a load confirmation. The spawned llama-server child
can still crash (GGML_ASSERT) seconds later. Only source of truth:
`GET /models` → `status.value == "loaded"` (scripts/wait-loaded.py).

## Log fingerprinting (prompt-input ID)

Every workflow step's before-shell appends to
`results/model-probe/<model>/fingerprint.log`:

```
<timestamp> <case_id> <step> sha1=<prompt-hash> head=<first 80 chars>
```

Use: `pulse.py --run-dir D --case C` reprints fingerprint + full prompt
for instant failure triage. Failure → hash → exact input, no guessing.

## Clean-wall accounting

- duration only from rc=0 runs (metrics.json)
- killed/partial runs logged but NEVER averaged into M7
- docker restarts/settle tracked in run log header
