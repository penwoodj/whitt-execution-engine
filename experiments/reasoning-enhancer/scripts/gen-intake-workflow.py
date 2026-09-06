#!/usr/bin/env python3
"""Generate the over-context INTAKE workflow (InfiniRetri-mapped), v3.1:
per case — N sequential chunk-intake steps (4B). Each chunk: resume guard
(already-ingested chunk -> before-shell exit 7 -> GWT skip to next chunk),
inference, then evidence-sufficiency gate (evidence-status.py exit-7 ->
GWT skips remaining chunks to synth). Then cascade synth r1 -> check ->
[r2 -> check] -> [r3 targeted grep rescue -> check] -> select-best. All 4B:
ONE model load per run. Cases given by --cases '1,2,3'; keep <= 5 cases
per run (engine executes <= 100 workflow-loop iterations; v3.1 worst
case is 17 steps/case).

Also generates oc-baseline.yml with --mode baseline: one 9B step per
case fed the FULL document (exceeds the 32k window — demonstrating the
raw-model limit; server truncates or errors, either way checks fail).

Usage:
  gen-intake-workflow.py --cases 1,2,3,4,5 [--out workflows/oc-intake.yml]
  gen-intake-workflow.py --mode baseline [--out workflows/oc-baseline.yml]
"""

import argparse
import sys
from pathlib import Path

import yaml

REA = Path(__file__).parent.parent
CASES = REA / "cases" / "overcontext"

INTAKE_STEP = """    c{nn}_i{cc:02d}:
      generative_entity: "${{models.reader}}"
      prompt: |
        You are the INTAKE READER for a long document.

        {{{{bookmarks.shell_output.stdout}}}}

        Follow the TASK NOW instruction in the context above exactly.
      model_overrides: {{ temperature: 0.2, max_tokens: 450 }}
      when:
        before_step_starts:
          - shell:
              command: "if [ -f {run}/i{cc:02d}-merge.exit ]; then exit 7; else exit 0; fi"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - gwt:
              - given: "{{{{bookmarks.shell_output.exit_code}}}} == 7"
                then: {next_step}
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase intake --chunk {cc}; EXIT=$?; echo $EXIT > {run}/i{cc:02d}.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/intake-last.txt"
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/intake-merge.py --run-dir {run} > {run}/i{cc:02d}-merge.log 2>&1; echo $? > {run}/i{cc:02d}-merge.exit; python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/evidence-status.py --case {case} --run-dir {run}"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - gwt:
              - given: "{{{{bookmarks.shell_output.exit_code}}}} == 7"
                then: c{nn}_r1
"""

R1_STEP = """    c{nn}_r1:
      generative_entity: "${{models.synthesizer}}"
      prompt: |
        ROLE: ANSWER SYNTHESIZER — write the final deliverable from the retained notes.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - The output IS the deliverable. No preamble. No commentary.
        - Obey EVERY constraint in HARD OUTPUT RULES exactly.
      model_overrides: {{ temperature: 0.4, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase oc-synth --round 1 --out {run}/oc-synth-r1-prompt.txt; EXIT=$?; echo $EXIT > {run}/oc-synth-r1-prompt.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/enhanced-answer-r1.txt"
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/check-answer.py --case {case} --text-file {run}/enhanced-answer-r1.txt --out {run}/check-r1.json > {run}/check-r1.log 2>&1; EXIT=$?; echo $EXIT > {run}/check-r1.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - gwt:
              - given: "{{{{bookmarks.shell_output.exit_code}}}} == 0"
                then: c{nn}_sel
"""

R2_STEP = """    c{nn}_r2:
      generative_entity: "${{models.synthesizer_r2}}"
      prompt: |
        ROLE: SKEPTICAL AUDITOR — fix the failed checks from the retained notes.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - The output IS the deliverable. No preamble. No commentary.
        - Obey EVERY constraint in HARD OUTPUT RULES exactly.
      model_overrides: {{ temperature: 0.7, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase oc-synth --round 2 --out {run}/oc-synth-r2-prompt.txt; EXIT=$?; echo $EXIT > {run}/oc-synth-r2-prompt.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/enhanced-answer-r2.txt"
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/check-answer.py --case {case} --text-file {run}/enhanced-answer-r2.txt --out {run}/check-r2.json > {run}/check-r2.log 2>&1; EXIT=$?; echo $EXIT > {run}/check-r2.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - gwt:
              - given: "{{{{bookmarks.shell_output.exit_code}}}} == 0"
                then: c{nn}_sel
"""

R3_STEP = """    c{nn}_r3:
      generative_entity: "${{models.synthesizer}}"
      prompt: |
        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - The output IS the deliverable. No preamble. No commentary.
        - Obey EVERY constraint in HARD OUTPUT RULES exactly.
      model_overrides: {{ temperature: 0.3, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/evidence-rescue.py --case {case} --run-dir {run} --out {run}/rescue-prompt.txt"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/enhanced-answer-r3.txt"
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/check-answer.py --case {case} --text-file {run}/enhanced-answer-r3.txt --out {run}/check-r3.json > {run}/check-r3.log 2>&1; EXIT=$?; echo $EXIT > {run}/check-r3.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
"""

SEL_STEP = """    c{nn}_sel:
      generative_entity: "${{models.synthesizer}}"
      prompt: "OK."
      model_overrides: {{ temperature: 0.0, max_tokens: 3 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/format-repair.py --run-dir {run} --case {case} || true; python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/select-best.py --case {case} --run-dir {run} > {run}/select-best.log 2>&1 || true"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - skip_step: true
"""

