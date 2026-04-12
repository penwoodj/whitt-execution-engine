# Validation Criteria: Task 00 - Local Memory Storage

## Overview

Validate that local memory storage system provides structured and unstructured memory, CRUD operations, versioning, and persistent storage in `./workspace/memory/` directory with ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Foundation Complete
- [ ] **Storage interface implemented**: `MemoryStorage` trait with async methods
- [ ] **Schema definitions complete**: `StructuredMemory` and `UnstructuredMemory` structs
- [ ] **Error types defined**: `MemoryError` enum covering all failure modes
- [ ] **Unit tests pass**: All storage interface tests (`cargo test --package agentsdk-memory`)

**Verification Commands:**
```bash
# Verify trait exists and compiles
cargo check --package agentsdk-memory

# Run unit tests
cargo test --package agentsdk-memory --lib

# Expected output: All tests pass, no compilation errors
```

### Checkpoint 2: CRUD Operations Functional
- [ ] **Create operation works**: Can create both structured and unstructured memory
- [ ] **Read operation works**: Can retrieve memory by ID with version support
- [ ] **Update operation works**: Can update memory with version conflict detection
- [ ] **Delete operation works**: Can delete memory by ID
- [ ] **List operations work**: Can list memories by type and tags

**Verification Commands:**
```bash
# Run integration tests
cargo test --package agentsdk-memory --test integration_test

# Verify CRUD test coverage
cargo test --package agentsdk-memory -- --list | grep test_c

# Expected output: All CRUD tests pass (>90% code coverage)
```

### Checkpoint 3: Persistence and Recovery
- [ ] **Data persists to filesystem**: Files created in `./workspace/memory/`
- [ ] **Data survives restart**: Memory available after process restart
- [ ] **Versioning works**: Multiple versions preserved with timestamps
- [ ] **Directory structure created**: Correct hierarchy created automatically

**Verification Commands:**
```bash
# Verify directory structure
ls -la ./workspace/memory/
# Expected: structured/, unstructured/, versions/ directories exist

# Verify file persistence
find ./workspace/memory/ -type f | wc -l
# Expected: Files created based on test data

# Test persistence across restart
cargo test --package agentsdk-memory persistence_test
# Expected: Data available after simulated restart
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Create latency < 10ms**: Measured with benchmark suite
- [ ] **Read latency < 5ms**: Measured with benchmark suite
- [ ] **Update latency < 15ms**: Measured with benchmark suite
- [ ] **Delete latency < 10ms**: Measured with benchmark suite
- [ ] **List latency < 50ms for 1000 items**: Measured with benchmark suite

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-memory

# Verify performance metrics
grep -A 5 "create.*time" target/criterion/report/index.html
# Expected: Mean time < 10.0 ms

# Check all latency targets
cargo test --package agentsdk-memory performance_test
# Expected: All performance assertions pass
```

---

## Functional Requirements

### Memory Storage

#### Structured Memory
- [ ] **Can create structured memory with JSON data**
  - Test: `test_create_structured_memory()`
  - Command: `cargo test --package agentsdk-memory test_create_structured_memory`
  - Expected: PASS, memory ID returned, file created

- [ ] **Can create unstructured memory with text content**
  - Test: `test_create_unstructured_memory()`
  - Command: `cargo test --package agentsdk-memory test_create_unstructured_memory`
  - Expected: PASS, memory ID returned, file created

- [ ] **Can retrieve memory by ID**
  - Test: `test_retrieve_by_id()`
  - Command: `cargo test --package agentsdk-memory test_retrieve_by_id`
  - Expected: PASS, content matches what was stored

- [ ] **Can retrieve memory by version**
  - Test: `test_retrieve_by_version()`
  - Command: `cargo test --package agentsdk-memory test_retrieve_by_version`
  - Expected: PASS, specific version returned

- [ ] **Can update structured memory with version conflict detection**
  - Test: `test_update_with_version_conflict()`
  - Command: `cargo test --package agentsdk-memory test_update_with_version_conflict`
  - Expected: PASS, `VersionConflict` error for stale versions

- [ ] **Can delete memory by ID**
  - Test: `test_delete_memory()`
  - Command: `cargo test --package agentsdk-memory test_delete_memory`
  - Expected: PASS, file removed from filesystem

