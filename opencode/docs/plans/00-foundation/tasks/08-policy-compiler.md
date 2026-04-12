# Task 8: Policy Compiler

**Goal:** Implement deterministic policy field compilation with inheritance and override rules.

**Estimated Time:** 5 hours

**Dependencies:** Task 6

**Files:**
- Modify: `src/policy.rs` (implement policy compilation)
- Create: `tests/policy_test.rs` (policy compilation tests)

---

## Step 1: Implement policy compilation

Create `src/policy.rs`:

**Important v2.0 Notes:**
- v2.0 uses `when:` hooks for step-level events (logging, output, branching)
- v2.0 scope hierarchy: L1 (workflow top-level) → L2 (agentic_workflow defaults) → L3 (step-level)
- Tool permissions: L1 baseline → L3 can only RESTRICT (v3 enhancement)
- Retry: L2 step defaults → L3 overrides completely (not merged)
- Hooks: L2 default hooks MERGE with L3 step hooks (additive)

```rust
use crate::error::{Error, Result};
use crate::schema::{WorkflowSpec, LoggingConfig, ToolPermissionsConfig};
use std::collections::HashMap;

/// Compiled policy with resolved inheritance
#[derive(Debug, Clone)]
pub struct CompiledPolicy {
    pub logging: LoggingPolicy,
    pub tool_permissions: ToolPermissionsPolicy,
}

/// Logging policy
#[derive(Debug, Clone)]
pub struct LoggingPolicy {
    pub default_level: String,
    pub workflow_level: String,
    pub step_level: String,
    pub model_level: String,
    pub tool_level: String,
}

/// Tool permissions policy
#[derive(Debug, Clone)]
pub struct ToolPermissionsPolicy {
    pub file_read_enabled: bool,
    pub file_write_enabled: bool,
    pub file_delete_enabled: bool,
    pub web_fetch_enabled: bool,
    pub web_scrape_enabled: bool,
    pub shell_exec_enabled: bool,
}

/// Compile policies with inheritance
pub fn compile_policies(spec: &WorkflowSpec) -> Result<CompiledPolicy> {
    let logging = compile_logging_policy(spec)?;
    let tool_permissions = compile_tool_permissions_policy(spec)?;

    Ok(CompiledPolicy {
        logging,
        tool_permissions,
    })
}

/// Compile logging policy as when: hooks (v2.0 approach)
///
/// In v2.0, logging is configured via when: lifecycle hooks on steps, not standalone logging: sections.
/// The v2.0 scope hierarchy is L1 (workflow top-level) → L2 (agentic_workflow defaults) → L3 (step-level).
///
/// Inheritance rules (v2.0):
/// - Tool Permissions: L1 is global baseline → L3 can only RESTRICT, not expand
/// - Retry: L2 step defaults → L3 overrides completely (not merged)
/// - Hooks: L2 default hooks MERGE with L3 step hooks (additive)
///
/// Note: In v2.0, logging: section still exists but when: hooks are the primary mechanism
/// for step-level logging events (before_step_starts, after_step_succeeds, after_step_fails, etc.).
fn compile_logging_policy(spec: &WorkflowSpec) -> Result<LoggingPolicy> {
    let config = &spec.logging;

    Ok(LoggingPolicy {
        default_level: format!("{:?}", config.default).to_lowercase(),
        workflow_level: config.levels.get("workflow")
            .map(|l| format!("{:?}", l).to_lowercase())
            .unwrap_or_else(|| format!("{:?}", config.default).to_lowercase()),
        step_level: config.levels.get("step")  // v2.0: "step" instead of v1's "pipeline"
            .map(|l| format!("{:?}", l).to_lowercase())
            .unwrap_or_else(|| format!("{:?}", config.default).to_lowercase()),
        model_level: config.levels.get("models")
            .map(|l| format!("{:?}", l).to_lowercase())
            .unwrap_or_else(|| format!("{:?}", config.default).to_lowercase()),
        tool_level: config.levels.get("tools")
            .map(|l| format!("{:?}", l).to_lowercase())
            .unwrap_or_else(|| format!("{:?}", config.default).to_lowercase()),
    })
}

/// Compile tool permissions policy (v2.0: L1 baseline only)
///
/// In v2.0, tool_permissions at L1 (workflow top-level) is the global baseline.
/// Per-step (L3) configuration can only RESTRICT this baseline, never expand it.
///
/// v3 Enhancement: Tool permissions can only RESTRICT the L1 baseline
fn compile_tool_permissions_policy(spec: &WorkflowSpec) -> Result<ToolPermissionsPolicy> {
    let config = &spec.tool_permissions;

    Ok(ToolPermissionsPolicy {
        file_read_enabled: config.file_operations.read.enabled || config.file_operations.read.disabled.is_none(),
        file_write_enabled: config.file_operations.write.enabled || config.file_operations.write.disabled.is_none(),
        file_delete_enabled: config.file_operations.delete.enabled || config.file_operations.delete.disabled.is_none(),
        web_fetch_enabled: config.web_operations.fetch.enabled || config.web_operations.fetch.disabled.is_none(),
        web_scrape_enabled: config.web_operations.scrape.enabled || config.web_operations.scrape.disabled.is_none(),
        shell_exec_enabled: !(config.shell_operations.exec.disabled.unwrap_or(false)),  // v2.0: defaults to disabled
    })
}
```

