#!/usr/bin/env python3
"""Generate v4/v5/v6 technique workflows for ha-* cases (live or spoof).

v4: per-entity solve steps + deterministic aggregate
v5: single table solve + table gate + json_exact on model JSON
v6: single table solve + table gate + DETERMINISTIC aggregate step + judge

Spoof mode (REA+ M1): every LLM step gets before_step_starts
[spoof-write canned artifact, REAL gate shells, GWT routes, skip_step] —
engine runs fully, zero inference. Writes canned text files per scenario.

Usage: gen-v6.py --case ha-01 --technique v6 [--spoof] [--scenario wind0]
       gen-v6.py --all --technique v6 --spoof
"""
import argparse
import json
import os
import sys
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent.parent
CASES_DIR = str(HERE / "cases")
WORKFLOW_DIR = HERE / "workflows"
SPOOF_DIR = HERE / "fixtures" / "spoof-v6"
LIVE_OUT = "./docs/benchmarks/outputs/output"
SPOOF_OUT = "./docs/benchmarks/outputs/output/spoof-v6"
SCRIPTS = "experiments/harness-inspired/scripts"
WHITT_MODEL = "Qwen3-5-9B-Q4_K_M"


def correct_table_text(case):
    lines = []
    for e in case["v6"]["entities"]:
        lines.append(f"ENTITY {e['id']} | {json.dumps(e['answer'])}")
    return "\n".join(lines) + "\n"


def wrong_table_text(case):
    ents = case["v6"]["entities"]
    lines = []
    for i, e in enumerate(ents):
        if i == len(ents) - 1:
            continue
        ans = e["answer"]
        if isinstance(ans, int):
            ans = ans + 7
        elif isinstance(ans, dict):
            ans = {k: (v + 7 if isinstance(v, int) else v) for k, v in ans.items()}
        lines.append(f"ENTITY {e['id']} | {json.dumps(ans)}")
    return "\n".join(lines) + "\n"


def entity_block(case):
    return "\n".join(f"        - {e['id']}: {e['question']}" for e in case["v6"]["entities"])


def write_scenarios(case, out_dir):
    """Canned artifacts per scenario: which stages emit correct tables."""
    S = Path(out_dir)
    S.mkdir(parents=True, exist_ok=True)
    cid = case["case_id"]
    good, bad = correct_table_text(case), wrong_table_text(case)
    scen = {
        "wind0": {"solve": good, "fix_1": good, "fix_2": good},
        "wind1": {"solve": bad, "fix_1": good, "fix_2": good},
        "wind2": {"solve": bad, "fix_1": bad, "fix_2": good},
        "lose":  {"solve": bad, "fix_1": bad, "fix_2": bad},
    }
    for name, stages in scen.items():
        for stage, text in stages.items():
            (S / f"{cid}-{stage}-{name}.txt").write_text(text)
    (S / f"{cid}-judge.txt").write_text("VERDICT: PASS\nREASON: spoof\nSCORE: 9\n")


def spoof_gate_chain(canned_file, case_path, gate_label, case_id, out_dir, next_pass, next_fail):
    """Hook list for spoofed LLM steps: canned write + real gates + skip."""
    return f'''          - shell:
              command: "python3 {SCRIPTS}/spoof-write.py --artifact {out_dir}/ha-table.txt --text-file {SPOOF_DIR_NAME}/{canned_file}"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "python3 {SCRIPTS}/check-table.py --artifact {out_dir}/ha-table.txt --case {case_path} --out {out_dir}/ha-table-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: {next_pass}
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: {next_fail}
          - shell:
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-table-result.json {gate_label} {out_dir}/ha-trace.jsonl {case_id}"
              working_dir: "."
              fail_on_error: false
          - skip_step: true'''


SPOOF_DIR_NAME = "experiments/harness-inspired/fixtures/spoof-v6"


