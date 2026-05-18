# Implementation Plan: Multi-Model Benchmarking & Workflow Loops

## Overview

Implement 50-model sequential benchmarking from external drive, workflow loop execution engine, and agentic summarization-expansion workflows. All code in Rust. All YAML conforms to `docs/schema/unified-workflow-schema.yml`.

## Phases

### Phase A: Infrastructure (unblocks everything)

**Goal**: Models accessible to Docker without copying. Disk monitoring prevents main drive fill.

#### A1: Symlink Model Access
- **File**: `src/client/docker_manager.rs`
- **What**: Add `mount_external_models(models_dir: &Path)` that verifies `MODEL_HOST_PATH` points to external drive
- **Pattern**: Docker compose already uses `MODEL_HOST_PATH` env var. Verify at benchmark start that path exists and is mounted
- **Test**: Unit test that validates path exists, integration test that Docker sees models

#### A2: Model Discovery from External Drive
- **File**: `src/client/model_discovery.rs` (new)
- **What**: Scan external drive directory, build `Vec<ModelCandidate>` from GGUF files
- **Struct**:
  ```rust
  struct ModelCandidate {
      path: PathBuf,           // /run/media/jon/data/models/author/model/file.gguf
      model_id: String,        // derived from filename
      file_size_bytes: u64,
      estimated_vram_bytes: u64,
      fits_in_vram: bool,      // <= 6GB for our 8GB VRAM
      author: String,          // directory name
  }
  ```
- **Logic**: Walk `/run/media/jon/data/models/`, filter `.gguf`, compute VRAM fit, sort by size
- **Test**: Proptest with directory structure, unit test with known layout

#### A3: Disk Space Monitor
- **File**: `src/client/disk_monitor.rs` (new)
- **What**: Check free space on main drive before each model operation. Fail early if < 5GB free
- **Pattern**: `fs2::available_space()` or `statvfs` via `nix` crate
- **Test**: Unit test with mock filesystem

#### A4: Cleanup After Model Unload
- **File**: `src/client/http_client.rs`
- **What**: After `unload_model()`, verify server reports 0 loaded models. If temp files created, clean up
- **Pattern**: `unload_model()` → `list_models()` → assert model gone → clean temp dir
- **Test**: Integration test with mock server

**Estimated effort**: 4-6 hours
**Deliverable**: `cargo test --all-features` passes, external drive models discoverable

---

### Phase B: Multi-Model Benchmark Command

**Goal**: `whitt benchmark --models-dir PATH` loads and benchmarks each model sequentially.

#### B1: BenchmarkResult Types
- **File**: `src/benchmark/mod.rs` (new module)
- **What**: Structured result types for single-model and multi-model benchmarks
- **Structs**:
  ```rust
  struct ModelBenchmarkResult {
      model_id: String,
      model_path: PathBuf,
      file_size_bytes: u64,
      load_duration: Duration,
      inference_results: Vec<InferenceResult>,
      unload_duration: Duration,
      total_duration: Duration,
      tokens_per_second: f64,
      avg_latency_ms: f64,
      p50_latency_ms: f64,
      p99_latency_ms: f64,
      error: Option<String>,
  }

  struct InferenceResult {
      prompt: String,
      prompt_tokens: usize,
      completion_tokens: usize,
      total_tokens: usize,
      duration: Duration,
      tokens_per_second: f64,
  }

  struct BenchmarkSuiteResult {
      timestamp: String,
      gpu_info: String,
      total_models: usize,
      successful: usize,
      failed: usize,
      results: Vec<ModelBenchmarkResult>,
      comparison_csv: String,  // pre-formatted CSV
  }
  ```
- **Test**: Unit tests for percentile calculation, CSV formatting

#### B2: BenchmarkRunner
- **File**: `src/benchmark/runner.rs`
- **What**: Orchestrates sequential model swaps and benchmarking
- **Pattern**: Based on `model_chain.rs` load→infer→unload but generalized
- **Flow**:
  1. Discover models from `--models-dir` (Phase A2)
  2. For each model:
     a. Check disk space (Phase A3)
     b. Unload any loaded model
     c. Load model (with retry, 120s timeout)
     d. Run N prompts (configurable, default 3)
     e. Collect timing metrics
     f. Unload model (Phase A4)
     g. Record result
  3. Aggregate results into `BenchmarkSuiteResult`
  4. Output JSON + CSV + console table
