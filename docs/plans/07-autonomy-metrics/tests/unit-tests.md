# Phase 07 Autonomy & Metrics - Unit Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define comprehensive unit test specifications for all autonomy and metrics components

**Architecture:** Unit tests organized by component with specific function signatures and assertions

**Tech Stack:** Rust, cargo test, mockall, tokio-test

---

## Unit Test Overview

Unit tests cover individual components in isolation:
1. Contract Parsing/Validation
2. Metrics Types
3. Override Event Handling
4. Intervention Event Structure
5. Stop Condition Evaluation
6. Checkpoint Generation
7. Risk Assessment Model
8. Confidence Threshold Computation

---

## UT1: Contract Parsing/Validation Unit Tests

**Files:**
- Create: `tests/unit/contract_parsing_test.rs`

**Coverage:**
- Contract parsing from YAML
- Goal validation
- Autonomy level validation
- Stop condition validation
- Checkpoint frequency validation

- [ ] **Step 1: Write contract parsing test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, AutonomyContract, AutonomyLevel};

#[test]
fn test_parse_valid_contract() {
    let yaml = r#"
goals:
  - name: "Generate test file"
    type: "file_generation"
    target_path: "/tmp/test.txt"
    priority: 1
    acceptance_criteria:
      - "File exists"
      - "File is not empty"
autonomy_level: "bounded"
max_iterations: 100
timeout_seconds: 300
stop_conditions:
  - type: "iteration"
    value: 50
  - type: "time"
    value: 240
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    assert_eq!(contract.goals.len(), 1);
    assert_eq!(contract.goals[0].name, "Generate test file");
    assert_eq!(contract.autonomy_level, AutonomyLevel::Bounded);
    assert_eq!(contract.max_iterations, Some(100));
    assert_eq!(contract.timeout_seconds, Some(300));
    assert_eq!(contract.stop_conditions.len(), 2);
    assert_eq!(contract.checkpoint_frequency, 10);
}

#[test]
fn test_parse_contract_with_multiple_goals() {
    let yaml = r#"
goals:
  - name: "Goal 1"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
  - name: "Goal 2"
    type: "testing"
    priority: 2
    acceptance_criteria:
      - "Tests pass"
  - name: "Goal 3"
    type: "documentation"
    priority: 3
    acceptance_criteria:
      - "Docs written"
autonomy_level: "supervised"
max_iterations: 50
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    assert_eq!(contract.goals.len(), 3);
    assert_eq!(contract.goals[0].priority, 1);
    assert_eq!(contract.goals[1].priority, 2);
    assert_eq!(contract.goals[2].priority, 3);
}

#[test]
fn test_parse_invalid_autonomy_level() {
    let yaml = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "invalid"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let result = parse_contract(yaml);
    assert!(result.is_err(), "Invalid autonomy level should fail");
}
```

- [ ] **Step 2: Run contract parsing tests**

Run: `cargo test test_parse --test contract_parsing_test`
Expected: PASS

- [ ] **Step 3: Write goal validation test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{Goal, GoalType};

#[test]
fn test_goal_type_parsing() {
    let yaml = r#"
goals:
  - name: "File generation"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
  - name: "Testing"
    type: "testing"
    priority: 2
    acceptance_criteria:
      - "Tests pass"
  - name: "Documentation"
    type: "documentation"
    priority: 3
    acceptance_criteria:
      - "Docs written"
  - name: "Code refactoring"
    type: "code_refactoring"
    priority: 4
    acceptance_criteria:
      - "Code refactored"
  - name: "Deployment"
    type: "deployment"
    priority: 5
    acceptance_criteria:
      - "Deployed"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    assert_eq!(contract.goals[0].goal_type, GoalType::FileGeneration);
    assert_eq!(contract.goals[1].goal_type, GoalType::Testing);
    assert_eq!(contract.goals[2].goal_type, GoalType::Documentation);
    assert_eq!(contract.goals[3].goal_type, GoalType::CodeRefactoring);
    assert_eq!(contract.goals[4].goal_type, GoalType::Deployment);
}

#[test]
fn test_goal_with_acceptance_criteria() {
    let goal = Goal {
        name: "Test goal".to_string(),
        goal_type: GoalType::FileGeneration,
        target_path: Some("/tmp/test.txt".to_string()),
        priority: 1,
        acceptance_criteria: vec![
            "File exists".to_string(),
            "File is not empty".to_string(),
            "File has correct content".to_string(),
        ],
    };

    assert_eq!(goal.acceptance_criteria.len(), 3);
    assert!(goal.acceptance_criteria.contains(&"File exists".to_string()));
}
```

- [ ] **Step 4: Run goal validation tests**

Run: `cargo test test_goal --test contract_parsing_test`
Expected: PASS

- [ ] **Step 5: Write autonomy level validation test**

```rust
#[test]
fn test_autonomy_level_validation() {
    let yaml_manual = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "manual"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract_manual = parse_contract(yaml_manual).expect("Manual should parse");
    assert_eq!(contract_manual.autonomy_level, AutonomyLevel::Manual);

    let yaml_bounded = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let contract_bounded = parse_contract(yaml_bounded).expect("Bounded should parse");
    assert_eq!(contract_bounded.autonomy_level, AutonomyLevel::Bounded);

    let yaml_supervised = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "supervised"
max_iterations: 200
stop_conditions:
  - type: "iteration"
    value: 100
checkpoint_frequency: 20
"#;

    let contract_supervised = parse_contract(yaml_supervised).expect("Supervised should parse");
    assert_eq!(contract_supervised.autonomy_level, AutonomyLevel::Supervised);

    let yaml_autonomous = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "autonomous"
max_iterations: 500
stop_conditions:
  - type: "iteration"
    value: 100
  - type: "time"
    value: 300
  - type: "intervention"
    value: 5
checkpoint_frequency: 25
"#;

    let contract_autonomous = parse_contract(yaml_autonomous).expect("Autonomous should parse");
    assert_eq!(contract_autonomous.autonomy_level, AutonomyLevel::Autonomous);
}
```

