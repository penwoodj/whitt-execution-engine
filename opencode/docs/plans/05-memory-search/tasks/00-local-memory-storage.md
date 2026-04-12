# Task 00: Local Memory Storage

**Estimated Time:** 1-2 weeks
**Dependencies:** Phase 0-4 complete
**Priority:** CRITICAL (blocks all other memory/search tasks)

## Overview

Implement the foundational memory storage system with structured and unstructured memory types, CRUD operations, versioning, and persistent storage in `./workspace/memory/`.

## Files

### Create
- `crates/memory/Cargo.toml` - Memory crate manifest
- `crates/memory/src/lib.rs` - Public API exports
- `crates/memory/src/storage.rs` - Storage interface and implementation
- `crates/memory/src/schema.rs` - Memory data structures
- `crates/memory/src/versioning.rs` - Versioned reference system
- `crates/memory/src/operations.rs` - CRUD operations
- `crates/memory/src/error.rs` - Error types

### Modify
- `Cargo.toml` - Add memory workspace member

### Test
- `crates/memory/tests/integration_test.rs` - Integration tests
- `crates/memory/src/storage.rs` - Unit tests embedded

---

## Step-by-Step Implementation

### Step 1: Create memory crate structure

```bash
# Create crate directory
mkdir -p crates/memory/src

# Initialize Cargo.toml
cat > crates/memory/Cargo.toml << 'EOF'
[package]
name = "agentsdk-memory"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
sha2 = "0.10"
async-trait = "0.1"
EOF
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 1: Create memory crate structure**

### Step 2: Write error types

Create `crates/memory/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Memory not found: {0}")]
    NotFound(String),

    #[error("Version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: u64, actual: u64 },

    #[error("Invalid memory ID: {0}")]
    InvalidId(String),

    #[error("Storage limit exceeded: {max} bytes")]
    StorageLimitExceeded { max: u64 },

    #[error("Corrupted data: {0}")]
    Corrupted(String),
}
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 2: Write error types**

### Step 3: Write memory schema

Create `crates/memory/src/schema.rs`:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Memory ID wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s)
            .map_err(|e| format!("Invalid UUID: {}", e))?;
        Ok(Self(uuid))
    }

    pub fn as_str(&self) -> &str {
        self.0.to_hyphenated().to_string().leak()
    }
}

/// Memory types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    Structured,
    Unstructured,
}