- [ ] **Can list memories by type**
  - Test: `test_list_by_type()`
  - Command: `cargo test --package agentsdk-memory test_list_by_type`
  - Expected: PASS, only specified type returned

- [ ] **Can list memories by tags**
  - Test: `test_list_by_tags()`
  - Command: `cargo test --package agentsdk-memory test_list_by_tags`
  - Expected: PASS, only matching tags returned

### Versioning

- [ ] **Creates version backups on update**
  - Test: `test_version_backup_on_update()`
  - Command: `cargo test --package agentsdk-memory test_version_backup_on_update`
  - Expected: PASS, previous version preserved in `versions/` directory

- [ ] **Retrieves specific version**
  - Test: `test_retrieve_specific_version()`
  - Command: `cargo test --package agentsdk-memory test_retrieve_specific_version`
  - Expected: PASS, exact version content returned

- [ ] **Lists all versions for a memory**
  - Test: `test_list_all_versions()`
  - Command: `cargo test --package agentsdk-memory test_list_all_versions`
  - Expected: PASS, all versions with metadata returned

- [ ] **Detects version conflicts**
  - Test: `test_version_conflict_detection()`
  - Command: `cargo test --package agentsdk-memory test_version_conflict_detection`
  - Expected: PASS, `VersionConflict` error when version mismatch

### Persistence

- [ ] **Structured memory persisted to filesystem**
  - Test: `test_structured_persistence()`
  - Command: `cargo test --package agentsdk-memory test_structured_persistence`
  - Expected: PASS, file exists in `./workspace/memory/structured/`

- [ ] **Unstructured memory persisted to filesystem**
  - Test: `test_unstructured_persistence()`
  - Command: `cargo test --package agentsdk-memory test_unstructured_persistence`
  - Expected: PASS, file exists in `./workspace/memory/unstructured/`

- [ ] **Data survives process restart**
  - Test: `test_survive_restart()`
  - Command: `cargo test --package agentsdk-memory test_survive_restart`
  - Expected: PASS, data available after simulated restart

- [ ] **Storage directory structure created automatically**
  - Test: `test_directory_creation()`
  - Command: `cargo test --package agentsdk-memory test_directory_creation`
  - Expected: PASS, all required directories exist after first write

---

## Performance Requirements

### Latency Targets

- [ ] **Create operation < 10ms**
  - Benchmark: `bench_create_memory`
  - Command: `cargo bench --bench memory_bench bench_create_memory`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

- [ ] **Read operation < 5ms**
  - Benchmark: `bench_read_memory`
  - Command: `cargo bench --bench memory_bench bench_read_memory`
  - Expected: Mean < 5.0 ms, p95 < 8.0 ms

- [ ] **Update operation < 15ms**
  - Benchmark: `bench_update_memory`
  - Command: `cargo bench --bench memory_bench bench_update_memory`
  - Expected: Mean < 15.0 ms, p95 < 20.0 ms

- [ ] **Delete operation < 10ms**
  - Benchmark: `bench_delete_memory`
  - Command: `cargo bench --bench memory_bench bench_delete_memory`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

- [ ] **List operation < 50ms for 1000 items**
  - Benchmark: `bench_list_1000_items`
  - Command: `cargo bench --bench memory_bench bench_list_1000_items`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

### Throughput Targets

- [ ] **1000 create operations < 5 seconds**
  - Test: `test_bulk_create_1000()`
  - Command: `cargo test --package agentsdk-memory test_bulk_create_1000`
  - Expected: PASS, total time < 5.0s

- [ ] **10000 read operations < 10 seconds**
  - Test: `test_bulk_read_10000()`
  - Command: `cargo test --package agentsdk-memory test_bulk_read_10000`
  - Expected: PASS, total time < 10.0s

---

## Data Integrity

### Uniqueness and Consistency

- [ ] **Memory IDs are unique UUIDs**
  - Test: `test_uuid_uniqueness()`
  - Command: `cargo test --package agentsdk-memory test_uuid_uniqueness`
  - Expected: PASS, no collisions in 10000 IDs

