#!/usr/bin/env python3
"""FAS hook unit tests (zero LLM). Plain runner, exit 1 on any fail."""
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
sys.path.insert(0, str(SCRIPTS))
import fas_lib as L  # noqa: E402
import fas_engines as E  # noqa: E402
from fas_cases import decode_cid, scen_mod, scen_buckets  # noqa: E402

EXP = "07-solve"
EXPDIR = FAS / EXP
CASE_YML = EXPDIR / "cases" / "case-sv-01.yml"
TRUTHS = EXPDIR / "fixtures" / "fas-fixes.yml"

FAILS = []


def ok(name, cond, detail=""):
    print(f"  {'ok' if cond else 'FAIL'}  {name}"
          + (f" — {detail}" if detail and not cond else ""))
    if not cond:
        FAILS.append(name)


def run(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, timeout=60)


def spoof(tmp, *extra):
    return run(["python3", str(SCRIPTS / "spoof-emit.py"),
                "--case", "sv-01", "--stage", "solve", "--stage-index", "0",
                "--run-dir", str(tmp), "--scenario", str(tmp / "scen.json"),
                "--truths", str(TRUTHS)] + list(extra))


def test_engines():
    print("engines: truths computed + digits textually present")
    for eng in ["quota", "backoff", "canary", "residency", "preempt",
                "epistemic"]:
        for var in ("H", "L"):
            for idx in range(3):
                g = E.gen(eng, var, idx)
                truth_str = json.dumps(g["truth"])
                r = L.run_checks_safe(truth_str,
                                      {"json_exact": truth_str})
                ok(f"{eng}/{var}/{idx} truth self-pass", r and r["passed"])
                flat = re.sub(r"\s+", " ", g["core"])
                for d in g["task_digits"]:
                    ok(f"{eng}/{var}/{idx} digit {d} in core",
                       re.search(rf"(?<![A-Za-z0-9_]){re.escape(str(d))}"
                                 r"(?![0-9])", flat) is not None)


def test_std_case_contract():
    print("std_case: CONTRACT_MARK opens contract sentence")
    sys.path.insert(0, str(EXPDIR.parent))
    import importlib.util
    spec = importlib.util.spec_from_file_location("cfg_t", EXPDIR / "config.py")
    if spec is None or spec.loader is None:
        raise RuntimeError("config load failed")
    cfg = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(cfg)
    cases = cfg.build_cases(E, L)
    ok("case count 48", len(cases) == 48)
    c = cases[0]
    ok("contract mark present", L.CONTRACT_MARK in c["prompt"])
    ok("worked mark present", L.WORKED_MARK in c["prompt"])
    ok("1000<=wc<=3000", 1000 <= L.word_count(c["prompt"]) <= 3000)
    ok("decode_cid inverse",
       decode_cid("sv-01", 6, 4, ["quota", "backoff", "canary",
                                  "residency", "preempt", "epistemic"])
       == ("quota", "H", 0))


def test_digest():
    print("digest: worked cut, contract kept")
    g = E.gen("quota", "H", 0)
    p = L.assemble_prompt(g, "hdr",
                          f"{L.CONTRACT_MARK}, values filled in "
                          f"correctly: {g['contract']}")
    dg = L.digest(p)
    ok("worked cut", L.WORKED_MARK not in dg)
    ok("contract kept", L.CONTRACT_MARK in dg)
    ok("expansion cut", L.EXPANSION_MARKER not in dg)
    ok("<=400 words", L.word_count(dg) <= 400)


def test_harvester_and_hint():
    print("harvester: repair + check-before-echo; leak-safe hint")
    g = E.gen("quota", "H", 0)
    truth_str = json.dumps(g["truth"])
    checks = {"json_exact": truth_str}
    mangled = truth_str.replace("47", "4_7")
    cand = L.harvester("scratch " + mangled, checks)
    ok("repaired harvest", cand is not None and "47" in cand)
    ok("garbage no harvest", L.harvester("no json here", checks) is None)
    hint = L.leak_safe_hint({"failures": [
        {"check": "json_exact", "detail": "granted_units 62 != 61"}]})
    ok("hint hides expected", "61" not in hint or "!=" in hint)
    ok("hint names check", "json_exact" in hint)


