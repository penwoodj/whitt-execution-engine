# Property-Based Tests Specification

This document defines property-based test requirements for Phase 1 MVP Queue & Scheduler.

---

## Test Philosophy

- **Invariants over examples**: Test properties that hold for all inputs
- **No magic numbers**: Generate random, diverse test cases
- **Counterexample analysis**: Understand why a property fails
- **Fast execution**: Properties verified in <1 second each

---

## Property Test Scenarios

### 1. State Machine Invariants

**Purpose:** Verify queue state machine never enters invalid state

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_state_transitions_are_valid(start in any::<QueueState>(), end in any::<QueueState>()) {
        // If transition is valid, end state must be reachable from start state
        if start.can_transition_to(&end) {
            // Verify transition is in the allowed transitions set
            assert!(is_valid_transition(start, end));
        }
    }
}
```

**Invariant:** All valid transitions are in the predefined transition table

**Test Cases:** 1000 random state pairs

**File:** `tests/property/state_machine_invariants_test.rs`

---

### 2. Job State Consistency

**Purpose:** Verify job state and stats are always consistent

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_job_state_matches_stats(state in any::<QueueState>()) {
        let job = create_job_with_state(state);

        // If job is running, started_at must be set
        if state == QueueState::Running {
            assert!(job.stats.started_at.is_some());
        }

        // If job is completed, completed_at must be set
        if state == QueueState::Completed {
            assert!(job.stats.completed_at.is_some());
        }

        // If job failed, error must be set
        if state == QueueState::Failed {
            assert!(job.error.is_some());
        }
    }
}
```

**Invariant:** Job state is consistent with job statistics

**Test Cases:** 1000 random states

**File:** `tests/property/job_state_consistency_test.rs`

---

### 3. Retry Convergence

**Purpose:** Verify retry eventually succeeds or exhausts attempts

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_retry_converges(strategy in retry_strategy(), max_attempts in 0..10usize) in {
        let mut attempt = 0;
        let mut success = false;

        while attempt < max_attempts {
            if random_success() {
                success = true;
                break;
            }

            let delay = strategy.delay(attempt as u32);
            tokio::time::sleep(delay).await;

            attempt += 1;
        }

        // Either succeeded or exhausted attempts
        assert!(success || attempt == max_attempts);
    }
}
```

**Invariant:** Retry always terminates (succeeds or exhausts attempts)

**Test Cases:** 100 random retry configurations

**File:** `tests/property/retry_convergence_test.rs`

---

### 4. Loop Termination

**Purpose:** Verify all loop types eventually terminate

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_count_loop_terminates(count in 0..100usize) in {
        let iterations = execute_count_loop(count);

        // Count loop always runs exactly 'count' times
        assert_eq!(iterations, count);
    }
}
```

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_foreach_loop_terminates(items in vec(any::<i32>(), 0..20usize)) in {
        let iterations = execute_foreach_loop(&items);

        // Foreach loop always runs exactly len(items) times
        assert_eq!(iterations, items.len());
    }
}
```

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_retry_loop_terminates(max_retries in 0..10usize) in {
        let result = execute_retry_loop_with_failure(max_retries);

        // Either succeeds or fails after max_retries
        assert!(result.success || result.attempts == max_retries + 1);
    }
}
```

**Invariant:** All loop types terminate (no infinite loops)

**Test Cases:** 100 random configurations per loop type

**File:** `tests/property/loop_termination_test.rs`

---

### 5. Branch Determinism

**Purpose:** Verify branch evaluation is deterministic

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_branch_evaluation_deterministic(
        branches in vec(branch_strategy(), 1..5usize),
        context in any::<ExecutionContext>()
    ) in {
        // Evaluate branches 10 times
        let results: Vec<_> = (0..10)
            .map(|_| evaluate_branches(&branches, &context))
            .collect();

        // All results should be identical
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }
}
```

**Invariant:** Branch evaluation always returns same result for same input

**Test Cases:** 100 random branch sets with 10 evaluations each

**File:** `tests/property/branch_determinism_test.rs`

---

### 6. Parallel Execution Idempotency

**Purpose:** Verify parallel execution is idempotent

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_parallel_execution_idempotent(
        steps in vec(step_definition(), 1..5usize),
        concurrency in 1..5usize
    ) in {
        // Execute parallel group twice
        let result1 = execute_parallel(&steps, concurrency).await;
        let result2 = execute_parallel(&steps, concurrency).await;

        // Results should be equivalent (order may differ)
        assert_results_equivalent(&result1, &result2);
    }
}
```

