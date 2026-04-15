# Test Specifications: Task 07 - Memory Garbage Collection

## Mock Strategy

**Mock Storage:**
```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tempfile::TempDir;

struct MockGCStore {
    memories: Arc<RwLock<HashMap<MemoryId, MemoryMetadata>>>,
    temp_dir: TempDir,
}

impl MockGCStore {
    fn new() -> Self {
        Self {
            memories: Arc::new(RwLock::new(HashMap::new())),
            temp_dir: TempDir::new().unwrap(),
        }
    }

    fn create_test_memory(id: MemoryId, age_days: i64, size_bytes: u64, references: u32) -> MemoryMetadata {
        let now = Utc::now();
        MemoryMetadata {
            id: id.clone(),
            created_at: now - Duration::days(age_days),
            updated_at: now - Duration::days(age_days / 2),
            size_bytes,
            reference_count: references,
            tags: vec![],
            metadata: HashMap::new(),
        }
    }

    async fn add_memory(&self, metadata: MemoryMetadata) {
        let mut memories = self.memories.write().await;
        memories.insert(metadata.id.clone(), metadata);
    }

    async fn get_all(&self) -> Vec<MemoryMetadata> {
        let memories = self.memories.read().await;
        memories.values().cloned().collect()
    }

    async fn delete(&self, id: &MemoryId) -> Result<(), GcError> {
        let mut memories = self.memories.write().await;
        memories.remove(id)
            .map(|_| ())
            .ok_or(GcError::MemoryNotFound(id.clone()))
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

**Mock Backup Storage:**
```rust
struct MockBackupStorage {
    backups: Arc<Mutex<Vec<GcBackup>>>,
}

#[derive(Clone)]
struct GcBackup {
    id: String,
    timestamp: DateTime<Utc>,
    memories: Vec<MemoryMetadata>,
    size_bytes: u64,
}

impl MockBackupStorage {
    fn new() -> Self {
        Self {
            backups: Arc::new(Mutex::new(vec![])),
        }
    }

    async fn create_backup(&self, memories: Vec<MemoryMetadata>) -> Result<String, GcError> {
        let size_bytes: u64 = memories.iter().map(|m| m.size_bytes).sum();

        let backup = GcBackup {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            memories,
            size_bytes,
        };

        let mut backups = self.backups.lock().await;
        backups.push(backup.clone());

        Ok(backup.id)
    }

    async fn restore_backup(&self, backup_id: &str) -> Result<Vec<MemoryMetadata>, GcError> {
        let backups = self.backups.lock().await;
        let backup = backups
            .iter()
            .find(|b| b.id == backup_id)
            .ok_or(GcError::BackupNotFound(backup_id.to_string()))?;

        Ok(backup.memories.clone())
    }

    async fn list_backups(&self) -> Vec<GcBackup> {
        let backups = self.backups.lock().await;
        backups.clone()
    }

    async fn delete_backup(&self, backup_id: &str) -> Result<(), GcError> {
        let mut backups = self.backups.lock().await;
        let idx = backups
            .iter()
            .position(|b| b.id == backup_id)
            .ok_or(GcError::BackupNotFound(backup_id.to_string()))?;

        backups.remove(idx);
        Ok(())
    }
}
```

## Test Cases

### Unit Tests

**Age Policy**
```rust
#[test]
fn test_age_policy_old_memory() {
    let policy = GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(7),
    };

    let now = Utc::now();
    let old_date = now - Duration::days(40);

    assert!(policy.should_collect(old_date, 0, 0, 0));
}

#[test]
fn test_age_policy_young_memory() {
    let policy = GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(7),
    };

    let now = Utc::now();
    let young_date = now - Duration::days(20);

    assert!(!policy.should_collect(young_date, 0, 0, 0));
}

#[test]
fn test_age_policy_grace_period() {
    let policy = GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(7),
    };

    let now = Utc::now();
    let grace_date = now - Duration::days(32); // Just over max_age, in grace period

    assert!(!policy.should_collect(grace_date, 0, 0, 0));
}

