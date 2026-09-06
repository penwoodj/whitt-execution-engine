#!/usr/bin/env python3
"""Replay re-scorer. Re-evaluates archived per-stage artifact snapshots
against the CURRENT case definition — no model, no rerun (AgentAssay
deterministic-replay pattern).

Enables: edit case ground truth -> instantly re-score yesterday's runs.
Regression harness for case edits (the hi-str-01 typo lesson).

Usage: replay-rescore.py --run-dir DIR --case FILE [--out FILE]
Exit 0 if replay completed (report written), 1 on config error.
"""
import argparse
import importlib.util
import json
import sys
from pathlib import Path

import yaml

SCRIPTS_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPTS_DIR))


def _load_module(stem):
    spec = importlib.util.spec_from_file_location(stem, SCRIPTS_DIR / f"{stem}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


STAGES = ["solve", "fix_1", "fix_2"]


def replay(run_dir, case_path):
    run_gate = _load_module("check-table").run_gate
    aggregate = _load_module("aggregate").aggregate

    case = yaml.safe_load(Path(case_path).read_text())

    report = {"case": case.get("case_id"), "stages": {}, "final": None}
    expected = case["success_criteria"]["deterministic_checks"].get("json_exact")
    expected_obj = json.loads(expected) if expected else None

    for stage in STAGES:
        snap = Path(run_dir) / f"artifact.{stage}.txt"
        if not snap.is_file():
            report["stages"][stage] = {"present": False}
            continue
        res = run_gate(str(snap), case_path)
        report["stages"][stage] = {
            "present": True,
            "pass": res["pass"],
            "missing": res["missing"],
            "wrong_ids": [w["id"] for w in res["wrong"]],
            "extra": res.get("extra", []),
        }
        if res["pass"]:
            try:
                final = aggregate(case, res["answers"])
                report["final"] = {
                    "from_stage": stage,
                    "match": final == expected_obj,
                    "value": final,
                }
            except Exception as e:  # noqa: BLE001
                report["final"] = {"from_stage": stage, "error": str(e)}

    return report


def main():
    p = argparse.ArgumentParser(description="Replay re-scorer")
    p.add_argument("--run-dir", required=True)
    p.add_argument("--case", required=True)
    p.add_argument("--out", help="write report JSON to file")
    args = p.parse_args()

    try:
        report = replay(args.run_dir, args.case)
    except (FileNotFoundError, yaml.YAMLError) as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    sys.exit(0)


if __name__ == "__main__":
    main()
