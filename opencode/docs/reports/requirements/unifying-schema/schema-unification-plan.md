# Schema Unification Plan
## Resolving Differences Between Manual Schema and Example Workflows

**Date**: 2026-03-30
**Status**: Analysis Phase → Planning Phase
**Objective**: Create a unified, readable schema that maintains ALL AgentSDK features and ALL requirements

---

## 1. Executive Summary

### Current State
- **Manual Schema** (`manual/agentic-workflow-manual-brainstorm.yml`): 1,254 lines, highly detailed but uses custom structure
- **Example Workflows** (all 52 categorized examples): 18 files using different naming conventions, structures, and feature combinations
- **Requirements Coverage**: 22 original requirements, 25+ documented feature categories

### Core Problem
> Example schemas aren't very readable and use inconsistent patterns across files. Manual schema has better structure but may not align with examples.

### Solution Approach
**Hybrid Unification**: Extract readability improvements from manual, feature patterns from examples, and consolidate into a unified schema specification.

---

## 2. Structural Differences Analysis

### 2.1 Root-Level Organization

| Aspect | Manual Schema | Example Workflows | Gap Analysis |
|--------|---------------|-------------------|----------------|
| **Top-level sections** | `agentic_workflow`, `workflow_execution_strategy`, `models`, `pipeline`, `providers` | `models`, `execution`, `logging`, `pipeline`, `workflow_registry`, `orchestration` | Manual uses custom `workflow_execution_strategy` that maps to examples' `execution` |
| **Models section** | Under root, with `allowed_processors`, `max_allowed`, `min_allowed`, `attention_tokens` | Under root, simpler: `provider`, `model`, `backend`, `gpu_layers` | Manual adds detailed resource constraints (RAM%, VRAM%, CPU%) |
| **Execution section** | Nested under `workflow_execution_strategy` with sub-objects (load_unload, memory_pressure_handling, processing, parallel, timeout, error_handling, sub_workflow, synchronization, checkpoint) | Flat under `execution`: `mode`, `memory`, `max_parallel`, `max_parallel_steps`, `timeout`, `concurrency_limits` | Manual is more hierarchical with 10 sub-sections; examples are flatter |
| **Logging** | Not shown in manual (cut off at line 1254) | Separate section with `global`, `scopes`, `output_type`, `format` | Manual has `agentic_workflow.hardcoded_values` for interpolation |

**Key Difference**: Manual uses nested `workflow_execution_strategy.*` while examples use flat `execution.*` structure.

---

### 2.2 Model Configuration Differences

| Feature | Manual Schema | Example Workflows | Resolution Strategy |
|----------|---------------|-------------------|---------------------|
| **Model reference** | `${models.primary_analyzer}` | `${model}`, `"llama-3.2-3b-instruct"` | Manual uses variable names; examples use direct model strings |
| **Host/provider** | `host: lmstudio` (enum) | `provider: lmstudio|ollama|llamacpp` | Manual's `host` ≈ examples' `provider` |
| **Processor selection** | `allowed_processors: "(cpu_main \| cpu_alt_1) & gpu_cluster2_brady"` (logical expression) | `backend: vulkan|cuda|cpu|metal` | Manual's processor logic is more sophisticated |
| **Memory limits** | `max_allowed.ram: 13%`, `max_allowed.vram: 3.7GB`, `min_allowed.ram: 9%` | `execution.memory.max_allocated_memory_mb`, `model_memory_mb` | Manual uses percentage-based limits; examples use absolute MB |
| **Token limits** | `attention_tokens: 150000` (max), `min_allowed.attention_tokens: 73500` | Not shown explicitly (implied via model) | Manual adds explicit token budgeting |
| **Concurrent requests** | `concurrent_requests: 2` | Not shown | Manual adds concurrency limits per model |
| **Cache size** | `model_memory_cache_size: "min"` (enum: min|medium|max) | `execution.memory.load_unload_strategy` | Manual's cache enum ≈ examples' load/unload strategies |
| **Thinking budget** | `thinking.budget_tokens: 4096` | Not shown | Manual adds reasoning token budget |
| **Guards** | `guards.enabled: true` (Note: v2.0 uses presence=enabled convention), `guards.enforcement_policy: block` | Not shown | Manual adds input/output guards |