/// Structured memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredMemory {
    pub id: MemoryId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
    pub schema_version: String,
    pub data: serde_json::Value,
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Unstructured memory entry (text blobs, logs, history)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstructuredMemory {
    pub id: MemoryId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
    pub content: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Versioned memory reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReference {
    pub memory_id: MemoryId,
    pub version: u64,
    pub created_at: DateTime<Utc>,
    pub hash: String, // SHA-256 hash of content
}
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 3: Write memory schema**

### Step 4: Write storage interface

Create `crates/memory/src/storage.rs`:

```rust
use async_trait::async_trait;
use super::schema::{MemoryId, MemoryType, StructuredMemory, UnstructuredMemory};
use super::error::MemoryError;
use super::versioning::MemoryReference;

/// Storage backend trait
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn initialize(&self) -> Result<(), MemoryError>;

    async fn store_structured(
        &self,
        memory: &StructuredMemory,
    ) -> Result<MemoryReference, MemoryError>;

    async fn store_unstructured(
        &self,
        memory: &UnstructuredMemory,
    ) -> Result<MemoryReference, MemoryError>;

    async fn retrieve_structured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<StructuredMemory, MemoryError>;

    async fn retrieve_unstructured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<UnstructuredMemory, MemoryError>;

    async fn delete(
        &self,
        id: &MemoryId,
    ) -> Result<(), MemoryError>;

    async fn list_by_type(
        &self,
        memory_type: MemoryType,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError>;

    async fn list_by_tags(
        &self,
        tags: &[String],
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError>;
}

/// Filesystem storage implementation
pub struct FilesystemStorage {
    base_path: std::path::PathBuf,
}

impl FilesystemStorage {
    pub fn new(base_path: impl AsRef<std::path::Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    fn structured_path(&self, id: &MemoryId) -> std::path::PathBuf {
        self.base_path
            .join("structured")
            .join(format!("{}.json", id.as_str()))
    }

    fn unstructured_path(&self, id: &MemoryId) -> std::path::PathBuf {
        self.base_path
            .join("unstructured")
            .join(format!("{}.txt", id.as_str()))
    }

    async fn ensure_directories(&self) -> Result<(), MemoryError> {
        tokio::fs::create_dir_all(self.base_path.join("structured")).await?;
        tokio::fs::create_dir_all(self.base_path.join("unstructured")).await?;
        Ok(())
    }
}

#[async_trait]
impl StorageBackend for FilesystemStorage {
    async fn initialize(&self) -> Result<(), MemoryError> {
        self.ensure_directories().await
    }

    async fn store_structured(
        &self,
        memory: &StructuredMemory,
    ) -> Result<MemoryReference, MemoryError> {
        let path = self.structured_path(&memory.id);

        let content = serde_json::to_vec_pretty(memory)?;
        let hash = format!("{:x}", sha2::Sha256::digest(&content));

        tokio::fs::write(&path, content).await?;

        let reference = MemoryReference {
            memory_id: memory.id.clone(),
            version: memory.version,
            created_at: memory.updated_at,
            hash,
        };

        Ok(reference)
    }

    async fn store_unstructured(
        &self,
        memory: &UnstructuredMemory,
    ) -> Result<MemoryReference, MemoryError> {
        let path = self.unstructured_path(&memory.id);

        let content_bytes = memory.content.as_bytes();
        let hash = format!("{:x}", sha2::Sha256::digest(content_bytes));

        tokio::fs::write(&path, &memory.content).await?;

        let reference = MemoryReference {
            memory_id: memory.id.clone(),
            version: memory.version,
            created_at: memory.updated_at,
            hash,
        };

        Ok(reference)
    }

    async fn retrieve_structured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<StructuredMemory, MemoryError> {
        let path = self.structured_path(id);

        let content = tokio::fs::read(&path).await?;
        let mut memory: StructuredMemory = serde_json::from_slice(&content)?;

        if let Some(expected_version) = version {
            if memory.version != expected_version {
                return Err(MemoryError::VersionConflict {
                    expected: expected_version,
                    actual: memory.version,
                });
            }
        }

        Ok(memory)
    }

    async fn retrieve_unstructured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<UnstructuredMemory, MemoryError> {
        let path = self.unstructured_path(id);

        let content = tokio::fs::read_to_string(&path).await?;

        // For unstructured, we need to rebuild the structure
        // In a real implementation, we'd store metadata alongside content
        // For now, we'll just create a minimal structure
        let memory = UnstructuredMemory {
            id: id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 1,
            content,
            content_type: "text/plain".to_string(),
            size_bytes: content.len() as u64,
            tags: vec![],
            metadata: std::collections::HashMap::new(),
        };

        if let Some(expected_version) = version {
            if memory.version != expected_version {
                return Err(MemoryError::VersionConflict {
                    expected: expected_version,
                    actual: memory.version,
                });
            }
        }

        Ok(memory)
    }

    async fn delete(&self, id: &MemoryId) -> Result<(), MemoryError> {
        // Try both paths, ignore NotFound
        let structured_path = self.structured_path(id);
        let unstructured_path = self.unstructured_path(id);

        let _ = tokio::fs::remove_file(structured_path).await;
        let _ = tokio::fs::remove_file(unstructured_path).await;

        Ok(())
    }

    async fn list_by_type(
        &self,
        memory_type: MemoryType,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError> {
        let dir = match memory_type {
            MemoryType::Structured => self.base_path.join("structured"),
            MemoryType::Unstructured => self.base_path.join("unstructured"),
        };

        let mut entries = tokio::fs::read_dir(dir).await?;
        let mut ids = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") || filename.ends_with(".txt") {
                    let id_str = filename
                        .strip_suffix(".json")
                        .or_else(|| filename.strip_suffix(".txt"))
                        .unwrap_or(filename);

                    if let Ok(id) = MemoryId::from_string(id_str) {
                        ids.push(id);
                    }
                }
            }
        }

        ids.sort_by(|a, b| a.0.cmp(&b.0));

        let start = offset.min(ids.len());
        let end = (offset + limit).min(ids.len());
        Ok(ids[start..end].to_vec())
    }

    async fn list_by_tags(
        &self,
        tags: &[String],
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError> {
        // For now, this is expensive - we have to load all structured memories
        // In a real implementation, we'd have an index
        let mut ids = Vec::new();

        let structured_ids = self.list_by_type(MemoryType::Structured, usize::MAX, 0).await?;

        for id in structured_ids {
            if let Ok(memory) = self.retrieve_structured(&id, None).await {
                if tags.iter().all(|tag| memory.tags.contains(tag)) {
                    ids.push(id);
                }
            }
        }

        ids.sort_by(|a, b| a.0.cmp(&b.0));

        let start = offset.min(ids.len());
        let end = (offset + limit).min(ids.len());
        Ok(ids[start..end].to_vec())
    }
}
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 4: Write storage interface**

### Step 5: Write versioning system

Create `crates/memory/src/versioning.rs`:

```rust
use super::schema::{MemoryId, StructuredMemory, UnstructuredMemory};
use super::error::MemoryError;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

pub struct VersionManager {
    base_path: PathBuf,
    versions: RwLock<HashMap<String, Vec<u64>>>, // memory_id -> list of versions
}

impl VersionManager {
    pub fn new(base_path: impl AsRef<std::path::Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            versions: RwLock::new(HashMap::new()),
        }
    }

    pub fn version_path(&self, id: &MemoryId, version: u64) -> PathBuf {
        self.base_path
            .join("versions")
            .join(format!("{}_v{}.json", id.as_str(), version))
    }

    pub async fn create_version(
        &self,
        id: &MemoryId,
        memory: &StructuredMemory,
    ) -> Result<(), MemoryError> {
        let path = self.version_path(id, memory.version);
        let content = serde_json::to_vec_pretty(memory)?;

        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        tokio::fs::write(&path, content).await?;

        let mut versions = self.versions.write().await;
        versions
            .entry(id.as_str().to_string())
            .or_insert_with(Vec::new)
            .push(memory.version);

        Ok(())
    }

    pub async fn get_version(
        &self,
        id: &MemoryId,
        version: u64,
    ) -> Result<StructuredMemory, MemoryError> {
        let path = self.version_path(id, version);

        if !path.exists() {
            return Err(MemoryError::NotFound(format!(
                "Version {} for memory {} not found",
                version,
                id.as_str()
            )));
        }

        let content = tokio::fs::read(&path).await?;
        let memory: StructuredMemory = serde_json::from_slice(&content)?;

        Ok(memory)
    }

    pub async fn list_versions(&self, id: &MemoryId) -> Result<Vec<u64>, MemoryError> {
        let versions = self.versions.read().await;
        Ok(versions
            .get(id.as_str())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect())
    }
}
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 5: Write versioning system**