- **CLI flags**:
  - `--models-dir PATH` — scan directory for GGUF files
  - `--model-list FILE` — read specific model paths from file
  - `--prompts N` — number of prompts per model (default 3)
  - `--max-tokens N` — max tokens per prompt (default 128)
  - `--output FORMAT` — json, csv, table (default table)
  - `--filter-size-max BYTES` — skip models larger than this
  - `--filter-name PATTERN` — regex filter on model name
- **Test**: Integration test with mock backend, unit test for runner logic

#### B3: Remove Dead `--concurrent` Code
- **File**: `src/bin/whitt.rs`
- **What**: Remove `--concurrent` flag from benchmark subcommand (dead code, no implementation)
- **Alternative**: If we want concurrent later, it's a new feature with proper design
- **Test**: CLI help output verification

#### B4: Benchmark Report Generation
- **File**: `src/benchmark/report.rs`
- **What**: Format results as JSON, CSV, and console table
- **Output formats**:
  - JSON: Full structured output for programmatic use
  - CSV: One row per model, columns for key metrics
  - Table: Human-readable console output with aligned columns
- **Test**: Snapshot tests for each format

**Estimated effort**: 8-12 hours
**Deliverable**: `whitt benchmark --models-dir /run/media/jon/data/models/` runs 50-model benchmark

---

### Phase C: GPU vs CPU Comparison Mode

**Goal**: Run each model twice — once GPU-offloaded, once CPU-only — for comparison.

#### C1: Dual-Mode Runner
- **File**: `src/benchmark/runner.rs` (extend)
- **What**: Add `--compare-gpu-cpu` flag that runs each model in both modes
- **Mechanism**:
  - GPU run: Current Docker setup with Vulkan offloading
  - CPU run: Set `--n-gpu-layers 0` in server args (requires server restart)
  - Need: `DockerManager::restart_with_args(args)` method
- **Output**: Paired results per model showing GPU speedup factor
- **Test**: Integration test with mock server configs

#### C2: Server Config Switching
- **File**: `src/client/docker_manager.rs`
- **What**: Add method to restart server with different GPU offload settings
- **Pattern**: Modify `docker-compose.yml` env vars dynamically or pass CLI args
- **Test**: Unit test for config generation

**Estimated effort**: 4-6 hours
**Deliverable**: `whitt benchmark --models-dir PATH --compare-gpu-cpu` produces comparison table

---

### Phase D: Workflow Loop Execution Engine

**Goal**: Execute `loop` blocks from YAML workflow definitions. Schema lines 411-447.

#### D1: LoopConfig Types
- **File**: `src/config/loop_config.rs` (new)
- **What**: Parse loop configuration from YAML step definitions
- **Structs**:
  ```rust
  enum LoopType {
      Count(CountLoop),
      Validation(ValidationLoop),
      ForEach(ForEachLoop),
  }

  struct CountLoop {
      max_iterations: usize,
      iteration_variable: Option<String>,  // e.g., "current_file"
  }

  struct ValidationLoop {
      max_iterations: usize,
      tolerance: f64,
      exact_criteria: Vec<ValidationCriterion>,
  }

  struct ValidationCriterion {
      metric: String,
      operator: ComparisonOperator,  // >= | <= | == | != | > | <
      target: f64,
  }

  enum ComparisonOperator {
      Gte, Lte, Eq, Neq, Gt, Lt,
  }

  struct ForEachLoop {
      items: Vec<String>,           // or template reference
      iteration_variable: String,
  }
  ```
- **Test**: Unit tests for parsing each loop type from YAML

