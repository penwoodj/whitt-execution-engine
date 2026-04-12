# Validation Criteria: Task 07 - Memory Garbage Collection

## Overview

Validate that memory garbage collection system provides GC policies, preview, recovery, scheduling, and ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: GC Policies Complete
- [ ] **Age policy works correctly**: Removes old items
- [ ] **Size policy works correctly**: Removes to fit size limit
- [ ] **Reference count policy works correctly**: Removes unreferenced items
- [ ] **Combined policies work correctly**: Multiple policies applied
- [ ] **Policies are configurable**: Custom settings supported
- [ ] **Unit tests pass**: All policy tests (`cargo test --package agentsdk-garbage`)

**Verification Commands:**
```bash
# Verify garbage crate builds
cargo check --package agentsdk-garbage

# Run policy tests
cargo test --package agentsdk-garbage --lib policies

# Expected output: All policy tests pass
```

### Checkpoint 2: Preview Functional
- [ ] **Generates preview of items to collect**: List candidates
- [ ] **Shows total size and reclaimable size**: Space analysis
- [ ] **Lists candidates with reasons**: Explanation for each
- [ ] **Dry-run mode supported**: Preview without deletion
- [ ] **Unit tests pass**: All preview tests

**Verification Commands:**
```bash
# Run preview tests
cargo test --package agentsdk-garbage --lib preview

# Verify preview generation
cargo test --package agentsdk-garbage test_preview_generation

# Expected output: All preview tests pass
```

### Checkpoint 3: Recovery Working
- [ ] **Creates backup before deletion**: Safe deletion
- [ ] **Backup metadata recorded**: Restoration information saved
- [ ] **Can restore from backup**: Rollback capability
- [ ] **Can list all backups**: Inventory management
- [ ] **Can delete old backups**: Cleanup mechanism
- [ ] **Unit tests pass**: All recovery tests

**Verification Commands:**
```bash
# Run recovery tests
cargo test --package agentsdk-garbage --lib recovery

# Verify backup creation
cargo test --package agentsdk-garbage test_backup_creation

# Verify restore functionality
cargo test --package agentsdk-garbage test_restore_functionality

# Expected output: All recovery tests pass
```

### Checkpoint 4: Scheduling Functional
- [ ] **Can start scheduled GC**: Automatic cleanup
- [ ] **Can stop scheduled GC**: Cancellation capability
- [ ] **Can enable/disable GC**: Toggle capability
- [ ] **Interval configurable**: Custom schedule
- [ ] **Unit tests pass**: All scheduling tests

**Verification Commands:**
```bash
# Run scheduling tests
cargo test --package agentsdk-garbage --lib scheduling

# Verify GC scheduling
cargo test --package agentsdk-garbage test_gc_scheduling

# Expected output: All scheduling tests pass
```

### Checkpoint 5: Performance Meets Targets
- [ ] **Policy evaluation < 1ms per item**: Measured with benchmarks
- [ ] **Preview generation < 1s for 1000 items**: Measured with benchmarks
- [ ] **Backup creation < 5s for 1000 items**: Measured with benchmarks
- [ ] **Restore operation < 5s for 1000 items**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-garbage

# Verify policy evaluation performance
cargo bench --bench garbage_bench bench_policy_eval

# Verify backup performance
cargo bench --bench garbage_bench bench_backup_creation

