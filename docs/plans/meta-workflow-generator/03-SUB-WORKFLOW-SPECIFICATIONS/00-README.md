# 03 — Sub-Workflow Specifications

This directory contains one detailed specification per sub-workflow. Each spec is the
single source of truth for that sub-workflow's YAML design, prompts, hooks, and
acceptance criteria.

## Index

| # | File | SW | Output | Purpose |
|---|------|----|--------|---------|
| 1 | [01-task-deconstruction.md](./01-task-deconstruction.md) | SW1 | `tasks.md` | Prompt → atomic tasks with story-point complexity |
| 2 | [02-desired-output-state.md](./02-desired-output-state.md) | SW2 | `outputs.md` | Tasks → testable desired end-states |
| 3 | [03-agentic-categorization.md](./03-agentic-categorization.md) | SW3 | `categories.md` | Tasks → correlative agentic behavior categories |
| 4 | [04-yaml-substructure-translation.md](./04-yaml-substructure-translation.md) | SW4 | `structs.md` | Categories → YAML substructure intent (YAML blocks ONLY) |
| 5 | [05-final-workflow-assembly.md](./05-final-workflow-assembly.md) | SW5 | `workflow.yml` | Substructures → executable YAML workflow |

## Specification Template

Each sub-workflow spec follows this structure:

1. **Identity** — name, output file, predecessor inputs, successor consumers
2. **Pipeline Stages** — ordered list of step purposes
3. **Step Specifications** — one section per step with: keys present, prompt skeleton,
   hook wiring, model_overrides, retry budget
4. **Iteration Strategy** — what `iterate_values` chunks over, chunk sizing
5. **Quality Gates (GWT)** — criteria for evaluation step, fix-step routing
6. **Output Schema** — exact format of the single .md/.yml output file
7. **Acceptance Criteria** — what "done" looks like for this SW
8. **Failure Modes** — known risks and mitigations
9. **Baseline Reference** — where the Sisyphus manual baseline lives
10. **Iteration Log** — pointer to `08-ITERATION-LOG.md` entries

## Common Conventions

- All steps use `generative_entity: "${models.qwen35}"`.
- `model_overrides` set `temperature` (0.05–0.3) and `max_tokens` (200–4000) per step role.
- All file paths use `./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/` prefix.
- All log paths use `./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log`.
- All steps have `after_step_succeeds: [save_to OR append_to, log]` and `after_step_fails: [log error]`.
- `depends_on` is mandatory for every step except `step_00_bootstrap`.
