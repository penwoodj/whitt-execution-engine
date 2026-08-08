#!/bin/bash
# META-v6 batch prompt runner.
# Replaces: rerun-pXX.sh (5), run-all-prompts.sh, rerun-all-prompts.sh,
#           rerun-failed-prompts.sh, run-remaining-*.sh, exec-all-prompts.sh,
#           run-remaining-and-exec.sh.
#
# Usage:
#   run-prompts.sh --prompt 06 --prompt 07              # specific prompts
#   run-prompts.sh --range 05-24                        # range (inclusive)
#   run-prompts.sh --all                                # all available prompts
#   run-prompts.sh --failed                             # read FAILED_PROMPTS file
#   run-prompts.sh --prompt 06 --no-restart             # skip Docker restart between
#   run-prompts.sh --prompt 06 --timeout 5400           # per-prompt timeout (sec)
#   run-prompts.sh --all --report                       # write markdown report
#   run-prompts.sh --all --load-check                   # validate whitt workflow load per prompt
#
# Output: docs/benchmarks/outputs/meta-workflow/<run-id-prefix><NN>-<ts>/
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPT_DIR="${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real"
PIPELINE="${REPO}/scripts/meta-v6/debug/pipeline.sh"
FAILED_FILE="${REPO}/docs/benchmarks/outputs/meta-workflow/FAILED_PROMPTS.txt"
RESULTS_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow"

PREFIX="${META_PROMPT_PREFIX:-p}"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
RESTART_BETWEEN=true
TIMEOUT=5400
REPORT=false
LOAD_CHECK=false
PROMPTS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prompt) PROMPTS+=("$2"); shift 2 ;;
    --range)
      IFS='-' read -r start end <<< "$2"
      for ((n=10#$start; n<=10#$end; n++)); do
        PROMPTS+=("$(printf '%02d' "$n")")
      done
      shift 2 ;;
    --all)
      for pf in "${PROMPT_DIR}"/prompt-*.md; do
        [[ -f "$pf" ]] || continue
        n=$(basename "$pf" | sed -nE 's/^prompt-([0-9]+)-.*/\1/p')
        [[ -n "$n" ]] && PROMPTS+=("$n")
      done
      shift ;;
    --failed)
      [[ -f "$FAILED_FILE" ]] || { echo "No ${FAILED_FILE}"; exit 1; }
      while read -r n; do [[ -n "$n" ]] && PROMPTS+=("$n"); done < "$FAILED_FILE"
      shift ;;
    --no-restart) RESTART_BETWEEN=false; shift ;;
    --timeout) TIMEOUT="$2"; shift 2 ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    --report) REPORT=true; shift ;;
    --load-check) LOAD_CHECK=true; shift ;;
    -h|--help)
      sed -n '2,17p' "$0"
      exit 0 ;;
    *) echo "Unknown arg: $1" >&2; exit 2 ;;
  esac
done