- [ ] **Step 6: Run autonomy level validation tests**

Run: `cargo test test_autonomy_level --test contract_parsing_test`
Expected: PASS

- [ ] **Step 7: Write stop condition validation test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::StopConditionType;

#[test]
fn test_stop_condition_type_parsing() {
    let yaml = r#"
goals:
  - name: "Test"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
  - type: "time"
    value: 300
  - type: "quality"
    value: 80
  - type: "intervention"
    value: 5
  - type: "resource"
    value: 90
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).expect("Contract should parse");

    assert_eq!(contract.stop_conditions.len(), 5);
    assert_eq!(contract.stop_conditions[0].condition_type, StopConditionType::Iteration);
    assert_eq!(contract.stop_conditions[0].value, 50);
    assert_eq!(contract.stop_conditions[1].condition_type, StopConditionType::Time);
    assert_eq!(contract.stop_conditions[1].value, 300);
    assert_eq!(contract.stop_conditions[2].condition_type, StopConditionType::Quality);
    assert_eq!(contract.stop_conditions[2].value, 80);
    assert_eq!(contract.stop_conditions[3].condition_type, StopConditionType::Intervention);
    assert_eq!(contract.stop_conditions[3].value, 5);
    assert_eq!(contract.stop_conditions[4].condition_type, StopConditionType::Resource);
    assert_eq!(contract.stop_conditions[4].value, 90);
}
```

- [ ] **Step 8: Run stop condition validation tests**

Run: `cargo test test_stop_condition --test contract_parsing_test`
Expected: PASS

- [ ] **Step 9: Commit UT1 tests**

```bash
git add tests/unit/contract_parsing_test.rs
git commit -m "test(UT1): add contract parsing and validation unit tests"
```

---

## UT2: Metrics Types Unit Tests

**Files:**
- Create: `tests/unit/metrics_types_test.rs`

**Coverage:**
- Counter increment operations
- Gauge set operations
- Histogram observe operations
- Summary operations
- Metric label handling

- [ ] **Step 1: Write counter tests**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::{MetricsCollector, Counter};

#[tokio::test]
async fn test_counter_increment() {
    let collector = MetricsCollector::new();

    collector.increment_counter("test_counter", 1, &[]).await;
    let value = collector.get_counter_value("test_counter").await;

    assert_eq!(value, 1);
}

#[tokio::test]
async fn test_counter_increment_multiple() {
    let collector = MetricsCollector::new();

    collector.increment_counter("test_counter", 1, &[]).await;
    collector.increment_counter("test_counter", 2, &[]).await;
    collector.increment_counter("test_counter", 3, &[]).await;

    let value = collector.get_counter_value("test_counter").await;

    assert_eq!(value, 6);
}

#[tokio::test]
async fn test_counter_with_labels() {
    let collector = MetricsCollector::new();

    collector.increment_counter("tasks_completed", 1, &[
        ("task_type", "file_generation"),
        ("autonomy_level", "bounded"),
    ]).await;

    collector.increment_counter("tasks_completed", 2, &[
        ("task_type", "testing"),
        ("autonomy_level", "bounded"),
    ]).await;

    let value = collector.get_counter_value("tasks_completed").await;

    assert_eq!(value, 3);
}

#[tokio::test]
async fn test_counter_initial_value_zero() {
    let collector = MetricsCollector::new();

    let value = collector.get_counter_value("nonexistent_counter").await;

    assert_eq!(value, 0);
}
```

- [ ] **Step 2: Run counter tests**

Run: `cargo test test_counter --test metrics_types_test`
Expected: PASS

- [ ] **Step 3: Write gauge tests**

```rust
#[tokio::test]
async fn test_gauge_set() {
    let collector = MetricsCollector::new();

    collector.set_gauge("test_gauge", 42.5, &[]).await;
    let value = collector.get_gauge_value("test_gauge").await;

    assert_eq!(value, 42.5);
}

#[tokio::test]
async fn test_gauge_update() {
    let collector = MetricsCollector::new();

    collector.set_gauge("test_gauge", 10.0, &[]).await;
    let value1 = collector.get_gauge_value("test_gauge").await;
    assert_eq!(value1, 10.0);

    collector.set_gauge("test_gauge", 20.0, &[]).await;
    let value2 = collector.get_gauge_value("test_gauge").await;
    assert_eq!(value2, 20.0);
}

#[tokio::test]
async fn test_gauge_with_labels() {
    let collector = MetricsCollector::new();

    collector.set_gauge("active_tasks", 5, &[
        ("project", "yaml-to-rust"),
    ]).await;

    let value = collector.get_gauge_value("active_tasks").await;

    assert_eq!(value, 5);
}

#[tokio::test]
async fn test_gauge_initial_value_zero() {
    let collector = MetricsCollector::new();

    let value = collector.get_gauge_value("nonexistent_gauge").await;

    assert_eq!(value, 0.0);
}
```

- [ ] **Step 4: Run gauge tests**

Run: `cargo test test_gauge --test metrics_types_test`
Expected: PASS

- [ ] **Step 5: Write histogram tests**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::HistogramSummary;

#[tokio::test]
async fn test_histogram_observe() {
    let collector = MetricsCollector::new();

    collector.observe_histogram("test_histogram", 10.0, &[]).await;
    collector.observe_histogram("test_histogram", 20.0, &[]).await;
    collector.observe_histogram("test_histogram", 30.0, &[]).await;

    let summary = collector.get_histogram_summary("test_histogram").await;

    assert_eq!(summary.count, 3);
    assert_eq!(summary.sum, 60.0);
}

