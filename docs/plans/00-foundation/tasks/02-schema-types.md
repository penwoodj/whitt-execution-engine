# Task 2: Schema Types

**Goal:** Define all schema types matching [unified-workflow-schema.yml](../../schema/unified-workflow-schema.yml) sections: identification, providers, models, sub_workflows, agentic_workflow, workflow_execution_strategy, tool_permissions, memory, workspace.

**Estimated Time:** 4 hours

**Dependencies:** Task 0, Task 1

**Files:**
- Create: `src/schema/identification.rs` (workflow metadata types)
- Create: `src/schema/model.rs` (models section types)
- Create: `src/schema/workspace.rs` (workspace section types)
- Create: `src/schema/step.rs` (agentic_workflow step types)
- Create: `src/schema/loop.rs` (loop step types)
- Create: `src/schema/execution.rs` (execution strategy types)
- Create: `src/schema/mod.rs` (module declarations)
- Modify: `Cargo.toml` (add uuid, regex dependencies)

---

## Step 1: Create identification types

Create `src/schema/identification.rs`:

```rust
//! Workflow identification and metadata types
//!
//! Schema reference: Lines 14-21

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowIdentification {
    pub workflow_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderReference {
    pub provider_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<String>,
    pub config: Option<ProviderConfig>,
}
```

**Commit:** `feat: add workflow identification types`

---

## Step 2: Create model types

Create `src/schema/model.rs`:

```rust
//! Model configuration types
//!
//! Schema reference: Lines 64-158

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelHost {
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    Lmstudio,
    Ollama,
    LlamaCppWithVulkan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<ResourceValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vram: Option<ResourceValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<ResourceValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<ResourceValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attention_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_requests: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceValue {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>, // "GB", "MB", "%"
}
```

**Commit:** `feat: add model configuration types`

---

## Step 3: Create workspace types

Create `src/schema/workspace.rs`:

```rust
//! Workspace definition types
//!
//! Schema reference: Lines 699-723

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub root_path: String,
    pub directories: Directories,
    #[serde(default)]
    pub permissions: WorkspacePermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directories {
    pub output: String,
    pub checkpoints: String,
    pub logs: String,
    pub metrics: String,
    pub backups: String,
    pub temp: String,
    pub rag_knowledge_base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePermissions {
    #[serde(default = "default_permissions_mode")]
    pub default_mode: PermissionsMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionsMode {
    OwnerFull = 700,
    OwnerFullGroupRead = 740,
    OwnerFullGroupReadExec = 750,
    #[serde(rename = "owner_full_group_read_other_read_exec")]
    OwnerFullGroupReadOtherReadExec = 755,
    #[serde(rename = "owner_full_group_read_other_read")]
    OwnerFullGroupReadOtherRead = 744,
    #[serde(rename = "owner_full_group_full_other_read")]
    OwnerFullGroupFullOtherRead = 774,
    #[serde(rename = "owner_full_group_full_other_full")]
    OwnerFullGroupFullOtherFull = 777,
    #[serde(rename = "owner_read_only")]
    OwnerReadOnly = 400,
    #[serde(rename = "owner_read_write")]
    OwnerReadWrite = 600,
}
```

**Commit:** `feat: add workspace definition types`

---

## Step 4: Create step types

Create `src/schema/step.rs`:

```rust
//! Agentic workflow step types
//!
//! Schema reference: Lines 196-497

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgenticWorkflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardcoded_values: Option<HardcodedValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<WorkflowInputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_inputs: Option<UserInputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<LifecycleHooks>,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum WorkflowStep {
    Generative(GenerativeStep),
    Tool(ToolStep),
    ControlFlow(ControlFlowStep),
    SubWorkflow(SubWorkflowStep),
    Loop(LoopStep),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerativeStep {
    pub step_name: String,
    pub generative_entity: String, // "${models.model-name}"
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_overrides: Option<ModelOverrides>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<StepRetryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<Dependency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<StepHooks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStep {
    pub step_name: String,
    pub tool: String,
    pub input: serde_yaml::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<StepHooks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowStep {
    pub step_name: String,
    pub when: ControlFlowHooks,
}
```

**Commit:** `feat: add agentic workflow step types`

---

## Step 5: Create loop types

Create `src/schema/loop.rs`:

```rust
//! Loop step types
//!
//! Schema reference: Lines 411-453

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopStep {
    pub step_name: String,
    pub loop_config: LoopConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_workflow: Option<String>,
    pub input: serde_yaml::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<StepHooks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum LoopConfig {
    Validation(ValidationLoop),
    Count(CountLoop),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationLoop {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    pub max_iterations: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_criteria: Option<Vec<ExactCriterion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExactCriterion {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub target: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    GreaterEqual = ">=",
    LessEqual = "<=",
    Equal = "==",
    NotEqual = "!=",
    Greater = ">",
    Less = "<",
}
```

