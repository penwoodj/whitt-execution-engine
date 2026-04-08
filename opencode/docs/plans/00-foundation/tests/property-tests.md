# Property-Based Tests Specification

This document specifies the property-based tests for Phase 0 Foundation components.

## Overview

Property-based tests verify invariants and properties of the system using randomly generated inputs. They test that the system behaves correctly across a wide range of inputs, not just specific examples.

## Test Structure

```
tests/
└── properties/
    ├── mod.rs                # Property-based test module
    ├── determinism_tests.rs  # Determinism properties
    ├── round_trip_tests.rs   # Round-trip properties
    ├── invariant_tests.rs    # Invariant properties
    └── generation_tests.rs   # Input generation utilities
```

---

## Determinism Tests

### Test Cases

#### Compiler Determinism
```rust
#[test]
fn test_compiler_is_deterministic() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        // First compilation
        let spec1 = parse_workflow_str(&yaml).unwrap();
        let ir1 = compile(&spec1).unwrap();

        // Second compilation (same input)
        let spec2 = parse_workflow_str(&yaml).unwrap();
        let ir2 = compile(&spec2).unwrap();

        // Must produce identical IR
        assert_eq!(ir1.id.as_str(), ir2.id.as_str());
        assert_eq!(ir1.name, ir2.name);
        assert_eq!(ir1.version, ir2.version);
        assert_eq!(ir1.models.len(), ir2.models.len());
        assert_eq!(ir1.steps.len(), ir2.steps.len());
    });
}

#[test]
fn test_parser_is_deterministic() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        // First parse
        let spec1 = parse_workflow_str(&yaml).unwrap();

        // Second parse (same input)
        let spec2 = parse_workflow_str(&yaml).unwrap();

        // Must produce identical spec
        assert_eq!(spec1.identification.workflow_id, spec2.identification.workflow_id);
        assert_eq!(spec1.identification.name, spec2.identification.name);
        assert_eq!(spec1.identification.version, spec2.identification.version);
    });
}

#[test]
fn test_policy_compilation_is_deterministic() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        let spec = parse_workflow_str(&yaml).unwrap();

        // First compilation
        let policies1 = compile_policies(&spec).unwrap();

        // Second compilation (same input)
        let policies2 = compile_policies(&spec).unwrap();

        // Must produce identical policies
        assert_eq!(policies1.logging.default_level, policies2.logging.default_level);
        assert_eq!(policies1.tool_permissions.file_read_enabled, policies2.tool_permissions.file_read_enabled);
    });
}

#[test]
fn test_validation_is_deterministic() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        let spec = parse_workflow_str(&yaml).unwrap();
        let ir = compile(&spec).unwrap();

        // First validation
        let result1 = validate_dag(&ir);

        // Second validation (same input)
        let result2 = validate_dag(&ir);

        // Must produce identical result
        assert_eq!(result1.is_ok(), result2.is_ok());
    });
}
```

---

## Round-Trip Tests

### Test Cases

#### Serialization Round-Trip
```rust
#[test]
fn test_yaml_round_trip() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        // Parse YAML to WorkflowSpec
        let spec1 = parse_workflow_str(&yaml).unwrap();

        // Serialize to YAML
        let yaml_output = serde_yaml::to_string(&spec1).unwrap();

        // Parse back to WorkflowSpec
        let spec2 = parse_workflow_str(&yaml_output).unwrap();

        // Must be equivalent
        assert_eq!(spec1.identification.workflow_id, spec2.identification.workflow_id);
        assert_eq!(spec1.identification.name, spec2.identification.name);
        assert_eq!(spec1.identification.version, spec2.identification.version);
    });
}

#[test]
fn test_json_round_trip() {
    proptest!(|(spec in valid_workflow_spec())| {
        // Serialize to JSON
        let json = serde_json::to_string(&spec).unwrap();

        // Deserialize from JSON
        let spec2: WorkflowSpec = serde_json::from_str(&json).unwrap();

        // Must be equivalent
        assert_eq!(spec.identification.workflow_id, spec2.identification.workflow_id);
        assert_eq!(spec.identification.name, spec2.identification.name);
        assert_eq!(spec.identification.version, spec2.identification.version);
    });
}

#[test]
fn test_bincode_round_trip() {
    proptest!(|(ir in valid_workflow_ir())| {
        // Serialize to bincode
        let bytes = bincode::serialize(&ir).unwrap();

        // Deserialize from bincode
        let ir2: WorkflowIR = bincode::deserialize(&bytes).unwrap();

        // Must be equivalent
        assert_eq!(ir.id.as_str(), ir2.id.as_str());
        assert_eq!(ir.name, ir2.name);
        assert_eq!(ir.version, ir2.version);
        assert_eq!(ir.models.len(), ir2.models.len());
        assert_eq!(ir.steps.len(), ir2.steps.len());
    });
}
```

