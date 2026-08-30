# REA+ — Metric Dimensions (8)

Every run computes all 8 cheaply from artifacts. Each iteration version
targets exactly ONE (order in 05-TDD-ROADMAP). No single-lens claims.

## Definitions

| id | dimension | definition | source artifact | computed by |
|---|---|---|---|---|
| M1 | Accuracy | task-correct rate: all subchecks pass, external truth | check-*.json | judge.py |
| M2 | Format fidelity | exact-IO subcheck pass rate (contains_exact/yaml/json/bullet/word subsets) | check-*.json per-subcheck | judge.py |
| M3 | Reasoning depth | max hop-count solved per case (cases tagged hops:1..4) | case metadata + pass | judge.py |
| M4 | Robustness | negation/distractor/false-premise case pass rate (cases tagged robust:true) | case metadata + pass | judge.py |
| M5 | Instruction priority | override-compliance rate (cases tagged prio:true: later instruction contradicts earlier) | case metadata + pass | judge.py |
| M6 | Stability | seed variance: pass@3, σ of per-seed accuracy | ≥3 run dirs same config | judge.py |
| M7 | Latency | median + p90 clean wall per case | metrics.json duration (rc=0 only) | judge.py |
| M8 | Efficiency | model loads, load-time share, generated-token proxy (output chars), max VRAM class loaded | run logs | judge.py |

## Case tagging (authoring requirement)

Every stage-2 case YAML carries:

```yaml
rea_plus:
  hops: 3            # reasoning depth tag (M3)
  robust: true       # participates in M4
  prio: false        # participates in M5
  format_weight: 2   # how many of its subchecks are format-class (M2)
```

judge.py fails loudly if tags missing — no silent metric gaps.

## Reporting contract

Per iteration (07-TRACKING.md entry):

```
| vPn | target | M1 | M2 | M3 | M4 | M5 | M6(σ) | M7 med | M8 loads | verdict |
```

Rules:
- Target metric MUST improve vs previous green, else iteration red
- Any metric regressing >10% relative = blocking regression, fix before
  claiming target win (ratchet across dimensions)
- n=1 cells marked `*`; M6 requires n≥3 before any M6-targeted version
  can go green

## What this fixes from the critique

- Old suite: single lens (pass/fail on format-heavy checks)
- b39 bug class: M1 requires external truth; checks derived from
  reference fixes that themselves derive from public-bench answers
- "faster?" unanswerable before: M7/M8 measured per run, clean-only