**Commit:** `feat: add loop step types`

---

## Step 6: Create execution strategy types

Create `src/schema/execution.rs`:

```rust
//! Workflow execution strategy types
//!
//! Schema reference: Lines 503-598

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionStrategy {
    pub load_unload: LoadUnloadStrategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeoutConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling: Option<ErrorHandlingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_workflow: Option<SubWorkflowConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing: Option<CheckpointingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<SynchronizationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_resolution: Option<DependencyResolutionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadUnloadStrategy {
    OneAtATime,
    Lazy,
    Eager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub ram_allocation: ResourceAllocationStrategy,
    pub model_lifecycle: ModelLifecycle,
    pub pressure_handling: PressureHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLifecycle {
    pub load_unload_strategy: LoadUnloadStrategy,
    pub cache_size: CacheSize,
    #[serde(default)]
    pub swap_timeout_secs: u64,
    #[serde(default)]
    pub unload_unused: bool,
}
```

**Commit:** `feat: add workflow execution strategy types`

---

## Step 7: Create tool permissions types

Create `src/schema/tool_permissions.rs`:

```rust
//! Tool permissions types
//!
//! Schema reference: Lines 606-676

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operations: Option<FileOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_operations: Option<WebOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_operations: Option<ShellOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_operations: Option<ContentOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_operations: Option<SystemOperations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<FilePermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<FilePermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<FilePermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePermission {
    #[serde(default)]
    pub require_confirmation: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size_mb: Option<usize>,
}
```

**Commit:** `feat: add tool permissions types`

---

## Step 8: Create module declarations

Create `src/schema/mod.rs`:

```rust
//! Schema types for unified workflow specification
//!
//! This module defines all types matching the unified-workflow-schema.yml:
//! - Workflow identification (workflow_id, name, version)
//! - Provider configurations (LM Studio, Ollama, llama.cpp)
//! - Model definitions (resources, allocation, execution)
//! - Workspace structure (directories, permissions)
//! - Agentic workflow steps (generative, tool, control flow, loops)
//! - Execution strategy (load/unload, memory, timeout, retry)
//! - Tool permissions (file, web, shell operations)

pub mod identification;
pub mod model;
pub mod workspace;
pub mod step;
pub mod loop_step;
pub mod execution;
pub mod tool_permissions;

// Re-export for convenience
pub use identification::{WorkflowIdentification, ProviderReference};
pub use model::{ModelHost, ProviderType, ResourceLimit};
pub use workspace::{WorkspaceConfig, Directories, PermissionsMode};
pub use step::{AgenticWorkflow, WorkflowStep, GenerativeStep};
pub use loop_step::{LoopStep, LoopConfig, ValidationLoop};
pub use execution::{WorkflowExecutionStrategy, LoadUnloadStrategy};
pub use tool_permissions::{ToolPermissions, FileOperations};
```

**Commit:** `feat: add schema module declarations`

---

## Step 9: Add dependencies

Update `Cargo.toml`:

```toml
[dependencies]
# Already present from Task 0
uuid = { version = "1.8", features = ["v4", "serde"] }

# Add for regex validation
regex = "1.10"
```

**Commit:** `chore: add regex dependency for schema types`

---

## Step 10: Verify build

```bash
cargo build

# Expected: Compiling whitt-execution-engine v0.1.0
# Finished dev [unoptimized + debuginfo] target(s)
```

**Commit:** `fix: resolve compilation errors`

---

## Verification

After completing all steps, verify:

```bash
# 1. All schema modules compile
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. Types match schema
# Manual review: compare struct fields with unified-workflow-schema.yml

# 3. Serde derives work
# Test serialization/deserialization with sample YAML

# 4. Re-exports work
# Check that types are accessible via src/lib.rs
```

**Checkpoint Criteria:**
- ✅ All 8 schema modules created
- ✅ Types match unified-workflow-schema.yml structure
- ✅ Serde derives on all structs
- ✅ Proper handling of optional fields with skip_serializing_if
- ✅ Module declarations and re-exports in mod.rs
- ✅ Dependencies added (uuid, regex)
- ✅ Build passes

**Next:** Proceed to Task 3 (YAML Parser)

---

## Implementation Status

**Status**: 🔵 NOT STARTED

### What Exists
- **Alternative implementation**: [src/model/schema.rs](../../src/model/schema.rs) exists with:
  - `ModelsConfig` struct with models map and default_router ✅
  - `ModelSpec` struct with name, host type, memory, execution, tools ✅
  - `RamAllocation` with strategy and min_allowed/max_allowed fields ✅
  - `ModelMemory` with cache_size, kv_cache_quantization, attention_context fields ✅
  - `Execution` struct with timeout, max_turns, stop_on_tool_failure fields ✅
  - `Tools` struct with default_permissions, allowed_tools, forbidden_tools ✅
  - Partial model configuration, but not matching plan's expected structure