### Step 6: Write CRUD operations

Create `crates/memory/src/operations.rs`:

```rust
use super::schema::{MemoryId, MemoryType, StructuredMemory, UnstructuredMemory};
use super::storage::{FilesystemStorage, StorageBackend};
use super::versioning::VersionManager;
use super::error::MemoryError;

pub struct MemoryOperations {
    storage: FilesystemStorage,
    version_manager: VersionManager,
}

impl MemoryOperations {
    pub fn new(base_path: impl AsRef<std::path::Path>) -> Self {
        Self {
            storage: FilesystemStorage::new(base_path.as_ref()),
            version_manager: VersionManager::new(base_path.as_ref()),
        }
    }

    pub async fn initialize(&self) -> Result<(), MemoryError> {
        self.storage.initialize().await
    }

    /// Create structured memory
    pub async fn create_structured(
        &self,
        data: serde_json::Value,
        tags: Vec<String>,
        metadata: std::collections::HashMap<String, String>,
    ) -> Result<MemoryId, MemoryError> {
        let id = MemoryId::new();
        let now = chrono::Utc::now();

        let memory = StructuredMemory {
            id: id.clone(),
            created_at: now,
            updated_at: now,
            version: 1,
            schema_version: "2.0".to_string(),
            data,
            tags,
            metadata,
        };

        self.storage.store_structured(&memory).await?;
        Ok(id)
    }

    /// Update structured memory with versioning
    pub async fn update_structured(
        &self,
        id: &MemoryId,
        data: serde_json::Value,
        expected_version: u64,
    ) -> Result<u64, MemoryError> {
        let mut memory = self.storage.retrieve_structured(id, Some(expected_version)).await?;

        // Create version backup
        self.version_manager.create_version(id, &memory).await?;

        // Update memory
        memory.data = data;
        memory.version += 1;
        memory.updated_at = chrono::Utc::now();

        let reference = self.storage.store_structured(&memory).await?;
        Ok(reference.version)
    }

    /// Get structured memory
    pub async fn get_structured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<StructuredMemory, MemoryError> {
        self.storage.retrieve_structured(id, version).await
    }

    /// Create unstructured memory
    pub async fn create_unstructured(
        &self,
        content: String,
        content_type: String,
        tags: Vec<String>,
        metadata: std::collections::HashMap<String, String>,
    ) -> Result<MemoryId, MemoryError> {
        let id = MemoryId::new();
        let now = chrono::Utc::now();

        let memory = UnstructuredMemory {
            id: id.clone(),
            created_at: now,
            updated_at: now,
            version: 1,
            content,
            content_type,
            size_bytes: 0, // Will be calculated
            tags,
            metadata,
        };

        let mut memory_with_size = memory;
        memory_with_size.size_bytes = memory_with_size.content.len() as u64;

        self.storage.store_unstructured(&memory_with_size).await?;
        Ok(id)
    }

    /// Get unstructured memory
    pub async fn get_unstructured(
        &self,
        id: &MemoryId,
        version: Option<u64>,
    ) -> Result<UnstructuredMemory, MemoryError> {
        self.storage.retrieve_unstructured(id, version).await
    }

    /// Delete memory
    pub async fn delete(&self, id: &MemoryId) -> Result<(), MemoryError> {
        self.storage.delete(id).await
    }

    /// List memories by type
    pub async fn list_by_type(
        &self,
        memory_type: MemoryType,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError> {
        self.storage.list_by_type(memory_type, limit, offset).await
    }

    /// List memories by tags
    pub async fn list_by_tags(
        &self,
        tags: &[String],
        limit: usize,
        offset: usize,
    ) -> Result<Vec<MemoryId>, MemoryError> {
        self.storage.list_by_tags(tags, limit, offset).await
    }
}
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 6: Write CRUD operations**

### Step 7: Write lib.rs exports

Create `crates/memory/src/lib.rs`:

```rust
pub mod error;
pub mod schema;
pub mod storage;
pub mod versioning;
pub mod operations;

