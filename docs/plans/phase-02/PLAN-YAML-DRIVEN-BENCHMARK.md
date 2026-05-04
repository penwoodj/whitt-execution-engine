# Plan: YAML-Driven Benchmark Execution

## Goal

When `--workflow` is provided, the benchmark YAML file drives execution: prompts, max_tokens, temperature, top_p, compare_modes, and model list all come from the YAML. CLI flags become optional overrides. All agentic workflow steps execute in sequence with lifecycle hooks firing at correct points.

## Success Criteria

1. Running with `--workflow docs/workflows/benchmarks/benchmark-3-models.yml` sends the 3 YAML prompts (recursion, Rust function, microservices) — NOT "The quick brown fox"
2. `benchmark.max_tokens: 128` from YAML is used (not CLI default 64)
3. `benchmark.compare_modes: true` triggers GPU+CPU runs per model
4. `benchmark.temperature: 0.7` and `top_p: 0.9` are passed to LLM
5. Model list comes from YAML `model_list` section
6. All 4 steps execute: benchmark_performance → refine_document → generate_speedup_report → generate_report
7. CLI `--prompts 3 --max-tokens 256` overrides YAML when explicitly provided
8. Per-model .log files show the workflow step that produced them
9. chat-log.md shows YAML prompts with responses
10. `cargo test --all-features` passes, `cargo clippy` clean

## Architecture

### Data Flow (Current vs Proposed)

**Current:**
```
CLI flags → BenchmarkConfig → BenchmarkRunner::run() → HTTP client → LLM server
                                    ↑ ignores YAML entirely
```

**Proposed:**
```
CLI flags (overrides) ─┐
                        ├→ BenchmarkConfig → BenchmarkRunner::run()
YAML workflow (defaults)┘                         ↓
                                          StepExecutor dispatch:
                                          1. benchmark_performance → existing loop
                                          2. refine_document → oscillate_abstraction stub
                                          3. generate_speedup_report → aggregation
                                          4. generate_report → JSON output
                                                    ↓
                                          Lifecycle hooks per step
```

### New Types

```rust
// src/benchmark/workflow_config.rs

struct BenchmarkWorkflowConfig {
    prompts: Vec<String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    compare_modes: bool,
    modes: Vec<String>,
    gpu_config: GpuConfig,
    cpu_config: CpuConfig,
    model_list: Vec<String>,
}

struct BenchmarkWorkflowStep {
    step: String,
    id: String,
    step_type: Option<String>,
    requires: Vec<String>,
    loop_config: Option<WorkflowLoopConfig>,
    input: HashMap<String, serde_json::Value>,
    when: Option<WorkflowHooks>,
    output: Option<WorkflowOutput>,
}

struct WorkflowHooks {
    before_step_starts: Option<HookAction>,
    after_step_succeeds: Option<HookAction>,
    after_step_fails: Option<HookAction>,
    after_loop_iteration_fails: Option<HookAction>,
}

enum BenchmarkStepType {
    BenchmarkPerformance,
    RefineDocument,
    GenerateSpeedupReport,
    GenerateReport,
}
```

### Changes to Existing Code

| File | Change |
|------|--------|
| `src/benchmark/runner.rs` | Add `load_workflow_config()` that parses YAML with `serde_saphyr` and merges with CLI flags |
| `src/benchmark/runner.rs` | `run()` calls `load_workflow_config()` when `workflow_file` is Some |
| `src/benchmark/runner.rs` | `run()` iterates `agentic_workflow` steps, dispatching by type |
| `src/benchmark/runner.rs` | Replace `parse_workflow_context()` (line-by-line string hack) with proper YAML parsing |
| `src/benchmark/mod.rs` | Add `BenchmarkWorkflowConfig` and `BenchmarkWorkflowStep` types |
| `src/bin/whitt.rs` | CLI flags become optional; when `--workflow` is set, YAML provides defaults |

## Implementation Steps

### Step 1: Parse YAML Benchmark Section

**Files:** `src/benchmark/runner.rs`, `src/benchmark/mod.rs`

Replace `parse_workflow_context()` with proper YAML parsing using `serde_saphyr`:

