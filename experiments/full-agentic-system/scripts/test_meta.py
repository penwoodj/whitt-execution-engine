#!/usr/bin/env python3
"""Unit tests for top-level meta-system machinery:
meta-conf CLI, dry-meta-run routing semantics, generator output shape.
Plain runner, exit 1 on any failure."""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
sys.path.insert(0, str(SCRIPTS))

FAILS = []


def ok(name, cond):
    print(("PASS" if cond else "FAIL") + f"  {name}")
    if not cond:
        FAILS.append(name)


def sh(cmd, cwd=None):
    r = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True,
                       text=True)
    return r.stdout


def test_meta_conf():
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        (tmp / "p1.json").write_text('{"lane": "HEAVY"}')
        (tmp / "p2.json").write_text('{"lane": "SYNTH"}')
        (tmp / "p3.json").write_text('{"lane": "LIGHT"}')
        (tmp / "p4.json").write_text(
            '{"logprobs": [-0.01, -5.0, -5.0], "multi_part": false}')
        (tmp / "p5.json").write_text(
            '{"logprobs": [-1.2, -1.1, -1.3], "multi_part": false}')
        (tmp / "p6.json").write_text(
            '{"logprobs": [-0.1], "multi_part": true}')
        ok("conf lane passthrough HEAVY",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p1.json"
              ).strip() == '"HEAVY"')
        ok("conf lane passthrough SYNTH",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p2.json"
              ).strip() == '"SYNTH"')
        ok("conf lane passthrough LIGHT",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p3.json"
              ).strip() == '"LIGHT"')
        ok("conf peaked logprobs -> LIGHT",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p4.json"
              ).strip() == '"LIGHT"')
        ok("conf flat logprobs -> HEAVY",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p5.json"
              ).strip() == '"HEAVY"')
        ok("conf multi_part -> SYNTH",
           sh(f"python3 {SCRIPTS}/meta-conf.py --probe {tmp}/p6.json"
              ).strip() == '"SYNTH"')
        ok("conf missing probe -> default",
           sh(f"python3 {SCRIPTS}/meta-conf.py "
              f"--probe {tmp}/nope.json --default LIGHT"
              ).strip() == '"LIGHT"')


TINY_WF = """workflow_id: tiny
name: "tiny"
description: "tiny"
version: "1.0.0"
author: "t"
tags: [t]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
workflow_execution_strategy:
  timing:
    cooldown_after_unload_secs: 0
    min_tmp_space_mb: 50
models:
  m_fmt:
    name: "m"
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  when:
    after_workflow:
      - log:
          level: info
          to_file_path: TINY/final.log
          event_fields: [workflow_id]
  steps:
    gate:
      generative_entity: "${{models.m_fmt}}"
      prompt: route
      model_overrides:
        temperature: 0.0
        max_tokens: 3
      when:
        before_step_starts:
          - shell:
              command: "echo '\\"HEAVY\\"'"
              working_dir: .
              fail_on_error: true
          - gwt:
              - given: '{{bookmarks.shell_output.stdout}} == "HEAVY"'
                then: heavy_step
              - given: '{{bookmarks.shell_output.stdout}} == "LIGHT"'
                then: light_step
    light_step:
      generative_entity: "${{models.m_fmt}}"
      prompt: l
      model_overrides:
        temperature: 0.0
        max_tokens: 3
      when:
        before_step_starts:
          - shell:
              command: "echo '\\"SKIP\\"'"
              working_dir: .
              fail_on_error: true
          - gwt:
              - given: '{{bookmarks.shell_output.stdout}} == "PASS"'
                then: terminal
              - given: '{{bookmarks.shell_output.stdout}} == "SKIP"'
                then: terminal
    heavy_step:
      generative_entity: "${{models.m_fmt}}"
      prompt: h
      model_overrides:
        temperature: 0.0
        max_tokens: 3
      when:
        before_step_starts:
          - shell:
              command: "echo '\\"PASS\\"'"
              working_dir: .
              fail_on_error: true
          - gwt:
              - given: '{{bookmarks.shell_output.stdout}} == "PASS"'
                then: terminal
              - given: '{{bookmarks.shell_output.stdout}} == "SKIP"'
                then: terminal
    terminal:
      generative_entity: "${{models.m_fmt}}"
      prompt: terminal
      model_overrides:
        temperature: 0.0
        max_tokens: 3
      when:
        before_step_starts:
          - skip_step: true
"""