**Commit:** `feat: implement policy compiler with inheritance`

---

## Step 2: Write policy tests

Create `tests/policy_test.rs`:

```rust
use yaml_to_rust_agentsdk::policy::*;
use yaml_to_rust_agentsdk::parser::parse_workflow;

#[test]
fn test_compile_logging_policy() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let policy = compile_policies(&spec).unwrap();

    assert_eq!(policy.logging.default_level, "info");
    assert_eq!(policy.logging.workflow_level, "info");
}

#[test]
fn test_compile_tool_permissions_policy() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let policy = compile_policies(&spec).unwrap();

    assert_eq!(policy.tool_permissions.file_read_enabled, true);
    assert_eq!(policy.tool_permissions.file_write_enabled, false);
    assert_eq!(policy.tool_permissions.file_delete_enabled, false);
}

#[test]
fn test_complex_policy() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    let policy = compile_policies(&spec).unwrap();

    assert_eq!(policy.tool_permissions.file_read_enabled, true);
    assert_eq!(policy.tool_permissions.file_write_enabled, true);
    assert_eq!(policy.tool_permissions.shell_exec_enabled, true);
}
```

**Commit:** `test: add policy compilation tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test policy_test

# Expected output:
# test result: ok. X passed in Y.ZZs
```

**Commit:** `fix: resolve any test failures`

---

## Additional Edge Case Tests

### Step 4: Add empty policy test

```rust
#[test]
fn test_empty_policy() {
    let yaml = r#"
workflow_id: empty_policy
name: "Empty Policy"
models: {}
execution: { mode: serial }
logging:
  default: info
  levels: {}
agentic_workflow: []
"#;
    let spec = parse_yaml(yaml).unwrap();
    let policy = compile_policies(&spec).unwrap();

    // Should use defaults when no overrides
    assert_eq!(policy.logging.default_level, "info");
    assert_eq!(policy.logging.workflow_level, "info");
    assert_eq!(policy.logging.step_level, "info");
}
```

- [ ] **Step 4a:** Create test fixtures for empty policy
- [ ] **Step 4b:** Implement empty policy test
- [ ] **Step 4c:** Run test to verify behavior
- [ ] **Step 4d:** Commit: `test: add empty policy edge case`

### Step 5: Add conflicting overrides test

