#!/usr/bin/env python3
"""v7 workflow generator. v6 core + research improvements:

1. Hops-based model routing (P5): hops<=4 -> 4B worker, hops>=5 -> 9B
2. Cross-family judge (F4): Hermes-2-Pro-Mistral-7B judges Qwen workers
3. Per-stage artifact snapshots: artifact.{solve,fix_1,fix_2}.txt
4. Token metering per stage into trace (Devin ACU pattern)
5. Judge v2: 3 binary sub-checks before verdict (F3, variance -3-4x)
6. End-of-run leak audit step (F8/P17 as enforced invariant)
7. Thinking-model variant: --variant thinking (budget-forcing prompt shape,
   spoof-verified structure only — live proof deferred)

Adversarial spoof scenarios (gate hardening): adv_partial (missing row),
adv_malformed (garbage row), adv_wrongtype (bool/int confusion),
adv_dup (extra unknown row). All must route check=fail -> fix_1.

Usage: gen-v7.py --case ha-01 [--spoof] [--scenario wind0] [--variant std|thinking]
       gen-v7.py --all --spoof
"""
import argparse
import json
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent.parent
CASES_DIR = str(HERE / "cases")
WORKFLOW_DIR = HERE / "workflows"
SPOOF_DIR = HERE / "fixtures" / "spoof-v7"
LIVE_OUT = "./docs/benchmarks/outputs/output"
SPOOF_OUT = "./docs/benchmarks/outputs/output/spoof-v7"
SCRIPTS = "experiments/harness-inspired/scripts"
SPOOF_DIR_NAME = "experiments/harness-inspired/fixtures/spoof-v7"

MODEL_LIGHT = "Qwen3-4B-Instruct-2507-Q4_K_M"
MODEL_HEAVY = "Qwen3-5-9B-Q4_K_M"
MODEL_JUDGE = "Hermes-2-Pro-Mistral-7B.Q4_K_M"
MODEL_THINK = "Qwen3-4B-Thinking-2507-Q4_K_M"

SCENARIOS = ["wind0", "wind1", "wind2", "lose",
             "adv_partial", "adv_malformed", "adv_wrongtype", "adv_dup"]


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
        if isinstance(ans, bool):
            ans = not ans
        elif isinstance(ans, int):
            ans = ans + 7
        elif isinstance(ans, float):
            ans = ans + 7.5
        elif isinstance(ans, dict):
            ans = {k: (v + 7 if isinstance(v, int) and not isinstance(v, bool) else v) for k, v in ans.items()}
        elif isinstance(ans, str):
            ans = ans + "XX"
        lines.append(f"ENTITY {e['id']} | {json.dumps(ans)}")
    return "\n".join(lines) + "\n"


def corrupt_table(kind, case):
    ents = case["v6"]["entities"]
    rows = [f"ENTITY {e['id']} | {json.dumps(e['answer'])}" for e in ents]
    if kind == "adv_partial":
        return "\n".join(rows[:-1]) + "\n"
    if kind == "adv_malformed":
        broken = list(rows)
        broken[0] = f"ENTITY {ents[0]['id']} | not-json-{{garbage"
        broken.insert(1, "XX this line is junk |||")
        return "\n".join(broken) + "\n"
    if kind == "adv_wrongtype":
        broken = list(rows)
        for i in range(len(ents) - 1, -1, -1):
            ans = ents[i]["answer"]
            if isinstance(ans, bool):
                broken[i] = f"ENTITY {ents[i]['id']} | {json.dumps(int(ans))}"
            elif isinstance(ans, (int, float)):
                broken[i] = f"ENTITY {ents[i]['id']} | {json.dumps(str(ans))}"
            elif isinstance(ans, str):
                broken[i] = f"ENTITY {ents[i]['id']} | {json.dumps([ans])}"
            elif isinstance(ans, dict):
                broken[i] = f"ENTITY {ents[i]['id']} | {json.dumps(json.dumps(ans, sort_keys=True))}"
            else:
                continue
            break
        return "\n".join(broken) + "\n"
    if kind == "adv_dup":
        return "\n".join(rows + ["ENTITY ghost_row | 42"]) + "\n"
    raise ValueError(kind)


def value_format(answer):
    if isinstance(answer, bool):
        return "answer format: true or false"
    if isinstance(answer, int):
        return "answer format: a whole number"
    if isinstance(answer, float):
        return "answer format: a number (decimals allowed)"
    if isinstance(answer, dict):
        keys = ", ".join(f'"{k}": N' for k in answer)
        return f"answer format: JSON object {{{keys}}}"
    return "answer format: a single word in quotes"


def entity_block(case):
    return "\n".join(
        f"        - {e['id']}: {e['question']} [{value_format(e['answer'])}]"
        for e in case["v6"]["entities"]
    )


