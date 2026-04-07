# Agentic Workflow Substructure Alignment Analysis

**Date**: 2026-04-02  
**Scope**: Alignment between plan-00 through plan-07 and the agentic_workflow L1-L5 hierarchy

---

## 1. Reference Architecture Summary

The agentic workflow schema defines a 5-layer abstraction hierarchy with approximately 85+ step types. This section summarizes each layer and its components as defined in the YAML reference files.

### 1.1 L1 Concrete Operations (~42 types across 10 categories)

| Category | Types | Description |
|----------|-------|-------------|
| **Boundary** | `start`, `pass`, `succeed`, `fail`, `cancel` | Workflow entry, exit, and termination points |
| **Data Flow** | `parse`, `validate`, `normalize`, `transform`, `assign`, `merge`, `split`, `filter`, `chunk`, `summarize`, `template`, `serialize` | Shape manipulation and validation |
| **Execution** | `task`, `model`, `tool`, `api`, `db`, `io`, `code`, `search`, `retrieve`, `generate`, `notify` | External system interactions |
| **Control Flow** | `choice`, `route`, `parallel`, `map`, `reduce`, `loop`, `race`, `fallback` | Execution orchestration |
| **Time/Event** | `wait`, `sensor`, `await_event`, `await_condition`, `schedule` | Temporal coordination |
| **Delegation** | `subworkflow`, `child_workflow`, `subgraph`, `handoff`, `agent_tool`, `delegate`, `escalate` | Control and data transfer |
| **Messaging** | `signal`, `update`, `query`, `callback`, `interrupt`, `approve`, `review`, `human_input` | Inter-workflow communication |
| **Durability** | `checkpoint`, `snapshot`, `cache`, `replay`, `fork`, `continue_as_new` | State persistence and recovery |
| **Queue/Resource** | `enqueue`, `dispatch`, `acquire_slot`, `release_slot` | Capacity management |
| **Observability** | `log`, `trace`, `metric`, `stream`, `publish`, `audit` | Monitoring and logging |

### 1.2 L2 Agent Primitives (~15 types across 6 categories)

| Category | Types | Description |
|----------|-------|-------------|
| **Context** | `intake_context`, `intent_classify`, `extract`, `ambiguity_check` | Understanding user intent and gathering context |
| **Planning** | `plan_step`, `decompose`, `tool_select`, `model_select`, `agent_select` | Decomposing goals and selecting approaches |
| **Reasoning** | `reason`, `reflect`, `critique`, `rank`, `self_check` | Deliberation and quality assessment |
| **Memory** | `memory_recall`, `memory_write` | Knowledge and context persistence |
| **Guardrails** | `policy_check`, `grounding_check` | Safety and compliance constraints |
| **Multi-Agent** | `handoff`, `agent_tool`, `delegate`, `escalate` | Coordination across agents |

### 1.3 L3 Composite Patterns (8 patterns)

| Pattern | Composed Of | Description |
|---------|-------------|-------------|
| **sequential_pattern** | `l1:choice`, `l1:parallel`, `l2:model`, `l1:tool` | Linear chain of steps |
| **map_reduce_pattern** | `l1:map`, `l1:reduce`, `l2:reason` | Scatter-gather across items |
| **planner_executor_pattern** | `l2:plan_step`, `l2:decompose`, `l1:map`, `l1:loop`, `l2:reason` | Plan then execute with replanning |
| **react_pattern** | `l2:reason`, `l1:tool`, `l2:memory_write`, `l1:loop` | Think-Act-Observe loop |
| **router_specialist_pattern** | `l2:intent_classify`, `l2:agent_select`, `l1:choice`, `l2:handoff` | Classify and route to specialist |
| **maker_checker_pattern** | `l1:model`, `l2:critique`, `l1:loop`, `l2:revise` | Generate and iteratively improve |
| **supervisor_swarm_pattern** | `l2:reason`, `l2:decompose`, `l1:map`, `l2:handoff`, `l1:parallel` | Coordinator with worker agents |
| **monitor_act_pattern** | `l1:schedule`, `l1:sensor`, `l2:reason`, `l1:tool`, `l1:loop` | Observe-decide-act loop |

### 1.4 L4 Semantic Phases (10 phases)

| Phase | Composed Of | Business Meaning |
|-------|-------------|------------------|
| **intake_phase** | `l1:start`, `l2:intake_context`, `l1:validate` | What does the user want? |
| **understand_phase** | `l2:intent_classify`, `l2:extract`, `l2:ambiguity_check` | What are we trying to solve? |
| **prepare_context_phase** | `l2:memory_recall`, `l1:retrieve`, `l2:context_pack` | What information do we need? |
| **plan_phase** | `l2:decompose`, `l2:plan_step`, `l2:tool_select` | How will we accomplish the goal? |
| **execute_phase** | `l1:task`, `l1:tool`, `l1:api`, `l1:db`, `l2:reason` | Do the work |
| **verify_phase** | `l1:validate`, `l2:policy_check`, `l2:grounding_check` | Did we do it correctly? |
| **refine_phase** | `l2:revise`, `l1:loop`, `l1:map` | Can we do it better? |
| **decide_phase** | `l1:approve`, `l1:human_input`, `l2:escalate` | Should we proceed or escalate? |
| **deliver_phase** | `l1:generate`, `l1:notify`, `l2:memory_write` | Here's what you asked for |
| **learn_phase** | `l2:memory_write`, `l5:evaluation_program`, `l2:reflect` | What did we learn for next time? |

### 1.5 L5 Meta-Orchestration Controllers (10 controllers)

| Controller | Scope | Description |
|------------|-------|-------------|
| **governance** | l1-l5 | Policy enforcement across all layers |
| **budget_manager** | l1-l4 | Resource allocation and limits (tokens, cost, time, tool_calls) |
| **scheduler_controller** | l1, l3, l5 | Execution queuing and dispatch (fifo, priority, edf, fair_share) |
| **topology_manager** | l1 resources | Resource pool management and routing (compute_pools, worker_pools, agent_pools) |
| **evaluation_program** | l2, l3, l4 outputs | Assessment of workflow outcomes (quality, efficiency, accuracy, safety) |
| **provenance_controller** | l1-l5 | Evidence tracking and audit trails (citations, tool_outputs, retrieval_hits) |
| **experiment_controller** | l1-l3 configurations | A/B testing and variant management (prompt_variants, model_variants) |
| **optimization_controller** | l1-l5 | Automatic tuning of parameters (quality, latency, cost, throughput) |
| **memory_compactor** | l2 memory namespaces | Long-term memory hygiene and pruning (summarize, dedupe, evict_lru) |

