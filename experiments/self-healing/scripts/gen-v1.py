#!/usr/bin/env python3
"""gen-v1.py — generate a self-healing v1 zero-LLM workflow YAML per case.

Pattern source: experiments/harness-inspired/workflows/v9-hc-05-spoof.yml (proven).
Every step exits via before_step_starts hooks (guaranteed GWT route or skip_step)
BEFORE any inference — zero model loads/calls (Phase S hard constraint).

Decision logic mapping (classify.py exit -> GWT route):
  0 clean -> accept | 1 F1 -> heal_f1 | 2 F2 -> heal_f2 | 3/other F3/F4 -> heal_f34

Usage: gen-v1.py --case <case.yml> [--out-dir workflows/generated]
"""
import argparse
import os
import sys

import yaml

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))
EXP = os.path.join(REPO, "experiments", "self-healing")

MODELS = {
    "m_worker": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "m_ministral": "Ministral-3-3B-Instruct-2512-Q4_K_M",
    "m_coder": "Qwen2.5-Coder-3B-Instruct-Q8_0",
    "m_heavy": "Qwen3-5-9B-Q4_K_M",
    "m_judge": "Hermes-2-Pro-Mistral-7B.Q4_K_M",
}

WORKER_PROMPT = (
    "ROLE: on-call operator. Precise. Zero fluff.\n"
    "TASK: summarize the cache sweep results as a JSON report per REPORT SPEC.\n"
    "OUTPUT: single JSON object, nothing else."
)
HEAL_PROMPTS = {
    "corrective_prompt": (
        "ROLE: correction specialist (F1 hallucination).\n"
        "Restate REPORT SPEC keys, drop underivable fields, re-derive each value\n"
        "from rules alone. No verification claims you cannot derive."
    ),
    "tool_reselect": (
        "ROLE: tool-repair specialist (F2 execution error).\n"
        "Switch failed call to fallback endpoint, re-issue ONLY the failed call,\n"
        "keep prior successful results."
    ),
    "replan": (
        "ROLE: replanner (F3/F4).\n"
        "Rebuild plan: exclude failed subtask, decompose, priority re-execute\n"
        "from first failed subtask."
    ),
}


def sh(cmd):
    return {"shell": {"command": cmd, "working_dir": ".", "fail_on_error": False}}


def gwt(clauses):
    return {"gwt": [{"given": g, "then": t} for g, t in clauses]}


def skip():
    return {"skip_step": True}


def classify_routes(round_no, heal_f1, heal_f2, heal_f34, accept, final_fail=None):
    routes = [
        ("{{bookmarks.shell_output.exit_code}} == 0", accept),
        ("{{bookmarks.shell_output.exit_code}} == 1", heal_f1),
        ("{{bookmarks.shell_output.exit_code}} == 2", heal_f2),
        ("{{bookmarks.shell_output.exit_code}} == 3", heal_f34),
    ]
    if final_fail:
        routes.append(("{{bookmarks.shell_output.exit_code}} != 0", final_fail))
    else:
        routes.append(("{{bookmarks.shell_output.exit_code}} != 0", heal_f34))
    return routes


