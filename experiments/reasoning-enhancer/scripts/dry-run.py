#!/usr/bin/env python3
"""REA dry-run harness — drives the REAL pipeline scripts with fixture
LLM stand-ins; no model, no docker, no network.

Usage:
  dry-run.py [--keep]

Simulates the rea-v1.yml step order exactly (gate → decompose → validate
→ solve → planverify → toolverify → synth r1 → [synth r2] → select-best)
by subprocess-invoking emit-case.py / check-answer.py / select-best.py and
writing stand-in "model outputs" where the workflow saves step text.
Scenarios prove every terminal mode of select-best plus the escalation
memory and factored-context contracts. Exit 1 on any scenario failure.
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).parent
EXP = SCRIPTS.parent
FIXTURES = EXP / "fixtures" / "reference-fixes.yml"
EMIT = str(SCRIPTS / "emit-case.py")
CHECK = str(SCRIPTS / "check-answer.py")
SELECT = str(SCRIPTS / "select-best.py")

SUBQ_STANDIN = json.dumps({"subquestions": [
    "What is the total budget and the server cap in dollars?",
    "What is the exact license cost?",
    "What amount remains for monitoring after servers and licenses?",
    "What format must the final answer use?",
]})
SUBA_STANDIN = (
    "1. Total budget is $1200, so the server cap is 40 percent of 1200 = $480.\n"
    "2. Licenses cost exactly $300.\n"
    "3. Monitoring gets 1200 - 480 - 300 = $420.\n"
    "4. One line per budget line with dollar amounts, then a closing sentence.\n"
)
VQ_STANDIN = (
    "1. Does the servers amount stay at or under the 40 percent cap?\n"
    "2. Is the license amount exactly $300?\n"
    "3. Do the three amounts sum to the total budget?\n"
)
WEAK_CLEAN = (
    "- Allocation pending.\n"
    "Allocation pending. Final figures to follow once constraints are "
    "confirmed with the finance team."
)


def sh(args):
    p = subprocess.run([sys.executable] + args, capture_output=True, text=True)
    return p.returncode, p.stdout


def run_pipeline(case_file, run_dir, r1_text, r2_text=None, subq_text=SUBQ_STANDIN):
    run = Path(run_dir)
    run.mkdir(parents=True, exist_ok=True)
    prompts = {}

    def emit(phase, extra=(), log=None):
        args = [EMIT, "--case", str(case_file), "--run-dir", str(run),
                "--phase", phase] + list(extra)
        code, out = sh(args)
        prompts[phase] = out
        if log:
            (run / log).write_text(out)
        assert code == 0, f"emit {phase} exit {code}"
        return out

    emit("gate-prepare", log="gate-prepare.log")
    gate_exit = (run / "gate-exit.json").exists()
    if gate_exit:
        code, out = sh([SELECT, "--case", str(case_file), "--run-dir", str(run)])
        assert code == 0
        return json.loads(out), prompts, gate_exit

    emit("decompose", log="decompose-prompt.log")
    (run / "subquestions.txt").write_text(subq_text)
    code, out = sh([EMIT, "--case", str(case_file), "--run-dir", str(run),
                    "--phase", "validate-subq", "--out", str(run / "subq-valid.json")])
    subq_valid = code == 0

    if subq_valid:
        emit("solve", log="solve-prompt.log")
        solve_prompt = prompts["solve"]
        assert "DRAFT" not in solve_prompt.upper() or "UNDER REVIEW" not in solve_prompt, \
            "FACTORED CONTRACT VIOLATION: solve prompt must not contain the draft"
        (run / "subanswers.txt").write_text(SUBA_STANDIN)

        emit("planverify", log="planverify-prompt.log")
        (run / "verify-questions.txt").write_text(VQ_STANDIN)
    # invalid subq → workflow gwt routes directly to toolverify, skipping
    # solve + planverify (no subanswers.txt / verify-questions.txt written)

    emit("toolverify", extra=["--out", str(run / "verify.json")])
    assert (run / "verify.json").exists()

    emit("synthesize", extra=["--round", "1"], log="synthesize-r1-prompt.log")
    (run / "enhanced-answer-r1.txt").write_text(r1_text)
    code, _ = sh([CHECK, "--case", str(case_file),
                  "--text-file", str(run / "enhanced-answer-r1.txt"),
                  "--out", str(run / "check-r1.json")])
    r1_result = json.loads((run / "check-r1.json").read_text())
    assert code == (0 if r1_result["passed"] else 1), \
        "check-answer exit must mirror pass/fail (gwt round-2 routing contract)"
    r1_passed = r1_result["passed"]

    if not r1_passed and r2_text is not None:
        emit("synthesize", extra=["--round", "2"], log="synthesize-r2-prompt.log")
        (run / "enhanced-answer-r2.txt").write_text(r2_text)
        code, _ = sh([CHECK, "--case", str(case_file),
                      "--text-file", str(run / "enhanced-answer-r2.txt"),
                      "--out", str(run / "check-r2.json")])
        r2_result = json.loads((run / "check-r2.json").read_text())
        assert code == (0 if r2_result["passed"] else 1), \
            "check-answer exit must mirror pass/fail (gwt routing contract)"

    code, out = sh([SELECT, "--case", str(case_file), "--run-dir", str(run)])
    assert code == 0, "select-best must always exit 0"
    return json.loads(out), prompts, gate_exit


def load_case(name):
    return EXP / "cases" / name


def load_ref(cid):
    return yaml.safe_load(FIXTURES.read_text())[cid]


def scenario_happy(tmp):
    case = load_case("case-001.yml")
    ref = load_ref("rea-001")
    sb, prompts, gate = run_pipeline(case, tmp / "happy", r1_text=ref)
    fails = []
    if gate:
        fails.append("gate must NOT exit: draft fails checks")
    if sb["mode"] != "selected_r1" or not sb["passed"]:
        fails.append(f"want selected_r1/pass, got {sb}")
    if (tmp / "happy" / "final-answer.txt").read_text() != ref:
        fails.append("final-answer.txt != reference fix")
    if "OPEN FAILURES IN THE DRAFT" not in prompts["synthesize"]:
        fails.append("r1 prompt missing draft open-failures block")
    return fails


def scenario_round2(tmp):
    case = load_case("case-005.yml")
    case_data = yaml.safe_load(case.read_text())
    draft = case_data["draft_response"]
    ref = load_ref("rea-005")
    sb, prompts, gate = run_pipeline(case, tmp / "round2", r1_text=draft, r2_text=ref)
    fails = []
    if sb["mode"] != "selected_r2" or not sb["passed"]:
        fails.append(f"want selected_r2/pass, got {sb}")
    r2p = prompts["synthesize"]
    if "skeptical auditor" not in r2p:
        fails.append("r2 prompt missing persona lens")
    if "PRIOR ATTEMPT (round 1) FAILED THESE CHECKS" not in r2p:
        fails.append("r2 prompt missing escalation memory block")
    if "include the exact required phrase" not in r2p:
        fails.append("escalation memory missing contains_required fix hint")
    if (tmp / "round2" / "final-answer.txt").read_text() != ref:
        fails.append("final-answer.txt != reference fix")
    return fails


def scenario_early_exit(tmp):
    case = load_case("case-001.yml")
    ref = load_ref("rea-001")
    data = yaml.safe_load(case.read_text())
    data["draft_response"] = ref
    swapped = tmp / "case-001-passing-draft.yml"
    swapped.write_text(yaml.safe_dump(data, allow_unicode=True, width=100))
    sb, _, gate = run_pipeline(swapped, tmp / "early-exit", r1_text="unused")
    fails = []
    if not gate:
        fails.append("gate must exit when draft passes all checks")
    if sb["mode"] != "early_exit" or not sb["passed"]:
        fails.append(f"want early_exit/pass, got {sb}")
    final = tmp / "early-exit" / "final-answer.txt"
    if not final.exists() or final.read_text().strip() != ref.strip():
        fails.append("early exit must ship draft unchanged")
    for leftover in ("enhanced-answer-r1.txt", "subquestions.txt"):
        if (tmp / "early-exit" / leftover).exists():
            fails.append(f"early exit must skip pipeline, found {leftover}")
    return fails


def scenario_ratchet(tmp):
    case = load_case("case-003.yml")
    sb, _, gate = run_pipeline(case, tmp / "ratchet", r1_text=WEAK_CLEAN,
                               r2_text=WEAK_CLEAN)
    fails = []
    if gate:
        fails.append("gate must not exit")
    if sb["mode"] != "ratchet_input":
        fails.append(f"want ratchet_input, got {sb}")
    draft = yaml.safe_load(case.read_text())["draft_response"]
    final = tmp / "ratchet" / "final-answer.txt"
    if not final.exists() or final.read_text() != draft:
        fails.append("ratchet must ship input draft verbatim")
    return fails


def scenario_hard_fail(tmp):
    case = load_case("case-001.yml")
    leak = "As an AI, I cannot reveal the muffin count for this bakery order."
    degenerate = "x x x\nx x x\nx x x"
    sb, _, gate = run_pipeline(case, tmp / "hardfail", r1_text=leak,
                               r2_text=degenerate)
    fails = []
    if gate:
        fails.append("gate must not exit (draft fails forbidden_phrases)")
    if sb["mode"] != "hard_fail_no_ship":
        fails.append(f"want hard_fail_no_ship, got {sb}")
    if (tmp / "hardfail" / "final-answer.txt").exists():
        fails.append("hard-fail must NOT write final-answer.txt")
    return fails


def scenario_invalid_subq(tmp):
    case = load_case("case-004.yml")
    ref = load_ref("rea-004")
    garbage_subq = "this is not a question list the model babbled"
    sb, prompts, gate = run_pipeline(case, tmp / "invalid-subq", r1_text=ref,
                                     subq_text=garbage_subq)
    fails = []
    subq_valid = json.loads((tmp / "invalid-subq" / "subq-valid.json").read_text())
    if subq_valid.get("valid") is not False:
        fails.append(f"garbage subq must be flagged invalid, got {subq_valid}")
    if (tmp / "invalid-subq" / "subanswers.txt").exists():
        fails.append("invalid-subq routing must skip solve (no subanswers.txt)")
    if (tmp / "invalid-subq" / "verify-questions.txt").exists():
        fails.append("invalid-subq routing must skip planverify")
    if "SUB-ANSWERS" in prompts["synthesize"]:
        fails.append("synthesize must degrade without sub-answers")
    if sb["mode"] != "selected_r1" or not sb["passed"]:
        fails.append(f"want selected_r1/pass on degraded path, got {sb}")
    return fails


SCENARIOS = [
    ("HAPPY", scenario_happy),
    ("ROUND2", scenario_round2),
    ("EARLY-EXIT", scenario_early_exit),
    ("RATCHET", scenario_ratchet),
    ("HARD-FAIL", scenario_hard_fail),
    ("INVALID-SUBQ", scenario_invalid_subq),
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--keep", action="store_true")
    args = ap.parse_args()

    tmp = Path(tempfile.mkdtemp(prefix="rea-dryrun-"))
    any_fail = False
    for name, fn in SCENARIOS:
        try:
            fails = fn(tmp)
        except AssertionError as exc:
            fails = [f"assertion: {exc}"]
        except Exception as exc:
            fails = [f"crash: {type(exc).__name__}: {exc}"]
        if fails:
            any_fail = True
            print(f"FAIL {name}")
            for x in fails:
                print(f"  - {x}")
        else:
            print(f"PASS {name}")

    if any_fail:
        print(f"artifacts kept for debug: {tmp}")
        return 1
    if not args.keep:
        import shutil
        shutil.rmtree(tmp, ignore_errors=True)
    else:
        print(f"artifacts kept: {tmp}")
    print("OK: all dry-run scenarios passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