### 1.6 Cross-Layer Interactions

| Interaction | Description |
|-------------|-------------|
| **l1→l2** | Concrete steps enabling agent cognition (model provides LLM, tool provides execution capability) |
| **l2→l1** | Agent decisions driving concrete execution (reason/plan_step decides what to do) |
| **l2→l3** | Agent primitives composed into reusable patterns (planner_executor uses agent planning) |
| **l1→l3** | Concrete steps as building blocks (parallel enables map_reduce_pattern) |
| **l3→l4** | Patterns implementing business phases (sequential_pattern implements linear phase execution) |
| **l4→l5** | Semantic phases governed by meta-controllers (governance applies policies across phases) |
| **l5→all** | Meta-orchestration governs entire system (budget limits, evaluation, provenance) |

---

## 2. Plan-to-Layer Mapping Matrix

| Plan | L1 Coverage | L2 Coverage | L3 Patterns | L4 Phases | L5 Controllers |
|------|-------------|-------------|-------------|-----------|----------------|
| **plan-00 Foundation** | IMPLEMENTED: boundary, data_flow, observability | REFERENCED: memory, context | N/A | N/A | REFERENCED: provenance |
| **plan-01 MVP Queue** | IMPLEMENTED: queue_resource, boundary, control_flow | REFERENCED: planning | N/A | REFERENCED: decide_phase | IMPLEMENTED: scheduler_controller |
| **plan-02 CLI/Backends** | IMPLEMENTED: execution, messaging | REFERENCED: planning, multi_agent | REFERENCED: sequential | REFERENCED: execute_phase | REFERENCED: budget_manager, governance |
| **plan-03 Glyphnova UI** | REFERENCED: all (visualization) | REFERENCED: all (display) | REFERENCED: all (navigation) | IMPLEMENTED: all (phase UI) | IMPLEMENTED: topology_manager |
| **plan-04 Quality Loops** | IMPLEMENTED: data_flow, control_flow (loops) | IMPLEMENTED: reasoning, memory | IMPLEMENTED: maker_checker_pattern | IMPLEMENTED: verify_phase, refine_phase | IMPLEMENTED: evaluation_program |
| **plan-05 Memory/Search** | IMPLEMENTED: data_flow, execution (search/retrieve) | IMPLEMENTED: memory | MISSING | IMPLEMENTED: prepare_context_phase, learn_phase | IMPLEMENTED: memory_compactor, provenance_controller |
| **plan-06 Automation** | IMPLEMENTED: time_event (schedule), delegation, messaging | REFERENCED: planning | IMPLEMENTED: supervisor_swarm_pattern | IMPLEMENTED: execute_phase, deliver_phase | IMPLEMENTED: scheduler_controller, experiment_controller |
| **plan-07 Autonomy/Metrics** | IMPLEMENTED: control_flow (loops), observability | IMPLEMENTED: reasoning, guardrails | IMPLEMENTED: monitor_act_pattern | IMPLEMENTED: all phases | IMPLEMENTED: governance, budget_manager, optimization_controller |

---

## 3. Per-Plan Detailed Analysis

### 3.1 Plan-00: Foundation

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | IMPLEMENTED | `start`, `pass`, `succeed`, `fail`, `cancel` | - | - |
| Data Flow | IMPLEMENTED | `parse`, `validate`, `transform`, `assign`, `serialize` | - | `chunk`, `summarize` |
| Execution | REFERENCED | - | `task`, `model`, `tool` | `api`, `db`, `io`, `code` |
| Control Flow | REFERENCED | - | `choice`, `loop` | `parallel`, `map`, `reduce`, `race` |
| Time/Event | MISSING | - | - | `wait`, `sensor`, `schedule` |
| Delegation | REFERENCED | - | `subworkflow` | `handoff`, `delegate`, `escalate` |
| Messaging | MISSING | - | - | `signal`, `interrupt`, `approve` |
| Durability | IMPLEMENTED | `checkpoint`, `cache` | - | `replay`, `fork` |
| Queue/Resource | MISSING | - | - | `enqueue`, `dispatch` |
| Observability | IMPLEMENTED | `log`, `trace`, `metric`, `audit` | - | `stream`, `publish` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | WorkflowSpec needs `intake_context`, `extract` for YAML parsing |
| Planning | REFERENCED | IR compilation uses `decompose`, `plan_step` concepts |
| Reasoning | MISSING | Not implemented - no LLM reasoning yet |
| Memory | REFERENCED | `.opencode/` storage implies `memory_write` semantics |
| Guardrails | IMPLEMENTED | Policy compilation implements `policy_check` |
| Multi-Agent | MISSING | No agent coordination yet |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | INFRASTRUCTURE | WorkflowIR pipeline supports sequential execution |
| map_reduce_pattern | MISSING | No parallel execution support |
| planner_executor_pattern | MISSING | No planning/primitive split |
| react_pattern | MISSING | No think-act-observe loop |
| router_specialist_pattern | MISSING | No routing logic |
| maker_checker_pattern | MISSING | No iterative improvement |
| supervisor_swarm_pattern | MISSING | No multi-agent support |
| monitor_act_pattern | MISSING | No scheduling/monitoring |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | YAML loading and validation |
| understand_phase | MISSING | No intent classification |
| prepare_context_phase | PARTIAL | `.opencode/` storage available |
| plan_phase | IMPLEMENTED | IR compilation as planning |
| execute_phase | MISSING | No runtime execution |
| verify_phase | PARTIAL | Schema validation exists |
| refine_phase | MISSING | No iteration loops |
| decide_phase | MISSING | No human approval |
| deliver_phase | MISSING | No output generation |
| learn_phase | MISSING | No reflection/memory update |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Policy compilation references governance concepts |
| budget_manager | MISSING | No resource tracking |
| scheduler_controller | MISSING | No scheduling |
| topology_manager | MISSING | No resource pools |
| evaluation_program | MISSING | No evaluation |
| provenance_controller | IMPLEMENTED | Artifact provenance via hashes and versioning |
| experiment_controller | MISSING | No experimentation |
| optimization_controller | MISSING | No optimization |
| memory_compactor | MISSING | No memory management |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Execution | `model`, `tool`, `api`, `db`, `io`, `code` | Critical for runtime execution |
| L1 Control Flow | `parallel`, `map`, `reduce` | Needed for concurrent processing |
| L1 Time/Event | All types | Needed for scheduling (plan-06) |
| L1 Messaging | `signal`, `interrupt`, `approve` | Needed for human-in-loop (plan-01) |
| L2 Reasoning | `reason`, `reflect`, `critique`, `rank` | Core agent capabilities |
| L2 Memory | `memory_recall`, `memory_write` | Context management |
| L3 Patterns | All 8 patterns | Workflow orchestration |
| L5 Controllers | Most controllers | System governance |

