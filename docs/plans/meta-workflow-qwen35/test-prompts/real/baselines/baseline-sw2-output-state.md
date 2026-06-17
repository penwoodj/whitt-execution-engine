# Baseline Output State Specification

**Source task breakdown:** Parallel inference for same-model multi-target route_to (T1-T12)
**Generated:** 2026-06-17
**Purpose:** Define desired system state after each task completes
**Scope:** Code changes in `src/benchmark/runner.rs` (4544 lines)

---

## Output State Matrix

| Task ID | Input State | Expected Output State | Verification Method |
|---------|-------------|----------------------|---------------------|
| T1 | File `src/benchmark/runner.rs` without `JoinSet` import | Import statement `use tokio::task::JoinSet;` present at file top, resolves in compilation | `grep -n "use tokio::task::JoinSet" src/benchmark/runner.rs` + `cargo check` |
| T2 | Second `impl BenchmarkRunner` block around line 999 | New method `send_inference_request()` with signature `(client, model_id, prompt, max_tokens, temperature, top_p, system_prompt, semaphore) -> (String, usize, Duration, Option<String>)` | `cargo check` passes, method callable with correct parameter types |
| T2.1 | `send_inference_request()` method signature | Semaphore permit acquired at method start: `semaphore.acquire().await`, held for inference duration, released on drop | Code inspection shows semaphore guard, no manual release calls |
| T2.2 | `send_inference_request()` with `system_prompt: Option<&str>` | `Vec<ChatMessage>` constructed: system message added first if present, user message always added | Code shows conditional push for system, unconditional push for user |
| T2.3 | `send_inference_request()` with `chat_completion` call | Response extraction: text from `resp.choices.first().message.content`, tokens from `resp.usage.completion_tokens`, duration from `Instant::now().elapsed()`, error returns default tuple | Return tuple structure matches specification, error handling preserves all fields |
| T3 | Multi-target route_to block around lines 1903-1947 | Boolean `all_same_model` computed before target loop, checks all `generative_entity` references map to same model name | Variable declared and populated before iteration, type `bool` |
| T4 | `all_same_model == true` branch with target list | Parallel execution path: model loaded once, all requests spawned via `JoinSet`, `join_all().await` collects all, responses sorted by index | Code shows single load call, `JoinSet::new()`, spawn loop, join_all loop, sort call |
| T4.1 | Model loading in sequential path | Load call moved before target iteration, `model_loaded` flag set once, shared by all targets | `client.load_model()` called once, flag checked in spawn loop |
| T4.2 | Request building with target list | `Vec<JoinHandle>` created in loop, each spawned with `tokio::spawn` wrapping `send_inference_request` | Code shows spawn inside iteration, handle stored or pushed to collection |
| T4.3 | Response collection from `JoinSet` | Results collected from `join_set.join_next().await`, mapped back to original order via index tagging | Loop collects results, final sort_by_key on index field |
| T5 | Collected inference responses in `parallel_results` | Sequential iteration over ordered results: `after_step_succeeds` hooks run per target, `step_outputs[target] = response_text` populated, bookmarks updated | Hook execution calls present, step_outputs insertions preserve order |
| T6 | All hook processing complete for all targets | Single `client.unload_model()` call after target loop, cooldown applied once | One unload call, one `sleep(cooldown_after_unload)` call |
| T7 | `all_same_model == false` branch | Existing sequential code path unchanged: load → infer → unload per target in loop | Control flow matches pre-change implementation (diff shows zero changes to this branch) |
| T8 | Branch decision point with `all_same_model` | `info!` logging added: `[benchmark] parallel inference: N targets, same model` or `[benchmark] sequential: mixed models` visible in log output | Log file contains message with target count and path type |
| T9 | Existing test infrastructure with MockBackend | New integration test added: configures route_to with 3 same-model targets, asserts all responses present, asserts model loaded once, asserts total duration < 3× single call | Test passes with `cargo test integration_test_parallel_same_model`, fails if parallel path broken |
| T10 | Existing test infrastructure | New integration test added: configures route_to with 2 different-model targets, asserts sequential execution preserved | Test passes with `cargo test integration_test_sequential_mixed_models` |
| T11 | Benchmark log entry format | Log entry for parallel path includes fields: `parallel_targets=N`, `wall_time=X.Xs`, `sequential_est=Y.Ys`, `speedup=Z.ZZx` | Log file shows parsed metrics, speedup calculation present |
| T12 | AGENTS.md or benchmark docs | Documentation note added: "Same-model multi-target route_to executes inference concurrently via tokio::JoinSet, model loaded/unloaded once per route_to group" | Markdown file contains descriptive sentence about optimization |

