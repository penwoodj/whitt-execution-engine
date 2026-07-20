#!/bin/bash
# Functional test for p04 (salesreport). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/salesreport.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/sales.csv" <<'EOF'
date,region,product,units,unit_price
2026-01-05,west,widget,10,2.50
2026-01-06,east,gadget,5,10.00
2026-01-07,west,gadget,2,10.00
2026-01-08,north,widget,20,2.50
EOF
OUT=$(python3 "$PY" "$TMP/sales.csv" "$TMP/report.md" 2>&1)
echo "$OUT" | grep -q "REPORT OK" || { echo "FUNC_FAIL: no REPORT OK (got: $(echo "$OUT" | head -2))"; exit 1; }
[ -s "$TMP/report.md" ] || { echo "FUNC_FAIL: report missing"; exit 1; }
# total revenue = 25 + 50 + 20 + 50 = 145
grep -qE '145(\.0+)?' "$TMP/report.md" || { echo "FUNC_FAIL: total revenue 145 not found"; exit 1; }
# best-selling product by units = widget (30)
grep -qi 'widget' "$TMP/report.md" || { echo "FUNC_FAIL: best seller widget not found"; exit 1; }
# regions present
for r in west east north; do grep -qi "$r" "$TMP/report.md" || { echo "FUNC_FAIL: region $r missing"; exit 1; }; done
# missing csv → nonzero exit, no traceback
if python3 "$PY" "$TMP/nope.csv" "$TMP/x.md" >/dev/null 2>&1; then
  echo "FUNC_FAIL: missing csv should exit nonzero"; exit 1
fi
echo "FUNC_PASS"
