# Checkpoint Criteria

This document defines gate criteria for each checkpoint in the Phase 1 implementation.

---

## Checkpoint 1: Storage Layer Complete (After Task 02)

### Criteria

**ChatSession Storage**
- [ ] ChatSession can be created, loaded, and deleted
- [ ] Session state transitions work correctly
- [ ] Session metadata is persisted accurately
- [ ] Human interactions are stored and retrieved
- [ ] Checkpoints can be added and retrieved

**Queue Storage**
- [ ] Jobs can be saved, loaded, and deleted
- [ ] State queries (by state, by workflow) work correctly
- [ ] Job indices are updated on state changes
- [ ] Recovery of running jobs works after simulated crash

**Verification Tests**
```bash
# Run all unit tests for chat_session and queue modules
cargo test chat_session --lib
cargo test queue_storage --lib

# Expected: All tests pass
```

### Acceptance

All criteria met and all tests pass. Storage is ready for scheduler integration.

---

## Checkpoint 2: Scheduler Core Complete (After Task 03)

### Criteria

**Scheduler Functionality**
- [ ] Scheduler can submit jobs to queue
- [ ] Workers are spawned and configured correctly
- [ ] Job prioritization works (Priority enum ordering)
- [ ] Job execution loop runs continuously
- [ ] Commands are processed (submit, cancel, pause, resume, status)

**Integration with Storage**
- [ ] Jobs transition through states correctly
- [ ] Running jobs are tracked in running_jobs map
- [ ] Pending queue holds jobs awaiting execution
- [ ] State changes are persisted to storage

**Cancellation**
- [ ] Jobs can be cancelled from any state
- [ ] Workers receive cancellation requests
- [ ] Cancelled jobs transition to Cancelled state

**Recovery**
- [ ] Running jobs are recovered on startup
- [ ] Recovered jobs reset to Pending state
- [ ] Recovery logs are emitted

**Verification Tests**
```bash
# Run scheduler unit tests
cargo test scheduler --lib

# Run integration test for basic job execution
cargo test --test integration queue_scheduler_pipeline

# Expected: All tests pass
```

### Acceptance

All criteria met and all tests pass. Scheduler can manage job lifecycle.

---

## Checkpoint 3: Execution Engine Complete (After Task 07)

### Criteria

**Step Executor**
- [ ] Agent steps can be executed (with mock LLM)
- [ ] Tool steps execute echo, read_file, write_file
- [ ] Code steps support python and bash
- [ ] Sub-workflow steps are dispatched correctly

**Loop Runner**
- [ ] Count loop runs specified number of iterations
- [ ] Foreach loop iterates over items
- [ ] While loop evaluates condition correctly
- [ ] Validation loop checks output and terminates on success
- [ ] Retry loop retries on failure with backoff
- [ ] Infinite loop runs with max iteration safety

**Branch Evaluator**
- [ ] Equality and inequality operators work
- [ ] Comparison operators (gt, lt, gte, lte) work
- [ ] Contains operator works for strings and arrays
- [ ] In operator works for membership
- [ ] Yes/no decision enforcement is strict

**Parallel Executor**
- [ ] Parallel groups execute concurrently
- [ ] Concurrency limits are respected
- [ ] Fail-fast stops group on failure
- [ ] Semaphore prevents over-subscription

**Verification Tests**
```bash
# Run all execution engine tests
cargo test executor --lib
cargo test loop_runner --lib
cargo test branch_evaluator --lib
cargo test parallel_executor --lib

# Run integration test for step execution
cargo test --test integration executor_pipeline

# Expected: All tests pass
```

### Acceptance

All criteria met and all tests pass. Execution engine can run workflow steps.

---

## Checkpoint 4: Safety & Controls Complete (After Task 09)

### Criteria

**Human Gating**
- [ ] Operations are classified (safe/risky/dangerous)
- [ ] Classification rules match expected patterns
- [ ] Dangerous path detection works
- [ ] Confirmation prompts display correctly
- [ ] Diff preview shows changes
- [ ] Yes/no decision is strict

**Execution Modes**
- [ ] Serial mode executes steps sequentially
- [ ] Parallel mode executes independent steps concurrently
- [ ] Hybrid mode analyzes dependencies
- [ ] All modes handle errors appropriately

**Integration**
- [ ] Gating is invoked before risky operations
- [ ] Diff preview is accurate
- [ ] Staging works (apply and rollback)
- [ ] Execution mode selection works

**Verification Tests**
```bash
# Run safety tests
cargo test human_gating --lib

# Run execution mode tests
cargo test execution_mode --lib

# Run integration test for human gating
cargo test --test integration human_gating_pipeline

# Expected: All tests pass
```

### Acceptance

All criteria met and all tests pass. Safety controls are enforced.

---

## Checkpoint 5: Full Phase 1 Complete (After Task 12)

### Criteria

**Observability**
- [ ] Logging framework initializes with configuration
- [ ] 9 log levels work correctly
- [ ] Scoped logging works per component
- [ ] Metrics are collected for pipelines/steps/tools
- [ ] Metrics export to JSON works

**Retry & Error Handling**
- [ ] Retry strategies work (fixed, linear, exponential)
- [ ] Error classification is accurate
- [ ] Transient errors are retried
- [ ] Permanent errors fail immediately
- [ ] User errors are not retried
- [ ] Error escalation handlers work

**End-to-End**
- [ ] Workflows can be submitted and executed
- [ ] Human gates work correctly
- [ ] Loops, branches, and parallel groups execute
- [ ] Metrics and logs are collected
- [ ] Jobs can be paused, resumed, cancelled

**Recovery**
- [ ] Process crashes are recovered
- [ ] Running jobs resume execution
- [ ] Storage remains consistent after crashes

**Verification Tests**
```bash
# Run all observability tests
cargo test logging --lib
cargo test metrics --lib
cargo test retry --lib

# Run end-to-end integration tests
cargo test --test integration e2e_workflow_execution

# Run example workflows (at least 10)
cargo test --test e2e_example_workflows

# Expected: All tests pass, at least 10 example workflows succeed
```

### Acceptance

All criteria met, all tests pass, at least 10 example workflows execute end-to-end.

---

## Summary

5 checkpoints cover:
- **Checkpoint 1**: Storage foundation (Tasks 00-02)
- **Checkpoint 2**: Scheduler core (Task 03)
- **Checkpoint 3**: Execution engine (Tasks 04-07)
- **Checkpoint 4**: Safety & controls (Tasks 08-09)
- **Checkpoint 5**: Observability & E2E (Tasks 10-12)

Each checkpoint requires:
- All functionality criteria met
- All unit tests passing
- All integration tests passing
- Git tag created for milestone

Proceed to execution when checkpoint is approved.
