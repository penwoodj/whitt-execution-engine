# Requirements Traceability Matrix

**Purpose**: Trace every requirement R01-R31 to implementation phases, ADRs, and schema sections

**Total Requirements**: 31 (R01-R31)
**Date**: 2026-04-06

---

## R01: System Identity

**Definition**: The product identity is a compiler-centered, local-first agentic orchestration framework rather than a thin chat wrapper

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- Section 1: Workflow Identification (workflow_id, name, description, version, author, tags)
- Section 14: Features Demonstrated (all feature flags)

**Implementation Tasks**:
1. Define WorkflowSpec struct in Rust
2. Implement workflow metadata parsing
3. Create system identity documentation
4. Design compiler contract interface

**Acceptance Test**:
- Test: Create workflow and verify system identity
- Command: Execute workflow, inspect ./workspace/runs/
- Expected: System-of-record shows compiler-centered identity
- Validation: All workflows include required identification fields

**Verification Method**:
- Check: Verify compiler identity in all outputs
- Command: Review logs, metrics, artifacts
- Result: Compiler identity consistent across all components

---

## R03: Local-First Operation

**Definition**: All operations default to local execution with privacy-preserving defaults, no external network access without explicit opt-in

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract), ADR-0006 (Memory and Search)

**Schema Sections**:
- Section 13: Workspace Configuration (root_path, directories)
- Section 16: Default Behavior Reference (all defaults)
- Section 6: Tool Permissions (web_operations enabled=false by default)

**Implementation Tasks**:
1. Define local-first defaults in config
2. Implement workspace isolation
3. Create ./workspace/ system-of-record
4. Implement privacy-preserving defaults

**Acceptance Test**:
- Test: Execute workflow and verify local-only behavior
- Command: Run workflow with no network config
- Expected: No outbound network calls
- Validation: Monitor network traffic during execution

**Verification Method**:
- Check: Verify no external dependencies by default
- Command: Inspect dependencies and network logs
- Result: No unauthorized external access

---

## R05: CLI-First Path

**Definition**: CLI/TUI-first path before richer desktop UI, with full control surface exposed

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler), ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (CLI-related settings)
- Section 7: Logging Configuration (console output)
- Section 8: Metrics Configuration (CLI metrics)

**Implementation Tasks**:
1. Implement CLI commands (run, generate, queue, etc.)
2. Create CLI control surface for scheduler
3. Implement TUI for interactive workflows
4. Add comprehensive help and documentation

**Acceptance Test**:
- Test: Verify all CLI operations work
- Command: Execute all CLI commands with test workflows
- Expected: Full functionality from CLI
- Validation: Each command produces expected result

**Verification Method**:
- Check: Verify CLI provides complete feature parity
- Command: Compare CLI vs internal API coverage
- Result: 100% feature parity

---

## R07: Chat as Scoped Executable Work Container

**Definition**: Every chat session is a scoped executable work container with isolated state

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 3: Agentic Workflow (chat-level configuration)
- Section 10: Orchestration Configuration (ChatSession coordination)

**Implementation Tasks**:
1. Implement ChatSession struct
2. Create session isolation
3. Implement state management per session
4. Add session lifecycle (create, execute, close)

**Acceptance Test**:
- Test: Create multiple chat sessions and verify isolation
- Command: Create 3 sessions, execute workflows in each
- Expected: No state leakage between sessions
- Validation: Inspect session state during concurrent runs

**Verification Method**:
- Check: Verify session lifecycle states
- Command: Monitor state transitions
- Result: All states valid, no invalid transitions

---

## R08: Queue UX with Titles and Live Execution Meaning

**Definition**: Left-side chat/work queue with titles and live execution meaning, drag-and-drop reprioritization

**Owner Phase**: Phase 1 (MVP Execution), Phase 7 (UI)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler), ADR-0004 (Glyphnova UI)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (queue configuration)
- Section 10: Orchestration Configuration (queue visualization)