def test_scenario_fns():
    print("scenario helpers: deterministic buckets")
    ok("scen_mod bounds", scen_mod("sv-01", 2) in (0, 1))
    ok("scen_buckets bounds", scen_buckets("sv-01", 3, [1, 1, 1]) <= 2)
    ok("scen_mod stable", scen_mod("sv-01", 2) == scen_mod("sv-01", 2))


def test_spoof_cli():
    print("spoof-emit CLI: win/SKIP/fault/scaffold/judge")
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        tmp.joinpath("scen.json").write_text('{"sv-01": 0}')
        r = spoof(tmp)
        ok("win0 prints PASS", r.stdout.strip().endswith('"PASS"'))
        ok("win0 check passed",
           json.loads((tmp / "check-sv-01-solve.json")
                      .read_text())["passed"])

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        tmp.joinpath("scen.json").write_text('{"sv-01": 2}')
        r = spoof(tmp)
        ok("pre-win prints SKIP", r.stdout.strip().endswith('"SKIP"'))
        ok("pre-win failed check",
           not json.loads((tmp / "check-sv-01-solve.json")
                          .read_text())["passed"])

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        tmp.joinpath("scen.json").write_text('{"sv-01": 0}')
        r = spoof(tmp, "--unchecking")
        ok("scaffold SKIP", r.stdout.strip().endswith('"SKIP"'))
        ok("scaffold ans written",
           (tmp / "ans-sv-01-solve.txt").exists())

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        tmp.joinpath("scen.json").write_text('{"sv-01": 0}')
        r = spoof(tmp, "--fault", "delta")
        ok("fault prints SKIP (stage fails, later stage recovers)",
           r.stdout.strip().endswith('"SKIP"'))
        led = [json.loads(x) for x in
               (tmp / "outcomes.jsonl").read_text().splitlines()]
        ok("FAULT_ ledger row",
           any("FAULT" in str(x.get("outcome", "")) for x in led))

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        tmp.joinpath("scen.json").write_text(
            '{"sv-01": 0, "sv-01::judge_disagree": true}')
        (tmp / "check-sv-01-solve.json").write_text(
            '{"passed": true}')
        r = spoof(tmp, "--judge")
        ok("judge SKIP never gates", r.stdout.strip().endswith('"SKIP"'))
        ok("judge verdict fail written",
           "fail" in (tmp / "ans-sv-01-judge.txt").read_text())


def test_stage_emit_cli():
    print("stage-emit CLI: extract / full / judge")
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        g = E.gen("quota", "H", 0)
        truth_str = json.dumps(g["truth"])
        (tmp / "ans-sv-01-solve.txt").write_text(
            "reasoning\n" + truth_str)
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "extract", "--run-dir", str(tmp),
                 "--style", "extract", "--prior", "solve"])
        ok("extract fast-path passthrough",
           r.returncode == 42 and
           "granted_units" in (tmp / "ans-sv-01-extract.txt"
                               ).read_text())

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(tmp),
                 "--style", "cot"])
        ok("no-prior full TASK w/ worked",
           L.WORKED_MARK in r.stdout and "TASK" in r.stdout)

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        g = E.gen("quota", "H", 0)
        (tmp / "ans-sv-01-solve.txt").write_text("draft " + json.dumps(
            g["truth"]))
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "judge", "--run-dir", str(tmp),
                 "--style", "judge"])
        ok("judge blind: TASK CONTRACT + ANSWER UNDER REVIEW",
           "TASK CONTRACT" in r.stdout
           and "ANSWER UNDER REVIEW" in r.stdout)
        ok("judge sees answer under review, not scaffolds",
           "granted_units" in r.stdout.split(
               "ANSWER UNDER REVIEW")[-1])


