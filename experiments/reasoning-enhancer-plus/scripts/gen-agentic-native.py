#!/usr/bin/env python3
"""Generate engine-native agentic-suite workflows in three techniques.

decompose: plan step then checked solve step (scaffold-first).
dual:      two checked solver attempts then a harvest gate.
repair:    checked solve then two checked repair rounds with hints.
Spoof mode (default on until GPU frees): every step's gate calls
spoof-emit, which scripts outcomes from the scenario map — no model
loads, no inference; GWT routes on PASS/SKIP tokens exactly where
real runs route on check results.
"""
import argparse
import hashlib
import json
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REPO = EXP.parent
CASES = EXP / "cases/agentic"

MODELS = {
    "fmt": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "deep": "Bonsai-27B-Q1_0",
    "think": "Qwen3-4B-Thinking-2507-Q4_K_M",
}

TECHNIQUES = {
    "decompose": [("plan", "think", 1000), ("solve", "fmt", 250)],
    "dual": [("attempt1", "fmt", 250), ("attempt2", "deep", 1200),
             ("select", "fmt", 5)],
    "repair": [("solve", "fmt", 250), ("repair1", "deep", 1200),
               ("repair2", "think", 3500)],
}


def scenario_for(cases, technique):
    n_checkable = len([s for s, _, _ in TECHNIQUES[technique]
                       if s not in ("plan", "select")])
    out = {}
    for c in cases:
        h = int(hashlib.sha256(
            f"{technique}:{c}".encode()).hexdigest()[:8], 16)
        if h % 10 < 6:
            w = 0
        elif h % 10 < 9:
            w = 1
        else:
            w = n_checkable - 1
        out[c] = min(w, n_checkable - 1)
    return out


def step_block(idx, cid, stage, mkey, tok, run_dir, nxt, cpath,
               scenario, stages_csv, sidx, spoof):
    sid = f"s{idx:03d}_{stage}_{cid}"
    prior = {"solve": "plan", "select": "attempt1,attempt2"}.get(stage, "")
    uncheck = " --unchecking" if spoof and stage in ("plan", "select") else ""
    if spoof:
        gate = (f"python3 {HERE}/spoof-emit.py --case {cpath} "
                f"--stage {stage} --stage-index {sidx} "
                f"--stages {stages_csv} --run-dir {run_dir} "
                f"--scenario {scenario}{uncheck}")
    else:
        gate = (f"python3 {HERE}/stage-emit.py --case {cpath} "
                f"--stage {stage} --run-dir {run_dir}"
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
    if not spoof and stage not in ("plan", "select"):
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
    ap = argparse.ArgumentParser()
    ap.add_argument("--technique", required=True,
                    choices=list(TECHNIQUES))
    ap.add_argument("--out", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--no-spoof", action="store_true")
    args = ap.parse_args()
    run_dir = Path(args.run_dir).resolve()
    run_dir.mkdir(parents=True, exist_ok=True)
    spoof = not args.no_spoof

    cases = []
    for p in sorted(CASES.glob("case-*.yml")):
        cases.append((yaml.safe_load(p.read_text())["case_id"],
                      str(p.resolve())))
    scen = scenario_for([c for c, _ in cases], args.technique)
    scen_p = run_dir / "scenario.json"
    scen_p.write_text(json.dumps(scen, indent=1))

    shape = TECHNIQUES[args.technique]
    plan = []
    for cid, cpath in cases:
        checked = [s for s, _, _ in shape if s != "plan" and s != "select"]
        stages_csv = ",".join(checked)
        for stage, mk, tok in shape:
            if stage in ("plan", "select"):
                sidx = 0
            else:
                sidx = checked.index(stage)
            plan.append((cid, cpath, stage, mk, tok, sidx, stages_csv))

    blocks = []
    ids = []
    for i, (cid, cpath, stage, mk, tok, sidx, csv) in enumerate(plan):
        nxt = "terminal" if i + 1 == len(plan) else None
        blocks.append((cid, cpath, stage, mk, tok, sidx, csv, nxt))
    final = []
    for i, (cid, cpath, stage, mk, tok, sidx, csv, _) in enumerate(blocks):
        nxt = (f"s{i+2:03d}_{blocks[i+1][2]}_{blocks[i+1][0]}"
               if i + 1 < len(blocks) else "terminal")
        sid, block = step_block(i + 1, cid, stage, mk, tok,
                                run_dir, nxt, cpath, scen_p, csv,
                                sidx, spoof)
        ids.append(sid)
        final.append(block)

    model_defs = "\n".join(
        f"  m_{m}:\n    name: {m}\n    host:\n"
        f"      type: llama_cpp_with_vulkan" for m in MODELS.values())

    wf = f"""workflow_id: agentic_{args.technique}
name: "Agentic suite — {args.technique} technique{' (spoof)' if spoof else ''}"
description: "20 agentic reasoning cases, {args.technique} shape."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [rea-plus, agentic, {args.technique}]
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
{chr(10).join(final)}
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
    print(f"wrote {args.out}: {len(cases)} cases, "
          f"{len(final)} steps, scenario win-depths {dict(sorted(dist.items()))}")


if __name__ == "__main__":
    main()