def write_scenarios(case, out_dir):
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
    for kind in ("adv_partial", "adv_malformed", "adv_wrongtype", "adv_dup"):
        scen[kind] = {"solve": corrupt_table(kind, case), "fix_1": good, "fix_2": good}
    for name, stages in scen.items():
        for stage, text in stages.items():
            (S / f"{cid}-{stage}-{name}.txt").write_text(text)
    (S / f"{cid}-judge.txt").write_text(
        "KEYS: yes\nVALUES: yes\nFORMAT: yes\nVERDICT: PASS\nREASON: spoof\nSCORE: 9\n")


def worker_ref(case):
    return "${models.m_light}" if case["hops"] <= 4 else "${models.m_heavy}"


def escalation_ref(case, variant="std"):
    if variant == "thinking":
        return "${models.m_heavy}"
    return "${models.m_heavy}"


def spoof_gate_chain(canned_file, case_path, gate_label, case_id, out_dir, stage,
                     next_pass, next_fail):
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
          - shell:
              command: "cp {out_dir}/ha-table.txt {out_dir}/artifact.{stage}.txt"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "python3 {SCRIPTS}/meter.py --artifact {out_dir}/ha-table.txt --stage {stage} --trace {out_dir}/ha-trace.jsonl --case {case_id}"
              working_dir: "."
              fail_on_error: false
          - skip_step: true'''


def live_success_hooks(case_path, cid, out_dir, stage, next_pass, next_fail, gate_label):
    body = f'''        after_step_succeeds:
          - save_to:
              - "$table"
              - "{out_dir}/ha-table.txt"
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
              command: "python3 {SCRIPTS}/trace-check.py {out_dir}/ha-table-result.json {gate_label} {out_dir}/ha-trace.jsonl {cid}"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "cp {out_dir}/ha-table.txt {out_dir}/artifact.{stage}.txt"
              working_dir: "."
              fail_on_error: false
          - shell:
              command: "python3 {SCRIPTS}/meter.py --artifact {out_dir}/ha-table.txt --stage {stage} --trace {out_dir}/ha-trace.jsonl --case {cid}"
              working_dir: "."
              fail_on_error: false'''
    return body


def gen_v7(case, spoof, scenario="wind0", variant="std"):
    cid = case["case_id"]
    case_path = f"{CASES_DIR}/{cid}.yml"
    out_dir = SPOOF_OUT if spoof else LIVE_OUT
    ents = entity_block(case)
    n_ents = len(case["v6"]["entities"])
    wref = worker_ref(case)
    esc_ref = escalation_ref(case, variant)
    light_name = MODEL_THINK if variant == "thinking" else MODEL_LIGHT
    heavy_name = MODEL_THINK if variant == "thinking" else MODEL_HEAVY
    stage_files = {
        "solve": f"{cid}-solve-{scenario}.txt" if spoof else None,
        "fix_1": f"{cid}-fix_1-{scenario}.txt" if spoof else None,
        "fix_2": f"{cid}-fix_2-{scenario}.txt" if spoof else None,
    }

    think_block = """        THINK FIRST inside <think></think>. Reason step by step per entity.
        BUDGET: when near completion, verify each row once more before
        ending your think block. Then output the table.
""" if variant == "thinking" else ""
    solve_tokens = 2048 if variant == "thinking" else 1280
    fix_tokens = 2560 if variant == "thinking" else 1600

    if spoof:
        solve_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["solve"], case_path, "check", cid, out_dir, "solve", "s04_aggregate", "s02_fix_1")}
'''
        fix1_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["fix_1"], case_path, "fix_1", cid, out_dir, "fix_1", "s04_aggregate", "s03_fix_2")}
'''
        fix2_hooks = f'''      when:
        before_step_starts:
{spoof_gate_chain(stage_files["fix_2"], case_path, "fix_2", cid, out_dir, "fix_2", "s04_aggregate", "s06_end_fail")}
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
{live_success_hooks(case_path, cid, out_dir, "solve", "s04_aggregate", "s02_fix_1", "check")}
'''
        fix1_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
{live_success_hooks(case_path, cid, out_dir, "fix_1", "s04_aggregate", "s03_fix_2", "fix_1")}
'''
        fix2_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/table-feedback.py --result {out_dir}/ha-table-result.json --case {case_path}"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$cell_feedback"
{live_success_hooks(case_path, cid, out_dir, "fix_2", "s04_aggregate", "s06_end_fail", "fix_2")}
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
                then: s08_audit
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
                then: s08_audit
