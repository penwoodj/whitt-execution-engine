"""Exp 03 — cache-addressing. H1: content-keyed stage results
(sha256(stage|input)) skip recompute on repeat/overlap with ZERO
wrong-reuse across collision traps.

Pairs: cases come in PAIRS (same engine+variant, adjacent idx) that share
the worked example and shape but differ in task digits -> truths differ.
A cache keyed on shape/worked alone would wrongly reuse pair-sibling
answers; a content key (full prompt) must not. Spoof: first-of-pair may
win early, sibling has its OWN cid so has_pass never crosses. EXTRA_SCENARIO
tags pair_id + member. The workflow adds a 'cache' unchecked stage whose
emitted key (stage-emit ledger fingerprint) is the reuse surface.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_mod  # noqa: E402

NEEDS = ["E3", "P10"]

_ENG = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]


def build_cases(E, L):
    cases = []
    n = 0
    pair = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for pairbase in range(3):
                pair += 1
                for member, idx in enumerate((pairbase * 2,
                                               pairbase * 2 + 1)):
                    n += 1
                    cid = f"ca-{n:02d}"
                    cases.append(std_case(
                        E, L, engine, variant, idx, cid, NEEDS,
                        derivation=(f"{engine}/{variant}#{idx} "
                                    f"pair=P{pair:02d}m{member}")))
    return cases


SHAPES = {
    "H": [("cache", "fmt", 30, "extract"),
          ("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("cache", "fmt", 30, "extract"),
          ("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"cache", "judge"}
PRIORS = {"extract": "solve"}
CHECK_HOOK_STAGES = {"extract"}


def SCENARIO(cid, n_h_checked):
    # solve or extract win; cache stage is pre-solve (unchecked, SKIP)
    return scen_mod(cid, n_h_checked, salt="ca") % 2


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        nn = int(c["case_id"].split("-")[1])
        member = (nn - 1) % 2
        pair = f"P{(nn - 1) // 2 + 1:02d}"
        out[c["case_id"]] = {"pair_id": pair,
                             "member": member,
                             "sibling_truth_differs": True}
    return out