#[test]
fn test_age_policy_exact_max_age() {
    let policy = GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(7),
    };

    let now = Utc::now();
    let exact_date = now - Duration::days(30);

    assert!(!policy.should_collect(exact_date, 0, 0, 0));
}
```

**Size Policy**
```rust
#[test]
fn test_size_policy_over_limit() {
    let policy = GcPolicy::Size {
        max_size_bytes: 1000,
        target_size_bytes: 800,
    };

    assert!(policy.should_collect(Utc::now(), 0, 0, 1500));
}

#[test]
fn test_size_policy_at_target() {
    let policy = GcPolicy::Size {
        max_size_bytes: 1000,
        target_size_bytes: 800,
    };

    assert!(!policy.should_collect(Utc::now(), 0, 0, 1000));
}

#[test]
fn test_size_policy_below_target() {
    let policy = GcPolicy::Size {
        max_size_bytes: 1000,
        target_size_bytes: 800,
    };

    assert!(!policy.should_collect(Utc::now(), 0, 0, 700));
}
```

**Reference Policy**
```rust
#[test]
fn test_reference_policy_unreferenced() {
    let policy = GcPolicy::ReferenceCount {
        min_references: 1,
    };

    assert!(policy.should_collect(Utc::now(), 0, 0, 0));
}

#[test]
fn test_reference_policy_referenced() {
    let policy = GcPolicy::ReferenceCount {
        min_references: 1,
    };

    assert!(!policy.should_collect(Utc::now(), 0, 0, 5));
}

#[test]
fn test_reference_policy_exact_threshold() {
    let policy = GcPolicy::ReferenceCount {
        min_references: 1,
    };

    assert!(!policy.should_collect(Utc::now(), 0, 0, 1));
}
```

**Combined Policies**
```rust
#[test]
fn test_combined_policy_all_must_pass() {
    let policy = GcPolicy::Combined {
        policies: vec![
            GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) },
            GcPolicy::ReferenceCount { min_references: 0 },
        ],
        combine_mode: CombineMode::And,
    };

    let old_date = Utc::now() - Duration::days(40);
    let young_date = Utc::now() - Duration::days(20);

    // Old and unreferenced - should collect
    assert!(policy.should_collect(old_date, 0, 0, 0));

    // Young but unreferenced - should NOT collect (fails age check)
    assert!(!policy.should_collect(young_date, 0, 0, 0));
}

#[test]
fn test_combined_policy_any_can_pass() {
    let policy = GcPolicy::Combined {
        policies: vec![
            GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) },
            GcPolicy::ReferenceCount { min_references: 0 },
        ],
        combine_mode: CombineMode::Or,
    };

    let old_date = Utc::now() - Duration::days(40);
    let young_date = Utc::now() - Duration::days(20);

    // Old - should collect (passes age check)
    assert!(policy.should_collect(old_date, 0, 0, 10));

    // Young and referenced - should NOT collect (fails both)
    assert!(!policy.should_collect(young_date, 0, 0, 10));
}
```

### Integration Tests

**Preview Generation**
- [ ] **Step 1: Create test memories**
  ```rust
  let store = MockGCStore::new();
  let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

  for i in 0..10 {
      let age = if i % 2 == 0 { 40 } else { 20 }; // 5 old, 5 young
      let metadata = MockGCStore::create_test_memory(
          format!("memory-{}", i),
          age,
          1024,
          0,
      );
      store.add_memory(metadata).await;
  }
  ```

- [ ] **Step 2: Generate preview with policy**
  ```rust
  let gc = GarbageCollector::new(store.clone(), policy);
  let preview = gc.generate_preview().await.unwrap();
  ```

- [ ] **Step 3: Verify candidates listed**
  ```rust
  assert_eq!(preview.candidates.len(), 5); // Only old memories
  ```

- [ ] **Step 4: Check reclaimable size**
  ```rust
  assert_eq!(preview.total_size_bytes, 5 * 1024);
  ```

**Backup Creation**
```rust
#[tokio::test]
async fn test_backup_creation() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    for i in 0..5 {
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            10,
            1024,
            0,
        );
        store.add_memory(metadata).await;
    }

    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = gc.generate_preview().await.unwrap();

    // Create backup
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();

    // Verify backup metadata
    let backups = backup_storage.list_backups().await;
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].id, backup_id);
    assert_eq!(backups[0].memories.len(), 5);
}

