#!/usr/bin/env python3
"""gen-live-v2.py — live (REAL LLM) workflow generator for self-healing v2.

Usage:
    python3 gen-live-v2.py --case <case.yml> [--out-dir <dir>]

Differences vs gen-v2.py (spoof):
  - attempt steps RUN REAL INFERENCE (no skip hook; prep-gwt guards siblings)
  - prompt = "{{bookmarks.shell_output.stdout}}" fed by step's own prep shell
  - save_to captures raw model text; inject_live parses+corrupts per case profile
  - judge + heal_replan are REAL LLM steps with pre/post shells
  - results base: experiments/self-healing/results/live-v2/
  - unload_unused: true (VRAM safety, hot-swap proven in benchmark-3-models.yml)
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import Final

import yaml

REPO = Path(__file__).resolve().parents[3]
EXP = REPO / "experiments" / "self-healing"

MODEL_SOURCES: Final[dict[str, str]] = {
    "m_worker": "/run/media/jon/data/models/Qwen3-4B-Instruct-2507-Q4_K_M.gguf",
    "m_fast": "/run/media/jon/data/models/Ministral-3-3B-Instruct-2512-Q4_K_M.gguf",
    "m_coder": "/run/media/jon/data/models/Qwen2.5-Coder-3B-Instruct-Q8_0.gguf",
    "m_heavy": "/run/media/jon/data/models/Qwen3-8B-Q4_K_M.gguf",
    "m_judge": "/run/media/jon/data/models/Hermes-2-Pro-Mistral-7B.Q4_K_M.gguf",
}
MODELS = {alias: Path(source_path).name for alias, source_path in MODEL_SOURCES.items()}
LIVE_LOAD_PARAMS = {
    "context_size": 4096,
    "gpu_layers": 99,
    "cont_batching": False,
    "no_cache_prompt": True,
    "parallel": 1,
}
LIVE_RESOURCE_ADMISSION = {
    "enforcement_policy": "block",
    "minimum_available": {"ram": "6GiB", "vram": "6GiB", "swap_free": "4GiB"},
    "model_estimate": {
        "kv_cache": "288MiB",
        "compute_buffer": "512MiB",
        "host_runtime": "700MiB",
        "expected_runtime_secs": 900,
    },
    "telemetry": {"write_profile": True},
}
SCHEMA_LINE_COMMENTS: Final[dict[str, int]] = {
    "source_path": 911,
    "load_params": 75,
    "context_size": 76,
    "gpu_layers": 81,
    "cont_batching": 85,
    "no_cache_prompt": 86,
    "parallel": 87,
    "load_unload": 534,
    "resource_admission": 562,
    "enforcement_policy": 563,
    "minimum_available": 564,
    "ram": 565,
    "vram": 566,
    "swap_free": 567,
    "model_estimate": 568,
    "kv_cache": 569,
    "compute_buffer": 570,
    "host_runtime": 571,
    "expected_runtime_secs": 572,
    "telemetry": 573,
    "write_profile": 574,
}

# step offsets within a round (round base + offset)
OFF = {
    "route": 0,
    "att_fast": 1,
    "att_coder": 2,
    "att_general": 3,
    "att_heavy": 4,
    "detect": 5,
    "triage": 6,
    "jgate": 7,
    "judge": 8,
    "jeval": 9,
    "hlight": 10,
    "budget": 11,
    "hreplan": 12,
}
ROUND_BASE = {1: 1, 2: 14, 3: 27}

ACCEPT = "step_40_accept"
FINAL_FAIL = "step_41_final_fail"
REPORT = "step_42_report"
END_PASS = "step_43_end_pass"
END_FAIL = "step_44_end_fail"
END = "step_45_end"

EC = "{{bookmarks.shell_output.exit_code}}"
STDOUT = "{{bookmarks.shell_output.stdout}}"
S = "experiments/self-healing/scripts"
SL = "experiments/self-healing/scripts/live"


def sid(round_: int, kind: str) -> str:
    return "step_%02d_%s_%d" % (ROUND_BASE[round_] + OFF[kind], kind, round_)


def sh(cmd: str) -> dict:
    return {"shell": {"command": cmd, "working_dir": ".", "fail_on_error": False}}


def gwt(clauses: list[tuple[str, str]]) -> dict:
    return {"gwt": [{"given": g, "then": t} for g, t in clauses]}


def skip() -> dict:
    return {"skip_step": True}


ALIAS_OF_VARIANT = {"att_fast": "m_fast", "att_coder": "m_coder",
                    "att_general": "m_worker", "att_heavy": "m_heavy"}
ROLE_OF_VARIANT = {"att_fast": "worker_fast", "att_coder": "worker_coder",
                   "att_general": "worker_general", "att_heavy": "heavy_replanner"}


def att_variant_key(kind: str) -> str:
    return kind.replace("att_", "")


def build(case_path: Path) -> dict:
    case = yaml.safe_load(case_path.read_text())
    case_id = case["case_id"]
    run_dir = str(EXP / "results" / "live-v2" / case_id)
    case_abs = str(case_path.resolve())
    base = str(EXP / "results" / "live-v2")

    steps: dict[str, dict] = {}

    def add(name: str, model: str, prompt: str, before: list, after: list | None = None,
            overrides: dict | None = None) -> None:
        st = {
            "generative_entity": "${models.%s}" % model,
            "prompt": prompt,
            "model_overrides": overrides or {"temperature": 0.2, "max_tokens": 10000},
            "when": {"before_step_starts": before},
        }
        if after:
            st["when"]["after_step_succeeds"] = after
        steps[name] = st

    # step_00_init
    add(
        "step_00_init",
        "m_worker",
        "init",
        [
            sh(f"mkdir -p {run_dir} && python3 {S}/end_event_v2.py --run-dir {run_dir} --status init"),
            gwt([("1 == 1", sid(1, "route"))]),
            skip(),
        ],
    )

    for r in (1, 2, 3):
        b = ROUND_BASE[r]

        # route_r
        add(
            sid(r, "route"),
            "m_worker",
            f"route round {r}",
            [
                sh(f"python3 {S}/route_v2.py --case {case_abs} --run-dir-base {base} --round {r}"),
                gwt([
                    (f"{EC} == 0", sid(r, "att_fast")),
                    (f"{EC} == 1", sid(r, "att_coder")),
                    (f"{EC} == 2", sid(r, "att_general")),
                    (f"{EC} != 2", sid(r, "att_heavy")),
                ]),
                skip(),
            ],
        )

        # attempt variants — REAL INFERENCE
        for kind in ("att_fast", "att_coder", "att_general", "att_heavy"):
            variant = att_variant_key(kind)
            mkey = ALIAS_OF_VARIANT[kind]
            add(
                sid(r, kind),
                mkey,
                STDOUT,
                [
                    sh(
                        f"python3 {SL}/prep_attempt_live.py --case {case_abs} "
                        f"--run-dir-base {base} --round {r} --variant {variant}"
                    ),
                    gwt([(f"{EC} == 42", sid(r, "detect"))]),
                ],
                [
                    {"save_to": f"{run_dir}/attempt_{r}_raw.txt"},
                    sh(
                        f"python3 {SL}/inject_live.py --case {case_abs} --run-dir-base {base} "
                        f"--attempt {r} --model-role {ROLE_OF_VARIANT[kind]} --raw {run_dir}/attempt_{r}_raw.txt"
                    ),
                ],
            )

        # detect_r
        add(
            sid(r, "detect"),
            "m_worker",
            f"detect round {r}",
            [
                sh(f"python3 {S}/detect_v2.py --case {case_abs} --run-dir-base {base} --attempt {r}"),
                gwt([(f"{EC} == 0", sid(r, "triage"))]),
                skip(),
            ],
        )

        # triage_r (round 3 has no heals: light/replan both → final_fail)
        if r < 3:
            triage_clauses = [
                (f"{EC} == 0", sid(r, "jgate")),
                (f"{EC} == 1", sid(r, "hlight")),
                (f"{EC} == 2", sid(r, "budget")),
                (f"{EC} != 0", FINAL_FAIL),
            ]
        else:
            triage_clauses = [
                (f"{EC} == 0", sid(r, "jgate")),
                (f"{EC} != 0", FINAL_FAIL),
            ]
        add(
            sid(r, "triage"),
            "m_worker",
            f"triage round {r}",
            [
                sh(f"python3 {S}/triage_v2.py --case {case_abs} --run-dir-base {base} --attempt {r}"),
                gwt(triage_clauses),
                skip(),
            ],
        )

        # jgate_r — prints judge prompt only if gray zone
        add(
            sid(r, "jgate"),
            "m_worker",
            f"judge gate round {r}",
            [
                sh(f"python3 {SL}/jgate_prep_live.py --case {case_abs} --run-dir-base {base} --attempt {r}"),
                gwt([
                    (f"{EC} == 0", ACCEPT),
                    (f"{EC} != 0", sid(r, "judge")),
                ]),
                skip(),
            ],
        )

        # judge_r — REAL LLM (Hermes), fed by jgate stdout
        add(
            sid(r, "judge"),
            "m_judge",
            STDOUT,
            [],
            [
                {"save_to": f"{run_dir}/judge_{r}_raw.txt"},
                sh(f"python3 {SL}/judge_eval_live.py --run-dir {run_dir} --round {r} --raw {run_dir}/judge_{r}_raw.txt"),
            ],
            overrides={"temperature": 0.0, "max_tokens": 300},
        )

        # jeval_r — judge verdict exit code
        if r < 3:
            jeval_clauses = [
                (f"{EC} == 0", ACCEPT),
                (f"{EC} != 0", sid(r + 1, "route")),
            ]
        else:
            jeval_clauses = [
                (f"{EC} == 0", ACCEPT),
                (f"{EC} != 0", FINAL_FAIL),
            ]
        add(
            sid(r, "jeval"),
            "m_worker",
            f"judge eval round {r}",
            [
                gwt(jeval_clauses),
                skip(),
            ],
        )

        if r < 3:
            # hlight_r — script heal (no LLM)
            add(
                sid(r, "hlight"),
                "m_worker",
                f"heal light round {r}",
                [
                    sh(f"python3 {S}/heal_light_v2.py --case {case_abs} --run-dir-base {base} --round {r}"),
                    gwt([("1 == 1", sid(r + 1, "route"))]),
                    skip(),
                ],
            )

            # budget_r — replan budget gate
            add(
                sid(r, "budget"),
                "m_worker",
                f"budget round {r}",
                [
                    sh(f"python3 {S}/budget_v2.py --run-dir {run_dir} --reserve 1 --site replan_r{r}"),
                    gwt([
                        (f"{EC} == 0", sid(r, "hreplan")),
                        (f"{EC} != 0", FINAL_FAIL),
                    ]),
                    skip(),
                ],
            )

            # hreplan_r — REAL LLM (heavy), pre shell prints replan prompt
            add(
                sid(r, "hreplan"),
                "m_heavy",
                STDOUT,
                [
                    sh(
                        f"python3 {SL}/heal_replan_live.py --mode pre --case {case_abs} "
                        f"--run-dir-base {base} --round {r}"
                    ),
                    gwt([(f"{EC} != 0", FINAL_FAIL)]),
                ],
                [
                    {"save_to": f"{run_dir}/heal_{r}_raw.txt"},
                    sh(
                        f"python3 {SL}/heal_replan_live.py --mode post --case {case_abs} "
                        f"--run-dir-base {base} --round {r} --raw {run_dir}/heal_{r}_raw.txt"
                    ),
                ],
                overrides={"temperature": 0.2, "max_tokens": 800},
            )

    # terminal steps
    add(
        ACCEPT,
        "m_worker",
        "accept",
        [
            sh(f"python3 {S}/outcome_v2.py --case {case_abs} --run-dir-base {base} --outcome accept"),
            gwt([("1 == 1", REPORT)]),
            skip(),
        ],
    )
    add(
        FINAL_FAIL,
        "m_worker",
        "final fail",
        [
            sh(f"python3 {S}/outcome_v2.py --case {case_abs} --run-dir-base {base} --outcome final_fail"),
            gwt([("1 == 1", REPORT)]),
            skip(),
        ],
    )
    add(
        REPORT,
        "m_worker",
        "report",
        [
            sh(f"python3 {S}/report_v2.py --case {case_abs} --run-dir {run_dir}"),
            gwt([
                (f"{EC} == 0", END_PASS),
                (f"{EC} != 0", END_FAIL),
            ]),
            skip(),
        ],
    )
    add(
        END_PASS,
        "m_worker",
        "end pass",
        [
            sh(f"python3 {S}/end_event_v2.py --run-dir {run_dir} --status pass"),
            gwt([("1 == 1", END)]),
            skip(),
        ],
    )
    add(
        END_FAIL,
        "m_worker",
        "end fail",
        [
            sh(f"python3 {S}/end_event_v2.py --run-dir {run_dir} --status fail"),
            gwt([("1 == 1", END)]),
            skip(),
        ],
    )
    add(
        END,
        "m_worker",
        "end",
        [gwt([("1 == 0", "step_00_init")]), skip()],
    )

    # route-target existence assert
    targets = [
        tgt
        for st in steps.values()
        for h in st.get("when", {}).get("before_step_starts", [])
        if "gwt" in h
        for tgt in [c["then"] for c in h["gwt"]]
    ]
    missing = sorted(t for t in set(targets) if t not in steps)
    assert not missing, "gen-live-v2 route targets missing: %s" % missing

    wf = {
        "workflow_id": "self_healing_live_v2_%s" % case_id.replace("-", "_"),
        "name": "Self-Healing v2 LIVE — %s" % case_id,
        "description": (
            "Live real-LLM self-healing workflow. Real inference at attempt/"
            "judge/heal_replan sites; deterministic detect/triage/heals elsewhere. Case: %s"
        ) % case_id,
        "version": "2.0.0",
        "author": "self-healing experiment",
        "tags": ["self-healing", "v2", "live", "real-llm", case_id],
        "schema_version": "2.0.0",
        "min_schema_version": "2.0.0",
        "providers": {
            "llama_cpp_with_vulkan": {"config": {"host": "localhost", "port": 8080}},
        },
        "models": {
            alias: {
                "name": mname,
                "source_path": MODEL_SOURCES[alias],
                "host": {"type": "llama_cpp_with_vulkan"},
                "load_params": LIVE_LOAD_PARAMS.copy(),
            }
            for alias, mname in MODELS.items()
        },
        "workflow_execution_strategy": {
            "load_unload": "one_at_a_time",
            "resource_admission": LIVE_RESOURCE_ADMISSION,
            "timing": {"cooldown_after_unload_secs": 3, "min_tmp_space_mb": 1024},
            "memory": {"model_lifecycle": {"unload_unused": True}},
        },
        "agentic_workflow": {
            "when": {"after_workflow": [{"log": {"level": "info"}}]},
            "steps": steps,
        },
    }
    return wf


def render_workflow(workflow: dict) -> str:
    rendered = yaml.safe_dump(workflow, sort_keys=False, width=100)
    for field, schema_line in SCHEMA_LINE_COMMENTS.items():
        rendered = re.sub(
            rf"^(\s*{field}:.*)$",
            rf"\1 # schema line {schema_line}",
            rendered,
            flags=re.MULTILINE,
        )
    return rendered


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--out-dir", default=str(EXP / "workflows" / "generated-live-v2"))
    args = ap.parse_args()

    case_path = Path(args.case).resolve()
    wf = build(case_path)

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    out = out_dir / ("sh-live-v2-%s.yml" % case_path.stem)
    out.write_text(render_workflow(wf))
    print(out)


if __name__ == "__main__":
    sys.exit(main())
