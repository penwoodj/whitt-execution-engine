# 00 — Master Plan

> **Status:** Planning Phase — Documentation Only
> **Phase:** Meta-Workflow Generator Development (Qwen 3.5-9B)
> **Approach:** Plan-driven development with live validation on llama.cpp (CPU-only)

## Executive Overview

The Qwen 3.5-9B Meta-Workflow Generator is a sophisticated agentic system that transforms high-complexity natural language prompts into executable YAML workflow files. The system achieves this through a five-stage pipeline (SW1-SW5) where each sub-workflow produces a single markdown artifact that accumulates and iterates toward a final output. This architectural approach compensates for the limitations of smaller local models (≤ 9B parameters) by decomposing the complex task of prompt-to-workflow translation into focused, scoped stages with internal iteration loops, GWT-based quality gates, and deterministic fix cycles.

The execution engine is a Rust-based workflow runner located at `src/benchmark/runner.rs` with a YAML-driven hook system defined in `src/workflow/hooks/mod.rs`. The model backend is llama.cpp with Vulkan in Docker (CPU-only execution with Q8_0 KV cache acceleration). The target model is Qwen 3.5-9B-UD-Q4_K_XL.gguf, a quantized model optimized for local inference with 262,144 token context window, 5 CPU threads, Q8_0 KV cache, and 1 parallel processing slot.

## Primary Objectives

The project has seven core objectives that drive all development decisions:

1. **Generate Production-Ready Workflow YAMLs** from natural language task specifications that pass schema validation defined in `docs/schema/unified-workflow-schema.yml` (version 2.0.0). Every generated YAML must satisfy the schema line-by-line with no unknown keys, valid hook configurations, and proper template interpolation.

2. **Wire All 7 Wired Hook Triggers** in generated YAMLs with appropriate actions. The runner implements 10 triggers in `src/workflow/hooks/context.rs`, with 7 fully wired at specific line numbers in `src/benchmark/runner.rs`. Each sub-workflow must utilize these triggers appropriately: `before_step_starts` (line 1257), `after_step_starts` (line 1286), `after_step_fails` (line 1318), `after_all_retries_exhausted` (line 1355), `after_step_succeeds` (line 1340), `on_requires_failed` (line 1535), and `after_loop_iteration_fails` (line 1592).

3. **Achieve Quality Surpassing OpenCode Baseline** on live user prompt dataset. Quality is measured across seven dimensions: schema validity (100% required), input coverage (100% required), task granularity (≤ 5 story points per leaf task), hook presence (≥ 5 triggers per step), GWT expression validity (100% required), action variety (all 12 action types used across workflow), and template interpolation (nested depth 3+, variables used properly).

4. **Validate Via Iteration Protocol** with documented improvements. Each sub-workflow must follow a strict iteration loop: run on prompt dataset, collect outputs in timestamped directories, analyze against baseline using the quality rubric in `05-QUALITY-BENCHMARK.md`, identify gaps (missing hooks, invalid GWT, poor granularity), improve YAML or prompts, rerun, compare scores, continue until quality surpasses baseline or max 5 iterations reached.

5. **Build Unit Test Suite** only after live system validation confirms behavior. Unit tests target verified execution paths in the engine: hook execution in `src/workflow/hooks/actions.rs`, GWT evaluation in `src/workflow/hooks/gwt.rs`, template interpolation in the runner, schema validation logic. Tests use `cargo test` framework and reference existing patterns in `tests/hooks_integration.rs`.

6. **Live System Testing First** — every sub-workflow must pass end-to-end execution on the Docker llama.cpp server before any unit test is written. This pragmatic approach finds architectural issues faster than test-driven development for integration-heavy systems. All testing flows through `./target/release/whitt benchmark --workflow <file>` with log analysis via `scripts/analyze-run.sh`.

7. **Create Extensive Documentation** in this plan suite. Each of the 12 files contains 3000-5000 words of actionable, non-placeholder content. Real file paths, real struct names, real line numbers. No TBDs, no hand-waving, no assumptions. Every step is specific, executable, and verifiable against the codebase.

## Success Criteria

Success is defined by a comprehensive set of quantitative and qualitative thresholds that must all be met before the project is considered complete:

