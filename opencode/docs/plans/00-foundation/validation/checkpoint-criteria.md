# Checkpoint Criteria

This document defines the gate criteria for each of the 12 task checkpoints in Phase 0 Foundation.

## Task 0: Cargo Project Setup

### Gate Criteria
- ✅ All dependencies added to Cargo.toml
- ✅ CI configuration created and valid
- ✅ Cargo configuration optimized
- ✅ Library structure created with all modules
- ✅ Build passes without errors
- ✅ Module stubs compile successfully
- ✅ Main.rs placeholder compiles

### Pass Conditions
```bash
cargo build                    # Must pass
cargo clippy -- -D warnings    # Zero warnings (or justified)
cargo fmt -- --check           # No formatting errors
```

---

## Task 1: Error Module

### Gate Criteria
- ✅ All Phase 0 error types added (20+ variants)
- ✅ All error constructors implemented
- ✅ bincode dependency added
- ✅ Error tests created and passing
- ✅ Error display formatting is clear and helpful
- ✅ Error sources are properly wrapped (thiserror)

### Pass Conditions
```bash
cargo test error_test          # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Error Types Checklist
- ParseError (YAML parsing)
- YamlSyntaxError (YAML syntax)
- SchemaValidationError (Schema validation)
- TypeError (Type checking)
- UndefinedReferenceError (Undefined references)
- CircularDependencyError (Circular references)
- PolicyInheritanceError (Policy inheritance)
- PolicyOverrideError (Policy override)
- InterpolationError (Variable interpolation)
- UnknownScopeError (Unknown interpolation scope)
- StorageError (sled errors)
- SerializationError (bincode errors)
- DatabaseError (Database operations)
- FileSystemError (File system operations)
- ThresholdError (Threshold validation)
- DefaultValueError (Default values)
- DagValidationError (DAG validation)
- IrCompilationError (IR compilation)
- InvalidVersionError (Invalid version)

---

## Task 2: Schema Types

### Gate Criteria
- ✅ All schema types defined (Sections 1,2,13-18)
- ✅ serde derives generate correct Serialize/Deserialize
- ✅ garde validation rules compile
- ✅ schemars generates valid JSON schema
- ✅ All default values implemented
- ✅ Schema tests created and passing
- ✅ WorkflowSpec compiles successfully
- ✅ No unused fields or dead code warnings

### Pass Conditions
```bash
cargo test schema_test         # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Schema Sections Checklist
- ✅ Section 1: Workflow Identification (WorkflowIdentification)
- ✅ Section 2: Model Configuration (ModelConfig, ModelProvider, ResourceLimits, etc.)
- ✅ Section 13: Workspace Configuration (WorkspaceConfig)
- ✅ Section 14: Features Demonstrated (FeaturesDemonstrated)
- ✅ Step types (Step, StepType, RetryConfig, OutputConfig, etc.)
- ✅ Loop configurations (LoopConfig, LoopType, CountLoopConfig, etc.)
- ✅ Execution strategy (WorkflowExecutionStrategy, ProcessingMode, etc.)
- ✅ Tool permissions (ToolPermissionsConfig, FileOperationsPermissions, etc.)
- ✅ Logging configuration (LoggingConfig, LogLevel, LoggingOutput, etc.)
- ✅ User inputs (UserInputs, UserInput, UiConfiguration, etc.)
- ✅ Complete WorkflowSpec with all fields

### Anti-Drift Check
Verify task 2 implements ONLY schema types:
- ❌ No parser code
- ❌ No IR types
- ❌ No interpolation logic
- ❌ No validation logic
- ❌ No persistence code

---

## Task 3: YAML Parser

### Gate Criteria
- ✅ YAML→WorkflowSpec parsing works with yaml_serde
- ✅ Error messages include file context and line numbers
- ✅ Parse-time validation catches schema errors
- ✅ Parser tests created and passing
- ✅ Minimal and complex workflow fixtures work
- ✅ Agentic workflow parsing works