**Resolution**: Merge both approaches - keep examples' simple structure, add manual's detailed constraints as optional fields.

---

### 2.3 Execution Strategy Differences

| Feature | Manual Schema | Example Workflows | Gap Analysis |
|----------|---------------|-------------------|----------------|
| **Processing strategy** | `workflow_execution_strategy.processing: parallel` | `execution.mode: parallel|serial|hybrid` | **1:1 mapping** - same concept, different path |
| **Load/unload strategy** | `load_unload: one_at_a_time` | `execution.memory.load_unload_strategy` | Same concept, nested differently |
| **Memory pressure** | `memory_pressure_handling: throttle` | `adaptive.memory_pressure_handling` | Manual has this at top level; examples nest under `adaptive` |
| **Parallel config** | `parallel.algorithm: round_robin`, `parallel.max_threads: 4`, `parallel.max_models: 3`, `parallel.max_concurrent_requests: 2`, `parallel.max_steps: 2` | `execution.max_parallel`, `execution.parallel_group`, `max_parallel_steps` | Manual has 5 parallel config fields; examples have 3 |
| **Timeout** | `timeout.total: 4h`, `timeout.tool_call: 30m`, `timeout.step: 8m`, `timeout.model.load_into_memory: 45s`, `timeout.model.time_to_processing_after_loaded: 10000ms` | `execution.timeout_secs`, `step.timeout_secs`, `tool_execution.timeout_secs` | Manual has granular timeout types; examples have fewer |
| **Error handling** | `error_handling.default_action: retry`, `error_handling.max_retries_per_step: 3`, `error_handling.retry_backoff_multiplier: 2.0` | `retry.default.max_attempts`, `retry.backoff_strategy` | Manual's `error_handling` ≈ examples' `retry` but with more control |
| **Sub-workflow** | `sub_workflow.inherit_policy`, `sub_workflow.override_policy`, `sub_workflow.reference_resolution`, `sub_workflow.isolated_environments` | `execution.sub_workflow_isolation.inherit_policy`, `execution.allow_nested_references`, `execution.circular_reference_detection` | Manual splits sub-workflow config; examples combine into execution section |
| **Synchronization** | `synchronization.enabled: true` (Note: v2.0 uses presence=enabled convention), `synchronization.barrier_sync_enabled: false`, `synchronization.lock_free_timeout_secs: 30`, `synchronization.conflict_resolution: last_writer_wins` | Not shown explicitly (implied in nested workflows) | Manual adds explicit sync control |
| **Checkpoint** | `checkpoint.level: timetravel`, `checkpoint.output: "./chat-file-...md"`, `checkpoint.timeback: 5h`, `checkpoint.max_size: 1000lns`, `checkpoint.interval.after_time: 5m`, `checkpoint.interval.after_steps: [...]` | `checkpoint.enabled`, `checkpoint.interval_steps` | Manual has richer checkpoint config (time travel, max size, time-based intervals) |

**Resolution**: Manual's `workflow_execution_strategy` should replace flat `execution` structure, or both should coexist with migration path.

---

### 2.4 Pipeline/Step Structure Differences

| Feature | Manual Schema | Example Workflows | Gap Analysis |
|----------|---------------|-------------------|----------------|
| **Step definition** | `agentic_workflow.steps.initialize_system` (nested under `agentic_workflow`) | `agentic_workflow: - step: initialize_system` (flat array, legacy: `pipeline:`) | Manual nests steps under `agentic_workflow`; examples use flat array |
| **Agent/model reference** | `generative_entity: "${models.primary_analyzer}"` | `model: "${models.primary}"` | Manual uses `generative_entity`; examples use `model` |
| **Prompt passing** | Nested as multi-line string under `prompt: \|` | Same structure | **Compatible** |
| **Output schema** | `output.save_to: system_output`, `output.format: json`, `output.fields: [...]` | Same structure | **Compatible** |
| **Retry per step** | `retry.condition: "..."`, `retry.max_attempts: 3`, `retry.level: full_workflow_unload_reload` | Same structure, manual adds `level` enum | Manual adds retry scope control |
| **Branching** | `branches: [{ branch_id, next_step, enabled_by, description }]` | Not shown in all examples (some have `validation_loop`, some have simple flow) | Manual has explicit `branches` structure; examples use `validation_loop` and `improvement_loop` |
| **Parallel groups** | `parallel_group: sub_agent_execution`, `max_parallel: 4` | `parallel_group: analysis_phase` (some examples) | **Compatible** |
| **Sub-agent spawning** | `sub_agents: [{ agent_id, workflow_ref, validation_config, interdependent_validations }]` | Not shown (examples have `sub_agents` as separate entity, not per step) | Manual nests sub-agent config in step; examples define `sub_agents` at root |
| **Event handling** | `event_handling.on_validation_result`, `event_handling.on_interdependent_failure`, `event_handling.on_checkpoint` | Not shown (implied in orchestration sections) | Manual adds explicit event hooks |
| **Orchestration config** | `orchestration_config: "${orchestration}"` | Not shown in steps (separate `orchestration` section) | Manual references top-level orchestration |

