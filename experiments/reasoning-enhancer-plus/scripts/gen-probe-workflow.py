#!/usr/bin/env python3
"""Generate probe workflow YAML for one model over the probe battery.

Usage: gen-probe-workflow.py --model MODEL [--out FILE] [--limit N]

Pattern follows reasoning-enhancer/workflows/baseline-9b.yml: N steps,
each = before-shell emit-probe (stdout -> prompt bookmark + fingerprint
append) -> inference (t0.0) -> save answer -> check-answer shell
(fail_on_error false). check-answer/check_lib imported from
reasoning-enhancer (single source of deterministic-check truth).
"""
import argparse
import sys
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REA_SCRIPTS = EXP.parent / "reasoning-enhancer" / "scripts"
CASES_DIR = EXP / "cases/probe"
STAGE2_DIR = EXP / "cases/stage2-train"

TEMPLATE = """\
workflow_id: rea_plus_probe
name: "REA+ Probe — {model}"
description: "Single-model capability probe over 20-item battery."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [rea-plus, probe, model-selection]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config: {{ host: localhost, port: 8080 }}

models:
  "probe_model":
    name: "{model}"
    host: {{ type: llama_cpp_with_vulkan }}
    sampling: {{ temperature: 0.0, max_tokens: {max_tok} }}

workflow_execution_strategy:
  memory: {{ model_lifecycle: {{ unload_unused: false }} }}

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "__OUTPUT_DIR__/last-run.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info

  steps:
{steps}

    step_99_finalize:
      generative_entity: "${{models.probe_model}}"
      prompt: "OK."
      model_overrides: {{ temperature: 0.0, max_tokens: 3 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 {repo}/scripts/collect-probe.py --run-dir __OUTPUT_DIR__ > __OUTPUT_DIR__/summary.log 2>&1; EXIT=$?; echo $EXIT > __OUTPUT_DIR__/summary.exit; exit 0"
              working_dir: "{repo}"
              fail_on_error: false
          - skip_step: true
        after_step_succeeds:
          - log:
              to_file_path: "__OUTPUT_DIR__/probe-complete.log"
              event_fields: [workflow_id]
              level: info
"""

STEP = """\
    step_{idx:02d}_{cid}:
      generative_entity: "${{models.probe_model}}"
      prompt: |
        {{{{bookmarks.shell_output.stdout}}}}
      model_overrides: {{ temperature: 0.0, max_tokens: {max_tok} }}
      when:
        before_step_starts:
          - shell:
              command: "python3 {repo}/scripts/emit-probe.py --case {case} --directive '{directive}' --out __OUTPUT_DIR__/prompt-{cid}.txt | python3 {repo}/scripts/fingerprint.py --case {cid} --step step_{idx:02d}_{cid} --run-dir __OUTPUT_DIR__; EXIT=$?; echo $EXIT > __OUTPUT_DIR__/emit-{cid}.exit; exit $EXIT"
              working_dir: "{repo}"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "__OUTPUT_DIR__/ans-{cid}.txt"
          - shell:
              command: "python3 {rea}/check-answer.py --case {case} --text-file __OUTPUT_DIR__/ans-{cid}.txt --out __OUTPUT_DIR__/check-{cid}.json > __OUTPUT_DIR__/check-{cid}.log 2>&1; EXIT=$?; echo $EXIT > __OUTPUT_DIR__/check-{cid}.exit; exit 0"
              working_dir: "{repo}"
              fail_on_error: false
"""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", required=True)
    ap.add_argument("--out")
    ap.add_argument("--limit", type=int)
    ap.add_argument("--out-dir", dest="out_dir")
    ap.add_argument("--stage2", action="store_true",
                    help="use stage2-train cases instead of probe battery")
    ap.add_argument("--cases-dir", help="override case directory")
    ap.add_argument("--directive", default="",
                    help="soft-switch line passed to emit-probe (e.g. /no_think)")
    ap.add_argument("--max-tok", type=int,
                    help="override max_tokens budget")
    args = ap.parse_args()

    repo = str(EXP)
    out_dir = args.out_dir or str(EXP / "results/model-probe" / args.model)
    max_tok = args.max_tok if args.max_tok else (
        3500 if "hinking" in args.model else 250)
    directive = args.directive
    Path(out_dir).mkdir(parents=True, exist_ok=True)
    if args.cases_dir:
        case_dir = Path(args.cases_dir)
    elif args.stage2:
        case_dir = STAGE2_DIR
    else:
        case_dir = CASES_DIR
    cases = sorted(case_dir.glob("case-*.yml"))
    if args.limit:
        cases = cases[: args.limit]

    steps = []
    for idx, path in enumerate(cases, start=1):
        cid = path.stem.replace("case-", "")
        steps.append(STEP.format(
            idx=idx, cid=cid, case=path, repo=repo,
            rea=REA_SCRIPTS, max_tok=max_tok, directive=directive))

    body = TEMPLATE.format(model=args.model, steps="\n".join(steps),
                            repo=repo, max_tok=max_tok)
    doc = body.replace("__OUTPUT_DIR__", out_dir)
    out = Path(args.out) if args.out else \
        EXP / "workflows" / f"probe-{args.model}.yml"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(doc)
    yaml.safe_load(doc)
    print(f"wrote {out} ({len(cases)} steps, max_tokens={max_tok})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