def test_dry_runner_routing():
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        (tmp / "workflows").mkdir()
        wf = tmp / "workflows" / "tiny.yml"
        wf.write_text(TINY_WF.replace("TINY", str(tmp)))
        out = sh(f"python3 {SCRIPTS}/dry-meta-run.py --workflow {wf}")
        s = json.loads(out)
        ok("tiny: completes under hop budget", s["hops"] <= 10)
        ok("tiny: heavy branch taken not fall-through",
           s["steps_executed"] == 3)  # gate, heavy_step, terminal-skip
        sumf = tmp / "runs" / "tiny-dry-summary.json"
        ok("tiny: summary written", sumf.exists())


def test_top_level_generated():
    yml = FAS / "top-level" / "workflows" / "meta-v1-spoof.yml"
    ok("top-level spoof yml exists", yml.exists())
    text = yml.read_text()
    prompts = yaml_prompts()
    for p in prompts:
        ok(f"gen emits conf gate {p}",
           f"conf_{p}:" in text)
        ok(f"gen emits routelog {p}", f"routelog_{p}:" in text)
    ok("gen uses lane-head route_to",
        '== "SYNTH"' in text and '== "LIGHT"' in text)


def yaml_prompts():
    import yaml
    d = yaml.safe_load(
        (FAS / "prompts" / "meta-prompts.yml").read_text())
    return [p["prompt_id"] for p in d["meta-prompts"]]


def test_top_level_live_priors():
    """Live extract stages MUST pass --prior so stage-emit finds prior
    answer files; without priors extract hits skip_stage() (exit 42),
    no ans-extract files are written, and routelog observes lane=null."""
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        out = Path(td) / "meta-live-test.yml"
        rd = Path(td) / "rd"
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "gen-meta-system.py"),
             "--out", str(out), "--run-dir", str(rd)],
            capture_output=True, text=True)
        ok("live gen exits 0", r.returncode == 0)
        text = out.read_text()
        expect = {
            "extract": "solve", "extract2": "solve2",
            "extract3": "verify", "extract_a": "worker_a",
            "extract_b": "worker_b", "extract_m": "merge",
        }
        emit_lines = [ln for ln in text.splitlines()
                      if "stage-emit.py" in ln and "command:" in ln]
        ok("live gen emits stage-emit commands", len(emit_lines) > 0)
        for stage, prior in expect.items():
            stage_lines = [ln for ln in emit_lines
                           if f"--stage {stage} " in ln]
            ok(f"live stage {stage} emitted", len(stage_lines) > 0)
            ok(f"live stage {stage} wires --prior {prior}",
               all(f"--prior {prior}" in ln for ln in stage_lines))


def test_top_level_dry_summary():
    s = FAS / "top-level" / "runs" / "meta-v1-spoof-dry-summary.json"
    if not s.exists():
        ok("top-level dry summary exists (run dry-meta-run first)",
           False)
        return
    d = json.loads(s.read_text())
    ok("dry: all 8 routelog checks passed", d["checks_passed"] == 8)
    ok("dry: zero failed checks", d["checks_failed"] == 0)


def main():
    test_meta_conf()
    test_dry_runner_routing()
    test_top_level_generated()
    test_top_level_live_priors()
    test_top_level_dry_summary()
    print()
    if FAILS:
        print(f"{len(FAILS)} FAILURES: {FAILS}")
        sys.exit(1)
    print("ALL META TESTS PASS")


if __name__ == "__main__":
    main()