### Pass Conditions
```bash
cargo test parser_test         # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Parser Features Checklist
- ✅ parse_workflow() - Parse from file path
- ✅ parse_workflow_str() - Parse from string
- ✅ validate_workflow() - Validate after parsing
- ✅ File context in error messages
- ✅ Minimal workflow fixture parses
- ✅ Complex workflow fixture parses
- ✅ Invalid YAML fails with helpful error
- ✅ Missing required fields fail validation

### Anti-Drift Check
Verify task 3 implements ONLY parsing:
- ❌ No IR compilation
- ❌ No DAG validation
- ❌ No policy compilation
- ❌ No persistence
- ❌ No threshold validation

---

## Task 4: Variable Interpolation

### Gate Criteria
- ✅ Parse-time interpolation (${...}) implemented
- ✅ Runtime interpolation ({{...}}) implemented
- ✅ Proper scoping (workflow, step, model, run, now)
- ✅ Interpolation tests created and passing
- ✅ Error handling for undefined variables
- ✅ Variable extraction works

### Pass Conditions
```bash
cargo test interpolation_test  # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Interpolation Features Checklist
- ✅ interpolate_parse() - ${...} syntax
- ✅ interpolate_runtime() - {{...}} syntax
- ✅ ParseContext - Parse-time context
- ✅ RuntimeContext - Runtime context
- ✅ Scope resolution (workflow, step, model, run, now)
- ✅ has_interpolation() - Detect interpolation
- ✅ extract_variables() - Extract all variables
- ✅ Undefined variables return errors

### Anti-Drift Check
Verify task 4 implements ONLY interpolation:
- ❌ No IR compilation
- ❌ No DAG validation
- ❌ No policy compilation
- ❌ No persistence
- ❌ No threshold validation

---

## Task 5: Workflow IR

### Gate Criteria
- ✅ WorkflowIR defined with strict typing
- ✅ All IDs are typed (WorkflowId, ModelId, StepId)
- ✅ No untyped values (no serde_yaml::Value anywhere)
- ✅ IR type tests created and passing
- ✅ Enums cover all variants from schema

### Pass Conditions
```bash
cargo test ir_test             # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### IR Types Checklist
- ✅ WorkflowIR (main IR struct)
- ✅ WorkflowId (typed ID)
- ✅ ModelId (typed ID)
- ✅ StepId (typed ID)
- ✅ ModelIR (model internal representation)
- ✅ ModelProviderIR (provider variants)
- ✅ ResourceAllocation (typed resources)
- ✅ ExecutionMode (execution variants)
- ✅ StepIR (step internal representation)
- ✅ StepTypeIR (step type variants)
- ✅ OutputConfigIR (output configuration)
- ✅ OutputFormatIR (output format variants)
- ✅ RetryConfigIR (retry configuration)
- ✅ BackoffStrategyIR (backoff strategy variants)

### Anti-Drift Check
Verify task 5 implements ONLY IR type definitions:
- ❌ No compilation logic from WorkflowSpec
- ❌ No DAG validation
- ❌ No policy compilation
- ❌ No persistence
- ❌ No threshold validation

---

## Task 6: IR Compiler

### Gate Criteria
- ✅ WorkflowSpec→WorkflowIR pipeline works end-to-end
- ✅ Type checking catches schema violations
- ✅ Interpolation happens during compilation
- ✅ Compiler tests created and passing
- ✅ Error messages include context
- ✅ All test fixtures compile successfully

### Pass Conditions
```bash
cargo test compiler_test       # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Compiler Features Checklist
- ✅ compile() - Main compilation function
- ✅ compile_model() - Model compilation
- ✅ compile_step() - Step compilation
- ✅ compile_step_type() - Step type conversion
- ✅ compile_output_config() - Output config compilation
- ✅ compile_retry_config() - Retry config compilation
- ✅ compile_execution_mode() - Execution mode compilation
- ✅ compile_resource_limits() - Resource limit compilation
- ✅ Interpolation during compilation
- ✅ Minimal workflow compiles
- ✅ Complex workflow compiles
- ✅ Type checking validates schema

