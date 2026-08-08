#!/bin/bash
# scripts/meta-v6/manual-runs-abc.sh
#
# Runs the 3 prompts (A/B/C) from META-WORKFLOW-MANUAL-RUNS.md end-to-end:
#   1. SW1-SW5 cascade to generate workflow.yml per prompt
#   2. Execute the generated workflow with the engine
#   3. Capture all artifacts + parity scores + execution output
#
# No fixing, just running. Per AGENTS.md synchronous rule: sequential, no
# background task() delegation. Uses nohup detach for long-running batch.
#
# Output: docs/benchmarks/outputs/meta-workflow/manual-runs-<ts>/
#
# Usage: manual-runs-abc.sh

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/manual-runs-20260803/inputs"
TS=$(date +%Y%m%d-%H%M%S)
RESULTS_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/manual-runs-${TS}"
mkdir -p "${RESULTS_DIR}"

# 3 prompts: A, B, C
PROMPTS=(
  "A:prompt-A-yaml-project-plan.md"
  "B:prompt-B-meeting-action-items.md"
  "C:prompt-C-gherkin-test-matrix.md"
)

SUMMARY_FILE="${RESULTS_DIR}/summary.md"
LOG_FILE="${RESULTS_DIR}/runner.log"

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOG_FILE"; }

cat > "${SUMMARY_FILE}" <<EOF
# Manual Runs (Prompts A/B/C) End-to-End

**Started:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Model:** Qwen3-5-9B-Q4_K_M (current meta-workflow-v6.yml config)

| P# | SW1-SW5 | bytes | parity | refuses | lines | generated.yml OK? | exec exit | exec output bytes | exec output |
|----|---------|-------|--------|---------|-------|-------------------|-----------|-------------------|-------------|
EOF

log "=== Manual Runs ABC ==="
log "Results: ${RESULTS_DIR}"

