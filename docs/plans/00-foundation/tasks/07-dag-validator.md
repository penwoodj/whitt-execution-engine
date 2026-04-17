# Task 7: DAG Validator

**Goal:** Implement DAG validation and circular reference detection using topological sort.

**Estimated Time:** 4 hours

**Dependencies:** Task 6

**Files:**
- Modify: `src/validation/mod.rs` (implement DAG validation)
- Create: `tests/dag_test.rs` (DAG validation tests)

---

## Step 1: Implement DAG validation

Add to `src/validation/mod.rs`:

```rust
use crate::error::{Error, Result};
use crate::ir::{WorkflowIR, StepId};

/// Validate workflow DAG (no circular dependencies)
pub fn validate_dag(ir: &WorkflowIR) -> Result<()> {
    let mut visited = std::collections::HashSet::new();
    let mut visiting = std::collections::HashSet::new();
    let mut path = Vec::new();

    for step_id in ir.steps.keys() {
        if !visited.contains(step_id) {
            visit(step_id, ir, &mut visited, &mut visiting, &mut path)?;
        }
    }

    Ok(())
}

fn visit(
    step_id: &StepId,
    ir: &WorkflowIR,
    visited: &mut std::collections::HashSet<StepId>,
    visiting: &mut std::collections::HashSet<StepId>,
    path: &mut Vec<StepId>,
) -> Result<()> {
    if visiting.contains(step_id) {
        // Found circular dependency
        let cycle: Vec<String> = path.iter().map(|id| id.as_str().to_string()).collect();
        let cycle_str = cycle.join(" -> ");
        return Err(Error::circular_dependency(format!("{} -> {}", cycle_str, step_id.as_str())));
    }

    if visited.contains(step_id) {
        return Ok(());
    }

    visiting.insert(step_id.clone());
    path.push(step_id.clone());

    if let Some(step) = ir.steps.get(step_id) {
        for dep_id in &step.dependencies {
            visit(dep_id, ir, visited, visiting, path)?;
        }
    }

    visiting.remove(step_id);
    path.pop();
    visited.insert(step_id.clone());

    Ok(())
}

/// Check if step reference exists
pub fn validate_step_references(ir: &WorkflowIR) -> Result<()> {
    for step in ir.steps.values() {
        for dep_id in &step.dependencies {
            if !ir.steps.contains_key(dep_id) {
                return Err(Error::undefined_reference(format!("step.{}", dep_id.as_str())));
            }
        }
    }

    Ok(())
}
```

**Commit:** `feat: implement DAG validation with circular dependency detection`

---

## Step 2: Write DAG tests

Create `tests/dag_test.rs`:

```rust
use whitt_execution_engine::compiler::*;
use whitt_execution_engine::parser::parse_workflow;
use whitt_execution_engine::validation::*;

#[test]
fn test_validate_dag_linear() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let ir = compile(&spec).unwrap();

    let result = validate_dag(&ir);
    assert!(result.is_ok());
}

#[test]
fn test_validate_dag_parallel() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    let ir = compile(&spec).unwrap();

    let result = validate_dag(&ir);
    assert!(result.is_ok());
}

#[test]
fn test_circular_dependency() {
    // Create IR with circular dependency manually
    use whitt_execution_engine::ir::*;
    let mut steps = std::collections::HashMap::new();

    let id1 = StepId::new("step_1");
    let id2 = StepId::new("step_2");
    let id3 = StepId::new("step_3");

    steps.insert(id1.clone(), StepIR {
        id: id1.clone(),
        step_type: StepTypeIR::Agent,
        model_id: None,
        prompt: String::new(),
        dependencies: vec![id2.clone()],
        output_config: OutputConfigIR {
            save_to_variable: None,
            format: OutputFormatIR::Json,
            file_path: None,
        },
        retry_config: RetryConfigIR {
            max_attempts: 3,
            backoff: BackoffStrategyIR::Fixed { delay_ms: 1000 },
        },
    });

    steps.insert(id2.clone(), StepIR {
        id: id2.clone(),
        step_type: StepTypeIR::Agent,
        model_id: None,
        prompt: String::new(),
        dependencies: vec![id3.clone()],
        output_config: OutputConfigIR {
            save_to_variable: None,
            format: OutputFormatIR::Json,
            file_path: None,
        },
        retry_config: RetryConfigIR {
            max_attempts: 3,
            backoff: BackoffStrategyIR::Fixed { delay_ms: 1000 },
        },
    });

    steps.insert(id3.clone(), StepIR {
        id: id3.clone(),
        step_type: StepTypeIR::Agent,
        model_id: None,
        prompt: String::new(),
        dependencies: vec![id1.clone()],
        output_config: OutputConfigIR {
            save_to_variable: None,
            format: OutputFormatIR::Json,
            file_path: None,
        },
        retry_config: RetryConfigIR {
            max_attempts: 3,
            backoff: BackoffStrategyIR::Fixed { delay_ms: 1000 },
        },
    });

    let ir = WorkflowIR {
        id: WorkflowId::new("test"),
        name: "Test".to_string(),
        version: "1.0.0".to_string(),
        models: std::collections::HashMap::new(),
        steps,
        execution_mode: ExecutionMode::Serial,
        workspace_path: "/workspace".to_string(),
    };

    let result = validate_dag(&ir);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Circular dependency"));
}
```

**Commit:** `test: add DAG validation tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test dag_test

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

# 2. All DAG tests pass
cargo test dag_test
# Expected: test result: ok. X passed

# 3. Circular dependencies are detected
```

**Checkpoint Criteria:**
- ✅ DAG validation detects circular dependencies
- ✅ Topological sort algorithm implemented
- ✅ Error messages include cycle path
- ✅ DAG tests created and passing
- ✅ Linear and parallel workflows validate correctly

**Anti-Drift Check:** Verify task 7 implements ONLY DAG validation. No policy compilation or persistence yet.

**Next:** Proceed to Task 8 (Policy Compiler)
