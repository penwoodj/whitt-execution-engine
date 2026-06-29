#!/bin/bash
# scripts/meta-v6/analyze-sw-logs.sh
# Parses pipeline log to extract per-SW phase timings and identify failures.
#
# Usage: analyze-sw-logs.sh <pipeline-log> [meta-run-dir]
#
# Output:
#   - Per-SW phase timing breakdown (start/end/duration per step within each SW)
#   - Failure identification (which SW/step failed)
#   - Root cause hints (specific error patterns)
#
# Exit codes:
#   0 = analysis succeeded (regardless of pipeline outcome)
#   1 = usage error or missing log

set -uo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <pipeline-log> [meta-run-dir]" >&2
    exit 1
fi

LOG="$1"
META_DIR="${2:-}"

if [[ ! -f "$LOG" ]]; then
    echo "ERROR: Log file not found: $LOG" >&2
    exit 1
fi

echo "============================================================"
echo "SW1-SW5 Pipeline Log Analysis"
echo "Log: $LOG"
echo "============================================================"
echo ""

# Phase 1: SW-level summary (start/end/duration)
echo "## SW-Level Summary"
echo ""
printf "| %-4s | %-12s | %-12s | %-10s | %-8s |\n" "SW" "START" "END" "DURATION" "STATUS"
printf "|------|--------------|--------------|------------|----------|\n"

for SW_N in 1 2 3 4 5; do
    SW_START_LINE=$(grep "\[pipeline\] SW${SW_N}:start" "$LOG" 2>/dev/null | head -1)
    SW_END_LINE=$(grep "\[pipeline\] SW${SW_N}:end" "$LOG" 2>/dev/null | head -1)
    SW_START=$(echo "$SW_START_LINE" | grep -oE '^\[[0-9:]+\]' | tr -d '[]')
    SW_END=$(echo "$SW_END_LINE" | grep -oE '^\[[0-9:]+\]' | tr -d '[]')
    SW_DUR=$(echo "$SW_END_LINE" | grep -oE 'duration=[0-9]+s' | head -1 | sed 's/duration=//;s/s$//')
    SW_EXIT=$(echo "$SW_END_LINE" | grep -oE 'exit=[0-9]+' | sed 's/exit=//')

    if [[ -z "$SW_START_LINE" ]]; then
        STATUS="not started"
    elif [[ -z "$SW_END_LINE" ]]; then
        STATUS="RUNNING"
    elif [[ "$SW_EXIT" == "0" ]]; then
        STATUS="PASS"
    else
        STATUS="FAIL(exit=$SW_EXIT)"
    fi

    printf "| SW%s | %-12s | %-12s | %-10s | %-8s |\n" \
        "$SW_N" \
        "${SW_START:--}" \
        "${SW_END:--}" \
        "${SW_DUR:--}s" \
        "$STATUS"
done
echo ""

# Phase 2: Per-SW output sizes
echo "## SW Output Sizes"
echo ""
if [[ -n "$META_DIR" && -d "$META_DIR" ]]; then
    for SW_N in 1 2 3 4 5; do
        SW_DIR=$(ls -td "${META_DIR%/}"/meta-*-sw${SW_N}-* 2>/dev/null | head -1)
        if [[ -n "$SW_DIR" ]]; then
            SW_OUTPUT=$(ls "$SW_DIR"/sw${SW_N}/ 2>/dev/null | grep -E '\.(md|txt|yml|yaml)$' | head -3 | tr '\n' ' ')
            SW_SIZE=$(du -sh "$SW_DIR" 2>/dev/null | cut -f1)
            echo "- SW${SW_N}: dir=$SW_SIZE outputs=[$SW_OUTPUT]"
        fi
    done
fi
echo ""

# Phase 3: Failure identification
echo "## Failure Analysis"
echo ""
FAILURES=$(grep -E "FATAL|FAILED|before_step_starts hook failed|Skipped by hook|exit=[1-9]" "$LOG" 2>/dev/null)
if [[ -z "$FAILURES" ]]; then
    echo "No failures detected."
else
    echo '```'
    echo "$FAILURES" | head -30
    echo '```'
fi
echo ""

# Phase 4: Specific failure pattern hints
echo "## Failure Pattern Hints"
echo ""

PATTERN_COUNT=$(grep -c "before_step_starts hook failed" "$LOG" 2>/dev/null)
PATTERN_COUNT=${PATTERN_COUNT:-0}
if [[ "$PATTERN_COUNT" -gt 0 ]]; then
    echo "- **Hook failures**: $PATTERN_COUNT step(s) had before_step_starts shell failures"
    echo "  Likely cause: shell \`cat\` command referenced non-existent file"
    echo "  Fix: Check SW4-emitted paths or run build-workflow.py with --meta-run-id"
    echo ""
fi

SKIP_COUNT=$(grep -c "Skipped by hook" "$LOG" 2>/dev/null)
SKIP_COUNT=${SKIP_COUNT:-0}
if [[ "$SKIP_COUNT" -gt 5 ]]; then
    echo "- **Mass step skips**: $SKIP_COUNT steps skipped by hook"
    echo "  Likely cause: bootstrap step failed OR workflow structure has dead steps"
    echo "  Fix: Verify step_00_bootstrap save_to path matches downstream cat"
    echo ""
fi

if grep -q "YAML parse error" "$LOG" 2>/dev/null; then
    echo "- **YAML parse error detected**"
    echo "  Likely cause: SW4 emitted invalid YAML structure"
    echo "  Fix: Check extract_step_blocks() trim logic in build-workflow.py"
    echo ""
fi

if grep -q "duplicate mapping key" "$LOG" 2>/dev/null; then
    echo "- **Duplicate YAML key detected**"
    echo "  Likely cause: SW4 emitted step_00_bootstrap + build-workflow.py added another"
    echo "  Fix: Check has_bootstrap dedup in build-workflow.py main()"
    echo ""
fi

# Phase 5: Final verdict
echo "## Final Verdict"
echo ""
if grep -q "PIPELINE_DONE" "$LOG" 2>/dev/null; then
    if grep -q "VERDICT: PASS" "$LOG" 2>/dev/null; then
        echo "✅ PIPELINE PASS"
    else
        echo "❌ PIPELINE FAIL"
    fi
    SCORE=$(grep -oE 'Score: [0-9]+/[0-9]+' "$LOG" | head -1)
    [[ -n "$SCORE" ]] && echo "Validator: $SCORE"
else
    echo "⏳ PIPELINE STILL RUNNING"
fi

echo ""
echo "============================================================"
