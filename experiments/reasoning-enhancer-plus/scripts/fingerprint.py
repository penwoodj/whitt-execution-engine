#!/usr/bin/env python3
"""Fingerprint stdin prompt: append ts/case/step/sha1/head to run-dir
fingerprint.log. Chained after emit-probe: emit-probe ... | fingerprint.py
"""
import argparse
import hashlib
import sys
import time
from pathlib import Path


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--step", required=True)
    ap.add_argument("--run-dir", required=True)
    args = ap.parse_args()

    text = sys.stdin.read()
    sha = hashlib.sha1(text.encode()).hexdigest()
    head = " ".join(text.split())[:80]
    line = (f"{int(time.time())} {args.case} {args.step} "
            f"sha1={sha} head={head}\n")

    fp = Path(args.run_dir) / "fingerprint.log"
    with fp.open("a") as fh:
        fh.write(line)

    sys.stdout.write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
