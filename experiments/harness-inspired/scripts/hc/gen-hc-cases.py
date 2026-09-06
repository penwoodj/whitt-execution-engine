#!/usr/bin/env python3
"""Compose 100 hc-* cases from 25 archetypes x 4 variants.
Single-source truth: archetype computes truth from params; prose embeds same
params; provenance emitted inline; contract block carries lint formulas.

Usage: python3 gen-hc-cases.py [--out-dir ../cases] [--only N[,M..]]
"""
import argparse
import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

import hc_styles as S
from hc_archetypes_a import ARCHETYPES_A
from hc_archetypes_b import ARCHETYPES_B
from hc_hints import enrich_question

ALL = ARCHETYPES_A + ARCHETYPES_B
assert len(ALL) == 25, f"need 25 archetypes, got {len(ALL)}"

PAD = [
    "the desk has one drawer and the drawer holds the reconciliation stapler, that is the entire administrative apparatus, everything else is the rules and the counts they produce",
    "old-timers here say the counts are the easy part and the boundaries are the career, they are right more often than the confident ones",
    "the standing joke is that every rule in this binder maps to a specific bad night, the joke is funny because it audits true",
    "when two ledgers disagree the one derived from the rules wins, the other one becomes a footnote in somebody's weekly note",
    "nothing in this desk's output survives on vibes, every figure traces to a clause, every clause traces to a night someone remembers",
    "the counts close at the same minute every night regardless of when the work actually finished, the clock is part of the contract",
    "trainees get told the same thing on day one, slow is fine, wrong is expensive, re-read the boundary clauses before you commit",
    "the binder's margins carry annotations from people who left, the annotations argue with each other, the rules themselves do not",
    "there is no partial credit at this desk, a count is right or it is tomorrow's incident review",
    "the numbers go upstream to people who act on them within the hour, that urgency is why the recheck tail exists on every report",
    "every so often someone proposes simplifying the rules, the simplification always loses exactly one boundary clause, the proposal always dies",
    "the audit trail matters more than speed, a fast wrong number has cost this desk more than any slow right one",
]

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
    opener = pick(S.OPENERS, i, 1).format(sysname=sysname, role=role, shift=shift)
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
        paras.insert(len(paras) - 3, PAD[pi % len(PAD)])
        words = sum(len(p.split()) for p in paras)
        pi += 3
    if words > 2000:
        raise SystemExit(f"case {i} over word budget: {words}")

    prompt = "\n\n".join(paras)
    heavy_arch = i % 25 in (3, 4, 5, 10, 13, 17, 19, 21, 23, 24)
    hops = (5 if heavy_arch else 3) + (v >= 2)
    hops = min(max(hops, 3), 6)
    difficulty = "heavy" if hops >= 5 else "light"

    case = {
        "case_id": f"hc-{i+1:02d}",
        "category": "field-ops",
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
            "entities": [
                {"id": e["id"],
                 "question": enrich_question(a["arch"], e["id"], e["question"]),
                 "answer": e["answer"]}
                for e in a["entities"]
            ],
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
        p.write_text(yaml.safe_dump(case, sort_keys=False, allow_unicode=True, width=200))
    print(f"wrote {len(idxs)} cases to {out}")


if __name__ == "__main__":
    main()
