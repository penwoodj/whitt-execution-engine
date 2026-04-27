# Task 06: Provenance Tracking

**Estimated Time:** 1-2 weeks
**Dependencies:** Task 05 complete
**Priority:** CRITICAL (ADR-0006 requires provenance timestamps for all operations)

## Overview

Implement provenance tracking with immutable timestamps, trace IDs, content hashes, query interface, and timeline visualization for all memory and web/search operations.

## Files

### Create
- `crates/provenance/Cargo.toml` - Provenance crate manifest
- `crates/provenance/src/lib.rs` - Public API exports
- `crates/provenance/src/models.rs` - Provenance data structures
- `crates/provenance/src/store.rs` - Provenance storage
- `crates/provenance/src/query.rs` - Provenance query interface
- `crates/provenance/src/timeline.rs` - Timeline visualization

### Modify
- `Cargo.toml` - Add provenance workspace member
- `crates/memory/src/operations.rs` - Integrate provenance recording
- `crates/search/src/engine.rs` - Integrate provenance recording
- `crates/scraping/src/extractor.rs` - Integrate provenance recording

### Test
- `crates/provenance/tests/integration_test.rs` - Integration tests

---

## Step-by-Step Implementation

### Step 1: Create provenance crate structure

```bash
mkdir -p crates/provenance/src

cat > crates/provenance/Cargo.toml << 'EOF'
[package]
name = "agentsdk-provenance"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
sha2 = "0.10"
hex = "0.4"
agentsdk-memory = { path = "../memory" }
EOF
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 1: Create provenance crate structure**

### Step 2: Write error types

Create `crates/provenance/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProvenanceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Trace not found: {0}")]
    TraceNotFound(String),

    #[error("Invalid trace ID: {0}")]
    InvalidTraceId(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 2: Write error types**

### Step 3: Write provenance models

Create `crates/provenance/src/models.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Provenance trace ID wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId(pub Uuid);

impl TraceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

/// Operation types tracked in provenance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    MemoryCreate,
    MemoryRead,
    MemoryUpdate,
    MemoryDelete,
    SearchQuery,
    SearchResult,
    ExternalSearch,
    WebScrape,
    IndexUpdate,
    EmbeddingGenerate,
}

/// Source of operation (who initiated it)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationSource {
    System,
    User(String),
    Tool(String),
    Workflow(String),
}

/// Single provenance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub trace_id: TraceId,
    pub parent_trace_id: Option<TraceId>,
    pub operation: OperationType,
    pub source: OperationSource,
    pub timestamp: DateTime<Utc>,
    pub memory_id: Option<agentsdk_memory::MemoryId>,
    pub content_hash: Option<String>,
    pub metadata: serde_json::Value,
}

/// Content hash computation
pub fn compute_content_hash(content: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_id_creation() {
        let id = TraceId::new();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_content_hash() {
        let content = b"test content";
        let hash1 = compute_content_hash(content);
        let hash2 = compute_content_hash(content);
        assert_eq!(hash1, hash2);

        let hash3 = compute_content_hash(b"different");
        assert_ne!(hash1, hash3);
    }
}
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 3: Write provenance models**

### Step 4: Write provenance store

Create `crates/provenance/src/store.rs`:

```rust
use super::error::ProvenanceError;
use super::models::{ProvenanceRecord, TraceId, OperationType};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ProvenanceStore {
    base_path: PathBuf,
    records: Arc<RwLock<Vec<ProvenanceRecord>>>,
}

impl ProvenanceStore {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let base_path = base_path.as_ref().join("provenance");
        std::fs::create_dir_all(&base_path).ok();

        Self {
            base_path,
            records: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record(&self, record: ProvenanceRecord) -> Result<(), ProvenanceError> {
        // Add to in-memory store
        {
            let mut records = self.records.write().await;
            records.push(record.clone());
        }

        // Persist to disk
        self.persist_record(&record).await?;

        Ok(())
    }

    async fn persist_record(&self, record: &ProvenanceRecord) -> Result<(), ProvenanceError> {
        let trace_id = record.trace_id.0.to_string();
        let file_path = self.base_path.join(format!("{}.json", trace_id));

        let json = serde_json::to_string_pretty(record)?;
        tokio::fs::write(&file_path, json).await?;

        Ok(())
    }

    pub async fn get_trace(&self, trace_id: &TraceId) -> Result<ProvenanceRecord, ProvenanceError> {
        let records = self.records.read().await;
        records
            .iter()
            .find(|r| r.trace_id == *trace_id)
            .cloned()
            .ok_or_else(|| ProvenanceError::TraceNotFound(trace_id.0.to_string()))
    }

    pub async fn get_trace_chain(
        &self,
        trace_id: &TraceId,
    ) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let mut chain = Vec::new();
        let mut current_trace_id = Some(trace_id.clone());

        while let Some(trace_id) = current_trace_id.take() {
            let record = self.get_trace(&trace_id).await?;
            current_trace_id = record.parent_trace_id.clone();
            chain.push(record);
        }

        // Reverse to get chronological order (oldest first)
        chain.reverse();
        Ok(chain)
    }

    pub async fn query_by_operation(
        &self,
        operation: OperationType,
        limit: usize,
    ) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let records = self.records.read().await;
        let results: Vec<_> = records
            .iter()
            .filter(|r| r.operation == operation)
            .take(limit)
            .cloned()
            .collect();

        Ok(results)
    }

    pub async fn query_by_memory(
        &self,
        memory_id: &agentsdk_memory::MemoryId,
    ) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let records = self.records.read().await;
        let results: Vec<_> = records
            .iter()
            .filter(|r| r.memory_id.as_ref() == Some(memory_id))
            .cloned()
            .collect();

        Ok(results)
    }

    pub async fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let records = self.records.read().await;
        let results: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect();

        Ok(results)
    }

    pub async fn get_all(&self) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let records = self.records.read().await;
        Ok(records.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::models::{OperationSource};

    #[tokio::test]
    async fn test_record_and_retrieve() {
        let store = ProvenanceStore::new("/tmp/test_provenance");

        let record = ProvenanceRecord {
            trace_id: TraceId::new(),
            parent_trace_id: None,
            operation: OperationType::MemoryCreate,
            source: OperationSource::System,
            timestamp: chrono::Utc::now(),
            memory_id: None,
            content_hash: None,
            metadata: serde_json::json!({}),
        };

        store.record(record.clone()).await.unwrap();

        let retrieved = store.get_trace(&record.trace_id).await.unwrap();
        assert_eq!(retrieved.trace_id, record.trace_id);
    }
}
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 4: Write provenance store**

