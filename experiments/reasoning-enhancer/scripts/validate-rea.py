#!/usr/bin/env python3
"""REA workflow structural validator (offline, no engine, no cargo).

Usage:
  validate-rea.py [workflow.yml]

Mirrors correction-atom's live-validated structure (correction-atom-v8.yml)
against the unified workflow schema surface REA uses: provider contract,
model roster bounds (<=2 distinct gguf, no 9B), zero-LLM step convention,
shell-hook working_dir, $-prefixed save_to targets under __OUTPUT_DIR__,
gwt routing targets, and the 3-placeholder runtime contract. Exit 1 with
findings on any violation; exit 0 prints OK.
"""

import argparse
import re
import sys
from pathlib import Path

import yaml

ALLOWED_PLACEHOLDERS = {"__REPO_ROOT__", "__OUTPUT_DIR__", "__CASE_FILE__"}
TOP_KEYS = ["workflow_id", "name", "description", "version", "author", "tags",
            "schema_version", "min_schema_version", "providers", "models",
            "workflow_execution_strategy", "agentic_workflow"]
ENTITY_RE = re.compile(r"^\$\{models\.([A-Za-z0-9_]+)\}$")
PLACEHOLDER_RE = re.compile(r"__[A-Z_]+__")


def find_findings(wf, wf_text, allow_9b=False, batch=False):
    f = []

    for key in TOP_KEYS:
        if key not in wf:
            f.append(f"missing top-level key: {key}")
    if f:
        return f
    if str(wf["schema_version"]) != "2.0.0":
        f.append(f"schema_version must be '2.0.0', got {wf['schema_version']!r}")

    providers = wf["providers"]
    if set(providers) != {"llama_cpp_with_vulkan"}:
        f.append(f"providers must be exactly llama_cpp_with_vulkan, got {sorted(providers)}")
    else:
        cfg = providers["llama_cpp_with_vulkan"].get("config") or {}
        if "host" not in cfg or "port" not in cfg:
            f.append("provider config missing host/port")

    models = wf["models"]
    if not models:
        f.append("models empty")
    ggufs = set()
    for alias, spec in models.items():
        gguf = spec.get("name", "")
        ggufs.add(gguf)
        if not allow_9b and ("9B" in gguf or "9b" in gguf):
            f.append(f"model {alias}: 9B forbidden (v8 learning 3)")
        if (spec.get("host") or {}).get("type") != "llama_cpp_with_vulkan":
            f.append(f"model {alias}: host.type must be llama_cpp_with_vulkan")
        sampling = spec.get("sampling") or {}
        if "temperature" not in sampling or "max_tokens" not in sampling:
            f.append(f"model {alias}: sampling needs temperature + max_tokens")
        elif sampling["max_tokens"] > 500:
            f.append(f"model {alias}: max_tokens {sampling['max_tokens']} > 500 budget")
    if len(ggufs) > 2:
        f.append(f"{len(ggufs)} distinct gguf models > 2 (sequential swap bound)")

    try:
        unload = wf["workflow_execution_strategy"]["memory"]["model_lifecycle"]["unload_unused"]
        if unload is not True and not (allow_9b and unload is False):
            f.append("workflow_execution_strategy.memory.model_lifecycle.unload_unused must be true "
                     "(baseline --allow-9b may set false to keep the model resident)")
    except (KeyError, TypeError):
        f.append("workflow_execution_strategy.memory.model_lifecycle.unload_unused missing")

    steps = wf["agentic_workflow"].get("steps") or {}
    if not steps:
        return f + ["agentic_workflow.steps empty"]
    step_cap = 300 if batch else 10
    if len(steps) > step_cap:
        f.append(f"{len(steps)} steps > {step_cap} bound")
    llm_cap = 300 if batch else 6

    llm_steps = 0
    for sname, step in steps.items():
        entity = step.get("generative_entity", "")
        m = ENTITY_RE.match(entity)
        if not m:
            f.append(f"{sname}: generative_entity {entity!r} not ${{models.alias}}")
        elif m.group(1) not in models:
            f.append(f"{sname}: entity alias {m.group(1)!r} not in models")

        prompt = step.get("prompt", "")
        overrides = step.get("model_overrides") or {}
        when = step.get("when") or {}
        is_zero_llm = prompt.strip() == "OK."

        if is_zero_llm:
            if overrides.get("temperature") != 0.0 or overrides.get("max_tokens", 99) > 3:
                f.append(f"{sname}: zero-LLM step must override temperature 0.0, max_tokens<=3")
            if not _has_hook(when.get("before_step_starts"), "skip_step"):
                f.append(f"{sname}: zero-LLM step missing skip_step hook")
        else:
            llm_steps += 1
            if "temperature" not in overrides or "max_tokens" not in overrides:
                f.append(f"{sname}: LLM step missing model_overrides temperature/max_tokens")

        for trigger in ("before_step_starts", "after_step_succeeds"):
            for hook in when.get(trigger) or []:
                _check_hook(sname, trigger, hook, steps, f, batch=batch)

    if llm_steps > llm_cap:
        f.append(f"{llm_steps} LLM steps > {llm_cap} inference budget")
    if "step_99_finalize" not in steps:
        f.append("step_99_finalize missing (gwt finalize target)")

    bad = set(PLACEHOLDER_RE.findall(wf_text)) - ALLOWED_PLACEHOLDERS
    for ph in sorted(bad):
        f.append(f"unknown placeholder {ph} (allowed: {sorted(ALLOWED_PLACEHOLDERS)})")

    return f


