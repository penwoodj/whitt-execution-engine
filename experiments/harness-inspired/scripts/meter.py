#!/usr/bin/env python3
"""Token meter. Estimates tokens (chars/4) of an artifact file and appends
a trace entry. Devin ACU pattern: per-stage spend in the replayable ledger.

Usage: meter.py --artifact FILE --stage NAME --trace FILE [--case ID]
"""
import argparse
import datetime
import json
import math
import sys
from pathlib import Path


def estimate_tokens(text):
    return math.ceil(len(text) / 4)


def main():
    p = argparse.ArgumentParser(description="Token meter")
    p.add_argument("--artifact", required=True)
    p.add_argument("--stage", required=True)
    p.add_argument("--trace", required=True)
    p.add_argument("--case", default="unknown")
    args = p.parse_args()

    try:
        text = Path(args.artifact).read_text()
    except FileNotFoundError:
        text = ""

    entry = {
        "gate": "meter",
        "stage": args.stage,
        "tokens_est": estimate_tokens(text),
        "case": args.case,
        "timestamp": datetime.datetime.utcnow().isoformat() + "Z",
    }
    with open(args.trace, "a") as f:
        f.write(json.dumps(entry) + "\n")
    print(entry["tokens_est"])
    sys.exit(0)


if __name__ == "__main__":
    main()
