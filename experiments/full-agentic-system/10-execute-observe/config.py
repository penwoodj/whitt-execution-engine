"""Exp 10 — execute-observe. H1: observation -> failure-class mapping
recovers >=60% of injected tool faults (self-healing orchestrator 98.8%
vs 93.8% full-replan; ChaosLLM: incorrect/delta hardest).

Case design: workflow adds an 'execute' stage whose spoofed observation
carries an injected fault from FAULT_PLAN. Faults rotate across the 5
classes per case (timeout, unreachable, garble, delta, schema_drift —
S38/S39 taxonomy). Recovery = a later 'observe' classification + retry
stage wins. Slow/no-op faults included via clean controls (no fault).
Stage-emit 'gather' style doubles as the observation formatter.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, hsh  # noqa: E402

NEEDS = ["P10", "N2"]

_OBS_NOTE = ("when a tool observation comes back broken the class is "
             "the clue: silent or hanging means retry once, garbled "
             "means re-read the tail for what survived, a subtle wrong "
             "number means cross-check against the rule that produced "
             "it, and a shape that does not parse means re-emit, do not "
             "repair prose with prose. classify before reacting, the "
             "reaction is per class, never generic")

_ENG = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]
_FAULTS = ["timeout", "unreachable", "garble", "delta", "schema_drift"]


def _fault_for(nn):
    r = hsh(f"eo-{nn:02d}", "fault") % 6
    return None if r == 5 else _FAULTS[r % 5]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"eo-{n:02d}"
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_OBS_NOTE,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"fault={_fault_for(n)}"))
    return cases


SHAPES = {
    "H": [("execute", "fmt", 300, "gather"),
          ("observe", "fmt", 200, "cot"),
          ("solve", "fmt", 600, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("retry", "fmt", 300, "cot"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("execute", "fmt", 200, "gather"),
          ("solve", "fmt", 500, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"solve": "observe", "extract": "solve",
          "retry": "extract", "extract2": "retry"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    nn = int(cid.split("-")[1])
    if _fault_for(nn) is None:
        return 0 if hsh(cid, "w") % 2 else 1   # clean: early win
    # fault: extract(1) sees fault-tainted answer, retry recovers (idx 2)
    return min(2, n_h_checked - 1)


def _build_fault_plan():
    plan = {}
    for nn in range(1, 37):
        f = _fault_for(nn)
        if f:
            plan[f"eo-{nn:02d}:execute"] = f
    return plan


FAULT_PLAN = _build_fault_plan()


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        nn = int(c["case_id"].split("-")[1])
        out[c["case_id"]] = {"fault": _fault_for(nn)}
    return out