[[ ${#PROMPTS[@]} -eq 0 ]] && { echo "No prompts selected. Use --prompt/--range/--all/--failed" >&2; exit 2; }

readarray -t PROMPTS < <(printf '%s\n' "${PROMPTS[@]}" | sort -u)

REPORT_FILE=""
if $REPORT; then
  REPORT_FILE="${RESULTS_DIR}/all-prompts-results-${TIMESTAMP}.md"
  cat > "${REPORT_FILE}" <<EOF
# All Prompts Test Results

**Started:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Prompts:** ${PROMPTS[*]}

| # | Prompt | Status | Duration | Bytes | YAML | whitt-loads | Notes |
|---|--------|--------|----------|-------|------|-------------|-------|
EOF
fi

echo "[run-prompts] START $(date)"
echo "[run-prompts] prompts=${PROMPTS[*]} count=${#PROMPTS[@]} timeout=${TIMEOUT}s restart=${RESTART_BETWEEN} report=${REPORT} load-check=${LOAD_CHECK}"

PASS=0
FAIL=0
declare -a FAILED_LIST=()

for p_num in "${PROMPTS[@]}"; do
  prompt_file=$(ls "${PROMPT_DIR}/prompt-${p_num}-"*.md 2>/dev/null | head -1)
  if [[ -z "$prompt_file" ]]; then
    echo "[run-prompts] P${p_num}: SKIP (no prompt file)"
    $REPORT && echo "| ${p_num} | (not found) | SKIP | - | - | - | - | no file |" >> "${REPORT_FILE}"
    continue
  fi
  short_name=$(basename "$prompt_file" .md | head -c 60)

  meta_run_id="${PREFIX}${p_num}-$(date +%Y%m%d-%H%M%S)"
  meta_dir="${REPO}/docs/benchmarks/outputs/meta-workflow/${meta_run_id}"
  log_file="/tmp/${PREFIX}${p_num}-${TIMESTAMP}.log"

  echo "[run-prompts] P${p_num}: starting (${meta_run_id})"

  if $RESTART_BETWEEN; then
    docker restart whitt-llama-server >/dev/null 2>&1 || true
    sleep 10
  fi

  t0=$(date +%s)
  timeout "$TIMEOUT" bash "$PIPELINE" "$prompt_file" "/tmp/${PREFIX}${p_num}-deliverable.md" "$meta_run_id" > "$log_file" 2>&1
  exit_code=$?
  t1=$(date +%s)
  duration=$((t1 - t0))

  deliverable=""
  for loc in "deliverables/deliverable.md" "exec/outputs/deliverable.md"; do
    if [[ -f "${meta_dir}/${loc}" ]]; then
      [[ "$loc" = "exec/outputs/deliverable.md" ]] && {
        mkdir -p "${meta_dir}/deliverables"
        cp "${meta_dir}/${loc}" "${meta_dir}/deliverables/deliverable.md"
      }
      deliverable="${meta_dir}/deliverables/deliverable.md"
      break
    fi
  done

  size=0
  yaml_valid="n/a"
  whitt_loads="n/a"
  notes=""
  status="FAIL"

  if [[ -n "$deliverable" && $(wc -c < "$deliverable") -gt 100 ]]; then
    size=$(wc -c < "$deliverable")
    status="PASS"
    notes="ok"
    PASS=$((PASS + 1))
  else
    notes="exit=${exit_code} no deliverable"
    FAIL=$((FAIL + 1))
    FAILED_LIST+=("$p_num")
  fi

  if $LOAD_CHECK; then
    gen_wf="${meta_dir}/meta/generated-workflow.yml"
    if [[ -f "$gen_wf" ]]; then
      if python3 -c "import yaml; yaml.safe_load(open('${gen_wf}'))" 2>/dev/null; then
        yaml_valid="yes"
        if "${REPO}/target/release/whitt" workflow "$gen_wf" > /tmp/whitt-load-${p_num}.log 2>&1; then
          whitt_loads="yes"
        else
          whitt_loads="no"
          notes="${notes}; whitt-load-fail"
        fi
      else
        yaml_valid="no"
        notes="${notes}; yaml-parse-fail"
      fi
    fi
  fi

  echo "[run-prompts] P${p_num}: ${status} (${duration}s, ${size}B, yaml=${yaml_valid} whitt=${whitt_loads})"

  $REPORT && echo "| ${p_num} | ${short_name} | ${status} | ${duration}s | ${size}B | ${yaml_valid} | ${whitt_loads} | ${notes} |" >> "${REPORT_FILE}"
done

if [[ ${#FAILED_LIST[@]} -gt 0 ]]; then
  printf '%s\n' "${FAILED_LIST[@]}" > "$FAILED_FILE"
  echo "[run-prompts] Failed list written to ${FAILED_FILE}"
fi

if $REPORT; then
  cat >> "${REPORT_FILE}" <<EOF

## Summary

- **PASS:** ${PASS}
- **FAIL:** ${FAIL}
- **Completed:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
EOF
  echo "[run-prompts] Report: ${REPORT_FILE}"
fi

echo "[run-prompts] COMPLETE pass=${PASS} fail=${FAIL} at $(date)"
[[ $FAIL -eq 0 ]] && exit 0 || exit 1
