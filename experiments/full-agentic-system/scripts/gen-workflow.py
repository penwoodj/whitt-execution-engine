#!/usr/bin/env python3
"""Config-driven workflow YAML generator for FAS experiments.

Usage: python3 gen-workflow.py --exp 07-solve --spoof
       python3 gen-workflow.py --exp 07-solve           (live variant)

Reads <exp>/config.py:
  SHAPES    {"H": [(stage, mkey, max_tokens, style), ...],
             "L": [...]}
  UNCHECKED set of stage names never checked (spoof scaffolds)
  PRIORS    {stage: "prior1,prior2"}
  SCENARIO  callable(cid, n_checked_H) -> win index (spoof)
  CHECK_HOOK_STAGES  set of stages that get check-runner hook (live)
  FAULT_PLAN {"cid:stage": fault_class}  (exp 10 spoof observation faults)
  EXTRA_SCENARIO {cid: {key: val}}       merged into scenario.json

Emits <exp>/workflows/v1-spoof.yml (or v1-live.yml) + run-dir scenario.json.
"""
import argparse
import importlib.util
import json
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
REPO = FAS.parents[1]
sys.path.insert(0, str(SCRIPTS))

MODELS = {
    "fmt": "Qwen3-5-9B-Q4_K_M",
    "deep": "Qwen3-5-9B-Q4_K_M",
    "think": "Qwen3-5-9B-Q4_K_M",
}


