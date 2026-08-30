#!/usr/bin/env python3
"""Compose 100 hd-* cases from 25 extra-hard archetypes x 4 variants.
Mirrors gen-hc-cases.py conventions: single-source truth, inline provenance,
contract block carries lint formulas, prompt band 1000-2000 words.

Usage: python3 gen-hd-cases.py [--out-dir ../../cases] [--only N[,M..]]
"""
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

import hd_styles as S
from hd_archetypes_a import ARCHETYPES_A
from hd_archetypes_b import ARCHETYPES_B

ALL = ARCHETYPES_A + ARCHETYPES_B
assert len(ALL) == 25, f"need 25 archetypes, got {len(ALL)}"

HEAVY_ARCHS = {
    "quota_proration", "disk_quota_conv", "escrow_ladder", "rebate_tiers",
    "ring_rotation_2cyc", "canary_percent", "backoff_scale", "overbook_bump",
    "triage_2hop", "sla_pause_clock", "accrual_diff", "label_surgery",
    "checksum_weighted",
}

OPEN_SPEC = "report your findings as one json object, output only this json with exactly these keys: {keys}"


def pick(pool, i, salt):
    return pool[(i * 7 + salt) % len(pool)]


def build_case(i):
    arch_fn = ALL[i % 25]
    v = i // 25
    a = arch_fn(v)
    sysname = pick(S.SYSNAMES, i, 3)
    role = pick(S.ROLES, i, 5)
    shift = pick(S.SHIFTS, i, 11)
    dname = pick(S.DNAMES, i, 13)
    opener = S.OPENERS[i % len(S.OPENERS)].format(sysname=sysname, role=role.strip(), shift=shift)
    tail = pick(S.TAILS, i, 2)
    distract = pick(S.DISTRRACTORS, i, 4).format(dname=dname, sysname=sysname)
    connect = pick(S.CONNECT, i, 6)

    truth = a["truth"]
    keys = ", ".join(f'"{k}": {fmt_placeholder(val)}' for k, val in truth.items())
    spec = OPEN_SPEC.format(keys=keys)

    paras = [opener + ", " + connect + "."]
    paras.extend(a["body"])
    paras.append("the rules of the desk, as they stand tonight: " +
                 " ".join(f"{idx}. {r}." for idx, r in enumerate(a["rules"], 1)))
    paras.append("the facts of record: " + " ".join(a["facts"]) + ".")
    paras.append(distract + ".")
    paras.append(spec + ".")
    paras.append(tail + ".")

    words = sum(len(p.split()) for p in paras)
    target = 1020 + (i * 37) % 700
    pi = i
    while words < target:
        paras.insert(len(paras) - 3, S.PAD[pi % len(S.PAD)])
        words = sum(len(p.split()) for p in paras)
        pi += 3
    if words > 2000:
        raise SystemExit(f"case {i} over word budget: {words}")

    prompt = "\n\n".join(paras)
    heavy = a["arch"] in HEAVY_ARCHS
    hops = (5 if heavy else 4) + (v >= 2)
    hops = min(max(hops, 4), 7)
    difficulty = "heavy" if heavy else "light"

    case = {
        "case_id": f"hd-{i+1:02d}",
        "category": "field-ops-hard",
        "difficulty": difficulty,
        "hops": hops,
        "archetype": a["arch"],
        "variant": v,
        "domain": a["domain"],
        "objective": f"derive {', '.join(truth.keys())} from the stated rules",
        "prompt": prompt,
        "auxiliary": "all facts needed are in the prompt itself",
        "provenance": "\n".join(a["prov"]) + f"\narchetype={a['arch']} variant={v} seed={i}",
        "success_criteria": {
            "deterministic_checks": {
                "json_exact": json.dumps(truth, sort_keys=False),
                "forbidden_phrases": ["i cannot", "sorry", "as an ai"],
            }
        },
        "task_core": {
            "rules": a["rules"],
            "facts": a["facts"],
            "output_spec": {k: type(val).__name__ for k, val in truth.items()},
        },
        "contract": {k: {"formula": f, "literals": [
            {"name": t[0], "regex": t[1], "value": t[2], **({"map": t[3]} if len(t) > 3 else {})}
            for t in lits]}
                     for k, (f, lits) in a["contract"].items()},
        "v6": {
            "entities": [e for e in a["entities"]],
            "aggregate": a["aggregate"],
        },
    }
    return case


def fmt_placeholder(val):
    if isinstance(val, bool):
        return "true-or-false"
    if isinstance(val, float):
        return "F"
    if isinstance(val, int):
        return "N"
    return "S-or-empty"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default=str(Path(__file__).parent.parent.parent / "cases"))
    ap.add_argument("--only", default="")
    args = ap.parse_args()

    out = Path(args.out_dir)
    out.mkdir(parents=True, exist_ok=True)
    idxs = [int(x) - 1 for x in args.only.split(",") if x.strip()] or list(range(100))
    for i in idxs:
        case = build_case(i)
        p = out / f"{case['case_id']}.yml"
        p.write_text(yaml.safe_dump(case, sort_keys=False, allow_unicode=True, width=400))
    print(f"wrote {len(idxs)} cases to {out}")


if __name__ == "__main__":
    main()
