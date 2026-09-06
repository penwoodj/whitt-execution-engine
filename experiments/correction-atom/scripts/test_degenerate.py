#!/usr/bin/env python3
"""TDD tests for degenerate-output + source-grounding detection.

Run: python3 test_degenerate.py → exit 0 = pass, 1 = fail.
Written BEFORE implementation (watch fail first).

Trigger: v6 case-011 live run — 0.5B emitted word salad that PASSED all
structural checks (3 garbage bullets, 43 garbage words) and early-exit
shipped it in 6s. Verifier must reject degenerate/ungrounded output.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import check_degenerate, run_checks_for_angle

FAILURES = []


def check(name, cond, detail=""):
    if cond:
        print(f"  PASS {name}")
    else:
        print(f"  FAIL {name} {detail}")
        FAILURES.append(name)


CLEAN_CASE = {
    "case_id": "t",
    "auxiliary": "The validator scores each workflow on 8 criteria using a "
                 "50-point scale. Below 45 triggers a fix cycle. After 3 "
                 "failed cycles the system escalates to a human reviewer.",
    "broken_output": "- The validator uses 8 criteria on a 100-point scale.\n- After 4 cycles escalate.\n",
    "deterministic_checks": {"bullet_count_min": 3, "bullet_count_max": 3},
}

GOOD_TEXT = ("- The validator scores each workflow on 8 criteria using a 50-point scale.\n"
             "- A score below 45 triggers a fix cycle.\n"
             "- After 3 failed cycles the system escalates to a human reviewer.\n")

GARBAGE = ("def\n\n- The - 8005 8  80000  8000080008      val the val for el, el\n\n"
           "-ell\n\n- 8008080000800000000000000000000el\n\nval el\n- manual\n-8\n-\n-")


# ── 1. Good corrected text passes ─────────────────────────────
r = check_degenerate(GOOD_TEXT, CLEAN_CASE)
check("good_text_passes", r["passed"], f"got {r}")

# ── 2. Real v6 garbage fails ──────────────────────────────────
r = check_degenerate(GARBAGE, CLEAN_CASE)
check("garbage_fails", not r["passed"], f"got {r}")
check("garbage_reason_present", len(r.get("reasons", [])) > 0)

# ── 3. Ungrounded paraphrase (valid structure, invented content) ──
UNGROUND = ("- Apples grow on trees in autumn.\n- The kitchen sells fish on Fridays.\n"
            "- My neighbor plays violin at midnight.\n")
r = check_degenerate(UNGROUND, CLEAN_CASE)
check("ungrounded_fails", not r["passed"], f"got {r}")

# ── 4. Word longer than 25 chars flagged ──────────────────────
r = check_degenerate("- a" + "x" * 40 + " b c\n- normal line here\n- another normal line\n", CLEAN_CASE)
check("long_token_flagged", any("long" in x for x in r.get("reasons", [])), f"got {r}")

# ── 5. run_checks_for_angle integrates degenerate check ───────
res = run_checks_for_angle(1, CLEAN_CASE, GOOD_TEXT)
check("degenerate_in_checks", "degenerate" in res["checks"], f"checks={list(res['checks'])}")
check("overall_pass_good", res["passed"] is True)

res_g = run_checks_for_angle(1, CLEAN_CASE, GARBAGE)
check("overall_fail_garbage", res_g["passed"] is False)
check("degenerate_check_fails_garbage", res_g["checks"]["degenerate"]["passed"] is False)

# ── 6. REGRESSION: v6 case-011 run-1 shipped garbage ──────────
# Static excerpt (run dir was overwritten by the fixed rerun).
reg = Path(__file__).parent / "fixtures" / "garbage-v6-case011.txt"
if reg.exists():
    import yaml
    case = yaml.safe_load((Path(__file__).parent.parent / "cases" / "case-011.yml").read_text())
    r = check_degenerate(reg.read_text(), case)
    check("regression_v6_case011_final", not r["passed"], f"real garbage output accepted: {r}")
else:
    print("  SKIP regression_v6_case011_final (fixture missing)")

# ── 7. Case without auxiliary → grounding skipped, structure checks only ──
no_aux = {"case_id": "t", "broken_output": "x", "deterministic_checks": {}}
r = check_degenerate(GOOD_TEXT, no_aux)
check("no_auxiliary_tolerated", r["passed"], f"got {r}")

print()
if FAILURES:
    print(f"FAILED: {len(FAILURES)} → {FAILURES}")
    sys.exit(1)
print("ALL DEGENERATE-CHECK TESTS PASS")
