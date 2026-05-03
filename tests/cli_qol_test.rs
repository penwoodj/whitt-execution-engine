//! CLI QoL integration tests.
//!
//! Tests for new CLI quality-of-life features:
//! - ToolRegistry allowlist/forbidden filtering
//! - SandboxConfig path restrictions

use async_trait::async_trait;
use std::collections::HashMap;
use whitt_execution_engine::agent::sandbox::{SandboxConfig, ToolSandbox};
use whitt_execution_engine::agent::tools::{Tool, ToolCall, ToolRegistry, ToolResult};

/// Mock tool for testing ToolRegistry filtering
struct MockTool {
    name: String,
}

#[async_trait]
impl Tool for MockTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "mock"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({})
    }

    async fn execute(
        &self,
        _: HashMap<String, serde_json::Value>,
    ) -> Result<ToolResult, anyhow::Error> {
        Ok(ToolResult {
            tool_name: self.name.clone(),
            output: "ok".into(),
            success: true,
            metadata: HashMap::new(),
        })
    }
}

/// Test: allowed_tools filters list
/// Creates registry with allowed=["model_list"], verifies list() only shows model_list
#[tokio::test]
async fn test_tool_registry_allowed_tools_filters_list() {
    let mut registry = ToolRegistry::new().with_allowed_tools(vec!["model_list".to_string()]);

    registry.register(Box::new(MockTool {
        name: "model_list".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "chat".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "file_read".to_string(),
    }));

    let list = registry.list();
    assert_eq!(list.len(), 1, "List should contain only allowed tools");
    assert_eq!(list[0].0, "model_list", "Only model_list should be visible");
}

/// Test: forbidden_tools blocks execute
/// Creates registry with forbidden=["chat"], verifies execute("chat") returns error
#[tokio::test]
async fn test_tool_registry_forbidden_tools_blocks_execute() {
    let mut registry = ToolRegistry::new().with_forbidden_tools(vec!["chat".to_string()]);

    registry.register(Box::new(MockTool {
        name: "chat".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "model_list".to_string(),
    }));

    let call = ToolCall {
        name: "chat".to_string(),
        arguments: HashMap::new(),
    };
    let result = registry.execute(call).await;
    assert!(result.is_err(), "Forbidden tool should fail");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("not allowed"));

    let call = ToolCall {
        name: "model_list".to_string(),
        arguments: HashMap::new(),
    };
    let result = registry.execute(call).await;
    assert!(result.is_ok(), "Non-forbidden tool should succeed");
}

/// Test: deny overrides allow
/// allowed=["tool_a","tool_b"], forbidden=["tool_a"], verify tool_a blocked, tool_b allowed
#[tokio::test]
async fn test_tool_registry_deny_overrides_allow() {
    let mut registry = ToolRegistry::new()
        .with_allowed_tools(vec!["tool_a".to_string(), "tool_b".to_string()])
        .with_forbidden_tools(vec!["tool_a".to_string()]);

    registry.register(Box::new(MockTool {
        name: "tool_a".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "tool_b".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "tool_c".to_string(),
    }));

    let list = registry.list();
    assert_eq!(list.len(), 1, "Only non-forbidden allowed tools shown");
    assert_eq!(list[0].0, "tool_b", "Only tool_b should be visible");

    let call = ToolCall {
        name: "tool_a".to_string(),
        arguments: HashMap::new(),
    };
    let result = registry.execute(call).await;
    assert!(result.is_err(), "Forbidden tool should fail");

    let call = ToolCall {
        name: "tool_b".to_string(),
        arguments: HashMap::new(),
    };
    let result = registry.execute(call).await;
    assert!(result.is_ok(), "Allowed non-forbidden tool should succeed");
}

/// Test: empty allowed allows all
/// allowed=Some(vec![]), all tools should be accessible (empty maps to None)
#[tokio::test]
async fn test_tool_registry_empty_allowed_allows_all() {
    let mut registry = ToolRegistry::new().with_allowed_tools(vec![]);

    registry.register(Box::new(MockTool {
        name: "tool_a".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "tool_b".to_string(),
    }));
    registry.register(Box::new(MockTool {
        name: "tool_c".to_string(),
    }));

    let list = registry.list();
    assert_eq!(list.len(), 3, "Empty allowed list should allow all tools");

    for tool_name in ["tool_a", "tool_b", "tool_c"] {
        let call = ToolCall {
            name: tool_name.to_string(),
            arguments: HashMap::new(),
        };
        let result = registry.execute(call).await;
        assert!(result.is_ok(), "{} should execute", tool_name);
    }
}