### What's Missing
- **Schema module structure** not implemented:
  - `src/schema/identification.rs` - NOT IMPLEMENTED (plan expects WorkflowIdentification struct)
  - `src/schema/model.rs` (plan expects this file, but current implementation uses `src/model/schema.rs` with different structure)
  - `src/schema/workspace.rs` - NOT IMPLEMENTED (plan expects WorkspaceConfig, Directories structs)
  - `src/schema/step.rs` - NOT IMPLEMENTED (plan expects AgenticWorkflow, WorkflowStep enums)
  - `src/schema/loop.rs` - NOT IMPLEMENTED (plan expects LoopStep, LoopConfig enums)
  - `src/schema/execution.rs` - NOT IMPLEMENTED (plan expects WorkflowExecutionStrategy)
  - `src/schema/tool_permissions.rs` - NOT IMPLEMENTED (plan expects ToolPermissions struct)
  - `src/schema/mod.rs` - NOT IMPLEMENTED (plan expects module declarations and re-exports)

- **Structs not implemented** (per plan):
  - `WorkflowIdentification` (workflow_id, name, description, version, author, tags) - NOT IMPLEMENTED
  - `ModelHost` (provider_type, connection_settings) - NOT IMPLEMENTED
  - `ResourceLimit` and `ResourceValue` - NOT IMPLEMENTED
  - `WorkspaceConfig`, `Directories`, `PermissionsMode` - NOT IMPLEMENTED
  - `AgenticWorkflow`, `WorkflowStep`, `GenerativeStep`, `ToolStep` - NOT IMPLEMENTED
  - `LoopConfig`, `ValidationLoop`, `CountLoop` - NOT IMPLEMENTED
  - `ExactCriterion`, `ComparisonOperator` - NOT IMPLEMENTED
  - `WorkflowExecutionStrategy`, `LoadUnloadStrategy` - NOT IMPLEMENTED
  - `MemoryConfig`, `ModelLifecycle`, `PressureHandling` - NOT IMPLEMENTED
  - `ToolPermissions`, `FileOperations`, `WebOperations` - NOT IMPLEMENTED
  - `FilePermission`, `FileOperation` enum - NOT IMPLEMENTED

- **Dependencies** not added to Cargo.toml:
  - `regex` dependency (plan Step 9) - NOT PRESENT in Cargo.toml

### QA Coverage
- **Status**: No dedicated QA tests for this task
- **Coverage**: From EPOC Extended POC findings:
  - **AREA-06 MODEL SCHEMA PARSING** (Area 6) - ✅ PASS - Model configuration types implemented (ModelsConfig, ModelSpec, RamAllocation, etc.)
  - **Note**: QA confirms model schema parsing works, but structure differs from plan's expected layout

### Schema Alignment
- **Schema Ref**: Lines 14-21 (identification), Lines 64-158 (models), Lines 196-353 (agentic_workflow inputs), Lines 354-497 (steps), Lines 503-598 (workflow_execution_strategy), Lines 606-676 (tool_permissions)
- **Coverage**: Partial — Some schema structures mapped to code, but not in planned file locations
- **Gaps**:
  - Lines 14-21 (identification) - NOT IMPLEMENTED as WorkflowIdentification
  - Lines 64-158 (models) - PARTIAL (ModelsConfig exists but different structure)
  - Lines 196-353 (workflow inputs, user inputs, retry, hooks) - NOT IMPLEMENTED
  - Lines 354-497 (step definitions, tool steps, control flow, sub-workflows, loops) - NOT IMPLEMENTED
  - Lines 503-598 (execution strategy) - PARTIAL (some structures exist but not WorkflowExecutionStrategy)
  - Lines 606-676 (tool permissions) - PARTIAL (Tools struct exists but not ToolPermissions)

### Evidence
- **Alternative location**: [src/model/schema.rs](../../src/model/schema.rs) (400+ lines) has model types
- **Current structure**: ModelsConfig, ModelSpec, RamAllocation, Execution, Tools structs (different from plan)
- **Build**: ✅ `cargo build` passes (using existing implementation)
- **Tests**: ✅ `cargo test --lib` passes (model schema tests exist)

### Plan vs Reality Notes
- **Plan expects**: Separate `src/schema/` directory with 7 module files (identification.rs, model.rs, workspace.rs, step.rs, loop.rs, execution.rs, tool_permissions.rs)
- **Current reality**: Model schema is in `src/model/schema.rs` (under `model/` directory, not `schema/`)
- **Plan vs reality difference**: Plan's schema organization doesn't match current codebase structure
- **Dependencies**: Plan expects `regex` dependency to be added, but it's not in Cargo.toml yet

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
