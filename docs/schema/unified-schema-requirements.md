# Unified Schema Simplification — Requirements (v2)

## Overview
This document captures all requirements from the schema simplification planning sessions (Rounds 1-9). These requirements MUST be preserved and satisfied by the unified-workflow-schema.yml v2.0.

---

## 1. Structural Requirements

### 1.1 No `type:` Property on Steps
Steps infer their type from keys present:
- `generative_entity` + `prompt` → agent step
- `tool` key → tool step
- Only `when` → control step
- `sub_workflow` key → sub-workflow step
- `loop` key → loop step

### 1.2 No `pipeline:` Format
All workflows use `agentic_workflow:` with named steps. Pipeline array format removed.

### 1.3 No `output:` Section on Steps
Output achieved through `when` hooks. Steps auto-create variables named after the step with substructure (raw_text, response, metadata). Access via `{{step.step_name.output}}`.

### 1.4 No Standalone `logging:` Section
Logging ONLY through `when` hook `log:` actions. No top-level `logging:` key.

### 1.5 No `enabled:` Keys Anywhere
Presence = enabled. Comment out to disable. Use `disabled: true` for explicit off.

### 1.6 No `features_demonstrated:` Section
Vestigial key for human review of examples. Removed from schema entirely.

### 1.7 No `framework:` Key on Models
Backend implementation detail (agentsdk). Not schema-configurable. Can be swapped at engine level later.

### 1.8 No `models.routing:` Section
Model routing handled by model router project, not execution engine. Schema only needs `default_router: automatic`.

---

## 2. Hook System Requirements

### 2.1 `when` / `lifecycle_hooks` Aliases
Both keys work identically. `when` is preferred for readability.

### 2.2 Hook Timing: before / after / during
- `before_*` = fires BEFORE event, no results available
- `after_*` = fires AFTER event completes, full results available
- `during_*` = fires AS results stream in, partial results available (streaming)

### 2.3 Hook Value: Single Object or Array
A hook value can be a single object (one action) or an array of objects (ordered execution, enables conditional branching).

### 2.4 `then` is Optional in Hooks
Actions can be placed directly on the hook object without wrapping in `then:`.

### 2.5 Logging ONLY Through Hooks
No other logging mechanism exists. All logging uses `log:` action inside `when:` hooks.

### 2.6 Natural Language Hook Names
Hooks read like natural language: `before_step_starts`, `after_step_succeeds`, `after_step_fails`, `during_step_streaming`. No jargon.

### 2.7 Flat Hook Notation Preferred
Both `after_step_succeeds` (flat) and nested `after.step.succeeds` work. Flat is preferred.

### 2.8 Control Flow Steps Use `when.gwt`
Control steps have `when.gwt` as their primary mechanism, not generic `after_step_succeeds`. Step-specific hooks: `before_gwt_evaluates`, `during_gwt_evaluates`, `after_gwt_evaluates`.

---

## 3. GWT (Given/When/ Then) Requirements

### 3.1 GWT at Step Level = LLM/Agent Evaluated
At top-level of `when:`, GWT uses natural language descriptions that the LLM/agent evaluates for routing.

### 3.2 GWT in Hooks = Deterministic
Inside hook objects, GWT uses boolean expressions for deterministic conditional routing.

### 3.3 `gwt` and `given_when_then` are Aliases
`gwt` is the preferred short form. Both parse identically. (NOT `gtw` — that was a spelling error.)

### 3.4 `then.route_to` Enables Branching
Route to single step, array of steps (parallel), or sub-workflow name.

---

## 4. Output & Save Requirements

### 4.1 Default Step Output Variable
Each step auto-creates a variable named `step_name.output` with substructure: `.raw_text`, `.response` (parsed YAML/JSON), `.metadata` (model, duration, tokens).

### 4.2 `save_to` / `append_to` / `set_variable` are Aliases
All three are the same underlying action. They append to variable or file path. New variable = append to empty string.

### 4.3 `save_to` Accepts String or Array
Single string = variable name or file path (detected by `./` or `/` prefix). Array = save to multiple destinations.

### 4.4 Default Format: YAML for Structured, String for Unstructured
No `format:` key needed by default. Optional `format:` for explicit conversion only.

### 4.5 `bookmark` = Alias for `checkpoint`
`checkpoint` is canonical. `bookmark` and `save_state` are aliases. Not a replacement — all three work.

---

## 5. Retry & Loop Requirements

### 5.1 Retry ≠ Loop
- **Retry** = re-attempt SAME step on failure (backoff, conditions, adjustment strategies)
- **Loop** = iterate DIFFERENT executions (list iteration, validation convergence, sub-workflow repetition)

### 5.2 Workflow Retry is Flat with `step:` Subkey
```yaml
retry:
  max_attempts: 10        # workflow-level
  backoff: exponential
  step:                   # defaults for ALL steps
    max_attempts: 3
    backoff: linear       # DEFAULT for steps: linear
```

### 5.3 Default Backoff: Linear for Steps
Step retry defaults to `linear`. Workflow retry defaults to `exponential`.