---

## Per-Task Output State Details

### T1: Add tokio::JoinSet import
**Expected Output State:**
- File `src/benchmark/runner.rs` contains import: `use tokio::task::JoinSet;`
- Import located in top-of-file imports section (before first impl block)
- Type `JoinSet` resolvable throughout module
- No naming conflicts or shadowing

**Evaluation Criteria:**
- Import syntax correct: `use tokio::task::JoinSet;`
- File compiles without import errors: `cargo check` passes
- Type `JoinSet` can be instantiated in later code

**Test Criteria:**
- `grep -n "tokio::task::JoinSet" src/benchmark/runner.rs` returns one match
- Line number < 100 (in imports section)
- No `extern crate` or `mod` statements needed

---

### T2: Create send_inference_request helper method
**Expected Output State:**
- New async method `send_inference_request()` added to second `impl BenchmarkRunner` block
- Method signature: `async fn send_inference_request(...) -> (String, usize, std::time::Duration, Option<String>)`
- Parameters: client (LlamaHttpClient), model_id (&str), prompt (&str), max_tokens (usize), temperature (f64), top_p (f64), system_prompt (Option<&str>), semaphore (&Arc<Semaphore>)
- Return tuple contains: response text, completion tokens, duration, error message (if any)

**Evaluation Criteria:**
- Method compiles without type errors
- Return type matches expected tuple structure
- All parameters used in method body
- No unused variables or dead code

**Test Criteria:**
- `cargo check --features client` passes
- Method signature matches specification exactly
- Return tuple has 4 elements with correct types

---

### T2.1: Semaphore permit acquisition
**Expected Output State:**
- Semaphore permit acquired at method start via `semaphore.acquire().await`
- Guard assigned to variable with underscore prefix: `let _permit = ...`
- Permit held for entire inference call duration
- Permit released automatically on drop at method end

**Evaluation Criteria:**
- Semaphore acquisition present in method
- Error handling for semaphore closed (unwrap_or_else)
- No explicit release calls needed
- Panic on semaphore failure with descriptive message

**Test Criteria:**
- Code shows `semaphore.acquire().await` before inference
- Guard variable persists through inference call
- No `permit.forget()` or manual release

---

### T2.2: ChatMessage construction
**Expected Output State:**
- `Vec<ChatMessage>` allocated with capacity 2
- System message added first if `system_prompt` present: `ChatMessage::system(...)`
- User message added unconditionally: `ChatMessage::user(prompt.to_string())`
- Messages vector passed to ChatCompletionRequest

**Evaluation Criteria:**
- Vector capacity sufficient for system + user
- System message conditional on `Option<&str>` presence
- User message always present
- Message order preserved (system first if present)

**Test Criteria:**
- `Vec::with_capacity(2)` present
- Conditional `if let Some(sys) = system_prompt` block
- Unconditional `messages.push(ChatMessage::user(...))`
- Messages vector non-empty when passed to request

---

### T2.3: Response extraction
**Expected Output State:**
- `std::time::Instant` started before `chat_completion` call
- On success: text from `resp.choices.first().message.content`, cleaned via `Self::clean_response_text()`, tokens from `resp.usage.completion_tokens`, duration from `instant.elapsed()`, error = None
- On error: error message string, tokens = 0, duration from instant, error = Some(e.to_string())

**Evaluation Criteria:**
- Instant timing covers only inference (not hook processing)
- Response text cleaned before returning
- Error case returns valid tuple with all fields
- Token count extracted from correct response field