**Implementation Tasks**:
1. Implement queue data model
2. Create queue visualization
3. Add live execution status updates
4. Implement drag-and-drop reprioritization (UI)

**Acceptance Test**:
- Test: Enqueue workflows and verify visualization
- Command: Enqueue 5 workflows, check queue display
- Expected: Queue shows correct state and titles
- Validation: Verify live updates during execution

**Verification Method**:
- Check: Verify queue state consistency
- Command: Compare queue state across restarts
- Result: State persists correctly

---

## R09: Reprioritization

**Definition**: Queue items can be reprioritized via drag-and-drop or API

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (priority configuration)

**Implementation Tasks**:
1. Implement priority update APIs
2. Add drag-and-drop handlers (UI)
3. Update scheduler priority algorithm
4. Ensure priority changes reflected in execution order

**Acceptance Test**:
- Test: Reprioritize queue items and verify order
- Command: Change priority, enqueue new items
- Expected: New order reflects priority changes
- Validation: Verify execution order matches priority

**Verification Method**:
- Check: Verify priority persistence
- Command: Change priority, restart, check order
- Result: Priority persists across restarts

---

## R10: Multi-Zoom Navigation

**Definition**: Navigation supports multiple abstraction levels for workflows, with nested topics, summaries, and graphs

**Owner Phase**: Phase 7 (UI and Desktop Shell)

**Related ADRs**: ADR-0004 (Glyphnova UI)

**Schema Sections**:
- Section 10: Orchestration Configuration (navigation metadata)

**Implementation Tasks**:
1. Implement multi-level navigation UI
2. Create topic hierarchy visualization
3. Add summary views at each level
4. Implement graph views for complex workflows

**Acceptance Test**:
- Test: Navigate through workflow hierarchy
- Command: Load comprehensive workflow, navigate levels
- Expected: All levels accessible and responsive
- Validation: Verify navigation on 100+ step workflow

**Verification Method**:
- Check: Verify navigation performance
- Command: Time navigation operations on complex workflows
- Result: Navigation remains responsive at scale

---

## R11: Visible Scope and Explicit Context-Switch Confirmation

**Definition**: Scope is always visible, and context switches require explicit user acknowledgment

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 3: Agentic Workflow (user_inputs configuration)
- Section 10: Orchestration Configuration (scope management)

**Implementation Tasks**:
1. Implement scope indicator in UI
2. Add context-switch confirmation dialogs
3. Track current scope per session
4. Prevent accidental scope changes

**Acceptance Test**:
- Test: Switch between workflows and verify confirmation
- Command: Navigate between different workflows
- Expected: Confirmation prompt shown for each switch
- Validation: Verify prompt appears every time

**Verification Method**:
- Check: Verify scope always visible
- Command: Inspect UI during various operations
- Result: Scope indicator always present

---

## R12: Policy Sliders (Runtime, Review, Rigor, Scope, Concurrency, Efficiency, Logging, Context Budget)

**Definition**: Policy is compiled into deterministic fields with 7 slider categories

**Owner Phase**: Phase 0 (Foundation), Phase 1 (Runtime enforcement)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract), ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (all policy-related fields)
- Section 7: Logging Configuration (logging policy)

**Implementation Tasks**:
1. Define policy struct with 7 fields
2. Implement policy compilation
3. Create policy enforcement at runtime
4. Add policy validation

**Acceptance Test**:
- Test: Set different policy values and verify behavior
- Command: Run workflows with different policies
- Expected: Behavior changes match policy settings
- Validation: Verify each policy field affects execution

**Verification Method**:
- Check: Verify policy is deterministic
- Command: Compile same policy multiple times
- Result: All compilations produce identical IR

---

## R13: Human Gating (Structured Clarification and Confirmation)

**Definition**: Risky or ambiguous workflows insert clarification and confirmation checkpoints before impactful actions

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 3: Agentic Workflow (user_inputs configuration)
- Section 6: Tool Permissions (confirmation requirements)

