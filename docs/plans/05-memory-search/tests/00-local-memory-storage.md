# Test Specifications: Task 00 - Local Memory Storage

## Mock Strategy

Use `tempfile` crate for test isolation:
- Create temporary directory for each test with `TempDir::new().unwrap()`
- Ensure cleanup after test completion via `Drop` trait
- Use `sled` in-memory configuration for fast testing
- Create test fixtures with deterministic IDs

**Mock Sled Store:**
```rust
struct TestMemoryStore {
    db: sled::Db,
    temp_dir: TempDir,
}

impl TestMemoryStore {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let config = sled::Config::default()
            .path(temp_dir.path().join("test_db"))
            .temporary(true);
        let db = config.open().unwrap();
        Self { db, temp_dir }
    }
}
```

**Test Data Fixtures:**
```rust
fn create_test_artifact(id: &str) -> Artifact {
    Artifact {
        id: id.to_string(),
        content: serde_json::json!({"key": "value"}),
        tags: vec!["test".to_string()],
        metadata: HashMap::new(),
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
```

## Test Cases

### Unit Tests

**Memory Creation and Storage**
```rust
#[tokio::test]
async fn test_create_structured_memory() {
    let temp_dir = TempDir::new().unwrap();
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();

    let id = ops.create_structured(
        serde_json::json!({"key": "value"}),
        vec!["test".to_string()],
        std::collections::HashMap::new(),
    ).await.unwrap();

    assert!(!id.as_str().is_empty());
    assert!(uuid::Uuid::parse_str(id.as_str()).is_ok());
}

#[tokio::test]
async fn test_store_artifact() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");

    let result = store.store(artifact.clone()).await;
    assert!(result.is_ok());

    let retrieved = store.retrieve(&artifact.id).await.unwrap();
    assert_eq!(retrieved.id, artifact.id);
    assert_eq!(retrieved.version, artifact.version);
}

#[tokio::test]
async fn test_retrieve_nonexistent() {
    let store = TestMemoryStore::new();
    let result = store.retrieve("nonexistent-id").await;

    assert!(matches!(result, Err(MemoryError::NotFound(_))));
}
```

**Versioning**
```rust
#[tokio::test]
async fn test_version_conflict() {
    let store = TestMemoryStore::new();
    let mut artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    // Simulate concurrent update with wrong version
    artifact.version = 2;
    artifact.content = serde_json::json!({"new_key": "new_value"});
    let result = store.update(&artifact.id, artifact.content, 1).await;

    assert!(matches!(result, Err(MemoryError::VersionConflict(_))));
}

#[tokio::test]
async fn test_versioning_increments_on_update() {
    let store = TestMemoryStore::new();
    let mut artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    let update_content = serde_json::json!({"updated": true});
    store.update(&artifact.id, update_content, artifact.version).await.unwrap();

    let retrieved = store.retrieve(&artifact.id).await.unwrap();
    assert_eq!(retrieved.version, 2);
}

#[tokio::test]
async fn test_version_history_tracking() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    // Update multiple times
    for i in 2..=5 {
        store.update(&artifact.id, serde_json::json!({"version": i}), i - 1).await.unwrap();
    }

    let history = store.get_version_history(&artifact.id).await.unwrap();
    assert_eq!(history.len(), 5);
    assert_eq!(history[0].version, 1);
    assert_eq!(history[4].version, 5);
}
```

**Tagging**
```rust
#[tokio::test]
async fn test_add_tags_to_memory() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    let new_tags = vec!["tag1".to_string(), "tag2".to_string()];
    store.add_tags(&artifact.id, new_tags.clone()).await.unwrap();

    let retrieved = store.retrieve(&artifact.id).await.unwrap();
    assert!(retrieved.tags.contains(&"tag1".to_string()));
    assert!(retrieved.tags.contains(&"tag2".to_string()));
}

#[tokio::test]
async fn test_list_by_tag() {
    let store = TestMemoryStore::new();

    // Create artifacts with different tags
    let artifact1 = create_test_artifact("id1");
    let mut artifact2 = create_test_artifact("id2");
    let mut artifact3 = create_test_artifact("id3");
    artifact2.tags = vec!["common".to_string()];
    artifact3.tags = vec!["common".to_string(), "unique".to_string()];

    store.store(artifact1).await.unwrap();
    store.store(artifact2).await.unwrap();
    store.store(artifact3).await.unwrap();

    let results = store.list_by_tag(&["common".to_string()]).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_list_by_multiple_tags() {
    let store = TestMemoryStore::new();

    let mut artifact1 = create_test_artifact("id1");
    let mut artifact2 = create_test_artifact("id2");
    let mut artifact3 = create_test_artifact("id3");
    artifact1.tags = vec!["tag1".to_string()];
    artifact2.tags = vec!["tag1".to_string(), "tag2".to_string()];
    artifact3.tags = vec!["tag2".to_string()];

    store.store(artifact1).await.unwrap();
    store.store(artifact2).await.unwrap();
    store.store(artifact3).await.unwrap();

    // AND logic: must have all tags
    let results = store.list_by_tags(&["tag1", "tag2"]).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "id2");
}
```