#[tokio::test]
async fn test_histogram_min_max() {
    let collector = MetricsCollector::new();

    collector.observe_histogram("test_histogram", 5.0, &[]).await;
    collector.observe_histogram("test_histogram", 15.0, &[]).await;
    collector.observe_histogram("test_histogram", 25.0, &[]).await;

    let summary = collector.get_histogram_summary("test_histogram").await;

    assert_eq!(summary.min, 5.0);
    assert_eq!(summary.max, 25.0);
}

#[tokio::test]
async fn test_histogram_average() {
    let collector = MetricsCollector::new();

    collector.observe_histogram("test_histogram", 10.0, &[]).await;
    collector.observe_histogram("test_histogram", 20.0, &[]).await;
    collector.observe_histogram("test_histogram", 30.0, &[]).await;

    let summary = collector.get_histogram_summary("test_histogram").await;

    assert_eq!(summary.avg, 20.0);
}

#[tokio::test]
async fn test_histogram_percentiles() {
    let collector = MetricsCollector::new();

    // Generate 100 samples
    for i in 1..=100 {
        collector.observe_histogram("test_histogram", i as f64, &[]).await;
    }

    let summary = collector.get_histogram_summary("test_histogram").await;

    assert!(summary.p50 >= 45.0 && summary.p50 <= 55.0); // ~50
    assert!(summary.p95 >= 90.0 && summary.p95 <= 100.0); // ~95
    assert!(summary.p99 >= 95.0 && summary.p99 <= 100.0); // ~99
}
```

- [ ] **Step 6: Run histogram tests**

Run: `cargo test test_histogram --test metrics_types_test`
Expected: PASS

- [ ] **Step 7: Write summary tests**

```rust
#[tokio::test]
async fn test_summary_observe() {
    let collector = MetricsCollector::new();

    collector.observe_summary("test_summary", 10.0, &[]).await;
    collector.observe_summary("test_summary", 20.0, &[]).await;
    collector.observe_summary("test_summary", 30.0, &[]).await;

    let stats = collector.get_summary_stats("test_summary").await;

    assert_eq!(stats.count, 3);
    assert_eq!(stats.sum, 60.0);
}

#[tokio::test]
async fn test_summary_average() {
    let collector = MetricsCollector::new();

    collector.observe_summary("test_summary", 10.0, &[]).await;
    collector.observe_summary("test_summary", 20.0, &[]).await;
    collector.observe_summary("test_summary", 30.0, &[]).await;

    let stats = collector.get_summary_stats("test_summary").await;

    assert_eq!(stats.avg, 20.0);
}
```

- [ ] **Step 8: Run summary tests**

Run: `cargo test test_summary --test metrics_types_test`
Expected: PASS

- [ ] **Step 9: Commit UT2 tests**

```bash
git add tests/unit/metrics_types_test.rs
git commit -m "test(UT2): add metrics types unit tests"
```

---

## UT3: Override Event Handling Unit Tests

**Files:**
- Create: `tests/unit/override_handling_test.rs`

**Coverage:**
- Pause command handling
- Resume command handling
- Stop command handling
- Modify scope command handling
- Adjust autonomy command handling
- Event emission

- [ ] **Step 1: Write pause command test**

```rust
use yaml_to_rust_agentsdk::autonomy::override::{
    OverrideController, OverrideCommand, OverrideReason, ExecutionState
};

#[tokio::test]
async fn test_pause_command_changes_state_to_paused() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, ExecutionState::Paused));
}

#[tokio::test]
async fn test_pause_command_emits_event() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::Pause { .. }));
    assert!(matches!(event.new_state, ExecutionState::Paused));
}
```

- [ ] **Step 2: Run pause command tests**

Run: `cargo test test_pause_command --test override_handling_test`
Expected: PASS

- [ ] **Step 3: Write resume command test**

```rust
#[tokio::test]
async fn test_resume_command_changes_state_to_running() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Pause first
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    assert!(matches!(controller.get_state().await, ExecutionState::Paused));

    // Resume
    controller.send_command(OverrideCommand::Resume).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, ExecutionState::Running));
}

#[tokio::test]
async fn test_resume_command_emits_event() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Pause
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Resume
    controller.send_command(OverrideCommand::Resume).await;

    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::Resume));
    assert!(matches!(event.new_state, ExecutionState::Running));
}
```

- [ ] **Step 4: Run resume command tests**

Run: `cargo test test_resume_command --test override_handling_test`
Expected: PASS

- [ ] **Step 5: Write stop command test**

```rust
#[tokio::test]
async fn test_stop_command_changes_state_to_stopped() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::Stop {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, ExecutionState::Stopped));
}

