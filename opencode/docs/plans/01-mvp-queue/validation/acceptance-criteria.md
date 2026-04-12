# Acceptance Criteria

This document defines exit criteria for Phase 1 MVP Queue & Scheduler implementation.

---

## Functional Requirements

### Core Functionality
- [ ] ChatSession lifecycle works (create, activate, pause, resume, complete, fail, cancel)
- [ ] Queue state machine supports all 8 states and valid transitions
- [ ] Persistent storage survives process crashes and recovers state
- [ ] Scheduler manages worker pool and job prioritization
- [ ] Step execution works for all 4 step types (agent, tool, code, workflow)
- [ ] All 6 loop types work correctly (count, foreach, while, validation, retry, infinite)
- [ ] Branch evaluation supports all comparison operators
- [ ] Parallel execution respects concurrency limits
- [ ] Human gating classifies and gates operations correctly
- [ ] All 3 execution modes work (serial, parallel, hybrid)

### Safety & Controls
- [ ] Operation classification is accurate (safe/risky/dangerous)
- [ ] Human confirmation prompts display correctly
- [ ] Diff preview shows staged changes
- [ ] Staging works with apply and rollback
- [ ] Yes/no decisions are strict (no ambiguous inputs)
- [ ] Dangerous operations require confirmation

### Observability
- [ ] Logging framework supports 9 levels
- [ ] Scoped logging works per component
- [ ] Metrics are collected for pipeline/step/tool/custom levels
- [ ] Metrics export to JSON works
- [ ] Retry strategies work (fixed, linear, exponential)
- [ ] Error classification is accurate
- [ ] Error escalation handlers work

---

## Non-Functional Requirements

### Performance
- [ ] Scheduler can handle 100+ concurrent jobs
- [ ] Worker pool scales to configured max_workers
- [ ] Storage operations complete in <100ms for typical operations
- [ ] Job state transitions complete in <10ms

#### Performance Criteria
- [ ] Queue throughput ≥ 100 jobs/second for simple workflows
- [ ] Step execution latency ≤ 500ms for non-LLM steps
- [ ] LLM step execution latency ≤ 30s (target: 10s)
- [ ] Parallel step startup overhead ≤ 50ms
- [ ] State checkpoint write time ≤ 200ms for typical workflow
- [ ] State checkpoint restore time ≤ 500ms for typical workflow

- [ ] **Step 1a:** Benchmark queue throughput with 1000+ jobs
- [ ] **Step 1b:** Benchmark step execution latency for all step types
- [ ] **Step 1c:** Optimize if targets not met
- [ ] **Step 1d:** Document performance characteristics

### Reliability
- [ ] No data loss on process crash
- [ ] Storage recovers all running jobs on restart
- [ ] Scheduler gracefully shuts down (completes or times out running jobs)
- [ ] Error handling prevents cascading failures

#### Recovery Criteria
- [ ] Crash recovery restores all incomplete jobs with correct state
- [ ] Checkpoint restoration accuracy = 100% (no data loss or corruption)
- [ ] Journal replay completes within 2x normal execution time
- [ ] Partial state recovery handles incomplete checkpoints gracefully
- [ ] Recovery process logs all restored jobs with timestamps

- [ ] **Step 2a:** Test crash recovery with 50 concurrent jobs
- [ ] **Step 2b:** Test checkpoint restoration accuracy
- [ ] **Step 2c:** Test partial recovery scenarios
- [ ] **Step 2d:** Document recovery procedures

### Scalability
- [ ] Queue supports 10,000+ jobs
- [ ] Storage handles 100MB+ of job data
- [ ] Logging doesn't impact performance
- [ ] Metrics collection has <1% overhead

#### Scalability Criteria
- [ ] System handles 1000+ concurrent queue items without degradation
- [ ] System handles 100+ parallel steps without resource exhaustion
- [ ] Storage scales linearly with job count (no performance cliff)
- [ ] Memory usage ≤ 2GB for 1000 concurrent jobs
- [ ] CPU usage scales proportionally with concurrency
- [ ] Log rotation handles 10GB+ of logs without impact