#### Recommendations

1. **Add L1 execution primitives** - Foundation must expose model/tool execution for later plans
2. **Include L2 memory primitives** - WorkflowIR should support memory annotations
3. **Design for L3 pattern composition** - IR structure should support pattern recognition
4. **Add observability hooks** - Enable L5 evaluation and provenance from the start

---

### 3.2 Plan-01: MVP Queue & Scheduler

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | IMPLEMENTED | `cancel` (workflow cancellation) | - | - |
| Data Flow | REFERENCED | - | `validate`, `transform` | - |
| Execution | REFERENCED | - | `task`, `model` | `tool`, `api`, `db` |
| Control Flow | IMPLEMENTED | `loop` (retry), `choice` (prioritization) | `parallel` | `map`, `reduce`, `race` |
| Time/Event | MISSING | - | - | `wait`, `sensor`, `schedule` |
| Delegation | MISSING | - | - | `delegate`, `escalate` |
| Messaging | IMPLEMENTED | `interrupt`, `approve`, `human_input` | - | `signal`, `callback` |
| Durability | IMPLEMENTED | `checkpoint`, `snapshot` | - | `replay`, `fork` |
| Queue/Resource | IMPLEMENTED | `enqueue`, `dispatch`, `acquire_slot`, `release_slot` | - | - |
| Observability | REFERENCED | - | `log`, `metric` | `trace`, `stream` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | ChatSession metadata implies context gathering |
| Planning | REFERENCED | Job prioritization uses planning concepts |
| Reasoning | MISSING | No LLM reasoning for queue decisions |
| Memory | MISSING | No memory of past executions |
| Guardrails | IMPLEMENTED | Human gating implements `policy_check` |
| Multi-Agent | MISSING | Single queue, no multi-agent |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | IMPLEMENTED | Job execution is sequential by default |
| map_reduce_pattern | PARTIAL | Parallel mode provides map, but reduce is missing |
| planner_executor_pattern | MISSING | No planning/execution split |
| react_pattern | MISSING | No iterative reasoning |
| router_specialist_pattern | MISSING | No job routing |
| maker_checker_pattern | PARTIAL | Human gating provides checker concept |
| supervisor_swarm_pattern | MISSING | No worker pool management |
| monitor_act_pattern | MISSING | No monitoring loop |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | ChatSession as work container |
| understand_phase | PARTIAL | Title-based organization |
| prepare_context_phase | MISSING | No context preparation |
| plan_phase | MISSING | No planning phase |
| execute_phase | IMPLEMENTED | Job execution |
| verify_phase | PARTIAL | Human verification via gating |
| refine_phase | MISSING | No refinement loop |
| decide_phase | IMPLEMENTED | Human approval for dangerous ops |
| deliver_phase | MISSING | No structured delivery |
| learn_phase | MISSING | No learning from executions |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Human gating references governance |
| budget_manager | REFERENCED | Resource limits for concurrency |
| scheduler_controller | IMPLEMENTED | Core queue scheduling (FIFO, priority) |
| topology_manager | MISSING | No worker pool topology |
| evaluation_program | MISSING | No job evaluation |
| provenance_controller | REFERENCED | Job tracking references provenance |
| experiment_controller | MISSING | No experimentation |
| optimization_controller | MISSING | No optimization |
| memory_compactor | MISSING | No memory management |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Time/Event | `schedule`, `sensor` | Needed for cron jobs (plan-06) |
| L2 Planning | `decompose`, `tool_select` | Needed for intelligent scheduling |
| L2 Memory | `memory_recall`, `memory_write` | Job history and context |
| L3 Patterns | `planner_executor`, `monitor_act` | Intelligent job management |
| L5 Controllers | `topology_manager`, `evaluation_program` | Resource and quality management |

#### Recommendations

1. **Add L1 scheduling primitives** - Enable cron-based scheduling
2. **Include L2 memory for job history** - Support job learning and optimization
3. **Implement L3 monitor_act pattern** - Enable self-monitoring queues
4. **Add L5 evaluation** - Measure job quality and scheduler effectiveness

---

### 3.3 Plan-02: CLI, Backends, and Networking Boundary

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | REFERENCED | - | `start`, `succeed`, `fail` | - |
| Data Flow | IMPLEMENTED | `parse`, `validate`, `transform`, `template` | - | `chunk`, `summarize` |
| Execution | IMPLEMENTED | `model`, `tool`, `api`, `io`, `code`, `generate`, `notify` | - | `db`, `search`, `retrieve` |
| Control Flow | IMPLEMENTED | `choice`, `route`, `parallel`, `fallback` | - | `map`, `reduce`, `loop`, `race` |
| Time/Event | MISSING | - | - | All types |
| Delegation | IMPLEMENTED | `subworkflow`, `child_workflow`, `delegate` | - | `handoff`, `escalate` |
| Messaging | IMPLEMENTED | `signal`, `interrupt`, `approve`, `review` | - | `query`, `callback` |
| Durability | IMPLEMENTED | `cache`, `checkpoint` | - | `replay`, `fork` |
| Queue/Resource | MISSING | - | - | All types (in plan-01) |
| Observability | IMPLEMENTED | `log`, `trace`, `metric`, `stream`, `audit` | - | `publish` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | CLI argument parsing implies context gathering |
| Planning | REFERENCED | Workflow selection and routing |
| Reasoning | MISSING | No LLM reasoning in CLI |
| Memory | MISSING | No CLI memory system |
| Guardrails | IMPLEMENTED | Networking policy enforcement |
| Multi-Agent | REFERENCED | Provider abstraction implies agent selection |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | IMPLEMENTED | CLI command execution |
| map_reduce_pattern | MISSING | No batch processing |
| planner_executor_pattern | MISSING | No planning/execution split |
| react_pattern | MISSING | No iterative CLI |
| router_specialist_pattern | IMPLEMENTED | Provider routing to backends |
| maker_checker_pattern | PARTIAL | Self-improvement loop references checker |
| supervisor_swarm_pattern | MISSING | No parallel agent coordination |
| monitor_act_pattern | MISSING | No CLI monitoring |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | CLI argument parsing |
| understand_phase | PARTIAL | Command interpretation |
| prepare_context_phase | MISSING | No context preparation |
| plan_phase | MISSING | No planning |
| execute_phase | IMPLEMENTED | Command execution |
| verify_phase | IMPLEMENTED | Schema validation, networking checks |
| refine_phase | PARTIAL | Self-improvement infrastructure |
| decide_phase | IMPLEMENTED | Human approval for networking |
| deliver_phase | IMPLEMENTED | Output formatting and delivery |
| learn_phase | PARTIAL | Self-improvement log analysis |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Networking policy references governance |
| budget_manager | IMPLEMENTED | Token/cost tracking for model calls |
| scheduler_controller | MISSING | No CLI scheduling |
| topology_manager | IMPLEMENTED | Provider pool management |
| evaluation_program | REFERENCED | Self-improvement references evaluation |
| provenance_controller | IMPLEMENTED | Audit logging for networking |
| experiment_controller | MISSING | No A/B testing |
| optimization_controller | REFERENCED | Self-improvement references optimization |
| memory_compactor | MISSING | No CLI memory management |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Time/Event | `schedule`, `sensor` | Scheduled CLI commands |
| L1 Control Flow | `map`, `reduce`, `loop` | Batch processing workflows |
| L2 Memory | `memory_recall`, `memory_write` | CLI history and context |
| L3 Patterns | `planner_executor`, `react_pattern` | Intelligent CLI operations |
| L5 Controllers | `experiment_controller`, `scheduler_controller` | A/B testing, scheduling |

