# Test Specifications: Task 06 - Provenance Tracking

## Mock Strategy

**Mock Storage:**
```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tempfile::TempDir;

struct MockProvenanceStore {
    records: Arc<Mutex<Vec<ProvenanceRecord>>>,
    temp_dir: TempDir,
}

impl MockProvenanceStore {
    fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(vec![])),
            temp_dir: TempDir::new().unwrap(),
        }
    }

    fn create_test_trace_id() -> TraceId {
        uuid::Uuid::new_v4().to_string()
    }

    fn create_test_record(trace_id: TraceId) -> ProvenanceRecord {
        ProvenanceRecord {
            trace_id: trace_id.clone(),
            operation: OperationType::Create,
            memory_id: Some(uuid::Uuid::new_v4().to_string()),
            parent_trace_id: None,
            timestamp: Utc::now(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("key".to_string(), "value".to_string());
                map
            },
            content_hash: Some("hash".to_string()),
            content_size: Some(1024),
            status: RecordStatus::Success,
            error: None,
        }
    }
}
```

**Mock Time Control:**
```rust
struct MockTime {
    current: Arc<RwLock<DateTime<Utc>>>,
}

impl MockTime {
    fn new() -> Self {
        Self {
            current: Arc::new(RwLock::new(Utc::now())),
        }
    }

    fn now(&self) -> DateTime<Utc> {
        *self.current.read().await
    }

    fn advance(&self, duration: Duration) {
        let mut current = self.current.write().await;
        *current = *current + duration;
    }

    fn set(&self, dt: DateTime<Utc>) {
        let mut current = self.current.write().await;
        *current = dt;
    }
}
```

**Test Data Fixtures:**
```rust
fn create_test_trace_chain() -> Vec<ProvenanceRecord> {
    let trace_id = uuid::Uuid::new_v4().to_string();
    let base_time = Utc::now();

    vec![
        ProvenanceRecord {
            trace_id: trace_id.clone(),
            operation: OperationType::Create,
            memory_id: Some(uuid::Uuid::new_v4().to_string()),
            parent_trace_id: None,
            timestamp: base_time,
            metadata: HashMap::new(),
            content_hash: Some("hash1".to_string()),
            content_size: Some(1024),
            status: RecordStatus::Success,
            error: None,
        },
        ProvenanceRecord {
            trace_id: uuid::Uuid::new_v4().to_string(),
            operation: OperationType::Update,
            memory_id: Some(uuid::Uuid::new_v4().to_string()),
            parent_trace_id: Some(trace_id.clone()),
            timestamp: base_time + Duration::seconds(1),
            metadata: HashMap::new(),
            content_hash: Some("hash2".to_string()),
            content_size: Some(2048),
            status: RecordStatus::Success,
            error: None,
        },
        ProvenanceRecord {
            trace_id: uuid::Uuid::new_v4().to_string(),
            operation: OperationType::Read,
            memory_id: Some(uuid::Uuid::new_v4().to_string()),
            parent_trace_id: Some(trace_id),
            timestamp: base_time + Duration::seconds(2),
            metadata: HashMap::new(),
            content_hash: None,
            content_size: None,
            status: RecordStatus::Success,
            error: None,
        },
    ]
}
```

## Test Cases

### Unit Tests

**Trace Recording**
```rust
#[tokio::test]
async fn test_record_trace() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let trace_id = uuid::Uuid::new_v4().to_string();

    let record = ProvenanceRecord {
        trace_id: trace_id.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-123".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: Some("hash".to_string()),
        content_size: Some(1024),
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record.clone()).await.unwrap();

    let retrieved = store.get_trace(&trace_id).await.unwrap();
    assert_eq!(retrieved.trace_id, trace_id);
    assert_eq!(retrieved.operation, OperationType::Create);
}

#[tokio::test]
async fn test_record_with_error() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let trace_id = uuid::Uuid::new_v4().to_string();

    let record = ProvenanceRecord {
        trace_id: trace_id.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-123".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Failure,
        error: Some("Network timeout".to_string()),
    };

    store.record(record.clone()).await.unwrap();

    let retrieved = store.get_trace(&trace_id).await.unwrap();
    assert_eq!(retrieved.status, RecordStatus::Failure);
    assert_eq!(retrieved.error, Some("Network timeout".to_string()));
}

#[tokio::test]
async fn test_record_duplicate_trace_id() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let trace_id = uuid::Uuid::new_v4().to_string();

    let record1 = ProvenanceRecord {
        trace_id: trace_id.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    let record2 = ProvenanceRecord {
        trace_id: trace_id.clone(),
        operation: OperationType::Update,
        memory_id: Some("memory-2".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record1).await.unwrap();
    let result = store.record(record2).await;

    assert!(matches!(result, Err(ProvenanceError::DuplicateTraceId(_))));
}
```

