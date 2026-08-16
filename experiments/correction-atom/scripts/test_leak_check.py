#!/usr/bin/env python3
"""TDD tests for prompt-leak detection in check_angle_lib.

Run: python3 test_leak_check.py  → exit 0 = pass, 1 = fail.
Written BEFORE implementation (watch fail first).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import check_prompt_leak, run_checks_for_angle, PROMPT_LEAK_MARKERS

FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


# ── 1. Clean text → no leaks ──────────────────────────────────
clean = "- The validator uses 8 criteria.\n- Below 45 triggers a fix.\n- 3 failures escalate.\n"
check("clean_text_no_leak", check_prompt_leak(clean) == [],
      f"got {check_prompt_leak(clean)}")

# ── 2. Each marker detected ───────────────────────────────────
leaky_samples = [
    "TEXT UNDER REVIEW:\nfoo",
    "Emit corrected text only.",
    "PERSONA: FORMAT AUDITOR",
    "PRIOR ATTEMPT PASSED. Copy input verbatim.",
    "PRIOR ATTEMPT FAILED. Reason: x",
    "LENS: Surface structure",
    "RECOGNIZE:",
    "FIX RULES:",
    "TASK SPEC + AUXILIARY:",
    "FIRST WORD MUST MATCH TASK SPEC FORMAT",
]
for s in leaky_samples:
    leaks = check_prompt_leak(s)
    check(f"leak_detected[{s[:30]!r}]", len(leaks) > 0, f"got {leaks}")

# ── 3. Case-insensitive detection ─────────────────────────────
check("case_insensitive", len(check_prompt_leak("text under review")) > 0)

# ── 4. run_checks_for_angle applies prompt_leak ALWAYS ────────
case = {"case_id": "t", "deterministic_checks": {"bullet_count_min": 1}}
res = run_checks_for_angle(1, case, clean)
check("prompt_leak_check_present", "prompt_leak" in res["checks"],
      f"checks={list(res['checks'].keys())}")
check("prompt_leak_passes_clean", res["checks"].get("prompt_leak", {}).get("passed") is True)
check("overall_pass_clean", res["passed"] is True)

res_leak = run_checks_for_angle(1, case, "TEXT UNDER REVIEW\n- a\n- b\n")
check("prompt_leak_fails_leaky", res_leak["checks"]["prompt_leak"]["passed"] is False)
check("overall_fail_leaky", res_leak["passed"] is False)
check("leak_reported_in_found",
      "text under review" in res_leak["checks"]["prompt_leak"]["found"])

# ── 5. REGRESSION: real v5 leaked file must be caught ─────────
# exp-v5-case-011/after-angle-4.txt PASSED v5 checks despite echoing
# the whole retry prompt. New check must flag it.
reg_path = Path(__file__).parent.parent / "results" / "exp-v5-case-011" / "after-angle-4.txt"
if reg_path.exists():
    leaked = reg_path.read_text()
    leaks = check_prompt_leak(leaked)
    check("regression_v5_case011_angle4", len(leaks) > 0,
          f"real leaked file not flagged")
else:
    print("  SKIP regression_v5_case011_angle4 (fixture missing)")

# ── 6. Empty marker list sanity ───────────────────────────────
check("markers_nonempty", len(PROMPT_LEAK_MARKERS) >= 8)

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL LEAK-CHECK TESTS PASS")
