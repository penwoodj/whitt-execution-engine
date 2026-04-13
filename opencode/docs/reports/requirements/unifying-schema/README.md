# Unifying Schema

Unified workflow schema documentation and requirements consolidation. The unified-workflow-schema.yml (801 lines) is the single source of truth for workflow structure and semantics.

## Schema Documents

- unified-workflow-schema.yml - Source of truth schema (801 lines, 19 domains, 325 fields)
- unified-schema-requirements.md - All requirements from schema simplification planning (13 requirement categories)
- schema-consolidated-report.md - Schema consolidation report documenting unification process
- schema-unification-plan.md - Unification plan and methodology
- manual-first-schema-unification-plan.md - Manual-first unification approach
- example-workflow-differences.md - Examples of workflow variations and differences

## Schema Structure

**19 Domains** organized by ownership phase:
1. Workflow Identification (Phase 0)
2. Model Configuration (Phase 0, 3)
3. Agentic Workflow (Phase 1)
4. Pipeline Definition (Phase 1)
5. Workflow Execution Strategy (Phase 1, 2, 4)
6. Tool Permissions (Phase 2, 5, 6)
7. Logging Configuration (Phase 1)
8. Metrics Configuration (Phase 1, 7)
9. Validation Configuration (Phase 1)
10. Orchestration Configuration (Phase 1, 2)
11. Provider Configuration (Phase 3)
12. RAG Configuration (Phase 2, 5)
13. Workspace Configuration (Phase 0)
14. Features Demonstrated (Phase 0 - documentation only)
15. Variable Interpolation (Phase 0, 1)
16. Default Behavior Reference (Phase 0, 1)
17. Scope & Inheritance Rules (Phase 0, 1, 2)
18. Numeric Threshold Guidance (Phase 0, 1)
19. Duplicate Configuration Systems (Phase 1, 2)

## Key Features

- Natural language key names for human readability
- Presence-based configuration (no `enabled:` keys)
- Step type inference from keys present (no `type:` property)
- Unified `when` hook system with natural language timing
- Five loop types with minimum constraint resolution
- Three-level semantic hierarchy (workflow → agentic_workflow → step)

## Related

- [Schema Simplification Plan](../../plans/unified-schema-simplification/plan.md) - 12-phase simplification execution
- [Schema-to-Phase Matrix](../../plans/traceability/schema-to-phase-matrix.md) - Domain ownership mapping
- [Roadmap ADRs](../roadmap/) - Architecture decisions driving schema design
