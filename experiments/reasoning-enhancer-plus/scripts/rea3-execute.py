#!/usr/bin/env python3
"""REA+ v3/v4/v5 execution driver — phased primaries + batched rescue.

Versions (config dicts below):
  v3: all-pass attempt. Classifier v2 (env-math split), stepwise math
      prompts, Thinking in rescue chains, batched rescue passes.
  v4: efficiency. v3 routing + cheap-screen phase (1.7B first on
      fmt/pln/general), known-dead skip, prompt dedup, short fmt chain.
  v5: 3 models only (4B-Instruct, Bonsai, Thinking), probe 20/20 target.

Resume-safe: state.json per version dir. Usage:
  rea3-execute.py --version v3 [--limit-min N]
"""
import argparse
import hashlib
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

WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
MODELS_DIR = "/home/jon/code/whitt-execution-engine/models"
API = "http://localhost:8080"

M_FMT = "Qwen3-4B-Instruct-2507-Q4_K_M"
M_PLN = "gemma-3n-E4B-it-Q4_K_M"
M_GEN = "Qwen3-4B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q4_K_M-imat"
M_AUD = "nvidia_Orchestrator-8B-Q5_K_L"
M_DEEP = "Bonsai-27B-Q1_0"
M_THINK = "Qwen3-4B-Thinking-2507-Q4_K_M"
M_AESC = "AesCoder-4B.Q6_K"
M_SMALL = "Qwen3-1.7B-abliterated-q8_0"

STEPWISE = ("Solve methodically: (1) identify every quantity that must "
            "be computed and the formula for each; (2) compute them one "
            "by one showing each result; (3) double-check each against "
            "the question's units and wording; (4) then output ONLY the "
            "final answer in the exact required format — no reasoning "
            "text in the final line.\n\nTASK:\n")

CHANNELS_V3 = {
    "fmt": (M_FMT, 250),
    "pln": (M_PLN, 250),
    "logic": (M_DEEP, 2000),
    "derive": (M_DEEP, 2000),
    "audit": (M_AUD, 250),
    "general": (M_GEN, 250),
}

CHAINS_V3 = {
    "fmt": [(M_AESC, 400, False), (M_DEEP, 2000, True), (M_THINK, 3500, True)],
    "pln": [(M_DEEP, 2000, True), (M_THINK, 3500, True)],
    "logic": [(M_THINK, 3500, True), (M_GEN, 1000, False)],
    "derive": [(M_THINK, 3500, True), (M_GEN, 1000, False)],
    "audit": [(M_THINK, 3500, True), (M_DEEP, 2000, True)],
    "general": [(M_DEEP, 2000, True), (M_THINK, 3500, True)],
}

CHAINS_V5 = {
    "fmt": [(M_DEEP, 2000, True)],
    "pln": [(M_DEEP, 2000, True)],
    "logic": [(M_THINK, 3500, True)],
    "derive": [(M_THINK, 3500, True)],
    "audit": [(M_THINK, 3500, True)],
    "general": [(M_DEEP, 2000, True)],
}

VERSIONS = {
    "v3": {"channels": CHANNELS_V3, "chains": CHAINS_V3,
           "screen": None, "dead_skip": False, "dedup": False},
    "v3.1": {"channels": CHANNELS_V3, "chains": CHAINS_V3,
             "screen": None, "dead_skip": False, "dedup": False,
             "extract": True, "decompose_redo": True},
    "v4": {"channels": CHANNELS_V3, "chains": CHAINS_V3,
           "screen": (M_SMALL, 250, ("fmt", "pln", "general", "logic")),
           "dead_skip": True, "dedup": True, "extract": True},
    "v5": {"channels": dict(CHANNELS_V3, fmt=(M_FMT, 250), pln=(M_FMT, 250),
                            logic=(M_DEEP, 2000), audit=(M_DEEP, 2000),
                            general=(M_FMT, 250)),
           "chains": CHAINS_V5, "screen": None, "dead_skip": False,
           "dedup": False, "extract": True},
    "v6": {"channels": dict(CHANNELS_V3, logic=(M_DEEP, 1200),
                            derive=(M_DEEP, 1200)),
           "chains": {
               "fmt": [[M_AESC, 400, False], [M_THINK, 3500, True],
                    [M_DEEP, 1500, True]],
               "pln": [[M_DEEP, 1500, True], [M_THINK, 3500, True]],
               "general": [[M_DEEP, 1500, True], [M_THINK, 3500, True]],
               "logic": [[M_THINK, 3500, True], [M_DEEP, 1500, True]],
               "derive": [[M_THINK, 3500, True], [M_DEEP, 1500, True]],
               "audit": [[M_THINK, 3500, True]]},
           "screen": (M_SMALL, 250, ("fmt", "pln", "general", "logic",
                                     "derive", "audit")),
           "dead_skip": True, "dedup": True,
           "extract": True, "extract_first": True},
}

