#!/usr/bin/env python3
"""Emit probe prompt to stdout (workflow bookmark source) + optional file.

Usage: emit-probe.py --case CASE.yml [--out FILE]
Prints: "TASK:\n<prompt>\n\n[auxiliary as SOURCE MATERIAL:\n<aux>]\n\n
Answer the task directly and completely."
"""
import argparse
import sys
from pathlib import Path

import yaml


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--out")
    ap.add_argument("--directive", default="",
                    help="soft switch line prepended before TASK (e.g. /no_think)")
    args = ap.parse_args()

    case = yaml.safe_load(Path(args.case).read_text())
    prompt = (case.get("prompt") or "").strip()
    aux = (case.get("auxiliary") or "").strip()

    parts = []
    if args.directive:
        parts.append(args.directive)
    parts.append(f"TASK:\n{prompt}")
    if aux:
        parts.append(f"SOURCE MATERIAL:\n{aux}")
    parts.append("Answer the task directly and completely.")
    out = "\n\n".join(parts)

    if args.out:
        Path(args.out).write_text(out)
    print(out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
