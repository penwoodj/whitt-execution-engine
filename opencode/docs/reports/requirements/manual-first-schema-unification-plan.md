# Manual-First Schema Unification Plan

**Date**: 2026-03-30
**Status**: Updated - Manual Design Priority
**Objective**: Preserve manual schema structure while supporting ALL example workflow features and requirements

---

## 1. Design Philosophy: Manual as Foundation

### 1.1 Why Manual Schema Structure is Superior

**Advantages of Manual Design:**
1. **Logical Grouping**: Related concepts grouped under purposeful names
   - `workflow_execution_strategy.processing` → clearer than `execution.mode`
   - `workflow_execution_strategy.memory_pressure_handling` → explicit behavior control

2. **Sophisticated Resource Management**:
   - `allowed_processors: "(cpu_main | cpu_alt_1) & gpu_cluster2_brady"` - logical compute selection
   - `max_allowed.ram: 13%` - percentage-based allocation adapts to system specs
   - `min_allowed.attention_tokens: 73500` - safety floor prevents OOM

3. **Granular Control**:
   - `timeout.tool_call: 30m`, `timeout.step: 8m`, `timeout.model.time_to_processing_after_loaded: 10000ms`
   - `retry.level: full_workflow_unload_reload` - control retry scope
   - `checkpoint.level: timetravel` - advanced checkpoint modes

4. **Extensibility**:
   - `hooks` system for lifecycle customization
   - `interpolation` operators for rich expressions
   - `user_inputs` for interactive workflows
   - `guards` for safety constraints

5. **Clear Semantics**:
   - `generative_entity` vs `model` - distinguishes agent vs simple model
   - `hardcoded_values` for workflow variables
   - `framework variables` with clear mappings

**Manual Schema Strengths:** More expressive, better organized, more sophisticated control than examples.

---

## 2. Mapping: Examples to Manual Schema

### 2.1 Core Mappings

| Example Feature | Manual Schema Location | Notes |
|-----------------|------------------------|-------|
| `execution.mode: parallel` | `workflow_execution_strategy.processing: parallel` | Direct 1:1 mapping |
| `execution.memory.max_allocated_memory_mb` | `workflow_execution_strategy.memory_pressure_handling` + add `max_allocated_memory_mb` | Manual groups memory differently, need to add |
| `execution.memory.model_memory_mb` | Model-specific: `models."name".max_allowed.ram%` | Manual uses percentages, need to support both |
| `execution.memory.load_unload_strategy` | `workflow_execution_strategy.load_unload` | Direct mapping |
| `execution.max_parallel` | `workflow_execution_strategy.parallel.max_steps` | Add if not present |
| `execution.timeout_secs` | `workflow_execution_strategy.timeout.total` | Manual has granular timeouts |
| `retry.default.max_attempts` | `workflow_execution_strategy.error_handling.max_retries_per_step` | Manual nests under error_handling |
| `logging.global.level` | Need to add full logging section to manual | Manual doesn't show this section |
| `pipeline: - step:` | `agentic_workflow.steps: { step_name: ... }` | Manual uses object, examples use array |

### 2.2 Feature Addition Matrix

**Features in Examples but NOT in Manual:**

| Feature Category | Missing Features | Manual Integration Strategy |
|----------------|------------------|--------------------------|
| **Logging** | Complete logging section (9-level hierarchy), scopes, output types | Add `logging` root-level section |
| **RAG** | `rag.enabled`, `rag.backend`, `rag.config`, CRUD operations | Add `rag` root-level section |
| **Web Operations** | URL parameters (headers, auth, timeout, method), scrape, search | Add `web_operations` or integrate into tools |
| **Script/CLI** | `script_run`, `cli_run` with env_vars, working_dir, output capture | Add `shell_operations` or integrate into tools |
| **File Operations** | Explicit `file_operations` (read, write, delete, backup, archive) | Already partially covered, need full schema |
| **Loops** | count, time, infinite loops; validation (exact/abstract); retry loops | `validation_loop` exists, need other types |
| **Workflow Registry** | `workflow_registry.registered_workflows`, discovery, versioning | Already in manual |
| **Workflow References** | 5 patterns (direct_import, inline_reference, registry_lookup, nested_execution, conditional_reference) | Already in manual |
| **Orchestration** | `orchestration.sub_agent_relationships`, `interdependent_validations` | Already in manual |
| **Metrics** | `metrics.enabled`, `metrics.collect`, `metrics.output` | Add `metrics` root-level section |
| **Permissions** | Global, agent-specific, tool permissions, folder permissions | Add `permissions` root-level section |
| **State Management** | `state.save_interval_secs`, `state.state_file_path`, `versioning` | Already partially covered |
| **Tools** | Built-in tools (file, web, shell, grep, yaml_validator) | Need to formalize tools section |
| **Branching** | `branches` structure with `branch_id`, `next_step`, `enabled_by` | Already in manual |
| **Event Handling** | Event propagation, aggregation strategies | Already partially covered |
| **Checkpointing** | Advanced checkpointing (time travel, max size, time-based intervals) | Already in manual |

