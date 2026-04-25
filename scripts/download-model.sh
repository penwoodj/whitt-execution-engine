#!/bin/bash
# DEPRECATED: Use `whitt download <repo> --file <filename> --output <dir>` instead.
# This script is retained for backward compatibility but all functionality
# has been migrated to the Rust CLI binary.
set -euo pipefail

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ] || [ $# -eq 0 ]; then
    echo "Usage: $0 <huggingface-repo> [filename] [local-filename]"
    echo "Download a GGUF model from HuggingFace into the configured models directory."
    echo ""
    echo "Arguments:"
    echo "  repo            HuggingFace repo (e.g. Qwen/Qwen2.5-0.5B-Instruct-GGUF)"
    echo "  filename        File to download (prompted if omitted)"
    echo "  local-filename  Save as different name (default: same as filename)"
    echo ""
    echo "Environment:"
    echo "  MODEL_OUTPUT_DIR      Where to save models (default: ./models)"
    echo "  HF_BRANCH             Git branch (default: main)"
    echo "  HUGGING_FACE_HUB_TOKEN / HF_TOKEN  Auth token for gated models"
    echo "  SKIP_CONFIRM          Set to 'true' to skip confirmation (default: false)"
    exit 0
fi

REPO="$1"
FILENAME="${2:-}"
LOCAL_FILENAME="${3:-$FILENAME}"
BRANCH="${HF_BRANCH:-main}"
OUTPUT_DIR="${MODEL_OUTPUT_DIR:-./models}"
SKIP_CONFIRM="${SKIP_CONFIRM:-false}"
TOKEN="${HUGGING_FACE_HUB_TOKEN:-${HF_TOKEN:-}}"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_detail() { echo -e "${CYAN}[DETAIL]${NC} $1"; }

if [ -z "$REPO" ]; then
    log_error "No repo specified"
    exit 1
fi

HF_PYTHON=""
for py in \
    /home/jon/.local/share/pipx/venvs/huggingface-hub/bin/python \
    $(which python3 2>/dev/null || true) \
    $(which python 2>/dev/null || true); do
    if [ -n "$py" ] && [ -f "$py" ] && $py -c "from huggingface_hub import list_repo_files" 2>/dev/null; then
        HF_PYTHON="$py"
        break
    fi
done

if [ -z "$HF_PYTHON" ]; then
    log_error "Neither 'hf' CLI nor Python huggingface_hub found."
    log_error "Install with: pip install huggingface-hub"
    exit 1
fi

log_info "Querying available files for: $REPO"
GGUF_FILES=$($HF_PYTHON -c "
from huggingface_hub import list_repo_files
try:
    files = list(list_repo_files('$REPO', repo_type='model'))
    for f in sorted(files):
        if f.endswith('.gguf'):
            print(f)
except Exception as e:
    import sys
    print(f'ERROR: {e}', file=sys.stderr)
    sys.exit(1)
" 2>&1) || { log_error "Failed to list repo files. Check repo name or auth token."; exit 1; }

if echo "$GGUF_FILES" | head -1 | grep -q "^ERROR:"; then
    log_error "Failed to list repo files:"
    echo "$GGUF_FILES"
    exit 1
fi

if [ -z "$GGUF_FILES" ]; then
    log_error "No .gguf files found in $REPO"
    exit 1
fi

if [ -z "$FILENAME" ]; then
    echo ""
    echo -e "${CYAN}Available GGUF files in $REPO:${NC}"
    echo "$GGUF_FILES" | nl -ba
    echo ""

    read -rp "Enter the number or filename to download: " SELECTION

    if echo "$SELECTION" | grep -qE '^[0-9]+$'; then
        FILENAME=$(echo "$GGUF_FILES" | sed -n "${SELECTION}p")
    else
        FILENAME="$SELECTION"
    fi

    if [ -z "$FILENAME" ]; then
        log_error "No file selected"
        exit 1
    fi
fi

if ! echo "$GGUF_FILES" | grep -qF "$FILENAME"; then
    log_warn "Filename '$FILENAME' not found in repo listing. Will attempt download anyway."
fi

DEST_PATH="${OUTPUT_DIR}/${LOCAL_FILENAME:-$FILENAME}"
DEST_DIR=$(dirname "$DEST_PATH")
DEST_BASENAME=$(basename "$DEST_PATH")

log_detail "Repo:       $REPO"
log_detail "File:       $FILENAME"
log_detail "Branch:     $BRANCH"
log_detail "Output:     $DEST_PATH"

if [ -f "$DEST_PATH" ]; then
    EXISTING_SIZE=$(du -h "$DEST_PATH" | awk '{print $1}')
    log_detail "Existing:   $DEST_PATH ($EXISTING_SIZE)"

    if [ "$SKIP_CONFIRM" = "false" ]; then
        read -rp "File exists. Overwrite? [y/N]: " CONFIRM
        if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
            log_info "Cancelled"
            exit 0
        fi
    fi
fi

if [ "$SKIP_CONFIRM" = "false" ]; then
    echo ""
    read -rp "Download $FILENAME from $REPO to $DEST_PATH? [Y/n]: " CONFIRM
    if [ "$CONFIRM" = "n" ] || [ "$CONFIRM" = "N" ]; then
        log_info "Cancelled"
        exit 0
    fi
fi

mkdir -p "$DEST_DIR"

HF_ARGS=()
if [ -n "$BRANCH" ]; then
    HF_ARGS+=(--revision "$BRANCH")
fi
if [ -n "$TOKEN" ]; then
    export HUGGING_FACE_HUB_TOKEN="$TOKEN"
    HF_ARGS+=(--token "$TOKEN")
fi

log_info "Downloading: $REPO/$FILENAME"
log_info "Destination: $DEST_PATH"
START_TIME=$(date +%s)

if timeout 600 hf download \
    --repo-type model \
    "${HF_ARGS[@]}" \
    "$REPO" \
    "$FILENAME" \
    --local-dir "$OUTPUT_DIR"; then

    if [ "$FILENAME" != "$DEST_BASENAME" ]; then
        mv "${OUTPUT_DIR}/${FILENAME}" "$DEST_PATH"
    fi

    END_TIME=$(date +%s)
    ELAPSED=$((END_TIME - START_TIME))
    DL_SIZE=$(du -h "$DEST_PATH" | awk '{print $1}')
    DL_SPEED="N/A"
    if [ $ELAPSED -gt 0 ]; then
        BYTES=$(stat -c%s "$DEST_PATH" 2>/dev/null || echo "0")
        if [ "$BYTES" -gt 0 ]; then
            MBPS=$(echo "scale=1; $BYTES / $ELAPSED / 1048576" | bc 2>/dev/null || echo "N/A")
            DL_SPEED="${MBPS} MB/s"
        fi
    fi

    echo ""
    log_info "Download complete!"
    log_detail "File:     $DEST_PATH"
    log_detail "Size:     $DL_SIZE"
    log_detail "Time:     ${ELAPSED}s"
    log_detail "Speed:    $DL_SPEED"
else
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 124 ]; then
        log_error "Download timed out after 600 seconds"
    else
        log_error "Download failed (exit code: $EXIT_CODE)"
    fi
    rm -f "$DEST_PATH"
    exit 1
fi