'''

    audit_hooks = f'''      when:
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
'''

    audit_pass_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl audit pass case={cid}"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '1 == 1'
                then: s_end
'''

    audit_leak_hooks = f'''      when:
        before_step_starts:
          - shell:
              command: "python3 {SCRIPTS}/trace-append.py {out_dir}/ha-trace.jsonl audit fail leak=true case={cid}"
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

    import re
    prompt = re.sub(
        r"\s*Output\s+ONLY\s+this\s+JSON\s+with\s+exactly\s+these\s+keys:.*?\} *\.",
        "",
        case["prompt"].strip(),
        flags=re.DOTALL,
    ).strip()
    prompt_ind = "\n".join("        " + l for l in prompt.split("\n"))
    variant_tag = "-thinking" if variant == "thinking" else ""

    return f'''workflow_id: harness_v7_{cid}{variant_tag}{'_spoof' if spoof else ''}
name: "Harness v7 {'SPOOF ' if spoof else ''}- {cid}{variant_tag}"
description: "TABLE->GATE->AGGREGATE->JUDGE-V2->LEAK-AUDIT. Case: {cid} hops={case['hops']} route={'light' if case['hops'] <= 4 else 'heavy'}"
version: "7.0.0"
author: "Experiment"
tags: [harness, agentic, v7, routing, snapshot, meter, audit{' ,spoof' if spoof else ''}{', thinking' if variant == 'thinking' else ''}]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  m_light:
    name: "{light_name}"
    host:
      type: llama_cpp_with_vulkan
  m_heavy:
    name: "{heavy_name}"
    host:
      type: llama_cpp_with_vulkan
  m_judge:
    name: "{MODEL_JUDGE}"
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

        {prompt_ind}

{think_block}        METHOD: for EACH entity below, derive its answer from the rules.
        Track state step by step in plain text. Entities you do not manage
        NEVER count. Check deadline/budget rules BEFORE finalizing.

        Entities (one line each, id then question):
{ents}

        OUTPUT CONTRACT — your reply MUST end with exactly {n_ents} lines,
        one per entity above, in this exact format:
        ENTITY <id> | <answer>
        Example line (format only): ENTITY case_X | 12
        No other text after these lines. No JSON. No explanations.
      model_overrides: {{ temperature: 0.1, max_tokens: {solve_tokens} }}
{solve_hooks}
    s02_fix_1:
      generative_entity: "{wref}"
      prompt: |
        ROLE: correction specialist. Entity table failed checks. Fix rows.

        Original task:
        {prompt_ind}

        Failed cells (id + your observed value, correct values withheld):
        {{{{bookmarks.cell_feedback}}}}

        Prior table:
        {{{{bookmarks.table}}}}

        METHOD: re-read the task rules above, then re-derive each failed
        entity. Use these exact entity ids:

{ents}

        OUTPUT CONTRACT — your reply MUST end with exactly {n_ents} lines,
        one per entity, in this exact format:
        ENTITY <id> | <answer>
        Example line (format only): ENTITY case_X | 12
        No other text after these lines.
      model_overrides: {{ temperature: 0.0, max_tokens: {fix_tokens} }}
{fix1_hooks}
    s03_fix_2:
      generative_entity: "{esc_ref}"
      prompt: |
        ROLE: escalation solver. Two attempts wrong. Rebuild table from scratch.

        Original task:
        {prompt_ind}

        Failed cells (id + observed, withheld correct):
        {{{{bookmarks.cell_feedback}}}}

        METHOD: discard prior attempts. Re-derive EVERY entity answer from
        the rules. Use these exact entity ids:

{ents}

        OUTPUT CONTRACT — your reply MUST end with exactly {n_ents} lines,
        one per entity, in this exact format:
        ENTITY <id> | <answer>
        Example line (format only): ENTITY case_X | 12
        No other text after these lines. No prose summaries.
      model_overrides: {{ temperature: 0.0, max_tokens: {fix_tokens} }}
{fix2_hooks}
    s04_aggregate:
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
{end_step_hooks}'''


def load_case(cid):
    return yaml.safe_load(Path(CASES_DIR, f"{cid}.yml").read_text())


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--spoof", action="store_true")
    ap.add_argument("--scenario", default="wind0", choices=SCENARIOS)
    ap.add_argument("--variant", default="std", choices=["std", "thinking"])
    args = ap.parse_args()

    if args.all:
        cases = sorted(p.stem for p in Path(CASES_DIR).glob("h[ab]-*.yml"))
    else:
        cases = [args.case]
    WORKFLOW_DIR.mkdir(parents=True, exist_ok=True)

    for cid in cases:
        case = load_case(cid)
        if args.spoof:
            write_scenarios(case, SPOOF_DIR)
        yml = gen_v7(case, args.spoof, args.scenario, args.variant)
        suffix = "-spoof" if args.spoof else ""
        variant_suffix = "-thinking" if args.variant == "thinking" else ""
        path = WORKFLOW_DIR / f"v7-{cid}{variant_suffix}{suffix}.yml"
        path.write_text(yml)
        print(f"generated {path}")


if __name__ == "__main__":
    main()