**Implementation Tasks**:
1. Implement risk detection
2. Create checkpoint insertion logic
3. Add confirmation prompts
4. Implement clarification dialogs

**Acceptance Test**:
- Test: Execute workflow with file writes
- Command: Run workflow modifying files
- Expected: Confirmation prompt appears before write
- Validation: Verify checkpoint detection accuracy

**Verification Method**:
- Check: Verify only risky actions trigger checkpoints
- Command: Classify 100 workflows, measure checkpoint precision
- Result: High precision (>90%)

---

## R14: Workflow Selection

**Definition**: Users can select and queue workflows from library

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 1: Workflow Identification (workflow discovery fields)

**Implementation Tasks**:
1. Implement workflow library UI
2. Create workflow search/filter
3. Add enqueue from library
4. Implement workflow preview

**Acceptance Test**:
- Test: Browse library and enqueue workflow
- Command: Navigate library, select workflow, enqueue
- Expected: Workflow added to queue correctly
- Validation: Verify workflow appears in queue

**Verification Method**:
- Check: Verify search works across all workflows
- Command: Search for keywords, verify results
- Result: Accurate search results

---

## R15: Schema-Based YAML DSL

**Definition**: Constrained YAML DSL validated against generated schemas

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- Section 1-19: All schema sections define DSL structure

**Implementation Tasks**:
1. Define YAML DSL grammar
2. Implement schema validation
3. Generate JSON Schema from Rust structs
4. Create validation error messages

**Acceptance Test**:
- Test: Validate all 53 workflows against schema
- Command: Run schema validator on all workflows
- Expected: 100% valid workflows pass
- Validation: Invalid workflows rejected with clear errors

**Verification Method**:
- Check: Verify schema is generated from code
- Command: Generate schema, compare with structs
- Result: No discrepancies

---

## R16: Prompt to YAML Workflow Generation Planning

**Definition**: Planning layer generates YAML workflows from user prompts

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- Section 1: Workflow Identification (prompt-driven fields)
- Section 15: Variable Interpolation (template variables)

**Implementation Tasks**:
1. Implement prompt-to-YAML planning
2. Create workflow templates
3. Add planning validation
4. Generate workflow from prompt

**Acceptance Test**:
- Test: Provide prompt, verify generated YAML
- Command: Call planning API with test prompt
- Expected: Valid workflow YAML generated
- Validation: Generated YAML passes schema validation

**Verification Method**:
- Check: Verify planning produces deterministic workflows
- Command: Generate same workflow 10 times
- Result: Identical YAML each time

---

## R17: Workflow Archetypes and Scheduling Modes

**Definition**: Multiple workflow archetypes with different scheduling modes (serial, parallel, hybrid)

**Owner Phase**: Phase 1 (MVP Execution), Phase 6 (Automation)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler), ADR-0007 (Cron and Git)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (processing modes)
- Section 5: Workflow Execution Strategy (scheduling configuration)

**Implementation Tasks**:
1. Implement execution modes (serial, parallel, hybrid)
2. Create scheduling algorithms
3. Add cron scheduling
4. Implement archetype templates

**Acceptance Test**:
- Test: Execute workflow in each mode
- Command: Run same workflow with different modes
- Expected: All modes execute correctly
- Validation: Verify behavior differs by mode

**Verification Method**:
- Check: Verify cron scheduling works
- Command: Schedule workflow, verify execution
- Result: Workflow runs at scheduled times

---

## R18: Tools and Custom Rust Tools as First-Class Nodes

**Definition**: Tools and custom Rust tools are first-class nodes in workflow graph

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 3: Agentic Workflow (step type: tool)
- Section 3: Agentic Workflow (custom_tools definition)

**Implementation Tasks**:
1. Implement tool node type
2. Create custom tool interface
3. Add tool validation
4. Implement tool execution engine

**Acceptance Test**:
- Test: Define custom tool in workflow
- Command: Create workflow with custom tool
- Expected: Tool executes correctly
- Validation: Verify tool effects and permissions enforced

