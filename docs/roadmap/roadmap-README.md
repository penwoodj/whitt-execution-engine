# Roadmap

Architecture Decision Records (ADRs), research plans, and governance documents defining project roadmap and execution order. Master index ADR-0000 establishes the roadmap structure.

## Roadmap Index

- adr-0000-roadmap-index.yml - Master index linking all ADRs and research plans with execution order, dependency mapping, and quality gates

## Architecture Decision Records (ADR-0001 through ADR-0008)

- adr-0001-foundation-compiler-contract.yml - Foundation phase: schema types, IR compilation, policy compilation, storage
- adr-0002-mvp-queue-scheduler-safety.yml - MVP Queue & Scheduler: chat sessions, queue state machine, human-gated safety
- adr-0003-cli-backends-networking-boundary.yml - CLI & Backends: provider abstraction, tool permissions, networking boundary
- adr-0004-glyphnova-ui-control-plane.yml - Glyphnova UI: desktop shell, queue visualization, shared API
- adr-0005-quality-loops-benchmarks-artifact-workflows.yml - Quality Loops: generate-verify-repair, benchmarks, artifact workflows
- adr-0006-memory-search-scraping.yml - Memory & Search: local memory, hybrid search, web scraping
- adr-0007-cron-git-refinement.yml - Automation: cron scheduling, git experiments, merge proposals
- adr-0008-autonomy-and-metrics.yml - Autonomy: bounded loops, objective metrics, dashboards

## Research Plans

- research-plan-01-foundation.yml - Foundation phase research plan
- research-plan-02-mvp-queue-cli.yml - MVP queue and CLI research plan
- research-plan-03-networking-ui-backends.yml - Networking, UI, and backends research plan
- research-plan-04-quality-memory-search.yml - Quality, memory, and search research plan
- research-plan-05-automation-autonomy-metrics.yml - Automation, autonomy, and metrics research plan

## Supporting Directories

- metadata/ - ADR metadata and version history
- research/ - Additional research documents and findings

## Related

- [Schema Reference](../requirements/unifying-schema/unified-workflow-schema.yml) - Single source of truth for workflow structure
- [Master Plan](../../plans/INDEX.md) - Detailed implementation plans linked to ADRs
- [Requirements](../requirements/) - All requirements documents and example workflows
