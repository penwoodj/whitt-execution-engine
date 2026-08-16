#!/usr/bin/env python3
"""Case-authoring lint: every case must be solvable and non-trivial.

For each cases/case-NNN.yml + matching reference fix in
scripts/fixtures/reference-fixes.yml:
  1. broken_output must FAIL its own deterministic checks
  2. reference fix must PASS all checks

Usage: test-cases.py [case_dir] → exit 0 = all valid
"""
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import run_checks_for_angle

FIXTURES = Path(__file__).parent / "fixtures" / "reference-fixes.yml"


def main():
    case_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent.parent / "cases"
    fixes = yaml.safe_load(FIXTURES.read_text())
    bad = []
    for case_file in sorted(case_dir.glob("case-*.yml")):
        cid = case_file.stem.replace("case-", "")
        case = yaml.safe_load(case_file.read_text())
        if cid not in fixes:
            bad.append((cid, "no reference fix in fixture"))
            continue
        r_broken = run_checks_for_angle(1, case, case.get("broken_output", ""))
        r_fix = run_checks_for_angle(1, case, fixes[cid])
        if r_broken["passed"]:
            bad.append((cid, "broken_output PASSES its own checks (case trivial)"))
        if not r_fix["passed"]:
            failed = [n for n, c in r_fix["checks"].items()
                      if isinstance(c, dict) and not c.get("passed", True)]
            bad.append((cid, f"reference fix FAILS: {failed}"))
    for cid, why in bad:
        print(f"  BAD case-{cid}: {why}")
    if bad:
        print(f"\n{len(bad)} problem(s) across {len(list(case_dir.glob('case-*.yml')))} cases")
        sys.exit(1)
    print(f"ALL {len(list(case_dir.glob('case-*.yml')))} CASES VALID")


if __name__ == "__main__":
    main()