def build(case_path):
    with open(case_path) as f:
        case = yaml.safe_load(f)
    case_id = case["case_id"]
    run_dir = os.path.join(EXP, "results", "sh-v1", case_id)
    case_abs = os.path.abspath(case_path)
    S = "experiments/self-healing/scripts"

    steps = {}

    steps["step_00_init"] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "init-noop",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("mkdir -p %s" % run_dir),
            sh("python3 %s/end_event.py --run-dir %s --status init" % (S, run_dir)),
            gwt([("1 == 1", "step_01_attempt_1")]),
            skip(),
        ]},
    }

    heal_targets = {
        1: ("step_04_heal_f1_1", "step_05_heal_f2_1", "step_06_heal_f34_1"),
        2: ("step_10_heal_f1_2", "step_11_heal_f2_2", "step_12_heal_f34_2"),
    }
    attempt_after_heal = {1: "step_07_attempt_2", 2: "step_13_attempt_3"}

    # Engine iterates steps sorted by name; numbering must be strictly
    # increasing in chain order or rounds interleave and route targets break.
    chain_nums = {1: (1, 2, 3), 2: (7, 8, 9), 3: (13, 14, 15)}
    for n in (1, 2, 3):
        ano, dno, cno = chain_nums[n]
        steps["step_%02d_attempt_%d" % (ano, n)] = {
            "generative_entity": "${models.m_worker}",
            "prompt": WORKER_PROMPT,
            "model_overrides": {"temperature": 0.1, "max_tokens": 1024},
            "when": {"before_step_starts": [
                sh("python3 %s/agent_run.py --case %s --run-dir %s --attempt %d" % (S, case_abs, run_dir, n)),
                gwt([("1 == 1", "step_%02d_detect_%d" % (dno, n))]),
                skip(),
            ]},
        }
        steps["step_%02d_detect_%d" % (dno, n)] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "detect-noop (deterministic detector suite)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/detect.py --case %s --run-dir %s --attempt %d" % (S, case_abs, run_dir, n)),
                gwt([("1 == 1", "step_%02d_classify_%d" % (cno, n))]),
                skip(),
            ]},
        }
        if n < 3:
            routes = classify_routes(n, *heal_targets[n], "step_16_accept")
        else:
            routes = [
                ("{{bookmarks.shell_output.exit_code}} == 0", "step_16_accept"),
                ("{{bookmarks.shell_output.exit_code}} != 0", "step_17_final_fail"),
            ]
        steps["step_%02d_classify_%d" % (cno, n)] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "classify-noop (R = 0.35C + 0.35S + 0.30E vs theta=0.65)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/classify.py --case %s --run-dir %s --attempt %d" % (S, case_abs, run_dir, n)),
                gwt(routes),
                skip(),
            ]},
        }

    heal_defs = [
        ("step_04_heal_f1_1", "m_ministral", "corrective_prompt", 1, "step_07_attempt_2"),
        ("step_05_heal_f2_1", "m_coder", "tool_reselect", 1, "step_07_attempt_2"),
        ("step_06_heal_f34_1", "m_heavy", "replan", 1, "step_07_attempt_2"),
        ("step_10_heal_f1_2", "m_ministral", "corrective_prompt", 2, "step_13_attempt_3"),
        ("step_11_heal_f2_2", "m_coder", "tool_reselect", 2, "step_13_attempt_3"),
        ("step_12_heal_f34_2", "m_heavy", "replan", 2, "step_13_attempt_3"),
    ]
    for sid, mkey, strat, rnd, nxt in heal_defs:
        steps[sid] = {
            "generative_entity": "${models.%s}" % mkey,
            "prompt": HEAL_PROMPTS[strat],
            "model_overrides": {"temperature": 0.2, "max_tokens": 1536},
            "when": {"before_step_starts": [
                sh("python3 %s/heal.py --case %s --run-dir %s --round %d --strategy %s"
                   % (S, case_abs, run_dir, rnd, strat)),
                gwt([("1 == 1", nxt)]),
                skip(),
            ]},
        }

    steps["step_16_accept"] = {
        "generative_entity": "${models.m_judge}",
        "prompt": (
            "ROLE: blind auditor. Task + result only.\n"
            "Answer: KEYS/VALUES/FORMAT yes-no, VERDICT PASS|FAIL, SCORE 1-10."
        ),
        "model_overrides": {"temperature": 0.0, "max_tokens": 160},
        "when": {"before_step_starts": [
            sh("python3 %s/outcome.py --case %s --run-dir %s --outcome accept" % (S, case_abs, run_dir)),
            gwt([("1 == 1", "step_18_report")]),
            skip(),
        ]},
    }
    steps["step_17_final_fail"] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "FAIL: attempts exhausted below theta.",
        "model_overrides": {"temperature": 0.0, "max_tokens": 8},
        "when": {"before_step_starts": [
            sh("python3 %s/outcome.py --case %s --run-dir %s --outcome final_fail" % (S, case_abs, run_dir)),
            gwt([("1 == 1", "step_18_report")]),
            skip(),
        ]},
    }
    steps["step_18_report"] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "report-noop (TSR/FDA/RSR oracle)",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/report.py --case %s --run-dir %s" % (S, case_abs, run_dir)),
            gwt([
                ("{{bookmarks.shell_output.exit_code}} == 0", "step_19_end_pass"),
                ("{{bookmarks.shell_output.exit_code}} != 0", "step_20_end_fail"),
            ]),
            skip(),
        ]},
    }
    steps["step_19_end_pass"] = {
        "generative_entity": "${models.m_worker}",
        "prompt": "term",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/end_event.py --run-dir %s --status pass" % (S, run_dir)),
            gwt([("1 == 1", "step_21_end")]),
            skip(),
        ]},
    }
    steps["step_20_end_fail"] = {
        "generative_entity": "${models.m_worker}",
        "prompt": "term",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/end_event.py --run-dir %s --status fail" % (S, run_dir)),
            gwt([("1 == 1", "step_21_end")]),
            skip(),
        ]},
    }
    steps["step_21_end"] = {
        "generative_entity": "${models.m_worker}",
        "prompt": "term",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [skip()]},
    }

    targets = [
        tgt
        for st in steps.values()
        for h in st.get("when", {}).get("before_step_starts", [])
        if "gwt" in h
        for tgt in [c["then"] for c in h["gwt"]]
    ]
    missing = sorted(t for t in set(targets) if t not in steps)
    assert not missing, "gen-v1 route targets missing from steps: %s" % missing

    wf = {
        "workflow_id": "self_healing_v1_%s" % case_id.replace("-", "_"),
        "name": "Self-Healing v1 SPOOF — %s" % case_id,
        "description": ("attempt->detect->classify->heal(routed)->retry chain, "
                        "ZERO-LLM spoof. Case: %s" % case_id),
        "version": "1.0.0",
        "author": "self-healing experiment",
        "tags": ["self-healing", "spoof", "zero-llm", case_id],
        "schema_version": "2.0.0",
        "min_schema_version": "2.0.0",
        "providers": {"llama_cpp_with_vulkan": {"config": {"host": "localhost", "port": 8080}}},
        "models": {k: {"name": v, "host": {"type": "llama_cpp_with_vulkan"}}
                   for k, v in MODELS.items()},
        "workflow_execution_strategy": {
            "timing": {"cooldown_after_unload_secs": 0, "min_tmp_space_mb": 50},
            "memory": {"model_lifecycle": {"unload_unused": False}},
        },
        "agentic_workflow": {
            "when": {"after_workflow": [{
                "log": {"event_fields": ["workflow_id", "total_steps", "succeeded", "failed"],
                        "level": "info"}
            }]},
            "steps": steps,
        },
    }
    return wf, case_id


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--out-dir", default=os.path.join(EXP, "workflows", "generated"))
    args = ap.parse_args()

    wf, case_id = build(args.case)
    os.makedirs(args.out_dir, exist_ok=True)
    out = os.path.join(args.out_dir, "sh-v1-%s.yml" % case_id)
    with open(out, "w") as f:
        yaml.safe_dump(wf, f, sort_keys=False, default_flow_style=False, width=100)
    print(out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
