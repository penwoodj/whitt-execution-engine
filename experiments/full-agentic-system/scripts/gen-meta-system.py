#!/usr/bin/env python3
"""Top-level meta-system workflow generator (FAS stitched v1).

Reads prompts/meta-prompts.yml. Per prompt emits:
  conf gate (meta-conf.py; probe file carries designed lane) with GWT
  route_to LIGHT/HEAVY/SYNTH lane heads, then per-lane stage chains,
  then a checked routelog stage (spoof-emit writes route truth;
  check-runner validates json_exact), then terminal.

This is the ONLY generator that uses GWT route_to to *different lane
heads* (the E2E gap identified in the stitched-control-flow analysis).

Usage: gen-meta-system.py --spoof [--out ...] [--run-dir ...]
"""
import argparse
import json
import sys
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
REPO = FAS.parents[1]

MODEL_GGUF = "Qwen3-4B-Instruct-2507-Q4_K_M.gguf"
MODELS = {
    "fmt": MODEL_GGUF,
    "deep": MODEL_GGUF,
    "think": MODEL_GGUF,
}
# Rule 9 (schema L912): absolute GGUF path, basename == model name.
SOURCE_PATH = Path("/run/media/jon/data/models/") / MODEL_GGUF

LANES = {
    "HEAVY": [("plan", "think", 1000, "plan"),
              ("solve", "fmt", 600, "cot"),
              ("extract", "fmt", 200, "extract"),
              ("replan", "fmt", 250, "replan"),
              ("solve2", "fmt", 600, "cot"),
              ("extract2", "fmt", 200, "extract"),
              ("verify", "think", 3000, "verify"),
              ("extract3", "fmt", 200, "extract")],
    "SYNTH": [("split", "fmt", 250, "plan"),
              ("worker_a", "fmt", 600, "cot"),
              ("extract_a", "fmt", 200, "extract"),
              ("worker_b", "fmt", 600, "cot"),
              ("extract_b", "fmt", 200, "extract"),
              ("merge", "fmt", 700, "cot"),
              ("extract_m", "fmt", 200, "extract")],
    "LIGHT": [("solve", "fmt", 400, "cot"),
              ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"plan", "replan", "split", "judge"}

LIVE_PRIORS = {
    "extract": "solve",
    "extract2": "solve2",
    "extract3": "verify",
    "extract_a": "worker_a",
    "extract_b": "worker_b",
    "extract_m": "merge",
    "solve2": "solve,extract",
    "verify": "extract2",
    "replan": "solve,extract",
}


def yq(s):
    return json.dumps(s)


def gate_block(sid, probe, heads, run_dir):
    """Conf gate: meta-conf reads probe -> lane token; GWT routes."""
    clauses = "\n".join(
        f'            - given: \'{{{{bookmarks.shell_output.stdout}}}} '
        f'== "{lane}"\'\n              then: {head}'
        for lane, head in heads)
    return "\n".join([
        f"    {sid}:",
        "      generative_entity: \"${models.m_fmt}\"",
        "      prompt: route",
        "      model_overrides:",
        "        temperature: 0.0",
        "        max_tokens: 3",
        "      when:",
        "        before_step_starts:",
        "          - shell:",
        f"              command: python3 {SCRIPTS}/meta-conf.py "
        f"--probe {probe} --default LIGHT",
        f"              working_dir: {REPO}",
        "              fail_on_error: true",
        "          - gwt:",
        clauses,
    ])


def stage_block(sid, nxt, mkey, mt, gate_cmd, check_cmd, run_dir,
                save_name=None, live=False, after_route=None):
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
            '            - given: '
            '\'{{bookmarks.shell_output.stdout}} == "PASS"\'',
            f"              then: {nxt}",
            '            - given: '
            '\'{{bookmarks.shell_output.stdout}} == "SKIP"\'',
            f"              then: {nxt}",
        ]
    save_path = (f"{run_dir}/ans-{save_name}.txt" if save_name
                 else f"{run_dir}/ans-{{{{step.step_name}}}}.txt")
    y = [f"    {sid}:",
         f"      generative_entity: \"${{models.m_{mkey}}}\"",
         "      prompt: |",
         "        {{bookmarks.shell_output.stdout}}",
         "      model_overrides:",
         "        temperature: 0.0",
         f"        max_tokens: {mt}",
         "      when:",
         "        before_step_starts:",
         "          - shell:",
         f"              command: {gate_cmd}",
         f"              working_dir: {REPO}",
         "              fail_on_error: true"] + gwt_lines + [
         "        after_step_succeeds:",
         "          - save_to:",
         "              - $text_state",
         f"              - {save_path}"]
    if check_cmd:
        y += ["          - shell:",
              f"              command: {check_cmd}",
              f"              working_dir: {REPO}",
              "              fail_on_error: false"]
    y += ["          - log:",
           "              level: info",
           f"              to_file_path: {run_dir}/progress.log",
           "              event_fields: [step_name, model_name, "
           "duration_ms, token_count]"]
    if after_route:
        y += ["          - gwt:",
              "              - given: '1 == 1'",
              f"                then: {after_route}"]
    return "\n".join(y)


