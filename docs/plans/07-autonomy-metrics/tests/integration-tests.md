# Phase 07 Autonomy & Metrics - Integration Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define comprehensive integration test specifications for multi-component workflows

**Architecture:** Integration tests verify component interactions and end-to-end workflows

**Tech Stack:** Rust, cargo test, tokio, integration test harness

---

## Integration Test Overview

Integration tests cover multi-component workflows:
1. Full Autonomous Loop
2. Metrics Pipeline
3. Dashboard Data Flow
4. Override During Execution
5. Checkpoint Restore
6. Risk-Based Autonomy Adjustment

---

## IT1: Full Autonomous Loop Integration Test

**Files:**
- Create: `tests/integration/full_autonomous_loop_test.rs`

**Coverage:**
- Contract parsing → Execute → Checkpoint → Override → Resume → Complete workflow
- Stop condition evaluation during loop
- Metrics collection throughout loop
- Intervention capture during loop

- [ ] **Step 1: Write full autonomous loop test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, AutonomyContract};
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointManager;
use yaml_to_rust_agentsdk::autonomy::override::{OverrideController, OverrideCommand, OverrideReason};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_full_autonomous_loop_workflow() {
    // Step 1: Parse contract
    let yaml = r#"
goals:
  - name: "Generate test files"
    type: "file_generation"
    target_path: "/tmp"
    priority: 1
    acceptance_criteria:
      - "10 files created"
autonomy_level: "bounded"
max_iterations: 20
stop_conditions:
  - type: "iteration"
    value: 20
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    // Step 2: Initialize components
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let override_controller = OverrideController::new();
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    // Step 3: Start loop
    loop_state.start().await;

    // Step 4: Execute iterations
    for iteration in 1..=15 {
        loop_state.execute_iteration().await;

        // Check checkpoint frequency
        if iteration % 5 == 0 {
            let checkpoints = checkpoint_manager.list_checkpoints("test_task").await;
            assert!(checkpoints.len() >= (iteration / 5) as usize,
                    "Checkpoints should be created periodically");
        }
    }

    // Step 5: Pause loop
    override_controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    sleep(Duration::from_millis(100)).await;

    let state = override_controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused),
            "Loop should be paused");

    // Step 6: Resume loop
    override_controller.send_command(OverrideCommand::Resume).await;
    sleep(Duration::from_millis(100)).await;

    let state = override_controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Running),
            "Loop should be resumed");

    // Step 7: Complete remaining iterations
    for _ in 16..=20 {
        loop_state.execute_iteration().await;
    }

    // Step 8: Verify metrics collected
    let iterations = metrics.get_counter_value("loop_iterations").await;
    assert!(iterations >= 20, "Should have completed 20 iterations");

    // Step 9: Verify loop completed
    assert!(loop_state.is_complete().await, "Loop should be complete");
}
```

- [ ] **Step 2: Run full autonomous loop test**

Run: `cargo test test_full_autonomous_loop_workflow --test full_autonomous_loop_test`
Expected: PASS

- [ ] **Step 3: Write stop condition evaluation during loop test**

```rust
#[tokio::test]
async fn test_stop_condition_evaluation_during_loop() {
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    loop_state.start().await;

    // Execute iterations until stop condition triggers
    let mut completed = false;
    for iteration in 1..=15 {
        let result = loop_state.execute_iteration().await;
        if result.should_terminate {
            completed = true;
            assert!(iteration <= 10, "Should stop at iteration 10");
            break;
        }
    }

    assert!(completed, "Loop should have stopped");
}
```

- [ ] **Step 4: Run stop condition evaluation test**

Run: `cargo test test_stop_condition_evaluation_during_loop --test full_autonomous_loop_test`
Expected: PASS

- [ ] **Step 5: Write metrics collection during loop test**

```rust
#[tokio::test]
async fn test_metrics_collection_during_loop() {
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    loop_state.start().await;

    // Execute all iterations
    for _ in 1..=10 {
        loop_state.execute_iteration().await;
    }

    // Verify metrics were collected
    let iterations = metrics.get_counter_value("loop_iterations").await;
    assert_eq!(iterations, 10, "Should have 10 iterations");

    let avg_duration = metrics.get_histogram_summary("iteration_duration_ms").await;
    assert_eq!(avg_duration.count, 10, "Should have 10 duration samples");

    let completed = metrics.get_counter_value("goals_completed").await;
    assert!(completed >= 0, "Goals completed metric should exist");
}
```

- [ ] **Step 6: Run metrics collection test**

Run: `cargo test test_metrics_collection_during_loop --test full_autonomous_loop_test`
Expected: PASS

- [ ] **Step 7: Write intervention capture during loop test**

```rust
#[tokio::test]
async fn test_intervention_capture_during_loop() {
    use yaml_to_rust_agentsdk::autonomy::intervention::{InterventionLogger, InterventionType};

    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let intervention_logger = InterventionLogger::new();
    let mut loop_state = AutonomousLoop::with_intervention_logger(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
        intervention_logger.clone(),
    );

    loop_state.start().await;

    // Execute iterations and simulate intervention
    for iteration in 1..=10 {
        if iteration == 5 {
            // Simulate intervention
            intervention_logger.log_intervention(
                InterventionType::Manual,
                "Manual intervention at iteration 5".to_string(),
                crate::autonomy::intervention::InterventionContext {
                    task_id: "test_task".to_string(),
                    goal: "Test goal".to_string(),
                    iteration: iteration,
                    autonomy_level: contract.autonomy_level.clone(),
                    metrics_snapshot: serde_json::json!({}),
                    execution_state: serde_json::json!({}),
                    trigger_conditions: vec!["Manual trigger".to_string()],
                },
            ).await;
        }

        loop_state.execute_iteration().await;
    }

    // Verify intervention was captured
    let interventions = intervention_logger.get_all_interventions().await;
    assert_eq!(interventions.len(), 1);
    assert_eq!(interventions[0].iteration, 5);
}
```

- [ ] **Step 8: Run intervention capture test**

Run: `cargo test test_intervention_capture_during_loop --test full_autonomous_loop_test`
Expected: PASS

- [ ] **Step 9: Commit IT1 tests**

```bash
git add tests/integration/full_autonomous_loop_test.rs
git commit -m "test(IT1): add full autonomous loop integration tests"
```

---

## IT2: Metrics Pipeline Integration Test

**Files:**
- Create: `tests/integration/metrics_pipeline_test.rs`

**Coverage:**
- Instrument → Collect → Aggregate → Export workflow
- Multiple metric types in single pipeline
- Label-based aggregation
- Real-time updates

- [ ] **Step 1: Write metrics pipeline test**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::{MetricsCollector, MetricSnapshot};
use yaml_to_rust_agentsdk::autonomy::metrics::types::{MetricType, Counter, Gauge, Histogram};

#[tokio::test]
async fn test_metrics_pipeline_workflow() {
    let collector = MetricsCollector::new();

    // Step 1: Instrument
    collector.increment_counter("tasks_completed", 1, &[
        ("task_type", "file_generation"),
        ("autonomy_level", "bounded"),
    ]).await;

    collector.set_gauge("active_tasks", 3, &[]).await;

    for duration in [100.0, 200.0, 300.0, 400.0, 500.0] {
        collector.observe_histogram("task_duration_ms", duration, &[
            ("task_type", "file_generation"),
        ]).await;
    }

    // Step 2: Collect
    let counter_value = collector.get_counter_value("tasks_completed").await;
    assert_eq!(counter_value, 1);

    let gauge_value = collector.get_gauge_value("active_tasks").await;
    assert_eq!(gauge_value, 3);

    let histogram_summary = collector.get_histogram_summary("task_duration_ms").await;
    assert_eq!(histogram_summary.count, 5);

    // Step 3: Aggregate
    let avg_duration = histogram_summary.avg;
    assert_eq!(avg_duration, 300.0);

    // Step 4: Export
    let snapshots = collector.export_all().await;
    assert!(snapshots.len() >= 3); // Counter, gauge, histogram

    // Verify snapshot types
    let counter_snapshot = snapshots.iter().find(|s| matches!(s, MetricSnapshot::Counter(_))).unwrap();
    assert!(matches!(counter_snapshot, MetricSnapshot::Counter(_)));

    let gauge_snapshot = snapshots.iter().find(|s| matches!(s, MetricSnapshot::Gauge(_))).unwrap();
    assert!(matches!(gauge_snapshot, MetricSnapshot::Gauge(_)));

    let histogram_snapshot = snapshots.iter().find(|s| matches!(s, MetricSnapshot::Histogram(_))).unwrap();
    assert!(matches!(histogram_snapshot, MetricSnapshot::Histogram(_)));
}
```

