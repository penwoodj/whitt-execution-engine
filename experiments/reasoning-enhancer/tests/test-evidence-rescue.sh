#!/usr/bin/env bash
# TDD test for evidence-rescue.py (v3.0 cascade r3 prompt builder).
#
# Contract:
#   - reads check-r1.json + check-r2.json failures from --run-dir
#   - extracts missing tokens from contains_required failure details
#     (detail "missing: ['X', 'Y']"); none parseable -> falls back to ALL
#     contains_required tokens
#   - greps chunk files for each token (verbatim first, then len>=5
#     identifier parts) and collects containing lines verbatim
#   - prints rescue prompt to stdout: TASK + TARGETED RESCUE EVIDENCE +
#     OPEN FAILURES + HARD OUTPUT RULES
#   - --out F also writes prompt to F; exit 0 always

set -u
REA="$(cd "$(dirname "$0")/.." && pwd)"
S="$REA/scripts/evidence-rescue.py"
TMP="$(mktemp -d /tmp/evresc-test.XXXXXX)"
trap 'rm -rf "$TMP"' EXIT
pass=0; fail=0
ck() {
  if [ "$2" -eq "$3" ]; then pass=$((pass+1)); echo "ok   - $1";
  else fail=$((fail+1)); echo "FAIL - $1 (want rc=$2 got rc=$3)"; fi
}

CASE01="$REA/cases/overcontext/case-o01.yml"

mkdir -p "$TMP/run"
cat > "$TMP/run/check-r1.json" <<'J1'
{"passed": false,
 "failures": [{"check": "contains_required", "passed": false,
               "detail": "missing: ['EV-8802', 'EV-9917']"}]}
J1
cp "$TMP/run/check-r1.json" "$TMP/run/check-r2.json"

python3 "$S" --case "$CASE01" --run-dir "$TMP/run" --out "$TMP/run/rescue-prompt.txt" > "$TMP/stdout.txt" 2>"$TMP/err.txt"
ck "exit 0 always" 0 $?

grep -q "QUESTION\|TASK" "$TMP/stdout.txt" && { pass=$((pass+1)); echo "ok   - prompt carries the task"; } || { fail=$((fail+1)); echo "FAIL - no task section"; }

hit=$(grep -c "EV-8802\|EV-9917" "$TMP/stdout.txt")
if [ "$hit" -ge 2 ]; then pass=$((pass+1)); echo "ok   - missing tokens and their doc lines present";
else fail=$((fail+1)); echo "FAIL - missing tokens/lines absent (hits=$hit)"; fi

grep -q "HARD OUTPUT RULES" "$TMP/stdout.txt" && { pass=$((pass+1)); echo "ok   - hard rules section"; } || { fail=$((fail+1)); echo "FAIL - no hard rules"; }
grep -q "LOGFLOOD\|DISKWEAR" "$TMP/stdout.txt" && { pass=$((pass+1)); echo "ok   - verbatim doc line content injected"; } || { fail=$((fail+1)); echo "FAIL - doc line with code not injected"; }
diff -q "$TMP/stdout.txt" "$TMP/run/rescue-prompt.txt" >/dev/null && { pass=$((pass+1)); echo "ok   - --out file matches stdout"; } || { fail=$((fail+1)); echo "FAIL - out file != stdout"; }

# degenerate: no parseable failures -> fallback to all required tokens
mkdir -p "$TMP/run2"
cat > "$TMP/run2/check-r1.json" <<'J2'
{"passed": false, "failures": [{"check": "bullet_count_min", "passed": false, "detail": "0 bullets < min 3"}]}
J2
cp "$TMP/run2/check-r1.json" "$TMP/run2/check-r2.json"
python3 "$S" --case "$CASE01" --run-dir "$TMP/run2" > "$TMP/stdout2.txt"
ck "degenerate exit 0" 0 $?
grep -q "EV-4471" "$TMP/stdout2.txt" && { pass=$((pass+1)); echo "ok   - fallback to all required tokens"; } || { fail=$((fail+1)); echo "FAIL - fallback missing"; }

echo "----------------------------------------"
echo "pass=$pass fail=$fail"
[ "$fail" -eq 0 ]