**Test Criteria:**
- `let inf_start = std::time::Instant::now()` present
- `cleaned = Self::clean_response_text(&raw_response)` present
- `tokens = resp.usage.completion_tokens` present
- Error match arm returns 4-tuple

---

### T3: Identify same-model multi-target route_to branch
**Expected Output State:**
- Boolean `all_same_model` declared and initialized to `true` before target loop
- Inside loop: `generative_entity` parsed from each target step
- Model name extracted via `ge.strip_prefix("${models.").and_then(|s| s.strip_suffix('}'))`
- Comparison: if `first_model.as_deref() != Some(model_name)`, set `all_same_model = false`

**Evaluation Criteria:**
- Variable `all_same_model` type `bool`
- `first_model` type `Option<String>` to store first model name
- Parsing logic handles both `${models.key}` and direct key syntax
- Short-circuit: once `all_same_model = false`, no further comparisons needed

**Test Criteria:**
- Variable declared before iteration
- Conditional set to `false` when mismatch detected
- No resetting of `all_same_model` inside loop
- Value accessible after loop for branch decision

---

### T4: Implement parallel inference path
**Expected Output State:**
- `if all_same_model && !target_infos.is_empty() && target_infos.len() > 1` block wraps parallel logic
- Model loaded once: `client.load_model(server_model_id).await` before spawn loop
- All requests built upfront in target iteration
- `JoinSet::new()` created, spawn loop calls `join_set.spawn(async move { ... })`
- `join_set.join_all().await` waits for all completions
- Results collected, sorted by index to restore order

**Evaluation Criteria:**
- Parallel path only triggers for same-model multi-target (N > 1)
- Model lifecycle: load → all inferences → unload (single load/unload)
- All inference calls are truly concurrent (tokio::spawn)
- Results maintain original target order
- Total wall time ≈ single inference time, not N × single

**Test Criteria:**
- `if all_same_model && ...` guard present
- Single `load_model()` call outside spawn loop
- `JoinSet::new()` and `join_set.spawn()` present
- `parallel_results.sort_by_key(|r| r.0)` present
- Single `unload_model()` call after join_all

---

### T4.1: Load model once outside loop
**Expected Output State:**
- `client.load_model(server_model_id).await` called before target iteration
- `already_loaded` check prevents redundant loads
- Model name from `first_model.as_deref().unwrap_or("")`
- Unload of conflicting models before load (if model already loaded)
- Load confirmation logged: `[benchmark] model {} loaded for parallel inference`

**Evaluation Criteria:**
- Load call position: before spawn loop, after `all_same_model` check
- Unload of existing models handled to avoid conflicts
- Flag `model_loaded` not needed (model state managed by client)
- Log message confirms load occurred

**Test Criteria:**
- Load call line number < spawn loop line number
- Unload loop present: `for m in loaded { if m.status.value == "loaded" && m.id != server_model_id { ... } }`
- Info log with model name present
- No nested load calls

---

### T4.2: Build request batch
**Expected Output State:**
- Loop iterates `target_infos` to create inference requests
- Each target's prompt extracted: `prompt.clone().unwrap_or_default()`
- Model overrides applied: `max_tokens`, `temperature` from `target_step.model_overrides` or defaults
- `client_clone = client.clone()` for each spawned task
- `model_id = server_model_id.to_string()`
- `sem = self.inference_semaphore.clone()` passed to inference
- `join_set.spawn()` wraps async call to `send_inference_request`

**Evaluation Criteria:**
- Spawn captures all required context (client, model, prompt, semaphore)
- Each spawn is independent (clone client, move model_id, move prompt)
- Semaphore passed to limit concurrency
- Returns index + target_id + inference result tuple

**Test Criteria:**
- `tokio::spawn(async move { ... })` present
- `client.clone()` inside loop
- Index captured via `.enumerate()`
- Return value includes original index for sorting

---

### T4.3: Collect ordered responses
**Expected Output State:**
- `while let Some(result) = join_set.join_next().await` loop collects completed tasks
- `if let Ok(r) = result` handle successful completion
- Results pushed to `parallel_results: Vec<...>`
- After loop: `parallel_results.sort_by_key(|r| r.0)` restores original order
- Sorted results ready for sequential hook processing