- [ ] **Version numbers are monotonically increasing**
  - Test: `test_version_monotonic()`
  - Command: `cargo test --package agentsdk-memory test_version_monotonic`
  - Expected: PASS, versions always increase

- [ ] **Created_at timestamps are immutable**
  - Test: `test_created_at_immutability()`
  - Command: `cargo test --package agentsdk-memory test_created_at_immutability`
  - Expected: PASS, created_at never changes after creation

- [ ] **Updated_at timestamps are updated on modification**
  - Test: `test_updated_at_updates()`
  - Command: `cargo test --package agentsdk-memory test_updated_at_updates`
  - Expected: PASS, updated_at changes on each update

- [ ] **Content hashes are computed correctly (SHA-256)**
  - Test: `test_hash_correctness()`
  - Command: `cargo test --package agentsdk-memory test_hash_correctness`
  - Expected: PASS, hash matches SHA-256 of content

---

## Error Handling

### Error Types

- [ ] **NotFound error for non-existent memory**
  - Test: `test_not_found_error()`
  - Command: `cargo test --package agentsdk-memory test_not_found_error`
  - Expected: `MemoryError::NotFound("memory_id")`

- [ ] **VersionConflict error for mismatched versions**
  - Test: `test_version_conflict_error()`
  - Command: `cargo test --package agentsdk-memory test_version_conflict_error`
  - Expected: `MemoryError::VersionConflict { expected, actual }`

- [ ] **InvalidId error for malformed IDs**
  - Test: `test_invalid_id_error()`
  - Command: `cargo test --package agentsdk-memory test_invalid_id_error`
  - Expected: `MemoryError::InvalidId("malformed_uuid")`

- [ ] **StorageLimitExceeded error when quota exceeded**
  - Test: `test_storage_limit_error()`
  - Command: `cargo test --package agentsdk-memory test_storage_limit_error`
  - Expected: `MemoryError::StorageLimitExceeded { current, limit }`

### Recovery Scenarios

- [ ] **Recovers from disk full error gracefully**
  - Test: `test_disk_full_recovery()`
  - Command: `cargo test --package agentsdk-memory test_disk_full_recovery`
  - Expected: PASS, error returned but storage intact

- [ ] **Recovers from corrupted file gracefully**
  - Test: `test_corrupted_file_recovery()`
  - Command: `cargo test --package agentsdk-memory test_corrupted_file_recovery`
  - Expected: PASS, corrupted file skipped, others accessible

- [ ] **Handles concurrent access correctly**
  - Test: `test_concurrent_access()`
  - Command: `cargo test --package agentsdk-memory test_concurrent_access`
  - Expected: PASS, no data corruption with 10 concurrent writers

---

## Schema Compliance

### Structured Memory Schema

- [ ] **Fields match schema specification**
  - Test: `test_structured_schema_compliance()`
  - Command: `cargo test --package agentsdk-memory test_structured_schema_compliance`
  - Expected: All required fields present, types correct

- [ ] **JSON serialization/deserialization works**
  - Test: `test_json_serde()`
  - Command: `cargo test --package agentsdk-memory test_json_serde`
  - Expected: PASS, data survives round-trip

- [ ] **Metadata fields are preserved**
  - Test: `test_metadata_preservation()`
  - Command: `cargo test --package agentsdk-memory test_metadata_preservation`
  - Expected: Tags, labels, and custom metadata preserved

### Unstructured Memory Schema

- [ ] **Fields match schema specification**
  - Test: `test_unstructured_schema_compliance()`
  - Command: `cargo test --package agentsdk-memory test_unstructured_schema_compliance`
  - Expected: All required fields present, types correct

- [ ] **Large content handled correctly**
  - Test: `test_large_content()`
  - Command: `cargo test --package agentsdk-memory test_large_content`
  - Expected: PASS, 10MB content stored and retrieved

- [ ] **Content type detection works**
  - Test: `test_content_type_detection()`
  - Command: `cargo test --package agentsdk-memory test_content_type_detection`
  - Expected: PASS, MIME type detected correctly

---

## ADR-0006 Compliance

### Local Memory First

