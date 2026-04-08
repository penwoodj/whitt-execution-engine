# Workflow to Phase Matrix

**Purpose**: Map all 53 example workflows to their testing phases based on feature requirements

**Total Workflows**: 53
**Total Categories**: 19
**Date**: 2026-04-06

---

## Category 01: Model Configuration (4 Workflows)

**Testing Phases**: Phase 0 (Foundation), Phase 2 (Provider Abstraction)

### 01. 01-basic-model-selection-providers.yaml

**Path**: `01-model-configuration/01-basic-model-selection-providers.yaml`

**Owner Phases**: Phase 0, Phase 3

**Features Tested**:
- Multi-model support
- Provider configuration (LM Studio, Ollama, llama.cpp)
- Default model router
- Model switching
- Resource limits (RAM, VRAM, CPU, GPU)

**Phase 0 Tests**:
- Validate workflow schema parsing
- Verify model definitions compile to WorkflowIR
- Test default router behavior
- Expected: All model references resolve correctly

**Phase 3 Tests**:
- Test provider abstraction layer
- Verify all providers work with same workflow
- Test model switching without workflow changes
- Expected: Seamless provider transitions

**Acceptance Criteria**:
- [ ] Schema validation passes
- [ ] Multi-model loading works
- [ ] Provider abstraction normalizes differences
- [ ] Resource limits enforced
- [ ] Default routing selects appropriate model

---

### 02. 02-model-parameters-tuning.yaml

**Path**: `01-model-configuration/02-model-parameters-tuning.yaml`

**Owner Phases**: Phase 0, Phase 2

**Features Tested**:
- Model parameter overrides
- Temperature tuning
- Top-p configuration
- Max tokens
- Model-level tool permissions
- Per-step model overrides

**Phase 0 Tests**:
- Validate parameter schema
- Verify parameter inheritance
- Test parameter validation ranges
- Expected: All parameters within valid ranges

**Phase 2 Tests**:
- Test parameter application at runtime
- Verify per-step overrides
- Test model behavior with different parameters
- Expected: Parameters affect model output

**Acceptance Criteria**:
- [ ] All model parameters valid
- [ ] Inheritance hierarchy correct
- [ ] Per-step overrides apply
- [ ] Parameter ranges enforced
- [ ] Model behavior reflects parameters

---

### 03. 03-model-lifecycle-management.yaml

**Path**: `01-model-configuration/03-model-lifecycle-management.yaml`

**Owner Phases**: Phase 0, Phase 1

**Features Tested**:
- Model lifecycle hooks (on_create, on_run_start, on_run_complete, on_run_error)
- Model loading/unloading
- Memory management (cache_size, kv_cache_quantization)
- Warmup and cooldown
- Multiple model instances

**Phase 0 Tests**:
- Validate lifecycle hook schema
- Verify all hook points defined
- Test hook compilation
- Expected: All hooks compile to IR

**Phase 1 Tests**:
- Test model lifecycle transitions
- Verify hooks fire at correct times
- Test memory management
- Expected: Lifecycle state machine correct

**Acceptance Criteria**:
- [ ] All hooks defined and accessible
- [ ] Hooks fire at correct lifecycle points
- [ ] Model states transition correctly
- [ ] Memory managed efficiently
- [ ] Multiple model instances work

---

### 04. 04-cost-tracking-budgets.yaml

**Path**: `01-model-configuration/04-cost-tracking-budgets.yaml`

**Owner Phases**: Phase 0, Phase 2

**Features Tested**:
- Resource allocation strategies
- Cost tracking (tokens, compute time)
- Budget limits
- Resource enforcement
- Allocation strategies (static, dynamic, adaptive)

**Phase 0 Tests**:
- Validate budget schema
- Verify resource limit definitions
- Test allocation strategy configuration
- Expected: All budget fields valid

**Phase 2 Tests**:
- Test resource allocation at runtime
- Verify budget enforcement
- Test cost tracking accuracy
- Expected: Budgets respected, costs tracked

**Acceptance Criteria**:
- [ ] Budget schema valid
- [ ] Allocation strategies work
- [ ] Resource limits enforced
- [ ] Costs tracked accurately
- [ ] Overspend prevented

---

## Category 02: Step Types (4 Workflows)

**Testing Phases**: Phase 1 (MVP Execution), Phase 2 (Advanced Steps)

### 05. 01-llm-inference-steps.yaml

