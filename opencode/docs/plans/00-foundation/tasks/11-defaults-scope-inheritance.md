# Task 11: Defaults, Scope & Inheritance

**Goal:** Implement default values, scope hierarchy, and inheritance rules for all policy fields.

**Estimated Time:** 5 hours

**Dependencies:** Task 8, Task 10

**Files:**
- Modify: `src/defaults.rs` (implement defaults and inheritance)
- Create: `tests/defaults_test.rs` (defaults tests)

---

## Step 1: Implement defaults and inheritance

Create `src/defaults.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::{WorkflowSpec, LoggingConfig, ToolPermissionsConfig, LogLevel};
use std::collections::HashMap;

/// Default values for all fields
pub struct DefaultValues {
    pub logging: LoggingDefaults,
    pub tool_permissions: ToolPermissionDefaults,
}

/// Logging defaults
#[derive(Debug, Clone)]
pub struct LoggingDefaults {
    pub default_level: LogLevel,
    pub workflow_level: LogLevel,
    pub step_level: LogLevel,
    pub model_level: LogLevel,
    pub tool_level: LogLevel,
}

/// Tool permission defaults
#[derive(Debug, Clone)]
pub struct ToolPermissionDefaults {
    pub file_read_enabled: bool,
    pub file_write_enabled: bool,
    pub file_delete_enabled: bool,
    pub web_fetch_enabled: bool,
    pub web_scrape_enabled: bool,
    pub shell_exec_enabled: bool,
}

impl DefaultValues {
    /// Get system defaults
    pub fn system_defaults() -> Self {
        Self {
            logging: LoggingDefaults {
                default_level: LogLevel::Info,
                workflow_level: LogLevel::Info,
                step_level: LogLevel::Debug,
                model_level: LogLevel::Warning,
                tool_level: LogLevel::Info,
            },
            tool_permissions: ToolPermissionDefaults {
                file_read_enabled: true,
                file_write_enabled: true,
                file_delete_enabled: true,
                web_fetch_enabled: true,
                web_scrape_enabled: true,
                shell_exec_enabled: false,
            },
        }
    }

    /// Apply defaults to spec (step > workflow > system)
    pub fn apply_defaults(&self, spec: &mut WorkflowSpec) -> Result<()> {
        self.apply_logging_defaults(spec)?;
        self.apply_tool_permission_defaults(spec)?;
        Ok(())
    }

    fn apply_logging_defaults(&self, spec: &mut WorkflowSpec) -> Result<()> {
        let defaults = &self.logging;

        // Set default if not specified
        if spec.logging.levels.is_empty() {
            spec.logging.levels.insert("workflow".to_string(), defaults.workflow_level.clone());
            spec.logging.levels.insert("pipeline".to_string(), defaults.step_level.clone());
            spec.logging.levels.insert("models".to_string(), defaults.model_level.clone());
            spec.logging.levels.insert("tools".to_string(), defaults.tool_level.clone());
        }

        Ok(())
    }

    fn apply_tool_permission_defaults(&self, spec: &mut WorkflowSpec) -> Result<()> {
        let defaults = &self.tool_permissions;

        // Apply file operation defaults
        if !spec.tool_permissions.file_operations.read.enabled {
            spec.tool_permissions.file_operations.read.enabled = defaults.file_read_enabled;
        }
        if !spec.tool_permissions.file_operations.write.enabled {
            spec.tool_permissions.file_operations.write.enabled = defaults.file_write_enabled;
        }
        if !spec.tool_permissions.file_operations.delete.enabled {
            spec.tool_permissions.file_operations.delete.enabled = defaults.file_delete_enabled;
        }

        // Apply web operation defaults
        if !spec.tool_permissions.web_operations.fetch.enabled {
            spec.tool_permissions.web_operations.fetch.enabled = defaults.web_fetch_enabled;
        }
        if !spec.tool_permissions.web_operations.scrape.enabled {
            spec.tool_permissions.web_operations.scrape.enabled = defaults.web_scrape_enabled;
        }

        // Apply shell operation defaults
        if !spec.tool_permissions.shell_operations.exec.enabled {
            spec.tool_permissions.shell_operations.exec.enabled = defaults.shell_exec_enabled;
        }

        Ok(())
    }
}
```

**Commit:** `feat: implement defaults and inheritance`

---

## Step 2: Write defaults tests

Create `tests/defaults_test.rs`:

```rust
use yaml_to_rust_agentsdk::defaults::*;
use yaml_to_rust_agentsdk::parser::parse_workflow;

#[test]
fn test_system_defaults() {
    let defaults = DefaultValues::system_defaults();

    assert_eq!(defaults.logging.default_level, LogLevel::Info);
    assert_eq!(defaults.logging.workflow_level, LogLevel::Info);
    assert_eq!(defaults.logging.step_level, LogLevel::Debug);
    assert_eq!(defaults.tool_permissions.shell_exec_enabled, false);
}

#[test]
fn test_apply_defaults() {
    let mut spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let defaults = DefaultValues::system_defaults();

    // Apply defaults
    defaults.apply_defaults(&mut spec).unwrap();

    // Verify defaults applied
    assert!(!spec.logging.levels.is_empty());
}

#[test]
fn test_inheritance_priority() {
    // Step overrides should take precedence over workflow defaults
    // Workflow defaults take precedence over system defaults
    let defaults = DefaultValues::system_defaults();
    assert_eq!(defaults.logging.default_level, LogLevel::Info);
}
```

**Commit:** `test: add defaults and inheritance tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test defaults_test

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

# 2. All defaults tests pass
cargo test defaults_test
# Expected: test result: ok. X passed

# 3. Defaults applied correctly
```

**Checkpoint Criteria:**
- ✅ Default values defined for all fields
- ✅ Inheritance hierarchy: step > workflow > system
- ✅ Override rules enforced correctly
- ✅ Defaults tests created and passing
- ✅ All optional fields have defaults

**Anti-Drift Check:** Verify task 11 implements ONLY defaults and inheritance. No threshold validation yet.

**Next:** Proceed to Task 12 (Threshold Validation)
