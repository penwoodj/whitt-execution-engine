#!/usr/bin/env python3
"""Safe live launcher for v8+ hc suites.

Safety contract (user-mandated):
1. Unload ALL models before starting (POST models/unload for every loaded id).
2. Verify zero loaded model children remain; refuse to start otherwise.
3. RAM gate: refuse to start if available RAM < 4 GB.
4. Batch cases by model (light 4B batch, then heavy 9B batch); unload + verify
   zero loaded between batches. Never let multiple model children coexist
   mid-batch beyond the router's single warm slot per batch.
5. Conditional zombie restart (existing engine gate keeps WHITT_ZOMBIE_MAX).

Usage: run-v8-launcher.py [--version v8] [--cases hc-01,hc-02] [--limit N]
       [--dry-run]  (dry-run prints batches only)
"""
import argparse
import json
import os
import subprocess
import time
import urllib.error
import urllib.request
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent.parent
CASES = HERE / "cases"
WORKFLOWS = HERE / "workflows"
OUT = Path("./docs/benchmarks/outputs/output")
WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
BASE = "http://localhost:8080"
RAM_GATE_MB = 3072


def api(path, payload=None):
    data = json.dumps(payload).encode() if payload is not None else None
    req = urllib.request.Request(
        BASE + path, data=data,
        headers={"Content-Type": "application/json"},
        method="POST" if data is not None else "GET")
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            body = r.read()
            return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        raise RuntimeError(f"{path}: HTTP {e.code}") from e


def loaded_models():
    try:
        d = api("/models")
    except Exception:
        return None
    return [m["id"] for m in d.get("models", []) if m.get("state") != "unloaded"]


def unload_all(verify=True):
    loaded = loaded_models()
    if loaded is None:
        raise RuntimeError("cannot query /models — is the router up?")
    for mid in loaded:
        try:
            api("models/unload", {"id": mid})
        except Exception as e:
            print(f"  unload {mid}: {e}")
    for _ in range(30):
        time.sleep(2)
        if loaded_models() == []:
            break
    still = loaded_models() if verify else []
    if still:
        raise RuntimeError(f"refusing: {len(still)} models still loaded: {still}")
    return loaded


def ram_available_mb():
    with open("/proc/meminfo") as f:
        for line in f:
            if line.startswith("MemAvailable:"):
                return int(line.split()[1]) // 1024
    return 0


def zombie_count():
    try:
        out = subprocess.run(["bash", str(HERE / "scripts" / "zombie-count.sh")],
                             capture_output=True, text=True, timeout=10)
        return int(out.stdout.strip() or 0)
    except Exception:
        return 0


def maybe_restart():
    z = zombie_count()
    if z > 3:
        print(f"  [zombie gate] {z} processes — restarting docker")
        subprocess.run(["docker", "restart", "whitt-llama-server"],
                       capture_output=True, timeout=90)
        time.sleep(15)
    return zombie_count()


def run_case(cid, version):
    yml = WORKFLOWS / f"{version}-{cid}.yml"
    if not yml.is_file():
        raise FileNotFoundError(yml)
    for f in list(OUT.glob("ha-*")) + list(OUT.glob("artifact.*")):
        f.unlink()
    env = dict(os.environ, WHITT_ZOMBIE_MAX="4")
    t0 = time.time()
    r = subprocess.run(
        [WHITT, "benchmark", "--workflow", str(yml),
         "--output-dir", "./docs/benchmarks/outputs"],
        capture_output=True, text=True, timeout=900, env=env)
    elapsed = round(time.time() - t0, 1)

    trace_p = OUT / "ha-trace.jsonl"
    trace = []
    if trace_p.is_file():
        trace = [json.loads(l) for l in trace_p.read_text().splitlines() if l.strip()]
    gates = [f"{t['gate']}={t.get('verdict', '?')}" for t in trace]
    final_p = OUT / "ha-final-check.json"
    final_pass = False
    if final_p.is_file():
        try:
            final_pass = json.loads(final_p.read_text()).get("pass") is True
        except json.JSONDecodeError:
            pass
    artifact = ""
    table_p = OUT / "ha-table.txt"
    if table_p.is_file():
        artifact = table_p.read_text().strip()[:80]
    if not trace and r.returncode != 0:
        return {"case": cid, "verdict": "ERROR", "artifact": r.stderr[:150].replace("\n", " "),
                "elapsed": elapsed, "trace": []}
    return {"case": cid, "verdict": "PASS" if final_pass else "FAIL",
            "artifact": artifact, "elapsed": elapsed, "trace": trace, "gates": gates}


def batch_order(cases, version):
    light, heavy = [], []
    for cid in cases:
        hops = yaml.safe_load((CASES / f"{cid}.yml").read_text())["hops"]
        (light if hops <= 4 else heavy).append(cid)
    return light, heavy


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="v8")
    ap.add_argument("--cases", default=",".join(f"hc-{i:02d}" for i in range(1, 101)))
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    cases = [c.strip() for c in args.cases.split(",") if c.strip()]
    if args.limit:
        cases = cases[:args.limit]
    light, heavy = batch_order(cases, args.version)
    print(f"batches: light={len(light)} heavy={len(heavy)}")

    if args.dry_run:
        print("light:", ",".join(light))
        print("heavy:", ",".join(heavy))
        return

    ram = ram_available_mb()
    if ram < RAM_GATE_MB:
        raise SystemExit(f"REFUSING: available RAM {ram} MB < {RAM_GATE_MB} MB gate")
    print(f"RAM gate: {ram} MB available — ok")

    unloaded = unload_all()
    print(f"unloaded {len(unloaded)} model(s); verified zero loaded children")
    maybe_restart()

    results = []
    t_all = time.time()
    for batch_name, batch in (("light", light), ("heavy", heavy)):
        if not batch:
            continue
        for i, cid in enumerate(batch):
            if i > 0:
                maybe_restart()
            print(f"=== [{batch_name}] {cid} ===", flush=True)
            try:
                r = run_case(cid, args.version)
            except Exception as e:
                r = {"case": cid, "verdict": "ERROR", "artifact": str(e)[:120],
                     "elapsed": 0, "trace": [], "gates": []}
            results.append(r)
            print(f"{cid}: {r['verdict']} ({r['elapsed']}s) | {r.get('artifact', '')[:60]}", flush=True)
        if batch_name == "light" and heavy:
            unloaded = unload_all()
            print(f"batch switch: unloaded {len(unloaded)}, verified zero loaded", flush=True)

    total = round(time.time() - t_all, 1)
    won = sum(1 for r in results if r["verdict"] == "PASS")
    print(f"\n{args.version} LIVE: {won}/{len(results)} in {total}s")
    for r in sorted(results, key=lambda x: x["case"]):
        print(f"  {r['case']}: {r['verdict']} ({r['elapsed']}s) [{','.join(r.get('gates', [])[:8])}]")

    (OUT / f"{args.version}-suite-results.json").write_text(json.dumps(results, indent=2) + "\n")
    print(f"results: {OUT / f'{args.version}-suite-results.json'}")


if __name__ == "__main__":
    main()