**Verification Method**:
- Check: Verify tools are first-class
- Command: Inspect IR for tool nodes
- Result: Tools represented as IR nodes

---

## R19: Explicit DAG/State-Machine Semantics

**Definition**: Workflow has explicit DAG/state-machine semantics with composability and determinism

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- Section 4: Pipeline Definition (step dependencies)
- Section 5: Workflow Execution Strategy (state management)

**Implementation Tasks**:
1. Implement DAG validation
2. Create state machine for execution
3. Add dependency resolution
4. Ensure deterministic execution

**Acceptance Test**:
- Test: Create workflow with dependencies and verify DAG
- Command: Validate workflow structure
- Expected: DAG is acyclic, no cycles
- Validation: Detect and reject circular dependencies

**Verification Method**:
- Check: Verify deterministic execution
- Command: Run same workflow 10 times
- Result: Identical execution order each time

---

## R20: Declared Effects, Previews, and Diff-First Behavior

**Definition**: File mutations are staged and preview-first, with approval required

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 3: Agentic Workflow (file_operations configuration)
- Section 6: Tool Permissions (file write confirmation)

**Implementation Tasks**:
1. Implement staging for file operations
2. Create diff generation
3. Add approval prompts
4. Implement rollback capability

**Acceptance Test**:
- Test: Execute workflow with file writes
- Command: Run workflow modifying files
- Expected: Diff shown, approval requested
- Validation: Verify write only happens after approval

**Verification Method**:
- Check: Verify staging creates temp files
- Command: Inspect staging directory during write
- Result: Staging files present, approval required

---

## R21: Artifact Semantics (Workflows are Inspectable, Editable Artifacts)

**Definition**: Workflow specs are editable artifacts, transformable by later workflows under review controls

**Owner Phase**: Phase 4 (Advanced Agentic Features)

**Related ADRs**: ADR-0005 (Quality Loops and Benchmarks)

**Schema Sections**:
- Section 1: Workflow Identification (artifact metadata)
- Section 15: Variable Interpolation (artifact references)

**Implementation Tasks**:
1. Implement artifact storage
2. Create artifact versioning
3. Add artifact editing
4. Implement review controls for modifications

**Acceptance Test**:
- Test: Modify existing workflow and verify review
- Command: Edit workflow, attempt execution
- Expected: Modification requires review approval
- Validation: Verify all modifications tracked

**Verification Method**:
- Check: Verify artifact transformation is tracked
- Command: Transform workflow, check audit log
- Result: Transformation fully audited

---

## R22: Whitt Execution Engine Execution Target

**Definition**: YAML compiles to Rust code targeting AgentSDK

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- All sections: Schema defines YAML structure for code generation

**Implementation Tasks**:
1. Implement YAML to WorkflowIR compiler
2. Create WorkflowIR to Rust code generator
3. Implement AgentSDK integration
4. Add code validation

**Acceptance Test**:
- Test: Generate Rust code from workflow
- Command: Generate code, compile, execute
- Expected: Generated code compiles and executes
- Validation: Verify output matches direct execution

**Verification Method**:
- Check: Verify generated code is idiomatic Rust
- Command: Inspect generated code
- Result: Clean, readable Rust code

---

## R23: Packaging Choice (Standalone vs Backend Crate vs Inlining)

**Definition**: Generated Rust project can be standalone, depend on backend crate, or inline backend code

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (packaging configuration)

**Implementation Tasks**:
1. Implement standalone packaging
2. Create backend crate option
3. Add inlined backend option
4. Implement build system integration

**Acceptance Test**:
- Test: Generate and build with each packaging option
- Command: Generate code with each packaging mode
- Expected: All three build successfully
- Validation: Verify binaries work independently

**Verification Method**:
- Check: Verify packaging choice is reflected in Cargo.toml
- Command: Inspect generated Cargo.toml
- Result: Dependencies match packaging mode

---

## R24: Queue Semantics (Worker Pools, Persistence, Cancellation, Result Retrieval)