# Expected output: All latency targets met
```

---

## Functional Requirements

### GC Policies

#### Age Policy
- [ ] **Age policy works correctly**
  - Test: `test_age_policy()`
  - Command: `cargo test --package agentsdk-garbage test_age_policy`
  - Expected: PASS, removes items older than threshold

- [ ] **Age threshold configurable**
  - Test: `test_age_policy_config()`
  - Command: `cargo test --package agentsdk-garbage test_age_policy_config`
  - Expected: PASS, custom age threshold works

#### Size Policy
- [ ] **Size policy works correctly**
  - Test: `test_size_policy()`
  - Command: `cargo test --package agentsdk-garbage test_size_policy`
  - Expected: PASS, removes to fit size limit

- [ ] **Size threshold configurable**
  - Test: `test_size_policy_config()`
  - Command: `cargo test --package agentsdk-garbage test_size_policy_config`
  - Expected: PASS, custom size limit works

- [ ] **Prioritizes largest items first**
  - Test: `test_size_policy_priority()`
  - Command: `cargo test --package agentsdk-garbage test_size_policy_priority`
  - Expected: PASS, removes largest items to free space

#### Reference Count Policy
- [ ] **Reference count policy works correctly**
  - Test: `test_refcount_policy()`
  - Command: `cargo test --package agentsdk-garbage test_refcount_policy`
  - Expected: PASS, removes unreferenced items

- [ ] **Updates reference counts correctly**
  - Test: `test_refcount_updates()`
  - Command: `cargo test --package agentsdk-garbage test_refcount_updates`
  - Expected: PASS, refcounts accurate

#### Combined Policies
- [ ] **Combined policies work correctly**
  - Test: `test_combined_policy()`
  - Command: `cargo test --package agentsdk-garbage test_combined_policy`
  - Expected: PASS, all policies applied in order

- [ ] **Policies are configurable**
  - Test: `test_policy_configuration()`
  - Command: `cargo test --package agentsdk-garbage test_policy_configuration`
  - Expected: PASS, custom policy sets work

### Preview

#### Preview Generation
- [ ] **Generates preview of items to collect**
  - Test: `test_preview_generation()`
  - Command: `cargo test --package agentsdk-garbage test_preview_generation`
  - Expected: PASS, list of candidates returned

- [ ] **Shows total size and reclaimable size**
  - Test: `test_size_analysis()`
  - Command: `cargo test --package agentsdk-garbage test_size_analysis`
  - Expected: PASS, size metrics accurate

- [ ] **Lists candidates with reasons**
  - Test: `test_candidate_reasons()`
  - Command: `cargo test --package agentsdk-garbage test_candidate_reasons`
  - Expected: PASS, each item includes deletion reason

- [ ] **Dry-run mode supported**
  - Test: `test_dry_run_mode()`
  - Command: `cargo test --package agentsdk-garbage test_dry_run_mode`
  - Expected: PASS, preview without deletion

### Recovery

#### Backup Creation
- [ ] **Creates backup before deletion**
  - Test: `test_backup_creation()`
  - Command: `cargo test --package agentsdk-garbage test_backup_creation`
  - Expected: PASS, backup created before GC

- [ ] **Backup metadata recorded**
  - Test: `test_backup_metadata()`
  - Command: `cargo test --package agentsdk-garbage test_backup_metadata`
  - Expected: PASS, restoration info saved

- [ ] **Backup contains all deleted data**
  - Test: `test_backup_completeness()`
  - Command: `cargo test --package agentsdk-garbage test_backup_completeness`
  - Expected: PASS, all deleted items in backup

#### Restore Operations
- [ ] **Can restore from backup**
  - Test: `test_restore_operation()`
  - Command: `cargo test --package agentsdk-garbage test_restore_operation`
  - Expected: PASS, data restored from backup

- [ ] **Can list all backups**
  - Test: `test_list_backups()`
  - Command: `cargo test --package agentsdk-garbage test_list_backups`
  - Expected: PASS, all backups listed with metadata

- [ ] **Can delete old backups**
  - Test: `test_delete_old_backups()`
  - Command: `cargo test --package agentsdk-garbage test_delete_old_backups`
  - Expected: PASS, old backups removed

#### Restore Accuracy
- [ ] **Restore recovers data exactly**
  - Test: `test_restore_accuracy()`
  - Command: `cargo test --package agentsdk-garbage test_restore_accuracy`
  - Expected: PASS, data identical to pre-delete state

- [ ] **Restores metadata correctly**
  - Test: `test_metadata_restore()`
  - Command: `cargo test --package agentsdk-garbage test_metadata_restore`
  - Expected: PASS, metadata restored accurately

### Scheduling

#### GC Scheduling
- [ ] **Can start scheduled GC**
  - Test: `test_start_scheduled_gc()`
  - Command: `cargo test --package agentsdk-garbage test_start_scheduled_gc`
  - Expected: PASS, GC runs on schedule

- [ ] **Can stop scheduled GC**
  - Test: `test_stop_scheduled_gc()`
  - Command: `cargo test --package agentsdk-garbage test_stop_scheduled_gc`
  - Expected: PASS, scheduled GC cancelled

- [ ] **Can enable/disable GC**
  - Test: `test_gc_toggle()`
  - Command: `cargo test --package agentsdk-garbage test_gc_toggle`
  - Expected: PASS, GC can be enabled/disabled

- [ ] **Interval configurable**
  - Test: `test_interval_config()`
  - Command: `cargo test --package agentsdk-garbage test_interval_config`
  - Expected: PASS, custom interval works

#### GC Execution
- [ ] **Scheduled GC runs automatically**
  - Test: `test_automatic_gc()`
  - Command: `cargo test --package agentsdk-garbage test_automatic_gc`
  - Expected: PASS, GC runs at interval

- [ ] **GC respects enable/disable state**
  - Test: `test_gc_respects_state()`
  - Command: `cargo test --package agentsdk-garbage test_gc_respects_state`
  - Expected: PASS, disabled GC doesn't run

---

## Performance Requirements

### Policy Evaluation

- [ ] **Policy evaluation < 1ms per item**
  - Benchmark: `bench_policy_eval`
  - Command: `cargo bench --bench garbage_bench bench_policy_eval`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

### Preview Performance

- [ ] **Preview generation < 1s for 1000 items**
  - Benchmark: `bench_preview_generation`
  - Command: `cargo bench --bench garbage_bench bench_preview_generation`
  - Expected: Mean < 1.0s, p95 < 1.5s

### Backup Performance

- [ ] **Backup creation < 5s for 1000 items**
  - Benchmark: `bench_backup_creation`
  - Command: `cargo bench --bench garbage_bench bench_backup_creation`
  - Expected: Mean < 5.0s, p95 < 6.0s

### Restore Performance

- [ ] **Restore operation < 5s for 1000 items**
  - Benchmark: `bench_restore_operation`
  - Command: `cargo bench --bench garbage_bench bench_restore_operation`
  - Expected: Mean < 5.0s, p95 < 6.0s

---

## Safety

### Data Integrity

- [ ] **No data loss during GC**
  - Test: `test_no_data_loss()`
  - Command: `cargo test --package agentsdk-garbage test_no_data_loss`
  - Expected: PASS, deleted items recoverable

- [ ] **Backups contain all deleted data**
  - Test: `test_backup_completeness()`
  - Command: `cargo test --package agentsdk-garbage test_backup_completeness`
  - Expected: PASS, all deleted items backed up

- [ ] **Restore recovers data exactly**
  - Test: `test_restore_completeness()`
  - Command: `cargo test --package agentsdk-garbage test_restore_completeness`
  - Expected: PASS, restore matches original

### Preview Accuracy

- [ ] **Preview matches actual deletion**
  - Test: `test_preview_accuracy()`
  - Command: `cargo test --package agentsdk-garbage test_preview_accuracy`
  - Expected: PASS, preview lists exactly what will be deleted

---

## Error Handling

### Backup Errors

- [ ] **Handles backup creation errors**
  - Test: `test_backup_creation_error()`
  - Command: `cargo test --package agentsdk-garbage test_backup_creation_error`
  - Expected: PASS, GC aborted on backup failure

- [ ] **Handles restore errors**
  - Test: `test_restore_error()`
  - Command: `cargo test --package agentsdk-garbage test_restore_error`
  - Expected: PASS, error with helpful message

- [ ] **Handles storage errors**
  - Test: `test_storage_error_handling()`
  - Command: `cargo test --package agentsdk-garbage test_storage_error_handling`
  - Expected: PASS, graceful degradation on errors

### GC Errors

- [ ] **Handles policy evaluation errors**
  - Test: `test_policy_error_handling()`
  - Command: `cargo test --package agentsdk-garbage test_policy_error_handling`
  - Expected: PASS, error with details

- [ ] **Handles deletion errors gracefully**
  - Test: `test_deletion_error_handling()`
  - Command: `cargo test --package agentsdk-garbage test_deletion_error_handling`
  - Expected: PASS, continues with remaining items

---

## ADR-0006 Compliance

### GC Policies

- [ ] **GC policies configurable**
  - Test: `test_gc_policy_configurability()`
  - Command: `cargo test --package agentsdk-garbage test_gc_policy_configurability`
  - Expected: PASS, all policies configurable

- [ ] **Preview shows what will be deleted**
  - Test: `test_gc_preview()`
  - Command: `cargo test --package agentsdk-garbage test_gc_preview`
  - Expected: PASS, preview available before deletion

- [ ] **Recovery can restore deleted data**
  - Test: `test_gc_recovery()`
  - Command: `cargo test --package agentsdk-garbage test_gc_recovery`
  - Expected: PASS, rollback possible

- [ ] **All GC operations track provenance**
  - Test: `test_gc_provenance()`
  - Command: `cargo test --package agentsdk-garbage test_gc_provenance`
  - Expected: PASS, all GC operations logged

---

## Integration Points

### Memory Integration

- [ ] **GC works with memory storage**
  - Test: `test_memory_integration()`
  - Command: `cargo test --package agentsdk-garbage test_memory_integration`
  - Expected: PASS, GC cleans memory storage

- [ ] **GC updates memory indices**
  - Test: `test_index_integration()`
  - Command: `cargo test --package agentsdk-garbage test_index_integration`
  - Expected: PASS, indices updated after GC

### Provenance Integration

- [ ] **GC operations recorded in provenance**
  - Test: `test_provenance_integration()`
  - Command: `cargo test --package agentsdk-garbage test_provenance_integration`
  - Expected: PASS, GC events in trace log

- [ ] **Backup metadata in provenance**
  - Test: `test_backup_provenance()`
  - Command: `cargo test --package agentsdk-garbage test_backup_provenance`
  - Expected: PASS, backup info in trace log

---

## Log Verification Patterns

### GC Operation Logs

- [ ] **GC runs logged**
  - Grep: `grep '"operation":"gc_run"' ./workspace/logs/garbage.log | wc -l`
  - Expected: Count equals number of GC runs

- [ ] **GC policy logged**
  - Grep: `grep '"operation":"gc_run"' ./workspace/logs/garbage.log | jq -r '.policy'`
  - Expected: Policy present for all runs

- [ ] **Items deleted logged**
  - Grep: `grep '"operation":"gc_delete"' ./workspace/logs/garbage.log | wc -l`
  - Expected: Count equals number of deletions

### Backup Logs

- [ ] **Backup creation logged**
  - Grep: `grep '"operation":"backup_create"' ./workspace/logs/garbage.log | wc -l`
  - Expected: Count equals number of backups

- [ ] **Backup location logged**
  - Grep: `grep '"operation":"backup_create"' ./workspace/logs/garbage.log | jq -r '.backup_path'`
  - Expected: Path present for all backups

- [ ] **Backup size logged**
  - Grep: `grep '"operation":"backup_create"' ./workspace/logs/garbage.log | jq -r '.backup_size_bytes'`
  - Expected: Size present for all backups

### Restore Logs

- [ ] **Restore operations logged**
  - Grep: `grep '"operation":"restore"' ./workspace/logs/garbage.log | wc -l`
  - Expected: Count equals number of restores

- [ ] **Restore source logged**
  - Grep: `grep '"operation":"restore"' ./workspace/logs/garbage.log | jq -r '.backup_id'`
  - Expected: Backup ID present for all restores

### Error Logs

- [ ] **GC errors logged**
  - Grep: `grep '"level":"error"' ./workspace/logs/garbage.log | grep gc | jq -r '.error'`
  - Expected: Error details present for all failures

- [ ] **Backup errors logged**
  - Grep: `grep '"level":"error"' ./workspace/logs/garbage.log | grep backup | jq -r '.error'`
  - Expected: Error details present for all failures

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for policies**
  - Command: `cargo test --package agentsdk-garbage --lib policies`
  - Expected: All policy tests pass

- [ ] **Unit tests for preview**
  - Command: `cargo test --package agentsdk-garbage --lib preview`
  - Expected: All preview tests pass

- [ ] **Unit tests for recovery**
  - Command: `cargo test --package agentsdk-garbage --lib recovery`
  - Expected: All recovery tests pass

- [ ] **Unit tests for scheduling**
  - Command: `cargo test --package agentsdk-garbage --lib scheduling`
  - Expected: All scheduling tests pass

### Integration Tests

- [ ] **Integration tests for GC workflow**
  - Command: `cargo test --package agentsdk-garbage --test integration_test`
  - Expected: All integration tests pass

### Recovery Tests

- [ ] **Recovery tests**
  - Command: `cargo test --package agentsdk-garbage --test recovery`
  - Expected: All recovery tests pass

---

## Final Checklist

### Implementation Complete
- [ ] GC policies implemented and tested
- [ ] Preview functional
- [ ] Recovery working
- [ ] Scheduling functional
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] GC policies configurable
- [ ] Preview shows what will be deleted
- [ ] Recovery can restore deleted data
- [ ] All GC operations track provenance

### Integration Ready
- [ ] Works with memory storage
- [ ] Works with indices
- [ ] Works with provenance

### Documentation Complete
- [ ] API documentation generated
- [ ] Policy configuration documented
- [ ] Recovery procedures documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~560 lines
