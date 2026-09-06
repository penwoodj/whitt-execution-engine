#!/usr/bin/env python3
"""REA benchmark driver — orchestrates the 50-case 9B-vs-workflow benchmark.

Usage:
  run-benchmark.py baseline [case-glob ...]   # 9B direct answers (must FAIL checks)
  run-benchmark.py enhance  [case-glob ...]   # inject baseline as draft, run enhancer
  run-benchmark.py report                     # aggregate evidence table

baseline: runs workflows/baseline-9b.yml per case via run-enhancer.sh
(REA_WORKFLOW override), verifies check-baseline.json shows FAILURE, stores
the answer as the case's benchmark draft under results/benchmark/baseline/.

enhance: writes a runtime case copy whose draft_response is the stored 9B
baseline answer, runs the enhancer workflow, stores select-best summary.

Both phases are resumable: completed cases are skipped. Machine-safety
remains with run-enhancer.sh (preflight, watchdog, cooldown, timeout).
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

import yaml

REA = Path(__file__).parent.parent
SCRIPTS = REA / "scripts"
RUNNER = SCRIPTS / "run-enhancer.sh"
CASES = REA / "cases" / "benchmark"
BENCH = REA / "results" / "benchmark"


def sh(cmd, env_extra=None):
    env = None
    if env_extra:
        env = dict(__import__("os").environ)
        env.update(env_extra)
    return subprocess.run(cmd, capture_output=True, text=True, env=env)


def case_files(patterns):
    if not patterns:
        return sorted(CASES.glob("case-b*.yml"))
    out = []
    for p in patterns:
        out += sorted(CASES.glob(p if p.startswith("case-") else f"case-{p}.yml"))
    return sorted(set(out))


def settle(max_wait=180):
    """9B load/unload evicts gigabytes; wait out the post-run swap storm
    so the next preflight doesn't abort on transient activity."""
    waited = 0
    while waited < max_wait:
        r = subprocess.run(["vmstat", "1", "8"], capture_output=True, text=True)
        total = 0
        for ln in r.stdout.splitlines()[2:]:
            parts = ln.split()
            if len(parts) >= 8:
                try:
                    total += int(parts[6]) + int(parts[7])
                except ValueError:
                    pass
        if total < 20000:
            return waited
        time.sleep(10)
        waited += 10
    return waited


def run_runner(case_path, workflow_env=None, keep_loaded=None, timeout=300):
    env = {"REA_WORKFLOW": str(workflow_env)} if workflow_env else {}
    if keep_loaded:
        env["REA_KEEP_LOADED"] = keep_loaded
    r = sh(["bash", str(RUNNER), str(case_path), str(timeout)], env_extra=env or None)
    outdir = None
    for ln in r.stdout.splitlines():
        if ln.startswith("[rea] output="):
            outdir = ln.split("=", 1)[1].strip()
    return r.returncode, outdir, r.stdout


def run_with_retry(case_path, workflow_env=None, keep_loaded=None):
    """One attempt; if preflight aborts on the post-load swap storm,
    settle and retry once. Returns (rc, outdir, stdout)."""
    code, outdir, stdout = run_runner(case_path, workflow_env, keep_loaded)
    if not outdir and "PREFLIGHT FAILED" in stdout:
        w = settle()
        print(f"[bm] settled {w}s after preflight abort, retrying once", flush=True)
        code, outdir, stdout = run_runner(case_path, workflow_env, keep_loaded)
    return code, outdir, stdout


def phase_baseline(files):
    BENCH.mkdir(parents=True, exist_ok=True)
    for cf in files:
        case = yaml.safe_load(cf.read_text())
        cid = case["case_id"]
        marker = BENCH / f"baseline-{cid}.json"
        if marker.exists():
            print(f"[bm] {cid}: baseline done (cached)")
            continue
        print(f"[bm] {cid}: running 9B baseline...", flush=True)
        code, outdir, stdout = run_with_retry(cf, workflow_env=REA / "workflows" / "baseline-9b.yml",
                                              keep_loaded="Qwen3-5-9B-Q4_K_M")
        if not outdir:
            print(f"[bm] {cid}: NO OUTPUT DIR (abort?) — rc={code}")
            print(stdout[-500:])
            return 1
        ans_p = Path(outdir) / "baseline-answer.txt"
        chk_p = Path(outdir) / "check-baseline.json"
        if not ans_p.exists() or not chk_p.exists():
            print(f"[bm] {cid}: baseline artifacts missing in {outdir}")
            return 1
        chk = json.loads(chk_p.read_text())
        marker.write_text(json.dumps({
            "case_id": cid,
            "run_dir": str(outdir),
            "answer": ans_p.read_text(),
            "passed": chk["passed"],
            "subchecks_passed": chk["subchecks_passed"],
            "subchecks_total": chk["subchecks_total"],
            "failures": chk["failures"],
        }, indent=2))
        nfail = chk["subchecks_total"] - chk["subchecks_passed"]
        verdict = "FAIL (good)" if not chk["passed"] else "PASS (NEEDS STRENGTHENING)"
        print(f"[bm] {cid}: baseline {verdict} — {nfail} subcheck(s) failing")
        settle()


