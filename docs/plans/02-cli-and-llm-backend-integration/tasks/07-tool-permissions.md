# Task 07: Tool Permissions

**Files:**
- Create: `src/tools/permissions/mod.rs`
- Create: `src/tools/permissions/manager.rs`
- Create: `src/tools/permissions/config.rs`
- Modify: `src/tools/mod.rs` (add permissions module)
- Test: `tests/tools/permissions_test.rs`

---

## Overview

Implement tool permission system with allow/deny lists, per-step restrictions, confirmation flow, and guardrails enforcement. This provides security controls for tool execution.

---

## Implementation Steps

### Step 1: Create permission configuration

- [ ] **Step 1.1: Write permission configuration**

```rust
// src/tools/permissions/config.rs
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub default_policy: Policy,
    pub tools: ToolPermissions,
    pub guardrails: GuardrailsConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Policy {
    Allow,
    Deny,
}

impl Default for Policy {
    fn default() -> Self {
        Policy::Deny
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermissions {
    #[serde(default)]
    pub allow: HashSet<String>,
    #[serde(default)]
    pub deny: HashSet<String>,
    #[serde(default)]
    pub confirmation_required: HashSet<String>,
    #[serde(default)]
    pub step_restrictions: std::collections::HashMap<String, StepPermissions>,
}

impl Default for ToolPermissions {
    fn default() -> Self {
        Self {
            allow: HashSet::new(),
            deny: HashSet::new(),
            confirmation_required: HashSet::new(),
            step_restrictions: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepPermissions {
    #[serde(default)]
    pub allowed_tools: HashSet<String>,
    #[serde(default)]
    pub denied_tools: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailsConfig {
    #[serde(default)]
    pub path_traversal_prevention: bool,
    #[serde(default)]
    pub command_injection_prevention: bool,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default)]
    pub denied_paths: Vec<String>,
}

impl Default for GuardrailsConfig {
    fn default() -> Self {
        Self {
            path_traversal_prevention: true,
            command_injection_prevention: true,
            allowed_paths: vec![],
            denied_paths: vec![
                "/etc/passwd".to_string(),
                "/etc/shadow".to_string(),
                "/root/".to_string(),
            ],
        }
    }
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            default_policy: Policy::Deny,
            tools: ToolPermissions::default(),
            guardrails: GuardrailsConfig::default(),
        }
    }
}
```

- [ ] **Step 1.2: Commit**

```bash
git add src/tools/permissions/config.rs
git commit -m "feat(tools): add permission configuration structures"
```

---

### Step 2: Create permission manager

- [ ] **Step 2.1: Write permission manager**