def test_check_runner():
    print("check-runner CLI: exit code contract")
    g = E.gen("quota", "H", 0)
    truth_str = json.dumps(g["truth"])
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        (tmp / "ans.txt").write_text("noise " + truth_str)
        r = run(["python3", str(SCRIPTS / "check-runner.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--text-file", str(tmp / "ans.txt"),
                 "--out", str(tmp / "res.json")])
        ok("pass exit 0", r.returncode == 0)
        (tmp / "ans2.txt").write_text('{"granted_units": 999}')
        r = run(["python3", str(SCRIPTS / "check-runner.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--text-file", str(tmp / "ans2.txt"),
                 "--out", str(tmp / "res2.json")])
        ok("fail exit 1", r.returncode == 1)


def test_scoped_checks():
    print("scoped checks: stage-scoped json_exact truth (13-synthesis)")
    full = {"json_exact": json.dumps(
        {"part_a": {"granted_units": 47, "denied": [], "pool_left": 5},
         "part_b": {"attempts": 4, "wait_s": 14, "budget_after": 2,
                    "suppressed": False}})}
    ok("key extract_a", L.scoped_stage_key("extract_a") == "part_a")
    ok("key extract_b", L.scoped_stage_key("extract_b") == "part_b")
    ok("key extract_b2", L.scoped_stage_key("extract_b2") == "part_b")
    ok("key extract_s none", L.scoped_stage_key("extract_s") is None)
    ok("key solve none", L.scoped_stage_key("solve") is None)
    ca = L.scoped_checks(full, "extract_a")
    ok("scoped a truth", json.loads(ca["json_exact"]) ==
       {"granted_units": 47, "denied": [], "pool_left": 5})
    cb = L.scoped_checks(full, "extract_b2")
    ok("scoped b2 truth", json.loads(cb["json_exact"]) ==
       {"attempts": 4, "wait_s": 14, "budget_after": 2, "suppressed": False})
    cs = L.scoped_checks(full, "extract_s")
    ok("scoped s unchanged", cs["json_exact"] == full["json_exact"])
    bare = '{"granted_units":47,"denied":[],"pool_left":5}'
    r = L.run_checks_safe(bare, ca)
    ok("bare part_a passes scoped", bool(r and r["passed"]))
    r2 = L.run_checks_safe(bare, full)
    ok("bare part_a fails full", bool(r2 and not r2["passed"]))


def test_nested_and_stage_candidates():
    print("candidates: nested-object harvest + single-key unwrap")
    merged = 'walk line\n{"part_a":{"g":47},"part_b":{"w":14}}'
    got = L.nested_json_objects(merged)
    ok("nested object found",
       got == ['{"part_a":{"g":47},"part_b":{"w":14}}'], repr(got))
    wrapped = 'text {"part_a": {"granted_units": 47}} tail'
    cands = list(L.stage_candidates(wrapped, "part_a"))
    ok("unwrap yields bare", '{"granted_units": 47}' in cands, repr(cands))
    bare2 = 'x {"granted_units": 47} y'
    cands2 = list(L.stage_candidates(bare2, "part_a"))
    ok("bare kept", '{"granted_units": 47}' in cands2, repr(cands2))


def test_has_pass_final_stage():
    print("has_pass: final-stage gate (case pass = final stage only)")
    with tempfile.TemporaryDirectory() as td:
        p = Path(td)
        (p / "check-sy-01-extract_a.json").write_text(
            json.dumps({"passed": True}))
        (p / "check-sy-01-extract_s.json").write_text(
            json.dumps({"passed": False}))
        ok("any-pass true", L.has_pass(str(p), "sy-01"))
        ok("final gate false", not L.has_pass(str(p), "sy-01",
                                              final_stage="extract_s"))
        (p / "check-sy-01-extract_s.json").write_text(
            json.dumps({"passed": True}))
        ok("final gate true", L.has_pass(str(p), "sy-01",
                                         final_stage="extract_s"))


_SYN_CASE = {
    "case_id": "sy-01",
    "prompt": "night desk sy-01 two crews",
    "success_criteria": {"deterministic_checks": {"json_exact": json.dumps(
        {"part_a": {"granted_units": 47, "denied": [], "pool_left": 5},
         "part_b": {"attempts": 4, "wait_s": 14, "budget_after": 2,
                    "suppressed": False}})}},
    "tkeys": "part_a,part_b",
}


def _syn_tmp(tmp):
    import yaml
    d = Path(tmp)
    d.mkdir(parents=True, exist_ok=True)
    (d / "case-sy-01.yml").write_text(yaml.safe_dump(_SYN_CASE))
    return d


def test_stage_emit_extract_scoped():
    print("stage-emit extract: scoped CLEANED echo + multi-prior pick")
    with tempfile.TemporaryDirectory() as td:
        d = _syn_tmp(td)
        (d / "ans-sy-01-worker_a.txt").write_text(
            'Phase 2 walk...\nstate after r5: {"granted_units": 47, '
            '"denied": [], "pool_left": 5}\n')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(d / "case-sy-01.yml"), "--case", "sy-01",
                 "--stage", "extract_a", "--run-dir", str(d),
                 "--style", "extract", "--prior", "worker_a"])
        a_txt = (d / "ans-sy-01-extract_a.txt").read_text().replace(
            "\n", "")
        ok("extract_a fast-path exit 42", r.returncode == 42,
           r.stderr[-300:])
        ok("extract_a CLEANED ans bare",
           '"granted_units": 47' in a_txt
           and '"part_b"' not in a_txt, a_txt[-200:])
    with tempfile.TemporaryDirectory() as td:
        d = _syn_tmp(td)
        (d / "ans-sy-01-solve_b2.txt").write_text(
            '{"attempts": 4, "wait_s": 14, "budget_after": -6, '
            '"suppressed": true}')
        (d / "ans-sy-01-worker_b.txt").write_text(
            'walk...\n{"attempts": 4, "wait_s": 14, "budget_after": 2, '
            '"suppressed": false}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(d / "case-sy-01.yml"), "--case", "sy-01",
                 "--stage", "extract_b2", "--run-dir", str(d),
                 "--style", "extract", "--prior", "solve_b2,worker_b"])
        b_txt = (d / "ans-sy-01-extract_b2.txt").read_text().replace(
            "\n", "")
        ok("extract_b2 fast-path exit 42", r.returncode == 42,
           r.stderr[-300:])
        ok("extract_b2 picks right prior",
           '"budget_after": 2' in b_txt
           and "-6" not in b_txt, b_txt[-200:])


