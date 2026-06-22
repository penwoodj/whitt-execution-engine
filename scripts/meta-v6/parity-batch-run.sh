#!/bin/bash
# scripts/meta-v6/parity-batch-run.sh
# Runs all 11 prompts through Cycle 2 pipeline (engine auto-inject + fix-yaml.py)
# Per docs/plans/meta-workflow-parity/04-ITERATION-STRATEGY.md Phase 5

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-2-results"
 mkdir -p "${RESULTS_DIR}"

echo "=== Cycle 2 Parity Batch Run ==="
echo "Start: $(date)"
echo "Results: ${RESULTS_DIR}"
echo ""

declare -a SUMMARY=()

run_prompt() {
  local n="$1"
  local prompt_file
  prompt_file=$(ls "${PROMPTS_DIR}/prompt-${n}-"*.md 2>/dev/null | head -1)
  [ -z "$prompt_file" ] && {
    echo "[P${n}] SKIP: no prompt file"
    SUMMARY+=("P${n}|SKIP|no prompt|-|-|-")
    return
  }

  local ts
  ts=$(date +%Y%m%d-%H%M%S)
  local prompt_dir="${RESULTS_DIR}/prompt-${n}"
  mkdir -p "${prompt_dir}"

  echo ""
  echo "=== [P${n}] $(basename "$prompt_file") ==="

  # 1. Restart docker for stability
  echo "[P${n}] restarting docker..."
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8

  # 2. Run meta-v6 pipeline
  echo "[P${n}] running meta-v6 pipeline (~45min)..."
  local meta_out="${prompt_dir}/meta-out"
  mkdir -p "${meta_out}/logs"
  local meta_log="${prompt_dir}/meta.log"

  cd "$REPO"
  ./target/release/whitt benchmark \
    --workflow ./docs/benchmarks/workflows/meta-workflow-v6.yml \
    --output-dir "${meta_out}" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${meta_log}" 2>&1 &
  local meta_pid=$!
  wait $meta_pid
  local meta_exit=$?

  if [ "$meta_exit" -ne 0 ]; then
    echo "[P${n}] FAIL: meta-v6 exit ${meta_exit}"
    SUMMARY+=("P${n}|META_FAIL|exit ${meta_exit}|-|-|-")
    return
  fi

  # 3. Find SW5 output (search any timestamp)
  local sw5_yml
  sw5_yml=$(find "${meta_out}" -path "*sw5*" -name "workflow.yml" -o -path "*sw5*" -name "03-assembled.yml" 2>/dev/null | head -1)
  if [ -z "$sw5_yml" ]; then
    echo "[P${n}] FAIL: no SW5 workflow output"
    SUMMARY+=("P${n}|SW5_MISSING|no workflow.yml|-|-|-")
    return
  fi
  echo "[P${n}] SW5: ${sw5_yml}"

  # 4. Run fix-yaml.py
  echo "[P${n}] running fix-yaml.py..."
  python3 "${REPO}/scripts/meta-v6/fix-yaml.py" "$sw5_yml" > "${prompt_dir}/fix-yaml.log" 2>&1
  cp "$sw5_yml" "${prompt_dir}/workflow-fixed.yml"

  # 5. Validate fixed workflow
  if ! python3 -c "import yaml; yaml.safe_load(open('${prompt_dir}/workflow-fixed.yml'))" 2>/dev/null; then
    echo "[P${n}] FAIL: fixed YAML still invalid"
    SUMMARY+=("P${n}|YAML_INVALID|fix-yaml insufficient|-|-|-")
    return
  fi

  # 6. Score structure
  local struct_score
  struct_score=$(bash "${REPO}/scripts/meta-v6/parity-check.sh" "${prompt_dir}/workflow-fixed.yml" 2>/dev/null | grep "^TOTAL:" | awk '{print $2}')
  echo "[P${n}] structure score: ${struct_score}/25"

  # 7. Execute workflow
  echo "[P${n}] executing workflow..."
  local exec_dir="${prompt_dir}/exec"
  mkdir -p "${exec_dir}/outputs/logs"

  # Backup src/ files we might modify
  local backup_dir="${prompt_dir}/src-backup"
  mkdir -p "$backup_dir"
  rsync -a --exclude=target --exclude=outputs --exclude=models --exclude=.git --exclude=node_modules "${REPO}/src/" "$backup_dir/" 2>/dev/null

  ./target/release/whitt benchmark \
    --workflow "${prompt_dir}/workflow-fixed.yml" \
    --output-dir "${exec_dir}" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${prompt_dir}/exec.log" 2>&1 &
  local exec_pid=$!
  wait $exec_pid
  local exec_exit=$?

  # Always restore src/ from backup
  rsync -a --delete "$backup_dir/" "${REPO}/src/" 2>/dev/null

  if [ "$exec_exit" -ne 0 ]; then
    echo "[P${n}] FAIL: workflow exec exit ${exec_exit}"
    SUMMARY+=("P${n}|EXEC_FAIL|exit ${exec_exit}|${struct_score}|-|-")
    return
  fi

  # 8. Count outputs + refusals
  local exec_outputs
  exec_outputs=$(find "${REPO}/outputs" -type f -size +100c -newer "${prompt_dir}/workflow-fixed.yml" 2>/dev/null | wc -l)
  local exec_refuses
  exec_refuses=$(find "${REPO}/outputs" -type f -newer "${prompt_dir}/workflow-fixed.yml" -exec grep -lE "I cannot access|As an AI|I'm unable to" {} \; 2>/dev/null | wc -l)
  local src_changed=0
  if ! diff -r "$backup_dir" "${REPO}/src/" > /dev/null 2>&1; then
    src_changed=1
  fi

  echo "[P${n}] outputs=${exec_outputs}, refuses=${exec_refuses}, src_changed=${src_changed}"

  # 9. Cleanup ./outputs/ for next iteration
  rm -rf "${REPO}/outputs/"*

  SUMMARY+=("P${n}|DONE|exec ok|${struct_score}|outs=${exec_outputs},refuses=${exec_refuses}|src_changed=${src_changed}")
}

for n in 05 06 07 08 09 10 11 12 13 14 15; do
  run_prompt "$n"
done

echo ""
echo "========================================"
echo "FINAL SUMMARY"
echo "========================================"
printf "%s\n" "${SUMMARY[@]}" | tee "${RESULTS_DIR}/summary.txt"
echo ""
echo "End: $(date)"