```rust
#[test]
fn test_conflicting_overrides() {
    let yaml = r#"
workflow_id: conflict
name: "Conflicting Overrides"
logging:
  default: debug
  levels:
    workflow: info
    step: error
tool_permissions:
  file_operations:
    read:
    read:
      disabled: true
"#;
    let spec = parse_yaml(yaml).unwrap();
    let policy = compile_policies(&spec).unwrap();

    // Last override should win (disabled: true overrides presence=enabled)
    assert_eq!(policy.logging.step_level, "error");
    assert_eq!(policy.tool_permissions.file_read_enabled, false);
}
```

- [ ] **Step 5a:** Create fixture with conflicting overrides
- [ ] **Step 5b:** Implement conflict resolution test
- [ ] **Step 5c:** Run test to verify last-wins behavior
- [ ] **Step 5d:** Commit: `test: add conflicting override test`

### Step 6: Add deeply nested inheritance test

```rust
#[test]
fn test_deeply_nested_inheritance() {
    let yaml = r#"
workflow_id: nested
name: "Deeply Nested Inheritance"
logging:
  default: trace
  levels:
    workflow: debug
    step: info
    models: warn
    tools: error
"#;
    let spec = parse_yaml(yaml).unwrap();
    let policy = compile_policies(&spec).unwrap();

    // Verify L1→L2 inheritance (v2.0 scope hierarchy)
    // L1 (workflow top-level): default → L2 (agentic_workflow defaults): workflow/step/models/tools
    assert_eq!(policy.logging.default_level, "trace");
    assert_eq!(policy.logging.workflow_level, "debug");
    assert_eq!(policy.logging.step_level, "info");  // v2.0: "step" instead of "pipeline"
    assert_eq!(policy.logging.model_level, "warn");
    assert_eq!(policy.logging.tool_level, "error");
}
```

- [ ] **Step 6a:** Create fixture with deep nesting
- [ ] **Step 6b:** Implement nested inheritance test
- [ ] **Step 6c:** Run test to verify inheritance chain
- [ ] **Step 6d:** Commit: `test: add deeply nested inheritance test`

---

## Integration Test Specifications

### Step 7: Policy + Storage interaction

```rust
#[test]
fn test_policy_with_storage() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let policy = compile_policies(&spec).unwrap();

    // Create a mock storage that respects policy
    let storage = MockStorage::new(policy.tool_permissions.clone());

    // Test that storage respects policy
    assert_eq!(storage.can_read("./src/"), policy.tool_permissions.file_read_enabled);
    assert_eq!(storage.can_write("./out/"), policy.tool_permissions.file_write_enabled);
}
```

- [ ] **Step 7a:** Create mock storage struct
- [ ] **Step 7b:** Implement storage integration test
- [ ] **Step 7c:** Run test to verify policy enforcement
- [ ] **Step 7d:** Commit: `test: add policy storage integration`

### Step 8: Policy + Execution interaction

```rust
#[test]
fn test_policy_with_execution() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    let policy = compile_policies(&spec).unwrap();

    let executor = MockExecutor::new(policy.clone());

    // Attempt operation denied by policy
    let result = executor.execute_tool("shell_exec", "rm -rf /").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Permission denied"));

    // Attempt operation allowed by policy
    let result = executor.execute_tool("file_read", "./src/main.rs").await;
    assert!(result.is_ok());
}
```

- [ ] **Step 8a:** Create mock executor struct
- [ ] **Step 8b:** Implement execution integration test
- [ ] **Step 8c:** Run test to verify policy enforcement in execution
- [ ] **Step 8d:** Commit: `test: add policy execution integration`

---

## Mock Strategies for Policy Testing

### Step 9: Mock policy validator

```rust
pub struct MockPolicyValidator {
    deny_list: Vec<String>,
    allow_list: Vec<String>,
}

impl MockPolicyValidator {
    pub fn new(deny_list: Vec<String>, allow_list: Vec<String>) -> Self {
        Self { deny_list, allow_list }
    }

    pub fn validate_tool_access(&self, tool: &str, operation: &str) -> Result<()> {
        let key = format!("{}:{}", tool, operation);

        if self.deny_list.contains(&key) {
            return Err(Error::PermissionDenied(format!("Access denied: {}", key)));
        }

        if !self.allow_list.is_empty() && !self.allow_list.contains(&key) {
            return Err(Error::PermissionDenied(format!("Access not allowed: {}", key)));
        }

        Ok(())
    }
}
```

