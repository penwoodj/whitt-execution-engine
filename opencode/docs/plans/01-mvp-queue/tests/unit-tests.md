# Unit Tests Specification

This document defines unit test requirements for each component in Phase 1.

---

## Test Philosophy

- **Test-first approach**: Write failing tests before implementation
- **Isolation**: Each test is independent and can run in any order
- **Coverage**: Test all code paths, including error cases
- **Clarity**: Test names describe what they test
- **Speed**: Unit tests complete in <1 second total

---

## Component Test Specifications

### ChatSession (Task 00)

**Tests Required:**
- `test_session_id_generation`: UUID generation works
- `test_session_id_from_string`: Parsing from string works
- `test_state_transitions`: All valid transitions succeed
- `test_invalid_state_transitions`: Invalid transitions fail
- `test_create_session`: Session created with correct defaults
- `test_activate_session`: Created -> Active transition
- `test_pause_session`: Active -> Paused transition
- `test_resume_session`: Paused -> Active transition
- `test_complete_session`: Active -> Completed transition with outputs
- `test_fail_session`: Active -> Failed transition with error
- `test_cancel_session`: Any active state -> Cancelled
- `test_add_interaction`: Interaction added to session
- `test_add_checkpoint`: Checkpoint added to session
- `test_add_log_entry`: Log entry added to session
- `test_load_nonexistent_session`: Returns error
- `test_delete_session`: Session removed from storage

**Coverage Target:** 95%+

---

### Queue State Machine (Task 01)

**Tests Required:**
- `test_state_transitions_all_valid`: All defined transitions are valid
- `test_state_transitions_invalid`: Invalid transitions fail
- `test_terminal_states`: Completed/Failed/Cancelled/Timeout are terminal
- `test_active_states`: Pending/Scheduled/Running/Paused are active
- `test_job_creation`: Job created with correct state
- `test_job_transition_scheduled`: Pending -> Scheduled works
- `test_job_transition_running`: Scheduled -> Running works
- `test_job_transition_completed`: Running -> Completed works
- `test_job_transition_failed`: Running -> Failed works
- `test_job_transition_cancelled`: Cancel from any state
- `test_job_transition_timeout`: Timeout works
- `test_job_transition_invalid`: Invalid transition returns error
- `test_job_stats_updated`: Stats updated on transitions
- `test_job_worker_id_set`: Worker ID set on schedule
- `test_job_step_count_incremented`: Step count increments
- `test_job_retry_count_incremented`: Retry count increments
- `test_job_timeout_check`: Timeout detection works
- `test_job_retry_check`: Retry limit check works

**Coverage Target:** 95%+

---

### Queue Storage (Task 02)

**Tests Required:**
- `test_in_memory_storage`: In-memory DB works
- `test_job_save_and_load`: Job persisted and loaded
- `test_job_save_overwrites`: Save overwrites existing job
- `test_job_delete`: Job removed from storage
- `test_job_not_found`: Loading nonexistent job returns None
- `test_job_state_index`: State index works
- `test_job_workflow_index`: Workflow index works
- `test_job_list_all`: All jobs listed
- `test_job_query_by_state`: Filter by state works
- `test_job_query_by_workflow`: Filter by workflow works
- `test_job_count`: Count is accurate
- `test_recovery_running_jobs`: Running jobs recovered
- `test_session_save_and_load`: Session persisted and loaded
- `test_session_delete`: Session removed from storage
- `test_session_list_all`: All sessions listed
- `test_flush`: Flush writes to disk

**Coverage Target:** 90%+

---

### Scheduler Core (Task 03)

**Tests Required:**
- `test_scheduler_creation`: Scheduler initializes correctly
- `test_scheduler_submit_job`: Job submitted to queue
- `test_scheduler_cancel_job`: Job cancelled from queue
- `test_scheduler_pause_job`: Job paused during execution
- `test_scheduler_resume_job`: Paused job resumed
- `test_scheduler_get_status`: Job status retrieved
- `test_worker_pool_creation`: Workers spawned correctly
- `test_job_prioritization`: Critical > High > Normal > Low
- `test_recovery_running_jobs`: Running jobs recovered on startup
- `test_shutdown`: Scheduler shuts down gracefully

