# ADR to Phase Matrix

**Purpose**: Map all architectural constraints from ADRs to owning implementation phases

**Total ADRs**: 9 (ADR-0000 through ADR-0008)
**Total Constraints**: 47
**Date**: 2026-04-06

---

## ADR-0000: Roadmap Index

**Owner Phase**: Phase 0 (Foundation)

**Constraint Count**: 1

### Constraint 1: Master index linking all ADRs and research plans

**Quote**: "Adopt a roadmap structure with master index linking all ADRs and research plans"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Verify all ADRs are linked from index
- Expected: ADR-0000 references ADR-0001 through ADR-0008
- Command: `grep -r "follow_on_adrs:" opencode/docs/reports/roadmap/adr-*.yml | wc -l`
- Result: All ADRs have `follow_on_adrs` field

**Verification Method**:
- Check: Validate ADR dependency graph is acyclic
- Expected: No circular references between ADRs
- Command: Analyze `follow_on_adrs` fields for cycles
- Result: Clean DAG structure

---

## ADR-0001: Foundation Compiler Contract

**Owner Phase**: Phase 0 (Foundation)

**Constraint Count**: 8

### Constraint 1: Canonical execution unit: typed WorkflowSpec and compiled WorkflowIR

**Quote**: "Canonical execution unit: typed WorkflowSpec and compiled WorkflowIR"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Create WorkflowSpec from valid YAML
- Expected: Successfully compiled to WorkflowIR
- Command: Parser compiles YAML to Rust structs
- Result: No compilation errors

**Verification Method**:
- Check: Validate WorkflowIR is deterministic
- Expected: Same input always produces same IR
- Command: Compile same YAML 10 times, hash results
- Result: All hashes identical

### Constraint 2: Authoring surface: constrained YAML DSL, validated against generated schemas

**Quote**: "Authoring surface: constrained YAML DSL, validated against generated schemas"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Validate YAML against JSON Schema
- Expected: All valid workflows pass schema validation
- Command: Schema validator on all 53 workflows
- Result: 100% pass rate

**Verification Method**:
- Check: Verify schema is generated from Rust structs
- Expected: JSON Schema matches Rust types
- Command: Generate schema from structs, compare
- Result: No discrepancies

### Constraint 3: Every chat: scoped executable work container with local ./workspace/ system-of-record

**Quote**: "Every chat: scoped executable work container with local ./workspace/ system-of-record"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Create workflow and verify ./workspace/ directory structure
- Expected: Required subdirectories created
- Command: Execute workflow, check directory
- Result: All required paths present

**Verification Method**:
- Check: Validate system-of-record isolation per chat
- Expected: Different chats don't interfere
- Command: Run 2 workflows concurrently
- Result: Separate artifacts in separate directories

### Constraint 4: Policy: compiled into deterministic fields (runtime, review, rigor, scope, concurrency, logging, context budget)

**Quote**: "Policy: compiled into deterministic fields"

**Owning Phase**: Phase 0 (Foundation), Phase 1 (Runtime)

**Compliance Test**:
- Test: Verify policy fields are compile-time resolvable
- Expected: No dynamic policy evaluation at runtime
- Command: Inspect WorkflowIR policy section
- Result: All values are constants

**Verification Method**:
- Check: Validate all 7 policy fields present
- Expected: runtime, review, rigor, scope, concurrency, logging, context_budget
- Command: Check WorkflowSpec structure
- Result: All fields required

### Constraint 5: All runs persist: workflow specs, policy snapshots, artifact references, hashes, provenance metadata

**Quote**: "All runs persist: workflow specs, policy snapshots, artifact references, hashes, provenance metadata"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Execute workflow and check persistence
- Expected: All required artifacts saved
- Command: Run workflow, examine ./workspace/runs/
- Result: spec, policy, artifacts, hash, provenance present

**Verification Method**:
- Check: Validate artifact hash matches source
- Expected: SHA-256 of workflow spec matches stored hash
- Command: Hash workflow YAML, compare with stored
- Result: Hashes match

### Constraint 6: Reproducibility and privacy defaults

**Quote**: "reproducibility and privacy defaults"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Re-execute workflow and verify reproducibility
- Expected: Same inputs produce same outputs
- Command: Run workflow twice with same inputs
- Result: Outputs identical

**Verification Method**:
- Check: Validate no external network access by default
- Expected: No outbound network calls without explicit opt-in
- Command: Monitor network during workflow execution
- Result: No unauthorized network traffic

