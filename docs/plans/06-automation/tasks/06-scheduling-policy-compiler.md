# Task 06: Scheduling Policy Compiler

**Component:** Compile cron policies to WorkflowIR scheduling nodes

**Dependencies:** Task 00 (Cron Scheduler)

**Estimated Time:** 5-7 days

**Goal:** Build a scheduling policy compiler that compiles cron expressions and scheduling policies into WorkflowIR `SchedulingNode` variants, ensuring policies are compiled (not interpreted) at runtime per ADR-0007.

---

## Overview

The scheduling policy compiler:

- Compiles cron policies to `SchedulingNode` in WorkflowIR
- Compiles resource limits to scheduling nodes
- Supports long-running workflows
- Ensures no runtime interpretation (compile-time only)

**ADR-0007 Compliance:**
- Cron policies COMPILED to WorkflowIR (not interpreted at runtime)
- Runtime scheduler uses pre-compiled scheduling metadata
- No cron expression parsing at execution time

---

## File Structure

**New Files:**
- `automation/compiler/mod.rs` - Module exports
- `automation/compiler/scheduling_node.rs` - SchedulingNode compilation
- `automation/compiler/cron_compiler.rs` - Cron expression compilation
- `automation/compiler/resource_compiler.rs` - Resource limit compilation

---

## Implementation Steps

### Step 1: Create compiler module structure

**Files:** Create `automation/compiler/mod.rs`

```rust
pub mod scheduling_node;
pub mod cron_compiler;
pub mod resource_compiler;

pub use scheduling_node::{SchedulingNodeCompiler, SchedulingNode};
pub use cron_compiler::{CronCompiler, CompiledCronSchedule};
pub use resource_compiler::{ResourceCompiler, CompiledResourceLimits};
```

---

### Step 2: Add SchedulingNode to WorkflowIR

**Files:** Modify `workflow_ir/node.rs`

```rust
// Add to existing Node enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Node {
    // Existing variants...

    /// Scheduling node for time-based execution
    Scheduling(SchedulingNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingNode {
    pub id: String,
    pub schedule: CompiledCronSchedule,
    pub resource_limits: Option<CompiledResourceLimits>,
    pub workflow_id: String,
}
```

---

### Step 3: Implement compiled cron schedule

**Files:** Create `automation/compiler/cron_compiler.rs`

```rust
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledCronSchedule {
    /// Compiled cron expression (pre-validated)
    pub cron_expression: String,

    /// Pre-calculated next run time
    pub next_run: DateTime<chrono::Utc>,

    /// Timezone
    pub timezone: Option<String>,

    /// Cron schedule for runtime use
    pub schedule: cron::Schedule,
}

pub struct CronCompiler;

impl CronCompiler {
    pub fn compile(
        cron_expression: &str,
        timezone: Option<&str>,
    ) -> Result<CompiledCronSchedule, Box<dyn std::error::Error>> {
        // Parse and validate cron expression
        let schedule = cron::Schedule::from_str(cron_expression)?;

        // Calculate next run time
        let next_run = schedule.after(&chrono::Utc::now()).next().unwrap();

        // Validate timezone
        if let Some(tz) = timezone {
            tz.parse::<chrono_tz::Tz>()?;
        }

        Ok(CompiledCronSchedule {
            cron_expression: cron_expression.to_string(),
            next_run,
            timezone: timezone.map(|t| t.to_string()),
            schedule,
        })
    }

    pub fn precompile_for_workflowir(
        cron_expression: &str,
        timezone: Option<&str>,
    ) -> Result<workflow_ir::node::CompiledCronSchedule, Box<dyn std::error::Error>> {
        let compiled = Self::compile(cron_expression, timezone)?;

        Ok(workflow_ir::node::CompiledCronSchedule {
            cron_expression: compiled.cron_expression,
            next_run: compiled.next_run.to_rfc3339(),
            timezone: compiled.timezone,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_cron() {
        let compiled = CronCompiler::compile("0 * * * *", None).unwrap();
        assert_eq!(compiled.cron_expression, "0 * * * *");
        assert!(compiled.next_run > chrono::Utc::now());
    }
}
```

---

### Step 4: Implement compiled resource limits

**Files:** Create `automation/compiler/resource_compiler.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledResourceLimits {
    pub max_cpu_cores: Option<u32>,
    pub max_memory_mb: Option<u64>,
    pub max_duration_seconds: Option<u64>,
    pub max_parallel_tasks: Option<usize>,
}

pub struct ResourceCompiler;

impl ResourceCompiler {
    pub fn compile(
        max_cpu_cores: Option<u32>,
        max_memory_mb: Option<u64>,
        max_duration_seconds: Option<u64>,
        max_parallel_tasks: Option<usize>,
    ) -> CompiledResourceLimits {
        CompiledResourceLimits {
            max_cpu_cores,
            max_memory_mb,
            max_duration_seconds,
            max_parallel_tasks,
        }
    }

    pub fn validate(limits: &CompiledResourceLimits) -> Result<(), String> {
        if let Some(cores) = limits.max_cpu_cores {
            if cores == 0 {
                return Err("max_cpu_cores must be > 0".to_string());
            }
        }

        if let Some(memory) = limits.max_memory_mb {
            if memory == 0 {
                return Err("max_memory_mb must be > 0".to_string());
            }
        }

        Ok(())
    }
}
```