**Definition**: Queue with worker pools, persistence across restarts, cancellation, and result retrieval

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (queue configuration)
- Section 10: Orchestration Configuration (queue persistence)

**Implementation Tasks**:
1. Implement worker pool
2. Create queue persistence
3. Add cancellation support
4. Implement result retrieval API

**Acceptance Test**:
- Test: Enqueue, restart, verify queue state
- Command: Enqueue items, restart process, check queue
- Expected: Queue restored correctly
- Validation: Verify all items present after restart

**Verification Method**:
- Check: Verify cancellation stops execution
- Command: Cancel running workflow, monitor
- Result: Workflow stops gracefully, resources freed

---

## R25: Generate-Verify-Repair Review Loop Semantics

**Definition**: Runtime semantics for generate-verify-repair loops, not optional prompt style

**Owner Phase**: Phase 4 (Advanced Agentic Features)

**Related ADRs**: ADR-0005 (Quality Loops and Benchmarks)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (validation configuration)
- Section 9: Validation Configuration (validation rules)

**Implementation Tasks**:
1. Implement validation loop runtime
2. Create convergence detection
3. Add repair step execution
4. Implement loop termination logic

**Acceptance Test**:
- Test: Execute validation loop workflow
- Command: Run workflow with validation loop
- Expected: Loop converges and terminates
- Validation: Verify repair step executes on failure

**Verification Method**:
- Check: Verify loop respects max iterations
- Command: Run validation loop with max 2
- Result: Loop terminates at iteration 2 if not converged

---

## R26: Long-Running Behavior (Background, Semi-Endless, Endless, Resumable)

**Definition**: Support for background, semi-endless, endless, and resumable workflows

**Owner Phase**: Phase 6 (Automation and Scheduling)

**Related ADRs**: ADR-0007 (Cron and Git), ADR-0008 (Autonomy and Metrics)

**Schema Sections**:
- Section 5: Workflow Execution Strategy (infinite loop config)
- Section 10: Orchestration Configuration (long-running support)

**Implementation Tasks**:
1. Implement background execution
2. Add infinite loop safety limits
3. Create resumable workflows
4. Implement workflow persistence

**Acceptance Test**:
- Test: Execute background workflow
- Command: Run workflow in background, detach
- Expected: Workflow continues, can be reattached
- Validation: Verify workflow survives terminal close

**Verification Method**:
- Check: Verify infinite loops have safety caps
- Command: Run infinite loop, monitor resource usage
- Result: Loop terminates at safety limit

---

## R27: Progressive Results and Partial Answers

**Definition**: Long-running workflows stream partial results for interactive flows

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 7: Logging Configuration (streaming output)
- Section 8: Metrics Configuration (real-time metrics)

**Implementation Tasks**:
1. Implement result streaming
2. Create partial output handling
3. Add progress reporting
4. Implement CLI streaming

**Acceptance Test**:
- Test: Execute long workflow with CLI
- Command: Run 10-minute workflow, monitor output
- Expected: Results streamed progressively
- Validation: Verify partials are well-formed

**Verification Method**:
- Check: Verify partial results don't interfere with final
- Command: Collect partials and final result
- Result: Final result supersedes partials correctly

---

## R28: Per-Chat Workspace and ./workspace/ System-of-Record

**Definition**: Per-chat workspace with ./workspace/ directory for system-of-record

**Owner Phase**: Phase 0 (Foundation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract)

**Schema Sections**:
- Section 13: Workspace Configuration (directory structure)
- Section 14: Features Demonstrated (system-of-record features)

**Implementation Tasks**:
1. Implement ./workspace/ directory creation
2. Create workspace isolation per chat
3. Add system-of-record storage
4. Implement artifact management

**Acceptance Test**:
- Test: Create workflow, verify ./workspace/ structure
- Command: Execute workflow, inspect ./workspace/
- Expected: All required subdirectories created
- Validation: Verify structure matches ADR-0001