#[tokio::test]
async fn test_stop_command_emits_event() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::Stop {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::Stop { .. }));
    assert!(matches!(event.new_state, ExecutionState::Stopped));
}
```

- [ ] **Step 6: Run stop command tests**

Run: `cargo test test_stop_command --test override_handling_test`
Expected: PASS

- [ ] **Step 7: Write modify scope command test**

```rust
#[tokio::test]
async fn test_modify_scope_command_emits_event() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::ModifyScope {
        new_goals: vec!["Updated goal".to_string()],
        reason: OverrideReason::Manual,
    }).await;

    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::ModifyScope { .. }));
}
```

- [ ] **Step 8: Run modify scope command test**

Run: `cargo test test_modify_scope_command --test override_handling_test`
Expected: PASS

- [ ] **Step 9: Write adjust autonomy command test**

```rust
#[tokio::test]
async fn test_adjust_autonomy_command_emits_event() {
    let controller = OverrideController::new();
    let _handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    controller.send_command(OverrideCommand::AdjustAutonomy {
        new_level: crate::autonomy::contract::AutonomyLevel::Manual,
        reason: OverrideReason::Manual,
    }).await;

    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    let event = event.unwrap().unwrap();
    assert!(matches!(event.command, OverrideCommand::AdjustAutonomy { .. }));
}
```

- [ ] **Step 10: Run adjust autonomy command test**

Run: `cargo test test_adjust_autonomy_command --test override_handling_test`
Expected: PASS

- [ ] **Step 11: Commit UT3 tests**

```bash
git add tests/unit/override_handling_test.rs
git commit -m "test(UT3): add override event handling unit tests"
```

---

## UT4: Intervention Event Structure Unit Tests

**Files:**
- Create: `tests/unit/intervention_structure_test.rs`

**Coverage:**
- Intervention event creation
- Event field validation
- Intervention types
- Context capture
- Resolution tracking

- [ ] **Step 1: Write intervention event creation test**

```rust
use yaml_to_rust_agentsdk::autonomy::intervention::{
    InterventionLogger, InterventionType, InterventionContext
};

#[tokio::test]
async fn test_intervention_event_creation() {
    let logger = InterventionLogger::new();

    let context = InterventionContext {
        task_id: "task_123".to_string(),
        goal: "Generate test file".to_string(),
        iteration: 42,
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        metrics_snapshot: serde_json::json!({"test": "data"}),
        execution_state: serde_json::json!({"state": "running"}),
        trigger_conditions: vec!["Quality threshold".to_string()],
    };

    let id = logger.log_intervention(
        InterventionType::Manual,
        "Test intervention".to_string(),
        context,
    ).await;

    assert!(!id.to_string().is_empty());
}

#[tokio::test]
async fn test_intervention_event_fields_populated() {
    let logger = InterventionLogger::new();

    let context = InterventionContext {
        task_id: "task_456".to_string(),
        goal: "Test goal".to_string(),
        iteration: 10,
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        metrics_snapshot: serde_json::json!({}),
        execution_state: serde_json::json!({}),
        trigger_conditions: vec![],
    };

    let id = logger.log_intervention(
        InterventionType::ErrorDetected,
        "Error occurred".to_string(),
        context,
    ).await;

    let interventions = logger.get_all_interventions().await;
    let intervention = &interventions[0];

    assert_eq!(intervention.task_id, "task_456");
    assert_eq!(intervention.goal, "Test goal");
    assert_eq!(intervention.iteration, 10);
    assert!(matches!(intervention.intervention_type, InterventionType::ErrorDetected));
    assert_eq!(intervention.resolved, false);
}
```

- [ ] **Step 2: Run intervention event creation tests**

Run: `cargo test test_intervention_event --test intervention_structure_test`
Expected: PASS

- [ ] **Step 3: Write intervention type test**

```rust
#[tokio::test]
async fn test_intervention_types() {
    let logger = InterventionLogger::new();

    let types = vec![
        InterventionType::Manual,
        InterventionType::QualityThreshold,
        InterventionType::Timeout,
        InterventionType::ErrorDetected,
        InterventionType::ResourceExhausted,
        InterventionType::InterventionRate,
        InterventionType::ConfidenceLow,
        InterventionType::SafetyViolation,
    ];

    for intervention_type in types {
        let context = InterventionContext {
            task_id: "test".to_string(),
            goal: "Test".to_string(),
            iteration: 1,
            autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
            metrics_snapshot: serde_json::json!({}),
            execution_state: serde_json::json!({}),
            trigger_conditions: vec![],
        };

        logger.log_intervention(
            intervention_type.clone(),
            "Test".to_string(),
            context,
        ).await;
    }

    let interventions = logger.get_all_interventions().await;
    assert_eq!(interventions.len(), 8);
}
```

- [ ] **Step 4: Run intervention type tests**

Run: `cargo test test_intervention_types --test intervention_structure_test`
Expected: PASS

- [ ] **Step 5: Write context capture test**

```rust
#[tokio::test]
async fn test_intervention_context_capture() {
    let logger = InterventionLogger::new();

    let context = InterventionContext {
        task_id: "task_789".to_string(),
        goal: "Complex goal".to_string(),
        iteration: 100,
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Supervised,
        metrics_snapshot: serde_json::json!({
            "tasks_completed": 50,
            "avg_duration_ms": 1500.0,
            "error_rate": 0.05,
        }),
        execution_state: serde_json::json!({
            "current_file": "/tmp/test.txt",
            "files_created": ["/tmp/test1.txt", "/tmp/test2.txt"],
        }),
        trigger_conditions: vec![
            "Quality threshold".to_string(),
            "High error rate".to_string(),
        ],
    };

    let id = logger.log_intervention(
        InterventionType::QualityThreshold,
        "Quality dropped".to_string(),
        context,
    ).await;

    let interventions = logger.get_all_interventions().await;
    let intervention = &interventions[0];

    assert_eq!(intervention.context.task_id, "task_789");
    assert_eq!(intervention.context.goal, "Complex goal");
    assert_eq!(intervention.context.iteration, 100);
    assert_eq!(intervention.context.metrics_snapshot["tasks_completed"], 50);
    assert_eq!(intervention.context.execution_state["current_file"], "/tmp/test.txt");
    assert_eq!(intervention.context.trigger_conditions.len(), 2);
}
```

- [ ] **Step 6: Run context capture tests**

Run: `cargo test test_intervention_context --test intervention_structure_test`
Expected: PASS

- [ ] **Step 7: Write resolution tracking test**

```rust
#[tokio::test]
async fn test_intervention_resolution_tracking() {
    let logger = InterventionLogger::new();

    let context = InterventionContext {
        task_id: "task_xyz".to_string(),
        goal: "Test".to_string(),
        iteration: 1,
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        metrics_snapshot: serde_json::json!({}),
        execution_state: serde_json::json!({}),
        trigger_conditions: vec![],
    };

    let id = logger.log_intervention(
        InterventionType::ErrorDetected,
        "Test error".to_string(),
        context,
    ).await;

    // Check initially unresolved
    let interventions = logger.get_all_interventions().await;
    assert!(!interventions[0].resolved);

    // Mark as resolved
    let result = logger.mark_resolved(id, "Fixed the error".to_string()).await;
    assert!(result);

    // Check resolved
    let interventions = logger.get_all_interventions().await;
    assert!(interventions[0].resolved);
    assert_eq!(interventions[0].resolution.as_ref().unwrap(), "Fixed the error");
}
```

- [ ] **Step 8: Run resolution tracking tests**

Run: `cargo test test_intervention_resolution --test intervention_structure_test`
Expected: PASS

- [ ] **Step 9: Commit UT4 tests**

```bash
git add tests/unit/intervention_structure_test.rs
git commit -m "test(UT4): add intervention event structure unit tests"
```

---

## UT5: Stop Condition Evaluation Unit Tests

**Files:**
- Create: `tests/unit/stop_condition_evaluation_test.rs`

**Coverage:**
- Iteration stop condition
- Time stop condition
- Quality stop condition
- Intervention stop condition
- Resource stop condition

- [ ] **Step 1: Write iteration stop condition test**

```rust
use yaml_to_rust_agentsdk::autonomy::stop_conditions::{
    StopConditionEvaluator, StopCondition, StopConditionType, ExecutionContext
};
use std::time::Duration;

