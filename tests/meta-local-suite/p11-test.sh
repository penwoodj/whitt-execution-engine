#!/bin/bash
# Functional test for p11 (linkextract). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/linkextract.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/page.html" <<'EOF'
<html><body>
<a href="https://example.com/a">A</a>
<a href="http://example.org/b?q=1">B</a>
<a href="https://example.com/a">dup</a>
<a href="/relative/path">rel</a>
<a href="#frag">frag</a>
<a href="mailto:x@y.z">mail</a>
<a href="javascript:void(0)">js</a>
<img src="https://cdn.example.net/img.png">
<script src="app.js"></script>
</body></html>
EOF
OUT=$(python3 "$PY" "$TMP/page.html" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
echo "$OUT"
EXPECTED=$(printf 'http://example.org/b?q=1\nhttps://cdn.example.net/img.png\nhttps://example.com/a\n')
[ "$OUT" = "$EXPECTED" ] || { echo "FUNC_FAIL: output mismatch"; exit 1; }
ERR=$(python3 "$PY" "$TMP/nope.html" 2>&1) && { echo "FUNC_FAIL: missing file exit"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
