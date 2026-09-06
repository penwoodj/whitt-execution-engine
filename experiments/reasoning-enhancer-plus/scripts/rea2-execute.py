#!/usr/bin/env python3
"""REA+ v2 execution driver (implements rea2-v1-percase.yml and
rea2-v2-phasebatch.yml semantics through proven engine paths).

v1: per case — classify -> specialist -> verify -> escalate chain
    (fmt-fail -> AesCoder; else -> Hrtic; then -> Bonsai 2000).
    Cases grouped by primary model for load efficiency only.
v2: phase batches — fmt/pln/general/audit/logic+derive phases, one
    load each; same-model repair pass on failures; AesCoder fmt-repair
    phase; final Bonsai rescue for all still-failed.

Resume-safe: state JSON per mode; completed cases skipped.
Usage: rea2-execute.py --mode v1|v2 [--limit-min N]
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
REPO = EXP.parent
sys.path.insert(0, str(HERE))
import importlib.util as _ilu
_spec = _ilu.spec_from_file_location("classify_task", HERE / "classify-task.py")
_ct = _ilu.module_from_spec(_spec)
_spec.loader.exec_module(_ct)
CHANNELS = _ct.CHANNELS
classify = _ct.classify

WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
MODELS_DIR = "/home/jon/code/whitt-execution-engine/models"
API = "http://localhost:8080"

CAT_MODEL = {ch: spec["model"] for ch, spec in CHANNELS.items()}
FMT_I = CAT_MODEL["fmt"]
PLN_M = CAT_MODEL["pln"]
GENERAL = CAT_MODEL["general"]
AUD_M = CAT_MODEL["audit"]
DEEP = CAT_MODEL["logic"]
FMT_REPAIR = "AesCoder-4B.Q6_K"

TOK = {FMT_I: 250, PLN_M: 250, GENERAL: 250, AUD_M: 250,
       DEEP: 1000, FMT_REPAIR: 400}
RETRY_TOK = {GENERAL: 1000, DEEP: 2000, FMT_REPAIR: 400}

SUITES = [
    ("probe", EXP / "cases/probe"),
    ("band", EXP / "cases/stage2-train"),
    ("regression", EXP / "cases/regression"),
]


def http_json(path, timeout=10):
    try:
        with urllib.request.urlopen(f"{API}{path}", timeout=timeout) as r:
            return json.load(r)
    except Exception:
        return None


def post(path, body, timeout=240):
    req = urllib.request.Request(
        f"{API}{path}", data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return json.load(r)
    except Exception as e:
        return {"error": str(e)}


def sh(cmd, timeout=900, env=None):
    e = dict(os.environ)
    e["WHITT_MAX_CONCURRENT_INFERENCES"] = "1"
    if env:
        e.update(env)
    return subprocess.run(cmd, shell=True, capture_output=True,
                          text=True, timeout=timeout, env=e)


def loaded_models():
    d = http_json("/models", 5) or {}
    return [m["id"] for m in d.get("data", [])
            if m.get("status", {}).get("value") == "loaded"]


def server_guards():
    zombies = sh("docker exec whitt-llama-server ps aux 2>/dev/null | "
                 "grep -c '[l]lama-server'").stdout.strip()
    if zombies.isdigit() and int(zombies) > 2:
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True)
        time.sleep(45)
    if not http_json("/health", 5):
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True)
        time.sleep(45)
    free_mb = int(sh("free -m | awk '/Mem:/{print $7}'").stdout.strip() or 0)
    if free_mb < 3072:
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True)
        time.sleep(45)


def ensure_loaded(model):
    t0 = time.time()
    if model in loaded_models():
        return 0.0
    for m in loaded_models():
        post("/models/unload", {"model": m}, 30)
        time.sleep(3)
    for attempt in range(3):
        post("/models/load", {"model": model, "n_gpu_layers": 99})
        r = sh(f"python3 {HERE}/wait-loaded.py --model {model} --timeout 150",
               timeout=180)
        if r.stdout.strip().endswith("loaded"):
            return time.time() - t0
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True)
        time.sleep(45)
    raise RuntimeError(f"load failed: {model}")


def load_cases():
    cases = []
    for suite, d in SUITES:
        for p in sorted(d.glob("case-*.yml")):
            c = yaml.safe_load(p.read_text())
            cat, _ = classify(f"{c.get('prompt','')} {c.get('auxiliary','')}")
            cases.append({"suite": suite, "cid": c["case_id"],
                          "path": str(p), "cat": cat,
                          "model": CAT_MODEL[cat]})
    return cases


def augment_prompt(case_path, out_path, check_json):
    c = yaml.safe_load(Path(case_path).read_text())
    fails = []
    for f in (check_json or {}).get("failures", []):
        fails.append(f"{f.get('check')}: {f.get('detail')} "
                     f"[hint: {f.get('fix_hint')}]")
    note = ("\n\nPREVIOUS ATTEMPT FAILED these deterministic checks:\n- "
            + "\n- ".join(fails)
            + "\nRe-solve carefully and output ONLY the final answer "
              "in the exact required form.")
    c["prompt"] = (c.get("prompt") or "").strip() + note
    Path(out_path).write_text(yaml.safe_dump(c, sort_keys=False, width=78))


def run_batch(model, entries, run_dir, tok, augmented=None):
    """entries: list of case dicts. Returns {cid: check_json}."""
    run_dir.mkdir(parents=True, exist_ok=True)
    if augmented:
        td = Path(tempfile.mkdtemp(prefix="rea2-batch-"))
        for e in entries:
            if e["cid"] in augmented:
                augment_prompt(e["path"], td / f"case-{e['cid']}.yml",
                               augmented[e["cid"]])
            else:
                shutil.copy(e["path"], td / f"case-{e['cid']}.yml")
        cases_dir = td
    else:
        cases_dir = Path(tempfile.mkdtemp(prefix="rea2-batch-"))
        for e in entries:
            shutil.copy(e["path"], cases_dir / f"case-{e['cid']}.yml")
    wf = run_dir / f"wf-{model}-{int(time.time())}.yml"
    r = sh(f"python3 {HERE}/gen-probe-workflow.py --model {model} "
           f"--cases-dir {cases_dir} --max-tok {tok} "
           f"--out-dir {run_dir} --out {wf}")
    if not wf.exists():
        raise RuntimeError(f"gen failed: {r.stderr[-300:]}")
    r = sh(f"{WHITT} benchmark --workflow {wf} --output-dir {run_dir} "
           f"--models-dir {MODELS_DIR} --load-timeout 180 --prompts 1",
           timeout=1500)
    checks = {}
    for e in entries:
        cj = run_dir / f"check-{e['cid']}.json"
        if cj.exists():
            try:
                checks[e["cid"]] = json.loads(cj.read_text())
            except json.JSONDecodeError:
                checks[e["cid"]] = None
    return checks


def fresh_dir(run_dir, cids):
    for cid in cids:
        for f in run_dir.glob(f"*{cid}*"):
            f.unlink()


def mode_v1(cases, outroot, limit_min):
    state_p = outroot / "state.json"
    state = json.loads(state_p.read_text()) if state_p.exists() else {}
    t_start = time.time()
    order = sorted(cases, key=lambda c: (c["model"], c["suite"], c["cid"]))
    for c in order:
        if c["cid"] in state:
            continue
        if (time.time() - t_start) / 60 > limit_min:
            print("[v1] time limit — resume later"); break
        server_guards()
        rec = {"cat": c["cat"], "attempts": []}
        chain = [(c["model"], TOK[c["model"]], "primary")]
        if c["cat"] == "fmt":
            chain.append((FMT_REPAIR, RETRY_TOK[FMT_REPAIR], "fmt-repair"))
        chain.append((GENERAL, RETRY_TOK[GENERAL], "generalist"))
        chain.append((DEEP, RETRY_TOK[DEEP], "deep"))
        passed = None
        for model, tok, label in chain:
            if passed:
                break
            ensure_loaded(model)
            d = outroot / c["cid"] / label
            fresh_dir(d, [c["cid"]])
            aug = rec["attempts"][0][1] if rec["attempts"] else None
            checks = run_batch(model, [c], d, tok,
                               augmented={c["cid"]: aug} if aug else None)
            cj = checks.get(c["cid"])
            ok = bool(cj and cj.get("passed"))
            rec["attempts"].append([label, cj])
            print(f"[v1] {c['cid']:16} {label:10} {'PASS' if ok else 'fail'}")
            if ok:
                passed = label
        rec["passed"] = bool(passed)
        state[c["cid"]] = rec
        state_p.write_text(json.dumps(state, indent=1))
    done = sum(1 for c in cases if c["cid"] in state)
    print(f"[v1] {done}/{len(cases)} cases complete")
    return state


def mode_v2(cases, outroot, limit_min):
    state_p = outroot / "state.json"
    state = json.loads(state_p.read_text()) if state_p.exists() else {}
    t_start = time.time()
    phases = [("fmt", FMT_I, TOK[FMT_I]),
              ("pln", PLN_M, TOK[PLN_M]),
              ("general", GENERAL, TOK[GENERAL]),
              ("audit", AUD_M, TOK[AUD_M]),
              ("logic", DEEP, TOK[DEEP]),
              ("derive", DEEP, TOK[DEEP])]
    stage_recs = state.setdefault("phases", {})
    for cat, model, tok in phases:
        if cat in stage_recs and stage_recs[cat].get("done"):
            continue
        if (time.time() - t_start) / 60 > limit_min:
            print("[v2] time limit — resume later"); break
        entries = [c for c in cases if c["cat"] == cat]
        if not entries:
            stage_recs[cat] = {"done": True, "n": 0}; 
            state_p.write_text(json.dumps(state, indent=1)); continue
        server_guards()
        ensure_loaded(model)
        d = outroot / f"phase-{cat}"
        fresh_dir(d, [e["cid"] for e in entries])
        checks = run_batch(model, entries, d, tok)
        fails = {e["cid"]: cj for e in entries
                 if not (cj := checks.get(e["cid"])) or not cj.get("passed")}
        rec = {"n": len(entries), "pass1": len(entries) - len(fails),
               "repair": {}, "done": False}
        print(f"[v2] phase {cat:8} {len(entries)} cases "
              f"pass1={rec['pass1']}")
        if fails:
            checks2 = run_batch(model,
                                [c for c in entries if c["cid"] in fails],
                                d / "repair", tok + 250,
                                augmented=fails)
            for cid, cj in checks2.items():
                ok = bool(cj and cj.get("passed"))
                rec["repair"][cid] = ok
                if ok:
                    fails.pop(cid, None)
        rec["still_failed"] = sorted(fails)
        rec["done"] = True
        stage_recs[cat] = rec
        state_p.write_text(json.dumps(state, indent=1))
    if all(stage_recs.get(c, {}).get("done")
           for c, _, _ in phases) and not state.get("rescue_done"):
        still = {c: None for _, rec in stage_recs.items()
                 for c in rec.get("still_failed", [])}
        if still:
            server_guards()
            ensure_loaded(DEEP)
            entries = [c for c in cases if c["cid"] in still]
            prev = {}
            for cat, rec in stage_recs.items():
                for cid in rec.get("still_failed", []):
                    for f in (outroot / f"phase-{cat}").rglob(
                            f"check-{cid}.json"):
                        try:
                            prev[cid] = json.loads(f.read_text())
                        except Exception:
                            pass
            d = outroot / "rescue"
            fresh_dir(d, list(still))
            checks = run_batch(DEEP, entries, d, RETRY_TOK[DEEP],
                               augmented=prev)
            state["rescue"] = {cid: bool(cj and cj.get("passed"))
                               for cid, cj in checks.items()}
        state["rescue_done"] = True
        state_p.write_text(json.dumps(state, indent=1))
    print("[v2] phases:",
          {k: v.get("pass1", "-") for k, v in stage_recs.items()})
    return state


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", required=True, choices=["v1", "v2"])
    ap.add_argument("--limit-min", type=float, default=12.0)
    args = ap.parse_args()
    outroot = EXP / "results" / "rea2-runs" / args.mode
    outroot.mkdir(parents=True, exist_ok=True)
    cases = load_cases()
    router = {c["cid"]: (c["cat"], c["model"], c["suite"]) for c in cases}
    (outroot / "routing.json").write_text(json.dumps(
        {cid: {"cat": v[0], "model": v[1], "suite": v[2]}
         for cid, v in router.items()}, indent=1))
    print(f"[{args.mode}] {len(cases)} cases routed: "
          + str({m: sum(1 for c in cases if c['model'] == m)
                 for m in TOK}))
    if args.mode == "v1":
        mode_v1(cases, outroot, args.limit_min)
    else:
        mode_v2(cases, outroot, args.limit_min)


if __name__ == "__main__":
    main()