- [ ] **Step 3a:** Load test with 1000 concurrent queue items
- [ ] **Step 3b:** Load test with 100+ parallel steps
- [ ] **Step 3c:** Monitor resource usage (memory, CPU, I/O)
- [ ] **Step 3d:** Document scalability limits and recommendations

### Usability
- [ ] CLI interface is intuitive
- [ ] Error messages are clear and actionable
- [ ] Logs provide sufficient debugging information
- [ ] Human gating prompts are easy to understand

---

## Quality Requirements

### Code Quality
- [ ] 90%+ test coverage for all modules
- [ ] All lints pass (clippy, rustfmt)
- [ ] Code follows Rust best practices
- [ ] Documentation is complete (module docs, trait docs, public API docs)

### Testing
- [ ] Unit tests for all components
- [ ] Integration tests for major workflows
- [ ] Property tests for critical invariants
- [ ] All tests pass consistently

### Documentation
- [ ] README with usage examples
- [ ] API documentation for public interfaces
- [ ] Architecture documentation
- [ ] Operator guide for deployment

---

## Integration Requirements

### Phase 0 Integration
- [ ] Uses WorkflowSpec from Phase 0
- [ ] Uses WorkflowIR from Phase 0
- [ ] Uses YAML parser from Phase 0
- [ ] Uses ./workspace/ storage from Phase 0
- [ ] Uses variable interpolation from Phase 0

#### Compatibility Criteria
- [ ] Schema backward compatibility with Phase 0 workflows
- [ ] Migration paths exist for Phase 0 data formats
- [ ] Breaking changes documented with migration guide
- [ ] Version detection prevents incompatible data loading
- [ ] Legacy format support (graceful deprecation)

- [ ] **Step 4a:** Test backward compatibility with Phase 0 workflows
- [ ] **Step 4b:** Create migration tests for data format changes
- [ ] **Step 4c:** Document breaking changes and migration paths
- [ ] **Step 4d:** Verify version detection works correctly

### Example Workflows
- [ ] At least 10 example workflows execute successfully
- [ ] All loop types tested with examples
- [ ] All step types tested with examples
- [ ] Human gating tested with examples
- [ ] Parallel execution tested with examples

---

## ADR-0002 Compliance

- [ ] ChatSession used as work container
- [ ] Human-gated safety implemented
- [ ] Staged diffs with preview
- [ ] CLI-first interface
- [ ] Confirmation prompts for risky/dangerous operations

---

## Security Requirements

#### Security Criteria
- [ ] Safe scope enforcement prevents unauthorized access
- [ ] Permission boundary validation respects all policy settings
- [ ] Tool sandboxing isolates execution context
- [ ] Input sanitization prevents injection attacks
- [ ] Rate limiting prevents resource exhaustion attacks
- [ ] Audit logging captures all security-relevant events

- [ ] **Step 5a:** Test safe scope enforcement with various permissions
- [ ] **Step 5b:** Test permission boundary validation edge cases
- [ ] **Step 5c:** Test tool sandboxing isolation
- [ ] **Step 5d:** Review and audit all security-sensitive code

---

## Observability Requirements

#### Observability Criteria
- [ ] Log format verification passes (structured JSON, correct levels)
- [ ] Metrics completeness covers all critical operations
- [ ] Distributed tracing for cross-component operations
- [ ] Log aggregation supports filtering and search
- [ ] Alerting thresholds defined for critical metrics
- [ ] Debug mode provides detailed execution traces

- [ ] **Step 6a:** Verify log format compliance with schema
- [ ] **Step 6b:** Audit metrics coverage (gaps identified)
- [ ] **Step 6c:** Test distributed tracing across components
- [ ] **Step 6d:** Define and test alerting thresholds

---

## Exit Criteria

**Phase 1 is complete when:**

