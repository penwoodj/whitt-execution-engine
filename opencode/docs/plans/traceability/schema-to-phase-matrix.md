# Schema to Phase Matrix

**Purpose**: Map every schema domain from unified-workflow-schema.yml to its owning implementation phase

**Schema Version**: 1.0.0
**Date**: 2026-04-06
**Total Schema Sections**: 19

---

## Section 1: Workflow Identification

**Owner Phase**: Phase 0 (Foundation)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `workflow_id` | string (required) | - | Phase 0 | All workflows must have unique ID, validated at parse time |
| `name` | string (required) | - | Phase 0 | Human-readable name required for queue visualization |
| `description` | string (required) | - | Phase 0 | Detailed description for discoverability |
| `version` | string | "1.0.0" | Phase 0 | Semantic versioning for workflow evolution |
| `author` | string | "" | Phase 0 | Optional attribution |
| `tags` | array | [] | Phase 0 | Categorization for workflow discovery |

**Test Coverage**:
- `01-basic-model-selection-providers.yaml` - All identification fields populated
- `02-model-parameters-tuning.yaml` - Version tag demonstration
- All 53 workflows require identification fields

**Phase 0 Validation**:
- WorkflowSpec struct defines all identification fields with types
- Parser validates required fields (workflow_id, name, description)
- UUID generation for workflow_id if not provided
- Tag-based search implemented

---

## Section 2: Model Configuration

**Owner Phase**: Phase 0 (Foundation), Phase 3 (Backends - provider abstraction)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `models.global_config_path` | string | "./workspace/model-config.yml" | Phase 0 | `01-basic-model-selection-providers.yaml` | External config file loaded and merged |
| `models.default_router` | enum | "automatic" | Phase 0 | All workflows | Auto-routing selects model by task type |
| `models."identifier"` | object | - | Phase 0 | `03-model-lifecycle-management.yaml` | Model definitions compiled into WorkflowIR |
| `models."identifier".name` | string | "" | Phase 0 | - | Variable reference name validated |
| `models."identifier".host.type` | enum | "lmstudio" | Phase 3 | `01-basic-model-selection-providers.yaml` | Provider abstraction normalizes local runners |
| `models."identifier".host.connection_settings` | object | {} | Phase 3 | - | Provider-specific settings validated |
| `models."identifier".max_allowed.ram` | percentage/absolute | "13%" | Phase 0 | `03-model-lifecycle-management.yaml` | Resource limits enforced at runtime |
| `models."identifier".max_allowed.vram` | percentage/absolute | "3.7GB" | Phase 0 | - | GPU memory allocation validated |
| `models."identifier".max_allowed.cpu` | percentage | "49%" | Phase 0 | - | CPU quota enforced |
| `models."identifier".max_allowed.gpu` | percentage | "74%" | Phase 0 | - | GPU reservation validated |
| `models."identifier".max_allowed.attention_tokens` | integer | 150000 | Phase 0 | - | Context window size applied |
| `models."identifier".max_allowed.concurrent_requests` | integer | 2 | Phase 0 | - | Concurrent request throttling |
| `models."identifier".min_allowed.ram` | percentage | "9%" | Phase 0 | - | Minimum requirements validated at start |
| `models."identifier".min_allowed.vram` | percentage | "2.4GB" | Phase 0 | - | - |
| `models."identifier".min_allowed.attention_tokens` | integer | 73500 | Phase 0 | - | Useful minimum enforced |
| `models."identifier".ram_allocation.strategy` | enum | "dynamic" | Phase 0 | - | Resource allocation mode applied |
| `models."identifier".model_memory.cache_size` | enum | "min" | Phase 0 | - | KV cache size configured |
| `models."identifier".model_memory.kv_cache_quantization` | enum | "auto" | Phase 0 | - | Quantization applied to memory |
| `models."identifier".model_memory.attention_context` | enum | "auto" | Phase 0 | - | Context size source resolved |
| `models."identifier".execution.timeout.load_into_memory` | duration | "45s" | Phase 0 | - | Load timeout enforced |
| `models."identifier".execution.timeout.time_to_first_response` | duration | "1m" | Phase 0 | - | First token timeout monitored |
| `models."identifier".execution.timeout.total_time_to_response` | duration | "4h" | Phase 0 | - | Total execution timeout enforced |
| `models."identifier".execution.max_turns` | integer | 10 | Phase 0 | - | Turn limit enforced |
| `models."identifier".execution.stop_on_tool_failure` | boolean | false | Phase 0 | - | Tool failure handling applied |
| `models."identifier".execution.accumulate_tool_results` | boolean | true | Phase 0 | - | Result aggregation enabled |
| `models."identifier".thinking.budget_tokens` | integer | 4096 | Phase 0 | - | Reasoning token budget enforced |
| `models."identifier".thinking.capture_in_output` | boolean | true | Phase 0 | - | Output capture configured |
| `models."identifier".thinking.capture_in_events` | boolean | true | Phase 0 | - | Event emission enabled |
| `models."identifier".thinking.stream_to_log` | boolean | true | Phase 0 | - | Log streaming configured |
| `models."identifier".tools.default_permissions` | object | - | Phase 2 | `01-file-read-write-batch.yaml` | Default tool permissions applied |
| `models."identifier".tools.allowed_tools` | array | [] | Phase 2 | - | Tool allowlist enforced |
| `models."identifier".tools.forbidden_tools` | array | [] | Phase 2 | - | Tool denylist enforced |
| `models."identifier".hooks.*` | object | - | Phase 1 | `03-model-lifecycle-management.yaml` | All lifecycle hooks fire correctly |
| `models."identifier".guardrails.enforcement_policy` | enum | "block" | Phase 1 | `02-guardrails-content-safety.yaml` | Guardrails policy enforced |
| `models."identifier".guardrails.input.guards` | array | - | Phase 1 | - | Input guards active |
| `models."identifier".guardrails.output.guards` | array | - | Phase 1 | - | Output guards active |
| `models."identifier".guardrails.tool_use.guards` | array | - | Phase 1 | - | Tool use guards active |
| `models."identifier".framework.type` | enum | "agentsdk" | Phase 0 | All workflows | Framework type validated |
| `models."identifier".framework.custom_executor_name` | string | "" | Phase 0 | - | Custom executor loaded if specified |

