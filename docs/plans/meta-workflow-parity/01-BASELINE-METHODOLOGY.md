# 01 - BASELINE METHODOLOGY

## WHAT IS A "BASELINE"

Opencode attempt = I (Sisyphus agent with full tool access) execute each prompt. Save:
- Files read
- Files written/modified  
- Commands run (cargo, bash)
- Final output artifacts
- Self-evaluation: did prompt objective get met?

## EXECUTION RULES

1. **No meta-v6 involvement** — pure opencode, no shortcuts
2. **Max tool usage** — use Read, Write, Edit, Bash, Grep freely
3. **Time-bound** — ~5min per prompt, no perfectionism
4. **Honest evaluation** — even if attempt fails, document honestly
5. **Save artifacts** — under `baselines/opencode/prompt-{N}/`

## OUTPUT STRUCTURE PER PROMPT

```
baselines/opencode/prompt-{N}/
├── attempt.md           # what I did, step by step
├── outputs/             # actual produced artifacts (code, docs, etc)
├── verification.log     # commands run + results
└── evaluation.md        # brutal honest self-eval
```

## EVALUATION DIMENSIONS

For each baseline attempt, score 0-5 on:

| Dimension | Question |
|-----------|----------|
| **Completeness** | All prompt requirements addressed? |
| **Correctness** | Code/doc actually works/valid? |
| **Depth** | Surface-level OR substantive? |
| **Verification** | Tested OR hand-waved? |
| **Honesty** | Limitations documented? |

Score ≥4/5 across all = baseline qualifies as "quality bar".

## WHAT BASELINE IS NOT

- NOT the meta-v6 SW1 task breakdown (those exist for prompt-14, 15)
- NOT the SW5 generated workflow (those are what we're testing)
- NOT a "best possible" answer (time-boxed at 5min)

## 11 PROMPTS TO BASELINE

| # | Prompt ID | Domain | Estimated Complexity |
|---|-----------|--------|---------------------|
| 05 | task.md | TBD | TBD |
| 06 | weasyprint pdf | Code gen | Med |
| 07 | task.md | TBD | TBD |
| 08 | full-path run issue | Debug | Low |
| 09 | phase 2.5 components | Code gen | Med |
| 10 | reports/initial | Research | High |
| 11 | reports/initial (dup?) | Research | High |
| 12 | skill instruction | Doc gen | Med |
| 13 | task.md | TBD | TBD |
| 14 | parallel inference joinset | Code gen | High |
| 15 | agent react layer rust | Code gen | High |

## SEQUENCING

Start with prompt-14 (parallel inference) — most concrete, has prior SW1 baseline. Establish methodology. Then propagate to others.

End with research-heavy prompts (10, 11) — those need broader tool usage (web fetch).