#[tokio::test]
async fn test_iteration_stop_condition_before_limit() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Iteration,
        value: 50,
    };

    let context = ExecutionContext {
        iteration: 25,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(!result.should_stop);
    assert_eq!(result.reason, "");
}

#[tokio::test]
async fn test_iteration_stop_condition_at_limit() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Iteration,
        value: 50,
    };

    let context = ExecutionContext {
        iteration: 50,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
    assert!(result.reason.contains("50"));
}

#[tokio::test]
async fn test_iteration_stop_condition_after_limit() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Iteration,
        value: 50,
    };

    let context = ExecutionContext {
        iteration: 75,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
}
```

- [ ] **Step 2: Run iteration stop condition tests**

Run: `cargo test test_iteration_stop --test stop_condition_evaluation_test`
Expected: PASS

- [ ] **Step 3: Write time stop condition test**

```rust
#[tokio::test]
async fn test_time_stop_condition_before_timeout() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Time,
        value: 300, // 5 minutes
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(200),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(!result.should_stop);
}

#[tokio::test]
async fn test_time_stop_condition_at_timeout() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Time,
        value: 300,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(300),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
    assert!(result.reason.contains("Timeout"));
}
```

- [ ] **Step 4: Run time stop condition tests**

Run: `cargo test test_time_stop --test stop_condition_evaluation_test`
Expected: PASS

- [ ] **Step 5: Write quality stop condition test**

```rust
#[tokio::test]
async fn test_quality_stop_condition_above_threshold() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Quality,
        value: 80, // 80% threshold
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.85,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(!result.should_stop);
}

#[tokio::test]
async fn test_quality_stop_condition_below_threshold() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Quality,
        value: 80,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.75,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
    assert!(result.reason.contains("Quality"));
}
```

- [ ] **Step 6: Run quality stop condition tests**

Run: `cargo test test_quality_stop --test stop_condition_evaluation_test`
Expected: PASS

- [ ] **Step 7: Write intervention stop condition test**

```rust
#[tokio::test]
async fn test_intervention_stop_condition_below_limit() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Intervention,
        value: 5,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 3,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(!result.should_stop);
}

#[tokio::test]
async fn test_intervention_stop_condition_at_limit() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Intervention,
        value: 5,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 5,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.5,
            memory: 0.6,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
    assert!(result.reason.contains("Intervention"));
}
```

- [ ] **Step 8: Run intervention stop condition tests**

Run: `cargo test test_intervention_stop --test stop_condition_evaluation_test`
Expected: PASS

- [ ] **Step 9: Write resource stop condition test**

```rust
#[tokio::test]
async fn test_resource_stop_condition_below_threshold_cpu() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Resource,
        value: 85, // 85% threshold
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.80,
            memory: 0.60,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(!result.should_stop);
}

#[tokio::test]
async fn test_resource_stop_condition_above_threshold_cpu() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Resource,
        value: 85,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.90,
            memory: 0.60,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
    assert!(result.reason.contains("Resource"));
}

#[tokio::test]
async fn test_resource_stop_condition_above_threshold_memory() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Resource,
        value: 85,
    };

    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage {
            cpu: 0.70,
            memory: 0.90,
        },
    };

    let result = evaluator.evaluate(&condition, &context).await;

    assert!(result.should_stop);
}
```

- [ ] **Step 10: Run resource stop condition tests**

Run: `cargo test test_resource_stop --test stop_condition_evaluation_test`
Expected: PASS

- [ ] **Step 11: Commit UT5 tests**

```bash
git add tests/unit/stop_condition_evaluation_test.rs
git commit -m "test(UT5): add stop condition evaluation unit tests"
```

---

## UT6: Checkpoint Generation Unit Tests

**Files:**
- Create: `tests/unit/checkpoint_generation_test.rs`

**Coverage:**
- Periodic checkpoint generation
- Event-triggered checkpoint generation
- Checkpoint state capture
- Checkpoint metadata
- Checkpoint ID generation

- [ ] **Step 1: Write periodic checkpoint test**

```rust
use yaml_to_rust_agentsdk::autonomy::checkpoint::{
    CheckpointManager, CheckpointMetadata, CheckpointType
};

