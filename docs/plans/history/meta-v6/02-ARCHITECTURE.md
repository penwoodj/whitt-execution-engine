# META-v6 Architecture

> **Purpose:** Define system architecture, data flow, component interaction, and hook firing points

## System Overview

META-v6 is a meta-workflow generator that chains 5 sub-workflows (SW1-SW5) to transform natural language tasks into executable YAML workflows.

```
User Prompt (tasks.md)
    ↓
SW1: Task Analysis
    ↓ outputs.md
SW2: Output Structure
    ↓ categories.md
SW3: Category Mapping
    ↓ structs.md
SW4: Struct Generation
    ↓ workflow.yml skeleton
SW5: Workflow Assembly
    ↓ workflow.yml (validated)
```

**Technology Stack:**
- Execution Engine: Rust (`target/release/whitt`)
- Backend: llama.cpp with Vulkan (Docker)
- Model: Qwen3-5-9B-Q4_K_M (6.53 GB)
- Context: 262144 tokens
- KV Cache: Q8_0 (~5.43 GB)
- Total RAM: 11.96 GB (fits 15.5 GB)

## Component Architecture

### 1. Rust Execution Engine

**Binary:** `./target/release/whitt`
**Entry Point:** `src/bin/whitt.rs`
**Core Runner:** `src/benchmark/runner.rs`

**Key Responsibilities:**
- Parse workflow YAML files
- Load/unload models via llama.cpp backend
- Execute steps with LLM inference
- Fire hook triggers at execution points
- Handle errors, retries, and control flow
- Stream responses (SSE) or buffered output

**Hook Engine:** `src/workflow/hooks/mod.rs`
- HookEngine manages state and bookmarks
- HookResult merges multiple hook results
- Priority: Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue

### 2. Llama.cpp Backend

**Container:** `whitt-llama-server` (Docker)
**Image:** llama.cpp with Vulkan backend
**Port:** 8080
**Base Config:** `docker/docker-compose.yml` (not AMD/NVIDIA variant)

**Backend Implementation:** `src/backend/llama_vulkan.rs`
- Implements `LLMBackend` trait
- Manages HTTP client to llama.cpp server
- Handles model loading/unloading
- Provides chat completion API

**HTTP Client:** `src/client/http_client.rs`
- SSE streaming support (not used in META-v6)
- Request/response handling
- Error handling and retries

### 3. Hook System

**Context Types:** `src/workflow/hooks/context.rs` (10 variants)
- `BeforeStepStartsContext` - Before step execution
- `DuringStepStreamingContext` - During streaming (DEAD, not wired)
- `AfterStepStartsContext` - After step starts
- `AfterStepSucceedsContext` - After step succeeds
- `AfterStepFailsContext` - After step fails
- `AfterAllRetriesExhaustedContext` - After retry limit
- `BeforeGwtEvaluatesContext` - Before GWT evaluation (PARTIAL)
- `AfterGwtEvaluatesContext` - After GWT evaluation (PARTIAL)
- `OnRequiresFailedContext` - When step dependency fails
- `AfterLoopIterationFailsContext` - When loop iteration fails

**Action Types:** `src/workflow/hooks/actions.rs` (12 variants)
- `Log(LogAction)` - Write to file or stdout
- `AppendTo(AppendToAction)` - Append to file or variable
- `SaveTo(SaveToAction)` - Save to file or variable
- `RouteTo(RouteToAction)` - Route to specific step
- `Bookmark(BookmarkAction)` - Store state in memory/file
- `Notify(NotifyAction)` - Send notification via channel
- `Fail(FailAction)` - Fail workflow with message
- `Shell(ShellAction)` - Execute shell command
- `SkipStep(bool)` - Skip current step
- `SkipRemaining(bool)` - Skip remaining steps
- `Gwt(Vec<GwtClause>)` - Guard When Then conditional
- `IterateValues(HashMap)` - Iterate values (PASSTHROUGH)

**GWT Evaluator:** `src/workflow/hooks/gwt.rs`
- Lexer, parser, evaluator for GWT expressions
- Supports: literals, field paths, comparisons, logical ops, arithmetic
- Known bug: string == comparison may be broken

### 4. Schema Validator

**Schema File:** `docs/schema/unified-workflow-schema.yml` (805 lines)
**Validation Logic:** In runner, validates against schema
**Provider Requirements:**
- Key: `llama_cpp_with_vulkan` (line 28)
- Host.type: `llama_cpp_with_vulkan` (line 71)
- Config wrapper with `host:` and `port:` (lines 29-31)

**Validation Constraints:**
- Only keys defined in schema allowed
- No non-schema extensions
- No redundant config
- Schema version ≥ 2.0.0

## Data Flow

### Phase 1: Task Decomposition (SW1)

**Input:** `tasks.md` (natural language tasks)
**Output:** `outputs.md` (output specifications)
**Process:**
1. Parse tasks.md for task list
2. Decompose into hierarchical steps
3. Assign story points (≤5 per leaf)
4. Define expected outputs per step
5. Write outputs.md

