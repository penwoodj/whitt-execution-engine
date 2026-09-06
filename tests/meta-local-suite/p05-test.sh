#!/bin/bash
# Functional test for p05 (rustapi). Args: $1 = artifacts dir, $2 = repo root
set -uo pipefail
ART="${1:?artifacts dir}"
REPO="${2:-$(cd "$(dirname "$0")/../.." && pwd)}"
PY=$(ls "$ART"/rustapi.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

OUT=$(python3 "$PY" "$REPO/src/client/http_client.rs" 2>&1) || { echo "FUNC_FAIL: script errored: $(echo "$OUT" | head -2)"; exit 1; }
echo "$OUT" | head -5
# must be a markdown table mentioning known pub fns
echo "$OUT" | grep -q '|' || { echo "FUNC_FAIL: no markdown table"; exit 1; }
HITS=0
for fn in new health load_model unload_model chat_completion list_models; do
  echo "$OUT" | grep -q "$fn" && HITS=$((HITS+1))
done
[ "$HITS" -ge 4 ] || { echo "FUNC_FAIL: only $HITS known pub fns found (need >=4)"; exit 1; }
# missing file → nonzero, no traceback
ERR=$(python3 "$PY" /tmp/definitely-missing.rs 2>&1) && { echo "FUNC_FAIL: missing file should exit nonzero"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback on missing file"; exit 1; }
echo "FUNC_PASS"
