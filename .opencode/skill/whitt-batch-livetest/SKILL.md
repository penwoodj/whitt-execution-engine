# whitt-batch-livetest — Batch Live-Test Protocol for .yml Agentic Workflows

MUST USE when running large batches (>10) of test cases against `.yml` agentic
workflows in this repo with live system testing (real or spoofed LLM runs via
`whitt benchmark`), especially iterations expected to last over an hour.

Encodes: batch cycling, regression gates, dedup/efficiency improvement cycles,
evidence tracking, and hardware safety rules. Follow it EVERY run.

## Companion tool

`python3 .opencode/skill/whitt-batch-livetest/scripts/batchctl.py <cmd>`
(see `--help`; state at `experiments/<exp>/results/<phase>/cycle-state.json`).

## Protocol (per suite of N cases, batch size 10)

### 1. Init
```
batchctl.py init --suite <case-dir> --results <results-dir> [--size 10]
```
Creates `cycle-state.json` ledger: every case `pending`, batches of 10,
`baseline_locked=false`.

### 2. Per batch — the 4-gate cycle (NEVER skip a gate)
```
GATE A  Run:      batchctl.py run-next        # emits whitt commands for next 10
GATE B  Triage:   mark pass/fail per case (oracle report exit 0 = pass)
                   fix + re-run until batch 10/10 pass
GATE C  Dedup+Efficiency: on the .yml workflow driving the batch:
                   - measure: wall time/case, LLM calls/case, token volume
                     (batchctl.py efficiency)
                   - dedup: remove redundant steps/scripts/model swaps
                     (consecutive same-model reloads, back-to-back python
                     invocations that can merge, duplicate detections)
                   - efficiency: cut LLM calls (early-exit gates, tighter
                     routing, lower max_tokens) — NEVER by weakening oracles
                   - re-run batch: quality (oracle pass count) must be >=
                     before, speed must improve; else revert
GATE D  Regression: batchctl.py regress-check → re-run ALL previously
                   passing cases; any regression = fix workflow BEFORE
                   next batch; re-mark
```
Only after A-D green: `batchctl.py advance` → next batch. Repeat.

### 3. Evidence rules (HARD)
- Every claim backed by: log file path + report.json + grep counts.
- Zero-LLM proof gate when spoofing: `grep -cE "Prompt tokens|eval count|
  tokens/s|llama_|loading model|unloading model|\[infer|total duration" <log>`
  == 0 per log.
- Real-LLM runs: prove inference happened (grep `Prompt tokens|eval count`
  > 0 in whitt log AND llama server log).
- `WORKFLOW_RELIABILITY_TRACKING.md` / experiment `06-TRACKING.md` row per
  iteration with: yaml path, log path, 8-check evidence, comparison table.

### 4. Engine invocation (this repo)
```
WHITT_ZOMBIE_MAX=8 /home/jon/code/whitt-execution-engine/target/release/whitt \
  benchmark --workflow <yml> --output-dir <results>/whitt-logs
```
- Use `set -o pipefail` when piping. Redirect full log per case:
  `<results>/whitt-logs/<case>-run<k>.log`.
- Validate FIRST: `python3 scripts/meta-v6/validate-workflow.py <yml>`.
- `WHITT_ZOMBIE_MAX=8` = established spoof/live escape hatch (preflight
  zombie grep self-match; rea+ precedent).

### 5. Hardware safety (RX 580 8GB / 16GB RAM) — HARD
- NEVER 2+ models resident. Server single-slot hot-swap only (sequential).
- 7B+ models: gpu_layers 99 (server config default — do not override).
- Between cases: `free -h`; if <3GB available → `docker restart
  whitt-llama-server` + `sleep 30` before next.
- After every 5 consecutive model-swap-heavy cases: 60s cooldown + RAM check.
- Abort on: `curl localhost:8080/health` failure, or docker logs show
  `vk::|DeviceLost|Vulkan.*Error` → restart docker first.

### 6. Repo cleanliness (HARD)
- All outputs under `experiments/<exp>/results/...`. Nothing at repo root.
- Workflows under `experiments/<exp>/workflows/generated*/`.
- Every new YAML key → schema line comment + validator pass first.

### 7. Context/session hygiene
Sessions run >1h: compress closed sections aggressively; on compression
failure or context pressure → write `.opencode-handoff.md` per protocol and
continue in fresh session. batchctl state file = source of truth across
sessions.

## Quality bar
"Working" = oracle pass on live system with evidence. Speed improvements are
only real if quality (oracle passes) is same or better on re-run.
