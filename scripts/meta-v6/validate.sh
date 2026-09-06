#!/bin/bash
# META-v6 validator: checks final generated-workflow.yml exists and is valid YAML
# Output: preview of generated workflow + line count + YAML validity check
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
WF="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta/generated-workflow.yml"

if [[ ! -f "${WF}" ]]; then
  echo "FAIL: generated-workflow.yml not found at ${WF}"
  exit 1
fi

LINES=$(wc -l < "${WF}")
BYTES=$(wc -c < "${WF}")
echo "=== generated-workflow.yml (${LINES} lines, ${BYTES} bytes) ==="
head -30 "${WF}"
echo "..."
echo "=== YAML validity ==="
if python3 "${REPO}/scripts/validate-yaml.py" "${WF}" >/dev/null 2>&1; then
  echo "YAML_VALID"
else
  echo "YAML_INVALID_OR_NO_VALIDATOR"
fi
echo "=== META-v6 PIPELINE COMPLETE: ${META_RUN_ID} ==="
