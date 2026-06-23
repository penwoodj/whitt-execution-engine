#!/bin/bash
set -uo pipefail
REPO="/home/jon/code/whitt-execution-engine"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-2-results"

for n in 06 08 09; do
  d="${RESULTS_DIR}/prompt-${n}"
  wf="${d}/workflow-fixed.yml"
  [ -f "$wf" ] || { echo "[P${n}] SKIP: no workflow"; continue; }

  echo "=== [P${n}] executing ==="
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8

  backup_dir="${d}/src-backup"
  mkdir -p "$backup_dir"
  rsync -a --exclude=target --exclude=outputs --exclude=models --exclude=.git "${REPO}/src/" "$backup_dir/" 2>/dev/null

  cd "$REPO"
  rm -rf "${REPO}/outputs/"*
  ./target/release/whitt benchmark \
    --workflow "$wf" \
    --output-dir "${d}/exec" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${d}/exec.log" 2>&1
  rc=$?

  rsync -a --delete "$backup_dir/" "${REPO}/src/" 2>/dev/null

  if [ "$rc" -ne 0 ]; then
    echo "[P${n}] FAIL: exit $rc"
    continue
  fi

  bash "${REPO}/scripts/meta-v6/parity-check.sh" "$wf" "${d}/exec" > "${d}/exec-score.txt" 2>&1
  total=$(grep "^TOTAL:" "${d}/exec-score.txt" | awk '{print $2}')
  verdict=$(grep "^VERDICT:" "${d}/exec-score.txt" | awk '{$1=""; print $0}' | sed 's/^ *//')
  echo "[P${n}] exec=${total} verdict=${verdict}"

  rm -rf "${REPO}/outputs/"*
done

echo "=== Final summary ==="
for n in 05 06 07 08 09 10 11 12 13 14 15; do
  d="${RESULTS_DIR}/prompt-${n}"
  struct=$(grep "^TOTAL:" "${d}/structure-score.txt" 2>/dev/null | head -1 | awk '{print $2}')
  exec=$(grep "^TOTAL:" "${d}/exec-score.txt" 2>/dev/null | head -1 | awk '{print $2}')
  printf "P%s: struct=%s exec=%s\n" "$n" "${struct:-NA}" "${exec:-NA}"
done | tee "${RESULTS_DIR}/final-summary.txt"