| Criterion | Pass Threshold | Evidence Required | Verification Method |
|-----------|----------------|-------------------|---------------------|
| **Output Validity** | 100% of generated YAMLs pass schema validation (`schema_valid=true` in logs) | Log file analysis with grep for `schema_valid` | `scripts/analyze-run.sh LOG OUTPUT` |
| **Input Coverage** | 100% of input tasks covered in output (no dropped tasks) | Manual verification comparing input prompt tasks.md to final workflow.yml | Line-by-line task audit |
| **Task Granularity** | ≤ 5 story points per leaf task (measured by subtask complexity counting) | Structs.md parsing and hierarchical task tree analysis | Automated script counting subtasks |
| **Hook Coverage** | All 7 wired triggers present in generated YAMLs with appropriate actions | YAML inspection counting triggers per step | `grep -c "when:" workflow.yml` |
| **GWT Expressions** | All WHEN clauses have valid GWT expressions (parse without error) | GWT evaluator parsing test in `src/workflow/hooks/gwt.rs` | Unit test with expression samples |
| **Quality Score** | ≥ 80% on manual quality rubric (see `05-QUALITY-BENCHMARK.md`) | Quality report with per-dimension scores | Scoring script on YAML outputs |
| **Live Validation** | All 5 SWs run end-to-end without panics on Docker llama.cpp | Script exit codes and log `[workflow:end]` events | `./target/release/whitt benchmark --workflow` |
| **No Regressions** | Existing 567 tests pass (`cargo test --all-features`) | Test output showing 0 failures | `cargo test --all-features` |
| **Clippy Clean** | 0 warnings (`cargo clippy --all-features -- -W clippy::all`) | Clippy output | `cargo clippy --all-features -- -W clippy::all` |
| **LSP Diagnostics** | 0 errors on all modified files | LSP output via `lsp_diagnostics` | LSP tool on changed files |
| **Build Success** | `cargo build --release` exits with code 0 | Build output | `cargo build --release` |
| **YAML Executable** | Final workflow.yml runs without errors on live system | Benchmark log with `[workflow:end]` | `./target/release/whitt benchmark --workflow workflow.yml` |

## Architecture Overview

The meta-workflow generator follows a five-stage pipeline architecture where each stage produces a single markdown artifact that passes through to the next stage:

```
User Prompt (high-complexity agentic task description)
    ↓ SW1: Task Deconstruction
Task Breakdown with Story-Point Complexity (tasks.md)
    ↓ SW2: Desired Output State Specification
Testable End-State Criteria Per Task (outputs.md)
    ↓ SW3: Agentic Categorization
Behavior Category Assignments Per Task (categories.md)
    ↓ SW4: YAML Substructure Translation
Category-Step Skeleton Mappings (structs.md)
    ↓ SW5: Final Workflow Assembly
Executable Validated Workflow YAML (workflow.yml)
    ↓ Docker llama.cpp Execution
Live System Output with Hooks Fired
```

**Data Flow Characteristics:**

1. **Single File Per Stage** — Each sub-workflow produces exactly one markdown output file. SW1 produces `tasks.md`, SW2 produces `outputs.md`, SW3 produces `categories.md`, SW4 produces `structs.md`, SW5 produces `workflow.yml`. These files accumulate through the pipeline, with each stage reading the previous stage's output and appending new structured content.

2. **Markdown Accumulation Pattern** — Files grow incrementally. SW1 creates the hierarchical task tree. SW2 appends desired state sections under each task. SW3 appends category labels with reasoning. SW4 appends YAML code blocks with skeleton structures. SW5 uses the accumulated content to assemble the final YAML.

3. **Hook-Driven State Capture** — Every sub-workflow uses hooks extensively to capture execution state for quality tracking. Hooks fire at trigger points defined in the runner, log to files in `./workspace/logs/`, and create bookmarks that persist across step boundaries. The hook engine lives at `src/workflow/hooks/mod.rs` with action dispatch in `src/workflow/hooks/actions.rs`.

4. **Schema Validation at End** — SW5 validates the final YAML against `docs/schema/unified-workflow-schema.yml`. The validation logic checks for unknown top-level keys, validates all field types, ensures proper nesting, and verifies that hook configurations match the schema's expected format. Schema validation errors cause immediate iteration.

5. **Live System Execution** — The orchestrator (`meta-workflow-generator-v6.yml`) chains the five sub-workflows via shell invocations. Each sub-workflow runs as a standalone `whitt benchmark --workflow` execution. State passes via the filesystem — each sub-workflow reads predecessor outputs via `cat` commands in shell hooks.

**Hook System Integration:**

The hook system is the foundation of quality tracking and iteration loops. The codebase implements:

- **10 Triggers** defined in `src/workflow/hooks/context.rs` as context struct variants: `BeforeStepStartsContext`, `AfterStepSucceedsContext`, `AfterStepFailsContext`, `AfterAllRetriesExhaustedContext`, `AfterStepStartsContext`, `BeforeGwtEvaluatesContext`, `AfterGwtEvaluatesContext`, `OnRequiresFailedContext`, `AfterLoopIterationFailsContext`, and `DuringStepStreamingContext`.

- **12 Actions** defined in `src/workflow/step.rs` lines 174-323 as `HookAction` enum variants: `Log(LogAction)`, `AppendTo(AppendToAction)`, `SaveTo(SaveToAction)`, `RouteTo(RouteToAction)`, `Bookmark(BookmarkAction)`, `Notify(NotifyAction)`, `Fail(FailAction)`, `Shell(ShellAction)`, `SkipStep(bool)`, `SkipRemaining(bool)`, `Gwt(Vec<GwtClause>)`, and `IterateValues(HashMap)`.

- **7 Fully Wired Triggers** firing at specific locations in `src/benchmark/runner.rs`: `before_step_starts` (line 1257), `after_step_starts` (line 1286), `after_step_fails` (line 1318), `after_all_retries_exhausted` (line 1355), `after_step_succeeds` (line 1340), `on_requires_failed` (line 1535), and `after_loop_iteration_fails` (line 1592).