### Constraint 7: Stable foundation for CLI and UI shells

**Quote**: "stable foundation for CLI and UI shells"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Verify CLI can use foundation APIs
- Expected: CLI commands execute workflows correctly
- Command: Test all CLI operations
- Result: All operations functional

**Verification Method**:
- Check: Validate UI can use same foundation APIs
- Expected: UI uses same workflow execution engine
- Command: Inspect UI code for foundation integration
- Result: No duplicate implementation

### Constraint 8: ADR-aware directory layout before code generation

**Quote**: "Define a stable ADR-aware directory layout before code generation begins"

**Owning Phase**: Phase 0 (Foundation)

**Compliance Test**:
- Test: Verify ./workspace/ directory structure is stable
- Expected: All required subdirectories exist
- Command: Check directory structure against spec
- Result: Structure matches ADR-0001

**Verification Method**:
- Check: Validate validation criteria for v0.1.0 as artifacts stored beside foundation spec
- Expected: Validation criteria document exists
- Command: Find validation criteria file
- Result: File present and referenced

---

## ADR-0002: MVP Queue and Scheduler

**Owner Phase**: Phase 1 (MVP Execution)

**Constraint Count**: 8

### Constraint 1: ChatSession: primary scoped work container

**Quote**: "ChatSession: primary scoped work container"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Create ChatSession and verify isolation
- Expected: Each session has independent state
- Command: Create 3 sessions, verify independence
- Result: No state leakage between sessions

**Verification Method**:
- Check: Validate session lifecycle (create → execute → close)
- Expected: Lifecycle transitions correctly
- Command: Execute session lifecycle
- Result: All transitions valid

### Constraint 2: Queue items: executable runs with explicit lifecycle states

**Quote**: "Queue items: executable runs with explicit lifecycle states"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Enqueue workflow and track state changes
- Expected: States transition: pending → running → completed/failed
- Command: Monitor queue state during execution
- Result: All transitions correct

**Verification Method**:
- Check: Validate all lifecycle states are reachable
- Expected: pending, running, paused, completed, failed, cancelled
- Command: Transition through all states
- Result: All states reachable

### Constraint 3: Scheduler: supports prioritization, cancellation, retries, persistence, and result retrieval

**Quote**: "Scheduler: supports prioritization, cancellation, retries, persistence, and result retrieval"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Prioritize items in queue
- Expected: Higher priority items execute first
- Command: Enqueue items with different priorities
- Result: Execution order respects priority

**Verification Method**:
- Test: Verify cancellation works
- Expected: Cancelled item stops execution and removed
- Command: Cancel running workflow
- Result: Workflow stops gracefully

**Compliance Test**:
- Test: Verify persistence across restarts
- Expected: Queue state survives process restart
- Command: Enqueue items, restart, check queue
- Result: Queue restored

### Constraint 4: Policy sliders: compile into deterministic scheduler and gating settings

**Quote**: "Policy sliders: compile into deterministic scheduler and gating settings"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Verify policy fields affect scheduler behavior
- Expected: Policy changes reflected in scheduler
- Command: Modify policy, observe behavior
- Result: Behavior matches policy

**Verification Method**:
- Check: Validate policy fields are immutable during execution
- Expected: Policy cannot be changed mid-execution
- Command: Attempt to modify policy during run
- Result: Modification rejected

### Constraint 5: Risky or ambiguous workflows: insert clarification and confirmation checkpoints before impactful actions

**Quote**: "Risky or ambiguous workflows: insert clarification and confirmation checkpoints before impactful actions"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Execute workflow with file writes
- Expected: Confirmation prompt before write
- Command: Run workflow with file writes
- Result: User prompted for approval

**Verification Method**:
- Check: Validate checkpoint detection is accurate
- Expected: Only risky actions trigger checkpoints
- Command: Classify 100 workflows, verify checkpoint accuracy
- Result: High precision (>90%)

### Constraint 6: File mutation: staged and preview-first, with approval required for destructive or broad-scope changes

**Quote**: "File mutation: staged and preview-first, with approval required for destructive or broad-scope changes"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Execute workflow with file writes
- Expected: Diff shown before write, approval required
- Command: Run workflow modifying file
- Result: Diff displayed, approval requested

**Verification Method**:
- Test: Verify staging creates temporary files
- Expected: Changes written to staging area first
- Command: Inspect staging directory during write
- Result: Staging files present

