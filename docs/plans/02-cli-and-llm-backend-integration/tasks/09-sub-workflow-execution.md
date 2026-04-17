# Task 09: Sub-Workflow Execution

**Files:**
- Create: `src/workflows/mod.rs`
- Create: `src/workflows/nesting.rs`
- Create: `src/workflows/isolation.rs`
- Create: `src/workflows/composition.rs`
- Modify: `src/lib.rs` (add workflows module)
- Test: `tests/workflows/subworkflow_test.rs`

---

## Overview

Implement sub-workflow execution with nesting, composition, circular reference detection, and isolation. This allows workflows to call other workflows as reusable components.

---

## Implementation Steps

### Step 1: Create workflow nesting

- [ ] **Step 1.1: Write nesting logic**

```rust
// src/workflows/nesting.rs
use crate::ir::WorkflowIR;
use anyhow::{Context, Result};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct WorkflowGraph {
    nodes: Vec<WorkflowNode>,
    edges: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct WorkflowNode {
    pub id: String,
    pub workflow: WorkflowIR,
    pub is_external: bool,
}

impl WorkflowGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: String, workflow: WorkflowIR, is_external: bool) {
        self.nodes.push(WorkflowNode {
            id,
            workflow,
            is_external,
        });
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.push((from, to));
    }

    pub fn detect_cycles(&self) -> Result<Vec<String>> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut recursion_stack: HashSet<String> = HashSet::new();
        let mut cycle: Vec<String> = Vec::new();

        for node in &self.nodes {
            if !visited.contains(&node.id) {
                if self.detect_cycles_recursive(&node.id, &mut visited, &mut recursion_stack, &mut cycle)? {
                    return Ok(cycle);
                }
            }
        }

        Ok(vec![])
    }

    fn detect_cycles_recursive(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
        cycle: &mut Vec<String>,
    ) -> Result<bool> {
        visited.insert(node_id.to_string());
        recursion_stack.insert(node_id.to_string());
        cycle.push(node_id.to_string());

        for (from, to) in &self.edges {
            if from == node_id {
                if !visited.contains(to) {
                    if self.detect_cycles_recursive(to, visited, recursion_stack, cycle)? {
                        return Ok(true);
                    }
                } else if recursion_stack.contains(to) {
                    return Ok(true);
                }
            }
        }

        recursion_stack.remove(node_id);
        cycle.pop();
        Ok(false)
    }

    pub fn get_execution_order(&self) -> Result<Vec<String>> {
        let cycle = self.detect_cycles()?;
        if !cycle.is_empty() {
            anyhow::bail!("Circular dependency detected: {:?}", cycle);
        }

        let mut visited: HashSet<String> = HashSet::new();
        let mut result: Vec<String> = Vec::new();

        for node in &self.nodes {
            if !visited.contains(&node.id) {
                self.topological_sort(&node.id, &mut visited, &mut result)?;
            }
        }

        Ok(result)
    }

    fn topological_sort(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        visited.insert(node_id.to_string());

        for (from, to) in &self.edges {
            if from == node_id && !visited.contains(to) {
                self.topological_sort(to, visited, result)?;
            }
        }

        result.push(node_id.to_string());
        Ok(())
    }
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 1.2: Commit**

```bash
git add src/workflows/nesting.rs
git commit -m "feat(workflows): add workflow graph with cycle detection"
```

---

### Step 2: Create isolation mechanisms

- [ ] **Step 2.1: Write isolation logic**

```rust
// src/workflows/isolation.rs
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct WorkflowIsolation {
    pub workspace_root: PathBuf,
    pub sandboxed: bool,
    pub readonly: bool,
}

impl WorkflowIsolation {
    pub fn new(workspace_root: PathBuf, sandboxed: bool, readonly: bool) -> Self {
        Self {
            workspace_root,
            sandboxed,
            readonly,
        }
    }

    pub fn create_workspace(&self) -> Result<PathBuf> {
        let workspace = self.workspace_root.join(uuid::Uuid::new_v4().to_string());

        std::fs::create_dir_all(&workspace)
            .with_context(|| format!("Failed to create workspace: {}", workspace.display()))?;

        Ok(workspace)
    }