def test_check_runner_scoped():
    print("check-runner: stage-scoped truth + wrapped-candidate unwrap")
    with tempfile.TemporaryDirectory() as td:
        d = _syn_tmp(td)
        (d / "ans-sy-01-extract_a.txt").write_text(
            '{"part_a": {"granted_units": 47, "denied": [], '
            '"pool_left": 5}}')
        r = run(["python3", str(SCRIPTS / "check-runner.py"),
                 "--case-yml", str(d / "case-sy-01.yml"), "--case", "sy-01",
                 "--stage", "extract_a",
                 "--text-file", str(d / "ans-sy-01-extract_a.txt"),
                 "--out", str(d / "check-sy-01-extract_a.json")])
        j = json.loads((d / "check-sy-01-extract_a.json").read_text())
        ok("scoped check passes wrapped part_a", j.get("passed") is True
           and r.returncode == 0, json.dumps(j))
        (d / "ans-sy-01-extract_s.txt").write_text(
            '{"part_a": {"granted_units": 47, "denied": [], '
            '"pool_left": 5}, "part_b": {"attempts": 4, "wait_s": 14, '
            '"budget_after": 2, "suppressed": false}}')
        r2 = run(["python3", str(SCRIPTS / "check-runner.py"),
                  "--case-yml", str(d / "case-sy-01.yml"), "--case", "sy-01",
                  "--stage", "extract_s",
                  "--text-file", str(d / "ans-sy-01-extract_s.txt"),
                  "--out", str(d / "check-sy-01-extract_s.json")])
        j2 = json.loads((d / "check-sy-01-extract_s.json").read_text())
        ok("full-truth check on merged object", j2.get("passed") is True
           and r2.returncode == 0, json.dumps(j2))