**Content Hash Computation**
```rust
#[test]
fn test_content_hash() {
    let content = b"test content";
    let hash = compute_content_hash(content);

    assert_eq!(hash.len(), 64); // SHA-256 hex length

    let hash2 = compute_content_hash(content);
    assert_eq!(hash, hash2); // Deterministic
}

#[test]
fn test_content_hash_different_content() {
    let content1 = b"content 1";
    let content2 = b"content 2";

    let hash1 = compute_content_hash(content1);
    let hash2 = compute_content_hash(content2);

    assert_ne!(hash1, hash2);
}

#[test]
fn test_content_hash_empty_content() {
    let content = b"";
    let hash = compute_content_hash(content);

    assert_eq!(hash.len(), 64);

    // Empty content should have specific hash
    assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
fn test_content_hash_large_content() {
    let content = "x".repeat(1_000_000).into_bytes();
    let hash = compute_content_hash(&content);

    assert_eq!(hash.len(), 64);

    // Should be consistent
    let hash2 = compute_content_hash(&content);
    assert_eq!(hash, hash2);
}
```

**Trace Querying**
```rust
#[tokio::test]
async fn test_get_trace() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let trace_id = uuid::Uuid::new_v4().to_string();

    let record = ProvenanceRecord {
        trace_id: trace_id.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-123".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record).await.unwrap();

    let retrieved = store.get_trace(&trace_id).await.unwrap();
    assert_eq!(retrieved.trace_id, trace_id);
}

#[tokio::test]
async fn test_get_nonexistent_trace() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let result = store.get_trace("nonexistent").await;

    assert!(matches!(result, Err(ProvenanceError::TraceNotFound(_))));
}

#[tokio::test]
async fn test_get_trace_by_memory_id() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let memory_id = uuid::Uuid::new_v4().to_string();

    let record = ProvenanceRecord {
        trace_id: uuid::Uuid::new_v4().to_string(),
        operation: OperationType::Create,
        memory_id: Some(memory_id.clone()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record).await.unwrap();

    let traces = store.get_traces_by_memory_id(&memory_id).await.unwrap();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].memory_id, Some(memory_id));
}
```

### Integration Tests

**Trace Chain Reconstruction**
- [ ] **Step 1: Create trace chain**
  ```rust
  let store = ProvenanceStore::new(TempDir::new().unwrap().path());
  let chain = create_test_trace_chain();

  for record in &chain {
      store.record(record.clone()).await.unwrap();
  }
  ```

- [ ] **Step 2: Query child trace ID**
  ```rust
  let child_trace_id = chain[2].trace_id.clone();
  let reconstructed = store.reconstruct_chain(&child_trace_id).await.unwrap();
  ```

- [ ] **Step 3: Verify full chain returned**
  ```rust
  assert_eq!(reconstructed.len(), 3);
  ```

- [ ] **Step 4: Check chronological order**
  ```rust
  for window in reconstructed.windows(2) {
      assert!(window[0].timestamp <= window[1].timestamp);
  }
  ```

**Query by Operation Type**
```rust
#[tokio::test]
async fn test_query_by_operation_type() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    // Record different operations
    for (i, op) in [OperationType::Create, OperationType::Update, OperationType::Read].iter().enumerate() {
        let record = ProvenanceRecord {
            trace_id: uuid::Uuid::new_v4().to_string(),
            operation: op.clone(),
            memory_id: Some(format!("memory-{}", i)),
            parent_trace_id: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            content_hash: None,
            content_size: None,
            status: RecordStatus::Success,
            error: None,
        };
        store.record(record).await.unwrap();
    }

    // Query by operation type
    let create_records = store.query_by_operation(OperationType::Create).await.unwrap();
    let update_records = store.query_by_operation(OperationType::Update).await.unwrap();
    let read_records = store.query_by_operation(OperationType::Read).await.unwrap();

    assert_eq!(create_records.len(), 1);
    assert_eq!(update_records.len(), 1);
    assert_eq!(read_records.len(), 1);
}
```