def phase_enhance(files):
    BENCH.mkdir(parents=True, exist_ok=True)
    for cf in files:
        case = yaml.safe_load(cf.read_text())
        cid = case["case_id"]
        marker = BENCH / f"enhance-{cid}.json"
        bl = BENCH / f"baseline-{cid}.json"
        if marker.exists():
            print(f"[bm] {cid}: enhance done (cached)")
            continue
        if not bl.exists():
            print(f"[bm] {cid}: no baseline yet — run baseline phase first")
            return 1
        baseline = json.loads(bl.read_text())
        runtime_case = BENCH / f"runtime-{cid}.yml"
        case["draft_response"] = baseline["answer"]
        runtime_case.write_text(yaml.safe_dump(case, sort_keys=False, width=100))
        print(f"[bm] {cid}: running enhancer on 9B draft...", flush=True)
        code, outdir, stdout = run_with_retry(runtime_case)
        if not outdir:
            print(f"[bm] {cid}: NO OUTPUT DIR (abort?) — rc={code}")
            print(stdout[-500:])
            return 1
        sb_p = Path(outdir) / "select-best.json"
        if not sb_p.exists():
            print(f"[bm] {cid}: select-best.json missing in {outdir}")
            return 1
        sb = json.loads(sb_p.read_text())
        marker.write_text(json.dumps({
            "case_id": cid,
            "run_dir": str(outdir),
            "mode": sb["mode"],
            "selected": sb.get("selected"),
            "passed": sb.get("passed"),
        }, indent=2))
        verdict = "PASS" if sb.get("passed") else "FAIL (WORKFLOW MISS)"
        print(f"[bm] {cid}: enhance {verdict} — mode={sb['mode']}")


def phase_report():
    rows = []
    for bl in sorted(BENCH.glob("baseline-rea-*.json")):
        cid = bl.stem.replace("baseline-", "")
        b = json.loads(bl.read_text())
        en = BENCH / f"enhance-{cid}.json"
        e = json.loads(en.read_text()) if en.exists() else None
        eb = BENCH / f"enhance-batch-{cid}.json"
        ebatch = json.loads(eb.read_text()) if eb.exists() else None
        rows.append((cid, b, e, ebatch))
    if not rows:
        print("no results")
        return 1
    have_batch = any(r[3] for r in rows)
    bhead = f" {'batch':>5}" if have_batch else ""
    print(f"{'case':9} {'9B alone':>9} {'wf':>5}{bhead} {'mode':16} {'fails(bl)':>9}")
    bl_fail = en_pass = bt_pass = both = 0
    misses = []
    for cid, b, e, ebatch in rows:
        b_ok_fail = not b["passed"]
        e_ok = bool(e and e.get("passed"))
        if b_ok_fail:
            bl_fail += 1
        if e_ok:
            en_pass += 1
        if b_ok_fail and e_ok:
            both += 1
        elif not e_ok:
            misses.append(cid)
        if ebatch and ebatch.get("passed"):
            bt_pass += 1
        nfail = b["subchecks_total"] - b["subchecks_passed"]
        bcol = ""
        if have_batch:
            bcol = f" {('PASS' if ebatch and ebatch.get('passed') else 'FAIL') if ebatch else '—':>5}"
        print(f"{cid:9} {'FAIL' if b_ok_fail else 'PASS':>9} "
              f"{('PASS' if e_ok else 'FAIL') if e else '—':>5}{bcol} "
              f"{e['mode'] if e else '—':16} {nfail:>9}")
    print(f"\n9B-alone fails: {bl_fail}/{len(rows)} | workflow passes: {en_pass}/{len(rows)} | "
          f"BOTH PROPERTIES: {both}/{len(rows)}")
    if have_batch:
        print(f"batch passes: {bt_pass}/{sum(1 for r in rows if r[3])}")
        wj = BENCH / "batch-wall.json"
        if wj.exists():
            print(f"batch wall: {json.loads(wj.read_text())['wall_seconds']}s "
                  f"(vs per-case enhance total: see enhance-*.json run dirs)")
    if misses:
        print("workflow misses:", " ".join(misses))
    summary = {
        "total": len(rows),
        "baseline_fail": bl_fail,
        "enhance_pass": en_pass,
        "batch_pass": bt_pass,
        "both": both,
        "misses": misses,
    }
    (BENCH / "summary.json").write_text(json.dumps(summary, indent=2))
    return 0