### Constraint 7: CLI: first full control surface for queue inspection, enqueueing, priority updates, approvals, and result retrieval

**Quote**: "CLI: first full control surface for queue inspection, enqueueing, priority updates, approvals, and result retrieval"

**Owning Phase**: Phase 1 (MVP Execution), Phase 2 (CLI implementation)

**Compliance Test**:
- Test: Inspect queue from CLI
- Expected: Queue listing shows all items
- Command: `whitt-execution-engine queue list`
- Result: All items displayed

**Verification Method**:
- Test: Enqueue workflow from CLI
- Expected: Workflow added to queue
- Command: `whitt-execution-engine queue add workflow.yaml`
- Result: Workflow in queue

### Constraint 8: Dual execution modes: Execution Engine (direct) and Code Generator (Rust compilation)

**Quote**: "Dual execution modes: Execution Engine and Code Generator"

**Owning Phase**: Phase 1 (MVP Execution)

**Compliance Test**:
- Test: Execute workflow in direct mode
- Expected: YAML → WorkflowIR → Execution without compilation
- Command: `whitt-execution-engine run workflow.yaml`
- Result: Workflow executes directly

**Verification Method**:
- Test: Execute workflow in code generation mode
- Expected: YAML → WorkflowIR → Rust Code → Compile → Execution
- Command: `whitt-execution-engine generate workflow.yaml`
- Result: Rust code generated and compiled

---

## ADR-0003: CLI and Backends

**Owner Phase**: Phase 2 (MVP Execution + Backends)

**Constraint Count**: 7

### Constraint 1: Runtime core: shell-agnostic, exposed through CLI/TUI first

**Quote**: "Runtime core: shell-agnostic, exposed through CLI/TUI first"

**Owning Phase**: Phase 2 (CLI implementation)

**Compliance Test**:
- Test: Verify runtime works without TTY
- Expected: All operations work in non-interactive mode
- Command: Run CI environment tests
- Result: All tests pass

**Verification Method**:
- Test: Verify CLI provides full functionality
- Expected: All operations accessible via CLI
- Command: Test all CLI commands
- Result: All commands functional

### Constraint 2: Model access: provider abstraction normalizing local runners

**Quote**: "Model access: provider abstraction normalizing local runners"

**Owning Phase**: Phase 2 (Backend implementation)

**Compliance Test**:
- Test: Switch between llama.cpp, LM Studio, Ollama
- Expected: All providers work with same workflow
- Command: Run workflow with each provider
- Result: All providers execute correctly

**Verification Method**:
- Test: Verify provider abstraction is transparent
- Expected: Provider selection doesn't affect workflow logic
- Command: Compare execution traces across providers
- Result: Only provider-specific differences

### Constraint 3: Packaging: generated Rust project depending on shared backend crate

**Quote**: "Packaging: generated Rust project depending on shared backend crate"

**Owning Phase**: Phase 2 (Packaging strategy)

**Compliance Test**:
- Test: Generate Rust code and verify dependency
- Expected: Generated Cargo.toml includes backend crate
- Command: Generate code, inspect Cargo.toml
- Result: Backend dependency present

**Verification Method**:
- Test: Verify compiled binary works independently
- Expected: Binary runs without YAML files
- Command: Build and run generated binary
- Result: Binary executes workflow

### Constraint 4: Networking: explicit opt-in capability boundary with provenance tracking

**Quote**: "Networking: explicit opt-in capability boundary with provenance tracking"

**Owning Phase**: Phase 2 (Networking boundary)

**Compliance Test**:
- Test: Execute workflow with web access
- Expected: Provenance tracked for all network calls
- Command: Run workflow with web operations
- Result: Provenance logs show all requests

**Verification Method**:
- Test: Verify opt-in is required
- Expected: Networking blocked without explicit permission
- Command: Attempt web access without permission
- Result: Access denied

### Constraint 5: Progressive delivery: partial results for interactive flows

**Quote**: "Progressive delivery: partial results for interactive flows"

**Owning Phase**: Phase 2 (Progressive results)

**Compliance Test**:
- Test: Execute long-running workflow with CLI
- Expected: Partial results streamed to console
- Command: Run long workflow, monitor output
- Result: Results displayed progressively

**Verification Method**:
- Test: Verify partial results are valid
- Expected: Incremental results are well-formed
- Command: Parse intermediate outputs
- Result: All partials valid