**Resolution**: Keep examples' flat `agentic_workflow` structure for compatibility (legacy: `pipeline`), add manual's advanced features (branches, sub_agents in steps, event_handling) as optional extensions.

---

### 2.5 Logging Differences

| Feature | Manual Schema | Example Workflows | Gap Analysis |
|----------|---------------|-------------------|----------------|
| **Logging location** | Not shown (cut off) | `logging: { global, scopes, output_type, format, console, log_file }` | Manual has no logging shown; examples have full logging config |
| **Scopes** | Not shown | `logging.scopes: { execution, memory, tool, validation, agent, state_management }` | Examples have hierarchical scopes; manual may need to add this |
| **Hierarchical output** | Not shown | `logging_parameters: { workflow_level, step_level, tool_level, agent_level }` (13-logging-monitoring/01-hierarchical-logging-system.yaml, 19-comprehensive-integration/01-complex-orchestration-sub-agents.yaml) | Some examples have nested logging parameters |

**Resolution**: Adopt examples' logging structure fully, as manual doesn't complete this section.

---

### 2.6 Hooks and Interpolation System

| Feature | Manual Schema | Example Workflows | Gap Analysis |
|----------|---------------|-------------------|----------------|
| **Hardcoded values** | `agentic_workflow.hardcoded_values: { var_asdf, var_fdsa }` | Not shown | Manual adds interpolation variable system |
| **Framework variables** | Detailed list: `workflow`, `run`, `now`, `now_iso8601`, `now_epoch_ms`, `random_float`, `task`, `error`, `result`, `turn`, `tool.call`, `tool.result`, `chunk`, `stream_response`, `duration_ms`, `validation` | Not shown in examples (implicit) | Manual documents all available interpolation variables |
| **Operators** | `comparison: [==, !=, >, >=, <, <=]`, `logical: [&&, ||, !]`, `string: [contains, starts_with, ends_with, matches, length]`, `collection: [in, not_in, empty, not_empty, length, all, any, none, has]`, `type: [is_null, is_not_null, is_type]`, `existence: [defined, undefined]`, `grouping: [(), ()]`, `arithmetic: [*, /, %, +, -]`, `get: [.]` | Not shown | Manual defines full operator system for interpolation expressions |
| **Property mappings** | `hook_property_access_mapping: { on_create, on_run_start, on_run_complete, on_run_error, on_turn_start, on_turn_complete, on_turn_error, on_thinking, on_tool_call, on_tool_start, on_tool_result, on_tool_error, on_stream_chunk, on_stream_complete }` | Not shown | Manual defines which variables are available in which hooks |
| **Agent hooks** | `hooks.on_agent_create, on_run_start, on_run_complete, on_run_error, on_turn_start, on_turn_complete, on_turn_error, on_thinking, on_memory_recall, on_memory_store, on_stream_chunk, on_stream_complete` | Not shown | Manual defines agent lifecycle hooks |

**Resolution**: Manual's interpolation and hooks system is a **major feature addition** not present in examples. Must be integrated as optional/advanced feature.

---

### 2.7 User Inputs

