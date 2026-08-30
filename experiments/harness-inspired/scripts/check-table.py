#!/usr/bin/env python3
"""Firing-table gate. Validates ENTITY rows against case v6.entities.

Leak-safe: reports missing/wrong cell ids and OBSERVED values; never
writes expected values into evidence.

Usage: check-table.py --artifact FILE --case FILE [--only ID] [--out FILE]
Exit 0 all present+match, 1 otherwise.
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from table_lib import parse_table, explicit_ids


def typed_equal(a, b):
    if isinstance(a, bool) or isinstance(b, bool):
        return isinstance(a, bool) and isinstance(b, bool) and a == b
    if isinstance(a, dict) and isinstance(b, dict):
        return a.keys() == b.keys() and all(typed_equal(a[k], b[k]) for k in a)
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return float(a) == float(b)
    if isinstance(a, str) and isinstance(b, str):
        return a == b
    return False


def run_gate(artifact_path, case_path, only=None):
    text = Path(artifact_path).read_text()
    case = yaml.safe_load(Path(case_path).read_text())
    entities = case["v6"]["entities"]
    if only:
        entities = [e for e in entities if e["id"] == only]
    observed = parse_table(text)

    known = {e["id"] for e in entities}
    extra = sorted((set(observed) - known) & explicit_ids(text))

    cells, missing, wrong = [], [], []
    for e in entities:
        eid = e["id"]
        if eid not in observed:
            missing.append(eid)
            cells.append({"id": eid, "present": False, "match": False, "observed": None})
            continue
        obs = observed[eid]
        match = typed_equal(obs, e["answer"])
        if not match:
            wrong.append({"id": eid, "observed": obs})
        cells.append({"id": eid, "present": True, "match": match, "observed": obs})

    ok = not missing and not wrong and not extra
    result = {
        "pass": ok,
        "missing": missing,
        "wrong": [{"id": w["id"], "observed": w["observed"]} for w in wrong],
        "extra": extra,
        "cells": cells,
        "answers": observed,
        "total": len(entities),
        "passed": len(entities) - len(missing) - len(wrong),
    }
    return result


def main():
    p = argparse.ArgumentParser(description="Firing-table completeness+value gate")
    p.add_argument("--artifact", required=True)
    p.add_argument("--case", required=True)
    p.add_argument("--only", help="check a single entity id")
    p.add_argument("--out", help="write result JSON to file")
    args = p.parse_args()

    try:
        result = run_gate(args.artifact, args.case, args.only)
    except FileNotFoundError as e:
        result = {"pass": False, "missing": [], "wrong": [], "cells": [],
                  "answers": {}, "total": 0, "passed": 0,
                  "error": f"artifact_missing: {e}"}

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    sys.exit(0 if result["pass"] else 1)


if __name__ == "__main__":
    main()
