# Quality Loops Validation Criteria

## ADR-0005 Compliance (CRITICAL)

### Runtime Semantics

- [ ] RepairLoopConfig is NOT optional - part of ExecutionEngineConfig
- [ ] Quality loops run automatically, not as optional prompt pattern
- [ ] Convergence detection is MANDATORY (must terminate)
- [ ] Verifier failures AUTOMATICALLY trigger repair step
- [ ] Quality thresholds ENFORCED at runtime, not suggested
- [ ] Loop state machine transitions are deterministic and testable
- [ ] No manual intervention required during loop execution
- [ ] Quality thresholds applied before convergence checks
- [ ] Loop failures bubble up as structured errors, not panics

### Configuration Validation

- [ ] max_iterations has default (10) and cannot be zero
- [ ] iteration_timeout has default (300s) and cannot be zero
- [ ] ConvergenceCriteria.min_confidence in (0.0, 1.0]
- [ ] ConvergenceCriteria.max_consecutive_failures >= 1
- [ ] QualityThresholds.min_pass_rate in [0.0, 1.0]
- [ ] QualityThresholds.max_errors >= 0
- [ ] QualityThresholds.max_warnings >= 0
- [ ] RepairStrategy defaults to Aggressive if not specified
- [ ] BackoffStrategy defaults to Exponential if not specified
- [ ] Configuration parsing rejects invalid combinations (e.g., min_confidence > 1.0)
- [ ] Configuration validation emits specific error messages for each constraint violation
- [ ] Zero or negative values for time-based thresholds are rejected
- [ ] Negative iteration counts are rejected

### Convergence Validation

#### MUST Terminate

- [ ] Loop terminates when max_iterations reached
- [ ] Loop terminates when iteration_timeout exceeded
- [ ] Loop terminates when convergence criteria met
- [ ] Loop terminates when max_consecutive_failures reached
- [ ] Loop terminates when quality plateau detected (if enabled)
- [ ] No infinite loops possible
- [ ] Loop timeout accounts for individual step timeouts
- [ ] Timeout detection uses wall-clock time, not CPU time
- [ ] Convergence state is persisted before termination
- [ ] Termination reason is recorded in LoopExecutionState

#### Convergence Detection

- [ ] check_convergence() returns Success, Failure, or Continue
- [ ] Quality thresholds checked before convergence criteria
- [ ] ConvergenceCriteria.min_confidence enforced
- [ ] ConvergenceCriteria.max_consecutive_failures enforced
- [ ] Quality plateau detection uses epsilon threshold
- [ ] Plateau detection uses configurable window size
- [ ] Convergence detection handles missing quality scores gracefully
- [ ] Convergence state is monotonic (no false negatives)
- [ ] Early termination on impossible-to-meet criteria

### State Tracking

- [ ] LoopExecutionState tracks all iterations
- [ ] Each iteration records artifact, verification, repair
- [ ] State includes timestamps for debugging
- [ ] State persists (if persist_state enabled)
- [ ] State can be reconstructed for debugging
- [ ] consecutive_failures accurately tracked
- [ ] State serialization uses serde for interoperability
- [ ] State file includes version field for migration
- [ ] State can be resumed from last iteration
- [ ] State corruption detection via checksums
- [ ] State includes full artifact snapshots for each iteration

## Loop Configuration Parsing

### Configuration Structure

- [ ] RepairLoopConfig deserializes from YAML correctly
- [ ] ConvergenceCriteria parses all fields
- [ ] QualityThresholds parses all fields
- [ ] RepairStrategy enum parses correctly (Aggressive/Conservative/Exhaustive)
- [ ] BackoffStrategy enum parses correctly (None/Linear/Exponential)
- [ ] Nested structures parsed with proper error messages
- [ ] Unknown fields in configuration are rejected or warned
- [ ] Configuration validates cross-field constraints

### Strategy Selection

