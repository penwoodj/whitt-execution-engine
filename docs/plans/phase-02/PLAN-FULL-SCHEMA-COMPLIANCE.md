# Plan: Full Compliance with unified-workflow-schema.yml

## Goal

Achieve full structural compliance with the authoritative schema at `docs/schema/unified-workflow-schema.yml` (805 lines). All benchmark YAML files rewrite to match. Rust types cover every schema section. A validator checks compliance. The workflow engine executes compliant workflows end-to-end.

## Success Criteria

1. All 4 benchmark YAML files (`benchmark-3/5/15/50-models.yml`) pass schema validation
2. Schema validator rejects files missing required sections (`providers`, `workspace`, `workflow_execution_strategy`)
3. `steps:` map format (not `- step:` array) used throughout
4. All step types from schema have corresponding Rust structs: AgentStep, ToolStep, ControlFlowStep, SubWorkflowStep, LoopStep
5. Hook types complete: log, append_to, save_to, notify, bookmark, fail, skip_*, gwt
6. Variable interpolation works: `${models.xxx}` structural, `{{step.xxx.output}}` runtime
7. `cargo test --all-features` passes with schema validation tests
8. Live execution of benchmark-3-models.yml produces all expected outputs
9. QA criteria from QA-WORKFLOW-EXECUTION-DISCREPANCY.md all pass

## Schema Compliance Matrix

| Schema Section | Current Status | Target | Effort |
|---|---|---|---|
| `workflow_id` | ✅ Present | ✅ Compliant | None |
| `name` | ✅ Present | ✅ Compliant | None |
| `description` | ✅ Present | ✅ Compliant | None |
| `min_schema_version` | ⚠️ Non-schema key | Use `version` | Low |
| `providers` | ❌ Missing | Required: lmstudio config | Medium |
| `models` | ⚠️ Partial (`models.primary`) | Full model definition with host/ram/execution | Medium |
| `agentic_workflow.steps` | ❌ Uses `- step:` array | `steps:` map with named keys | High |
| `workflow_execution_strategy` | ❌ Missing | Required: memory, timeout, error_handling | Medium |
| `tool_permissions` | ❌ Missing | Optional but recommended | Low |
| `workspace` | ❌ Missing | Required: root_path, directories | Low |
| `memory` | ❌ Missing | Optional (RAG) | Deferred |
| `benchmark` (custom) | ⚠️ Non-schema key | Move into step definitions | High |
| `model_list` | ⚠️ Non-schema key | Move into `workflow_execution_strategy` or step input | Medium |
| `execution` | ⚠️ Non-schema key | Move into `workflow_execution_strategy` | Low |
| `logging` | ⚠️ Non-schema key | Move into hooks + workspace | Low |
| Hook timing (6 points) | ❌ Only 2 used | All 6: before_step_starts, during_step_streaming, after_step_succeeds, after_step_fails, after_all_retries_exhausted, before_gwt/after_gwt | Medium |
| Step type inference | ❌ Explicit `type:` | Infer from keys present | Medium |
| Variable interpolation | ⚠️ Partial | Full `${}` + `{{}}` with nested path resolution | High |
| Dependency resolution | ❌ Not implemented | `requires:` / `depends_on:` with topological sort | High |

## Architecture

### New Module: `src/workflow/`

```
src/workflow/
├── mod.rs              — WorkflowEngine, public API
├── schema.rs           — Rust types for all schema sections
├── parser.rs           — YAML → WorkflowDef parsing
├── validator.rs        — Schema compliance checker
├── executor.rs         — Step dispatch and lifecycle
├── hooks.rs            — Hook execution (log, append_to, save_to, etc.)
├── variables.rs        — Variable scoping and interpolation
└── dependency.rs       — Topological sort for step ordering
```