#### D2: LoopExecutor
- **File**: `src/agent/loop_executor.rs` (new)
- **What**: Execute a step with loop config
- **Flow**:
  ```
  match loop_type:
    Count => for i in 0..max_iterations:
      set iteration_variable = i
      execute step
      run hooks (after_step_succeeds, after_loop_iteration_fails)
    
    Validation => loop up to max_iterations:
      execute step
      evaluate criteria
      if all criteria met: break (success)
      run hooks
    
    ForEach => for item in items:
      set iteration_variable = item
      execute step
      run hooks
  ```
- **Pattern**: Similar to `ReActAgent` max_iterations loop but generic over step type
- **Context passing**: Each iteration's output feeds into next iteration's input via `WorkflowContext`
- **Test**: Unit tests with mock step executor, property tests for iteration counts

#### D3: Loop Hooks
- **File**: `src/agent/loop_hooks.rs` (new)
- **What**: Execute hook callbacks at loop lifecycle points
- **Hooks** (from schema):
  - `before_step_starts` — before each iteration
  - `after_step_succeeds` — after successful iteration
  - `after_loop_iteration_fails` — after failed iteration
- **Actions**: `log`, `append_to`, `save_to`, `bookmark`
- **Test**: Unit tests for each hook type

#### D4: Integration with StepExecutor
- **File**: `src/agent/executor.rs` (modify)
- **What**: Check if step has `loop` key, delegate to `LoopExecutor` if present
- **Pattern**: In `execute_step()`, check for loop config, wrap in loop executor
- **Test**: Integration test with YAML workflow containing loop step

**Estimated effort**: 12-16 hours
**Deliverable**: YAML workflows with `loop.count` and `loop.validation` execute correctly

---

### Phase E: Agentic Summarization-Expansion Workflow

**Goal**: Workflow step type that reads doc chunk → summarizes → expands → oscillates 3x.

#### E1: File Input Tool
- **File**: `src/agent/tools.rs` (extend)
- **What**: Add `read_file_chunk` tool that reads a file section by offset/limit
- **Tool**:
  ```rust
  Tool::ReadFileChunk {
      path: String,
      offset: usize,    // byte offset
      length: usize,    // bytes to read
  }
  ```
- **Test**: Unit test with test file

#### E2: Output Capture Between Iterations
- **File**: `src/agent/loop_executor.rs` (extend)
- **What**: Each iteration's output stored in context, accessible as `{{loop.iteration_output}}` in next iteration
- **Mechanism**: After each iteration, store output in `WorkflowContext` under `loop.previous_output`
- **Test**: Unit test verifying iteration N can access iteration N-1 output

#### E3: Summarize-Expansion Step Type
- **File**: `src/agent/step_types.rs` (new)
- **What**: New step type that alternates between summarization and expansion
- **Config** (YAML):
  ```yaml
  - step: summarize_expand
    type: oscillate_abstraction
    input:
      file_path: "./docs/plans/my-plan.md"
      chunk_size: 4000       # chars per chunk
      oscillations: 3         # summarize → expand → summarize → expand → ...
    loop:
      count:
        max_iterations: 6     # oscillations * 2
        iteration_variable: current_phase
    output:
      save_to: refined_summary
  ```
- **Flow**:
  1. Read file chunk
  2. Iteration 0: Summarize (compress to key points)
  3. Iteration 1: Expand (elaborate with detail)
  4. Iteration 2: Summarize again (compress further)
  5. Iteration 3: Expand again (add depth)
  6. Iteration 4: Summarize (final compression)
  7. Iteration 5: Expand (final elaboration)
  8. Output final result
- **Prompt templates**:
  - Summarize: "Summarize the following text, extracting key points and removing filler. Preserve all technical substance: {{loop.previous_output}}"
  - Expand: "Expand on the following summary, adding detail, examples, and explanations where they improve understanding: {{loop.previous_output}}"
- **Test**: Integration test with mock LLM, verify oscillation pattern

#### E4: Workflow File Input Parameter
- **File**: `src/config/unified.rs` (extend)
- **What**: Allow workflow to accept file path as runtime parameter
- **Pattern**: `whitt run workflow.yml --input-file ./docs/plans/my-plan.md`
- **Test**: CLI integration test