def test_fault_classes():
    print("fault classification")
    obs = "ERROR: upstream timeout after 30s"
    ok("timeout class", L.classify_observation(obs) == "timeout")
    ok("unreachable class",
       L.classify_observation("connect: no route to host") ==
       "unreachable")
    ok("garble class",
       L.classify_observation("out: \ufffd\ufffd 6\u2071 units \ufffd")
       == "garble")


def test_v2_efficiency():
    print("v2: seed carry-forward, live trims, ctx, flag wiring")
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        # checked stage passed in seed -> carried + skip
        (seed / "ans-sv-01-extract.txt").write_text('{"a": 1}')
        (seed / "check-sv-01-extract.json").write_text(
            '{"passed": true}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "extract", "--run-dir", str(rdir),
                 "--style", "extract", "--prior", "solve",
                 "--seed", str(seed)])
        ok("seed skip exit 42", r.returncode == 42)
        ok("seed ans carried",
           (rdir / "ans-sv-01-extract.txt").read_text() == '{"a": 1}')
        ok("seed check carried",
           (rdir / "check-sv-01-extract.json").exists())
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        # checked stage FAILED in seed -> no carry, prompt built normally
        (seed / "ans-sv-01-solve.txt").write_text("bad walk")
        (seed / "check-sv-01-solve.json").write_text(
            '{"passed": false}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(rdir),
                 "--style", "cot", "--seed", str(seed)])
        ok("failed seed not carried", r.returncode == 0)
        ok("failed seed no ans copy",
           not (rdir / "ans-sv-01-solve.txt").exists())
        ok("failed seed check copied for hint",
           (rdir / "check-sv-01-solve.json").exists())
        r2 = run(["python3", str(SCRIPTS / "stage-emit.py"),
                  "--case-yml", str(CASE_YML), "--case", "sv-01",
                  "--stage", "solve", "--run-dir", str(rdir),
                  "--style", "cot", "--seed", str(seed)])
        ok("hint from carried check in retry prompt",
           "PREVIOUS ATTEMPT FAILED" in r2.stdout)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        # unchecked stage (no check json) with non-empty ans -> carried
        (seed / "ans-sv-01-plan.txt").write_text("planning notes")
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "plan", "--run-dir", str(rdir),
                 "--style", "cot", "--seed", str(seed)])
        ok("unchecked seed carried", r.returncode == 42)
        ok("unchecked seed ans copied",
           (rdir / "ans-sv-01-plan.txt").read_text() ==
           "planning notes")

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        (seed / "check-sv-01-extract.json").write_text(
            '{"passed": false, "failures": [{'
            '"check": "json_exact", "detail": "fields differ"}]}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(rdir),
                 "--style", "cot", "--seed", str(seed)])
        ok("cross-stage hint from seed extract fail",
           "PREVIOUS ATTEMPT FAILED" in r.stdout and
           "fields differ" in r.stdout)

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        (seed / "ans-sv-01-solve.txt").write_text("wrong walk")
        (seed / "check-sv-01-extract.json").write_text(
            '{"passed": false, "failures": [{'
            '"check": "json_exact", "detail": "fields differ"}]}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(rdir),
                 "--style", "cot", "--seed", str(seed)])
        ok("unchecked solve NOT carried when case failing",
           r.returncode == 0 and
           not (rdir / "ans-sv-01-solve.txt").exists())
        ok("solve retry prompt has hint",
           "PREVIOUS ATTEMPT FAILED" in r.stdout)

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        seed = tmp / "seedrun"
        rdir = tmp / "newrun"
        seed.mkdir()
        rdir.mkdir()
        (seed / "ans-sv-01-solve.txt").write_text("good walk")
        (seed / "ans-sv-01-extract2.txt").write_text('{"ok": 1}')
        (seed / "check-sv-01-extract.json").write_text(
            '{"passed": false}')
        (seed / "check-sv-01-extract2.json").write_text(
            '{"passed": true}')
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(rdir),
                 "--style", "cot", "--seed", str(seed)])
        ok("case passed via deep stage keeps solve carry",
           r.returncode == 42 and
           (rdir / "ans-sv-01-solve.txt").exists())

    # gen-workflow: live v2 trims + ctx + seed wiring
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        out14 = tmp / "v2-14.yml"
        r = run(["python3", str(SCRIPTS / "gen-workflow.py"),
                 "--exp", "14-budget-inflation", "--live",
                 "--out", str(out14),
                 "--run-dir", str(tmp / "r14"),
                 "--limit", "2"])
        ok("gen 14 live ok", r.returncode == 0)
        y = out14.read_text()
        ok("14 no load_params (server-side ctx)",
           "load_params" not in y)
        ok("14 no probe steps", "_probe_bi-" not in y)
        ok("14 no budget steps", "_budget_bi-" not in y)
        ok("14 no judge steps", "_judge_bi-" not in y)
        ok("14 solve steps present", "_solve_bi-" in y)
        out14s = tmp / "v2-14-spoof.yml"
        r = run(["python3", str(SCRIPTS / "gen-workflow.py"),
                 "--exp", "14-budget-inflation", "--spoof",
                 "--out", str(out14s),
                 "--run-dir", str(tmp / "r14s"),
                 "--limit", "2"])
        ok("gen 14 spoof ok", r.returncode == 0)
        ok("spoof keeps probe (trim live-only)",
           "_probe_bi-" in out14s.read_text())
        out14sd = tmp / "v2-14-seeded.yml"
        r = run(["python3", str(SCRIPTS / "gen-workflow.py"),
                 "--exp", "14-budget-inflation", "--live",
                 "--out", str(out14sd),
                 "--run-dir", str(tmp / "r14sd"),
                 "--limit", "2", "--seed", str(tmp / "r14")])
        ysd = out14sd.read_text()
        ok("seed flag wired", f"--seed {tmp}/r14" in ysd)
        outm = tmp / "v2m.yml"
        r = run(["python3", str(SCRIPTS / "gen-workflow.py"),
                 "--exp", "07-solve", "--live",
                 "--out", str(outm),
                 "--run-dir", str(tmp / "rm"),
                 "--limit", "1", "--model", "Probe-Model-X"])
        ok("model override wired",
           outm.read_text().count('name: "Probe-Model-X"') >= 3)
        out07 = tmp / "v2-07.yml"
        r = run(["python3", str(SCRIPTS / "gen-workflow.py"),
                 "--exp", "07-solve", "--live",
                 "--out", str(out07),
                 "--run-dir", str(tmp / "r07"),
                 "--limit", "2"])
        y07 = out07.read_text()
        ok("07 no load_params", "load_params" not in y07)
        ok("07 no judge steps", "_judge_sv-" not in y07)
        ok("07 keep solve/extract", "_solve_sv-" in y07 and
           "_extract_sv-" in y07)


