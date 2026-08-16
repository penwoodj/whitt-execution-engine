#!/usr/bin/env python3
"""Generate the batch-enhance workflow: ALL benchmark cases in ONE run
with exactly TWO model loads (Phase A 1.7B decompose x50, then Phase B/C
4B solve/synth x50) instead of 100 loads across 50 per-case runs.

Per case: decompose (1.7B) -> solve (4B, factored) -> synth r1 + check
(gwt pass -> case's select step, skipping r2) -> synth r2 + check ->
zero-LLM select-best. Gate/planverify dropped (drafts known-failing;
planverify was observability-only). toolverify folded into r1's
before_step_starts shell chain. Paths are ABSOLUTE (only __REPO_ROOT__
placeholder remains, sed-substituted by the runner).

Reads results/benchmark/baseline-rea-bNN.json markers; writes runtime
cases (draft = 9B baseline answer) under results/benchmark/batch-runtime/
and workflows/batch-enhance.yml.

Usage: gen-batch-enhance.py [--out workflows/batch-enhance.yml]
"""

import argparse
import json
import sys
from pathlib import Path

import yaml

REA = Path(__file__).parent.parent
BENCH = REA / "results" / "benchmark"
CASES = REA / "cases" / "benchmark"
RUNTIME = BENCH / "batch-runtime"

DEC_STEP = """    c{num}_dec:
      generative_entity: "${{models.decomposer}}"
      prompt: |
        ROLE: TASK DECOMPOSER — split the task into answerable sub-questions.

        LENS: The task is a checklist of facts and steps. Missing one = wrong answer.

        RECOGNIZE:
        - Numbers that must be derived (counts, sums, differences)
        - Multiple requirements bundled in one sentence
        - Constraints (budgets, limits, retention rules)
        - Required output format (bullets, YAML, fields)

        DECOMPOSE RULES:
        - One fact or one step per sub-question.
        - 3 to 5 sub-questions, together covering the WHOLE task.
        - Each answerable from the task + source material alone.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - Output ONLY the JSON object. No preamble, no commentary.
      model_overrides: {{ temperature: 0.3, max_tokens: 300 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase decompose --out {run}/decompose-prompt.txt; EXIT=$?; echo $EXIT > {run}/decompose-prompt.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/subquestions.txt"
"""

SOLVE_STEP = """    c{num}_solve:
      generative_entity: "${{models.sub_solver}}"
      prompt: |
        ROLE: SUB-QUESTION SOLVER — answer each sub-question exactly.

        LENS: Each sub-question is independent. Derive, never guess.

        RECOGNIZE:
        - Arithmetic hidden in words (totals, differences, splits)
        - Unit and count precision
        - Answers traceable to source material
        - Sub-question ordering

        SOLVE RULES:
        - Derive numbers step by step; write only the result.
        - Show derived numbers as inline arithmetic (42 - 31 - 5 = 6).
        - Use ONLY the source material provided above.
        - Terse and exact — no reasoning narration.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - One numbered answer line per sub-question, same numbering.
        - No preamble, no commentary, no restating questions.
      model_overrides: {{ temperature: 0.2, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase solve --out {run}/solve-prompt.txt; EXIT=$?; echo $EXIT > {run}/solve-prompt.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $text_state
              - "{run}/subanswers.txt"
"""

R1_STEP = """    c{num}_r1:
      generative_entity: "${{models.synthesizer}}"
      prompt: |
        ROLE: ANSWER SYNTHESIZER — write the final deliverable.

        LENS: Sub-answers are authoritative; the draft is a suspect starting point.

        RECOGNIZE:
        - Open failures listed below — each is a concrete defect to fix
        - Draft numbers disagreeing with sub-answers
        - Missing requirements and format breaks
        - Forbidden phrases and meta-language

        SYNTH RULES:
        - Fix every open failure listed above.
        - Prefer the SUB-ANSWERS over the draft when they disagree.
        - Re-derive numbers from source when unsure.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - Output the FULL final deliverable (complete text, not a diff).
        - The output IS the deliverable — never describe or announce it.
        - State each number or amount exactly once; never restate a value
          or mention any total.
        - Obey EVERY constraint in HARD OUTPUT RULES exactly, including
          word windows and character-exact strings.
        - First character must match the required output format.
        - No preamble. No commentary. No instructions.
      model_overrides: {{ temperature: 0.4, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase toolverify --out {run}/verify.json || true; EXIT=$?; echo $EXIT > {run}/toolverify.exit; exit $EXIT"
              working_dir: "__REPO_ROOT__"
              fail_on_error: false
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase synthesize --round 1 --out {run}/synthesize-r1-prompt.txt; EXIT=$?; echo $EXIT > {run}/synthesize-r1-prompt.exit; exit $EXIT"
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
              - given: "{{bookmarks.shell_output.exit_code}} == 0"
                then: c{num}_sel
"""

