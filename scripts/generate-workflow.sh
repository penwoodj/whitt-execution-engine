#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
RUN_ID="gen-$(date +%Y%m%d-%H%M%S)"

VERBOSE=false
PROMPT=""
VERSION="v5"
PROVIDER="llamacpp"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --verbose) VERBOSE=true; shift ;;
    --v4) VERSION="v4"; shift ;;
    --v5) VERSION="v5"; shift ;;
    --lmstudio) PROVIDER="lmstudio"; shift ;;
    -h|--help)
      echo "Usage: $0 \"<task prompt>\" [--verbose] [--v4|--v5] [--lmstudio]"
      echo ""
      echo "  --verbose   Show full engine output"
      echo "  --v4        Use v4 meta-workflow (6 steps, no self-review)"
      echo "  --v5        Use v5 meta-workflow (8 steps, context-loaded, self-review) [default]"
      echo "  --lmstudio  Run against LM Studio (port 1234) using the -lmstudio template variant"
      echo ""
      echo "Output: ./docs/benchmarks/outputs/meta-workflow/<run-id>/final-workflow.yml"
      exit 0 ;;
    *)         PROMPT="$1"; shift ;;
  esac
done

if [[ -z "$PROMPT" ]]; then
  echo "Usage: $0 \"<task prompt>\" [--verbose]"
  exit 1
fi

OUTPUT_DIR="$SCRIPT_DIR/../docs/benchmarks/outputs/meta-workflow/$RUN_ID"
if [[ "$PROVIDER" == "lmstudio" ]]; then
  TEMPLATE="$SCRIPT_DIR/../docs/benchmarks/workflows/meta-workflow-${VERSION}-lmstudio.yml"
else
  TEMPLATE="$SCRIPT_DIR/../docs/benchmarks/workflows/meta-workflow-${VERSION}.yml"
fi
if [[ ! -f "$TEMPLATE" ]]; then
  echo "[ERROR] Template not found: $TEMPLATE"
  exit 1
fi
mkdir -p "$OUTPUT_DIR/logs"

echo "═══════════════════════════════════════════════════════════════"
echo "  Meta-Workflow Generator"
echo "  Run ID: $RUN_ID"
echo "  Task: ${PROMPT:0:80}..."
echo "═══════════════════════════════════════════════════════════════"

WORKFLOW_FILE="$OUTPUT_DIR/input-workflow.yml"
awk -v task="$PROMPT" '{ gsub(/__TASK_PLACEHOLDER__/, task); print }' "$TEMPLATE" > "$WORKFLOW_FILE"
# BSD (macOS) sed requires a backup suffix with -i; use a temp file for portability
sed "s|__RUN_ID__|$RUN_ID|g" "$WORKFLOW_FILE" > "$WORKFLOW_FILE.tmp" && mv "$WORKFLOW_FILE.tmp" "$WORKFLOW_FILE"

if [[ "$PROVIDER" == "lmstudio" ]]; then
  if ! curl -sf http://localhost:1234/v1/models > /dev/null 2>&1; then
    echo "[ERROR] LM Studio server not running on port 1234. Start with: lms server start (or LM Studio > Developer > Start Server)"
    exit 1
  fi
else
  if ! curl -sf http://localhost:8080/health > /dev/null 2>&1; then
    echo "[ERROR] llama.cpp server not running. Start with: whitt server start"
    exit 1
  fi
fi

echo "[1/3] Running meta-workflow ${VERSION} (3-6 min)..."

cd "$PROJECT_DIR"

ENGINE_ARGS=(benchmark --workflow "$WORKFLOW_FILE" --min-tmp-space 10)
if [[ "$PROVIDER" == "lmstudio" ]]; then
  # No local GGUF dir: model discovery falls back to LM Studio's /v1/models.
  ENGINE_ARGS=(--url "http://localhost:1234" "${ENGINE_ARGS[@]}")
else
  ENGINE_ARGS+=(--models-dir "$PROJECT_DIR/models")
fi

if [ "$VERBOSE" = true ]; then
  cargo run --release --features client --bin whitt -- "${ENGINE_ARGS[@]}" 2>&1 | tee "$OUTPUT_DIR/logs/engine.log"
else
  cargo run --release --features client --bin whitt -- "${ENGINE_ARGS[@]}" > "$OUTPUT_DIR/logs/engine.log" 2>&1
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