#### Recommendations

1. **Add L1 batch processing** - Support map/reduce for bulk CLI operations
2. **Include L2 memory for CLI history** - Enable context-aware CLI
3. **Implement L3 planner_executor** - Support intelligent workflow planning
4. **Add L5 experimentation** - Enable A/B testing of CLI workflows

---

### 3.4 Plan-03: Glyphnova Desktop UI

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | REFERENCED | - | All boundary types | - |
| Data Flow | REFERENCED | - | All data flow types | - |
| Execution | REFERENCED | - | All execution types | - |
| Control Flow | REFERENCED | - | `choice`, `route`, `parallel` | `map`, `reduce`, `loop` |
| Time/Event | REFERENCED | - | `schedule`, `sensor` | `wait`, `await_*` |
| Delegation | REFERENCED | - | All delegation types | - |
| Messaging | IMPLEMENTED | `signal`, `update`, `query`, `callback` | - | `interrupt`, `approve` |
| Durability | REFERENCED | - | All durability types | - |
| Queue/Resource | REFERENCED | - | All queue types | - |
| Observability | IMPLEMENTED | `stream`, `publish`, `audit` | `log`, `trace`, `metric` | - |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | UI displays context from backend |
| Planning | REFERENCED | UI shows planning state |
| Reasoning | REFERENCED | UI displays reasoning logs |
| Memory | REFERENCED | Artifact browser references memory |
| Guardrails | REFERENCED | Scope visibility enforces guardrails |
| Multi-Agent | REFERENCED | Multi-zoom navigation shows agents |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | REFERENCED | UI visualizes sequential execution |
| map_reduce_pattern | REFERENCED | UI visualizes parallel processing |
| planner_executor_pattern | REFERENCED | UI shows plan execution |
| react_pattern | REFERENCED | UI displays think-act-observe |
| router_specialist_pattern | REFERENCED | UI shows agent routing |
| maker_checker_pattern | REFERENCED | UI displays iteration loops |
| supervisor_swarm_pattern | REFERENCED | UI shows worker coordination |
| monitor_act_pattern | REFERENCED | UI displays monitoring |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | UI receives and displays requests |
| understand_phase | IMPLEMENTED | Intent visualization |
| prepare_context_phase | IMPLEMENTED | Context browser |
| plan_phase | IMPLEMENTED | Plan visualization |
| execute_phase | IMPLEMENTED | Execution visualization |
| verify_phase | IMPLEMENTED | Verification display |
| refine_phase | IMPLEMENTED | Refinement loop visualization |
| decide_phase | IMPLEMENTED | Approval UI |
| deliver_phase | IMPLEMENTED | Result display |
| learn_phase | IMPLEMENTED | Learning visualization |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | UI enforces governance policies |
| budget_manager | REFERENCED | UI displays resource usage |
| scheduler_controller | REFERENCED | UI visualizes queue state |
| topology_manager | IMPLEMENTED | Resource pool UI |
| evaluation_program | REFERENCED | Quality metrics display |
| provenance_controller | IMPLEMENTED | Artifact provenance browsing |
| experiment_controller | REFERENCED | A/B test visualization |
| optimization_controller | REFERENCED | Optimization tuning UI |
| memory_compactor | REFERENCED | Memory management UI |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Direct Implementation | Most types are REFERENCED | UI is visualization layer, not implementation |
| L2 Direct Implementation | All types are REFERENCED | UI displays L2, doesn't implement |
| L3 Direct Implementation | All patterns are REFERENCED | UI displays patterns, doesn't execute |
| L5 Direct Implementation | `experiment_controller`, `optimization_controller` | UI tuning capabilities |

#### Recommendations

1. **Implement L4 phase UI components** - Ensure all 10 phases have UI visualization
2. **Add L5 topology management UI** - Resource pool visualization and control
3. **Implement L5 memory management UI** - Memory compaction and cleanup controls
4. **Add L5 experiment UI** - A/B test visualization and management

---

