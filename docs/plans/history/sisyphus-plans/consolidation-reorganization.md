# Master Plan: Documentation Reorganization + Schema Updates + Content Migration

**Created**: 2026-04-15
**Status**: IN PROGRESS
**Branch**: initial-creation

---

## Overview

13 tasks across 4 phases. Each phase gets its own commit. Plan file updated after each phase.

---

## Phase 1: Content Migration Out (External Repos)

### Task 1.1: Move model-router content to ~/code/model-router/

**Source**: `opencode/docs/reports/requirements/model-router/` (entire directory)
- `README.md` — comprehensive model router analysis
- `opencode-brainstorming/` — 3 system designs (Python/SQLite, Rust/Sled, Node.js/LevelDB)
- `chatgpt-brainstorming/` — 3-phase progressive rollout reports + rollout plan
- `combined-brainstorming/DECISIONS.md` — synthesis document
- `model_routing_data.csv` — routing data

**Target**: `~/code/model-router/` (repo exists, has chatgpt-ideas/, opencode-ideas/, etc.)
- Move to `~/code/model-router/inspiration-reports/whitt-execution-engine/` to preserve provenance
- Delete from whitt-execution-engine after move

**Additional**: Search entire project for model routing, model override, sub-workflow model override references in schemas/docs. Move those too or annotate with migration notes.

**Commit**: "Move model-router content to ~/code/model-router/"

### Task 1.2: Final sweep for agent-queue/parallelism content

**Already done**: Most parallel refs moved/annotated. Verify:
- `grep -rn "agent.queue\|agent-queue" opencode/docs/` — find any remaining agent-queue references
- Ensure all parallel refs are migration comments only
- Remove any remaining active parallel execution schema fields

**Commit**: (combined with 1.1 or separate if needed)

---

## Phase 2: Schema Updates (Before Reorganization)

### Task 2.1: Remove `processing` key from unified schema

**File**: `unified-workflow-schema.yml` line 494
- Remove `processing: serial  # serial only — parallel moved to agent-queue` entirely
- Processing is serial by definition, no need for the field

### Task 2.2: Add retry subschema keys

**File**: `unified-workflow-schema.yml`
- **Workflow retry** (line ~245): Add `initial_delay: "1s"`, `max_delay: "30s"`, `multiplier: 2.0`, `jitter: true`
- **Step retry** (line ~251): Already has jitter as float (0.2). Add `initial_delay`, `max_delay`, `multiplier` if missing
- **Error handling retry** (line ~530): Already has `base_delay_ms`, `max_delay_ms`, `jitter_factor`. Verify consistency.

**Note**: The workflow-level retry uses `delay_ms`, step retry uses `delay_ms` + `base_ms` + `max_ms` + `jitter`, error_handling uses `base_delay_ms` + `max_delay_ms` + `jitter_factor`. Need to unify naming. User wants: `initial_delay`, `max_delay`, `multiplier`, `jitter`. Align all three locations.

### Task 2.3: Analyze synchronization vs dependency_resolution overlap

**Current schema**:
```yaml
synchronization:
  coordination:
    deadlock_detection: true
    deadlock_resolution_timeout_secs: 120
  conflict_handling:
    resolution_strategy: last_writer_wins
  event_propagation:
    mode: bidirectional

dependency_resolution:
  strategy: wait_for_all
  timeout_secs: 300
  propagate_failure_to_dependents: true
  on_dependency_failure: fail_workflow
```

**Analysis needed**: Both deal with step ordering/waiting. `synchronization.coordination` has deadlock detection (runtime concern for parallel — already moved). `dependency_resolution` is about step ordering via `depends_on`/`requires`. These may overlap.

**Decision**: Report findings to user. The deadlock detection in synchronization is parallel-runtime territory. dependency_resolution is core serial execution concern.

### Task 2.4: Create hooks semantics document

**New file**: `docs/schema/hooks-semantics.md` (after reorganization)
- Document hooks for each schema substructure type:
  - `models` — hooks for model lifecycle (load, unload, fail, timeout)
  - `agents` — hooks for agent spawning, completion, error
  - `providers` — hooks for provider connection, disconnection, rate limit
  - `agent_workflows` — hooks for workflow lifecycle (start, complete, fail, checkpoint)
  - `steps` — hooks varying by step type (before/after/during/abort)
- `skip_on_load_failure` → replaced by hooks
- Include lifecycle hook phases, hook action types, error handling in hooks

### Task 2.5: Create hooks integration plan

**New file**: `docs/schema/hooks-integration-plan.md`
- Plan for adding hooks to unified schema
- Phase-by-phase integration steps
- Downstream doc updates needed

**Commit**: "Schema updates: remove processing, add retry keys, hooks docs"

---

## Phase 3: Documentation Reorganization

### Current Structure:
```
opencode/docs/
  plans/              (18 subdirs)
  reports/
    initial-plans/    (13 items)
    requirements/     (15 items + subdirs)
    roadmap/          (17 items)
  research/           (1 subdir)
  README.md

docs/
  infinite-context/   (14 items)

ROOT FILES:
  DEVELOPER_GUIDE.md
  ENVIRONMENT_VARIABLES.md
  TESTING_GUIDE.md
  RESEARCH_UPSTREAM_SUCCESS_FACTORS.md
  AUTHORS.md, CHANGELOG.md, CODE_OF_CONDUCT.md, CONTRIBUTING.md
  INSTALL.md, LICENSE, README.md
```

