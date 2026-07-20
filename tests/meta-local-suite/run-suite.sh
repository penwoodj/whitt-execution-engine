#!/bin/bash
# tests/meta-local-suite/run-suite.sh
# Runs the 5-prompt meta-workflow suite end-to-end and functionally tests
# each result. Each prompt: generate → execute → verify → functional test.
#
# Usage:
#   run-suite.sh              # run all prompts not yet PASSed
#   run-suite.sh p03 p05      # run specific prompts
#
# State: results recorded in tests/meta-local-suite/results.tsv
#   columns: prompt  run_dir  pipeline_exit  functional  timestamp
set -uo pipefail

SUITE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "${SUITE_DIR}/../.." && pwd)"
RESULTS="${SUITE_DIR}/results.tsv"
touch "$RESULTS"

PROMPTS=("$@")
if [ ${#PROMPTS[@]} -eq 0 ]; then
  PROMPTS=()
  for f in "${SUITE_DIR}"/p0*-*.md; do
    P=$(basename "$f" | cut -d- -f1)
    # skip prompts that already have a FUNC_PASS row
    if ! grep -q "^${P}.*FUNC_PASS" "$RESULTS" 2>/dev/null; then
      PROMPTS+=("$P")
    fi
  done
fi

echo "suite: running prompts: ${PROMPTS[*]:-none (all passed)}"

for P in "${PROMPTS[@]}"; do
  PROMPT_FILE=$(ls "${SUITE_DIR}/${P}-"*.md 2>/dev/null | head -1)
  TEST_FILE="${SUITE_DIR}/${P}-test.sh"
  [ -z "$PROMPT_FILE" ] && { echo "suite: no prompt file for $P, skipping"; continue; }

  echo ""
  echo "════════════════════════════════════════════════════════"
  echo "suite: $P — $(basename "$PROMPT_FILE")  $(date '+%H:%M:%S')"
  echo "════════════════════════════════════════════════════════"

  # keep the machine awake for the long run
  caffeinate -i /bin/bash "${REPO}/scripts/meta-v6/run-meta-local.sh" "$PROMPT_FILE" "suite-${P}"
  PIPE_EXIT=$?

  RUN_DIR=$(ls -td "${REPO}/docs/benchmarks/outputs/meta-workflow/local-runs/suite-${P}-"* 2>/dev/null | head -1)
  FUNC="SKIPPED"
  if [ -n "$RUN_DIR" ] && [ -x "$TEST_FILE" ] || [ -f "$TEST_FILE" ]; then
    if [ -d "${RUN_DIR}/artifacts" ]; then
      echo "suite: $P functional test"
      if bash "$TEST_FILE" "${RUN_DIR}/artifacts" "$REPO"; then
        FUNC="FUNC_PASS"
      else
        FUNC="FUNC_FAIL"
      fi
    else
      FUNC="FUNC_FAIL(no-artifacts)"
    fi
  fi

  printf '%s\t%s\t%s\t%s\t%s\n' "$P" "${RUN_DIR:-none}" "$PIPE_EXIT" "$FUNC" "$(date '+%Y-%m-%d %H:%M:%S')" >> "$RESULTS"
  echo "suite: $P → pipeline_exit=$PIPE_EXIT functional=$FUNC"
done

echo ""
echo "════════════ suite results ════════════"
column -t -s$'\t' "$RESULTS" 2>/dev/null || cat "$RESULTS"
