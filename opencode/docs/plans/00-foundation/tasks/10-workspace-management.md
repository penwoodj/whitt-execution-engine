# Task 10: Workspace Management

**Goal:** Implement workspace path resolution and directory creation with permission checks.

**Estimated Time:** 4 hours

**Dependencies:** Task 9

**Files:**
- Modify: `src/workspace.rs` (implement workspace management)
- Create: `tests/workspace_test.rs` (workspace tests)

---

## Step 1: Implement workspace management

Create `src/workspace.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::WorkspaceConfig;
use std::path::Path;

/// Workspace manager
pub struct WorkspaceManager {
    config: WorkspaceConfig,
}

impl WorkspaceManager {
    /// Create new workspace manager
    pub fn new(config: WorkspaceConfig) -> Self {
        Self { config }
    }

    /// Initialize workspace directories
    pub fn initialize(&self) -> Result<()> {
        self.create_dir(&self.config.root_path)?;
        self.create_dir(&self.config.output_path)?;
        self.create_dir(&self.config.checkpoint_path)?;
        self.create_dir(&self.config.log_path)?;
        self.create_dir(&self.config.metrics_path)?;
        self.create_dir(&self.config.temp_path)?;

        Ok(())
    }

    /// Create directory if it doesn't exist
    fn create_dir(&self, path: &str) -> Result<()> {
        std::fs::create_dir_all(path)
            .map_err(|e| Error::file_system("create", path.into(), e.to_string()))?;
        Ok(())
    }

    /// Resolve path relative to workspace root
    pub fn resolve_path(&self, relative_path: &str) -> String {
        if Path::new(relative_path).is_absolute() {
            relative_path.to_string()
        } else {
            format!("{}/{}", self.config.root_path, relative_path)
        }
    }

    /// Get workspace configuration
    pub fn config(&self) -> &WorkspaceConfig {
        &self.config
    }

    /// Check if path is allowed (within workspace)
    pub fn is_path_allowed(&self, path: &str) -> bool {
        let resolved = self.resolve_path(path);
        Path::new(&resolved).starts_with(&self.config.root_path)
    }

    /// Clean temp directory
    pub fn clean_temp(&self) -> Result<()> {
        let temp_path = &self.config.temp_path;
        if Path::new(temp_path).exists() {
            std::fs::remove_dir_all(temp_path)
                .map_err(|e| Error::file_system("remove", temp_path.into(), e.to_string()))?;
        }
        Ok(())
    }
}
```

**Commit:** `feat: implement workspace management`

---

## Step 2: Write workspace tests

Create `tests/workspace_test.rs`:

```rust
use yaml_to_rust_agentsdk::workspace::*;
use yaml_to_rust_agentsdk::schema::WorkspaceConfig;
use tempfile::TempDir;

#[test]
fn test_workspace_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let config = WorkspaceConfig {
        root_path: temp_dir.path().to_str().unwrap().to_string(),
        ..Default::default()
    };

    let manager = WorkspaceManager::new(config);
    manager.initialize().unwrap();

    assert!(Path::new(manager.config().root_path.as_str()).exists());
    assert!(Path::new(manager.config().output_path.as_str()).exists());
}

#[test]
fn test_path_resolution() {
    let config = WorkspaceConfig {
        root_path: "/workspace".to_string(),
        ..Default::default()
    };

    let manager = WorkspaceManager::new(config);

    let resolved = manager.resolve_path("output/test.txt");
    assert_eq!(resolved, "/workspace/output/test.txt");

    let resolved = manager.resolve_path("/absolute/path");
    assert_eq!(resolved, "/absolute/path");
}

#[test]
fn test_path_allowed() {
    let config = WorkspaceConfig {
        root_path: "/workspace".to_string(),
        ..Default::default()
    };

    let manager = WorkspaceManager::new(config);

    assert!(manager.is_path_allowed("output/test.txt"));
    assert!(manager.is_path_allowed("/workspace/output/test.txt"));
    assert!(!manager.is_path_allowed("/etc/passwd"));
}

#[test]
fn test_clean_temp() {
    let temp_dir = TempDir::new().unwrap();
    let config = WorkspaceConfig {
        root_path: temp_dir.path().to_str().unwrap().to_string(),
        temp_path: format!("{}/temp", temp_dir.path().to_str().unwrap()),
        ..Default::default()
    };

    let manager = WorkspaceManager::new(config);
    manager.initialize().unwrap();

    // Create a temp file
    let temp_file = format!("{}/temp/test.txt", temp_dir.path().to_str().unwrap());
    std::fs::write(&temp_file, "test").unwrap();

    manager.clean_temp().unwrap();
    assert!(!Path::new(&config.temp_path).exists());
}
```

**Commit:** `test: add workspace management tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test workspace_test

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

# 2. All workspace tests pass
cargo test workspace_test
# Expected: test result: ok. X passed

# 3. Workspace directories created correctly
```

**Checkpoint Criteria:**
- ✅ Workspace directories created on initialization
- ✅ Path resolution works (absolute/relative)
- ✅ Permission checks enforce workspace boundaries
- ✅ Workspace tests created and passing
- ✅ Temp directory cleanup works

**Anti-Drift Check:** Verify task 10 implements ONLY workspace management. No defaults or threshold validation yet.

**Next:** Proceed to Task 11 (Defaults, Scope & Inheritance)
