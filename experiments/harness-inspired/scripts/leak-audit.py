#!/usr/bin/env python3
"""Mechanical leak auditor. Proves feedback artifacts never contain
expected values (F8/P17 enforcement as guaranteed invariant).

Scans run-dir feedback files for literals of expected-but-NOT-observed
entity answers. Model's own observed values are legitimate feedback;
expected values for cells the model got wrong are leaks.

Usage: leak-audit.py --run-dir DIR --case FILE [--table-result FILE]
Exit 0 clean, 1 leak found, 2 config error.
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

FEEDBACK_PATTERNS = ("feedback",)


def strong_literals(case):
    """Expected literals that are textually unambiguous leak signals.

    Small bare numbers (e.g. "0", "3") collide with observed values and
    incidental text — weak signals, excluded. Dicts, lists, booleans,
    strings and numbers >= 100 are strong.
    """
    lits = {}
    for e in case["v6"]["entities"]:
        ans = e["answer"]
        if isinstance(ans, bool) or isinstance(ans, (dict, list)) or isinstance(ans, str):
            lits[json.dumps(ans, sort_keys=True)] = e["id"]
        elif isinstance(ans, (int, float)) and abs(ans) >= 100:
            lits[json.dumps(ans, sort_keys=True)] = e["id"]
    exact = case["success_criteria"]["deterministic_checks"].get("json_exact")
    if exact:
        try:
            lits[json.dumps(json.loads(exact), sort_keys=True)] = "json_exact"
        except (json.JSONDecodeError, ValueError):
            pass
    return lits


def observed_literals(table_result):
    obs = {}
    for w in table_result.get("wrong", []):
        obs[json.dumps(w.get("observed"), sort_keys=True)] = w["id"]
    for c in table_result.get("cells", []):
        if c.get("present") and c.get("observed") is not None:
            obs[json.dumps(c.get("observed"), sort_keys=True)] = c["id"]
    return obs


def feedback_files(run_dir):
    return [p for p in Path(run_dir).glob("*.txt") if any(k in p.name for k in FEEDBACK_PATTERNS)]


def audit(run_dir, case_path, table_result):
    case = yaml.safe_load(Path(case_path).read_text())
    exp = strong_literals(case)
    obs = observed_literals(table_result)

    leaks = []
    for fb in feedback_files(run_dir):
        text = fb.read_text()
        for lit, eid in exp.items():
            if lit in text and lit not in obs:
                leaks.append({"file": fb.name, "entity": eid, "literal": lit})
    return leaks


def main():
    p = argparse.ArgumentParser(description="Leak auditor")
    p.add_argument("--run-dir", required=True)
    p.add_argument("--case", required=True)
    p.add_argument("--table-result", help="check-table result JSON (default: <run-dir>/ha-table-result.json)")
    args = p.parse_args()

    tr_path = Path(args.table_result) if args.table_result else Path(args.run_dir) / "ha-table-result.json"
    try:
        table_result = json.loads(tr_path.read_text())
        leaks = audit(args.run_dir, args.case, table_result)
    except (FileNotFoundError, json.JSONDecodeError, yaml.YAMLError) as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(2)

    report = {"leaks": leaks, "clean": not leaks, "scanned": [p.name for p in feedback_files(args.run_dir)]}
    print(json.dumps(report, indent=2))
    sys.exit(0 if not leaks else 1)


if __name__ == "__main__":
    main()