**Test Coverage**:
- `01-basic-model-selection-providers.yaml` - Multi-model configuration
- `02-model-parameters-tuning.yaml` - Parameter overrides
- `03-model-lifecycle-management.yaml` - Full lifecycle hooks
- `04-cost-tracking-budgets.yaml` - Resource limits and allocation

**Phase 0 Validation**:
- Model definitions compiled into WorkflowIR
- Global config file loaded and merged
- Resource limits validated against system
- All model properties typed and validated

**Phase 3 Validation**:
- Provider abstraction layer normalizes llama.cpp, LM Studio, Ollama
- Provider-specific settings handled correctly
- Backend switching without workflow changes

---

## Section 3: Agentic Workflow

**Owner Phase**: Phase 1 (MVP Execution)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `agentic_workflow.hardcoded_values` | object | - | Phase 1 | All workflows | Static variables available at parse time |
| `agentic_workflow.user_inputs.execution_mode` | enum | "interactive" | Phase 1 | `01-user-input-prompts-validation.yaml` | User input mode applied |
| `agentic_workflow.user_inputs.workflow_level.ask_at_start` | boolean | true | Phase 1 | - | Pre-execution prompts work |
| `agentic_workflow.user_inputs.workflow_level.inputs[].id` | string | - | Phase 1 | - | Input identifier unique |
| `agentic_workflow.user_inputs.workflow_level.inputs[].type` | enum | "text_area" | Phase 1 | - | Input type validated |
| `agentic_workflow.user_inputs.workflow_level.inputs[].label` | string | - | Phase 1 | - | UI label displayed |
| `agentic_workflow.user_inputs.workflow_level.inputs[].default_value` | any | - | Phase 1 | - | Defaults pre-populated |
| `agentic_workflow.user_inputs.workflow_level.inputs[].validation.required` | boolean | true | Phase 1 | - | Required inputs enforced |
| `agentic_workflow.user_inputs.workflow_level.inputs[].validation.pattern` | regex | - | Phase 1 | - | Pattern validation applied |
| `agentic_workflow.user_inputs.step_level.ask_at_step_start` | boolean | true | Phase 1 | - | Step-level prompts fire |
| `agentic_workflow.user_inputs.ui_configuration.layout` | enum | "tabbed" | Phase 7 | - | UI layout rendered |
| `agentic_workflow.user_inputs.ui_configuration.panels` | array | - | Phase 7 | - | UI panels configured |
| `agentic_workflow.steps` | object | - | Phase 1 | All workflows | Step definitions compiled |
| `agentic_workflow.steps."step_id".name` | string | "" | Phase 1 | - | Step name displayed |
| `agentic_workflow.steps."step_id".description` | string | "" | Phase 1 | - | Step description shown |
| `agentic_workflow.steps."step_id".type` | enum | "agent" | Phase 1 | - | Step type determines executor |
| `agentic_workflow.steps."step_id".generative_entity` | string | - | Phase 1 | - | Model reference resolved |
| `agentic_workflow.steps."step_id".model_overrides` | object | - | Phase 1 | `02-model-parameters-tuning.yaml` | Per-step model config applied |
| `agentic_workflow.steps."step_id".prompt` | string | - | Phase 1 | All workflows | Prompt with interpolation rendered |
| `agentic_workflow.steps."step_id".inputs` | object | - | Phase 1 | - | Input variables passed |
| `agentic_workflow.steps."step_id".file_operations` | array | - | Phase 1 | `01-file-read-write-batch.yaml` | Pre-step file ops execute |
| `agentic_workflow.steps."step_id".context.*` | boolean | true | Phase 1 | - | Context injection configured |
| `agentic_workflow.steps."step_id".output.save_to` | string | - | Phase 1 | - | Output variable stored |
| `agentic_workflow.steps."step_id".output.format` | enum | "json" | Phase 1 | - | Output format validated |
| `agentic_workflow.steps."step_id".output.file_output.*` | object | - | Phase 1 | - | File output written |
| `agentic_workflow.steps."step_id".retry.*` | object | - | Phase 1 | - | Retry strategy applied |
| `agentic_workflow.steps."step_id".branches.*` | object | - | Phase 1 | `01-event-based-branching.yaml` | Branches evaluated |
| `agentic_workflow.steps."step_id".pre_hooks` | object | - | Phase 1 | - | Pre-execution hooks fire |
| `agentic_workflow.steps."step_id".post_hooks` | object | - | Phase 1 | - | Post-execution hooks fire |

**Test Coverage**:
- All 52 categorized workflows use agentic_workflow
- `01-user-input-prompts-validation.yaml` - Complete user input configuration
- `01-event-based-branching.yaml` - Branches and conditions
- `01-complex-orchestration-sub-agents.yaml` - Nested workflows

**Phase 1 Validation**:
- Agentic workflow compiled to WorkflowIR
- User inputs captured before execution
- Step dependencies resolved
- Variable interpolation works at runtime

---