**Persistence**
```rust
#[tokio::test]
async fn test_persistence_across_restart() {
    let temp_dir = TempDir::new().unwrap();

    {
        // Create and store memory
        let ops = MemoryOperations::new(temp_dir.path());
        ops.initialize().await.unwrap();
        let id = ops.create_structured(
            serde_json::json!({"persistent": "data"}),
            vec![],
            HashMap::new(),
        ).await.unwrap();
    } // ops dropped here

    // Recreate and verify persistence
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();
    let retrieved = ops.retrieve(&id).await.unwrap();
    assert_eq!(retrieved.content["persistent"], "data");
}

#[tokio::test]
async fn test_flush_persists_data() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    store.flush().await.unwrap();

    // Verify data in sled tree
    let tree = store.db.open_tree("artifacts").unwrap();
    let bytes = tree.get(artifact.id.as_bytes()).unwrap().unwrap();
    let stored: Artifact = bincode::deserialize(&bytes).unwrap();
    assert_eq!(stored.id, artifact.id);
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn proptest_round_trip_artifact(
            id in "[a-z0-9-]{36}",
            key in "[a-zA-Z0-9]{1,20}",
            value in "[a-zA-Z0-9]{1,50}"
        ) {
            let store = TestMemoryStore::new();
            let mut content = serde_json::json!({});
            content[key] = serde_json::Value::String(value.clone());

            let artifact = Artifact {
                id: id.clone(),
                content,
                tags: vec![],
                metadata: HashMap::new(),
                version: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            store.store(artifact.clone()).await.unwrap();
            let retrieved = store.retrieve(&id).await.unwrap();

            prop_assert_eq!(retrieved.id, artifact.id);
            prop_assert_eq!(retrieved.content[key], value);
        }

        #[test]
        fn proptest_versioning_invariant(
            initial_version in 1u32..=10u32,
            num_updates in 0u32..=5u32
        ) {
            let store = TestMemoryStore::new();
            let mut artifact = create_test_artifact("test-id");
            artifact.version = initial_version;

            store.store(artifact.clone()).await.unwrap();

            for i in 1..=num_updates {
                let next_version = initial_version + i;
                store.update(
                    &artifact.id,
                    serde_json::json!({"update": i}),
                    next_version - 1
                ).await.unwrap();
            }

            let retrieved = store.retrieve(&artifact.id).await.unwrap();
            prop_assert_eq!(retrieved.version, initial_version + num_updates);
        }

        #[test]
        fn proptest_tag_invariant(
            num_tags in 0usize..=10usize,
            tag_prefix in "[a-z]{3}"
        ) {
            let store = TestMemoryStore::new();
            let mut artifact = create_test_artifact("test-id");

            let tags: Vec<String> = (0..num_tags)
                .map(|i| format!("{}-{}", tag_prefix, i))
                .collect();

            artifact.tags = tags.clone();
            store.store(artifact).await.unwrap();

            for tag in &tags {
                let results = store.list_by_tag(&[tag.clone()]).await.unwrap();
                prop_assert_eq!(results.len(), 1);
            }
        }
    }
}
```

### Integration Tests

**Full CRUD Workflow**
- [ ] **Step 1: Create structured memory**
  ```rust
  let id = ops.create_structured(
      serde_json::json!({"name": "test"}),
      vec!["tag1".to_string()],
      HashMap::new()
  ).await.unwrap();
  ```

- [ ] **Step 2: Retrieve by ID**
  ```rust
  let retrieved = ops.retrieve(&id).await.unwrap();
  assert_eq!(retrieved.content["name"], "test");
  ```

- [ ] **Step 3: Update with correct version**
  ```rust
  ops.update(&id, serde_json::json!({"name": "updated"}), 1).await.unwrap();
  ```

- [ ] **Step 4: Retrieve updated version**
  ```rust
  let updated = ops.retrieve(&id).await.unwrap();
  assert_eq!(updated.content["name"], "updated");
  assert_eq!(updated.version, 2);
  ```

- [ ] **Step 5: Delete memory**
  ```rust
  ops.delete(&id).await.unwrap();
  ```