**Features in Manual but NOT in Examples:**
- All 15 advanced features from previous analysis (hooks, interpolation operators, user_inputs, guards, etc.)

---

## 3. Proposed Schema Structure (Manual-First)

### 3.1 Root-Level Sections (Preserving Manual Order)

```yaml
---
workflow_id: unique_workflow_identifier
name: "Human-Readable Workflow Name"
description: "Workflow purpose and scope"

# ─── Manual Core Sections ─────────────────────
agentic_workflow:
  hardcoded_values: { ... }
  user_inputs:
    steps: { ... }
  hooks:
    # Lifecycle hooks
  steps:
    step_name:
      generative_entity: ...
      model_overrides: { ... }
      prompt: |
        ...
      output: { ... }
      retry: { ... }
      branches: [ ... ]
      parallel_group: ...
      max_parallel: ...
      sub_agents: [ ... ]
      event_handling: { ... }
      orchestration_config: ...
      timeout_secs: ...

workflow_execution_strategy:
  load_unload: one_at_a_time | lazy | eager | adaptive
  memory_pressure_handling: throttle | swap | fail_fast | graceful_degradation
  processing: parallel | serial | hybrid
  parallel:
    algorithm: round_robin | priority_queue | shortest_job_first | longest_job_first
    max_threads: 4
    max_models: 3
    max_concurrent_requests: 2
    max_steps: 2
  timeout:
    total: 4h
    tool_call: 30m
    step: 8m
    model:
      load_into_memory: 45s
      time_to_processing_after_loaded: 10000ms
      time_to_processing: 2h
      time_to_responding: 1m
      time_to_response: 4h
  error_handling:
    default_action: retry | retry_critical_path | skip | fail_workflow
    max_retries_per_step: 3
    retry_backoff_multiplier: 2.0
    escalate_to_model_after: 3
  sub_workflow:
    inherit_policy: { ... }
    override_policy: { ... }
    reference_resolution: { ... }
    isolated_environments: { ... }
  synchronization:
    enabled: true
    barrier_sync_enabled: false
    lock_free_timeout_secs: 30
    conflict_resolution: last_writer_wins | merge_results | fail_on_conflict
  checkpoint:
    level: timetravel | git | debug | info | compressed_summary
    output: "./chat-file-..."
    timeback: 5h
    max_size: 1000lns | 1000tk | 1000gb | 1000mb | 1000kb
    interval:
      after_time: 5m
      after_steps: [step_name1, step_name2]

# ─── Model Configuration (Manual Structure) ─────────────
models:
  global_model_config: "./path/to/config.yml"

  "llama-3.2-3b-instruct":
    name: primary_analyzer  # Variable reference name
    host: lmstudio | ollama | llama_cpp_with_vulkan
    allowed_processors: "(cpu_main | cpu_alt_1) & gpu_cluster2_brady"
    max_allowed:
      ram: 13%
      vram: 3.7GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 150000
      concurrent_requests: 2
    min_allowed:
      ram: 9%
      vram: 2.4GB  # or 24%
      cpu: 30%
      gpu: 50%
      attention_tokens: 73500
    model_memory_cache_size: min | max | medium | medium-min | medium-max
    thinking:
      budget_tokens: 4096
      capture_in_output: true
      capture_in_events: true
      stream_to_log: true
    hooks:
      on_agent_create: ...
      on_run_start: ...
      on_run_complete: ...
      on_run_error: ...
      on_turn_start: ...
      on_turn_complete: ...
      on_turn_error: ...
      on_thinking: ...
      on_memory_recall: ...
      on_memory_store: ...
      on_stream_chunk: ...
      on_stream_complete: ...
    guards:
      enabled: true
      enforcement_policy: block | warn | allow
      input:
        guards: [prompt_injection, pii_redaction, max_length]
      output:
        guards: [toxicity_filter, pii_redaction, format_validation, max_length]

# ─── Provider Defaults (Manual Structure) ─────────────
providers:
  lmstudio: "./path/to/lmstudio-defaults.yml" | { defaults: ... }
  ollama: "./path/to/ollama-defaults.yml" | { defaults: ... }
  llama_cpp_with_vulkan: "./path/to/llamacpp-defaults.yml" | { defaults: ... }

# ─── Missing from Manual (Add from Examples) ─────────────
logging:
  global:
    level: debug | info | warn | error | trace
    detail: low | medium | high | very_high
    output_type: chat | log | stateless_direct_io | mixed
    format: json | text
    console: true
    file: true
    log_file: /workspace/logs/workflow.log
    file_rotation:
      enabled: true
      max_size_mb: 100
      max_files: 10
    include_timestamps: true
    include_level: true
    include_source: true
    include_context: true

  scopes:
    workflow_execution:
      level: info
      detail: medium
      output_type: log
      format: json
      console: false
      file: true
      log_file: /workspace/logs/workflow_execution.log

    step_execution:
      level: debug
      detail: high
      output_type: chat
      format: json
      console: true
      file: true
      log_file: /workspace/logs/step_execution.log

    agent_execution:
      level: info
      detail: medium
      output_type: stateless_direct_io
      format: json
      console: false
      file: true
      log_file: /workspace/logs/agent_execution.log

    tool_execution:
      level: debug
      detail: very_high
      output_type: log
      format: json
      console: true
      file: true
      log_file: /workspace/logs/tool_execution.log

    file_operations:
      level: debug
      detail: very_high
      output_type: stateless_direct_io
      format: json
      console: false
      file: true
      log_file: /workspace/logs/file_operations.log
      include_file_details: true
      include_size_info: true
      include_performance: true

    web_operations:
      level: info
      detail: high
      output_type: chat
      format: json
      console: true
      file: true
      log_file: /workspace/logs/web_operations.log

    api_calls:
      level: info
      detail: medium
      output_type: chat
      format: json
      console: true
      file: true
      log_file: /workspace/logs/api_calls.log
      include_request_details: true
      include_response_details: true
      include_duration: true

    state_management:
      level: debug
      detail: low
      output_type: log
      format: json
      console: false
      file: true
      log_file: /workspace/logs/state_management.log
      include_state_changes: true
      include_memory_usage: false

    performance_metrics:
      level: info
      detail: medium
      output_type: stateless_direct_io
      format: json
      console: false
      file: true
      log_file: /workspace/logs/performance_metrics.log
      include_metrics: true
      include_thresholds: true

  logging_parameters:
    workflow_level:
      level: debug
      detail: high
      output_type: log
      format: json

    step_level:
      level: info
      detail: medium
      output_type: chat
      format: json

    tool_level:
      level: debug
      detail: high
      output_type: stateless_direct_io
      format: json

    agent_level:
      level: info
      detail: medium
      output_type: log
      format: json

# ─── Additional Missing Sections (Add from Examples) ─────────────
rag:
  enabled: true
  backend: faiss | chroma | qdrant | pgvector
  config:
    knowledge_base_path: /workspace/rag/knowledge_base
    document_store: json | sqlite | postgresql
    embedding_store: faiss | pgvector
    metadata_store: json | sqlite
    embedding_dimension: 768
    chunk_size_words: 800
    overlap_words: 100
    versioning:
      enabled: true
      max_versions: 10
    backup:
      enabled: true
      interval_days: 7

tools:
  file:
    read:
      enabled: true
      require_confirmation: false
      allowed_paths: [./src, ./config]
    write:
      enabled: true
      require_confirmation: true
      backup_existing: true
      allowed_paths: [./output, ./workspace]
    delete:
      enabled: true
      require_confirmation: true
      allowed_paths: [./temp, ./cache]
    backup:
      enabled: true
      create_archive: true
      output_path: /workspace/backups
    archive:
      enabled: true
      compression: gzip | tar | zip

  web:
    fetch:
      enabled: true
      timeout_seconds: 30
      follow_redirects: true
      validate_ssl: true
      allow_insecure: false
      authentication:
        type: bearer_token | basic_auth | api_key_header | api_key_query
      caching:
        enabled: true
        cache_ttl_secs: 3600

    scrape:
      enabled: true
      extract_text: true
      extract_links: true
      extract_images: false
      extract_metadata: true
      max_size_bytes: 1048576
      timeout_seconds: 60
      encoding: utf-8
      allow_subsequent_requests: false

    search:
      enabled: true
      engine: google | bing | duckduckgo
      max_results: 10
      safe_search: true
      language: en
      region: us
      time_range: "1year"

  shell:
    exec:
      enabled: true
      require_confirmation: true
      timeout_seconds: 60
      allowed_commands: [cargo, rustc, git]

  yaml_validator:
    enabled: true
    config_path: /schemas/workflow_schema.json

  grep:
    enabled: true

retry:
  default:
    max_attempts: 3
    backoff_strategy: exponential | linear | fixed | none
    on_failure: escalate_model | stop_workflow | manual_intervention

  step_specific:
    step_name:
      max_attempts: 5
      backoff_strategy: exponential
      delay_ms: 1000

  escalation:
    fallback_models:
      - provider: ollama
        model: "llama3.2"
      - provider: llama_cpp_with_vulkan
        model: "/models/llama-3.2-3b.Q4_K_M.gguf"

state_management:
  enabled: true
  save_interval_secs: 60
  checkpoint_interval_steps: 3
  state_file_path: /workspace/.workflow_state/local_file_crud.json

metrics:
  enabled: true
  collect:
    - total_step_duration_seconds
    - total_file_operations
    - total_api_calls
    - total_state_changes
    - total_checkpoints
    - total_errors
    - total_warnings
    - execution_time_seconds
    - validation_iterations
    - retry_count
    - memory_used_mb
    - cpu_usage_percent
    - token_usage
  output:
    path: /workspace/output/workflow_metrics.json
    format: json

permissions:
  global:
    require_confirmation: false
    allowed_operations: [read, write, execute]

  agent_specific:
    agent_id:
      require_confirmation: true
      allowed_operations: [read, write]
      allowed_tools: [grep, file_read]

  tool_permissions:
    file_read:
      require_confirmation: false
      allowed_paths: [./src]
    file_write:
      require_confirmation: true
      allowed_paths: [./output]

  folder_permissions:
    ./config:
      operations: [read]
      require_confirmation: false
    ./output:
      operations: [write]
      require_confirmation: false

orchestration:
  sub_agent_relationships:
    - agent_id: analyzer
      dependencies: []
      dependents: [validator, optimizer]
      interdependent_validations:
        - validation: analysis_quality
          requires:
            - from: validator
              metric: validation_score
              condition: ">= 0.8"

  overall_validation_criteria:
    - criteria: code_quality_score
      description: "Code validation results"
      threshold: 0.9
      weight: 0.3
    - criteria: documentation_completeness
      description: "Doc generation results"
      threshold: 0.85
      weight: 0.25
    - criteria: all_sub_workflows_valid
      description: "All sub-workflows passed"
      threshold: true
      weight: 0.25
    - criteria: performance_acceptable
      description: "Performance metrics"
      threshold: 0.85
      weight: 0.2

workflow_registry:
  registry_path: /workflows/
  discover_mode: auto | explicit
  versioning: true

  registered_workflows:
    - workflow_id: code_analyzer
      name: Code Analyzer Workflow
      path: ./workflows/code_analyzer.yml
      version: 1.0
      tags: [code, analysis, rust]
      metadata:
        author: transpiler_team
        description: Analyzes Rust codebase for patterns
        estimated_duration_secs: 120
        required_context: src/

validation:
  workflow_references:
    - validate_exists: true
      check_registry: true
      check_file_paths: true
    - validate_no_circular_references: true
      max_depth_check: 5

  sub_workflow_execution:
    - validate_policy_compliance: true
      check_scope: true
      check_memory_limits: true
    - validate_timeout_compliance: true
      check_retry_logic: true

  aggregated_results:
    - validate_completeness: true
      required_outputs:
        - code_analysis_results
        - test_results
        - documentation_results
    - validate_consistency: true
      check_for_conflicts: true
    - validate_metrics: true
      track_execution_time: true

metrics:
  enabled: true
  collect:
    - workflow_execution_time
    - sub_workflow_count
    - memory_usage
    - policy_violations
    - reference_resolution_time
  output:
    path: /workspace/output/workflow_performance.json
    format: json

# ─── Pipeline (Manual Structure) ─────────────
# Note: Manual uses `agentic_workflow.steps: { step_name: ... }`
# Examples use `pipeline: [ - step: step_name ... ]`
# Both should be supported for backward compatibility

pipeline:
  # Step structure as shown in manual with optional array syntax
  # Supports all step types: simple, parallel, branching, looping, sub-workflow
  # Includes all advanced features from manual (branches, sub_agents, event_handling)
```

