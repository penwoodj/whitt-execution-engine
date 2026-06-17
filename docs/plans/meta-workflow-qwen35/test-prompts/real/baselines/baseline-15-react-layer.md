# Task Breakdown — Baseline

**Source prompt:** TASK: Implement the Agent ReAct Layer for a Rust workflow engine. Create 7 files. Rust project (whitt-execution-engine) using serde, tokio, async-trait, thiserror, anyhow, tracing...

**Source:** Manual breakdown by opencode (Sisyphus) — serves as quality baseline for SW1 iteration.

**Total tasks:** 13
**Total story points:** 34
**Maximum leaf complexity:** 3 pts

---

## GROUP A: T1-T5 (Foundation Layer)

### T1 - Create module structure (1pts)
**Story points:** 1
**Why:** Trivial — just module declarations and re-exports, mechanical work.
**Action:** Create `src/agent/mod.rs` with pub mod declarations and re-exports for tools, react, executor, streaming, persistence, sandbox.
**Depends on:** None (first task)
**GWT criteria:**
- **Given:** Rust project exists with `src/lib.rs`
- **When:** `src/agent/mod.rs` is created with 6 module declarations
- **Then:** `cargo check` compiles (with warnings for missing modules)

### T2 - Define tool types and trait (2pts)
**Story points:** 2
**Why:** Small — defines ToolCall, ToolResult structs + Tool trait. Straightforward serde + async_trait.
**Action:** Create `src/agent/tools.rs` with ToolCall, ToolResult structs and Tool trait definition.
**Depends on:** T1
**GWT criteria:**
- **Given:** `src/agent/mod.rs` declares `pub mod tools`
- **When:** tools.rs defines ToolCall (name+arguments), ToolResult (tool_name+output+success+metadata), Tool trait (name, description, parameters_schema, execute)
- **Then:** `cargo check` compiles, trait is importable from `crate::agent::tools`

### T3 - Implement ToolRegistry (3pts)
**Story points:** 3
**Why:** Medium — needs HashMap storage, registration, lookup, execution dispatch. Multiple methods to implement.
**Action:** Implement ToolRegistry struct with new(), register(), get(), list(), execute() methods.
**Depends on:** T2
**GWT criteria:**
- **Given:** Tool trait + ToolCall/ToolResult types defined
- **When:** ToolRegistry is implemented with HashMap<String, Box<dyn Tool>> storage
- **Then:** Can register a tool, look it up by name, execute it with ToolCall, returns ToolResult

#### T3.1 - Implement ToolRegistry::new and register (1pts)
**Action:** new() creates empty HashMap, register() inserts Box<dyn Tool> keyed by name.
**GWT:**
- **Given:** ToolRegistry struct defined
- **When:** new() and register() methods are implemented
- **Then:** Can create empty registry and add tools to it

#### T3.2 - Implement ToolRegistry::get and list (1pts)
**Action:** get() returns Option<&dyn Tool> by name lookup, list() returns Vec of (name, description) pairs.
**GWT:**
- **Given:** Registry has tools registered
- **When:** get() and list() methods are called
- **Then:** get returns correct tool or None, list returns all registered tool names+descriptions

#### T3.3 - Implement ToolRegistry::execute (1pts)
**Action:** execute() looks up tool by ToolCall.name, calls tool.execute(arguments), returns Result.
**GWT:**
- **Given:** A registered tool and a ToolCall with matching name
- **When:** execute() is called with the ToolCall
- **Then:** Returns Ok(ToolResult) on success, Err if tool not found or execution fails

### T4 - Implement 6 concrete tools (5pts)
**Story points:** 5 (REQUIRES breakdown — 6 independent tools, each needs own logic)
**Why:** High complexity — 6 tools each with unique logic: ModelList, ModelLoad, ModelUnload, Chat, FileRead, FinalAnswer. Each needs Arc<Mutex> state management.
**Action:** Create 6 tool structs implementing Tool trait, each with specific args schema and execute logic.
**Depends on:** T3