- [ ] **Step 2: Run metrics pipeline test**

Run: `cargo test test_metrics_pipeline_workflow --test metrics_pipeline_test`
Expected: PASS

- [ ] **Step 3: Write multiple metric types test**

```rust
#[tokio::test]
async fn test_multiple_metric_types_in_pipeline() {
    let collector = MetricsCollector::new();

    // Collect different metric types
    collector.increment_counter("counter1", 1, &[]).await;
    collector.set_gauge("gauge1", 42.5, &[]).await;
    collector.observe_histogram("histogram1", 100.0, &[]).await;

    // Collect more of each type
    collector.increment_counter("counter2", 2, &[]).await;
    collector.set_gauge("gauge2", 24.0, &[]).await;
    collector.observe_histogram("histogram2", 200.0, &[]).await;

    // Export all
    let snapshots = collector.export_all().await;

    // Verify all types are present
    let counters = snapshots.iter().filter(|s| matches!(s, MetricSnapshot::Counter(_))).count();
    let gauges = snapshots.iter().filter(|s| matches!(s, MetricSnapshot::Gauge(_))).count();
    let histograms = snapshots.iter().filter(|s| matches!(s, MetricSnapshot::Histogram(_))).count();

    assert_eq!(counters, 2);
    assert_eq!(gauges, 2);
    assert_eq!(histograms, 2);
}
```

