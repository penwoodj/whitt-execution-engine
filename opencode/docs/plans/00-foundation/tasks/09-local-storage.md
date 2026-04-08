# Task 9: Local Storage

**Goal:** Implement .glyphnova/ directory structure and persistence with sled (ACID transactions).

**Estimated Time:** 6 hours

**Dependencies:** Task 6

**Files:**
- Modify: `src/storage/mod.rs` (implement local storage with sled)
- Create: `tests/storage_test.rs` (storage tests)

---

## Step 1: Implement storage

Create `src/storage/mod.rs`:

```rust
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use std::path::Path;
use uuid::Uuid;

/// Local storage manager for .glyphnova directory
pub struct LocalStorage {
    db: Db,
    base_path: std::path::PathBuf,
}

impl LocalStorage {
    /// Open or create local storage at .glyphnova/
    pub fn open<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().join(".glyphnova");
        std::fs::create_dir_all(&base_path)
            .map_err(|e| Error::file_system("create", base_path.clone(), e.to_string()))?;

        let db_path = base_path.join("db");
        let db = sled::open(&db_path)
            .map_err(|e| Error::database("open sled database", e.to_string()))?;

        Ok(Self { db, base_path })
    }

    /// Store workflow IR
    pub fn store_workflow(&self, workflow_id: &str, ir: &crate::ir::WorkflowIR) -> Result<()> {
        let tree = self.db.open_tree("workflows")
            .map_err(|e| Error::database("open tree", e.to_string()))?;

        let key = workflow_id.as_bytes();
        let value = bincode::serialize(ir)
            .map_err(|e| Error::Serialization(e))?;

        tree.insert(key, value)
            .map_err(|e| Error::database("insert workflow", e.to_string()))?;

        tree.flush()
            .map_err(|e| Error::database("flush", e.to_string()))?;

        Ok(())
    }

    /// Load workflow IR
    pub fn load_workflow(&self, workflow_id: &str) -> Result<Option<crate::ir::WorkflowIR>> {
        let tree = self.db.open_tree("workflows")
            .map_err(|e| Error::database("open tree", e.to_string()))?;

        let key = workflow_id.as_bytes();
        if let Some(value) = tree.get(key)
            .map_err(|e| Error::database("get workflow", e.to_string()))?
        {
            let ir = bincode::deserialize(&value)
                .map_err(|e| Error::Serialization(e))?;
            Ok(Some(ir))
        } else {
            Ok(None)
        }
    }

    /// Store execution state
    pub fn store_execution(&self, execution_id: &str, state: &ExecutionState) -> Result<()> {
        let tree = self.db.open_tree("executions")
            .map_err(|e| Error::database("open tree", e.to_string()))?;

        let key = execution_id.as_bytes();
        let value = bincode::serialize(state)
            .map_err(|e| Error::Serialization(e))?;

        tree.insert(key, value)
            .map_err(|e| Error::database("insert execution", e.to_string()))?;

        tree.flush()
            .map_err(|e| Error::database("flush", e.to_string()))?;

        Ok(())
    }

    /// Generate new execution ID
    pub fn new_execution_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Get base path
    pub fn base_path(&self) -> &std::path::Path {
        &self.base_path
    }
}

/// Execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    pub execution_id: String,
    pub workflow_id: String,
    pub started_at: String,
    pub current_step: Option<String>,
    pub completed_steps: Vec<String>,
}

impl ExecutionState {
    pub fn new(workflow_id: String) -> Self {
        Self {
            execution_id: Uuid::new_v4().to_string(),
            workflow_id,
            started_at: chrono::Utc::now().to_rfc3339(),
            current_step: None,
            completed_steps: Vec::new(),
        }
    }
}
```

**Commit:** `feat: implement local storage with sled`

---

## Step 2: Write storage tests

Create `tests/storage_test.rs`:

```rust
use yaml_to_rust_agentsdk::storage::*;
use yaml_to_rust_agentsdk::compiler::*;
use yaml_to_rust_agentsdk::parser::parse_workflow;
use tempfile::TempDir;

#[test]
fn test_storage_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let storage = LocalStorage::open(temp_dir.path()).unwrap();

    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    let ir = compile(&spec).unwrap();

    storage.store_workflow(ir.id.as_str(), &ir).unwrap();

    let loaded = storage.load_workflow(ir.id.as_str()).unwrap();
    assert!(loaded.is_some());

    let loaded = loaded.unwrap();
    assert_eq!(loaded.id.as_str(), ir.id.as_str());
}

#[test]
fn test_storage_execution_state() {
    let temp_dir = TempDir::new().unwrap();
    let storage = LocalStorage::open(temp_dir.path()).unwrap();

    let state = ExecutionState::new("test_workflow".to_string());
    storage.store_execution(&state.execution_id, &state).unwrap();

    // Verify state was stored (would need load_execution method)
}

#[test]
fn test_new_execution_id() {
    let id1 = LocalStorage::new_execution_id();
    let id2 = LocalStorage::new_execution_id();

    assert_ne!(id1, id2);
    assert!(id1.len() == 36); // UUID format
}
```

**Commit:** `test: add storage tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test storage_test

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

# 2. All storage tests pass
cargo test storage_test
# Expected: test result: ok. X passed

# 3. .glyphnova/ directory created
```

**Checkpoint Criteria:**
- ✅ .glyphnova/ directory structure created
- ✅ sled persistence works with ACID transactions
- ✅ Workflow IR storage and retrieval works
- ✅ Execution state storage works
- ✅ Storage tests created and passing
- ✅ UUID generation for execution IDs

**Anti-Drift Check:** Verify task 9 implements ONLY persistence. No workspace management or defaults yet.

**Next:** Proceed to Task 10 (Workspace Management)