#### T4.1 - Implement ModelListTool (1pts)
**Action:** Holds Arc to ModelRegistry, execute() queries registry for all models with lifecycle state, returns JSON array.
**GWT:**
- **Given:** ModelListTool created with Arc<RwLock<ModelRegistry>>
- **When:** execute() is called with no arguments
- **Then:** Returns ToolResult with JSON array of model names + states (Active/Unloaded/Loading)

#### T4.2 - Implement ModelLoadTool + ModelUnloadTool (2pts)
**Action:** Load tool calls backend.load_model(), updates registry state to Active. Unload calls backend.unload_model(), updates state to Unloaded.
**GWT:**
- **Given:** Both tools hold Arc to backend + registry
- **When:** execute() called with model_name argument
- **Then:** Load tool loads model + sets Active, Unload tool unloads + sets Unloaded, both return ToolResult with success/failure

#### T4.3 - Implement ChatTool (1pts)
**Action:** Holds Arc to backend, execute() calls backend.chat(model_name, message, temperature), returns response content.
**GWT:**
- **Given:** ChatTool created with Arc<dyn LlmBackend>
- **When:** execute() called with model_name, message, optional temperature
- **Then:** Returns ToolResult with chat response content, or error if model not loaded

#### T4.4 - Implement FileReadTool + FinalAnswerTool (1pts)
**Action:** FileRead reads file with allowed_paths enforcement. FinalAnswer stores answer + sets completion flag.
**GWT:**
- **Given:** FileReadTool with allowed_paths config, FinalAnswerTool with completion flag
- **When:** FileRead.execute(path) and FinalAnswer.execute(answer) are called
- **Then:** FileRead returns file content or access-denied error, FinalAnswer returns answer and signals loop exit

### T5 - Implement ReactAgent core loop (5pts)
**Story points:** 5 (REQUIRES breakdown — complex async loop with multiple branches)
**Why:** High complexity — async ReAct loop with LLM calls, tool call parsing, execution, iteration tracking, max_iterations guard. Multiple failure modes.
**Action:** Create ReactAgent struct with run() method implementing the Reason-Act loop.
**Depends on:** T4

#### T5.1 - Implement ReactAgent struct + constructor (1pts)
**Action:** Define ReactAgent with backend, tools, model, max_iterations, system_prompt. Implement new(), with_system_prompt(), with_max_iterations().
**GWT:**
- **Given:** Backend, ToolRegistry, model name available
- **When:** ReactAgent::new() called with these dependencies
- **Then:** Returns ReactAgent with default max_iterations=10, empty system_prompt

#### T5.2 - Implement run() message building + LLM call (2pts)
**Action:** run() builds message list (system + tool definitions + user), calls backend.chat(), tracks iterations and tokens.
**GWT:**
- **Given:** ReactAgent configured with backend, tools, model
- **When:** run(user_message) is called
- **Then:** Sends system prompt + tool schemas + user message to LLM, receives response

#### T5.3 - Implement tool call parsing + execution + loop control (2pts)
**Action:** Parse LLM response for JSON tool calls, execute via registry, append results to messages, check for FinalAnswer or max_iterations.
**GWT:**
- **Given:** LLM response potentially contains tool call JSON
- **When:** Response is parsed and tool is executed
- **Then:** Tool result appended to messages, loop continues unless FinalAnswer called or max_iterations reached

### GROUP A Complexity
- Tasks processed: 5
- Total subtasks generated: 10
- Sum of subtask story points: 16
- Group complexity rating: HIGH
  - 10 subtasks total, multiple 2pt leaves
- Any task >= 5pts NOT expanded? NO

---

## GROUP B: T6-T10 (Execution + Streaming Layer)

### T6 - Implement StepExecutor with retry (3pts)
**Story points:** 3
**Why:** Medium — retry logic with 3 backoff strategies (exponential, linear, fixed) + duration parsing.
**Action:** Create StepExecutor with RetryConfig, implement execute_step() with backoff + jitter.
**Depends on:** T1