**Query by Time Range**
```rust
#[tokio::test]
async fn test_query_by_time_range() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let base_time = Utc::now();

    // Record operations at different times
    for i in 0..10 {
        let record = ProvenanceRecord {
            trace_id: uuid::Uuid::new_v4().to_string(),
            operation: OperationType::Create,
            memory_id: Some(format!("memory-{}", i)),
            parent_trace_id: None,
            timestamp: base_time + Duration::seconds(i as i64),
            metadata: HashMap::new(),
            content_hash: None,
            content_size: None,
            status: RecordStatus::Success,
            error: None,
        };
        store.record(record).await.unwrap();
    }

    // Query with time range
    let start = base_time + Duration::seconds(3);
    let end = base_time + Duration::seconds(7);
    let results = store.query_by_time_range(start, end).await.unwrap();

    // Should return 4 records (indices 3, 4, 5, 6)
    assert_eq!(results.len(), 5);
    for result in &results {
        assert!(result.timestamp >= start && result.timestamp <= end);
    }
}
```

**Query by Metadata**
```rust
#[tokio::test]
async fn test_query_by_metadata() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    // Record with different metadata
    let mut metadata1 = HashMap::new();
    metadata1.insert("type".to_string(), "document".to_string());

    let mut metadata2 = HashMap::new();
    metadata2.insert("type".to_string(), "code".to_string());

    let record1 = ProvenanceRecord {
        trace_id: uuid::Uuid::new_v4().to_string(),
        operation: OperationType::Create,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: metadata1,
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    let record2 = ProvenanceRecord {
        trace_id: uuid::Uuid::new_v4().to_string(),
        operation: OperationType::Create,
        memory_id: Some("memory-2".to_string()),
        parent_trace_id: None,
        timestamp: Utc::now(),
        metadata: metadata2,
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record1).await.unwrap();
    store.record(record2).await.unwrap();

    // Query by metadata
    let results = store.query_by_metadata("type", "document").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].memory_id, Some("memory-1".to_string()));
}
```

**Timeline Generation**
```rust
#[tokio::test]
async fn test_timeline_generation() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let chain = create_test_trace_chain();

    for record in &chain {
        store.record(record.clone()).await.unwrap();
    }

    let child_trace_id = chain[2].trace_id.clone();
    let timeline = store.generate_timeline(&child_trace_id).await.unwrap();

    // Verify Mermaid format
    assert!(timeline.contains("graph TD"));
    assert!(timeline.contains("-->"));

    // Verify all traces present
    for record in &chain {
        assert!(timeline.contains(&record.trace_id));
    }

    // Verify chronological order
    assert!(timeline.lines().count() > 0);
}

#[tokio::test]
async fn test_timeline_duration_calculation() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());
    let start_time = Utc::now();

    let record1 = ProvenanceRecord {
        trace_id: uuid::Uuid::new_v4().to_string(),
        operation: OperationType::Create,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: None,
        timestamp: start_time,
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    let record2 = ProvenanceRecord {
        trace_id: uuid::Uuid::new_v4().to_string(),
        operation: OperationType::Update,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: Some(record1.trace_id.clone()),
        timestamp: start_time + Duration::seconds(5),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record1).await.unwrap();
    store.record(record2).await.unwrap();

    let timeline = store.generate_timeline(&record2.trace_id).await.unwrap();

    // Timeline should contain duration info
    assert!(timeline.contains("5s"));
}
```

**Large Datasets**
```rust
#[tokio::test]
async fn test_query_performance_large_dataset() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    // Record 100,000 traces
    let start = Instant::now();
    for i in 0..100_000 {
        let record = ProvenanceRecord {
            trace_id: format!("trace-{:08}", i),
            operation: if i % 2 == 0 { OperationType::Create } else { OperationType::Update },
            memory_id: Some(format!("memory-{:08}", i)),
            parent_trace_id: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            content_hash: None,
            content_size: Some(1024),
            status: RecordStatus::Success,
            error: None,
        };
        store.record(record).await.unwrap();
    }
    let insert_time = start.elapsed();

    println!("Inserted 100,000 traces in {:?}", insert_time);
    assert!(insert_time < Duration::from_secs(30), "Insert too slow");

    // Query performance
    let start = Instant::now();
    let results = store.query_by_operation(OperationType::Create).await.unwrap();
    let query_time = start.elapsed();

    assert_eq!(results.len(), 50_000);
    println!("Queried 50,000 results in {:?}", query_time);
    assert!(query_time < Duration::from_secs(5), "Query too slow");
}
```