DEAD_DEFAULT = ["s2-env-11", "s2-env-13"]

SUITES = [("probe", EXP / "cases/probe"),
          ("band", EXP / "cases/stage2-train"),
          ("regression", EXP / "cases/regression")]


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


def sh(cmd, timeout=900):
    e = dict(os.environ)
    e["WHITT_MAX_CONCURRENT_INFERENCES"] = "1"
    return subprocess.run(cmd, shell=True, capture_output=True,
                          text=True, timeout=timeout, env=e)


def loaded_models():
    d = http_json("/models", 5) or {}
    return [m["id"] for m in d.get("data", [])
            if m.get("status", {}).get("value") == "loaded"]


def server_guards():
    z = sh("docker exec whitt-llama-server ps aux 2>/dev/null | "
           "grep -c '[l]lama-server'").stdout.strip()
    if z.isdigit() and int(z) > 2:
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True); time.sleep(45)
    if not http_json("/health", 5):
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True); time.sleep(45)
    free_mb = int(sh("free -m | awk '/Mem:/{print $7}'").stdout.strip() or 0)
    if free_mb < 3072:
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True); time.sleep(45)


def ensure_loaded(model):
    if model in loaded_models():
        return
    for m in loaded_models():
        post("/models/unload", {"model": m}, 30); time.sleep(3)
    for _ in range(3):
        post("/models/load", {"model": model, "n_gpu_layers": 99})
        r = sh(f"python3 {HERE}/wait-loaded.py --model {model} "
               f"--timeout 150", timeout=180)
        if r.stdout.strip().endswith("loaded"):
            return
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True); time.sleep(45)
    raise RuntimeError(f"load failed: {model}")


ENV_MATH = ("compute", "total_pages", "per_page", "next_page", "chunks",
            "records", "window", "quota", "remaining", "last_page",
            "ceil", "rate", "used", "progress", "evict", "backoff",
            "delay", "wait", "completion", "makespan", "discount",
            "saved", "cut", "held", "available", "gb", "in dollars",
            "profit", "refund", "fee", "revenue", "percent of", "how many",
            "average", "points per", "yards", "cubic", "final price")
FMT_ONLY = ("bullet", "all caps", "verbatim", "nothing else",
            "single line", "markdown table", "output only this json",
            "exactly these keys", "listen:", "yaml", "emit only",
            "reversed", "letters", "words,")
LOGIC_KW = ("policy", "escalat", "on-call", "grant", "deny", "verdict",
            "first match wins", "iff", "clause", "gate:", "rollback",
            "ship if", "rules:", "except", "priority lanes", "drains")


def classify_v2(text):
    t = text.lower()
    if any(k in t for k in LOGIC_KW) and "json" not in t[:120]:
        return "logic"
    if any(k in t for k in ENV_MATH):
        return "derive"
    if any(k in t for k in FMT_ONLY):
        return "fmt"
    if any(k in t for k in ("audit", "review", "spot the", "verify",
                            "inconsistenc", "orchestrat", "delegate")):
        return "audit"
    if any(k in t for k in ("schedule", "queue", "plan", "allocate",
                            "worker", "priority")):
        return "pln"
    return "general"


def load_cases(dedup_cache=None):
    cases = []
    for suite, d in SUITES:
        for p in sorted(d.glob("case-*.yml")):
            c = yaml.safe_load(p.read_text())
            body = f"{c.get('prompt','')} {c.get('auxiliary','')}"
            cat = classify_v2(body)
            entry = {"suite": suite, "cid": c["case_id"], "path": str(p),
                     "cat": cat}
            if dedup_cache is not None:
                h = hashlib.sha1(body.encode()).hexdigest()[:12]
                entry["dupe_of"] = dedup_cache.get(h)
                dedup_cache[h] = c["case_id"]
            cases.append(entry)
    return cases


