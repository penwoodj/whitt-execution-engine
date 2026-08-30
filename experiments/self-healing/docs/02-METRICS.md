# Self-Healing Experiment — Metrics

**Version:** v1 | **Blueprint metrics:** TSR, FDA, RSR (paper 2605.06737)

## Primary Metrics (per case + suite aggregate in report.json)

### TSR — Task Success Rate
```
TSR = cases ending in accept / total cases
```
Phase S expected: 5/6 = 0.833 (f-persist designed to fail). Phase L target: ≥ paper's qualitative claim of significant increase vs basic retry (exact paper numbers unavailable — see research doc 01 limitations).

### FDA — Failure Detection Accuracy
```
FDA = (failures detected AND correctly classified) / (injected failure attempts)
```
Denominator = sum over cases of len(fail_attempts). Phase S expected: 1.0 (deterministic). Phase L bar: κ ≥ 0.77 vs ground truth (MAST LLM-judge bar, research doc 07). Also tracked: false accepts (clean classified as failure) and false rejects (failure classified clean) — both must be 0.

### RSR — Recovery Success Rate
```
RSR = detected failures recovered by attempt 3 / detected failures
```
Phase S expected: 4/5 = 0.8 (f-persist unrecoverable by design).

## Secondary Metrics

| Metric | Definition | Source |
|--------|-----------|--------|
| Attempts-to-success | per recovered failure | H4 |
| R per attempt | R = ω₁C+ω₂S+ω₃E values | H3 calibration log |
| Heal-strategy correctness | routed strategy == class-mapped strategy | H1/H2 |
| Propagation containment | F4 case downstream steps NOT executed after upstream fail detected | paper "reduces failure propagation" |
| Hook fire count | every expected hook fires once per step visit | engine discipline |
| Inference event count | MUST be 0 in Phase S | HARD gate |

## Metric Emission Points

- `trace.jsonl` — one event per significant action (attempt, detect, classify, heal, accept, fail, report) with full context. Report validates sequence vs case `expected.path`.
- `report.json` — final per-case: {case_id, outcome, attempts_used, classification_per_attempt, R_per_attempt, metrics} + suite aggregate when run across all cases.
- report.py exit code = oracle: 0 iff outcome + path + classifications + R-gate all match expected. Non-zero → workflow routes to fail-end step (GWT on exit code).

## Thresholds Register (single source of truth in code: sh_lib.py)

| Name | Value | Rationale |
|------|-------|-----------|
| θ (reliability) | 0.65 | paper value |
| ω₁, ω₂, ω₃ | 0.35, 0.35, 0.30 | initial split; C and S weighted per paper emphasis on consistency+semantics; tunable later |
| MAX_ATTEMPTS | 3 | docs 02–05 convergence (k=3 knee) |
| CONFIDENCE_FLOOR | 0.50 | below → F1 signal contributes |
| PSC_UNCERTAIN | 0.30 | self-consistency uncertainty flag (doc 08) |
| LENGTH_BOUNDS | [50, 5000] chars | report-task sanity |