### Constraint 6: Tool nodes: custom Rust tools with declared effects and permissions

**Quote**: "Tool nodes: custom Rust tools with declared effects and permissions"

**Owning Phase**: Phase 2 (Custom tools)

**Compliance Test**:
- Test: Define custom tool in workflow
- Expected: Tool effects and permissions validated
- Command: Create workflow with custom tool
- Result: Tool accepted with declared permissions

**Verification Method**:
- Test: Verify custom tool executes correctly
- Expected: Tool runs and returns result
- Command: Execute workflow with custom tool
- Result: Tool produces correct output

### Constraint 7: Dual execution modes (Execution Engine, Code Generation, Self-Improvement)

**Quote**: "Dual execution modes plus self-improvement"

**Owning Phase**: Phase 2 (Self-improvement)

**Compliance Test**:
- Test: Execute workflow improvement loop
- Expected: Logs → analysis → workflow improvement → regeneration
- Command: Run workflow improvement cycle
- Result: Workflow improved and regenerated

**Verification Method**:
- Test: Verify improvement uses execution metrics
- Expected: Metrics drive improvements
- Command: Inspect improvement decisions
- Result: Metrics-based decisions

---

## ADR-0004: Glyphnova UI and Multi-Zoom Control Plane

**Owner Phase**: Phase 7 (UI and Desktop Shell)

**Constraint Count**: 6

### Constraint 1: Desktop shell uses Rust backend and web frontend

**Quote**: "Desktop shell uses a Rust backend and a web frontend"

**Owning Phase**: Phase 7 (UI implementation)

**Compliance Test**:
- Test: Launch UI and verify both components
- Expected: Rust backend and web frontend running
- Command: `whitt-execution-engine ui`
- Result: Both components started

**Verification Method**:
- Test: Verify backend exposes API to frontend
- Expected: Frontend can call backend APIs
- Command: Inspect API documentation
- Result: All documented endpoints accessible

### Constraint 2: Queue visualization reflects persistent scheduler states

**Quote**: "Queue visualization reflects persistent scheduler states rather than a separate UI-only model"

**Owning Phase**: Phase 7 (UI visualization)

**Compliance Test**:
- Test: Update queue from CLI, verify UI updates
- Expected: UI shows same state as scheduler
- Command: Modify queue, check UI
- Result: UI matches scheduler state

**Verification Method**:
- Test: Verify no UI-only state divergence
- Expected: No state mismatches after operations
- Command: Perform 10 operations, compare states
- Result: 100% consistency

### Constraint 3: Scope visibility is always present and context switches require explicit user acknowledgment

**Quote**: "Scope visibility is always present and context switches require explicit user acknowledgment"

**Owning Phase**: Phase 7 (UI context safety)

**Compliance Test**:
- Test: Switch between workflows in UI
- Expected: Context switch prompt appears
- Command: Navigate between workflows
- Result: Confirmation prompt shown

**Verification Method**:
- Test: Verify scope indicators are always visible
- Expected: Current scope shown in UI
- Command: Inspect UI layout
- Result: Scope indicator prominent

### Constraint 4: Navigation supports multiple abstraction levels

**Quote**: "Navigation supports multiple abstraction levels"

**Owning Phase**: Phase 7 (UI navigation)

**Compliance Test**:
- Test: Navigate through workflow hierarchy
- Expected: Can zoom in/out on workflow details
- Command: Use navigation controls
- Result: All levels accessible

**Verification Method**:
- Test: Verify navigation performance on complex workflows
- Expected: Smooth navigation with 100+ steps
- Command: Load comprehensive workflow
- Result: Navigation remains responsive

### Constraint 5: Drag-and-drop reprioritization is a UI projection of scheduler priority APIs

**Quote**: "Drag-and-drop reprioritization is a UI projection of the same scheduler priority APIs used by CLI"

**Owning Phase**: Phase 7 (UI scheduler integration)

**Compliance Test**:
- Test: Reprioritize queue item via drag-and-drop
- Expected: Priority updated in scheduler
- Command: Drag item to new position
- Result: Scheduler reflects new priority

**Verification Method**:
- Test: Verify drag-and-drop uses same APIs as CLI
- Expected: API calls identical
- Command: Compare API calls CLI vs UI
- Result: Identical sequences

### Constraint 6: Shared backend API boundary

**Quote**: "Shared backend API boundary"

**Owning Phase**: Phase 7 (API design)

