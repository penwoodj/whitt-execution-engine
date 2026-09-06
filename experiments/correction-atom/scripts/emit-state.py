#!/usr/bin/env python3
"""Emit prompt context with escalation memory: spec, prior-angle
failures (with fix hints), and current best text.

TEXT = highest-numbered after-angle-*.txt in the run dir (fallback:
case broken_output). Prior failures aggregated from check-angle-*.json —
a check only counts as failing if the LATEST angle that produced it
still fails it (later angles may have fixed earlier failures).

Usage: emit-state.py <case.yml> <run_output_dir>
"""
import json
import re
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import format_check_failure, fix_hint


def latest_text(outdir: Path, fallback: str) -> str:
    best_num, best = -1, None
    for f in outdir.glob("after-angle-*.txt"):
        m = re.search(r"after-angle-(\d+)\.txt", f.name)
        if m:
            n = int(m.group(1))
            content = f.read_text().strip()
            if content and n > best_num:
                best_num, best = n, content
    return best if best is not None else fallback.strip()


def open_failures(outdir: Path) -> dict:
    per_check = {}
    for f in outdir.glob("check-angle-*.json"):
        m = re.search(r"check-angle-(\d+)\.json", f.name)
        if not m:
            continue
        n = int(m.group(1))
        try:
            chk = json.loads(f.read_text())
        except Exception:
            continue
        for name, c in chk.get("checks", {}).items():
            if not isinstance(c, dict):
                continue
            prev = per_check.get(name)
            if prev is None or n > prev[0]:
                per_check[name] = (n, c.get("passed") is True, c)
    return {k: v[2] for k, v in per_check.items() if not v[1]}


def main():
    if len(sys.argv) < 3:
        print("usage: emit-state.py <case.yml> <run_output_dir>", file=sys.stderr)
        sys.exit(2)
    case = yaml.safe_load(Path(sys.argv[1]).read_text())
    outdir = Path(sys.argv[2])

    text = latest_text(outdir, case.get("broken_output", ""))
    spec = case.get("task_spec", "").strip()
    aux = case.get("auxiliary", "").strip()

    print("TASK SPEC + AUXILIARY:")
    print(spec)
    if aux:
        print()
        print(aux)

    failures = open_failures(outdir)
    if failures:
        print()
        print("PRIOR ANGLE FAILURES (still unfixed — fix ALL of these):")
        for i, (name, c) in enumerate(sorted(failures.items()), 1):
            print(f"{i}. {format_check_failure(name, c)} → FIX: {fix_hint(name, c)}")

    print()
    print("TEXT UNDER REVIEW:")
    print(text)


if __name__ == "__main__":
    main()
