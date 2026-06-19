# Plan History

Archived implementation plans from earlier development phases.

## Directories

- `phase-02/` — Phase 02 implementation plans (CLI & LLM backend integration)
  - CRITICAL-REVIEW.md — Architecture review
  - PLAN-FULL-SCHEMA-COMPLIANCE.md — Schema compliance plan
  - PLAN-YAML-DRIVEN-BENCHMARK.md — YAML-driven benchmark plan

- `sisyphus-plans/` — Agent-generated implementation plans from POC phases
  - bugfix-and-yaml-config-poc.md — Bugfixes + YAML config system
  - cli-and-agentic-cot-poc.md — CLI tool + agentic chain-of-thought
  - consolidation-reorganization.md — Documentation reorganization
  - full-config-integration.md — Full config integration & CLI completion
  - model-chain-poc.md — 3-model workflow chain
  - model-management-poc.md — Model management + hot-swap
  - phase-03-task0-verifier.md — Verifier interface system

- `meta-workflow-generator/` — Original meta-workflow plan suite (Jun 13-14)
  - Predecessor to meta-v6 and meta-workflow-qwen35
  - Replaced by qwen35-specific suite when model was finalized

- `meta-v6/` — Intermediate meta-workflow plan suite (Jun 14-16)
  - Iteration reports (iter1-4), baselines, quality reports
  - Superseded by meta-workflow-qwen35 which subsumed its learnings

- `meta-workflow-qwen35-early-prompts/` — Initial prompt dataset (Jun 17)
  - 15 prompts with INDEX.md, extracted from OpenCode sessions
  - Replaced by `meta-workflow-qwen35/test-prompts/real/` (curated, deduplicated, baselines included)
  - 4 prompts excluded as context dumps, 11 active

## Active Plans

Current active plans are in `docs/plans/`:
- Phases 00-07 (engine implementation)
- `meta-workflow-qwen35/` — current meta-workflow generator plan suite (active)
  - `test-prompts/real/` — curated active prompt dataset with baselines