- [ ] **Memory stored in `./workspace/memory/` directory**
  - Test: `test_storage_location()`
  - Command: `cargo test --package agentsdk-memory test_storage_location`
  - Expected: All files in `./workspace/memory/` hierarchy

- [ ] **No external storage dependencies**
  - Test: `test_local_only_storage()`
  - Command: `cargo test --package agentsdk-memory test_local_only_storage`
  - Expected: PASS, no network calls or external APIs

- [ ] **Versioned references maintained**
  - Test: `test_versioned_references()`
  - Command: `cargo test --package agentsdk-memory test_versioned_references`
  - Expected: All versions accessible via reference system

- [ ] **All storage operations record content hashes**
  - Test: `test_content_hashing()`
  - Command: `cargo test --package agentsdk-memory test_content_hashing`
  - Expected: All operations include SHA-256 hash in metadata

### Priority Enforcement

- [ ] **Local storage is ALWAYS used before any external storage**
  - Test: `test_local_storage_priority()`
  - Command: `cargo test --package agentsdk-memory test_local_storage_priority`
  - Expected: Local operations complete without external access attempts

- [ ] **No fallback to external storage**
  - Test: `test_no_external_fallback()`
  - Command: `cargo test --package agentsdk-memory test_no_external_fallback`
  - Expected: Errors propagate, no external storage used

---

## Integration Points

### Phase 1: Workflow Engine Integration

- [ ] **Memory storage accessible from workflow context**
  - Test: `test_workflow_context_access()`
  - Command: `cargo test --package agentsdk-memory --test workflow_integration`
  - Expected: PASS, workflow can store/retrieve memory

- [ ] **Memory IDs compatible with workflow variables**
  - Test: `test_variable_compatibility()`
  - Command: `cargo test --package agentsdk-memory test_variable_compatibility`
  - Expected: PASS, memory IDs usable as workflow variables

### Phase 2: Tool System Integration

- [ ] **Memory storage tool works**
  - Test: `test_memory_storage_tool()`
  - Command: `cargo test --package agentsdk-memory --test tool_integration`
  - Expected: PASS, tool can call storage operations

- [ ] **Memory retrieve tool works**
  - Test: `test_memory_retrieve_tool()`
  - Command: `cargo test --package agentsdk-memory test_memory_retrieve_tool`
  - Expected: PASS, tool can retrieve stored memory

### Phase 3: Context Management Integration

- [ ] **Context can inject memory into prompts**
  - Test: `test_context_memory_injection()`
  - Command: `cargo test --package agentsdk-memory test_context_memory_injection`
  - Expected: PASS, memory content available in LLM context

- [ ] **Memory can be extracted from LLM outputs**
  - Test: `test_context_memory_extraction()`
  - Command: `cargo test --package agentsdk-memory test_context_memory_extraction`
  - Expected: PASS, LLM outputs parsed and stored

---

## Log Verification Patterns

### Log Format

- [ ] **Logs follow structured JSON format**
  - Grep: `jq -r '.level' ./workspace/logs/memory.log | sort | uniq -c`
  - Expected: Counts for debug, info, warn, error levels

- [ ] **All operations include trace ID**
  - Grep: `grep -o '"trace_id":"[^"]*"' ./workspace/logs/memory.log | wc -l`
  - Expected: Count equals number of operations

- [ ] **Timestamps are ISO 8601 format**
  - Grep: `grep -oP '"timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z"' ./workspace/logs/memory.log | wc -l`
  - Expected: All timestamps match format

### Operation Logs

- [ ] **Create operations logged**
  - Grep: `grep '"operation":"create"' ./workspace/logs/memory.log | wc -l`
  - Expected: Count equals number of create operations

- [ ] **Read operations logged**
  - Grep: `grep '"operation":"read"' ./workspace/logs/memory.log | wc -l`
  - Expected: Count equals number of read operations

- [ ] **Update operations logged**
  - Grep: `grep '"operation":"update"' ./workspace/logs/memory.log | wc -l`
  - Expected: Count equals number of update operations

- [ ] **Delete operations logged**
  - Grep: `grep '"operation":"delete"' ./workspace/logs/memory.log | wc -l`
  - Expected: Count equals number of delete operations

### Error Logs

