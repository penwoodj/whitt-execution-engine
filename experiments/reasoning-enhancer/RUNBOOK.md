# REA v1 — Live-Test Runbook

Execute ONLY when the other experiment finishes and the machine is free
(user directive). Everything below assumes Docker llama.cpp (Vulkan) stack
per AGENTS.md.

## 0. Preconditions

```bash
# other experiment stopped; docker healthy
curl -fsS http://localhost:8080/health || echo "server down — start docker stack first"
docker logs whitt-llama-server 2>&1 | tail -50 | grep -E "vk::|DeviceLost|Vulkan.*Error" \
  && echo "VULKAN ERRORS — docker restart whitt-llama-server first" || true
ls /home/jon/code/whitt-execution-engine/models/ | grep -E "Qwen3-1.7B-abliterated|Qwen3-4B-Instruct"
# whitt binary present
ls -la /home/jon/code/whitt-execution-engine/target/release/whitt
```

## 1. Pre-flight self-tests (offline, no model)

```bash
cd experiments/reasoning-enhancer
python3 scripts/test-cases.py        # 6 cases lint clean
python3 scripts/validate-rea.py      # workflow structure
python3 scripts/dry-run.py           # 6 scenarios green
```

All must pass before ANY live run.

## 2. Smoke (P1) — one case

```bash
cd experiments/reasoning-enhancer
bash scripts/run-enhancer.sh cases/case-001.yml
# watch: tail -f results/<ts>-rea-001/run.log  (separate terminal)
```

Expect: SUCCESS exit=0 ≤300s; results dir contains select-best.json,
final-answer.txt, metrics.json. Inspect:

```bash
cat results/<ts>-rea-001/select-best.json
cat results/<ts>-rea-001/metrics.json
```

If mode=selected_r1 and passed=true → smoke gate passed → proceed P2.

## 3. Suite (P2) — all 6 cases

```bash
cd experiments/reasoning-enhancer
for c in cases/case-00*.yml; do
  bash scripts/run-enhancer.sh "$c" || echo "case $c failed — continue"
  free -h   # RAM check between runs (watchdog also active)
done
```

Runner enforces: preflight, sequential (WHITT_MAX_CONCURRENT_INFERENCES=1),
RAM-gated cooldown, 300s timeout, ram-watchdog, unload-all-first.

## 4. Analysis (P3)

```bash
cd experiments/reasoning-enhancer/results
python3 - <<'EOF'
import glob, json
rows = []
for m in sorted(glob.glob("*/metrics.json")):
    d = json.load(open(m))
    rows.append((m.split("/")[0], d["mode"], d["case_passed"],
                 d["rounds_completed"], d["wall_clock_seconds"]))
for r in rows: print(f"{r[0]:28s} mode={r[1]:18s} passed={r[2]} rounds={r[3]} {r[4]}s")
n = len(rows); p = sum(1 for r in rows if r[2])
med = sorted(r[4] for r in rows)[n // 2]
print(f"\nG3 pass {p}/{n} (want >=5)   G4 median {med}s (want <=180)")
EOF
```

G5 ratchet audit: for each run dir, select-best.json candidates — confirm
selected subchecks_passed ≥ input subchecks_passed (or mode=hard_fail_no_ship).
G6: any run with mode=selected_rN, passed=true, input failed.

Fill PLAN.md suite table. Compare final-answer.txt quality vs fixtures
references manually — checks are necessary, not sufficient.

## 5. Troubleshooting

| Symptom | Fix |
|---|---|
| whitt complains about model filter / loads wrong models | rea-v1 needs BOTH Qwen 1.7B + 4B (no --filter-name). If engine requires a filter flag, check `whitt benchmark --help` semantics — do NOT filter to one model; report to user |
| Vulkan / DeviceLost in docker logs | docker restart whitt-llama-server; sleep 45; rerun case |
| timeout 124 | inspect run.log step timestamps; likely 4B load slow — retry once; if repeat, reduce to r1-only variant (PLAN ablation 1) |
| step shell fails (gate-prepare.exit non-zero) | run emit-case.py manually with same args; scripts are pure python3+pyyaml |
| RAM watchdog killed run | wait ≥120s, verify ≥3GB free, rerun; never disable watchdog |

## 6. Safety rules (SAFETY.md — binding)

R1 sequential only · R2 preflight every run · R3 watchdog active · R4 300s
cap · R5 cooldown RAM-gated · R6 unload-all-first · R9 no network · R10
case lint gate · R11 output confinement to results dir · R14 ratchet
invariant — never ship worse-than-input.
