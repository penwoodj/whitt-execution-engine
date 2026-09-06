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


def test_gen_prompts_override_and_only():
    """v4 campaign (06-VERSION-ITERATION-RULESET.md): generator must
    accept --prompts (170-case corpus), --only (batch filter),
    --case-dir + --fixtures (test isolation, no clobber of real
    meta-fixes.yml / mp case files)."""
    import tempfile
    import shutil
    with tempfile.TemporaryDirectory() as td:
        td = Path(td)
        pf = td / "mini-prompts.yml"
        pf.write_text(
            "meta-prompts:\n"
            "  - prompt_id: tcA001\n"
            "    text: >\n"
            "      do a pipeline thing in /tmp/opencode/x\n"
            "    meta: {lane: HEAVY, shape: multi-stage-pipeline,"
            " batch: 1}\n"
            "  - prompt_id: tcA002\n"
            "    text: >\n"
            "      build a tool and verify it\n"
            "    meta: {lane: SYNTH, shape: tool-build-verify,"
            " batch: 1}\n"
            "  - prompt_id: tcA003\n"
            "    text: >\n"
            "      quick summary pass\n"
            "    meta: {lane: LIGHT, shape: quick-artifact, batch: 2}\n")
        out = td / "meta-v4-b01.yml"
        rd = td / "rd"
        cdir = td / "cases"
        fx = td / "fixtures" / "meta-fixes.yml"
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "gen-meta-system.py"),
             "--prompts", str(pf), "--only", "tcA001,tcA002",
             "--out", str(out), "--run-dir", str(rd),
             "--case-dir", str(cdir), "--fixtures", str(fx)],
            capture_output=True, text=True)
        ok("gen --prompts/--only exit 0", r.returncode == 0)
        ok("gen reports 2 prompts", "2 prompts" in r.stdout)
        text = out.read_text()
        ok("filtered in: tcA001 conf gate", "conf_tcA001:" in text)
        ok("filtered in: tcA002 synth lane stage",
           "ss0_split_tcA002:" in text)
        ok("filtered out: tcA003 absent", "tcA003" not in text)
        ok("case yml written tcA001",
           (cdir / "case-tcA001.yml").exists())
        ok("case yml NOT written tcA003",
           not (cdir / "case-tcA003.yml").exists())
        cy = (cdir / "case-tcA001.yml").read_text()
        ok("case yml carries batch meta", "batch" in cy)
        ok("probe written to run-dir", (rd / "probe-tcA001.json").exists())
        ok("fixtures written to override path", fx.exists())
        real_fx = FAS / "top-level" / "fixtures" / "meta-fixes.yml"
        ok("real meta-fixes.yml untouched",
           "tcA001" not in real_fx.read_text())


def test_ingest_testcases():
    """ingest-testcases.py: 170 corpus files -> meta-prompts-170.yml with
    id/lane/shape/batch mapping per 06-VERSION-ITERATION-RULESET.md §9."""
    import tempfile
    import yaml as _y
    with tempfile.TemporaryDirectory() as td:
        outp = Path(td) / "meta-prompts-170.yml"
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "ingest-testcases.py"),
             "--out", str(outp)], capture_output=True, text=True)
        ok("ingest exit 0", r.returncode == 0)
        d = _y.safe_load(outp.read_text())["meta-prompts"]
        ok("ingest 170 entries", len(d) == 170)
        by_id = {p["prompt_id"]: p for p in d}
        ok("tier ids present", all(
            f"tcA{i:03d}" in by_id for i in (1, 10, 100)) and
           "tcB001" in by_id and "tcC050" in by_id)
        ok("all lanes valid", all(
            p["meta"]["lane"] in ("HEAVY", "SYNTH", "LIGHT") for p in d))
        ok("batch mapping", by_id["tcA001"]["meta"]["batch"] == 1 and
           by_id["tcA010"]["meta"]["batch"] == 1 and
           by_id["tcA011"]["meta"]["batch"] == 2 and
           by_id["tcB001"]["meta"]["batch"] == 11 and
           by_id["tcC001"]["meta"]["batch"] == 13 and
           by_id["tcC050"]["meta"]["batch"] == 17)
        a1 = by_id["tcA001"]
        ok("prompt text non-trivial",
           len(a1["text"].split()) > 800)
        ok("lane matches file header (A001 HEAVY)",
           a1["meta"]["lane"] == "HEAVY")
        ok("tier-C all LIGHT", all(
            by_id[f"tcC{i:03d}"]["meta"]["lane"] == "LIGHT"
            for i in range(1, 51)))