| Feature | Manual Schema | Example Workflows | Gap Analysis |
|----------|---------------|-------------------|----------------|
| **User inputs** | `agentic_workflow.user_inputs.steps: { ... }` | Not shown | Manual adds comprehensive user input system with text, multi-select, tabs, confirm/cancel |
| **Modular UI support** | Text area, vertical UI lines, labeled buttons, multiple choice, multi-select, navigation tabs, panels, categorization | Not shown | Manual adds rich UI component library for user interactions |

**Resolution**: Manual's user input system is another **major feature addition** not in examples. Add as optional/advanced feature.

---

## 3. Missing/Extra Features Analysis

### 3.1 Features in Manual but NOT in Examples

| Feature | Description | Impact | Priority |
|----------|-------------|--------|----------|
| **Processor selection logic** | `allowed_processors: "(cpu_main | cpu_alt_1) & gpu_cluster2_brady"` with logical operators | Sophisticated compute resource allocation | HIGH |
| **Percentage-based memory limits** | `max_allowed.ram: 13%`, `max_allowed.vram: 3.7GB` | Dynamic resource allocation based on system specs | HIGH |
| **Explicit token budgeting** | `attention_tokens`, `thinking.budget_tokens`, `model_memory_cache_size` | Predictable resource usage, prevent OOM | HIGH |
| **Granular timeout types** | `timeout.tool_call`, `timeout.step`, `timeout.model.load_into_memory`, `timeout.model.time_to_processing_after_loaded` | Fine-grained timeout control | MEDIUM |
| **Retry scope control** | `retry.level: full_workflow_unload_reload | workflow_unload | workflow_restart | step_unload | step_restart | prompt_restart | tools_retry` | Control retry granularity | MEDIUM |
| **Explicit branching** | `branches: [{ branch_id, next_step, enabled_by, description }]` | Alternative execution paths with conditions | MEDIUM |
| **Sub-agent config per step** | `sub_agents` nested in step with `validation_config`, `interdependent_validations` | Dynamic sub-agent spawning with validation | HIGH |
| **Event handling hooks** | `event_handling.on_validation_result`, `on_interdependent_failure`, `on_checkpoint` | Reactive workflow behavior | MEDIUM |
| **Advanced checkpointing** | `checkpoint.level: time_travel`, `checkpoint.timeback: 5h`, `checkpoint.max_size: 1000lns` | Time travel checkpoints, size limits | MEDIUM |
| **Synchronization primitives** | `synchronization.barrier_sync_enabled`, `lock_free_timeout_secs`, `conflict_resolution` | Coordination for parallel workflows | LOW |
| **Hooks system** | Full agent lifecycle hooks + variable mapping | Extensibility, custom behaviors | HIGH |
| **Interpolation operators** | Full operator set (comparison, logical, string, collection, type, arithmetic, grouping) | Rich conditional logic in expressions | HIGH |
| **Framework variables** | 50+ predefined variables (workflow, run, now, task, error, result, turn, tool, chunk, stream, duration_ms, validation) | Comprehensive context access | HIGH |
| **User input system** | Rich UI components (text, multi-select, tabs, panels, confirm/cancel) | Interactive workflows | MEDIUM |
| **Thinking capture** | `thinking.capture_in_output`, `capture_in_events`, `stream_to_log` | Reasoning transparency | MEDIUM |
| **Guards** | Input/output guards with enforcement policies | Safety, PII redaction, format validation | MEDIUM |

**Total**: 15 major features in manual not present in examples

---

### 3.2 Features in Examples but NOT in Manual