**Path**: `02-step-types/01-llm-inference-steps.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Agent step type
- LLM inference
- Prompt configuration
- Output capture
- Variable interpolation
- Context injection

**Phase 1 Tests**:
- Test agent step execution
- Verify prompt rendering
- Test output capture
- Expected: LLM inference works correctly

**Acceptance Criteria**:
- [ ] Agent steps execute
- [ ] Prompts render with variables
- [ ] LLM inference succeeds
- [ ] Output captured correctly
- [ ] Context injected as configured

---

### 06. 02-code-execution-steps.yaml

**Path**: `02-step-types/02-code-execution-steps.yaml`

**Owner Phases**: Phase 1, Phase 2

**Features Tested**:
- Code execution step type
- Language support (Rust, Python)
- Environment isolation
- Capture stdout/stderr
- Error handling

**Phase 1 Tests**:
- Test code step execution
- Verify language handling
- Test output capture
- Expected: Code executes and returns result

**Phase 2 Tests**:
- Test environment isolation
- Verify error handling
- Test resource limits
- Expected: Isolated execution, proper error handling

**Acceptance Criteria**:
- [ ] Code steps execute
- [ ] Multiple languages supported
- [ ] Environments isolated
- [ ] Output captured
- [ ] Errors handled gracefully

---

### 07. 03-tool-invocation-steps.yaml

**Path**: `02-step-types/03-tool-invocation-steps.yaml`

**Owner Phases**: Phase 1, Phase 2

**Features Tested**:
- Tool step type
- Built-in tools (grep, file_read, file_write)
- Custom tool definitions
- Tool permissions
- Tool result capture

**Phase 1 Tests**:
- Test tool invocation
- Verify built-in tools work
- Test result capture
- Expected: Tools execute and return results

**Phase 2 Tests**:
- Test custom tool definitions
- Verify tool permissions
- Test tool error handling
- Expected: Custom tools work, permissions enforced

**Acceptance Criteria**:
- [ ] Built-in tools work
- [ ] Custom tools can be defined
- [ ] Tool results captured
- [ ] Permissions enforced
- [ ] Tool errors handled

---

### 08. 04-hybrid-step-workflows.yaml

**Path**: `02-step-types/04-hybrid-step-workflows.yaml`

**Owner Phases**: Phase 1, Phase 2

**Features Tested**:
- Hybrid step type (agent + tool)
- Mixed execution patterns
- Conditional tool use
- Dynamic step configuration

**Phase 1 Tests**:
- Test hybrid step execution
- Verify agent + tool coordination
- Test dynamic configuration
- Expected: Hybrid steps work seamlessly

**Phase 2 Tests**:
- Test conditional tool invocation
- Verify agent-tool interaction
- Test error handling
- Expected: Conditional logic works, errors handled

**Acceptance Criteria**:
- [ ] Hybrid steps execute
- [ ] Agent and tool coordination works
- [ ] Conditional tool use functions
- [ ] Dynamic configuration applied
- [ ] Errors handled appropriately

---

## Category 03: Data Flow (3 Workflows)

**Testing Phases**: Phase 0 (Foundation), Phase 1 (Data Flow)

### 09. 01-workflow-level-variables.yaml

**Path**: `03-data-flow/01-workflow-level-variables.yaml`

**Owner Phases**: Phase 0

**Features Tested**:
- Workflow variable definitions
- Variable interpolation `${workflow.xxx}`
- Variable scope and access
- Default values
- Type validation

**Phase 0 Tests**:
- Test workflow variable parsing
- Verify variable interpolation
- Test variable scope rules
- Expected: Variables resolve correctly at parse time

**Acceptance Criteria**:
- [ ] Workflow variables defined
- [ ] Interpolation syntax works
- [ ] Scope rules enforced
- [ ] Type validation applied
- [ ] Default values used

---

### 10. 02-step-outputs-as-input.yaml

**Path**: `03-data-flow/02-step-outputs-as-input.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Step output capture
- Output as input to next step
- Variable interpolation `{{step.step_id.output}}`
- Output formatting
- Field selection

**Phase 1 Tests**:
- Test output capture
- Verify output as input works
- Test field selection
- Expected: Outputs pass between steps correctly

**Acceptance Criteria**:
- [ ] Step outputs captured
- [ ] Output-to-input works
- [ ] Variable interpolation correct
- [ ] Output format applied
- [ ] Field selection works

---

### 11. 03-context-injection-previous-outputs.yaml

**Path**: `03-data-flow/03-context-injection-previous-outputs.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Context injection configuration
- Previous outputs inclusion
- System context
- Tool results in context
- Context window management

**Phase 1 Tests**:
- Test context injection
- Verify previous outputs included
- Test context window limits
- Expected: Context assembled correctly

**Acceptance Criteria**:
- [ ] Context injection works
- [ ] Previous outputs included
- [ ] System context added
- [ ] Tool results injected
- [ ] Context window managed

---

## Category 04: Parallel Execution (3 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 12. 01-parallel-groups-execution.yaml

**Path**: `04-parallel-execution/01-parallel-groups-execution.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Parallel step groups
- `parallel_group` configuration
- Concurrency limits
- Load balancing
- Synchronization

**Phase 1 Tests**:
- Test parallel group execution
- Verify concurrency limits
- Test synchronization
- Expected: Steps run in parallel, limits respected

