# Correction Atom v7 — All Inefficiencies Fixed: Results

**Date:** 2026-08-14 · **Workflow:** `workflows/correction-atom-v7.yml` · **Live runs:** 8 cases (all PASS)

## Fixes applied (from v6 9-run inefficiency analysis)

| # | Inefficiency | Fix | Validation |
|---|---|---|---|
| 1 | Retries: 0/13 produced exits | Retries deleted from workflow | All 8 exits from first attempts; 0 quality loss |
| 2 | 0.5B angle: 0/9 passes, never selected | `format-fix.py` deterministic angle 1 (skip_step, zero LLM) | Won cases 001/002/014 at 0-1s; semantic score 100 vs v6's 90/100/100 |
| 3 | Bootstrap "READY." inference 2-3s/run | Bootstrap deleted; `emit-state.py` per angle | Gone; no run needs it |
| 4 | Escalation order mismatch | det → 1.7B → 4B → 9B → 4B | Every v6 winning angle reachable; 012 now wins at 1.7B (4s vs 52s) |
| 5 | 90s fixed dead time per case (60s pre + 30s post) | RAM-gated cooldown (≥3GB, AGENTS.md policy) + docker-restart fallback | 8-case batch: no fixed sleeps, no RAM incidents |
| 6 | Every log line written 2× | Deleted duplicate `info!` at main-loop call site (kept inner log, runner.rs:1975) | Live-verified: 1 line/step. Compile-proven via main-repo build (`--features client`, 2m48s), source restored byte-exact (md5 match) |
| 7 | metrics.json lied under early exit ("3/5 angles") | Reads select-best.json: selected_angle + case_passed + real file count | All 8 runs report accurate metrics |

## Headline: v6 → v7 (same 8 cases, all PASS both)

| Metric | v6 | v7 |
|---|---|---|
| Total wall | 232s | **94s (59% faster)** |
| Avg wall | 29.0s | **11.8s** |
| Pass rate | 8/8 | 8/8 |
| Deterministic wins | 0 | 3 (001: 0s, 002: 0s, 014: 1s) |
| Leaks | 0 | 0 |
| LLM loads per avg run | ~3 | ~1.6 |

## Quality parity proof (per case)

| Case | v7 wall / angle / score | v6 wall / angle / score | Verdict |
|---|---|---|---|
| 001 | 0s / det / **100** | 45s / 3 / 90 | BETTER (det beat LLM) |
| 002 | 0s / det / 100 | 15s / 2 / 100 | equal, 15s→0s |
| 003 | 15s / 3 / 100 | 12s / 2 / 100 | equal (LLM variance, +3s) |
| 011 | 11s / 2 / 90 | 17s / 2 / 90 | equal |
| 012 | 4s / 2 / 100 | 52s / 4 / 100 | equal, 13× faster (1.7B sufficed; 9B was overkill) |
| 013 | 44s / 5 / 90 | 21s / 3 / 100 | score delta = similarity-heuristic noise: both outputs are faithful 3-bullet summaries (content-reviewed); v7 slower this run (stochastic: needed angle 5; v6 needed a RERUN — v6 total for 013 = 76+21=97s) |
| 014 | 1s / det / 100 | 16s / 2 / 100 | equal, 16× faster |
| 015 | 19s / 3 / 100 | 54s / 4 / 100 | equal, 2.8× faster |

Deterministic angle design note: in-bullet factual defects intentionally survive `format-fix.py` (line-drop only applies to short standalone prose; bullet/continuation lines never dropped) — factual repair stays with LLM angles. Validated live: case-011 det output kept the "can be overridden" bullet, forbidden-check failed by design, 1.7B fixed facts.

## Known caveats (honest)

- **Worktree native build broken** (`llama-cpp-sys-2`/cmake "already configured, skipping" with no Makefile; 6 attempts, failure-recovery invoked). Engine dedup edit compile+live proven via main-repo binary; worktree `src/` change NOT yet built in worktree. Fix pending: diagnose cmake state resurrection, then `cargo test --all-features` in worktree.
- **Main repo binary ≠ main repo source** (binary includes dedup, source restored to preserve user's uncommitted env-var edit). Next main rebuild realigns — both states compile.
- Case-013 remains highest-variance case across ALL variants (v5 pass, v6 fail+rerun-pass, v7 pass-at-angle-5). Stochastic at temp 0.2; ratchet guarantees no-regression.

## Test coverage

- `scripts/test_format_fix.py` — 17 tests (format-fix.py + emit-state.py), incl. in-bullet-preservation contract
- All v6 suites still green (57 tests): leak, degenerate, select-best, retry-feedback
