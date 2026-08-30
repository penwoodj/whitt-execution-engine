#!/usr/bin/env python3
"""gen-v2.py — generate a self-healing v2 zero-LLM workflow YAML per case.

Step graph (docs/v2/02-WORKFLOW-V2-DESIGN.md, 46 steps; every step exits
through before_step_starts hooks BEFORE any inference — zero-LLM spoof):

  00 init
  ROUND r in 1..3:
    route_r            (route_v2: 0 fast | 1 coder | 2 general | 3 heavy)
    att_{fast,coder,general,heavy}_r   (agent_run_v2, model-specialized)
    detect_r triage_r  (0 clean | 1 light | 2 replan | 3 fail)
    judge_gate_r judge_r   (gray zone only)
    heal_light_r       (F1/F2 script heal)
    budget_r hreplan_r (F3/F4 heavy replan, budget-gated)
  40 accept | 41 final_fail -> 42 report -> 43/44 end_pass/fail -> 45 end

Numbering strictly increasing per chain order (v1 iteration-4 lesson);
generator asserts every GWT target exists.
"""
from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

import yaml

REPO = Path(__file__).resolve().parents[3]
EXP = REPO / "experiments" / "self-healing"

MODELS = {
    "m_worker": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "m_fast": "Ministral-3-3B-Instruct-2512-Q4_K_M",
    "m_coder": "Qwen2.5-Coder-3B-Instruct-Q8_0",
    "m_heavy": "Qwen3-5-9B-Q4_K_M",
    "m_judge": "Hermes-2-Pro-Mistral-7B.Q4_K_M",
}

WORKER_PROMPT = (
    "ROLE: domain specialist. Deliver the artifact per the case contract.\n"
    "OUTPUT: single JSON object per contract, nothing else."
)
JUDGE_PROMPT = (
    "ROLE: gray-zone auditor. Candidate artifact + contract only.\n"
    "ANSWER: ACCEPT or REJECT plus one-line basis."
)
REPLAN_PROMPT = (
    "ROLE: heavy replanner (F3/F4). Recompute plan from the artifact's own "
    "tables; reconcile cross-section numbers; restore dropped fields; "
    "re-emit complete artifact within contract."
)
LIGHT_PROMPT = (
    "ROLE: light corrector (F1/F2). Apply corrective instruction to the "
    "prior attempt; re-emit the full contract object."
)

ROUND_BASE = {1: 1, 2: 13, 3: 25}
OFF = {
    "route": 0,
    "att_fast": 1, "att_coder": 2, "att_general": 3, "att_heavy": 4,
    "detect": 5, "triage": 6, "jgate": 7, "judge": 8,
    "hlight": 9, "budget": 10, "hreplan": 11,
}
ATT_VARIANTS = ("att_fast", "att_coder", "att_general", "att_heavy")
ROLE_OF_VARIANT = {
    "att_fast": "worker_fast", "att_coder": "worker_coder",
    "att_general": "worker_general", "att_heavy": "heavy_replanner",
}
MODEL_OF_VARIANT = {
    "att_fast": "m_fast", "att_coder": "m_coder",
    "att_general": "m_worker", "att_heavy": "m_heavy",
}
ACCEPT, FINAL_FAIL = "step_40_accept", "step_41_final_fail"


def sid(round_no: int, kind: str) -> str:
    return "step_%02d_%s_%d" % (ROUND_BASE[round_no] + OFF[kind], kind, round_no)


def sh(cmd: str) -> dict:
    return {"shell": {"command": cmd, "working_dir": ".", "fail_on_error": False}}


def gwt(clauses: list[tuple[str, str]]) -> dict:
    return {"gwt": [{"given": g, "then": t} for g, t in clauses]}


def skip() -> dict:
    return {"skip_step": True}


