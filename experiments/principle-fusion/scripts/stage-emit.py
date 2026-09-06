#!/usr/bin/env python3
"""Stage gate + prompt emitter for the principle-fusion workflow.

Prints quoted "PASS" when the case already has a passing check
(GWT routes on, stage skipped); "SKIP" when a required prior is
missing; otherwise prints the stage prompt.

Fusion principles embodied here:
  P1  byte-exact style prefixes (from fusion_lib.STYLE_PREFIX)
  P8  first solve sees the FULL prompt (worked example included —
      N8 digit disjointness keeps it non-confusing); downstream
      stages see the deterministic digest only
  E10 digest cuts expansion boilerplate + worked example
  N2  extract stages harvest prior text check-before-echo; the model
      is never asked to re-emit what already checks green
  N6  failure hints are leak-safe: check id + observed + question,
      never the expected value
  N5  judge stage is a BLIND lane: contract digest + latest answer
      only — no truth, no failure details, no routing power
"""
import argparse
import json
import re
import sys
from pathlib import Path

import yaml

from fusion_lib import (STYLE_PREFIX, digest, harvester, has_pass,
                        latest_check, leak_safe_hint,
                        ledger_fingerprint, ledger_outcome)

PHRASE_MAP = [
    ("Answer with just the number, no comma separators.",
     "number only, no commas."),
    ("Answer with just the number.", "number only."),
    ("Output nothing else", "nothing else"),
    ("No extra keys, no trailing text.",
     "no extra keys, no trailing text."),
    ("Emit ONLY this JSON (any key order)",
     "emit ONLY this JSON, any key order"),
    ("Answer the task directly and completely.",
     "answer directly, completely."),
]


def caveman(text):
    """Deterministic info-preserving compression: mapped phrases,
    article/filler strip. Numbers, units, negations, constraints
    untouched."""
    for a, b in PHRASE_MAP:
        text = text.replace(a, b)
    text = re.sub(r"\b(?:the|a|an)\b\s+", "", text, flags=re.I)
    text = re.sub(
        r"\b(?:please|basically|actually|simply)\b\s+", "",
        text, flags=re.I)
    text = re.sub(r"in order to\b", "to", text, flags=re.I)
    text = re.sub(r"[ \t]+", " ", text)
    return text.strip()


def quote(s):
    return '"' + s + '"'


def newest_answer(run_dir, cid):
    """Newest non-empty answer artifact for the case (judge/plan/
    replan scaffolding excluded — they are not answers)."""
    skip = ("-judge", "-plan", "-replan")
    for f in sorted(Path(run_dir).glob(f"ans-{cid}-*.txt"),
                    key=lambda p: p.stat().st_mtime, reverse=True):
        if any(s in f.name for s in skip):
            continue
        if f.stat().st_size:
            return f.read_text()
    return None


def prior_answer(run_dir, cid, prior_csv):
    """First existing answer among the comma list (order = trust)."""
    for st in [x for x in prior_csv.split(",") if x]:
        f = Path(run_dir) / f"ans-{cid}-{st}.txt"
        if f.exists() and f.stat().st_size:
            return f.read_text()
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--stage", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--style", default="",
                    choices=["", "plan", "cot", "replan", "wait",
                             "verify", "extract", "judge"])
    ap.add_argument("--prior", default="",
                    help="comma stages whose ans-{cid}-{stage}.txt "
                         "outputs are embedded as context")
    args = ap.parse_args()

    c = yaml.safe_load(Path(args.case).read_text())
    cid = c["case_id"]
    checks = (c.get("success_criteria") or {}
              ).get("deterministic_checks") or {}
    req = checks.get("contains_required") or []
    json_spec = checks.get("json_exact")
    prompt = (c.get("prompt") or "").strip()
    aux = (c.get("auxiliary") or "").strip()

    cj = latest_check(args.run_dir, cid)
    if cj is not None and cj.get("passed"):
        ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
        print(quote("PASS"))
        return

    # --- N2: extract stages harvest prior text, check-before-echo ---
    if args.style == "extract":
        prior_text = prior_answer(args.run_dir, cid, args.prior)
        if not prior_text:
            ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
            print(quote("SKIP"))
            return
        cleaned = harvester(prior_text, checks)
        if cleaned:
            ledger_outcome(args.run_dir, cid, args.stage, "RUN")
            print(cleaned)
            return
        hint = ""
        if req:
            hint = (" Answer must contain, verbatim: "
                    + ", ".join(f"'{r}'" for r in req) + ".")
        form = ""
        if json_spec:
            form = (" Output EXACTLY this JSON object, values filled "
                    "in correctly: " + json_spec[:200])
        print("SOLUTION TEXT:\n" + prior_text
              + "\n\nFrom that text only:" + hint + form
              + " Nothing else.")
        ledger_outcome(args.run_dir, cid, args.stage, "RUN")
        return

    # --- N5: blind judge lane — contract digest + answer only ---
    if args.style == "judge":
        ans = newest_answer(args.run_dir, cid)
        if not ans:
            ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
            print(quote("SKIP"))
            return
        out = (STYLE_PREFIX["judge"]
               + "\n\nTASK CONTRACT:\n" + caveman(digest(prompt))
               + "\n\nANSWER UNDER REVIEW:\n" + ans[:1500]
               + "\n\nScore strictly against the contract. "
                 "Output ONLY the verdict JSON.")
        ledger_outcome(args.run_dir, cid, args.stage, "RUN")
        ledger_fingerprint(args.run_dir, cid, args.stage, out)
        print(out)
        return

    # --- reasoning stages ---
    parts = []
    if args.style:
        parts.append(STYLE_PREFIX[args.style])
    for st in [x for x in args.prior.split(",") if x]:
        f = Path(args.run_dir) / f"ans-{cid}-{st}.txt"
        if f.exists() and f.stat().st_size:
            label = "PLANNING NOTES" if st in ("plan", "replan") \
                else f"ATTEMPT ({st})"
            parts.append(f"{label}:\n{f.read_text()[:1200]}")
    if args.prior:
        parts.append(
            "From the material above, output ONLY the final answer "
            "in the exact required form. TASK REQUIREMENTS:\n"
            + caveman(digest(prompt)))
    else:
        parts.append(caveman(f"TASK:\n{prompt}"))
    if aux:
        parts.append(f"SOURCE MATERIAL:\n{aux}")
    parts.append("Answer the task directly and completely.")

    if cj is not None and cj.get("failures"):
        parts.append(
            caveman("PREVIOUS ATTEMPT FAILED these deterministic "
                    "checks:\n" + leak_safe_hint(cj))
            + "\nRe-solve carefully and output ONLY the final answer "
              "in the exact required form.")
    out = "\n\n".join(parts)
    ledger_outcome(args.run_dir, cid, args.stage, "RUN")
    ledger_fingerprint(args.run_dir, cid, args.stage, out)
    print(out)


if __name__ == "__main__":
    sys.exit(main())
