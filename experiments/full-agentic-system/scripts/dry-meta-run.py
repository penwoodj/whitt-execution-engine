#!/usr/bin/env python3
"""Dry meta-system runner — engine simulator, zero LLM.

Parses a FAS-generated workflow YAML (mapping steps under
agentic_workflow.steps), walks it exactly as the engine would:
  1. before_step_starts shell hooks -> REAL subprocess, stdout captured
     into bookmarks['shell_output']['stdout']
  2. gwt clauses evaluated on that stdout ('==' on quoted literals);
     route_to target jumps (supports lane-head routing)
  3. skip_step -> skip
  4. model call SIMULATED: output = the ans file the spoof gate wrote
     (spoof-emit is the model stand-in); recorded as text_state
  5. after_step_succeeds hooks: save_to (simulated copy), shell check
     hooks (REAL subprocess), log (skipped)
  6. terminal -> stop

Loop-guarded (max hops). Writes outcomes.jsonl + summary.json.

Usage: dry-meta-run.py --workflow <yml> [--max-hops 5000]
"""
import argparse
import json
import re
import shlex
import subprocess
import sys
import time
from pathlib import Path

import yaml

GWT_RE = re.compile(
    r"bookmarks\.shell_output\.stdout\}*\s*==\s*\"([^\"]+)\"")


def run_cmd(cmd, cwd):
    r = subprocess.run(cmd, shell=True, cwd=str(cwd),
                       capture_output=True, text=True, timeout=120)
    return r.stdout


def eval_gwt(clauses, stdout):
    """Return route target or None (continue). clauses: parsed list."""
    for c in clauses:
        m = GWT_RE.search(c.get("given", ""))
        if m and stdout.strip() == m.group(1):
            return c.get("then")
    return None


def parse_shell_hooks(block):
    hooks = block if isinstance(block, list) else [block]
    out = []
    for h in hooks:
        if isinstance(h, dict) and "shell" in h:
            out.append(h["shell"])
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--workflow", required=True)
    ap.add_argument("--max-hops", type=int, default=5000)
    a = ap.parse_args()

    wf = yaml.safe_load(Path(a.workflow).read_text())
    steps = wf["agentic_workflow"]["steps"]
    if isinstance(steps, list):
        smap = {s["id"]: s for s in steps}
        order = [s["id"] for s in steps]
    else:
        smap = steps
        order = list(steps.keys())

    bookmarks = {"shell_output": {"stdout": ""}}
    text_state = ""
    cur = order[0]
    hops = 0
    executed = []
    t0 = time.time()
    outcomes = []

    while cur in smap and hops < a.max_hops:
        hops += 1
        step = smap[cur]
        when = (step.get("when") or {}).get("before_step_starts") or []
        if any(isinstance(h, dict) and h.get("skip_step")
               for h in (when if isinstance(when, list) else [when])):
            executed.append({"id": cur, "skipped": True})
            nxt = _next_in_order(order, cur)
            if nxt is None:
                break
            cur = nxt
            continue

        # 1. gates (real)
        stdout = ""
        for sh in parse_shell_hooks(when):
            stdout = run_cmd(sh.get("command", ""),
                             sh.get("working_dir") or ".")
        bookmarks["shell_output"]["stdout"] = stdout

        # engine convention: gates emit quoted tokens, GWT literals bare
        tok = stdout.strip()
        if len(tok) >= 2 and tok[0] == '"' and tok[-1] == '"':
            tok = tok[1:-1]
        bookmarks["shell_output"]["_tok"] = tok

        # 2. gwt routing
        gwt = None
        for h in (when if isinstance(when, list) else [when]):
            if isinstance(h, dict) and "gwt" in h:
                gwt = h["gwt"]
        target = eval_gwt(gwt, tok) if gwt else None
        if target and target in smap:
            if tok == "PASS":
                _run_after_hooks(step, executed, outcomes, cur, target)
            else:
                executed.append({"id": cur, "routed_to": target,
                                 "stdout": stdout.strip()[:40]})
            cur = target
            continue

        # 4. simulated model output: spoof gates already wrote the
        # answer files; the model call would emit that content. No-op
        # here (bookmarks/artifacts already on disk).
        text_state = stdout

        _run_after_hooks(step, executed, outcomes, cur)

        nxt = _next_in_order(order, cur)
        if nxt is None:
            break
        cur = nxt

    dur = round(time.time() - t0, 2)
    summary = {
        "workflow": str(a.workflow),
        "steps_executed": len(executed),
        "hops": hops,
        "duration_s": dur,
        "checks_passed": sum(1 for o in outcomes
                             if o["check_passed"] is True),
        "checks_failed": sum(1 for o in outcomes
                             if o["check_passed"] is False),
    }
    out_dir = Path(a.workflow).parents[1] / "runs"
    out_dir.mkdir(parents=True, exist_ok=True)
    tag = Path(a.workflow).stem
    (out_dir / f"{tag}-dry-outcomes.jsonl").write_text(
        "\n".join(json.dumps(o) for o in outcomes) + "\n")
    (out_dir / f"{tag}-dry-summary.json").write_text(
        json.dumps(summary, indent=1) + "\n")
    print(json.dumps(summary))


def _next_in_order(order, cur):
    try:
        i = order.index(cur)
    except ValueError:
        return None
    return order[i + 1] if i + 1 < len(order) else None


def _run_after_hooks(step, executed, outcomes, cur, routed_to=None):
    check_out = None
    after = (step.get("after_step_succeeds")
             or (step.get("when") or {}).get("after_step_succeeds")
             or [])
    for h in (after if isinstance(after, list) else [after]):
        if isinstance(h, dict) and "shell" in h:
            cmd = h["shell"].get("command", "")
            run_cmd(cmd, h["shell"].get("working_dir") or ".")
            m = re.search(r"--out (\S+)", cmd)
            if m:
                check_out = m.group(1)
    passed_check = None
    if check_out and Path(check_out).exists():
        try:
            passed_check = bool(json.loads(
                Path(check_out).read_text()).get("passed"))
        except (json.JSONDecodeError, OSError):
            passed_check = None
    executed.append({"id": cur, "ran": True, "routed_to": routed_to,
                     "check_passed": passed_check})
    outcomes.append({"step": cur, "check_passed": passed_check})


if __name__ == "__main__":
    main()
