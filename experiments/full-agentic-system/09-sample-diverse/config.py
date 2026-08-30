"""Exp 09 — sample-diverse. H1: cross-model resample converts >=30% of
same-model-stuck cases (fusion: cross-model retry beat same-model 100%).

Case design: stuck-set = H cases whose scenario win lands exactly on the
sample2 stage (deep, after fmt wait retry failed). L cases act as
never-stuck controls. The wait stage BEFORE sample2 encodes the
same-model retry that does NOT convert (win strictly after wait).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, hsh, decode_cid  # noqa: E402

NEEDS = ["P2", "E1"]

_STUCK_NOTE = ("if the same head keeps landing one field off, stop "
               "asking the same head: the second opinion is a different "
               "head entirely, same question, fresh priors, no memory "
               "of the first attempt's reasoning, because a head married "
               "to its own story re-tells it, it does not re-think it")

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]


def _stuck(cid, variant):
    # H: 60% stuck-until-sample2 (win 4), 20% early, 20% verify-deep
    # L: never stuck (win 0)
    if variant != "H":
        return False
    return hsh(cid, "sd") % 5 < 3


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"sd-{n:02d}"
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_STUCK_NOTE,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"stuck={_stuck(cid, variant)}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 600, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("wait", "fmt", 400, "wait"),
          ("extract2", "fmt", 200, "extract"),
          ("sample2", "deep", 1200, "sample"),
          ("extract3", "fmt", 200, "extract"),
          ("verify", "think", 2500, "verify"),
          ("extract4", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"extract": "solve", "wait": "extract", "extract2": "wait",
          "extract3": "sample2", "verify": "extract3",
          "extract4": "verify"}
CHECK_HOOK_STAGES = {"extract", "extract2", "extract3", "extract4"}


def SCENARIO(cid, n_h_checked):
    variant = "H" if hsh(cid) % 2 else "L"  # not used; variant from cid
    # recover variant from case parity: sd-NN -> odd NN = H first cycle
    nn = int(cid.split("-")[1])
    variant = "H" if (nn - 1) // 3 % 2 == 0 else "L"
    if _stuck(cid, variant):
        # extract(0) fails, wait(1)=same-model retry fails,
        # sample2 lane: win at extract3 (idx 3)
        return min(3, n_h_checked - 1)
    if variant == "H" and hsh(cid, "deep") % 5 == 4:
        return min(n_h_checked - 1, 5)   # verify-deep rescue
    return 0


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, variant, _ = decode_cid(c["case_id"], 6, 3, _ENG)
        out[c["case_id"]] = {"stuck": _stuck(c["case_id"], variant)}
    return out
