#!/usr/bin/env python3
"""Generate the engine-native REA+ v6 workflow YAML.

Cases sorted by primary model (load reuse). Per case: case-major stage
chain primary -> extract -> d0 -> d1 -> d2 -> fextract; each stage
gates on its own check via stage-emit.py + GWT route-on-pass to the
next case's primary. Logging through log hooks per stage.
Usage: gen-rea3-native.py --out FILE [--cases-dir ...]* [--run-dir D]
"""
import argparse
import importlib.util as ilu
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REPO = EXP.parent.parent

spec = ilu.spec_from_file_location("r3", HERE / "rea3-execute.py")
r3 = ilu.module_from_spec(spec)
spec.loader.exec_module(r3)

CFG = r3.VERSIONS["v6"]
CHANNELS = CFG["channels"]
CHAINS = CFG["chains"]

M_SMALL = "Qwen3-1.7B-abliterated-q8_0"

channels_spec = {"pln": (CFG["channels"]["pln"][0], 250, False),
                 "audit": (CFG["channels"]["audit"][0], 2000, True)}

def think_budget(cid, cat, meta):
    """Deterministic per-case budget: hops metadata when present,
    else category defaults. Floor 1500 (easy band mean 1101),
    ceiling 5000 (finish mid-derivation heavy cases)."""
    hops = meta.get("hops")
    if hops is None:
        hops = {"derive": 4, "logic": 3, "fmt": 2, "audit": 3,
                "pln": 3, "general": 2}.get(cat, 3)
    if hops >= 4:
        return 5000
    if hops == 3:
        return 2500
    return 1500
M_FMT = r3.M_FMT
MODELS = sorted({m for m, _ in CHANNELS.values()}
                | {m for ch in CHAINS.values() for m, _, _ in ch})

SUITES = [("probe", EXP / "cases/probe"),
          ("band", EXP / "cases/stage2-train"),
          ("regression", EXP / "cases/regression")]