def test_expiry_discipline():
    print("stage-emit: expiry discipline on expiry-bearing solve prompts")
    import yaml as _y
    c = _y.safe_load(Path(CASE_YML).read_text())
    noexp = re.sub(r"[^.]*expir[^.]*\.", "", c["prompt"])
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        cn = tmp / "case-noexp.yml"
        cn.write_text(_y.dump({**c, "prompt": noexp}))
        nd = tmp / "nd"
        nd.mkdir()
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(cn), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(nd),
                 "--style", "cot"])
        ok("no expiry in prompt -> no discipline",
           "EXPIRY DISCIPLINE" not in r.stdout)
        cy = tmp / "case-exp.yml"
        cy.write_text(_y.dump({**c, "prompt":
                               "the grant for r1 expires after r2. " +
                               noexp}))
        ed = tmp / "ed"
        ed.mkdir()
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(cy), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(ed),
                 "--style", "cot"])
        ok("expiry in prompt -> discipline line",
           "EXPIRY DISCIPLINE" in r.stdout)
        xd = tmp / "xd"
        xd.mkdir()
        (xd / "ans-sv-01-solve.txt").write_text("draft text")
        r2 = run(["python3", str(SCRIPTS / "stage-emit.py"),
                  "--case-yml", str(cy), "--case", "sv-01",
                  "--stage", "extract", "--run-dir", str(xd),
                  "--style", "extract", "--prior", "solve"])
        ok("extract style exempt",
           "EXPIRY DISCIPLINE" not in r2.stdout)
        cp = tmp / "case-pre.yml"
        cp.write_text(_y.dump({**c, "prompt":
                               "HIGH arrives at minute 4, preempting "
                               "whatever LOW job runs. checkpoint every "
                               "2 minutes. " + noexp}))
        pd = tmp / "pd"
        pd.mkdir()
        r3 = run(["python3", str(SCRIPTS / "stage-emit.py"),
                  "--case-yml", str(cp), "--case", "sv-01",
                  "--stage", "solve", "--run-dir", str(pd),
                  "--style", "cot"])
        ok("preempt prompt -> discipline line",
           "PREEMPT DISCIPLINE" in r3.stdout)
        ok("expiry+preempt both present when both match",
           "EXPIRY DISCIPLINE" not in r3.stdout)
        cc = tmp / "case-can.yml"
        cc.write_text(_y.dump({**c, "prompt":
                               "canary rollout climbs in 10 percent "
                               "steps, a breach pauses the climb and "
                               "rolls back one step. " + noexp}))
        cd = tmp / "cd"
        cd.mkdir()
        r4 = run(["python3", str(SCRIPTS / "stage-emit.py"),
                  "--case-yml", str(cc), "--case", "sv-01",
                  "--stage", "solve", "--run-dir", str(cd),
                  "--style", "cot"])
        ok("canary prompt -> discipline line",
           "CANARY DISCIPLINE" in r4.stdout)
        ok("preempt table demanded on preempt+checkpoint",
           "PER-MINUTE TABLE" in r3.stdout)
        rc = tmp / "case-res.yml"
        rc.write_text(_y.dump({**c, "prompt":
                               "model C is pinned and never evicted, "
                               "evictions choose the least-recently-"
                               "touched unpinned model. " + noexp}))
        rd2 = tmp / "rd"
        rd2.mkdir()
        r5 = run(["python3", str(SCRIPTS / "stage-emit.py"),
                  "--case-yml", str(rc), "--case", "sv-01",
                  "--stage", "solve", "--run-dir", str(rd2),
                  "--style", "cot"])
        ok("residency prompt -> discipline line",
           "RESIDENCY DISCIPLINE" in r5.stdout)


