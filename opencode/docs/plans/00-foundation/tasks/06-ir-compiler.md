# Task 6: IR Compiler

**Goal:** Implement WorkflowSpec→WorkflowIR compilation pipeline with type checking and validation.

**Estimated Time:** 8 hours

**Dependencies:** Task 3, Task 4, Task 5

**Files:**
- Modify: `src/compiler/mod.rs` (implement compilation pipeline)
- Create: `tests/compiler_test.rs` (compiler tests)

---

## Step 1: Implement compiler

Add to `src/compiler/mod.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::{WorkflowSpec, ModelConfig, Step, StepType, OutputFormat, BackoffStrategy, ModelProvider};
use crate::ir::*;
use crate::interpolation::{ParseContext, interpolate_parse};
use std::collections::HashMap;

/// Compile WorkflowSpec to WorkflowIR
pub fn compile(spec: &WorkflowSpec) -> Result<WorkflowIR> {
    let ctx = ParseContext::from_spec(spec);

    let mut models = HashMap::new();
    for (name, model_spec) in spec.models.models.iter() {
        models.insert(
            ModelId::new(name),
            compile_model(model_spec)?,
        );
    }

    let mut steps = HashMap::new();
    if let Some(agentic) = &spec.agentic_workflow {
        for (step_id, step_spec) in agentic_workflow.steps.iter() {
            steps.insert(
                StepId::new(step_id),
                compile_step(step_spec, &ctx)?,
            );
        }
    }

    Ok(WorkflowIR {
        id: WorkflowId::new(spec.identification.workflow_id.clone()),
        name: spec.identification.name.clone(),
        version: spec.identification.version.clone(),
        models,
        steps,
        execution_mode: compile_execution_mode(&spec.execution),
        workspace_path: spec.workspace.root_path.clone(),
    })
}

fn compile_model(spec: &ModelConfig) -> Result<ModelIR> {
    let provider = match spec.host.provider_type {
        ModelProvider::LmStudio => ModelProviderIR::LmStudio {
            host: "localhost".to_string(),
            port: 1234,
        },
        ModelProvider::Ollama => ModelProviderIR::Ollama {
            base_url: "http://localhost:11434".to_string(),
        },
        ModelProvider::LlamaCppWithVulkan => ModelProviderIR::LlamaCpp {
            model_path: "./models/model.gguf".to_string(),
            backend: "vulkan".to_string(),
        },
    };

    let max_resources = compile_resource_limits(&spec.max_allowed)?;
    let min_resources = compile_resource_limits(&spec.min_allowed)?;

    Ok(ModelIR {
        id: ModelId::new(spec.name.clone()),
        provider,
        max_resources,
        min_resources,
    })
}

fn compile_resource_limits(limits: &ResourceLimits) -> Result<ResourceAllocation> {
    Ok(ResourceAllocation {
        ram_mb: compile_resource_limit(&limits.ram)?,
        vram_mb: compile_resource_limit(&limits.vram)?,
        cpu_percent: 50.0,
        gpu_percent: 80.0,
    })
}

fn compile_resource_limit(limit: &ResourceLimit) -> Result<u64> {
    match limit {
        ResourceLimit::Percentage(p) => {
            // Assume 16GB RAM for percentage conversion
            Ok((16.0 * 1024.0 * p) as u64)
        }
        ResourceLimit::Absolute { value, unit } => {
            match unit.as_str() {
                "GB" => Ok(value * 1024),
                "MB" => Ok(*value),
                _ => Err(Error::type_error("GB/MB", unit, "resource limit")),
            }
        }
    }
}

fn compile_step(spec: &Step, ctx: &ParseContext) -> Result<StepIR> {
    let model_id = if let Some(model_ref) = &spec.model {
        if let Some(start) = model_ref.find("${models.") {
            let end = model_ref.find('}').unwrap();
            let model_name = &model_ref[start + 10..end];
            Some(ModelId::new(model_name))
        } else {
            None
        }
    } else {
        None
    };

    let prompt = spec.prompt.clone().unwrap_or_default();
    let prompt = interpolate_parse(&prompt, ctx)?;

    let dependencies = spec.depends_on
        .as_ref()
        .map(|deps| deps.iter().map(|d| StepId::new(d)).collect())
        .unwrap_or_default();

    Ok(StepIR {
        id: StepId::new(spec.step.clone()),
        step_type: compile_step_type(&spec.step_type)?,
        model_id,
        prompt,
        dependencies,
        output_config: compile_output_config(spec.output.as_ref())?,
        retry_config: compile_retry_config(spec.retry.as_ref())?,
    })
}

fn compile_step_type(step_type: &StepType) -> Result<StepTypeIR> {
    Ok(match step_type {
        StepType::Agent => StepTypeIR::Agent,
        StepType::Tool => StepTypeIR::Tool {
            tool_name: "default".to_string(),
        },
        StepType::SubWorkflow => StepTypeIR::SubWorkflow {
            workflow_id: WorkflowId::new("default"),
        },
        StepType::Control => StepTypeIR::Control {
            control_type: "default".to_string(),
        },
        StepType::Loop => StepTypeIR::Agent, // Loops compile to agent steps
    })
}

fn compile_output_config(output: Option<&OutputConfig>) -> Result<OutputConfigIR> {
    let output = output.unwrap();
    Ok(OutputConfigIR {
        save_to_variable: output.save_to.clone(),
        format: match output.format {
            OutputFormat::Json => OutputFormatIR::Json,
            OutputFormat::Yaml => OutputFormatIR::Yaml,
            OutputFormat::Text => OutputFormatIR::Text,
            OutputFormat::Markdown => OutputFormatIR::Text,
        },
        file_path: output.file_output.as_ref().map(|f| f.path.clone()),
    })
}

fn compile_retry_config(retry: Option<&RetryConfig>) -> Result<RetryConfigIR> {
    let retry = retry.unwrap();
    Ok(RetryConfigIR {
        max_attempts: retry.max_attempts,
        backoff: match retry.backoff {
            BackoffStrategy::Exponential => BackoffStrategyIR::Exponential {
                base_ms: retry.base_ms.unwrap_or(1000),
                max_ms: retry.max_ms.unwrap_or(30000),
            },
            BackoffStrategy::Linear => BackoffStrategyIR::Linear {
                delay_ms: retry.delay_ms,
            },
            BackoffStrategy::Fixed => BackoffStrategyIR::Fixed {
                delay_ms: retry.delay_ms,
            },
        },
    })
}

fn compile_execution_mode(spec: &WorkflowExecutionStrategy) -> ExecutionMode {
    match spec.processing {
        ProcessingMode::Serial => ExecutionMode::Serial,
        ProcessingMode::Parallel => ExecutionMode::Parallel {
            max_workers: spec.parallel.as_ref()
                .and_then(|p| Some(p.max_threads))
                .unwrap_or(4),
        },
        ProcessingMode::Hybrid => ExecutionMode::Hybrid {
            parallel_threshold: 2,
        },
    }
}
```

