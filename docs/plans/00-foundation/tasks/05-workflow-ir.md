# Task 5: Workflow IR

**Goal:** Define WorkflowIR typed internal representation with strict type checking.

**Estimated Time:** 6 hours

**Dependencies:** Task 2

**Files:**
- Create: `src/ir/mod.rs` (define WorkflowIR types)
- Create: `tests/ir_test.rs` (IR type tests)

---

## Step 1: Define WorkflowIR types

Create `src/ir/mod.rs`:

```rust
use crate::error::{Error, Result};
use std::collections::HashMap;

/// WorkflowIR - Internal typed representation for execution
#[derive(Debug, Clone)]
pub struct WorkflowIR {
    pub id: WorkflowId,
    pub name: String,
    pub version: String,
    pub models: HashMap<ModelId, ModelIR>,
    pub steps: HashMap<StepId, StepIR>,
    pub execution_mode: ExecutionMode,
    pub workspace_path: String,
}

/// Strictly typed workflow ID (no raw strings)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkflowId(pub String);

impl WorkflowId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Strictly typed model ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);

impl ModelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Strictly typed step ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StepId(pub String);

impl StepId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Model IR (internal representation)
#[derive(Debug, Clone)]
pub struct ModelIR {
    pub id: ModelId,
    pub provider: ModelProviderIR,
    pub max_resources: ResourceAllocation,
    pub min_resources: ResourceAllocation,
}

/// Model provider (typed)
#[derive(Debug, Clone, PartialEq)]
pub enum ModelProviderIR {
    LmStudio { host: String, port: u16 },
    Ollama { base_url: String },
    LlamaCpp { model_path: String, backend: String },
}

/// Resource allocation (typed)
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub ram_mb: u64,
    pub vram_mb: u64,
    pub cpu_percent: f64,
    pub gpu_percent: f64,
}

/// Execution mode (typed)
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    Serial,
    Parallel { max_workers: u32 },
    Hybrid { parallel_threshold: u32 },
}

/// Step IR (internal representation)
#[derive(Debug, Clone)]
pub struct StepIR {
    pub id: StepId,
    pub step_type: StepTypeIR,
    pub model_id: Option<ModelId>,
    pub prompt: Option<String>,
    pub tool_name: Option<String>,
    pub input: Option<serde_json::Value>,
    pub dependencies: Vec<StepId>,
    pub when: Option<WhenConfigIR>,
    pub retry_config: RetryConfigIR,
}

/// Step type (INFERRED from keys, not parsed from YAML)
/// - generative_entity + prompt → Agent
/// - tool key → Tool
/// - sub_workflow key → SubWorkflow
/// - when key with gwt → Control
/// - loop key → Loop
#[derive(Debug, Clone, PartialEq)]
pub enum StepTypeIR {
    Agent,
    Tool,
    SubWorkflow { workflow_id: WorkflowId },
    Control,
    Loop,
}

/// When configuration (hooks)
#[derive(Debug, Clone)]
pub struct WhenConfigIR {
    pub gwt: Option<Vec<GwtRuleIR>>,
    pub requires: Vec<DependencyConfigIR>,
}

/// GWT rule (Given When Then)
#[derive(Debug, Clone)]
pub struct GwtRuleIR {
    pub given: String,
    pub when: Option<String>,
    pub then: ThenClauseIR,
}

/// Then clause
#[derive(Debug, Clone, PartialEq)]
pub enum ThenClauseIR {
    RouteTo(String),
    RouteToMultiple(Vec<String>),
}

/// Dependency configuration
#[derive(Debug, Clone)]
pub struct DependencyConfigIR {
    pub step: String,
    pub condition: Option<String>,
}

/// Retry configuration (typed)
#[derive(Debug, Clone)]
pub struct RetryConfigIR {
    pub max_attempts: u32,
    pub backoff: BackoffStrategyIR,
}

/// Backoff strategy (typed)
#[derive(Debug, Clone, PartialEq)]
pub enum BackoffStrategyIR {
    Exponential { base_ms: u64, max_ms: u64 },
    Linear { delay_ms: u64 },
    Fixed { delay_ms: u64 },
}
```

**Commit:** `feat: define WorkflowIR types with strict typing`