**Estimated effort**: 8-12 hours
**Deliverable**: YAML workflow with oscillate_abstraction step processes plan files

---

### Phase F: Benchmark YAML Suite Generation

**Goal**: Generate valid YAML benchmark files for 3, 5, 15, and 50 models.

#### F1: Model List Builder
- **File**: `src/benchmark/model_selector.rs` (new)
- **What**: Select N diverse models from external drive for benchmark
- **Strategy**:
  - Tier 1 (0.5-1.5B): 10 models
  - Tier 2 (1.5-3B): 15 models
  - Tier 3 (3-4B): 15 models
  - Tier 4 (4-7B): 10 models
  - Ensure architecture diversity: Qwen, Llama, Mistral, Gemma, Phi, Falcon
  - Ensure specialization diversity: code, reasoning, chat, instruct
- **Test**: Unit test verifying tier distribution

#### F2: YAML Generator
- **File**: `src/benchmark/yaml_generator.rs` (new)
- **What**: Generate valid YAML workflow files from model list
- **Output**: Files in `docs/workflows/benchmarks/`
  - `benchmark-3-models.yml`
  - `benchmark-5-models.yml`
  - `benchmark-15-models.yml`
  - `benchmark-50-models.yml`
- **YAML structure** (conforms to schema):
  ```yaml
  workflow_id: benchmark_50_models
  name: "50-Model Benchmark Suite"
  min_schema_version: "2.0.0"
  
  models:
    primary:
      provider: lmstudio
      model: "${benchmark.current_model}"
  
  execution:
    mode: serial
    memory:
      max_allocated_memory_mb: 8192
      unload_unused: true
  
  agentic_workflow:
    - step: benchmark_model
      id: bench_step
      loop:
        count:
          max_iterations: 50
          iteration_variable: current_model
      input:
        prompt: "benchmark_prompt"
      output:
        save_to: benchmark_results
  ```
- **Validation**: Each generated YAML must parse as `UnifiedConfig` without errors
- **Test**: Round-trip test — generate YAML → parse → verify fields

#### F3: Benchmark Runner Command
- **File**: `src/bin/whitt.rs` (extend)
- **What**: `whitt benchmark-suite YAML_FILE` command
- **Flow**: Parse YAML → discover models → run sequential benchmark → output results
- **Test**: Integration test with 3-model YAML

**Estimated effort**: 6-8 hours
**Deliverable**: 4 benchmark YAML files, all validated against schema

---

## Dependency Graph

```
Phase A (Infrastructure)
    │
    ├──→ Phase B (Benchmarking)
    │       │
    │       └──→ Phase C (GPU vs CPU)
    │
    ├──→ Phase D (Workflow Loops)
    │       │
    │       └──→ Phase E (Agentic Workflow)
    │
    └──→ Phase F (YAML Generation) ← depends on B + D
```

## Environment Constraints

- **Main drive**: 159GB free. Must not fill. Disk monitor fails at < 5GB.
- **Data drive**: `/run/media/jon/data/models/` — 3.1TB free, 293 GGUF files
- **GPU**: AMD 8GB VRAM (Vulkan). Constraints enforced in `docker/entrypoint.sh`
- **Docker**: Container `whitt-qa-server`, port 8081
- **Server**: llama.cpp with `--no-cache-prompt`, no `--cont-batching`, KV cache f16

## Estimated Total Effort

| Phase | Hours | Risk |
|-------|-------|------|
| A: Infrastructure | 4-6 | Low |
| B: Benchmarking | 8-12 | Medium |
| C: GPU vs CPU | 4-6 | Medium |
| D: Workflow Loops | 12-16 | High |
| E: Agentic Workflow | 8-12 | Medium |
| F: YAML Generation | 6-8 | Low |
| **Total** | **42-60** | — |

## Commit Strategy

- One commit per phase completion
- Conventional commits: `feat(benchmark):`, `feat(workflow):`, `feat(agent):`
- Each commit must pass `cargo test --all-features` + `cargo clippy --all-features`
- No commit with warnings or errors