run_one_prompt() {
  local entry="$1"
  local label="${entry%%:*}"
  local filename="${entry##*:}"
  local prompt_file="${PROMPTS_DIR}/${filename}"

  if [ ! -s "$prompt_file" ]; then
    log "P${label}: SKIP no prompt file at ${prompt_file}"
    echo "| ${label} | - | - | - | - | - | no prompt | - | - | - |" >> "${SUMMARY_FILE}"
    return
  fi

  log "P${label}: starting (${filename})"

  # Hardware safety: docker restart + RAM check
  log "P${label}: docker restart"
  docker restart whitt-llama-server > /dev/null 2>&1 || log "P${label}: WARN docker restart failed"
  sleep 30

  local ram_avail
  ram_avail=$(awk '/MemAvailable/ {printf "%d", $2/1024}' /proc/meminfo)
  if [ "$ram_avail" -lt 3000 ]; then
    log "P${label}: ABORT RAM low (${ram_avail}MB avail)"
    echo "| ${label} | - | - | - | - | - | ram_abort | - | - | - |" >> "${SUMMARY_FILE}"
    return
  fi
  log "P${label}: RAM ok (${ram_avail}MB avail)"

  # Save prompt copy
  cp "$prompt_file" "${RESULTS_DIR}/p${label}-prompt.md"

  # Bootstrap creates META_RUN_ID + dirs + copies prompt
  local boot_out
  boot_out=$(bash "${REPO}/scripts/meta-v6/bootstrap.sh" "$prompt_file" 2>&1) || {
    log "P${label}: BOOTSTRAP FAIL"
    echo "| ${label} | bootstrap_fail | - | - | - | - | - | - | - | - |" >> "${SUMMARY_FILE}"
    return
  }
  local meta_run_id
  meta_run_id=$(echo "$boot_out" | grep '^META_RUN_ID=' | cut -d= -f2)
  log "P${label}: META_RUN_ID=${meta_run_id}"

  # Run SW1-SW5 wrappers sequentially
  local sw_exits="" fail=0
  for sw in sw1 sw2 sw3 sw4 sw5; do
    log "P${label}: running ${sw}..."
    if bash "${REPO}/scripts/meta-v6/run-${sw}.sh" "$meta_run_id" "$prompt_file" >> "${RESULTS_DIR}/p${label}-${sw}.log" 2>&1; then
      sw_exits+="${sw}:OK "
    else
      sw_exits+="${sw}:FAIL(${?}) "
      fail=1
      break
    fi
  done

  if [ "$fail" -ne 0 ]; then
    log "P${label}: SW PIPELINE FAIL - ${sw_exits}"
    echo "| ${label} | ${sw_exits} | - | - | - | - | FAIL | - | - | - |" >> "${SUMMARY_FILE}"
    return
  fi

  # Locate generated-workflow.yml
  local generated="${REPO}/docs/benchmarks/outputs/meta-workflow/${meta_run_id}/meta/generated-workflow.yml"
  if [ ! -s "$generated" ]; then
    log "P${label}: NO generated-workflow.yml"
    echo "| ${label} | ${sw_exits} | 0 | - | - | - | FAIL (no yml) | - | - | - |" >> "${SUMMARY_FILE}"
    return
  fi

  # Copy to results
  cp "$generated" "${RESULTS_DIR}/p${label}-workflow.yml"
  local bytes
  bytes=$(wc -c < "$generated")
  local lines
  lines=$(wc -l < "$generated")
  log "P${label}: generated-workflow.yml = ${bytes}B / ${lines}L"

  # Parity-check structural validation
  local parity_out parity_score
  parity_out=$(bash "${REPO}/scripts/meta-v6/parity-check.sh" "$generated" "$generated" 2>&1 || true)
  parity_score=$(echo "$parity_out" | grep -oE 'Score: [0-9]+/[0-9]+' | head -1)
  echo "$parity_out" > "${RESULTS_DIR}/p${label}-parity.log"
  log "P${label}: parity ${parity_score}"

  # Refusal check
  local refuses
  refuses=$(grep -cE "I cannot|I'm unable|I don't have access|As an AI" "$generated" 2>/dev/null || echo 0)

  local yml_ok="yes"
  if [ "$bytes" -lt 500 ] || [ "$lines" -lt 10 ]; then
    yml_ok="THIN"
  fi

  # === EXECUTE THE GENERATED WORKFLOW ===
  log "P${label}: executing generated workflow..."
  local exec_dir="${RESULTS_DIR}/p${label}-exec"
  mkdir -p "$exec_dir"

  # Docker restart before execution too (model swap from SW5)
  docker restart whitt-llama-server > /dev/null 2>&1 || true
  sleep 20

  local exec_out_file="${RESULTS_DIR}/p${label}-exec.log"
  local exec_exit=0
  timeout 3600 ./target/release/whitt benchmark \
    --workflow "$generated" \
    --output-dir "$exec_dir" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 1800 > "$exec_out_file" 2>&1 || exec_exit=$?
  log "P${label}: exec exit=${exec_exit}"

  # Capture execution outputs
  local exec_bytes=0 exec_outputs=0
  if [ -d "${exec_dir}/outputs" ]; then
    exec_outputs=$(find "${exec_dir}/outputs" -type f | wc -l)
    exec_bytes=$(find "${exec_dir}/outputs" -type f -exec cat {} + 2>/dev/null | wc -c)
  fi
  log "P${label}: exec outputs=${exec_outputs} files, ${exec_bytes}B total"

  # Sample of execution output (first output file's content, truncated)
  local sample="(no outputs)"
  local first_out
  first_out=$(find "${exec_dir}/outputs" -type f 2>/dev/null | head -1)
  if [ -n "$first_out" ]; then
    sample=$(head -c 500 "$first_out" 2>/dev/null | tr '\n' ' ' | sed 's/|/\\|/g')
  fi

  echo "| ${label} | ${sw_exits} | ${bytes} | ${parity_score} | ${refuses} | ${lines} | ${yml_ok} | ${exec_exit} | ${exec_bytes} | ${sample:0:200} |" >> "${SUMMARY_FILE}"

  # Save execution artifacts path
  echo "exec_dir: ${exec_dir}" > "${RESULTS_DIR}/p${label}-exec-path.txt"
  ls -la "$exec_dir" >> "${RESULTS_DIR}/p${label}-exec-path.txt" 2>&1
}

for entry in "${PROMPTS[@]}"; do
  run_one_prompt "$entry"
done

# Final summary
PASS_COUNT=$(grep -c "| yes |" "$SUMMARY_FILE" || echo 0)
EXEC_OK=$(grep -c "| 0 |" "$SUMMARY_FILE" || echo 0)

cat >> "${SUMMARY_FILE}" <<EOF

## Summary

- **Generated workflows structurally OK:** ${PASS_COUNT}/3
- **Execution exit 0:** ${EXEC_OK}/3

**Ended:** $(date -u +%Y-%m-%dT%H:%M:%SZ)

NOTE: This run executed generated workflows against live Docker llama.cpp.
      Inspect p<X>-exec/outputs/ for actual model output content.
      Inspect p<X>-workflow.yml for the generated workflow structure.
EOF

log "=== DONE ==="
log "Summary: ${SUMMARY_FILE}"
echo "${RESULTS_DIR}" > "${REPO}/.last-manual-run"