| Feature | Description | Impact | Priority |
|----------|-------------|--------|----------|
| **Complete logging section** | `logging.global`, `logging.scopes`, `logging_parameters` (13-logging-monitoring/01-hierarchical-logging-system.yaml, 19-comprehensive-integration/01-complex-orchestration-sub-agents.yaml) | Full hierarchical logging | CRITICAL |
| **Nested logging hierarchy** | 8-9 levels: global → workflow → step → agent → tool → file → API → state → performance | Fine-grained observability | HIGH |
| **RAG configuration** | `rag.enabled`, `rag.backend`, `rag.config` (08-rag-operations/01-document-indexing-retrieval.yaml, 08-rag-operations/02-rag-generation-context-aware.yaml, 19-comprehensive-integration/01-complex-orchestration-sub-agents.yaml) | Knowledge base management | HIGH |
| **RAG operations** | add, query, update, delete with filters (08-rag-operations/01-document-indexing-retrieval.yaml) | Knowledge base CRUD | HIGH |
| **Web operations with URL params** | `web_fetch`, `web_scrape`, `web_search` with full URL parameters (07-web-operations/03-url-parameters-requests.yaml) | External data access | MEDIUM |
| **Script and CLI execution** | `script_run`, `cli_run` with env_vars, working_dir, timeout (09-script-cli/01-script-execution.yaml) | System integration | MEDIUM |
| **File operations** | Explicit `file_operations` with read, write, delete, backup, archive (06-file-operations/01-file-read-write-batch.yaml) | Local file management | HIGH |
| **Loop variations** | count, time, infinite, validation (exact/abstract), retry (05-loops-convergence/01-for-loops-explicit-iteration.yaml) | Diverse loop types | HIGH |
| **Web scraping to RAG** | End-to-end web scrape → embed → RAG add pipeline (08-rag-operations/02-rag-generation-context-aware.yaml) | Knowledge base automation | MEDIUM |
| **Prompt refinement procedures** | Multi-step prompt improvement loop (03-data-flow/01-workflow-level-variables.yaml) | Quality improvement | MEDIUM |
| **Workflow registry** | `workflow_registry.registered_workflows` with discovery (10-sub-workflows/01-nested-workflow-references.yaml) | Dynamic workflow loading | MEDIUM |
| **Workflow references** | 5 reference patterns (direct_import, inline_reference, registry_lookup, nested_execution, conditional_reference) (10-sub-workflows/01-nested-workflow-references.yaml) | Nested workflow orchestration | HIGH |
| **Explicit CRUD operations** | `file_operations` with explicit operations (06-file-operations/01-file-read-write-batch.yaml) | Predictable file I/O | MEDIUM |
| **Orchestration configuration** | `orchestration.sub_agent_relationships`, `interdependent_validations` (10-sub-workflows/01-nested-workflow-references.yaml, 17-user-inputs-ui/01-user-input-prompts-validation.yaml) | Complex coordination | HIGH |
| **Metrics collection** | `metrics.enabled`, `metrics.collect`, `metrics.output` (most examples) | Performance tracking | MEDIUM |
| **Retry configuration** | `retry.default`, `retry.sub_agent_specific`, `retry.escalation` (01-model-configuration/03-model-lifecycle-management.yaml, 05-loops-convergence/01-for-loops-explicit-iteration.yaml, 03-data-flow/01-workflow-level-variables.yaml) | Flexible retry strategies | MEDIUM |
| **Permission systems** | `permissions.global`, `permissions.agent_specific`, `tool_permissions`, `folder_permissions` (01-model-configuration/04-cost-tracking-budgets.yaml, 06-file-operations/01-file-read-write-batch.yaml) | Security and access control | HIGH |
| **State management** | `state.save_interval_secs`, `state.checkpoint_interval_steps`, `state.state_file_path` (06-file-operations/01-file-read-write-batch.yaml) | Persistence and recovery | MEDIUM |

**Total**: 18 major features in examples not present in manual

---

## 4. Readability Issues in Example Schemas

### 4.1 Inconsistency Issues

| Issue | Description | Example Impact | Solution |
|--------|-------------|-----------------|----------|
| **Inconsistent section ordering** | `models`, `execution`, `logging` appear in different orders across examples | Hard to find sections | Standardize section order |
| **Inconsistent property names** | `model_memory_mb` vs `model.memory_mb`, `max_parallel_steps` vs `max_parallel` | Confusing references | Create canonical property name mapping |
| **Inconsistent nesting** | Some properties nested under `execution`, some at root level | Navigation difficult | Establish clear nesting hierarchy |
| **Inconsistent loop structures** | `validation_loop` vs `improvement_loop` vs `convergence_loop` | Pattern mismatch | Unify loop structure |
| **Inconsistent branch handling** | Some use `branches`, some use `validation_loop.stop_conditions` | Mixed patterns | Standardize branching mechanism |

### 4.2 Verbosity Issues