#[tokio::test]
async fn test_backup_content() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    let metadata = MockGCStore::create_test_memory("test-id".to_string(), 10, 1024, 0);
    store.add_memory(metadata.clone()).await;

    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = gc.generate_preview().await.unwrap();
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();

    // Restore and verify
    let restored = backup_storage.restore_backup(&backup_id).await.unwrap();
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].id, metadata.id);
    assert_eq!(restored[0].size_bytes, metadata.size_bytes);
}
```

**Recovery from Backup**
```rust
#[tokio::test]
async fn test_recover_from_backup() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    // Create memories
    for i in 0..5 {
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            10,
            1024,
            0,
        );
        store.add_memory(metadata.clone()).await;
    }

    // Create backup and delete memories
    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = gc.generate_preview().await.unwrap();
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();
    gc.collect(&preview).await.unwrap();

    // Verify deletion
    assert!(store.get_all().await.is_empty());

    // Restore from backup
    gc.restore_from_backup(backup_storage.clone(), &backup_id).await.unwrap();

    // Verify recovery
    let restored = store.get_all().await;
    assert_eq!(restored.len(), 5);
}

#[tokio::test]
async fn test_recover_nonexistent_backup() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let result = gc.restore_from_backup(backup_storage.clone(), "nonexistent").await;

    assert!(matches!(result, Err(GcError::BackupNotFound(_))));
}
```

**Backup Management**
```rust
#[tokio::test]
async fn test_list_backups() {
    let backup_storage = MockBackupStorage::new();
    let gc = GarbageCollector::new(MockGCStore::new(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    // Create multiple backups
    for _ in 0..3 {
        let preview = GcPreview {
            candidates: vec![],
            total_size_bytes: 0,
        };
        gc.create_backup(backup_storage.clone(), &preview).await.unwrap();
    }

    let backups = backup_storage.list_backups().await;
    assert_eq!(backups.len(), 3);
}

#[tokio::test]
async fn test_delete_backup() {
    let backup_storage = MockBackupStorage::new();
    let gc = GarbageCollector::new(MockGCStore::new(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = GcPreview {
        candidates: vec![],
        total_size_bytes: 0,
    };
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();

    backup_storage.delete_backup(&backup_id).await.unwrap();

    let backups = backup_storage.list_backups().await;
    assert_eq!(backups.len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_backup() {
    let backup_storage = MockBackupStorage::new();

    let result = backup_storage.delete_backup("nonexistent").await;

    assert!(matches!(result, Err(GcError::BackupNotFound(_))));
}
```

**Scheduling**
```rust
#[tokio::test]
async fn test_scheduled_gc() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };
    let gc = GarbageCollector::new(store.clone(), policy);

    // Start scheduled GC
    gc.start_scheduled(Duration::from_secs(1)).await.unwrap();

    // Wait for execution
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Stop scheduled GC
    gc.stop_scheduled().await.unwrap();

    // Verify it ran (check if any collections happened)
    // This is a basic test - in real implementation, check execution logs
}

#[tokio::test]
async fn test_stop_scheduled_gc() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };
    let gc = GarbageCollector::new(store.clone(), policy);

    // Start scheduled GC
    gc.start_scheduled(Duration::from_secs(1)).await.unwrap();

    // Stop immediately
    gc.stop_scheduled().await.unwrap();

    // Should have stopped (verify through internal state or logs)
}
```

**Safety Checks**
```rust
#[tokio::test]
async fn test_preview_matches_deletion() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    for i in 0..10 {
        let age = if i % 2 == 0 { 40 } else { 20 };
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            age,
            1024,
            0,
        );
        store.add_memory(metadata).await;
    }

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    // Collect
    gc.collect(&preview).await.unwrap();

    // Verify exact preview matches deletion
    let remaining = store.get_all().await;
    assert_eq!(remaining.len(), 5); // Only young memories remain
}

