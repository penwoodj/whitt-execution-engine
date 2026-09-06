#!/usr/bin/env python3
"""Generate v10/v11 principle-driven agentic workflows.

Techniques from docs/web-research/AGENTIC-META-PRINCIPLES.md:
  plandispatch (P4/E1), samplevote (P2), adaptive (P6/E6),
  meta (v11 combination of live-A/B winners: solve -> extract ->
  deep-sample rescue -> extract2 -> think-verify -> extract3;
  light cases pruned to solve -> extract).
"""
import argparse
import hashlib
import json
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REPO = EXP.parents[1]
CASES = EXP / "cases/agentic"

MODELS = {
    "small": "Qwen3-1.7B-abliterated-q8_0",
    "fmt": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "deep": "Bonsai-27B-Q1_0",
    "think": "Qwen3-4B-Thinking-2507-Q4_K_M",
}

SHAPES = {
    "plandispatch": {
        "H": [("plan", "think", 1000, "plan"),
              ("solve", "fmt", 250, "cot"),
              ("wait", "fmt", 400, "wait"),
              ("extract", "fmt", 150, "extract")],
        "L": [("solve", "fmt", 250, "cot"),
              ("extract", "fmt", 150, "extract")],
    },
    "samplevote": {
        "H": [("sample1", "fmt", 250, "cot"),
              ("sample2", "deep", 1200, "cot"),
              ("harvest", "fmt", 150, "extract"),
              ("extract2", "fmt", 150, "extract")],
        "L": [("sample1", "fmt", 250, "cot"),
              ("extract2", "fmt", 150, "extract")],
    },
    "adaptive": {
        "H": [("plan", "deep", 1000, "plan"),
              ("solve", "fmt", 250, "cot"),
              ("recheck", "think", 2000, "verify"),
              ("final_extract", "fmt", 150, "extract")],
        "L": [("solve", "fmt", 250, "cot"),
              ("final_extract", "fmt", 150, "extract")],
    },
    "swiss": {
        "H": [("solve", "fmt", 600, "cot"),
              ("extract", "fmt", 200, "extract"),
              ("sample2", "deep", 1500, "cot"),
              ("extract2", "fmt", 200, "extract"),
              ("verify", "think", 3500, "verify"),
              ("extract3", "fmt", 200, "extract")],
        "L": [("solve", "fmt", 400, "cot"),
              ("extract", "fmt", 200, "extract")],
    },
    "meta": {
        "H": [("solve", "fmt", 250, "cot"),
              ("extract", "fmt", 150, "extract"),
              ("sample2", "deep", 1200, "cot"),
              ("extract2", "fmt", 150, "extract"),
              ("verify", "think", 2500, "verify"),
              ("extract3", "fmt", 150, "extract")],
        "L": [("solve", "fmt", 250, "cot"),
              ("extract", "fmt", 150, "extract")],
    },
}
UNCHECKED = {"plan", "harvest"}

TRUTHS = None

PRIORS = {
    "plandispatch": {"wait": "solve", "extract": "wait,solve"},
    "samplevote": {"extract2": "sample1,sample2"},
    "adaptive": {"final_extract": "recheck,solve"},
    "meta": {"extract": "solve", "extract2": "sample2",
             "verify": "extract", "extract3": "verify"},
    "swiss": {"extract": "solve", "extract2": "sample2",
              "verify": "extract", "extract3": "verify"},
}

CANON = {"swiss": ["solve_small", "solve", "extract", "sample2",
                   "extract2", "verify", "extract3"]}


def scenario_for(cases, shape, technique_name="x"):
    checked = [s for s, _, _, _ in shape if s not in UNCHECKED]
    out = {}
    for cid, diff in cases:
        h = int(hashlib.sha256(cid.encode()).hexdigest()[:8], 16)
        if diff == "L":
            out[cid] = 0 if h % 10 < 8 else len(checked) - 1
        else:
            out[cid] = 0 if h % 10 < 3 else (
                1 if h % 10 < 7 else len(checked) - 1)
        if technique_name in ("meta", "swiss"):
            out[cid] = min(out[cid], 1) if diff == "H" else 0
    return out, len(checked)


