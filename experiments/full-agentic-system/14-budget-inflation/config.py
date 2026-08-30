"""Exp 14 — budget-inflation (budget-control atom). Standard engine tasks
under an explicit token budget; a pre-execution inflation probe (S57 CBE
analog: entropy of short samples) picks CHEAP lane vs ESCALATE lane vs
ABORT. Retry exhaustion must escalate FRESH (discard failed context,
S57 contamination: -34.8pp) — sample2 stage is fresh-prompted.

Truth = engine truth (quality gate); routing/inflation outcomes are
artifact-measured (conf-*, budget-*, ledger). Inflation class by hash:
LOW (probe clean, L lane) / HIGH (probe noisy, H lane) / STUCK
(solve delta-faulted, escalation path win).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import hsh, std_case, scen_buckets  # noqa: E402

_ENG = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]
NEEDS = ["E1", "E3", "E6"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"bi-{n:02d}"
                h = hsh(cid, "bi")
                if h % 5 == 4:
                    note = ("budget posture: the night is thin, the desk "
                            "cannot afford deep retries, one clean pass "
                            "or a fresh hand-off, no mid-chain reworking")
                else:
                    note = ("budget posture: the desk holds reserve for "
                            "one escalation, spend it only after a lane "
                            "fails clean-through, never to polish a "
                            "passing answer")
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=note,
                    derivation=f"{engine}/{variant}#{idx}"))
    return cases


SHAPES = {
    "H": [("probe", "fmt", 60, ""),
          ("budget", "fmt", 120, "plan"),
          ("solve", "fmt", 600, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("wait", "fmt", 300, "wait"),
          ("extract2", "fmt", 200, "extract"),
          ("sample2", "deep", 1200, "cot"),
          ("extract3", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("probe", "fmt", 60, ""),
          ("budget", "fmt", 120, "plan"),
          ("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
}
UNCHECKED = {"probe", "budget", "judge"}
PRIORS = {"extract": "solve", "wait": "solve", "extract2": "wait",
          "sample2": "extract2", "extract3": "sample2"}
CHECK_HOOK_STAGES = {"extract", "extract2", "extract3"}


def SCENARIO(cid, n_h_checked):
    h = hsh(cid, "bi")
    if h % 5 < 2:            # LOW inflation: win cheap at solve
        return 0
    if h % 5 == 2:           # mid: win after wait
        return min(2, n_h_checked - 1)
    return min(4, n_h_checked - 1)  # HIGH/stuck: fresh escalation wins


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        cid = c["case_id"]
        h = hsh(cid, "bi")
        cls = ("LOW" if h % 5 < 2 else
               "MID" if h % 5 == 2 else "HIGH")
        out[cid] = {"inflation_class": cls,
                    "thin" if h % 5 == 4 else "reserve": True}
    return out


_FAULT = {}
_n = 0
for _e in _ENG:
    for _v in ("H", "L"):
        for _i in range(3):
            _n += 1
            _cid = f"bi-{_n:02d}"
            if hsh(_cid, "bi") % 5 >= 3:
                _FAULT[f"{_cid}:solve"] = "delta"
FAULT_PLAN = _FAULT
