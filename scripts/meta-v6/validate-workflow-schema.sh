#!/bin/bash
# Schema lint for SW5 output workflow.yml files.
# Quick static check: YAML parses + has required top-level keys + providers correct.
# Usage: validate-workflow-schema.sh <workflow.yml>
set -uo pipefail

WF="$1"
if [[ ! -f "$WF" ]]; then
  echo "MISSING"
  exit 1
fi

# 1. YAML parses
if ! python3 -c "import yaml; d=yaml.safe_load(open('$WF'))" 2>/dev/null; then
  echo "YAML_PARSE_FAIL"
  exit 1
fi

# 2. Required top-level keys
REQUIRED_KEYS=("workflow_id" "name" "providers" "models" "agentic_workflow")
MISSING=""
for k in "${REQUIRED_KEYS[@]}"; do
  if ! python3 -c "import yaml; d=yaml.safe_load(open('$WF')); assert '$k' in d, 'missing $k'" 2>/dev/null; then
    MISSING="${MISSING}${k} "
  fi
done
if [[ -n "$MISSING" ]]; then
  echo "MISSING_KEYS: ${MISSING}"
  exit 1
fi

# 3. Provider must be llama_cpp_with_vulkan
if ! python3 -c "
import yaml
d=yaml.safe_load(open('$WF'))
assert 'llama_cpp_with_vulkan' in d.get('providers', {}), 'wrong provider'
" 2>/dev/null; then
  echo "WRONG_PROVIDER"
  exit 1
fi

# 4. agentic_workflow must have steps
STEPS=$(python3 -c "
import yaml
d=yaml.safe_load(open('$WF'))
steps = d.get('agentic_workflow', {}).get('steps', {})
print(len(steps) if isinstance(steps, dict) else 0)
" 2>/dev/null)
if [[ "${STEPS:-0}" -lt 1 ]]; then
  echo "NO_STEPS"
  exit 1
fi

echo "VALID (${STEPS} steps)"
exit 0