def step_block(idx, cid, stage, mkey, tok, style, run_dir, nxt, cpath,
               scenario, sidx, stages_csv, spoof, technique):
    sid = f"s{idx:03d}_{stage}_{cid}"
    uncheck = " --unchecking" if spoof and stage in UNCHECKED else ""
    prior = PRIORS.get(technique, {}).get(stage, "")
    if spoof:
        gate = (f"python3 {HERE}/spoof-emit.py --case {cpath} "
                f"--stage {stage} --stage-index {sidx} "
                f"--stages {stages_csv} --run-dir {run_dir} "
                f"--scenario {scenario}{uncheck}"
                + (f" --truths {TRUTHS}" if TRUTHS else ""))
    else:
        gate = (f"python3 {HERE}/stage-emit.py --case {cpath} "
                f"--stage {stage} --style {style} --run-dir {run_dir}"
                + (f" --prior {prior}" if prior else ""))
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
        "              - given: '{{bookmarks.shell_output.stdout}} == \"PASS\"'",
        f"                then: {nxt}",
        "              - given: '{{bookmarks.shell_output.stdout}} == \"SKIP\"'",
        f"                then: {nxt}",
        "        after_step_succeeds:",
        "          - save_to:",
        "              - $text_state",
        f"              - \"{run_dir}/ans-{cid}-{stage}.txt\"",
    ]
    if not spoof and stage not in UNCHECKED:
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
        "              event_fields: [step_name, model_name, duration_ms, token_count]",
    ]
    return sid, "\n".join(lines)


def main():
    global TRUTHS
    ap = argparse.ArgumentParser()
    ap.add_argument("--technique", required=True, choices=list(SHAPES))
    ap.add_argument("--out", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--no-spoof", action="store_true")
    ap.add_argument("--order", choices=["case", "stage"],
                    default="case")
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
    scen = {}
    plan = []
    for cid, diff, cpath in cases:
        shape = SHAPES[args.technique][diff]
        s, n_checked = scenario_for([(cid, diff)], shape, args.technique)
        scen[cid] = s[cid]
        checked = [x for x in shape if x[0] not in UNCHECKED]
        csv = ",".join(x[0] for x in checked)
        names = [x[0] for x in checked]
        for stage, mk, tok, style in shape:
            sidx = names.index(stage) if stage in names else 0
            plan.append((cid, cpath, stage, mk, tok, style, sidx, csv))
    if args.order == "stage":
        canon = CANON.get(args.technique,
                          ["solve_small", "solve", "extract",
                           "sample2", "harvest", "extract2",
                           "recheck", "verify", "final_extract",
                           "extract3", "plan", "sample1", "wait"])
        plan.sort(key=lambda it: canon.index(it[2])
                  if it[2] in canon else len(canon))
    (run_dir / "scenario.json").write_text(json.dumps(scen, indent=1))
    shape_map = {}
    for diff_shape in SHAPES[args.technique].values():
        names = [x[0] for x in diff_shape if x[0] not in UNCHECKED]
        for i, n in enumerate(names):
            shape_map.setdefault(n, i)
    (run_dir / "shape.json").write_text(json.dumps(shape_map, indent=1))

    blocks = []
    for i, (cid, cpath, stage, mk, tok, style, sidx, csv) in enumerate(plan):
        nxt = (f"s{i+2:03d}_{plan[i+1][2]}_{plan[i+1][0]}"
               if i + 1 < len(plan) else "terminal")
        sid, block = step_block(i + 1, cid, stage, mk, tok, style,
                                run_dir, nxt, cpath,
                                run_dir / "scenario.json",
                                sidx, csv, spoof, args.technique)
        blocks.append(block)

    model_defs = "\n".join(
        f"  m_{m}:\n    name: {m}\n    host:\n"
        f"      type: llama_cpp_with_vulkan" for m in MODELS.values())

    wf = f"""workflow_id: agentic10_{args.technique}
name: "Agentic v10 — {args.technique}{' (spoof)' if spoof else ''}"
description: "Principle-driven technique from web research."
version: "10.0.0"
author: "Whitt Execution Engine"
tags: [rea-plus, agentic-v10, {args.technique}]
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
    dist = {}
    for c, w in scen.items():
        dist[w] = dist.get(w, 0) + 1
    print(f"wrote {args.out}: {len(cases)} cases, {len(blocks)} steps, "
          f"win-depths {dict(sorted(dist.items()))}")


if __name__ == "__main__":
    main()
