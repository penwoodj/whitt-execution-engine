#!/usr/bin/env python3
"""REA case linter.

Usage:
  test-cases.py [--cases-dir ../cases] [--fixtures ../fixtures/reference-fixes.yml]

For every case file: schema of required keys, known check keys only, draft
FAILS at least one of its own checks, reference fix PASSES all checks
(v8 learning 7 — a case whose checks accept the broken draft or reject the
reference is a broken case, never weaken the checks). Also verifies the
workflow file exists with its runtime placeholders. Exit 1 on any failure.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

from check_lib import KNOWN_CHECK_KEYS, run_checks

REQUIRED_KEYS = {"case_id", "difficulty", "domain", "prompt", "draft_response",
                 "success_criteria"}
DIFFICULTIES = {"easy", "medium", "hard"}


def lint_case(case, reference_text, failures, case_path, no_draft_check=False):
    cid = case.get("case_id", case_path.name)
    missing = REQUIRED_KEYS - set(case)
    if missing:
        failures.append(f"{cid}: missing keys {sorted(missing)}")
        return
    if case["difficulty"] not in DIFFICULTIES:
        failures.append(f"{cid}: bad difficulty {case['difficulty']}")
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    unknown = set(checks) - KNOWN_CHECK_KEYS
    if unknown:
        failures.append(f"{cid}: unknown check keys {sorted(unknown)}")

    draft_result = run_checks(case["draft_response"], checks)
    if not no_draft_check:
        if draft_result["passed"]:
            failures.append(f"{cid}: draft PASSES own checks — case cannot test enhancement")
        draft_fail_count = draft_result["subchecks_total"] - draft_result["subchecks_passed"]
        if draft_fail_count < 2:
            failures.append(
                f"{cid}: draft fails only {draft_fail_count} subcheck(s) — need >= 2 "
                "(fails-without property must be decisive, not one flipped subcheck)")

    if reference_text is None:
        failures.append(f"{cid}: no reference fix in fixtures")
    else:
        ref_result = run_checks(reference_text, checks)
        if not ref_result["passed"]:
            for f in ref_result["failures"]:
                failures.append(f"{cid}: reference fix fails {f['check']}: {f['detail']}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cases-dir", default=str(Path(__file__).parent.parent / "cases"))
    ap.add_argument("--fixtures", default=str(Path(__file__).parent.parent / "fixtures" / "reference-fixes.yml"))
    ap.add_argument("--workflow", default=str(Path(__file__).parent.parent / "workflows" / "rea-v1.yml"))
    ap.add_argument("--no-draft-check", action="store_true",
                    help="benchmark mode: draft comes from a live 9B baseline run, "
                         "so the authoring-time draft check is skipped")
    args = ap.parse_args()

    fixtures_path = Path(args.fixtures)
    references = yaml.safe_load(fixtures_path.read_text()) if fixtures_path.exists() else {}

    case_files = sorted(Path(args.cases_dir).glob("case-*.yml"))
    if not case_files:
        print("FAIL: no case files found")
        return 1

    failures = []
    for cf in case_files:
        case = yaml.safe_load(cf.read_text())
        cid = str(case.get("case_id", cf.name))
        before = len(failures)
        lint_case(case, references.get(case.get("case_id")), failures, cf,
                  no_draft_check=args.no_draft_check)
        status = "ok" if len(failures) == before else "FAIL"
        print(f"lint {cf.name}: {cid} "
              f"({case.get('difficulty')}/{case.get('domain')}) {status}")

    wf = Path(args.workflow)
    if not wf.exists():
        failures.append(f"workflow missing: {wf}")
    else:
        wf_text = wf.read_text()
        for ph in ("__REPO_ROOT__", "__OUTPUT_DIR__", "__CASE_FILE__"):
            if ph not in wf_text:
                failures.append(f"workflow missing placeholder {ph}")

    if failures:
        print("\n".join(f"FAIL: {f}" for f in failures))
        return 1
    print(f"OK: {len(case_files)} cases lint clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