---

## 4. Key Design Decisions

### 4.1 Backward Compatibility Strategy

**Decision:** Support BOTH manual and example syntax for critical sections

**Rationale:**
- Manual uses `agentic_workflow.steps: { step_name: ... }` (object)
- Examples use `pipeline: [ - step: step_name ... ]` (array)
- Both serve the same purpose

**Implementation:**
- Accept both structures
- Normalize internally to unified representation
- Document migration path
- Provide migration tool

### 4.2 Property Naming

**Decision:** Keep manual naming for new features, map example properties

**Canonical Name Mapping:**

| Manual Name | Example Name (Legacy) | Status |
|-------------|------------------------|--------|
| `workflow_execution_strategy` | `execution` | Keep manual |
| `processing` | `mode` | Keep manual, support legacy |
| `load_unload` | `load_unload_strategy` | Keep manual |
| `memory_pressure_handling` | `adaptive.memory_pressure_handling` | Keep manual (shorter) |
| `allowed_processors` | `backend` | Keep manual (more expressive) |
| `generative_entity` | `model` | Keep manual (more precise) |
| `agentic_workflow.steps` | `pipeline` | Support both syntaxes |

### 4.3 Percentage vs Absolute Values

**Decision:** Support BOTH with precedence to manual's percentage system

**Implementation:**
- Parse percentage values: `13%` → convert to GB/MB based on system specs
- Parse absolute values: `3.7GB` → use directly
- Apply system validation: check against `min_allowed` and system capacity
- Provide helpful error messages if limits exceeded