### 5.4 Five Loop Types Preserved
count, time, validation, retry, infinite — all as named subkeys under `loops:` or `loop:`.

### 5.5 Competing Loop Configs: Min-Constraint Principle
When multiple loop types are configured simultaneously, engine enforces all and stops at whichever constraint triggers first.

---

## 6. Control Flow Requirements

### 6.1 `requires` Accepts Mixed Strings + Objects
- Bare string = "step must not fail" (simple check)
- Object with `condition:` = "step must meet condition" (complex check)
- Array can mix both

### 6.2 `on_requires_failed` is a When Hook
Not a simple string. Supports full hook structure: string value, or object with `if`/`given` and response actions.

### 6.3 Skip Actions
- `skip_step` — skip just this step, continue workflow
- `skip_loop` — skip current loop iteration (not whole loop)
- `skip_sub_workflow` — skip current sub-workflow execution
- `skip_remaining` — skip all remaining steps

### 6.4 `route_to` Stays
No rename. `route_to` is the routing key.

### 6.5 Dependency Relationships
Standard: `depends_on` (finish-to-start, default), `starts_with`, `finishes_with`, `triggers_close`.

---

## 7. Sub-Workflow Requirements

### 7.1 Top-Level Configuration
Like models/agents. String = path, object = inline definition, object with `path:` = path + overrides.

### 7.2 Top-Down Referencing
Sub-workflows defined ABOVE can be referenced by those BELOW.

### 7.3 No `exposes` Key
Sub-workflows output like any other step via `{{step.sub_workflow_step_name.output}}`.

### 7.4 No `forced_values`
Removed for MVP. Model router handles value forcing more securely.

### 7.5 Variable Override Precedence
Sub-workflow explicit > parent passed > sub-workflow defaults > parent defaults.

---

## 8. Removed Features (Model Router Handles)

### 8.1 Adaptive Behavior
ALL adaptive features removed from execution engine. Model router handles: resource monitoring, memory pressure handling, concurrency adjustment, performance-based scheduling.

### 8.2 Model Routing
`models.routing:` removed. Model router project handles dynamic routing, scaling, and model override at runtime.

### 8.3 Advanced Scheduling
Removed. Scheduling expressed through workflow structure (depends_on, parallel_group, route_to).

---

## 9. Input Requirements

> **Note**: Parallel execution features have moved to the [agent-queue](https://github.com/penwoodj/agent-queue) project. References to `depends_on`, `parallel_group`, `route_to` describe historical manual schema capabilities.

### 9.1 Workflow-Level `inputs`
Typed inputs with name, description, type (string default, number, boolean, object, list types), and default values. Replaces `input_variables`.

### 9.2 User Inputs at Step Level
`user_input:` can be placed directly on a step. Not required to go through `inputs_by_step`.

### 9.3 No `input_variables`
Removed. Use `inputs:` at agentic_workflow level.

---

## 10. Semantic Hierarchy

### 10.1 Three Levels
- **L1 (workflow top-level)**: providers, models, sub_workflows, workspace, memory, tool_permissions
- **L2 (agentic_workflow)**: retry.step:, when (default hooks), hardcoded_values, inputs
- **L3 (step-level)**: retry, when, requires, depends_on, parallel_group, prompt, tool

### 10.2 Merge Rules
- Tool permissions: L3 can only RESTRICT L1 baseline
- Retry: L3 overrides L2 completely (not merged)
- Hooks: L3 MERGES with L2 (additive)

### 10.3 `allocation` → `ram_allocation`
Renamed for clarity throughout schema.

---

## 11. Tool Permissions Requirements

### 11.1 Presence = Enabled
No `enabled:` key. Presence of permission key means it's allowed. `disabled: true` to turn off.

### 11.2 AI Operations Absorbed
`ai_operations:` removed. Capabilities absorbed into `tool_permissions` with `rate_limits` (inline or file-referenced via `rate_limits_file:`).

### 11.3 System Operations Robust Subschema
`system_operations` includes: `process_management`, `network_operations`, `file_system_mount`, `environment_variables`, `service_management` — each with detailed config options.

### 11.4 Natural Language Permission Enums
Workspace permissions use natural language values (e.g., `owner_full_group_read_exec_other_read_exec` = 755) alongside numeric.

---

## 12. Memory & Workspace

### 12.1 RAG Under Memory
`rag:` moved from top-level to `memory.rag:`. Presence of `rag:` key = enabled (no `enabled:` key).

### 12.2 Workspace Stays Top-Level
Defines physical environment. Referenced by hooks and steps via `${workspace.*}`.

### 12.3 No .glyphnova References
All `.glyphnova/` references removed. Moved to whitt repository. Schema uses generic workspace paths.

---

## 13. Schema Organization

### 13.1 Order
providers → models → sub_workflows → agentic_workflow (config + steps) → workflow_execution_strategy → tool_permissions → memory → workspace → metadata

### 13.2 All Keys Show Enum Values
Comments list all possible enum values for each key. Light explanations.

### 13.3 Self-Documenting
Property names convey meaning. Minimal external docs needed.