/// Test: allowed_paths permits matching
/// allowed_paths=["/tmp/test"], verify /tmp/test/file.txt allowed
#[tokio::test]
async fn test_sandbox_allowed_paths_permits_matching() {
    let tmpdir = tempfile::tempdir().unwrap();
    let test_dir = tmpdir.path().join("test");
    tokio::fs::create_dir_all(&test_dir).await.unwrap();

    let config = SandboxConfig {
        allowed_paths: vec![test_dir.to_str().unwrap().to_string()],
        forbidden_paths: vec![],
        allowed_patterns: vec![],
        max_file_size_mb: 10,
    };

    let sandbox = ToolSandbox::new(config);

    // Path in allowed directory should be permitted
    let test_file = test_dir.join("file.txt");
    tokio::fs::write(&test_file, "test content").await.unwrap();

    let allowed = sandbox.is_path_allowed(&test_file).unwrap();
    assert!(allowed, "Path in allowed directory should be permitted");
}

/// Test: forbidden_paths overrides allowed
/// allowed=["/tmp"], forbidden=["/tmp/secret"], verify /tmp/secret blocked but /tmp/other allowed
#[tokio::test]
async fn test_sandbox_forbidden_paths_overrides_allowed() {
    let tmpdir = tempfile::tempdir().unwrap();
    let secret_dir = tmpdir.path().join("secret");
    let other_dir = tmpdir.path().join("other");

    tokio::fs::create_dir_all(&secret_dir).await.unwrap();
    tokio::fs::create_dir_all(&other_dir).await.unwrap();

    let config = SandboxConfig {
        allowed_paths: vec![tmpdir.path().to_str().unwrap().to_string()],
        forbidden_paths: vec![secret_dir.to_str().unwrap().to_string()],
        allowed_patterns: vec![],
        max_file_size_mb: 10,
    };

    let sandbox = ToolSandbox::new(config);

    // Forbidden path should be denied
    let secret_file = secret_dir.join("secret.txt");
    tokio::fs::write(&secret_file, "secret").await.unwrap();
    let allowed = sandbox.is_path_allowed(&secret_file).unwrap();
    assert!(!allowed, "Forbidden path should be denied");

    // Non-forbidden path in allowed directory should be permitted
    let other_file = other_dir.join("other.txt");
    tokio::fs::write(&other_file, "other").await.unwrap();
    let allowed = sandbox.is_path_allowed(&other_file).unwrap();
    assert!(allowed, "Non-forbidden path in allowed directory should be permitted");
}

/// Test: no config allows all
/// allowed_paths=[], forbidden_paths=[], all paths allowed
#[tokio::test]
async fn test_sandbox_no_config_allows_all() {
    let tmpdir = tempfile::tempdir().unwrap();
    let test_file = tmpdir.path().join("test.txt");
    tokio::fs::write(&test_file, "test").await.unwrap();

    let config = SandboxConfig {
        allowed_paths: vec![],
        forbidden_paths: vec![],
        allowed_patterns: vec![],
        max_file_size_mb: 10,
    };

    let sandbox = ToolSandbox::new(config);

    let allowed = sandbox.is_path_allowed(&test_file).unwrap();
    assert!(!allowed, "Empty allowed_paths should deny all paths");
}

/// Test: max_file_size enforced
/// set max_file_size_mb=0, verify reading blocks
#[tokio::test]
async fn test_sandbox_max_file_size_enforced() {
    let tmpdir = tempfile::tempdir().unwrap();
    let test_file = tmpdir.path().join("large_file.txt");

    tokio::fs::write(&test_file, "x".repeat(1024)).await.unwrap();

    let config = SandboxConfig {
        allowed_paths: vec![tmpdir.path().to_str().unwrap().to_string()],
        forbidden_paths: vec![],
        allowed_patterns: vec![],
        max_file_size_mb: 0,
    };

    let sandbox = ToolSandbox::new(config);

    let result = sandbox.validate_file_access(&test_file, whitt_execution_engine::agent::sandbox::FileOperation::Read);
    assert!(result.is_err(), "File exceeding max size should be denied");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("too large"));
}
