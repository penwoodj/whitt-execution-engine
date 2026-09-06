# Version Iteration Ruleset — Top-Level Workflow + Subworkflows (v4 campaign)

Binding rules for iterating the top-level workflow (current: `meta-v3-fast`)
and its subworkflows against the 170-case corpus
(`prompt-analysis/test-cases/`, tiers A/B/C). Extends `03-ORCHESTRATION.md`
(per-experiment loop) with version-level and batch-level discipline.
Every batch in the v4 campaign follows this document. Deviations require
an explicit note in the batch log saying what deviated and why.

## 1. Edit-target decision matrix — where a change goes

Symptom → edit target. Always the narrowest target that fixes the class
of failure, never a wider one "while we're in there".

| Symptom | Edit target | Never edit instead |
|---|---|---|
| Wrong lane route (case lands HEAVY, truth LIGHT) | `meta-conf.py` probe/fixture data, or lane assignment in `meta-prompts-170.yml` (only if assignment itself wrong) | lane chains in the workflow yml |
| Gate/routing logic wrong (skip-if-won not firing, lane head not reached) | top-level yml: GWT clauses, `route_to`, hook wiring — via `gen-meta-system.py` templates | subworkflow atom ymls |
| Stage produces wrong/low-quality output in ONE lane/atom | that subworkflow's generator templates (`gen-workflow.py` / `stage-emit.py` stage blocks) | top-level composition |
| Same defect appears across MANY cases in ≥2 lanes | generator script (single source), then regenerate — never hand-edit generated ymls | the generated yml files themselves |
| Prompt-assembly defect (stage emit, carry, hints) | `stage-emit.py` / `fas_lib.py` | workflow yml |
| Engine crash, hook misfire, schema validation failure | `src/` engine code with TDD (failing test first) | generator workarounds that mask the bug |
| Case itself broken (fixture truth wrong, unrunnable ask) | the case file in `prompt-analysis/test-cases/` + note in batch log; regenerate prompt entry | workflow or engine to satisfy a bad case |

Hand rule: **generated ymls are build artifacts**. If a fix belongs in the
yml, it belongs in the generator. Hand-edited ymls rot on the next regen.

## 2. Versioning

- `meta-vN[-bNN]`: vN = workflow version, bNN = batch it was generated for.
- Bump the MINOR version (v4 → v5) only for: lane-composition change, new/
  removed atom, gate-logic change. Prompt-text or template tweaks inside a
  batch keep the version and are recorded as `v4-bNN-rM` (revision M).
- Every generated workflow is validated (`scripts/meta-v6/validate-workflow.py`)
  before any engine run. Validation failure = batch does not start.

## 3. Batch protocol (10 cases per batch, 17 batches)

Batch order: A-001..010 = b01 … A-091..100 = b10, B-001..020 = b11-b12,
C-001..050 = b13-b17.

```
PHASE 0  INGEST      ingest-testcases.py syncs case entries; gen-meta-system
                      --only <batch ids> → top-level/workflows/meta-v4-bNN.yml;
                      validate. Abort batch on validation failure.
PHASE 1  RUN         live engine run, REAL LLM CALLS, fresh (no seeds).
                      Record per-case wall + pass/fail into batch log.
PHASE 2  TRIAGE      every failure classified via §1 matrix: ROUTE / GATE /
                      STAGE-QUALITY / GENERATOR / ENGINE / CASE-DEFECT.
                      Fix at the target. Then:
                        2a. rerun FAILING CASES ONLY (targeted retest)
                        2b. then rerun the batch-previous 10 (or all prior
                            cases in current batch) — order per user rule:
                            failing first, then previous-in-batch, then
                            continue remaining new cases.
PHASE 3  REGRESSION  run ALL previously passing cases (union b01..bNN-1 plus
                      passed cases of bNN). Any previously-passing case now
                      failing = regression → fix-forward before proceeding.
PHASE 4  DEDUP+EFF   on the CURRENT v4 yml with the batch's cases:
                      4a. yml-dedup-audit.py: duplicate step/hook bodies
                          across lanes → consolidate via generator template
                          (shared blocks), regenerate, validate.
                      4b. measure: per-stage timings from run logs; compare
                          against previous revision's timings for the same
                          cases (not against different cases).
                      4c. improve only if BOTH hold: wall-clock faster AND
                          quality not worse (same-or-more passes; check
                          strictness unchanged). Apply → regenerate → PHASE 1
                          rerun of this batch (fresh) → PHASE 3 again.
                      4d. efficiency results logged even when "no change" —
                          a measured decision not to optimize is a result.
PHASE 5  GATE        batch complete iff: all 10 new cases pass AND full
                      regression green AND dedup/eff cycle recorded.
                      Only then start bNN+1. Otherwise loop PHASE 2-4
                      (max 3 fix cycles per QA rule; then STOP and escalate
                      to user with the batch log).
```