R2_STEP = """    c{num}_r2:
      generative_entity: "${{models.synthesizer_r2}}"
      prompt: |
        ROLE: SKEPTICAL AUDITOR — re-derive, then write the deliverable.

        LENS: The prior attempt failed real checks. Distrust it completely.

        RECOGNIZE:
        - Each listed prior failure — the exact defect still to fix
        - Numbers the prior attempt got wrong
        - Requirements it skipped or answered partially
        - Meta-language, hedging, forbidden phrases

        AUDIT RULES:
        - Re-derive every number and fact from the source material yourself.
        - Verify counts and sums before writing.
        - Fix every listed failure.

        {{{{bookmarks.shell_output.stdout}}}}

        RULES:
        - Output the FULL final deliverable (complete text, not a diff).
        - The output IS the deliverable — never describe or announce it.
        - State each number or amount exactly once; never restate a value
          or mention any total.
        - Obey EVERY constraint in HARD OUTPUT RULES exactly, including
          word windows and character-exact strings.
        - First character must match the required output format.
        - No preamble. No commentary. No instructions. This is the final attempt.
      model_overrides: {{ temperature: 0.7, max_tokens: 500 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 __REPO_ROOT__/experiments/reasoning-enhancer/scripts/emit-case.py --case {case} --run-dir {run} --phase synthesize --round 2 --out {run}/synthesize-r2-prompt.txt; EXIT=$?; echo $EXIT > {run}/synthesize-r2-prompt.exit; exit $EXIT"
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
"""

SEL_STEP = """    c{num}_sel:
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
      generative_entity: "${{models.synthesizer}}"
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

HEADER = """# Generated by gen-batch-enhance.py — {n} benchmark cases, ONE run,
# TWO model loads (1.7B x50 decompose -> 4B x50 solve/synth(+r2 when
# r1 checks fail via gwt routing to the case's select step)).
workflow_id: rea_batch_enhance
name: "REA Batch Enhance — 50 Cases, 2 Loads"
description: "Phase-grouped enhance: all decomposes, all solves, then per-case r1/[r2]/select with early-exit routing."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, enhance, batch]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config: {{ host: localhost, port: 8080 }}

models:
  "decomposer":
    name: "Qwen3-1.7B-abliterated-q4_k_m"
    host: {{ type: llama_cpp_with_vulkan }}
    sampling: {{ temperature: 0.3, max_tokens: 300 }}
  "sub_solver":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host: {{ type: llama_cpp_with_vulkan }}
    sampling: {{ temperature: 0.2, max_tokens: 500 }}
  "synthesizer":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host: {{ type: llama_cpp_with_vulkan }}
    sampling: {{ temperature: 0.4, max_tokens: 500 }}
  "synthesizer_r2":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host: {{ type: llama_cpp_with_vulkan }}
    sampling: {{ temperature: 0.7, max_tokens: 500 }}

workflow_execution_strategy:
  memory: {{ model_lifecycle: {{ unload_unused: true }} }}

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "{summary_log}"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info

  steps:
"""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(REA / "workflows" / "batch-enhance.yml"))
    ap.add_argument("--cases", default="",
                    help="comma-separated case numbers (1-50), e.g. '1,2,5' — default all baselined")
    args = ap.parse_args()

    if args.cases:
        wanted = {int(x) for x in args.cases.split(",") if x.strip()}
    else:
        wanted = None

    ready = []
    RUNTIME.mkdir(parents=True, exist_ok=True)
    for cf in sorted(CASES.glob("case-b*.yml")):
        num = int(cf.stem.replace("case-b", ""))
        if wanted is not None and num not in wanted:
            continue
        case = yaml.safe_load(cf.read_text())
        cid = case["case_id"]
        bl = BENCH / f"baseline-{cid}.json"
        if not bl.exists():
            continue
        run = RUNTIME / f"b{num:02d}"
        run.mkdir(parents=True, exist_ok=True)
        rcase = run / "runtime-case.yml"
        case["draft_response"] = json.loads(bl.read_text())["answer"]
        rcase.write_text(yaml.safe_dump(case, sort_keys=False, width=100))
        ready.append((f"{num:02d}", str(rcase.resolve()), str(run.resolve())))

    if not ready:
        print("no baselined cases found — run baseline phase first")
        return 1

    summary_log = str((RUNTIME / "batch-summary.log").resolve())
    parts = [HEADER.format(n=len(ready), summary_log=summary_log)]
    for num, case, run in ready:
        parts.append(DEC_STEP.format(num=num, case=case, run=run))
    for num, case, run in ready:
        parts.append(SOLVE_STEP.format(num=num, case=case, run=run))
    for num, case, run in ready:
        parts.append(R1_STEP.format(num=num, case=case, run=run))
        parts.append(R2_STEP.format(num=num, case=case, run=run))
        parts.append(SEL_STEP.format(num=num, case=case, run=run))
    parts.append(FINALIZE_STEP.format(summary_log=summary_log))
    Path(args.out).write_text("".join(parts))
    llm = 4 * len(ready)
    print(f"generated {args.out}: {len(ready)} cases, "
          f"{5 * len(ready) + 1} steps, {llm} LLM steps (r2 skipped via gwt when r1 passes)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