**Coverage Target:** 85%+ (some integration behavior tested separately)

---

### Step Executor (Task 04)

**Tests Required:**
- `test_tool_executor_echo`: Echo tool returns input
- `test_tool_executor_read_file`: File read works
- `test_tool_executor_write_file`: File written correctly
- `test_tool_executor_invalid_tool`: Unknown tool returns error
- `test_agent_executor_mock_llm`: Mock LLM returns response
- `test_code_executor_python`: Python code executed
- `test_code_executor_bash`: Bash command executed
- `test_code_executor_unsupported_language`: Error on unsupported language
- `test_workflow_executor_sub_workflow`: Sub-workflow dispatched
- `test_context_variables`: Variables retrieved correctly
- `test_context_outputs`: Outputs captured correctly

**Coverage Target:** 90%+

---

### Loop Runner (Task 05)

**Tests Required:**
- `test_count_loop`: Count loop runs N times
- `test_foreach_loop`: Foreach iterates over items
- `test_while_loop_condition`: While loop checks condition
- `test_while_loop_termination`: While loop terminates when false
- `test_validation_loop_success`: Terminates on valid output
- `test_validation_loop_retry`: Retries on invalid output
- `test_retry_loop_success`: Stops on success
- `test_retry_loop_max_retries`: Fails after max retries
- `test_retry_loop_backoff`: Backoff applied between retries
- `test_infinite_loop_with_limit`: Infinite loop stopped at max iterations
- `test_loop_variables`: Loop variables set correctly

**Coverage Target:** 90%+

---

### Branch Evaluator (Task 06)

**Tests Required:**
- `test_operator_eq`: Equality works
- `test_operator_ne`: Inequality works
- `test_operator_gt`: Greater than works
- `test_operator_lt`: Less than works
- `test_operator_gte`: Greater or equal works
- `test_operator_lte`: Less or equal works
- `test_operator_contains_string`: String contains works
- `test_operator_contains_array`: Array contains works
- `test_operator_in`: Membership works
- `test_operator_is_null`: Null check works
- `test_operator_is_not_null`: Not null check works
- `test_branch_all_conditions`: All conditions must be true
- `test_branch_first_match`: First matching branch wins
- `test_branch_no_match`: Error if no branch matches
- `test_event_routing`: Events routed to correct branch
- `test_yes_no_strict`: Only yes/no accepted
- `test_yes_no_case_insensitive`: Yes/yes/Y/Y all work

**Coverage Target:** 95%+

---

### Parallel Executor (Task 07)

**Tests Required:**
- `test_parallel_empty_group`: Error on empty group
- `test_parallel_single_step`: Single step works
- `test_parallel_multiple_steps`: Steps execute concurrently
- `test_parallel_concurrency_limit`: Limit respected
- `test_parallel_invalid_concurrency`: Error if limit > steps
- `test_parallel_fail_fast`: Group stops on failure
- `test_parallel_fail_fast_false`: Continues on failure
- `test_parallel_result_collection`: All outputs collected
- `test_parallel_completion_count`: Completion count accurate
- `test_parallel_failed_count`: Failed count accurate
- `test_parallel_cancelled`: Cancel flag set on fail_fast

**Coverage Target:** 90%+

---

### Human Gating (Task 08)