---

### Step 5: Implement scheduling node compiler

**Files:** Create `automation/compiler/scheduling_node.rs`

```rust
use super::cron_compiler::{CronCompiler, CompiledCronSchedule};
use super::resource_compiler::{ResourceCompiler, CompiledResourceLimits};
use uuid::Uuid;

pub struct SchedulingNodeCompiler;

impl SchedulingNodeCompiler {
    pub fn compile(
        workflow_id: String,
        cron_expression: &str,
        timezone: Option<&str>,
        resource_limits: Option<CompiledResourceLimits>,
    ) -> Result<workflow_ir::node::SchedulingNode, Box<dyn std::error::Error>> {
        // Compile cron schedule
        let compiled_schedule = CronCompiler::compile(cron_expression, timezone)?;

        // Validate resource limits
        if let Some(limits) = &resource_limits {
            ResourceCompiler::validate(limits)?;
        }

        Ok(workflow_ir::node::SchedulingNode {
            id: Uuid::new_v4().to_string(),
            schedule: workflow_ir::node::CompiledCronSchedule {
                cron_expression: compiled_schedule.cron_expression,
                next_run: compiled_schedule.next_run.to_rfc3339(),
                timezone: compiled_schedule.timezone,
            },
            resource_limits: resource_limits.clone(),
            workflow_id,
        })
    }

    pub fn compile_for_workflowir(
        workflow_id: String,
        cron_expression: &str,
        timezone: Option<&str>,
        max_cpu_cores: Option<u32>,
        max_memory_mb: Option<u64>,
        max_duration_seconds: Option<u64>,
        max_parallel_tasks: Option<usize>,
    ) -> Result<workflow_ir::node::SchedulingNode, Box<dyn std::error::Error>> {
        let resource_limits = if max_cpu_cores.is_some()
            || max_memory_mb.is_some()
            || max_duration_seconds.is_some()
            || max_parallel_tasks.is_some()
        {
            Some(CompiledResourceLimits {
                max_cpu_cores,
                max_memory_mb,
                max_duration_seconds,
                max_parallel_tasks,
            })
        } else {
            None
        };

        Self::compile(workflow_id, cron_expression, timezone, resource_limits)
    }
}
```

---

### Step 6: Create integration test

**Files:** Create `tests/integration/scheduling_compiler_test.rs`

```rust
use agentsdk::automation::compiler::{SchedulingNodeCompiler, CronCompiler, ResourceCompiler};

#[test]
fn test_compile_cron_schedule() {
    let compiled = CronCompiler::compile("0 * * * *", None).unwrap();
    assert_eq!(compiled.cron_expression, "0 * * * *");
}

#[test]
fn test_compile_resource_limits() {
    let limits = ResourceCompiler::compile(Some(4), Some(1024), Some(3600), Some(10));
    assert_eq!(limits.max_cpu_cores, Some(4));
    assert_eq!(limits.max_memory_mb, Some(1024));
}

#[test]
fn test_compile_scheduling_node() {
    let node = SchedulingNodeCompiler::compile_for_workflowir(
        "workflow-1".to_string(),
        "0 9 * * *",
        Some("UTC"),
        Some(2),
        Some(512),
        Some(1800),
        Some(5),
    )
    .unwrap();

    assert_eq!(node.workflow_id, "workflow-1");
    assert_eq!(node.schedule.cron_expression, "0 9 * * *");
}
```

---

### Step 7: Update automation module and commit

**Files:** Modify `automation/mod.rs` and commit

```bash
git add automation/compiler workflow_ir/node.rs tests/integration/scheduling_compiler_test.rs
git commit -m "feat(automation): implement scheduling policy compiler (Task 06)

- Compile cron policies to SchedulingNode in WorkflowIR
- Add resource limit compilation
- Implement SchedulingNodeCompiler
- Update WorkflowIR with SchedulingNode variant
- Write comprehensive unit and integration tests
- Follow ADR-0007: policies compiled, not interpreted at runtime

Refs: Phase 6, Task 06"
```

---

## Summary

Task 06 implements scheduling policy compiler with:

✅ Cron policy compilation to SchedulingNode
✅ Resource limit compilation
✅ SchedulingNodeCompiler implementation
✅ WorkflowIR integration
✅ Unit and integration tests
✅ ADR-0007 compliance (compile-time, not runtime interpretation)

**Next Steps:** Task 07 (Automation CLI)
