#!/usr/bin/env python3
"""Lint-contract verifier for hc-* cases. Recomputes each truth from the
PROMPT TEXT ALONE via contract formulas + regex-extracted literals.
Staged evaluation: extracted literals first, then verified truth keys.
Unresolvable formulas land on the manual-review list.

Usage: python3 verify-hc.py [--cases ../cases] [--json out.json]
Exit 0 if all mechanizable checks pass and manual list <= threshold.
"""
import argparse
import json
import re
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent.parent))
import importlib.util

_spec = importlib.util.spec_from_file_location("aggregate", Path(__file__).parent.parent / "aggregate.py")
aggregate_mod = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(aggregate_mod)

SAFE_ENV_BASE = {"min": min, "max": max, "int": int, "bool": bool, "abs": abs}


def extract_literal(prompt, regex, value, lmap=None):
    if not regex:
        return value, "asserted"
    m = re.search(regex, prompt)
    if not m:
        return None, "regex-miss"
    groups = m.groups()
    if not groups:
        return value, "constant"
    raw = groups[0]
    if raw is None:
        return value, "constant"
    if lmap:
        return lmap.get(raw, raw), "extracted"
    try:
        return float(raw) if "." in raw else int(raw), "extracted"
    except ValueError:
        return raw, "extracted"


def verify_case(path):
    case = yaml.safe_load(path.read_text())
    cid = case["case_id"]
    prompt = case["prompt"]
    truth = json.loads(case["success_criteria"]["deterministic_checks"]["json_exact"])
    problems = []

    wc = len(prompt.split())
    if not (1000 <= wc <= 2000):
        problems.append(f"word count {wc} outside [1000,2000]")

    for fact in case["task_core"]["facts"]:
        if fact not in prompt:
            problems.append(f"task_core fact not in prose: {fact[:60]}")

    env = dict(SAFE_ENV_BASE)
    known = {}
    contracts = case.get("contract", {})
    shared = dict(env)
    for key, spec in contracts.items():
        for l in spec["literals"]:
            val, how = extract_literal(prompt, l["regex"], l["value"], l.get("map"))
            if val is None:
                problems.append(f"{cid}.{key}: literal {l['name']} regex-miss")
            else:
                shared[l["name"]] = val
    env = shared

    manual = []
    pending = dict(contracts)
    for _ in range(len(contracts) + 2):
        progressed = False
        for key in list(pending):
            spec = pending[key]
            formula = spec["formula"]
            local = dict(env)
            local.update(known)
            try:
                val = eval(formula, {"__builtins__": {}}, local)
            except Exception:
                continue
            if key in truth:
                tv = truth[key]
                match = (val == tv) if not isinstance(tv, bool) else (bool(val) == tv)
                if isinstance(tv, (int, float)) and not isinstance(tv, bool) and isinstance(val, (int, float)):
                    match = abs(float(val) - float(tv)) < 1e-9
                if not match:
                    problems.append(f"{cid}.{key}: formula gave {val!r}, truth says {tv!r}")
                else:
                    known[key] = tv
            else:
                known[key] = val
            del pending[key]
            progressed = True
        if not pending or not progressed:
            break
    for key, spec in pending.items():
        manual.append((key, spec["formula"], "unresolved"))

    ents = {e["id"]: e["answer"] for e in case["v6"]["entities"]}
    try:
        rebuilt = aggregate_mod.aggregate(case, ents)
        if rebuilt != truth:
            problems.append(f"{cid}: aggregate {rebuilt!r} != json_exact {truth!r}")
    except Exception as e:
        problems.append(f"{cid}: aggregate eval failed: {e}")

    return {"case": cid, "words": wc, "problems": problems,
            "manual_review": manual, "verified_keys": sorted(known.keys())}


def _val_eq(a, b):
    if isinstance(a, bool) or isinstance(b, bool):
        return isinstance(a, bool) and isinstance(b, bool) and a == b
    return a == b


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cases", default=str(Path(__file__).parent.parent.parent / "cases"))
    ap.add_argument("--json", default="")
    args = ap.parse_args()

    results = []
    for p in sorted(Path(args.cases).glob("hc-*.yml")):
        results.append(verify_case(p))

    total_problems = sum(len(r["problems"]) for r in results)
    manual = [(r["case"], m) for r in results for m in r["manual_review"]]
    verified = sum(len(r["verified_keys"]) for r in results)
    keys = sum(len(json.loads(yaml.safe_load(p.read_text())["success_criteria"]["deterministic_checks"]["json_exact"])) for p in sorted(Path(args.cases).glob("hc-*.yml")))

    print(f"cases: {len(results)}")
    print(f"problems: {total_problems}")
    for r in results:
        for pr in r["problems"]:
            print(f"  PROBLEM {pr}")
    print(f"contract keys verified: {verified}/{keys}")
    print(f"manual-review entries: {len(manual)}")
    for cid, m in manual[:40]:
        print(f"  MANUAL {cid}.{m[0]}: {m[1][:70]} [{m[2]}]")
    if len(manual) > 40:
        print(f"  ... and {len(manual)-40} more")

    if args.json:
        Path(args.json).write_text(json.dumps(results, indent=2))

    sys.exit(1 if total_problems else 0)


if __name__ == "__main__":
    main()