**Invariant:** Parallel execution of same inputs produces equivalent outputs

**Test Cases:** 100 random step sets

**File:** `tests/property/parallel_idempotency_test.rs`

---

### 7. Storage Persistence

**Purpose:** Verify storage operations are persistent

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_storage_persistence(job in any::<Job>()) in {
        let storage = QueueStorage::open_in_memory().unwrap();

        // Save job
        storage.save_job(&job).unwrap();

        // Load job
        let loaded = storage.load_job(&job.id).unwrap();

        // Jobs should be equal
        assert_eq!(Some(job), loaded);
    }
}
```

**Invariant:** Storage roundtrip preserves data

**Test Cases:** 100 random jobs

**File:** `tests/property/storage_persistence_test.rs`

---

### 8. Priority Ordering

**Purpose:** Verify priority queue maintains correct ordering

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_priority_ordering(jobs in vec(job_with_priority(), 1..20usize)) in {
        let mut heap = BinaryHeap::new();

        for job in &jobs {
            heap.push(PriorityJob::new(job.clone()));
        }

        // Pop all jobs
        let mut popped = Vec::new();
        while let Some(priority_job) = heap.pop() {
            popped.push(priority_job.job);
        }

        // Verify jobs are sorted by priority (descending)
        for window in popped.windows(2) {
            assert!(window[0].priority >= window[1].priority);
        }
    }
}
```

**Invariant:** Priority queue maintains descending priority order

**Test Cases:** 100 random job sets

**File:** `tests/property/priority_ordering_test.rs`

---

### 9. Metrics Accumulation

**Purpose:** Verify metrics accumulate correctly

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_metrics_accumulation(metrics in vec(metric_value(), 1..10usize)) in {
        let mut collector = MetricsCollector::new();

        // Add metrics
        for metric in &metrics {
            collector.add_metric(metric.clone());
        }

        // Verify count
        assert_eq!(collector.metric_count(), metrics.len());

        // Verify all metrics present
        for metric in &metrics {
            assert!(collector.contains_metric(metric));
        }
    }
}
```

**Invariant:** Metrics collector accurately tracks all metrics

**Test Cases:** 100 random metric sets

**File:** `tests/property/metrics_accumulation_test.rs`

---

### 10. Concurrency Safety

**Purpose:** Verify concurrent access is safe

**Property:**
```rust
prop_compose! {
    #[test]
    fn prop_concurrent_access_safe(
        jobs in vec(job_definition(), 1..10usize),
        workers in 2..5usize
    ) in {
        let storage = Arc::new(QueueStorage::open_in_memory().unwrap());
        let mut handles = Vec::new();

        // Spawn workers
        for worker_id in 0..workers {
            let storage = Arc::clone(&storage);
            let jobs = jobs.clone();

            let handle = tokio::spawn(async move {
                for job in jobs {
                    let _ = storage.save_job(&job);
                }
            });

            handles.push(handle);
        }

        // Wait for all workers
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify no data corruption (all jobs saved)
        let all_jobs = storage.list_jobs().unwrap();
        assert_eq!(all_jobs.len(), jobs.len() * workers);
    }
}
```

**Invariant:** Concurrent access doesn't corrupt data

**Test Cases:** 50 random configurations

**File:** `tests/property/concurrency_safety_test.rs`

---

## Running Property Tests

```bash
# Install proptest
cargo install proptest

# Run all property tests
cargo test --test property

# Run specific property test
cargo test --test property state_machine_invariants

# Run with more test cases
PROPTEST_CASES=10000 cargo test --test property

# Run in watch mode
cargo watch -x test --test property
```

---

## Property Test Organization

```
tests/
  property/
    state_machine_invariants_test.rs
    job_state_consistency_test.rs
    retry_convergence_test.rs
    loop_termination_test.rs
    branch_determinism_test.rs
    parallel_idempotency_test.rs
    storage_persistence_test.rs
    priority_ordering_test.rs
    metrics_accumulation_test.rs
    concurrency_safety_test.rs
```

---

## Success Criteria

- All 10 property-based test scenarios pass
- No shrinking takes >5 seconds (find counterexamples quickly)
- Coverage of edge cases >95%
- Test execution time <30 seconds total