**Validation Rules:**
1. If `max_allowed` (percentage) provided, calculate absolute value
2. Check against `min_allowed` (percentage or absolute)
3. Check against system capacity
4. If `min_allowed > system_capacity`, error with helpful message
5. Suggest alternative models if limits can't be met

### 4.4 Advanced Features as Optional Extensions

**Decision:** Mark advanced features as optional with clear opt-in

**Optional/Advanced Features:**
1. `agentic_workflow.user_inputs` - Interactive workflows
2. `hooks` system - Lifecycle customization
3. `interpolation` operators - Rich expressions
4. `guards` - Safety constraints
5. `allowed_processors` - Sophisticated compute selection
6. `percentage-based` memory - Dynamic resource allocation
7. `retry.level` - Retry scope control
8. `checkpoint.level: timetravel` - Advanced checkpoint modes
9. `synchronization` - Parallel coordination

**Core Features (Always Required):**
- Basic workflow structure
- Model configuration
- Execution strategy
- Pipeline steps
- Output specification
- Basic retry

---

## 5. Implementation Roadmap

### Phase 1: Unified Schema Specification (Weeks 1-2)

**Objective:** Define complete schema with manual-first design

**Tasks:**
1. [ ] Create unified schema specification document
   - Manual structure as primary
   - Example mappings documented
   - Backward compatibility rules
   - Optional/advanced features marked