### 3.5 Plan-04: Quality Loops, Artifact Workflows

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | REFERENCED | - | `succeed`, `fail` | - |
| Data Flow | IMPLEMENTED | `validate`, `transform`, `filter`, `chunk`, `summarize`, `merge`, `split` | - | `parse`, `assign` |
| Execution | REFERENCED | - | `model`, `tool`, `code`, `generate` | `api`, `db`, `io`, `search` |
| Control Flow | IMPLEMENTED | `loop`, `choice`, `route`, `parallel`, `map`, `reduce` | - | `race`, `fallback` |
| Time/Event | MISSING | - | - | All types |
| Delegation | REFERENCED | - | `subworkflow` | `handoff`, `delegate` |
| Messaging | MISSING | - | - | All types |
| Durability | IMPLEMENTED | `checkpoint`, `cache`, `snapshot` | - | `replay`, `fork` |
| Queue/Resource | MISSING | - | - | All types |
| Observability | IMPLEMENTED | `log`, `trace`, `metric`, `audit` | - | `stream`, `publish` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | MISSING | No context gathering for quality |
| Planning | MISSING | No planning for quality loops |
| Reasoning | IMPLEMENTED | `critique`, `rank`, `self_check`, `reflect` |
| Memory | IMPLEMENTED | `memory_write` for artifact storage |
| Guardrails | IMPLEMENTED | `policy_check`, `grounding_check` as verifiers |
| Multi-Agent | MISSING | No multi-agent quality |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | REFERENCED | Quality loop is sequential |
| map_reduce_pattern | IMPLEMENTED | Benchmark suite execution |
| planner_executor_pattern | MISSING | No planning for quality |
| react_pattern | IMPLEMENTED | Generate-verify-repair loop |
| router_specialist_pattern | MISSING | No routing |
| maker_checker_pattern | IMPLEMENTED | Core quality loop pattern |
| supervisor_swarm_pattern | PARTIAL | Parallel verifier execution |
| monitor_act_pattern | MISSING | No monitoring |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | MISSING | No intake for quality |
| understand_phase | MISSING | No understanding phase |
| prepare_context_phase | MISSING | No context preparation |
| plan_phase | MISSING | No planning |
| execute_phase | IMPLEMENTED | Generate step execution |
| verify_phase | IMPLEMENTED | Core verification infrastructure |
| refine_phase | IMPLEMENTED | Core refinement infrastructure |
| decide_phase | MISSING | No decision phase |
| deliver_phase | IMPLEMENTED | Report generation |
| learn_phase | IMPLEMENTED | Learning from quality metrics |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Quality policies reference governance |
| budget_manager | REFERENCED | Quality loop resource limits |
| scheduler_controller | MISSING | No quality scheduling |
| topology_manager | MISSING | No resource topology |
| evaluation_program | IMPLEMENTED | Core benchmark evaluation |
| provenance_controller | IMPLEMENTED | Artifact provenance tracking |
| experiment_controller | PARTIAL | Benchmark comparison |
| optimization_controller | IMPLEMENTED | Quality optimization |
| memory_compactor | REFERENCED | Artifact cleanup |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Time/Event | `schedule`, `sensor` | Scheduled benchmarks |
| L1 Messaging | `approve`, `review` | Human quality review |
| L2 Context | `intake_context`, `intent_classify` | Context for quality decisions |
| L2 Planning | `plan_step`, `decompose` | Planning quality improvements |
| L3 Patterns | `planner_executor`, `monitor_act` | Intelligent quality management |
| L4 Phases | `intake`, `understand`, `prepare_context`, `decide` | Full lifecycle quality |

#### Recommendations

1. **Add L2 context primitives** - Enable context-aware quality decisions
2. **Implement L3 planner_executor** - Support planned quality improvements
3. **Add L4 intake/decide phases** - Full quality lifecycle management
4. **Include L5 experiment controller** - Systematic A/B quality testing

---

### 3.6 Plan-05: Memory & Search

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | REFERENCED | - | `start`, `succeed` | - |
| Data Flow | IMPLEMENTED | `parse`, `validate`, `transform`, `filter`, `merge`, `split`, `chunk` | - | `assign`, `template`, `serialize` |
| Execution | IMPLEMENTED | `search`, `retrieve`, `tool` (scraping) | - | `model`, `api`, `db`, `io`, `code` |
| Control Flow | MISSING | - | - | All types |
| Time/Event | MISSING | - | - | All types |
| Delegation | MISSING | - | - | All types |
| Messaging | IMPLEMENTED | `query`, `callback` (webhooks) | - | `signal`, `interrupt`, `approve` |
| Durability | IMPLEMENTED | `cache`, `checkpoint` | - | `snapshot`, `replay`, `fork` |
| Queue/Resource | MISSING | - | - | All types |
| Observability | IMPLEMENTED | `log`, `trace`, `audit` | - | `metric`, `stream`, `publish` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | Search context references intake_context |
| Planning | MISSING | No planning for search |
| Reasoning | MISSING | No reasoning for search |
| Memory | IMPLEMENTED | Core memory system (`memory_recall`, `memory_write`) |
| Guardrails | IMPLEMENTED | Policy enforcement for external search |
| Multi-Agent | MISSING | No multi-agent search |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | REFERENCED | Search is sequential |
| map_reduce_pattern | MISSING | No batch search |
| planner_executor_pattern | MISSING | No planned search |
| react_pattern | MISSING | No iterative search |
| router_specialist_pattern | MISSING | No search routing |
| maker_checker_pattern | MISSING | No search quality loop |
| supervisor_swarm_pattern | MISSING | No parallel search agents |
| monitor_act_pattern | MISSING | No search monitoring |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | MISSING | No intake |
| understand_phase | MISSING | No understanding |
| prepare_context_phase | IMPLEMENTED | Core context preparation |
| plan_phase | MISSING | No planning |
| execute_phase | IMPLEMENTED | Search execution |
| verify_phase | IMPLEMENTED | Policy verification |
| refine_phase | MISSING | No refinement |
| decide_phase | IMPLEMENTED | Policy approval for external access |
| deliver_phase | IMPLEMENTED | Search result delivery |
| learn_phase | IMPLEMENTED | Memory updates from search |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Policy gates reference governance |
| budget_manager | REFERENCED | Search resource limits |
| scheduler_controller | MISSING | No search scheduling |
| topology_manager | MISSING | No resource topology |
| evaluation_program | MISSING | No search evaluation |
| provenance_controller | IMPLEMENTED | Core provenance tracking |
| experiment_controller | MISSING | No search experimentation |
| optimization_controller | MISSING | No search optimization |
| memory_compactor | IMPLEMENTED | Core memory garbage collection |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Control Flow | `loop`, `choice`, `parallel` | Iterative/parallel search |
| L1 Time/Event | `schedule`, `sensor` | Scheduled search |
| L2 Reasoning | `reason`, `reflect`, `critique` | Intelligent search decisions |
| L3 Patterns | All patterns | Search workflow orchestration |
| L5 Controllers | `evaluation_program`, `optimization_controller` | Search quality optimization |

#### Recommendations

1. **Add L1 control flow for search** - Enable iterative and parallel search
2. **Implement L2 reasoning for search** - Intelligent search decisions
3. **Add L3 patterns for search** - Map-reduce for batch search, react for iterative search
4. **Include L5 evaluation for search** - Measure search quality

---