**Acceptance Criteria**:
- [ ] Parallel groups execute
- [ ] Concurrency limits enforced
- [ ] Load balancing works
- [ ] Synchronization correct
- [ ] Dependencies respected

---

### 13. 02-resource-concurrency-control.yaml

**Path**: `04-parallel-execution/02-resource-concurrency-control.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Resource-based concurrency limits
- Memory constraints
- CPU/GPU scheduling
- Max concurrent requests
- Resource allocation

**Phase 1 Tests**:
- Test resource-based limits
- Verify memory constraints
- Test CPU/GPU scheduling
- Expected: Resources allocated correctly

**Acceptance Criteria**:
- [ ] Resource limits enforced
- [ ] Memory constraints respected
- [ ] CPU/GPU scheduled correctly
- [ ] Max requests limited
- [ ] Allocation efficient

---

### 14. 03-load-balancing-strategies.yaml

**Path**: `04-parallel-execution/03-load-balancing-strategies.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Load balancing algorithms (round_robin, priority_queue, shortest_job_first)
- Worker selection
- Least loaded strategy
- IP hashing
- Performance optimization

**Phase 1 Tests**:
- Test load balancing algorithms
- Verify worker selection
- Test performance
- Expected: Load distributed evenly

**Acceptance Criteria**:
- [ ] All algorithms work
- [ ] Worker selection correct
- [ ] Load balanced
- [ ] Performance optimal
- [ ] No resource starvation

---

## Category 05: Loops & Convergence (4 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 15. 01-for-loops-explicit-iteration.yaml

**Path**: `05-loops-convergence/01-for-loops-explicit-iteration.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Count-based loops
- For loops
- Foreach loops
- While loops
- Validation loops
- Retry loops
- Infinite loops

**Phase 1 Tests**:
- Test all loop types
- Verify iteration control
- Test termination conditions
- Expected: All loop types work correctly

**Acceptance Criteria**:
- [ ] Count loops iterate correctly
- [ ] For loops work
- [ ] Foreach loops iterate collections
- [ ] While loops evaluate conditions
- [ ] Validation loops converge
- [ ] Retry loops backoff correctly
- [ ] Infinite loops have safety limits

---

### 16. 02-foreach-loops-data-collections.yaml

**Path**: `05-loops-convergence/02-foreach-loops-data-collections.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Foreach over arrays
- Foreach over object keys
- Iteration variable
- Collection exhaustion
- Error handling

**Phase 1 Tests**:
- Test foreach loops
- Verify iteration over arrays
- Test iteration over objects
- Expected: All elements processed

**Acceptance Criteria**:
- [ ] Arrays iterated completely
- [ ] Object keys iterated
- [ ] Iteration variable correct
- [ ] Collection exhaustion detected
- [ ] Errors handled

---

### 17. 03-while-loops-conditional-termination.yaml

**Path**: `05-loops-convergence/03-while-loops-conditional-termination.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- While loops with conditions
- Conditional expressions
- Loop termination
- Prevent infinite loops
- Condition evaluation

**Phase 1 Tests**:
- Test while loops
- Verify condition evaluation
- Test termination
- Expected: Loops terminate when condition false

**Acceptance Criteria**:
- [ ] While conditions evaluated
- [ ] Loop terminates correctly
- [ ] No infinite loops
- [ ] Conditions support all operators
- [ ] Performance acceptable

---

### 18. 04-convergence-reduction-aggregation.yaml

**Path**: `05-loops-convergence/04-convergence-reduction-aggregation.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Validation loops
- Convergence criteria
- Exact criteria
- Abstract criteria
- Tolerance thresholds
- Aggregation strategies

**Phase 1 Tests**:
- Test validation loops
- Verify convergence detection
- Test aggregation
- Expected: Loops converge, results aggregated

**Acceptance Criteria**:
- [ ] Validation loops work
- [ ] Convergence detected
- [ ] Exact criteria enforced
- [ ] Abstract criteria evaluated
- [ ] Tolerance respected
- [ ] Aggregation correct

---

## Category 06: File Operations (2 Workflows)

**Testing Phase**: Phase 2 (MVP Execution)

### 19. 01-file-read-write-batch.yaml

**Path**: `06-file-operations/01-file-read-write-batch.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- File read operations
- File write operations
- Batch processing
- File formats (JSON, YAML, text)
- Encoding handling
- Error handling

**Phase 2 Tests**:
- Test file reads
- Test file writes
- Test batch operations
- Expected: All file operations work

**Acceptance Criteria**:
- [ ] Files read correctly
- [ ] Files written correctly
- [ ] Batch processing works
- [ ] Formats handled
- [ ] Encoding correct
- [ ] Errors handled

---

### 20. 02-file-permissions-backup.yaml

**Path**: `06-file-operations/02-file-permissions-backup.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- File permissions (read, write, delete)
- Backup existing files
- Confirmation prompts
- Allowed/forbidden paths
- Permission levels