**Evaluation Criteria:**
- JoinSet drained completely (all tasks awaited)
- Error handling for join failure (though spawn should never panic)
- Sort by index restores deterministic order
- Vector length matches original target count

**Test Criteria:**
- `join_set.join_next().await` present in loop
- `sort_by_key(|r| r.0)` present after loop
- No manual index reconstruction
- `parallel_results.len() == target_infos.len()`

---

### T5: Sequential hook processing after parallel inference
**Expected Output State:**
- Loop iterates `parallel_results` in sorted order
- For each result: retrieve original step via `target_infos[*idx].1`
- Run `after_step_succeeds` hooks for each target (sequential, as hooks require &mut self)
- Insert into `step_outputs`: `step_outputs.insert(step.step_id.clone(), format!("...", tokens, duration))`
- Update bookmarks if applicable
- Log completion: `info!("[benchmark] parallel target {} completed: {} tokens in {:?}", ...)`

**Evaluation Criteria:**
- Hook execution preserves sequential semantics (no parallel hooks)
- Step_outputs populated with actual inference results
- Errors in hooks don't crash (handled gracefully)
- Token count and duration from parallel inference preserved

**Test Criteria:**
- Hook execution calls present in loop
- `step_outputs.insert()` present
- Log message with tokens and duration present
- No tokio::spawn for hooks

---

### T6: Unload model once after all targets
**Expected Output State:**
- After target processing loop: single `client.unload_model(server_model_id).await`
- Cooldown applied: `sleep(self.config.cooldown_after_unload).await`
- Log message: `[benchmark] [{}] cooldown after parallel unload: sleeping {}s`
- Model unloaded only once per route_to group

**Evaluation Criteria:**
- Unload call occurs after all hooks complete
- Cooldown matches sequential path behavior
- Log message confirms unload and cooldown duration
- No nested unload calls

**Test Criteria:**
- `unload_model()` line number > hook loop line number
- `sleep(cooldown_after_unload)` present immediately after
- Only one unload call in parallel path
- Log message includes model name and cooldown seconds

---

### T7: Preserve sequential fallback for mixed-model targets
**Expected Output State:**
- `else` branch (when `!all_same_model` or `target_infos.len() <= 1`) falls through to existing sequential code
- Sequential path unchanged: load → infer → unload per target in loop
- Target iteration: `for target_id in targets` with load + infer + unload inside
- No parallel optimization applied
- Behavior identical to pre-change implementation

**Evaluation Criteria:**
- `else` branch contains existing sequential code
- No JoinSet or parallel spawns in this branch
- Model lifecycle per target (not per group)
- Hook processing unchanged

**Test Criteria:**
- Diff between pre-change and post-change shows zero changes to sequential branch
- `for target_id in targets` loop present in else block
- Load + infer + unload sequence preserved
- Single-target route_to still works

---

### T8: Add logging for parallel vs sequential path
**Expected Output State:**
- At branch decision point: log path selection
- Parallel path: `info!("[benchmark] parallel execution: {} targets with same model {}", target_infos.len(), model_name)`
- Sequential path: `info!("[benchmark] sequential: mixed models")` or similar
- Logs include target count and model name (for parallel) or path type (for sequential)
- Log visible in benchmark output file

**Evaluation Criteria:**
- Log messages present in both branches
- Log level `info!` (not `debug!` or `trace!`)
- Log content includes diagnostic info (target count, model name)
- Logs appear at correct point in execution flow

**Test Criteria:**
- `info!("[benchmark] parallel execution: ...")` present
- `info!("[benchmark] sequential: ...")` present
- Log file contains one of these messages per route_to
- Log format consistent with existing benchmark logs

---

### T9: Add integration test for same-model parallel path
**Expected Output State:**
- New test function `test_parallel_same_model_route_to()` in tests/ directory
- Test setup: MockBackend, workflow with route_to to 3 same-model targets
- Test execution: run workflow, verify all 3 responses present in step_outputs
- Test assertion: mock model loaded exactly once (counter check)
- Test assertion: total duration < 3 × single_call_duration (or similar speedup metric)
- Test passes when parallel path works, fails if parallel path broken

