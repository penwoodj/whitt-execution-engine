#!/bin/bash
# Machine-pass check: returns PASS if all machine gates passed.
# If PASS, the LLM evaluator can be skipped (saves ~60-90s).
# Usage: machine-pass-check.sh <sw_id> <eval_target_file> <input_file> <output_file>
set -uo pipefail

SW_ID="$1"
EVAL_TARGET="$2"
INPUT_FILE="$3"
OUTPUT_FILE="$4"

RESULT=FAIL
REASON=""

case "$SW_ID" in
  sw1)
    EXPANDED="$OUTPUT_FILE"
    if [ ! -f "$EXPANDED" ]; then
      REASON="expanded.md missing"
    else
      HIGH_PT_LEAVES=$(grep -oE '\([0-9]+pts\)' "$EXPANDED" 2>/dev/null | grep -oE '[0-9]+' | awk '$1>=5' | wc -l)
      PARENT_COUNT=$(grep -cE '^## T[0-9]' "$EXPANDED" 2>/dev/null || echo 0)
      if [ "$HIGH_PT_LEAVES" -eq 0 ] && [ "$PARENT_COUNT" -ge 8 ]; then
        RESULT=PASS
        REASON="0_high_pt_leaves_${PARENT_COUNT}_parents"
      else
        REASON="${HIGH_PT_LEAVES}_high_pt_leaves_${PARENT_COUNT}_parents"
      fi
    fi
    ;;
  sw2|sw3)
    if [ ! -f "$OUTPUT_FILE" ]; then
      REASON="output.md missing"
    else
      TOTAL_INPUT=$(grep -oE 'T[0-9]+' "$INPUT_FILE" 2>/dev/null | sort -u | wc -l)
      FOUND_OUTPUT=$(grep -oE 'T[0-9]+' "$OUTPUT_FILE" 2>/dev/null | sort -u | wc -l)
      PCT=$((TOTAL_INPUT > 0 ? FOUND_OUTPUT * 100 / TOTAL_INPUT : 0))
      if [ "$PCT" -ge 80 ]; then
        RESULT=PASS
        REASON="${PCT}_pct_coverage_${FOUND_OUTPUT}_of_${TOTAL_INPUT}"
      else
        REASON="${PCT}_pct_coverage_too_low"
      fi
    fi
    ;;
  sw4)
    STRUCTS="$OUTPUT_FILE"
    if [ ! -f "$STRUCTS" ]; then
      REASON="structs.md missing"
    else
      YAML_BLOCKS=$(grep -c '```yaml' "$STRUCTS" 2>/dev/null || echo 0)
      NON_YAML=$(grep -c -e '```rust' -e '```python' -e '```bash' -e '```json' "$STRUCTS" 2>/dev/null || echo 0)
      PLACEHOLDERS=$(grep -c '<placeholder>' "$STRUCTS" 2>/dev/null || echo 0)
      INPUT_TASKS=$(grep -cE '^(##|###) T[0-9]' "$INPUT_FILE" 2>/dev/null || echo 0)
      COVERAGE=$((INPUT_TASKS > 0 ? YAML_BLOCKS * 100 / INPUT_TASKS : 0))
      if [ "$NON_YAML" -eq 0 ] && [ "$PLACEHOLDERS" -eq 0 ] && [ "$COVERAGE" -ge 90 ]; then
        RESULT=PASS
        REASON="${YAML_BLOCKS}_yaml_blocks_${COVERAGE}_pct_coverage"
      else
        REASON="${YAML_BLOCKS}_blocks_${NON_YAML}_non_yaml_${PLACEHOLDERS}_placeholders_${COVERAGE}_pct"
      fi
    fi
    ;;
  sw5)
    WORKFLOW="$OUTPUT_FILE"
    if [ ! -f "$WORKFLOW" ]; then
      REASON="workflow.yml missing"
    else
      YAML_VALID=$(python3 -c "import yaml; yaml.safe_load(open('$WORKFLOW'))" 2>&1)
      if [ -z "$YAML_VALID" ]; then
        RESULT=PASS
        REASON="yaml_parses_ok"
      else
        REASON="yaml_parse_failed"
      fi
    fi
    ;;
  *)
    REASON="unknown_sw_$SW_ID"
    ;;
esac

# If PASS, write eval file with PASS verdict and exit 0 (signals to skip LLM eval)
if [ "$RESULT" = "PASS" ]; then
  printf 'MACHINE_GATE: PASS (%s)\n\nVERDICT: PASS' "$REASON" > "$EVAL_TARGET"
  printf '%s' PASS
  exit 0
fi

# Otherwise, exit 1 to signal "run LLM evaluator"
printf '%s' NEEDS_LLM_EVAL
exit 1
