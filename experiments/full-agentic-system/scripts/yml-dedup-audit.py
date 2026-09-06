#!/usr/bin/env python3
"""yml structural dedup audit for generated workflows (ruleset §6).

Normalizes step blocks (case ids + stage names + run-dir paths masked),
hashes, groups identical scaffolds, reports duplicate groups + size that
generator-level template consolidation would save."""
import argparse
import hashlib
import re
import sys
from pathlib import Path


def normalize(block):
    b = re.sub(r"tc[A-Z]\d{3}", "PID", block)
    b = re.sub(r"\b(sh\d|ss\d|sl\d|s\d)_([a-z0-9_]+)_PID", r"\1_\2_PID", b)
    b = re.sub(r"runs/v4/[a-z0-9-]+", "RUNDIR", b)
    b = re.sub(r"ans-[A-Za-z0-9_.-]+", "ANS", b)
    return b


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("yml")
    ap.add_argument("--min-group", type=int, default=3)
    a = ap.parse_args()
    text = Path(a.yml).read_text()
    steps = re.split(
        r"\n(?=    (?:conf_|routelog_|terminal:|(?:sh|ss|sl)\d_)[a-zA-Z0-9_]*:)",
        text)
    step_blocks = [s for s in steps
                   if re.match(r"    (?:conf_|routelog_|terminal:|"
                               r"(?:sh|ss|sl)\d_)[a-zA-Z0-9_]*:", s)
                   and "shell:" in s]
    groups = {}
    for s in step_blocks:
        h = hashlib.sha256(normalize(s).encode()).hexdigest()[:12]
        groups.setdefault(h, []).append(s)
    dup = {h: g for h, g in groups.items() if len(g) >= a.min_group}
    total_dup = sum(len(g) for g in dup.values())
    saved = sum((len(g) - 1) * len(g[0]) for g in dup.values())
    print(f"steps with shell hooks: {len(step_blocks)}")
    print(f"normalized-duplicate groups (>= {a.min_group}): {len(dup)}")
    print(f"steps in duplicate groups: {total_dup}")
    print(f"approx bytes saved by full consolidation: {saved}")
    for h, g in sorted(dup.items(), key=lambda kv: -len(kv[1]))[:8]:
        name = re.match(r"    ([a-zA-Z0-9_]+):", g[0]).group(1)
        kind = "stage" if any("stage-emit" in x for x in g[:1]) else \
               ("gate" if any("meta-conf" in x for x in g[:1]) else "other")
        print(f"  {h}  n={len(g):<4} kind={kind:<6} e.g. {name}")
    sys.exit(0)


if __name__ == "__main__":
    main()