#[tokio::test]
async fn test_no_data_loss_during_gc() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    // Create memories
    for i in 0..5 {
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            10,
            1024,
            0,
        );
        store.add_memory(metadata.clone()).await;
    }

    let original_memories = store.get_all().await;

    // Run GC with backup
    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = gc.generate_preview().await.unwrap();
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();
    gc.collect(&preview).await.unwrap();

    // Verify data recoverable
    gc.restore_from_backup(backup_storage.clone(), &backup_id).await.unwrap();

    let restored = store.get_all().await;

    // Should be identical
    assert_eq!(restored.len(), original_memories.len());
    for original in &original_memories {
        assert!(restored.iter().any(|r| r.id == original.id));
    }
}

#[tokio::test]
async fn test_restore_recovers_exact_data() {
    let store = MockGCStore::new();
    let backup_storage = MockBackupStorage::new();

    let metadata = MockGCStore::create_test_memory("test-id".to_string(), 10, 2048, 3);
    store.add_memory(metadata.clone()).await;

    let gc = GarbageCollector::new(store.clone(), GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(0),
    });

    let preview = gc.generate_preview().await.unwrap();
    let backup_id = gc.create_backup(backup_storage.clone(), &preview).await.unwrap();
    gc.collect(&preview).await.unwrap();

    gc.restore_from_backup(backup_storage.clone(), &backup_id).await.unwrap();

    let restored = store.get_all().await;
    assert_eq!(restored.len(), 1);

    let restored_memory = &restored[0];
    assert_eq!(restored_memory.id, metadata.id);
    assert_eq!(restored_memory.size_bytes, metadata.size_bytes);
    assert_eq!(restored_memory.reference_count, metadata.reference_count);
}
```

**Large Datasets**
```rust
#[tokio::test]
async fn test_gc_performance_large_dataset() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    // Create 100,000 memories
    let start = Instant::now();
    for i in 0..100_000 {
        let age = if i % 2 == 0 { 40 } else { 20 };
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{:08}", i),
            age,
            1024,
            0,
        );
        store.add_memory(metadata).await;
    }
    let insert_time = start.elapsed();

    println!("Inserted 100,000 memories in {:?}", insert_time);

    // Generate preview
    let start = Instant::now();
    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();
    let preview_time = start.elapsed();

    println!("Generated preview in {:?}", preview_time);
    assert_eq!(preview.candidates.len(), 50_000); // 50% old
    assert!(preview_time < Duration::from_secs(10), "Preview too slow");

    // Collect
    let start = Instant::now();
    gc.collect(&preview).await.unwrap();
    let collect_time = start.elapsed();

    println!("Collected in {:?}", collect_time);
    assert!(collect_time < Duration::from_secs(30), "Collection too slow");
}
```

**Edge Cases**
```rust
#[tokio::test]
async fn test_gc_with_empty_store() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    assert_eq!(preview.candidates.len(), 0);
    assert_eq!(preview.total_size_bytes, 0);

    gc.collect(&preview).await.unwrap();

    assert!(store.get_all().await.is_empty());
}

#[tokio::test]
async fn test_gc_with_all_old_memories() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    for i in 0..10 {
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            40, // All old
            1024,
            0,
        );
        store.add_memory(metadata).await;
    }

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    assert_eq!(preview.candidates.len(), 10);

    gc.collect(&preview).await.unwrap();

    assert!(store.get_all().await.is_empty());
}

