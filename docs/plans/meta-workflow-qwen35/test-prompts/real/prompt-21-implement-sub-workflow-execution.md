# Task: Implement Sub-Workflow Execution

## Objective
The schema declares `sub_workflow` step type but the engine never executes it. Currently sub-workflows are simulated via shell hooks calling wrapper scripts.

## Requirements
1. Read `docs/schema/unified-workflow-schema.yml` to find sub_workflow step definition
2. Read `src/workflow/step.rs` to understand step parsing
3. Read `src/benchmark/runner.rs` to understand execution flow
4. Implement native sub-workflow execution:
   - Parse `sub_workflow` step type from YAML
   - Load sub-workflow YAML file
   - Create isolated execution context (separate bookmarks)
   - Execute sub-workflow steps recursively
   - Pass inputs/outputs between parent and child workflows
5. Write the complete implementation

## Output
Complete Rust implementation including:
- `SubWorkflowExecutor` struct
- YAML loading and validation
- Isolated context management
- Recursive execution logic
- Input/output passing