- [ ] RepairStrategy::Aggressive uses max_retries=3, backoff=Exponential
- [ ] RepairStrategy::Conservative uses max_retries=5, backoff=Linear
- [ ] RepairStrategy::Exhaustive uses max_retries=10, backoff=Exponential(2.0)
- [ ] Strategy selection respects configuration overrides
- [ ] Strategy metadata includes iteration cost estimates
- [ ] Strategy can be changed mid-loop (with warning)
- [ ] Strategy selection logged at loop start

## Step Validation

### Generate Step

- [ ] generate_artifact() uses LLM backend
- [ ] Returns GenerateResult with artifact and metadata
- [ ] Metadata includes duration_ms, tokens_used, model
- [ ] Handles LLM errors gracefully
- [ ] Generates valid artifact (non-empty content)
- [ ] Generate step timeout enforced
- [ ] Generate step includes system prompt in metadata
- [ ] Generate step logs model selection reasoning

### Verify Step

- [ ] verify_artifact() uses VerifierRegistry
- [ ] Runs all applicable verifiers
- [ ] Returns VerifyResult with individual results
- [ ] Combines results using AND logic
- [ ] Handles NoVerifiers error
- [ ] Returns aggregated metrics
- [ ] Verifier execution is parallel when possible
- [ ] Verify step timeout enforced per verifier
- [ ] Verify step logs each verifier result
- [ ] Verify step includes verifier version in metadata

### Repair Step

- [ ] repair_artifact() constructs repair prompt from failures
- [ ] repair_prompt includes original artifact and errors
- [ ] Uses LLM backend for repair
- [ ] Respects max_attempts
- [ ] Returns RepairResult with success/failure
- [ ] Tracks number of attempts
- [ ] Repair step includes context from failed verification
- [ ] Repair step logs number of fix attempts
- [ ] Repair step includes diff between original and repaired artifact

## Backoff Strategy

- [ ] BackoffStrategy::None uses constant delay
- [ ] BackoffStrategy::Linear increases by increment each iteration
- [ ] BackoffStrategy::Exponential multiplies by multiplier
- [ ] Exponential backoff respects max_delay
- [ ] delay_for_iteration() returns Duration >= 1ms
- [ ] Backoff jitter applied (randomization) to avoid thundering herd
- [ ] Backoff logged before each retry
- [ ] Backoff can be overridden per-iteration via callback

## Scheduler and Queue Integration

### Job Scheduling

- [ ] Quality loops submitted as jobs to scheduler
- [ ] Job priority based on loop configuration
- [ ] Job metadata includes loop type and strategy
- [ ] Scheduler can cancel in-progress loops
- [ ] Scheduler can pause and resume loops
- [ ] Loop state checkpointed on pause
- [ ] Queue supports loop job types
- [ ] Queue priorities: quality_loop > benchmark > report

### Job Execution

- [ ] Scheduler provides ExecutionContext to loop
- [ ] Loop updates job progress during execution
- [ ] Loop emits job events (start, iteration, converge, fail)
- [ ] Loop failure recorded as job failure
- [ ] Loop success recorded as job success
- [ ] Job history includes all loop iterations
- [ ] Job cancellation propagates to loop immediately

## Logging and Observability

### Log Output

- [ ] Loop start logged with full configuration
- [ ] Each iteration logged with iteration number
- [ ] Generate step logged with artifact summary
- [ ] Verify step logged with individual verifier results
- [ ] Repair step logged with repair attempts
- [ ] Convergence logged with confidence score
- [ ] Loop termination logged with reason
- [ ] Metrics logged at loop completion
- [ ] Logs use structured JSON format
- [ ] Log level configurable per loop

### Log Format

- [ ] Loop logs include: loop_id, iteration, timestamp, step, status
- [ ] Generate logs: artifact_size, tokens, duration_ms, model
- [ ] Verify logs: verifier_id, result, duration_ms, message
- [ ] Repair logs: attempt, success, diff_summary
- [ ] Convergence logs: confidence, quality_score, reason
- [ ] Termination logs: reason, total_iterations, total_duration_ms
- [ ] Error logs: error_type, message, backtrace (if available)