def test_gen_chain_exit_routing():
    """Live lane chains MUST route to routelog on completion — without
    an after_step_succeeds GWT the engine falls through sequentially
    into the next declared chain (observed live b01-r1: HEAVY cases
    executed SYNTH+LIGHT stages too)."""
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        td = Path(td)
        pf = td / "p.yml"
        pf.write_text(
            "meta-prompts:\n"
            "  - prompt_id: tcX001\n"
            "    text: >\n"
            "      a heavy thing\n"
            "    meta: {lane: HEAVY, shape: multi-stage-pipeline,"
            " batch: 1}\n")
        out = td / "m.yml"
        subprocess.run(
            [sys.executable, str(SCRIPTS / "gen-meta-system.py"),
             "--prompts", str(pf), "--out", str(out),
             "--run-dir", str(td / "rd"),
             "--case-dir", str(td / "c"),
             "--fixtures", str(td / "f.yml")],
            capture_output=True, text=True, check=True)
        text = out.read_text()
        import re
        blocks = re.split(r"\n(?=    [a-zA-Z0-9_]+:)", text)
        chain_last = {"sh7_extract3_tcX001": "routelog_tcX001",
                      "sh6_extract_m_tcX001": "routelog_tcX001",
                      "sh1_extract_tcX001": "routelog_tcX001"}
        for sid, want in chain_last.items():
            blk = next((b for b in blocks
                        if b.startswith(f"    {sid}:")), None)
            ok(f"{sid} block exists", blk is not None)
            if blk:
                ok(f"{sid} after-hook routes to {want}",
                   "after_step_succeeds:" in blk and
                   f"then: {want}" in blk.split(
                       "after_step_succeeds:")[1])
        mid = next((b for b in blocks
                    if b.startswith("    sh1_solve_tcX001:")), "")
        ok("mid-chain stage after-hook absent or no fall-through route",
           "then: routelog_tcX001" not in mid.split(
               "after_step_succeeds:")[1] if "after_step_succeeds:" in mid
           else True)


def test_gen_resource_admission():
    """AGENTS rule 9 + schema L562-574/L912: generated live yml must
    declare workflow_execution_strategy.resource_admission (block policy,
    positive minima + estimates, write_profile telemetry) and every model
    needs absolute source_path, basename == model name, nonzero stat."""
    import tempfile
    import re
    with tempfile.TemporaryDirectory() as td:
        td = Path(td)
        pf = td / "mini-prompts.yml"
        pf.write_text(
            "meta-prompts:\n"
            "  - prompt_id: tcA001\n"
            "    text: >\n"
            "      do a pipeline thing\n"
            "    meta: {lane: HEAVY, shape: multi-stage-pipeline,"
            " batch: 1}\n")
        out = td / "gen.yml"
        rd = td / "rd"
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "gen-meta-system.py"),
             "--prompts", str(pf), "--only", "tcA001",
             "--out", str(out), "--run-dir", str(rd)],
            capture_output=True, text=True)
        ok("gen exit 0", r.returncode == 0)
        text = out.read_text()
        ok("resource_admission block present",
           "resource_admission:" in text)
        if "resource_admission:" in text:
            blk = text.split("resource_admission:", 1)[1].split(
                "\nmodels:", 1)[0]
            ok("enforcement_policy: block",
               "enforcement_policy: block" in blk)
            for key in ("ram:", "vram:", "swap_free:"):
                ok(f"minimum_available {key} positive",
                   re.search(key + r"\s*\d+", blk) is not None)
            for key in ("kv_cache:", "compute_buffer:", "host_runtime:",
                        "expected_runtime_secs:"):
                ok(f"model_estimate {key} positive",
                   re.search(key + r"\s*\d+", blk) is not None)
            ok("telemetry write_profile: true",
               "write_profile: true" in blk)
        sps = re.findall(r"source_path: (\S+)", text)
        ok("models declare source_path", len(sps) >= 1)
        for sp in sps:
            p = Path(sp)
            ok(f"basename matches model name ({p.name})",
                f'name: "{p.name}"' in text)
            ok(f"source_path exists ({sp})", p.exists())
            ok(f"source_path nonzero ({sp})",
               p.exists() and p.stat().st_size > 0)


def main():
    test_meta_conf()
    test_dry_runner_routing()
    test_top_level_generated()
    test_top_level_live_priors()
    test_top_level_dry_summary()
    test_gen_prompts_override_and_only()
    test_ingest_testcases()
    test_gen_chain_exit_routing()
    test_gen_resource_admission()
    print()
    if FAILS:
        print(f"{len(FAILS)} FAILURES: {FAILS}")
        sys.exit(1)
    print("ALL META TESTS PASS")


if __name__ == "__main__":
    main()