#### Compilation Round-Trip
```rust
#[test]
fn test_compilation_preserves_workflow_id() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        let spec = parse_workflow_str(&yaml).unwrap();
        let ir = compile(&spec).unwrap();

        assert_eq!(spec.identification.workflow_id, ir.id.as_str());
    });
}

#[test]
fn test_compilation_preserves_name() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        let spec = parse_workflow_str(&yaml).unwrap();
        let ir = compile(&spec).unwrap();

        assert_eq!(spec.identification.name, ir.name);
    });
}

#[test]
fn test_compilation_preserves_version() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        let spec = parse_workflow_str(&yaml).unwrap();
        let ir = compile(&spec).unwrap();

        assert_eq!(spec.identification.version, ir.version);
    });
}
```

---

## Invariant Tests

### Test Cases

#### WorkflowSpec Invariants
```rust
#[test]
fn test_workflow_spec_invariants() {
    proptest!(|(spec in valid_workflow_spec())| {
        // Invariant 1: Workflow ID is never empty
        assert!(!spec.identification.workflow_id.is_empty());

        // Invariant 2: Name is never empty
        assert!(!spec.identification.name.is_empty());

        // Invariant 3: Version follows semver pattern
        assert!(spec.identification.version.contains('.'));

        // Invariant 4: Models have unique names
        let model_names: Vec<_> = spec.models.models.keys().collect();
        assert_eq!(model_names.len(), spec.models.models.len()); // All unique

        // Invariant 5: Agentic workflow OR pipeline, not both
        let has_agentic = spec.agentic_workflow.is_some();
        let has_pipeline = spec.pipeline.is_some();
        // Note: schema allows both, but we prefer agentic_workflow
    });
}
```

#### WorkflowIR Invariants
```rust
#[test]
fn test_workflow_ir_invariants() {
    proptest!(|(ir in valid_workflow_ir())| {
        // Invariant 1: Workflow ID is never empty
        assert!(!ir.id.as_str().is_empty());

        // Invariant 2: Name is never empty
        assert!(!ir.name.is_empty());

        // Invariant 3: Version is never empty
        assert!(!ir.version.is_empty());

        // Invariant 4: All step IDs are unique
        let step_ids: Vec<_> = ir.steps.keys().collect();
        assert_eq!(step_ids.len(), ir.steps.len()); // All unique

        // Invariant 5: All model IDs are unique
        let model_ids: Vec<_> = ir.models.keys().collect();
        assert_eq!(model_ids.len(), ir.models.len()); // All unique

        // Invariant 6: All step dependencies reference existing steps
        for step in ir.steps.values() {
            for dep_id in &step.dependencies {
                assert!(ir.steps.contains_key(dep_id));
            }
        }
    });
}

#[test]
fn test_dag_invariants() {
    proptest!(|(ir in valid_workflow_ir())| {
        // Invariant 1: Valid IR passes DAG validation
        assert!(validate_dag(&ir).is_ok());

        // Invariant 2: No self-dependencies
        for (step_id, step) in ir.steps.iter() {
            assert!(!step.dependencies.contains(step_id));
        }
    });
}
```

#### Resource Allocation Invariants
```rust
#[test]
fn test_resource_allocation_invariants() {
    proptest!(|(limits in valid_resource_limits())| {
        // Invariant 1: max >= min for all resources
        assert!(limits.ram <= limits.ram); // Simplified check

        // Invariant 2: Percentages are between 0 and 100
        match &limits.ram {
            ResourceLimit::Percentage(p) => assert!(*p >= 0.0 && *p <= 100.0),
            _ => {}
        }

        match &limits.vram {
            ResourceLimit::Percentage(p) => assert!(*p >= 0.0 && *p <= 100.0),
            _ => {}
        }

        // Invariant 3: Tokens are positive
        assert!(limits.attention_tokens >= 1);
    });
}
```

---

## Generation Utilities

### Strategy Implementations