- **2 Partially Wired Triggers**: `before_gwt_evaluates` and `after_gwt_evaluates` fire with info-level logging only at `src/workflow/hooks/actions.rs` lines 404 and 414. Full wire requires passing `hook_config` through the `execute_action` dispatch chain into `execute_gwt`.

- **1 Dead Trigger**: `during_step_streaming` is not fired in the runner because the benchmark mode uses `stream:false` hardcoded. This trigger requires SSE streaming path integration with `LlamaHttpClient::chat_completion` to access chunk-level hooks.

- **HookResult Merge Logic** in `src/workflow/hooks/mod.rs` defines priority: `Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue`. Multiple actions can fire on the same trigger, with results merged according to this priority.

**Execution Engine Integration:**

The benchmark runner located at `src/benchmark/runner.rs` is the core execution engine:

- **BenchmarkConfig Struct** at line 30-50 defines runtime configuration: server URL, models directory, prompt list, max tokens, temperature, top_p, output directory, workflow file path, cooldown timing, model load timeout, and disk space requirements.

- **WorkflowWorkflowConfig Struct** at line 76-86 defines per-workflow runtime state: prompts vector, max tokens, sampling params (temperature, top_p), comparison mode flags, model list, GPU layer count, and critical `load_params_env_vars` vector that carries llama.cpp environment variables.

- **WorkflowStep Struct** at line 88-100 represents a single step loaded from YAML: step name, ID, requirements vector, when clause hooks, prompt text, generative entity reference, model overrides JSON, and loop configuration.

- **LoadParams::to_env_vars()** at `src/model/schema.rs` line 964-980 converts model load parameters to environment variable pairs. This is critical for wiring Qwen 3.5-9B configuration into the Docker container: context size, batch size, cache types, GPU layers, thread count, mmap flag, flash attention flag, continuous batching flag, no-cache-prompt flag, and parallel slot count.

## Plan Document Structure

This master plan references 11 detailed documents that together form a complete planning suite:

1. **01-OBJECTIVES-AND-SCOPE.md** — Detailed objectives extracted from user prompt, in-scope vs out-of-scope boundaries, acceptance criteria for each of the 5 sub-workflows, definition of "surpasses opencode baseline quality" with quantitative thresholds, success metrics, and failure mode definitions.

2. **02-ARCHITECTURE.md** — System architecture diagrams, 5-sub-workflow pipeline detailed design (SW1: task deconstruction → SW2: desired output state → SW3: agentic categorization → SW4: YAML substructure translation → SW5: final workflow assembly), data flow contracts between stages, single .md output file accumulation strategy, hook integration points per sub-workflow, and orchestrator shell-based invocation pattern.

3. **03-SUBWORKFLOW-SPECIFICATIONS.md** — Detailed specifications for each of the 5 sub-workflows: input format (markdown file structure), output format (markdown file structure), step-by-step execution pipelines, evaluation criteria per stage, fix loops with GWT conditions, chunk sizing strategy (start small, grow as quality improves), complexity scoring rules (5+ story points must be broken down, 2-3 human days = mid-level team), and model configuration references.

4. **04-ITERATION-PROTOCOL.md** — How to iterate each sub-workflow until quality surpasses opencode baseline: baseline creation process using single-shot opencode attempts, comparison metrics (7 dimensions), quality gates (minimum scores per dimension), when to move to next sub-workflow (all gates pass), maximum iterations per sub-workflow (5), failure recovery strategies, and iteration log schema for tracking improvements.

5. **05-QUALITY-BENCHMARK.md** — How to create baseline task breakdowns in opencode for same prompts, comparison methodology (automated scoring script vs manual rubric), scoring rubric (7 dimensions with weightings), what "surpasses" means quantitatively (≥ 10% improvement in quality score), evaluation criteria checklist per dimension, and baseline dataset management.

6. **06-TEST-DATASET.md** — Prompt dataset specification: 10-15 complex prompts extracted from real OpenCode sessions (reference sessions: ses_13d246579ffeoYfYtn38hAEldq, ses_17a245a9cffeWo9uVFAyuF6C9I, ses_18911fba3ffeCV0aETu2P7em8Q, ses_21eda916dffexLBSamby9C941e, ses_252ddda20ffeFXdMZYwVt5gbWu), prompt categories (file operations, code generation, system configuration, debugging workflows), complexity distribution (30% low, 50% medium, 20% high), and how to extract and clean prompts (fix spelling/grammar only, preserve technical specificity).

7. **07-HOOKS-STRATEGY.md** — Which hooks to use in each sub-workflow, when: triggers (before_step_starts, after_step_succeeds, after_step_fails, after_all_retries_exhausted), actions (log for state capture, save_to for artifact persistence, shell for inter-SW communication, bookmark for state transfer, gwt for conditional routing, append_to for accumulation), hook chains for evaluation-fix loops (log → evaluate → route_to → fix → bookmark), and logging strategy for quality tracking (structured logs with JSON events).

8. **08-CONFIG-AND-INFRASTRUCTURE.md** — Qwen 3.5-9B configuration details: context 262144 tokens, gpu_layers 0 (CPU-only), threads 5, parallel 1 slot, cache Q8_0 for both K and V tensors. Docker compose setup details, llama.cpp server flags mapping, LoadParams::to_env_vars() wiring at `src/model/schema.rs:964-980`, sampling config flow from schema to runtime, and model download commands from HuggingFace.

