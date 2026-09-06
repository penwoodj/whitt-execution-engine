# V7 Zero-LLM Verification Report

> 2026-08-23. All 7 approved improvements built + verified. ZERO LLM calls,
> zero model load/unload. Docker touched only via health endpoint.

## Improvements Delivered

| # | Improvement | Research basis | Artifact | Verified by |
|---|-------------|----------------|----------|-------------|
| 1 | Hops-based model routing (≤4→4B, ≥5→9B) | P5 difficulty-aware routing | gen-v7.py models block: m_light/m_heavy, 11 worker refs + 1 judge ref per workflow; ha-01/03/07 light, ha-04/05/09/10 heavy | unit tests + validator |
| 2 | Per-stage artifact snapshots | AgentAssay replay + Codex GhostSnapshot | artifact.{solve,fix_1,fix_2}.txt via cp-after-GWT hooks | replay-rescore wind2 test |
| 3 | Mechanical leak detector | F8 leak-safe feedback + P17 role-drift | leak-audit.py: strong-literal scan over *feedback*.txt | injected-leak test (exit 1) + clean-path 0 |
| 4 | Adversarial spoof scenarios | regression hardening | adv_partial/adv_malformed/adv_wrongtype/adv_dup scenarios | battery 4×10 green, all route check=fail→fix_1→win |
| 5 | Judge v2: 3 binary sub-checks + cross-family judge | F3 binary criteria + F4 self-preference | KEYS/VALUES/FORMAT prompt on Hermes-2-Pro-Mistral-7B | unit test on generated prompt |
| 6 | Token metering per stage | Devin ACU metering | meter.py → trace gate=meter entries (tokens_est=chars/4) | battery traces show meter entries |
| 7 | Thinking-model variant | s1 budget-forcing shape | --variant thinking: Qwen3-4B-Thinking, <think> budget block, 2048/2560 caps | 10 thinking-spoof YAMLs validate + smoke |

## Evidence Chain

1. **Validation**: 30 YAMLs (10 live + 10 spoof + 10 thinking-spoof) PASS
   `scripts/meta-v6/validate-workflow.py` — run from repo root (validator
   CWD dependency documented).
2. **Spoof battery**: 80/80 GREEN = 10 cases × {wind0, wind1, wind2, lose,
   adv_partial, adv_malformed, adv_wrongtype, adv_dup}. Real engine
   (`whitt benchmark`), zero inference (skip_step + route pre-inference),
   82.4s total, ~1s/run. Every EXPECT_SEQ ends audit=pass.
3. **Replay re-scoring**: wind2 snapshots re-scored against current case —
   solve=FAIL fix_1=FAIL fix_2=PASS, final.match=TRUE, value==json_exact.
4. **Leak detection**: injected 'should be "VIOLATION"' into ha-10 lose-run
   feedback → leak-audit exit 1 naming entity round_2. All battery runs
   (clean feedback) exit 0.
5. **Unit tests**: test_v7_scripts.py 16/16; harness-inspired suite total
   133/133.
6. **Trace enrichment**: every path now emits meter entries + audit=pass;
   judge/end_fail mutually exclusive across all 80 runs.

## Known Limitations (honest)

- leak-audit cannot textually detect single-digit scalar leaks ("3"
  appears legitimately) — strong-literal heuristic documented in script.
- Thinking variant verified structurally only; live inference deferred
  (needs model + permission).
- Live 10-case v7 run pending LLM permission:
  `python3 experiments/harness-inspired/scripts/run-v6-suite.py --gen v7`
  (runner supports conditional restart via zombie-count.sh).

## Bug Ledger (found by own verification, all fixed)

1. leak-audit scalar collision → strong-literal exclusion
2. adv_wrongtype no-op on dict/str answers → 4-type flip matrix
3. thinking model name computed but unwired → light_name/heavy_name
4. test assertion drift (m_light string, judge line)
5. validator false-FAILs from non-root CWD → repo-root contract