- [ ] **Errors include full error context**
  - Grep: `grep '"level":"error"' ./workspace/logs/memory.log | jq -r '.message'`
  - Expected: Descriptive error messages with details

- [ ] **Errors include stack traces in debug mode**
  - Grep: `grep '"level":"error"' ./workspace/logs/memory.log | jq -r '.stack_trace' | head -1`
  - Expected: Stack trace present when log level is debug

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for all CRUD operations**
  - Command: `cargo test --package agentsdk-memory --lib -- --nocapture`
  - Expected: All unit tests pass, >95% coverage

- [ ] **Unit tests for versioning logic**
  - Command: `cargo test --package agentsdk-memory versioning`
  - Expected: All versioning tests pass

- [ ] **Unit tests for hash computation**
  - Command: `cargo test --package agentsdk-memory hashing`
  - Expected: All hash tests pass

- [ ] **Unit tests for error handling**
  - Command: `cargo test --package agentsdk-memory errors`
  - Expected: All error tests pass

### Integration Tests

- [ ] **Integration tests for full workflow**
  - Command: `cargo test --package agentsdk-memory --test integration_test`
  - Expected: All integration tests pass

- [ ] **Integration tests for persistence**
  - Command: `cargo test --package agentsdk-memory persistence_integration`
  - Expected: All persistence tests pass

- [ ] **Integration tests for concurrent access**
  - Command: `cargo test --package agentsdk-memory concurrent_integration`
  - Expected: All concurrent tests pass

### Error Case Tests

- [ ] **Error case tests for all error types**
  - Command: `cargo test --package agentsdk-memory error_cases`
  - Expected: All error case tests pass

- [ ] **Edge case tests for boundary conditions**
  - Command: `cargo test --package agentsdk-memory edge_cases`
  - Expected: All edge case tests pass

### Concurrent Access Tests

- [ ] **Concurrent read tests**
  - Command: `cargo test --package agentsdk-memory concurrent_reads`
  - Expected: No data corruption under concurrent reads

- [ ] **Concurrent write tests**
  - Command: `cargo test --package agentsdk-memory concurrent_writes`
  - Expected: No data corruption under concurrent writes

- [ ] **Concurrent read-write tests**
  - Command: `cargo test --package agentsdk-memory concurrent_read_write`
  - Expected: No data corruption under mixed operations

---

## Performance Benchmarks

### Latency Benchmarks

- [ ] **P50 latency < target**
  - Command: `cargo bench --bench memory_bench | grep "P50"`
  - Expected: All P50 values < target latency

- [ ] **P95 latency < target * 1.5**
  - Command: `cargo bench --bench memory_bench | grep "P95"`
  - Expected: All P95 values < target * 1.5

- [ ] **P99 latency < target * 2.0**
  - Command: `cargo bench --bench memory_bench | grep "P99"`
  - Expected: All P99 values < target * 2.0

### Throughput Benchmarks

- [ ] **Operations per second > target**
  - Command: `cargo bench --bench memory_bench throughput`
  - Expected: All throughput values > target

- [ ] **Sustained throughput over time**
  - Command: `cargo test --package agentsdk-memory sustained_throughput`
  - Expected: No degradation over 10 minutes

### Resource Usage

- [ ] **Memory usage < target**
  - Command: `cargo test --package agentsdk-memory memory_usage`
  - Expected: Memory usage < 100MB for 10000 items

- [ ] **Disk usage efficient**
  - Command: `du -sh ./workspace/memory/`
  - Expected: < 1GB for 10000 items with compression

---

## Final Checklist

### Implementation Complete
- [ ] All CRUD operations implemented and tested
- [ ] Versioning system functional
- [ ] Persistence works across restarts
- [ ] All error cases handled gracefully
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] Local memory storage in `./workspace/memory/`
- [ ] No external storage dependencies
- [ ] Versioned references maintained
- [ ] Content hashes recorded for all operations

### Integration Ready
- [ ] Workflow engine can access memory
- [ ] Tool system can call storage operations
- [ ] Context management can inject memory

### Documentation Complete
- [ ] API documentation generated
- [ ] Usage examples provided
- [ ] Performance characteristics documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 95%

**Total Lines:** ~550 lines
