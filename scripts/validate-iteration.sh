#!/usr/bin/env bash
# =============================================================================
# Iteration Validation Gate — Enforces evidence-based iteration claims
# =============================================================================
#
# Validates that a generator iteration has ALL required evidence before
# claiming improvement. Every iteration MUST pass this gate.
#
# Usage:
#   ./scripts/validate-iteration.sh [LOG_FILE] [OUTPUT_DIR] [YAML_FILE] [PREV_LOG_FILE]
#
#   LOG_FILE       - Path to current iteration run log
#   OUTPUT_DIR     - Path to workflow output directory
#   YAML_FILE      - Path to the workflow YAML file used
#   PREV_LOG_FILE  - (optional) Path to previous iteration log for comparison
#
# Exit codes:
#   0 - All 8 validation points pass
#   1 - One or more validation points fail (see output for details)
#
# The 8 required validation points:
#   1. Generated YAML exists and is valid
#   2. YAML validation result documented
#   3. Live workflow run result (log exists, has structured events)
#   4. Log inspection summary (can extract key metrics)
#   5. Bug/correctness analysis (from analyze-run.sh or manual)
#   6. Workflow quality analysis (from analyze-run.sh or manual)
#   7. Comparison against previous iteration
#   8. Clear next action determined
# =============================================================================

set -euo pipefail

LOG_FILE="${1:?Usage: $0 LOG_FILE OUTPUT_DIR YAML_FILE [PREV_LOG_FILE]}"
OUTPUT_DIR="${2:?Usage: $0 LOG_FILE OUTPUT_DIR YAML_FILE [PREV_LOG_FILE]}"
YAML_FILE="${3:?Usage: $0 LOG_FILE OUTPUT_DIR YAML_FILE [PREV_LOG_FILE]}"
PREV_LOG_FILE="${4:-}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_DIR"

# Sanitize grep count: ensure single integer
_gc() { local val="${1:-0}"; val=$(echo "$val" | head -1 | tr -d '[:space:]'); echo "${val:-0}"; }

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0
RESULTS=()

gate_check() {
    local name="$1"
    local status="$2"
    local detail="$3"

    if [ "$status" = "PASS" ]; then
        RESULTS+=("✅ ${name}: ${detail}")
        PASS_COUNT=$((PASS_COUNT + 1))
    elif [ "$status" = "FAIL" ]; then
        RESULTS+=("❌ ${name}: ${detail}")
        FAIL_COUNT=$((FAIL_COUNT + 1))
    elif [ "$status" = "WARN" ]; then
        RESULTS+=("⚠️  ${name}: ${detail}")
        WARN_COUNT=$((WARN_COUNT + 1))
    fi
}

echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}  Iteration Validation Gate${NC}"
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "  Log:      ${CYAN}${LOG_FILE}${NC}"
echo -e "  Output:   ${CYAN}${OUTPUT_DIR}${NC}"
echo -e "  YAML:     ${CYAN}${YAML_FILE}${NC}"
echo -e "  Previous: ${CYAN}${PREV_LOG_FILE:-none}${NC}"
echo ""

# =========================================================================
# GATE 1: Generated YAML exists and is parseable
# =========================================================================