#[tokio::test]
async fn test_periodic_checkpoint_generation() {
    let manager = CheckpointManager::new();

    for iteration in (0..=50).step_by(10) {
        let state = serde_json::json!({
            "iteration": iteration,
            "current_goal": "Test",
        });

        manager.generate_checkpoint(
            "task_123".to_string(),
            state,
            CheckpointMetadata {
                reason: "Periodic checkpoint".to_string(),
                checkpoint_type: CheckpointType::Periodic,
                iteration,
                tags: vec!["periodic".to_string()],
            },
        ).await;
    }

    let checkpoints = manager.list_checkpoints("task_123").await;
    assert_eq!(checkpoints.len(), 6); // 0, 10, 20, 30, 40, 50
}

#[tokio::test]
async fn test_periodic_checkpoint_iteration_numbers() {
    let manager = CheckpointManager::new();

    let iteration = 42;
    manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": iteration}),
        CheckpointMetadata {
            reason: "Periodic".to_string(),
            checkpoint_type: CheckpointType::Periodic,
            iteration,
            tags: vec![],
        },
    ).await;

    let checkpoints = manager.list_checkpoints("task_123").await;
    assert_eq!(checkpoints[0].metadata.iteration, 42);
}
```

- [ ] **Step 2: Run periodic checkpoint tests**

Run: `cargo test test_periodic_checkpoint --test checkpoint_generation_test`
Expected: PASS

- [ ] **Step 3: Write event-triggered checkpoint test**

```rust
#[tokio::test]
async fn test_event_triggered_checkpoint_before_risk() {
    let manager = CheckpointManager::new();

    manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 42}),
        CheckpointMetadata {
            reason: "Before risky operation".to_string(),
            checkpoint_type: CheckpointType::BeforeRisk,
            iteration: 42,
            tags: vec!["risky".to_string()],
        },
    ).await;

    let checkpoints = manager.list_checkpoints("task_123").await;
    assert_eq!(checkpoints.len(), 1);
    assert_eq!(checkpoints[0].metadata.checkpoint_type, CheckpointType::BeforeRisk);
    assert!(checkpoints[0].metadata.tags.contains(&"risky".to_string()));
}

#[tokio::test]
async fn test_event_triggered_checkpoint_after_intervention() {
    let manager = CheckpointManager::new();

    manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 43}),
        CheckpointMetadata {
            reason: "After manual intervention".to_string(),
            checkpoint_type: CheckpointType::AfterIntervention,
            iteration: 43,
            tags: vec!["intervention".to_string()],
        },
    ).await;

    let checkpoints = manager.list_checkpoints("task_123").await;
    assert_eq!(checkpoints.len(), 1);
    assert_eq!(checkpoints[0].metadata.checkpoint_type, CheckpointType::AfterIntervention);
}

#[tokio::test]
async fn test_manual_checkpoint() {
    let manager = CheckpointManager::new();

    manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 44}),
        CheckpointMetadata {
            reason: "Manual checkpoint".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 44,
            tags: vec![],
        },
    ).await;

    let checkpoints = manager.list_checkpoints("task_123").await;
    assert_eq!(checkpoints.len(), 1);
    assert_eq!(checkpoints[0].metadata.checkpoint_type, CheckpointType::Manual);
}
```

- [ ] **Step 4: Run event-triggered checkpoint tests**

Run: `cargo test test_event_triggered_checkpoint --test checkpoint_generation_test`
Expected: PASS

- [ ] **Step 5: Write checkpoint state capture test**

```rust
#[tokio::test]
async fn test_checkpoint_captures_full_state() {
    let manager = CheckpointManager::new();

    let original_state = serde_json::json!({
        "iteration": 42,
        "current_goal": "Generate test file",
        "files_created": ["/tmp/test1.txt", "/tmp/test2.txt"],
        "tasks_completed": 15,
        "metrics": {
            "avg_duration_ms": 1500.0,
            "error_rate": 0.05,
        },
    });

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        original_state.clone(),
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 42,
            tags: vec![],
        },
    ).await;

    let checkpoint = manager.load_checkpoint(&checkpoint_id).await.unwrap();

    assert_eq!(checkpoint.state["iteration"], 42);
    assert_eq!(checkpoint.state["current_goal"], "Generate test file");
    assert_eq!(checkpoint.state["files_created"].as_array().unwrap().len(), 2);
    assert_eq!(checkpoint.state["tasks_completed"], 15);
    assert_eq!(checkpoint.state["metrics"]["avg_duration_ms"], 1500.0);
}

#[tokio::test]
async fn test_checkpoint_preserves_nested_structures() {
    let manager = CheckpointManager::new();

    let state = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": 42,
                    "array": [1, 2, 3],
                }
            }
        }
    });

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        state.clone(),
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 1,
            tags: vec![],
        },
    ).await;

    let checkpoint = manager.load_checkpoint(&checkpoint_id).await.unwrap();

    assert_eq!(checkpoint.state["level1"]["level2"]["level3"]["value"], 42);
    assert_eq!(checkpoint.state["level1"]["level2"]["level3"]["array"].as_array().unwrap().len(), 3);
}
```

- [ ] **Step 6: Run checkpoint state capture tests**

Run: `cargo test test_checkpoint_state --test checkpoint_generation_test`
Expected: PASS

- [ ] **Step 7: Write checkpoint metadata test**

```rust
#[tokio::test]
async fn test_checkpoint_metadata_captured() {
    let manager = CheckpointManager::new();

    let metadata = CheckpointMetadata {
        reason: "Test checkpoint".to_string(),
        checkpoint_type: CheckpointType::Manual,
        iteration: 42,
        tags: vec!["manual".to_string(), "test".to_string()],
    };

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({}),
        metadata.clone(),
    ).await;

    let checkpoint = manager.load_checkpoint(&checkpoint_id).await.unwrap();

    assert_eq!(checkpoint.metadata.reason, "Test checkpoint");
    assert_eq!(checkpoint.metadata.checkpoint_type, CheckpointType::Manual);
    assert_eq!(checkpoint.metadata.iteration, 42);
    assert_eq!(checkpoint.metadata.tags.len(), 2);
    assert!(checkpoint.metadata.tags.contains(&"manual".to_string()));
    assert!(checkpoint.metadata.tags.contains(&"test".to_string()));
}