| Issue | Description | Example Impact | Solution |
|--------|-------------|-----------------|----------|
| **Redundant fields** | Same information repeated across sections (e.g., retry config in step AND at root) | Bloat, maintenance burden | Deduplicate with inheritance |
| **Overly verbose values** | Long descriptive strings in `description` fields | Hard to scan | Keep descriptions concise |
| **Missing defaults** | Required fields don't have sensible defaults | Boilerplate required | Provide sensible defaults in schema |
| **Flat structure for complex features** | All properties at same level, no logical grouping | Cognitive overload | Use nested objects for related properties |
| **No comments or hints** | Schema doesn't explain what fields do | Learning curve | Add inline documentation |

### 4.3 Completeness Issues

| Issue | Description | Example Impact | Solution |
|--------|-------------|-----------------|----------|
| **Optional features mixed with required** | Can't tell which fields are optional | Confusion | Mark required/optional clearly |
| **No validation rules** | Schema doesn't specify constraints (e.g., must be positive integer) | Invalid workflows | Add validation schema |
| **No deprecation guidance** | Old patterns not marked | Mixed old/new usage | Document deprecated vs current |
| **No example values** | Hard to understand expected format | Trial-and-error | Add example values in comments |

---

## 5. Proposed Resolution Strategy

### 5.1 Unification Approach: Three-Layer Schema

**Layer 1: Core Schema (Required for all workflows)**
- Flat, readable structure based on examples
- Essential properties only
- Clear required/optional markings
- Example values in comments

**Layer 2: Extended Schema (Optional advanced features from manual)**
- Manual's advanced features (hooks, interpolation operators, user inputs, guards)
- Can be layered on top of Core Schema
- Optional extensions via feature flags

**Layer 3: Legacy/Migration Layer**
- Deprecated patterns from examples (e.g., old loop structures)
- Migration paths from old to new
- Backward compatibility mode

### 5.2 Section Ordering Standardization

**Standard Order**:
1. `workflow_id`, `name`, `description` (metadata)
2. `models` (model configuration)
3. `providers` (provider defaults and overrides)
4. `execution` (execution strategy)
5. `logging` (logging configuration)
6. `pipeline` (workflow steps)
7. `validation` (top-level validation)
8. `retry` (global retry config)
9. `state_management` (checkpoints and state)
10. `metrics` (performance tracking)
11. `permissions` (access control)
12. `orchestration` (complex coordination)

### 5.3 Property Name Canonicalization

| Manual Name | Example Name | Canonical Name | Rationale |
|--------------|---------------|-----------------|-----------|
| `workflow_execution_strategy` | `execution` | `execution` | Shorter, already used in examples |
| `processing` | `mode` | `processing_mode` | Avoids mode keyword conflict |
| `allowed_processors` | `backend` | `processor_selection` | More descriptive |
| `max_allowed` | `memory.max_allocated_memory_mb` | `resource_limits` | Groups related limits |
| `memory_pressure_handling` | `adaptive.memory_pressure_handling` | `memory_handling` | Flattens path |
| `load_unload` | `memory.load_unload_strategy` | `model_lifecycle` | Groups model lifecycle |
| `sub_workflow.*` | `execution.sub_workflow_isolation.*` | `sub_workflow` | Flattens under execution |
| `checkpoint.level` | `checkpoint.enabled` | `checkpoint.mode` | More descriptive |
| `generative_entity` | `model` | `model_or_agent` | Covers both cases |

### 5.4 Readability Improvements

**Improvement 1: Logical Grouping**
- Group related properties under nested objects
- Use 2-3 levels maximum depth
- Group by concern (memory, parallelization, timeouts)

**Improvement 2: Default Values**
- Provide sensible defaults for all optional fields
- Use `_default` suffix for clarity
- Document what default means

**Improvement 3: Validation Annotations**
- Add type hints in comments: `# string | number | boolean`
- Add value constraints: `# min: 0, max: 100`
- Add allowed values: `# allowed: [parallel, serial, hybrid]`

**Improvement 4: Inline Documentation**
- Brief purpose for each section
- Example values for complex fields
- References to related documentation

**Improvement 5: Example-Simple-to-Complex Progression**
- Start with minimal workflow (required fields only)
- Add features incrementally
- Show multiple complete examples

---

## 6. Implementation Roadmap

### Phase 1: Schema Definition (Week 1-2)

**Objective**: Define unified core schema specification