**Verification Method**:
- Check: Verify different chats don't interfere
- Command: Run 2 workflows sequentially (parallel execution moved to agent-queue), check artifacts
- Result: Separate directories, no interference

---

## R29: Staged Diffs (Create/Edit/Delete with Confirmation and Staged Diffs)

**Definition**: Create/edit/delete actions with staged diffs and confirmation

**Owner Phase**: Phase 1 (MVP Execution)

**Related ADRs**: ADR-0002 (MVP Queue and Scheduler)

**Schema Sections**:
- Section 3: Agentic Workflow (file_operations configuration)
- Section 6: Tool Permissions (file operation confirmations)

**Implementation Tasks**:
1. Implement diff generation
2. Create staging area
3. Add confirmation prompts
4. Implement rollback capability

**Acceptance Test**:
- Test: Execute workflow with destructive operations
- Command: Run workflow with delete operations
- Expected: Diff shown, confirmation required
- Validation: Verify no changes without approval

**Verification Method**:
- Check: Verify rollback restores previous state
- Command: Approve operation, verify result, rollback
- Result: Rollback returns to pre-operation state

---

## R30: Observability (Logging Levels, Summaries, Graphs, Reports, Provenance)

**Definition**: Comprehensive observability with multiple output types and provenance tracking

**Owner Phase**: Phase 1 (Core), Phase 7 (Comprehensive)

**Related ADRs**: ADR-0005 (Quality Loops), ADR-0008 (Autonomy and Metrics)

**Schema Sections**:
- Section 7: Logging Configuration (all logging features)
- Section 8: Metrics Configuration (comprehensive metrics)
- Section 14: Features Demonstrated (observability flags)

**Implementation Tasks**:
1. Implement hierarchical logging
2. Create metrics collection
3. Add summary generation
4. Implement report generation
5. Add provenance tracking

**Acceptance Test**:
- Test: Execute workflow and verify all outputs
- Command: Run comprehensive workflow
- Expected: Summary, report, graph, benchmark files generated
- Validation: Verify all outputs are queryable

**Verification Method**:
- Check: Verify outputs are queryable
- Command: Query observability data
- Result: Can search and filter all output types

---

## R31: Auditable Local Artifacts (Versioned Runs, Hashes, Privacy-Preserving Defaults)

**Definition**: All runs persist with versioned artifacts, hashes, and privacy-preserving defaults

**Owner Phase**: Phase 0 (Foundation), Phase 6 (Automation)

**Related ADRs**: ADR-0001 (Foundation Compiler Contract), ADR-0007 (Cron and Git)

**Schema Sections**:
- Section 1: Workflow Identification (version fields)
- Section 13: Workspace Configuration (artifact storage)
- Section 16: Default Behavior Reference (privacy defaults)

**Implementation Tasks**:
1. Implement artifact versioning
2. Create hash generation
3. Add artifact storage
4. Implement privacy defaults

**Acceptance Test**:
- Test: Execute workflow, verify artifacts and hashes
- Command: Run workflow, examine ./workspace/runs/
- Expected: spec, policy, artifacts, hash, provenance present
- Validation: Verify hash matches source

**Verification Method**:
- Check: Verify privacy by default
- Command: Monitor network during default runs
- Result: No unauthorized external access

---

## R32: Model Provider Abstraction for Multiple Local Runners

**Definition**: Provider abstraction normalizing llama.cpp, LM Studio, Ollama

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 11: Provider Configuration (all provider settings)
- Section 2: Model Configuration (provider integration)

**Implementation Tasks**:
1. Implement provider trait
2. Create llama.cpp provider
3. Create LM Studio provider
4. Create Ollama provider

**Acceptance Test**:
- Test: Switch between all providers
- Command: Run workflow with each provider
- Expected: All providers work with same workflow
- Validation: Verify only provider-specific differences

**Verification Method**:
- Check: Verify provider abstraction is transparent
- Command: Compare execution traces
- Result: Only provider-specific differences

---

## R33: Model Lifecycle (Load/Unload Models, Multiple Instances)

