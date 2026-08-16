#!/usr/bin/env python3
"""TDD tests for select-best.py ratchet selector.

Run: python3 test_select_best.py  → exit 0 = pass, 1 = fail.
Written BEFORE implementation (watch fail first).
"""
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT = Path(__file__).parent / "select-best.py"
FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


def write_fixture(dirpath, angle, passed, subchecks_passed, text):
    (dirpath / f"after-angle-{angle}.txt").write_text(text)
    checks = {f"c{i}": {"passed": (i < subchecks_passed)} for i in range(3)}
    (dirpath / f"check-angle-{angle}.json").write_text(
        json.dumps({"angle": angle, "passed": passed, "checks": checks}))


def run_select(dirpath):
    r = subprocess.run(
        [sys.executable, str(SCRIPT), str(dirpath)],
        capture_output=True, text=True, timeout=30)
    return r


# ── 1. First passing angle wins (ratchet) ─────────────────────
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    write_fixture(d, 1, False, 1, "angle1 partial")
    write_fixture(d, 2, True, 3, "angle2 GOOD")
    write_fixture(d, 3, True, 3, "angle3 also-good-later")
    r = run_select(d)
    check("exit0_first_pass", r.returncode == 0, f"rc={r.returncode} stderr={r.stderr[:200]}")
    final = (d / "final-corrected.txt").read_text() if (d / "final-corrected.txt").exists() else None
    check("first_pass_selected", final == "angle2 GOOD", f"final={final!r}")
    meta = json.loads((d / "select-best.json").read_text()) if (d / "select-best.json").exists() else {}
    check("meta_mode_first_pass", meta.get("mode") == "first_pass", f"meta={meta}")
    check("meta_angle_2", meta.get("selected_angle") == 2)

# ── 2. No passing → best partial (most subchecks), tie → earliest ──
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    write_fixture(d, 1, False, 1, "angle1 weak")
    write_fixture(d, 2, False, 3, "angle2 strongest partial")
    write_fixture(d, 3, False, 3, "angle3 tied partial LATER")
    r = run_select(d)
    check("exit0_best_partial", r.returncode == 0)
    final = (d / "final-corrected.txt").read_text()
    check("best_partial_selected", final == "angle2 strongest partial", f"final={final!r}")
    meta = json.loads((d / "select-best.json").read_text())
    check("meta_mode_best_partial", meta.get("mode") == "best_partial", f"meta={meta}")

# ── 3. Missing files tolerated ────────────────────────────────
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    write_fixture(d, 3, False, 2, "only angle3 exists")
    r = run_select(d)
    check("exit0_missing_ok", r.returncode == 0)
    check("missing_others_ok", (d / "final-corrected.txt").read_text() == "only angle3 exists")

# ── 4. Empty dir → no crash, no final written (or empty) ──────
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    r = run_select(d)
    check("exit0_empty_dir", r.returncode == 0, f"rc={r.returncode}")

# ── 5. Idempotent: rerun overwrites cleanly ───────────────────
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    write_fixture(d, 1, True, 3, "angle1 good v1")
    run_select(d)
    (d / "after-angle-1.txt").write_text("angle1 good v2")
    run_select(d)
    check("idempotent_rerun", (d / "final-corrected.txt").read_text() == "angle1 good v2")

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL SELECT-BEST TESTS PASS")