#[tokio::test]
async fn test_checkpoint_timestamp() {
    let manager = CheckpointManager::new();

    let before = std::time::SystemTime::now();

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({}),
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 1,
            tags: vec![],
        },
    ).await;

    let after = std::time::SystemTime::now();

    let checkpoint = manager.load_checkpoint(&checkpoint_id).await.unwrap();

    assert!(checkpoint.timestamp >= before);
    assert!(checkpoint.timestamp <= after);
}
```

- [ ] **Step 8: Run checkpoint metadata tests**

Run: `cargo test test_checkpoint_metadata --test checkpoint_generation_test`
Expected: PASS

- [ ] **Step 9: Commit UT6 tests**

```bash
git add tests/unit/checkpoint_generation_test.rs
git commit -m "test(UT6): add checkpoint generation unit tests"
```

---

## UT7: Risk Assessment Model Unit Tests

**Files:**
- Create: `tests/unit/risk_assessment_test.rs`

**Coverage:**
- Risk probability computation
- Impact computation
- Severity computation
- Risk score calculation
- Risk-based autonomy level adjustment

- [ ] **Step 1: Write risk probability test**

```rust
use yaml_to_rust_agentsdk::autonomy::risk::{RiskAssessor, RiskProfile, Probability};

#[test]
fn test_risk_probability_low() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::ReadFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![],
    };

    let probability = risk_assessor.compute_probability(&profile);

    assert_eq!(probability, Probability::Low);
}

#[test]
fn test_risk_probability_high() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::DeleteFile,
        path: Some("/etc/passwd".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
        intervention_history: vec![
            crate::autonomy::risk::InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: "Previous failure".to_string(),
            },
        ],
    };

    let probability = risk_assessor.compute_probability(&profile);

    assert_eq!(probability, Probability::High);
}
```

- [ ] **Step 2: Run risk probability tests**

Run: `cargo test test_risk_probability --test risk_assessment_test`
Expected: PASS

- [ ] **Step 3: Write risk impact test**

```rust
#[test]
fn test_risk_impact_low() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![],
    };

    let impact = risk_assessor.compute_impact(&profile);

    assert_eq!(impact, Impact::Low);
}

#[test]
fn test_risk_impact_high() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::DeleteFile,
        path: Some("/etc/passwd".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
        intervention_history: vec![],
    };

    let impact = risk_assessor.compute_impact(&profile);

    assert_eq!(impact, Impact::Critical);
}
```

- [ ] **Step 4: Run risk impact tests**

Run: `cargo test test_risk_impact --test risk_assessment_test`
Expected: PASS

- [ ] **Step 5: Write risk severity test**

```rust
#[test]
fn test_risk_severity_computation() {
    let risk_assessor = RiskAssessor::new();

    // Low probability + Low impact = Low severity
    let severity1 = risk_assessor.compute_severity(Probability::Low, Impact::Low);
    assert_eq!(severity1, Severity::Low);

    // High probability + High impact = Critical severity
    let severity2 = risk_assessor.compute_severity(Probability::High, Impact::Critical);
    assert_eq!(severity2, Severity::Critical);

    // Medium probability + Medium impact = Medium severity
    let severity3 = risk_assessor.compute_severity(Probability::Medium, Impact::Medium);
    assert_eq!(severity3, Severity::Medium);
}
```

- [ ] **Step 6: Run risk severity tests**

Run: `cargo test test_risk_severity --test risk_assessment_test`
Expected: PASS

- [ ] **Step 7: Write risk score calculation test**

```rust
#[test]
fn test_risk_score_calculation() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![],
    };

    let risk_score = risk_assessor.compute_risk_score(&profile);

    assert!(risk_score >= 0.0);
    assert!(risk_score <= 1.0);
}

#[test]
fn test_risk_score_increases_with_intervention_history() {
    let risk_assessor = RiskAssessor::new();

    let profile_without_history = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![],
    };

    let score1 = risk_assessor.compute_risk_score(&profile_without_history);

    let profile_with_history = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::WriteFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![
            crate::autonomy::risk::InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: "Previous failure".to_string(),
            },
        ],
    };

    let score2 = risk_assessor.compute_risk_score(&profile_with_history);

    assert!(score2 > score1, "Risk score should increase with intervention history");
}
```

- [ ] **Step 8: Run risk score calculation tests**

Run: `cargo test test_risk_score --test risk_assessment_test`
Expected: PASS

- [ ] **Step 9: Write risk-based autonomy adjustment test**

```rust
#[test]
fn test_autonomy_adjustment_low_risk() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::ReadFile,
        path: Some("/tmp/test.txt".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
        intervention_history: vec![],
    };

    let risk_score = risk_assessor.compute_risk_score(&profile);
    let adjusted_level = risk_assessor.adjust_autonomy_level(&profile);

    // Low risk should not reduce autonomy
    assert!(adjusted_level == crate::autonomy::contract::AutonomyLevel::Bounded ||
            adjusted_level == crate::autonomy::contract::AutonomyLevel::Supervised);
}