#### T6.1 - Define BackoffStrategy + RetryConfig (1pts)
**Action:** Define enum BackoffStrategy (Exponential, Linear, Fixed), struct RetryConfig (max_retries, backoff, initial_delay, max_delay).
**GWT:**
- **Given:** executor.rs file created
- **When:** BackoffStrategy enum + RetryConfig struct defined
- **Then:** Types are importable, can construct RetryConfig with each strategy variant

#### T6.2 - Implement execute_step with retry loop (2pts)
**Action:** execute_step tries step_fn, on failure calculates delay per strategy, sleeps, retries up to max_retries.
**GWT:**
- **Given:** RetryConfig with max_retries=3, Exponential backoff
- **When:** execute_step called with a failing step_fn
- **Then:** Retries 3 times with increasing delays, logs each attempt, returns Err after max retries

### T7 - Implement parse_backoff_strategy + parse_duration (2pts)
**Story points:** 2
**Why:** Small — utility functions for parsing config strings into enums/durations.
**Action:** Implement parse_backoff_strategy(strategy: &str) and parse_duration("1s", "500ms", "2m").
**Depends on:** T6
**GWT criteria:**
- **Given:** BackoffStrategy enum + Duration type available
- **When:** parse_backoff_strategy("exponential", Some(2.0)) and parse_duration("500ms") called
- **Then:** Returns BackoffStrategy::Exponential{multiplier:2.0} and Duration::from_millis(500)

### T8 - Implement SSE streaming types (2pts)
**Story points:** 2
**Why:** Small — struct definitions + enum for stream events, no complex logic.
**Action:** Create StreamingResponse struct, StreamEvent enum (Content, ToolCall, Done, Error), SSEStream type alias.
**Depends on:** T1
**GWT criteria:**
- **Given:** streaming.rs file created
- **When:** StreamingResponse + StreamEvent + SSEStream types defined
- **Then:** Types compile, SSEStream is Pin<Box<dyn Stream<Item=Result<StreamEvent>>>>

### T9 - Implement streaming response parser (3pts)
**Story points:** 3
**Why:** Medium — parse SSE byte stream into StreamEvent variants, handle partial chunks, content accumulation.
**Action:** Implement function to convert reqwest::Response bytes → SSEStream of StreamEvent.
**Depends on:** T8

#### T9.1 - Implement SSE line parser (1pts)
**Action:** Parse "data: {...}" SSE lines from byte chunks, handle multi-line events.
**GWT:**
- **Given:** Raw byte chunks from HTTP response
- **When:** SSE line parser processes chunks
- **Then:** Extracts "data:" prefixed lines, handles \n\n delimiters

#### T9.2 - Implement StreamEvent conversion + content accumulation (2pts)
**Action:** Convert parsed JSON lines to StreamEvent variants, accumulate content into StreamingResponse.
**GWT:**
- **Given:** Parsed SSE data lines with JSON payloads
- **When:** JSON is deserialized and mapped to StreamEvent
- **Then:** Content chunks accumulate, tool calls emit ToolCall events, [DONE] emits Done event

### T10 - Implement streaming integration with ReactAgent (2pts)
**Story points:** 2
**Why:** Small — wire streaming response into ReactAgent for real-time output during tool execution.
**Action:** Add streaming support to ReactAgent: option to use backend.chat_stream() instead of chat().
**Depends on:** T5, T9
**GWT criteria:**
- **Given:** ReactAgent + SSEStream types defined
- **When:** ReactAgent.run_stream() is called
- **Then:** Yields StreamEvent items as they arrive, still executes tools on completion

### GROUP B Complexity
- Tasks processed: 5
- Total subtasks generated: 5
- Sum of subtask story points: 12
- Group complexity rating: MEDIUM
  - 5 subtasks total, some 2pt leaves
- Any task >= 5pts NOT expanded? NO (no tasks ≥5 in this group)

---

## GROUP C: T11-T13 (Persistence + Sandbox Layer)

### T11 - Implement WorkflowPersistence (3pts)
**Story points:** 3
**Why:** Medium — checkpoint serialization, state save/load, multiple serialization formats.
**Action:** Create persistence.rs with WorkflowPersistence struct for saving/loading workflow state.
**Depends on:** T1

