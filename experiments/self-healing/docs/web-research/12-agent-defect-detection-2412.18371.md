# Source: LLM Agent Defect Detection (2412.18371) + Microsoft Agentic Failure Taxonomy

- **URLs:**
  - https://arxiv.org/abs/2412.18371 (Agentable defect detection)
  - Microsoft "Taxonomy of Failure Modes in Agentic AI Systems" whitepaper
- **Retrieved:** 2026-08-24 (websearch)

## Defect Detection (2412.18371)

- 8 agent defect types mined from 6,854 StackOverflow posts: ADAL, IETI, LOPE, TRE, ALS, MNFT, LARD, EPDD
- Code Property Graph (LLM-free static analysis) + LLM semantic enrichment
- Precision 88.79%, recall 91.03%
- Most common: LARD (LLM API defects) 34%, EPDD (dependency conflicts) 28%

## Microsoft Taxonomy

- Novel agentic failure modes: agent injection, impersonation, provisioning poisoning
- Heightened classic modes: hallucination, bias
- Security (CIA loss) vs Safety (RAI violations) split
- Risk = Likelihood × Impact (5×5)

## What We Borrow

1. **Static/dynamic split** — their CPG static analysis ≈ our pre-flight structural validation (validate-workflow.py checks before execution); dynamic detector suite runs during workflow. Two-phase detection already matches repo discipline
2. **LARD dominance (34% API defects)** — justifies F2 execution-error as first-class failure class with dedicated tool-reselect heal, not generic retry
3. **Severity × likelihood framing** — case suite includes probability weighting notes (primary paper 30% injection; our cases cover uniform coverage first, probability-weighted suite later)
4. **Security failure note** — out of scope v1 (sandbox/permissions engine concern), tracked as deferred

## Divergence

Their targets are source-code defects in agent apps; ours are runtime output failures. Borrowed structure only.