## Section 4: Pipeline Definition

**Owner Phase**: Phase 1 (MVP Execution)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `pipeline` | array | - | Phase 1 | All workflows | Array-based step definitions |
| `pipeline[].step` | string (required) | - | Phase 1 | - | Step identifier unique |
| `pipeline[].id` | string | - | Phase 1 | - | Optional ID for reference |
| `pipeline[].name` | string | - | Phase 1 | - | Human-readable name |
| `pipeline[].description` | string | - | Phase 1 | - | Detailed description |
| `pipeline[].type` | enum | "agent" | Phase 1 | - | Step type determined |
| `pipeline[].model` | string | - | Phase 1 | - | Model reference resolved |
| `pipeline[].agent.limited_tools` | array | - | Phase 1 | - | Tools restricted |
| `pipeline[].agent.additional_tools` | array | - | Phase 1 | - | Tools added |
| `pipeline[].input.*` | object | - | Phase 1 | - | Input configuration applied |
| `pipeline[].output.*` | object | - | Phase 1 | - | Output configuration applied |
| `pipeline[].retry.*` | object | - | Phase 1 | - | Retry strategy applied |
| `pipeline[].branches.*` | object | - | Phase 1 | - | Branches configured |
| `pipeline[].parallel_group` | string | - | Phase 1 | `01-parallel-groups-execution.yaml` | Parallel execution grouped |
| `pipeline[].max_parallel` | integer | - | Phase 1 | - | Concurrency limit enforced |
| `pipeline[].timeout_secs` | integer | - | Phase 1 | - | Step timeout enforced |
| `pipeline[].depends_on` | array | - | Phase 1 | - | Dependencies ordered |
| `pipeline[].loop.*` | object | - | Phase 1 | `01-for-loops-explicit-iteration.yaml` | Loop semantics applied |

**Test Coverage**:
- Pipeline format tested with simple workflows
- `01-llm-inference-steps.yaml` - Basic pipeline steps
- `01-parallel-groups-execution.yaml` - Parallel execution

**Phase 1 Validation**:
- Pipeline compiled to WorkflowIR
- Dependency graph built from depends_on
- Parallel groups identified
- Loop types validated

---

## Section 5: Workflow Execution Strategy

**Owner Phase**: Phase 1 (Core), Phase 2 (Complete), Phase 4 (Advanced features)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `workflow_execution_strategy.load_unload` | enum | "one_at_a_time" | Phase 1 | All workflows | Model lifecycle managed |
| `workflow_execution_strategy.processing` | enum | "parallel" | Phase 1 | All workflows | Execution mode applied |
| `workflow_execution_strategy.parallel.enabled` | boolean | true | Phase 1 | - | Parallelization enabled |
| `workflow_execution_strategy.parallel.algorithm` | enum | "round_robin" | Phase 1 | - | Scheduling algorithm applied |
| `workflow_execution_strategy.parallel.load_balancing.strategy` | enum | "least_loaded" | Phase 1 | - | Load balancing configured |
| `workflow_execution_strategy.parallel.max_threads` | integer | 4 | Phase 1 | - | Thread limit enforced |
| `workflow_execution_strategy.parallel.max_models` | integer | 3 | Phase 1 | - | Model concurrency limited |
| `workflow_execution_strategy.parallel.max_concurrent_requests` | integer | 2 | Phase 1 | - | Request throttling |
| `workflow_execution_strategy.memory.*` | object | - | Phase 1 | - | Memory management applied |
| `workflow_execution_strategy.memory.pressure_handling.strategy` | enum | "throttle" | Phase 1 | - | Pressure handling active |
| `workflow_execution_strategy.timeout.*` | object | - | Phase 1 | - | Timeouts enforced |
| `workflow_execution_strategy.timeout.timeout_strategy` | enum | "continue_with_partial" | Phase 2 | - | Timeout behavior applied |
| `workflow_execution_strategy.error_handling.default_action` | enum | "retry" | Phase 1 | - | Default error handling |
| `workflow_execution_strategy.error_handling.retry.*` | object | - | Phase 1 | - | Retry configuration applied |
| `workflow_execution_strategy.error_handling.escalation.*` | object | - | Phase 2 | - | Escalation configured |
| `workflow_execution_strategy.error_handling.handling_by_type.*` | object | - | Phase 2 | - | Type-specific handling |
| `workflow_execution_strategy.sub_workflow.enabled` | boolean | true | Phase 2 | - | Sub-workflows enabled |
| `workflow_execution_strategy.sub_workflow.inherit_policy.enabled` | boolean | true | Phase 2 | - | Policy inheritance active |
| `workflow_execution_strategy.sub_workflow.reference_resolution.strategy` | enum | "hierarchical" | Phase 2 | - | Reference resolution configured |
| `workflow_execution_strategy.sub_workflow.isolated_environments.*` | object | - | Phase 2 | - | Isolation applied |
| `workflow_execution_strategy.sub_workflow.circular_reference_detection.*` | object | - | Phase 2 | - | Circular references prevented |
| `workflow_execution_strategy.synchronization.enabled` | boolean | true | Phase 2 | - | Synchronization active |
| `workflow_execution_strategy.checkpointing.enabled` | boolean | true | Phase 1 | - | Checkpointing enabled |
| `workflow_execution_strategy.checkpointing.triggers.*` | object | - | Phase 1 | - | Triggers fire |
| `workflow_execution_strategy.checkpointing.storage.*` | object | - | Phase 1 | - | State stored |
| `workflow_execution_strategy.checkpointing.cleanup.*` | object | - | Phase 1 | - | Cleanup runs |
| `workflow_execution_strategy.resource_allocation.*` | object | - | Phase 1 | - | Resources allocated |
| `workflow_execution_strategy.dependency_resolution.*` | object | - | Phase 1 | - | Dependencies resolved |
| `workflow_execution_strategy.concurrency_limits.*` | object | - | Phase 1 | - | Limits enforced |
| `workflow_execution_strategy.step_prioritization.*` | object | - | Phase 1 | - | Prioritization applied |
| `workflow_execution_strategy.performance_optimization.*` | object | - | Phase 1 | - | Optimizations active |
| `workflow_execution_strategy.state_transition_hooks.*` | object | - | Phase 1 | - | Hooks fire |
| `workflow_execution_strategy.advanced_scheduling.*` | object | - | Phase 4 | - | Advanced scheduling |
| `workflow_execution_strategy.adaptive.*` | object | - | Phase 4 | - | Adaptive behavior |

