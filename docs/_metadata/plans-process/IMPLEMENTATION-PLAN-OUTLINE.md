# Implementation Plan Suite — Updated Outline

> Aligned with unified schema v2.0 (801 lines). This outline reflects all schema changes, documentation alignment, and plan phase updates as of 2026-04-11.

---

## Schema Context

| Item | Value |
|------|-------|
| Schema version | 2.0.0 |
| Schema location | `reports/requirements/unifying-schema/unified-workflow-schema.yml` |
| Schema size | 801 lines (down from 1705) |
| Example workflows | 52 v2-compliant YAMLs + 2 archived v1 artifacts |
| ADRs | 9 (adr-0000 through adr-0008) |
| Plan phases | 10 (00-foundation through 08-final-validation + deep-research) |

---

## Phase Overview

### Phase 0 — Foundation (`plans/00-foundation/`)
**Scope**: Cargo project, error module, schema types, YAML parser, workflow spec, IR compiler, DAG validator, policy compiler, variable interpolation, defaults/scope, local storage.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/`
- `input_variables` → `inputs`
- `allocation` → `ram_allocation`
- `enabled: true` → presence-based
- `pipeline` → `agentic_workflow`
- `ai_operations` → `tool_permissions`
- `framework:` removed from model definitions

**Key tasks**: 12 tasks + integration/unit/property tests + acceptance criteria + checkpoint criteria

### Phase 1 — Core Execution Engine (`plans/01-core-execution-engine/`)
**Scope**: Persistent queue storage, priority queue, round-robin/fair-share scheduler, queue persistence, cancellation, timeout.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/`

**Key tasks**: Queue storage, scheduler, priority management, persistence

### Phase 2 — CLI & LLM Backend Integration (`plans/02-cli-and-llm-backend-integration/`)
**Scope**: CLI foundation, LLM backend trait, provider backends (LM Studio, Ollama, llama.cpp, OpenAI), tool permissions, tool execution, sub-workflow execution, code generation, RAG integration, self-improvement loop.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/`
- `enabled` → presence-based
- `input_variables` → `inputs`
- NOTE: Rust code examples still use `use glyphnova::` crate imports (requires actual code rename)

**Key tasks**: 13 tasks covering full provider abstraction layer + validation criteria

### Phase 3 — Desktop UI (`plans/03-glyphnova-ui/`)
**Scope**: Desktop shell setup, shared backend API, multi-zoom control plane.

**v2 schema alignment status**: ⚠️ Partial
- Directory name still references "glyphnova" (cosmetic — rename pending)
- File content updated where applicable
- NOTE: This is UI work — schema alignment less critical here

**Key tasks**: Shell setup, backend API, control plane components

### Phase 4 — Quality Loops (`plans/04-quality-loops/`)
**Scope**: Generate-verify-repair loops, benchmark harness, quality scoring, regression testing.

**v2 schema alignment status**: ✅ Updated
- `enabled` → presence-based in benchmark examples

**Key tasks**: Quality loop implementation, benchmark harness, scoring

### Phase 5 — Memory & Search (`plans/05-memory-search/`)
**Scope**: Local memory storage, fulltext search, semantic search, search query engine, external search adapters, web scraping, provenance tracking, garbage collection, integration.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/` across all 8 validation files + tasks
- `schema_version: 1.0` → needs update to 2.0 in task file

**Key tasks**: 9 tasks + 8 validation files with mock/test specifications + legacy test archive

### Phase 6 — Automation (`plans/06-automation/`)
**Scope**: Cron scheduler, merge proposal generation, manual refinement capture, experiment result tracking, rollback/cleanup.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/` across all 5 task files
- `enabled` → presence-based

**Key tasks**: 5 tasks covering git automation workflow

### Phase 7 — Autonomy & Metrics (`plans/07-autonomy-metrics/`)
**Scope**: Metrics collection, intervention tracking, autonomous decision-making, observability.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/`
- `enabled` → presence-based
- Mock metrics collector updated

