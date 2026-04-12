# Test Specifications: Task 06 - Provenance Tracking

## Mock Strategy

**Mock Storage:**
```rust
struct MockProvenanceStore {
    records: Arc<Mutex<Vec<ProvenanceRecord>>>,
}

impl MockProvenanceStore {
    fn new() -> Self {
        Self { records: Arc::new(Mutex::new(vec![])) }
    }
}
```

## Test Cases

### Unit Tests

**Trace Recording**
```rust
#[tokio::test]
async fn test_record_trace() {
    let store = ProvenanceStore::new(temp_dir.path());
    let record = create_test_record();
    store.record(record).await.unwrap();
    
    let retrieved = store.get_trace(&trace_id).await.unwrap();
    assert_eq!(retrieved.operation, expected_operation);
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
```

### Integration Tests

**Trace Chain Reconstruction**
- Record parent operation
- Record child operation
- Query child trace ID
- Verify full chain returned
- Check chronological order

**Query by Operation Type**
- Record multiple operations
- Query by specific type
- Verify only matching ops returned

**Query by Time Range**
- Record operations at different times
- Query with time range
- Verify correct filtering

**Query by Metadata**
- Record with metadata
- Query by metadata key/value
- Verify correct results

**Timeline Generation**
- Create trace chain
- Generate timeline
- Verify Mermaid format
- Check duration calculation

## Mock Dependencies

- `tempfile`: Test isolation
- In-memory storage: Fast testing
- Pre-defined trace IDs: Deterministic tests