**Test Coverage**:
- `01-parallel-groups-execution.yaml` - Parallel execution
- `01-checkpointing-save-restore.yaml` - Checkpointing
- `01-memory-allocation-strategies.yaml` - Memory management

**Phase 1 Validation**:
- Core execution strategy implemented
- Basic parallelization works
- Timeout and error handling functional
- Checkpointing saves state

**Phase 2 Validation**:
- Sub-workflow coordination complete
- Escalation paths work
- Isolation enforced

**Phase 4 Validation**:
- Advanced scheduling algorithms work
- Adaptive resource management active
- Performance optimizations applied

---

## Section 6: Tool Permissions

**Owner Phase**: Phase 2 (MVP Execution)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `tool_permissions.file_operations.read.enabled` | boolean | true | Phase 2 | `01-file-read-write-batch.yaml` | File read allowed |
| `tool_permissions.file_operations.read.allowed_paths` | array | - | Phase 2 | - | Paths validated |
| `tool_permissions.file_operations.write.enabled` | boolean | true | Phase 2 | - | File write allowed |
| `tool_permissions.file_operations.write.require_confirmation` | boolean | true | Phase 2 | - | Confirmation required |
| `tool_permissions.file_operations.delete.enabled` | boolean | true | Phase 2 | - | File delete allowed |
| `tool_permissions.file_operations.delete.require_confirmation` | boolean | true | Phase 2 | - | Confirmation required |
| `tool_permissions.web_operations.fetch.enabled` | boolean | true | Phase 2 | `01-web-fetch-scrape.yaml` | Web fetch allowed |
| `tool_permissions.web_operations.fetch.allowed_domains` | array | - | Phase 2 | - | Domains validated |
| `tool_permissions.web_operations.scrape.enabled` | boolean | true | Phase 2 | - | Web scraping allowed |
| `tool_permissions.web_operations.scrape.respect_robots_txt` | boolean | true | Phase 2 | - | Robots.txt respected |
| `tool_permissions.shell_operations.exec.enabled` | boolean | true | Phase 2 | - | Shell exec allowed |
| `tool_permissions.shell_operations.exec.require_confirmation` | boolean | true | Phase 2 | - | Confirmation required |
| `tool_permissions.shell_operations.exec.allowed_commands` | array | - | Phase 2 | - | Commands validated |
| `tool_permissions.content_operations.web_search.enabled` | boolean | true | Phase 5 | - | Web search allowed |
| `tool_permissions.content_operations.content_generation.enabled` | boolean | true | Phase 5 | - | Content generation allowed |
| `tool_permissions.system_operations.*` | object | - | Phase 6 | - | System ops controlled |

**Test Coverage**:
- `01-file-read-write-batch.yaml` - File permissions
- `02-file-permissions-backup.yaml` - Permission granularity
- `01-web-fetch-scrape.yaml` - Web permissions
- `01-allow-deny-lists-scopes.yaml` - Allow/deny enforcement

**Phase 2 Validation**:
- Permission system enforced at runtime
- Global and per-step permissions merged
- Confirmation prompts fire
- Scope boundaries respected

---

## Section 7: Logging Configuration

**Owner Phase**: Phase 1 (MVP Execution)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `logging.enabled` | boolean | true | Phase 1 | All workflows | Logging active |
| `logging.levels.default` | enum | "info" | Phase 1 | - | Default log level applied |
| `logging.levels.workflow` | enum | "info" | Phase 1 | - | Workflow logs captured |
| `logging.levels.pipeline` | enum | "debug" | Phase 1 | - | Pipeline logs captured |
| `logging.levels.models` | enum | "warning" | Phase 1 | - | Model logs captured |
| `logging.levels.tools` | enum | "info" | Phase 1 | - | Tool logs captured |
| `logging.levels.execution` | enum | "debug" | Phase 1 | - | Execution logs captured |
| `logging.scopes.workflow.enabled` | boolean | true | Phase 1 | - | Workflow scope logged |
| `logging.scopes.pipeline.enabled` | boolean | true | Phase 1 | - | Pipeline scope logged |
| `logging.scopes.models.enabled` | boolean | true | Phase 1 | - | Model scope logged |
| `logging.scopes.tools.enabled` | boolean | true | Phase 1 | - | Tool scope logged |
| `logging.scopes.execution.enabled` | boolean | true | Phase 1 | - | Execution scope logged |
| `logging.output.console.enabled` | boolean | true | Phase 1 | - | Console output active |
| `logging.output.console.color` | boolean | true | Phase 1 | - | Color output rendered |
| `logging.output.file.enabled` | boolean | true | Phase 1 | - | File logging active |
| `logging.output.file.path` | string | "/workspace/logs/workflow.log" | Phase 1 | - | Log file written |
| `logging.output.file.rotation.enabled` | boolean | true | Phase 1 | - | Rotation active |
| `logging.errors.log_parsing_errors` | boolean | true | Phase 1 | - | Parsing errors logged |
| `logging.errors.error_log_file` | string | "/workspace/logs/errors.log" | Phase 1 | - | Error log written |

