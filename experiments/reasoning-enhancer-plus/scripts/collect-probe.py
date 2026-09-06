#!/usr/bin/env python3
"""Aggregate probe run artifacts into summary.json (category accuracy,
per-item detail, wall). Reads check-*.json in run-dir; case categories
resolved from cases/probe/*.yml.
"""
import argparse
import json
import sys
import time
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
CASES_DIR = EXP / "cases/probe"
CATEGORIES = ["FMT", "DRV", "LOG", "PLN", "AUD"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    args = ap.parse_args()
    run = Path(args.run_dir)

    cat_of = {}
    for p in CASES_DIR.glob("case-*.yml"):
        data = yaml.safe_load(p.read_text()) or {}
        if data.get("case_id"):
            cat_of[data["case_id"]] = data.get("category", "?")

    items = []
    for f in sorted(run.glob("check-*.json")):
        try:
            c = json.loads(f.read_text())
        except json.JSONDecodeError:
            continue
        cid = c.get("case_id") or f.stem.replace("check-", "")
        items.append({
            "case_id": cid,
            "category": cat_of.get(cid, "?"),
            "passed": bool(c.get("passed")),
            "failures": [d.get("check") for d in c.get("details", [])
                         if isinstance(d, dict) and not d.get("passed")],
        })

    by_cat = {c: [i for i in items if i["category"] == c]
              for c in CATEGORIES}
    if not items:
        print("REFUSING: 0 check artifacts — run produced nothing; "
              "not writing summary.json")
        return 1
    summary = {
        "model": run.name,
        "ts": int(time.time()),
        "items": items,
        "category_pass": {
            c: (f"{sum(i['passed'] for i in v)}/{len(v)}" if v else "n/a")
            for c, v in by_cat.items()
        },
        "total_pass": f"{sum(i['passed'] for i in items)}/{len(items)}",
    }

    stamps = sorted(p.stat().st_mtime for p in run.glob("check-*.json"))
    if len(stamps) >= 2:
        summary["wall_s"] = round(stamps[-1] - stamps[0], 1)

    (run / "summary.json").write_text(json.dumps(summary, indent=2))
    print(json.dumps({"model": summary["model"],
                      "total": summary["total_pass"],
                      "cats": summary["category_pass"]}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