**Phase 2 Tests**:
- Test permission enforcement
- Verify backup creation
- Test confirmation prompts
- Expected: Permissions enforced, backups created

**Acceptance Criteria**:
- [ ] Permissions enforced
- [ ] Backups created before writes
- [ ] Confirmations shown
- [ ] Allowed paths respected
- [ ] Forbidden paths blocked
- [ ] Permission levels correct

---

## Category 07: Web Operations (3 Workflows)

**Testing Phases**: Phase 2 (MVP Execution), Phase 5 (Advanced Web)

### 21. 01-web-fetch-scrape.yaml

**Path**: `07-web-operations/01-web-fetch-scrape.yaml`

**Owner Phases**: Phase 2, Phase 5

**Features Tested**:
- Web fetch operations
- Web scraping
- HTML parsing
- Content extraction
- Robots.txt compliance
- Timeout handling

**Phase 2 Tests**:
- Test web fetch
- Verify timeout handling
- Test error handling
- Expected: Web fetch works within limits

**Phase 5 Tests**:
- Test web scraping
- Verify robots.txt compliance
- Test content extraction
- Expected: Scraping compliant, content extracted

**Acceptance Criteria**:
- [ ] Web fetch works
- [ ] Scraping extracts content
- [ ] HTML parsing correct
- [ ] Robots.txt respected
- [ ] Timeouts enforced
- [ ] Errors handled

---

### 22. 02-api-integration-rest.yaml

**Path**: `07-web-operations/02-api-integration-rest.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- REST API integration
- HTTP methods (GET, POST, PUT, DELETE)
- Headers and authentication
- Request/response handling
- Rate limiting
- Error handling

**Phase 2 Tests**:
- Test all HTTP methods
- Verify authentication
- Test rate limiting
- Expected: API calls succeed, limits respected

**Acceptance Criteria**:
- [ ] All HTTP methods work
- [ ] Authentication succeeds
- [ ] Headers sent correctly
- [ ] Responses parsed
- [ ] Rate limits enforced
- [ ] Errors handled

---

### 23. 03-url-parameters-requests.yaml

**Path**: `07-web-operations/03-url-parameters-requests.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- URL parameters
- Query string handling
- URL encoding/decoding
- Custom headers
- SSL validation
- Follow redirects

**Phase 2 Tests**:
- Test URL parameters
- Verify query strings
- Test SSL validation
- Expected: URLs handled correctly

**Acceptance Criteria**:
- [ ] URL parameters sent
- [ ] Query strings encoded
- [ ] Headers applied
- [ ] SSL validated
- [ ] Redirects followed
- [ ] Errors handled

---

## Category 08: RAG Operations (2 Workflows)

**Testing Phases**: Phase 2 (Basic RAG), Phase 5 (Advanced RAG)

### 24. 01-document-indexing-retrieval.yaml

**Path**: `08-rag-operations/01-document-indexing-retrieval.yaml`

**Owner Phases**: Phase 2, Phase 5

**Features Tested**:
- Document indexing
- Embedding generation
- Vector storage
- Retrieval queries
- Similarity search
- Context injection

**Phase 2 Tests**:
- Test document indexing
- Verify embedding generation
- Test retrieval
- Expected: Documents indexed, retrieved correctly

**Phase 5 Tests**:
- Test advanced retrieval
- Verify hybrid search
- Test performance optimization
- Expected: Fast, accurate retrieval

**Acceptance Criteria**:
- [ ] Documents indexed
- [ ] Embeddings generated
- [ ] Vectors stored
- [ ] Retrieval works
- [ ] Similarity search accurate
- [ ] Context injected

---

### 25. 02-rag-generation-context-aware.yaml

**Path**: `08-rag-operations/02-rag-generation-context-aware.yaml`

**Owner Phases**: Phase 2, Phase 5

**Features Tested**:
- Context-aware generation
- Retrieval result formatting
- Citation handling
- Context window management
- Re-ranking

**Phase 2 Tests**:
- Test context-aware generation
- Verify retrieval formatting
- Test citation inclusion
- Expected: LLM uses retrieved context correctly

**Phase 5 Tests**:
- Test advanced context management
- Verify re-ranking
- Test multi-source retrieval
- Expected: Optimal context selection

**Acceptance Criteria**:
- [ ] Generation context-aware
- [ ] Results formatted correctly
- [ ] Citations included
- [ ] Context window managed
- [ ] Re-ranking improves results

---

## Category 09: Script & CLI (2 Workflows)

**Testing Phase**: Phase 2 (MVP Execution)

### 26. 01-script-execution.yaml