### Step 5: Write query interface

Create `crates/provenance/src/query.rs`:

```rust
use super::error::ProvenanceError;
use super::models::{TraceId, OperationType, ProvenanceRecord};
use super::store::ProvenanceStore;
use chrono::{DateTime, Utc};

pub struct ProvenanceQuery {
    store: ProvenanceStore,
}

impl ProvenanceQuery {
    pub fn new(store: ProvenanceStore) -> Self {
        Self { store }
    }

    pub async fn build_timeline(
        &self,
        trace_id: &TraceId,
    ) -> Result<Vec<TimelineEvent>, ProvenanceError> {
        let chain = self.store.get_trace_chain(trace_id).await?;

        let timeline: Vec<TimelineEvent> = chain
            .into_iter()
            .map(|record| TimelineEvent {
                timestamp: record.timestamp,
                operation: record.operation,
                description: format!("{:?}", record.operation),
                metadata: record.metadata,
            })
            .collect();

        Ok(timeline)
    }

    pub async fn search_by_metadata(
        &self,
        key: &str,
        value: &str,
    ) -> Result<Vec<ProvenanceRecord>, ProvenanceError> {
        let all_records = self.store.get_all().await?;

        let results: Vec<_> = all_records
            .into_iter()
            .filter(|record| {
                if let Some(val) = record.metadata.get(key) {
                    val.as_str() == Some(value)
                } else {
                    false
                }
            })
            .collect();

        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub timestamp: DateTime<Utc>,
    pub operation: OperationType,
    pub description: String,
    pub metadata: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_event_creation() {
        let event = TimelineEvent {
            timestamp: chrono::Utc::now(),
            operation: OperationType::MemoryCreate,
            description: "Memory created".to_string(),
            metadata: serde_json::json!({}),
        };

        assert_eq!(event.description, "Memory created");
    }
}
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 5: Write query interface**

### Step 6: Write timeline visualization

Create `crates/provenance/src/timeline.rs`:

```rust
use super::query::TimelineEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub events: Vec<TimelineEvent>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl Timeline {
    pub fn new(events: Vec<TimelineEvent>) -> Self {
        let start_time = events
            .first()
            .map(|e| e.timestamp)
            .unwrap_or_else(Utc::now);

        let end_time = events
            .last()
            .map(|e| e.timestamp)
            .unwrap_or_else(Utc::now);

        Self {
            events,
            start_time,
            end_time,
        }
    }

    pub fn to_mermaid(&self) -> String {
        let mut mermaid = String::from("timeline\n");

        for (i, event) in self.events.iter().enumerate() {
            let time_str = event.timestamp.format("%H:%M:%S");
            mermaid.push_str(&format!(
                "    {} : {} - {}\n",
                time_str,
                format!("{:?}", event.operation),
                event.description
            ));
        }

        mermaid
    }

    pub fn duration(&self) -> chrono::Duration {
        self.end_time - self.start_time
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let now = chrono::Utc::now();
        let events = vec![
            TimelineEvent {
                timestamp: now,
                operation: crate::models::OperationType::MemoryCreate,
                description: "Created".to_string(),
                metadata: serde_json::json!({}),
            }
        ];

        let timeline = Timeline::new(events);
        assert_eq!(timeline.event_count(), 1);
    }

    #[test]
    fn test_mermaid_output() {
        let now = chrono::Utc::now();
        let events = vec![
            TimelineEvent {
                timestamp: now,
                operation: crate::models::OperationType::MemoryCreate,
                description: "Test".to_string(),
                metadata: serde_json::json!({}),
            }
        ];

        let timeline = Timeline::new(events);
        let mermaid = timeline.to_mermaid();

        assert!(mermaid.contains("timeline"));
    }
}
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 6: Write timeline visualization**

