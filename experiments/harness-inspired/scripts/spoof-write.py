#!/usr/bin/env python3
"""Spoof inference writer. Writes a canned stage artifact exactly where the
real step output would land, so downstream REAL gate scripts run on it.
Zero network, zero GPU (REA+ spoof-emit pattern, adapted to ha-suite).

Usage: spoof-write.py --artifact PATH --text-file FILE
"""
import argparse
import sys
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--artifact", required=True)
    p.add_argument("--text-file", required=True)
    args = p.parse_args()

    text = Path(args.text_file).read_text()
    target = Path(args.artifact)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text)
    print(f"spoofed:{target}", file=sys.stderr)
    sys.exit(0)


if __name__ == "__main__":
    main()