### Anti-Drift Check
Verify task 6 implements ONLY compilation:
- ❌ No DAG validation
- ❌ No policy compilation
- ❌ No persistence
- ❌ No threshold validation
- ❌ No workspace management

---

## Task 7: DAG Validator

### Gate Criteria
- ✅ DAG validation detects circular dependencies
- ✅ Topological sort algorithm implemented
- ✅ Error messages include cycle path
- ✅ DAG tests created and passing
- ✅ Linear and parallel workflows validate correctly

### Pass Conditions
```bash
cargo test dag_test            # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### DAG Validation Features Checklist
- ✅ validate_dag() - Main DAG validation
- ✅ visit() - DFS visitor for cycle detection
- ✅ validate_step_references() - Step reference validation
- ✅ Linear workflow validates
- ✅ Parallel workflow validates
- ✅ Circular dependency detected
- ✅ Error message includes cycle path (step_1 -> step_2 -> step_3 -> step_1)

### Anti-Drift Check
Verify task 7 implements ONLY DAG validation:
- ❌ No policy compilation
- ❌ No persistence
- ❌ No threshold validation
- ❌ No workspace management
- ❌ No defaults

---

## Task 8: Policy Compiler

### Gate Criteria
- ✅ Policy fields compile deterministically
- ✅ Inheritance from default to workflow to step
- ✅ Override rules enforce precedence
- ✅ Policy tests created and passing
- ✅ Logging and tool permissions compile correctly

### Pass Conditions
```bash
cargo test policy_test         # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Policy Compilation Features Checklist
- ✅ compile_policies() - Main policy compilation
- ✅ compile_logging_policy() - Logging policy compilation
- ✅ compile_tool_permissions_policy() - Tool permissions compilation
- ✅ CompiledPolicy struct
- ✅ LoggingPolicy struct
- ✅ ToolPermissionsPolicy struct
- ✅ Deterministic compilation (same input = same output)
- ✅ Inheritance hierarchy enforced
- ✅ Override precedence (step > workflow > default)

### Anti-Drift Check
Verify task 8 implements ONLY policy compilation:
- ❌ No persistence
- ❌ No threshold validation
- ❌ No workspace management
- ❌ No defaults

---

## Task 9: Local Storage

### Gate Criteria
- ✅ .glyphnova/ directory structure created
- ✅ sled persistence works with ACID transactions
- ✅ Workflow IR storage and retrieval works
- ✅ Execution state storage works
- ✅ Storage tests created and passing
- ✅ UUID generation for execution IDs

### Pass Conditions
```bash
cargo test storage_test        # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Storage Features Checklist
- ✅ LocalStorage struct
- ✅ open() - Open or create storage
- ✅ store_workflow() - Store workflow IR
- ✅ load_workflow() - Load workflow IR
- ✅ store_execution() - Store execution state
- ✅ new_execution_id() - Generate UUID
- ✅ base_path() - Get base path
- ✅ ExecutionState struct
- ✅ .glyphnova/ directory created
- ✅ sled database initialized
- ✅ ACID transactions work

### Anti-Drift Check
Verify task 9 implements ONLY persistence:
- ❌ No workspace management
- ❌ No threshold validation
- ❌ No defaults

---

## Task 10: Workspace Management

### Gate Criteria
- ✅ Workspace directories created on initialization
- ✅ Path resolution works (absolute/relative)
- ✅ Permission checks enforce workspace boundaries
- ✅ Workspace tests created and passing
- ✅ Temp directory cleanup works

### Pass Conditions
```bash
cargo test workspace_test      # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Workspace Management Features Checklist
- ✅ WorkspaceManager struct
- ✅ new() - Create workspace manager
- ✅ initialize() - Initialize workspace directories
- ✅ resolve_path() - Resolve path relative to workspace
- ✅ is_path_allowed() - Check if path is within workspace
- ✅ clean_temp() - Clean temp directory
- ✅ config() - Get workspace configuration
- ✅ All directories created (root, output, checkpoint, log, metrics, temp)
- ✅ Absolute paths preserved
- ✅ Relative paths resolved to workspace
- ✅ Path permission checks enforce boundaries

