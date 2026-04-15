# Unified Schema Simplification

Schema simplification plan executed in 12 phases, reducing unified-workflow-schema.yml from 1705 lines to 801 lines while preserving all features. Complete as of commit ac0a645.

## Simplification Documents

- plan.md - Complete 12-phase simplification plan with before/after examples and validation criteria

## Simplification Objectives

1. **Human-readable** - Natural language key names, skimmable structure, non-technical accessibility
2. **Extensible** - Easy to add new features, backwards-compatible design
3. **Simple** - Deduplicated structure, holdable in memory, logical cohesion
4. **Feature preservation** - All existing schema capabilities maintained

## 12 Execution Phases

1. Remove Pipeline + Replace Output with When Hooks
2. Consolidate Loops + Step Type Inference (no `type` property)
3. Introduce `when` / `lifecycle_hooks` Schema
4. Remove Advanced Scheduling
5. Remove Adaptive Behavior (moved to model router)
6. Tool Permissions + Absorb AI Operations
7. Remove .glyphnova References + Simplify Refs
8. Flatten Output Fields
9. Documentation Cleanup + Thresholds
10. Move RAG Under Memory + Remove `enabled` Keys
11. Preserved Sections (No Changes)
12. Final Assembly and Validation

## Key Changes

- Removed `pipeline:` array format, using only `agentic_workflow:` with named steps
- Replaced `output:` section with `when` hook actions (save_to, log, etc.)
- Removed `type:` property from steps (type inferred from keys present)
- Consolidated all 5 loop types under `loops:` with named subkeys
- Unified hook system with `when:` / `lifecycle_hooks` aliases
- Removed `enabled:` keys (presence = enabled)
- Absorbed `ai_operations:` into `tool_permissions`

## Related

- [Unified Schema v2.0](../../reports/requirements/unifying-schema/unified-workflow-schema.yml) - Simplified schema (801 lines)
- [Schema Requirements](../../reports/requirements/unifying-schema/unified-schema-requirements.md) - All simplification requirements
