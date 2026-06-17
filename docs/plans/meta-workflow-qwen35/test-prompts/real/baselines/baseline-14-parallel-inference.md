# Baseline Task Breakdown: Parallel Inference for Same-Model Multi-Target RouteTo

**Source Prompt:** `prompt-14-task-add-true-parallel-inference-for-same-model-multi-target.md`
**Created By:** OpenCode (single-shot baseline)
**Purpose:** Quality bar for SW1 task-deconstruction sub-workflow to surpass.

---

## Task Breakdown

### T1: Add tokio::JoinSet import [STORY: 1]
- **Given** runner.rs imports section at top of file
- **When** adding `use tokio::task::JoinSet;` to imports
- **Then** import compiles without conflict and JoinSet type is available throughout module

### T2: Create send_inference_request helper method [STORY: 3]
- **Given** second `impl BenchmarkRunner` block around line 999
- **When** adding `send_inference_request()` async fn that:
  - Takes client, model_id, prompt, max_tokens, temperature, top_p, system_prompt, semaphore
  - Acquires semaphore permit
  - Builds ChatMessage vector (system optional + user)
  - Calls `client.chat_completion(request).await`
  - Extracts response text, completion tokens, duration
  - Returns tuple `(String, usize, Duration, Option<String>)`
- **Then** method compiles, type-checks, and signature matches all call sites

#### T2.1: Semaphore permit acquisition [STORY: 1]
- **Given** `semaphore: &Arc<Semaphore>` parameter
- **When** calling `semaphore.acquire().await`
- **Then** permit held for duration of inference call, released on drop

#### T2.2: ChatMessage construction [STORY: 1]
- **Given** system_prompt is `Option<&str>`
- **When** building messages vector
- **Then** system message added first if present, user message always added

#### T2.3: Response extraction [STORY: 1]
- **Given** chat_completion returns `ChatCompletionResponse`
- **When** extracting text, tokens, duration
- **Then** all three fields correctly parsed, error case returns default tuple

### T3: Identify same-model multi-target route_to branch [STORY: 2]
- **Given** multi-target route_to handling around lines 1903-1947
- **When** adding check: do all targets reference same `generative_entity`?
- **Then** boolean `all_same_model` computed before target loop

### T4: Implement parallel inference path [STORY: 5]
- **Given** `all_same_model == true` branch
- **When** executing:
  - Load model ONCE outside target loop
  - Build all chat_completion requests upfront
  - Push each to `JoinSet::new()`
  - `join_set.join_all().await` to wait for all
  - Collect responses in order
- **Then** all inference calls execute concurrently, total wall time ~= single call (not N × single)

#### T4.1: Load model once outside loop [STORY: 1]
- **Given** model loading is expensive
- **When** moving load call before target iteration
- **Then** model loaded single time, `model_loaded` flag true for all targets

#### T4.2: Build request batch [STORY: 2]
- **Given** list of targets with prompts
- **When** iterating targets to create `Vec<JoinHandle>`
- **Then** each handle spawned with `tokio::spawn` wrapping `send_inference_request`

#### T4.3: Collect ordered responses [STORY: 2]
- **Given** JoinSet completes in arbitrary order
- **When** collecting results
- **Then** responses mapped back to original target order via index tagging

### T5: Sequential hook processing after parallel inference [STORY: 3]
- **Given** all inference responses collected
- **When** iterating responses in original target order:
  - Run `after_step_succeeds` hooks per target
  - Store `step_outputs[target] = response_text`
  - Update bookmarks
- **Then** hooks execute sequentially (no HookEngine refactor needed), state consistent

### T6: Unload model once after all targets [STORY: 1]
- **Given** all targets complete hook processing
- **When** calling model unload
- **Then** model unloaded single time, cooldown applies once

### T7: Preserve sequential fallback for mixed-model targets [STORY: 2]
- **Given** `all_same_model == false`
- **When** falling through to existing sequential code path
- **Then** behavior identical to pre-change (load → infer → unload per target)

