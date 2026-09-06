# Web Research Index

> Harness-inspired experiment. Compiled 2026-08-23. 5 librarian research threads.

| Doc | Topic | Key Takeaways |
|-----|-------|---------------|
| 01-smart-strategies.md | Uncommon high-leverage harness strategies | Adopt: trace replay, fresh-context verify, early-exit cascade, leak-safe feedback. Skip: STV/Weaver (need training), budget forcing (no think tokens) |
| 02-judge-leak-safety.md | LLM-judge + leak-safety patterns | Binary criteria judges; feedback = location+observed, never expected; repair@k plateaus at 2; det-first gating = 95% cost cut |
| 03-product-harnesses.md | 10 product harness mechanics | Steal: append-only replayable logs, fresh-context children, reflection cap 2-3, auto-review gates |
| 04-weak-model-principles.md | 20 weak-model meta-principles | Deterministic offloading; reason-free-constrain-late; no self-consistency voting; role-isolated history-free correction |

## Synthesis → v3 Design

1. Det gate (json_exact) = primary verdict; judge = secondary blind signal
2. FIX x2 cap (repair@2 plateau; Aider reflection=3)
3. fix_2 = history-free from-scratch solve (role isolation P19, Cursor Agent+)
4. Feedback = check name + category + model's observed value; NEVER expected (F8)
5. Judge = 3 binary sub-checks then verdict (F3)
6. Reason-first prompts, JSON last (P8), no constrained decoding (P9)
7. Trace = append-only replayable ledger (OpenHands, AgentAssay)
8. Early-exit: det-PASS skips fix cascade entirely (FrugalGPT, F10)