### Observability

- [ ] Metrics emitted: loop_duration, iterations, convergence_rate
- [ ] Metrics emitted: quality_score, errors, warnings
- [ ] Metrics emitted: token_usage, cost_estimate
- [ ] Metrics tagged with: loop_id, strategy, backend
- [ ] OpenTelemetry tracing spans for each step
- [ ] Tracing includes parent span for scheduler job
- [ ] Distributed tracing context propagated to LLM calls

## Mock Strategies

### Mock LLM for Generate Step

```rust
struct MockLLM {
    response: String,
    tokens: usize,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: generate_artifact with mock LLM
#[test]
fn test_generate_with_mock_llm() {
    let mock = MockLLM {
        response: "generated artifact".to_string(),
        tokens: 50,
        latency_ms: 100,
        fail_after: None,
    };

    let result = generate_artifact(&mock, &Prompt { .. });
    assert!(result.is_ok());
    assert_eq!(result.unwrap().artifact, "generated artifact");
}
```

### Mock Verifier for Verify Step

```rust
struct MockVerifier {
    result: VerifyResult,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: verify_artifact with mock verifier
#[test]
fn test_verify_with_mock_verifier() {
    let mock = MockVerifier {
        result: VerifyResult::Pass { score: 0.95 },
        latency_ms: 50,
        fail_after: None,
    };

    let result = verify_artifact(&mock, &Artifact { .. });
    assert!(result.is_ok());
    assert_eq!(result.unwrap().score, 0.95);
}
```

### Mock Repair Engine

```rust
struct MockRepairEngine {
    success_rate: f64,
    latency_ms: u64,
}

// Test: repair_artifact with mock repair engine
#[test]
fn test_repair_with_mock_engine() {
    let mock = MockRepairEngine {
        success_rate: 1.0,
        latency_ms: 200,
    };

    let result = repair_artifact(&mock, &Artifact { .. }, &Vec::new());
    assert!(result.is_ok());
}
```

## Cargo Test Commands

### Unit Tests

```bash
# Run all quality loops unit tests
cargo test --lib quality_loops::tests::unit -- --nocapture

# Test configuration validation
cargo test --lib quality_loops::config::tests::validate_config -- --exact

# Test convergence detection
cargo test --lib quality_loops::convergence::tests::check_convergence -- --exact

# Test backoff strategies
cargo test --lib quality_loops::backoff::tests::delay_for_iteration -- --exact

# Test state tracking
cargo test --lib quality_loops::state::tests::loop_execution_state -- --exact

# Test strategy selection
cargo test --lib quality_loops::strategy::tests::select_strategy -- --exact

# Expected output:
# test quality_loops::config::tests::validate_config ... ok
# test quality_loops::convergence::tests::check_convergence ... ok
# test quality_loops::backoff::tests::delay_for_iteration ... ok
# test quality_loops::state::tests::loop_execution_state ... ok
# test quality_loops::strategy::tests::select_strategy ... ok
```

### Integration Tests

```bash
# Run full quality loop integration test
cargo test --test quality_loops_integration test_full_loop -- --exact --nocapture

# Test loop convergence
cargo test --test quality_loops_integration test_loop_converges -- --exact

# Test loop failure scenarios
cargo test --test quality_loops_integration test_loop_fails_max_iterations -- --exact

# Test loop timeout
cargo test --test quality_loops_integration test_loop_times_out -- --exact

# Expected output:
# test quality_loops_integration::test_full_loop ... ok
# test quality_loops_integration::test_loop_converges ... ok
# test quality_loops_integration::test_loop_fails_max_iterations ... ok
# test quality_loops_integration::test_loop_times_out ... ok
```

### Scheduler Integration Tests

