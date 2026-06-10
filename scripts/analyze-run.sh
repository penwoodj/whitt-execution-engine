#!/usr/bin/env bash
# =============================================================================
# Workflow Run Analyzer — Bug/Correctness + Quality Two-Dimensional Analysis
# =============================================================================
#
# Analyzes benchmark run logs on two dimensions:
#   1. BUG/CORRECTNESS: Did the workflow execute correctly?
#   2. QUALITY: Was the generated workflow good/useful/clear?
#
# Supports two log formats:
#   - NEW: Structured events ([workflow:start], [step:ok], etc.)
#   - OLD: Legacy [benchmark] prefix logs
#
# Usage:
#   ./scripts/analyze-run.sh [LOG_FILE] [OUTPUT_DIR]
#
#   LOG_FILE   - Path to benchmark run log (stdout capture or file)
#   OUTPUT_DIR - Path to workflow output directory (default: docs/benchmarks/outputs/output)
#
# Examples:
#   ./scripts/analyze-run.sh docs/benchmarks/outputs/logs/benchmark-50-run.log
#   ./scripts/analyze-run.sh nohup.out docs/benchmarks/outputs/output
#   cargo run --release --bin whitt -- benchmark --workflow X.yml 2>&1 | tee /tmp/run.log
#   ./scripts/analyze-run.sh /tmp/run.log
#
# Output: Analysis report to stdout + detailed report saved to OUTPUT_DIR/../logs/run-analysis.md
# =============================================================================

set -euo pipefail

LOG_FILE="${1:?Usage: $0 LOG_FILE [OUTPUT_DIR]}"
OUTPUT_DIR="${2:-docs/benchmarks/outputs/output}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_DIR"

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# --- Temp files ---
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Sanitize grep counts — ensure numeric, default 0
_num() { local val="${1:-0}"; val=$(echo "$val" | head -1 | tr -d '[:space:]'); echo "${val:-0}"; }

# =============================================================================
# PHASE 1: Parse logs into structured data
# =============================================================================

echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}  Workflow Run Analysis${NC}"
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "Log: ${CYAN}${LOG_FILE}${NC}"
echo -e "Output dir: ${CYAN}${OUTPUT_DIR}${NC}"
echo ""

if [ ! -f "$LOG_FILE" ]; then
    echo -e "${RED}ERROR: Log file not found: ${LOG_FILE}${NC}"
    exit 1
fi

LOG_SIZE=$(wc -l < "$LOG_FILE")
echo -e "Log size: ${LOG_SIZE} lines"
echo ""

# Detect log format
HAS_STRUCTURED=$(grep -c '\[workflow:\|\[step:\|\[generator:\|\[hook:' "$LOG_FILE" 2>/dev/null || true)
HAS_STRUCTURED=${HAS_STRUCTURED:-0}
HAS_LEGACY=$(grep -c '\[benchmark\]' "$LOG_FILE" 2>/dev/null || true)
HAS_LEGACY=${HAS_LEGACY:-0}

if [ "$HAS_STRUCTURED" -gt 0 ]; then
    echo -e "Log format: ${GREEN}Structured (new)${NC} ($HAS_STRUCTURED structured events)"
    LOG_FORMAT="structured"
elif [ "$HAS_LEGACY" -gt 0 ]; then
    echo -e "Log format: ${YELLOW}Legacy (old)${NC} ($HAS_LEGACY [benchmark] entries)"
    LOG_FORMAT="legacy"
else
    echo -e "${RED}ERROR: No recognized log format found${NC}"
    exit 1
fi

echo ""

# =============================================================================
# PHASE 2: Extract run metadata
# =============================================================================

echo -e "${BOLD}── Run Metadata ──${NC}"

# Extract workflow ID
WORKFLOW_ID="unknown"
if [ "$LOG_FORMAT" = "structured" ]; then
    WORKFLOW_ID=$(grep '\[workflow:start\]' "$LOG_FILE" | head -1 | grep -oP 'workflow_id=\K[^ ]+' || echo "unknown")
else
    WORKFLOW_ID=$(grep 'benchmark command' "$LOG_FILE" | head -1 | grep -oP 'benchmark command.*' || echo "legacy-run")
