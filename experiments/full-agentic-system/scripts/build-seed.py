#!/usr/bin/env python3
"""Build a merged best-of seed dir from prior live runs of an experiment.

Usage: build-seed.py <exp> [--runs live] [--out runs/seed-best]
Union across runs/*: each ans-*.txt / check-*.json / scenario.json keeps
its newest-mtime version. Retries seeded from this dir skip every stage
that already passed and get hints for every stage that failed.
"""
import argparse
import shutil
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("exp")
    ap.add_argument("--runs", default="live")
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    expdir = FAS / a.exp
    srcs = sorted(p for p in (expdir / "runs").iterdir()
                  if p.is_dir() and not p.name.startswith("seed-"))
    if not srcs:
        sys.exit(f"no run dirs under {expdir}/runs")
    out = Path(a.out) if a.out else expdir / "runs" / "seed-best"
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    n = 0
    for src in srcs:
        for f in src.iterdir():
            if not f.is_file() or f.name.endswith(".log"):
                continue
            tgt = out / f.name
            if not tgt.exists() or (f.stat().st_mtime >
                                    tgt.stat().st_mtime):
                shutil.copy2(f, tgt)
                n += 1
    print(f"{out}: merged {n} files from {len(srcs)} runs")


if __name__ == "__main__":
    main()