1. **All functional requirements met** (core functionality, safety & controls, observability)
2. **All non-functional requirements met** (performance, reliability, scalability, usability)
3. **All quality requirements met** (code quality, testing, documentation)
4. **All integration requirements met** (Phase 0, example workflows)
5. **ADR-0002 fully compliant**
6. **All 5 checkpoints validated**
7. **All tests pass** (unit, integration, property)
8. **At least 10 example workflows execute end-to-end**
9. **No blocking bugs or known issues**
10. **Documentation is complete and accurate**
11. **Performance targets met** (throughput, latency, scalability)
12. **Security requirements satisfied** (scope enforcement, permissions, sandboxing)
13. **Observability requirements satisfied** (logs, metrics, tracing)
14. **Recovery capabilities verified** (crash recovery, checkpoint restoration)
15. **Compatibility requirements met** (backward compatibility, migration paths)

---

## Performance Benchmarks

### Required Benchmarks

#### Queue Throughput
```bash
cargo test --release -- --ignored benchmark_queue_throughput

# Expected output:
# Queue throughput: 150 jobs/sec (target: ≥100)
# Concurrent jobs: 1000
# Avg job latency: 6.67ms
```

- [ ] **Bench 1a:** Implement queue throughput benchmark
- [ ] **Bench 1b:** Run benchmark with 1000 concurrent jobs
- [ ] **Bench 1c:** Verify ≥100 jobs/sec target met
- [ ] **Bench 1d:** Document results and optimization findings

#### Step Execution Latency
```bash
cargo test --release -- --ignored benchmark_step_latency

# Expected output:
# Tool step latency: 10ms (target: ≤500)
# Code step latency: 25ms (target: ≤500)
# LLM step latency: 12.5s (target: ≤30)
# Workflow step latency: 150ms (target: ≤500)
```

- [ ] **Bench 2a:** Implement step latency benchmark for all types
- [ ] **Bench 2b:** Run benchmark with 100 steps per type
- [ ] **Bench 2c:** Verify all targets met
- [ ] **Bench 2d:** Document latency characteristics per step type

#### Parallel Execution Scalability
```bash
cargo test --release -- --ignored benchmark_parallel_scalability

# Expected output:
# 1 parallel: 100ms baseline
# 10 parallel: 15ms (6.67x speedup)
# 100 parallel: 2ms (50x speedup)
# 1000 parallel: 3ms (33x speedup, resource saturation)
```

- [ ] **Bench 3a:** Implement parallel scalability benchmark
- [ ] **Bench 3b:** Test with 1, 10, 100, 1000 parallel steps
- [ ] **Bench 3c:** Analyze speedup and resource usage
- [ ] **Bench 3d:** Document optimal concurrency range

---

## Scalability Tests

### Required Scalability Tests

#### Concurrent Queue Items
```bash
cargo test --release -- --ignored test_1000_concurrent_queue_items

# Expected output:
# 1000 concurrent jobs created successfully
# All jobs queued without error
# Queue state consistent
# Memory usage: 1.8GB (target: ≤2GB)
```

- [ ] **Scale 1a:** Create test for 1000 concurrent queue items
- [ ] **Scale 1b:** Verify queue state consistency
- [ ] **Scale 1c:** Monitor memory usage during test
- [ ] **Scale 1d:** Document memory requirements

#### Parallel Steps
```bash
cargo test --release -- --ignored test_100_parallel_steps

# Expected output:
# 100 parallel steps executed successfully
# Resource usage: 85% CPU, 1.5GB RAM
# No resource exhaustion errors
# All steps completed within 10s
```

- [ ] **Scale 2a:** Create test for 100 parallel steps
- [ ] **Scale 2b:** Monitor resource usage (CPU, RAM, I/O)
- [ ] **Scale 2c:** Verify no resource exhaustion
- [ ] **Scale 2d:** Document resource limits

---

## Sign-Off

**Implementing Engineer:** _________________ **Date:** _________________

**Code Reviewer:** _________________ **Date:** _________________

**QA Engineer:** _________________ **Date:** _________________

**Phase 1 Approved:** [ ] Yes  [ ] No

**Comments:**

__________________________________________________________________

__________________________________________________________________

__________________________________________________________________