def build_batch_dir(entries, out_dir, stepwise=False, last_fail=None):
    out_dir.mkdir(parents=True, exist_ok=True)
    for e in entries:
        c = yaml.safe_load(Path(e["path"]).read_text())
        p = (c.get("prompt") or "").strip()
        if stepwise:
            p = STEPWISE + p
        if last_fail and e["cid"] in last_fail:
            fails = [f"{f.get('check')}: {f.get('detail')}"
                     for f in (last_fail[e["cid"]] or {}).get("failures", [])]
            p += ("\n\nPREVIOUS ATTEMPT FAILED these checks:\n- "
                  + "\n- ".join(fails)
                  + "\nCorrect answer must pass every check. Re-solve "
                    "carefully; output ONLY the final answer.")
        c["prompt"] = p
        (out_dir / f"case-{e['cid']}.yml").write_text(
            yaml.safe_dump(c, sort_keys=False, width=78))


def run_batch(model, entries, run_dir, tok, stepwise=False, last_fail=None):
    td = Path(tempfile.mkdtemp(prefix="rea3-"))
    build_batch_dir(entries, td, stepwise, last_fail)
    run_dir.mkdir(parents=True, exist_ok=True)
    for e in entries:
        for f in run_dir.glob(f"*{e['cid']}*"):
            f.unlink()
    wf = run_dir / f"wf-{model.replace('/', '_')}-{int(time.time())}.yml"
    r = sh(f"python3 {HERE}/gen-probe-workflow.py --model {model} "
           f"--cases-dir {td} --max-tok {tok} --out-dir {run_dir} "
           f"--out {wf}")
    if not wf.exists():
        raise RuntimeError(f"gen failed: {r.stderr[-300:]}")
    sh(f"{WHITT} benchmark --workflow {wf} --output-dir {run_dir} "
       f"--models-dir {MODELS_DIR} --load-timeout 180 --prompts 1",
       timeout=2400)
    checks = {}
    for e in entries:
        cj = run_dir / f"check-{e['cid']}.json"
        try:
            checks[e["cid"]] = json.loads(cj.read_text())
        except Exception:
            checks[e["cid"]] = None
    return checks