9. **09-LIVE-SYSTEM-TESTING.md** — Protocol for live Docker testing: start llama.cpp server container, verify model loaded via `whitt model list`, run sub-workflow via `whitt benchmark --workflow`, inspect logs using `./scripts/analyze-run.sh LOG OUTPUT`, validate output .md files for proper structure and content, iterate based on failures identified in logs, and all testing MUST be live system first with unit tests only after live validation confirms behavior.

10. **10-UNIT-TEST-STRATEGY.md** — Strategy for writing unit tests ONLY after live system testing produces successful results: test what's confirmed working (no speculative tests), cover execution engine behavior (hook firing, GWT evaluation, template interpolation), use `cargo test` framework with test organization mirroring source structure, reference existing test patterns in `src/workflow/hooks/actions.rs` unit tests and `tests/hooks_integration.rs` integration tests, and ensure tests cover critical paths identified during live testing.

11. **11-EXECUTION-CHECKLIST.md** — Step-by-step checklist for executing the entire plan: phase gates (each sub-workflow must pass before advancing), deliverables per phase (artifacts, logs, iteration reports), verification commands (cargo test, cargo clippy, lsp_diagnostics, build success), commit points (when to checkpoint progress), and when to move to next phase (quality gates passed).

## Known Constraints and Limitations

The project operates under several hard constraints derived from the codebase, infrastructure, and architectural decisions:

### Model Constraint: Qwen 3.5-9B CPU-Only Only

The model specification is locked to Qwen 3.5-9B-UD-Q4_K_XL.gguf with CPU-only execution. This constraint comes from the LoadParams struct defaults at `src/model/schema.rs`:

- `context_size: 262144` (line 983-985) — Full context window requires ≥ 32 GB RAM
- `gpu_layers: 0` (line 1003-1005) — CPU-only inference, no GPU layer offload
- `threads: 5` (line 1007-1009) — 5 CPU threads for inference
- `cache_type_k: "q8_0"` and `cache_type_v: "q8_0"` (lines 995-1001) — Q8_0 KV cache acceleration
- `parallel: 1` (line 1023-1025) — Single parallel processing slot
- `flash_attn: true` (line 1015-1017) — Flash attention enabled for Vulkan compatibility
- `cont_batching: false` (line 950-951) — Continuous batching disabled (required for Vulkan)
- `no_cache_prompt: true` (line 955-956) — Prompt caching disabled (required for Vulkan)

These constraints are enforced via environment variables in `LoadParams::to_env_vars()` at lines 964-980. The Docker container must expose these as `LLAMA_ARG_*` environment variables for llama.cpp server startup. The mapping is:

- `context_size` → `LLAMA_ARG_CTX_SIZE`
- `batch_size` → `LLAMA_ARG_BATCH_SIZE`
- `ubatch_size` → `LLAMA_ARG_UBATCH_SIZE`
- `cache_type_k` → `LLAMA_ARG_CACHE_TYPE_K`
- `cache_type_v` → `LLAMA_ARG_CACHE_TYPE_V`
- `gpu_layers` → `LLAMA_ARG_N_GPU_LAYERS`
- `threads` → `LLAMA_ARG_N_THREADS`
- `use_mmap` → `LLAMA_ARG_USE_MMAP`
- `flash_attn` → `LLAMA_ARG_FLASH_ATTN`
- `cont_batching` → `LLAMA_ARG_CONT_BATCHING`
- `no_cache_prompt` → `LLAMA_ARG_NO_CACHE_PROMPT`
- `parallel` → `LLAMA_ARG_PARALLEL`

### Backend Constraint: Llama.cpp with Vulkan Only

The project uses llama.cpp with Vulkan backend in Docker, NOT LM Studio, NOT Ollama. This constraint is defined in the unified schema at `docs/schema/unified-workflow-schema.yml` line 28 and enforced in the codebase:

- Provider key MUST be `llama_cpp_with_vulkan` (schema line 28)
- Provider config MUST use `config:` wrapper with `host:` and `port:` (schema line 29-31)
- Model host.type MUST be `llama_cpp_with_vulkan` (schema line 71)
- Validation in `WorkflowFile::validate()` MUST reject any provider other than `llama_cpp_with_vulkan` in current POC scope

Docker compose configuration uses the base `docker/docker-compose.yml` file (not AMD or NVIDIA variants) with entrypoint mounted at `/entrypoint.sh:ro` and server entrypoint as `['tini', '--', '/entrypoint.sh']`.

### Schema Constraint: Unified-Workflow-Schema.yml Is Source of Truth

The unified schema at `docs/schema/unified-workflow-schema.yml` (830 lines) is THE source of truth for all workflow YAMLs. This constraint is critical:

- Schema version minimum: 2.0.0 (line 829)
- Only keys defined in the schema are allowed in workflow YAMLs
- Non-schema extensions (benchmark:, model_list:, logging:, execution:) are FORBIDDEN
- Redundant config is FORBIDDEN: if providers already defines host/port, models must NOT duplicate with connection_settings
- If `workflow_execution_strategy.load_unload` is set, `model_lifecycle.load_unload_strategy` must NOT duplicate it
- `WorkflowFile::validate()` MUST reject unknown top-level keys not in the schema
- Every new key added to YAMLs MUST have a schema line reference comment
- When deferring schema features: add `# 🔵 DEFERRED: <explanation>` comment in the YAML