- [ ] **Step 6: Verify deletion**
  ```rust
  let result = ops.retrieve(&id).await;
  assert!(matches!(result, Err(MemoryError::NotFound(_))));
  ```

**Concurrent Access**
```rust
#[tokio::test]
async fn test_concurrent_create() {
    let store = TestMemoryStore::new();
    let num_tasks = 100;

    let handles: Vec<_> = (0..num_tasks)
        .map(|i| {
            let store = store.clone();
            tokio::spawn(async move {
                let artifact = Artifact {
                    id: format!("id-{}", i),
                    content: serde_json::json!({"index": i}),
                    tags: vec![],
                    metadata: HashMap::new(),
                    version: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                store.store(artifact).await
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    // Verify all stored
    for i in 0..num_tasks {
        let result = store.retrieve(&format!("id-{}", i)).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_concurrent_update_with_optimistic_locking() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    let num_updaters = 10;
    let handles: Vec<_> = (0..num_updaters)
        .map(|i| {
            let store = store.clone();
            tokio::spawn(async move {
                let content = serde_json::json!({"updater": i});
                // All use version 1, only one should succeed
                store.update(&artifact.id.clone(), content, 1).await
            })
        })
        .collect();

    let mut success_count = 0;
    let mut conflict_count = 0;

    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(MemoryError::VersionConflict(_)) => conflict_count += 1,
            _ => panic!("Unexpected error"),
        }
    }

    assert_eq!(success_count, 1);
    assert_eq!(conflict_count, num_updaters - 1);
}
```

**Large Datasets**
```rust
#[tokio::test]
async fn test_large_dataset_performance() {
    let store = TestMemoryStore::new();
    let num_items = 100_000;

    // Bulk insert
    let start = Instant::now();
    for i in 0..num_items {
        let artifact = Artifact {
            id: format!("id-{:08}", i),
            content: serde_json::json!({"index": i, "data": "x".repeat(100)}),
            tags: vec![format!("tag-{}", i % 100)],
            metadata: HashMap::new(),
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        store.store(artifact).await.unwrap();
    }
    let insert_time = start.elapsed();

    println!("Inserted {} items in {:?}", num_items, insert_time);
    assert!(insert_time < Duration::from_secs(30), "Insert too slow");

    // Random access
    let start = Instant::now();
    for i in 0..1000 {
        let random_id = format!("id-{:08}", i * 100);
        store.retrieve(&random_id).await.unwrap();
    }
    let access_time = start.elapsed();

    println!("Retrieved 1000 items in {:?}", access_time);
    assert!(access_time < Duration::from_secs(5), "Access too slow");
}

#[tokio::test]
async fn test_pagination_with_large_dataset() {
    let store = TestMemoryStore::new();
    let num_items = 1000;

    for i in 0..num_items {
        let mut artifact = create_test_artifact(&format!("id-{}", i));
        artifact.tags = vec!["test".to_string()];
        store.store(artifact).await.unwrap();
    }

    // Test pagination
    let page_size = 100;
    for page in 0..(num_items / page_size) {
        let results = store.list_by_tag_paginated(
            &["test".to_string()],
            page * page_size,
            page_size
        ).await.unwrap();

        assert_eq!(results.len(), page_size);
    }
}
```

**Error Handling**
```rust
#[tokio::test]
async fn test_retrieve_nonexistent_id() {
    let store = TestMemoryStore::new();
    let result = store.retrieve("nonexistent").await;

    assert!(matches!(result, Err(MemoryError::NotFound(_))));
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[tokio::test]
async fn test_update_with_wrong_version() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    let result = store.update(&artifact.id, serde_json::json!({}), 999).await;
    assert!(matches!(result, Err(MemoryError::VersionConflict(_))));
}

#[tokio::test]
async fn test_delete_nonexistent_id() {
    let store = TestMemoryStore::new();
    let result = store.delete("nonexistent").await;

    assert!(matches!(result, Err(MemoryError::NotFound(_))));
}

#[tokio::test]
async fn test_list_with_invalid_tags() {
    let store = TestMemoryStore::new();
    let result = store.list_by_tag(&["nonexistent".to_string()]).await.unwrap();
    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_corrupted_data_recovery() {
    let temp_dir = TempDir::new().unwrap();
    let store = TestMemoryStore::new();

    let artifact = create_test_artifact("test-id");
    store.store(artifact.clone()).await.unwrap();

    // Corrupt the data in sled
    let tree = store.db.open_tree("artifacts").unwrap();
    let corrupted = vec![0xFF; 100];
    tree.insert(artifact.id.as_bytes(), corrupted).unwrap();

    // Should handle gracefully
    let result = store.retrieve(&artifact.id).await;
    assert!(matches!(result, Err(MemoryError::CorruptedData(_))));
}
```

