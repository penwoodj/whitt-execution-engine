#!/usr/bin/env python3
"""TDD tests for v8 verifier upgrades: number-word normalization,
emit-state escalation memory, select-best hard-fail gate.

Run: python3 test_v8_features.py → exit 0 = pass.
Written BEFORE implementation (watch fail first).
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPTS = Path(__file__).parent
sys.path.insert(0, str(SCRIPTS))
from check_angle_lib import run_checks_for_angle, normalize_numbers

FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


# ── 1. Number-word normalization ──────────────────────────────
check("norm_three", normalize_numbers("after three retries") == "after 3 retries",
      repr(normalize_numbers("after three retries")))
check("norm_ordinals", "3rd" in normalize_numbers("the third failed retry"))
check("norm_fifth", "5th" in normalize_numbers("on the fifth attempt"))
check("norm_passthrough_digits", normalize_numbers("45 points") == "45 points")

CASE_NUM = {
    "case_id": "n",
    "broken_output": "job parked after three retries",
    "deterministic_checks": {"contains_required": ["3rd failed retry"]},
}
res = run_checks_for_angle(1, CASE_NUM, "job parked after the 3rd failed retry")
check("required_matches_ordinal_digit", res["passed"], str(res["checks"].get("contains_required")))
res = run_checks_for_angle(1, CASE_NUM, "job parked after the third failed retry")
check("required_matches_ordinal_word", res["passed"], str(res["checks"].get("contains_required")))

# ── 2. emit-state.py escalation memory ────────────────────────
def run_state(case_dict, files: dict) -> str:
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        cf = d / "case.yml"
        import yaml
        cf.write_text(yaml.dump(case_dict))
        for name, content in files.items():
            (d / name).write_text(content)
        r = subprocess.run([sys.executable, str(SCRIPTS / "emit-state.py"), str(cf), str(d)],
                           capture_output=True, text=True, timeout=15)
        assert r.returncode == 0, r.stderr
        return r.stdout

base_case = {"task_spec": "3 bullets", "auxiliary": "validator scores 8 criteria on 50-point scale",
             "broken_output": "- old\n- text\n- here\n"}
out = run_state(base_case, {
    "after-angle-1.txt": "- candidate text from angle one\n- b\n- c\n",
    "check-angle-1.json": json.dumps({
        "angle": 1, "passed": False,
        "checks": {"bullet_count": {"value": 3, "min": 3, "max": 3, "passed": True},
                   "forbidden_phrases": {"found": ["as requested"], "passed": False}}}),
})
check("mem_section_present", "PRIOR ANGLE FAILURES" in out)
check("mem_lists_failed_only", "forbidden_phrases" in out and "bullet_count: value=3" not in out)
check("mem_has_fix_hint", "delete these phrases" in out.lower() or "greeting" in out.lower())
check("mem_shows_current_text", "candidate text from angle one" in out)

out2 = run_state(base_case, {
    "after-angle-1.txt": "- a\n- b\n- c\n",
    "check-angle-1.json": json.dumps({"angle": 1, "passed": True, "checks": {}}),
})
check("mem_absent_when_all_pass", "PRIOR ANGLE FAILURES" not in out2)

# ── 3. select-best.py hard-fail gate ──────────────────────────
def run_select(files: dict):
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        for name, content in files.items():
            (d / name).write_text(content)
        r = subprocess.run([sys.executable, str(SCRIPTS / "select-best.py"), str(d)],
                           capture_output=True, text=True, timeout=15)
        assert r.returncode == 0, r.stderr
        meta = json.loads((d / "select-best.json").read_text())
        final = (d / "final-corrected.txt").read_text() if (d / "final-corrected.txt").exists() else None
        return meta, final

HARD_FAIL = {"angle": 2, "passed": False, "checks": {
    "bullet_count": {"value": 3, "min": 3, "max": 3, "passed": True},
    "prompt_leak": {"found": ["text under review"], "passed": False}}}
SOFT_FAIL = {"angle": 2, "passed": False, "checks": {
    "bullet_count": {"value": 2, "min": 3, "max": 3, "passed": False},
    "word_count": {"value": 99, "min": 10, "max": 30, "passed": False}}}

meta, final = run_select({
    "after-angle-2.txt": "leaky candidate\n",
    "check-angle-2.json": json.dumps(HARD_FAIL),
})
check("hardfail_mode", meta.get("mode") == "hard_fail_input_fallback", str(meta))
check("hardfail_final_empty_or_flagged", final is None or meta.get("passed") is False)

meta, final = run_select({
    "after-angle-2.txt": "soft partial candidate\n",
    "check-angle-2.json": json.dumps(SOFT_FAIL),
})
check("softfail_still_ships_partial", final == "soft partial candidate\n".rstrip("\n") or final == "soft partial candidate\n")
check("softfail_mode_best_partial", meta.get("mode") == "best_partial", str(meta))

meta, final = run_select({
    "after-angle-2.txt": "good passing output\n",
    "check-angle-2.json": json.dumps({"angle": 2, "passed": True, "checks": {}}),
})
check("pass_not_gated", meta.get("mode") == "first_pass" and final == "good passing output\n")

meta, final = run_select({
    "after-angle-2.txt": "leaky\n",
    "check-angle-2.json": json.dumps(HARD_FAIL),
    "after-angle-3.txt": "clean later angle\n",
    "check-angle-3.json": json.dumps({"angle": 3, "passed": True, "checks": {}}),
})
check("later_pass_beats_earlier_hardfail", meta.get("mode") == "first_pass" and final == "clean later angle\n")

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL V8-FEATURE TESTS PASS")
