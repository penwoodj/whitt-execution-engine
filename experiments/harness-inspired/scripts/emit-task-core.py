#!/usr/bin/env python3
"""Deterministic task-core digest. Renders case task_core (rules/facts/output_spec)
into a compact prompt block. Zero LLM involvement.

Usage: emit-task-core.py --case cases/hc-01.yml [--out FILE]
Default out: docs/benchmarks/outputs/output/hc-core.txt
"""
import argparse
import sys
from pathlib import Path

import yaml


def render(case):
    tc = case["task_core"]
    lines = []
    lines.append("RULES:")
    for r in tc["rules"]:
        lines.append(f"- {r}")
    lines.append("")
    lines.append("FACTS:")
    for f in tc["facts"]:
        lines.append(f"- {f}")
    lines.append("")
    lines.append("REPORT SPEC (exact keys, exact types):")
    for k, t in tc["output_spec"].items():
        lines.append(f"- {k}: {t}")
    return "\n".join(lines)


def main():
    p = argparse.ArgumentParser(description="Deterministic task-core digest")
    p.add_argument("--case", required=True)
    p.add_argument("--out", default="./docs/benchmarks/outputs/output/hc-core.txt")
    args = p.parse_args()

    case = yaml.safe_load(Path(args.case).read_text())
    text = render(case)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text)
    print(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