**Test Coverage**:
- `01-hierarchical-logging-system.yaml` - Complete logging configuration
- All 52 workflows include logging settings

**Phase 1 Validation**:
- Hierarchical logging implemented
- Log levels respected
- Output destinations work
- Error logging separate

---

## Section 8: Metrics Configuration

**Owner Phase**: Phase 1 (Core), Phase 7 (Comprehensive)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `metrics.enabled` | boolean | true | Phase 1 | All workflows | Metrics collection active |
| `metrics.collection.pipeline_level` | array | - | Phase 1 | - | Pipeline metrics collected |
| `metrics.collection.step_level` | array | - | Phase 1 | - | Step metrics collected |
| `metrics.collection.model_level` | array | - | Phase 1 | - | Model metrics collected |
| `metrics.collection.tool_level` | array | - | Phase 1 | - | Tool metrics collected |
| `metrics.collection.custom_metrics` | array | - | Phase 7 | - | Custom metrics tracked |
| `metrics.performance_optimization.cache.enabled` | boolean | true | Phase 1 | - | Caching active |
| `metrics.performance_optimization.batching.enabled` | boolean | true | Phase 1 | - | Batching enabled |
| `metrics.performance_optimization.prefetching.enabled` | boolean | true | Phase 1 | - | Prefetching active |
| `metrics.output.path` | string | "/workspace/metrics/workflow_metrics.json" | Phase 1 | - | Metrics file written |
| `metrics.output.format` | enum | "json" | Phase 1 | - | Format validated |

**Test Coverage**:
- `02-metrics-collection.yaml` - Core metrics
- Comprehensive workflows track all metric types

**Phase 1 Validation**:
- Core metrics collected
- Performance optimization active
- Metrics output written

**Phase 7 Validation**:
- Custom metrics framework
- Comprehensive observability dashboards
- Metrics-driven insights

---

## Section 9: Validation Configuration

**Owner Phase**: Phase 1 (MVP Execution)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `validation.workflow_schema.enabled` | boolean | true | Phase 1 | All workflows | Schema validation active |
| `validation.workflow_schema.required_fields` | array | [workflow_id, name, models] | Phase 1 | - | Required fields enforced |
| `validation.step_outputs.enabled` | boolean | true | Phase 1 | - | Output validation active |
| `validation.step_outputs.validate_format` | enum | "json" | Phase 1 | - | Format validated |
| `validation.business_rules.enabled` | boolean | true | Phase 1 | - | Business rules checked |
| `validation.business_rules.rules` | array | - | Phase 1 | - | Rules evaluated |

**Test Coverage**:
- All workflows validated at startup
- `01-basic-model-selection-providers.yaml` - Schema validation
- Workflow-level validation rules tested

**Phase 1 Validation**:
- Schema validation catches errors
- Output validation enforces format
- Business rules evaluated correctly

---

## Section 10: Orchestration Configuration

**Owner Phase**: Phase 1 (Partial), Phase 2 (Complete)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `orchestration.enabled` | boolean | true | Phase 1 | All workflows | Orchestration active |
| `orchestration.step_coordination.max_concurrent_steps` | integer | 10 | Phase 1 | - | Concurrency limited |
| `orchestration.step_coordination.inter_step_dependencies.enabled` | boolean | true | Phase 1 | - | Dependencies tracked |
| `orchestration.sub_agent_orchestration.enabled` | boolean | true | Phase 2 | `01-complex-orchestration-sub-agents.yaml` | Sub-agents orchestrated |
| `orchestration.sub_agent_orchestration.sub_agents[]` | object | - | Phase 2 | - | Sub-agents defined |
| `orchestration.sub_agent_orchestration.event_handling.*` | object | - | Phase 2 | - | Events handled |
| `orchestration.validation_aggregation.strategy` | enum | "hierarchical" | Phase 2 | - | Aggregation strategy applied |
| `orchestration.validation_aggregation.overall_validation_criteria` | array | - | Phase 2 | - | Criteria weighted |
| `orchestration.checkpoint_coordination.enabled` | boolean | true | Phase 2 | - | Checkpoint coordination active |

**Test Coverage**:
- `01-complex-orchestration-sub-agents.yaml` - Sub-agent orchestration
- `04-convergence-reduction-aggregation.yaml` - Validation aggregation

**Phase 1 Validation**:
- Step coordination works
- Basic dependencies resolved

**Phase 2 Validation**:
- Sub-agent spawning functional
- Interdependent validation works
- Event propagation correct

---

## Section 11: Provider Configuration

**Owner Phase**: Phase 2 (MVP Execution + Backends)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `providers.lmstudio.config` | object | - | Phase 3 | `01-basic-model-selection-providers.yaml` | LM Studio configured |
| `providers.lmstudio.config_file` | string | - | Phase 3 | - | Config file loaded |
| `providers.lmstudio.hosting.*` | object | - | Phase 3 | - | Hosting configured |
| `providers.lmstudio.requests.*` | object | - | Phase 3 | - | Request handling active |
| `providers.ollama.config_file` | string | - | Phase 3 | - | Ollama configured |
| `providers.llama_cpp_with_vulkan.config_file` | string | - | Phase 3 | - | llama.cpp configured |

**Test Coverage**:
- `01-basic-model-selection-providers.yaml` - All providers
- `02-model-parameters-tuning.yaml` - Provider switching

