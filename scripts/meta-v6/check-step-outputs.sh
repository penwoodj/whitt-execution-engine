#!/bin/bash
# scripts/meta-v6/check-step-outputs.sh
# Per-step output quality checker.
#
# Inspects EVERY step output file in an exec dir, looking for:
#   1. Empty files (size 0)
#   2. Refusal patterns ("I cannot", "As an AI", etc.)
#   3. Hallucination patterns (fake crate names, nonexistent modules)
#   4. Placeholder/TODO content
#   5. Markdown fence leakage (```yaml in output)
#
# Outputs JSON to stdout for consumption by integration-test.sh.
#
# Usage: check-step-outputs.sh <exec-dir>
#   <exec-dir> contains output/ subdir with step_*.txt files

set -uo pipefail

EXEC_DIR="${1:?usage: $0 <exec-dir>}"

if [ ! -d "$EXEC_DIR" ]; then
  echo "{\"error\": \"exec dir not found: $EXEC_DIR\"}"
  exit 1
fi

OUTPUT_DIR="${EXEC_DIR}/output"
if [ ! -d "$OUTPUT_DIR" ]; then
  OUTPUT_DIR="${EXEC_DIR}/outputs/output"
fi
# Some run layouts put step logs in logs/ + deliverables in deliverables/
LOGS_DIR="${EXEC_DIR}/logs"
DELIVERABLES_DIR="${EXEC_DIR}/deliverables"
# Top-level run layout (not exec/ subdir): logs/ at run root
[ ! -d "$LOGS_DIR" ] && LOGS_DIR="${EXEC_DIR}/../logs"
[ ! -d "$DELIVERABLES_DIR" ] && DELIVERABLES_DIR="${EXEC_DIR}/../deliverables"

# If no step outputs found in output/, fall back to logs/step_*.log + deliverables/
HAS_STEP_OUTPUTS=$(ls "$OUTPUT_DIR"/step_* 2>/dev/null | wc -l)
if [ "$HAS_STEP_OUTPUTS" -eq 0 ] && [ -d "$LOGS_DIR" ]; then
  OUTPUT_DIR="$LOGS_DIR"
fi

# Refusal patterns (per REALITY-ASSESSMENT.md C5 + extended)
REFUSAL_RE='I cannot|I'"'"'m unable|I don'"'"'t have access|As an AI|I would need to (read|access|see)|cannot access|cannot read|do not have access|I do not have|I'"'"'m not able to'

# Hallucination patterns: fake crate refs, nonexistent modules, made-up line numbers
HALLUC_RE='crate::ast|crate::unknown|use crate::nonexistent|// line [0-9]{4,}|fake_|placeholder_'

# Placeholder/TODO patterns
PLACEHOLDER_RE='TODO:|FIXME:|XXX:|<INSERT|<PLACEHOLDER|<YOUR_|<FILL|<IMPLEMENT|<TBD>|<PENDING>'

# Markdown fence leakage
FENCE_RE='^```(yaml|yml|rust|typescript|tsx|python)'

# Scan all step_*.txt files
STEPS_JSON="["
FIRST=1

shopt -s nullglob
STEP_FILES=( "$OUTPUT_DIR"/step_*.txt "$OUTPUT_DIR"/step_*.rs "$OUTPUT_DIR"/step_*.json "$OUTPUT_DIR"/step_*.md "$OUTPUT_DIR"/step_*.log )
shopt -u nullglob

# Also include the final deliverable if present
if [ -d "$DELIVERABLES_DIR" ]; then
  for d in "$DELIVERABLES_DIR"/*; do
    [ -f "$d" ] && STEP_FILES+=("$d")
  done
fi

if [ ${#STEP_FILES[@]} -eq 0 ]; then
  echo "{\"error\": \"no step_* files in $OUTPUT_DIR or $DELIVERABLES_DIR\", \"steps\": []}"
  exit 0
fi

for f in "${STEP_FILES[@]}"; do
  fname=$(basename "$f")
  size=$(stat -c %s "$f" 2>/dev/null || echo 0)
  lines=$(wc -l < "$f" 2>/dev/null || echo 0)

  refusals=$(grep -cE "$REFUSAL_RE" "$f" 2>/dev/null)
  [ -z "$refusals" ] && refusals=0
  halluc=$(grep -cE "$HALLUC_RE" "$f" 2>/dev/null)
  [ -z "$halluc" ] && halluc=0
  placeholder=$(grep -cE "$PLACEHOLDER_RE" "$f" 2>/dev/null)
  [ -z "$placeholder" ] && placeholder=0
  fences=$(grep -cE "$FENCE_RE" "$f" 2>/dev/null)
  [ -z "$fences" ] && fences=0

  # Determine if empty (< 50 bytes = effectively empty)
  if [ "$size" -lt 50 ]; then
    empty=true
  else
    empty=false
  fi

  # Determine if substantive (> 500 bytes, > 10 lines)
  if [ "$size" -gt 500 ] && [ "$lines" -gt 10 ]; then
    substantive=true
  else
    substantive=false
  fi

  # Verdict for this step
  if $empty; then
    verdict="EMPTY"
  elif [ "$refusals" -gt 0 ]; then
    verdict="REFUSAL"
  elif [ "$placeholder" -gt 2 ]; then
    verdict="PLACEHOLDER"
  elif ! $substantive; then
    verdict="THIN"
  else
    verdict="OK"
  fi

  if [ $FIRST -eq 1 ]; then
    FIRST=0
  else
    STEPS_JSON+=","
  fi

  STEPS_JSON+=$(cat <<EOF
{"file":"${fname}","size":${size},"lines":${lines},"empty":${empty},"substantive":${substantive},"refusals":${refusals},"hallucination":${halluc},"placeholder":${placeholder},"fences":${fences},"verdict":"${verdict}"}
EOF
)
done

STEPS_JSON+="]"

# Aggregate stats
echo "$STEPS_JSON" | python3 -c "
import json, sys
steps = json.loads(sys.stdin.read())
n = len(steps)
empty = sum(1 for s in steps if s['empty'])
refusal = sum(1 for s in steps if s['refusals'] > 0)
placeholder = sum(1 for s in steps if s['placeholder'] > 2)
thin = sum(1 for s in steps if not s['substantive'] and not s['empty'])
ok = sum(1 for s in steps if s['verdict'] == 'OK')
print(json.dumps({
  'total_steps': n,
  'empty': empty,
  'refusal': refusal,
  'placeholder': placeholder,
  'thin': thin,
  'ok': ok,
  'pass_step_quality': (empty == 0 and refusal == 0 and placeholder == 0),
  'steps': steps,
}, indent=2))
"