fi
echo -e "  Workflow ID: ${CYAN}${WORKFLOW_ID}${NC}"

# Extract run timestamp
RUN_TIMESTAMP=$(head -1 "$LOG_FILE" | grep -oP '\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}' || echo "unknown")
echo -e "  Run timestamp: ${RUN_TIMESTAMP}"

# Extract model count
MODEL_COUNT=$(grep -oP '\[\d+/\d+\]' "$LOG_FILE" | tail -1 | grep -oP '/\K\d+' || echo "?")
echo -e "  Models: ${MODEL_COUNT}"

# Extract prompt preview
PROMPT_PREVIEW=$(grep 'prompt=' "$LOG_FILE" | head -1 | grep -oP 'prompt="[^"]*"' | head -c 200 || echo "")
if [ -n "$PROMPT_PREVIEW" ]; then
    echo -e "  Prompt: ${PROMPT_PREVIEW}"
fi

echo ""

# =============================================================================
# PHASE 3: Bug / Correctness Analysis
# =============================================================================

echo -e "${BOLD}── Bug / Correctness Analysis ──${NC}"

# Count steps succeeded vs failed
if [ "$LOG_FORMAT" = "structured" ]; then
    STEPS_OK=$(grep -c '\[step:ok\]' "$LOG_FILE" 2>/dev/null || true)
    STEPS_FAIL=$(grep -c '\[step:fail\]' "$LOG_FILE" 2>/dev/null || true)
    STEPS_QUALITY=$(grep -c '\[step:quality\]' "$LOG_FILE" 2>/dev/null || true)
    STEPS_OK=${STEPS_OK:-0}; STEPS_FAIL=${STEPS_FAIL:-0}; STEPS_QUALITY=${STEPS_QUALITY:-0}
else
    STEPS_OK=$(grep -c 'saved output to' "$LOG_FILE" 2>/dev/null || true)
    STEPS_FAIL=$(grep -cE 'ERROR|step.*fail|model.*fail|crash|panic' "$LOG_FILE" 2>/dev/null || true)
    STEPS_OK=${STEPS_OK:-0}; STEPS_FAIL=${STEPS_FAIL:-0}; STEPS_QUALITY=0
fi

TOTAL_STEPS=$((STEPS_OK + STEPS_FAIL))
echo -e "  Steps total: ${TOTAL_STEPS}"
echo -e "  Steps succeeded: ${GREEN}${STEPS_OK}${NC}"
echo -e "  Steps failed: ${RED}${STEPS_FAIL}${NC}"

# Error classification
echo ""
echo -e "  ${BOLD}Error Classification:${NC}"

GENERATOR_ERRORS=0; EXECUTOR_ERRORS=0; HOOK_ERRORS=0; BUG_ERRORS=0; LEGACY_ERRORS=0
GENERATOR_ERRORS=$(grep -c '\[error\].*category=generator' "$LOG_FILE" 2>/dev/null || true)
EXECUTOR_ERRORS=$(grep -c '\[error\].*category=executor' "$LOG_FILE" 2>/dev/null || true)
HOOK_ERRORS=$(grep -c '\[error\].*category=hook' "$LOG_FILE" 2>/dev/null || true)
BUG_ERRORS=$(grep -c '\[error\].*category=bug' "$LOG_FILE" 2>/dev/null || true)
LEGACY_ERRORS=$(grep -cE 'panic|unwrap.*failed|thread.*panicked' "$LOG_FILE" 2>/dev/null || true)
GENERATOR_ERRORS=${GENERATOR_ERRORS:-0}; EXECUTOR_ERRORS=${EXECUTOR_ERRORS:-0}
HOOK_ERRORS=${HOOK_ERRORS:-0}; BUG_ERRORS=${BUG_ERRORS:-0}; LEGACY_ERRORS=${LEGACY_ERRORS:-0}