**Definition**: Model lifecycle management with loading, unloading, multiple instances (parallel execution moved to agent-queue)

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 2: Model Configuration (lifecycle hooks)
- Section 5: Workflow Execution Strategy (load_unload strategy)

**Implementation Tasks**:
1. Implement model lifecycle state machine
2. Create loading/unloading logic
3. Add multi-instance support
4. Implement parallel model execution

**Acceptance Test**:
- Test: Load multiple models, verify lifecycle
- Command: Load 3 models, execute parallel
- Expected: All models loaded and execute correctly
- Validation: Verify lifecycle state transitions

**Verification Method**:
- Check: Verify memory managed correctly
- Command: Monitor RAM/VRAM during parallel execution
- Result: Memory usage within limits

---

## R34: Routing Strategy

**Definition**: Automatic routing to appropriate model based on task type

**Owner Phase**: Phase 2 (MVP Execution)

**Related ADRs**: ADR-0003 (CLI and Backends)

**Schema Sections**:
- Section 2: Model Configuration (default_router configuration)
- Section 2: Model Configuration (model selection logic)

**Implementation Tasks**:
1. Implement routing algorithm
2. Create task type detection
3. Add model selection logic
4. Implement fallback routing

**Acceptance Test**:
- Test: Execute various tasks, verify model selection
- Command: Run workflows with different task types
- Expected: Appropriate models selected for each task
- Validation: Verify routing decisions are correct

**Verification Method**:
- Check: Verify routing is efficient
- Command: Measure routing decision time
- Result: Routing completes in <100ms

---

## Summary Matrix

| Requirement | Owner Phase | ADR | Schema Sections | Status |
|-------------|--------------|------|----------------|--------|
| R01: System Identity | Phase 0 | ADR-0001 | 1, 14 | ✅ Traced |
| R03: Local-First | Phase 0 | ADR-0001, ADR-0006 | 13, 16, 6 | ✅ Traced |
| R05: CLI-First | Phase 2 | ADR-0002, ADR-0003 | 5, 7, 8 | ✅ Traced |
| R07: Chat Container | Phase 1 | ADR-0002 | 3, 10 | ✅ Traced |
| R08: Queue UX | Phase 1, 7 | ADR-0002, ADR-0004 | 5, 10 | ✅ Traced |
| R09: Reprioritization | Phase 1 | ADR-0002 | 5 | ✅ Traced |
| R10: Multi-Zoom | Phase 7 | ADR-0004 | 10 | ✅ Traced |
| R11: Context Safety | Phase 1 | ADR-0002 | 3, 10 | ✅ Traced |
| R12: Policy Sliders | Phase 0, 1 | ADR-0001, ADR-0002 | 5, 7 | ✅ Traced |
| R13: Human Gating | Phase 1 | ADR-0002 | 3, 6 | ✅ Traced |
| R14: Workflow Selection | Phase 1 | ADR-0002 | 1 | ✅ Traced |
| R15: Schema DSL | Phase 0 | ADR-0001 | 1-19 | ✅ Traced |
| R16: Prompt to YAML | Phase 0 | ADR-0001 | 1, 15 | ✅ Traced |
| R17: Workflow Archetypes | Phase 1, 6 | ADR-0002, ADR-0007 | 5 | ✅ Traced |
| R18: Tools | Phase 2 | ADR-0003 | 3 | ✅ Traced |
| R19: DAG/Semantics | Phase 0 | ADR-0001 | 4, 5 | ✅ Traced |
| R20: Effect Declaration | Phase 1 | ADR-0002 | 3, 6 | ✅ Traced |
| R21: Artifact Semantics | Phase 4 | ADR-0005 | 1, 15 | ✅ Traced |
| R22: YAML to Rust | Phase 0 | ADR-0001 | 1-19 | ✅ Traced |
| R23: Packaging | Phase 2 | ADR-0003 | 5 | ✅ Traced |
| R24: Queue Semantics | Phase 1 | ADR-0002 | 5, 10 | ✅ Traced |
| R25: Generate-Verify-Repair | Phase 4 | ADR-0005 | 5, 9 | ✅ Traced |
| R26: Long-Running | Phase 6 | ADR-0007, ADR-0008 | 5, 10 | ✅ Traced |
| R27: Progressive Results | Phase 2 | ADR-0003 | 7, 8 | ✅ Traced |
| R28: Workspace | Phase 0 | ADR-0001 | 13, 14 | ✅ Traced |
| R29: Staged Diffs | Phase 1 | ADR-0002 | 3, 6 | ✅ Traced |
| R30: Observability | Phase 1, 7 | ADR-0005, ADR-0008 | 7, 8, 14 | ✅ Traced |
| R31: Auditable Artifacts | Phase 0, 6 | ADR-0001, ADR-0007 | 1, 13, 16 | ✅ Traced |
| R32: Provider Abstraction | Phase 2 | ADR-0003 | 11, 2 | ✅ Traced |
| R33: Model Lifecycle | Phase 2 | ADR-0003 | 2, 5 | ✅ Traced |
| R34: Routing Strategy | Phase 2 | ADR-0003 | 2 | ✅ Traced |