**Phase 3 Validation**:
- Provider abstraction works
- Config files loaded correctly
- Provider switching seamless
- Hardware acceleration supported

---

## Section 12: RAG Configuration

**Owner Phase**: Phase 2 (Basic), Phase 5 (Advanced)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `rag.enabled` | boolean | false | Phase 2 | `01-document-indexing-retrieval.yaml` | RAG enabled |
| `rag.knowledge_base.path` | string | "/workspace/rag/knowledge_base" | Phase 2 | - | Knowledge base accessible |
| `rag.knowledge_base.format` | enum | "vector_db" | Phase 2 | - | Storage format applied |
| `rag.knowledge_base.chunk_size` | integer | 512 | Phase 2 | - | Chunking configured |
| `rag.knowledge_base.chunk_overlap` | integer | 50 | Phase 2 | - | Overlap applied |
| `rag.embedding_model.model_ref` | string | - | Phase 2 | - | Embedding model loaded |
| `rag.embedding_model.dimension` | integer | 768 | Phase 2 | - | Embedding dimension set |
| `rag.retrieval.max_results` | integer | 10 | Phase 2 | - | Results limited |
| `rag.retrieval.similarity_threshold` | float | 0.7 | Phase 2 | - | Threshold applied |

**Test Coverage**:
- `01-document-indexing-retrieval.yaml` - Basic RAG
- `02-rag-generation-context-aware.yaml` - Context-aware generation

**Phase 2 Validation**:
- Basic RAG functional
- Document indexing works
- Retrieval returns relevant results
- Context injection successful

**Phase 5 Validation**:
- Advanced retrieval strategies
- Hybrid search (exact + semantic)
- Multi-source retrieval

---

## Section 13: Workspace Configuration

**Owner Phase**: Phase 0 (Foundation)

**Schema Fields**:
| Field | Type | Default | Implementation Phase | Example Workflows | Acceptance Criteria |
|--------|------|----------|---------------------|-------------------|
| `workspace.root_path` | string | "/workspace" | Phase 0 | All workflows | Root path set |
| `workspace.directories.output` | string | "/workspace/output" | Phase 0 | - | Output directory created |
| `workspace.directories.checkpoints` | string | "/workspace/checkpoints" | Phase 0 | - | Checkpoint directory created |
| `workspace.directories.logs` | string | "/workspace/logs" | Phase 0 | - | Log directory created |
| `workspace.directories.metrics` | string | "/workspace/metrics" | Phase 0 | - | Metrics directory created |
| `workspace.directories.backups` | string | "/workspace/backups" | Phase 0 | - | Backup directory created |
| `workspace.directories.temp` | string | "/workspace/temp" | Phase 0 | - | Temp directory created |
| `workspace.directories.rag_knowledge_base` | string | "/workspace/rag/knowledge_base" | Phase 0 | - | RAG directory created |
| `workspace.permissions.*` | object | - | Phase 0 | - | Permissions set |

**Test Coverage**:
- All workflows use workspace paths
- Directory creation tested in Phase 0

**Phase 0 Validation**:
- All directories created on init
- Workspace paths resolved correctly
- Permissions applied

---

## Section 14: Features Demonstrated

**Owner Phase**: Phase 0 (Foundation - documentation)

**Note (v2.0)**: This section was removed from the unified schema v2.0. The `features_demonstrated` key is no longer used in workflow definitions.

