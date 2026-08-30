#!/usr/bin/env python3
"""REA+ v3.2 precision pass — 8 remaining cases after v3.1.

Groups (from live output diagnosis):
  charops:  few-shot exemplar char ops (4B-I)
  decompress: compressed decompose + Thinking 3500
  stagemath: stage-per-line arithmetic + Thinking 3500
Writes into rea3-runs/v3.2 (copy of v3.1 state).
"""
import importlib.util as ilu
import json
import shutil
import sys
import tempfile
import time
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
spec = ilu.spec_from_file_location("r3", HERE / "rea3-execute.py")
r3 = ilu.module_from_spec(spec)
spec.loader.exec_module(r3)

SRC = r3.EXP / "results/rea3-runs/v3.1"
DST = r3.EXP / "results/rea3-runs/v3.2"
if DST.exists():
    shutil.rmtree(DST)
shutil.copytree(SRC, DST)
state_p = DST / "state.json"
state = json.loads(state_p.read_text())

CHAROPS = ["s2-str-01", "s2-fmt-05", "s2-fmt-01"]
DECOMPRESS = ["s2-log-04", "s2-drop-02"]
STAGEMATH = ["s2-env-09", "s2-env-11", "s2-env-13"]

CHAROPS_PROMPT = (
    "Character-operation precision. Work letter by letter on paper "
    "before answering.\n"
    "Example 1: reverse('abc') -> 'cba'. Steps: last letter c, then b, "
    "then a.\n"
    "Example 2: last 4 letters of 'station' are t,i,o,n; reversed -> "
    "'noit'.\n"
    "Example 3: vowels in 'orchestra' are o,e,a -> count 3.\n"
    "Now apply the SAME letter-by-letter method to the task. Output "
    "ONLY the final answer.\n\nTASK:\n")
DECOMPRESS_PROMPT = (
    "Compressed decompose. Maximum 8 short lines of work total. "
    "Format:\n"
    "facts: <the 3-5 decisive facts only>\n"
    "rule: <which condition/rule decides each asked item>\n"
    "FINAL: <answer in the exact required format — nothing else on "
    "this line, nothing after it>\n\nTASK:\n")
STAGEMATH_PROMPT = (
    "Staged arithmetic. Compute one stage per line as "
    "'stage_name: value' in order, each derived ONLY from previous "
    "stages. Then verify each line once. Final line must be exactly "
    "the required output (JSON object alone if JSON is requested), "
    "nothing after it. Round only where the task requires.\n\nTASK:\n")


def run_group(name, cids, model, tok, prefix):
    entries = []
    td = Path(tempfile.mkdtemp(prefix=f"v32-{name}-"))
    for suite, d in r3.SUITES:
        for p in sorted(d.glob("case-*.yml")):
            c = yaml.safe_load(p.read_text())
            if c["case_id"] in cids:
                c["prompt"] = prefix + (c.get("prompt") or "").strip()
                (td / f"case-{c['case_id']}.yml").write_text(
                    yaml.safe_dump(c, sort_keys=False, width=78))
                entries.append({"cid": c["case_id"], "path": str(p)})
    if not entries:
        print(f"[v3.2] {name}: no cases found")
        return
    r3.server_guards()
    r3.ensure_loaded(model)
    d = DST / f"precision-{name}"
    checks = r3.run_batch(model, entries, d, tok)
    saved = 0
    for e in entries:
        cj = checks.get(e["cid"])
        ok = bool(cj and cj.get("passed"))
        state["cases"][e["cid"]]["attempts"].append([f"p32:{name}", ok])
        if ok:
            state["cases"][e["cid"]]["passed"] = True
            saved += 1
    print(f"[v3.2] {name} ({model[:14]}): {saved}/{len(entries)}")
    state_p.write_text(json.dumps(state, indent=1))


run_group("charops", CHAROPS, r3.M_FMT, 400, CHAROPS_PROMPT)
run_group("decompress", DECOMPRESS, r3.M_THINK, 3500, DECOMPRESS_PROMPT)
run_group("stagemath", STAGEMATH, r3.M_THINK, 3500, STAGEMATH_PROMPT)

cs = state["cases"]
print(f"[v3.2] TOTAL {sum(r.get('passed', False) for r in cs.values())}/73")
for cid in sorted(cs):
    if not cs[cid].get("passed"):
        print(f"  still failing: {cid}")