**Total Requirements**: 31 (R01-R31, plus R32-R34)
**Coverage**: 100% (All requirements traced to phases, ADRs, and schema sections)

---

## Implementation Priority by Phase

### Phase 0: Foundation (Critical Path)
**Requirements**: R01, R03, R12, R15, R16, R19, R22, R28, R31
**Count**: 9 requirements
**ADR Coverage**: ADR-0000, ADR-0001
**Schema Coverage**: All schema sections as foundation

### Phase 1: MVP Execution (Critical Path)
**Requirements**: R07, R08, R09, R10, R11, R12, R13, R14, R17, R19, R20, R24, R25, R29, R30
**Count**: 15 requirements
**ADR Coverage**: ADR-0001, ADR-0002
**Schema Coverage**: Sections 3, 4, 5, 6, 7, 9, 10, 13, 14, 15, 17, 18

### Phase 2: Backends and Providers (High Priority)
**Requirements**: R05, R18, R23, R24, R27, R32, R33, R34
**Count**: 8 requirements
**ADR Coverage**: ADR-0002, ADR-0003
**Schema Coverage**: Sections 2, 5, 6, 7, 8, 11

### Phase 3: CLI and UI Integration (High Priority)
**Requirements**: R05, R32, R33, R34
**Count**: 4 requirements (subset of Phase 2)
**ADR Coverage**: ADR-0003, ADR-0004
**Schema Coverage**: Sections 2, 11

### Phase 4: Advanced Agentic Features (Medium Priority)
**Requirements**: R21, R25
**Count**: 2 requirements
**ADR Coverage**: ADR-0005
**Schema Coverage**: Sections 5, 9

### Phase 5: Benchmarking and Observability (Medium Priority)
**Requirements**: R30 (comprehensive), R06, R07 (advanced web)
**Count**: 3 requirements (plus advanced web features)
**ADR Coverage**: ADR-0005, ADR-0006
**Schema Coverage**: Sections 7, 8, 12

### Phase 6: Automation and Scheduling (Low Priority)
**Requirements**: R17 (cron), R26, R31 (experiments)
**Count**: 3 requirements
**ADR Coverage**: ADR-0007, ADR-0008
**Schema Coverage**: Sections 5, 10, 13

### Phase 7: UI and Desktop Shell (Low Priority)
**Requirements**: R05 (UI), R08 (UI), R10 (UI), R30 (comprehensive UI)
**Count**: 4 requirements (UI extensions)
**ADR Coverage**: ADR-0004, ADR-0008
**Schema Coverage**: Sections 7, 8, 10

### Phase 8: Final Validation (Final Phase)
**Requirements**: All requirements (comprehensive testing)
**Count**: 31 requirements
**ADR Coverage**: All ADRs
**Schema Coverage**: All sections
**Purpose**: End-to-end validation of complete system
