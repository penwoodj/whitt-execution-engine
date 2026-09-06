#!/usr/bin/env python3
"""Generate the principle-fusion native workflows (spoof + live).

Fusion H lane (all principles, engine-owned procedure — structure
REMOVES responsibility from the small models, never adds it):
  plan(think) -> solve(fmt) -> extract -> replan(fmt) -> solve2 ->
  extract2 -> wait(fmt, bounded budget) -> extract3 ->
  sample2(Bonsai-27B, cross-model diversity) -> extract4 ->
  verify(think, backward check) -> extract5 -> judge(blind lane)
Fusion L lane: solve -> extract -> judge (shared blind judge).

Live variant adds the N3 confidence pre-gate (fusion_conf.py) that
routes HEAVY vs LIGHT per case from first-token logprob entropy;
spoof variant uses the hash scenario instead (no model available).

UNCHECKED (scaffold, never det-checked): plan, replan, judge.
"""
import argparse
import hashlib
import json
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REPO = EXP.parents[1]
CASES = EXP / "cases"

MODELS = {
    "fmt": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "deep": "Bonsai-27B-Q1_0",
    "think": "Qwen3-4B-Thinking-2507-Q4_K_M",
}

SHAPES = {
    "H": [("plan", "think", 1000, "plan"),
          ("solve", "fmt", 600, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("replan", "fmt", 250, "replan"),
          ("solve2", "fmt", 600, "cot"),
          ("extract2", "fmt", 200, "extract"),
          ("wait", "fmt", 400, "wait"),
          ("extract3", "fmt", 200, "extract"),
          ("sample2", "deep", 1500, "cot"),
          ("extract4", "fmt", 200, "extract"),
          ("verify", "think", 3500, "verify"),
          ("extract5", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
}

UNCHECKED = {"plan", "replan", "judge"}

PRIORS = {
    "extract": "solve",
    "solve2": "replan,solve",
    "extract2": "solve2",
    "wait": "solve2",
    "extract3": "wait",
    "extract4": "sample2",
    "extract5": "verify",
}

TRUTHS = None


def scenario_for(cases):
    """Hash-deterministic scenario: win index over checked stages +
    10% judge-blind-lane disagreement (on det-passing cases)."""
    out = {}
    for cid, diff in cases:
        h = int(hashlib.sha256(cid.encode()).hexdigest()[:8], 16)
        checked = [s for s, _, _, _ in SHAPES[diff]
                   if s not in UNCHECKED]
        if diff == "L":
            win = 0 if h % 10 < 8 else len(checked) - 1
        else:
            win = 0 if h % 10 < 3 else (
                len(checked) // 2 if h % 10 < 7 else len(checked) - 1)
        out[cid] = win
        jh = int(hashlib.sha256(
            f"{cid}::judge".encode()).hexdigest()[:8], 16)
        out[f"{cid}::judge_disagree"] = jh % 10 == 0
    return out


def checked_csv(shape):
    return ",".join(s for s, _, _, _ in shape if s not in UNCHECKED)


def step_block(sid, cid, stage, mkey, tok, style, run_dir, clauses,
               cpath, gate, checked_hook):
    """Emit one step block. clauses = [(given, then), ...]."""
    lines = [
        f"    {sid}:",
        f"      generative_entity: \"${{models.m_{MODELS[mkey]}}}\"",
        "      prompt: |",
        "        {{bookmarks.shell_output.stdout}}",
        f"      model_overrides: {{ temperature: 0.0, max_tokens: {tok} }}",
        "      when:",
        "        before_step_starts:",
        "          - shell:",
        f"              command: \"{gate}\"",
        f"              working_dir: \"{REPO}\"",
        "              fail_on_error: true",
        "          - gwt:",
    ]
    for given, then in clauses:
        lines += [
            "              - given: "
            f"'{{{{bookmarks.shell_output.stdout}}}} == {given}'",
            f"                then: {then}",
        ]
    lines += [
        "        after_step_succeeds:",
        "          - save_to:",
        "              - $text_state",
        f"              - \"{run_dir}/ans-{cid}-{stage}.txt\"",
    ]
    if checked_hook:
        lines += [
            "          - shell:",
            f"              command: \"python3 {REPO}/experiments/reasoning-enhancer/scripts/check-answer.py --case {cpath} --text-file {run_dir}/ans-{cid}-{stage}.txt --out {run_dir}/check-{cid}-{stage}.json > {run_dir}/check-{cid}-{stage}.log 2>&1; exit 0\"",
            f"              working_dir: \"{REPO}\"",
            "              fail_on_error: false",
        ]
    lines += [
        "          - log:",
        "              level: info",
        f"              to_file_path: \"{run_dir}/progress.log\"",
        "              event_fields: [step_name, model_name, "
        "duration_ms, token_count]",
    ]
    return "\n".join(lines)


def spoof_gate(cpath, stage, sidx, csv, run_dir, scenario):
    uncheck = " --unchecking" if stage in UNCHECKED else ""
    judge = " --judge" if stage == "judge" else ""
    return (f"python3 {HERE}/spoof-emit.py --case {cpath} "
            f"--stage {stage} --stage-index {sidx} "
            f"--stages {csv} --run-dir {run_dir} "
            f"--scenario {scenario}{uncheck}{judge}"
            + (f" --truths {TRUTHS}" if TRUTHS else ""))


def live_gate(cpath, stage, style, run_dir):
    prior = PRIORS.get(stage, "")
    return (f"python3 {HERE}/stage-emit.py --case {cpath} "
            f"--stage {stage} --style {style} --run-dir {run_dir}"
            + (f" --prior {prior}" if prior else ""))


def build_spoof(cases, run_dir):
    scen = scenario_for([(c, d) for c, d, _ in cases])
    blocks = []
    plan = []
    for cid, diff, cpath in cases:
        shape = SHAPES[diff]
        csv = checked_csv(shape)
        names = [s for s, _, _, _ in shape if s not in UNCHECKED]
        for stage, mk, tok, style in shape:
            sidx = names.index(stage) if stage in names else 0
            plan.append((cid, cpath, stage, mk, tok, style, sidx, csv))
    for i, (cid, cpath, stage, mk, tok, style, sidx, csv) \
            in enumerate(plan):
        sid = f"s{i + 1:03d}_{stage}_{cid}"
        nxt = (f"s{i + 2:03d}_{plan[i + 1][2]}_{plan[i + 1][0]}"
               if i + 1 < len(plan) else "terminal")
        clauses = [('\"PASS\"', nxt), ('\"SKIP\"', nxt)]
        gate = spoof_gate(cpath, stage, sidx, csv, run_dir,
                          run_dir / "scenario.json")
        blocks.append(step_block(sid, cid, stage, mk, tok, style,
                                 run_dir, clauses, cpath, gate,
                                 checked_hook=False))
    return scen, blocks


def build_live(cases, run_dir):
    blocks = []
    conf_ids = []
    per_case = []
    idx = 1
    for cid, diff, cpath in cases:
        conf_sid = f"s{idx:03d}_conf_{cid}"
        idx += 1
        h = []
        for stage, mk, tok, style in SHAPES["H"]:
            h.append((f"s{idx:03d}_{stage}_{cid}", stage, mk, tok,
                      style))
            idx += 1
        l = []
        for stage, mk, tok, style in SHAPES["L"]:
            l.append((f"s{idx:03d}_{stage}L_{cid}", stage, mk, tok,
                      style))
            idx += 1
        jsid = f"s{idx:03d}_judge_{cid}"
        idx += 1
        conf_ids.append(conf_sid)
        per_case.append((cid, cpath, conf_sid, h, l, jsid))

    for k, (cid, cpath, conf_sid, h, l, jsid) in enumerate(per_case):
        nxt_conf = (conf_ids[k + 1] if k + 1 < len(conf_ids)
                    else "terminal")
        # conf pre-gate (N3): shell-only block, tiny model budget
        gate = (f"python3 {HERE}/fusion_conf.py --run-dir {run_dir} "
                f"--case {cpath} --logprobs-file "
                f"{run_dir}/probe-{cid}.json --default LIGHT")
        blocks.append(step_block(
            conf_sid, cid, "conf", "fmt", 3, "", run_dir,
            [('\"HEAVY\"', h[0][0]), ('\"LIGHT\"', l[0][0])],
            cpath, gate, checked_hook=False))
        for chain, tail in ((h, jsid), (l, jsid)):
            for i, (sid, stage, mk, tok, style) in enumerate(chain):
                nxt = chain[i + 1][0] if i + 1 < len(chain) else tail
                clauses = [('\"PASS\"', nxt), ('\"SKIP\"', nxt)]
                blocks.append(step_block(
                    sid, cid, stage, mk, tok, style, run_dir, clauses,
                    cpath, live_gate(cpath, stage, style, run_dir),
                    checked_hook=stage not in UNCHECKED))
        blocks.append(step_block(
            jsid, cid, "judge", "think", 300, "judge", run_dir,
            [('\"PASS\"', nxt_conf), ('\"SKIP\"', nxt_conf)],
            cpath, live_gate(cpath, "judge", "judge", run_dir),
            checked_hook=False))
    return blocks


def emit(args, blocks, spoof):
    model_defs = "\n".join(
        f"  m_{m}:\n    name: {m}\n    host:\n"
        f"      type: llama_cpp_with_vulkan" for m in MODELS.values())
    run_dir = Path(args.run_dir).resolve()
    wf = f"""workflow_id: fusion_native_{'spoof' if spoof else 'live'}
name: "Principle Fusion — {'spoof' if spoof else 'live'}"
description: "All 23 principles fused: engine-owned procedure, det-overrides-judge two-lane verdict, extract-harvester, conf pre-gate (live only)."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [principle-fusion, {'spoof' if spoof else 'live'}]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

workflow_execution_strategy:
  timing:
    cooldown_after_unload_secs: 0
    min_tmp_space_mb: 50

models:
{model_defs}

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "{run_dir}/final.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
{chr(10).join(blocks)}
    terminal:
      generative_entity: "${{models.m_{MODELS['fmt']}}}"
      prompt: "terminal"
      model_overrides: {{ temperature: 0.0, max_tokens: 3 }}
      when:
        before_step_starts:
          - skip_step: true
"""
    Path(args.out).write_text(wf)


def main():
    global TRUTHS
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--no-spoof", action="store_true",
                    help="build the LIVE variant")
    ap.add_argument("--cases-dir", default=None)
    ap.add_argument("--truths", default=None)
    args = ap.parse_args()
    run_dir = Path(args.run_dir).resolve()
    run_dir.mkdir(parents=True, exist_ok=True)
    spoof = not args.no_spoof
    TRUTHS = args.truths
    cases_dir = Path(args.cases_dir) if args.cases_dir else CASES

    cases = []
    for p in sorted(cases_dir.glob("case-*.yml")):
        c = yaml.safe_load(p.read_text())
        cases.append((c["case_id"],
                      (c.get("rea_plus") or {}).get("difficulty", "L"),
                      str(p.resolve())))

    if spoof:
        scen, blocks = build_spoof(cases, run_dir)
        (run_dir / "scenario.json").write_text(
            json.dumps(scen, indent=1))
        dist = {}
        for c, w in ((k, v) for k, v in scen.items()
                     if "::" not in k):
            dist[w] = dist.get(w, 0) + 1
        n_dis = sum(1 for k, v in scen.items()
                    if k.endswith("::judge_disagree") and v)
    else:
        blocks = build_live(cases, run_dir)
        dist = {}
        n_dis = 0
    emit(args, blocks, spoof)
    print(f"wrote {args.out}: {len(cases)} cases, {len(blocks)} "
          f"steps, win-depths {dict(sorted(dist.items()))}, "
          f"judge-disagree cases {n_dis}")


if __name__ == "__main__":
    main()
