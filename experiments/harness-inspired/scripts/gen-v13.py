#!/usr/bin/env python3
"""v8 workflow generator for hc-100 (field-ops digest cases).

Deltas from v7:
1. Digest input: prompts embed task_core (RULES/FACTS/REPORT SPEC), not the
   1500-word prose. Deterministic render, zero LLM.
2. Exact preserved METHOD wording (proven 'step by step' prefix).
3. Fix prompts embed exact JSON FORM (keys+types, no values).
4. Heavy rescue chain: fix_2 (9B escalation) + fix_3 (heavy only, 9B,
   3072 tokens). Light cases stop at fix_2.
5. Fat token budgets: solve 2048, fix_1 2560, fix_2 3072, fix_3 3072.

Usage: gen-v8.py --case hc-01 [--spoof] [--scenario wind0]
       gen-v8.py --all [--spoof --scenario NAME]
"""
import argparse
import importlib.util
import json
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location("gen_v7", HERE / "gen-v7.py")
g7 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g7)

CASES_DIR = g7.CASES_DIR
WORKFLOW_DIR = g7.WORKFLOW_DIR
SPOOF_DIR = g7.SPOOF_DIR
LIVE_OUT = g7.LIVE_OUT
SPOOF_OUT = g7.SPOOF_OUT
SCRIPTS = g7.SCRIPTS
SPOOF_DIR_NAME = g7.SPOOF_DIR_NAME

SCENARIOS = g7.SCENARIOS

METHOD_EXACT = (
    "METHOD: for EACH entity below, derive its answer from the rules.\n"
    "Track state step by step in plain text. Entities you do not manage\n"
    "NEVER count. Check deadline/budget rules BEFORE finalizing."
)


def digest_block(case):
    tc = case["task_core"]
    lines = ["RULES:"]
    lines += [f"- {r}" for r in tc["rules"]]
    lines.append("")
    lines.append("FACTS:")
    lines += [f"- {f}" for f in tc["facts"]]
    lines.append("")
    lines.append("REPORT SPEC (exact keys, exact types):")
    lines += [f"- {k}: {t}" for k, t in tc["output_spec"].items()]
    return "\n".join(lines)


def json_form(case):
    tc = case["task_core"]
    parts = []
    for k, t in tc["output_spec"].items():
        if t == "bool":
            parts.append(f'"{k}": <true-or-false>')
        elif t == "int":
            parts.append(f'"{k}": <int>')
        elif t == "float":
            parts.append(f'"{k}": <number>')
        else:
            parts.append(f'"{k}": <"{t}">')
    return "{" + ", ".join(parts) + "}"


def indent(text, spaces=8):
    pad = " " * spaces
    return "\n".join(pad + l if l.strip() else "" for l in text.split("\n"))


def output_contract(n_ents):
    return (
        f"OUTPUT CONTRACT — your reply MUST end with exactly {n_ents} lines,\n"
        "one per entity above, in this exact format:\n"
        "ENTITY <id> | <answer>\n"
        "Example line (format only): ENTITY case_X | 12\n"
        "No other text after these lines. No JSON. No explanations."
    )


def write_fix3_scenarios(case):
    cid = case["case_id"]
    good = g7.correct_table_text(case)
    bad = g7.wrong_table_text(case)
    for name in SCENARIOS:
        text = bad if name == "lose" else good
        (SPOOF_DIR / f"{cid}-fix_3-{name}.txt").write_text(text)


