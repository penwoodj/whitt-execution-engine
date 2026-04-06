# Barebones Approach Assessment Report

**Branch**: initial-creation-barebones
**Date**: 2026-03-26
**Purpose**: Assess which requirements can be met by pure Rust Agent SDK vs requiring transpiler orchestration

---

## Executive Summary

This analysis evaluates whether a "barebones" approach—relying primarily on pure Rust Agent SDK features rather than complex transpiler orchestration—can satisfy the project requirements.

**Finding**: Pure Agent SDK covers ~40% of requirements (core agent capabilities), but ~60% of features (workflow-level orchestration) would be missing or severely limited.

---

## Analysis Methodology

Evaluated requirements from:
- `opencode/docs/reports/requirements/requirements.md` - Original specification
- `opencode/docs/reports/requirements/schema-consolidated-report.md` - Schema consolidation
- Example workflows (53 categorized examples across 19 categories) - Feature demonstrations

Assessed each requirement against AutoAgents SDK capabilities to determine:
1. ✅ **Fully Supported** - Native SDK feature
2. ⚠️ **Partially Supported** - SDK provides base, needs transpiler layer
3. ❌ **High Risk / Missing** - Not in SDK, requires custom implementation

---

## Requirements Coverage Matrix

### ✅ Fully Supported by Pure Rust Agent SDK (~40%)

These features are native to the AutoAgents library and require no transpiler orchestration:

#### Core Agent Capabilities
- **Model Management** - Multi-provider support (LM Studio, Ollama, llama.cpp, OpenAI)
- **Backend Selection** - Vulkan, CUDA, Metal, CPU
- **Parameter Tuning** - Temperature, top-p, max tokens configuration
- **Basic Agent Orchestration** - Agents with tools, sub-agent spawning
- **Tool Calling** - Built-in tools (file ops, web, shell, grep)
- **Retry Logic** - Exponential/linear backoff, escalation strategies
- **Stateful Conversations** - Memory management with sliding windows
- **Basic Permissions** - Tool-level access control
- **Async Execution** - Full Tokio runtime support

#### Execution Primitives
- **Parallel/Serial Execution** - Basic concurrency control
- **Tool Execution** - File operations, web fetch, shell commands
- **Agent Spawning** - Create sub-agents dynamically

---

### ⚠️ Partially Supported / Needs Transpiler Layer (~25%)

These features exist in SDK but require transpiler orchestration to provide the full UX:

#### Workflow DSL and Compilation
- **YAML Schema DSL** - SDK doesn't understand YAML workflows; transpiler converts to SDK API calls
- **WorkflowIR Compilation** - Intermediate representation is transpiler-specific
- **Variable References** - SDK has no concept of `${step.step_1.output}`; transpiler resolves and passes values

#### Advanced Loop Control
- **Loop Variations** (5 types) - SDK supports basic loops, but:
  - Validation-based loops require transpiler to evaluate criteria
  - Convergence loops need delta computation between iterations
  - Time-based loops need timeout management
- **Termination Conditions** - Multiple stop conditions require transpiler orchestration

#### Validation and Quality
- **Validation Loops** - SDK has no concept of "loop until criteria met"; transpiler must implement with SDK primitives
- **Weighted Multi-Criteria** - SDK returns results; transpler applies weights and thresholds

#### Multi-Agent Coordination
- **Multi-Agent Aggregation** - SDK can spawn agents, but:
  - Merge/vote/collect/average strategies are schema-level
  - Result combination logic requires transpiler implementation

#### Logging and Observability
- **Hierarchical Logging** (9 levels) - SDK has logging, but per-scope configuration is transpiler-specific
- **Detailed Metrics** - SDK provides some metrics; comprehensive tracking requires transpiler layer

---

### ❌ High Risk / Missing Without Custom Transpiler (~35%)

These features are **NOT** in Agent SDK and would be missing or severely limited:

#### Event-Driven Workflow Orchestration
- **Event-Based Execution** - Workflow-level event propagation, validation result events
- **Interdependency Events** - Coordinating between agents based on each other's results
- **Bidirectional Event Propagation** - Up and down event flow through hierarchy
- **Event Handlers** - Custom event-driven workflow execution

#### Conditional Logic and Branching
- **Conditional Branching** - Strict yes/no decision routing based on LLM output
- **Branch Enable Conditions** - Multiple decision paths with enable/disable logic
- **Event-Based Routing** - Flow control driven by validation/failure events
- **Fallback Branches** - Alternative execution paths on failure

#### State and Persistence
- **State Management** - Checkpoint save/restore, versioning
- **Delta Updates** - Efficient state change tracking
- **Integrity Validation** - State checksums and validation on save/restore
- **State Snapshots** - Point-in-time state captures
- **Auto-Save Intervals** - Periodic state persistence