### Step 7: Write lib.rs exports

Create `crates/provenance/src/lib.rs`:

```rust
pub mod error;
pub mod models;
pub mod store;
pub mod query;
pub mod timeline;

pub use error::ProvenanceError;
pub use models::{TraceId, OperationType, OperationSource, ProvenanceRecord, compute_content_hash};
pub use store::ProvenanceStore;
pub use query::{ProvenanceQuery, TimelineEvent};
pub use timeline::Timeline;
```

Run: `cargo check --package agentsdk-provenance`
Expected: SUCCESS

- [ ] **Step 7: Write lib.rs exports**

### Step 8: Add to workspace

Modify `Cargo.toml`:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/provenance",
]
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 8: Add to workspace**

### Step 9: Write integration tests

Create `crates/provenance/tests/integration_test.rs`:

```rust
use agentsdk_provenance::{ProvenanceStore, ProvenanceQuery, Timeline};
use agentsdk_provenance::{TraceId, OperationType, OperationSource, ProvenanceRecord};
use tempfile::TempDir;

#[tokio::test]
async fn test_full_provenance_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let store = ProvenanceStore::new(temp_dir.path());

    // Create a trace
    let parent_id = TraceId::new();
    let child_id = TraceId::new();

    let parent_record = ProvenanceRecord {
        trace_id: parent_id.clone(),
        parent_trace_id: None,
        operation: OperationType::MemoryCreate,
        source: OperationSource::System,
        timestamp: chrono::Utc::now(),
        memory_id: None,
        content_hash: Some("abc123".to_string()),
        metadata: serde_json::json!({"test": "data"}),
    };

    store.record(parent_record).await.unwrap();

    let child_record = ProvenanceRecord {
        trace_id: child_id.clone(),
        parent_trace_id: Some(parent_id.clone()),
        operation: OperationType::SearchQuery,
        source: OperationSource::System,
        timestamp: chrono::Utc::now(),
        memory_id: None,
        content_hash: None,
        metadata: serde_json::json!({"query": "test"}),
    };

    store.record(child_record).await.unwrap();

    // Query timeline
    let query = ProvenanceQuery::new(store.clone());
    let timeline_events = query.build_timeline(&child_id).await.unwrap();

    assert_eq!(timeline_events.len(), 2);

    // Build timeline
    let timeline = Timeline::new(timeline_events);
    let mermaid = timeline.to_mermaid();

    assert!(mermaid.contains("timeline"));
}

#[tokio::test]
async fn test_content_hash() {
    use agentsdk_provenance::compute_content_hash;

    let content = b"test content";
    let hash1 = compute_content_hash(content);
    let hash2 = compute_content_hash(content);

    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 64); // SHA-256 hex length
}
```

