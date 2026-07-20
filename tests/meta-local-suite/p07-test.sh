#!/bin/bash
# Functional test for p07 (mdtoc). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/mdtoc.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/doc.md" <<'EOF'
# Big Title

## Getting Started

text

### Install Steps

## Usage Guide

### Advanced Options

#### too deep
EOF
OUT=$(python3 "$PY" "$TMP/doc.md" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
echo "$OUT"
LINE1=$(echo "$OUT" | sed -n '1p'); LINE2=$(echo "$OUT" | sed -n '2p')
LINE3=$(echo "$OUT" | sed -n '3p'); LINE4=$(echo "$OUT" | sed -n '4p')
[ "$LINE1" = "- [Getting Started](#getting-started)" ] || { echo "FUNC_FAIL: line1=[$LINE1]"; exit 1; }
[ "$LINE2" = "  - [Install Steps](#install-steps)" ]   || { echo "FUNC_FAIL: line2=[$LINE2]"; exit 1; }
[ "$LINE3" = "- [Usage Guide](#usage-guide)" ]         || { echo "FUNC_FAIL: line3=[$LINE3]"; exit 1; }
[ "$LINE4" = "  - [Advanced Options](#advanced-options)" ] || { echo "FUNC_FAIL: line4=[$LINE4]"; exit 1; }
echo "$OUT" | grep -qi "too deep" && { echo "FUNC_FAIL: included #### heading"; exit 1; }
echo "$OUT" | grep -qi "big title" && { echo "FUNC_FAIL: included # heading"; exit 1; }
ERR=$(python3 "$PY" "$TMP/nope.md" 2>&1) && { echo "FUNC_FAIL: missing file exit"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