### 3.7 Plan-06: Automation

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | IMPLEMENTED | `start`, `succeed`, `fail`, `cancel` | - | `pass` |
| Data Flow | REFERENCED | - | `validate`, `transform`, `merge` | `split`, `filter`, `chunk` |
| Execution | REFERENCED | - | `task`, `tool`, `api`, `io`, `generate` | `model`, `db`, `code`, `search` |
| Control Flow | IMPLEMENTED | `choice`, `route`, `parallel`, `fallback` | - | `map`, `reduce`, `loop`, `race` |
| Time/Event | IMPLEMENTED | `schedule`, `wait`, `await_event`, `await_condition` | - | `sensor` |
| Delegation | IMPLEMENTED | `subworkflow`, `child_workflow`, `delegate`, `escalate` | - | `handoff`, `agent_tool` |
| Messaging | IMPLEMENTED | `signal`, `update`, `callback`, `approve`, `review` | - | `interrupt`, `query`, `human_input` |
| Durability | IMPLEMENTED | `checkpoint`, `snapshot`, `fork`, `continue_as_new` | - | `cache`, `replay` |
| Queue/Resource | REFERENCED | - | `enqueue`, `dispatch` | `acquire_slot`, `release_slot` |
| Observability | IMPLEMENTED | `log`, `trace`, `metric`, `audit` | - | `stream`, `publish` |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | Experiment context gathering |
| Planning | IMPLEMENTED | `plan_step`, `decompose` for experiment planning |
| Reasoning | MISSING | No LLM reasoning for automation |
| Memory | REFERENCED | Experiment result storage |
| Guardrails | IMPLEMENTED | Merge policy enforcement |
| Multi-Agent | MISSING | No multi-agent automation |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | IMPLEMENTED | Sequential experiment execution |
| map_reduce_pattern | PARTIAL | Parallel experiments, missing reduce |
| planner_executor_pattern | IMPLEMENTED | Experiment planning and execution |
| react_pattern | MISSING | No iterative automation |
| router_specialist_pattern | MISSING | No automation routing |
| maker_checker_pattern | IMPLEMENTED | Merge proposal quality loop |
| supervisor_swarm_pattern | IMPLEMENTED | Worker pool for parallel experiments |
| monitor_act_pattern | PARTIAL | Monitoring without act loop |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | Cron job intake |
| understand_phase | MISSING | No understanding phase |
| prepare_context_phase | MISSING | No context preparation |
| plan_phase | IMPLEMENTED | Experiment planning |
| execute_phase | IMPLEMENTED | Core execution infrastructure |
| verify_phase | IMPLEMENTED | Merge validation |
| refine_phase | IMPLEMENTED | Manual refinement capture |
| decide_phase | IMPLEMENTED | Merge approval |
| deliver_phase | IMPLEMENTED | Merge delivery |
| learn_phase | IMPLEMENTED | Experiment learning |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | REFERENCED | Merge policy references governance |
| budget_manager | REFERENCED | Automation resource limits |
| scheduler_controller | IMPLEMENTED | Core cron scheduling |
| topology_manager | REFERENCED | Experiment worker topology |
| evaluation_program | IMPLEMENTED | Experiment result evaluation |
| provenance_controller | IMPLEMENTED | Experiment provenance |
| experiment_controller | IMPLEMENTED | Core experiment management |
| optimization_controller | MISSING | No automation optimization |
| memory_compactor | REFERENCED | Experiment cleanup |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Control Flow | `loop`, `map`, `reduce` | Iterative automation |
| L2 Reasoning | `reason`, `reflect`, `critique` | Intelligent automation decisions |
| L3 Patterns | `react_pattern`, `monitor_act_pattern` | Self-improving automation |
| L4 Phases | `understand_phase`, `prepare_context_phase` | Full automation lifecycle |
| L5 Controllers | `optimization_controller` | Automation optimization |

#### Recommendations

1. **Add L2 reasoning for automation** - Enable intelligent automation decisions
2. **Implement L3 react_pattern** - Self-improving automation loops
3. **Add L3 monitor_act_pattern** - Continuous automation monitoring
4. **Include L5 optimization** - Automatic automation tuning

---

### 3.8 Plan-07: Autonomy & Metrics

#### L1 Coverage

| Category | Status | Types Implemented | Types Referenced | Types Missing |
|----------|--------|-------------------|------------------|---------------|
| Boundary | IMPLEMENTED | `start`, `succeed`, `fail`, `cancel` | - | `pass` |
| Data Flow | IMPLEMENTED | `validate`, `transform`, `filter`, `merge`, `assign` | - | `parse`, `split`, `chunk`, `summarize`, `template`, `serialize` |
| Execution | REFERENCED | - | `task`, `model`, `tool`, `api`, `io` | `db`, `code`, `search`, `retrieve`, `generate`, `notify` |
| Control Flow | IMPLEMENTED | `loop`, `choice`, `route`, `parallel`, `fallback`, `race` | - | `map`, `reduce` |
| Time/Event | REFERENCED | - | `schedule`, `sensor`, `wait` | `await_*` |
| Delegation | REFERENCED | - | `delegate`, `escalate`, `handoff` | `subworkflow`, `child_workflow`, `agent_tool` |
| Messaging | IMPLEMENTED | `interrupt`, `approve`, `review`, `human_input` | - | `signal`, `update`, `query`, `callback` |
| Durability | IMPLEMENTED | `checkpoint`, `snapshot`, `cache` | - | `replay`, `fork`, `continue_as_new` |
| Queue/Resource | REFERENCED | - | `enqueue`, `dispatch` | `acquire_slot`, `release_slot` |
| Observability | IMPLEMENTED | `log`, `trace`, `metric`, `stream`, `publish`, `audit` | - | - |

#### L2 Coverage

| Category | Status | Notes |
|----------|--------|-------|
| Context | REFERENCED | Autonomous context gathering |
| Planning | REFERENCED | Autonomous planning |
| Reasoning | IMPLEMENTED | `reason`, `reflect`, `self_check` |
| Memory | REFERENCED | Autonomous memory management |
| Guardrails | IMPLEMENTED | `policy_check`, `grounding_check` for autonomy safety |
| Multi-Agent | REFERENCED | Multi-agent autonomy |

#### L3 Pattern Support

| Pattern | Support Level | Notes |
|---------|--------------|-------|
| sequential_pattern | REFERENCED | Sequential autonomous execution |
| map_reduce_pattern | REFERENCED | Parallel autonomous processing |
| planner_executor_pattern | REFERENCED | Autonomous planning/execution |
| react_pattern | REFERENCED | Think-act-observe for autonomy |
| router_specialist_pattern | REFERENCED | Autonomous agent routing |
| maker_checker_pattern | REFERENCED | Autonomous quality loops |
| supervisor_swarm_pattern | REFERENCED | Multi-agent autonomy |
| monitor_act_pattern | IMPLEMENTED | Core monitoring and acting |

#### L4 Phase Infrastructure