#### T11.1 - Define checkpoint types + serialization (1pts)
**Action:** Define Checkpoint struct (step_name, state, timestamp, intermediate_results), derive Serialize/Deserialize.
**GWT:**
- **Given:** persistence.rs file created
- **When:** Checkpoint struct defined with serde derives
- **Then:** Can serialize checkpoint to JSON, deserialize back

#### T11.2 - Implement save_checkpoint + load_checkpoint (2pts)
**Action:** save_checkpoint writes to file path, load_checkpoint reads + deserializes. Handle missing files gracefully.
**GWT:**
- **Given:** Checkpoint type defined
- **When:** save_checkpoint(path, checkpoint) then load_checkpoint(path)
- **Then:** Saves JSON to file, loads it back identically, returns None if file missing

### T12 - Implement ToolSandbox security layer (3pts)
**Story points:** 3
**Why:** Medium — path validation, permission checks, resource limits. Security-critical.
**Action:** Create sandbox.rs with ToolSandbox enforcing allowed_paths, blocked commands, resource limits.
**Depends on:** T4

#### T12.1 - Implement path validation (2pts)
**Action:** Validate file paths against allowed_paths whitelist, reject path traversal (../), enforce workspace boundary.
**GWT:**
- **Given:** ToolSandbox with allowed_paths=["/workspace/src/**"]
- **When:** validate_path("/workspace/src/main.rs") and validate_path("/etc/passwd") called
- **Then:** First returns Ok, second returns Err("Path outside allowed paths")

#### T12.2 - Implement command + resource restrictions (1pts)
**Action:** Block dangerous commands (rm -rf, sudo), enforce max file size, max execution time.
**GWT:**
- **Given:** ToolSandbox with blocked_commands + resource_limits
- **When:** Command validation + resource checks are applied
- **Then:** Dangerous commands rejected, oversized files rejected, timeouts enforced

### T13 - Write integration tests + documentation (3pts)
**Story points:** 3
**Why:** Medium — integration tests for full ReAct loop, tool execution, retry, streaming. Multiple test scenarios.
**Action:** Write integration tests covering: tool registration, ReAct loop with mock backend, retry with backoff, streaming parse, persistence round-trip.
**Depends on:** T5, T6, T9, T11

#### T13.1 - Write tool + registry integration tests (1pts)
**Action:** Test ToolRegistry register/get/list/execute, test each of 6 tools with mock dependencies.
**GWT:**
- **Given:** ToolRegistry + 6 tools implemented
- **When:** Integration tests register tools and execute them
- **Then:** All tools return expected ToolResult, registry lookup works correctly

#### T13.2 - Write ReactAgent loop integration test (1pts)
**Action:** Test full ReAct loop with mock backend returning scripted responses, verify tool calls + final answer.
**GWT:**
- **Given:** Mock backend returning tool-call then final-answer responses
- **When:** ReactAgent.run() executes
- **Then:** Tool is called, result appended, final answer returned, iterations tracked

#### T13.3 - Write retry + streaming + persistence tests (1pts)
**Action:** Test StepExecutor retry with each backoff strategy, test SSE parsing, test checkpoint save/load round-trip.
**GWT:**
- **Given:** StepExecutor, streaming parser, persistence module
- **When:** Tests exercise retry failures, SSE parsing, checkpoint save+load
- **Then:** Retries follow backoff strategy, SSE events parsed correctly, checkpoints round-trip successfully

### GROUP C Complexity
- Tasks processed: 3
- Total subtasks generated: 7
- Sum of subtask story points: 9
- Group complexity rating: MEDIUM
  - 7 subtasks total, mostly 1pt leaves
- Any task >= 5pts NOT expanded? NO

---

## Complexity Distribution

| Points | Count |
|--------|-------|
| 1 | 4 |
| 2 | 5 |
| 3 | 4 |
| 5 | 0 |
| 8 | 0 |
| 13 | 0 |

**Total parent tasks:** 13
**Total leaf subtasks:** 22
**Total story points:** 37 (sum of leaves)
**Max leaf complexity:** 2 pts
**Groups processed:** 3 (A: 5 tasks, B: 5 tasks, C: 3 tasks)