**Schema Fields** (v1 reference, preserved for historical context):
| Feature | Type | Implementation Phase | Example Workflows | Acceptance Criteria |
|---------|------|---------------------|-------------------|-------------------|
| `features_demonstrated.model_configuration` | boolean | Phase 0 | `01-basic-model-selection-providers.yaml` | Feature working |
| `features_demonstrated.multi_model_support` | boolean | Phase 0 | All multi-model workflows | Multiple models load |
| `features_demonstrated.execution_modes` | boolean | Phase 1 | All workflows | All modes work |
| `features_demonstrated.memory_management` | boolean | Phase 1 | `03-model-lifecycle-management.yaml` | Memory managed |
| `features_demonstrated.hierarchical_logging` | boolean | Phase 1 | `01-hierarchical-logging-system.yaml` | Logging hierarchical |
| `features_demonstrated.retry_logic` | boolean | Phase 1 | `01-retry-strategies-backoff.yaml` | Retry works |
| `features_demonstrated.loop_variations` | boolean | Phase 1 | `01-for-loops-explicit-iteration.yaml` | All loop types work |
| `features_demonstrated.validation_loops` | boolean | Phase 1 | All validation workflows | Validation loops converge |
| `features_demonstrated.tool_execution` | boolean | Phase 1 | All workflows | Tools execute |
| `features_demonstrated.tool_permissions` | boolean | Phase 2 | `01-allow-deny-lists-scopes.yaml` | Permissions enforced |
| `features_demonstrated.convergence_loops` | boolean | Phase 1 | `04-convergence-reduction-aggregation.yaml` | Convergence works |
| `features_demonstrated.rag_operations` | boolean | Phase 2 | `01-document-indexing-retrieval.yaml` | RAG functional |
| `features_demonstrated.nested_workflow_references` | boolean | Phase 2 | `01-nested-workflow-references.yaml` | Nesting works |
| `features_demonstrated.web_operations` | boolean | Phase 2 | `01-web-fetch-scrape.yaml` | Web ops work |
| `features_demonstrated.script_cli_execution` | boolean | Phase 2 | `01-script-execution.yaml` | Scripts run |
| `features_demonstrated.auto_model_routing` | boolean | Phase 1 | All workflows | Auto-routing works |
| `features_demonstrated.conditional_branching` | boolean | Phase 1 | `01-event-based-branching.yaml` | Branching works |
| `features_demonstrated.event_based_workflows` | boolean | Phase 1 | All workflows | Events handled |
| `features_demonstrated.interdependent_validation` | boolean | Phase 2 | `01-complex-orchestration-sub-agents.yaml` | Interdependence works |
| `features_demonstrated.prompt_passing` | boolean | Phase 1 | All workflows | Prompts passed |
| `features_demonstrated.variable_references` | boolean | Phase 1 | All workflows | Variables resolve |
| `features_demonstrated.metrics_collection` | boolean | Phase 1 | `02-metrics-collection.yaml` | Metrics collected |
| `features_demonstrated.state_management` | boolean | Phase 1 | `02-state-management.yaml` | State managed |
| `features_demonstrated.file_operations` | boolean | Phase 1 | `01-file-read-write-batch.yaml` | File ops work |
| `features_demonstrated.checkpoint_save_restore` | boolean | Phase 1 | `01-checkpointing-save-restore.yaml` | Checkpoints work |
| `features_demonstrated.bidirectional_event_propagation` | boolean | Phase 2 | `01-complex-orchestration-sub-agents.yaml` | Events propagate |
| `features_demonstrated.sub_agent_orchestration` | boolean | Phase 2 | `01-complex-orchestration-sub-agents.yaml` | Sub-agents orchestrated |
| `features_demonstrated.weighted_validation_criteria` | boolean | Phase 2 | `04-convergence-reduction-aggregation.yaml` | Weights applied |
| `features_demonstrated.strict_yes_no_decisions` | boolean | Phase 1 | All workflows | Decisions enforced |
| `features_demonstrated.user_inputs` | boolean | Phase 1 | `01-user-input-prompts-validation.yaml` | Inputs work |
| `features_demonstrated.hooks` | boolean | Phase 1 | `01-pre-workflow-post-workflow-hooks.yaml` | Hooks fire |
| `features_demonstrated.guardrails` | boolean | Phase 1 | `02-guardrails-content-safety.yaml` | Guardrails active |
| `features_demonstrated.provider_config_files` | boolean | Phase 3 | `01-basic-model-selection-providers.yaml` | Config files load |
| `features_demonstrated.global_model_config` | boolean | Phase 0 | `01-basic-model-selection-providers.yaml` | Global config works |
| `features_demonstrated.schema_versioning` | boolean | Phase 0 | All workflows | Versioning enforced |
| `features_demonstrated.configuration_profiles` | boolean | Phase 0 | All workflows | Profiles loaded |

**Test Coverage**:
- All 52 categorized workflows demonstrate features
- Comprehensive integration workflows show combined features

**Phase 0 Validation**:
- Feature flags documented
- All features listed for discoverability

---

## Section 15: Variable Interpolation

**Owner Phase**: Phase 0 (Foundation)

**Schema Fields**:
| Syntax | Purpose | Resolves | Implementation Phase | Example Workflows | Acceptance Criteria |
|---------|-----------|-----------|---------------------|-------------------|
| `${models.xxx}` | Model reference | Parse time | Phase 0 | All workflows | Reference resolves |
| `${workflow.xxx}` | Workflow variable | Parse time | Phase 0 | All workflows | Reference resolves |
| `${workspace.xxx}` | Workspace path | Parse time | Phase 0 | All workflows | Path resolves |
| `{{step.step_id.output}}` | Step output | Runtime | Phase 1 | All workflows | Output captured |
| `{{now}}` | Timestamp | Runtime | Phase 1 | All workflows | Timestamp injected |
| `{{workflow_id}}` | Workflow ID | Runtime | Phase 1 | All workflows | ID injected |
| `{{run.number}}` | Run number | Runtime | Phase 1 | All workflows | Number injected |

**Test Coverage**:
- All workflows use variable interpolation
- `01-workflow-level-variables.yaml` - Workflow variables
- `02-step-outputs-as-input.yaml` - Step outputs

**Phase 0 Validation**:
- Structural references resolved at parse time
- Variable syntax validated
- Reference targets checked

**Phase 1 Validation**:
- Dynamic values resolved at runtime
- Runtime injection works correctly
- Variable scope rules enforced

---

## Section 16: Default Behavior Reference

**Owner Phase**: Phase 0 (Foundation)

**Schema Fields**:
| Field | Default | Implementation Phase | Acceptance Criteria |
|--------|----------|---------------------|-------------------|
| `execution.mode` | "serial" | Phase 1 | Default mode applied |
| `retry` (absent) | No retry | Phase 1 | Fail immediately |
| `retry.condition` (absent) | Always retry | Phase 1 | Retries if block present |
| `retry.default.max_attempts` | 3 | Phase 1 | Three attempts |
| `retry.default.backoff` | exponential | Phase 1 | Exponential backoff |
| `retry.default.delay_ms` | 1000 | Phase 1 | 1 second delay |
| `logging.level` | "info" | Phase 1 | Info level |
| `logging.enabled` | true | Phase 1 | Logging on |
| `logging.output_type` | "chat" | Phase 1 | Chat output |
| `checkpointing.enabled` | true | Phase 1 | Checkpointing on |
| `checkpointing.on_failure` | create | Phase 1 | Create on failure |
| `checkpointing.on_timeout` | create | Phase 1 | Create on timeout |
| `checkpointing.on_user_interrupt` | create | Phase 1 | Create on interrupt |
| `error_handling.default_action.type` | retry | Phase 1 | Retry by default |
| `tool_permissions` (absent) | All allowed | Phase 2 | Open by default |
| `models.default_router` | automatic | Phase 1 | Auto-routing |