**Path**: `09-script-cli/01-script-execution.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- Script execution (Bash, Python, JavaScript)
- Working directory
- Environment variables
- Timeout handling
- Output capture
- Error handling

**Phase 2 Tests**:
- Test script execution
- Verify environment setup
- Test output capture
- Expected: Scripts run correctly

**Acceptance Criteria**:
- [ ] Scripts execute
- [ ] Working directory set
- [ ] Environment variables passed
- [ ] Timeouts enforced
- [ ] Output captured
- [ ] Errors handled

---

### 27. 02-cli-commands-environment.yaml

**Path**: `09-script-cli/02-cli-commands-environment.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- CLI command execution
- Command validation
- Allowed commands
- Forbidden commands
- Shell environment
- Working directories

**Phase 2 Tests**:
- Test CLI command execution
- Verify command validation
- Test allowed/forbidden lists
- Expected: Commands execute, invalid blocked

**Acceptance Criteria**:
- [ ] CLI commands run
- [ ] Commands validated
- [ ] Allowed commands execute
- [ ] Forbidden commands blocked
- [ ] Shell environment correct
- [ ] Working directories respected

---

## Category 10: Sub-Workflows (2 Workflows)

**Testing Phase**: Phase 2 (MVP Execution)

### 28. 01-nested-workflow-references.yaml

**Path**: `10-sub-workflows/01-nested-workflow-references.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- Nested workflow references
- Sub-workflow invocation
- Parameter passing
- Output capture
- Circular reference detection

**Phase 2 Tests**:
- Test sub-workflow invocation
- Verify parameter passing
- Test output capture
- Expected: Sub-workflows work correctly

**Acceptance Criteria**:
- [ ] Sub-workflows invoked
- [ ] Parameters passed
- [ ] Outputs captured
- [ ] Circular references detected
- [ ] Error handling correct

---

### 29. 02-workflow-composition-patterns.yaml

**Path**: `10-sub-workflows/02-workflow-composition-patterns.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- Workflow composition
- Shared sub-workflows
- Library workflows
- Reusable patterns
- Composition validation

**Phase 2 Tests**:
- Test workflow composition
- Verify reusability
- Test composition validation
- Expected: Workflows compose correctly

**Acceptance Criteria**:
- [ ] Workflows compose
- [ ] Shared sub-workflows work
- [ ] Library patterns reusable
- [ ] Composition valid
- [ ] Errors detected

---

## Category 11: Conditional Branching (2 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 30. 01-event-based-branching.yaml

**Path**: `11-conditional-branching/01-event-based-branching.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Event-based branching
- Branch conditions
- Event types (validation, interdependency, checkpoint, failure)
- Branch execution
- Default branch

**Phase 1 Tests**:
- Test event-based branches
- Verify condition evaluation
- Test branch selection
- Expected: Branches execute based on events

**Acceptance Criteria**:
- [ ] Events trigger branches
- [ ] Conditions evaluated
- [ ] Branches execute
- [ ] Default branch works
- [ ] No infinite loops

---

### 31. 02-decision-logic-workflows.yaml

**Path**: `11-conditional-branching/02-decision-logic-workflows.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Decision logic
- Boolean expressions
- Comparison operators
- Logical operators (AND, OR, NOT)
- Complex conditions

**Phase 1 Tests**:
- Test decision logic
- Verify operators
- Test complex conditions
- Expected: Decisions correct

**Acceptance Criteria**:
- [ ] Boolean expressions work
- [ ] All operators function
- [ ] Complex conditions evaluated
- [ ] Decision paths correct
- [ ] No ambiguity

---

## Category 12: Error Handling & Retries (3 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 32. 01-retry-strategies-backoff.yaml

**Path**: `12-error-handling-retries/01-retry-strategies-backoff.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Retry strategies (exponential, linear, fixed)
- Max attempts
- Backoff configuration
- Retry conditions
- Error classification

**Phase 1 Tests**:
- Test retry strategies
- Verify backoff calculations
- Test error classification
- Expected: Retries work correctly

**Acceptance Criteria**:
- [ ] All strategies work
- [ ] Backoff calculations correct
- [ ] Max attempts enforced
- [ ] Retry conditions evaluated
- [ ] Errors classified

---

### 33. 02-error-propagation-escalation.yaml

**Path**: `12-error-handling-retries/02-error-propagation-escalation.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Error propagation
- Escalation paths
- Error types (transient, permanent, user, system)
- Escalation triggers
- Human notification

**Phase 1 Tests**:
- Test error propagation
- Verify escalation
- Test notification
- Expected: Errors propagate and escalate

**Acceptance Criteria**:
- [ ] Errors propagate
- [ ] Escalation paths work
- [ ] Error types distinguished
- [ ] Triggers fire
- [ ] Notifications sent

---

### 34. 03-graceful-failure-recovery.yaml

