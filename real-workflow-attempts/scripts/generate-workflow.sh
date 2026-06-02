#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEMPLATE="$SCRIPT_DIR/../meta-workflow-template.yml"
RUN_ID="gen-$(date +%Y%m%d-%H%M%S)"

RUN_GENERATED=false
VERBOSE=false
PROMPT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --run)     RUN_GENERATED=true; shift ;;
    --verbose) VERBOSE=true; shift ;;
    -h|--help)
      echo "Usage: $0 \"<task prompt>\" [--run] [--verbose]"
      echo ""
      echo "  --run      Also run the generated workflow end-to-end"
      echo "  --verbose  Show full engine output"
      echo ""
      echo "Output: ./real-workflow-attempts/outputs/<run-id>/final-workflow.yml"
      exit 0 ;;
    *)         PROMPT="$1"; shift ;;
  esac
done

if [[ -z "$PROMPT" ]]; then
  echo "Usage: $0 \"<task prompt>\" [--run] [--verbose]"
  exit 1
fi

OUTPUT_DIR="$SCRIPT_DIR/../outputs/$RUN_ID"
mkdir -p "$OUTPUT_DIR/logs"

echo "═══════════════════════════════════════════════════════════════"
echo "  Meta-Workflow Generator"
echo "  Run ID: $RUN_ID"
echo "  Task: ${PROMPT:0:80}..."
echo "═══════════════════════════════════════════════════════════════"

WORKFLOW_FILE="$OUTPUT_DIR/input-workflow.yml"
awk -v task="$PROMPT" '{ gsub(/__TASK_PLACEHOLDER__/, task); print }' "$TEMPLATE" > "$WORKFLOW_FILE"
sed -i "s|__RUN_ID__|$RUN_ID|g" "$WORKFLOW_FILE"

if ! curl -sf http://localhost:8080/health > /dev/null 2>&1; then
  echo "[ERROR] llama.cpp server not running. Start with: whitt server start"
  exit 1
fi

echo "[1/3] Running meta-workflow (2-4 min)..."

cd "$PROJECT_DIR"

if [ "$VERBOSE" = true ]; then
  cargo run --release --features client --bin whitt -- benchmark --workflow "$WORKFLOW_FILE" 2>&1 | tee "$OUTPUT_DIR/logs/engine.log"
else
  cargo run --release --features client --bin whitt -- benchmark --workflow "$WORKFLOW_FILE" > "$OUTPUT_DIR/logs/engine.log" 2>&1
fi

FINAL="$OUTPUT_DIR/final-workflow.yml"

if [[ ! -f "$FINAL" ]]; then
  echo "[ERROR] Output not found at $FINAL"
  echo "Check: $OUTPUT_DIR/logs/engine.log"
  exit 1
fi

echo "[2/3] Validating..."
python3 "$SCRIPT_DIR/validate-yaml.py" "$FINAL" 2>&1 | tail -3
cargo run --release --features client --bin whitt -- workflow "$FINAL" 2>&1 | tail -1

echo "[3/3] Done."
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  OUTPUT: $FINAL"
echo "  LOGS:   $OUTPUT_DIR/logs/"
echo "═══════════════════════════════════════════════════════════════"

if [ "$RUN_GENERATED" = true ]; then
  E2E_DIR="$OUTPUT_DIR/e2e-run"
  mkdir -p "$E2E_DIR"
  echo ""
  echo "[BONUS] Running generated workflow..."
  cd "$E2E_DIR"
  cargo run --release --features client --bin whitt --manifest-path "$PROJECT_DIR/Cargo.toml" -- \
    benchmark --workflow "$FINAL" 2>&1 | tee "$E2E_DIR/engine.log" || true
  echo "  E2E output: $E2E_DIR/"
fi
