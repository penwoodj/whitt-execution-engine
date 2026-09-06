#!/usr/bin/env python3
"""Ingest the 170-case corpus (prompt-analysis/test-cases) into
prompts/meta-prompts-170.yml for the meta-v4 campaign
(docs/06-VERSION-ITERATION-RULESET.md §9).

Mapping: tier-A lane from file header "(HEAVY|SYNTH|LIGHT)", shape by
lane; tier-B lane from header, shape single-tool-audit; tier-C all
LIGHT, shape quick-artifact. Batches of 10 in corpus order:
A=b01-10, B=b11-12, C=b13-17.
"""
import argparse
import re
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
REPO = FAS.parents[1]
TC = REPO / "prompt-analysis" / "test-cases"

LANE_RE = re.compile(r"\((HEAVY|SYNTH|LIGHT)\)\s*$")

SHAPES = {
    "A": {"HEAVY": "multi-stage-pipeline", "SYNTH": "tool-build-verify",
          "LIGHT": "single-pass-report"},
    "B": "single-tool-audit",
    "C": "quick-artifact",
}

LANE_OVERRIDES = {
    "A-001": "HEAVY",
    "A-002": "HEAVY",
    "A-003": "HEAVY",
}


def parse_case(path):
    lines = path.read_text().splitlines()
    header = lines[0]
    m = LANE_RE.search(header)
    lane = m.group(1) if m else None
    body = "\n".join(lines[1:]).strip()
    return lane, body


def tier_files(tier):
    d = TC / f"tier-{tier}-{'1000plus' if tier == 'A' else '350' if tier == 'B' else '50-100'}"
    return sorted(d.glob("case-*.md"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(FAS / "prompts" /
                                         "meta-prompts-170.yml"))
    a = ap.parse_args()

    entries = []
    for tier in ("A", "B", "C"):
        files = tier_files(tier)
        expected = {"A": 100, "B": 20, "C": 50}[tier]
        assert len(files) == expected, \
            f"tier-{tier}: {len(files)} files, expected {expected}"
        for f in files:
            num = int(re.search(r"case-[A-Z]-(\d{3})", f.name).group(1))
            lane, body = parse_case(f)
            lane = LANE_OVERRIDES.get(f"{tier}-{num:03d}", lane)
            if tier == "C":
                lane = "LIGHT"
            assert lane, f"no lane in header: {f.name}"
            shape = SHAPES[tier][lane] if tier == "A" else SHAPES[tier]
            entries.append({
                "prompt_id": f"tc{tier}{num:03d}",
                "text": body,
                "meta": {"lane": lane, "shape": shape},
            })

    for i, e in enumerate(entries):
        e["meta"]["batch"] = i // 10 + 1
    assert len(entries) == 170

    doc = {"meta-prompts": entries}
    Path(a.out).parent.mkdir(parents=True, exist_ok=True)
    Path(a.out).write_text(yaml.safe_dump(doc, width=200, sort_keys=False))
    lanes = [e["meta"]["lane"] for e in entries]
    print(f"wrote {a.out}: {len(entries)} entries, "
          f"H={lanes.count('HEAVY')} S={lanes.count('SYNTH')} "
          f"L={lanes.count('LIGHT')}, batches 1-{entries[-1]['meta']['batch']}")


if __name__ == "__main__":
    main()