def gen_v6(case, spoof, scenario="wind0"):
    cid = case["case_id"]
    case_path = f"{CASES_DIR}/{cid}.yml"
    out_dir = SPOOF_OUT if spoof else LIVE_OUT
    ents = entity_block(case)
    stage_files = {
        "solve": f"{cid}-solve-{scenario}.txt" if spoof else None,
        "fix_1": f"{cid}-fix_1-{scenario}.txt" if spoof else None,
        "fix_2": f"{cid}-fix_2-{scenario}.txt" if spoof else None,
    }

    if spoof:
        solve_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["solve"], case_path, "check", cid, out_dir, "s04_aggregate", "s02_fix_1")}
'''
        fix1_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["fix_1"], case_path, "fix_1", cid, out_dir, "s04_aggregate", "s03_fix_2")}
'''
        fix2_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["fix_2"], case_path, "fix_2", cid, out_dir, "s04_aggregate", "s06_end_fail")}
'''
        judge_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/spoof-write.py --artifact {out_dir}/ha-verdict.txt --text-file {SPOOF_DIR_NAME}/{cid}-judge.txt"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "python3 {SCRIPTS}/trace-judge.py {out_dir}/ha-verdict.txt {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
          - skip_step: true
'''
    else:
        solve_hooks = f'''      when:
        after_step_succeeds:
          - save_to:
              - "$table"
              - "{out_dir}/ha-table.txt"
          - shell:
              command: "python3 {SCRIPTS}/check-table.py --artifact {out_dir}/ha-table.txt --case {case_path} --out {out_dir}/ha-table-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s04_aggregate
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s02_fix_1
          - shell:
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-table-result.json check {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
'''
        fix1_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
        after_step_succeeds:
          - save_to:
              - "$table"
              - "{out_dir}/ha-table.txt"
          - shell:
              command: "python3 {SCRIPTS}/check-table.py --artifact {out_dir}/ha-table.txt --case {case_path} --out {out_dir}/ha-table-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s04_aggregate
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s03_fix_2
          - shell:
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-table-result.json fix_1 {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
'''
        fix2_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
        after_step_succeeds:
          - save_to:
              - "$table"
              - "{out_dir}/ha-table.txt"
          - shell:
              command: "python3 {SCRIPTS}/check-table.py --artifact {out_dir}/ha-table.txt --case {case_path} --out {out_dir}/ha-table-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s04_aggregate
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s06_end_fail
          - shell:
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-table-result.json fix_2 {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
'''
        judge_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "cat {out_dir}/ha-final.json"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$final_json"
        after_step_succeeds:
          - save_to:
              - "$verdict"
              - "{out_dir}/ha-verdict.txt"
          - shell:
              command: "python3 {SCRIPTS}/trace-judge.py {out_dir}/ha-verdict.txt {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
'''

    aggregate_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/aggregate.py --case {case_path} --table-result {out_dir}/ha-table-result.json --out {out_dir}/ha-final.json"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "python3 {SCRIPTS}/check-deterministic.py --artifact {out_dir}/ha-final.json --criteria {case_path} --out {out_dir}/ha-final-check.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s05_judge
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s06_end_fail
          - shell:
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-final-check.json final {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
          - skip_step: true
'''

    end_fail_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/failflag.py --result {out_dir}/ha-table-result.json --final-check {out_dir}/ha-final-check.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s07_fail_trace
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s_end
          - skip_step: true
'''

    fail_trace_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl end_fail fail case={cid}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '1 == 1'
                then: s_end
'''

    end_step_hooks = '''      when:
        before_step_starts:
          - skip_step: true
'''

    prompt = case["prompt"].strip()
    prompt_ind = "\n".join("        " + l for l in prompt.split("\n"))

    return f'''workflow_id: harness_v6_{cid}{'_spoof' if spoof else ''}
name: "Harness v6 {'SPOOF ' if spoof else ''}- {cid}"
description: "TABLE->GATE->DETERMINISTIC-AGGREGATE->JUDGE. Case: {cid}"
version: "6.0.0"
author: "Experiment"
tags: [harness, agentic, v6, table, aggregate{' ,spoof' if spoof else ''}]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  m_worker:
    name: "{WHITT_MODEL}"
    host:
      type: llama_cpp_with_vulkan
  m_judge:
    name: "{WHITT_MODEL}"
    host:
      type: llama_cpp_with_vulkan

workflow_execution_strategy:
  timing:
    cooldown_after_unload_secs: 0
    min_tmp_space_mb: 50
  memory:
    model_lifecycle:
      unload_unused: false

agentic_workflow:
  when:
    after_workflow:
      - log:
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    s00_screen:
      generative_entity: "${{models.m_worker}}"
      prompt: "pre-flight"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/screen-validate.py {case_path}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s06_end_fail
          - skip_step: true

    s01_solve:
      generative_entity: "${{models.m_worker}}"
      prompt: |
        ROLE: on-call operator. Precise. Zero fluff.

        {prompt_ind}

        METHOD: for EACH entity below, derive its answer from the rules.
        Track state step by step in plain text. Entities you do not manage
        NEVER count. Check deadline/budget rules BEFORE finalizing.
        Then output ONLY these lines, one per entity, nothing else:
        ENTITY <id> | <answer>

        Entities:
{ents}
      model_overrides: {{ temperature: 0.1, max_tokens: 1280 }}
{solve_hooks}
    s02_fix_1:
      generative_entity: "${{models.m_worker}}"
      prompt: |
        ROLE: correction specialist. Entity table failed checks. Fix rows.

        Failed cells (id + your observed value, correct values withheld):
        {{{{bookmarks.cell_feedback}}}}

        Prior table:
        {{{{bookmarks.table}}}}

        METHOD: re-derive each failed entity from task rules. Output ONLY
        the full corrected table: one ENTITY line per entity.
      model_overrides: {{ temperature: 0.0, max_tokens: 1280 }}
{fix1_hooks}
    s03_fix_2:
      generative_entity: "${{models.m_worker}}"
      prompt: |
        ROLE: escalation solver. Two attempts wrong. Rebuild table from scratch.

        Original task:
        {prompt_ind}

        Failed cells (id + observed, withheld correct):
        {{{{bookmarks.cell_feedback}}}}

        METHOD: discard prior attempts. Re-derive EVERY entity answer from
        the rules. Output ONLY the full table: one ENTITY line per entity.
      model_overrides: {{ temperature: 0.0, max_tokens: 1600 }}
{fix2_hooks}
    s04_aggregate:
      generative_entity: "${{models.m_worker}}"
      prompt: "deterministic-aggregate-noop"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{aggregate_hooks}
    s05_judge:
      generative_entity: "${{models.m_judge}}"
      prompt: |
        ROLE: blind auditor. See only task + result. No author knowledge.

        Task objective: {case.get('objective', '')}
        Result: {{{{bookmarks.final_json}}}}

        Check: does result answer task completely? Values plausible?
        Output EXACTLY 3 lines:
        VERDICT: PASS or FAIL
        REASON: one sentence
        SCORE: 1-10
      model_overrides: {{ temperature: 0.0, max_tokens: 128 }}
{judge_hooks}
    s06_end_fail:
      generative_entity: "${{models.m_worker}}"
      prompt: "FAIL"
      model_overrides: {{ temperature: 0.0, max_tokens: 8 }}
{end_fail_hooks}
    s07_fail_trace:
      generative_entity: "${{models.m_worker}}"
      prompt: "trace-only"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{fail_trace_hooks}
    s_end:
      generative_entity: "${{models.m_worker}}"
      prompt: "term"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{end_step_hooks}'''


def load_case(cid):
    return yaml.safe_load(Path(CASES_DIR, f"{cid}.yml").read_text())


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--technique", default="v6", choices=["v6"])
    ap.add_argument("--spoof", action="store_true")
    ap.add_argument("--scenario", default="wind0", choices=["wind0", "wind1", "wind2", "lose"])
    args = ap.parse_args()

    cases = [f"ha-{i:02d}" for i in range(1, 11)] if args.all else [args.case]
    WORKFLOW_DIR.mkdir(parents=True, exist_ok=True)

    for cid in cases:
        case = load_case(cid)
        if args.spoof:
            write_scenarios(case, SPOOF_DIR)
        yml = gen_v6(case, args.spoof, args.scenario)
        suffix = "-spoof" if args.spoof else ""
        path = WORKFLOW_DIR / f"v6-{cid}{suffix}.yml"
        path.write_text(yml)
        print(f"generated {path}")


if __name__ == "__main__":
    main()
