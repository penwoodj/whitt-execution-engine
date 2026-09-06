#!/usr/bin/env python3
"""TDD tests for build-retry-feedback.py structured retry feedback.

Run: python3 test_retry_feedback.py  → exit 0 = pass, 1 = fail.
Written BEFORE implementation (watch fail first).
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT = Path(__file__).parent / "build-retry-feedback.py"
FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


def run_feedback(angle, check_dict):
    with tempfile.TemporaryDirectory() as td:
        d = Path(td)
        (d / f"check-angle-{angle}.json").write_text(json.dumps(check_dict))
        r = subprocess.run(
            [sys.executable, str(SCRIPT), str(angle), str(d)],
            capture_output=True, text=True, timeout=15)
        return r


# ── 1. Multi-failure → numbered list + fix hints ──────────────
chk = {
    "angle": 1, "passed": False,
    "checks": {
        "bullet_count": {"value": 0, "min": 3, "max": 3, "passed": False},
        "word_count": {"value": 14, "min": 30, "max": 100, "passed": False},
        "forbidden_phrases": {"found": ["here are the key points"], "passed": False},
        "prompt_leak": {"found": ["text under review"], "passed": False},
    },
}
r = run_feedback(1, chk)
check("exit0", r.returncode == 0, f"rc={r.returncode} stderr={r.stderr[:300]}")
out = r.stdout
check("has_failed_checks_header", "FAILED CHECKS" in out)
check("numbered_items", "1." in out and "4." in out)
check("bullet_hint", "3" in out and "bullet" in out.lower())
check("forbidden_names_phrase", "here are the key points" in out)
check("leak_hint", "only the deliverable" in out.lower() or "no instructions" in out.lower())
check("full_rewrite_instruction", "FULL" in out and "rewrite" in out.lower())
check("no_preamble_chatter", "no preamble" in out.lower())

# ── 2. Per-angle lens line present ────────────────────────────
check("lens_line_angle1", "format auditor" in out.lower())
r3 = run_feedback(3, chk)
check("lens_line_angle3", "requirements tracer" in r3.stdout.lower())
r4 = run_feedback(4, chk)
check("lens_line_angle4", "hallucination hunter" in r4.stdout.lower())

# ── 3. Passing check JSON (defensive, unreachable in practice) ──
ok_chk = {"angle": 1, "passed": True, "checks": {"bullet_count": {"value": 3, "min": 3, "max": 3, "passed": True}}}
r = run_feedback(1, ok_chk)
check("exit0_passing", r.returncode == 0)
check("passing_notes_all_pass", "all checks pass" in r.stdout.lower())

# ── 4. Missing check file (defensive) ─────────────────────────
with tempfile.TemporaryDirectory() as td:
    r = subprocess.run([sys.executable, str(SCRIPT), "2", td],
                       capture_output=True, text=True, timeout=15)
    check("exit0_missing_file", r.returncode == 0, f"rc={r.returncode}")
    check("missing_file_message", "failed" in r.stdout.lower() or "missing" in r.stdout.lower())

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL RETRY-FEEDBACK TESTS PASS")
