# Tasks

## Files
- [00-cargo-project-setup.md](./00-cargo-project-setup.md) - Initialize Cargo project structure, configure dependencies, and set up build system
- [01-error-module.md](./01-error-module.md) - Define error types and error handling infrastructure for the framework
- [02-schema-types.md](./02-schema-types.md) - Implement Rust types matching YAML workflow schema structure
- [03-yaml-parser.md](./03-yaml-parser.md) - Parse YAML workflow files using serde-saphyr into Rust types
- [04-variable-interpolation.md](./04-variable-interpolation.md) - Support variable references and substitutions in workflow definitions
- [05-workflow-ir.md](./05-workflow-ir.md) - Define internal intermediate representation for workflow execution
- [06-ir-compiler.md](./06-ir-compiler.md) - Compile parsed YAML into WorkflowIR for execution engine
- [07-dag-validator.md](./07-dag-validator.md) - Validate workflow structure and detect circular dependencies
- [08-policy-compiler.md](./08-policy-compiler.md) - Compile policy configurations into executable policy objects
- [09-local-storage.md](./09-local-storage.md) - Implement local file system storage for workflows and state
- [10-workspace-management.md](./10-workspace-management.md) - Manage workspace directories for logs, output, and state
- [11-defaults-scope-inheritance.md](./11-defaults-scope-inheritance.md) - Handle default values and scope inheritance in workflow configurations
- [12-threshold-validation.md](./12-threshold-validation.md) - Implement validation logic for threshold-based convergence criteria

## See Also
- [Parent Foundation plan](../plan.md)