### Target Structure:
```
docs/
  schema/
    unified-workflow-schema.yml
    schema-consolidated-report.md
    schema-unification-plan.md
    manual-first-schema-unification-plan.md
    unified-schema-requirements.md
    example-workflow-differences.md
    hooks-semantics.md
    hooks-integration-plan.md
  
  workflows/
    examples/
      requirements-oriented-auto/  (19 category dirs)
      chatgpt/                     (4 files)
      manual/                      (2 files)
    review-cycles/                 (11 review cycle files)
    coverage-analysis.md
    feature-verification.md
    completion-summary.md
    missing-workflows-summary.md
  
  requirements/
    requirements.md
    index.md
    critical-evaluation.md
    advanced-agentic-features.md
    constraints-and-assumptions.md
    configuration-defaults.md
    llamacpp-vulkan-integration.md
    transpiler-feature-matrix-r0054-r0075.md
    benchmark-yaml-examples.md
    compiler-vs-transpiler-analysis.md
    schema-additions-needed.md
    benchmark-100-model-userflows/
  
  plans/
    phases/
      00-foundation/
      01-mvp-queue/
      02-cli-backends/
      03-glyphnova-ui/
      04-quality-loops/
      05-memory-search/
      06-automation/
      07-autonomy-metrics/
      08-final-validation/
    initial-plans/                 (from reports/initial-plans/)
    unified-schema-simplification/
    deep-research/
    transpiler/
    traceability/
    validation-criteria/
    ARCHITECTURE.md
    IMPLEMENTATION-PLAN-OUTLINE.md
    INDEX.md
  
  roadmap/
    adrs/                          (ADR yml files)
    research-plans/                (research-plan-*.yml)
    metadata/
    research/
  
  research/
    upstream-success-factors.md    (from RESEARCH_UPSTREAM_SUCCESS_FACTORS.md)
    infinite-context/              (from docs/infinite-context/)
    next-steps/                    (from opencode/docs/research/next-steps/)
    initial-research/              (from plans/initial-research/)
  
  guides/
    developer-guide.md             (from DEVELOPER_GUIDE.md)
    testing-guide.md               (from TESTING_GUIDE.md)
    environment-variables.md       (from ENVIRONMENT_VARIABLES.md)
    install.md                     (from INSTALL.md)
    contributing.md                (from CONTRIBUTING.md)
  
  project/
    README.md                      (root README stays at root)
    AUTHORS.md                     (stays at root)
    CHANGELOG.md                   (stays at root)
    CODE_OF_CONDUCT.md             (stays at root)
    LICENSE                        (stays at root)
```

### Deduplication Strategy:
1. Compare `plans/` vs `reports/initial-plans/` — likely overlap in phase structure
2. Compare `roadmap/adr-*` vs `plans/ARCHITECTURE.md` — architectural decisions
3. Compare `research/next-steps/` vs `roadmap/research/` — research directions
4. Compare `DEVELOPER_GUIDE.md` vs `TESTING_GUIDE.md` with plan docs for overlap
5. Lossless merge: keep all unique content, annotate duplicates with "see also" references

### Reorganization Steps:
1. Create target directory structure under `docs/`
2. Move files (using git mv for history preservation)
3. Deduplicate during move
4. Update all internal references/links
5. Delete old `opencode/docs/` directory
6. Delete old root-level doc files that moved
7. Verify nothing lost

**Commit**: Multiple commits per logical group

---

## Phase 4: New Plan Files + Downstream Updates

### Task 4.1: Create GitHub Actions improvements plan

**New file**: `docs/plans/github-actions-improvements.md`
- Reference from roadmap after 100-model benchmark
- CI/CD pipeline improvements

### Task 4.2: Create testing strategy plan

**New file**: `docs/plans/testing-strategy.md`
- Multi-layer testing strategy as requirements
- Unit (co-located), Integration (52 YAMLs), Property-based (proptest), Snapshot (insta), Benchmark (criterion), Mock providers (mockall)
- Incremental implementation + verification
- Continuous batch testing

### Task 4.3: Update downstream documentation

After all schema changes and reorganization:
- Update all references in requirements docs
- Update roadmap files to reflect new structure
- Update plan files to reflect new paths
- Verify all cross-references valid

**Commit**: "Add testing strategy, GitHub Actions plans, update downstream docs"

---

## Phase 5: Final Verification

### Task 5.1: File inventory comparison

- List all files before reorganization
- List all files after reorganization
- Verify 1:1 mapping (nothing lost)
- Verify all new files created
- Verify all external repo moves complete

### Task 5.2: Link validation

- grep for old paths in all markdown/yaml files
- Verify all internal links resolve

### Task 5.3: Schema validation

- Verify unified-workflow-schema.yml is valid YAML
- Verify all schema changes applied correctly
- Verify no stale references to moved content

---

## Progress Tracking

| Phase | Task | Status | Commit |
|-------|------|--------|--------|
| 1.1 | Move model-router to ~/code/model-router/ | PENDING | |
| 1.2 | Agent-queue/parallelism final sweep | PENDING | |
| 2.1 | Remove processing key | PENDING | |
| 2.2 | Add retry subschema keys | PENDING | |
| 2.3 | Sync vs dependency analysis | PENDING | |
| 2.4 | Hooks semantics doc | PENDING | |
| 2.5 | Hooks integration plan | PENDING | |
| 3 | Docs reorganization | PENDING | |
| 4.1 | GitHub Actions plan | PENDING | |
| 4.2 | Testing strategy plan | PENDING | |
| 4.3 | Downstream doc updates | PENDING | |
| 5 | Final verification | PENDING | |