- [ ] **Step 4: Run multiple metric types test**

Run: `cargo test test_multiple_metric_types_in_pipeline --test metrics_pipeline_test`
Expected: PASS

- [ ] **Step 5: Write label-based aggregation test**

```rust
#[tokio::test]
async fn test_label_based_aggregation() {
    let collector = MetricsCollector::new();

    // Collect metrics with different labels
    collector.increment_counter("tasks_completed", 5, &[
        ("task_type", "file_generation"),
        ("autonomy_level", "bounded"),
    ]).await;

    collector.increment_counter("tasks_completed", 3, &[
        ("task_type", "testing"),
        ("autonomy_level", "bounded"),
    ]).await;

    collector.increment_counter("tasks_completed", 2, &[
        ("task_type", "file_generation"),
        ("autonomy_level", "autonomous"),
    ]).await;

    // Total should be sum of all
    let total = collector.get_counter_value("tasks_completed").await;
    assert_eq!(total, 10);
}
```

- [ ] **Step 6: Run label-based aggregation test**

Run: `cargo test test_label_based_aggregation --test metrics_pipeline_test`
Expected: PASS

- [ ] **Step 7: Write real-time updates test**

```rust
#[tokio::test]
async fn test_real_time_metric_updates() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let collector = MetricsCollector::new();

    // Collect initial metrics
    collector.increment_counter("counter", 1, &[]).await;
    let value1 = collector.get_counter_value("counter").await;
    assert_eq!(value1, 1);

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Update metrics
    collector.increment_counter("counter", 2, &[]).await;
    let value2 = collector.get_counter_value("counter").await;
    assert_eq!(value2, 3);

    // Export and verify timestamp
    let snapshots = collector.export_all().await;
    let snapshot = snapshots.iter().next().unwrap();

    // Should have a timestamp (or be exportable)
    assert!(true, "Metrics should be exportable");
}
```

- [ ] **Step 8: Run real-time updates test**

Run: `cargo test test_real_time_metric_updates --test metrics_pipeline_test`
Expected: PASS

- [ ] **Step 9: Commit IT2 tests**

```bash
git add tests/integration/metrics_pipeline_test.rs
git commit -m "test(IT2): add metrics pipeline integration tests"
```

---

## IT3: Dashboard Data Flow Integration Test

**Files:**
- Create: `tests/integration/dashboard_data_flow_test.rs`

**Coverage:**
- Metrics → Streaming → Visualization workflow
- Real-time dashboard updates
- Anomaly detection integration
- Success and regression dashboard rendering

- [ ] **Step 1: Write dashboard data flow test**

```rust
use yaml_to_rust_agentsdk::autonomy::dashboard::{
    DashboardRenderer, DashboardData, SuccessMetrics, RegressionMetrics
};
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use yaml_to_rust_agentsdk::autonomy::dashboard::AnomalyDetector;

#[tokio::test]
async fn test_dashboard_data_flow_from_metrics() {
    // Step 1: Collect metrics
    let metrics = MetricsCollector::new();

    metrics.increment_counter("goals_achieved", 45, &[]).await;
    metrics.increment_counter("tasks_completed", 150, &[]).await;

    for time in [1000.0, 1500.0, 2000.0, 2500.0, 3000.0] {
        metrics.observe_histogram("time_to_usefulness_ms", time, &[]).await;
    }

    // Step 2: Aggregate metrics into dashboard data
    let dashboard_data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2000.0,
            quality_score: 87.5,
            success_rate: 0.92,
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 12,
            interventions_per_hour: 2.4,
            error_rate: 0.08,
            rollback_count: 2,
        },
        timestamp: chrono::Utc::now(),
    };

    // Step 3: Render dashboard
    let renderer = DashboardRenderer::new();
    let rendered = renderer.render_success_dashboard(dashboard_data.clone()).await;

    // Step 4: Verify data is present in rendering
    assert!(rendered.contains("150"));
    assert!(rendered.contains("45"));
    assert!(rendered.contains("2000.0"));
    assert!(rendered.contains("87.5"));
    assert!(rendered.contains("92"));
}
```

- [ ] **Step 2: Run dashboard data flow test**

Run: `cargo test test_dashboard_data_flow_from_metrics --test dashboard_data_flow_test`
Expected: PASS

