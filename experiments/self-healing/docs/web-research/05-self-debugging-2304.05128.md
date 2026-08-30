# Source: Teaching Large Language Models to Self-Debug (Self-Debugging)

- **URL:** https://arxiv.org/abs/2304.05128
- **Retrieved:** 2026-08-24 (abstract + websearch)

## Method Mechanics

Rubber-duck debugging for code agents:

```
code = generate(problem)
for turn in 1..10:
    explanation = explain(code)          # code explains itself line-by-line
    feedback = execute_and_check(code)   # unit tests / expected output diff
    if feedback clean: break
    code = fix(code, explanation, feedback)
```

- **Max 10 debugging turns** (upper bound; most converge <4)
- Feedback types ranked by power: unit tests > execution output > simple textual diff > explanation-only

## Healing Trigger

Execution result mismatch vs expected output, or test failure.

## Metrics

+2–3% Spider baseline; +12% with unit-test feedback (TransCoder/MBPP). Feedback richness dominates.

## What We Borrow

1. **Feedback richness ladder** — our detection.json carries multiple signal types (schema, refusal, tool errors, contradictions) = richer feedback → better heal decisions
2. **Explain-before-fix pattern** — heal steps in live phase will require model to restate failed signal + rule before patching (spoofed heal records already carry failed_signal field)
3. **Turn cap 10 as hard ceiling** — our 3 attempts is well inside; document as conservative bound per Self-Refine/Reflexion convergence data

## Divergence

Code-domain specific. Our v1 cases are JSON-report tasks, not code gen. F2 execution-error cases simulate tool call failures instead of compile errors.