**Tasks**:
1. [ ] Create unified schema specification document
   - Section ordering standardization
   - Property name canonicalization
   - Required/optional markings
   - Default values
   - Validation rules

2. [ ] Define Layer 1 (Core Schema)
   - All features from examples
   - Readability improvements applied
   - Missing from manual: interpolation, hooks, user inputs

3. [ ] Define Layer 2 (Extended Schema)
   - Manual's advanced features
   - Optional extensions
   - Feature flags

4. [ ] Define Layer 3 (Migration Layer)
   - Deprecated patterns
   - Migration paths
   - Backward compatibility

**Deliverables**:
- `schema/core-schema-specification.md`
- `schema/extended-schema-specification.md`
- `schema/migration-guide.md`

---

### Phase 2: Schema Validation (Week 2-3)

**Objective**: Validate unified schema against all requirements

**Tasks**:
1. [ ] Review against 22 original requirements
   - Mark each as covered/not covered
   - Document gaps
   - Update requirements if needed

2. [ ] Review against manual schema features
   - Verify all 15 features preserved
   - Check compatibility
   - Document changes needed

3. [ ] Review against all 53 categorized workflow examples
   - Verify all examples work with core schema
   - Document required changes
   - Create migration scripts if needed

4. [ ] Create validation test suite
   - Automated schema validation
   - Round-trip tests (YAML → parse → YAML)
   - Feature coverage tests

**Deliverables**:
- `schema/requirements-coverage-validation.md`
- `schema/validation-test-suite.yml`
- `examples/migration-status-report.md`

---

### Phase 3: Example Workflow Updates (Week 3-4)

**Objective**: Update all examples to use unified core schema

**Tasks**:
1. [ ] Update all 52 categorized examples to core schema
   - Apply property name changes
   - Restructure sections
   - Add missing logging

2. [ ] Update all 52 categorized examples to core schema
   - Apply property name changes
   - Restructure sections
   - Fix nested workflows

3. [ ] Update all 52 categorized examples to core schema
   - Apply property name changes
   - Restructure sections
   - Add missing features

4. [ ] Validate all updated examples
   - Schema validation
   - Round-trip tests
   - Execution tests (if possible)

**Deliverables**:
- Updated `all 52 categorized examples.yml` files
- `examples/update-summary-report.md`
- `examples/validation-results.md`

---

### Phase 4: Documentation Generation (Week 4-5)

**Objective**: Create comprehensive, readable documentation

**Tasks**:
1. [ ] Write schema reference documentation
   - Complete property catalog
   - Examples for each property
   - Best practices
   - Common patterns

2. [ ] Create quick start guide
   - Minimal workflow example
   - Step-by-step tutorial
   - Common workflows patterns

3. [ ] Write migration guide
   - From examples to unified schema
   - From manual schema to unified schema
   - Common migration issues and solutions

4. [ ] Create cheat sheet
   - Common properties with defaults
   - Quick reference for core features
   - Performance tuning tips

**Deliverables**:
- `docs/schema-reference.md`
- `docs/quick-start-guide.md`
- `docs/migration-guide.md`
- `docs/cheat-sheet.md`

---

### Phase 5: Tooling Support (Week 5-6)

**Objective**: Create tools to help users work with unified schema

**Tasks**:
1. [ ] Schema validation tool
   - Validate YAML workflow files
   - Check required/optional
   - Suggest fixes for errors

2. [ ] Workflow generator CLI
   - Interactive workflow creation
   - Template-based generation
   - Auto-completion for properties

3. [ ] Migration tool
   - Convert old examples to new schema
   - Convert manual schema to new schema
   - Preserve semantics

4. [ ] Documentation generator
   - Auto-generate docs from schema
   - Keep docs in sync with schema
   - Generate examples from schema

**Deliverables**:
- `tools/schema-validator/`
- `tools/workflow-generator/`
- `tools/migration-tool/`
- `tools/doc-generator/`

---

## 7. Decision Points Requiring User Input

### 7.1 Schema Structure Decision

**Question**: Should we adopt manual's `workflow_execution_strategy` structure or flatten it?

**Options**:
1. **Keep nested structure** (`workflow_execution_strategy.processing`, `workflow_execution_strategy.memory`, etc.)
   - Pros: Organized, groups related concepts
   - Cons: More nesting, longer property paths