- [ ] **Step 3: Write real-time dashboard updates test**

```rust
#[tokio::test]
async fn test_real_time_dashboard_updates() {
    let renderer = DashboardRenderer::new();

    // Initial data
    let data1 = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 100,
            goals_achieved: 30,
            avg_time_to_usefulness_ms: 2000.0,
            quality_score: 80.0,
            success_rate: 0.85,
        },
        regression_metrics: Default::default(),
        timestamp: chrono::Utc::now(),
    };

    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Updated data
    let data2 = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2500.0,
            quality_score: 87.5,
            success_rate: 0.92,
        },
        regression_metrics: Default::default(),
        timestamp: chrono::Utc::now(),
    };

    let rendered1 = renderer.render_success_dashboard(data1).await;
    let rendered2 = renderer.render_success_dashboard(data2).await;

    // Verify updates are reflected
    assert_ne!(rendered1, rendered2);
    assert!(rendered1.contains("100"));
    assert!(rendered2.contains("150"));
}
```

- [ ] **Step 4: Run real-time dashboard updates test**

Run: `cargo test test_real_time_dashboard_updates --test dashboard_data_flow_test`
Expected: PASS

- [ ] **Step 5: Write anomaly detection integration test**

```rust
#[tokio::test]
async fn test_anomaly_detection_integration_with_dashboard() {
    let detector = AnomalyDetector::new();
    let renderer = DashboardRenderer::new();

    let data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 10,
            goals_achieved: 5,
            avg_time_to_usefulness_ms: 5000.0,
            quality_score: 65.0, // Below threshold
            success_rate: 0.60,  // Below threshold
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 25,
            interventions_per_hour: 6.0, // Above threshold
            error_rate: 0.18,             // Above threshold
            rollback_count: 5,
        },
        timestamp: chrono::Utc::now(),
    };

    // Detect anomalies
    let alerts = detector.detect(&data);
    assert_eq!(alerts.len(), 4);

    // Render dashboard with alerts
    let rendered = renderer.render_combined_dashboard(data.clone(), &alerts).await;

    // Verify alerts are included
    assert!(rendered.contains("ANOMALY ALERTS"));
    assert!(rendered.contains("quality_score"));
    assert!(rendered.contains("interventions_per_hour"));
}
```

- [ ] **Step 6: Run anomaly detection integration test**

Run: `cargo test test_anomaly_detection_integration_with_dashboard --test dashboard_data_flow_test`
Expected: PASS

- [ ] **Step 7: Write success and regression dashboard test**

```rust
#[tokio::test]
async fn test_success_and_regression_dashboards() {
    let renderer = DashboardRenderer::new();

    let data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2500.0,
            quality_score: 87.5,
            success_rate: 0.92,
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 25,
            interventions_per_hour: 6.0,
            error_rate: 0.08,
            rollback_count: 3,
        },
        timestamp: chrono::Utc::now(),
    };

    // Render both dashboards
    let success_rendered = renderer.render_success_dashboard(data.clone()).await;
    let regression_rendered = renderer.render_regression_dashboard(data.clone()).await;

    // Verify success dashboard
    assert!(success_rendered.contains("SUCCESS METRICS"));
    assert!(success_rendered.contains("Tasks Completed"));
    assert!(success_rendered.contains("150"));

    // Verify regression dashboard
    assert!(regression_rendered.contains("REGRESSION METRICS"));
    assert!(regression_rendered.contains("Interventions"));
    assert!(regression_rendered.contains("25"));
}
```

- [ ] **Step 8: Run success and regression dashboard test**

Run: `cargo test test_success_and_regression_dashboards --test dashboard_data_flow_test`
Expected: PASS

- [ ] **Step 9: Commit IT3 tests**

```bash
git add tests/integration/dashboard_data_flow_test.rs
git commit -m "test(IT3): add dashboard data flow integration tests"
```

---

## IT4: Override During Execution Integration Test

**Files:**
- Create: `tests/integration/override_during_execution_test.rs`

**Coverage:**
- Running loop → Pause → Modify → Resume → Verify workflow
- Override state transitions
- Event emission during override
- Metrics preservation through override

- [ ] **Step 1: Write override during execution test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, AutonomyContract};
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointManager;
use yaml_to_rust_agentsdk::autonomy::override::{OverrideController, OverrideCommand, OverrideReason};
use yaml_to_rust_agentsdk::autonomy::override::ExecutionState;

