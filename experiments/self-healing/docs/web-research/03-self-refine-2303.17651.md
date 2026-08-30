# Source: Self-Refine — Iterative Refinement with Self-Feedback

- **URL:** https://arxiv.org/abs/2303.17651
- **Retrieved:** 2026-08-24 (abstract + websearch)

## Method Mechanics

Same LLM plays 3 roles: Generator → Feedback → Refiner. Loop:

```
output = generate(task)
for k in 1..K:
    fb = feedback(task, output)       # actionable, localized
    if fb says "output looks good": break
    output = refine(task, output, fb)
```

- **K:** fixed 3–4 iterations typical
- **Feedback quality gate:** feedback must be specific + reference the flaw, else refinement degrades

## Healing Trigger

Self-feedback indicates flaw. Stop when feedback signals acceptable OR K exhausted.

## Metrics

~20% absolute improvement across 7 tasks (dialogue, code, sentiment, reasoning).

## What We Borrow

1. **k=3 iteration cap** — empirically the knee; aligns with Reflexion max_trials and our 3-attempt budget
2. **Feedback must be structured** — our heal records carry: failed_signal, classification, corrective_instruction (not free text) — spoofed now, real prompts later
3. **"Looks good" early exit** — classify.py exit 0 = accept path = early termination of heal chain (clean case never enters heal steps)

## Divergence

Self-Refine feedback = same model self-critique (weak for 4B models — harness experiment confirmed weak self-judging). We use deterministic detectors as feedback source instead; model only fills corrective slots.