def gen_v8(case, spoof, scenario="wind0"):
    cid = case["case_id"]
    case_path = f"{CASES_DIR}/{cid}.yml"
    out_dir = SPOOF_OUT if spoof else LIVE_OUT
    heavy = case["hops"] >= 5
    ents = g7.entity_block(case)
    n_ents = len(case["v6"]["entities"])
    wref = g7.worker_ref(case)
    esc_ref = g7.escalation_ref(case, "std")
    digest = indent(digest_block(case))
    form = json_form(case)
    contract = indent(output_contract(n_ents))
    method = indent(METHOD_EXACT)

    stage_files = {
        "solve": f"{cid}-solve-{scenario}.txt" if spoof else None,
        "fix_1": f"{cid}-fix_1-{scenario}.txt" if spoof else None,
        "fix_2": f"{cid}-fix_2-{scenario}.txt" if spoof else None,
        "fix_3": f"{cid}-fix_3-{scenario}.txt" if spoof else None,
    }

    fix2_next_fail = "s03b_fix_3" if heavy else "s06_end_fail"

    if spoof:
        solve_hooks = f"""      when:
        before_step_starts:
{g7.spoof_gate_chain(stage_files["solve"], case_path, "check", cid, out_dir, "solve", "s04_aggregate", "s02_fix_1")}
"""
        fix1_hooks = f"""      when:
        before_step_starts:
{g7.spoof_gate_chain(stage_files["fix_1"], case_path, "fix_1", cid, out_dir, "fix_1", "s04_aggregate", "s03_fix_2")}
"""
        fix2_hooks = f"""      when:
        before_step_starts:
{g7.spoof_gate_chain(stage_files["fix_2"], case_path, "fix_2", cid, out_dir, "fix_2", "s04_aggregate", fix2_next_fail)}
"""
        fix3_hooks = f"""      when:
        before_step_starts:
{g7.spoof_gate_chain(stage_files["fix_3"], case_path, "fix_3", cid, out_dir, "fix_3", "s04_aggregate", "s06_end_fail")}
"""
        judge_hooks = f"""      when:
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
"""
    else:
        solve_hooks = f"""      when:
{g7.live_success_hooks(case_path, cid, out_dir, "solve", "s04_aggregate", "s02_fix_1", "check")}
"""
        fix1_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
{g7.live_success_hooks(case_path, cid, out_dir, "fix_1", "s04_aggregate", "s03_fix_2", "fix_1")}
"""
        fix2_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
{g7.live_success_hooks(case_path, cid, out_dir, "fix_2", "s04_aggregate", fix2_next_fail, "fix_2")}
"""
        fix3_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
{g7.live_success_hooks(case_path, cid, out_dir, "fix_3", "s04_aggregate", "s06_end_fail", "fix_3")}
"""
        judge_hooks = f"""      when:
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
"""

    aggregate_hooks = f"""      when:
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
"""

    end_fail_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/failflag.py --result {out_dir}/ha-table-result.json --final-check {out_dir}/ha-final-check.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s07_fail_trace
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s08_audit
          - skip_step: true
"""

    fail_trace_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl end_fail fail case={cid}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '1 == 1'
                then: s08_audit
"""

    audit_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/leak-audit.py --run-dir {out_dir} --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{{{bookmarks.shell_output.exit_code}}}} == 0'
                then: s09_audit_pass
              - given: '{{{{bookmarks.shell_output.exit_code}}}} != 0'
                then: s10_audit_leak
          - skip_step: true
"""

    audit_pass_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl audit pass case={cid}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '1 == 1'
                then: s_end
"""

    audit_leak_hooks = f"""      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl audit fail leak=true case={cid}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '1 == 1'
                then: s_end
"""

    end_step_hooks = """      when:
        before_step_starts:
          - skip_step: true
"""

    fix3_step = f"""    s03b_fix_3:
      generative_entity: "{esc_ref}"
      prompt: |
        ROLE: last-resort rescuer. Three attempts wrong. Full rebuild.
        Independent recomputation, distrust every prior number, recompute every value from the rules alone.

        TASK CORE:
{digest}

        FINAL REPORT FORM (derive values yourself):
        {form}

        Failed cells (id + observed, withheld correct):
        {{{{bookmarks.cell_feedback}}}}

{method}

        Entities (one line each, id then question):
{ents}

{contract}
      model_overrides: {{ temperature: 0.4, max_tokens: 3072 }}
{fix3_hooks}
""" if heavy else ""

    return f"""workflow_id: harness_v13_{cid}{'_spoof' if spoof else ''}
name: "Harness v13 {'SPOOF ' if spoof else ''}- {cid}"
description: "DIGEST->GATE->FIX(rescue)->AGGREGATE->JUDGE->AUDIT. Case: {cid} hops={case['hops']} route={'light' if not heavy else 'heavy'}"
version: "9.0.0"
author: "Experiment"
tags: [harness, agentic, v13, digest, rescue{', spoof' if spoof else ''}]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  m_light:
    name: "{g7.MODEL_LIGHT}"
    host:
      type: llama_cpp_with_vulkan
  m_heavy:
    name: "{g7.MODEL_HEAVY}"
    host:
      type: llama_cpp_with_vulkan
  m_judge:
    name: "{g7.MODEL_JUDGE}"
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
      generative_entity: "{wref}"
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
      generative_entity: "{wref}"
      prompt: |
        ROLE: on-call operator. Precise. Zero fluff.

        TASK CORE:
{digest}