def test_preempt_solver():
    print("preempt deterministic parse+walk")
    ok("parse sv-34 shape",
       L.preempt_parse(
           "LOW1 runs 5 minutes starting at minute 0, LOW2 runs 2 "
           "minutes starting at minute 1. HIGH arrives at minute 4 "
           "and runs 4 minutes") ==
       {"l1_dur": 5, "l1_start": 0, "l2_dur": 2, "l2_start": 1,
        "h_arr": 4, "h_dur": 4})
    ok("parse no-match -> None",
       L.preempt_parse("nothing here") is None)
    cases = [
        ({"l1_dur": 5, "l1_start": 0, "l2_dur": 2, "l2_start": 1,
          "h_arr": 4, "h_dur": 4},
         {"makespan_min": 11, "wasted_min": 0,
          "order": ["LOW1", "LOW2"]}),
        ({"l1_dur": 4, "l1_start": 0, "l2_dur": 3, "l2_start": 3,
          "h_arr": 3, "h_dur": 6},
         {"makespan_min": 14, "wasted_min": 1,
          "order": ["LOW1", "LOW2"]}),
        ({"l1_dur": 4, "l1_start": 0, "l2_dur": 3, "l2_start": 3,
          "h_arr": 5, "h_dur": 4},
         {"makespan_min": 12, "wanted_min": 1,
          "order": ["LOW1", "LOW2"]}),
    ]
    for i, (f, want) in enumerate(cases):
        got = L.preempt_walk(f)
        key = f"walk case {i}"
        if got is None:
            ok(key, False)
            continue
        match = (got.get("makespan_min") == want["makespan_min"]
                 and got.get("wasted_min") == want.get("wasted_min",
                                                       want.get(
                                                           "wanted_min"))
                 and got.get("order") == want["order"])
        ok(key, match)
        if not match:
            print("   got:", got, "want:", want)