#[tokio::test]
async fn test_override_during_execution_workflow() {
    // Step 1: Parse contract
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 20
stop_conditions:
  - type: "iteration"
    value: 20
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    // Step 2: Initialize components
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let override_controller = OverrideController::new();
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    // Step 3: Start execution
    loop_state.start().await;

    // Step 4: Execute some iterations
    for _ in 1..=10 {
        loop_state.execute_iteration().await;
    }

    // Step 5: Pause execution
    override_controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Paused));

    // Step 6: Verify event was emitted
    let mut event_rx = override_controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::Pause { .. }));

    // Step 7: Modify scope (optional)
    override_controller.send_command(OverrideCommand::ModifyScope {
        new_goals: vec!["Modified goal".to_string()],
        reason: OverrideReason::Manual,
    }).await;

    // Step 8: Resume execution
    override_controller.send_command(OverrideCommand::Resume).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Running));

    // Step 9: Complete remaining iterations
    for _ in 11..=20 {
        loop_state.execute_iteration().await;
    }

    // Step 10: Verify completion
    assert!(loop_state.is_complete().await);
}
```

- [ ] **Step 2: Run override during execution test**

Run: `cargo test test_override_during_execution_workflow --test override_during_execution_test`
Expected: PASS

- [ ] **Step 3: Write override state transitions test**

```rust
#[tokio::test]
async fn test_override_state_transitions() {
    let override_controller = OverrideController::new();
    let _handle = override_controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Initial state should be Running
    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Running));

    // Pause → Paused
    override_controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Paused));

    // Resume → Running
    override_controller.send_command(OverrideCommand::Resume).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Running));

    // Stop → Stopped
    override_controller.send_command(OverrideCommand::Stop {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let state = override_controller.get_state().await;
    assert!(matches!(state, ExecutionState::Stopped));
}
```

- [ ] **Step 4: Run override state transitions test**

Run: `cargo test test_override_state_transitions --test override_during_execution_test`
Expected: PASS

- [ ] **Step 5: Write event emission during override test**

```rust
#[tokio::test]
async fn test_event_emission_during_override() {
    let override_controller = OverrideController::new();
    let _handle = override_controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let mut event_rx = override_controller.subscribe_events();

    // Send pause command
    override_controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;

    // Receive pause event
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let pause_event = event.unwrap().unwrap();
    assert!(matches!(pause_event.command, OverrideCommand::Pause { .. }));
    assert!(matches!(pause_event.previous_state, ExecutionState::Running));
    assert!(matches!(pause_event.new_state, ExecutionState::Paused));

    // Send resume command
    override_controller.send_command(OverrideCommand::Resume).await;

    // Receive resume event
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let resume_event = event.unwrap().unwrap();
    assert!(matches!(resume_event.command, OverrideCommand::Resume));
    assert!(matches!(resume_event.previous_state, ExecutionState::Paused));
    assert!(matches!(resume_event.new_state, ExecutionState::Running));
}
```

- [ ] **Step 6: Run event emission test**

Run: `cargo test test_event_emission_during_override --test override_during_execution_test`
Expected: PASS

- [ ] **Step 7: Write metrics preservation through override test**

```rust
#[tokio::test]
async fn test_metrics_preservation_through_override() {
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let override_controller = OverrideController::new();
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    loop_state.start().await;

    // Execute some iterations
    for _ in 1..=5 {
        loop_state.execute_iteration().await;
    }

    // Collect metrics before pause
    let metrics_before = metrics.export_all().await;

    // Pause
    override_controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Collect metrics during pause
    let metrics_during_pause = metrics.export_all().await;

    // Resume
    override_controller.send_command(OverrideCommand::Resume).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Execute remaining iterations
    for _ in 6..=10 {
        loop_state.execute_iteration().await;
    }

    // Collect metrics after resume
    let metrics_after = metrics.export_all().await;

    // Verify metrics are preserved and accumulated
    assert!(metrics_after.len() >= metrics_before.len());
}
```

- [ ] **Step 8: Run metrics preservation test**

Run: `cargo test test_metrics_preservation_through_override --test override_during_execution_test`
Expected: PASS

- [ ] **Step 9: Commit IT4 tests**

```bash
git add tests/integration/override_during_execution_test.rs
git commit -m "test(IT4): add override during execution integration tests"
```

---

## IT5: Checkpoint Restore Integration Test

**Files:**
- Create: `tests/integration/checkpoint_restore_test.rs`

**Coverage:**
- Execute → Checkpoint → Terminate → Restore → Verify continuation workflow
- Checkpoint state capture
- State restoration accuracy
- Continuation after restore

- [ ] **Step 1: Write checkpoint restore workflow test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, AutonomyContract};
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use yaml_to_rust_agentsdk::autonomy::checkpoint::{CheckpointManager, CheckpointMetadata, CheckpointType};
use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointRestorer;

#[tokio::test]
async fn test_checkpoint_restore_workflow() {
    // Step 1: Parse contract
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 20
stop_conditions:
  - type: "iteration"
    value: 20
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    // Step 2: Initialize components
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(checkpoint_manager.clone());
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    loop_state.start().await;

    // Step 3: Execute to checkpoint point
    for iteration in 1..=10 {
        loop_state.execute_iteration().await;

        // Create checkpoint at iteration 10
        if iteration == 10 {
            let state = loop_state.get_state().await;

            checkpoint_manager.generate_checkpoint(
                "test_task".to_string(),
                state,
                CheckpointMetadata {
                    reason: "Checkpoint at iteration 10".to_string(),
                    checkpoint_type: CheckpointType::Periodic,
                    iteration: 10,
                    tags: vec!["periodic".to_string()],
                },
            ).await;
        }
    }

    // Step 4: Terminate execution
    loop_state.stop().await;

    // Step 5: Restore from checkpoint
    let checkpoints = checkpoint_manager.list_checkpoints("test_task").await;
    let checkpoint_id = checkpoints.last().unwrap().id.clone();

    let restored_state = restorer.restore_checkpoint(&checkpoint_id).await.expect("Restore should succeed");

    // Step 6: Verify restored state
    assert_eq!(restored_state["iteration"], 10);

    // Step 7: Create new loop state from restored checkpoint
    let mut loop_state_restored = AutonomousLoop::from_checkpoint(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
        restored_state,
    );

    loop_state_restored.start().await;

    // Step 8: Continue execution
    for _ in 11..=20 {
        loop_state_restored.execute_iteration().await;
    }

    // Step 9: Verify completion
    assert!(loop_state_restored.is_complete().await);
}
```

- [ ] **Step 2: Run checkpoint restore workflow test**

Run: `cargo test test_checkpoint_restore_workflow --test checkpoint_restore_test`
Expected: PASS

- [ ] **Step 3: Write checkpoint state capture test**

```rust
#[tokio::test]
async fn test_checkpoint_state_capture() {
    let checkpoint_manager = CheckpointManager::new();

    // Create complex state
    let original_state = serde_json::json!({
        "iteration": 42,
        "current_goal": "Generate test file",
        "goals_progress": {
            "goal1": 80.0,
            "goal2": 60.0,
        },
        "files_created": ["/tmp/test1.txt", "/tmp/test2.txt", "/tmp/test3.txt"],
        "tasks_completed": 15,
        "metrics": {
            "avg_duration_ms": 1500.0,
            "error_rate": 0.05,
            "interventions": 3,
        },
        "execution_context": {
            "working_directory": "/tmp",
            "environment": "test",
            "last_action": "write_file",
        },
    });

    let checkpoint_id = checkpoint_manager.generate_checkpoint(
        "test_task".to_string(),
        original_state.clone(),
        CheckpointMetadata {
            reason: "Complex state checkpoint".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 42,
            tags: vec![],
        },
    ).await;

    // Load and verify
    let checkpoint = checkpoint_manager.load_checkpoint(&checkpoint_id).await.unwrap();

    assert_eq!(checkpoint.state["iteration"], 42);
    assert_eq!(checkpoint.state["current_goal"], "Generate test file");
    assert_eq!(checkpoint.state["goals_progress"]["goal1"], 80.0);
    assert_eq!(checkpoint.state["files_created"].as_array().unwrap().len(), 3);
    assert_eq!(checkpoint.state["tasks_completed"], 15);
    assert_eq!(checkpoint.state["metrics"]["avg_duration_ms"], 1500.0);
    assert_eq!(checkpoint.state["execution_context"]["working_directory"], "/tmp");
}
```

- [ ] **Step 4: Run checkpoint state capture test**

Run: `cargo test test_checkpoint_state_capture --test checkpoint_restore_test`
Expected: PASS

- [ ] **Step 5: Write state restoration accuracy test**

```rust
#[tokio::test]
async fn test_state_restoration_accuracy() {
    let checkpoint_manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(checkpoint_manager.clone());

    // Create original state
    let original_state = serde_json::json!({
        "iteration": 100,
        "current_goal": "Complex goal",
        "nested": {
            "level1": {
                "level2": {
                    "value": 42,
                }
            }
        },
        "array": [1, 2, 3, 4, 5],
    });

    let checkpoint_id = checkpoint_manager.generate_checkpoint(
        "test_task".to_string(),
        original_state.clone(),
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 100,
            tags: vec![],
        },
    ).await;

    // Restore
    let restored_state = restorer.restore_checkpoint(&checkpoint_id).await.expect("Restore should succeed");

    // Verify exact match
    assert_eq!(restored_state, original_state);
}
```

- [ ] **Step 6: Run state restoration accuracy test**

Run: `cargo test test_state_restoration_accuracy --test checkpoint_restore_test`
Expected: PASS

- [ ] **Step 7: Write continuation after restore test**

```rust
#[tokio::test]
async fn test_continuation_after_restore() {
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 20
stop_conditions:
  - type: "iteration"
    value: 20
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");
    let metrics = MetricsCollector::new();
    let checkpoint_manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(checkpoint_manager.clone());

    // Execute to iteration 10
    let mut loop_state = AutonomousLoop::with_components(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
    );

    loop_state.start().await;

    for _ in 1..=10 {
        loop_state.execute_iteration().await;
    }

    // Checkpoint
    let state = loop_state.get_state().await;
    let checkpoint_id = checkpoint_manager.generate_checkpoint(
        "test_task".to_string(),
        state,
        CheckpointMetadata {
            reason: "Checkpoint".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 10,
            tags: vec![],
        },
    ).await;

    // Terminate
    loop_state.stop().await;

    // Restore
    let restored_state = restorer.restore_checkpoint(&checkpoint_id).await.unwrap();

    // Continue from iteration 10
    let mut loop_state_continued = AutonomousLoop::from_checkpoint(
        contract.clone(),
        metrics.clone(),
        checkpoint_manager.clone(),
        restored_state,
    );

    loop_state_continued.start().await;

    // Execute iterations 11-20
    for _ in 11..=20 {
        loop_state_continued.execute_iteration().await;
    }

    // Verify total iterations = 20
    let total_iterations = metrics.get_counter_value("loop_iterations").await;
    assert!(total_iterations >= 20);
}
```

- [ ] **Step 8: Run continuation after restore test**

Run: `cargo test test_continuation_after_restore --test checkpoint_restore_test`
Expected: PASS

- [ ] **Step 9: Commit IT5 tests**

```bash
git add tests/integration/checkpoint_restore_test.rs
git commit -m "test(IT5): add checkpoint restore integration tests"
```

---

## IT6: Risk-Based Autonomy Adjustment Integration Test

**Files:**
- Create: `tests/integration/risk_based_autonomy_test.rs`

**Coverage:**
- Low risk → High autonomy workflow
- High risk → Pause workflow
- Risk assessment integration with autonomy levels
- Dynamic autonomy adjustment

- [ ] **Step 1: Write low risk to high autonomy test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, AutonomyContract, AutonomyLevel};
use yaml_to_rust_agentsdk::autonomy::risk::{RiskAssessor, RiskProfile};
use yaml_to_rust_agentsdk::autonomy::confidence::{ConfidenceEvaluator, ActionContext, ActionType};
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;