**Data Transformation:**
```markdown
# tasks.md
- Task: Build authentication system
  Subtasks:
    - Design login flow
    - Implement JWT tokens
    - Add rate limiting

# outputs.md
- Step: login_flow_design
  Output: login_flow_diagram.md
- Step: jwt_implementation
  Output: auth/jwt_handler.rs
- Step: rate_limiting
  Output: middleware/rate_limiter.rs
```

### Phase 2: Output Structure (SW2)

**Input:** `outputs.md` (output specifications)
**Output:** `categories.md` (category design)
**Process:**
1. Group outputs by category
2. Define directory structure
3. Identify file types
4. Define naming conventions
5. Write categories.md

**Data Transformation:**
```markdown
# categories.md
- Category: auth
  Directory: src/auth/
  Outputs:
    - jwt_handler.rs
    - login_handler.rs
- Category: middleware
  Directory: src/middleware/
  Outputs:
    - rate_limiter.rs
    - auth_checker.rs
```

### Phase 3: Category Mapping (SW3)

**Input:** `categories.md` (category design)
**Output:** `structs.md` (struct definitions)
**Process:**
1. Map categories to workflow steps
2. Define step dependencies
3. Assign models per step
4. Define templates
5. Write structs.md

**Data Transformation:**
```markdown
# structs.md
- Step: implement_jwt_handler
  Category: auth
  DependsOn: login_flow_design
  Model: Qwen3-5-9B
  Template: "Implement {{category}}/{{output}}"
  Hooks:
    - before_step_starts: log start
    - after_step_succeeds: save output
```

### Phase 4: Struct Generation (SW4)

**Input:** `structs.md` (struct definitions)
**Output:** `workflow.yml` (YAML skeleton)
**Process:**
1. Generate YAML skeleton
2. Add providers, models, steps
3. Insert hook triggers
4. Add GWT expressions
5. Write workflow.yml

**Data Transformation:**
```yaml
# workflow.yml skeleton
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  - name: Qwen3-5-9B
    host:
      type: llama_cpp_with_vulkan
      config:
        model_path: /models/Qwen3-5-9B-Q4_K_M.gguf

steps:
  - name: login_flow_design
    model: Qwen3-5-9B
    hooks:
      before_step_starts:
        - log: "Starting step..."
```

### Phase 5: Workflow Assembly (SW5)

**Input:** `workflow.yml` skeleton
**Output:** `workflow.yml` (validated)
**Process:**
1. Validate against schema
2. Fix any validation errors
3. Add workflow-level hooks
4. Add error handling
5. Finalize workflow.yml

**Validation:**
- Schema valid?
- Provider key correct?
- Host type correct?
- No non-schema keys?
- No redundant config?

## Hook Firing Points

### Runner Firing Logic

**Central Function:** `src/benchmark/runner.rs:813-848`
```rust
fn execute_hooks_for_trigger(
    engine: &mut HookEngine,
    trigger: &str,
    context: WorkflowHookContext,
    hook_config: &Vec<Hook>,
) -> Result<HookResult, Error>
```

### Trigger Locations in Runner

| Trigger | Runner Line | Context Struct | Status |
|---------|-------------|----------------|--------|
| `before_step_starts` | 1257 | `BeforeStepStartsContext` | ✅ WIRED |
| `after_step_starts` | 1286 | `AfterStepStartsContext` | ✅ WIRED |
| `after_step_fails` | 1318 | `AfterStepFailsContext` | ✅ WIRED |
| `after_step_succeeds` | 1340 | `AfterStepSucceedsContext` | ✅ WIRED |
| `after_all_retries_exhausted` | 1355 | `AfterAllRetriesExhaustedContext` | ✅ WIRED |
| `on_requires_failed` | 1535 | `OnRequiresFailedContext` | ✅ WIRED |
| `after_loop_iteration_fails` | 1592 | `AfterLoopIterationFailsContext` | ✅ WIRED |
| `before_gwt_evaluates` | 404 (actions.rs) | `BeforeGwtEvaluatesContext` | ⚠️ PARTIAL (logging only) |
| `after_gwt_evaluates` | 414 (actions.rs) | `AfterGwtEvaluatesContext` | ⚠️ PARTIAL (logging only) |
| `during_step_streaming` | NOT FIRED | `DuringStepStreamingContext` | ❌ DEAD |

### Hook Execution Flow

```
Step Execution
    ↓
before_step_starts fires
    ↓
HookEngine::execute_hooks_for_trigger()
    ↓
For each action in hook_config:
    execute_action() in src/workflow/hooks/actions.rs
    ↓
Execute based on action type:
    - Log → execute_log()
    - SaveTo → execute_save_to()
    - RouteTo → execute_route_to()
    - Gwt → execute_gwt()
    ↓
Return HookResult
    ↓
Merge HookResults (priority: Fail > SkipRemaining > ...)
    ↓
Act on result:
    - Continue → proceed
    - Fail → fail workflow
    - RouteTo → jump to target
    - SkipStep → skip current
    ↓
Step executes (or not)
    ↓
after_step_starts / after_step_succeeds / after_step_fails fires
```