# For legacy logs, classify differently
if [ "$LOG_FORMAT" = "legacy" ]; then
    YAML_ERRORS=$(grep -ciE 'invalid yaml|parse error|schema.*invalid' "$LOG_FILE" 2>/dev/null || true)
    MODEL_ERRORS=$(grep -ciE 'model.*not found|model.*failed to load|model.*error' "$LOG_FILE" 2>/dev/null || true)
    HOOK_ERR_LEGACY=$(grep -ciE 'hook.*error|hook.*fail' "$LOG_FILE" 2>/dev/null || true)
    SERVER_ERRORS=$(grep -ciE 'server.*error|connection.*refused|docker.*error' "$LOG_FILE" 2>/dev/null || true)
    YAML_ERRORS=${YAML_ERRORS:-0}; MODEL_ERRORS=${MODEL_ERRORS:-0}
    HOOK_ERR_LEGACY=${HOOK_ERR_LEGACY:-0}; SERVER_ERRORS=${SERVER_ERRORS:-0}
    GENERATOR_ERRORS=$YAML_ERRORS
    EXECUTOR_ERRORS=$((MODEL_ERRORS + SERVER_ERRORS))
fi

echo -e "    Generator errors: ${GENERATOR_ERRORS}"
echo -e "    Executor errors:  ${EXECUTOR_ERRORS}"
echo -e "    Hook errors:      ${HOOK_ERRORS}"
echo -e "    Bug/code errors:  ${BUG_ERRORS}"
echo -e "    Legacy panics:    ${LEGACY_ERRORS}"

# False success detection
FALSE_SUCCESS=0
FALSE_SUCCESS_DETAILS=""
if [ "$LOG_FORMAT" = "structured" ]; then
    FALSE_SUCCESS_LINES=$(grep '\[step:quality\].*json_parsable=false' "$LOG_FILE" 2>/dev/null || true)
    if [ -n "$FALSE_SUCCESS_LINES" ]; then
        FALSE_SUCCESS=$(echo "$FALSE_SUCCESS_LINES" | wc -l | tr -d ' ')
    fi
fi

