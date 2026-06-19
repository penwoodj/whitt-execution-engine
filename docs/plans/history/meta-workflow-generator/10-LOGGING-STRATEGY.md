# 10 — Logging Strategy

Defines the per-step log fields, log file layout, and log analysis protocol.

---

## 1. Log File Layout

```
docs/benchmarks/outputs/meta-workflow/<RUN_ID>/
├── input/
│   └── prompt.txt                          # raw input prompt
├── logs/
│   ├── meta.log                            # META orchestrator events
│   ├── sw1.log                             # SW1 events
│   ├── sw1-stdout.log                      # SW1 stdout (from whitt benchmark)
│   ├── sw2.log                             # SW2 events
│   ├── sw2-stdout.log
│   ├── sw3.log
│   ├── sw3-stdout.log
│   ├── sw4.log
│   ├── sw4-stdout.log
│   ├── sw5.log
│   ├── sw5-stdout.log
│   ├── final-validation.log               # validate-yaml.py output
│   └── final-validation-result.txt        # final result summary
├── sw1/
│   ├── 00-bootstrap.txt                    # bookmark outputs
│   ├── 01-initial.txt                      # initial task list
│   ├── 02-expanded.md                      # expanded subtasks
│   ├── 03-eval.txt                         # GWT evaluation
│   └── tasks.md                            # FINAL OUTPUT
├── sw2/
│   ├── 00-bootstrap.txt
│   ├── 02-chunks.md
│   ├── 03-eval.txt
│   └── outputs.md                          # FINAL OUTPUT
├── sw3/
│   ├── 00-bootstrap.txt
│   ├── 02-chunks.md
│   ├── 03-eval.txt
│   └── categories.md                       # FINAL OUTPUT
├── sw4/
│   ├── 00-bootstrap.txt
│   ├── 02-chunks.md
│   ├── 03-eval.txt
│   └── structs.md                          # FINAL OUTPUT
└── sw5/
    ├── 00-bootstrap.txt
    ├── 01-assembled.yml
    ├── 02-fixed.yml
    ├── 03-task-audit.md
    ├── 04-eval.txt
    └── workflow.yml                        # FINAL OUTPUT
```

---

## 2. Per-Step Event Fields (MANDATORY)

Every LLM step MUST log these fields in `after_step_succeeds`:

| Field | Source | Why |
|-------|--------|-----|
| `step_name` | auto | Identifies step |
| `duration_ms` | auto | Performance tracking |
| `total_tokens` | context | Output size tracking |
| `quality_score` | context | Token ratio (NOT semantic — informational only) |

Every LLM step with loop iteration MUST add:

| Field | Source | Why |
|-------|--------|-----|
| `loop.iteration` | auto | Which iteration |
| `iteration_variable` | context | What value (e.g., chunk_id) |

Every shell hook step MUST log:

| Field | Source | Why |
|-------|--------|-----|
| `step_name` | auto | Identifies step |
| `duration_ms` | auto | Shell execution time |
| `exit_code` | shell output | Success/failure of command |

Every error/failure hook MUST log:

| Field | Source | Why |
|-------|--------|-----|
| `step_name` | auto | Which step failed |
| `error_message` | context | What went wrong |
| `error.type` | context | Error classification |

---

## 3. Log Entry Format

Log entries are JSON-lines (one JSON object per line) emitted by the engine.
The `log` hook action supports `to_file_path` (JSON-lines file) and optional
`stdout: true` for console mirroring.

Example log line:
```json
{"timestamp":"2026-06-13T21:30:00Z","workflow_id":"sw1_task_deconstruction","step_name":"step_01_initial_breakdown","duration_ms":45230,"total_tokens":1487,"quality_score":0.74,"level":"info","event":"after_step_succeeds"}
```

---

## 4. Log Analysis Protocol

### After Every Iteration