#### Workflow Composition and References
- **Nested Workflow References** (5 patterns) - Workflow registry system
- **Circular Reference Detection** - Prevent infinite loops in workflow dependencies
- **Isolation Strategies** - Managing scope across nested workflows
- **Policy Inheritance** - Config passing between workflow levels
- **Workflow Registry** - Central repository of reusable workflows

#### Advanced Validation
- **Interdependent Validation** - Agents validating based on other agents' results
- **Hierarchical Validation Aggregation** - Weighted bottom-up validation
- **Validation Propagation** - Results flowing through agent hierarchy
- **Top-Level Overall Validation** - Final workflow-level validation

#### Developer Experience Features
- **CLI Argument Injection** - Runtime parameter passing for generated code
- **Prompt Template Libraries** - TOML-based template organization with Askama
- **Model Sweep Workflows** - Running same task across multiple models with comparison
- **Refinement Loop Patterns** - Load previous output, generate improvement prompt
- **Timespan Configuration** - Operation-specific timeouts (instant/fast/medium/slow)

#### Metrics and Observability
- **Comprehensive Metrics Collection**:
  - Timing metrics (first token, thinking time, tool execution time)
  - LLM metrics (token I/O, tokens per second, context usage)
  - Execution metrics (line-item logging, parallel tracking)
  - Model settings logging
- **Line-Item Level Logging** - Detailed per-operation tracking
- **Parallel Execution Tracking** - Multi-agent coordination metrics
- **Smart Graceful Degradation** - Resource-aware execution adjustment

#### Safety and Human Interaction
- **Human Gating** - Structured clarification and confirmation
- **Staged Diffs** - Preview changes before applying
- **Diff-First Behavior** - Show what will change before executing
- **Declared Effects** - Predict and display workflow impact

---

## Impact Analysis

### What Works with Pure SDK

You would have a functional agent system with:
- Agents that can call tools
- Multiple model providers
- Basic parallel/serial execution
- File, web, and shell operations
- Simple retry logic
- Memory management

**Use Case**: Single-agent workflows with linear execution steps and basic tool usage.

### What's Missing

You would lose the **declarative workflow system**:
- No YAML DSL for workflows
- No conditional branching or event-driven execution
- No validation loops (iterate until criteria met)
- No state management (checkpoints, versioning, resume)
- No nested workflow composition
- No hierarchical orchestration with interdependent validation
- No developer experience features (CLI args, prompt templates, metrics)

**Result**: You'd have an agent library, not a workflow engine. Users would write imperative Rust code instead of declarative YAML.

---

## Recommendation

### Do NOT use pure SDK approach if you want:

1. **Declarative Workflows** - Users writing YAML instead of Rust code
2. **Event-Driven Execution** - Conditional branching, validation events, interdependency
3. **State Management** - Checkpoints, versioning, resume capability
4. **Composable Workflows** - Nested references, workflow registry, policy inheritance
5. **Developer Experience** - CLI arguments, prompt templates, comprehensive metrics

### Pure SDK approach might work if:

1. **Simple Use Cases Only** - Single-agent workflows with linear steps
2. **Imperative Code OK** - Users comfortable writing Rust instead of YAML
3. **No State Needed** - No checkpoint/resume requirements
4. **Basic Orchestration** - No complex branching or validation loops

---

## Conclusion

The transpiler provides significant value beyond the Agent SDK:

**Transpiler Value Proposition**:
- **Workflow orchestration layer** (~60% of features)
- **Declarative YAML DSL** instead of imperative Rust
- **Event-driven execution** with conditional branching
- **State management** (checkpoints, versioning, persistence)
- **Nested workflow composition** and registry
- **Developer experience** features (CLI args, templates, metrics)

**Agent SDK Value**:
- **Core agent capabilities** (~40% of features)
- **Model provider abstraction**
- **Tool calling framework**
- **Async execution engine**
- **Basic orchestration primitives**

**Recommendation**: Proceed with full transpiler approach. The missing features constitute the system's unique value proposition—a declarative workflow engine that enables users to define complex agentic behaviors without writing code.

---

## Decision Point

**Question**: Should we simplify the transpiler by removing advanced features and relying more on Agent SDK?

**Answer**: No. The advanced features are what differentiate this system. Removing them would leave:
- A basic agent library (many already exist)
- No declarative workflow system
- No state management
- No event-driven orchestration

**Result**: Less competitive, lower value, fails to meet requirements for a "barebones but usable" system.

---

## References

- **Requirements**: `opencode/docs/reports/requirements/requirements.md`
- **Schema Consolidated**: `opencode/docs/reports/requirements/schema-consolidated-report.md`
- **Example Workflows**: `opencode/docs/reports/requirements/example-workflows/`
- **AutoAgents SDK**: https://github.com/liquidos-ai/AutoAgents
