#!/usr/bin/env python3
"""Lint probe battery: reference fixes pass own checks; ids unique;
categories balanced 4x5; every fixture entry has a truth-derivation
comment. Exit 1 on any violation.
"""
import re
import sys
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REA = EXP.parent / "reasoning-enhancer" / "scripts"
sys.path.insert(0, str(REA))
from check_lib import run_checks  # noqa: E402

CASES = EXP / "cases/probe"
FIXTURES = EXP / "fixtures/probe-fixes.yml"
S2_CASES = EXP / "cases/stage2-train"
S2_FIXTURES = EXP / "fixtures/stage2-train-fixes.yml"


REQUIRED_META = ("hops", "robust", "prio", "format_weight",
                 "archetype", "provenance")
ARCHETYPES = {"env": 11, "stack": 3, "clause": 2, "string": 1}


def lint_stage2(fails):
    fixes = yaml.safe_load(S2_FIXTURES.read_text()) or {}
    raw = S2_FIXTURES.read_text()
    ids, arch, hops_hist = [], {}, {}
    robust_n = prio_n = fw_n = 0
    for p in sorted(S2_CASES.glob("case-*.yml")):
        data = yaml.safe_load(p.read_text()) or {}
        cid = data.get("case_id") or p.stem
        ids.append(cid)
        meta = data.get("rea_plus") or {}
        for k in REQUIRED_META:
            if k not in meta:
                fails.append(f"{cid}: rea_plus missing '{k}'")
        a = meta.get("archetype")
        arch[a] = arch.get(a, 0) + 1
        h = meta.get("hops")
        hops_hist[h] = hops_hist.get(h, 0) + 1
        robust_n += bool(meta.get("robust"))
        prio_n += bool(meta.get("prio"))
        fw_n += (meta.get("format_weight") or 0) >= 1

        ref = fixes.get(cid)
        if ref is None:
            fails.append(f"{cid}: no stage2 reference fix")
            continue
        checks = (data.get("success_criteria") or {}).get(
            "deterministic_checks") or {}
        if not checks:
            fails.append(f"{cid}: empty deterministic_checks")
            continue
        if isinstance(ref, str):
            res = run_checks(ref, checks)
            if not res["passed"]:
                fails.append(f"{cid}: reference FAILS own checks: "
                             f"{res['failures']}")
        block = re.search(
            rf"^{re.escape(cid)}:(.*?)(?=^\w[\w-]*:|\Z)", raw,
            re.M | re.S)
        seg = block.group(0) if block else ""
        if "# truth:" not in seg:
            fails.append(f"{cid}: stage2 fixture missing '# truth:'")

    if len(ids) != len(set(ids)):
        fails.append("stage2: duplicate case_ids")
    if arch != ARCHETYPES:
        fails.append(f"stage2 archetype counts {arch} != {ARCHETYPES}")
    n_all = len(ids)
    if robust_n < n_all // 4:
        fails.append(f"stage2 robust {robust_n} < {n_all // 4}")
    if prio_n < n_all // 6:
        fails.append(f"stage2 prio {prio_n} < {n_all // 6}")
    if fw_n < n_all * 2 // 3:
        fails.append(f"stage2 format_weight>=1 on {fw_n} < {n_all * 2 // 3}")
    missing = set(fixes) - set(ids)
    if missing:
        fails.append(f"stage2 fixtures without cases: {sorted(missing)}")
    return len(ids), hops_hist, robust_n, prio_n


def main():
    fails = []
    fixes = yaml.safe_load(FIXTURES.read_text()) or {}
    raw = FIXTURES.read_text()

    ids = []
    cats = {}
    for p in sorted(CASES.glob("case-*.yml")):
        data = yaml.safe_load(p.read_text()) or {}
        cid = data.get("case_id") or p.stem
        ids.append(cid)
        cat = data.get("category", "?")
        cats[cat] = cats.get(cat, 0) + 1

        ref = fixes.get(cid)
        if ref is None:
            fails.append(f"{cid}: no reference fix in fixture")
            continue
        checks = (data.get("success_criteria") or {}).get(
            "deterministic_checks") or {}
        if not checks:
            fails.append(f"{cid}: empty deterministic_checks")
            continue
        res = run_checks(ref, checks)
        if not res["passed"]:
            det = [d for d in res.get("details", [])
                   if isinstance(d, dict) and not d.get("passed")]
            fails.append(f"{cid}: reference FAILS own checks: {det}")

        block = re.search(
            rf"^{re.escape(cid)}:(.*?)(?=^\w[\w-]*:|\Z)", raw,
            re.M | re.S)
        seg = block.group(0) if block else ""
        if "# truth:" not in seg:
            fails.append(f"{cid}: fixture missing '# truth:' derivation")

    if len(ids) != len(set(ids)):
        fails.append("duplicate case_ids")
    expect = {c: 4 for c in ("FMT", "DRV", "LOG", "PLN", "AUD")}
    if cats != expect:
        fails.append(f"category balance broken: {cats} != {expect}")
    missing = set(fixes) - set(ids)
    if missing:
        fails.append(f"fixtures without cases: {sorted(missing)}")

    n2, hops_hist, rob, pri = lint_stage2(fails)

    if fails:
        print("LINT FAIL:")
        for f in fails:
            print(f"  - {f}")
        return 1
    print(f"LINT PASS: {len(ids)} probe cases + {n2} stage2 cases "
          f"(hops={dict(sorted(hops_hist.items()))}, robust={rob}, "
          f"prio={pri}); all references green, truths present")
    return 0


if __name__ == "__main__":
    sys.exit(main())