    pub fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .with_context(|| "Failed to get current directory")?
                .join(path)
        };

        if self.sandboxed {
            // Ensure path is within workspace
            if !absolute.starts_with(&self.workspace_root) {
                anyhow::bail!(
                    "Path '{}' is outside sandbox workspace '{}'",
                    absolute.display(),
                    self.workspace_root.display()
                );
            }
        }

        Ok(absolute)
    }

    pub fn check_readonly(&self) -> Result<()> {
        if self.readonly {
            anyhow::bail!("Workflow is readonly, modifications not allowed");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_isolation_validate_path() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_root = temp_dir.path().join("workspace");
        std::fs::create_dir(&workspace_root).unwrap();

        let isolation = WorkflowIsolation::new(workspace_root, true, false);

        // Valid path within workspace
        let valid_path = isolation.workspace_root.join("file.txt");
        let result = isolation.validate_path(&valid_path);
        assert!(result.is_ok());

        // Invalid path outside workspace
        let invalid_path = temp_dir.path().join("outside.txt");
        let result = isolation.validate_path(&invalid_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_readonly_check() {
        let temp_dir = TempDir::new().unwrap();
        let isolation = WorkflowIsolation::new(temp_dir.path().to_path_buf(), false, true);

        let result = isolation.check_readonly();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("readonly"));
    }
}
```

- [ ] **Step 2.2: Commit**

```bash
git add src/workflows/isolation.rs
git commit -m "feat(workflows): add workflow isolation mechanisms"
```

---

### Step 3: Create composition logic

- [ ] **Step 3.1: Write composition logic**

```rust
// src/workflows/composition.rs
use crate::ir::{WorkflowIR, StepIR};
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct WorkflowComposer {
    workflows: std::collections::HashMap<String, WorkflowIR>,
}

impl WorkflowComposer {
    pub fn new() -> Self {
        Self {
            workflows: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, id: String, workflow: WorkflowIR) {
        self.workflows.insert(id, workflow);
    }

    pub fn compose(&self, main_workflow_id: &str) -> Result<WorkflowIR> {
        let main_workflow = self.workflows.get(main_workflow_id)
            .ok_or_else(|| anyhow::anyhow!("Main workflow '{}' not found", main_workflow_id))?;

        let mut composed = main_workflow.clone();
        composed.steps = self.compose_steps(&composed.steps, std::collections::HashSet::new())?;

        Ok(composed)
    }

    fn compose_steps(
        &self,
        steps: &[StepIR],
        mut visited: std::collections::HashSet<String>,
    ) -> Result<Vec<StepIR>> {
        let mut composed_steps = Vec::new();

        for step in steps {
            if let Some(subworkflow_id) = step.subworkflow.as_ref() {
                // Check for circular reference
                if visited.contains(subworkflow_id) {
                    anyhow::bail!("Circular reference detected: {}", subworkflow_id);
                }

                // Get sub-workflow
                let sub_workflow = self.workflows.get(subworkflow_id)
                    .ok_or_else(|| anyhow::anyhow!("Sub-workflow '{}' not found", subworkflow_id))?;

                // Compose sub-workflow recursively
                visited.insert(subworkflow_id.clone());
                let sub_steps = self.compose_steps(&sub_workflow.steps, visited.clone())?;

                // Update step context for sub-workflow steps
                for mut sub_step in sub_steps {
                    sub_step.context = step.context.clone();
                    composed_steps.push(sub_step);
                }
            } else {
                composed_steps.push(step.clone());
            }
        }

        Ok(composed_steps)
    }

    pub fn validate_composition(&self, main_workflow_id: &str) -> Result<CompositionReport> {
        let main_workflow = self.workflows.get(main_workflow_id)
            .ok_or_else(|| anyhow::anyhow!("Main workflow '{}' not found", main_workflow_id))?;

        let mut report = CompositionReport {
            workflow_id: main_workflow_id.to_string(),
            total_steps: 0,
            subworkflows: Vec::new(),
            cycles: Vec::new(),
            valid: true,
        };

        self.validate_composition_recursive(main_workflow, &mut report, std::collections::HashSet::new())?;

        report.valid = report.cycles.is_empty();

        Ok(report)
    }

    fn validate_composition_recursive(
        &self,
        workflow: &WorkflowIR,
        report: &mut CompositionReport,
        mut visited: std::collections::HashSet<String>,
    ) -> Result<()> {
        report.total_steps += workflow.steps.len();

        for step in &workflow.steps {
            if let Some(subworkflow_id) = step.subworkflow.as_ref() {
                // Check for circular reference
                if visited.contains(subworkflow_id) {
                    report.cycles.push(subworkflow_id.clone());
                    continue;
                }

                // Validate sub-workflow exists
                if !self.workflows.contains_key(subworkflow_id) {
                    anyhow::bail!("Sub-workflow '{}' not found", subworkflow_id);
                }

                report.subworkflows.push(subworkflow_id.clone());

                // Recursively validate
                visited.insert(subworkflow_id.clone());
                let sub_workflow = self.workflows.get(subworkflow_id).unwrap();
                self.validate_composition_recursive(sub_workflow, report, visited.clone())?;
            }
        }

        Ok(())
    }
}

impl Default for WorkflowComposer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CompositionReport {
    pub workflow_id: String,
    pub total_steps: usize,
    pub subworkflows: Vec<String>,
    pub cycles: Vec<String>,
    pub valid: bool,
}
```

- [ ] **Step 3.2: Write module exports**

```rust
// src/workflows/mod.rs
pub mod nesting;
pub mod isolation;
pub mod composition;

pub use nesting::{WorkflowGraph, WorkflowNode};
pub use isolation::{WorkflowIsolation};
pub use composition::{WorkflowComposer, CompositionReport};
```

- [ ] **Step 3.3: Commit**

```bash
git add src/workflows/composition.rs src/workflows/mod.rs
git commit -m "feat(workflows): add workflow composition with validation"
```

---

### Step 4: Write tests

- [ ] **Step 4.1: Write integration tests**

```rust
// tests/workflows/subworkflow_test.rs
use whitt_execution_engine::workflows::{WorkflowGraph, WorkflowComposer, CompositionReport};
use whitt_execution_engine::ir::{WorkflowIR, StepIR};

#[test]
fn test_workflow_graph_cycle_detection() {
    let mut graph = WorkflowGraph::new();

    graph.add_node("A".to_string(), WorkflowIR::default(), false);
    graph.add_node("B".to_string(), WorkflowIR::default(), false);
    graph.add_node("C".to_string(), WorkflowIR::default(), false);

    // A -> B -> C (no cycle)
    graph.add_edge("A".to_string(), "B".to_string());
    graph.add_edge("B".to_string(), "C".to_string());

    let cycle = graph.detect_cycles().unwrap();
    assert!(cycle.is_empty());
}

#[test]
fn test_workflow_graph_with_cycle() {
    let mut graph = WorkflowGraph::new();

    graph.add_node("A".to_string(), WorkflowIR::default(), false);
    graph.add_node("B".to_string(), WorkflowIR::default(), false);
    graph.add_node("C".to_string(), WorkflowIR::default(), false);

    // A -> B -> C -> A (cycle)
    graph.add_edge("A".to_string(), "B".to_string());
    graph.add_edge("B".to_string(), "C".to_string());
    graph.add_edge("C".to_string(), "A".to_string());

    let cycle = graph.detect_cycles().unwrap();
    assert!(!cycle.is_empty());
    assert_eq!(cycle.len(), 4); // A, B, C, A (includes start twice)
}

#[test]
fn test_workflow_composition() {
    let mut composer = WorkflowComposer::new();

    // Create sub-workflow
    let sub_workflow = WorkflowIR {
        id: "sub".to_string(),
        name: "Sub Workflow".to_string(),
        steps: vec![
            StepIR {
                id: "sub-step-1".to_string(),
                name: "Sub Step 1".to_string(),
                prompt: "Do something".to_string(),
                subworkflow: None,
                context: Default::default(),
            },
        ],
    };

    // Create main workflow that uses sub-workflow
    let main_workflow = WorkflowIR {
        id: "main".to_string(),
        name: "Main Workflow".to_string(),
        steps: vec![
            StepIR {
                id: "main-step-1".to_string(),
                name: "Main Step 1".to_string(),
                prompt: "Do something else".to_string(),
                subworkflow: None,
                context: Default::default(),
            },
            StepIR {
                id: "call-sub".to_string(),
                name: "Call Sub".to_string(),
                prompt: "".to_string(),
                subworkflow: Some("sub".to_string()),
                context: Default::default(),
            },
        ],
    };

    composer.register("sub".to_string(), sub_workflow);
    composer.register("main".to_string(), main_workflow);

    let composed = composer.compose("main").unwrap();
    assert_eq!(composed.steps.len(), 2); // main-step-1 and sub-step-1
}

#[test]
fn test_composition_validation() {
    let mut composer = WorkflowComposer::new();

    let workflow = WorkflowIR {
        id: "main".to_string(),
        name: "Main".to_string(),
        steps: vec![],
    };

    composer.register("main".to_string(), workflow);

    let report = composer.validate_composition("main").unwrap();
    assert!(report.valid);
    assert_eq!(report.workflow_id, "main");
}
```

- [ ] **Step 4.2: Commit**

```bash
git add tests/workflows/subworkflow_test.rs
git commit -m "test(workflows): add sub-workflow execution tests"
```

---

## Summary

This task implements sub-workflow execution including:

1. **Workflow graph** with topological sorting and cycle detection
2. **Isolation mechanisms** with sandboxing and readonly mode
3. **Workflow composition** for nesting and combining workflows
4. **Validation** for circular references and dependencies
5. **Comprehensive tests** for all workflow composition scenarios

**Key Features:**
- Recursive sub-workflow execution
- Circular reference detection
- Workspace isolation with sandboxing
- Readonly mode for safety
- Composition reports with validation
- Topological execution order

**Next:** Task 10 - Code Generation