### T8: Add logging for parallel vs sequential path [STORY: 1]
- **Given** branch decision point
- **When** logging `[benchmark] parallel inference: N targets, same model` or `[benchmark] sequential: mixed models`
- **Then** log output visible in benchmark log for debugging

### T9: Add integration test for same-model parallel path [STORY: 3]
- **Given** existing test infrastructure with MockBackend
- **When** creating test that:
  - Configures route_to with 3 targets, same model
  - Runs benchmark
  - Asserts all 3 responses present in step_outputs
  - Asserts model loaded exactly once (mock counter)
  - Asserts total duration < 3 × single_call_duration
- **Then** test passes and would fail if parallel path broken

### T10: Add integration test for mixed-model sequential fallback [STORY: 2]
- **Given** existing test infrastructure
- **When** creating test that:
  - Configures route_to with 2 targets, different models
  - Runs benchmark
  - Asserts sequential execution preserved
- **Then** test passes confirming no regression on mixed-model path

### T11: Update benchmark log format for parallel metrics [STORY: 2]
- **Given** benchmark log currently shows per-step metrics
- **When** adding `parallel_targets` and `parallel_speedup` fields to log
- **Then** log entry for parallel path includes: `parallel_targets=3, wall_time=2.1s, sequential_est=6.0s, speedup=2.86x`

### T12: Documentation update [STORY: 1]
- **Given** AGENTS.md or benchmark docs
- **When** adding note about parallel inference optimization
- **Then** behavior documented: same-model multi-target route_to now runs inference concurrently

---

## Group Breakdown (Groups of 5)

### GROUP A: Core Parallel Path (T1-T5) — HIGH Complexity
| Task | Points | Subtasks |
|------|--------|----------|
| T1 | 1 | 0 (leaf) |
| T2 | 3 | 3 (T2.1, T2.2, T2.3) |
| T3 | 2 | 0 (leaf) |
| T4 | 5 | 3 (T4.1, T4.2, T4.3) |
| T5 | 3 | 0 (leaf) |

**Group Summary:** 5 tasks, 14 points, 6 subtasks. **HIGH complexity.**
Core implementation of the parallel inference feature. T4 is the centerpiece (5 points, 3 subtasks).

### GROUP B: Fallback + Observability (T6-T10) — MEDIUM Complexity
| Task | Points | Subtasks |
|------|--------|----------|
| T6 | 1 | 0 (leaf) |
| T7 | 2 | 0 (leaf) |
| T8 | 1 | 0 (leaf) |
| T9 | 3 | 0 (leaf) |
| T10 | 2 | 0 (leaf) |

**Group Summary:** 5 tasks, 9 points, 0 subtasks. **MEDIUM complexity.**
Ensures backward compatibility, testability, and observability of the new feature.

### GROUP C: Polish + Docs (T11-T12) — LOW Complexity
| Task | Points | Subtasks |
|------|--------|----------|
| T11 | 2 | 0 (leaf) |
| T12 | 1 | 0 (leaf) |

**Group Summary:** 2 tasks, 3 points, 0 subtasks. **LOW complexity.**
Final polish: metrics logging and documentation.

---

## Complexity Distribution

| Complexity | Tasks | Total Points | Subtasks |
|------------|-------|--------------|----------|
| HIGH (≥8 pts/group) | T1-T5 | 14 | 6 |
| MEDIUM (4-7 pts/group) | T6-T10 | 9 | 0 |
| LOW (≤3 pts/group) | T11-T12 | 3 | 0 |

**Overall:** 12 tasks, 26 story points, 6 leaf subtasks. 1 task at 5 points (T4) broken down into 3 subtasks.

## Quality Checks
- [x] All tasks have GWT criteria
- [x] Tasks ≥5 story points broken down (T4: 3 subtasks)
- [x] Groups of 5 used for complexity evaluation
- [x] Coverage: imports → implementation → tests → docs
- [x] No overly verbose tasks (each ≤3 sentences in description)
- [x] Sequential fallback preserved (T7)
- [x] Test coverage for both paths (T9, T10)