{method}

        Entities (one line each, id then question):
{ents}

{contract}
      model_overrides: {{ temperature: 0.1, max_tokens: 2048 }}
{solve_hooks}
    s02_fix_1:
      generative_entity: "{wref}"
      prompt: |
        ROLE: correction specialist. Entity table failed checks. Fix rows.
        Verify each entity against its rule sentence, one at a time, re-check the arithmetic not the assumption.

        TASK CORE:
{digest}

        FINAL REPORT FORM (derive values yourself):
        {form}

        Failed cells (id + your observed value, correct values withheld):
        {{{{bookmarks.cell_feedback}}}}

        Prior table:
        {{{{bookmarks.table}}}}

{method}

        Entities (one line each, id then question):
{ents}

{contract}
      model_overrides: {{ temperature: 0.2, max_tokens: 2560 }}
{fix1_hooks}
    s03_fix_2:
      generative_entity: "{esc_ref}"
      prompt: |
        ROLE: escalation solver. Two attempts wrong. Rebuild table from scratch.
        Discard both prior attempts entirely, re-derive from the digest as if seeing it fresh.

        TASK CORE:
{digest}

        FINAL REPORT FORM (derive values yourself):
        {form}

        Failed cells (id + observed, withheld correct):
        {{{{bookmarks.cell_feedback}}}}

{method}

        Entities (one line each, id then question):
{ents}

{contract}
      model_overrides: {{ temperature: 0.3, max_tokens: 3072 }}
{fix2_hooks}
{fix3_step}    s04_aggregate:
      generative_entity: "{wref}"
      prompt: "deterministic-aggregate-noop"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{aggregate_hooks}
    s05_judge:
      generative_entity: "${{models.m_judge}}"
      prompt: |
        ROLE: blind auditor. See only task + result. No author knowledge.

        Task objective: {case.get('objective', '')}
        Result: {{{{bookmarks.final_json}}}}

        Answer EXACTLY 6 lines, nothing else:
        KEYS: yes or no
        VALUES: yes or no
        FORMAT: yes or no
        VERDICT: PASS or FAIL
        REASON: one sentence
        SCORE: 1-10

        VERDICT must be PASS only if all three of KEYS, VALUES, FORMAT are yes.
      model_overrides: {{ temperature: 0.0, max_tokens: 160 }}
{judge_hooks}
    s06_end_fail:
      generative_entity: "{wref}"
      prompt: "FAIL"
      model_overrides: {{ temperature: 0.0, max_tokens: 8 }}
{end_fail_hooks}
    s07_fail_trace:
      generative_entity: "{wref}"
      prompt: "trace-only"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{fail_trace_hooks}
    s08_audit:
      generative_entity: "{wref}"
      prompt: "leak-audit-noop"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{audit_hooks}
    s09_audit_pass:
      generative_entity: "{wref}"
      prompt: "audit-pass-trace"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{audit_pass_hooks}
    s10_audit_leak:
      generative_entity: "{wref}"
      prompt: "audit-leak-trace"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{audit_leak_hooks}
    s_end:
      generative_entity: "{wref}"
      prompt: "term"
      model_overrides: {{ temperature: 0.0, max_tokens: 1 }}
{end_step_hooks}"""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--spoof", action="store_true")
    ap.add_argument("--scenario", default="wind0", choices=SCENARIOS)
    args = ap.parse_args()

    if args.all:
        cases = sorted(p.stem for p in Path(CASES_DIR).glob("hc-*.yml"))
    else:
        cases = [args.case]
    WORKFLOW_DIR.mkdir(parents=True, exist_ok=True)

    for cid in cases:
        case = g7.load_case(cid)
        if args.spoof:
            g7.write_scenarios(case, SPOOF_DIR)
            write_fix3_scenarios(case)
        yml = gen_v8(case, args.spoof, args.scenario)
        suffix = "-spoof" if args.spoof else ""
        path = WORKFLOW_DIR / f"v13-{cid}{suffix}.yml"
        path.write_text(yml)
        print(f"generated {path}")


if __name__ == "__main__":
    main()