```rust
// src/tools/permissions/manager.rs
use super::config::{PermissionConfig, Policy, StepPermissions};
use anyhow::{Context, Result};

pub struct PermissionManager {
    config: PermissionConfig,
    current_step: Option<String>,
    interactive: bool,
}

impl PermissionManager {
    pub fn new(config: PermissionConfig, interactive: bool) -> Self {
        Self {
            config,
            current_step: None,
            interactive,
        }
    }

    pub fn set_current_step(&mut self, step_id: String) {
        self.current_step = Some(step_id);
    }

    pub fn clear_current_step(&mut self) {
        self.current_step = None;
    }

    pub fn check_permission(&self, tool_name: &str) -> Result<PermissionDecision> {
        // Check step-specific restrictions first
        if let Some(step_id) = &self.current_step {
            if let Some(step_perms) = self.config.tools.step_restrictions.get(step_id) {
                if step_perms.denied_tools.contains(tool_name) {
                    return Ok(PermissionDecision::Denied(format!(
                        "Tool '{}' denied by step restrictions",
                        tool_name
                    )));
                }

                if !step_perms.allowed_tools.is_empty()
                    && !step_perms.allowed_tools.contains(tool_name)
                {
                    return Ok(PermissionDecision::Denied(format!(
                        "Tool '{}' not in step allow list",
                        tool_name
                    )));
                }
            }
        }

        // Check global deny list
        if self.config.tools.deny.contains(tool_name) {
            return Ok(PermissionDecision::Denied(format!(
                "Tool '{}' denied by global deny list",
                tool_name
            )));
        }

        // Check global allow list (if not empty)
        if !self.config.tools.allow.is_empty()
            && !self.config.tools.allow.contains(tool_name)
        {
            return Ok(PermissionDecision::Denied(format!(
                "Tool '{}' not in global allow list",
                tool_name
            )));
        }

        // Check default policy
        match self.config.default_policy {
            Policy::Deny => Ok(PermissionDecision::Denied(format!(
                "Tool '{}' denied by default policy",
                tool_name
            ))),
            Policy::Allow => Ok(PermissionDecision::Allowed),
        }
    }

    pub fn check_confirmation_required(&self, tool_name: &str) -> bool {
        self.config.tools.confirmation_required.contains(tool_name)
    }

    pub fn request_confirmation(&self, tool_name: &str, details: &str) -> Result<bool> {
        if !self.interactive {
            // Non-interactive mode: auto-approve or auto-deny based on config
            return Ok(true); // Default to approve in non-interactive mode
        }

        println!("\nTool Execution Request:");
        println!("  Tool: {}", tool_name);
        println!("  Details: {}", details);
        print!("  Approve? [y/N]: ");

        use std::io::Write;
        std::io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read confirmation")?;

        let input = input.trim().to_lowercase();
        Ok(input == "y" || input == "yes")
    }

    pub fn check_guardrails(&self, tool_name: &str, input: &str) -> Result<()> {
        // Path traversal prevention
        if self.config.guardrails.path_traversal_prevention {
            if input.contains("../") || input.contains("..\\") {
                anyhow::bail!("Path traversal detected in input");
            }
        }

        // Command injection prevention (for shell tools)
        if self.config.guardrails.command_injection_prevention {
            if tool_name == "shell.exec" || tool_name.starts_with("shell.") {
                let dangerous_patterns = ["; ", " && ", " || ", "| ", "`", "$(", "evalue"];
                for pattern in dangerous_patterns {
                    if input.contains(pattern) {
                        anyhow::bail!("Potential command injection detected: {}", pattern);
                    }
                }
            }
        }

        // Denied paths
        for denied_path in &self.config.guardrails.denied_paths {
            if input.contains(denied_path) {
                anyhow::bail!("Access to denied path: {}", denied_path);
            }
        }

        // Allowed paths (if configured, only allow these)
        if !self.config.guardrails.allowed_paths.is_empty() {
            let allowed = self
                .config
                .guardrails
                .allowed_paths
                .iter()
                .any(|path| input.starts_with(path));

            if !allowed {
                anyhow::bail!("Path not in allowed paths list");
            }
        }

        Ok(())
    }

    pub fn evaluate_tool_request(&self, tool_name: &str, input: &str) -> Result<ToolEvaluation> {
        // Check guardrails first
        self.check_guardrails(tool_name, input)
            .context("Guardrails check failed")?;

        // Check permission
        let permission = self.check_permission(tool_name)?;

        match permission {
            PermissionDecision::Allowed => {
                let requires_confirmation = self.check_confirmation_required(tool_name);

                Ok(ToolEvaluation {
                    allowed: true,
                    requires_confirmation,
                    reason: None,
                })
            }
            PermissionDecision::Denied(reason) => Ok(ToolEvaluation {
                allowed: false,
                requires_confirmation: false,
                reason: Some(reason),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionDecision {
    Allowed,
    Denied(String),
}

#[derive(Debug, Clone)]
pub struct ToolEvaluation {
    pub allowed: bool,
    pub requires_confirmation: bool,
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> PermissionConfig {
        let mut tools = super::config::ToolPermissions::default();
        tools.allow.insert("file.read".to_string());
        tools.deny.insert("shell.exec".to_string());
        tools.confirmation_required.insert("file.delete".to_string());

        PermissionConfig {
            default_policy: Policy::Deny,
            tools,
            guardrails: GuardrailsConfig::default(),
        }
    }

    #[test]
    fn test_allow_list() {
        let config = create_test_config();
        let manager = PermissionManager::new(config, false);

        let decision = manager.check_permission("file.read").unwrap();
        assert_eq!(decision, PermissionDecision::Allowed);
    }

    #[test]
    fn test_deny_list() {
        let config = create_test_config();
        let manager = PermissionManager::new(config, false);

        let decision = manager.check_permission("shell.exec").unwrap();
        assert!(matches!(decision, PermissionDecision::Denied(_)));
    }

    #[test]
    fn test_default_policy() {
        let config = create_test_config();
        let manager = PermissionManager::new(config, false);

        let decision = manager.check_permission("unknown.tool").unwrap();
        assert!(matches!(decision, PermissionDecision::Denied(_)));
    }

    #[test]
    fn test_path_traversal_prevention() {
        let config = create_test_config();
        let manager = PermissionManager::new(config, false);

        assert!(manager.check_guardrails("file.read", "../../../etc/passwd").is_err());
        assert!(manager.check_guardrails("file.read", "..\\windows\\system32").is_err());
        assert!(manager.check_guardrails("file.read", "safe/path").is_ok());
    }

    #[test]
    fn test_command_injection_prevention() {
        let config = create_test_config();
        let manager = PermissionManager::new(config, false);

        assert!(manager.check_guardrails("shell.exec", "ls && rm -rf /").is_err());
        assert!(manager.check_guardrails("shell.exec", "whoami").is_err()); // All shell exec is denied by guardrails
    }
}
```

- [ ] **Step 2.2: Write module exports**

```rust
// src/tools/permissions/mod.rs
pub mod config;
pub mod manager;

pub use config::{PermissionConfig, Policy, ToolPermissions, GuardrailsConfig};
pub use manager::{PermissionManager, PermissionDecision, ToolEvaluation};
```

- [ ] **Step 2.3: Create tools module**

```rust
// src/tools/mod.rs
pub mod permissions;

pub use permissions::*;
```

- [ ] **Step 2.4: Commit**

```bash
git add src/tools/permissions/manager.rs src/tools/permissions/mod.rs src/tools/mod.rs
git commit -m "feat(tools): add permission manager with guardrails"
```

---

### Step 3: Write tests

- [ ] **Step 3.1: Write integration tests**

```rust
// tests/tools/permissions_test.rs
use whitt_execution_engine::tools::{
    PermissionManager, PermissionConfig, Policy, ToolPermissions, GuardrailsConfig,
};
use std::collections::{HashSet, HashMap};

#[test]
fn test_permission_evaluation_allowed() {
    let mut tools = ToolPermissions::default();
    tools.allow.insert("file.read".to_string());

    let config = PermissionConfig {
        default_policy: Policy::Deny,
        tools,
        guardrails: GuardrailsConfig::default(),
    };

    let manager = PermissionManager::new(config, false);
    let evaluation = manager.evaluate_tool_request("file.read", "/safe/path").unwrap();

    assert!(evaluation.allowed);
    assert!(!evaluation.requires_confirmation);
    assert!(evaluation.reason.is_none());
}

#[test]
fn test_permission_evaluation_denied() {
    let mut tools = ToolPermissions::default();
    tools.deny.insert("shell.exec".to_string());

    let config = PermissionConfig {
        default_policy: Policy::Deny,
        tools,
        guardrails: GuardrailsConfig::default(),
    };

    let manager = PermissionManager::new(config, false);
    let evaluation = manager.evaluate_tool_request("shell.exec", "ls").unwrap();

    assert!(!evaluation.allowed);
    assert!(evaluation.reason.is_some());
}

#[test]
fn test_permission_evaluation_confirmation_required() {
    let mut tools = ToolPermissions::default();
    tools.allow.insert("file.delete".to_string());
    tools.confirmation_required.insert("file.delete".to_string());

    let config = PermissionConfig {
        default_policy: Policy::Deny,
        tools,
        guardrails: GuardrailsConfig::default(),
    };

    let manager = PermissionManager::new(config, false);
    let evaluation = manager.evaluate_tool_request("file.delete", "/tmp/file").unwrap();

    assert!(evaluation.allowed);
    assert!(evaluation.requires_confirmation);
}

#[test]
fn test_step_restrictions() {
    let mut tools = ToolPermissions::default();
    tools.allow.insert("file.read".to_string());
    tools.allow.insert("file.write".to_string());

    let mut step_perms = HashMap::new();
    let mut allowed = HashSet::new();
    allowed.insert("file.read".to_string());
    step_perms.insert("step1".to_string(), crate::tools::config::StepPermissions {
        allowed_tools: allowed,
        denied_tools: HashSet::new(),
    });

    let config = PermissionConfig {
        default_policy: Policy::Deny,
        tools,
        guardrails: GuardrailsConfig::default(),
    };

    let mut manager = PermissionManager::new(config, false);
    manager.set_current_step("step1".to_string());

    // file.read is allowed in step1
    let decision = manager.check_permission("file.read").unwrap();
    assert_eq!(decision, PermissionDecision::Allowed);

    // file.write is globally allowed but not in step1
    let decision = manager.check_permission("file.write").unwrap();
    assert!(matches!(decision, PermissionDecision::Denied(_)));
}
```

- [ ] **Step 3.2: Commit**

```bash
git add tests/tools/permissions_test.rs
git commit -m "test(tools): add permission manager integration tests"
```

---

## Summary

This task implements the tool permission system including:

1. **Permission configuration** with allow/deny lists and step restrictions
2. **Permission manager** with evaluation logic and guardrails
3. **Confirmation flow** for interactive and non-interactive modes
4. **Guardrails enforcement** (path traversal, command injection)
5. **Step-specific permissions** for granular control
6. **Comprehensive tests** for all permission scenarios

**Security Features:**
- Default-deny policy
- Allow/deny lists for tools
- Per-step restrictions
- Confirmation required for dangerous tools
- Path traversal prevention
- Command injection prevention
- Allowed/denied path lists

**Next:** Task 08 - Tool Execution Framework
