#!/usr/bin/env python3
"""batchctl.py — batch live-test cycle state manager for whitt workflow suites.

Companion to .opencode/skill/whitt-batch-livetest/SKILL.md.

Encodes the 4-gate batch cycle:
  A run-next   B triage/fix to 10/10   C dedup+efficiency (quality >=, speed >)
  D regression on all previously passing cases → advance

State file: <results>/cycle-state.json (source of truth across sessions).
Case verdict source: <results>/<case_id>/report.json oracle_pass (exit 0).
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"


def load_state(results: Path) -> dict:
    f = results / "cycle-state.json"
    if not f.exists():
        sys.exit("no cycle-state.json — run: batchctl.py init --suite ... --results ...")
    return json.loads(f.read_text())


def save_state(results: Path, st: dict) -> None:
    (results / "cycle-state.json").write_text(json.dumps(st, indent=2))


def case_ids(st: dict) -> list[str]:
    return [c["case_id"] for c in st["cases"]]


def scan_reports(results: Path, st: dict) -> int:
    """Refresh pass/fail from report.json files. Returns failures in focus set."""
    changed = 0
    for c in st["cases"]:
        rp = results / c["case_id"] / "report.json"
        if rp.exists():
            try:
                ok = bool(json.loads(rp.read_text()).get("oracle_pass"))
            except (json.JSONDecodeError, OSError):
                ok = False
            new = "pass" if ok else "fail"
            if c["status"] != new:
                c["status"] = new
                c["last_change"] = time.strftime("%Y-%m-%dT%H:%M:%S")
                changed += 1
    if changed:
        save_state(results, st)
    return changed


def focus_set(st: dict) -> list[dict]:
    """Current-batch cases + regression watchlist (previously passing)."""
    cur = [c for c in st["cases"] if c["batch"] == st["current_batch"]]
    prev = [c for c in st["cases"]
            if c["batch"] < st["current_batch"] and c["status"] == "pass"]
    return cur + prev


def cmd_init(args: argparse.Namespace) -> int:
    suite = Path(args.suite).resolve()
    results = Path(args.results).resolve()
    results.mkdir(parents=True, exist_ok=True)
    (results / "whitt-logs").mkdir(exist_ok=True)
    cases = sorted(p.name[:-4] for p in suite.glob("*.yml"))
    if not cases:
        sys.exit(f"no .yml cases in {suite}")
    st = {
        "suite_dir": str(suite),
        "results_dir": str(results),
        "batch_size": args.size,
        "current_batch": 1,
        "baseline_locked": False,
        "efficiency_log": [],
        "cases": [
            {"case_id": cid, "batch": i // args.size + 1,
             "status": "pending", "runs": 0, "last_change": None}
            for i, cid in enumerate(cases)
        ],
    }
    save_state(results, st)
    print(f"init: {len(cases)} cases, {st['cases'][-1]['batch']} batches of "
          f"{args.size} → {results / 'cycle-state.json'}")
    return 0


def cmd_status(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    n = scan_reports(results, st)
    print(f"batch {st['current_batch']}/{st['cases'][-1]['batch']}"
          f"{'  LOCKED' if st['baseline_locked'] else ''}"
          f"  (reports refreshed: {n})")
    for b in sorted({c["batch"] for c in st["cases"]}):
        cs = [c for c in st["cases"] if c["batch"] == b]
        counts = {}
        for c in cs:
            counts[c["status"]] = counts.get(c["status"], 0) + 1
        mark = "→" if b == st["current_batch"] else " "
        print(f"{mark} B{b}: " + " ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    return 0


def cmd_run_next(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    wf_dir = Path(args.workflow_dir).resolve() if args.workflow_dir else None
    suite = Path(st["suite_dir"])
    cur = [c for c in st["cases"] if c["batch"] == st["current_batch"]]
    if not cur:
        sys.exit("no batch left")
    for c in cur:
        c["runs"] += 1
    save_state(results, st)
    print(f"# batch {st['current_batch']} — {len(cur)} cases. Run sequentially:")
    for c in cur:
        wf = (wf_dir / f"sh-live-{c['case_id']}.yml" if wf_dir
              else suite / f"{c['case_id']}.yml")
        log = results / "whitt-logs" / f"{c['case_id']}-run{c['runs']}.log"
        print(f"WHITT_ZOMBIE_MAX=8 {WHITT} benchmark "
              f"--workflow {wf} --output-dir {results}/whitt-logs "
              f"> {log} 2>&1; echo '{c['case_id']} EXIT='$?")
        print(f"# then: free -h  (abort if <3GB available)")
    return 0


def cmd_mark(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    for c in st["cases"]:
        if c["case_id"] == args.case:
            c["status"] = args.verdict
            c["last_change"] = time.strftime("%Y-%m-%dT%H:%M:%S")
            save_state(results, st)
            print(f"{args.case} → {args.verdict}")
            return 0
    sys.exit(f"unknown case {args.case}")


def cmd_gate(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    scan_reports(results, st)
    cur = [c for c in st["cases"] if c["batch"] == st["current_batch"]]
    prev = [c for c in st["cases"]
            if c["batch"] < st["current_batch"] and c["status"] == "pass"]
    ok_batch = all(c["status"] == "pass" for c in cur)
    ok_prev = all(c["status"] == "pass" for c in prev)
    eff = st["efficiency_log"]
    verdict = (f"batch10/10={'Y' if ok_batch else 'N'} "
               f"regression={'clean' if ok_prev else 'REGRESSION'} "
               f"efficiency_cycle={'done' if eff and eff[-1].get('accepted') else 'pending'}")
    print(verdict)
    if ok_batch and ok_prev:
        return 0
    return 1


def cmd_regress_check(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    wf_dir = Path(args.workflow_dir).resolve() if args.workflow_dir else None
    suite = Path(st["suite_dir"])
    prev = [c for c in st["cases"]
            if c["batch"] < st["current_batch"] and c["status"] == "pass"]
    if not prev:
        print("no previously passing cases — nothing to regress")
        return 0
    print(f"# regression: re-run {len(prev)} previously passing cases")
    for c in prev:
        c["runs"] += 1
        wf = (wf_dir / f"sh-live-{c['case_id']}.yml" if wf_dir
              else suite / f"{c['case_id']}.yml")
        log = results / "whitt-logs" / f"{c['case_id']}-regress{c['runs']}.log"
        print(f"WHITT_ZOMBIE_MAX=8 {WHITT} benchmark "
              f"--workflow {wf} --output-dir {results}/whitt-logs "
              f"> {log} 2>&1; echo '{c['case_id']} EXIT='$?; free -h | head -2")
    save_state(results, st)
    return 0


def cmd_efficiency(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    rows = []
    for c in st["cases"]:
        d = results / c["case_id"]
        t = d / "trace.jsonl"
        if not t.exists():
            continue
        calls = 0
        for line in t.read_text().splitlines():
            try:
                ev = json.loads(line)
            except json.JSONDecodeError:
                continue
            if ev.get("event") == "llm_call":
                calls += 1
        rp = d / "report.json"
        ok = None
        if rp.exists():
            try:
                ok = bool(json.loads(rp.read_text()).get("oracle_pass"))
            except (json.JSONDecodeError, OSError):
                ok = None
        rows.append((c["case_id"], calls, ok))
    done = [r for r in rows if r[2] is not None]
    if done:
        total_calls = sum(r[1] for r in done)
        passed = sum(1 for r in done if r[2])
        print(f"cases={len(done)} passing={passed} total_llm_calls={total_calls} "
              f"avg_calls={total_calls / len(done):.2f}")
    for cid, calls, ok in rows:
        print(f"  {cid:<36} llm_calls={calls} oracle={ok}")
    if args.record:
        entry = {
            "ts": time.strftime("%Y-%m-%dT%H:%M:%S"),
            "rows": [{"case": r[0], "calls": r[1], "oracle": r[2]} for r in rows],
            "accepted": bool(args.accepted),
            "note": args.note or "",
        }
        st["efficiency_log"].append(entry)
        save_state(results, st)
        print(f"recorded efficiency snapshot #{len(st['efficiency_log'])} "
              f"(accepted={args.accepted})")
    return 0


def cmd_advance(args: argparse.Namespace) -> int:
    results = Path(args.results).resolve()
    st = load_state(results)
    rc = cmd_gate(argparse.Namespace(results=args.results))
    if rc != 0:
        sys.exit("gate not green — cannot advance")
    st["current_batch"] += 1
    st["baseline_locked"] = True
    save_state(results, st)
    print(f"advanced to batch {st['current_batch']}")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("init")
    p.add_argument("--suite", required=True)
    p.add_argument("--results", required=True)
    p.add_argument("--size", type=int, default=10)
    p.set_defaults(fn=cmd_init)

    p = sub.add_parser("status")
    p.add_argument("--results", required=True)
    p.set_defaults(fn=cmd_status)

    p = sub.add_parser("run-next")
    p.add_argument("--results", required=True)
    p.add_argument("--workflow-dir")
    p.set_defaults(fn=cmd_run_next)

    p = sub.add_parser("mark")
    p.add_argument("--results", required=True)
    p.add_argument("--case", required=True)
    p.add_argument("--verdict", choices=["pass", "fail", "pending"], required=True)
    p.set_defaults(fn=cmd_mark)

    p = sub.add_parser("gate")
    p.add_argument("--results", required=True)
    p.set_defaults(fn=cmd_gate)

    p = sub.add_parser("regress-check")
    p.add_argument("--results", required=True)
    p.add_argument("--workflow-dir")
    p.set_defaults(fn=cmd_regress_check)

    p = sub.add_parser("efficiency")
    p.add_argument("--results", required=True)
    p.add_argument("--record", action="store_true")
    p.add_argument("--accepted", action="store_true")
    p.add_argument("--note", default="")
    p.set_defaults(fn=cmd_efficiency)

    p = sub.add_parser("advance")
    p.add_argument("--results", required=True)
    p.set_defaults(fn=cmd_advance)

    args = ap.parse_args()
    return args.fn(args)


if __name__ == "__main__":
    sys.exit(main())
