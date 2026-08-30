"""Exp 01 — typed-envelope. H1: explicit typed I/O contract in prompt
(keys + value menus + error-class listing) cuts FORMAT failures without
hurting content. 3 presentation conditions (S41 3-condition design):
  A (idx%3==0): plain contract line only
  B (idx%3==1): + envelope note enumerating types per key
  C (idx%3==2): + error-class menu (what a malformed answer means)
Condition recorded via EXTRA_SCENARIO for slice analysis (S37).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets, decode_cid  # noqa: E402

NEEDS = ["N1", "P7"]

_ENV_B = ("envelope note, binding the output shape: every key in the "
          "contract carries exactly one value of the stated kind, counts "
          "and sums are numbers never words, ids are strings exactly as "
          "written, lists hold only what the contract names, and a value "
          "the rules cannot produce is still a well-typed value, an empty "
          "list where a list belongs")

_ENV_C = ("error classes, so a malformed report is diagnosable: a "
          "missing key is FORMAT, a value of the wrong kind is FORMAT, "
          "an id never seen tonight is LEAK, a count that ignores an "
          "event is CONTENT, and none of them are recoverable by "
          "appending prose after the JSON")

_ENG = ["quota", "canary", "epistemic", "backoff", "residency", "preempt"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"te-{n:02d}"
                cond = idx % 3
                extra = ""
                if cond == 1:
                    extra = _ENV_B
                elif cond == 2:
                    extra = _ENV_B + ". " + _ENV_C
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=extra,
                    derivation=f"{engine}/{variant}#{idx} cond={'ABC'[cond]}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"extract": "solve"}
CHECK_HOOK_STAGES = {"extract"}


def SCENARIO(cid, n_h_checked):
    # spread across solve-win (0) and extract-win (1); judge never gates
    return scen_buckets(cid, n_h_checked, [5, 3])


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, _, idx = decode_cid(c["case_id"], 6, 3, _ENG)
        out[c["case_id"]] = {"condition": "ABC"[idx % 3]}
    return out