## Template Interpolation

**Syntax:** `{{variable.path}}`
**Supported Paths:**
- `{{step.<step_name>.output}}` - Previous step output
- `{{workflow_variables.<var_name>}}` - Workflow-level variables
- `{{step_name}}` - Current step name
- `{{model_name}}` - Model name

**Example:**
```yaml
steps:
  - name: generate_code
    model: Qwen3-5-9B
    hooks:
      after_step_succeeds:
        - log:
            message: "Generated code for {{step_name}}"
            to_file_path: "logs/{{step_name}}.log"
```

**Implementation:** Runner resolves templates before hook execution.

## Error Handling

### Step-Level Errors

**Retry Logic:**
- Default: 3 retries
- Configurable via `max_retries` in step config
- Hook `after_step_fails` fires on each failure

**Exhaustion:**
- After max retries, `after_all_retries_exhausted` fires
- Workflow continues or fails based on hook action

### Dependency Errors

**Dependency Check:**
- Runner checks `depends_on` before step execution
- If dependency failed, `on_requires_failed` fires
- Workflow continues or fails based on hook action

### Loop Errors

**Iteration Loop:**
- For loops execute steps N times
- If iteration fails, `after_loop_iteration_fails` fires
- Workflow continues or fails based on hook action

## Memory Management

### KV Cache Strategy

**Config:** `config.yml`
```yaml
cache_type_k: "q8_0"
cache_type_v: "q8_0"
n_ctx: 262144
parallel: 1
```

**Memory Calculation:**
- Model: 6.53 GB
- KV cache (Q8_0 @ 262144): ~5.43 GB
- Total: 11.96 GB
- Available: 15.5 GB
- Headroom: 3.54 GB

**Constraints:**
- Must use `--no-cache-prompt` (Vulkan cannot serialize KV cache)
- Must NOT use `--cont-batching` (triggers serialization on slot release)
- Single model loaded at a time

### Bookmarks Storage

**In-Memory:** `HookEngine.bookmarks: HashMap<String, serde_json::Value>`
**Persistent:** Optional file write via `Bookmark::Path` action
**Access:** Bookmarks available in all subsequent hook contexts

## Concurrency Model

**Parallel Execution:**
- `parallel: 1` in config (single pipeline)
- No parallel step execution
- Sequential step processing only

**Locking:**
- HookEngine mutex protected for bookmark access
- HTTP client single-threaded
- Backend state managed internally

## Performance Characteristics

**Load Time:** ~30 seconds (timeout 1800s)
**Inference Time:** Varies by prompt length
**Hook Execution:** <10ms per action
**Validation:** <100ms per YAML

**Bottlenecks:**
- LLM inference (dominant)
- Model loading (one-time cost)
- Network latency to llama.cpp server

**Optimization Targets:**
- Reduce prompt length
- Cache common templates
- Batch hook actions (future)

## Integration Points

### External Systems

**llama.cpp Server:**
- Endpoint: `http://localhost:8080`
- Protocol: HTTP + SSE (unused)
- Model loading: POST `/models/load`
- Inference: POST `/completion`

**File System:**
- Input: `tasks.md` from user
- Intermediates: `outputs.md`, `categories.md`, `structs.md`
- Output: `workflow.yml`
- Logs: `docs/benchmarks/outputs/meta-workflow/`

### Internal Modules

**Config Module:** `src/config/mod.rs`
- Loads `config.yml`
- Validates settings
- Provides KV cache config

**Model Module:** `src/model/mod.rs`
- Model registry
- Resource management
- Interpolation

**Client Module:** `src/client/mod.rs`
- HTTP client
- Model download
- Docker management

## Security Considerations

**Sandboxing:**
- Shell actions run in subprocess (no sandbox)
- No filesystem restrictions
- No network restrictions
- ⚠️ WARNING: Shell commands execute with full permissions

**Input Validation:**
- Schema validation prevents malformed YAMLs
- GWT expressions validated before execution
- Template paths validated

**Known Risks:**
- R1: Shell action injection
- R2: GWT expression injection
- R3: Template path traversal

## Known Limitations

**L1: during_step_streaming Dead**
- Not wired in runner
- Requires SSE streaming path
- Workaround: Use buffered output (stream:false)

**L2: GWT String Comparison Broken**
- String == may not work correctly
- Workaround: Use numeric/boolean comparisons

**L3: Partial GWT Trigger Wiring**
- before/after_gwt_evaluates only log
- Full wire requires hook_config passing
- Workaround: Accept logging-only for now

**L4: Single Model Only**
- No multi-model parallelism
- Workaround: Sequential model loading

**L5: No Hook Testing**
- No automated hook validation
- Workaround: Manual verification

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending