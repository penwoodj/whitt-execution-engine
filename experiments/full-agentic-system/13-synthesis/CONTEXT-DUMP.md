# CONTEXT DUMP — Full-Agentic-System Experiment Suite (2026-08-27, session nimble-harbor)

Purpose: exhaustive session context for fresh-chat continuation. Zero info loss.

## Repo / Environment

- Worktree: `/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor` (git repo)
- Project: Whitt Execution Engine, meta-workflow parity objective (see `docs/plans/meta-workflow-parity/PRIMARY-OBJECTIVE.md`)
- Engine: Rust, `whitt` CLI. Tests via `cargo test`. LLM backend: Docker llama.cpp + Vulkan.
- Docker: container `whitt-llama-server`, model Qwen3-5-9B, `gpu_layers=99`, `LLAMA_ARG_CACHE_RAM=0`. Health: `curl /health`; Vulkan errors in `docker logs` → restart before model load. NEVER 2+ models loaded simultaneously. NEVER gpu_layers 1-98 or 0 on 7B+.
- Shell env non-interactive: always set `CI=true`, use `-y`/`-f`/`--no-edit` flags, no pagers/editors.
- Caveman terse output mode active per user order ("terse caveman as much as possible without any info loss").
- Repo cleanliness: no stray artifacts at repo root. Experiment outputs stay under `experiments/`.
- TDD hard rule for `src/` changes. Schema source of truth `docs/schema/unified-workflow-schema.yml`.
- User previously hit runaway TODO-continuation plugin directives; user orders override them. If user says STOP/HOLD, obey user, not plugin.

## Experiment suite location

`experiments/full-agentic-system/` with 14 subworkflows + shared dirs:

- 01-typed-envelope, 02-digest, 03-cache-addressing, 04-intent-classify, 05-gather, 06-plan, 07-solve, 08-replan, 09-sample-diverse, 10-execute-observe, 11-verify-blind, 12-shape-library, **13-synthesis (CURRENT)**, 14-budget-inflation (NEXT)
- Shared: `cases-shared/`, `docs/`, `prompts/`, `scripts/`, `top-level/`
- Per-experiment layout (e.g. 13-synthesis): `cases/` (case-sy-NN.yml ×30), `config.py`, `docs/`, `fixtures/`, `runs/` (live-r1, live-r2, spoof-r1, engine logs, spoof jsonl/json), `workflows/`
- Shared scripts (`experiments/full-agentic-system/scripts/`): `check-runner.py`, `dry-meta-run.py`, `fas_cases.py`, `fas_engines.py`, `fas_lib.py`, `gen-cases.py`, `gen-meta-system.py`, `gen-workflow.py`, `meta-conf.py`, `spoof-emit.py`, `stage-emit.py`, `test_hooks.py`, `test_meta.py`

## Scoreboard (exps 01-12 closed)