## 4. Test-selection discipline (never run everything blindly)

- Fresh batch run: the 10 new cases only.
- Retest after a fix: the failing cases only — a fix is verified where it
  failed, not by re-running 170 files.
- Batch regression: prior batch + current-batch passers.
- Full-suite regression: once per batch, at PHASE 3, and at any version
  bump (§2). Full-suite is a checkpoint, not a dev loop.
- Seed-carry/seed-hint machinery is RETRY RESCUE ONLY: never used to declare
  a fresh batch run passing. A case passes when a fresh run passes it.

## 5. Regression policy

- Never delete, weaken, or `# DEFERRED`-out a passing check to make a
  regression go green. Weakening a check = failing the batch.
- Regressions fix-forward at the §1 target. If the fix would change ≥2
  edit-target classes at once, split into two recorded fixes.
- Tally method: per-run-dir any-pass (established; mtime-based tallies are
  invalid when seed copies exist).

## 6. Dedup + efficiency criteria (what "better" means, provably)

- Structural dedup: identical hook blocks, repeated step scaffolds, and
  duplicated stage emits across lanes are consolidated in the GENERATOR;
  the generated yml shrinks or stays flat while case count grows.
- Speed: per-case wall from engine logs, median of the batch, compared
  revision-to-revision on the same case ids. Improvement claims must cite
  both numbers. <2x on a micro-loop is not worth a regen cycle — record
  and move on.
- Quality: pass-count per batch (same cases) + spot judge on 2 cases per
  batch where output text is compared old-vs-new. Quality may not drop
  even when speed rises.
- Wall-clock honesty: LLM tokens/sec drift, docker state, and machine load
  are confounders — timing comparisons are same-session where possible,
  and any cross-session comparison is labeled as such in the log.

## 7. Live-system safety (hard, from AGENTS.md + model-probe findings)

- Real LLM calls: Qwen3-5-9B-Q4_K_M, gpu_layers stays server-side (99);
  NEVER `load_params` in workflow yml (compose-path bug = CPU crash vector).
- Pre-flight per run: `/health` ok, `free -h` ≥3GB available, docker logs
  clean of `vk::|DeviceLost`. Between batches: RAM re-check; restart docker
  if below floor. Max 5 consecutive batch runs before a cooldown check.
- Non-interactive everywhere; engine runs are foreground with timeouts.

## 8. Batch log (per batch, appended to `top-level/runs/v4/BATCH-LOG.md`)

```
## bNN (date, workflow revision, seed: none/refresh)
- ingest: N cases, validation: PASS/FAIL
- run r1: X/10 pass, wall per case [...] 
- triage: [failure → class → target → fix]
- retest: failing-only results; batch-previous results
- regression: all-prior green Y/N, fixes [...]
- dedup: findings + consolidations applied
- efficiency: median wall prev → new, quality parity note
- GATE: PASS/FAIL → next batch / escalation
```

The log is the evidence chain. A batch without a log entry did not happen.

## 9. Current campaign bindings

- Corpus: 170 cases, lanes/shape from case headers (tier-A title line,
  tier-B title line, tier-C = LIGHT).
- Version under iteration: **meta-v4** (first: `meta-v4-b01`).
- Baseline for efficiency comparisons: v3-fast top-level timings
  (`docs/runtime-breakdown.csv`) where comparable; else batch-internal
  r1-vs-rM on same case ids.
- Generator extensions for this campaign (prompts file + batch filter)
  land in `gen-meta-system.py` / `ingest-testcases.py` with tests in
  `test_meta.py` — same TDD discipline as the engine.