### Hook Constraint: Partial Wiring and Dead Trigger

The hook system has known limitations:

- 7/10 triggers fully wired in runner at specific line numbers
- 2/10 triggers partially wired (before_gwt_evaluates, after_gwt_evaluates) with info logging only
- 1/10 trigger dead (during_step_streaming) — requires SSE streaming path integration
- Full wire for GWT triggers requires passing `hook_config` through execute_action dispatch chain
- Streaming trigger requires architectural change to access chunk-level hooks from LlamaHttpClient

### Testing Constraint: Live System First, Unit Tests Later

Testing follows a strict order: live system validation first, unit tests only after behavior confirmed. This constraint is non-negotiable:

- Every sub-workflow must pass end-to-end execution on Docker llama.cpp before unit test is written
- Unit tests target ONLY verified execution paths, no speculative tests
- All testing flows through `./target/release/whitt benchmark --workflow <file>`
- Log analysis via `scripts/analyze-run.sh` provides evidence of behavior
- Unit tests use patterns from `tests/hooks_integration.rs` and `src/workflow/hooks/actions.rs`

## Execution Phases

The project follows a five-phase execution model that aligns with the sub-workflow pipeline:

### Phase 1: Task Deconstruction (SW1)

**Objective:** Break high-complexity prompt into atomic tasks with story-point complexity.

**Input:** Raw user prompt (natural language task description).

**Output:** `tasks.md` — Hierarchical task tree with complexity scores.

**Pipeline Steps:**

1. Receive raw prompt as input via workflow input variable.
2. LLM call: break prompt into candidate atomic tasks (group of ~5).
3. LLM call per group: score complexity in story points (1-13 scale) by expanding each task into agentic subtasks — count + complexity of subtasks = parent's score.
4. Any task scoring ≥ 5 pts (≈ 2-3 working days for mid-level engineer) MUST be expanded into subtasks. Loop until every task is < 5 pts or already atomic.
5. GWT evaluation: tasks must satisfy (a) atomic, (b) well-named, (c) not overly verbose, (d) follow quality engineering principles, (e) GWT criteria fully flushed out for each. Route back to fix step if any fail.
6. Final accumulated output: `tasks.md` with hierarchical task tree.

**Hook Configuration:**

- `before_step_starts`: Log task breakdown start, bookmark initial state.
- `after_step_succeeds`: Save `tasks.md` to output directory, log quality metrics.
- `after_step_fails`: Log failure details, route to fix step with specific error guidance.

**Quality Gates:**

- All tasks have story point scores (1-13 scale).
- No task exceeds 5 story points (must be decomposed).
- Task hierarchy is complete and consistent.
- Task names follow engineering best practices (verb-noun pattern, atomic scope).

**Iteration Protocol:**

- Run SW1 on prompt dataset (10-15 prompts).
- Analyze outputs vs baseline opencode attempts.
- Check: task coverage completeness (no dropped subtasks).
- Check: complexity accuracy (scores reflect actual effort).
- Check: naming quality (clear, atomic, actionable).
- If gates fail: iterate prompt templates or fix logic.
- Max 5 iterations per prompt before escalation.

### Phase 2: Desired Output State Specification (SW2)

**Objective:** Define testable end-state criteria for each task from SW1.

**Input:** `tasks.md` from SW1.

**Output:** `outputs.md` — Testable desired state per task with GWT criteria.

**Pipeline Steps:**

1. Read `tasks.md` from SW1 via shell `cat` in hook.
2. Chunk tasks by complexity groups (start with chunks of 3, gradually increase to 5).
3. For each chunk, LLM call: produce "desired end-state" for each task — what observable output state proves this task is complete? Include testable criteria (GWT-style).
4. GWT evaluation per chunk: each desired state must be (a) testable, (b) unambiguous, (c) tied to a specific task, (d) not over-scoped.
5. Iterate chunks until quality bar passes; merge all chunks into single `outputs.md`.

**Hook Configuration:**

- `before_step_starts`: Load tasks from `tasks.md`, log chunking strategy.
- `after_step_succeeds`: Save `outputs.md`, validate criteria count matches task count.
- `after_step_fails`: Log specific criteria failure, route to fix with guidance.

**Quality Gates:**

- Every task has corresponding desired state criteria.
- Criteria are testable (observable, measurable).
- Criteria are unambiguous (clear pass/fail conditions).
- No over-scoping (criteria match task scope exactly).

**Iteration Protocol:**