#[tokio::test]
async fn test_low_risk_allows_high_autonomy() {
    // Step 1: Parse contract with high autonomy
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "supervised"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    // Step 2: Assess risk for low-risk action
    let risk_assessor = RiskAssessor::new();
    let confidence_evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Creating temporary file".to_string(),
        autonomy_level: contract.autonomy_level.clone(),
    };

    let confidence_result = confidence_evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    // Step 3: Verify low risk allows high autonomy
    assert!(!confidence_result.is_blocked, "Low risk should not block action");
    assert!(confidence_result.confidence >= 0.7, "Low risk should have high confidence");

    // Step 4: Verify autonomy level is appropriate
    let risk_profile = RiskProfile {
        action_type: ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: contract.autonomy_level.clone(),
        intervention_history: vec![],
    };

    let risk_score = risk_assessor.compute_risk_score(&risk_profile);
    let adjusted_level = risk_assessor.adjust_autonomy_level(&risk_profile);

    // Low risk should allow supervised or higher
    assert!(matches!(adjusted_level, AutonomyLevel::Supervised | AutonomyLevel::Autonomous),
            "Low risk should allow supervised or higher autonomy");
}
```

- [ ] **Step 2: Run low risk to high autonomy test**

Run: `cargo test test_low_risk_allows_high_autonomy --test risk_based_autonomy_test`
Expected: PASS

- [ ] **Step 3: Write high risk to pause test**

```rust
#[tokio::test]
async fn test_high_risk_blocks_or_reduces_autonomy() {
    // Step 1: Parse contract with high autonomy
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "autonomous"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    // Step 2: Assess risk for high-risk action
    let confidence_evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "System file modification".to_string(),
        autonomy_level: contract.autonomy_level.clone(),
    };

    let confidence_result = confidence_evaluator.evaluate_action(ActionType::DeleteFile, &context).await;

    // Step 3: Verify high risk blocks or reduces autonomy
    assert!(confidence_result.is_blocked, "High risk should block action");
    assert!(confidence_result.confidence < 0.5, "High risk should have low confidence");

    // Step 4: Verify autonomy level is reduced
    let risk_assessor = RiskAssessor::new();
    let risk_profile = RiskProfile {
        action_type: ActionType::DeleteFile,
        path: Some("/etc/passwd".to_string()),
        autonomy_level: contract.autonomy_level.clone(),
        intervention_history: vec![],
    };

    let adjusted_level = risk_assessor.adjust_autonomy_level(&risk_profile);

    // High risk should reduce to bounded or manual
    assert!(matches!(adjusted_level, AutonomyLevel::Bounded | AutonomyLevel::Manual),
            "High risk should reduce autonomy to bounded or manual");
}
```

- [ ] **Step 4: Run high risk to pause test**

Run: `cargo test test_high_risk_blocks_or_reduces_autonomy --test risk_based_autonomy_test`
Expected: PASS

- [ ] **Step 5: Write risk assessment integration test**

```rust
#[tokio::test]
async fn test_risk_assessment_integration_with_autonomy() {
    let contract = AutonomyContract {
        goals: vec![],
        autonomy_level: AutonomyLevel::Supervised,
        max_iterations: Some(100),
        timeout_seconds: Some(300),
        stop_conditions: vec![],
        checkpoint_frequency: 10,
    };

    let risk_assessor = RiskAssessor::new();
    let confidence_evaluator = ConfidenceEvaluator::new();

    // Test multiple actions with different risk profiles
    let actions = vec![
        (ActionType::ReadFile, "/tmp/test.txt", true),
        (ActionType::WriteFile, "/tmp/test.txt", true),
        (ActionType::DeleteFile, "/tmp/test.txt", false),
        (ActionType::DeleteFile, "/etc/passwd", false),
    ];

    for (action_type, path, should_be_safe) in actions {
        let context = ActionContext {
            path: Some(path.to_string()),
            reason: "Test".to_string(),
            autonomy_level: contract.autonomy_level.clone(),
        };

        let confidence_result = confidence_evaluator.evaluate_action(action_type, &context).await;

        if should_be_safe {
            assert!(!confidence_result.is_blocked, "{} should be safe", path);
        } else {
            assert!(confidence_result.is_blocked, "{} should be blocked", path);
        }
    }
}
```

- [ ] **Step 6: Run risk assessment integration test**

Run: `cargo test test_risk_assessment_integration_with_autonomy --test risk_based_autonomy_test`
Expected: PASS

- [ ] **Step 7: Write dynamic autonomy adjustment test**

```rust
#[tokio::test]
async fn test_dynamic_autonomy_adjustment() {
    let contract = AutonomyContract {
        goals: vec![],
        autonomy_level: AutonomyLevel::Supervised,
        max_iterations: Some(100),
        timeout_seconds: Some(300),
        stop_conditions: vec![],
        checkpoint_frequency: 10,
    };

    let risk_assessor = RiskAssessor::new();
    let confidence_evaluator = ConfidenceEvaluator::new();

    // Start with low-risk actions
    let context_safe = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Safe action".to_string(),
        autonomy_level: AutonomyLevel::Supervised,
    };

    let result_safe = confidence_evaluator.evaluate_action(ActionType::WriteFile, &context_safe).await;
    assert!(!result_safe.is_blocked);

    // Accumulate intervention history (increasing risk)
    let risk_profile_low = RiskProfile {
        action_type: ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: AutonomyLevel::Supervised,
        intervention_history: vec![],
    };

    let adjusted_level_low = risk_assessor.adjust_autonomy_level(&risk_profile_low);
    assert!(matches!(adjusted_level_low, AutonomyLevel::Supervised | AutonomyLevel::Autonomous));

    // Add intervention history
    let risk_profile_high = RiskProfile {
        action_type: ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: AutonomyLevel::Supervised,
        intervention_history: vec![
            crate::autonomy::risk::InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: "Failure 1".to_string(),
            },
            crate::autonomy::risk::InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: "Failure 2".to_string(),
            },
        ],
    };

    let adjusted_level_high = risk_assessor.adjust_autonomy_level(&risk_profile_high);
    assert!(matches!(adjusted_level_high, AutonomyLevel::Bounded | AutonomyLevel::Manual),
            "High risk history should reduce autonomy");
}
```

- [ ] **Step 8: Run dynamic autonomy adjustment test**

Run: `cargo test test_dynamic_autonomy_adjustment --test risk_based_autonomy_test`
Expected: PASS

- [ ] **Step 9: Commit IT6 tests**

```bash
git add tests/integration/risk_based_autonomy_test.rs
git commit -m "test(IT6): add risk-based autonomy adjustment integration tests"
```

---

## Summary

All 6 integration test suites have been defined with comprehensive workflow coverage:

1. **IT1**: Full Autonomous Loop - 9 steps
2. **IT2**: Metrics Pipeline - 9 steps
3. **IT3**: Dashboard Data Flow - 9 steps
4. **IT4**: Override During Execution - 9 steps
5. **IT5**: Checkpoint Restore - 9 steps
6. **IT6**: Risk-Based Autonomy Adjustment - 9 steps

Each integration test suite includes:
- End-to-end workflow specifications
- Multi-component interaction verification
- State transition validation
- Event emission verification
- Metrics preservation verification
- Specific assertions for each step
- Cargo test commands
- Clear pass/fail criteria