```bash
RUN_ID="<the-run-id>"
LOG_DIR="./docs/benchmarks/outputs/meta-workflow/$RUN_ID/logs"

# 1. Verify workflow completed
grep '"event":"after_workflow"' "$LOG_DIR/sw1.log" | tail -1 | jq .

# 2. Count steps executed
grep -c '"event":"after_step_succeeds"' "$LOG_DIR/sw1.log"

# 3. Check for errors
grep '"level":"error"' "$LOG_DIR/sw1.log" | jq -r '.step_name + ": " + .error_message'

# 4. Per-step duration breakdown
grep '"event":"after_step_succeeds"' "$LOG_DIR/sw1.log" | jq -r '.step_name + " " + (.duration_ms|tostring) + "ms"'

# 5. Total tokens generated
grep '"event":"after_step_succeeds"' "$LOG_DIR/sw1.log" | jq -r '.total_tokens // 0' | awk '{s+=$1} END {print s}'

# 6. Loop iteration counts
grep '"event":"after_step_succeeds"' "$LOG_DIR/sw1.log" | jq -r 'select(.iteration) | .step_name + " iter=" + (.iteration|tostring)' | sort | uniq -c
```

### Using `scripts/analyze-run.sh`

```bash
./scripts/analyze-run.sh \
    ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/logs/sw1.log \
    ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/sw1/tasks.md
```

Outputs a summary including:
- Total steps run
- Pass/fail count
- Per-step durations
- Output file size + line count
- Schema validation result

### Using `scripts/validate-iteration.sh` (8-Point Gate)

```bash
./scripts/validate-iteration.sh \
    ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/logs/sw1.log \
    ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/sw1/tasks.md \
    docs/benchmarks/workflows/sw1-task-deconstruction.yml \
    [previous-iteration-log-path]
```

Exits 0 if all 8 checks pass. Exits non-zero with diagnostic output otherwise.

---

## 5. Diagnostic Patterns

### "Step produced no output"

```
Symptom: total_tokens=0 in log
Cause: LLM call failed silently OR prompt was malformed
Fix: Check shell_output bookmark for error; verify prompt interpolation
```

### "Infinite loop on fix step"

```
Symptom: step_04_fix appears 5+ times in log for same chunk
Cause: GWT gate condition never satisfied
Fix: Hard-cap fix iterations via counter bookmark; advance with WARN
```

### "Sub-workflow invocation timeout"

```
Symptom: step_XX_invoke_swN shell hook duration > 3600000ms (1hr)
Cause: SW is slow due to Qwen3.5-9B CPU bottleneck
Fix: Reduce max_tokens in SW prompts; or split SW into smaller chunks
```

### "YAML parse error in SW5 output"

```
Symptom: validate-yaml.py output contains "ERROR:"
Cause: LLM emitted malformed YAML
Fix: scripts/fix-generated-yaml.py auto-fixes common issues; route to fix step
```

---

## 6. Audit Trail (For Reproducibility)

Every run MUST be reproducible from its artifacts. To reproduce:

```bash
# 1. Read the iteration log entry
cat docs/plans/meta-workflow-generator/08-ITERATION-LOG.md | grep -A 30 "RUN_ID"

# 2. Re-run with same RUN_ID and prompt
RUN_ID="<original-run-id>"
./scripts/generate-workflow.sh \
    --template docs/benchmarks/workflows/sw1-task-deconstruction.yml \
    --run-id "$RUN_ID" \
    --prompt "$(cat docs/plans/meta-workflow-generator/artifacts/dataset/prompt-NN.txt)" \
    --output /tmp/sw1-reproduce.yml

whitt benchmark --workflow /tmp/sw1-reproduce.yml \
    --output-dir /tmp/reproduce-$RUN_ID
```

Note: LLM outputs are non-deterministic (temperature > 0); reproducibility is
approximate. Set `temperature: 0.0` for closer reproducibility (at cost of quality).

---

## 7. Log Retention

- Keep all logs for the latest 3 iterations per SW
- Archive older iterations: `tar -czf archives/sw<N>-iter-NNN.tar.gz <run-dirs>`
- Golden artifacts are kept indefinitely
- Baseline artifacts are kept indefinitely

Cleanup script (future): `scripts/cleanup-old-runs.sh` to enforce retention.