def build(case_path: str) -> tuple[dict, str]:
    with open(case_path) as f:
        case = yaml.safe_load(f)
    case_id = case["case_id"]
    run_dir = str(EXP / "results" / "sh-v2" / case_id)
    case_abs = os.path.abspath(case_path)
    S = "experiments/self-healing/scripts"
    EC = "{{bookmarks.shell_output.exit_code}}"

    steps: dict[str, dict] = {}

    steps["step_00_init"] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "init-noop",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("mkdir -p %s" % run_dir),
            sh("python3 %s/end_event_v2.py --run-dir %s --status init" % (S, run_dir)),
            gwt([("1 == 1", sid(1, "route"))]),
            skip(),
        ]},
    }

    for r in (1, 2, 3):
        last = r == 3

        steps[sid(r, "route")] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "route-noop (worker variant selection)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/route_v2.py --case %s --run-dir-base %s --round %d"
                   % (S, case_abs, EXP / "results" / "sh-v2", r)),
                gwt([(f"{EC} == 0", sid(r, "att_fast")),
                     (f"{EC} == 1", sid(r, "att_coder")),
                     (f"{EC} == 2", sid(r, "att_general")),
                     (f"{EC} != 2", sid(r, "att_heavy"))]),
                skip(),
            ]},
        }

        for v in ATT_VARIANTS:
            steps[sid(r, v)] = {
                "generative_entity": "${models.%s}" % MODEL_OF_VARIANT[v],
                "prompt": WORKER_PROMPT,
                "model_overrides": {"temperature": 0.1, "max_tokens": 2048},
                "when": {"before_step_starts": [
                    sh("python3 %s/agent_run_v2.py --case %s --run-dir-base %s "
                       "--attempt %d --model-role %s"
                       % (S, case_abs, EXP / "results" / "sh-v2", r, ROLE_OF_VARIANT[v])),
                    gwt([("1 == 1", sid(r, "detect"))]),
                    skip(),
                ]},
            }

        steps[sid(r, "detect")] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "detect-noop (deterministic suite)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/detect_v2.py --case %s --run-dir-base %s --attempt %d"
                   % (S, case_abs, EXP / "results" / "sh-v2", r)),
                gwt([("1 == 1", sid(r, "triage"))]),
                skip(),
            ]},
        }

        light_t = FINAL_FAIL if last else sid(r, "hlight")
        replan_t = FINAL_FAIL if last else sid(r, "budget")
        steps[sid(r, "triage")] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "triage-noop (R = 0.35C + 0.35S + 0.30E vs theta=0.65)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/triage_v2.py --case %s --run-dir-base %s --attempt %d"
                   % (S, case_abs, EXP / "results" / "sh-v2", r)),
                gwt([(f"{EC} == 0", sid(r, "jgate")),
                     (f"{EC} == 1", light_t),
                     (f"{EC} == 2", replan_t),
                     (f"{EC} != 0", FINAL_FAIL)]),
                skip(),
            ]},
        }

        steps[sid(r, "jgate")] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "judge-gate-noop (gray zone check)",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/judge_gate_v2.py --case %s --run-dir-base %s --attempt %d"
                   % (S, case_abs, EXP / "results" / "sh-v2", r)),
                gwt([(f"{EC} == 0", ACCEPT), (f"{EC} != 0", sid(r, "judge"))]),
                skip(),
            ]},
        }

        judge_reject_t = FINAL_FAIL if last else sid(r, "budget")
        steps[sid(r, "judge")] = {
            "generative_entity": "${models.m_judge}",
            "prompt": JUDGE_PROMPT,
            "model_overrides": {"temperature": 0.0, "max_tokens": 160},
            "when": {"before_step_starts": [
                sh("python3 %s/judge_v2.py --case %s --run-dir-base %s --attempt %d"
                   % (S, case_abs, EXP / "results" / "sh-v2", r)),
                gwt([(f"{EC} == 0", ACCEPT), (f"{EC} != 0", judge_reject_t)]),
                skip(),
            ]},
        }

        if not last:
            nxt_route = sid(r + 1, "route")
            steps[sid(r, "hlight")] = {
                "generative_entity": "${models.m_fast}",
                "prompt": LIGHT_PROMPT,
                "model_overrides": {"temperature": 0.2, "max_tokens": 1024},
                "when": {"before_step_starts": [
                    sh("python3 %s/heal_light_v2.py --case %s --run-dir-base %s --round %d"
                       % (S, case_abs, EXP / "results" / "sh-v2", r)),
                    gwt([("1 == 1", nxt_route)]),
                    skip(),
                ]},
            }
            steps[sid(r, "budget")] = {
                "generative_entity": "${models.m_worker}",
                "prompt": "budget-noop (call ceiling gate)",
                "model_overrides": {"temperature": 0.0, "max_tokens": 1},
                "when": {"before_step_starts": [
                    sh("python3 %s/budget_v2.py --run-dir %s --reserve 1 --site replan_r%d"
                       % (S, run_dir, r)),
                    gwt([(f"{EC} == 0", sid(r, "hreplan")), (f"{EC} != 0", FINAL_FAIL)]),
                    skip(),
                ]},
            }
            steps[sid(r, "hreplan")] = {
                "generative_entity": "${models.m_heavy}",
                "prompt": REPLAN_PROMPT,
                "model_overrides": {"temperature": 0.2, "max_tokens": 2048},
                "when": {"before_step_starts": [
                    sh("python3 %s/heal_replan_v2.py --case %s --run-dir-base %s --round %d"
                       % (S, case_abs, EXP / "results" / "sh-v2", r)),
                    gwt([(f"{EC} == 0", nxt_route), (f"{EC} != 0", FINAL_FAIL)]),
                    skip(),
                ]},
            }

    steps[ACCEPT] = {
        "generative_entity": "${models.m_judge}",
        "prompt": "accept-noop",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/outcome_v2.py --case %s --run-dir-base %s --outcome accept"
               % (S, case_abs, EXP / "results" / "sh-v2")),
            gwt([("1 == 1", "step_42_report")]),
            skip(),
        ]},
    }
    steps[FINAL_FAIL] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "fail-noop (attempts or budget exhausted)",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/outcome_v2.py --case %s --run-dir-base %s --outcome final_fail"
               % (S, case_abs, EXP / "results" / "sh-v2")),
            gwt([("1 == 1", "step_42_report")]),
            skip(),
        ]},
    }
    steps["step_42_report"] = {
        "generative_entity": "${models.m_heavy}",
        "prompt": "report-noop (O1-O4 oracle)",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [
            sh("python3 %s/report_v2.py --case %s --run-dir %s"
               % (S, case_abs, run_dir)),
            gwt([(f"{EC} == 0", "step_43_end_pass"), (f"{EC} != 0", "step_44_end_fail")]),
            skip(),
        ]},
    }
    for name, status in (("step_43_end_pass", "pass"), ("step_44_end_fail", "fail")):
        steps[name] = {
            "generative_entity": "${models.m_worker}",
            "prompt": "term",
            "model_overrides": {"temperature": 0.0, "max_tokens": 1},
            "when": {"before_step_starts": [
                sh("python3 %s/end_event_v2.py --run-dir %s --status %s" % (S, run_dir, status)),
                gwt([("1 == 1", "step_45_end")]),
                skip(),
            ]},
        }
    steps["step_45_end"] = {
        "generative_entity": "${models.m_worker}",
        "prompt": "term",
        "model_overrides": {"temperature": 0.0, "max_tokens": 1},
        "when": {"before_step_starts": [skip()]},
    }

    targets = [
        c["then"]
        for st in steps.values()
        for h in st.get("when", {}).get("before_step_starts", [])
        if "gwt" in h
        for c in h["gwt"]
    ]
    missing = sorted(t for t in set(targets) if t not in steps)
    assert not missing, "gen-v2 route targets missing from steps: %s" % missing

    wf = {
        "workflow_id": "self_healing_v2_%s" % case_id.replace("-", "_"),
        "name": "Self-Healing v2 SPOOF — %s" % case_id,
        "description": ("route->specialized attempt->detect->triage->gated heal "
                        "chain with budget governor and gray-zone judge. ZERO-LLM "
                        "spoof. Case: %s" % case_id),
        "version": "2.0.0",
        "author": "self-healing experiment",
        "tags": ["self-healing", "v2", "spoof", "zero-llm", case_id],
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


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--out-dir", default=str(EXP / "workflows" / "generated-v2"))
    args = ap.parse_args()

    wf, case_id = build(args.case)
    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    out = out_dir / ("sh-v2-%s.yml" % case_id)
    with out.open("w") as f:
        yaml.safe_dump(wf, f, sort_keys=False, default_flow_style=False, width=100)
    print(out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
