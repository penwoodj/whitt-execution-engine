#!/usr/bin/env python3
"""Spoof model for engine-native workflow iteration without LLMs.

Replaces the stage-emit + inference pair in spoof runs: reads the
scenario map (cid -> winning stage index over that case's checked
stages), writes scripted ans/check artifacts exactly where real
inference would, and prints the routing token the GWT gate consumes:
PASS (case won / already won) or SKIP (scripted failure, chain
walks on). Zero network, zero GPU.
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

from agentic_ledger import ledger_fingerprint, ledger_outcome


def load_fixtures(path=None):
    if path:
        fx = Path(path)
    else:
        fx = (Path(__file__).resolve().parent.parent
              / "fixtures/agentic-fixes.yml")
    return yaml.safe_load(fx.read_text()) or {}


def has_pass(run_dir, cid):
    for f in Path(run_dir).glob(f"check-{cid}-*.json"):
        try:
            if json.loads(f.read_text()).get("passed"):
                return True
        except Exception:
            pass
    return False


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
    args = ap.parse_args()

    c = yaml.safe_load(Path(args.case).read_text())
    cid = c["case_id"]
    stages = [s for s in args.stages.split(",") if s]

    if has_pass(args.run_dir, cid):
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
