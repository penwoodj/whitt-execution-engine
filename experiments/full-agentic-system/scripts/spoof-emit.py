#!/usr/bin/env python3
"""Scenario-driven LLM stand-in for FAS experiments (spoof mode).

Same discipline as REA+/fusion spoof-emit: win-index scenario, UNCHECKED
scaffolds, PASS/SKIP gate tokens, check artifacts. Adds:
  --fault CLASS   emit a faulty observation instead of an answer (exp 10)
  --judge         blind-lane judge spoof (never gates)
"""
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fas_lib as L  # noqa: E402

def emit_token(tok):
    """Print routing token WITHOUT trailing newline.

    Engine GWT compares raw shell stdout: `{{bookmarks.shell_output.stdout}}
    == "PASS"` after template substitution becomes `<stdout> == "PASS"`.
    A trailing newline breaks the lexer (expr error -> false). The dry-run
    simulator strips whitespace, so no-newline output stays compatible.
    """
    sys.stdout.write(f'"{tok}"')
    sys.stdout.flush()

FAULT_BODIES = {
    "timeout": "command timed out after 30s",
    "unreachable": "connection refused: no route to host",
    "garble": "out: \ufffd\ufffd\ufffd 6\u2071 units \ufffd\ufffd",
    "delta": "pool_left: 7",           # plausible, wrong (truth differs)
    "schema_drift": "error: unexpected field 'units' in request schema",
}


def load_fixtures(path):
    import yaml
    return yaml.safe_load(Path(path).read_text()) or {}


def write_outcome(run_dir, cid, stage, ans, passed, failures=None):
    out = {
        "case_id": cid, "stage": stage, "passed": passed,
        "subchecks_total": 1, "subchecks_passed": 1 if passed else 0,
        "failures": failures or [],
    }
    Path(run_dir).mkdir(parents=True, exist_ok=True)
    (Path(run_dir) / f"ans-{cid}-{stage}.txt").write_text(ans)
    (Path(run_dir) / f"check-{cid}-{stage}.json").write_text(
        json.dumps(out))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--stage", required=True)
    ap.add_argument("--stage-index", type=int, default=0)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--scenario", required=True)
    ap.add_argument("--truths", required=True)
    ap.add_argument("--unchecking", action="store_true")
    ap.add_argument("--judge", action="store_true")
    ap.add_argument("--fault", default=None,
                    choices=sorted(L.FAULT_CLASSES),
                    help="fault class to inject at this stage")
    ap.add_argument("--force", action="store_true",
                    help="final-record stage: bypass skip-if-won")
    a = ap.parse_args()

    cid, stage = a.case, a.stage
    scen = json.loads(Path(a.scenario).read_text())

    if a.judge:
        # blind lane: emit verdict, never gate
        if L.has_pass(a.run_dir, cid):
            disagree = scen.get(f"{cid}::judge_disagree", False)
            verdict = "fail" if disagree else "pass"
            (Path(a.run_dir) / f"ans-{cid}-judge.txt").write_text(
                json.dumps({"verdict": verdict}))
            L.ledger_outcome(a.run_dir, cid, "judge",
                             "JUDGE_DISAGREE" if disagree else "JUDGE")
        emit_token("SKIP")
        return

    if L.has_pass(a.run_dir, cid) and not a.force:
        emit_token("PASS")
        return

    if a.unchecking:
        # scaffold: ans artifact yes, but NOT a win — passed must stay
        # null so has_pass (skip-if-won) never short-circuits on it
        write_outcome(a.run_dir, cid, stage,
                      json.dumps({"spoofed": "scaffold"}), None)
        L.ledger_outcome(a.run_dir, cid, stage, "SCAFFOLD")
        emit_token("SKIP")
        return

    if a.fault:
        write_outcome(a.run_dir, cid, stage,
                      FAULT_BODIES[a.fault], False,
                      [{"check": "observation", "detail": a.fault,
                        "fix_hint": "classify and recover"}])
        L.ledger_outcome(a.run_dir, cid, stage, f"FAULT_{a.fault}")
        emit_token("SKIP")
        return

    truths = load_fixtures(a.truths)
    truth = truths.get(cid)
    if truth is None:
        sys.stderr.write('"SKIP"\n')
        emit_token("SKIP")
        return
    if not isinstance(truth, dict):      # fixture wraps in 'truth'
        truth = truth.get("truth", {}) if isinstance(truth, dict) else {}

    win = scen.get(cid, 0)
    if a.stage_index < win:
        write_outcome(a.run_dir, cid, stage,
                      json.dumps({"spoofed": "wrong"}), False,
                      [{"check": "json_exact", "detail": "spoofed failure",
                        "fix_hint": "spoof"}])
        L.ledger_outcome(a.run_dir, cid, stage, "WRONG")
        emit_token("SKIP")
    else:
        ans = json.dumps(truth)
        write_outcome(a.run_dir, cid, stage, ans, True)
        L.ledger_fingerprint(a.run_dir, cid, stage, ans)
        L.ledger_outcome(a.run_dir, cid, stage, "TRUTH")
        emit_token("PASS")


if __name__ == "__main__":
    main()