**Tests Required:**
- `test_classify_safe_operations`: Safe ops classified correctly
- `test_classify_risky_operations`: Risky ops classified correctly
- `test_classify_dangerous_operations`: Dangerous ops classified correctly
- `test_classify_unknown_operations`: Unknown ops default to risky
- `test_dangerous_path_detection`: Dangerous paths detected
- `test_safe_paths`: Safe paths pass
- `test_confirmation_prompt_display`: Prompt displays correctly
- `test_confirmation_prompt_risky`: Risky prompts show warning
- `test_confirmation_prompt_dangerous`: Dangerous prompts show alert
- `test_diff_preview_creation`: Diff created from changes
- `test_diff_display`: Diff displays correctly
- `test_stage_apply`: Apply works
- `test_stage_rollback`: Rollback works
- `test_yes_no_validation`: Invalid input rejected
- `test_yes_approved`: Yes returns true
- `test_no_rejected`: No returns false

**Coverage Target:** 90%+

---

### Execution Modes (Task 09)

**Tests Required:**
- `test_serial_execution`: Steps execute in order
- `test_serial_stops_on_failure`: Stops on first failure
- `test_parallel_execution`: Independent steps run concurrently
- `test_parallel_respects_dependencies`: Dependent steps run serially
- `test_hybrid_analysis`: Dependencies analyzed correctly
- `test_hybrid_mixed_execution`: Mixed serial/parallel works

**Coverage Target:** 85%+

---

### Logging Framework (Task 10)

**Tests Required:**
- `test_log_level_from_str`: Parsing works
- `test_log_level_ordering`: Levels compare correctly
- `test_log_level_to_tracing`: Conversion works
- `test_scoped_logger`: Scoped logging works
- `test_logging_config_default`: Defaults are correct
- `test_logging_config_custom`: Custom config works

**Coverage Target:** 80%+ (tracing crate is well-tested)

---

### Metrics Collection (Task 11)

**Tests Required:**
- `test_metric_counter`: Counter works
- `test_metric_gauge`: Gauge works
- `test_metric_histogram`: Histogram works
- `test_metric_labels`: Labels set correctly
- `test_pipeline_metrics`: Pipeline metrics collected
- `test_pipeline_duration`: Duration calculated
- `test_step_metrics`: Step metrics collected
- `test_step_success`: Success flag set
- `test_tool_metrics`: Tool metrics collected
- `test_metrics_collector_pipeline`: Collector tracks pipelines
- `test_metrics_collector_step`: Collector tracks steps
- `test_metrics_export`: Export to JSON works

**Coverage Target:** 90%+

---

### Retry & Error Handling (Task 12)

**Tests Required:**
- `test_retry_strategy_fixed`: Fixed delay works
- `test_retry_strategy_linear`: Linear backoff works
- `test_retry_strategy_exponential`: Exponential backoff works
- `test_retry_strategy_exponential_max`: Max delay respected
- `test_error_classification_transient`: Transient errors detected
- `test_error_classification_permanent`: Permanent errors detected
- `test_error_classification_user`: User errors detected
- `test_retry_policy_success`: Returns on success
- `test_retry_policy_retry`: Retries transient errors
- `test_retry_policy_no_retry_permanent`: No retry on permanent
- `test_retry_policy_no_retry_user`: No retry on user error
- `test_retry_policy_max_attempts`: Stops after max attempts
- `test_error_escalation`: Handlers called correctly
- `test_log_error_handler`: Logs errors
- `test_panic_error_handler`: Panics on fatal

**Coverage Target:** 90%+

---

## Test Organization

```
tests/
  unit/
    chat_session_test.rs
    queue_test.rs
    queue_storage_test.rs
    scheduler_test.rs
    executor_test.rs
    loop_runner_test.rs
    branch_evaluator_test.rs
    parallel_executor_test.rs
    human_gating_test.rs
    execution_mode_test.rs
    logging_test.rs
    metrics_test.rs
    retry_test.rs
```

---

## Running Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test chat_session --lib
cargo test queue --lib
cargo test scheduler --lib

# Run with coverage
cargo tarpaulin --lib --out Html

# Run tests in watch mode
cargo watch -x test --lib
```

---

## Coverage Goals

- **Overall coverage:** 90%+
- **Critical paths:** 95%+
- **Error handling:** 95%+
- **Public APIs:** 100%