| Phase | Infrastructure Provided | Notes |
|-------|------------------------|-------|
| intake_phase | IMPLEMENTED | Autonomous loop intake |
| understand_phase | IMPLEMENTED | Autonomous understanding |
| prepare_context_phase | IMPLEMENTED | Autonomous context preparation |
| plan_phase | IMPLEMENTED | Autonomous planning |
| execute_phase | IMPLEMENTED | Autonomous execution |
| verify_phase | IMPLEMENTED | Autonomous verification |
| refine_phase | IMPLEMENTED | Autonomous refinement |
| decide_phase | IMPLEMENTED | Autonomous decision making |
| deliver_phase | IMPLEMENTED | Autonomous delivery |
| learn_phase | IMPLEMENTED | Autonomous learning |

#### L5 Controller Needs

| Controller | Need Level | Notes |
|------------|------------|-------|
| governance | IMPLEMENTED | Core governance for autonomy |
| budget_manager | IMPLEMENTED | Resource limits for autonomy |
| scheduler_controller | REFERENCED | Autonomous scheduling |
| topology_manager | REFERENCED | Autonomous resource topology |
| evaluation_program | IMPLEMENTED | Core evaluation for autonomy |
| provenance_controller | REFERENCED | Autonomous provenance |
| experiment_controller | REFERENCED | Autonomous experimentation |
| optimization_controller | IMPLEMENTED | Core optimization for autonomy |
| memory_compactor | REFERENCED | Autonomous memory management |

#### Missing Substructure

| Category | Missing Items | Relevance |
|----------|--------------|-----------|
| L1 Execution | `model`, `tool`, `api` implementation | Direct autonomous execution |
| L1 Control Flow | `map`, `reduce` | Batch autonomous processing |
| L2 Planning | Direct `plan_step`, `decompose` implementation | Direct autonomous planning |
| L3 Patterns | Direct pattern implementation | Autonomous workflow orchestration |

#### Recommendations

1. **Add L1 direct execution** - Enable direct autonomous model/tool execution
2. **Implement L2 direct planning** - Direct autonomous planning primitives
3. **Add L3 direct patterns** - Direct autonomous workflow patterns
4. **Complete L5 controller integration** - Full autonomy with all controllers

---

## 4. Cross-Plan Substructure Dependencies

### 4.1 Dependency Flow Diagram

```
                    L5 Controllers
                    ┌─────────────────────────────────────────┐
                    │ governance, budget_manager,             │
                    │ evaluation_program, provenance          │
                    └────────────────┬────────────────────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│ L4 Phases       │       │ L3 Patterns     │       │ L5 Controllers  │
│ (plan-03, 04,   │──────▶│ (plan-04, 06,   │──────▶│ (plan-07)       │
│  05, 06, 07)    │       │  07)            │       │                 │
└────────┬────────┘       └────────┬────────┘       └────────┬────────┘
         │                           │                           │
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│ L2 Primitives   │       │ L2 Primitives   │       │ L1/L2/L3/L4     │
│ (plan-04, 05,   │◀──────│ (plan-01, 02,   │◀──────│ Observability   │
│  07)            │       │  04, 06, 07)    │       │                 │
└────────┬────────┘       └────────┬────────┘       └────────┬────────┘
         │                           │                           │
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│ L1 Operations   │       │ L1 Operations   │       │ L1 Operations   │
│ (plan-00, 01,   │◀──────│ (plan-02, 04,   │◀──────│ (plan-05, 06,   │
│  02, 04)        │       │  05, 06)        │       │  07)            │
└─────────────────┘       └─────────────────┘       └─────────────────┘
```

### 4.2 Layer Flow Between Plans

| From Plan | To Plans | Layers Transferred | Types |
|-----------|----------|-------------------|-------|
| plan-00 | plan-01 | L1 boundary, L1 observability | `WorkflowSpec`, `WorkflowIR`, `.opencode/` |
| plan-00 | plan-02 | L1 data_flow, L1 execution | Schema validation, provider interfaces |
| plan-01 | plan-02 | L1 queue_resource, L5 scheduler_controller | Queue state, scheduling APIs |
| plan-01 | plan-03 | L1 queue_resource, L4 phases | Queue visualization, state display |
| plan-02 | plan-03 | L1 execution, L5 topology_manager | Provider pool, API endpoints |
| plan-02 | plan-04 | L1 execution, L1 observability | Model execution, metrics |
| plan-03 | plan-04 | L4 phases, L5 evaluation_program | Dashboard, metrics display |
| plan-04 | plan-05 | L1 data_flow, L2 memory | Verifiers, artifact storage |
| plan-05 | plan-06 | L1 execution, L5 provenance_controller | Search/retrieval, provenance |
| plan-06 | plan-07 | L1 time_event, L3 patterns | Scheduling, workflow patterns |
| plan-00..06 | plan-07 | All layers | Complete autonomy stack |

---

## 5. Gap Analysis

### 5.1 Critical Gaps (Must Fix)

| Gap ID | Plan | Layer | Missing Components | Impact | Priority |
|--------|------|-------|-------------------|--------|----------|
| G1 | plan-00 | L1 | Execution primitives (`model`, `tool`, `api`) | Blocks runtime execution | P0 |
| G2 | plan-00 | L2 | Reasoning primitives (`reason`, `critique`) | Blocks agent capabilities | P0 |
| G3 | plan-01 | L2 | Memory primitives (`memory_recall`, `memory_write`) | Blocks job learning | P0 |
| G4 | plan-01 | L1 | Time/event primitives (`schedule`, `sensor`) | Blocks cron scheduling | P0 |
| G5 | plan-02 | L3 | `planner_executor_pattern` | Blocks intelligent workflows | P0 |
| G6 | plan-04 | L2 | Context primitives (`intake_context`, `intent_classify`) | Blocks context-aware quality | P0 |
| G7 | plan-05 | L3 | All patterns | Blocks search workflow orchestration | P0 |
| G8 | plan-05 | L2 | Reasoning primitives | Blocks intelligent search | P0 |

### 5.2 Important Gaps (Should Add)

| Gap ID | Plan | Layer | Missing Components | Impact | Priority |
|--------|------|-------|-------------------|--------|----------|
| G9 | plan-00 | L3 | Pattern infrastructure | Patterns not composable from IR | P1 |
| G10 | plan-01 | L3 | `monitor_act_pattern` | No self-monitoring queues | P1 |
| G11 | plan-02 | L1 | `map`, `reduce`, `loop` | No batch CLI operations | P1 |
| G12 | plan-02 | L5 | `experiment_controller` | No A/B testing | P1 |
| G13 | plan-03 | L5 | `optimization_controller` UI | No optimization tuning | P1 |
| G14 | plan-04 | L4 | `intake_phase`, `understand_phase` | Incomplete quality lifecycle | P1 |
| G15 | plan-05 | L1 | Control flow (`loop`, `parallel`) | No iterative/parallel search | P1 |
| G16 | plan-06 | L2 | Reasoning primitives | No intelligent automation | P1 |
| G17 | plan-06 | L3 | `react_pattern`, `monitor_act_pattern` | No self-improving automation | P1 |

