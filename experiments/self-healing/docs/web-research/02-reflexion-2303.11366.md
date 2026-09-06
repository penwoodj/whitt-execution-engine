# Source: Reflexion — Language Agents with Verbal Reinforcement Learning

- **URL:** https://arxiv.org/abs/2303.11366
- **Retrieved:** 2026-08-24 (abstract + websearch)

## Method Mechanics

Agent loop: Actor generates → Evaluator scores → self-reflection stored in episodic memory → next trial. Verbal feedback replaces gradient updates.

## Core Loop (concrete)

```
while evaluator not pass AND t < max_trials:
    trajectory = actor(env)
    score = evaluator(trajectory)
    reflection = self_reflect(trajectory, score)   # verbal, specific, actionable
    memory.add(reflection)                          # bounded Ω, FIFO/LRU eviction
```

- **max_trials:** typically 3–5
- **Memory bound:** Ω = 1–3 reflections kept (evict oldest)

## Healing Trigger

Evaluator failure feedback (unit test fail, heuristic check fail). NOT reliability-score based — binary pass/fail.

## Stop Condition

Evaluator pass OR trial budget exhausted.

## Metrics

- Pass@1 accuracy gains (HumanEval 91.0% w/ GPT-4 vs 80.1% base)
- Trial reduction over episodes (memory transfer effect)

## What We Borrow

1. **Bounded retry (3 attempts)** — matches our max 3 attempts, also matches Self-Refine k=3 finding
2. **Reflection artifact between attempts** — our heal.py writes heal record consumed by next attempt (spoofed "corrective context")
3. **Strategy escalation across trials** — v9 harness fix_1→fix_2→fix_3 temperature ladder = same principle; our heal chain f1→attempt_2→f1'→attempt_3 escalates
4. **Evict old reflections** — keep only last heal record per case (no unbounded context)

## Divergence

Reflexion uses LLM-generated reflection; our spoofed phase uses scripted heal records. Live phase (later, user-gated) would use real corrective prompting per primary paper F1 strategy.
