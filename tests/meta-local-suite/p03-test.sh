#!/bin/bash
# Functional test for p03 (kvstore): the deliverable's OWN unit tests must pass.
# Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
[ -f "$ART/kvstore.py" ] || { echo "FUNC_FAIL: kvstore.py missing"; exit 1; }
TESTF=$(ls "$ART"/test_kvstore.py 2>/dev/null || ls "$ART"/test*.py 2>/dev/null | head -1)
[ -z "$TESTF" ] && { echo "FUNC_FAIL: test file missing"; exit 1; }

TMP=$(mktemp -d)
cp "$ART/kvstore.py" "$TMP/"
cp "$TESTF" "$TMP/test_kvstore.py"
cd "$TMP"
if python3 -m unittest test_kvstore -v 2>&1 | tail -3 | grep -q "OK"; then
  echo "FUNC_PASS"
else
  python3 -m unittest test_kvstore 2>&1 | tail -5
  echo "FUNC_FAIL: unit tests failed"
  exit 1
fi
