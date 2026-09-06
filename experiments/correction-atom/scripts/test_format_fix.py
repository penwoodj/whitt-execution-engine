#!/usr/bin/env python3
"""TDD tests for format-fix.py (deterministic angle-1) and emit-state.py.

Run: python3 test_format_fix.py → exit 0 = pass.
Written BEFORE implementation (watch fail first).

Deterministic fixer contract:
- drop lines containing a forbidden phrase when line is short (<15 words)
- keep in-bullet forbidden phrases (factual fixes belong to LLM angles)
- bullet cases: extract bullet block when enough bullets survive
- never crash; degenerate input → unchanged text out
"""
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).parent
FIX = SCRIPTS / "format-fix.py"
STATE = SCRIPTS / "emit-state.py"
FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


def run_fix(case: dict) -> str:
    with tempfile.TemporaryDirectory() as td:
        cf = Path(td) / "case.yml"
        cf.write_text(yaml.dump(case))
        out = Path(td) / "out.txt"
        r = subprocess.run([sys.executable, str(FIX), str(cf), str(out)],
                           capture_output=True, text=True, timeout=15)
        if r.returncode != 0:
            return f"__RC{r.returncode}:{r.stderr[:120]}"
        return out.read_text()


CASE_001_LIKE = {
    "task_spec": "Summarize into exactly 3 bullets. Bullets only.",
    "auxiliary": "source paragraph text about iteration discipline protocols",
    "broken_output": (
        "The output does not clearly state whether it meets requirements.\n"
        "The following points address the key aspects as requested:\n\n"
        "- The generator must pass validator scoring with 8 criteria on a 50-point scale.\n"
        "- Three consecutive failed fix cycles result in escalation to the user.\n"
        "- Validator evidence is required for acceptance of any deliverable.\n"
    ),
    "deterministic_checks": {
        "bullet_count_min": 3, "bullet_count_max": 3,
        "forbidden_phrases": ["does not clearly state", "the output", "as requested"],
    },
}

CASE_011_LIKE = {
    "task_spec": "3 bullets about the validator scoring system.",
    "auxiliary": "the validator scores workflows and escalates after three failures",
    "broken_output": (
        "Here are the key points about the validator:\n\n"
        "- The validator uses 8 criteria scored on a 100-point scale.\n"
        "- After 4 failed fix cycles the system escalates to a human.\n"
        "- The validator's verdict can be overridden by the generator if it\n"
        "  provides sufficient justification for doing so right now.\n\n"
        "Hope this helps! Let me know if you need more details.\n"
    ),
    "deterministic_checks": {
        "bullet_count_min": 3, "bullet_count_max": 3,
        "forbidden_phrases": ["here are the key points", "hope this helps", "can be overridden"],
    },
}

CASE_002_LIKE = {
    "task_spec": "One line per task: T<n>: <CATEGORY>",
    "broken_output": (
        "ORIGINAL DRAFT:\n\n"
        "T1: TRANSFORM\nT2: VALIDATE\nT3: TRANSFORM\nT4: ROUTE\nT5: EXECUTE\n\n"
        "Note: T1 might be debatable, see analysis below.\n"
    ),
    "deterministic_checks": {
        "forbidden_phrases": ["original draft", "might be debatable"],
    },
}


# ── format-fix.py ─────────────────────────────────────────────
out = run_fix(CASE_001_LIKE)
lines = [l for l in out.splitlines() if l.strip()]
check("001_preamble_dropped", "does not clearly state" not in out.lower())
check("001_as_requested_dropped", "as requested" not in out.lower())
check("001_three_bullets", sum(1 for l in lines if l.strip().startswith("-")) == 3, repr(out[:200]))
check("001_bullet_content_kept", "50-point scale" in out)

out = run_fix(CASE_011_LIKE)
lines = [l for l in out.splitlines() if l.strip()]
check("011_greeting_dropped", "here are the key points" not in out.lower())
check("011_closing_dropped", "hope this helps" not in out.lower())
check("011_bullets_kept", sum(1 for l in lines if l.strip().startswith("-")) == 3)
check("011_inbullet_forbidden_kept", "can be overridden" in out.lower(),
      "in-bullet factual content must survive to LLM angles")

out = run_fix(CASE_002_LIKE)
check("002_draft_line_dropped", "original draft" not in out.lower())
check("002_note_dropped", "might be debatable" not in out.lower())
check("002_tlines_kept", all(f"T{i}:" in out for i in range(1, 6)))

no_op = run_fix({"task_spec": "x", "broken_output": "plain text stays\n",
                 "deterministic_checks": {}})
check("no_checks_unchanged", no_op.strip() == "plain text stays", repr(no_op))

# ── emit-state.py ─────────────────────────────────────────────
with tempfile.TemporaryDirectory() as td:
    d = Path(td)
    cf = d / "case.yml"
    cf.write_text(yaml.dump(CASE_001_LIKE))
    r = subprocess.run([sys.executable, str(STATE), str(cf), str(d)],
                       capture_output=True, text=True, timeout=15)
    check("state_rc0_fallback", r.returncode == 0, r.stderr[:120])
    check("state_has_text_section", "TEXT UNDER REVIEW:" in r.stdout)
    check("state_has_spec_section", "TASK SPEC + AUXILIARY" in r.stdout)
    check("state_fallback_broken", "50-point scale" in r.stdout)

    (d / "after-angle-1.txt").write_text("NEW BEST TEXT MARKER")
    r = subprocess.run([sys.executable, str(STATE), str(cf), str(d)],
                       capture_output=True, text=True, timeout=15)
    check("state_uses_latest_angle", "NEW BEST TEXT MARKER" in r.stdout)
    check("state_not_broken_anymore", "The generator must pass" not in r.stdout)

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL FORMAT-FIX TESTS PASS")