pub use error::MemoryError;
pub use schema::{MemoryId, MemoryType, StructuredMemory, UnstructuredMemory, MemoryReference};
pub use storage::{StorageBackend, FilesystemStorage};
pub use versioning::VersionManager;
pub use operations::MemoryOperations;
```

Run: `cargo check --package agentsdk-memory`
Expected: SUCCESS

- [ ] **Step 7: Write lib.rs exports**

### Step 8: Add to workspace

Modify `Cargo.toml` in workspace root:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/memory",
]
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 8: Add to workspace**

### Step 9: Write unit tests

Add tests to `crates/memory/src/storage.rs` (bottom of file):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_store_and_retrieve_structured() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(temp_dir.path());
        storage.initialize().await.unwrap();

        let id = MemoryId::new();
        let memory = StructuredMemory {
            id: id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 1,
            schema_version: "2.0".to_string(),
            data: serde_json::json!({"key": "value"}),
            tags: vec!["test".to_string()],
            metadata: std::collections::HashMap::new(),
        };

        let reference = storage.store_structured(&memory).await.unwrap();
        assert_eq!(reference.memory_id, id);

        let retrieved = storage.retrieve_structured(&id, None).await.unwrap();
        assert_eq!(retrieved.data, memory.data);
    }

    #[tokio::test]
    async fn test_version_conflict() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(temp_dir.path());
        storage.initialize().await.unwrap();

        let id = MemoryId::new();
        let memory = StructuredMemory {
            id: id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 1,
            schema_version: "2.0".to_string(),
            data: serde_json::json!({"key": "value"}),
            tags: vec![],
            metadata: std::collections::HashMap::new(),
        };

        storage.store_structured(&memory).await.unwrap();

        let result = storage.retrieve_structured(&id, Some(99)).await;
        assert!(matches!(result, Err(MemoryError::VersionConflict { .. })));
    }
}
```

