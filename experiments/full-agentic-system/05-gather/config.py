"""Exp 05 — gather. H1: gate-driven 2-iteration evidence accumulation
beats single-pass and 5-iteration on local models (agentic-RAG ablation:
2 iterations capture 95% of 5's gains).

Case design: engine events moved OUT of the core into a source-material
corpus (aux) with decoy blocks; fact position in corpus varies by idx
(early/mid/late) per Lost-in-the-Middle. Core = instructions only, so
digest digit-preservation is checked against instruction digits
(task_digits overridden to []). NOT_FOUND controls: idx%6==5 corpora
lack one referenced id; the honest report lists it under "not_found"
(truth overridden: engine truth + that id in not_found list = field is
dropped from its natural key and named instead) — computed by rule
"referenced id absent from corpus -> not_found", never typed.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets, hsh, decode_cid  # noqa: E402

NEEDS = ["P8", "N2"]

_DECOYS = [
    " unrelated note: the snack fund ledger closed at 9 credits, "
    "3 people owe 1 each, nobody audited it",
    " weather desk: overnight lows near 4 degrees, the backup chiller "
    "cycled twice, no thresholds were involved",
    " parking log: 17 badges seen, gate 2 stuck for 11 minutes, "
    "resolved without escalation",
    " mail room: two crates of 5 shelves each arrived, storage said "
    "no space was reserved, they went back",
    " test rig: fan curve settled at 33 percent, noise fine, the "
    "engineer wrote 1 page of notes",
]

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]


def _corpus(core, idx, missing):
    """Wrap engine core events in decoy blocks; optionally drop a fact."""
    text = core
    if missing:
        # strip the trailing id list sentence (events sentence w/ ids)
        cut = text.rfind(". ")
        text = text[:cut] + "."
    blocks = []
    pos = idx % 3
    for d in _DECOYS:
        blocks.append(d.strip())
    ins = min(pos, len(blocks))
    blocks.insert(ins, " TONIGHT'S EVENTS, verbatim: " + text.strip())
    if missing:
        blocks.append(" cross-check note: one id referenced by the rules "
                      "has no entry anywhere in tonight's materials, name "
                      "it under not_found instead of guessing")
    return "\n".join(blocks)


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(4):
                n += 1
                cid = f"ga-{n:02d}"
                g = E.gen(engine, variant, idx)
                missing = (idx % 6 == 5)
                aux = _corpus(g["core"], idx, missing)
                truth = dict(g["truth"])
                if missing:
                    ids = g.get("tkeys", "").split(",") if g.get("tkeys") \
                        else []
                    first_id = ids[0].strip() if ids else "r1"
                    truth = {"not_found": [first_id]}
                header = ("you are on rotation tonight, "
                          f"{engine} duty, and the morning report only "
                          "reads JSON. the event list sits further down "
                          "this same page among unrelated desk notes, "
                          "gather it, every event, before any rule fires")
                core_instr = (
                    "tonight's rules are the ones this post always "
                    "carries; the events themselves are recorded in the "
                    "events section of this same page, mixed in among "
                    "unrelated desk notes from other nights. gather the "
                    "events from this page verbatim, apply the rules "
                    "this station is bound to, and answer from the "
                    "gathered evidence alone, one pass to collect, one "
                    "pass to check nothing was missed")
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    aux=aux, header=header, core_override=core_instr,
                    task_digits=[] if missing else None,
                    truth_override=truth if missing else None,
                    derivation=(f"{engine}/{variant}#{idx} "
                                f"pos={'EML'[idx % 3]} "
                                f"nf={missing}")))
    return cases


SHAPES = {
    "H": [("gather", "fmt", 500, "gather"),
          ("solve", "fmt", 700, "cot"),
          ("gather2", "fmt", 500, "gather"),
          ("solve2", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("gather", "fmt", 400, "gather"),
          ("solve", "fmt", 500, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"solve": "gather", "solve2": "gather2",
          "extract": "solve2"}
CHECK_HOOK_STAGES = {"extract"}


def SCENARIO(cid, n_h_checked):
    # wins spread: single-pass (0) vs second-iteration (2) — the A/B
    return scen_buckets(cid, n_h_checked, [3, 1, 4], salt="ga")


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, _, idx = decode_cid(c["case_id"], 6, 4, _ENG)
        out[c["case_id"]] = {
            "corpus_pos": "EML"[idx % 3],
            "not_found": idx % 6 == 5}
    return out