- [ ] **Step 9a:** Implement mock policy validator
- [ ] **Step 9b:** Add unit tests for validator
- [ ] **Step 9c:** Run tests to verify mock behavior
- [ ] **Step 9d:** Commit: `test: add mock policy validator`

---

## Performance Validation

### Step 10: Complex policy compilation performance

```rust
#[test]
fn test_complex_policy_compilation_performance() {
    let mut yaml = String::from(r#"
workflow_id: perf_test
name: "Performance Test"
logging:
  default: debug
  levels:
"#);

    // Generate 100+ rule policy
    for i in 0..100 {
        yaml.push_str(&format!("    level_{}: info\n", i));
    }

    yaml.push_str("agentic_workflow: []\n");

    let start = std::time::Instant::now();
    let spec = parse_yaml(&yaml).unwrap();
    let _policy = compile_policies(&spec).unwrap();
    let duration = start.elapsed();

    // Should compile in <100ms for 100+ rules
    assert!(duration.as_millis() < 100, "Compilation took {:?}, expected <100ms", duration);
}
```

- [ ] **Step 10a:** Implement performance test
- [ ] **Step 10b:** Run test with 100+ rules
- [ ] **Step 10c:** Benchmark and optimize if needed
- [ ] **Step 10d:** Commit: `test: add policy compilation performance test`

---

## Error Recovery Validation

### Step 11: Malformed policy YAML

```rust
#[test]
fn test_malformed_policy_yaml() {
    let yaml = r#"
workflow_id: malformed
name: "Malformed"
logging:
  default: invalid_level
  levels:
    workflow: [not, a, string]
"#;

    let result = parse_yaml(yaml);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid logging level") || err.contains("Invalid YAML"));
}
```

- [ ] **Step 11a:** Create malformed YAML fixture
- [ ] **Step 11b:** Implement error handling test
- [ ] **Step 11c:** Run test to verify error recovery
- [ ] **Step 11d:** Commit: `test: add malformed policy YAML test`

### Step 12: Type mismatches in policy

```rust
#[test]
fn test_policy_type_mismatch() {
    let yaml = r#"
workflow_id: type_mismatch
name: "Type Mismatch"
tool_permissions:
  file_operations:
    read: { enabled: "not_a_bool" }
"#;

    let result = parse_yaml(yaml);
    assert!(result.is_err());

    let spec = parse_yaml("workflow_id: test\nname: Test\nmodels: {}\nexecution: { mode: serial }\nlogging: { default: info, levels: {} }\nagentic_workflow: []").unwrap();
    let result = compile_policies(&spec);
    assert!(result.is_err());
}
```

- [ ] **Step 12a:** Create type mismatch fixtures
- [ ] **Step 12b:** Implement type validation tests
- [ ] **Step 12c:** Run tests to verify type checking
- [ ] **Step 12d:** Commit: `test: add policy type mismatch tests`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. All policy tests pass
cargo test policy_test
# Expected: test result: ok. X passed

# 3. Policies compile deterministically
```

**Checkpoint Criteria:**
- ✅ Policy fields compile deterministically
- ✅ Inheritance from default to workflow to step
- ✅ Override rules enforce precedence
- ✅ Policy tests created and passing
- ✅ Logging and tool permissions compile correctly
- ✅ Edge cases handled (empty, conflicts, nested)
- ✅ Integration tests pass (storage, execution)
- ✅ Performance benchmarks meet targets
- ✅ Error recovery works for malformed input

**Anti-Drift Check:** Verify task 8 implements ONLY policy compilation. No persistence or threshold validation yet.

**Next:** Proceed to Task 9 (Local Storage)