def phase_baseline_batch():
    import yaml as _y
    gen = sh([sys.executable, str(SCRIPTS / "gen-batch-baseline.py")])
    print(gen.stdout.strip())
    if "no pending cases" in gen.stdout:
        return 0
    val = sh([sys.executable, str(SCRIPTS / "validate-rea.py"),
              str(REA / "workflows" / "baseline-batch.yml"), "--allow-9b", "--batch"])
    if val.returncode != 0:
        print(val.stdout)
        return 1
    r = sh(["bash", str(RUNNER), str(REA / "cases" / "benchmark" / "case-b01.yml"), "1800"],
           env_extra={"REA_WORKFLOW": str(REA / "workflows" / "baseline-batch.yml")})
    outdir = None
    for ln in r.stdout.splitlines():
        if ln.startswith("[rea] output="):
            outdir = ln.split("=", 1)[1].strip()
    if not outdir:
        print(r.stdout[-800:])
        return 1
    outdir = Path(outdir)
    harvested = 0
    for ans in sorted(outdir.glob("b*-answer.txt")):
        num = ans.stem.split("-")[0][1:]
        cid = f"rea-b{num}"
        case_file = CASES / f"case-b{num}.yml"
        chk = outdir / f"b{num}-check.json"
        marker = BENCH / f"baseline-{cid}.json"
        if not chk.exists() or marker.exists():
            continue
        c = _y.safe_load(case_file.read_text())
        cj = json.loads(chk.read_text())
        marker.write_text(json.dumps({
            "case_id": cid,
            "run_dir": str(outdir),
            "answer": ans.read_text(),
            "passed": cj["passed"],
            "subchecks_passed": cj["subchecks_passed"],
            "subchecks_total": cj["subchecks_total"],
            "failures": cj["failures"],
        }, indent=2))
        harvested += 1
        nfail = cj["subchecks_total"] - cj["subchecks_passed"]
        verdict = "FAIL (good)" if not cj["passed"] else "PASS (NEEDS STRENGTHENING)"
        print(f"[bm] {cid}: baseline {verdict} — {nfail} subcheck(s) failing")
    print(f"[bm] batch harvested {harvested} case result(s)")
    return 0


def phase_enhance_batch(timeout=3600):
    import shutil
    rt = BENCH / "batch-runtime"
    if rt.exists():
        shutil.rmtree(rt)
    for old in BENCH.glob("enhance-batch-rea-*.json"):
        old.unlink()
    all_cases = sorted(int(cf.stem.replace("case-b", ""))
                       for cf in CASES.glob("case-b*.yml"))
    chunks = [all_cases[i:i + 17] for i in range(0, len(all_cases), 17)]
    total_wall = 0
    for ci, chunk in enumerate(chunks, 1):
        case_arg = ",".join(str(n) for n in chunk)
        wf = REA / "workflows" / f"batch-enhance-{ci}.yml"
        gen = sh([sys.executable, str(SCRIPTS / "gen-batch-enhance.py"),
                  "--out", str(wf), "--cases", case_arg])
        print(gen.stdout.strip())
        val = sh([sys.executable, str(SCRIPTS / "validate-rea.py"), str(wf), "--batch"])
        if val.returncode != 0:
            print(val.stdout)
            return 1
        print(val.stdout.strip())
        dummy = CASES / "case-b01.yml"
        t0 = time.time()
        code, outdir, stdout = run_runner(dummy, workflow_env=wf, timeout=timeout)
        wall = int(time.time() - t0)
        total_wall += wall
        print(f"[bm] chunk {ci}/{len(chunks)} ({len(chunk)} cases) wall: {wall}s (rc={code})")
    harvested = 0
    for run in sorted((BENCH / "batch-runtime").glob("b*")):
        sb_p = run / "select-best.json"
        if not sb_p.exists():
            continue
        num = run.name[1:].zfill(2)
        sb = json.loads(sb_p.read_text())
        marker = BENCH / f"enhance-batch-rea-b{num}.json"
        marker.write_text(json.dumps({
            "case_id": f"rea-b{num}",
            "run_dir": str(run),
            "mode": sb["mode"],
            "selected": sb.get("selected"),
            "passed": sb.get("passed"),
        }, indent=2))
        harvested += 1
        verdict = "PASS" if sb.get("passed") else "FAIL"
        print(f"[bm] rea-b{num}: batch enhance {verdict} — mode={sb['mode']}")
    print(f"[bm] batch harvested {harvested} case(s) in {total_wall}s total")
    (BENCH / "batch-wall.json").write_text(json.dumps({"wall_seconds": total_wall}, indent=2))
    return 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("phase", choices=["baseline", "baseline-batch", "enhance",
                                      "enhance-batch", "report"])
    ap.add_argument("patterns", nargs="*")
    args = ap.parse_args()
    files = case_files(args.patterns)
    if args.phase == "baseline":
        return phase_baseline(files)
    if args.phase == "baseline-batch":
        return phase_baseline_batch()
    if args.phase == "enhance":
        return phase_enhance(files)
    if args.phase == "enhance-batch":
        return phase_enhance_batch()
    return phase_report()


if __name__ == "__main__":
    sys.exit(main())
