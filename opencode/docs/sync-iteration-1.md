# Schema Sync Iteration 1

**Date**: 2026-04-02
**Status**: In Progress
**Objective**: Fix all sync issues found in initial analysis across examples, ADRs, plan files, and READMEs

---

## Pre-Fix Audit Findings

### Example Workflows (16 files with issues)

| # | File | Issues Found |
|---|------|-------------|
| 1 | ex01 | ✅ CLEAN - Reference standard |
| 2 | ex02 | `file_operations:` at wrong indent (line 231-235, inside prompt block), `analysis:` at wrong indent (line 285), `improvements:`/`analysis:` at wrong indent (lines 323-324) |
| 3 | ex03 | No `agentic_workflow:` section — different format (AgentSDK agent config). May not need conversion. |
| 4 | ex04 | `agentic_workflow:` at 2-indent (should be root), `steps:` at 2-indent peer of agentic_workflow (should be child), step_1 missing step name key, duplicate `name:` key (lines 113-114), duplicate `allow:` key (lines 228-229), `file_operations:` at wrong indent |
| 5 | ex05 | TODO comments left (lines 81, 119), `previous_score:`/`previous_changes:` at wrong indent (lines 117-118) inside prompt block |
| 6 | ex06 | Need to check |
| 7 | ex07 | Need to check |
| 8 | ex08 | Need to check |
| 9 | ex09 | Need to check |
| 10 | ex10 | Need to check |
| 11 | ex11 | Still has `pipeline:` at line 499 |
| 12 | ex12 | Need to check |
| 13 | ex13 | Need to check |
| 14 | ex14 | Need to check |
| 15 | ex15 | Need to check |
| 16 | ex16 | Need to check |
| 17 | ex17 | Need to check |
| 18 | ex18 | Need to check |
| 19 | ex18-additional | Fragment file (sub-workflow refs only) — no `agentic_workflow:` expected |

### Plan Files (3 with stale terminology)

| # | File | Issues |
|---|------|--------|
| 1 | manual-first-schema-unification-plan.md | `pipeline: - step:` references, old property paths |
| 2 | schema-unification-plan.md | Says examples use `pipeline` — now use `agentic_workflow` |
| 3 | requirements-coverage-analysis.md | References `pipeline` section, old property paths |
| 4 | unified-schema-feature-verification.md | 100% coverage claim may be stale |

### ADR Files (6 missing schema_reference)

| # | File | Status |
|---|------|--------|
| 1 | adr-0002-mvp-queue-scheduler-safety.yml | ❌ Missing schema_reference |
| 2 | adr-0003-cli-backends-networking-boundary.yml | ❌ Missing schema_reference |
| 3 | adr-0004-glyphnova-ui-control-plane.yml | ❌ Missing schema_reference |
| 4 | adr-0005-quality-loops-benchmarks-artifact-workflows.yml | ❌ Missing schema_reference |
| 5 | adr-0006-memory-search-scraping.yml | ❌ Missing schema_reference |
| 6 | adr-0007-cron-git-refinement.yml | ❌ Missing schema_reference |
| 7 | adr-0008-autonomy-and-metrics.yml | ❌ Missing schema_reference |

### README Files (2 with stale examples)

| # | File | Issues |
|---|------|--------|
| 1 | transpiler/README.md | `pipeline:` YAML examples in Quick Start and Schema Examples |
| 2 | root README.md | `pipeline:` YAML in examples |

---

## Fix Log

### 1. Example Workflows
- [ ] ex02: Fix indentation
- [ ] ex04: Fix structure (re-indent, remove duplicates)
- [ ] ex05: Fix indentation, remove TODOs
- [ ] ex06-ex10: Check and fix
- [ ] ex11: Remove stray `pipeline:` reference
- [ ] ex12-ex18: Check and fix
- [ ] ex03: Determine if conversion needed
- [ ] ex18-additional: Determine if changes needed

### 2. Plan Files
- [ ] manual-first-schema-unification-plan.md
- [ ] schema-unification-plan.md
- [ ] requirements-coverage-analysis.md
- [ ] unified-schema-feature-verification.md

### 3. ADR Files
- [ ] adr-0002 through adr-0008: Add schema_reference

### 4. README Files
- [ ] transpiler/README.md
- [ ] root README.md

---

## Post-Fix Audit (will be filled after all fixes)
_TODO_
