#!/usr/bin/env python3
"""Spoof model for the principle-fusion workflow (zero LLM).

Replaces stage-emit + inference in spoof runs: reads scenario map
(cid -> winning stage index over the case's CHECKED stages), writes
scripted ans/check artifacts exactly where real inference would, and
prints the routing token the GWT gate consumes: PASS (won / already
won) or SKIP (scripted failure, chain walks on).

Fusion additions vs REA+ spoof-emit:
  --judge   blind-lane stage: writes a verdict JSON (never gates the
            chain); 10% of cases get verdict=fail while det passes,
            exercising the two-lane det-overrides-judge verdict (N5).
Zero network, zero GPU.
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

from fusion_lib import has_pass, ledger_fingerprint, ledger_outcome


def load_fixtures(path=None):
    if path:
        fx = Path(path)
    else:
        fx = (Path(__file__).resolve().parent.parent
              / "fixtures/fusion-fixes.yml")
    return yaml.safe_load(fx.read_text()) or {}


def write_outcome(run_dir, cid, stage, ans, passed, sidx=0):
    (Path(run_dir) / f"ans-{cid}-{stage}.txt").write_text(ans + "\n")
    rec = {"case_id": cid, "stage": stage, "passed": passed,
           "stage_index": sidx,
           "subchecks_total": 1,
           "subchecks_passed": 1 if passed else 0,
           "failures": [] if passed else [
               {"check": "json_exact",
                "detail": "spoofed failure",
                "fix_hint": "spoof"}]}
    (Path(run_dir) / f"check-{cid}-{stage}.json").write_text(
        json.dumps(rec, indent=1))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--stage", required=True)
    ap.add_argument("--stage-index", type=int, required=True)
    ap.add_argument("--stages", required=True,
                    help="comma list of this case's checked stages")
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--scenario", required=True)
    ap.add_argument("--truths", default=None)
    ap.add_argument("--unchecking", action="store_true",
                    help="stage never wins: emit ans only, route SKIP")
    ap.add_argument("--judge", action="store_true",
                    help="blind-lane stage: emit verdict JSON, SKIP")
    args = ap.parse_args()

    c = yaml.safe_load(Path(args.case).read_text())
    cid = c["case_id"]

    if has_pass(args.run_dir, cid):
        if args.judge:
            scen = json.loads(Path(args.scenario).read_text())
            disagree = scen.get(f"{cid}::judge_disagree", False)
            verdict = '{"verdict": "fail"}' if disagree \
                else '{"verdict": "pass"}'
            (Path(args.run_dir) / f"ans-{cid}-{args.stage}.txt") \
                .write_text(verdict + "\n")
            ledger_outcome(args.run_dir, cid, args.stage,
                           "JUDGE_DISAGREE" if disagree else "JUDGE")
            print('"SKIP"')
        else:
            ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
            print('"PASS"')
        return

    if args.unchecking:
        (Path(args.run_dir) / f"ans-{cid}-{args.stage}.txt").write_text(
            '{"spoofed": "scaffold"}\n')
        ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
        print('"SKIP"')
        return

    scen = json.loads(Path(args.scenario).read_text())
    win = scen.get(cid, 0)

    if args.stage_index < win:
        write_outcome(args.run_dir, cid, args.stage,
                      '{"spoofed": "wrong"}', False, args.stage_index)
        ledger_outcome(args.run_dir, cid, args.stage, "SKIP")
        print('"SKIP"')
        return

    truth = load_fixtures(args.truths).get(cid, "")
    write_outcome(args.run_dir, cid, args.stage, truth, True,
                  args.stage_index)
    ledger_outcome(args.run_dir, cid, args.stage, "PASS")
    ledger_fingerprint(args.run_dir, cid, args.stage, truth)
    print('"PASS"')


if __name__ == "__main__":
    sys.exit(main())