Run: `cargo test --package agentsdk-provenance --test integration_test`
Expected: All tests PASS

- [ ] **Step 9: Write integration tests**

### Step 10: Commit

```bash
git add crates/provenance/ Cargo.toml
git commit -m "feat(Phase5-Task06): implement provenance tracking with immutable timestamps, trace IDs, content hashes, query interface, and timeline visualization (ADR-0006 compliant)"
```

- [ ] **Step 10: Commit**

---

## ADR-0006 Critical Requirements

**MUST ENFORCE:**

1. **Immutable Timestamps:** All operations MUST record timestamps that cannot be modified
2. **Trace IDs:** All operations MUST have trace IDs for reconstruction
3. **Content Hashes:** All memory and web operations MUST record SHA-256 hashes
4. **Trace Chains:** Parent-child relationships MUST be maintained for operation chains
5. **Queryability:** All provenance data MUST be queryable by time, operation, and metadata

**FORBIDDEN:**

- ❌ Operations without trace IDs
- ❌ Modifying timestamps after recording
- ❌ Missing content hashes for memory/web operations
- ❌ Broken trace chains
- ❌ Unqueryable provenance data

---

## Validation Criteria

See [validation/06-provenance-tracking.md](../validation/06-provenance-tracking.md)

## Test Specifications

See [tests/06-provenance-tracking.md](../tests/06-provenance-tracking.md)


---

## QA Cross-References

### QA Criteria
- **QA Area**: Area 7 - Provenance Tracking
- **QA Criteria**: [../../qa/phase-04/QA-CRITERIA.md#area-7-provenance-tracking](../../qa/phase-04/QA-CRITERIA.md#area-7-provenance-tracking)
- **Priority**: P1
- **Test Types**: Unit, Integration

### Test Cases
- **Test Cases**: [../../qa/phase-04/QA-TEST-CASES.md](../../qa/phase-04/QA-TEST-CASES.md)
- **Key Tests**:
  - P04-031: Source attribution
  - P04-032: Immutable event log
  - P04-033: Artifact linking
  - P04-034: Timestamp preservation
  - P04-035: Hash verification
  - P04-036: Provenance query

### Schema References
- **Schema File**: [../../../schema/unified-workflow-schema.yml](../../../schema/unified-workflow-schema.yml)
- **Schema Section**: Lines 110-148 (provenance tracking: source_uri, timestamp, checksum)
- **Key Fields**:
  - `provenance.source_uri` (line 112)
  -   - `provenance.timestamp` (line 113)
  -   - `provenance.checksum` (line 114)

### Related Documentation
- **Cross-References**: [../../qa/phase-04/CROSS-REF.md](../../qa/phase-04/CROSS-REF.md)
- **Phase Plan**: [../plan.md](../plan.md)