**Path**: `12-error-handling-retries/03-graceful-failure-recovery.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Graceful failure handling
- Fallback actions
- Partial results
- Recovery workflows
- State preservation

**Phase 1 Tests**:
- Test graceful failures
- Verify fallbacks
- Test state preservation
- Expected: Failures handled gracefully

**Acceptance Criteria**:
- [ ] Failures graceful
- [ ] Fallbacks work
- [ ] Partial results returned
- [ ] Recovery succeeds
- [ ] State preserved

---

## Category 13: Logging & Monitoring (3 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 35. 01-hierarchical-logging-system.yaml

**Path**: `13-logging-monitoring/01-hierarchical-logging-system.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Hierarchical logging
- Log levels (debug, info, warning, error)
- Log scopes (workflow, pipeline, models, tools, execution)
- Output destinations (console, file, remote)
- Log rotation

**Phase 1 Tests**:
- Test log levels
- Verify log scopes
- Test output destinations
- Expected: All logging works correctly

**Acceptance Criteria**:
- [ ] Log levels work
- [ ] Hierarchical scopes correct
- [ ] All destinations work
- [ ] Rotation functions
- [ ] Performance acceptable

---

### 36. 02-metrics-collection.yaml

**Path**: `13-logging-monitoring/02-metrics-collection.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Metrics collection
- Pipeline metrics
- Step metrics
- Model metrics
- Tool metrics
- Custom metrics

**Phase 1 Tests**:
- Test all metric types
- Verify metric accuracy
- Test metric storage
- Expected: All metrics collected

**Acceptance Criteria**:
- [ ] Pipeline metrics collected
- [ ] Step metrics collected
- [ ] Model metrics collected
- [ ] Tool metrics collected
- [ ] Custom metrics work
- [ ] Storage correct

---

### 37. 03-structured-output-formats.yaml

**Path**: `13-logging-monitoring/03-structured-output-formats.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Structured output formats (JSON, YAML, Markdown)
- Output schemas
- Field selection
- Formatting options
- Validation

**Phase 1 Tests**:
- Test all formats
- Verify schemas
- Test field selection
- Expected: Outputs formatted correctly

**Acceptance Criteria**:
- [ ] JSON format correct
- [ ] YAML format correct
- [ ] Markdown format correct
- [ ] Schemas validated
- [ ] Fields selected
- [ ] Formatting options work

---

## Category 14: Checkpointing & State (2 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 38. 01-checkpointing-save-restore.yaml

**Path**: `14-checkpointing-state/01-checkpointing-save-restore.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Checkpoint creation
- State serialization
- State restoration
- Checkpoint intervals
- Checkpoint triggers
- Cleanup policies

**Phase 1 Tests**:
- Test checkpoint creation
- Verify state serialization
- Test restoration
- Expected: Checkpoints work correctly

**Acceptance Criteria**:
- [ ] Checkpoints created
- [ ] State serialized correctly
- [ ] Restoration works
- [ ] Intervals respected
- [ ] Triggers fire
- [ ] Cleanup functions

---

### 39. 02-state-management.yaml

**Path**: `14-checkpointing-state/02-state-management.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- State management
- State updates
- State queries
- State versioning
- Delta updates
- Integrity validation

**Phase 1 Tests**:
- Test state updates
- Verify state queries
- Test versioning
- Expected: State managed correctly

**Acceptance Criteria**:
- [ ] State updates work
- [ ] Queries return correct state
- [ ] Versioning functions
- [ ] Deltas applied
- [ ] Integrity validated

---

## Category 15: Resource Management (3 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 40. 01-memory-allocation-strategies.yaml

**Path**: `15-resource-management/01-memory-allocation-strategies.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Memory allocation strategies (static, dynamic, adaptive)
- RAM allocation
- VRAM allocation
- Allocation limits
- Pressure handling

**Phase 1 Tests**:
- Test allocation strategies
- Verify memory limits
- Test pressure handling
- Expected: Memory allocated correctly

**Acceptance Criteria**:
- [ ] All strategies work
- [ ] RAM allocation correct
- [ ] VRAM allocation correct
- [ ] Limits enforced
- [ ] Pressure handled

---

### 41. 02-cpu-gpu-scheduling.yaml

**Path**: `15-resource-management/02-cpu-gpu-scheduling.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- CPU scheduling
- GPU scheduling
- Load balancing
- Priority queues
- Preemption
- Time slicing

**Phase 1 Tests**:
- Test CPU scheduling
- Verify GPU scheduling
- Test load balancing
- Expected: Resources scheduled correctly

**Acceptance Criteria**:
- [ ] CPU scheduling works
- [ ] GPU scheduling works
- [ ] Load balanced
- [ ] Priorities respected
- [ ] Preemption correct

---

### 42. 03-resource-limits-throttling.yaml