### Anti-Drift Check
Verify task 10 implements ONLY workspace management:
- ❌ No threshold validation
- ❌ No defaults

---

## Task 11: Defaults, Scope & Inheritance

### Gate Criteria
- ✅ Default values defined for all fields
- ✅ Inheritance hierarchy: step > workflow > system
- ✅ Override rules enforced correctly
- ✅ Defaults tests created and passing
- ✅ All optional fields have defaults

### Pass Conditions
```bash
cargo test defaults_test       # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Defaults Features Checklist
- ✅ DefaultValues struct
- ✅ system_defaults() - Get system defaults
- ✅ apply_defaults() - Apply defaults to spec
- ✅ LoggingDefaults struct
- ✅ ToolPermissionDefaults struct
- ✅ apply_logging_defaults() - Apply logging defaults
- ✅ apply_tool_permission_defaults() - Apply tool permission defaults
- ✅ Inheritance hierarchy enforced
- ✅ Override precedence (step > workflow > system)
- ✅ All optional fields have defaults

### Anti-Drift Check
Verify task 11 implements ONLY defaults and inheritance:
- ❌ No threshold validation

---

## Task 12: Threshold Validation

### Gate Criteria
- ✅ All 11 numeric thresholds defined
- ✅ Range checks enforce min/max values
- ✅ Default values applied when not specified
- ✅ Error messages include actual vs expected values
- ✅ Threshold tests created and passing
- ✅ Validation happens at parse time

### Pass Conditions
```bash
cargo test threshold_test      # All tests pass
cargo build                    # No errors
cargo clippy -- -D warnings    # Zero warnings
```

### Threshold Features Checklist
- ✅ Threshold struct
- ✅ get_all_thresholds() - Get all 11 thresholds
- ✅ validate_resource_limits() - Validate resource limits
- ✅ validate_limit() - Validate single limit
- ✅ validate_percentage() - Validate percentage
- ✅ validate_tokens() - Validate token count
- ✅ validate_concurrent_requests() - Validate concurrent requests
- ✅ All 11 thresholds defined:
  1. max_allowed.ram: 0-100% (default: 13%)
  2. max_allowed.vram: 0-100% or absolute (default: 3.7GB)
  3. max_allowed.cpu: 0-100% (default: 49%)
  4. max_allowed.gpu: 0-100% (default: 74%)
  5. max_allowed.attention_tokens: 1-1,000,000 (default: 150,000)
  6. min_allowed.ram: 0-100% (default: 9%)
  7. min_allowed.vram: 0-100% or absolute (default: 2.4GB)
  8. min_allowed.cpu: 0-100% (default: 49%)
  9. min_allowed.gpu: 0-100% (default: 74%)
  10. min_allowed.attention_tokens: 1-1,000,000 (default: 73,500)
  11. max_concurrent_requests: 1-10 (default: 2)

### Final Checkpoint
All Phase 0 tasks complete! Proceed to acceptance criteria.

---

## Overall Phase 0 Verification

### Final Verification Commands
```bash
# 1. Build passes
cargo build --release

# 2. All tests pass
cargo test

# 3. No clippy warnings
cargo clippy -- -D warnings

# 4. Code is formatted
cargo fmt -- --check

# 5. LSP diagnostics clean
lsp_diagnostics filePath=/home/jon/code/yaml-to-rust-agentsdk/src/ severity=all

# 6. All verification layers pass
# See acceptance-criteria.md for details
```

### Phase 0 Complete When
- ✅ All 12 tasks complete with passing tests
- ✅ All checkpoint criteria met
- ✅ All 7 verification layers pass
- ✅ All acceptance criteria met (see acceptance-criteria.md)
- ✅ Zero external dependencies (all mocked)
- ✅ Full integration test suite passes
- ✅ Property tests demonstrate correctness invariants

**Next Phase:** Phase 1 (MVP Queue & Scheduler)