if [ -f "$YAML_FILE" ]; then
    # Check it's valid YAML (basic check — no Python parse errors)
    if python3 -c "import yaml; yaml.safe_load(open('${YAML_FILE}'))" 2>/dev/null; then
        YAML_STEPS=$(python3 -c "
import yaml
data = yaml.safe_load(open('${YAML_FILE}'))
steps = data.get('agentic_workflow', {}).get('steps', {})
print(len(steps) if isinstance(steps, dict) else 0)
" 2>/dev/null || echo "?")
        gate_check "1. YAML exists" "PASS" "valid YAML, ${YAML_STEPS} steps"
    else
        gate_check "1. YAML exists" "FAIL" "file exists but invalid YAML"
    fi
else
    gate_check "1. YAML exists" "FAIL" "file not found: ${YAML_FILE}"
fi

# =========================================================================
# GATE 2: YAML validation result documented in log
# =========================================================================

if [ -f "$LOG_FILE" ]; then
    SCHEMA_VALID=$(grep -o 'schema_valid=true\|schema_valid=false' "$LOG_FILE" 2>/dev/null | head -1 || echo "")
    if [ -n "$SCHEMA_VALID" ]; then
        if echo "$SCHEMA_VALID" | grep -q 'true'; then
            gate_check "2. YAML validation" "PASS" "schema_valid=true found in log"
        else
            gate_check "2. YAML validation" "FAIL" "schema_valid=false — YAML failed schema validation"
        fi
    else
        # Check for any YAML-related validation in log
        WORKFLOW_LOADED=$(_gc "$(grep -c '\[workflow:start\]' "$LOG_FILE" 2>/dev/null || echo 0)")
        if [ "$WORKFLOW_LOADED" -gt 0 ]; then
            gate_check "2. YAML validation" "WARN" "workflow loaded but no explicit schema_valid log (may be pre-validation code)"
        else
            gate_check "2. YAML validation" "FAIL" "no schema validation result in log"
        fi
    fi
else
    gate_check "2. YAML validation" "FAIL" "log file not found"
fi

# =========================================================================
# GATE 3: Live workflow run result
# =========================================================================

if [ -f "$LOG_FILE" ]; then
    HAS_START=$(_gc "$(grep -c '\[workflow:start\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    HAS_END=$(_gc "$(grep -c '\[workflow:end\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    HAS_JUDGMENT=$(_gc "$(grep -c '\[workflow:judgment\]' "$LOG_FILE" 2>/dev/null || echo 0)")

    if [ "$HAS_START" -gt 0 ] && [ "$HAS_END" -gt 0 ]; then
        # Extract judgment
        JUDGMENT=$(grep '\[workflow:judgment\]' "$LOG_FILE" | tail -1 | grep -oP 'correctness=\K[^ ]+' || echo "unknown")
        QUALITY_J=$(grep '\[workflow:judgment\]' "$LOG_FILE" | tail -1 | grep -oP 'quality=\K\w+' || echo "unknown")

        if [ "$JUDGMENT" = "PASS" ]; then
            gate_check "3. Live run" "PASS" "workflow completed, correctness=${JUDGMENT}, quality=${QUALITY_J}"
        else
            gate_check "3. Live run" "FAIL" "workflow completed but correctness=${JUDGMENT}, quality=${QUALITY_J}"
        fi
    elif [ "$HAS_START" -gt 0 ]; then
        gate_check "3. Live run" "FAIL" "workflow started but no [workflow:end] event — crash or incomplete?"
    else
        # Check for legacy benchmark format
        LEGACY_OK=$(_gc "$(grep -c 'saved output to\|Tokens/Second' "$LOG_FILE" 2>/dev/null || echo 0)")
        if [ "$LEGACY_OK" -gt 0 ]; then
            gate_check "3. Live run" "WARN" "legacy format — completed but no structured events"
        else
            gate_check "3. Live run" "FAIL" "no workflow start/end events in log"
        fi
    fi
else
    gate_check "3. Live run" "FAIL" "log file not found: ${LOG_FILE}"
fi

# =========================================================================
# GATE 4: Log inspection summary (key metrics extractable)
# =========================================================================

if [ -f "$LOG_FILE" ]; then
    STEPS_OK=$(_gc "$(grep -c '\[step:ok\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    STEPS_FAIL=$(_gc "$(grep -c '\[step:fail\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    QUALITY_OK=$(_gc "$(grep -c '\[step:quality\].*json_parsable=true' "$LOG_FILE" 2>/dev/null || echo 0)")
    QUALITY_BAD=$(_gc "$(grep -c '\[step:quality\].*json_parsable=false' "$LOG_FILE" 2>/dev/null || echo 0)")

    # Need at least step counts to be meaningful
    TOTAL=$((STEPS_OK + STEPS_FAIL))
    if [ "$TOTAL" -gt 0 ]; then
        gate_check "4. Log inspection" "PASS" "steps_ok=${STEPS_OK} steps_fail=${STEPS_FAIL} json_ok=${QUALITY_OK} json_bad=${QUALITY_BAD}"
    else
        # Try legacy counts
        LEGACY_STEPS=$(_gc "$(grep -c 'Tokens/Second' "$LOG_FILE" 2>/dev/null || echo 0)")
        if [ "$LEGACY_STEPS" -gt 0 ]; then
            gate_check "4. Log inspection" "WARN" "legacy format, ${LEGACY_STEPS} inferences found"
        else
            gate_check "4. Log inspection" "FAIL" "no extractable metrics from log"
        fi
    fi
else
    gate_check "4. Log inspection" "FAIL" "log file not found"
fi

# =========================================================================
# GATE 5: Bug/correctness analysis
# =========================================================================

# Check if analyze-run.sh report exists for this iteration
LOG_BASENAME=$(basename "$LOG_FILE" | sed 's/\.[^.]*$//')
LOG_DIR=$(dirname "$LOG_FILE")
ANALYSIS_REPORT="${LOG_DIR}/run-analysis.md"
if [ -f "$ANALYSIS_REPORT" ]; then
    CORRECTNESS=$(grep 'Bug / Correctness' "$ANALYSIS_REPORT" | grep -oP ':\s*\K\w+' || echo "unknown")
    if [ -n "$CORRECTNESS" ]; then
        if [ "$CORRECTNESS" = "PASS" ]; then
            gate_check "5. Bug/correctness" "PASS" "analysis report exists, result: ${CORRECTNESS}"
        else
            gate_check "5. Bug/correctness" "FAIL" "analysis report exists, result: ${CORRECTNESS}"
        fi
    else
        gate_check "5. Bug/correctness" "WARN" "analysis report exists but no clear result"
    fi
else
    # Check if we can derive correctness from log directly
    if [ -f "$LOG_FILE" ]; then
        JUDGMENT=$(grep '\[workflow:judgment\]' "$LOG_FILE" | tail -1 | grep -oP 'correctness=\K[^ ]+' || echo "")
        if [ -n "$JUDGMENT" ]; then
            if [ "$JUDGMENT" = "PASS" ]; then
                gate_check "5. Bug/correctness" "PASS" "derived from log: correctness=${JUDGMENT}"
            else
                gate_check "5. Bug/correctness" "FAIL" "derived from log: correctness=${JUDGMENT}"
            fi
        else
            gate_check "5. Bug/correctness" "FAIL" "no analysis report and no judgment in log — run ./scripts/analyze-run.sh first"
        fi
    else
        gate_check "5. Bug/correctness" "FAIL" "no analysis report and no log file"
    fi
fi

# =========================================================================
# GATE 6: Workflow quality analysis
# =========================================================================

# Check output files — exclude benchmark_report, benchmark_results, and unresolved template names
OUTPUT_JSONS=$(ls -1 "$OUTPUT_DIR"/*.json 2>/dev/null | grep -v 'benchmark_report\|benchmark_results\|{{' || true)
OUTPUT_COUNT=0
VALID_COUNT=0
INVALID_COUNT=0

for f in $OUTPUT_JSONS; do
    OUTPUT_COUNT=$((OUTPUT_COUNT + 1))
    if python3 -c "import json; json.load(open('$f'))" 2>/dev/null; then
        VALID_COUNT=$((VALID_COUNT + 1))
    else
        INVALID_COUNT=$((INVALID_COUNT + 1))
    fi
done

if [ "$OUTPUT_COUNT" -gt 0 ]; then
    if [ "$VALID_COUNT" -eq "$OUTPUT_COUNT" ]; then
        gate_check "6. Quality analysis" "PASS" "${VALID_COUNT}/${OUTPUT_COUNT} valid JSON outputs"
    elif [ "$VALID_COUNT" -gt 0 ]; then
        gate_check "6. Quality analysis" "FAIL" "only ${VALID_COUNT}/${OUTPUT_COUNT} valid JSON — ${INVALID_COUNT} invalid"
    else
        gate_check "6. Quality analysis" "FAIL" "0/${OUTPUT_COUNT} valid JSON outputs"
    fi
else
    gate_check "6. Quality analysis" "FAIL" "no output JSON files found in ${OUTPUT_DIR}"
fi

# =========================================================================
# GATE 7: Comparison against previous iteration
# =========================================================================

if [ -n "$PREV_LOG_FILE" ] && [ -f "$PREV_LOG_FILE" ]; then
    # Extract previous iteration metrics
    PREV_OK=$(_gc "$(grep -c '\[step:ok\]' "$PREV_LOG_FILE" 2>/dev/null || echo 0)")
    PREV_FAIL=$(_gc "$(grep -c '\[step:fail\]' "$PREV_LOG_FILE" 2>/dev/null || echo 0)")
    PREV_JSON_OK=$(_gc "$(grep -c '\[step:quality\].*json_parsable=true' "$PREV_LOG_FILE" 2>/dev/null || echo 0)")
    PREV_JSON_BAD=$(_gc "$(grep -c '\[step:quality\].*json_parsable=false' "$PREV_LOG_FILE" 2>/dev/null || echo 0)")

    CURR_OK=$(_gc "$(grep -c '\[step:ok\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    CURR_FAIL=$(_gc "$(grep -c '\[step:fail\]' "$LOG_FILE" 2>/dev/null || echo 0)")
    CURR_JSON_OK=$(_gc "$(grep -c '\[step:quality\].*json_parsable=true' "$LOG_FILE" 2>/dev/null || echo 0)")
    CURR_JSON_BAD=$(_gc "$(grep -c '\[step:quality\].*json_parsable=false' "$LOG_FILE" 2>/dev/null || echo 0)")

    # Determine if improvement
    IMPROVED=""
    if [ "$CURR_FAIL" -lt "$PREV_FAIL" ]; then
        IMPROVED="${IMPROVED} failures↓(${PREV_FAIL}→${CURR_FAIL})"
    fi
    if [ "$CURR_JSON_OK" -gt "$PREV_JSON_OK" ]; then
        IMPROVED="${IMPROVED} json_ok↑(${PREV_JSON_OK}→${CURR_JSON_OK})"
    fi
    if [ "$CURR_JSON_BAD" -lt "$PREV_JSON_BAD" ]; then
        IMPROVED="${IMPROVED} json_bad↓(${PREV_JSON_BAD}→${CURR_JSON_BAD})"
    fi

    if [ -n "$IMPROVED" ]; then
        gate_check "7. Comparison" "PASS" "improved:${IMPROVED}"
    elif [ "$CURR_FAIL" -eq "$PREV_FAIL" ] && [ "$CURR_JSON_OK" -eq "$PREV_JSON_OK" ]; then
        gate_check "7. Comparison" "PASS" "no regression (same metrics as previous)"
    else
        gate_check "7. Comparison" "FAIL" "REGRESSION: failures↑(${PREV_FAIL}→${CURR_FAIL}) or json_ok↓(${PREV_JSON_OK}→${CURR_JSON_OK})"
    fi
else
    if [ -z "$PREV_LOG_FILE" ]; then
        gate_check "7. Comparison" "WARN" "no previous iteration log provided (first iteration)"
    else
        gate_check "7. Comparison" "FAIL" "previous log file not found: ${PREV_LOG_FILE}"
    fi
fi

# =========================================================================
# GATE 8: Clear next action determined
# =========================================================================

# Derive next action from the results
NEXT_ACTION=""

if [ "$FAIL_COUNT" -gt 0 ]; then
    # Check what failed to determine action
    RESULTS_STR=$(printf '%s\n' "${RESULTS[@]}")

    if echo "$RESULTS_STR" | grep -q "correctness=FAIL\|correctness=FAIL"; then
        NEXT_ACTION="Fix code bug — correctness failures detected"
    elif echo "$RESULTS_STR" | grep -q "invalid JSON"; then
        NEXT_ACTION="Improve hooks — JSON cleaning or save_to issues"
    elif echo "$RESULTS_STR" | grep -q "YAML.*invalid\|schema_valid=false"; then
        NEXT_ACTION="Improve generated YAML — schema validation failure"
    elif echo "$RESULTS_STR" | grep -q "no.*events\|not found"; then
        NEXT_ACTION="Improve logging — missing structured events or files"
    else
        NEXT_ACTION="Fix code bug — unspecified gate failure"
    fi
elif [ "$WARN_COUNT" -gt 0 ]; then
    NEXT_ACTION="Improve logging — gate warnings suggest missing diagnostics"
else
    NEXT_ACTION="Stop because current result is acceptable"
fi

gate_check "8. Next action" "PASS" "${NEXT_ACTION}"

# =========================================================================
# FINAL SUMMARY
# =========================================================================

echo -e "${BOLD}── Validation Results ──${NC}"
echo ""
for r in "${RESULTS[@]}"; do
    echo -e "  ${r}"
done

echo ""
echo -e "${BOLD}── Gate Summary ──${NC}"
echo -e "  Passed:  ${GREEN}${PASS_COUNT}/8${NC}"
echo -e "  Failed:  ${RED}${FAIL_COUNT}/8${NC}"
echo -e "  Warnings: ${YELLOW}${WARN_COUNT}/8${NC}"

echo ""
echo -e "${BOLD}── Next Action ──${NC}"
echo -e "  ${CYAN}${NEXT_ACTION}${NC}"

echo ""

if [ "$FAIL_COUNT" -gt 0 ]; then
    echo -e "${RED}${BOLD}✗ GATE FAILED${NC} — ${FAIL_COUNT} validation point(s) failed."
    echo -e "${RED}Cannot claim improvement without passing all gate checks.${NC}"
    echo ""
    echo -e "${YELLOW}Evidence required but missing:${NC}"
    for r in "${RESULTS[@]}"; do
        if echo "$r" | grep -q "^❌"; then
            echo -e "  ${r}"
        fi
    done
    exit 1
elif [ "$WARN_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}${BOLD}⚠ GATE PASSED WITH WARNINGS${NC} — ${WARN_COUNT} warning(s)."
    echo -e "${YELLOW}Iteration may proceed but warnings should be addressed.${NC}"
    exit 0
else
    echo -e "${GREEN}${BOLD}✓ GATE PASSED${NC} — all 8 validation points satisfied."
    echo -e "${GREEN}Improvement claim is supported by evidence.${NC}"
    exit 0
fi