```bash
# Test quality loop as scheduler job
cargo test --test scheduler_integration test_quality_loop_job -- --exact --nocapture

# Test loop cancellation
cargo test --test scheduler_integration test_loop_cancellation -- --exact

# Test loop pause and resume
cargo test --test scheduler_integration test_loop_pause_resume -- --exact

# Expected output:
# test scheduler_integration::test_quality_loop_job ... ok
# test scheduler_integration::test_loop_cancellation ... ok
# test scheduler_integration::test_loop_pause_resume ... ok
```

### Mock Strategy Tests

```bash
# Test with mock LLM
cargo test --test mock_strategies test_generate_mock_llm -- --exact

# Test with mock verifier
cargo test --test mock_strategies test_verify_mock_verifier -- --exact

# Test with mock repair engine
cargo test --test mock_strategies test_repair_mock_engine -- --exact

# Expected output:
# test mock_strategies::test_generate_mock_llm ... ok
# test mock_strategies::test_verify_mock_verifier ... ok
# test mock_strategies::test_repair_mock_engine ... ok
```

### Log Verification Tests

```bash
# Test log output format
cargo test --test logging test_loop_log_format -- --exact --nocapture

# Test structured log parsing
cargo test --test logging test_parse_loop_logs -- --exact

# Test metrics emission
cargo test --test logging test_loop_metrics -- --exact

# Expected output:
# test logging::test_loop_log_format ... ok
# test logging::test_parse_loop_logs ... ok
# test logging::test_loop_metrics ... ok
```

## Log Verification Patterns

### Loop Start Log Pattern

```json
{
  "level": "info",
  "target": "quality_loops::loop",
  "loop_id": "<uuid>",
  "event": "loop_start",
  "config": {
    "max_iterations": 10,
    "iteration_timeout": 300000,
    "strategy": "Aggressive"
  }
}
```

**Verification**: `grep '"event":"loop_start"' logs/quality_loop.log | jq '.config.max_iterations == 10'`

### Iteration Log Pattern

```json
{
  "level": "debug",
  "target": "quality_loops::loop",
  "loop_id": "<uuid>",
  "iteration": 1,
  "event": "iteration_start"
}
```

**Verification**: `grep '"event":"iteration_start"' logs/quality_loop.log | jq '.iteration | tonumber' | sort -n | uniq`

### Generate Step Log Pattern

```json
{
  "level": "debug",
  "target": "quality_loops::generate",
  "loop_id": "<uuid>",
  "iteration": 1,
  "event": "generate_complete",
  "artifact_size": 1024,
  "tokens": 500,
  "duration_ms": 1200,
  "model": "llama-3.2-3b-instruct"
}
```

**Verification**: `grep '"event":"generate_complete"' logs/quality_loop.log | jq '.duration_ms | tonumber < 5000' | grep true`

### Verify Step Log Pattern

```json
{
  "level": "debug",
  "target": "quality_loops::verify",
  "loop_id": "<uuid>",
  "iteration": 1,
  "event": "verify_complete",
  "verifier_results": [
    {"verifier_id": "syntax_check", "result": "Pass", "duration_ms": 50},
    {"verifier_id": "quality_check", "result": "Fail", "duration_ms": 100}
  ]
}
```

**Verification**: `grep '"event":"verify_complete"' logs/quality_loop.log | jq '.verifier_results | length > 0' | grep true`

### Repair Step Log Pattern

```json
{
  "level": "debug",
  "target": "quality_loops::repair",
  "loop_id": "<uuid>",
  "iteration": 1,
  "event": "repair_complete",
  "attempt": 1,
  "success": true,
  "diff_summary": "2 lines changed"
}
```

**Verification**: `grep '"event":"repair_complete"' logs/quality_loop.log | jq '.success == true' | grep true`

### Convergence Log Pattern

```json
{
  "level": "info",
  "target": "quality_loops::convergence",
  "loop_id": "<uuid>",
  "iteration": 3,
  "event": "converged",
  "confidence": 0.95,
  "quality_score": 0.92,
  "reason": "min_confidence_met"
}
```

