# Self-Healing Experiment — Runbook (Phase S: Zero-LLM)

**HARD CONSTRAINT:** NO model load/unload/inference. All LLM behavior spoofed. Phase L requires explicit user authorization.

## One-Time Setup

```bash
# from repo root
cargo build --release          # whitt binary (no server needed for Phase S)
```

## Per-Case Run

```bash
# 1. Generate workflow for case
python3 experiments/self-healing/scripts/gen-v1.py --case experiments/self-healing/cases/probe/case-f1-01.yml

# 2. Structural validation (MUST exit 0)
python3 scripts/meta-v6/validate-workflow.py experiments/self-healing/workflows/generated/sh-v1-f1-01.yml

# 3. Live zero-LLM run
./target/release/whitt benchmark --workflow experiments/self-healing/workflows/generated/sh-v1-f1-01.yml --log-file experiments/self-healing/results/sh-v1/f1-01/benchmark.log
# (check whitt benchmark --help for exact log/output flags on your build)

# 4. Inspect artifacts
#    experiments/self-healing/results/sh-v1/<case>/trace.jsonl   — event sequence
#    experiments/self-healing/results/sh-v1/<case>/report.json   — metrics + oracle result
```

## Full Suite

```bash
for c in experiments/self-healing/cases/probe/*.yml; do
  python3 experiments/self-healing/scripts/gen-v1.py --case "$c"
done
# then run each generated workflow, then aggregate:
python3 experiments/self-healing/scripts/report.py --suite experiments/self-healing/results/sh-v1
```

## Zero-LLM Proof Gate (every run)

```bash
LOG=experiments/self-healing/results/sh-v1/<case>/benchmark.log
grep -ciE "model_load|model_unload|chat/completion|inference_request|loading model" "$LOG"   # MUST be 0
grep -c "skipped by before_step_starts hook" "$LOG"                                          # >0 expected
```

Failure of the zero gate = STOP, fix workflow (a step reached inference), never "just this once" allow.

## Expected Per-Case Outcomes (oracle)

| Case | final | attempts | heal routes taken |
|------|-------|----------|-------------------|
| clean-01 | pass | 1 | none |
| f1-01 | pass | 2 | corrective_prompt |
| f2-01 | pass | 2 | tool_reselect |
| f3-01 | pass | 2 | replan |
| f4-01 | pass | 3 | replan, replan |
| f1-persist-01 | fail | 3 | corrective_prompt ×3 |

Suite: TSR=5/6, FDA=8/8, RSR=4/5.

## Iteration Discipline

Per AGENTS.md Generator Iteration Protocol:
1. generate → 2. validate → 3. live run → 4. inspect trace/report → 5. bug analysis → 6. fix ONE thing → repeat.
Log every iteration in 06-TRACKING.md with evidence.

## Safety

- No docker/server interaction required in Phase S. Do NOT start whitt-llama-server for spoof runs.
- If any run unexpectedly attempts server contact: kill, diagnose skip_step/gwt coverage, fix before re-run.