**Path**: `15-resource-management/03-resource-limits-throttling.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Resource limits
- Throttling strategies
- Rate limiting
- Backpressure handling
- Resource enforcement

**Phase 1 Tests**:
- Test resource limits
- Verify throttling
- Test rate limiting
- Expected: Resources limited correctly

**Acceptance Criteria**:
- [ ] Limits enforced
- [ ] Throttling works
- [ ] Rate limiting functional
- [ ] Backpressure handled
- [ ] Enforcement consistent

---

## Category 16: Tool Permissions (2 Workflows)

**Testing Phase**: Phase 2 (MVP Execution)

### 43. 01-allow-deny-lists-scopes.yaml

**Path**: `16-tool-permissions/01-allow-deny-lists-scopes.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- Allow lists
- Deny lists
- Scope restrictions
- Permission levels
- Per-step permissions

**Phase 2 Tests**:
- Test allow lists
- Verify deny lists
- Test scopes
- Expected: Permissions enforced correctly

**Acceptance Criteria**:
- [ ] Allow lists work
- [ ] Deny lists work
- [ ] Scopes enforced
- [ ] Levels respected
- [ ] Per-step permissions apply

---

### 44. 02-fine-grained-step-control.yaml

**Path**: `16-tool-permissions/02-fine-grained-step-control.yaml`

**Owner Phases**: Phase 2

**Features Tested**:
- Fine-grained permissions
- Step-level control
- Tool-specific settings
- Permission inheritance
- Permission override

**Phase 2 Tests**:
- Test fine-grained permissions
- Verify step-level control
- Test inheritance
- Expected: Fine-grained control works

**Acceptance Criteria**:
- [ ] Fine-grained permissions work
- [ ] Step-level control correct
- [ ] Tool-specific settings applied
- [ ] Inheritance works
- [ ] Override functions

---

## Category 17: User Inputs & UI (2 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 45. 01-user-input-prompts-validation.yaml

**Path**: `17-user-inputs-ui/01-user-input-prompts-validation.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- User input prompts
- Input types (text_area, text_line, select, multi_select, confirm)
- Validation rules
- Default values
- Keyboard shortcuts

**Phase 1 Tests**:
- Test all input types
- Verify validation
- Test default values
- Expected: User inputs work correctly

**Acceptance Criteria**:
- [ ] All input types work
- [ ] Validation enforced
- [ ] Defaults used
- [ ] Shortcuts work
- [ ] UI responsive

---

### 46. 02-interactive-user-feedback.yaml

**Path**: `17-user-inputs-ui/02-interactive-user-feedback.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Interactive feedback
- User confirmations
- Progress updates
- Status displays
- Cancel operations

**Phase 1 Tests**:
- Test interactive feedback
- Verify confirmations
- Test cancel
- Expected: Interactive flows work

**Acceptance Criteria**:
- [ ] Feedback shown
- [ ] Confirmations work
- [ ] Progress updates accurate
- [ ] Status displays correct
- [ ] Cancel functions

---

## Category 18: Hooks & Lifecycle (4 Workflows)

**Testing Phase**: Phase 1 (MVP Execution)

### 47. 01-pre-workflow-post-workflow-hooks.yaml

**Path**: `18-hooks-lifecycle/01-pre-workflow-post-workflow-hooks.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Pre-workflow hooks
- Post-workflow hooks
- Workflow lifecycle events
- Hook actions (log, checkpoint, notify)
- Error handling hooks

**Phase 1 Tests**:
- Test pre-workflow hooks
- Verify post-workflow hooks
- Test hook actions
- Expected: All hooks fire correctly

**Acceptance Criteria**:
- [ ] Pre-workflow hooks fire
- [ ] Post-workflow hooks fire
- [ ] Lifecycle events correct
- [ ] Actions executed
- [ ] Errors handled

---

### 48. 02-step-level-hooks.yaml

**Path**: `18-hooks-lifecycle/02-step-level-hooks.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Step-level hooks
- Pre-step hooks
- Post-step hooks
- Step lifecycle events
- Hook parameter passing

**Phase 1 Tests**:
- Test step-level hooks
- Verify lifecycle events
- Test parameter passing
- Expected: Step hooks work correctly

**Acceptance Criteria**:
- [ ] Pre-step hooks fire
- [ ] Post-step hooks fire
- [ ] Lifecycle events correct
- [ ] Parameters passed
- [ ] Errors handled

---

### 49. 03-error-handling-hooks.yaml

