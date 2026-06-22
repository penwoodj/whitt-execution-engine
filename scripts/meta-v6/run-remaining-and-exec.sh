#!/bin/bash
# Run remaining prompts (12-15) through meta-v6 + execute in-scope workflows
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-2-results"

mkdir -p "$RESULTS_DIR"

run_meta_v6() {
  local n="$1"
  local prompt_file
  prompt_file=$(ls "${PROMPTS_DIR}/prompt-${n}-"*.md 2>/dev/null | head -1)
  [ -z "$prompt_file" ] && { echo "[P${n}] SKIP: no prompt"; return 1; }

  echo "=== [P${n}] $(basename "$prompt_file") ==="
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8

  cd "$REPO"
  local meta_out="${RESULTS_DIR}/prompt-${n}/meta-out"
  mkdir -p "${meta_out}/logs"

  echo "[P${n}] meta-v6 (~45min)..."
  ./target/release/whitt benchmark \
    --workflow ./docs/benchmarks/workflows/meta-workflow-v6.yml \
    --output-dir "${meta_out}" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${RESULTS_DIR}/prompt-${n}/meta.log" 2>&1
  local exit=$?

  if [ "$exit" -ne 0 ]; then
    echo "[P${n}] FAIL: meta-v6 exit ${exit}"
    return 1
  fi

  # Locate SW5 (search both possible output locations)
  local sw5_yml
  sw5_yml=$(find "${REPO}/docs/benchmarks/outputs/meta-workflow" -maxdepth 1 -type d -name "meta-meta-v6-*-sw5-*" 2>/dev/null | sort | tail -1 | xargs -I{} find "{}" -name "workflow.yml" -o -name "03-assembled.yml" 2>/dev/null | head -1)

  if [ -z "$sw5_yml" ]; then
    echo "[P${n}] FAIL: no SW5 output"
    return 1
  fi

  cp "$sw5_yml" "${RESULTS_DIR}/prompt-${n}/workflow-raw.yml"
  python3 "${REPO}/scripts/meta-v6/fix-yaml.py" "$sw5_yml" > "${RESULTS_DIR}/prompt-${n}/fix-yaml.log" 2>&1
  cp "$sw5_yml" "${RESULTS_DIR}/prompt-${n}/workflow-fixed.yml"

  bash "${REPO}/scripts/meta-v6/parity-check.sh" "${RESULTS_DIR}/prompt-${n}/workflow-fixed.yml" > "${RESULTS_DIR}/prompt-${n}/structure-score.txt" 2>&1
  echo "[P${n}] struct: $(grep '^STRUCTURAL:' ${RESULTS_DIR}/prompt-${n}/structure-score.txt)"
  return 0
}

for n in 12 13 14 15; do
  run_meta_v6 "$n"
done

echo ""
echo "=== Phase 1 complete ==="

echo "=== Phase 2: Executing in-scope workflows ==="

exec_workflow() {
  local n="$1"
  local wf="${RESULTS_DIR}/prompt-${n}/workflow-fixed.yml"
  [ ! -f "$wf" ] && { echo "[P${n}] SKIP exec: no workflow"; return; }

  echo "[P${n}] executing..."
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8

  cd "$REPO"
  local exec_dir="${RESULTS_DIR}/prompt-${n}/exec"
  mkdir -p "${exec_dir}/outputs/logs"

  local backup_dir="${RESULTS_DIR}/prompt-${n}/src-backup"
  mkdir -p "$backup_dir"
  rsync -a --exclude=target --exclude=outputs --exclude=models --exclude=.git "${REPO}/src/" "$backup_dir/" 2>/dev/null

  ./target/release/whitt benchmark \
    --workflow "$wf" \
    --output-dir "${exec_dir}" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${RESULTS_DIR}/prompt-${n}/exec.log" 2>&1
  local exit=$?

  rsync -a --delete "$backup_dir/" "${REPO}/src/" 2>/dev/null

  if [ "$exit" -ne 0 ]; then
    echo "[P${n}] EXEC FAIL: exit ${exit}"
    return
  fi

  bash "${REPO}/scripts/meta-v6/parity-check.sh" "$wf" "${exec_dir}" > "${RESULTS_DIR}/prompt-${n}/exec-score.txt" 2>&1
  echo "[P${n}] exec: $(grep '^TOTAL:' ${RESULTS_DIR}/prompt-${n}/exec-score.txt) $(grep '^VERDICT:' ${RESULTS_DIR}/prompt-${n}/exec-score.txt | tail -1)"

  rm -rf "${REPO}/outputs/"*
}

for n in 05 07 10 11 12 13 14 15; do
  exec_workflow "$n"
done

echo ""
echo "=== Phase 2 complete ==="
echo "=== Final summary ==="
for n in 05 06 07 08 09 10 11 12 13 14 15; do
  d="${RESULTS_DIR}/prompt-${n}"
  [ ! -d "$d" ] && continue
  s="?"
  e="NA"
  [ -f "${d}/structure-score.txt" ] && s=$(grep '^STRUCTURAL:' "${d}/structure-score.txt" | awk '{print $2}' | tr -d '/25')
  [ -f "${d}/exec-score.txt" ] && e=$(grep '^TOTAL:' "${d}/exec-score.txt" | awk '{print $2}' | tr -d '/50')
  v="NA"
  [ -f "${d}/exec-score.txt" ] && v=$(grep '^VERDICT:' "${d}/exec-score.txt" | tail -1 | awk '{print $2}')
  echo "P${n}: struct=${s}/25 exec=${e}/50 ${v}"
done | tee "${RESULTS_DIR}/final-summary.txt"