### Edge Cases

```rust
#[tokio::test]
async fn test_empty_query_returns_no_results() {
    let store = TestMemoryStore::new();
    let results = store.list_by_tag(&[]).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_empty_content_storage() {
    let store = TestMemoryStore::new();
    let artifact = Artifact {
        id: "empty".to_string(),
        content: serde_json::json!({}),
        tags: vec![],
        metadata: HashMap::new(),
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    store.store(artifact.clone()).await.unwrap();
    let retrieved = store.retrieve(&artifact.id).await.unwrap();
    assert_eq!(retrieved.content, serde_json::json!({}));
}

#[tokio::test]
async fn test_large_content_storage() {
    let store = TestMemoryStore::new();
    let large_content = "x".repeat(1_000_000); // 1MB

    let artifact = Artifact {
        id: "large".to_string(),
        content: serde_json::json!({"data": large_content}),
        tags: vec![],
        metadata: HashMap::new(),
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    store.store(artifact.clone()).await.unwrap();
    let retrieved = store.retrieve(&artifact.id).await.unwrap();

    assert_eq!(retrieved.content["data"].as_str().unwrap().len(), 1_000_000);
}

#[tokio::test]
async fn test_duplicate_id_handling() {
    let store = TestMemoryStore::new();
    let artifact = create_test_artifact("duplicate");

    store.store(artifact.clone()).await.unwrap();
    let result = store.store(artifact.clone()).await;

    assert!(matches!(result, Err(MemoryError::AlreadyExists(_))));
}

#[tokio::test]
async fn test_special_characters_in_tags() {
    let store = TestMemoryStore::new();
    let mut artifact = create_test_artifact("test-id");
    artifact.tags = vec![
        "tag-with-dash".to_string(),
        "tag_with_underscore".to_string(),
        "tag.with.dot".to_string(),
        "tag:with:colon".to_string(),
        "tag/with/slash".to_string(),
    ];

    store.store(artifact).await.unwrap();

    for tag in &artifact.tags {
        let results = store.list_by_tag(&[tag.clone()]).await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
```

## Cargo Commands

**Run all memory storage tests:**
```bash
cargo test --test memory_storage -- --nocapture
```

**Run specific test:**
```bash
cargo test test_create_structured_memory -- --nocapture
```

**Run property tests:**
```bash
cargo test proptest --test memory_storage -- --nocapture
```

**Run with output:**
```bash
RUST_LOG=debug cargo test --test memory_storage -- --nocapture
```

**Expected output:**
```
running 25 tests
test tests::unit::test_create_structured_memory ... ok
test tests::unit::test_store_artifact ... ok
test tests::unit::test_retrieve_nonexistent ... ok
test tests::unit::test_version_conflict ... ok
test tests::unit::test_versioning_increments_on_update ... ok
test tests::unit::test_version_history_tracking ... ok
test tests::unit::test_add_tags_to_memory ... ok
test tests::unit::test_list_by_tag ... ok
test tests::unit::test_list_by_multiple_tags ... ok
test tests::unit::test_persistence_across_restart ... ok
test tests::unit::test_flush_persists_data ... ok
test tests::integration::test_concurrent_create ... ok
test tests::integration::test_concurrent_update_with_optimistic_locking ... ok
test tests::integration::test_large_dataset_performance ... ok
test tests::integration::test_pagination_with_large_dataset ... ok
test tests::integration::test_retrieve_nonexistent_id ... ok
test tests::integration::test_update_with_wrong_version ... ok
test tests::integration::test_delete_nonexistent_id ... ok
test tests::integration::test_list_with_invalid_tags ... ok
test tests::integration::test_corrupted_data_recovery ... ok
test tests::edge_cases::test_empty_query_returns_no_results ... ok
test tests::edge_cases::test_empty_content_storage ... ok
test tests::edge_cases::test_large_content_storage ... ok
test tests::edge_cases::test_duplicate_id_handling ... ok
test tests::edge_cases::test_special_characters_in_tags ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Inserted 100000 items in 2.5s
Retrieved 1000 items in 150ms
```

## Mock Dependencies

- `tempfile`: Test directory isolation and cleanup
- `sled` with temporary configuration: In-memory database for fast testing
- `tokio::test`: Async test support
- `serde_json`: Test data serialization
- `proptest`: Property-based testing with strategies
- `uuid`: ID validation
- `chrono`: Timestamp handling
- `bincode`: Binary serialization for sled storage
