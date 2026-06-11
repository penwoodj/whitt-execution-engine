#!/usr/bin/env bash
set -euo pipefail

# Run json-bench-3step.yml with a specific model, validate output, record results
# Usage: ./scripts/run-model-bench.sh <model_name> <output_dir>
# Example: ./scripts/run-model-bench.sh "Qwen2.5-Coder-3B-Instruct-Q8_0" ./docs/benchmarks/outputs/model-bench/run-001

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
MODEL_NAME="${1:?Usage: run-model-bench.sh <model_name> <output_dir>}"
OUTPUT_DIR="${2:?Usage: run-model-bench.sh <model_name> <output_dir>}"
WORKFLOW="$PROJECT_DIR/docs/benchmarks/workflows/json-bench-3step.yml"
TIMING_FILE="$OUTPUT_DIR/timing.json"

mkdir -p "$OUTPUT_DIR"

docker exec whitt-llama-server pkill -f llama-server 2>/dev/null || true
sleep 2

echo "[bench] Testing model: $MODEL_NAME"
echo "[bench] Output dir: $OUTPUT_DIR"

# Create model-specific workflow by replacing MODEL_PLACEHOLDER
TEMP_WORKFLOW=$(mktemp /tmp/json-bench-XXXXXX.yml)
sed "s|MODEL_PLACEHOLDER|${MODEL_NAME}|g" "$WORKFLOW" > "$TEMP_WORKFLOW"

# Clear previous outputs
rm -f "$PROJECT_DIR/docs/benchmarks/outputs/output/step_01_deps_raw-output.txt"
rm -f "$PROJECT_DIR/docs/benchmarks/outputs/output/step_02_json-output.txt"
rm -f "$PROJECT_DIR/docs/benchmarks/outputs/output/final-result.json"

# Run benchmark
START_TIME=$(date +%s%3N)
BENCH_EXIT=0
cd "$PROJECT_DIR"
cargo run --release --features client --bin whitt -- benchmark \
  --workflow "$TEMP_WORKFLOW" \
  --models-dir "$PROJECT_DIR/models" \
  --min-tmp-space 10 \
  > "$OUTPUT_DIR/engine.log" 2>&1 || BENCH_EXIT=$?
END_TIME=$(date +%s%3N)
ELAPSED_MS=$((END_TIME - START_TIME))

# Clean up temp
rm -f "$TEMP_WORKFLOW"

# Check if engine succeeded
if [ $BENCH_EXIT -ne 0 ]; then
  echo "[bench] ENGINE FAIL (exit=$BENCH_EXIT, ${ELAPSED_MS}ms)"
  echo "{\"model\": \"$MODEL_NAME\", \"engine_pass\": false, \"exit_code\": $BENCH_EXIT, \"elapsed_ms\": $ELAPSED_MS}" > "$TIMING_FILE"
  # Save partial results if any
  cp "$PROJECT_DIR/docs/benchmarks/outputs/output/"*.txt "$OUTPUT_DIR/" 2>/dev/null || true
  cp "$PROJECT_DIR/docs/benchmarks/outputs/output/"*.json "$OUTPUT_DIR/" 2>/dev/null || true
  exit 0  # Don't fail the whole loop
fi

# Copy outputs
cp "$PROJECT_DIR/docs/benchmarks/outputs/output/step_01_deps_raw-output.txt" "$OUTPUT_DIR/" 2>/dev/null || true
cp "$PROJECT_DIR/docs/benchmarks/outputs/output/step_02_json-output.txt" "$OUTPUT_DIR/" 2>/dev/null || true
cp "$PROJECT_DIR/docs/benchmarks/outputs/output/final-result.json" "$OUTPUT_DIR/" 2>/dev/null || true

# Validate output
VALIDATE_EXIT=0
VALIDATE_OUTPUT=$("$PROJECT_DIR/scripts/validate-json-output.py" "$OUTPUT_DIR/final-result.json" 2>&1) || VALIDATE_EXIT=$?

echo "$VALIDATE_OUTPUT" > "$OUTPUT_DIR/validate.log"
echo "$VALIDATE_OUTPUT"

# Record timing
if [ $VALIDATE_EXIT -eq 0 ]; then
  STATUS="PASS"
else
  STATUS="PARTIAL"
fi

echo "{\"model\": \"$MODEL_NAME\", \"engine_pass\": true, \"validate_pass\": $([ $VALIDATE_EXIT -eq 0 ] && echo true || echo false), \"status\": \"$STATUS\", \"elapsed_ms\": $ELAPSED_MS}" > "$TIMING_FILE"

echo "[bench] $STATUS (${ELAPSED_MS}ms)"

# Restart docker between runs for clean state
docker exec whitt-llama-server pkill -f llama-server 2>/dev/null || true
sleep 2
docker restart whitt-llama-server > /dev/null 2>&1 || true
sleep 8

exit 0