**Key tasks**: 3 tasks + validation with mock specs

### Phase 8 — Final Validation (`plans/08-final-validation/`)
**Scope**: End-to-end validation, 100-model benchmarking, workflow generation tests, edge case coverage.

**v2 schema alignment status**: ✅ Updated

**Key tasks**: Comprehensive validation suite

### Deep Research (`plans/deep-research/`)
**Scope**: YAML schema research, Rust ecosystem research, LLM backend research, agent patterns research, testing strategy research, performance research.

**v2 schema alignment status**: ✅ Updated
- `.glyphnova/` → `./workspace/` across all 7 research files

**Key tasks**: 7 research documents with evidence collection methodology

---

## Cross-Cutting Documentation

### Validation Criteria (`plans/validation-criteria/`)
- phase-00-criteria.md: ✅ Updated (.glyphnova → workspace)
- phase-01-criteria.md: ✅ Updated (pipeline → agentic_workflow, step type inference comments)
- phase-02-criteria.md: ✅ Updated (step type inference comments)
- phase-03-criteria.md: ✅ Updated

### Traceability (`plans/traceability/`)
- requirements-traceability.md: ✅ Updated (.glyphnova → workspace)
- adr-to-phase-matrix.md: ✅ Updated (.glyphnova → workspace)
- schema-to-phase-matrix.md: ⚠️ Needs input_variables → inputs, ai_operations → tool_permissions

### Transpiler (`plans/transpiler/`)
- transpiler_implementation_plan.yml: ⚠️ Has .glyphnova reference (1 occurrence)
- transpiler_architecture.md: ✅ Updated (pipeline reference remains in historical context)

---

## Known Remaining Gaps

| Gap | Location | Priority | Status |
|-----|----------|----------|--------|
| Rust crate name `whitt::` | `plans/02-cli-and-llm-backend-integration/tasks/` (15+ files) | LOW | Requires actual code rename |
| Directory `03-glyphnova-ui/` | `plans/03-glyphnova-ui/` | LOW | Historical, phase removed |
| ADR-0004 filename | `reports/roadmap/adr-0004-glyphnova-ui-control-plane.yml` | LOW | Title + filename rename pending |
| `schema_version: 1.0` | `plans/05-memory-search/tasks/00-local-memory-storage.md` | MEDIUM | Pending |
| `input_variables` in traceability | `plans/traceability/schema-to-phase-matrix.md` | MEDIUM | Pending |
| `input_variables` in benchmarks | `reports/requirements/benchmark-100-model-userflows/` (3 files) | MEDIUM | Pending |
| `1705` in simplification plan | `plans/unified-schema-simplification/plan.md` | NONE | Historical record, skip |

---

## Implementation Dependencies

```
00-foundation ──→ 01-core-execution-engine ──→ 02-cli-and-llm-backend-integration ──→ 04-quality-loops
                                                      ↓
                               05-memory-search ──→ 06-automation ──→ 07-autonomy-metrics
                                                                        ↓
                                                               08-final-validation
```

---

## Files Changed in v2 Alignment (2026-04-11)

| Category | Files | Key Changes |
|----------|-------|-------------|
| Index files | 3 | Version 2.0.0, 801 lines, workspace paths |
| ADRs | 9 + 2 research reports | .glyphnova → workspace, schema paths |
| Plan phases (00-08) | 50+ | .glyphnova, enabled, input_variables, pipeline, allocation |
| Validation criteria | 4 | pipeline → agentic_workflow, type inference |
| Traceability | 3 | .glyphnova → workspace |
| Deep research | 7 | .glyphnova → workspace |
| Example workflows | 52 YAMLs | input_variables, features_demonstrated, enabled, pipeline, framework |
| Test specs | 7 legacy files | Archived to legacy/ subfolder |
| **Total** | **~130 files** | **1,488 v1 references addressed** |