**Evaluation Criteria:**
- Test covers parallel path execution
- Assertions verify correctness (all responses present)
- Assertions verify performance (speedup achieved)
- Test isolates parallel path (no mixed models)
- Test name descriptive: includes "parallel" and "same_model"

**Test Criteria:**
- `cargo test test_parallel_same_model` passes
- Test function signature: `#[test] fn test_parallel_same_model_route_to() { ... }`
- MockBackend configured to track load calls
- Duration assertion present (speedup check)

---

### T10: Add integration test for mixed-model sequential fallback
**Expected Output Expected Output State:**
- New test function `test_sequential_mixed_models_route_to()` in tests/ directory
- Test setup: MockBackend, workflow with route_to to 2 different-model targets
- Test execution: run workflow, verify sequential execution preserved
- Test assertion: model loaded/unloaded per target (not once per group)
- Test assertion: responses present in step_outputs
- Test passes when sequential path unchanged, fails if regression introduced

**Evaluation Criteria:**
- Test covers mixed-model scenario
- Assertions verify sequential behavior (model load/unload per target)
- Assertions verify correctness (all responses present)
- Test prevents regression on sequential path
- Test name descriptive: includes "sequential" and "mixed_models"

**Test Criteria:**
- `cargo test test_sequential_mixed_models` passes
- Test function signature: `#[test] fn test_sequential_mixed_models_route_to() { ... }`
- MockBackend counters show 2 loads, 2 unloads (one per target)
- Responses present for both targets

---

### T11: Update benchmark log format for parallel metrics
**Expected Output State:**
- Benchmark log entry includes new fields: `parallel_targets=N`, `wall_time=X.Xs`, `sequential_est=Y.Ys`, `speedup=Z.ZZx`
- Parallel_targets: number of targets in route_to
- Wall_time: actual elapsed time for parallel inference
- Sequential_est: estimated time if sequential (N × average_single_time)
- Speedup: sequential_est / wall_time
- Log entry appears after parallel inference completes
- Fields parseable from log file

**Evaluation Criteria:**
- Log format includes all 4 fields
- Fields appear in consistent order
- Speedup calculation: sequential_est / wall_time
- Fields present only for parallel path (not sequential)

**Test Criteria:**
- Log file contains `parallel_targets=` after parallel route_to
- Log file contains `speedup=` > 1.0 for parallel path
- Sequential path logs do NOT contain these fields
- Fields extractable via grep/awk

---

### T12: Documentation update
**Expected Output State:**
- AGENTS.md or benchmark docs updated with parallel inference note
- Note describes optimization: same-model multi-target route_to runs inference concurrently
- Note mentions tokio::JoinSet, single load/unload, sequential hooks
- Note includes technical details (semaphore limits, performance characteristics)
- Note accessible to developers working on benchmark system

**Evaluation Criteria:**
- Documentation file updated (diff shows addition)
- Note content accurate (matches implementation)
- Note placed in appropriate section (benchmark or performance)
- Note includes technical depth (not just "it's faster")

**Test Criteria:**
- `grep -n "parallel inference" docs/contributing/AGENTS.md` returns match
- Diff shows new lines added (not just whitespace)
- Note mentions tokio::JoinSet and concurrency
- Note appears in benchmark or performance section

---

## Summary

**Total tasks defined:** 12 (T1-T12) with 6 subtasks (T2.1, T2.2, T2.3, T4.1, T4.2, T4.3)

**Output state completeness:**
- ✅ Every task has input state
- ✅ Every task has expected output state
- ✅ Every task has verification method
- ✅ Evaluation criteria defined for all tasks
- ✅ Test criteria defined for all tasks

**Verification coverage:**
- Code inspection (grep, code review)
- Compilation checks (cargo check)
- Test execution (cargo test)
- Runtime verification (log inspection)
- Performance validation (speedup measurement)

**Quality bar:** This baseline represents minimum viable output for SW2. Live SW2 must produce more detailed states with more precise verification methods and richer evaluation criteria.