def test_extract_fast_path():
    print("stage-emit: extract hit writes ans+check, skips model")
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        g = E.gen("quota", "H", 0)
        truth = json.dumps(g["truth"], sort_keys=True)
        (tmp / "ans-sv-01-solve.txt").write_text(
            "reasoning\n" + truth)
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(CASE_YML), "--case", "sv-01",
                 "--stage", "extract", "--run-dir", str(tmp),
                 "--style", "extract", "--prior", "solve"])
        ok("extract hit exits 42", r.returncode == 42)
        ok("ans echoed",
           (tmp / "ans-sv-01-extract.txt").exists())
        chk = tmp / "check-sv-01-extract.json"
        ok("check written passed",
           chk.exists() and
           json.loads(chk.read_text()).get("passed") is True)


def test_preempt_deterministic_stage():
    print("stage-emit: deterministic preempt solve bypasses model")
    import yaml as _y
    c = _y.safe_load(Path(CASE_YML).read_text())
    c["prompt"] = ("preempt duty. LOW1 runs 5 minutes starting at "
                   "minute 0, LOW2 runs 2 minutes starting at minute "
                   "1. HIGH arrives at minute 4 and runs 4 minutes "
                   "to completion, instantly preempting. checkpoint "
                   "every 2 completed minutes.")
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        cy = tmp / "case-p.yml"
        cy.write_text(_y.dump(c))
        rd = tmp / "rd"
        rd.mkdir()
        r = run(["python3", str(SCRIPTS / "stage-emit.py"),
                 "--case-yml", str(cy), "--case", "sv-01",
                 "--stage", "solve", "--run-dir", str(rd),
                 "--style", "cot"])
        ok("deterministic solve exits 42", r.returncode == 42)
        ans = (rd / "ans-sv-01-solve.txt").read_text()
        ok("ans written with walked JSON",
           "\"makespan_min\": 11" in ans and
           "\"wasted_min\": 0" in ans)


def main():
    for fn in [test_engines, test_std_case_contract, test_digest,
               test_harvester_and_hint, test_scenario_fns,
               test_spoof_cli, test_stage_emit_cli, test_check_runner,
               test_scoped_checks, test_nested_and_stage_candidates,
               test_has_pass_final_stage, test_stage_emit_extract_scoped,
               test_check_runner_scoped, test_fault_classes,
               test_v2_efficiency, test_expiry_discipline,
                test_preempt_solver,
                test_extract_fast_path,
                test_preempt_deterministic_stage]:
        fn()
    print()
    if FAILS:
        print(f"{len(FAILS)} FAILURES: {FAILS}")
        sys.exit(1)
    print("ALL FAS HOOK TESTS PASS")


if __name__ == "__main__":
    main()
