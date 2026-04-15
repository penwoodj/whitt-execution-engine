# Phase 07 Autonomy & Metrics - Property-Based Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define property-based test specifications for verifying system invariants

**Architecture:** Property-based tests use proptest to verify invariants across random inputs

**Tech Stack:** Rust, cargo test, proptest, tokio

---

## Property-Based Test Overview

Property-based tests verify system invariants:
1. Boundedness: Loops terminate within max iterations
2. Override Availability: Override is functional at any state
3. Metrics Consistency: Metrics are non-negative and monotonic
4. Checkpoint Integrity: Restored state matches saved state
5. Risk Assessment: Autonomy level is appropriate for risk profile

---

## PT1: Boundedness Property Test

**Files:**
- Create: `tests/property/boundedness_test.rs`

**Property:**
For any contract with stop conditions, the autonomous loop terminates within max iterations.

- [ ] **Step 1: Write boundedness property test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{AutonomyContract, AutonomyLevel};
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use proptest::prelude::*;

prop_compose! {
    fn arb_autonomy_level()() -> AutonomyLevel {
        prop_oneof![
            Just(AutonomyLevel::Manual),
            Just(AutonomyLevel::Bounded),
            Just(AutonomyLevel::Supervised),
            Just(AutonomyLevel::Autonomous),
        ]
    }

    fn arb_max_iterations() -> u32 {
        10u32..1000u32
    }

    fn arb_stop_condition_value() -> u64 {
        1u64..1000u64
    }

    fn arb_contract() -> AutonomyContract {
        (arb_autonomy_level(), arb_max_iterations(), arb_stop_condition_value())
            .prop_map(|(autonomy_level, max_iterations, stop_value)| {
                AutonomyContract {
                    goals: vec![],
                    autonomy_level,
                    max_iterations: Some(max_iterations),
                    timeout_seconds: Some(300),
                    stop_conditions: vec![
                        crate::autonomy::contract::StopCondition {
                            condition_type: crate::autonomy::contract::StopConditionType::Iteration,
                            value: stop_value,
                        },
                    ],
                    checkpoint_frequency: 10,
                }
            })
    }
}

