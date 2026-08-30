"""Exp 04 — intent-classify (lane routing). H1: a deterministic pre-gate
routes >=85% of cases to the correct LIGHT/HEAVY lane. H2: DECLINE class
(cannot-answer, S47) recognized instead of forced.

Case design: every case carries surface complexity cues consistent with
its engine+variant (H variants get multi-constraint language + more
events; L variants get short clean lists). Decline overlay: idx%5==4
cases get an extra expansion note stating a referenced table is absent
from tonight's materials — the honest report DECLINES the missing piece
while still reporting the computable fields (truth = engine truth with
an added "declined_field" only for those cases is NOT valid — instead
the decline note is a decoy: the table is absent but the events in the
prompt suffice, correct lane = HEAVY because the absence forces care.
True DECLINE control lives in EXTRA_SCENARIO for the live phase where a
separate decline-truth set (05 corpora NOT_FOUND) provides the honest-
refusal measurement. Here routing accuracy is the metric, judged by
win-depth under each lane in spoof.)
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets, decode_cid  # noqa: E402

NEEDS = ["N3", "E6"]

_HEAVY_NOTE = ("tonight stacks: more than one rule family in play, an "
               "expiry or rollback that lands between events, and a "
               "verdict that depends on walking every event, the heavy "
               "night, budget the care")

_LIGHT_NOTE = ("tonight is the quiet kind: one rule family, a short "
               "list, one pass and a recheck, the light night, do not "
               "overpay")

_DECLINE_DECOY = ("a reference table the old shift used is not in "
                  "tonight's materials; the events on the page carry "
                  "everything the rules need, absence of the table is "
                  "not absence of the facts")

_ENG = ["quota", "canary", "epistemic", "backoff", "residency", "preempt"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"ic-{n:02d}"
                extra = _HEAVY_NOTE if variant == "H" else _LIGHT_NOTE
                decoy = idx % 5 == 4
                if decoy:
                    extra += ". " + _DECLINE_DECOY
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=extra,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"lane={variant} decoy={decoy}"))
    return cases


SHAPES = {
    "H": [("plan", "think", 900, "plan"),
          ("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("verify", "think", 2500, "verify"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"plan", "judge"}
PRIORS = {"extract": "solve", "verify": "extract", "extract2": "verify"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    # H-heavy spread incl. deep verify wins; L cases mostly win at 0
    return scen_buckets(cid, n_h_checked, [4, 3, 2, 1], salt="ic")


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, variant, idx = decode_cid(c["case_id"], 6, 3, _ENG)
        out[c["case_id"]] = {
            "intended_lane": variant,
            "decline_decoy": idx % 5 == 4}
    return out