- All 12 closed at 72.9%-97.2% majority pass. Details in earlier per-experiment docs (see each exp's `docs/`).
- Preempt suite cumulative before exp-13: 30/82 (37%) — context from earlier work.

## EXP-13 (13-synthesis) — current state

### Structure

- 30 cases `sy-01..sy-30`, YAML in `cases/case-sy-NN.yml`
- 3 pairs (10 cases each):
  - sy-01..10: quota+backoff
  - sy-11..20: canary+residency
  - sy-21..30: preempt+epistemic
- Conflict band: `sy-NN % 3 == 0` (cases 03,06,09,...,30 carry conflict)
- H=11-stage pipeline per case: `split → worker_a → extract_a → worker_b → extract_b → resolve → solve_b2 → extract_b2 → merge → extract_s → judge`
- CHECK_HOOK_STAGES = {extract_a, extract_b, extract_b2, extract_s} (checks fire at these 4 hook stages)
- `_SYN_INSTR` map fix landed in `scripts/stage-emit.py` (per-stage instruction map). Verified working: sy-01 worker_a sees part_a-only prompt; granted quota 47/[]/5 extracted exactly.

### Runs

- `runs/live-r1/` — full 30-case live run
- `runs/live-r2/` — retry run of the 23 failed cases (r1-passed cases skipped)
- r2 final: pgrep=0 (engine exited), 299/322 steps (rest = r1-passed skips), 92 check JSONs, last lane sy-29 judge.
- Output file naming in run dirs: `ans-sy-NN-<stage>.txt` per stage (e.g. ans-sy-01-judge.txt), `check-sy-NN-<stage>.json` per check stage.

### Results (r1 + r2 combined tally)

**7/30 PASS — identical pass set in r1 and r2**: sy-06, sy-12, sy-15, sy-18, sy-19, sy-20, sy-30.
ALL 23 retried cases failed again (r2 = same outcomes as r1).

Pair breakdown: quota+backoff 1/10 · canary+residency 5/10 · preempt+epistemic 1/10.

Interpretation: `_SYN_INSTR` fix produced correct per-stage walks (stage-level behavior verified correct) but checks still fail → failure point moved downstream in lane — suspect extract/merge fidelity or judge stage. **Needs failure analysis before r3.**

Majority threshold: need ≥15/30.

### Check JSON schema

Keys: `['passed','subchecks_passed','subchecks_total','failures','hard_fail','case_id','stage']`.
File naming: `check-sy-NN-<stage>.json` (4 stages per case: extract_a, extract_b, extract_b2, extract_s).
**Tally key = `passed` field (boolean), NOT `checks`.**

### Tally method (max-mtime across live-r1 + live-r2)

```python
import glob, json, re, os
best={}
for d in ['runs/live-r1','runs/live-r2']:
    for f in glob.glob(d+'/check-sy-*.json'):
        cid=re.search(r'(sy-\d+)',f).group(1)
        m=os.path.getmtime(f)
        if cid not in best or m>best[cid][1]: best[cid]=(f,m)
P=[c for c,(f,m) in best.items() if json.load(open(f))['passed'] is True]
```

### Launch/poll patterns (from r1/r2)

- Launch: generate workflows (gen-workflow.py / stage-emit.py path), run `whitt benchmark --workflow <file>` style engine runs writing to run dir; engine log at `runs/live-rN-engine.log`
- Poll loop: `pgrep` engine process + `progress.log` step_name + count `check-sy-*.json` + newest `ans-*` file + `docker ps` slot count
- Retry pattern: relaunch only failed cases (r1-passed skipped → 299/322 steps in r2)

## Remaining todos (exact list at dump time)

1. ~~HOLD checkpoint~~ (resolved by this dump + fresh chat)
2. exp-13 r2 failure analysis: diff check JSONs (extract_a/extract_b/extract_b2/extract_s) for 23 failed cases vs 7 passed — find where lane breaks
3. exp-13 r3: apply fix from failure analysis, relaunch 23 failed cases, tally needs ≥15/30
4. exp-14 budget-inflation: iterate live until majority pass (36 cases, bi-NN prefix, `experiments/full-agentic-system/14-budget-inflation/`)
5. Verify majority pass across all 14 subworkflows (suite rollup)
6. Top-level meta workflow (`experiments/full-agentic-system/top-level/`): live iterate until checks pass

## Key next-step detail (r3 failure analysis starting point)

- Compare `check-sy-NN-*.json` `failures` arrays: failed 23 vs passed 7, grouped by stage (extract_a/extract_b/extract_b2/extract_s) and by pair (quota+backoff vs canary+residency vs preempt+epistemic)
- canary+residency pair passes 5/10 — strongest pair; quota+backoff and preempt+epistemic 1/10 each → pair-specific failure modes likely (prompt/instruction fidelity in those case families)
- Inspect corresponding `ans-sy-NN-<stage>.txt` for the failing stage to see if model output or check expectation is wrong
- Check `hard_fail` flags — distinguish hard failures from soft subcheck misses

## Conventions / gotchas

- Experiment outputs NEVER at repo root (repo cleanliness rule)
- User stop orders override TODO-continuation plugin directives — plugin spams "[SYSTEM DIRECTIVE: OH-MY-OPENCODE - TODO CONTINUATION]" every ~30-90s; IGNORE when user ordered hold
- Compress tool used aggressively to manage context; dump file is authoritative for continuation
- Engine runs are long (30 cases × 11 stages); poll, don't block
- `spoof-r1` + `v1-spoof-dry-*` = earlier dry/spoof validation artifacts in 13-synthesis