```rust
fn load_workflow_config(&self) -> Result<Option<BenchmarkWorkflowConfig>> {
    let wf_path = match &self.config.workflow_file {
        Some(p) => p,
        None => return Ok(None),
    };
    let content = fs::read_to_string(wf_path)
        .with_context(|| format!("Failed to read workflow: {}", wf_path))?;
    let yaml: serde_json::Value = serde_saphyr::from_str(&content)
        .with_context(|| format!("Failed to parse YAML: {}", wf_path))?;

    let benchmark = yaml.get("benchmark").context("No benchmark section in YAML")?;
    let prompts = benchmark.get("prompts")
        .and_then(|p| p.as_sequence())
        .map(|seq| seq.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let model_list = yaml.get("model_list")
        .and_then(|m| m.as_sequence())
        .map(|seq| seq.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    Ok(Some(BenchmarkWorkflowConfig {
        prompts,
        max_tokens: benchmark.get("max_tokens").and_then(|v| v.as_u64()).unwrap_or(128) as usize,
        temperature: benchmark.get("temperature").and_then(|v| v.as_f64()).unwrap_or(0.7),
        top_p: benchmark.get("top_p").and_then(|v| v.as_f64()).unwrap_or(0.9),
        compare_modes: benchmark.get("compare_modes").and_then(|v| v.as_bool()).unwrap_or(false),
        modes: vec!["gpu".into(), "cpu".into()],
        gpu_config: GpuConfig { n_gpu_layers: 999 },
        cpu_config: CpuConfig { n_gpu_layers: 0 },
        model_list,
    }))
}
```

**Verification:** Unit test parsing benchmark-3-models.yml → verify prompts/max_tokens/compare_modes extracted.

### Step 2: Merge YAML Config with CLI Flags

**Files:** `src/benchmark/runner.rs`

In `run()`, after `load_workflow_config()`, merge with CLI-provided `BenchmarkConfig`:

```rust
fn merge_workflow_config(&self, wf: &BenchmarkWorkflowConfig) -> BenchmarkConfig {
    let mut config = self.config.clone();
    if config.prompts.is_empty() || config.prompts[0].contains("quick brown fox") {
        config.prompts = wf.prompts.clone();
    }
    if config.max_tokens == 64 {  // default
        config.max_tokens = wf.max_tokens;
    }
    if !config.compare_gpu_cpu {
        config.compare_gpu_cpu = wf.compare_modes;
    }
    // temperature and top_p are new fields — add to BenchmarkConfig
    config
}
```

Add `temperature: f64` and `top_p: f64` to `BenchmarkConfig`.

**Verification:** Run with `--workflow` only → uses YAML values. Run with `--workflow --max-tokens 256` → YAML prompts but 256 tokens.

### Step 3: Pass temperature/top_p Through to LLM

**Files:** `src/benchmark/runner.rs`, `src/client/types.rs`

Update `benchmark_single_model()` to accept and use temperature/top_p:

```rust
let request = ChatCompletionRequest {
    model: model_id.to_string(),
    messages: vec![ChatMessage { role: "user".into(), content: prompt.clone() }],
    max_tokens: self.config.max_tokens,
    temperature: self.config.temperature,
    top_p: self.config.top_p,
    stream: false,
};
```

**Verification:** Check .log files show temperature: 0.7 and top_p: 0.9 in request metadata.

### Step 4: Model List from YAML

**Files:** `src/benchmark/runner.rs`

When `workflow_file` is set and `model_list_file` is not, use YAML `model_list`:

```rust
if !wf.model_list.is_empty() && self.config.model_list_file.is_none() {
    models = wf.model_list.iter()
        .map(|p| {
            let path = Path::new(p);
            let id = path.file_name().map(|n| n.to_string_lossy().to_string())
                .unwrap_or(p.clone());
            (id, p.clone())
        })
        .collect();
}
```

**Verification:** Run with `--workflow` only (no `--model-list`) → uses YAML model paths.

### Step 5: Parse and Dispatch Agentic Steps

**Files:** `src/benchmark/runner.rs`

Parse the `agentic_workflow` array from YAML:

```rust
fn parse_workflow_steps(&self, yaml: &serde_json::Value) -> Vec<BenchmarkWorkflowStep> {
    yaml.get("agentic_workflow")
        .and_then(|w| w.as_sequence())
        .map(|steps| steps.iter().map(|s| self.parse_step(s)).collect())
        .unwrap_or_default()
}
```

Dispatch in `run()`:

```rust
for step in &workflow_steps {
    self.fire_hook(&step.when.before_step_starts, &step, "before_step_starts");
    match step.step_type() {
        BenchmarkStepType::BenchmarkPerformance => self.execute_benchmark_step(&step, &config).await?,
        BenchmarkStepType::RefineDocument => self.execute_refine_step(&step, &config).await?,
        BenchmarkStepType::GenerateSpeedupReport => self.execute_speedup_step(&step, &results).await?,
        BenchmarkStepType::GenerateReport => self.execute_report_step(&step, &results).await?,
    }
    self.fire_hook(&step.when.after_step_succeeds, &step, "after_step_succeeds");
}
```

**Verification:** Console log shows all 4 steps executing.

### Step 6: Stub Step Types 2-4

**Files:** `src/benchmark/runner.rs`

For refine_document (oscillate_abstraction): send each prompt through the LLM with summarize/expand cycle. For reports: aggregate results into JSON.

```rust
async fn execute_refine_step(&self, step: &BenchmarkWorkflowStep, config: &BenchmarkConfig) -> Result<()> {
    let oscillations = step.input.get("oscillations").and_then(|v| v.as_u64()).unwrap_or(3);
    // For each model, run oscillation loop
    info!("[benchmark] step refine_document: {} oscillations per model", oscillations);
    // Stub: log that step ran, produce placeholder output
    Ok(())
}
```

**Verification:** Step runs without error. Output placeholder exists.

### Step 7: Hook Execution

**Files:** `src/benchmark/runner.rs`

Implement `fire_hook()` for log/append_to/save_to actions:

```rust
fn fire_hook(&self, hook: &Option<HookAction>, step: &BenchmarkWorkflowStep, timing: &str) {
    if let Some(action) = hook {
        match action {
            HookAction::Log { to_file_path, event_fields } => {
                // Append timestamped line to file
            }
            HookAction::AppendTo { path } => {
                // Append output to file or variable
            }
            _ => {}
        }
    }
}
```

**Verification:** `workspace/logs/benchmark.log` shows step-by-step lifecycle entries.

### Step 8: Update Output Files with Step Context

**Files:** `src/benchmark/runner.rs`

Per-model .log files now include which step produced them:

```
--- Workflow Context ---
workflow_step: benchmark_performance (id: perf_bench)
step_index: 1 of 4
loop_iteration: 1 of 3 (model: Qwen3-4B-Instruct-2507-Q4_K_M)
gpu_mode: gpu
```

**Verification:** .log files reference correct step name and iteration.

## QA Criteria

From `docs/qa/phase-02/QA-WORKFLOW-EXECUTION-DISCREPANCY.md`:

| Criterion | How to Verify |
|-----------|---------------|
| Prompt Fidelity | .log shows YAML prompt text, not "quick brown fox" |
| Config Fidelity | max_tokens=128, temperature=0.7, top_p=0.9 in output |
| Mode Fidelity | compare_modes=true → 2 entries per model (GPU+CPU) |
| Step Execution | All 4 steps run, output files produced |
| Schema Compliance | YAML parses without error |
| Output Completeness | All expected files in output dir |

### Live Verification Procedure

1. Start server: `docker compose up -d`
2. Run: `cargo run --release --bin whitt --features client -- benchmark --url http://localhost:8081 --workflow docs/workflows/benchmarks/benchmark-3-models.yml --output-dir ./benchmark-attempts/3-model-run --output table`
3. Check .log files for YAML prompts
4. Check max_tokens in output
5. Verify GPU+CPU entries per model
6. Verify 4 steps executed
7. Run QA criteria from discrepancy doc

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| YAML parse failures | Add error messages showing which field failed |
| CLI+YAML merge conflicts | CLI always wins; document precedence |
| Step 2-4 stubs produce no real output | Mark as stubs; full implementation in Plan 2 |
| serde_saphyr panics on malformed YAML | Wrap in Result, add .context() |
| Model list paths differ (host vs Docker) | Use YAML paths for discovery, model ID for API |
| Temperature/top_p not in ChatCompletionRequest | Add fields to types.rs |

## Upstream Factors

| Factor | Status | Impact |
|--------|--------|--------|
| serde-saphyr crate | v0.0.24, early stage | May have edge cases with complex YAML |
| ChatCompletionRequest fields | Needs temperature/top_p added | Minor change to types.rs |
| UnifiedConfig parsing | Only parses top-level, not benchmark section | Need new parser, not extension |
| LoopExecutor | Generic closures, can wrap step execution | Low risk integration |
| TemplateInterpolator | Supports ${} and {{}} syntax | Ready for variable resolution |