2. **Flatten structure** (`execution.processing_mode`, `execution.memory_handling`, `execution.parallelization`)
   - Pros: Simpler, shorter paths
   - Cons: Flat structure, less grouping

3. **Hybrid approach** (Layer 1 flat, Layer 2 nested)
   - Pros: Best of both worlds
   - Cons: More complex to document

**Recommendation**: Option 3 (Hybrid) - Core schema flat for simplicity, Extended schema nested for advanced features.

---

### 7.2 Property Naming Decision

**Question**: Should we canonicalize property names to shortest meaningful name?

**Examples**:
- `processing` vs `processing_mode` vs `execution_mode`
- `memory_pressure_handling` vs `memory_handling` vs `adaptive_memory_handling`
- `load_unload` vs `model_lifecycle` vs `model_loading_strategy`

**Recommendation**: Use canonical name mapping table (see Section 5.3), keep examples using old names for backward compatibility with migration.

---

### 7.3 Backward Compatibility Decision

**Question**: Should we maintain backward compatibility with existing examples?

**Options**:
1. **Break compatibility** - Force migration to new schema
   - Pros: Clean slate, no legacy baggage
   - Cons: High migration cost

2. **Maintain compatibility** - Support both old and new schemas
   - Pros: Gradual migration
   - Cons: Maintenance burden

3. **Deprecation path** - Mark old as deprecated, remove in future
   - Pros: Clear migration path
   - Cons: Transitional complexity

**Recommendation**: Option 3 (Deprecation) - Support both for 1-2 versions, provide migration tools, deprecate old patterns.

---

### 7.4 Features Priority Decision

**Question**: Which manual features should be in Core vs Extended schema?

**Manual features requiring decision**:
1. **Interpolation operators** - Core or Extended?
2. **Hooks system** - Core or Extended?
3. **User input system** - Core or Extended?
4. **Guards** - Core or Extended?
5. **Processor selection logic** - Core or Extended?
6. **Percentage-based memory limits** - Core or Extended?
7. **Retry scope control** - Core or Extended?
8. **Explicit branching** - Core or Extended?

**Recommendation**: Put 1-4 in Extended (complex, rare use cases); 5-8 in Core (resource management is fundamental).

---

## 8. Success Criteria

### 8.1 Schema Quality

- [ ] All 22 original requirements covered
- [ ] All 15 manual features preserved
- [ ] All 53 categorized workflow example patterns supported
- [ ] Required/optional clearly marked
- [ ] Default values provided for all optional fields
- [ ] Validation rules documented

### 8.2 Readability

- [ ] Consistent section ordering across all examples
- [ ] Consistent property naming across all examples
- [ ] Logical grouping of related properties
- [ ] Inline documentation for complex fields
- [ ] Example values for non-obvious fields

### 8.3 Documentation

- [ ] Complete schema reference manual
- [ ] Quick start guide
- [ ] Migration guide from old schemas
- [ ] Best practices guide
- [ ] Troubleshooting guide

### 8.4 Tooling

- [ ] Schema validation tool
- [ ] Workflow generator CLI
- [ ] Migration tool
- [ ] Documentation generator

---

## 9. Open Questions

1. **How should we handle the manual schema's interpolation system?** 
   - Is it a core feature or optional extension?
   - Should it support ALL 50+ framework variables, or a subset?

2. **What should the deprecation timeline be?**
   - How many versions to maintain backward compatibility?
   - When to remove old patterns entirely?

3. **Should we create separate schemas for different use cases?**
   - Simple schema for basic workflows
   - Advanced schema for complex orchestration
   - Or one unified schema with layers?

4. **How should we validate example workflows against unified schema?**
   - Automated tests?
   - Manual review?
   - Both?

5. **What's the priority of this unification work?**
   - Should we pause other work to complete this first?
   - Or work in parallel?

---

## 10. Next Steps

1. **Review this plan** and provide feedback on decision points (Section 7)
2. **Prioritize features** based on your needs (which manual features are most important?)
3. **Approve or modify roadmap** (Section 6) based on priorities
4. **Begin Phase 1** - Schema definition

---

**Status**: Awaiting user feedback on decision points and priorities