def stage_step(idx, cid, stage, model, tok, stepwise, run_dir, nxt, cpath):
    sid = f"s{idx:03d}_{stage}_{cid}"
    gate = (f"python3 {HERE}/stage-emit.py --case {cpath} "
            f"--stage {stage} --run-dir {run_dir}"
            + (" --stepwise" if stepwise else ""))
    lines = [
        f"    {sid}:",
        f"      generative_entity: \"${{models.m_{model}}}\"",
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
        "        after_step_succeeds:",
        "          - save_to:",
        "              - $text_state",
        f"              - \"{run_dir}/ans-{cid}-{stage}.txt\"",
        "          - shell:",
        f"              command: \"python3 {REPO}/experiments/reasoning-enhancer/scripts/check-answer.py --case {cpath} --text-file {run_dir}/ans-{cid}-{stage}.txt --out {run_dir}/check-{cid}-{stage}.json > {run_dir}/check-{cid}-{stage}.log 2>&1; exit 0\"",
        f"              working_dir: \"{REPO}\"",
        "              fail_on_error: false",
        "          - log:",
        "              level: info",
        "              to_file_path: \"" + run_dir + "/native-progress.log\"",
        "              event_fields: [step_name, model_name, duration_ms, token_count]",
    ]
    return sid, "\n".join(lines)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--order", choices=["case", "stage"], default="case")
    ap.add_argument("--v8", action="store_true",
                    help="v8: 1.7B screen + cheap-first cascade + dynamic thinking budget")
    ap.add_argument("--v9", action="store_true",
                    help="v9: v8 + singleton reroute + boundary-carry bucket order")
    args = ap.parse_args()

    cases = []
    for suite, d in SUITES:
        for p in sorted(d.glob("case-*.yml")):
            c = yaml.safe_load(p.read_text())
            cat = r3.classify_v2(
                f"{c.get('prompt','')} {c.get('auxiliary','')}")
            meta = c.get("rea_plus") or {}
            cases.append({"cid": c["case_id"], "path": str(p),
                          "cat": cat, "meta": meta})
    if args.v8:
        v8_channels = dict(CHANNELS)
        v8_channels["logic"] = (M_FMT, 250)
        v8_channels["derive"] = (M_FMT, 250)
        channels = v8_channels
        screen_cats = ("fmt", "pln", "general", "logic")
    else:
        channels = CHANNELS
        screen_cats = ()
    if args.v9:
        channels = dict(channels)
        channels["pln"] = (M_FMT, 250)
        channels["audit"] = (M_FMT, 250)
    order = {m: i for i, m in enumerate(
        [M_FMT, CHANNELS["derive"][0], CHANNELS["pln"][0],
         CHANNELS["general"][0], CHANNELS["audit"][0]])}
    cases.sort(key=lambda c: (order.get(CHANNELS[c["cat"]][0], 9),
                              c["cid"]))
    if args.limit:
        cases = cases[:args.limit]

    body = []
    nxt_map = {}
    stages_of = {}
    for i, c in enumerate(cases):
        cid, cat = c["cid"], c["cat"]
        pm, ptok = channels[cat]
        stages = []
        if args.v8 and cat in screen_cats:
            stages.append(("screen", M_SMALL, 250, False))
        stages.append(("primary", pm, ptok, False))
        stages.append(("extract", M_FMT, 150, False))
        chain = [tuple(x) for x in CHAINS.get(cat, [])]
        if args.v9 and cat in ("pln", "audit"):
            spec = channels_spec.get(cat)
            if spec:
                chain = [tuple(spec)] + chain
        if args.v8 and cat in ("logic", "derive"):
            chain = [(m, tok if "Thinking" not in m
                      else think_budget(cid, cat, c["meta"]), sw)
                     for m, tok, sw in chain]
            if not any("Bonsai" in m for m, _, _ in chain):
                chain = chain + [(r3.M_DEEP, 1500, True)]
        for m, tok, sw in chain:
            stages.append((f"d{len([s for s in stages if s[0].startswith('d')])}",
                           m, tok, sw))
        stages.append(("fextract", M_FMT, 150, False))
        stages_of[cid] = stages
        nxt = (f"s{{}}_primary_{cases[i+1]['cid']}" if i + 1 < len(cases)
               else "terminal")
        nxt_map[cid] = nxt
    if args.order == "stage":
        stage_names = (["screen"] if (args.v8 or args.v9) else []) + \
            ["primary", "extract", "d0", "d1", "d2", "d3", "fextract"]
        plan = []
        last_model = None
        for sn in stage_names:
            bucket = [(c, s) for c in cases for s in stages_of[c["cid"]]
                      if s[0] == sn]
            bucket.sort(key=lambda cs: (cs[1][1], cs[0]["cid"]))
            if args.v9 and bucket:
                head = [b for b in bucket if b[1][1] == last_model]
                rest = [b for b in bucket if b[1][1] != last_model]
                bucket = head + rest
            if bucket:
                last_model = bucket[-1][1][1]
            plan.extend(bucket)
    else:
        plan = [(c, s) for c in cases for s in stages_of[c["cid"]]]
    idx = 0
    emitted = []
    for c, (stage, m, tok, sw) in plan:
        idx += 1
        sid, block = stage_step(idx, c["cid"], stage, m, tok, sw,
                                args.run_dir, "PLACEHOLDER_NEXT",
                                c["path"])
        emitted.append((sid, block, c["cid"]))
    final = []
    for i, (sid, block, cid) in enumerate(emitted):
        nxt = emitted[i + 1][0] if i + 1 < len(emitted) else "terminal"
        final.append(block.replace("PLACEHOLDER_NEXT", nxt))

    model_defs = "\n".join(
        f"  m_{m}:\n    name: {m}\n    host:\n      type: llama_cpp_with_vulkan"
        for m in MODELS)

    wf = f"""workflow_id: rea3_native_v6
name: "REA+ v6 native — case-major GWT chains"
description: "Engine-native v6: stage gates + GWT route-on-pass + log hooks."
version: "6.0.0"
author: "Whitt Execution Engine"
tags: [rea-plus, native, gwt-routing]
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
  memory:
    model_lifecycle:
      unload_unused: false

models:
{model_defs}

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "{args.run_dir}/native-final.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
{chr(10).join(final)}
    terminal:
      generative_entity: "${{models.m_{M_FMT}}}"
      prompt: "terminal"
      model_overrides: {{ temperature: 0.0, max_tokens: 3 }}
"""
    Path(args.out).write_text(wf)
    n_stages = len(emitted)
    print(f"wrote {args.out}: {len(cases)} cases, {n_stages} stages + terminal")


if __name__ == "__main__":
    main()
