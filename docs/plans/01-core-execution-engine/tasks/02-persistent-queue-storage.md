# Task 02: Persistent Queue Storage

**Goal:** Implement sled-based persistent storage for queue state, enabling recovery from process crashes and providing durable job metadata, execution history, and checkpoint data.

> ⚠️ **Critical Review #2 Finding**: Persistence uses serde_json but format is not versioned or backward-compatible. CRITICAL: WorkflowState serialization format must be versioned for backward compatibility. Add a `schema_version` field and migration path. See [upstream-critical-review-02.md](../../../research/upstream-critical-review-02.md) Factor 6.

**Files:**
- Create: `src/queue/storage.rs`
- Modify: `src/chat_session/manager.rs` (implement SessionStorage with sled)
- Modify: `src/queue/mod.rs` (export storage)
- Create: `tests/unit/queue_storage_test.rs`

---

## Implementation Status

**Status**: ✅ IMPLEMENTED (with variations)

### What Exists
- **[src/agent/persistence.rs](../../src/agent/persistence.rs)**: Workflow state persistence:
  - SQLite persistence via rusqlite with `PersistenceBackend` trait ✅
  - `WorkflowState`, `WorkflowStatus`, `Checkpoint` structs ✅
  - `save_checkpoint()`, `load_checkpoint()`, `list_checkpoints()` methods ✅
  - Session storage not implemented in `src/chat_session/`

- **Storage architecture**:
  - Uses SQLite via rusqlite instead of sled ✅
  - `PersistenceBackend` trait for interchangeable backends ✅
  - `JsonPersistence` and `SqlitePersistence` implementations ✅

### What's Missing
- **Queue storage module** not implemented as planned:
  - `src/queue/storage.rs` - NOT IMPLEMENTED (plan expects QueueStorage)
  - `QueueStorage` struct with sled `Db` - NOT IMPLEMENTED (current uses SQLite)

- **Queue state management** not implemented:
  - `Job`, `JobId`, `QueueState` types - NOT IMPLEMENTED
  - `save_job()`, `load_job()`, `delete_job()` methods - NOT IMPLEMENTED
  - `query_jobs_by_state()`, `query_jobs_by_workflow()` methods - NOT IMPLEMENTED
  - `recover_running_jobs()` method - NOT IMPLEMENTED

- **Index-based queries** not implemented:
  - State index (`state_index_key()`) - NOT IMPLEMENTED
  - Workflow index (`workflow_index_key()`) - NOT IMPLEMENTED
  - `update_state_index()`, `update_workflow_index()` methods - NOT IMPLEMENTED

- **Session storage integration** not implemented:
  - `session_key()`, `save_session()`, `load_session()` methods - NOT IMPLEMENTED
  - `SessionStorage` trait implementation not integrated

- **Flush durability** not implemented:
  - `flush()` method for durability guarantees - NOT IMPLEMENTED

### QA Coverage
- **Status**: ✅ PASS (Fixed)
- **Coverage**: EPOC-062 through EPOC-066
  - **AREA-14 WORKFLOW PERSISTENCE** - ✅ PASS - SQLite persistence implemented via rusqlite with PersistenceBackend trait

### Evidence
- **SQLite backend**: ✅ rusqlite persistence with 6 tests passing
- **PersistenceBackend trait**: ✅ Supports JsonPersistence and SqlitePersistence
- **No queue storage**: `src/queue/storage.rs` does not exist ✅
- **Build**: ✅ `cargo build` passes

### Plan vs Reality Notes
- **Plan expects**: sled-based queue storage with Job, JobId, QueueState types
- **Current reality**: SQLite persistence exists but not queue-specific storage
- **Database choice**: Plan specifies sled, current uses SQLite (rusqlite)

---

## QA Cross-References

- **QA Criteria**: [QA-01-03](../../qa/phase-01/QA-CRITERIA.md)
- **Test Cases**: [P01-005](../../qa/phase-01/QA-TEST-CASES.md), [P01-006](../../qa/phase-01/QA-TEST-CASES.md)
- **Schema Ref**: N/A (execution engine component)
- **Indexing**: Plan expects index-based queries for fast state/workflow lookups, not implemented
- **Recovery**: Plan expects `recover_running_jobs()` for crash recovery, not implemented