### Rust Types (schema.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDef {
    pub workflow_id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub providers: HashMap<String, ProviderDef>,
    #[serde(default)]
    pub models: ModelsDef,
    #[serde(default)]
    pub agentic_workflow: AgenticWorkflowDef,
    #[serde(default)]
    pub workflow_execution_strategy: ExecutionStrategyDef,
    #[serde(default)]
    pub tool_permissions: ToolPermissionsDef,
    #[serde(default)]
    pub workspace: WorkspaceDef,
    #[serde(default)]
    pub memory: Option<MemoryDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgenticWorkflowDef {
    #[serde(default)]
    pub retry: Option<RetryDef>,
    #[serde(default)]
    pub when: Option<HooksDef>,
    pub steps: HashMap<String, StepDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDef {
    pub step: Option<String>,
    pub generative_entity: Option<String>,
    pub prompt: Option<String>,
    pub tool: Option<String>,
    pub input: Option<HashMap<String, serde_json::Value>>,
    pub r#loop: Option<LoopDef>,
    pub sub_workflow: Option<String>,
    pub requires: Option<Vec<serde_json::Value>>,
    pub depends_on: Option<Vec<String>>,
    pub retry: Option<RetryDef>,
    pub when: Option<HooksDef>,
    pub output: Option<OutputDef>,
    pub model_overrides: Option<HashMap<String, serde_json::Value>>,
    pub user_input: Option<UserInputDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoopDef {
    Count { count: CountLoopDef },
    Validation { validation: ValidationLoopDef },
    ForEach { foreach: ForEachLoopDef },
    Combined { count: CountLoopDef, validation: ValidationLoopDef },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksDef {
    pub before_step_starts: Option<serde_json::Value>,
    pub during_step_streaming: Option<serde_json::Value>,
    pub after_step_succeeds: Option<serde_json::Value>,
    pub after_step_fails: Option<serde_json::Value>,
    pub after_all_retries_exhausted: Option<serde_json::Value>,
    pub before_gwt_evaluates: Option<serde_json::Value>,
    pub after_gwt_evaluates: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookActionDef {
    pub log: Option<LogActionDef>,
    pub append_to: Option<serde_json::Value>,
    pub save_to: Option<serde_json::Value>,
    pub bookmark: Option<bool>,
    pub notify: Option<NotifyActionDef>,
    pub fail: Option<String>,
    pub skip_step: Option<bool>,
    pub skip_loop: Option<bool>,
    pub skip_remaining: Option<bool>,
    pub gwt: Option<Vec<GwtClauseDef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogActionDef {
    pub to_file_path: String,
    #[serde(default)]
    pub event_fields: Vec<String>,
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_format")]
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDef {
    pub config: Option<ProviderConfigDef>,
    pub config_file: Option<String>,
    pub hosting: Option<HostingDef>,
    pub requests: Option<RequestsDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDef {
    #[serde(default = "default_workspace_root")]
    pub root_path: String,
    #[serde(default)]
    pub directories: WorkspaceDirectoriesDef,
    #[serde(default)]
    pub permissions: Option<WorkspacePermissionsDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStrategyDef {
    #[serde(default = "default_load_unload")]
    pub load_unload: String,
    #[serde(default)]
    pub memory: Option<MemoryStrategyDef>,
    #[serde(default)]
    pub timeout: Option<TimeoutDef>,
    #[serde(default)]
    pub error_handling: Option<ErrorHandlingDef>,
    #[serde(default)]
    pub checkpointing: Option<CheckpointingDef>,
    #[serde(default)]
    pub dependency_resolution: Option<DependencyResolutionDef>,
}

// ... remaining types for RetryDef, GwtClauseDef, etc.
```

### Benchmark YAML Rewrite (Schema-Compliant)

The benchmark-3-models.yml should become:

```yaml
workflow_id: benchmark_3_models
name: "3-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 3 diverse models"
version: "2.0.0"

providers:
  lmstudio:
    config:
      host: localhost
      port: 8081
      connection_timeout_secs: 30
    hosting:
      max_concurrent_models: 1
      gpu_allocation:
        vram_per_model_mb: 4096
      cpu_fallback:
        cpu_cores_per_model: 2

models:
  default_router: automatic
  benchmark-model:
    name: "Benchmark Target"
    host:
      type: lmstudio
      connection_settings: {}
    execution:
      timeout:
        load_into_memory: 60s
        total_time_to_response: 5m

workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: one_at_a_time
      unload_unused: true
      swap_timeout_secs: 30
  timeout:
    total: 1h
    per_operation:
      step: 30m
  error_handling:
    default_action:
      type: retry
    retry:
      max_attempts_per_step: 3

workspace:
  root_path: "./workspace"
  directories:
    output: "./workspace/output"
    logs: "./workspace/logs"
    checkpoints: "./workspace/checkpoints"
    metrics: "./workspace/metrics"

agentic_workflow:
  retry:
    max_attempts: 3
    backoff: linear
    initial_delay: "1s"

  when:
    after_step_fails:
      log:
        to_file_path: "./workspace/logs/benchmark-errors.log"
        event_fields: [step_name, error_type, error_message]
        level: error

  steps:
    benchmark_performance:
      step: benchmark_performance
      generative_entity: "${models.benchmark-model}"
      r#loop:
        count:
          max_iterations: 3
          iteration_variable: current_model
      input:
        prompts:
          - "Explain concept of recursion in programming, with a practical example."
          - "Write a Rust function that finds the longest increasing subsequence in a vector."
          - "Analyze tradeoffs between microservices and monolithic architectures."
        max_tokens: 128
        temperature: 0.7
        top_p: 0.9
        compare_modes: true
        gpu_n_layers: 999
        cpu_n_layers: 0
        model_paths:
          - /run/media/jon/data/models/lmstudio-community/Qwen3-4B-Instruct-2507-GGUF/Qwen3-4B-Instruct-2507-Q4_K_M.gguf
          - /run/media/jon/data/models/lmstudio-community/Phi-4-mini-reasoning-GGUF/Phi-4-mini-reasoning-Q4_K_M.gguf
          - /run/media/jon/data/models/hugging-quants/Llama-3.2-1B-Instruct-Q8_0-GGUF/llama-3.2-1b-instruct-q8_0.gguf
      when:
        before_step_starts:
          log:
            to_file_path: "./workspace/logs/benchmark.log"
            event_fields: [step_name, loop_iteration, current_model]
        after_step_succeeds:
          - append_to:
              - "./workspace/output/benchmark_results.yaml"
              - benchmark_collection
          - log:
              to_file_path: "./workspace/logs/completed.log"
              event_fields: [step_name, duration_ms, tokens_per_second]
        after_loop_iteration_fails:
          log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]
      output:
        save_to:
          - benchmark_results
          - "./workspace/output/benchmark_report.json"

    refine_document:
      step: refine_document
      generative_entity: "${models.benchmark-model}"
      requires: [benchmark_performance]
      r#loop:
        count:
          max_iterations: 3
          iteration_variable: current_model
      input:
        file_path: "./docs/plans/multi-model-benchmark/01-IMPLEMENTATION-PLAN.md"
        chunk_size: 4000
        oscillations: 3
        overlap: 200
      output:
        save_to:
          - refined_plans
          - "./workspace/output/refined_plan_{{loop.current_model}}.md"

    generate_speedup_report:
      step: generate_speedup_report
      requires: [benchmark_performance]
      input:
        benchmark_results: "{{step.benchmark_performance.output}}"
        comparison_mode: gpu_vs_cpu
      output:
        save_to:
          - speedup_report
          - "./workspace/output/speedup_report.json"

    generate_report:
      step: generate_report
      requires: [benchmark_performance, refine_document, generate_speedup_report]
      input:
        benchmark_results: "{{step.benchmark_performance.output}}"
        refined_plans: "{{step.refine_document.output}}"
        speedup_data: "{{step.generate_speedup_report.output}}"
      output:
        save_to:
          - final_report
          - "./workspace/output/benchmark_report.json"
```

## Implementation Steps

### Step 1: Create `src/workflow/schema.rs` — Rust Type Definitions

**Files:** `src/workflow/mod.rs`, `src/workflow/schema.rs`

Define all structs listed in Architecture section. Use `serde` derive macros. Use `#[serde(default)]` for optional sections. Handle `r#loop` escaping for reserved word.

**Verification:** `cargo check` passes. Unit test: deserialize schema-compliant YAML string into `WorkflowDef`.

### Step 2: Create `src/workflow/parser.rs` — YAML Parsing

**Files:** `src/workflow/parser.rs`

```rust
pub fn parse_workflow(path: &Path) -> Result<WorkflowDef> {
    let content = fs::read_to_string(path)?;
    let def: WorkflowDef = serde_saphyr::from_str(&content)
        .with_context(|| format!("Failed to parse workflow: {}", path.display()))?;
    Ok(def)
}
```

**Verification:** Parse all 4 benchmark YAML files without error.

### Step 3: Create `src/workflow/validator.rs` — Schema Compliance Checker

**Files:** `src/workflow/validator.rs`

```rust
pub struct ValidationResult {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

pub fn validate_workflow(def: &WorkflowDef) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if def.workflow_id.is_empty() {
        errors.push(ValidationIssue::required("workflow_id"));
    }
    if def.providers.is_empty() {
        warnings.push(ValidationIssue::recommended("providers"));
    }
    if def.workspace.root_path.is_empty() {
        errors.push(ValidationIssue::required("workspace.root_path"));
    }
    if def.agentic_workflow.steps.is_empty() {
        errors.push(ValidationIssue::required("agentic_workflow.steps"));
    }

    for (name, step) in &def.agentic_workflow.steps {
        validate_step(name, step, &mut errors, &mut warnings);
    }

    ValidationResult { errors, warnings }
}
```

**Verification:** Non-compliant YAML → errors reported. Compliant YAML → passes.

### Step 4: Rewrite Benchmark YAML Files

**Files:** `docs/workflows/benchmarks/benchmark-3-models.yml`, `benchmark-5-models.yml`, `benchmark-15-models.yml`, `benchmark-50-models.yml`

Rewrite using the schema-compliant format shown above. Key changes:
- `- step:` array → `steps:` map with named keys
- `model_list` → `input.model_paths` inside benchmark_performance step
- `benchmark` section → distributed into step inputs
- `execution` → `workflow_execution_strategy`
- `logging` → hooks under `when:`
- Add `providers`, `workspace` sections
- Remove `min_schema_version` (use `version`)

**Verification:** All 4 files pass `validate_workflow()`.

### Step 5: Update YAML Generator

**Files:** `src/benchmark/yaml_generator.rs`

Update `generate_benchmark_yaml()` and all variants to produce schema-compliant output. This ensures future generated YAMLs are also compliant.

**Verification:** Run generator → output passes validator.

### Step 6: Create `src/workflow/executor.rs` — Step Dispatch

**Files:** `src/workflow/executor.rs`

Implement `WorkflowEngine` that takes a `WorkflowDef` and executes steps:

```rust
pub struct WorkflowEngine {
    def: WorkflowDef,
    state: WorkflowState,
    interpolator: TemplateInterpolator,
}

impl WorkflowEngine {
    pub async fn execute(&mut self) -> Result<WorkflowResult> {
        let ordered_steps = self.resolve_execution_order()?;
        for step_name in &ordered_steps {
            let step = self.def.agentic_workflow.steps.get(step_name)
                .context(format!("Step {} not found", step_name))?;
            self.execute_step(step_name, step).await?;
        }
        Ok(self.state.into_result())
    }

    fn resolve_execution_order(&self) -> Result<Vec<String>> {
        // Topological sort based on requires/depends_on
        dependency::topological_sort(&self.def.agentic_workflow.steps)
    }

    async fn execute_step(&mut self, name: &str, step: &StepDef) -> Result<()> {
        self.fire_hooks(&step.when.before_step_starts, name, "before_step_starts")?;
        let step_type = infer_step_type(step);
        match step_type {
            StepType::Agent => self.execute_agent_step(name, step).await,
            StepType::Tool => self.execute_tool_step(name, step).await,
            StepType::Loop => self.execute_loop_step(name, step).await,
            StepType::ControlFlow => self.execute_control_step(name, step).await,
            StepType::SubWorkflow => self.execute_sub_workflow_step(name, step).await,
        }?;
        self.fire_hooks(&step.when.after_step_succeeds, name, "after_step_succeeds")?;
        Ok(())
    }
}

fn infer_step_type(step: &StepDef) -> StepType {
    if step.r#loop.is_some() { return StepType::Loop; }
    if step.sub_workflow.is_some() { return StepType::SubWorkflow; }
    if step.tool.is_some() { return StepType::Tool; }
    if step.generative_entity.is_some() && step.prompt.is_some() { return StepType::Agent; }
    if step.when.as_ref().and_then(|w| w.gwt.as_ref()).is_some() { return StepType::ControlFlow; }
    StepType::Agent // default
}
```

**Verification:** Execute benchmark-3-models.yml end-to-end.

### Step 7: Create `src/workflow/dependency.rs` — Topological Sort

**Files:** `src/workflow/dependency.rs`

Use petgraph or manual Kahn's algorithm:

```rust
pub fn topological_sort(steps: &HashMap<String, StepDef>) -> Result<Vec<String>> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (name, step) in steps {
        in_degree.entry(name.as_str()).or_insert(0);
        if let Some(deps) = &step.requires {
            for dep in deps {
                let dep_name = extract_step_name(dep);
                graph.entry(dep_name).or_default().push(name.as_str());
                *in_degree.entry(name.as_str()).or_insert(0) += 1;
            }
        }
    }

    // Kahn's algorithm
    let mut queue: VecDeque<&str> = in_degree.iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&name, _)| name)
        .collect();
    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.to_string());
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 { queue.push_back(neighbor); }
            }
        }
    }

    if result.len() != steps.len() {
        anyhow::bail!("Circular dependency detected in workflow steps");
    }
    Ok(result)
}
```

**Verification:** Unit tests with circular deps → error. Linear deps → correct order.

### Step 8: Create `src/workflow/hooks.rs` — Lifecycle Hook Execution

**Files:** `src/workflow/hooks.rs`

Implement all hook actions from schema:

| Action | Implementation |
|--------|---------------|
| `log` | `OpenOptions::new().append(true).open(to_file_path)` with formatted event_fields |
| `append_to` | Append YAML/text to file path or accumulate in variable |
| `save_to` | Write to file and/or set variable |
| `bookmark` | Create checkpoint via WorkflowState persistence |
| `notify` | Log notification (MVP: just log) |
| `fail` | Return error with message |
| `skip_step` | Return special Skip result |
| `skip_loop` | Break loop iteration |
| `skip_remaining` | Set workflow flag to stop |
| `gwt` | Evaluate given/when/then clauses and route |

**Verification:** Unit tests for each action type.

### Step 9: Create `src/workflow/variables.rs` — Variable Scoping

**Files:** `src/workflow/variables.rs`

Extend existing `TemplateInterpolator` with workflow-specific scopes:

```rust
pub struct WorkflowVariables {
    workflow_level: HashMap<String, String>,
    step_outputs: HashMap<String, StepOutput>,
    loop_variables: HashMap<String, String>,
    inputs: HashMap<String, String>,
}

impl WorkflowVariables {
    pub fn resolve_structural(&self, template: &str) -> String {
        // ${models.xxx}, ${workspace.xxx}, ${workflow.xxx}
    }

    pub fn resolve_runtime(&self, template: &str) -> String {
        // {{step.xxx.output}}, {{loop.xxx}}, {{inputs.xxx}}, {{now}}, {{workflow_id}}
    }
}
```

**Verification:** Unit tests for nested path resolution: `step.benchmark_performance.output.tokens_per_second`.

### Step 10: Integration — Connect WorkflowEngine to CLI

**Files:** `src/bin/whitt.rs`

Add workflow execution command:

```rust
Commands::Benchmark { workflow, .. } => {
    if let Some(wf_path) = workflow {
        let def = parse_workflow(Path::new(&wf_path))?;
        let validation = validate_workflow(&def);
        if !validation.errors.is_empty() {
            for err in &validation.errors { eprintln!("ERROR: {}", err); }
            anyhow::bail!("Workflow validation failed");
        }
        let mut engine = WorkflowEngine::new(def);
        let result = engine.execute().await?;
    }
}
```

**Verification:** Full end-to-end run with live server.

## QA Criteria

### Schema Validation Tests

```rust
#[test]
fn test_benchmark_3_models_compliant() {
    let def = parse_workflow(Path::new("docs/workflows/benchmarks/benchmark-3-models.yml")).unwrap();
    let result = validate_workflow(&def);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
}

#[test]
fn test_rejects_missing_providers() {
    let yaml = "workflow_id: test\nname: test\nagentic_workflow:\n  steps: {}";
    let def: WorkflowDef = serde_saphyr::from_str(yaml).unwrap();
    let result = validate_workflow(&def);
    assert!(result.warnings.iter().any(|w| w.message.contains("providers")));
}

#[test]
fn test_rejects_array_steps() {
    let yaml = "workflow_id: test\nagentic_workflow:\n  steps:\n    - step: test";
    let result = serde_saphyr::from_str::<WorkflowDef>(yaml);
    assert!(result.is_err() || /* steps is empty */);
}

#[test]
fn test_step_type_inference() {
    let step = StepDef { generative_entity: Some("x".into()), prompt: Some("y".into()), .. };
    assert_eq!(infer_step_type(&step), StepType::Agent);

    let step = StepDef { tool: Some("file_read".into()), .. };
    assert_eq!(infer_step_type(&step), StepType::Tool);
}
```

### Live Verification Procedure

1. Start server: `docker compose up -d`
2. Run: `cargo run --release --bin whitt --features client -- benchmark --url http://localhost:8081 --workflow docs/workflows/benchmarks/benchmark-3-models.yml --output-dir ./benchmark-attempts/schema-test --output table`
3. Verify all QA criteria from QA-WORKFLOW-EXECUTION-DISCREPANCY.md
4. Verify schema validation passes on the workflow file
5. Verify all 4 steps execute
6. Verify output files match schema expectations

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| `r#loop` serde escaping | Parse failures | Test with real YAML early |
| Complex hook merging (L2+L3) | Incorrect behavior | Start simple, add merging later |
| petgraph dependency overhead | Build time | Use manual Kahn's algorithm instead |
| serde_saphyr maturity | Edge case failures | Fallback to manual parsing for known sections |
| Breaking existing tests | CI failures | Keep old parser for backward compat during migration |
| Large YAML files (50 models) | Parse performance | Benchmark; lazy evaluation if needed |

## Upstream Factors

| Factor | Status | Impact |
|--------|--------|--------|
| `serde-saphyr` v0.0.24 | Early release, limited docs | May need workarounds for complex deserialization |
| `serde_json::Value` | Stable | Used for flexible fields (input, output) |
| `petgraph` | Stable, mature | Optional — can use manual topo sort |
| Existing `UnifiedConfig` | Parses partial schema | Extend or replace — not breaking change |
| `TemplateInterpolator` | Works for ${}/{{}} | Extend with workflow scopes |
| `LoopExecutor` | Generic closures | Wrap with step execution context |
| `WorkflowState` persistence | JSON + SQLite backends | Reuse for checkpointing |
| 50 example workflows | In docs/workflows/examples/ | Must parse after schema rewrite |

## Dependencies on Plan 1

Plan 1 (YAML-Driven Benchmark) implements the minimum viable parsing and execution. Plan 2 builds on it:
- Plan 1 creates the parsing and step dispatch in `src/benchmark/`
- Plan 2 moves the engine to `src/workflow/` as a general-purpose module
- Plan 1 uses crude parsing; Plan 2 uses full serde deserialization
- Plan 1 handles benchmark steps only; Plan 2 handles all step types
- Both plans share the same QA criteria and verification procedure

## Implementation Order

1. Plan 1 Steps 1-4 (YAML parsing, config merge, model list)
2. Plan 2 Steps 1-3 (schema types, parser, validator)
3. Plan 1 Steps 5-8 (step dispatch, stubs, hooks, output)
4. Plan 2 Step 4 (rewrite YAML files to schema-compliant format)
5. Plan 2 Steps 5-9 (full engine, dependency sort, hooks, variables)
6. Plan 2 Step 10 (CLI integration)
7. Full QA cycle with live system