def spoof_cmd(pid, stage, idx, run_dir):
    return (f"python3 {SCRIPTS}/spoof-emit.py --case {pid} "
            f"--stage {stage} --stage-index {idx} --run-dir {run_dir} "
            f"--scenario {run_dir}/scenario.json "
            f"--truths {FAS}/top-level/fixtures/meta-fixes.yml"
            + (" --unchecking" if stage in UNCHECKED else ""))


SEED_DIR = None


def live_cmd(pid, stage, style, run_dir):
    prior = LIVE_PRIORS.get(stage, "")
    prior_arg = f" --prior {prior}" if prior else ""
    seed_arg = f" --seed {SEED_DIR}" if SEED_DIR else ""
    return (f"python3 {SCRIPTS}/stage-emit.py --case-yml "
            f"{FAS}/top-level/cases/case-{pid}.yml --case {pid} "
            f"--stage {stage} --run-dir {run_dir} --style {style}"
            f"{prior_arg}{seed_arg}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--spoof", action="store_true")
    ap.add_argument("--out", default=None)
    ap.add_argument("--run-dir", default=None)
    ap.add_argument("--seed", default=None)
    ap.add_argument("--prompts", default=None,
                    help="prompts yml override (default meta-prompts.yml)")
    ap.add_argument("--only", default=None,
                    help="comma-separated prompt_id filter (batch)")
    ap.add_argument("--case-dir", default=None)
    ap.add_argument("--fixtures", default=None)
    a = ap.parse_args()
    global SEED_DIR
    SEED_DIR = str(Path(a.seed).resolve()) if a.seed else None

    mode = "spoof" if a.spoof else "live"
    out = Path(a.out) if a.out else FAS / "top-level" / "workflows" / \
        f"meta-v1-{mode}.yml"
    run_dir = (Path(a.run_dir) if a.run_dir else
               FAS / "top-level" / "runs" / f"{mode}-r1").resolve()
    case_dir = (Path(a.case_dir) if a.case_dir
                else FAS / "top-level" / "cases")
    fixtures_path = (Path(a.fixtures) if a.fixtures
                     else FAS / "top-level" / "fixtures" / "meta-fixes.yml")
    run_dir.mkdir(parents=True, exist_ok=True)
    case_dir.mkdir(parents=True, exist_ok=True)
    fixtures_path.parent.mkdir(parents=True, exist_ok=True)

    prompts = yaml.safe_load(
        (Path(a.prompts) if a.prompts
         else FAS / "prompts" / "meta-prompts.yml").read_text()
    )["meta-prompts"]
    if a.only:
        keep = {p.strip() for p in a.only.split(",") if p.strip()}
        prompts = [p for p in prompts if p["prompt_id"] in keep]

    # scenario + probes + case ymls + fixtures (route truth, computed
    # from registered meta — never hand-typed per stage)
    scenario = {}
    fixtures = ["# route truths (computed from prompt meta at generation)"]
    for p in prompts:
        pid = p["prompt_id"]
        lane = p["meta"]["lane"]
        scenario[pid] = 0 if mode == "live" else 1  # routelog wins at idx>=1
        scenario[f"{pid}::lane"] = lane
        (run_dir / f"probe-{pid}.json").write_text(
            json.dumps({"lane": lane}))
        truth = {"lane": lane, "route_ok": True,
                 "shape": p["meta"]["shape"]}
        tj = json.dumps(truth, sort_keys=True)
        fixtures += [f"{pid}:  # lane from meta-prompts.yml",
                     f"  {json.dumps(truth, sort_keys=True)}"]
        case_case = case_dir / f"case-{pid}.yml"
        case_case.write_text(
            yaml.safe_dump({
                "prompt_id": pid, "prompt": p["text"].strip(),
                "meta": p["meta"],
                "success_criteria": {"deterministic_checks": {
                    "json_exact": tj}}},
                sort_keys=False, width=78))
    fixtures_path.write_text(
        "\n".join(fixtures) + "\n")
    (run_dir / "scenario.json").write_text(json.dumps(scenario, indent=1))

    # emit steps
    steps = []
    step_order = []  # (sid, kind, payload)
    for p in prompts:
        pid = p["prompt_id"]
        lane = p["meta"]["lane"]
        conf_sid = f"conf_{pid}"
        lane_sids = {}
        for ln, chain in LANES.items():
            sids = []
            for i, (stage, mkey, mt, style) in enumerate(chain):
                sids.append((f"s{lane[0].lower()}{i}_{stage}_{pid}",
                             stage, mkey, mt, style, ln))
            lane_sids[ln] = sids
        rl_sid = f"routelog_{pid}"
        heads = {ln: lane_sids[ln][0][0] for ln in LANES}
        heads["LIGHT"] = heads["LIGHT"]  # default fall-through lane
        nxt_after = {ln: rl_sid for ln in LANES}

        step_order.append((conf_sid, "gate",
                           (pid, run_dir, heads)))
        for ln in ("HEAVY", "SYNTH", "LIGHT"):
            chain = lane_sids[ln]
            for i, (sid, stage, mkey, mt, style, _) in enumerate(chain):
                nxt = chain[i + 1][0] if i + 1 < len(chain) \
                    else nxt_after[ln]
                step_order.append((sid, "stage",
                                   (pid, stage, i, mkey, mt, style,
                                    nxt, run_dir, ln)))
        step_order.append((rl_sid, "routelog", (pid, run_dir)))

    for pos, (sid, kind, payload) in enumerate(step_order):
        if kind == "gate":
            pid, rd, heads = payload
            steps.append(gate_block(sid, rd / f"probe-{pid}.json",
                                    list(heads.items()), rd))
        elif kind == "stage":
            pid, stage, i, mkey, mt, style, nxt, rd, ln = payload
            cmd = (spoof_cmd(pid, stage, i, rd) if a.spoof
                   else live_cmd(pid, stage, style, rd))
            if not a.spoof and style == "cot":
                mt = max(mt, 2200)
            chain_last = nxt == f"routelog_{pid}"
            steps.append(stage_block(
                sid, nxt, mkey, mt, cmd, None, rd,
                save_name=f"{pid}-{stage}" if not a.spoof else None,
                live=not a.spoof,
                after_route=nxt if chain_last else None))
        else:  # routelog: spoof-emit writes route truth, then check
            pid, rd = payload
            nxt = step_order[pos + 1][0] if pos + 1 < len(step_order) \
                else "terminal"
            cmd = spoof_cmd(pid, "routelog", 1, rd) + " --force"
            chk = (f"python3 {SCRIPTS}/check-runner.py --case-yml "
                   f"{FAS}/top-level/cases/case-{pid}.yml --case {pid} "
                   f"--stage routelog --text-file "
                   f"{rd}/ans-{pid}-routelog.txt --out "
                   f"{rd}/check-{pid}-routelog.json > "
                   f"{rd}/check-{pid}-routelog.log 2>&1; exit 0")
            if not a.spoof:
                cmd = live_cmd(pid, "routelog", "routelog", rd)
            steps.append(stage_block(sid, nxt, "fmt", 300,
                                     cmd, chk, rd,
                                     save_name=f"{pid}-routelog" if not
                                     a.spoof else None,
                                     live=not a.spoof))

    steps.append(
        "    terminal:\n"
        "      generative_entity: \"${models.m_fmt}\"\n"
        "      prompt: terminal\n"
        "      model_overrides:\n        temperature: 0.0\n"
        "        max_tokens: 3\n"
        "      when:\n        before_step_starts:\n"
        "          - skip_step: true")

    yml = "\n".join([
        f"workflow_id: fas_meta_system_{mode}",
        "name: " + yq(f"FAS top-level meta system ({mode})"),
        "description: " + yq(
            "Stitched top-level control flow: conf gate -> lane heads "
            "(GWT route_to) -> per-lane atom chains -> checked routelog"),
        "version: \"1.0.0\"",
        "author: \"FAS experiment suite\"",
        "tags: [sub-workflow, fas, meta-system]",
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
        # schema L562-574: resource admission contract (rule 9)
        "  resource_admission:",
        "    enforcement_policy: block",
        "    minimum_available:",
        "      ram: 6GiB",
        "      vram: 6GiB",
        "      swap_free: 4GiB",
        "    model_estimate:",
        f"      kv_cache: 2.1GiB  # {SOURCE_PATH.name}:"
        " 2.33GiB weights, ctx 32768 q8_0",
        "      compute_buffer: 512MiB",
        "      host_runtime: 700MiB",
        f"      expected_runtime_secs: {400 * max(1, len(prompts))}",
        "    telemetry:",
        "      write_profile: true",
        "models:",
    ] + [f"  m_{k}:\n    name: {yq(v)}\n"
          f"    source_path: {SOURCE_PATH}\n    host:\n"
          f"      type: llama_cpp_with_vulkan" for k, v in MODELS.items()]
        + ["agentic_workflow:",
           "  when:",
           "    after_workflow:",
           "      - log:",
           "          level: info",
           f"          to_file_path: {run_dir}/final.log",
           "          event_fields: [workflow_id, total_steps, "
           "succeeded, failed]",
           "  steps:"]) + "\n" + "\n".join(steps) + "\n"

    if not SOURCE_PATH.exists() or SOURCE_PATH.stat().st_size == 0:
        sys.exit(f"fatal: model source missing or empty: {SOURCE_PATH}")

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(yml)
    lanes = [p["meta"]["lane"] for p in prompts]
    print(f"wrote {out}: {len(prompts)} prompts, {len(steps)} steps, "
          f"lanes H={lanes.count('HEAVY')} S={lanes.count('SYNTH')} "
          f"L={lanes.count('LIGHT')}")


if __name__ == "__main__":
    main()