**Commit:** `feat: implement WorkflowSpec→WorkflowIR compiler`

---

## Step 2: Write compiler tests

Create `tests/compiler_test.rs`:

```rust
use yaml_to_rust_agentsdk::compiler::*;
use yaml_to_rust_agentsdk::parser::parse_workflow;

#[test]
fn test_compile_minimal_workflow() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let ir = compile(&spec).unwrap();

    assert_eq!(ir.id.as_str(), "minimal_workflow");
    assert_eq!(ir.name, "Minimal Workflow");
    assert!(ir.models.contains_key(&crate::ir::ModelId::new("primary")));
}

#[test]
fn test_compile_complex_workflow() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    let ir = compile(&spec).unwrap();

    assert_eq!(ir.id.as_str(), "complex_workflow");
    assert_eq!(ir.version, "2.0.0");
    assert_eq!(ir.models.len(), 2);
    assert_eq!(ir.steps.len(), 2);
}

#[test]
fn test_compile_with_interpolation() {
    let yaml = r#"
workflow_id: test
name: "Test"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio

workspace:
  root_path: /workspace/test

features:
  categories:
    - ModelConfiguration
  compatibility_checks: true

execution:
  processing: serial
  load_unload: one_at_a_time

tool_permissions:
  file_operations:
    read:
      # presence = enabled
    write:
      disabled: true
    delete:
      disabled: true
  web_operations:
    fetch:
      disabled: true
    scrape:
      disabled: true
  shell_operations:
    exec:
      disabled: true

logging:
  # presence = enabled
  default: info
  output:
    console:
      # presence = enabled
    file:
      disabled: true
  errors:
    log_parsing_errors: true

agentic_workflow:
  steps:
    step_1:
      step: test_step
      model: "${models.primary}"
      prompt: "Use model ${models.primary.name}"
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    let ir = compile(&spec).unwrap();

    let step = ir.steps.get(&crate::ir::StepId::new("step_1")).unwrap();
    assert!(step.model_id.is_some());
}
```

**Commit:** `test: add compiler tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test compiler_test

# Expected output:
# test result: ok. X passed in Y.ZZs
```

**Commit:** `fix: resolve any test failures`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. All compiler tests pass
cargo test compiler_test
# Expected: test result: ok. X passed

# 3. End-to-end compilation works
# Parse → Compile should work for all fixtures
```

**Checkpoint Criteria:**
- ✅ WorkflowSpec→WorkflowIR pipeline works end-to-end
- ✅ Type checking catches schema violations
- ✅ Interpolation happens during compilation
- ✅ Compiler tests created and passing
- ✅ Error messages include context
- ✅ All test fixtures compile successfully

**Anti-Drift Check:** Verify task 6 implements ONLY compilation. No DAG validation, policy compilation, or persistence yet.

**Next:** Proceed to Task 7 (DAG Validator)