FINALIZE_STEP = """    step_99_finalize:
      generative_entity: "${{models.{entity}}}"
      prompt: "OK."
      model_overrides: {{ temperature: 0.0, max_tokens: 3 }}
      when:
        before_step_starts:
          - shell:
              command: "true"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
          - skip_step: true
        after_step_succeeds:
          - log:
              to_file_path: "{summary_log}"
              event_fields: [workflow_id, total_steps]
              level: info
"""

BASELINE_STEP = """    c{nn}_b:
      generative_entity: "${{models.baseline_model}}"
      prompt: |
        {{{{bookmarks.shell_output.stdout}}}}
      model_overrides: {{ temperature: 0.7, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase oc-baseline --out {run}/baseline-prompt.txt; EXIT=$?; echo $EXIT > {run}/baseline-prompt.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/baseline-answer.txt"
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/check-answer.py --case {case} --text-file {run}/baseline-answer.txt --out {run}/check-baseline.json > {run}/check-baseline.log 2>&1; EXIT=$?; echo $EXIT > {run}/check-baseline.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
"""


def header(n, wid, desc, models, summary_log):
    m = ""
    for alias, (gguf, t, mt) in models.items():
        m += (f'  "{alias}":\n    name: "{gguf}"\n'
              f"    host: {{ type: llama_cpp_with_vulkan }}\n"
              f"    sampling: {{ temperature: {t}, max_tokens: {mt} }}\n")
    return (f"# Generated by gen-intake-workflow.py — {desc}\n"
            f"workflow_id: {wid}\nname: \"{n}\"\ndescription: \"{desc}\"\n"
            f"version: \"2.0.0\"\nauthor: \"Whitt Execution Engine\"\n"
            f"tags: [overcontext, intake]\nschema_version: \"2.0.0\"\n"
            f"min_schema_version: \"2.0.0\"\n\n"
            f"providers:\n  llama_cpp_with_vulkan:\n    config: {{ host: localhost, port: 8080 }}\n\n"
            f"models:\n{m}\n"
            f"workflow_execution_strategy:\n  memory: {{ model_lifecycle: {{ unload_unused: true }} }}\n\n"
            f"agentic_workflow:\n  when:\n    after_workflow:\n      - log:\n          to_file_path: \"{summary_log}\"\n          event_fields: [workflow_id, total_steps, succeeded, failed]\n          level: info\n\n  steps:\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--cases", default="")
    ap.add_argument("--mode", choices=["intake", "baseline"], default="intake")
    args = ap.parse_args()

    nums = [int(x) for x in args.cases.split(",") if x.strip()] if args.cases else list(range(1, 31))
    parts = []
    if args.mode == "baseline":
        models = {"baseline_model": ("Qwen3-5-9B-Q4_K_M", 0.7, 500)}
        sdir = REA / "results" / "overcontext" / "baseline-runtime"
        sdir.mkdir(parents=True, exist_ok=True)
        parts.append(header("OC Baseline", "rea_oc_baseline",
                            "9B fed the FULL over-window document directly",
                            models, str((sdir / "summary.log").resolve())))
        for nn in nums:
            run = sdir / f"o{nn:02d}"
            run.mkdir(parents=True, exist_ok=True)
            case = (CASES / f"case-o{nn:02d}.yml").resolve()
            parts.append(BASELINE_STEP.format(nn=f"{nn:02d}", case=case, run=run.resolve()))
        parts.append(FINALIZE_STEP.format(entity="baseline_model", summary_log=str((sdir / "summary.log").resolve())))
    else:
        models = {"reader": ("Qwen3-4B-Instruct-2507-Q4_K_M", 0.2, 450),
                  "synthesizer": ("Qwen3-4B-Instruct-2507-Q4_K_M", 0.4, 500),
                  "synthesizer_r2": ("Qwen3-4B-Instruct-2507-Q4_K_M", 0.7, 500)}
        sdir = REA / "results" / "overcontext" / "intake-runtime"
        sdir.mkdir(parents=True, exist_ok=True)
        parts.append(header("OC Intake", "rea_oc_intake",
                            f"Chunked intake funnel ({len(nums)} cases, one 4B load)",
                            models, str((sdir / "summary.log").resolve())))
        for nn in nums:
            run = sdir / f"o{nn:02d}"
            run.mkdir(parents=True, exist_ok=True)
            case = (CASES / f"case-o{nn:02d}.yml").resolve()
            n_chunks = yaml.safe_load(case.read_text())["chunk_count"]
            for cc in range(1, n_chunks + 1):
                nxt = f"c{nn:02d}_i{cc + 1:02d}" if cc < n_chunks else f"c{nn:02d}_r1"
                parts.append(INTAKE_STEP.format(nn=f"{nn:02d}", cc=cc, next_step=nxt,
                                                 case=case, run=run.resolve()))
            parts.append(R1_STEP.format(nn=f"{nn:02d}", case=case, run=run.resolve()))
            parts.append(R2_STEP.format(nn=f"{nn:02d}", case=case, run=run.resolve()))
            parts.append(R3_STEP.format(nn=f"{nn:02d}", case=case, run=run.resolve()))
            parts.append(SEL_STEP.format(nn=f"{nn:02d}", case=case, run=run.resolve()))
        parts.append(FINALIZE_STEP.format(entity="synthesizer", summary_log=str((sdir / "summary.log").resolve())))

    Path(args.out).write_text("".join(parts))
    steps = "".join(parts).count("\n    c") + 1
    print(f"generated {args.out}: {steps} steps for cases {nums}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
