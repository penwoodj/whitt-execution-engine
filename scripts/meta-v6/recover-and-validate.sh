#!/bin/bash
# Recover + validate existing SW5 workflow.yml outputs from 06-19 batch run.
# Validates: YAML parses + whitt loads + required schema keys.
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
cd "${REPO}"

# Map prompt numbers to META run timestamps (from runner execution order)
declare -A PROMPT_TS
PROMPT_TS[05]="113305"
PROMPT_TS[06]="122447"
PROMPT_TS[07]="131256"
PROMPT_TS[08]="140236"
PROMPT_TS[09]="144611"
PROMPT_TS[10]="153433"
PROMPT_TS[11]="162609"
PROMPT_TS[12]="171422"
PROMPT_TS[13]="180828"
PROMPT_TS[14]="185610"
PROMPT_TS[15]="192508"

RESULTS_FILE="docs/benchmarks/outputs/meta-workflow/recovered-results-$(date +%Y%m%d-%H%M%S).md"

cat > "${RESULTS_FILE}" <<EOF
# Recovered SW5 Workflow Validation

**Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Source:** Run from 2026-06-19 (runner bug fixed, validating existing outputs)

| # | SW5 dir | Status | Lines | YAML parses | whitt loads | Has steps | Provider ok | Notes |
|---|---------|--------|-------|-------------|-------------|-----------|-------------|-------|
EOF

echo "Validating 11 SW5 outputs..."

PASS=0
FAIL=0
for num in 05 06 07 08 09 10 11 12 13 14 15; do
  ts=${PROMPT_TS[$num]}
  sw5dir=$(ls -d docs/benchmarks/outputs/meta-workflow/meta-meta-v6-20260619-${ts}-sw5-* 2>/dev/null | head -1)

  if [ -z "$sw5dir" ]; then
    echo "| ${num} | NOT FOUND | FAIL | - | - | - | - | - | no SW5 dir for ts=${ts} |" >> "${RESULTS_FILE}"
    FAIL=$((FAIL+1))
    continue
  fi

  wf="${sw5dir}/sw5/workflow.yml"
  if [ ! -f "$wf" ]; then
    echo "| ${num} | $(basename $sw5dir) | FAIL | - | - | - | - | - | workflow.yml missing |" >> "${RESULTS_FILE}"
    FAIL=$((FAIL+1))
    continue
  fi

  lines=$(wc -l < "$wf")
  short=$(basename "$sw5dir" | head -c 50)

  # 1. YAML parses
  yaml_status="no"
  if python3 -c "import yaml; yaml.safe_load(open('$wf'))" 2>/dev/null; then
    yaml_status="yes"
  fi

  # 2. whitt workflow load
  whitt_status="no"
  whitt_note=""
  if [ "$yaml_status" = "yes" ]; then
    if ./target/release/whitt workflow "$wf" > "${sw5dir}/whitt-load-recover.log" 2>&1; then
      whitt_status="yes"
    else
      whitt_status="no"
      whitt_note=$(tail -3 "${sw5dir}/whitt-load-recover.log" | head -c 200 | tr '\n' ' ')
    fi
  fi

  # 3. Required keys
  keys_status="no"
  steps_count=0
  provider_ok="no"
  if [ "$yaml_status" = "yes" ]; then
    keys_status=$(python3 -c "
import yaml
d = yaml.safe_load(open('$wf'))
required = ['workflow_id', 'name', 'providers', 'models', 'agentic_workflow']
missing = [k for k in required if k not in d]
if missing:
    print('MISSING:' + ','.join(missing))
else:
    steps = d.get('agentic_workflow', {}).get('steps', {})
    n = len(steps) if isinstance(steps, dict) else 0
    providers = d.get('providers', {})
    if 'llama_cpp_with_vulkan' in providers:
        print(f'OK:{n}')
    else:
        print(f'BAD_PROVIDER:{list(providers.keys())}')
" 2>&1)
    if [[ "$keys_status" == OK:* ]]; then
      keys_status="yes"
      steps_count=${keys_status#OK:}
      provider_ok="yes"
    fi
  fi

  overall="FAIL"
  notes=""
  if [ "$yaml_status" = "yes" ] && [ "$whitt_status" = "yes" ] && [ "$keys_status" = "yes" ]; then
    overall="PASS"
    PASS=$((PASS+1))
  else
    FAIL=$((FAIL+1))
    notes="${whitt_note}"
  fi

  echo "| ${num} | ${short} | ${overall} | ${lines} | ${yaml_status} | ${whitt_status} | ${steps_count} | ${provider_ok} | ${notes} |" >> "${RESULTS_FILE}"
  echo "  PROMPT ${num}: ${overall} (yaml=${yaml_status} whitt=${whitt_status} keys=${keys_status})"
done

cat >> "${RESULTS_FILE}" <<EOF

## Summary

- **PASS:** ${PASS}/11
- **FAIL:** ${FAIL}/11
EOF

echo ""
echo "=========================================="
echo "RESULTS: ${PASS} PASS / ${FAIL} FAIL of 11"
echo "Full report: ${RESULTS_FILE}"
cat "${RESULTS_FILE}" | tail -20