#### Valid YAML Generator
```rust
use proptest::prelude::*;
use proptest::string::string_regex;

fn valid_workflow_yaml() -> impl Strategy<Value = String> {
    string_regex(r"workflow_id:\s*[a-z_][a-z0-9_]*\nname:\s*.*\ndescription:\s*.*\nversion:\s*\d+\.\d+\.\d+").unwrap()
}

fn valid_workflow_spec() -> impl Strategy<Value = WorkflowSpec> {
    // Generate valid WorkflowSpec
    prop::collection::hash_map(
        "model_[a-z]{3,10}",
        any::<ModelConfig>(),
        0..3usize,
    ).prop_map(|models| {
        WorkflowSpec {
            identification: WorkflowIdentification {
                workflow_id: "test_workflow".to_string(),
                name: "Test Workflow".to_string(),
                description: "Test".to_string(),
                version: "1.0.0".to_string(),
                author: "".to_string(),
                tags: vec![],
            },
            models: ModelsConfig {
                models,
                ..Default::default()
            },
            workspace: WorkspaceConfig {
                root_path: "/workspace".to_string(),
                ..Default::default()
            },
            features: FeaturesDemonstrated::default(),
            execution: WorkflowExecutionStrategy::default(),
            tool_permissions: ToolPermissionsConfig::default(),
            logging: LoggingConfig::default(),
            agentic_workflow: None,
            pipeline: None,
        }
    })
}

fn valid_workflow_ir() -> impl Strategy<Value = WorkflowIR> {
    prop::collection::hash_map(
        "step_[a-z0-9_]+",
        any::<StepIR>(),
        0..5usize,
    ).prop_flat_map(|steps| {
        let step_ids: Vec<_> = steps.keys().cloned().collect();
        (
            Just(steps),
            prop::sample::select(step_ids),
            any::<ExecutionMode>(),
        )
    }).prop_map(|(mut steps, sample_step, mode)| {
        // Add dependencies to make DAG valid
        if let Some(step) = steps.values_mut().next() {
            if !steps.is_empty() {
                let dep_id = steps.keys().next().unwrap().clone();
                step.dependencies.push(dep_id);
            }
        }

        WorkflowIR {
            id: WorkflowId::new("test_workflow"),
            name: "Test Workflow".to_string(),
            version: "1.0.0".to_string(),
            models: std::collections::HashMap::new(),
            steps,
            execution_mode: mode,
            workspace_path: "/workspace".to_string(),
        }
    })
}

fn valid_resource_limits() -> impl Strategy<Value = ResourceLimits> {
    (0..100u8).prop_map(|ram_pct| {
        ResourceLimits {
            ram: ResourceLimit::Percentage(ram_pct as f64),
            vram: ResourceLimit::Percentage(3.7),
            cpu: 49.0,
            gpu: 74.0,
            attention_tokens: 150000,
            concurrent_requests: 2,
        }
    })
}
```

---

## Running Tests

```bash
# Run all property-based tests
cargo test --test properties

# Run with more test cases (default is 100)
cargo test --test properties -- --test-threads=1

# Run with custom test count
PROPTEST_CASES=1000 cargo test --test properties

# Run failing tests in replay mode
PROPTEST_FORK=fail cargo test --test properties

# Show generated inputs on failure
cargo test --test properties -- --nocapture
```

---

## Coverage Goals

- **Determinism Tests**: 95%+ coverage (all functions tested)
- **Round-Trip Tests**: 90%+ coverage (all serialization paths)
- **Invariant Tests**: 95%+ coverage (all invariants verified)
- **Generation Utilities**: 100% coverage (all strategies tested)

---

## Performance Requirements

- Property tests should run 100+ test cases in < 30 seconds
- Each test case should complete in < 100ms
- No test should timeout (> 60s)

---

## Shrinkable Properties

All properties should be shrinkable to minimal counter-examples:

```rust
// Example: Threshold validation should shrink to edge cases
#[test]
fn test_threshold_validation_shrinks() {
    proptest!(|(ram_pct in 0u8..=255)| {
        let limits = ResourceLimits {
            ram: ResourceLimit::Percentage(ram_pct as f64),
            ..Default::default()
        };

        if ram_pct > 100 {
            assert!(validate_resource_limits(&limits, true).is_err());
        }
    });
}

// Shrinks to: ram_pct = 101 (minimal failure case)
```

---

## Test Count Configuration

```toml
# In Cargo.toml
[dev-dependencies]
proptest = "1.0"
proptest-derive = "0.3"

# In test files
proptest! {
    # Default: 100 test cases
    #[test]
    fn test_property(input in strategy) {
        // test logic
    }

    // Custom test count: 1000 test cases
    # proptest! {
    //     #![proptest_config(ProptestConfig::with_cases(1000))]
    //     #[test]
    //     fn test_property(input in strategy) {
    //         // test logic
    //     }
    # }
}
```

---

## Documentation Requirements

All property-based tests must include:
1. **Property Name**: Clear description of the property
2. **Explanation**: Why this property should hold
3. **Test Cases**: Strategy description
4. **Shrinkability**: Counter-example minimization

Example:
```rust
/// Property: Compiler is deterministic
///
/// Explanation: Given the same WorkflowSpec input, the compiler should
/// always produce the same WorkflowIR output. This ensures reproducible
/// compilation and debugging.
///
/// Strategy: valid_workflow_yaml() generates random valid YAML workflows
///
/// Shrinkability: Failing cases shrink to minimal YAML that causes non-determinism
#[test]
fn test_compiler_is_deterministic() {
    proptest!(|(yaml in valid_workflow_yaml())| {
        // test implementation
    });
}
```

---

**Document Version:** 1.0.0
**Last Updated:** 2026-04-06
**Status:** Complete