2. [ ] Define property reference guide
   - Manual property names → description
   - Legacy property names → manual mapping
   - Required/optional markings
   - Default values

3. [ ] Create validation rules document
   - Type constraints
   - Value ranges
   - Inter-property dependencies
   - System resource constraints

**Deliverables:**
- `schema/unified-schema-spec.md`
- `schema/property-reference.md`
- `schema/validation-rules.md`

### Phase 2: Core Schema Implementation (Weeks 2-3)

**Objective:** Implement parser and validator for unified schema

**Tasks:**
1. [ ] Implement YAML parser with both syntaxes
   - Parse `agentic_workflow.steps: { ... }`
   - Parse `pipeline: [ - step: ... ]`
   - Normalize to internal representation

2. [ ] Implement model configuration parser
   - Percentage-based limits
   - Absolute limits
   - System spec integration
   - Helpful error messages

3. [ ] Implement execution strategy parser
   - `workflow_execution_strategy.*` structure
   - Validation of nested configs
   - Inter-property validation (e.g., `parallel.max_steps` vs `processing: parallel`)

4. [ ] Implement advanced features parser
   - Hooks system
   - Interpolation operators
   - User inputs
   - Guards

**Deliverables:**
- Parser implementation with tests
- Validator implementation with tests
- Internal representation normalization