---

## Step 2: Write IR type tests

Create `tests/ir_test.rs`:

```rust
use whitt_execution_engine::ir::*;

#[test]
fn test_workflow_id() {
    let id = WorkflowId::new("test_workflow");
    assert_eq!(id.as_str(), "test_workflow");
}

#[test]
fn test_model_id() {
    let id = ModelId::new("primary_model");
    assert_eq!(id.0, "primary_model");
}

#[test]
fn test_step_id() {
    let id = StepId::new("step_1");
    assert_eq!(id.0, "step_1");
}

#[test]
fn test_model_provider_variants() {
    let lmstudio = ModelProviderIR::LmStudio {
        host: "localhost".to_string(),
        port: 1234,
    };
    let ollama = ModelProviderIR::Ollama {
        base_url: "http://localhost:11434".to_string(),
    };

    assert_eq!(
        matches!(lmstudio, ModelProviderIR::LmStudio { .. }),
        true
    );
    assert_eq!(
        matches!(ollama, ModelProviderIR::Ollama { .. }),
        true
    );
}

#[test]
fn test_resource_allocation() {
    let alloc = ResourceAllocation {
        ram_mb: 8192,
        vram_mb: 4096,
        cpu_percent: 50.0,
        gpu_percent: 80.0,
    };

    assert_eq!(alloc.ram_mb, 8192);
    assert_eq!(alloc.gpu_percent, 80.0);
}

#[test]
fn test_execution_mode() {
    let serial = ExecutionMode::Serial;
    let parallel = ExecutionMode::Parallel { max_workers: 4 };
    let hybrid = ExecutionMode::Hybrid {
        parallel_threshold: 2,
    };

    assert_eq!(matches!(serial, ExecutionMode::Serial), true);
    assert_eq!(
        matches!(parallel, ExecutionMode::Parallel { .. }),
        true
    );
    assert_eq!(
        matches!(hybrid, ExecutionMode::Hybrid { .. }),
        true
    );
}

#[test]
fn test_step_type_variants() {
    let agent = StepTypeIR::Agent;
    let tool = StepTypeIR::Tool;
    let subworkflow = StepTypeIR::SubWorkflow {
        workflow_id: WorkflowId::new("sub_workflow"),
    };
    let control = StepTypeIR::Control;
    let loop_step = StepTypeIR::Loop;

    assert_eq!(matches!(agent, StepTypeIR::Agent), true);
    assert_eq!(matches!(tool, StepTypeIR::Tool), true);
    assert_eq!(
        matches!(subworkflow, StepTypeIR::SubWorkflow { .. }),
        true
    );
    assert_eq!(matches!(control, StepTypeIR::Control), true);
    assert_eq!(matches!(loop_step, StepTypeIR::Loop), true);
}

#[test]
fn test_backoff_strategy_variants() {
    let exponential = BackoffStrategyIR::Exponential {
        base_ms: 1000,
        max_ms: 30000,
    };
    let linear = BackoffStrategyIR::Linear { delay_ms: 2000 };
    let fixed = BackoffStrategyIR::Fixed { delay_ms: 1000 };

    assert_eq!(
        matches!(exponential, BackoffStrategyIR::Exponential { .. }),
        true
    );
    assert_eq!(
        matches!(linear, BackoffStrategyIR::Linear { .. }),
        true
    );
    assert_eq!(
        matches!(fixed, BackoffStrategyIR::Fixed { .. }),
        true
    );
}
```

**Commit:** `test: add IR type tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test ir_test

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

# 2. All IR type tests pass
cargo test ir_test
# Expected: test result: ok. X passed

# 3. All types are strictly typed (no serde_yaml::Value)
```

**Checkpoint Criteria:**
- ✅ WorkflowIR defined with strict typing
- ✅ All IDs are typed (WorkflowId, ModelId, StepId)
- ✅ No untyped values (no serde_yaml::Value anywhere)
- ✅ IR type tests created and passing
- ✅ Enums cover all variants from schema

**Anti-Drift Check:** Verify task 5 implements ONLY IR type definitions. No compilation logic from WorkflowSpec yet.

**Next:** Proceed to Task 6 (IR Compiler)
