# Self-Healing Experiment — Case Suite Spec

**Version:** v1 | Cases: `cases/probe/` | Format: YAML, consumed by agent_run.py + report.py

## Case Schema

```yaml
case_id: f1-01                    # unique
task:                             # the "agent task" being attempted
  id: cache-sweep-report
  prompt: |                       # real prompt text (Phase L uses verbatim)
    ...task core + REPORT SPEC...
  expected_schema: [status, evicted, retained]   # required JSON keys
failure:
  class: F1                       # F1|F2|F3|F4|none
  fail_attempts: [1]              # attempts emitting failed output (deterministic injection)
expected:                          # ORACLE — report.py asserts against actual
  final: pass                     # pass|fail
  path: [attempt_1, classify_1, heal_f1_1, attempt_2, classify_2, accept, report]
  attempts_to_success: 2          # null when final=fail
```

## Spoofed Output Contract (agent_run.py emits per attempt)

```json
{
  "attempt": 1,
  "text": "<report body — valid JSON on clean attempts>",
  "confidence": 0.42,
  "psc": 0.33,
  "tool_calls": [{"tool": "cache_api", "status": "ok"}],
  "reasoning_markers": [],
  "upstream_errors": []
}
```

Per-class injection signatures (what detectors see):

| Class | text | confidence | psc | tool_calls | upstream_errors |
|-------|------|-----------|-----|------------|-----------------|
| clean | valid JSON, schema-complete | 0.90 | 0.90 | all ok | [] |
| F1 | JSON valid but contains `source_verified: true` marker (unsupported claim) + one schema field hallucinated value | 0.42 | 0.55 | ok | [] |
| F2 | field missing (schema violation) | 0.75 | 0.70 | 1+ `status: error` | [] |
| F3 | JSON valid, reasoning_markers contains contradiction pair `["hot>ceiling", "hot<=ceiling"]` | 0.60 | 0.33 | ok | [] |
| F4 | partial JSON + truncated field | 0.55 | 0.60 | ok | [{"step": "upstream_digest", "error": "timeout"}] |

## v1 Probe Suite (6 cases)

| Case | class | fail_attempts | expected.final | attempts_to_success | Tests |
|------|-------|---------------|----------------|--------------------|-------|
| case-clean-01 | none | [] | pass | 1 | no false positive; accept at attempt 1; no heal entered |
| case-f1-01 | F1 | [1] | pass | 2 | hallucination detect + corrective-prompt route |
| case-f2-01 | F2 | [1] | pass | 2 | tool-error detect + tool-reselect route |
| case-f3-01 | F3 | [1] | pass | 2 | contradiction detect + replan route |
| case-f4-01 | F4 | [1,2] | pass | 3 | propagation detect + replan ×2 (bounded chain depth) |
| case-f1-persist-01 | F1 | [1,2,3] | fail | null | budget exhaustion → clean fail, RSR denominator |

Suite expectations: TSR=5/6, FDA=1.0 (5 failure attempts all correctly classified... f1-persist adds 3 failure attempts → total failure attempts = 1+1+1+2+3 = 8, all classified → FDA 8/8), RSR = 4/5 detected-failure-cases recovered (persist case not recovered).

## Detector → Class Priority (classify.py, deterministic)

```
if R >= θ: clean
else priority: upstream_errors→F4, tool_errors→F2, contradiction→F3, hallucination_marker/low-conf→F1, default→F3
```

Rationale: F4 = structural dependency failure dominates (paper: propagation); F2 hard signal (doc 12 LARD); F3 default catch (paper treats inconsistency as residual reasoning failure).

## Future Suites (post-v1)

- probability-weighted injection (paper 30%) via random seed in generator
- multi-failure cases (F2 then F1 on retry — class migration)
- regression bucket under cases/regression/ per REA+ convention