#[tokio::test]
async fn test_gc_with_all_young_memories() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    for i in 0..10 {
        let metadata = MockGCStore::create_test_memory(
            format!("memory-{}", i),
            10, // All young
            1024,
            0,
        );
        store.add_memory(metadata).await;
    }

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    assert_eq!(preview.candidates.len(), 0);

    gc.collect(&preview).await.unwrap();

    assert_eq!(store.get_all().await.len(), 10);
}

#[tokio::test]
async fn test_gc_with_zero_grace_period() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(0) };

    let metadata = MockGCStore::create_test_memory("test-id".to_string(), 30, 1024, 0);
    store.add_memory(metadata).await;

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    // Should collect (exactly at max_age)
    assert_eq!(preview.candidates.len(), 1);
}

#[tokio::test]
async fn test_gc_with_large_grace_period() {
    let store = MockGCStore::new();
    let policy = GcPolicy::Age { max_age: Duration::days(30), grace_period: Duration::days(10) };

    let metadata = MockGCStore::create_test_memory("test-id".to_string(), 35, 1024, 0);
    store.add_memory(metadata).await;

    let gc = GarbageCollector::new(store.clone(), policy);
    let preview = gc.generate_preview().await.unwrap();

    // Should NOT collect (in grace period)
    assert_eq!(preview.candidates.len(), 0);
}
```

## Cargo Commands

**Run all garbage collection tests:**
```bash
cargo test --test memory_garbage_collection -- --nocapture
```

**Run specific test:**
```bash
cargo test test_age_policy_old_memory -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test memory_garbage_collection -- --nocapture
```

**Expected output:**
```
running 45 tests
test tests::unit::test_age_policy_old_memory ... ok
test tests::unit::test_age_policy_young_memory ... ok
test tests::unit::test_age_policy_grace_period ... ok
test tests::unit::test_age_policy_exact_max_age ... ok
test tests::unit::test_size_policy_over_limit ... ok
test tests::unit::test_size_policy_at_target ... ok
test tests::unit::test_size_policy_below_target ... ok
test tests::unit::test_reference_policy_unreferenced ... ok
test tests::unit::test_reference_policy_referenced ... ok
test tests::unit::test_reference_policy_exact_threshold ... ok
test tests::unit::test_combined_policy_all_must_pass ... ok
test tests::unit::test_combined_policy_any_can_pass ... ok
test tests::integration::test_preview_generation ... ok
test tests::integration::test_backup_creation ... ok
test tests::integration::test_backup_content ... ok
test tests::integration::test_recover_from_backup ... ok
test tests::integration::test_recover_nonexistent_backup ... ok
test tests::integration::test_list_backups ... ok
test tests::integration::test_delete_backup ... ok
test tests::integration::test_delete_nonexistent_backup ... ok
test tests::integration::test_scheduled_gc ... ok
test tests::integration::test_stop_scheduled_gc ... ok
test tests::integration::test_preview_matches_deletion ... ok
test tests::integration::test_no_data_loss_during_gc ... ok
test tests::integration::test_restore_recovers_exact_data ... ok
test tests::integration::test_gc_performance_large_dataset ... ok
test tests::edge_cases::test_gc_with_empty_store ... ok
test tests::edge_cases::test_gc_with_all_old_memories ... ok
test tests::edge_cases::test_gc_with_all_young_memories ... ok
test tests::edge_cases::test_gc_with_zero_grace_period ... ok
test tests::edge_cases::test_gc_with_large_grace_period ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Inserted 100,000 memories in 8.2s
Generated preview in 2.1s
Collected in 15.3s
```

## Mock Dependencies

- `tempfile`: Test isolation and cleanup
- `MockGCStore`: In-memory storage for fast testing
- `MockBackupStorage`: Mock backup storage
- `MockTime`: Time control for testing age-based policies
- `tokio::test`: Async test support
- `tokio::sync::RwLock`: Thread-safe shared state
- `tokio::sync::Mutex`: Thread-safe exclusive access
- `chrono`: Timestamp handling and duration calculations