def _has_hook(hooks, key):
    return any(key in h for h in (hooks or []))


def _check_hook(sname, trigger, hook, steps, f, batch=False):
    if "shell" in hook:
        sh = hook["shell"]
        cmd = sh.get("command", "")
        if not cmd:
            f.append(f"{sname}.{trigger}: shell hook empty command")
        if sh.get("working_dir") != "__REPO_ROOT__":
            f.append(f"{sname}.{trigger}: shell working_dir must be __REPO_ROOT__ (repo-root confinement)")
        if "fail_on_error" not in sh:
            f.append(f"{sname}.{trigger}: shell hook missing fail_on_error")
        if cmd.startswith("python3") and "experiments/reasoning-enhancer/scripts/" not in cmd:
            f.append(f"{sname}.{trigger}: python3 hook outside experiments/reasoning-enhancer/scripts/")
        for must_true in ("--phase gate-prepare", "--phase toolverify"):
            if must_true in cmd and sh.get("fail_on_error") is not True and not batch:
                f.append(f"{sname}.{trigger}: {must_true} shell must fail_on_error: true")
    elif "save_to" in hook:
        sv = hook["save_to"]
        ok = (isinstance(sv, list) and len(sv) == 2 and sv[0].startswith("$")
              and (sv[1].startswith("__OUTPUT_DIR__/") or (batch and sv[1].startswith("/"))))
        if not ok:
            f.append(f"{sname}.{trigger}: save_to must be [$var, __OUTPUT_DIR__/...] — got {sv!r}")
    elif "gwt" in hook:
        for clause in hook["gwt"] or []:
            if "given" not in clause or "then" not in clause:
                f.append(f"{sname}.{trigger}: gwt clause needs given+then")
            elif clause["then"] not in steps:
                f.append(f"{sname}.{trigger}: gwt routes to unknown step {clause['then']!r}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("workflow", nargs="?", default=str(Path(__file__).parent.parent / "workflows" / "rea-v1.yml"))
    ap.add_argument("--allow-9b", action="store_true",
                    help="permit a 9B model (baseline workflows only — the enhancer never uses 9B)")
    ap.add_argument("--batch", action="store_true",
                    help="batch-baseline mode: relax step/LLM caps (sequential single-model "
                         "inferences are cheaper than swaps; still bounded at 60 steps)")
    args = ap.parse_args()
    path = args.workflow
    wf_text = Path(path).read_text()
    try:
        wf = yaml.safe_load(wf_text)
    except yaml.YAMLError as exc:
        print(f"FAIL: {path} does not parse: {exc}")
        return 1
    findings = find_findings(wf, wf_text, allow_9b=args.allow_9b, batch=args.batch)
    if findings:
        print("\n".join(f"FAIL: {x}" for x in findings))
        return 1
    llm = sum(1 for s in wf["agentic_workflow"]["steps"].values()
              if s.get("prompt", "").strip() != "OK.")
    print(f"OK: {path} structural validation passed "
          f"({len(wf['agentic_workflow']['steps'])} steps, {llm} LLM)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
