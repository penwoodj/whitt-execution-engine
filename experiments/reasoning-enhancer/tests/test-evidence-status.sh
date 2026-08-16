#!/usr/bin/env bash
# TDD test for evidence-status.py (v3.0 smart intake gate).
# RED first: script must not exist yet; run tests, watch them fail, implement, watch green.
#
# Gate contract:
#   - reads success_criteria.deterministic_checks.contains_required from --case
#   - gate tokens = required tokens present VERBATIM in chunk files
#     PLUS identifiers (alnum-dash parts, len>=5) of non-verbatim required
#     tokens that themselves appear verbatim in chunk files
#   - COMPLETE (exit 7) iff EVERY gate token appears in run-dir/notes.txt
#   - MORE (exit 0) otherwise; empty/no gate tokens -> MORE (read all, safe)
#   - always writes run-dir/evidence-status.json {complete, missing, gate_tokens}

set -u
REA="$(cd "$(dirname "$0")/.." && pwd)"
S="$REA/scripts/evidence-status.py"
TMP="$(mktemp -d /tmp/evstat-test.XXXXXX)"
trap 'rm -rf "$TMP"' EXIT
pass=0; fail=0
ck() { # name expected_rc actual_rc
  if [ "$2" -eq "$3" ]; then pass=$((pass+1)); echo "ok   - $1 (rc=$3)";
  else fail=$((fail+1)); echo "FAIL - $1 (want rc=$2 got rc=$3)"; fi
}

CASE01="$REA/cases/overcontext/case-o01.yml"
CASE06="$REA/cases/overcontext/case-o06.yml"

# --- fixture run dirs
mkdir -p "$TMP/r1" "$TMP/r2" "$TMP/r3" "$TMP/r4"
# o01 verbatim tokens: EV-4471 LOGFLOOD EV-8802 DISKWEAR EV-9917 LINKROT
printf 'id=EV-4471 code=LOGFLOOD spike\n' > "$TMP/r1/notes.txt"            # incomplete
printf -- '- id=EV-4471 code=LOGFLOOD\n- id=EV-8802 code=DISKWEAR\n- id=EV-9917 code=LINKROT\n' > "$TMP/r2/notes.txt"  # complete
# o06 identifiers: gateway_workers / search_shards (answers computed, not verbatim)
printf 'policy: scale search_shards after drift\n' > "$TMP/r3/notes.txt"    # 1 of 2
printf 'gateway_workers bump; search_shards bump\n' > "$TMP/r4/notes.txt"   # complete

# --- synthetic case: one token verbatim in doc, one absent from doc entirely
mkdir -p "$TMP/case/ch"
printf 'routine line here\nZZVERB-42 the special line\n' > "$TMP/case/ch/chunk-01.txt"
cat > "$TMP/case/case.yml" <<'YML'
case_id: syn-01
chunks_dir: ch
chunk_count: 1
success_criteria:
  deterministic_checks:
    contains_required: [ZZVERB-42, NEVER-IN-DOC-99]
YML
mkdir -p "$TMP/r5"; printf 'found ZZVERB-42 early\n' > "$TMP/r5/notes.txt"
mkdir -p "$TMP/r6"; : > "$TMP/r6/notes.txt"

# --- synthetic case: NO contains_required at all
mkdir -p "$TMP/case2"
cat > "$TMP/case2/case.yml" <<'YML'
case_id: syn-02
chunks_dir: ch
chunk_count: 1
success_criteria:
  deterministic_checks: {}
YML
mkdir -p "$TMP/r7"; printf 'anything\n' > "$TMP/r7/notes.txt"

# --- run
out1=$(python3 "$S" --case "$CASE01" --run-dir "$TMP/r1"); ck "o01 partial notes -> MORE" 0 $?
out2=$(python3 "$S" --case "$CASE01" --run-dir "$TMP/r2"); ck "o01 all verbatim -> COMPLETE(7)" 7 $?
out3=$(python3 "$S" --case "$CASE06" --run-dir "$TMP/r3"); ck "o06 one identifier -> MORE" 0 $?
out4=$(python3 "$S" --case "$CASE06" --run-dir "$TMP/r4"); ck "o06 both identifiers -> COMPLETE(7)" 7 $?
out5=$(python3 "$S" --case "$TMP/case/case.yml" --run-dir "$TMP/r5"); ck "syn absent-doc token excluded, verbatim present -> COMPLETE(7)" 7 $?
out6=$(python3 "$S" --case "$TMP/case/case.yml" --run-dir "$TMP/r6"); ck "syn verbatim missing -> MORE" 0 $?
out7=$(python3 "$S" --case "$TMP/case2/case.yml" --run-dir "$TMP/r7"); ck "no contains_required -> MORE (safe default)" 0 $?

# --- status json written with correct missing list
python3 - "$TMP/r1/evidence-status.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
missing = set(d["missing"])
assert "EV-8802" in missing and "EV-9917" in missing, f"missing wrong: {d['missing']}"
assert d["complete"] is False
print("ok   - status json missing list correct")
PY
[ $? -eq 0 ] && pass=$((pass+1)) || { fail=$((fail+1)); echo "FAIL - status json"; }

echo "----------------------------------------"
echo "pass=$pass fail=$fail"
[ "$fail" -eq 0 ]