**Compliance Test**:
- Test: Verify both CLI and UI use same backend
- Expected: No duplicate implementation
- Command: Inspect code structure
- Result: Single backend implementation

**Verification Method**:
- Test: Verify API is versioned and stable
- Expected: API version documented and stable
- Command: Check API documentation
- Result: Version info present

---

## ADR-0005: Quality Loops and Benchmarks

**Owner Phase**: Phase 4 (Advanced Agentic Features)

**Constraint Count**: 5

### Constraint 1: Generate-verify-repair loops: runtime semantics, not optional prompt style

**Quote**: "Generate-verify-repair loops: runtime semantics, not optional prompt style"

**Owning Phase**: Phase 4 (Quality loops)

**Compliance Test**:
- Test: Execute validation loop workflow
- Expected: Loop terminates when criteria met
- Command: Run validation loop workflow
- Result: Loop converges and terminates

**Verification Method**:
- Test: Verify repair step executes after failed verification
- Expected: Repair runs before retry
- Command: Monitor validation loop execution
- Result: Repair step executes on failure

### Constraint 2: Benchmark suites: stored artifacts tied to workflow version, policy snapshot, backend, and file type

**Quote**: "Benchmark suites: stored artifacts tied to workflow version, policy snapshot, backend, and file type"

**Owning Phase**: Phase 4 (Benchmarking)

**Compliance Test**:
- Test: Run benchmark and verify artifact storage
- Expected: All metadata stored with benchmark result
- Command: Execute benchmark workflow
- Result: Complete artifact metadata

**Verification Method**:
- Test: Verify benchmarks can be compared across runs
- Expected: Results comparable with historical data
- Command: Query benchmark history
- Result: Comparative data accessible

### Constraint 3: Workflow specs: editable artifacts, transformable by later workflows under review controls

**Quote**: "Workflow specs: editable artifacts, transformable by later workflows under review controls"

**Owning Phase**: Phase 4 (Workflow artifacts)

**Compliance Test**:
- Test: Modify existing workflow and verify review
- Expected: Modification requires review approval
- Command: Edit workflow, attempt execution
- Result: Review triggered

**Verification Method**:
- Test: Verify workflow transformation is tracked
- Expected: All transformations logged
- Command: Transform workflow via another workflow
- Result: Transformation audited

### Constraint 4: Observability: produces summaries, reports, graphs, and benchmark outputs

**Quote**: "Observability: produces summaries, reports, graphs, and benchmark outputs"

**Owning Phase**: Phase 4 (Observability)

**Compliance Test**:
- Test: Execute workflow and verify all outputs
- Expected: Summary, report, graph, benchmark files generated
- Command: Run comprehensive workflow
- Result: All output files present

**Verification Method**:
- Test: Verify outputs are queryable
- Expected: Can filter and search outputs
- Command: Query observability data
- Result: Queryable results returned

### Constraint 5: Systematic quality improvement

**Quote**: "systematic quality improvement"

**Owning Phase**: Phase 4 (Quality improvement)

**Compliance Test**:
- Test: Execute quality improvement cycle
- Expected: Metrics drive systematic improvements
- Command: Run improvement workflow
- Result: Targeted improvements identified

**Verification Method**:
- Test: Verify improvements are measurable
- Expected: Quality metrics improve over iterations
- Command: Track metrics across improvement cycles
- Result: Measurable progress

---

## ADR-0006: Local Memory Retrieval Web Search and Web Scraping

**Owner Phase**: Phase 5 (Benchmarking and Observability)

**Constraint Count**: 6

### Constraint 1: Local memory added before remote search and scraping

**Quote**: "Local memory is added before remote search and scraping"

**Owning Phase**: Phase 5 (Memory priority)

**Compliance Test**:
- Test: Verify local memory works before web access
- Expected: RAG functions without network
- Command: Run local RAG workflow
- Result: Retrieval from local memory only

**Verification Method**:
- Test: Verify web access requires explicit opt-in
- Expected: No web access without permission
- Command: Attempt web operation without permission
- Result: Access denied

### Constraint 2: Retrieval has two planes: local structured and unstructured memory plus external research and scraping tools

**Quote**: "Retrieval has two planes: local structured and unstructured memory plus external research and scraping tools"

**Owning Phase**: Phase 5 (Retrieval planes)

**Compliance Test**:
- Test: Query both local and external sources
- Expected: Results from both planes
- Command: Run retrieval workflow
- Result: Combined results from all sources