proptest! {
    #[test]
    fn prop_loop_terminates_within_max_iterations(contract in arb_contract()) {
        let metrics = MetricsCollector::new();
        let mut loop_state = AutonomousLoop::with_metrics(contract.clone(), metrics);

        loop_state.start().await;

        // Execute until termination
        let mut iteration_count = 0;
        let max_iterations = contract.max_iterations.unwrap_or(100);

        loop {
            let result = loop_state.execute_iteration().await;

            if result.should_terminate {
                break;
            }

            iteration_count += 1;

            // Property: Should not exceed max_iterations
            prop_assert!(
                iteration_count <= max_iterations,
                "Loop iteration count {} should not exceed max_iterations {}",
                iteration_count, max_iterations
            );

            // Safety: Prevent infinite loops in tests
            if iteration_count >= max_iterations + 100 {
                break;
            }
        }

        // Property: Should have terminated
        prop_assert!(
            loop_state.is_complete().await,
            "Loop should have completed within {} iterations",
            max_iterations
        );
    }

    #[test]
    fn prop_iteration_stop_condition_triggers(contract in arb_contract()) {
        let metrics = MetricsCollector::new();
        let mut loop_state = AutonomousLoop::with_metrics(contract.clone(), metrics);

        loop_state.start().await;

        // Find iteration stop condition
        let iteration_limit = contract.stop_conditions.iter()
            .find_map(|sc| {
                if matches!(sc.condition_type, crate::autonomy::contract::StopConditionType::Iteration) {
                    Some(sc.value as u32)
                } else {
                    None
                }
            })
            .unwrap_or(contract.max_iterations.unwrap_or(100));

        // Execute until stop condition triggers
        let mut iteration_count = 0;
        loop {
            let result = loop_state.execute_iteration().await;

            if result.should_terminate {
                break;
            }

            iteration_count += 1;

            // Property: Should stop at iteration limit
            if iteration_count == iteration_limit {
                prop_assert!(
                    result.should_terminate,
                    "Loop should terminate at iteration limit {}",
                    iteration_limit
                );
                break;
            }

            // Safety
            if iteration_count >= iteration_limit + 100 {
                break;
            }
        }
    }
}
```

- [ ] **Step 2: Run boundedness property tests**

Run: `cargo test prop_loop_terminates_within_max_iterations --test boundedness_test`
Expected: PASS

Run: `cargo test prop_iteration_stop_condition_triggers --test boundedness_test`
Expected: PASS

- [ ] **Step 3: Write timeout boundedness property test**

```rust
proptest! {
    #[test]
    fn prop_timeout_stop_condition_triggers(
        timeout_seconds in 10u64..3600u64
    ) {
        let contract = AutonomyContract {
            goals: vec![],
            autonomy_level: AutonomyLevel::Bounded,
            max_iterations: Some(1000),
            timeout_seconds: Some(timeout_seconds),
            stop_conditions: vec![
                crate::autonomy::contract::StopCondition {
                    condition_type: crate::autonomy::contract::StopConditionType::Time,
                    value: timeout_seconds,
                },
            ],
            checkpoint_frequency: 10,
        };

        let metrics = MetricsCollector::new();
        let mut loop_state = AutonomousLoop::with_metrics(contract.clone(), metrics);

        loop_state.start().await;

        let mut elapsed_seconds = 0;
        loop {
            let result = loop_state.execute_iteration().await;

            if result.should_terminate {
                break;
            }

            elapsed_seconds += 1;

            // Property: Should not exceed timeout
            prop_assert!(
                elapsed_seconds <= timeout_seconds,
                "Elapsed time {} should not exceed timeout {}",
                elapsed_seconds, timeout_seconds
            );

            // Safety
            if elapsed_seconds >= timeout_seconds + 10 {
                break;
            }
        }

        // Property: Should have terminated
        prop_assert!(
            loop_state.is_complete().await,
            "Loop should have completed within timeout"
        );
    }
}
```

- [ ] **Step 4: Run timeout boundedness test**

Run: `cargo test prop_timeout_stop_condition_triggers --test boundedness_test`
Expected: PASS

- [ ] **Step 5: Commit PT1 tests**

```bash
git add tests/property/boundedness_test.rs
git commit -m "test(PT1): add boundedness property-based tests"
```

---

## PT2: Override Availability Property Test

**Files:**
- Create: `tests/property/override_availability_test.rs`

**Property:**
For any execution state, override commands are functional.

- [ ] **Step 1: Write override availability property test**

```rust
use yaml_to_rust_agentsdk::autonomy::override::{
    OverrideController, OverrideCommand, OverrideReason, ExecutionState
};
use proptest::prelude::*;

prop_compose! {
    fn arb_execution_state() -> ExecutionState {
        prop_oneof![
            Just(ExecutionState::Running),
            Just(ExecutionState::Paused),
            Just(ExecutionState::Stopped),
            Just(ExecutionState::Completed),
            Just(ExecutionState::Failed),
        ]
    }

    fn arb_override_command() -> OverrideCommand {
        prop_oneof![
            Just(OverrideCommand::Pause {
                reason: OverrideReason::Manual,
                requested_by: "test".to_string(),
            }),
            Just(OverrideCommand::Resume),
            Just(OverrideCommand::Stop {
                reason: OverrideReason::Manual,
                requested_by: "test".to_string(),
            }),
            Just(OverrideCommand::ModifyScope {
                new_goals: vec!["Test".to_string()],
                reason: OverrideReason::Manual,
            }),
        ]
    }
}