# Check output files for issues
echo ""
echo -e "  ${BOLD}Output File Analysis:${NC}"
OUTPUT_FILES=$(ls -1 "$OUTPUT_DIR"/*.json 2>/dev/null || true)
OUTPUT_COUNT=0
if [ -n "$OUTPUT_FILES" ]; then
    OUTPUT_COUNT=$(echo "$OUTPUT_FILES" | wc -l | tr -d ' ')
fi
EMPTY_FILES=0
INVALID_JSON=0
VALID_JSON=0

for f in $OUTPUT_FILES; do
    if [ ! -s "$f" ]; then
        EMPTY_FILES=$((EMPTY_FILES + 1))
    elif ! python3 -c "import json; json.load(open('$f'))" 2>/dev/null; then
        INVALID_JSON=$((INVALID_JSON + 1))
    else
        VALID_JSON=$((VALID_JSON + 1))
    fi
done

echo -e "    Output files: ${OUTPUT_COUNT}"
echo -e "    Valid JSON:   ${GREEN}${VALID_JSON}${NC}"
echo -e "    Invalid JSON: ${RED}${INVALID_JSON}${NC}"
echo -e "    Empty files:  ${RED}${EMPTY_FILES}${NC}"

# Overall correctness verdict
echo ""
if [ "$STEPS_FAIL" -eq 0 ] && [ "$INVALID_JSON" -eq 0 ] && [ "$LEGACY_ERRORS" -eq 0 ]; then
    if [ "$FALSE_SUCCESS" -gt 0 ]; then
        echo -e "  ${YELLOW}⚠ CORRECTNESS: PASS WITH ISSUES${NC} (false success: $FALSE_SUCCESS steps reported ok but output not JSON)"
        BUG_RESULT="PASS_WITH_ISSUES"
    else
        echo -e "  ${GREEN}✓ CORRECTNESS: PASS${NC} (no errors, all steps ok, all outputs valid)"
        BUG_RESULT="PASS"
    fi
elif [ "$STEPS_FAIL" -gt 0 ] && [ "$STEPS_OK" -gt "$STEPS_FAIL" ]; then
    echo -e "  ${YELLOW}⚠ CORRECTNESS: PARTIAL PASS${NC} (${STEPS_FAIL}/${TOTAL_STEPS} steps failed)"
    BUG_RESULT="PARTIAL_PASS"
else
    echo -e "  ${RED}✗ CORRECTNESS: FAIL${NC} (${STEPS_FAIL} failures, ${INVALID_JSON} invalid outputs)"
    BUG_RESULT="FAIL"
fi

echo ""

# =============================================================================
# PHASE 4: Quality Analysis
# =============================================================================

echo -e "${BOLD}── Workflow Quality Analysis ──${NC}"

# Analyze output sizes and content quality
TOTAL_BYTES=0
TOTAL_LINES=0
MIN_SIZE=999999
MAX_SIZE=0
MIN_FILE=""
MAX_FILE=""
GOOD_OUTPUTS=0
POOR_OUTPUTS=0

for f in $OUTPUT_FILES; do
    if [ -s "$f" ]; then
        SIZE=$(wc -c < "$f")
        LINES=$(wc -l < "$f")
        TOTAL_BYTES=$((TOTAL_BYTES + SIZE))
        TOTAL_LINES=$((TOTAL_LINES + LINES))

        BASENAME=$(basename "$f")

        if [ "$SIZE" -lt "$MIN_SIZE" ]; then
            MIN_SIZE=$SIZE
            MIN_FILE=$BASENAME
        fi
        if [ "$SIZE" -gt "$MAX_SIZE" ]; then
            MAX_SIZE=$SIZE
            MAX_FILE=$BASENAME
        fi

        # Quality heuristics
        # Good: >500 bytes, valid JSON, has structure (keys)
        if [ "$SIZE" -gt 500 ]; then
            # Check if it has JSON structure (keys)
            if python3 -c "
import json, sys
data = json.load(open('$f'))
if isinstance(data, dict) and len(data.keys()) >= 5:
    sys.exit(0)
sys.exit(1)
" 2>/dev/null; then
                GOOD_OUTPUTS=$((GOOD_OUTPUTS + 1))
            else
                POOR_OUTPUTS=$((POOR_OUTPUTS + 1))
            fi
        else
            POOR_OUTPUTS=$((POOR_OUTPUTS + 1))
        fi
    fi
done

if [ "$OUTPUT_COUNT" -gt 0 ]; then
    AVG_BYTES=$((TOTAL_BYTES / OUTPUT_COUNT))
    AVG_LINES=$((TOTAL_LINES / OUTPUT_COUNT))
    echo -e "  Output stats:"
    echo -e "    Total: ${TOTAL_BYTES} bytes, ${TOTAL_LINES} lines across ${OUTPUT_COUNT} files"
    echo -e "    Average: ${AVG_BYTES} bytes, ${AVG_LINES} lines"
    echo -e "    Smallest: ${MIN_FILE} (${MIN_SIZE} bytes)"
    echo -e "    Largest:  ${MAX_FILE} (${MAX_SIZE} bytes)"
    echo ""
    echo -e "  Quality breakdown:"
    echo -e "    Good outputs (structured, >500B, 5+ keys): ${GREEN}${GOOD_OUTPUTS}${NC}"
    echo -e "    Poor outputs (small/unstructured):         ${YELLOW}${POOR_OUTPUTS}${NC}"
fi

# Analyze workflow structure from logs
echo ""
echo -e "  ${BOLD}Workflow Structure:${NC}"

if [ "$LOG_FORMAT" = "structured" ]; then
    STEP_NAMES=$(grep '\[generator:steps\]' "$LOG_FILE" | grep -oP 'step_names=\K.*' || echo "unknown")
    echo -e "    Steps: ${STEP_NAMES}"
    
    VALIDATION=$(grep '\[generator:validation\]' "$LOG_FILE" | head -1 || echo "")
    if echo "$VALIDATION" | grep -q 'schema_valid=true'; then
        echo -e "    Schema validation: ${GREEN}PASS${NC}"
    elif echo "$VALIDATION" | grep -q 'schema_valid=false'; then
        echo -e "    Schema validation: ${RED}FAIL${NC}"
    else
        echo -e "    Schema validation: ${YELLOW}Not found in logs${NC}"
    fi
    
    PROMPTS_INCLUSION=$(grep '\[generator:prompts\]' "$LOG_FILE" | head -1 || echo "")
    if echo "$PROMPTS_INCLUSION" | grep -q 'ignored'; then
        echo -e "    Prompts: ${YELLOW}Ignored (known gap G1)${NC}"
    fi
else
    WORKFLOW_FILE=$(grep 'workflow' "$LOG_FILE" | grep -oP '[^ ]+\.yml' | head -1 || echo "unknown")
    echo -e "    Workflow file: ${WORKFLOW_FILE}"
    echo -e "    (Legacy format — limited structure analysis available)"
fi

# Hook usage analysis
echo ""
echo -e "  ${BOLD}Hook Usage:${NC}"

if [ "$LOG_FORMAT" = "structured" ]; then
    HOOK_TRIGGERS=$(grep -c '\[hook:trigger\]' "$LOG_FILE" 2>/dev/null || true)
    HOOK_ACTIONS=$(grep -c '\[hook:action\]' "$LOG_FILE" 2>/dev/null || true)
    HOOK_NON_CONTINUE=$(grep -c '\[hook:non_continue\]' "$LOG_FILE" 2>/dev/null || true)
    HOOK_TRIGGERS=${HOOK_TRIGGERS:-0}; HOOK_ACTIONS=${HOOK_ACTIONS:-0}; HOOK_NON_CONTINUE=${HOOK_NON_CONTINUE:-0}
else
    HOOK_LOG_ENTRIES=$(grep -c 'timing=' "$LOG_FILE" 2>/dev/null || true)
    TRIGGER_TYPES=$(grep 'timing=' "$LOG_FILE" 2>/dev/null | grep -oP 'timing=\K[^ ]+' | sort -u | tr '\n' ', ' || echo "none")
    HOOK_LOG_ENTRIES=${HOOK_LOG_ENTRIES:-0}
fi

# Overall quality verdict
echo ""
if [ "$OUTPUT_COUNT" -eq 0 ]; then
    echo -e "  ${RED}✗ QUALITY: NO OUTPUTS${NC}"
    QUALITY_RESULT="NO_OUTPUTS"
elif [ "$GOOD_OUTPUTS" -eq 0 ] && [ "$POOR_OUTPUTS" -gt 0 ]; then
    echo -e "  ${RED}✗ QUALITY: FAIL${NC} (all outputs poor quality)"
    QUALITY_RESULT="FAIL"
elif [ "$GOOD_OUTPUTS" -gt 0 ] && [ "$POOR_OUTPUTS" -eq 0 ]; then
    echo -e "  ${GREEN}✓ QUALITY: PASS${NC} (all ${GOOD_OUTPUTS} outputs structured and substantial)"
    QUALITY_RESULT="PASS"
elif [ "$GOOD_OUTPUTS" -ge "$POOR_OUTPUTS" ]; then
    echo -e "  ${GREEN}✓ QUALITY: PASS${NC} (${GOOD_OUTPUTS} good, ${POOR_OUTPUTS} poor — majority good)"
    QUALITY_RESULT="PASS"
else
    echo -e "  ${YELLOW}⚠ QUALITY: PARTIAL${NC} (${GOOD_OUTPUTS} good, ${POOR_OUTPUTS} poor — majority poor)"
    QUALITY_RESULT="PARTIAL"
fi

echo ""

# =============================================================================
# PHASE 5: Specific Issues & Recommendations
# =============================================================================

echo -e "${BOLD}── Issues Found ──${NC}"

ISSUES=()
CODE_FIXES=()
WORKFLOW_FIXES=()

# Check for common issues
if [ "$LEGACY_ERRORS" -gt 0 ]; then
    ISSUES+=("LEGACY PANICS: $LEGACY_ERRORS thread panics detected — code bugs")
    CODE_FIXES+=("Fix panic sources in runner.rs (check unwrap() calls)")
fi

if [ "$INVALID_JSON" -gt 0 ]; then
    ISSUES+=("INVALID JSON: $INVALID_JSON/$OUTPUT_COUNT output files not valid JSON")
    CODE_FIXES+=("Consider adding clean_json_output() or adjusting prompt for these models")
fi

if [ "$EMPTY_FILES" -gt 0 ]; then
    ISSUES+=("EMPTY OUTPUTS: $EMPTY_FILES files are empty — models produced nothing")
    CODE_FIXES+=("Check if model loaded correctly, prompt appropriate for model type")
fi

if [ "$FALSE_SUCCESS" -gt 0 ]; then
    ISSUES+=("FALSE SUCCESS: $FALSE_SUCCESS steps reported ok but output not valid JSON")
    CODE_FIXES+=("Add post-inference JSON validation in execute_workflow_step()")
fi

if [ "$GENERATOR_ERRORS" -gt 0 ]; then
    ISSUES+=("GENERATOR ERRORS: $GENERATOR_ERRORS — YAML generation or validation failures")
    CODE_FIXES+=("Fix yaml_generator.rs: check string formatting, special chars in model names")
fi

if [ "$EXECUTOR_ERRORS" -gt 0 ]; then
    ISSUES+=("EXECUTOR ERRORS: $EXECUTOR_ERRORS — model load/inference failures")
    CODE_FIXES+=("Check Docker health, model compatibility, context window limits")
fi

if [ "$HOOK_ERRORS" -gt 0 ]; then
    ISSUES+=("HOOK ERRORS: $HOOK_ERRORS — hook action failures")
    CODE_FIXES+=("Check hook YAML config, file paths in save_to/log/append_to actions")
fi

if [ "$POOR_OUTPUTS" -gt "$GOOD_OUTPUTS" ] && [ "$OUTPUT_COUNT" -gt 0 ]; then
    ISSUES+=("POOR QUALITY MAJORITY: $POOR_OUTPUTS poor vs $GOOD_OUTPUTS good outputs")
    WORKFLOW_FIXES+=("Improve prompt specificity — add concrete JSON example, explicit format rules")
    WORKFLOW_FIXES+=("Consider filtering to models >=3B parameters for structured output tasks")
fi

if [ "$LOG_FORMAT" = "legacy" ]; then
    ISSUES+=("LEGACY LOG FORMAT: no structured events — run with updated code for better analysis")
    CODE_FIXES+=("Re-run with current code to get [workflow:start/end/judgment] events")
fi

# Check for truncation indicators
TRUNCATED=$(grep -ciE 'finish_reason.*length|truncated|max_tokens.*reached' "$LOG_FILE" 2>/dev/null || true)
TRUNCATED=${TRUNCATED:-0}
if [ "$TRUNCATED" -gt 0 ]; then
    ISSUES+=("OUTPUT TRUNCATION: $TRUNCATED outputs hit token limit (finish_reason=length)")
    CODE_FIXES+=("Increase context.size or reduce server.max_slots in config.yml")
    WORKFLOW_FIXES+=("Add max_tokens: 10000 to workflow step config")
fi

if [ ${#ISSUES[@]} -eq 0 ]; then
    echo -e "  ${GREEN}No issues found.${NC}"
else
    for i in "${!ISSUES[@]}"; do
        echo -e "  ${RED}$((i+1)). ${ISSUES[$i]}${NC}"
    done
fi

echo ""

echo -e "${BOLD}── Recommended Code Fixes ──${NC}"
if [ ${#CODE_FIXES[@]} -eq 0 ]; then
    echo -e "  ${GREEN}No code fixes needed.${NC}"
else
    for i in "${!CODE_FIXES[@]}"; do
        echo -e "  ${YELLOW}$((i+1)). ${CODE_FIXES[$i]}${NC}"
    done
fi

echo ""

echo -e "${BOLD}── Recommended Workflow/Generator Improvements ──${NC}"
if [ ${#WORKFLOW_FIXES[@]} -eq 0 ]; then
    echo -e "  ${GREEN}No workflow improvements needed.${NC}"
else
    for i in "${!WORKFLOW_FIXES[@]}"; do
        echo -e "  ${YELLOW}$((i+1)). ${WORKFLOW_FIXES[$i]}${NC}"
    done
fi

echo ""

# =============================================================================
# PHASE 6: Final Verdict & Re-run Decision
# =============================================================================

echo -e "${BOLD}── Final Verdict ──${NC}"
echo -e "  Bug/Correctness: ${CYAN}${BUG_RESULT}${NC}"
echo -e "  Quality:         ${CYAN}${QUALITY_RESULT}${NC}"

# Re-run decision
RERUN=false
RERUN_REASON=""

if [ "$BUG_RESULT" = "FAIL" ]; then
    RERUN=true
    RERUN_REASON="Correctness failures need verification after fixes"
elif [ "$BUG_RESULT" = "PARTIAL_PASS" ]; then
    RERUN=true
    RERUN_REASON="Some steps failed — re-run to verify after addressing failures"
elif [ "$QUALITY_RESULT" = "FAIL" ] || [ "$QUALITY_RESULT" = "NO_OUTPUTS" ]; then
    RERUN=true
    RERUN_REASON="Quality too low — re-run with improved prompt/config"
elif [ "$LOG_FORMAT" = "legacy" ]; then
    RERUN=true
    RERUN_REASON="Legacy log format — re-run with updated code for structured events"
fi

echo ""
if [ "$RERUN" = true ]; then
    echo -e "  ${YELLOW}⟳ RE-RUN RECOMMENDED: ${RERUN_REASON}${NC}"
else
    echo -e "  ${GREEN}✓ No re-run needed.${NC}"
fi

echo ""

# =============================================================================
# PHASE 7: Save analysis report
# =============================================================================

REPORT_DIR=$(dirname "$OUTPUT_DIR")/logs
mkdir -p "$REPORT_DIR"
REPORT_FILE="$REPORT_DIR/run-analysis.md"

cat > "$REPORT_FILE" <<EOF
# Run Analysis Report

**Generated:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")
**Log file:** ${LOG_FILE}
**Output dir:** ${OUTPUT_DIR}

## Run Metadata

| Field | Value |
|-------|-------|
| Workflow ID | ${WORKFLOW_ID} |
| Timestamp | ${RUN_TIMESTAMP} |
| Log format | ${LOG_FORMAT} |
| Models tested | ${MODEL_COUNT} |
| Log lines | ${LOG_SIZE} |

## Bug / Correctness: ${BUG_RESULT}

| Metric | Count |
|--------|-------|
| Steps succeeded | ${STEPS_OK} |
| Steps failed | ${STEPS_FAIL} |
| Generator errors | ${GENERATOR_ERRORS} |
| Executor errors | ${EXECUTOR_ERRORS} |
| Hook errors | ${HOOK_ERRORS} |
| Bug/code errors | ${BUG_ERRORS} |
| Panics | ${LEGACY_ERRORS} |
| False successes | ${FALSE_SUCCESS} |
| Valid JSON outputs | ${VALID_JSON} |
| Invalid JSON outputs | ${INVALID_JSON} |
| Empty outputs | ${EMPTY_FILES} |

## Quality: ${QUALITY_RESULT}

| Metric | Value |
|--------|-------|
| Good outputs | ${GOOD_OUTPUTS} |
| Poor outputs | ${POOR_OUTPUTS} |
| Average size | ${AVG_BYTES:-0} bytes |
| Average lines | ${AVG_LINES:-0} |
| Smallest | ${MIN_FILE} (${MIN_SIZE} bytes) |
| Largest | ${MAX_FILE} (${MAX_SIZE} bytes) |

## Issues

$(for i in "${!ISSUES[@]}"; do echo "$((i+1)). ${ISSUES[$i]}"; done)
$(if [ ${#ISSUES[@]} -eq 0 ]; then echo "None."; fi)

## Recommended Code Fixes

$(for i in "${!CODE_FIXES[@]}"; do echo "$((i+1)). ${CODE_FIXES[$i]}"; done)
$(if [ ${#CODE_FIXES[@]} -eq 0 ]; then echo "None."; fi)

## Recommended Workflow Improvements

$(for i in "${!WORKFLOW_FIXES[@]}"; do echo "$((i+1)). ${WORKFLOW_FIXES[$i]}"; done)
$(if [ ${#WORKFLOW_FIXES[@]} -eq 0 ]; then echo "None."; fi)

## Re-run Decision

$(if [ "$RERUN" = true ]; then echo "**RE-RUN RECOMMENDED**: ${RERUN_REASON}"; else echo "**No re-run needed**"; fi)

## Summary

| Dimension | Result |
|-----------|--------|
| Bug/Correctness | ${BUG_RESULT} |
| Quality | ${QUALITY_RESULT} |
| Re-run needed | ${RERUN} |
EOF

echo -e "Report saved to: ${CYAN}${REPORT_FILE}${NC}"
echo ""
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