**Verification Method**:
- Test: Verify retrieval planes are isolated
- Expected: Failure in one plane doesn't affect other
- Command: Block one plane, query other
- Result: Independent operation

### Constraint 3: Search combines exact or full-text retrieval with semantic retrieval

**Quote**: "Search combines exact or full-text retrieval with semantic retrieval rather than relying on vectors alone"

**Owning Phase**: Phase 5 (Hybrid search)

**Compliance Test**:
- Test: Query with exact match
- Expected: Exact matches returned
- Command: Search for exact term
- Result: Exact matches in results

**Verification Method**:
- Test: Verify semantic search works
- Expected: Conceptually similar results returned
- Command: Search with related terms
- Result: Semantic matches in results

### Constraint 4: Web access records provenance timestamps and extraction traces

**Quote**: "Web access records provenance timestamps and extraction traces as artifacts"

**Owning Phase**: Phase 5 (Web provenance)

**Compliance Test**:
- Test: Execute web operation and check provenance
- Expected: Timestamp, URL, extraction trace logged
- Command: Run web fetch workflow
- Result: Complete provenance recorded

**Verification Method**:
- test: Verify provenance is queryable
- Expected: Can search web operation history
- Command: Query web operation logs
- Result: History accessible

### Constraint 5: Robots and scope restrictions enforced before external scraping actions run

**Quote**: "Robots and scope restrictions are enforced before external scraping actions run"

**Owning Phase**: Phase 5 (Scraping safety)

**Compliance Test**:
- Test: Scrape website with robots.txt
- Expected: Robots rules respected
- Command: Run scraping workflow
- Result: No robots violations

**Verification Method**:
- test: Verify scope restrictions prevent over-scraping
- Expected: Scraping limited to allowed domains
- Command: Attempt scrape outside scope
- Result: Access denied

### Constraint 6: Memory and search artifacts stored in ./workspace/memory/ with versioned references

**Quote**: "Memory and search artifacts are stored in ./workspace/memory/ with versioned references"

**Owning Phase**: Phase 5 (Memory storage)

**Compliance Test**:
- Test: Store memory artifact and verify location
- Expected: Artifact in ./workspace/memory/
- Command: Store memory artifact
- Result: Correct path

**Verification Method**:
- Test: Verify memory artifacts are versioned
- Expected: Version references tracked
- Command: Store same memory twice
- Result: Different versions created

---

## ADR-0007: Cron Execution Git-Branch Experimentation and Refining Workflows

**Owner Phase**: Phase 6 (Automation and Scheduling)

**Constraint Count**: 6

### Constraint 1: Cron jobs instantiate versioned workflow runs from stable specs

**Quote**: "Cron jobs instantiate versioned workflow runs from stable specs"

**Owning Phase**: Phase 6 (Cron scheduling)

**Compliance Test**:
- Test: Schedule cron job and verify versioning
- Expected: Run associated with workflow version
- Command: Schedule job, check run metadata
- Result: Version info present

**Verification Method**:
- test: Verify stable spec is not modified during execution
- Expected: Spec remains read-only
- Command: Inspect spec during cron run
- Result: Spec unchanged

### Constraint 2: Backed-up text corpora are immutable inputs to scheduled experiments

**Quote**: "Backed-up text corpora are immutable inputs to scheduled experiments unless explicitly promoted"

**Owning Phase**: Phase 6 (Input immutability)

**Compliance Test**:
- Test: Run scheduled experiment
- Expected: Input corpus not modified
- Command: Execute cron job, check input files
- Result: Inputs unchanged

**Verification Method**:
- test: Verify promotion requires explicit action
- Expected: Cannot modify input without promote command
- Command: Attempt input modification
- Result: Modification rejected

### Constraint 3: Git-aware experiments run in isolated working copies or branches with explicit merge policies

**Quote**: "Git-aware experiments run in isolated working copies or branches with explicit merge policies"

**Owning Phase**: Phase 6 (Git isolation)

**Compliance Test**:
- Test: Run experiment and verify branch creation
- Expected: Experiment runs in dedicated branch
- Command: Execute git workflow
- Result: New branch created

**Verification Method**:
- test: Verify merge policies are enforced
- Expected: Merge follows defined policy
- Command: Attempt merge with violation
- Result: Merge blocked

### Constraint 4: Manual refinement is preserved as artifacted review events

