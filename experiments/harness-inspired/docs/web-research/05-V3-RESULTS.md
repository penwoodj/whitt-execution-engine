# Harness v3 Agentic Suite — Results

> 2026-08-23. Live system, Docker llama.cpp router, Qwen3-5-9B-Q4_K_M.

## Final: 8/10 PASS (9/10 observed on one run — ha-06 variance)

| Case | Hops | Verdict | Depth | Time |
|------|------|---------|-------|------|
| ha-01 skip-if-won resume | 3 | PASS | 0 | 23s |
| ha-02 det-first judge gating | 3 | PASS | 0 | 28s |
| ha-03 win-depth scoring | 4 | PASS | 0 | 30s |
| ha-04 adaptive fix budget | 5 | PASS | 0 | 39s |
| ha-05 cascade token budget | 6 | PASS | 0 | 53s |
| ha-06 two-lane verdict matrix | 5 | FAIL (variance; passed prior run) | — | 29s |
| ha-07 cooldown scheduling | 4 | PASS | 0 | 39s |
| ha-08 trace ledger | 3 | FAIL (chronic: end_fail-line inference) | — | 23s |
| ha-09 wasted-effort accounting | 6 | PASS | 0-2 | 55-61s |
| ha-10 leak-safe feedback policy | 5 | PASS | 0 | 10s |

## Version Progression (the iteration log)

| Version | Change | Pass | Key finding |
|---------|--------|------|-------------|
| v3.0 | 4B, JSON-only output | 2/10 | All outputs clean JSON — failures = wrong values, not format |
| v3.1 | 4B + CoT prompts + last-JSON extraction (P8) | 3/10 | CoT helps; 4B ceiling on hops 5-6 |
| v3.2 | 9B escalation (REA+ R4: CONTENT fail at all stages → escalate model) | 5/10 | fix_1 recoveries appear (leak-safe feedback works) |
| v3.3 | max_words removed (case bug: killed correct CoT answers), enumeration-first prompts | 8-9/10 | ha-09 had CORRECT JSON killed by max_words=70 for 3 runs |

## Research Principles Applied (docs/web-research/)

- P1/P2/P7 det-first: json_exact = primary verdict, judge secondary
- P8 reason-free/constrain-late: CoT then final-line JSON; checker extracts last JSON object
- P9 prompt format enforcement, no constrained decoding
- P15 no self-consistency voting
- P19 role isolation: fix_2 = history-free from-scratch corrector (recovered ha-04, ha-09 at depth 2)
- F8 leak-safe feedback: check names + categories + model's observed values, never expected
- F9 repair@k=2 cap
- F10 cascade gating: det-PASS skips fix cascade (all 8 wins at depth 0 = zero wasted stages)
- Trace replay: per-case JSONL ledger with case field (AgentAssay pattern)

## Bugs Found + Fixed This Cycle

1. trace-check.py 3-arg vs 4-arg call mismatch → silent trace loss → false FAIL verdicts
2. trace-append.py crash on bare case id (expects key=value)
3. extract_last_json initially nested inside run_checks (IndentationError-class breakage)
4. max_words check from v1 template killed correct CoT outputs (1263 words vs 70 limit)
5. Token truncation: enumeration CoT needs 2400 tokens (was 640)
6. WHITT_ZOMBIE_MAX=4 env needed for router-mode (engine false-positive on transient workers)

## Remaining (honest)

- ha-08: 9B never infers end_fail line applies (5/5 attempts miss). Model ceiling, R4 CONTENT.
- ha-06: distractor-trap variance (passed run 2, failed run 4). Borderline.
- Collector sees only last case's trace (runner resets trace per case) — per-case accumulation pending.
- 4B baseline for comparison matrix (variant matrix principle 15) not run — single-model scope this cycle.

## Cost

- 10 cases x 4 LLM stages max = ~40 calls worst case; actual final run: 10 solves + 0 fixes + 10 judges + 2 fix-cascades = ~20 calls. Depth-0 wins = early-exit working.