Run: `cargo test --package agentsdk-memory`
Expected: All tests PASS

- [ ] **Step 9: Write unit tests**

### Step 10: Write integration tests

Create `crates/memory/tests/integration_test.rs`:

```rust
use agentsdk_memory::{MemoryOperations, MemoryId, MemoryType};
use tempfile::TempDir;

#[tokio::test]
async fn test_full_crud_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();

    // Create structured memory
    let id = ops
        .create_structured(
            serde_json::json!({"name": "test", "value": 42}),
            vec!["tag1".to_string(), "tag2".to_string()],
            std::collections::HashMap::new(),
        )
        .await
        .unwrap();

    // Retrieve it
    let memory = ops.get_structured(&id, None).await.unwrap();
    assert_eq!(memory.data["name"], "test");
    assert_eq!(memory.data["value"], 42);

    // Update it
    let new_version = ops
        .update_structured(
            &id,
            serde_json::json!({"name": "updated", "value": 99}),
            1, // expected version
        )
        .await
        .unwrap();

    assert_eq!(new_version, 2);

    // Retrieve updated version
    let updated = ops.get_structured(&id, None).await.unwrap();
    assert_eq!(updated.data["name"], "updated");
    assert_eq!(updated.data["value"], 99);

    // Delete it
    ops.delete(&id).await.unwrap();

    // Verify deleted
    let result = ops.get_structured(&id, None).await;
    assert!(matches!(result, Err(agentsdk_memory::MemoryError::NotFound(_))));
}

#[tokio::test]
async fn test_unstructured_memory() {
    let temp_dir = TempDir::new().unwrap();
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();

    let content = "This is a test content".to_string();
    let id = ops
        .create_unstructured(
            content.clone(),
            "text/plain".to_string(),
            vec![],
            std::collections::HashMap::new(),
        )
        .await
        .unwrap();

    let retrieved = ops.get_unstructured(&id, None).await.unwrap();
    assert_eq!(retrieved.content, content);
}

#[tokio::test]
async fn test_list_by_tags() {
    let temp_dir = TempDir::new().unwrap();
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();

    // Create memories with different tags
    ops.create_structured(
        serde_json::json!({"name": "test1"}),
        vec!["common".to_string(), "unique1".to_string()],
        std::collections::HashMap::new(),
    )
    .await
    .unwrap();

    ops.create_structured(
        serde_json::json!({"name": "test2"}),
        vec!["common".to_string(), "unique2".to_string()],
        std::collections::HashMap::new(),
    )
    .await
    .unwrap();

    ops.create_structured(
        serde_json::json!({"name": "test3"}),
        vec!["other".to_string()],
        std::collections::HashMap::new(),
    )
    .await
    .unwrap();

    // List by common tag
    let ids = ops.list_by_tags(&["common".to_string()], 10, 0).await.unwrap();
    assert_eq!(ids.len(), 2);
}
```

Add `tempfile` to `crates/memory/Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3.8"
```

Run: `cargo test --package agentsdk-memory --test integration_test`
Expected: All tests PASS

- [ ] **Step 10: Write integration tests**

### Step 11: Commit

```bash
git add crates/memory/ Cargo.toml
git commit -m "feat(Phase5-Task00): implement local memory storage with structured/unstructured support, versioning, and CRUD operations"
```

- [ ] **Step 11: Commit**

---

## Validation Criteria

See [validation/00-local-memory-storage.md](../validation/00-local-memory-storage.md)

## Test Specifications

See [tests/00-local-memory-storage.md](../tests/00-local-memory-storage.md)
