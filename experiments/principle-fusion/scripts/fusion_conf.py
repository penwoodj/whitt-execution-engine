#!/usr/bin/env python3
"""N3 confidence pre-gate for the principle-fusion LIVE workflow.

Computes Shannon entropy over a first-token logprob array (harvested
from the engine's probe log for this case), routes HEAVY vs LIGHT
lane. Spoof runs never call this (no model, no logprobs) — spoof
lanes come from the hash scenario; live lanes come from here.

Prints the quoted routing token the GWT gate consumes:
  "HEAVY" | "LIGHT"
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

from fusion_lib import conf_route, entropy_from_logprobs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--case", required=True)
    ap.add_argument("--logprobs-file", default=None,
                    help="JSON array of logprobs (live probe). "
                         "Missing/empty file -> --default lane.")
    ap.add_argument("--default", default="LIGHT",
                    choices=["LIGHT", "HEAVY"])
    ap.add_argument("--threshold", type=float, default=0.8)
    args = ap.parse_args()

    c = yaml.safe_load(Path(args.case).read_text())
    cid = c["case_id"]

    logprobs = []
    src = "default"
    if args.logprobs_file:
        p = Path(args.logprobs_file)
        if p.exists() and p.stat().st_size:
            try:
                data = json.loads(p.read_text())
                if isinstance(data, list) and data:
                    logprobs = [float(x) for x in data]
                    src = "probe"
            except Exception:
                src = "default"

    if src == "probe":
        ent = entropy_from_logprobs(logprobs)
        lane = conf_route(logprobs, args.threshold)
    else:
        ent = None
        lane = args.default

    out = Path(args.run_dir) / f"conf-{cid}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps({
        "case_id": cid, "entropy": ent, "lane": lane,
        "source": src, "threshold": args.threshold}, indent=1))
    print(f'"{lane}"')


if __name__ == "__main__":
    sys.exit(main())