def load_config(exp):
    cfg_path = FAS / exp / "config.py"
    spec = importlib.util.spec_from_file_location(
        f"cfg_{exp.replace('-', '_')}", cfg_path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load config for {exp}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def yq(s):
    return json.dumps(s)


def step_block(sid, nxt, mkey, max_tokens, gate_cmd, check_cmd, run_dir,
               save_name=None, temp=0.0, live=False):
    # live: stage-emit keeps the assembled prompt on stdout for the model and
    # signals "skip this stage" via exit code 42 (prompt-as-stdout can never
    # satisfy token-equality GWT); spoof keeps the quoted-token convention.
    if live:
        gwt_lines = [
            "          - gwt:",
            "              - given: "
            "'{{bookmarks.shell_output.exit_code}} == 42'",
            f"                then: {nxt}",
        ]
    else:
        gwt_lines = [
            "          - gwt:",
            "              - given: "
            "'{{bookmarks.shell_output.stdout}} == \"PASS\"'",
            f"                then: {nxt}",
            "              - given: "
            "'{{bookmarks.shell_output.stdout}} == \"SKIP\"'",
            f"                then: {nxt}",
        ]
    # live mode: save_to must write ans-{cid}-{stage}.txt so check-runner,
    # stage-emit priors, and judge newest_answer all resolve (spoof mode
    # keeps the step-name artifact; spoof-emit writes the cid-stage file).
    save_path = (f"{run_dir}/ans-{save_name}.txt" if save_name
                 else f"{run_dir}/ans-{{{{step.step_name}}}}.txt")
    y = [f"    {sid}:",
         f"      generative_entity: \"${{models.m_{mkey}}}\"",
         "      prompt: |",
         "        {{bookmarks.shell_output.stdout}}",
         "      model_overrides:",
         f"        temperature: {temp}",
         f"        max_tokens: {max_tokens}",
         "      when:",
         "        before_step_starts:",
         "          - shell:",
         f"              command: {gate_cmd}",
         f"              working_dir: {REPO}",
         "              fail_on_error: true"] + gwt_lines + [
         "        after_step_succeeds:",
         "        - save_to:",
         "            - $text_state",
         f"            - {save_path}"]
    if check_cmd:
        y += ["        - shell:",
              f"            command: {check_cmd}",
              f"            working_dir: {REPO}",
              "            fail_on_error: false"]
    y += ["        - log:",
          "            level: info",
          f"            to_file_path: {run_dir}/progress.log",
          "            event_fields: [step_name, model_name, duration_ms, "
          "token_count]"]
    return "\n".join(y)


def terminal_block():
    return ("    terminal:\n"
            "      generative_entity: \"${models.m_fmt}\"\n"
            "      prompt: terminal\n"
            "      model_overrides:\n        temperature: 0.0\n"
            "        max_tokens: 3\n"
            "      when:\n        before_step_starts:\n"
            "          - skip_step: true")


def header(wid, name, desc):
    return "\n".join([
        f"workflow_id: {wid}",
        f"name: {yq(name)}",
        f"description: {yq(desc)}",
        "version: \"1.0.0\"",
        "author: \"FAS experiment suite\"",
        "tags: [sub-workflow, fas, atom-proof]",
        "schema_version: \"2.0.0\"",
        "min_schema_version: \"2.0.0\"",
        "providers:",
        "  llama_cpp_with_vulkan:",
        "    config:",
        "      host: localhost",
        "      port: 8080",
        "workflow_execution_strategy:",
        "  timing:",
        "    cooldown_after_unload_secs: 0",
        "    min_tmp_space_mb: 50",
        "models:",
    ] + [f"  m_{k}:\n    name: {yq(v)}\n    host:\n      type: llama_cpp_with_vulkan"
          for k, v in MODELS.items()] + [
        "agentic_workflow:",
        "  when:",
        "    after_workflow:",
        "      - log:",
        "          level: info",
        "          to_file_path: RUNDIR/final.log",
        "          event_fields: [workflow_id, total_steps, succeeded, failed]",
        "  steps:",
    ])


def gate_cmd_for(exp, stage, cid, idx, spoof, style, prior,
                 fault=None, unchecking=False, final_stage=None,
                 seed=None):
    if spoof:
        cmd = (f"python3 {SCRIPTS}/spoof-emit.py --case {cid} "
               f"--stage {stage} --stage-index {idx} --run-dir RUNDIR "
               f"--scenario RUNDIR/scenario.json "
               f"--truths {FAS}/{exp}/fixtures/fas-fixes.yml")
        if unchecking:
            cmd += " --unchecking"
        if fault:
            cmd += f" --fault {fault}"
        return cmd
    cmd = (f"python3 {SCRIPTS}/stage-emit.py --case-yml "
           f"{FAS}/{exp}/cases/case-{cid}.yml --case {cid} "
           f"--stage {stage} --run-dir RUNDIR --style {style}")
    if prior:
        cmd += f" --prior {prior}"
    if final_stage:
        cmd += f" --final-stage {final_stage}"
    if seed:
        cmd += f" --seed {seed}"
    return cmd


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exp", required=True)
    ap.add_argument("--spoof", action="store_true")
    ap.add_argument("--out", default=None)
    ap.add_argument("--run-dir", default=None)
    ap.add_argument("--limit", type=int, default=None,
                    help="only first N cases (live wiring probes)")
    ap.add_argument("--only", default=None,
                    help="comma-separated case ids to include (retry runs)")
    ap.add_argument("--live", action="store_true",
                    help="explicit live mode (default when not --spoof)")
    ap.add_argument("--seed", default=None,
                    help="prior run dir whose passed stages carry forward")
    ap.add_argument("--model", default=None,
                    help="override every model slot with this model id")
    a = ap.parse_args()
    global MODELS
    if a.model:
        MODELS = {k: a.model for k in MODELS}

    cfg = load_config(a.exp)
    import yaml as _y
    cases = [_y.safe_load(f.read_text())
             for f in sorted((FAS / a.exp / "cases").glob("case-*.yml"))]
    if not cases:
        sys.exit(f"no cases for {a.exp}; run gen-cases.py --write first")
    if a.limit:
        cases = cases[:a.limit]
    if a.only:
        wanted = {x.strip() for x in a.only.split(",") if x.strip()}
        cases = [c for c in cases if c["case_id"] in wanted]
        missing = wanted - {c["case_id"] for c in cases}
        if missing:
            sys.exit(f"unknown case ids: {sorted(missing)}")

    mode = "spoof" if a.spoof else "live"
    out = Path(a.out) if a.out else FAS / a.exp / "workflows" / f"v1-{mode}.yml"
    run_dir = (Path(a.run_dir) if a.run_dir
               else FAS / a.exp / "runs" / f"{mode}-r1").resolve()
    run_dir.mkdir(parents=True, exist_ok=True)

    shapes = cfg.SHAPES
    unchecked = getattr(cfg, "UNCHECKED", set())
    priors = getattr(cfg, "PRIORS", {})
    check_stages = getattr(cfg, "CHECK_HOOK_STAGES", set())
    final_stage = getattr(cfg, "FINAL_CHECK_STAGE", None)
    fault_plan = getattr(cfg, "FAULT_PLAN", {}) if a.spoof else {}
    extra_scen = getattr(cfg, "EXTRA_SCENARIO", None)
    cases_for_extra = None

    scenario = {}
    case_meta = []
    for c in cases:
        cid = c["case_id"]
        n_h_checked = sum(1 for st, *_ in shapes["H"] if st not in unchecked)
        scenario[cid] = (cfg.SCENARIO(cid, n_h_checked)
                         if a.spoof else 0)
        if callable(extra_scen):
            if cases_for_extra is None:
                cases_for_extra = extra_scen(cases)
            for k, v in (cases_for_extra.get(cid) or {}).items():
                scenario[f"{cid}::{k}"] = v
        elif extra_scen:
            for k, v in (extra_scen.get(cid) or {}).items():
                scenario[f"{cid}::{k}"] = v
        case_meta.append(cid)

    all_steps = []   # (sid, stage, mkey, mt, style, lane, cid)
    prior_targets = {p.strip()
                     for ps in priors.values()
                     for p in str(ps).split(",")}
    live_drops = (unchecked - prior_targets) if not a.spoof else set()
    for cid in case_meta:
        for lane in ("H", "L"):
            for (stage, mkey, mt, style) in shapes[lane]:
                if stage in live_drops:
                    continue
                suffix = "" if lane == "H" else "L"
                sid = f"s{len(all_steps)+1:03d}_{stage}{suffix}_{cid}"
                all_steps.append((sid, stage, mkey, mt, style, lane, cid))

    steps = []
    for pos, (sid, stage, mkey, mt, style, lane, cid) in enumerate(all_steps):
        nxt = all_steps[pos + 1][0] if pos + 1 < len(all_steps) else "terminal"
        idx_in_case = sum(1 for s in all_steps[:pos]
                          if s[6] == cid and s[5] == lane)
        fault = fault_plan.get(f"{cid}:{stage}")
        cmd = gate_cmd_for(a.exp, stage, cid, idx_in_case, a.spoof, style,
                           priors.get(stage, ""), fault=fault,
                           unchecking=(stage in unchecked),
                           final_stage=final_stage,
                           seed=(str(Path(a.seed).resolve())
                                 if a.seed else None))
        save_name = f"{cid}-{stage}" if not a.spoof else None
        temp = 0.7 if (not a.spoof and lane == "L") else 0.0
        if not a.spoof and style == "cot":
            mt = max(mt, 2200)
        check_cmd = None
        if not a.spoof and stage in check_stages:
            check_cmd = (f"python3 {SCRIPTS}/check-runner.py --case-yml "
                         f"{FAS}/{a.exp}/cases/case-{cid}.yml --case {cid} "
                         f"--stage {stage} --text-file "
                         f"{run_dir}/ans-{cid}-{stage}.txt --out "
                         f"{run_dir}/check-{cid}-{stage}.json > "
                         f"{run_dir}/check-{cid}-{stage}.log 2>&1; exit 0")
        steps.append(step_block(sid, nxt, mkey, mt, cmd, check_cmd, run_dir,
                                save_name=save_name, temp=temp,
                                live=not a.spoof))

    steps.append(terminal_block())
    yml = header(f"fas_{a.exp.replace('-', '_')}_{mode}",
                 f"FAS {a.exp} {mode}",
                 f"Atom proof workflow for {a.exp} ({mode} variant)") + \
        "\n" + "\n".join(steps) + "\n"
    yml = yml.replace("RUNDIR", str(run_dir))
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(yml)
    (run_dir / "scenario.json").write_text(json.dumps(scenario, indent=1))
    wins = {}
    for cid in case_meta:
        wins[scenario[cid]] = wins.get(scenario[cid], 0) + 1
    print(f"wrote {out}: {len(case_meta)} cases, {len(all_steps)} steps, "
          f"win-depths {dict(sorted(wins.items()))}")


if __name__ == "__main__":
    main()