#[test]
fn test_autonomy_adjustment_high_risk() {
    let risk_assessor = RiskAssessor::new();

    let profile = RiskProfile {
        action_type: crate::autonomy::scope::ActionType::DeleteFile,
        path: Some("/etc/passwd".to_string()),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
        intervention_history: vec![
            crate::autonomy::risk::InterventionRecord {
                timestamp: std::time::SystemTime::now(),
                reason: "Previous failure".to_string(),
            },
        ],
    };

    let risk_score = risk_assessor.compute_risk_score(&profile);
    let adjusted_level = risk_assessor.adjust_autonomy_level(&profile);

    // High risk should reduce autonomy
    assert!(adjusted_level == crate::autonomy::contract::AutonomyLevel::Bounded ||
            adjusted_level == crate::autonomy::contract::AutonomyLevel::Manual);
}
```

- [ ] **Step 10: Run risk-based autonomy adjustment tests**

Run: `cargo test test_autonomy_adjustment --test risk_assessment_test`
Expected: PASS

- [ ] **Step 11: Commit UT7 tests**

```bash
git add tests/unit/risk_assessment_test.rs
git commit -m "test(UT7): add risk assessment model unit tests"
```

---

## UT8: Confidence Threshold Computation Unit Tests

**Files:**
- Create: `tests/unit/confidence_threshold_test.rs`

**Coverage:**
- Confidence threshold computation
- Threshold-based blocking
- Action safety classification
- Autonomy-level-specific thresholds

- [ ] **Step 1: Write confidence threshold computation test**

```rust
use yaml_to_rust_agentsdk::autonomy::confidence::{
    ConfidenceEvaluator, ConfidenceThreshold, ActionContext, ActionType
};

#[test]
fn test_confidence_threshold_manual_level() {
    let config = ConfidenceThreshold {
        safe: 1.0,
        low_risk: 0.9,
        medium_risk: 0.8,
        high_risk: 0.7,
    };

    assert_eq!(config.safe, 1.0);
    assert_eq!(config.low_risk, 0.9);
    assert_eq!(config.medium_risk, 0.8);
    assert_eq!(config.high_risk, 0.7);
}

#[test]
fn test_confidence_threshold_bounded_level() {
    let config = ConfidenceThreshold {
        safe: 1.0,
        low_risk: 0.85,
        medium_risk: 0.7,
        high_risk: 0.6,
    };

    assert_eq!(config.safe, 1.0);
    assert_eq!(config.low_risk, 0.85);
    assert_eq!(config.medium_risk, 0.7);
    assert_eq!(config.high_risk, 0.6);
}
```

- [ ] **Step 2: Run confidence threshold tests**

Run: `cargo test test_confidence_threshold --test confidence_threshold_test`
Expected: PASS

- [ ] **Step 3: Write threshold-based blocking test**

```rust
#[tokio::test]
async fn test_threshold_blocking_high_confidence() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result = evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    assert!(!result.is_blocked);
    assert!(result.confidence >= 0.7);
}

#[tokio::test]
async fn test_threshold_blocking_low_confidence() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result = evaluator.evaluate_action(ActionType::DeleteFile, &context).await;

    assert!(result.is_blocked);
    assert!(result.confidence < 0.6);
}
```

- [ ] **Step 4: Run threshold blocking tests**

Run: `cargo test test_threshold_blocking --test confidence_threshold_test`
Expected: PASS

- [ ] **Step 5: Write action safety classification test**

```rust
#[tokio::test]
async fn test_action_safety_safe() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result = evaluator.evaluate_action(ActionType::ReadFile, &context).await;

    assert!(matches!(result.action_safety, crate::autonomy::confidence::ActionSafety::Safe));
}

#[tokio::test]
async fn test_action_safety_low_risk() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result = evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    assert!(matches!(result.action_safety, crate::autonomy::confidence::ActionSafety::Safe |
                                     crate::autonomy::confidence::ActionSafety::LowRisk));
}

#[tokio::test]
async fn test_action_safety_dangerous() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result = evaluator.evaluate_action(ActionType::DeleteFile, &context).await;

    assert!(matches!(result.action_safety, crate::autonomy::confidence::ActionSafety::Dangerous));
}
```

- [ ] **Step 6: Run action safety classification tests**

Run: `cargo test test_action_safety --test confidence_threshold_test`
Expected: PASS

- [ ] **Step 7: Write autonomy-level-specific threshold test**

```rust
#[tokio::test]
async fn test_autonomy_level_specific_thresholds() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Manual,
    };

    let result_manual = evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    let context_bounded = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result_bounded = evaluator.evaluate_action(ActionType::WriteFile, &context_bounded).await;

    // Manual level should have higher confidence (more conservative)
    assert!(result_manual.confidence >= result_bounded.confidence);
}
```

- [ ] **Step 8: Run autonomy-level-specific threshold tests**

Run: `cargo test test_autonomy_level_specific --test confidence_threshold_test`
Expected: PASS

- [ ] **Step 9: Commit UT8 tests**

```bash
git add tests/unit/confidence_threshold_test.rs
git commit -m "test(UT8): add confidence threshold computation unit tests"
```

---

## Summary

All 8 unit test suites have been defined with comprehensive test coverage:

1. **UT1**: Contract Parsing/Validation - 9 steps
2. **UT2**: Metrics Types - 9 steps
3. **UT3**: Override Event Handling - 11 steps
4. **UT4**: Intervention Event Structure - 9 steps
5. **UT5**: Stop Condition Evaluation - 11 steps
6. **UT6**: Checkpoint Generation - 9 steps
7. **UT7**: Risk Assessment Model - 11 steps
8. **UT8**: Confidence Threshold Computation - 9 steps

Each unit test suite includes:
- Exact function signatures
- Specific assertions
- Cargo test commands
- Clear pass/fail criteria
- Comprehensive coverage of component functionality