**Quote**: "Manual refinement is preserved as artifacted review events rather than an implicit side channel"

**Owning Phase**: Phase 6 (Refinement tracking)

**Compliance Test**:
- Test: Manually refine workflow
- Expected: Refinement captured as review event
- Command: Edit workflow, check artifacts
- Result: Review event present

**Verification Method**:
- test: Verify review events are queryable
- Expected: Can retrieve refinement history
- Command: Query review events
- Result: History accessible

### Constraint 5: Merge recommendations are outputs not auto-commits until confidence thresholds

**Quote**: "Merge recommendations are outputs not auto-commits until confidence and validation thresholds are mature"

**Owning Phase**: Phase 6 (Merge safety)

**Compliance Test**:
- Test: Generate merge recommendation
- Expected: Recommendation as output, not auto-merged
- Command: Run merge workflow
- Result: Recommendation generated, not committed

**Verification Method**:
- test: Verify auto-commit requires high confidence
- Expected: Auto-commit only when confidence >= threshold
- Command: Vary confidence scores
- Result: Auto-commit only at high confidence

### Constraint 6: Scheduling and cron policies compiled into WorkflowIR

**Quote**: "Scheduling and cron policies are compiled into WorkflowIR not interpreted at runtime"

**Owning Phase**: Phase 6 (IR compilation)

**Compliance Test**:
- Test: Verify cron policy in IR
- Expected: Policy is IR node, not dynamic check
- Command: Inspect compiled IR for cron workflow
- Result: Policy as IR node

**Verification Method**:
- test: Verify IR is deterministic
- Expected: Same workflow always produces same IR
- Command: Compile cron workflow multiple times
- Result: Identical IRs

---

## ADR-0008: Autonomous Loops and Metrics

**Owner Phase**: Phase 6 (Automation and Scheduling)

**Constraint Count**: 7

### Constraint 1: Autonomous loops declared as workflow modes with bounded goals, checkpoints, and stop conditions

**Quote**: "Autonomous loops are declared workflow modes with bounded goals, checkpoints, and stop conditions"

**Owning Phase**: Phase 6 (Autonomous loops)

**Compliance Test**:
- Test: Declare autonomous workflow
- Expected: Bounds, checkpoints, stop conditions enforced
- Command: Run autonomous workflow
- Result: Respects all constraints

**Verification Method**:
- test: Verify stop conditions fire correctly
- Expected: Loop terminates at defined condition
- Command: Run autonomous workflow to completion
- Result: Terminates at stop condition

### Constraint 2: Metrics for FE and BE UX instrumented before large-scale interface experimentation

**Quote**: "Metrics for FE and BE UX are instrumented before large-scale interface experimentation"

**Owning Phase**: Phase 6 (UX metrics)

**Compliance Test**:
- Test: Verify UI metrics collection
- Expected: All UI interactions tracked
- Command: Use UI, check metrics
- Result: Complete interaction log

**Verification Method**:
- test: Verify backend UX metrics collected
- Expected: API response times, error rates tracked
- Command: Inspect backend metrics
- Result: UX metrics present

### Constraint 3: Objective measurements include usefulness, time-to-usefulness, intervention rate, repair rate, benchmark quality, operator trust

**Quote**: "Objective measurements include usefulness, time-to-usefulness, intervention rate, repair rate, benchmark quality, operator trust"

**Owning Phase**: Phase 6 (Objective metrics)

**Compliance Test**:
- Test: Verify all metrics are collected
- Expected: Usefulness, intervention, repair, trust metrics present
- Command: Run workflow, check metrics
- Result: All metrics captured

**Verification Method**:
- test: Verify metrics are actionable
- Expected: Metrics drive improvement decisions
- Command: Analyze metrics for insights
- Result: Actionable insights derived

### Constraint 4: Human override, pause, and scope visibility available at all times

**Quote**: "Human override, pause, and scope visibility remain available at all times"

**Owning Phase**: Phase 6 (Human override)

**Compliance Test**:
- Test: Verify override controls during autonomous run
- Expected: Can pause/stop autonomous workflow
- Command: Run autonomous workflow, attempt override
- Result: Override successful

**Verification Method**:
- test: Verify scope is always visible
- Expected: Current scope displayed throughout
- Command: Monitor UI during autonomous run
- Result: Scope always shown

### Constraint 5: Autonomy inherits provenance policy and artifact requirements from earlier ADRs