**Verification**: `grep '"event":"converged"' logs/quality_loop.log | jq '.confidence | tonumber >= 0.9' | grep true`

### Termination Log Pattern

```json
{
  "level": "info",
  "target": "quality_loops::loop",
  "loop_id": "<uuid>",
  "event": "loop_terminated",
  "reason": "converged",
  "total_iterations": 3,
  "total_duration_ms": 4500
}
```

**Verification**: `grep '"event":"loop_terminated"' logs/quality_loop.log | jq '.total_iterations | tonumber <= 10' | grep true`

## Testing Criteria

### Unit Tests

- [ ] RepairLoopConfig default values
- [ ] BackoffStrategy.delay_for_iteration() correct
- [ ] LoopExecutionState creation and transitions
- [ ] consecutive_failures tracking
- [ ] is_max_iterations_reached() accurate
- [ ] is_timeout() accurate
- [ ] check_convergence() with high confidence
- [ ] check_convergence() with low confidence
- [ ] check_convergence() with too many errors
- [ ] has_quality_plateau() detection
- [ ] generate_artifact() with mock LLM
- [ ] verify_artifact() with mock verifier
- [ ] repair_artifact() with mock repair engine
- [ ] strategy selection logic
- [ ] configuration parsing validation

### Integration Tests

- [ ] Full quality loop execution
- [ ] Loop converges successfully
- [ ] Loop fails after max_iterations
- [ ] Loop times out
- [ ] Loop detects quality plateau
- [ ] Repair step fixes failures
- [ ] State persisted across iterations
- [ ] Backoff delays applied correctly
- [ ] Loop as scheduler job
- [ ] Loop cancellation mid-execution
- [ ] Loop pause and resume
- [ ] Scheduler job priority handling
- [ ] Queue integration for loop jobs

### Edge Cases

- [ ] Empty verifier registry
- [ ] All verifiers fail
- [ ] Repair never succeeds
- [ ] LLM backend errors
- [ ] Zero max_iterations (should error)
- [ ] Zero iteration_timeout (should error)
- [ ] Convergence criteria impossible to meet
- [ ] State file corruption
- [ ] State version mismatch
- [ ] Scheduler job cancellation during iteration
- [ ] Network timeout during LLM call
- [ ] Verifier panic handling

## Performance Criteria

- [ ] Single iteration completes < iteration_timeout
- [ ] Convergence check < 1ms
- [ ] State serialization < 10ms
- [ ] Backoff calculation < 1ms
- [ ] Memory usage bounded (< 500MB for typical loop)
- [ ] Log emission < 5ms per entry
- [ ] Metrics emission < 1ms per metric
- [ ] State file size < 10MB for 100 iterations
- [ ] Loop startup < 100ms
- [ ] Loop shutdown < 50ms

## Error Handling

- [ ] LoopError covers all failure modes
- [ ] GenerateError, VerifyError, RepairError specific
- [ ] MaxIterationsExceeded includes iteration count
- [ ] Timeout includes elapsed time
- [ ] Error messages actionable
- [ ] State includes error context
- [ ] Errors propagate to scheduler job
- [ ] Errors logged with stack traces (if available)
- [ ] Recovery strategies documented
- [ ] Error metrics emitted (error_type, count)

## ADR-0005 Constraint Compliance Summary

| Constraint | Validation | Test Coverage |
|------------|-------------|---------------|
| RepairLoopConfig required | Config parsing validates | config::tests::validate_config |
| Automatic execution | Loop runs without manual trigger | integration::test_full_loop |
| Mandatory convergence | check_convergence always returns | convergence::tests::check_convergence |
| Auto-repair on failure | Verify failure triggers repair | integration::test_repair_on_failure |
| Enforced quality thresholds | Threshold checked before convergence | state::tests::quality_thresholds |
| Deterministic execution | State machine transitions testable | state::tests::state_transitions |
| No infinite loops | All termination conditions tested | integration::test_termination |