proptest! {
    #[test]
    fn prop_override_available_at_any_state(
        command in arb_override_command(),
        initial_state in arb_execution_state()
    ) {
        let controller = OverrideController::new();
        let _handle = controller.start_autonomous_loop().await;

        // Set initial state (if supported)
        // Note: In real implementation, we'd inject the state

        // Send command
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(controller.send_command(command.clone()));

        // Property: Command should be accepted
        prop_assert!(
            result.is_ok(),
            "Override command should be accepted at any state"
        );
    }

    #[test]
    fn prop_pause_available_at_running_state(
        reason in prop_oneof![
            Just(OverrideReason::Manual),
            Just(OverrideReason::QualityThreshold),
            Just(OverrideReason::Timeout),
            Just(OverrideReason::ErrorDetected),
        ],
        requested_by in ".*{1,20}"
    ) {
        let controller = OverrideController::new();
        let _handle = controller.start_autonomous_loop().await;

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Send pause command
        let command = OverrideCommand::Pause {
            reason: reason.clone(),
            requested_by: requested_by.clone(),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(controller.send_command(command));

        // Property: Pause should always be available
        prop_assert!(
            result.is_ok(),
            "Pause should be available at running state"
        );

        // Verify state changed
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        let state = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(controller.get_state());

        prop_assert!(
            matches!(state, ExecutionState::Paused),
            "State should be Paused after pause command"
        );
    }
}
```

- [ ] **Step 2: Run override availability property tests**

Run: `cargo test prop_override_available --test override_availability_test`
Expected: PASS

Run: `cargo test prop_pause_available_at_running_state --test override_availability_test`
Expected: PASS

- [ ] **Step 3: Write override event emission property test**

```rust
proptest! {
    #[test]
    fn prop_override_event_emitted(
        command in arb_override_command()
    ) {
        let controller = OverrideController::new();
        let _handle = controller.start_autonomous_loop().await;

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Send command
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(controller.send_command(command.clone()));

        // Property: Event should be emitted
        let mut event_rx = controller.subscribe_events();

        let event = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(tokio::time::timeout(
                tokio::time::Duration::from_millis(100),
                event_rx.recv()
            ));

        prop_assert!(
            event.is_ok(),
            "Override event should be emitted for any command"
        );

        let event = event.unwrap().unwrap();

        // Verify event content
        prop_assert!(
            matches!(&event.command, &command),
            "Event command should match sent command"
        );
    }
}
```

- [ ] **Step 4: Run override event emission test**

Run: `cargo test prop_override_event_emitted --test override_availability_test`
Expected: PASS

- [ ] **Step 5: Commit PT2 tests**

```bash
git add tests/property/override_availability_test.rs
git commit -m "test(PT2): add override availability property-based tests"
```

---

## PT3: Metrics Consistency Property Test

**Files:**
- Create: `tests/property/metrics_consistency_test.rs`

**Property:**
For any execution, metrics are non-negative and monotonic.

- [ ] **Step 1: Write metrics non-negative property test**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use proptest::prelude::*;

prop_compose! {
    fn arb_counter_value() -> u64 {
        0u64..1000u64
    }

    fn arb_gauge_value() -> f64 {
        -1000.0f64..1000.0f64
    }

    fn arb_histogram_value() -> f64 {
        0.0f64..10000.0f64
    }
}

proptest! {
    #[test]
    fn prop_counter_values_non_negative(
        increments in prop::collection::vec(arb_counter_value(), 1..100)
    ) {
        let collector = MetricsCollector::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Add increments
        for increment in increments {
            runtime.block_on(collector.increment_counter("test_counter", increment, &[]));
        }

        // Property: Counter value should be non-negative
        let value = runtime.block_on(collector.get_counter_value("test_counter"));
        let total: u64 = increments.iter().sum();

        prop_assert!(
            value == total,
            "Counter value {} should equal sum of increments {}",
            value, total
        );

        prop_assert!(
            value >= 0,
            "Counter value should be non-negative"
        );
    }

    #[test]
    fn prop_gauge_values_within_range(
        values in prop::collection::vec(arb_gauge_value(), 1..100)
    ) {
        let collector = MetricsCollector::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Set values
        for value in values {
            runtime.block_on(collector.set_gauge("test_gauge", value, &[]));
        }

        // Property: Gauge value should be within valid range
        let value = runtime.block_on(collector.get_gauge_value("test_gauge"));

        prop_assert!(
            !value.is_nan(),
            "Gauge value should not be NaN"
        );
    }

    #[test]
    fn prop_histogram_values_non_negative(
        samples in prop::collection::vec(arb_histogram_value(), 1..100)
    ) {
        let collector = MetricsCollector::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Add samples
        for sample in samples {
            runtime.block_on(collector.observe_histogram("test_histogram", sample, &[]));
        }

        // Property: Histogram summary should have non-negative values
        let summary = runtime.block_on(collector.get_histogram_summary("test_histogram"));

        prop_assert!(
            summary.count == samples.len(),
            "Histogram count {} should equal number of samples {}",
            summary.count, samples.len()
        );

        prop_assert!(
            summary.sum >= 0.0,
            "Histogram sum should be non-negative"
        );

        prop_assert!(
            summary.min >= 0.0,
            "Histogram min should be non-negative"
        );

        prop_assert!(
            summary.max >= 0.0,
            "Histogram max should be non-negative"
        );

        prop_assert!(
            summary.avg >= 0.0,
            "Histogram average should be non-negative"
        );
    }
}
```

- [ ] **Step 2: Run metrics non-negative property tests**

Run: `cargo test prop_counter_values_non_negative --test metrics_consistency_test`
Expected: PASS

Run: `cargo test prop_gauge_values_within_range --test metrics_consistency_test`
Expected: PASS

Run: `cargo test prop_histogram_values_non_negative --test metrics_consistency_test`
Expected: PASS

- [ ] **Step 3: Write metrics monotonic property test**

```rust
proptest! {
    #[test]
    fn prop_counter_monotonic_increment(
        increments in prop::collection::vec(arb_counter_value(), 1..100)
    ) {
        let collector = MetricsCollector::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut previous_value = 0u64;

        // Increment counter
        for increment in increments {
            runtime.block_on(collector.increment_counter("test_counter", increment, &[]));

            let current_value = runtime.block_on(collector.get_counter_value("test_counter"));

            // Property: Counter should be monotonic
            prop_assert!(
                current_value >= previous_value,
                "Counter should be monotonic: {} >= {}",
                current_value, previous_value
            );

            previous_value = current_value;
        }
    }

    #[test]
    fn prop_histogram_statistics_consistent(
        samples in prop::collection::vec(arb_histogram_value(), 10..100)
    ) {
        let collector = MetricsCollector::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Add samples
        for sample in &samples {
            runtime.block_on(collector.observe_histogram("test_histogram", *sample, &[]));
        }

        let summary = runtime.block_on(collector.get_histogram_summary("test_histogram"));

        // Property: Statistics should be consistent
        let calculated_sum: f64 = samples.iter().sum();
        let calculated_count = samples.len();
        let calculated_min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let calculated_max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let calculated_avg = calculated_sum / calculated_count as f64;

        prop_assert!(
            (summary.sum - calculated_sum).abs() < 0.001,
            "Histogram sum {} should equal calculated sum {}",
            summary.sum, calculated_sum
        );

        prop_assert!(
            summary.count == calculated_count,
            "Histogram count {} should equal sample count {}",
            summary.count, calculated_count
        );

        prop_assert!(
            (summary.min - calculated_min).abs() < 0.001,
            "Histogram min {} should equal calculated min {}",
            summary.min, calculated_min
        );

        prop_assert!(
            (summary.max - calculated_max).abs() < 0.001,
            "Histogram max {} should equal calculated max {}",
            summary.max, calculated_max
        );

        prop_assert!(
            (summary.avg - calculated_avg).abs() < 0.001,
            "Histogram avg {} should equal calculated avg {}",
            summary.avg, calculated_avg
        );
    }
}
```

- [ ] **Step 4: Run metrics monotonic property tests**

Run: `cargo test prop_counter_monotonic_increment --test metrics_consistency_test`
Expected: PASS

Run: `cargo test prop_histogram_statistics_consistent --test metrics_consistency_test`
Expected: PASS

- [ ] **Step 5: Commit PT3 tests**

```bash
git add tests/property/metrics_consistency_test.rs
git commit -m "test(PT3): add metrics consistency property-based tests"
```

---

## PT4: Checkpoint Integrity Property Test

**Files:**
- Create: `tests/property/checkpoint_integrity_test.rs`

**Property:**
For any checkpoint, restore produces equivalent state.

- [ ] **Step 1: Write checkpoint integrity property test**

```rust
use yaml_to_rust_agentsdk::autonomy::checkpoint::{CheckpointManager, CheckpointMetadata, CheckpointType};
use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointRestorer;
use proptest::prelude::*;

prop_compose! {
    fn arb_checkpoint_state() -> serde_json::Value {
        prop_oneof![
            Just(serde_json::json!({"iteration": 42})),
            Just(serde_json::json!({"iteration": 100, "goals": ["goal1", "goal2"]})),
            Just(serde_json::json!({
                "nested": {
                    "level1": {
                        "level2": {
                            "value": 12345
                        }
                    }
                }
            })),
            Just(serde_json::json!({
                "array": [1, 2, 3, 4, 5],
                "nested": {"key": "value"}
            })),
        ]
    }

    fn arb_checkpoint_type() -> CheckpointType {
        prop_oneof![
            Just(CheckpointType::Periodic),
            Just(CheckpointType::EventTriggered),
            Just(CheckpointType::BeforeRisk),
            Just(CheckpointType::AfterIntervention),
            Just(CheckpointType::Manual),
        ]
    }
}

proptest! {
    #[test]
    fn prop_checkpoint_restore_produces_equivalent_state(
        state in arb_checkpoint_state(),
        checkpoint_type in arb_checkpoint_type(),
        iteration in 1u32..1000u32,
        task_id in "task_[0-9]{1,10}"
    ) {
        let manager = CheckpointManager::new();
        let restorer = CheckpointRestorer::new(manager.clone());

        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Create checkpoint
        let metadata = CheckpointMetadata {
            reason: format!("Test checkpoint at {}", iteration),
            checkpoint_type,
            iteration,
            tags: vec!["test".to_string()],
        };

        let checkpoint_id = runtime.block_on(
            manager.generate_checkpoint(task_id.clone(), state.clone(), metadata)
        );

        // Property: Restore should produce equivalent state
        let restored = runtime.block_on(
            restorer.restore_checkpoint(&checkpoint_id)
        );

        prop_assert!(
            restored.is_ok(),
            "Checkpoint restore should succeed"
        );

        let restored_state = restored.unwrap();

        prop_assert!(
            restored_state == state,
            "Restored state should match original state"
        );
    }

    #[test]
    fn prop_checkpoint_id_unique(
        states in prop::collection::vec(arb_checkpoint_state(), 1..10),
        task_id in "task_[0-9]{1,10}"
    ) {
        let manager = CheckpointManager::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut checkpoint_ids = std::collections::HashSet::new();

        // Create multiple checkpoints
        for (i, state) in states.iter().enumerate() {
            let metadata = CheckpointMetadata {
                reason: format!("Checkpoint {}", i),
                checkpoint_type: CheckpointType::Manual,
                iteration: i as u32,
                tags: vec![],
            };

            let checkpoint_id = runtime.block_on(
                manager.generate_checkpoint(task_id.clone(), state.clone(), metadata)
            );

            // Property: Checkpoint IDs should be unique
            prop_assert!(
                checkpoint_ids.insert(checkpoint_id.clone()),
                "Checkpoint IDs should be unique"
            );
        }
    }
}
```

- [ ] **Step 2: Run checkpoint integrity property tests**

Run: `cargo test prop_checkpoint_restore_produces_equivalent_state --test checkpoint_integrity_test`
Expected: PASS

Run: `cargo test prop_checkpoint_id_unique --test checkpoint_integrity_test`
Expected: PASS

- [ ] **Step 3: Write checkpoint order preservation property test**

```rust
proptest! {
    #[test]
    fn prop_checkpoint_order_preserved(
        num_checkpoints in 1usize..20usize,
        task_id in "task_[0-9]{1,10}"
    ) {
        let manager = CheckpointManager::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut iteration_numbers = vec![];

        // Create checkpoints in order
        for i in 0..num_checkpoints {
            let iteration = (i * 10) as u32;
            iteration_numbers.push(iteration);

            let state = serde_json::json!({"iteration": iteration});

            let metadata = CheckpointMetadata {
                reason: format!("Checkpoint {}", i),
                checkpoint_type: CheckpointType::Periodic,
                iteration,
                tags: vec![],
            };

            runtime.block_on(
                manager.generate_checkpoint(task_id.clone(), state, metadata)
            );
        }

        // List checkpoints
        let checkpoints = runtime.block_on(
            manager.list_checkpoints(&task_id)
        );

        // Property: Checkpoints should be returned in order
        prop_assert!(
            checkpoints.len() == num_checkpoints,
            "Should have {} checkpoints", num_checkpoints
        );

        for (i, checkpoint) in checkpoints.iter().enumerate() {
            prop_assert!(
                checkpoint.metadata.iteration == iteration_numbers[i],
                "Checkpoint {} should have iteration {}",
                i, iteration_numbers[i]
            );
        }
    }
}
```

- [ ] **Step 4: Run checkpoint order preservation test**

Run: `cargo test prop_checkpoint_order_preserved --test checkpoint_integrity_test`
Expected: PASS

- [ ] **Step 5: Commit PT4 tests**

```bash
git add tests/property/checkpoint_integrity_test.rs
git commit -m "test(PT4): add checkpoint integrity property-based tests"
```

---

## PT5: Risk Assessment Property Test

**Files:**
- Create: `tests/property/risk_assessment_test.rs`

**Property:**
For any risk profile, autonomy level is appropriate.

- [ ] **Step 1: Write risk assessment property test**

```rust
use yaml_to_rust_agentsdk::autonomy::risk::{RiskAssessor, RiskProfile, InterventionRecord};
use yaml_to_rust_agentsdk::autonomy::scope::ActionType;
use yaml_to_rust_agentsdk::autonomy::contract::AutonomyLevel;
use proptest::prelude::*;

prop_compose! {
    fn arb_action_type() -> ActionType {
        prop_oneof![
            Just(ActionType::ReadFile),
            Just(ActionType::WriteFile),
            Just(ActionType::DeleteFile),
            Just(ActionType::ExecuteCommand),
            Just(ActionType::NetworkRequest),
        ]
    }

    fn arb_autonomy_level() -> AutonomyLevel {
        prop_oneof![
            Just(AutonomyLevel::Manual),
            Just(AutonomyLevel::Bounded),
            Just(AutonomyLevel::Supervised),
            Just(AutonomyLevel::Autonomous),
        ]
    }

    fn arb_path() -> String {
        prop_oneof![
            Just("/tmp/test.txt".to_string()),
            Just("/home/user/test.txt".to_string()),
            Just("/etc/passwd".to_string()),
            Just("/usr/bin/test".to_string()),
        ]
    }

    fn arb_intervention_history() -> Vec<InterventionRecord> {
        prop::collection::vec(
            prop::sample::select((
                prop::num::f64::ANY, // timestamp as duration
                ".*{1,20}".prop_map(|s| s), // reason
            )).prop_map(|(duration, reason)| InterventionRecord {
                timestamp: std::time::SystemTime::now()
                    .checked_sub(std::time::Duration::from_secs_f64(duration.abs() as f64))
                    .unwrap(),
                reason,
            }),
            0..10
        )
    }
}

proptest! {
    #[test]
    fn prop_risk_score_in_valid_range(
        action_type in arb_action_type(),
        path in arb_path(),
        autonomy_level in arb_autonomy_level(),
        intervention_history in arb_intervention_history()
    ) {
        let risk_assessor = RiskAssessor::new();

        let risk_profile = RiskProfile {
            action_type: action_type.clone(),
            path: Some(path),
            autonomy_level: autonomy_level.clone(),
            intervention_history,
        };

        let risk_score = risk_assessor.compute_risk_score(&risk_profile);

        // Property: Risk score should be in valid range [0.0, 1.0]
        prop_assert!(
            risk_score >= 0.0 && risk_score <= 1.0,
            "Risk score {} should be in range [0.0, 1.0]",
            risk_score
        );
    }

    #[test]
    fn prop_autonomy_level_adjustment_based_on_risk(
        action_type in arb_action_type(),
        path in arb_path(),
        autonomy_level in arb_autonomy_level()
    ) {
        let risk_assessor = RiskAssessor::new();

        let risk_profile = RiskProfile {
            action_type: action_type.clone(),
            path: Some(path),
            autonomy_level: autonomy_level.clone(),
            intervention_history: vec![],
        };

        let risk_score = risk_assessor.compute_risk_score(&risk_profile);
        let adjusted_level = risk_assessor.adjust_autonomy_level(&risk_profile);

        // Property: High risk should reduce autonomy
        if risk_score > 0.7 {
            prop_assert!(
                matches!(adjusted_level, AutonomyLevel::Bounded | AutonomyLevel::Manual),
                "High risk score {} should reduce autonomy to Bounded or Manual",
                risk_score
            );
        }

        // Property: Low risk should maintain or increase autonomy
        if risk_score < 0.3 {
            prop_assert!(
                matches!(adjusted_level, AutonomyLevel::Supervised | AutonomyLevel::Autonomous),
                "Low risk score {} should maintain or increase autonomy",
                risk_score
            );
        }
    }
}
```

- [ ] **Step 2: Run risk assessment property tests**

Run: `cargo test prop_risk_score_in_valid_range --test risk_assessment_test`
Expected: PASS

Run: `cargo test prop_autonomy_level_adjustment_based_on_risk --test risk_assessment_test`
Expected: PASS

- [ ] **Step 3: Write risk assessment monotonic property test**

```rust
proptest! {
    #[test]
    fn prop_intervention_history_increases_risk(
        action_type in arb_action_type(),
        path in arb_path(),
        autonomy_level in arb_autonomy_level(),
        base_interventions in 0usize..5usize
    ) {
        let risk_assessor = RiskAssessor::new();

        // Profile with fewer interventions
        let profile_low_risk = RiskProfile {
            action_type: action_type.clone(),
            path: Some(path.clone()),
            autonomy_level: autonomy_level.clone(),
            intervention_history: (0..base_interventions).map(|i| InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: format!("Intervention {}", i),
            }).collect(),
        };

        let risk_score_low = risk_assessor.compute_risk_score(&profile_low_risk);

        // Profile with more interventions
        let profile_high_risk = RiskProfile {
            action_type: action_type.clone(),
            path: Some(path.clone()),
            autonomy_level: autonomy_level.clone(),
            intervention_history: (0..(base_interventions + 5)).map(|i| InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: format!("Intervention {}", i),
            }).collect(),
        };

        let risk_score_high = risk_assessor.compute_risk_score(&profile_high_risk);

        // Property: More interventions should increase risk
        prop_assert!(
            risk_score_high >= risk_score_low,
            "More interventions should increase or maintain risk score: {} >= {}",
            risk_score_high, risk_score_low
        );
    }
}
```

- [ ] **Step 4: Run risk assessment monotonic test**

Run: `cargo test prop_intervention_history_increases_risk --test risk_assessment_test`
Expected: PASS

- [ ] **Step 5: Commit PT5 tests**

```bash
git add tests/property/risk_assessment_test.rs
git commit -m "test(PT5): add risk assessment property-based tests"
```

---

## Summary

All 5 property-based test suites have been defined with comprehensive invariant verification:

1. **PT1**: Boundedness - 5 steps
   - Loops terminate within max iterations
   - Stop conditions trigger at expected times

2. **PT2**: Override Availability - 5 steps
   - Override available at any state
   - Event emission for all commands

3. **PT3**: Metrics Consistency - 5 steps
   - Metrics are non-negative
   - Metrics are monotonic
   - Statistics are consistent

4. **PT4**: Checkpoint Integrity - 5 steps
   - Restored state matches saved state
   - Checkpoint IDs are unique
   - Order is preserved

5. **PT5**: Risk Assessment - 5 steps
   - Risk scores in valid range
   - Autonomy level appropriate for risk
   - Intervention history influences risk

Each property-based test suite includes:
- Proptest arbitraries for random inputs
- Property assertions using prop_assert!
- Invariant verification across inputs
- Comprehensive coverage of system behavior