### Phase 3: Example Migration and Validation (Weeks 3-4)

**Objective:** Update all examples to work with unified parser

**Tasks:**
1. [ ] Update ex01-ex06 to unified schema
   - Keep compatible with manual structure
   - Test backward compatibility
   - Add missing features

2. [ ] Update ex07-ex12 to unified schema
   - Test all features
   - Validate round-trip

3. [ ] Update ex13-ex18 to unified schema
   - Test complex orchestration
   - Validate all features

4. [ ] Create migration tool
   - Convert examples to manual structure
   - Convert manual to example structure
   - Document migration results

**Deliverables:**
- Updated ex01-ex18.yml files
- Migration tool
- Migration documentation

### Phase 4: Documentation Generation (Weeks 4-5)

**Objective:** Create comprehensive documentation

**Tasks:**
1. [ ] Write schema reference manual
   - All sections documented
   - Examples for each property
   - Best practices
   - Common patterns

2. [ ] Write quick start guide
   - Minimal workflow example (manual-first)
   - Step-by-step tutorial
   - Common workflow patterns

3. [ ] Write migration guide
   - From examples to manual structure
   - From manual to examples (if needed)
   - Common migration issues and solutions

4. [ ] Write advanced features guide
   - Hooks system
   - Interpolation operators
   - User inputs
   - Guards and safety

**Deliverables:**
- `docs/schema-reference.md`
- `docs/quick-start.md`
- `docs/migration-guide.md`
- `docs/advanced-features.md`

### Phase 5: Tooling Support (Weeks 5-6)

**Objective:** Create tools for schema management

**Tasks:**
1. [ ] Schema validation tool
   - Validate YAML workflows
   - Check required/optional
   - Suggest fixes for errors
   - Check resource constraints

2. [ ] Workflow generator CLI
   - Interactive workflow creation
   - Template-based generation
   - Auto-completion for properties
   - Support both syntaxes

3. [ ] Migration tool
   - Convert between syntaxes
   - Batch migration of examples
   - Validation of migrated workflows

**Deliverables:**
- `tools/schema-validator/`
- `tools/workflow-generator/`
- `tools/migration-tool/`

---

## 6. Success Criteria

### 6.1 Schema Completeness

- [ ] All 22 original requirements covered
- [ ] All 18 example workflow features supported
- [ ] All 15 manual advanced features preserved
- [ ] Clear required/optional markings
- [ ] Default values for all optional fields

### 6.2 Compatibility

- [ ] All existing ex01-ex18 workflows parse successfully
- [ ] Manual schema parses successfully
- [ ] Both pipeline syntaxes (object and array) supported
- [ ] Both model config styles (percentage and absolute) supported
- [ ] Migration path documented and tested

### 6.3 Readability

- [ ] Clear section ordering
- [ ] Logical grouping of properties
- [ ] Inline documentation for complex fields
- [ ] Example values in comments
- [ ] Consistent property naming

### 6.4 Tooling

- [ ] Schema validation tool functional
- [ ] Workflow generator CLI functional
- [ ] Migration tool functional
- [ ] Documentation generator functional

---

## 7. Open Questions

1. **Should we create separate documentation for core vs advanced features?**
   - Core: Basic workflow definition
   - Advanced: Hooks, interpolation, guards

2. **What's the deprecation timeline for legacy example syntax?**
   - How many versions to support?
   - When to issue deprecation warnings?
   - When to remove entirely?

3. **Should we provide a "best practice" analyzer tool?**
   - Analyze workflow for common issues
   - Suggest improvements
   - Compare against examples

4. **How should we handle system specs for percentage-based limits?**
   - Read from system?
   - Config file?
   - Command-line argument?

5. **Should we create a visual schema designer?**
   - Web-based UI for workflow creation
   - Drag-and-drop for steps
   - Visual feedback for errors

---

## 8. Next Steps

1. **Review this manual-first plan** and provide feedback
2. **Prioritize advanced features** - which are most critical for your use cases?
3. **Approve or modify roadmap** (Phase 1-5) - 6-week timeline acceptable?
4. **Begin Phase 1** - once priorities are set

**Status:** Awaiting user feedback on design direction and priorities