**Quote**: "Autonomy inherits provenance policy and artifact requirements from earlier ADRs"

**Owning Phase**: Phase 6 (Inheritance)

**Compliance Test**:
- Test: Verify autonomous workflows use provenance
- Expected: All autonomous ops have provenance
- Command: Run autonomous workflow
- Result: Complete provenance tracked

**Verification Method**:
- test: Verify artifact requirements met
- Expected: All autonomous ops produce artifacts
- Command: Check ./workspace/ after autonomous run
- Result: All required artifacts present

### Constraint 6: Observability and metrics collection integrated into transpiler runtime

**Quote**: "Observability and metrics collection is integrated into transpiler runtime not bolted on"

**Owning Phase**: Phase 6 (Integrated observability)

**Compliance Test**:
- Test: Verify metrics collected at IR level
- Expected: No bolt-on metrics collection
- Command: Inspect IR execution trace
- Result: Metrics integral to execution

**Verification Method**:
- test: Verify observability hooks in IR
- Expected: Hooks are IR nodes, not external
- Command: Analyze IR structure
- Result: Observability as IR components

### Constraint 7: Bounded autonomy enables safe experimentation with clear limits

**Quote**: "bounded autonomy enables safe experimentation with clear limits"

**Owning Phase**: Phase 6 (Safe autonomy)

**Compliance Test**:
- Test: Verify autonomy bounds are enforced
- Expected: Workflow respects all limits
- Command: Run autonomous workflow to limits
- Result: Respects boundaries

**Verification Method**:
- test: Verify clear limits are documented
- Expected: Limits visible in workflow spec
- Command: Inspect autonomous workflow spec
- Result: Limits explicitly defined

---

## Cross-Cutting Constraints

These constraints span multiple phases and are verified incrementally.

### Constraint: System-of-record persistence

**Quote**: "Every run persists: workflow specs, policy snapshots, artifact references, hashes, provenance metadata"

**Owning Phases**: Phase 0 (Foundation), Phase 1 (Execution), Phase 2 (Persistence)

**Compliance Test**:
- Phase 0: Verify ./workspace/ structure created
- Phase 1: Verify run artifacts saved
- Phase 2: Verify persistence across restarts

**Verification Method**:
- Check: All artifacts are queryable
- Expected: Can retrieve any run's complete context
- Command: Query ./workspace/ for historical run
- Result: Complete context retrieved

### Constraint: Local-first defaults

**Quote**: "Local-first operation with privacy-preserving defaults"

**Owning Phases**: Phase 0 (Foundation), Phase 2 (Networking boundary), Phase 5 (Search opt-in)

**Compliance Test**:
- Phase 0: Verify no external calls by default
- Phase 2: Verify networking opt-in required
- Phase 5: Verify web access requires permission

**Verification Method**:
- Check: Verify no data leaves system without opt-in
- Expected: All outbound traffic logged and approved
- Command: Monitor network during default runs
- Result: No unauthorized egress

### Constraint: Type safety and validation

**Quote**: "WorkflowSpec defines all required fields with types, IR validation passes without errors"

**Owning Phases**: Phase 0 (Schema), Phase 1 (Validation)

**Compliance Test**:
- Phase 0: Verify all fields have types
- Phase 1: Verify validation catches errors

**Verification Method**:
- Check: Invalid workflows rejected
- Expected: Schema validation prevents invalid workflows
- Command: Test invalid workflows
- Result: All invalid workflows rejected

---

## Summary Matrix

| ADR | Owner Phase | Constraint Count | Status |
|------|--------------|------------------|--------|
| ADR-0000 | Phase 0 | 1 | ✅ Mapped |
| ADR-0001 | Phase 0 | 8 | ✅ Mapped |
| ADR-0002 | Phase 1 | 8 | ✅ Mapped |
| ADR-0003 | Phase 2 | 7 | ✅ Mapped |
| ADR-0004 | Phase 7 | 6 | ✅ Mapped |
| ADR-0005 | Phase 4 | 5 | ✅ Mapped |
| ADR-0006 | Phase 5 | 6 | ✅ Mapped |
| ADR-0007 | Phase 6 | 6 | ✅ Mapped |
| ADR-0008 | Phase 6 | 7 | ✅ Mapped |
| Cross-Cutting | Phase 0-6 | 3 | ✅ Mapped |

**Total Constraints**: 47
**Coverage**: 100% (All ADR constraints mapped to phases)
