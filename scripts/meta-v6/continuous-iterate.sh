#!/bin/bash
# scripts/meta-v6/continuous-iterate.sh
# Runs all 11 prompts via SW1-SW5 pipeline unattended.
# Tracks PASS/FAIL per prompt with timing.
# Auto-files bug reports for failures.
#
# Usage: continuous-iterate.sh [--start <P-num>] [--end <P-num>] [--output-dir <path>]

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real"
OUTPUT_BASE="$REPO/docs/benchmarks/outputs/meta-workflow"
RESULTS_FILE="$REPO/docs/plans/meta-workflow-parity/continuous-iterate-results.md"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
RUN_ID="continuous-$TIMESTAMP"

# Parse args
START=5
END=15
while [[ $# -gt 0 ]]; do
    case "$1" in
        --start) START="$2"; shift 2;;
        --end) END="$2"; shift 2;;
        --output-dir) OUTPUT_BASE="$2"; shift 2;;
        *) echo "Unknown arg: $1" >&2; exit 1;;
    esac
done

mkdir -p "$OUTPUT_BASE"

# Initialize results file
cat > "$RESULTS_FILE" << EOF
# Continuous Iteration Results — $RUN_ID

Started: $(date)
Range: P$START..P$END

| Prompt | Status | Validator | Size (B) | Runtime (s) | Bug Filed |
|--------|--------|-----------|----------|-------------|-----------|
EOF

# Iterate
for P_NUM in $(seq "$START" "$END"); do
    # Find prompt file (handles various naming conventions)
    PROMPT_FILE=$(ls "$PROMPTS_DIR"/prompt-${P_NUM}-*.md 2>/dev/null | head -1)
    if [[ -z "$PROMPT_FILE" ]]; then
        echo "[continuous] P$P_NUM: NO PROMPT FILE FOUND"
        echo "| P$P_NUM | NO_PROMPT | - | - | - | - |" >> "$RESULTS_FILE"
        continue
    fi

    META_RUN_ID="p${P_NUM}-sw-final-$TIMESTAMP"
    META_DIR="$OUTPUT_BASE/$META_RUN_ID"
    DELIVERABLE="$META_DIR/deliverables/deliverable.md"
    PIPELINE_LOG="/tmp/${META_RUN_ID}.log"
    BUG_FILE="$REPO/docs/plans/meta-workflow-parity/bugs/P${P_NUM}-${TIMESTAMP}.md"

    echo "============================================================"
    echo "[continuous] P$P_NUM: STARTING"
    echo "[continuous]   prompt: $PROMPT_FILE"
    echo "[continuous]   run_id: $META_RUN_ID"
    echo "============================================================"

    P_START=$(date +%s)

    # Run pipeline
    mkdir -p "$META_DIR/deliverables"
    bash "$REPO/scripts/meta-v6/debug/pipeline.sh" \
        "$PROMPT_FILE" \
        "$DELIVERABLE" \
        "$META_RUN_ID" > "$PIPELINE_LOG" 2>&1
    PIPELINE_EXIT=$?

    P_END=$(date +%s)
    DURATION=$((P_END - P_START))

    # Find actual deliverable (may be deliverable.md or different name)
    ACTUAL_DELIVERABLE=$(find "$META_DIR/deliverables" -type f -size +100c 2>/dev/null | head -1)
    WORKFLOW_YML="$META_DIR/meta/generated-workflow.yml"

    if [[ -z "$ACTUAL_DELIVERABLE" ]]; then
        echo "[continuous] P$P_NUM: FAIL (no deliverable)"
        echo "| P$P_NUM | FAIL | 0/50 | 0 | ${DURATION}s | yes |" >> "$RESULTS_FILE"

        # File bug report
        mkdir -p "$(dirname "$BUG_FILE")"
        cat > "$BUG_FILE" << EOF
# Bug: P$P_NUM pipeline failure — $TIMESTAMP

**Run ID:** $META_RUN_ID
**Failure:** No deliverable produced
**Pipeline log:** $PIPELINE_LOG
**Duration:** ${DURATION}s

## Pipeline log tail
\`\`\`
$(tail -30 "$PIPELINE_LOG" 2>/dev/null)
\`\`\`

## Workflow YAML (if exists)
$(test -f "$WORKFLOW_YML" && echo "EXISTS at $WORKFLOW_YML" || echo "MISSING")

## Next steps
- Inspect pipeline log for SW1-SW5 failures
- Check workflow YAML validity
- Identify missing fix
EOF
        echo "[continuous] Bug filed: $BUG_FILE"
        continue
    fi

    # Run validator
    VALIDATOR_OUTPUT=$(bash "$REPO/scripts/meta-v6/parity-check.sh" \
        "$WORKFLOW_YML" \
        "$ACTUAL_DELIVERABLE" \
        "$META_DIR" 2>&1)
    SCORE=$(echo "$VALIDATOR_OUTPUT" | grep -oE 'Score: [0-9]+/[0-9]+' | head -1 | grep -oE '^[0-9]+/[0-9]+' || echo "0/50")
    SCORE_NUM=$(echo "$SCORE" | cut -d/ -f1)
    VERDICT=$(echo "$VALIDATOR_OUTPUT" | grep -oE 'VERDICT: [A-Z]+' | head -1 || echo "VERDICT: UNKNOWN")

    SIZE=$(wc -c < "$ACTUAL_DELIVERABLE")

    if [[ "$VERDICT" == "VERDICT: PASS" ]]; then
        echo "[continuous] P$P_NUM: PASS ($SCORE, ${SIZE}B, ${DURATION}s)"
        echo "| P$P_NUM | PASS | $SCORE | $SIZE | ${DURATION}s | no |" >> "$RESULTS_FILE"
    else
        echo "[continuous] P$P_NUM: FAIL ($SCORE, ${SIZE}B, ${DURATION}s)"
        echo "| P$P_NUM | FAIL | $SCORE | $SIZE | ${DURATION}s | yes |" >> "$RESULTS_FILE"

        # File bug report
        mkdir -p "$(dirname "$BUG_FILE")"
        cat > "$BUG_FILE" << EOF
# Bug: P$P_NUM validator FAIL — $TIMESTAMP

**Run ID:** $META_RUN_ID
**Validator score:** $SCORE
**Deliverable size:** ${SIZE}B
**Duration:** ${DURATION}s

## Deliverable head
\`\`\`
$(head -20 "$ACTUAL_DELIVERABLE" 2>/dev/null)
\`\`\`

## Validator output
\`\`\`
$VALIDATOR_OUTPUT
\`\`\`

## Next steps
- Analyze validator failures
- Identify SW layer responsible (SW1-SW5)
- Apply fix + re-run
EOF
        echo "[continuous] Bug filed: $BUG_FILE"
    fi
done

# Summary
echo ""
echo "============================================================"
echo "[continuous] COMPLETE"
echo "[continuous] Results: $RESULTS_FILE"
echo "============================================================"
cat "$RESULTS_FILE"

# Compute totals
PASS_COUNT=$(grep -c '| PASS |' "$RESULTS_FILE")
FAIL_COUNT=$(grep -c '| FAIL |' "$RESULTS_FILE")
TOTAL=$((PASS_COUNT + FAIL_COUNT))
if [[ $TOTAL -gt 0 ]]; then
    PASS_RATE=$((PASS_COUNT * 100 / TOTAL))
    echo ""
    echo "Pass rate: $PASS_COUNT/$TOTAL (${PASS_RATE}%)"
fi
