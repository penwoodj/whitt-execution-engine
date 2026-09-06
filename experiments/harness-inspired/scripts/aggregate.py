#!/usr/bin/env python3
"""Deterministic aggregation. Evaluates case v6.aggregate over OBSERVED
entity answers from a check-table result JSON (P2 deterministic offloading:
composition math never touches the model).

Usage: aggregate.py --case FILE --table-result FILE --out FILE
Writes final JSON object to --out. Exit 0 on success, 1 on eval failure.
"""
import argparse
import json
import sys
from pathlib import Path

import yaml


def build_env(case):
    entities = case["v6"]["entities"]
    defaults = {e["id"]: e["answer"] for e in entities}

    def a(eid):
        return defaults[eid]

    return a


def aggregate(case, observed_answers):
    expr = case["v6"]["aggregate"]
    answers = dict(observed_answers)

    def a(eid):
        if isinstance(eid, dict):
            eid = eid["id"]
        return answers[eid]

    env = {"__builtins__": {}, "sum": sum, "len": len, "a": a, "int": int, "bool": bool, "str": str, "min": min, "max": max, "round": round, "float": float}
    for e in case["v6"]["entities"]:
        env[e["id"]] = e["id"]
    env["entities"] = case["v6"]["entities"]
    result = eval(expr, env, dict(env))
    return result


def main():
    p = argparse.ArgumentParser(description="Deterministic table aggregation")
    p.add_argument("--case", required=True)
    p.add_argument("--table-result", required=True, help="check-table result JSON (uses observed answers)")
    p.add_argument("--out", required=True, help="write final JSON here")
    args = p.parse_args()

    try:
        case = yaml.safe_load(Path(args.case).read_text())
        table_result = json.loads(Path(args.table_result).read_text())
        observed = table_result.get("answers", {})
        final = aggregate(case, observed)
        if not isinstance(final, dict):
            raise ValueError(f"aggregate expr must yield dict, got {type(final).__name__}")
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)

    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(final, sort_keys=True) + "\n")
    print(json.dumps(final, sort_keys=True))
    sys.exit(0)


if __name__ == "__main__":
    main()