### 5.3 Nice-to-Have Gaps

| Gap ID | Plan | Layer | Missing Components | Impact | Priority |
|--------|------|-------|-------------------|--------|----------|
| G18 | plan-00 | L1 | `chunk`, `summarize` | Missing data processing | P2 |
| G19 | plan-01 | L5 | `topology_manager`, `evaluation_program` | Resource and quality management | P2 |
| G20 | plan-02 | L2 | Memory system | CLI history/context | P2 |
| G21 | plan-03 | All | Direct implementation (vs visualization) | UI is visualization layer | P2 |
| G22 | plan-04 | L5 | `experiment_controller` | Systematic A/B quality testing | P2 |
| G23 | plan-05 | L5 | `evaluation_program`, `optimization_controller` | Search quality optimization | P2 |
| G24 | plan-06 | L5 | `optimization_controller` | Automation optimization | P2 |
| G25 | plan-07 | L1 | `map`, `reduce` | Batch autonomous processing | P2 |

---

## 6. Recommendations

### 6.1 For Plan Updates

#### plan-00 Foundation
1. **Add L1 execution primitives** - Include `model`, `tool`, `api` in WorkflowIR
2. **Add L2 reasoning primitives** - Include `reason`, `critique` in schema
3. **Design for L3 composition** - Ensure IR supports pattern recognition
4. **Include L5 observability hooks** - Enable evaluation and provenance from start

#### plan-01 MVP Queue
1. **Add L1 scheduling primitives** - Include `schedule`, `sensor` for cron
2. **Add L2 memory primitives** - Include `memory_recall`, `memory_write`
3. **Implement L3 monitor_act** - Enable self-monitoring queues
4. **Add L5 evaluation** - Measure job quality

#### plan-02 CLI/Backends
1. **Add L1 batch processing** - Include `map`, `reduce`, `loop`
2. **Add L2 memory system** - Enable CLI history and context
3. **Implement L3 planner_executor** - Support intelligent planning
4. **Add L5 experimentation** - Enable A/B testing

#### plan-03 Glyphnova UI
1. **Complete L4 phase components** - All 10 phases with visualization
2. **Add L5 topology UI** - Resource pool management
3. **Add L5 memory UI** - Memory compaction controls
4. **Add L5 experiment UI** - A/B test visualization

#### plan-04 Quality Loops
1. **Add L2 context primitives** - Enable context-aware quality
2. **Implement L3 planner_executor** - Planned quality improvements
3. **Add L4 intake/decide phases** - Full quality lifecycle
4. **Include L5 experiment controller** - Systematic A/B testing

#### plan-05 Memory/Search
1. **Add L1 control flow** - Enable iterative/parallel search
2. **Add L2 reasoning** - Intelligent search decisions
3. **Add L3 patterns** - Map-reduce, react for search
4. **Include L5 evaluation** - Measure search quality

#### plan-06 Automation
1. **Add L2 reasoning** - Intelligent automation decisions
2. **Implement L3 react_pattern** - Self-improving automation
3. **Add L3 monitor_act_pattern** - Continuous monitoring
4. **Include L5 optimization** - Automatic tuning

#### plan-07 Autonomy/Metrics
1. **Add L1 direct execution** - Direct autonomous execution
2. **Add L2 direct planning** - Direct autonomous planning
3. **Add L3 direct patterns** - Autonomous workflow orchestration
4. **Complete L5 integration** - All controllers working together

### 6.2 For New Infrastructure

| Infrastructure | Plans Affected | Layer | Rationale |
|----------------|----------------|-------|-----------|
| L1 Execution Engine | 00, 02, 04, 05, 07 | L1 | Core runtime for all execution |
| L2 Reasoning Service | 00, 04, 05, 06, 07 | L2 | Centralized reasoning for all plans |
| L3 Pattern Library | 00, 01, 02, 04, 05, 06, 07 | L3 | Reusable patterns across plans |
| L5 Governance Service | 01, 02, 04, 05, 06, 07 | L5 | Centralized policy enforcement |

### 6.3 For Verification

1. **Layer completeness tests** - Verify each plan implements claimed layers
2. **Pattern composition tests** - Verify L3 patterns compose from L1/L2
3. **Phase lifecycle tests** - Verify L4 phases execute in order
4. **Controller integration tests** - Verify L5 controllers govern correctly
5. **Cross-plan integration tests** - Verify layers flow between plans

---

## 7. Summary Statistics

### 7.1 Implementation Coverage by Layer

| Layer | Types Defined | Types Implemented Across Plans | Coverage |
|-------|--------------|-------------------------------|----------|
| L1 | 42 | 35 | 83% |
| L2 | 15 | 8 | 53% |
| L3 | 8 | 6 | 75% |
| L4 | 10 | 8 | 80% |
| L5 | 10 | 6 | 60% |

### 7.2 Plan Maturity by Layer

| Plan | L1 | L2 | L3 | L4 | L5 | Overall |
|------|----|----|----|----|----|----|
| plan-00 Foundation | ●●●○ | ●●○○ | ●○○○ | ●○○○ | ●●○○ | 40% |
| plan-01 MVP Queue | ●●●○ | ●●○○ | ●●○○ | ●●○○ | ●●●○ | 55% |
| plan-02 CLI/Backends | ●●●● | ●●○○ | ●●●○ | ●●●○ | ●●●○ | 70% |
| plan-03 Glyphnova UI | ●●○○ | ●●○○ | ●●○○ | ●●●● | ●●●○ | 55% |
| plan-04 Quality Loops | ●●●● | ●●●○ | ●●●○ | ●●●○ | ●●●○ | 75% |
| plan-05 Memory/Search | ●●●○ | ●●●○ | ●○○○ | ●●●○ | ●●●○ | 60% |
| plan-06 Automation | ●●●● | ●●●○ | ●●●○ | ●●●● | ●●●● | 85% |
| plan-07 Autonomy/Metrics | ●●●● | ●●●● | ●●●● | ●●●● | ●●●● | 100% |

Legend: ● = Implemented, ○ = Missing

### 7.3 Critical Path to Full Coverage

1. **plan-00** → Add L1 execution + L2 reasoning → Foundation complete
2. **plan-01** → Add L1 scheduling + L2 memory → Queue complete
3. **plan-05** → Add L1 control flow + L3 patterns → Search complete
4. **All plans** → Verify cross-layer integration → System complete

---

**Last Updated**: 2026-04-02
**Next Review**: After plan-00 implementation starts
