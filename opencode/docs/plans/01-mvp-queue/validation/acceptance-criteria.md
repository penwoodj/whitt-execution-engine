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

### Reliability
- [ ] No data loss on process crash
- [ ] Storage recovers all running jobs on restart
- [ ] Scheduler gracefully shuts down (completes or times out running jobs)
- [ ] Error handling prevents cascading failures

### Scalability
- [ ] Queue supports 10,000+ jobs
- [ ] Storage handles 100MB+ of job data
- [ ] Logging doesn't impact performance
- [ ] Metrics collection has <1% overhead

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
- [ ] Uses .glyphnova/ storage from Phase 0
- [ ] Uses variable interpolation from Phase 0

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