- Run SW2 on SW1 outputs for each prompt in dataset.
- Analyze outputs vs baseline (single-shot opencode desired states).
- Check: criteria completeness (one per task).
- Check: testability (can be verified via shell or tool).
- Check: scope alignment (criteria don't exceed task).
- If gates fail: iterate prompt templates or fix logic.
- Max 5 iterations per prompt before escalation.

### Phase 3: Agentic Categorization (SW3)

**Objective:** Assign behavior categories to each task to guide YAML structure mapping.

**Input:** `tasks.md` from SW1 + `outputs.md` from SW2.

**Output:** `categories.md` — Category labels with reasoning per task.

**Pipeline Steps:**

1. Read `tasks.md` + `outputs.md` from SW1 + SW2 via shell `cat`.
2. LLM call per task: assign category label from canonical set: "file-read", "transform-llm", "validate-gate", "loop-iterate", "branch-decision", "shell-execute", "tool-call", "checkpoint-state", "notify-external", "sub-workflow-ref".
3. GWT: categories must be (a) from canonical set, (b) mutually exclusive per task, (c) explain WHY category fits, (d) reference specific task characteristics.
4. Iterate until all tasks categorized consistently. Output: `categories.md`.

**Hook Configuration:**

- `before_step_starts`: Load tasks + outputs, log canonical set.
- `after_step_succeeds`: Save `categories.md`, validate category distribution.
- `after_step_fails`: Log category assignment error, route to fix with canonical set reference.

**Quality Gates:**

- All categories from canonical set (no invented categories).
- One category per task (mutually exclusive).
- Category reasoning provided for each assignment.
- Category choices consistent across similar tasks.

**Iteration Protocol:**

- Run SW3 on SW1+SW2 outputs for each prompt in dataset.
- Analyze outputs vs baseline (opencode manual categorization).
- Check: canonical set adherence (no invalid categories).
- Check: mutual exclusivity (one category per task).
- Check: reasoning quality (category justification provided).
- If gates fail: iterate prompt templates or fix logic.
- Max 5 iterations per prompt before escalation.

### Phase 4: YAML Substructure Translation (SW4)

**Objective:** Translate categorized tasks into YAML skeleton structures per category.

**Input:** `tasks.md` from SW1 + `outputs.md` from SW2 + `categories.md` from SW3.

**Output:** `structs.md` — YAML code blocks with skeleton structures + intent prose.

**Pipeline Steps:**

1. Read `tasks.md` + `outputs.md` + `categories.md` via shell `cat`.
2. LLM call per category: emit YAML substructure skeleton for each task in that category using `agentic_workflow.steps.<step_name>:` shape from schema.
3. GWT: each YAML substructure must (a) reference correct model (`${models.primary-analyzer}`), (b) have correct hook wiring for its category, (c) include dependencies on prior tasks (`requires:` list), (d) NOT contain non-YAML code blocks (no ```rust, ```python, ```bash — only ```yaml).
4. Iterate. Final: `structs.md` documents the YAML substructures + intent prose.

**Hook Configuration:**

- `before_step_starts`: Load all 3 previous outputs, log category groups.
- `after_step_succeeds`: Save `structs.md`, validate YAML syntax via `cargo run -- validate-yaml`.
- `after_step_fails`: Log YAML generation error, route to fix with schema line references.

**Quality Gates:**

- Every task has corresponding YAML skeleton structure.
- Skeletons use correct schema shape (`agentic_workflow.steps.<name>:`).
- Model references correct (`${models.primary-analyzer}`).
- Hook wiring appropriate for category.
- No non-YAML code blocks in YAML sections.

**Iteration Protocol:**

- Run SW4 on SW1+SW2+SW3 outputs for each prompt in dataset.
- Analyze outputs vs baseline (opencode manual YAML skeletons).
- Check: YAML syntax validity (parse without error).
- Check: schema shape adherence (correct keys, nesting).
- Check: model reference correctness (references defined model).
- Check: hook presence (appropriate hooks for category).
- If gates fail: iterate prompt templates or fix logic.
- Max 5 iterations per prompt before escalation.

### Phase 5: Final Workflow Assembly (SW5)

**Objective:** Assemble valid executable workflow YAML from skeleton structures.

**Input:** `structs.md` from SW4.

**Output:** `workflow.yml` — Executable validated agentic workflow YAML.

**Pipeline Steps:**

1. Read `structs.md` from SW4 via shell `cat`.
2. LLM call: assemble full workflow YAML (header + providers + models + execution_strategy + agentic_workflow.steps merging all substructures).
3. Deterministic post-process (shell hook): run `scripts/fix-generated-yaml.py` then `scripts/validate-yaml.py`.
4. Per-task exhaustive review loop: for each task in `tasks.md`, verify the task appears and is exhaustively handled in the assembled YAML. If any task is missing or under-implemented, route back to fix step.
5. Final pass: emit `workflow.yml`.

**Hook Configuration:**

- `before_step_starts`: Load structs, log assembly strategy.
- `after_step_succeeds`: Save `workflow.yml`, validate against schema (schema_valid=true in logs).
- `after_step_fails`: Log assembly error, route to fix with schema validation output.

**Quality Gates:**

- YAML passes schema validation (`schema_valid=true` in logs).
- All tasks from `tasks.md` appear in workflow.
- All desired states from `outputs.md` captured in hooks or conditions.
- All categories from `categories.md` reflected in structure.
- All skeleton structures from `structs.md` integrated correctly.
- No unknown top-level keys (schema compliance).
- Template interpolation works ({{...}} references resolve).

**Iteration Protocol:**

- Run SW5 on SW4 outputs for each prompt in dataset.
- Analyze outputs vs baseline (opencode manual workflow assembly).
- Check: schema validity (100% required).
- Check: task coverage completeness (no dropped tasks).
- Check: hook wiring completeness (7 triggers present).
- Check: template interpolation correctness (no broken references).
- If gates fail: iterate prompt templates or fix logic.
- Max 5 iterations per prompt before escalation.

## Dependency Graph Between Phases

The phases have strict dependency relationships that dictate execution order:

```
SW1 (Task Deconstruction)
    ↓ outputs tasks.md
SW2 (Desired Output State) ← REQUIRES tasks.md
    ↓ outputs outputs.md
SW3 (Agentic Categorization) ← REQUIRES tasks.md + outputs.md
    ↓ outputs categories.md
SW4 (YAML Substructure Translation) ← REQUIRES tasks.md + outputs.md + categories.md
    ↓ outputs structs.md
SW5 (Final Workflow Assembly) ← REQUIRES structs.md
    ↓ outputs workflow.yml
Live System Execution ← REQUIRES workflow.yml
```

**Dependencies Explained:**

- SW2 depends on SW1 because desired state criteria must be defined for each task identified in the task breakdown.
- SW3 depends on SW1 and SW2 because category assignment requires understanding both the task nature (from tasks.md) and the expected output (from outputs.md).
- SW4 depends on SW1, SW2, and SW3 because YAML skeleton structure generation needs task details, desired states, and category labels to produce appropriate step configurations.
- SW5 depends on SW4 because final workflow assembly needs the complete skeleton structures to integrate into the full YAML.

**No Parallel Execution Possible**

Given the strict data dependencies and the fact that each phase produces a single accumulated markdown file that all subsequent phases read, parallel execution is not possible. The orchestrator must execute phases sequentially, passing state via the filesystem.

**Failure Propagation**

If a phase fails to meet its quality gates, subsequent phases cannot proceed. The iteration protocol allows up to 5 retry attempts per phase per prompt. If 5 iterations fail to meet quality gates, the failure is escalated (documented in iteration report, prompt moved to deferred dataset, or architectural reconsideration triggered).

## Timeline and Milestones

The project follows a timeline-driven approach with specific milestones for each phase:

### Overall Timeline: 6 Phases × 2-3 Days Each = 12-18 Days

**Phase 1: SW1 Task Deconstruction (2-3 days)**
- Day 1: Create SW1 YAML, hook configuration, iteration templates.
- Day 2: Run live tests on prompt dataset, collect outputs.
- Day 3: Analyze vs baseline, iterate up to 5 times, document findings.

**Phase 2: SW2 Desired Output State (2-3 days)**
- Day 1: Create SW2 YAML, hook configuration, chunking strategy.
- Day 2: Run live tests on SW1 outputs, collect outputs.
- Day 3: Analyze vs baseline, iterate up to 5 times, document findings.

**Phase 3: SW3 Agentic Categorization (2-3 days)**
- Day 1: Create SW3 YAML, hook configuration, canonical set definition.
- Day 2: Run live tests on SW1+SW2 outputs, collect outputs.
- Day 3: Analyze vs baseline, iterate up to 5 times, document findings.

**Phase 4: SW4 YAML Substructure Translation (2-3 days)**
- Day 1: Create SW4 YAML, hook configuration, schema reference mapping.
- Day 2: Run live tests on SW1+SW2+SW3 outputs, collect outputs.
- Day 3: Analyze vs baseline, iterate up to 5 times, document findings.

**Phase 5: SW5 Final Workflow Assembly (2-3 days)**
- Day 1: Create SW5 YAML, hook configuration, validation scripts.
- Day 2: Run live tests on SW4 outputs, collect outputs.
- Day 3: Analyze vs baseline, iterate up to 5 times, document findings.

**Phase 6: Integration and Unit Testing (3-5 days)**
- Day 1: Run orchestrator end-to-end on all prompts.
- Day 2: Analyze full pipeline outputs, identify integration issues.
- Day 3: Fix integration issues, retest until all gates pass.
- Day 4: Write unit tests for verified execution paths.
- Day 5: Run test suite, verify 0 regressions, complete documentation.

### Milestone Definition

A milestone is achieved when all of the following conditions are met:

1. All quality gates for the phase pass (quantitative thresholds met).
2. Live system testing completes without panics on all prompts in dataset.
3. Iteration report documents improvements vs baseline (≥ 10% quality score improvement or equivalent qualitative gains).
4. Artifacts (markdown outputs, log files, iteration reports) are committed to repository.
5. No regressions in existing functionality (567 tests still pass, clippy clean).

### Blocker Resolution

If a milestone cannot be achieved within the allocated time:

1. Document the blocker in the iteration report with specific failure details.
2. Categorize blocker: infrastructure (Docker/model), architectural (engine limitation), or model quality (Qwen capability).
3. For infrastructure block: fix immediately (Docker config, model load parameters).
4. For architectural block: document limitation, defer feature, create issue for future work.
5. For model quality block: iterate prompts (max 5 more attempts), then defer prompt or escalate to architectural reconsideration.

### Success Definition

The entire project is successful when:

1. All 5 sub-workflows run end-to-end without panics on Docker llama.cpp.
2. Each sub-workflow has a `golden/` artifact matching or beating the `baseline-opencode/` artifact for the same input prompt.
3. The final assembled `workflow.yml` is schema-valid and executable by `whitt benchmark --workflow workflow.yml`.
4. All iteration gates documented in execution checklist pass (quality metrics met).
5. Unit tests in `tests/meta_workflow/` cover the confirmed execution paths (no untested critical paths).
6. Existing 567 tests pass with 0 failures (no regressions).
7. Clippy passes with 0 warnings on all modified files.

## Risk Mitigation

The project faces several categories of risk, each with specific mitigation strategies:

### Model Quality Risk

**Risk:** Qwen 3.5-9B may not produce sufficient quality for complex tasks, leading to infinite iteration loops or failure to meet quality gates.

**Mitigation:**

1. Start with prompt dataset calibrated to Qwen capabilities (extracted from successful OpenCode sessions).
2. Use chunking strategy to break large tasks into manageable pieces.
3. Implement iteration protocol with max 5 attempts to prevent infinite loops.
4. Fallback to simpler prompts for failed cases (defer complex prompts for future model improvements).
5. Document model limitations in iteration reports to inform future architectural decisions.

### Infrastructure Risk

**Risk:** Docker llama.cpp container may fail to start, OOM, or have connectivity issues, blocking all live testing.

**Mitigation:**

1. Pre-validate Docker compose configuration with base `docker/docker-compose.yml` (not AMD/NVIDIA variants).
2. Verify host has ≥ 32 GB RAM for Qwen 3.5-9B with 262144 context.
3. Set `gpu_layers: 0` for CPU-only execution (no GPU dependency).
4. Implement health check in orchestrator: poll port 8080 for server readiness.
5. Implement timeout in model load (30 minutes) to prevent hangs.
6. Implement cleanup in failure paths (container stop, port release).

### Schema Compliance Risk

**Risk:** Generated YAMLs may contain unknown keys, invalid structures, or break schema validation, requiring extensive iteration.

**Mitigation:**

1. Reference schema line numbers in all prompt templates (e.g., "Schema line 318-322 defines step structure").
2. Implement post-processing script `scripts/validate-yaml.py` that checks schema compliance before final output.
3. Use GWT expressions in SW5 to validate specific schema constraints (no unknown keys, proper nesting).
4. Implement hook in SW5 `after_step_succeeds` that parses YAML and runs schema validation.
5. Iterate SW5 templates with schema feedback until 100% validation rate achieved.

### Hook Integration Risk

**Risk:** Hooks may not fire correctly, actions may fail to execute, or bookmark state may not transfer between steps, breaking iteration loops.

**Mitigation:**

1. Use only 7 fully wired triggers (avoid 2 partially wired, 1 dead).
2. Reference specific line numbers in runner where triggers fire (e.g., "Hook fires at src/benchmark/runner.rs:1257").
3. Test hook chains incrementally (log → save → bookmark → route_to → fix).
4. Implement fallback in bookmark failure (retry bookmark 3 times, then fail workflow).
5. Use `log` action extensively for debugging hook execution flow.

### Data Flow Risk

**Risk:** Filesystem-based state passing may fail due to race conditions, file write errors, or path issues between sub-workflows.

**Mitigation:**

1. Use timestamp-based run IDs for all output directories (prevent conflicts).
2. Implement atomic file writes (write to temp, then rename to final).
3. Use `sed` for run ID substitution in orchestrator (proven pattern from v5).
4. Implement shell command validation (check exit codes, log stderr).
5. Use `cat` in shell hooks for reading predecessor outputs (proven reliable).

### Iteration Loop Risk

**Risk:** Iteration loops may get stuck in cycles, fail to converge, or exceed iteration limits without reaching quality gates.

**Mitigation:**

1. Implement max 5 iterations per sub-workflow per prompt (hard limit).
2. Document iteration count in bookmark state (track convergence).
3. Implement quality gates with numeric thresholds (pass/fail clear).
4. Use GWT expressions to detect stagnation (no improvement in 3 iterations → fail).
5. Implement escalation path: 5 failed iterations → defer prompt → document for future work.

## Next Steps

This master plan provides the foundation for executing the Qwen 3.5-9B meta-workflow generator project. The next immediate steps are:

1. **Review this master plan** — Confirm all objectives, success criteria, and constraints are understood.
2. **Read detailed specs** — Review 11 detailed documents for deep technical specifications per domain.
3. **Execute SW1** — Start with Phase 1: Task Deconstruction, following the execution checklist.
4. **Document findings** — Create iteration reports for each sub-workflow, recording quality improvements vs baseline.
5. **Iterate until done** — Follow the iteration protocol through all 5 phases, escalating blockers as needed.

The plan suite is comprehensive, actionable, and grounded in the actual codebase infrastructure. Every step references real files, real line numbers, and real struct names. No hand-waving, no placeholders, no TBDs. This plan is ready for execution.

---

**Document Status:** Draft
**Last Updated:** 2026-06-17
**Author:** Sisyphus-Junior (Whitt Execution Engine Planning)
**Review Status:** Ready for Execution