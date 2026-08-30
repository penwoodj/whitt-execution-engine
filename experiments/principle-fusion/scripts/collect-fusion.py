#!/usr/bin/env python3
"""Collect principle-fusion run results into summary.json.

Two-lane verdict per case (N5): deterministic lane = any passing
check; judge lane = verdict JSON from ans-{cid}-judge.txt; FINAL =
det always. Judge disagreement is recorded, never load-bearing.

Also reports win-depth distribution (which checked stage first
passed — the efficiency signal: earlier win = less compute) and a
failure taxonomy (FORMAT / LEAK / CONTENT).
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

from fusion_lib import has_pass, latest_check

FORMAT_CHECKS = {"json_exact", "line_count", "all_caps",
                 "bullet_count_min", "bullet_count_max",
                 "yaml_parsable"}
LEAK_CHECKS = {"forbidden_phrases"}


def judge_verdict(run_dir, cid):
    f = Path(run_dir) / f"ans-{cid}-judge.txt"
    if not f.exists() or not f.stat().st_size:
        return None
    try:
        return json.loads(f.read_text()).get("verdict")
    except Exception:
        return None


def winning_stage(run_dir, cid):
    for f in sorted(Path(run_dir).glob(f"check-{cid}-*.json"),
                    key=lambda p: p.stat().st_mtime):
        try:
            j = json.loads(f.read_text())
        except Exception:
            continue
        if j.get("passed"):
            return j.get("stage"), j.get("stage_index")
    return None, None


def failure_taxa(check_json):
    taxa = set()
    for f in (check_json or {}).get("failures", []):
        n = f.get("check", "?")
        if n in FORMAT_CHECKS:
            taxa.add("FORMAT")
        elif n in LEAK_CHECKS:
            taxa.add("LEAK")
        else:
            taxa.add("CONTENT")
    return sorted(taxa)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--cases-dir", default=None)
    ap.add_argument("--out", default=None)
    args = ap.parse_args()
    run_dir = Path(args.run_dir)
    cases_dir = Path(args.cases_dir) if args.cases_dir else (
        Path(__file__).resolve().parent.parent / "cases")

    rows = []
    for p in sorted(cases_dir.glob("case-*.yml")):
        c = yaml.safe_load(p.read_text())
        cid = c["case_id"]
        det = has_pass(run_dir, cid)
        jv = judge_verdict(run_dir, cid)
        wstage, widx = winning_stage(run_dir, cid)
        cj = latest_check(run_dir, cid)
        rows.append({
            "case_id": cid,
            "difficulty": (c.get("rea_plus") or {}).get(
                "difficulty", "L"),
            "det_passed": det,
            "judge_verdict": jv,
            "final": det,
            "judge_disagreed": jv == "fail" and det,
            "won_at_stage": wstage,
            "win_depth": widx,
            "failure_taxa": failure_taxa(cj) if not det else [],
        })

    n = len(rows) or 1
    dist = {}
    for r in rows:
        if r["det_passed"] and r["win_depth"] is not None:
            dist[r["win_depth"]] = dist.get(r["win_depth"], 0) + 1
    summary = {
        "run_dir": str(run_dir),
        "cases": rows,
        "totals": {
            "cases": len(rows),
            "det_pass_rate": sum(r["det_passed"] for r in rows) / n,
            "final_pass_rate": sum(r["final"] for r in rows) / n,
            "judge_disagreement_rate":
                sum(r["judge_disagreed"] for r in rows) / n,
            "judge_lane_coverage":
                sum(r["judge_verdict"] is not None
                    for r in rows) / n,
            "win_depth_distribution": dict(sorted(dist.items())),
        },
    }
    out = Path(args.out) if args.out else run_dir / "summary.json"
    out.write_text(json.dumps(summary, indent=1))
    t = summary["totals"]
    print(f"{len(rows)} cases | det {t['det_pass_rate']:.0%} | "
          f"judge lane {t['judge_lane_coverage']:.0%} | "
          f"disagree {t['judge_disagreement_rate']:.0%} | "
          f"win-depths {t['win_depth_distribution']}")
    print(f"wrote {out}")


if __name__ == "__main__":
    sys.exit(main())