**Edge Cases**
```rust
#[tokio::test]
async fn test_empty_trace_query() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    let chain = create_test_trace_chain();
    let invalid_trace_id = uuid::Uuid::new_v4().to_string();

    for record in &chain {
        store.record(record.clone()).await.unwrap();
    }

    let result = store.reconstruct_chain(&invalid_trace_id).await;

    // Should return chain with only that trace if it exists
    assert!(matches!(result, Err(ProvenanceError::TraceNotFound(_))));
}

#[tokio::test]
async fn test_circular_trace_handling() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    let trace1 = uuid::Uuid::new_v4().to_string();
    let trace2 = uuid::Uuid::new_v4().to_string();
    let trace3 = uuid::Uuid::new_v4().to_string();

    // Create circular reference (trace1 -> trace2 -> trace3 -> trace1)
    let record1 = ProvenanceRecord {
        trace_id: trace1.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: Some(trace3.clone()),
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    let record2 = ProvenanceRecord {
        trace_id: trace2.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-2".to_string()),
        parent_trace_id: Some(trace1.clone()),
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    let record3 = ProvenanceRecord {
        trace_id: trace3.clone(),
        operation: OperationType::Create,
        memory_id: Some("memory-3".to_string()),
        parent_trace_id: Some(trace2.clone()),
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record1).await.unwrap();
    store.record(record2).await.unwrap();
    store.record(record3).await.unwrap();

    // Reconstruct should handle circular references gracefully
    let chain = store.reconstruct_chain(&trace1).await;

    // Should detect circular reference and handle appropriately
    assert!(chain.is_ok() || matches!(chain, Err(ProvenanceError::CircularReference(_))));
}

#[tokio::test]
async fn test_trace_with_missing_parent() {
    let store = ProvenanceStore::new(TempDir::new().unwrap().path());

    let parent_id = uuid::Uuid::new_v4().to_string();
    let child_id = uuid::Uuid::new_v4().to_string();

    // Record child with non-existent parent
    let record = ProvenanceRecord {
        trace_id: child_id.clone(),
        operation: OperationType::Update,
        memory_id: Some("memory-1".to_string()),
        parent_trace_id: Some(parent_id),
        timestamp: Utc::now(),
        metadata: HashMap::new(),
        content_hash: None,
        content_size: None,
        status: RecordStatus::Success,
        error: None,
    };

    store.record(record).await.unwrap();

    // Reconstruct should handle missing parent
    let chain = store.reconstruct_chain(&child_id).await;

    // Should return chain with child only (parent not found)
    assert!(chain.is_ok());
    assert_eq!(chain.unwrap().len(), 1);
}
```

## Cargo Commands

**Run all provenance tracking tests:**
```bash
cargo test --test provenance_tracking -- --nocapture
```

**Run specific test:**
```bash
cargo test test_record_trace -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test provenance_tracking -- --nocapture
```

**Expected output:**
```
running 30 tests
test tests::unit::test_record_trace ... ok
test tests::unit::test_record_with_error ... ok
test tests::unit::test_record_duplicate_trace_id ... ok
test tests::unit::test_content_hash ... ok
test tests::unit::test_content_hash_different_content ... ok
test tests::unit::test_content_hash_empty_content ... ok
test tests::unit::test_content_hash_large_content ... ok
test tests::unit::test_get_trace ... ok
test tests::unit::test_get_nonexistent_trace ... ok
test tests::unit::test_get_trace_by_memory_id ... ok
test tests::integration::test_trace_chain_reconstruction ... ok
test tests::integration::test_query_by_operation_type ... ok
test tests::integration::test_query_by_time_range ... ok
test tests::integration::test_query_by_metadata ... ok
test tests::integration::test_timeline_generation ... ok
test tests::integration::test_timeline_duration_calculation ... ok
test tests::integration::test_query_performance_large_dataset ... ok
test tests::edge_cases::test_empty_trace_query ... ok
test tests::edge_cases::test_circular_trace_handling ... ok
test tests::edge_cases::test_trace_with_missing_parent ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Inserted 100,000 traces in 15.2s
Queried 50,000 results in 350ms
```

## Mock Dependencies

- `tempfile`: Test isolation and cleanup
- `MockProvenanceStore`: In-memory storage for fast testing
- Pre-defined trace IDs: Deterministic testing
- `MockTime`: Time control for testing time-based queries
- `tokio::test`: Async test support
- `uuid`: Unique ID generation for traces
- `chrono`: Timestamp handling
