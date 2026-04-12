# Test Specifications: Task 07 - Memory Garbage Collection

## Mock Strategy

**Mock Storage:**
```rust
struct MockGCStore {
    memories: Arc<RwLock<Vec<MemoryMetadata>>>,
}
```

## Test Cases

### Unit Tests

**Age Policy**
```rust
#[test]
fn test_age_policy() {
    let policy = GcPolicy::Age {
        max_age: Duration::days(30),
        grace_period: Duration::days(7),
    };
    
    let old_date = Utc::now() - Duration::days(40);
    assert!(policy.should_collect(old_date, 0, 0, 0));
}
```

**Size Policy**
```rust
#[test]
fn test_size_policy() {
    let policy = GcPolicy::Size {
        max_size_bytes: 1000,
        target_size_bytes: 800,
    };
    
    assert!(policy.should_collect(Utc::now(), 0, 0, 1500));
}
```

**Reference Policy**
```rust
#[test]
fn test_reference_policy() {
    let policy = GcPolicy::ReferenceCount {
        min_references: 1,
    };
    
    assert!(policy.should_collect(Utc::now(), 0, 0, 0));
    assert!(!policy.should_collect(Utc::now(), 0, 5, 0));
}
```

### Integration Tests

**Preview Generation**
- Create test memories
- Generate preview with policy
- Verify candidates listed
- Check reclaimable size

**Backup Creation**
- Select memories for deletion
- Create backup
- Verify backup metadata
- Check backup content

**Recovery from Backup**
- Create backup
- Delete memories
- Restore from backup
- Verify data recovered

**Backup Management**
- List all backups
- Delete old backup
- Verify deletion

**Scheduling**
- Start scheduled GC
- Verify runs periodically
- Stop scheduled GC
- Verify stopped

**Safety Checks**
- Preview matches actual deletion
- No data loss during GC
- Restore recovers exact data

## Mock Dependencies

- `tempfile`: Test isolation
- Mock memories: Test data
- Pre-defined timestamps: Predictable tests