**Test Coverage**:
- Default behavior tested with minimal workflows
- All workflows rely on sensible defaults

**Phase 0 Validation**:
- All defaults documented
- Default behavior consistent
- Graceful degradation works

---

## Section 17: Scope & Inheritance Rules

**Owner Phase**: Phase 0 (Foundation), Phase 1-2 (runtime enforcement)

**Schema Fields**:
| Domain | Rule | Implementation Phase | Acceptance Criteria |
|--------|------|---------------------|-------------------|
| Tool Permissions | Global baseline, per-step restricts | Phase 2 | Restrictions respected |
| Logging | Hierarchical inheritance | Phase 1 | Overrides applied |
| Retry | Workflow default, per-step override | Phase 1 | Override applied |
| Models | Per-step overrides | Phase 1 | Override works |
| Orchestration | Parent-child relationship | Phase 2 | Relationship enforced |

**Test Coverage**:
- Permission inheritance tested
- Logging override tested
- Retry override tested
- Model override tested

**Phase 0 Validation**:
- Rules documented in schema
- Inheritance hierarchy clear

**Phase 1-2 Validation**:
- Inheritance rules enforced at runtime
- Override semantics correct

---

## Section 18: Numeric Threshold Guidance

**Owner Phase**: Phase 0 (Foundation documentation)

**Schema Fields**:
| Field | Range | Default | Implementation Phase | Acceptance Criteria |
|--------|-------|----------|---------------------|-------------------|
| `thinking.budget_tokens` | 0-16384 | 4096 | Phase 1 | Enforced |
| `count_config.max_iterations` | 1-1000 | 5 | Phase 1 | Enforced |
| `validation_config.tolerance` | 0.0-1.0 | 0.03 | Phase 1 | Enforced |
| `validation_config.max_iterations` | 1-100 | 4 | Phase 1 | Enforced |
| `exact_criteria.target` | 0.0-1.0 | 0.95 | Phase 1 | Enforced |
| `infinite_config.max_iterations` | 1-10000 | 100 | Phase 1 | Enforced |
| `infinite_config.timeout_seconds` | 1-3600 | 30 | Phase 1 | Enforced |
| `overall_threshold` | 0.0-1.0 | 0.85 | Phase 2 | Enforced |
| `exec.timeout.load_into_memory` | 5-300s | 45s | Phase 1 | Enforced |
| `exec.timeout.first_response` | 5s-5m | 1m | Phase 1 | Enforced |
| `exec.timeout.total_response` | 1m-24h | 4h | Phase 1 | Enforced |

**Test Coverage**:
- Thresholds validated in all workflows
- Edge cases tested (min/max values)

**Phase 0 Validation**:
- All thresholds documented
- Ranges validated
- Defaults reasonable

---

## Section 19: Duplicate Configuration Systems Clarification

**Owner Phase**: Phase 1+2 (runtime implementation)

**Schema Fields**:
| System | Purpose | Implementation Phase | Acceptance Criteria |
|---------|---------|---------------------|-------------------|
| Parallelism | Multi-level control | Phase 1 | All levels work together |
| Permissions | Two enforcement surfaces | Phase 2 | Guardrails cannot exceed tool_permissions |
| Orchestration | Parent-child relationship | Phase 2 | Sub_agent_orchestration always child |

**Test Coverage**:
- Parallelism levels tested independently
- Permission interaction tested
- Orchestration hierarchy tested

**Phase 1 Validation**:
- Parallelism: parallel_group, parallel.algorithm, load_balancing work together

**Phase 2 Validation**:
- Permissions: tool_permissions and guardrails enforced correctly
- Orchestration: hierarchy maintained

---

## Summary Matrix

| Section | Owner Phase | Fields Count | Status |
|----------|--------------|---------------|--------|
| 1. Workflow Identification | Phase 0 | 6 | ✅ Defined |
| 2. Model Configuration | Phase 0, 3 | 47 | ✅ Defined |
| 3. Agentic Workflow | Phase 1 | 27 | ✅ Defined |
| 4. Pipeline Definition | Phase 1 | 18 | ✅ Defined |
| 5. Workflow Execution Strategy | Phase 1, 2, 4 | 52 | ✅ Defined |
| 6. Tool Permissions | Phase 2, 5, 6 | 16 | ✅ Defined |
| 7. Logging Configuration | Phase 1 | 20 | ✅ Defined |
| 8. Metrics Configuration | Phase 1, 7 | 12 | ✅ Defined |
| 9. Validation Configuration | Phase 1 | 10 | ✅ Defined |
| 10. Orchestration Configuration | Phase 1, 2 | 12 | ✅ Defined |
| 11. Provider Configuration | Phase 3 | 15 | ✅ Defined |
| 12. RAG Configuration | Phase 2, 5 | 8 | ✅ Defined |
| 13. Workspace Configuration | Phase 0 | 15 | ✅ Defined |
| 14. Features Demonstrated | Phase 0 | 38 | ✅ Defined |
| 15. Variable Interpolation | Phase 0, 1 | 7 | ✅ Defined |
| 16. Default Behavior Reference | Phase 0, 1 | 13 | ✅ Defined |
| 17. Scope & Inheritance Rules | Phase 0, 1, 2 | 5 | ✅ Defined |
| 18. Numeric Threshold Guidance | Phase 0, 1 | 11 | ✅ Defined |
| 19. Duplicate Config Systems | Phase 1, 2 | 3 | ✅ Defined |

**Total Fields**: 325
**Coverage**: 100% (All 19 sections mapped to phases)