**Path**: `18-hooks-lifecycle/03-error-handling-hooks.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Error handling hooks
- On-error actions
- Error context
- Recovery workflows
- Notification hooks

**Phase 1 Tests**:
- Test error handling hooks
- Verify on-error actions
- Test recovery
- Expected: Error hooks work correctly

**Acceptance Criteria**:
- [ ] Error hooks fire
- [ ] On-error actions execute
- [ ] Context preserved
- [ ] Recovery succeeds
- [ ] Notifications sent

---

### 50. 04-lifecycle-events.yaml

**Path**: `18-hooks-lifecycle/04-lifecycle-events.yaml`

**Owner Phases**: Phase 1

**Features Tested**:
- Lifecycle event emissions
- Event types (create, run, complete, error)
- Event ordering
- Event metadata
- Event subscriptions

**Phase 1 Tests**:
- Test lifecycle events
- Verify event ordering
- Test metadata
- Expected: All events emitted correctly

**Acceptance Criteria**:
- [ ] All lifecycle events emit
- [ ] Ordering correct
- [ ] Metadata complete
- [ ] Subscriptions work
- [ ] Performance acceptable

---

## Category 19: Comprehensive Integration (2 Workflows)

**Testing Phase**: Phase 8 (Final Validation)

### 51. 01-complex-orchestration-sub-agents.yaml

**Path**: `19-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`

**Owner Phases**: Phase 8

**Features Tested**:
- Complex orchestration
- Sub-agent spawning
- Interdependent validation
- Event propagation
- Aggregation strategies
- Comprehensive integration

**Phase 8 Tests**:
- Test full orchestration
- Verify sub-agent coordination
- Test validation aggregation
- Expected: Complete system works end-to-end

**Acceptance Criteria**:
- [ ] Orchestration complete
- [ ] Sub-agents coordinate
- [ ] Interdependency works
- [ ] Events propagate
- [ ] Aggregation correct
- [ ] System stable

---

### 52. 02-guardrails-content-safety.yaml

**Path**: `19-comprehensive-integration/02-guardrails-content-safety.yaml`

**Owner Phases**: Phase 8

**Features Tested**:
- Guardrails (input, output, tool_use)
- Content safety
- PII detection
- Toxicity filtering
- Policy enforcement
- Comprehensive safety

**Phase 8 Tests**:
- Test all guardrail types
- Verify content safety
- Test policy enforcement
- Expected: All guardrails work correctly

**Acceptance Criteria**:
- [ ] Input guardrails active
- [ ] Output guardrails active
- [ ] Tool use guardrails active
- [ ] Content detected
- [ ] PII filtered
- [ ] Toxicity blocked
- [ ] Policy enforced

---

## Manual Brainstorm (1 Workflow)

### 53. agentic-workflow-manual-brainstorm.yml

**Path**: `manual/agentic-workflow-manual-brainstorm.yml`

**Owner Phases**: All phases (documentation reference)

**Features Tested**:
- Manual brainstorm example
- All schema features
- Comprehensive coverage
- Human-readable reference

**Tests**:
- Verify all schema features documented
- Validate completeness
- Test comprehensibility
- Expected: Complete, readable reference

**Acceptance Criteria**:
- [ ] All features documented
- [ ] Coverage complete
- [ ] Human-readable
- [ ] Examples clear
- [ ] Reference useful

---

## Summary Matrix

| Category | Workflows | Owner Phases | Status |
|----------|-----------|--------------|--------|
| 01 - Model Configuration | 4 | Phase 0, 2, 3 | ✅ Mapped |
| 02 - Step Types | 4 | Phase 1, 2 | ✅ Mapped |
| 03 - Data Flow | 3 | Phase 0, 1 | ✅ Mapped |
| 04 - Parallel Execution | 3 | Phase 1 | ✅ Mapped |
| 05 - Loops & Convergence | 4 | Phase 1 | ✅ Mapped |
| 06 - File Operations | 2 | Phase 2 | ✅ Mapped |
| 07 - Web Operations | 3 | Phase 2, 5 | ✅ Mapped |
| 08 - RAG Operations | 2 | Phase 2, 5 | ✅ Mapped |
| 09 - Script & CLI | 2 | Phase 2 | ✅ Mapped |
| 10 - Sub-Workflows | 2 | Phase 2 | ✅ Mapped |
| 11 - Conditional Branching | 2 | Phase 1 | ✅ Mapped |
| 12 - Error Handling | 3 | Phase 1 | ✅ Mapped |
| 13 - Logging & Monitoring | 3 | Phase 1 | ✅ Mapped |
| 14 - Checkpointing & State | 2 | Phase 1 | ✅ Mapped |
| 15 - Resource Management | 3 | Phase 1 | ✅ Mapped |
| 16 - Tool Permissions | 2 | Phase 2 | ✅ Mapped |
| 17 - User Inputs & UI | 2 | Phase 1 | ✅ Mapped |
| 18 - Hooks & Lifecycle | 4 | Phase 1 | ✅ Mapped |
| 19 - Comprehensive Integration | 2 | Phase 8 | ✅ Mapped |
| Manual - Brainstorm | 1 | All phases | ✅ Mapped |

**Total Workflows**: 53
**Total Categories**: 19
**Coverage**: 100% (All workflows mapped to testing phases)