def run_version(ver, cases, outroot, limit_min, dead):
    cfg = VERSIONS[ver]
    state_p = outroot / "state.json"
    state = json.loads(state_p.read_text()) if state_p.exists() else {}
    recs = state.setdefault("cases", {})
    t0 = time.time()
    chains = cfg["chains"]

    def remaining_minutes():
        return limit_min - (time.time() - t0) / 60

    def extract_pass(flag):
        todo = [c for c in cases if c["cid"] in recs
                and not recs[c["cid"]].get("passed")]
        ex_entries = []
        for c in todo:
            ans = None
            for sub in ("rescue-d2", "rescue-d1", "rescue-d0", "primary"):
                f = outroot / sub / f"ans-{c['cid']}.txt"
                if f.exists() and f.stat().st_size > 0:
                    ans = f.read_text()[:1500]
                    break
            if not ans:
                continue
            orig = yaml.safe_load(Path(c["path"]).read_text())
            checks = (orig.get("success_criteria") or {}).get(
                "deterministic_checks") or {}
            req = checks.get("contains_required") or []
            ex_entries.append({
                "cid": c["cid"], "cat": c["cat"], "path": c["path"],
                "extract_text": ans, "required": req})
        if ex_entries:
            server_guards(); ensure_loaded(M_FMT)
            td = Path(tempfile.mkdtemp(prefix="rea3-ex-"))
            keep = []
            for e in ex_entries:
                orig = yaml.safe_load(Path(e["path"]).read_text())
                hint = ""
                if e["required"]:
                    hint = (" The answer must contain: "
                            + ", ".join(f"'{r}'" for r in e["required"])
                            + " — take it verbatim from the text if "
                              "present.")
                orig["prompt"] = (
                    "From the solution text below, output ONLY the final "
                    "answer, stripped of all reasoning, labels, and "
                    "commentary." + hint + " No explanation.\n\n"
                    "SOLUTION TEXT:\n" + e["extract_text"])
                orig["success_criteria"] = {"deterministic_checks": {
                    k: v for k, v in (
                        (yaml.safe_load(Path(e["path"]).read_text())
                         .get("success_criteria") or {}
                         ).get("deterministic_checks", {}).items())}}
                (td / f"case-{e['cid']}.yml").write_text(
                    yaml.safe_dump(orig, sort_keys=False, width=78))
                keep.append(e)
            d = outroot / "extract"
            wf = d / f"wf-extract-{int(time.time())}.yml"
            d.mkdir(parents=True, exist_ok=True)
            r = sh(f"python3 {HERE}/gen-probe-workflow.py --model {M_FMT} "
                   f"--cases-dir {td} --max-tok 150 --out-dir {d} --out {wf}")
            sh(f"{WHITT} benchmark --workflow {wf} --output-dir {d} "
               f"--models-dir {MODELS_DIR} --load-timeout 180 "
               f"--prompts 1", timeout=1800)
            for e in keep:
                cj = d / f"check-{e['cid']}.json"
                try:
                    ok = bool(json.loads(cj.read_text()).get("passed"))
                except Exception:
                    ok = False
                recs[e["cid"]]["attempts"].append(["extract", ok])
                if ok:
                    recs[e["cid"]]["passed"] = True
            print(f"[{ver}] extract pass: "
                  f"{sum(1 for e in keep if recs[e['cid']]['passed'])}"
                  f"/{len(keep)}")
        state[flag] = True
        state_p.write_text(json.dumps(state, indent=1))


    screen_pass = {}
    if cfg["screen"] and not state.get("screen_done"):
        s_model, s_tok, s_cats = cfg["screen"]
        entries = [c for c in cases if c["cat"] in s_cats
                   and c["cid"] not in recs]
        if entries:
            server_guards(); ensure_loaded(s_model)
            checks = run_batch(s_model, entries, outroot / "screen", s_tok)
            for e in entries:
                cj = checks.get(e["cid"])
                ok = bool(cj and cj.get("passed"))
                screen_pass[e["cid"]] = ok
                recs[e["cid"]] = {"cat": e["cat"], "attempts": [["screen", ok]],
                                  "passed": ok}
            print(f"[{ver}] screen {s_model}: "
                  f"{sum(screen_pass.values())}/{len(entries)} pass")
        state["screen_done"] = True
        state_p.write_text(json.dumps(state, indent=1))

    if not state.get("primary_done"):
        by_model = {}
        for c in cases:
            r = recs.get(c["cid"])
            if r and (r.get("passed")
                      or any(a[0].startswith("primary:")
                             for a in r.get("attempts", []))):
                continue
            m, tok = cfg["channels"][c["cat"]]
            by_model.setdefault((m, tok, c["cat"] == "derive"
                                 or c["cat"] == "logic"), []).append(c)
        for (m, tok, stepwise), entries in by_model.items():
            if remaining_minutes() < 2:
                print(f"[{ver}] primary partial — resume later")
                state_p.write_text(json.dumps(state, indent=1))
                return state
            entries = [e for e in entries
                       if not (recs.get(e["cid"], {}).get("passed")
                               or any(a[0].startswith("primary:")
                                      for a in recs.get(
                                          e["cid"], {}).get("attempts", [])))]
            if not entries:
                continue
            server_guards(); ensure_loaded(m)
            checks = run_batch(m, entries, outroot / "primary", tok,
                               stepwise=stepwise)
            for e in entries:
                cj = checks.get(e["cid"])
                ok = bool(cj and cj.get("passed"))
                rec = recs.setdefault(e["cid"], {"cat": e["cat"],
                                                 "attempts": []})
                rec["attempts"].append([f"primary:{m[:12]}", ok])
                rec["passed"] = ok
        state["primary_done"] = True
        state_p.write_text(json.dumps(state, indent=1))
        print(f"[{ver}] primaries done: "
              f"{sum(r.get('passed', False) for r in recs.values())} passing")

        if (cfg.get("extract") and cfg.get("extract_first")
                and not state.get("extract1_done")):
            extract_pass("extract1_done")

    max_depth = max(len(c) for c in chains.values())
    for depth in range(max_depth):
        if remaining_minutes() < 2:
            print(f"[{ver}] rescue partial — resume later")
            break
        todo = [c for c in cases
                if c["cid"] in recs and not recs[c["cid"]].get("passed")
                and c["cat"] in chains
                and depth < len(chains[c["cat"]])
                and not (cfg["dead_skip"] and c["cid"] in dead)
                and not any(a[0].startswith(f"d{depth}:")
                            for a in recs[c["cid"]]["attempts"])]
        if not todo:
            continue
        by_model = {}
        for c in todo:
            m, tok, sw = chains[c["cat"]][depth]
            by_model.setdefault((m, tok, sw), []).append(c)
        for (m, tok, sw), entries in sorted(by_model.items()):
            if remaining_minutes() < 1.5:
                break
            server_guards(); ensure_loaded(m)
            d = outroot / f"rescue-d{depth}"
            last_fail = {}
            for e in entries:
                for f in d.rglob(f"check-{e['cid']}.json"):
                    try:
                        last_fail[e["cid"]] = json.loads(f.read_text())
                    except Exception:
                        pass
                if e["cid"] not in last_fail:
                    for f in (outroot / "primary").rglob(
                            f"check-{e['cid']}.json"):
                        try:
                            last_fail[e["cid"]] = json.loads(f.read_text())
                        except Exception:
                            pass
            checks = run_batch(m, entries, d, tok, stepwise=sw,
                               last_fail=last_fail)
            for e in entries:
                cj = checks.get(e["cid"])
                ok = bool(cj and cj.get("passed"))
                recs[e["cid"]]["attempts"].append(
                    [f"d{depth}:{m[:12]}", ok])
                if ok:
                    recs[e["cid"]]["passed"] = True
            state_p.write_text(json.dumps(state, indent=1))
            print(f"[{ver}] rescue d{depth} {m[:14]:14} "
                  f"{sum(bool(cj and cj.get('passed')) for cj in checks.values())}"
                  f"/{len(entries)}")

    if cfg.get("extract") and not state.get("extract_done"):
        extract_pass("extract_done")

    if cfg.get("decompose_redo") and not state.get("decompose_done"):
        todo = [c for c in cases if c["cid"] in recs
                and not recs[c["cid"]].get("passed")]
        if todo:
            server_guards(); ensure_loaded(M_DEEP)
            td = Path(tempfile.mkdtemp(prefix="rea3-dc-"))
            for c in todo:
                orig = yaml.safe_load(Path(c["path"]).read_text())
                orig["prompt"] = (
                    "DECOMPOSE then SOLVE. Step A: list every distinct "
                    "sub-quantity with its formula and units. Step B: "
                    "compute each sub-quantity, one per line, checking "
                    "arithmetic. Step C: combine per the question. "
                    "Final line must be exactly 'FINAL: <answer>' with "
                    "the answer in the required format and nothing "
                    "after it.\n\nTASK:\n"
                    + (orig.get("prompt") or "").strip())
                (td / f"case-{c['cid']}.yml").write_text(
                    yaml.safe_dump(orig, sort_keys=False, width=78))
            d = outroot / "decompose"
            wf = d / f"wf-dc-{int(time.time())}.yml"
            d.mkdir(parents=True, exist_ok=True)
            sh(f"python3 {HERE}/gen-probe-workflow.py --model {M_DEEP} "
               f"--cases-dir {td} --max-tok 2500 --out-dir {d} --out {wf}")
            sh(f"{WHITT} benchmark --workflow {wf} --output-dir {d} "
               f"--models-dir {MODELS_DIR} --load-timeout 180 "
               f"--prompts 1", timeout=2400)
            for c in todo:
                cj = d / f"check-{c['cid']}.json"
                try:
                    ok = bool(json.loads(cj.read_text()).get("passed"))
                except Exception:
                    ok = False
                recs[c["cid"]]["attempts"].append(["decompose", ok])
                if ok:
                    recs[c["cid"]]["passed"] = True
            print(f"[{ver}] decompose redo: "
                  f"{sum(1 for c in todo if recs[c['cid']]['passed'])}"
                  f"/{len(todo)}")
        state["decompose_done"] = True
        state_p.write_text(json.dumps(state, indent=1))

    if cfg["dedup"]:
        for c in cases:
            if c.get("dupe_of") and c["dupe_of"] in recs:
                recs[c["cid"]] = dict(recs[c["dupe_of"]])
        state_p.write_text(json.dumps(state, indent=1))

    state_p.write_text(json.dumps(state, indent=1))
    passed = sum(r.get("passed", False) for r in recs.values()
                 if r.get("cat"))
    print(f"[{ver}] {passed}/{len(cases)} passing")
    return state


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", required=True, choices=list(VERSIONS))
    ap.add_argument("--limit-min", type=float, default=13.0)
    args = ap.parse_args()
    outroot = EXP / "results" / "rea3-runs" / args.version
    outroot.mkdir(parents=True, exist_ok=True)
    cases = load_cases(dedup_cache={} if VERSIONS[args.version]["dedup"]
                       else None)
    from collections import Counter
    print(f"[{args.version}] {len(cases)} cases: "
          f"{dict(Counter(c['cat'] for c in cases))}")
    dead = DEAD_DEFAULT if VERSIONS[args.version]["dead_skip"] else []
    run_version(args.version, cases, outroot, args.limit_min, dead)


if __name__ == "__main__":
    main()
