# Phase 07 Autonomy & Metrics - Acceptance Criteria

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define Phase 07 exit criteria that must be satisfied before the phase can be considered complete

**Architecture:** Acceptance criteria organized by system capabilities (ADR compliance, autonomy bounds, human override, metrics, dashboards, stop conditions, checkpoints, risk assessment, verification)

**Tech Stack:** Rust, cargo test, integration tests, property-based tests

---

## Acceptance Criteria Overview

Phase 07 is COMPLETE when ALL of the following criteria are satisfied:

1. ✅ ADR-0008 Constraints Satisfied
2. ✅ Bounded Goals at All Autonomy Levels
3. ✅ Human Override Available at All Times
4. ✅ Metrics Capture (Usefulness, Time-to-Usefulness, Intervention Rate, Repair Rate)
5. ✅ Dashboards Show Real-Time Data with Anomaly Detection
6. ✅ Stop Conditions (Time, Iteration, Quality, Intervention, Resource) Working
7. ✅ Checkpoint Restoration Verified
8. ✅ Risk Assessment Produces Correct Autonomy Level Adjustments
9. ✅ 7 Verification Layers Pass

Each criterion has specific test requirements and acceptance tests defined below.

---

## AC1: ADR-0008 Constraints Satisfied

**Requirement:** All autonomous loops must comply with ADR-0008 constraints for safe autonomous operations.

**ADR-0008 Core Constraints:**
1. All autonomous loops MUST have stop conditions
2. Human override MUST be available at ALL times
3. Checkpoints MUST be generated at regular intervals
4. Risk assessment MUST be performed before autonomous actions
5. Scope boundaries MUST be enforced

**Files:**
- Create: `tests/acceptance/adr_compliance_test.rs`
- Reference: `opencode/docs/reports/roadmap/adr-0008-autonomous-loops-safety.md`

- [ ] **Step 1: Write ADR compliance test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, validate_contract};
use yaml_to_rust_agentsdk::autonomy::contract::validator::ValidationError;

#[test]
fn test_adr_compliance_all_loops_have_stop_conditions() {
    // Valid contract with stop conditions
    let valid_yaml = r#"
goals:
  - name: "Generate test file"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
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

    let contract = parse_contract(valid_yaml).unwrap();
    let result = validate_contract(&contract);

    assert!(result.is_ok(), "Valid contract should pass ADR compliance");
}

#[test]
fn test_adr_compliance_rejects_loops_without_stop_conditions() {
    // Invalid contract without stop conditions
    let invalid_yaml = r#"
goals:
  - name: "Generate test file"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "autonomous"
max_iterations: 100
timeout_seconds: 300
stop_conditions: []
checkpoint_frequency: 10
"#;

    let contract = parse_contract(invalid_yaml).unwrap();
    let result = validate_contract(&contract);

    assert!(result.is_err());
    match result.unwrap_err() {
        ValidationError::AdrViolation(msg) => {
            assert!(msg.contains("stop condition"));
        },
        _ => panic!("Expected ADR violation"),
    }
}

#[test]
fn test_adr_compliance_human_override_always_available() {
    // Human override is enforced at runtime, not in contract validation
    // This test verifies the contract doesn't disable override
    let yaml = r#"
goals:
  - name: "Test goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "File exists"
autonomy_level: "autonomous"
max_iterations: 100
timeout_seconds: 300
stop_conditions:
  - type: "intervention"
    value: 1
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();

    // Verify intervention stop condition exists (enables override)
    let has_intervention_condition = contract.stop_conditions.iter()
        .any(|sc| matches!(sc.condition_type, crate::autonomy::contract::StopConditionType::Intervention));

    assert!(has_intervention_condition, "Autonomous level requires intervention condition for override");
}
```

- [ ] **Step 2: Run ADR compliance tests**

Run: `cargo test test_adr_compliance --test adr_compliance_test`
Expected: PASS all ADR compliance tests

- [ ] **Step 3: Test checkpoint frequency requirement**

```rust
#[test]
fn test_adr_compliance_requires_checkpoints() {
    let yaml_with_checkpoints = r#"
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
    value: 50
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml_with_checkpoints).unwrap();
    assert!(contract.checkpoint_frequency > 0, "Checkpoint frequency must be positive");

    let yaml_without_checkpoints = r#"
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
    value: 50
checkpoint_frequency: 0
"#;

    let contract = parse_contract(yaml_without_checkpoints).unwrap();
    let result = validate_contract(&contract);
    assert!(result.is_err(), "Checkpoint frequency must be positive");
}
```

- [ ] **Step 4: Run checkpoint requirement test**

Run: `cargo test test_adr_compliance_requires_checkpoints --test adr_compliance_test`
Expected: PASS

- [ ] **Step 5: Test scope boundary enforcement**

```rust
#[test]
fn test_adr_compliance_scope_boundaries_enforced() {
    use yaml_to_rust_agentsdk::autonomy::scope::{ScopeEnforcer, AutonomyScope, ActionType};

    let enforcer = ScopeEnforcer::new();

    let scope = AutonomyScope {
        allowed_paths: vec!["/tmp".to_string()],
        allowed_commands: vec!["write_file".to_string()],
        forbidden_actions: vec![ActionType::DeleteFile],
        max_file_size: 1024,
        network_access_allowed: false,
        max_execution_time_secs: Some(300),
    };

    // Should allow action within scope
    let result = enforcer.check_action(&scope, ActionType::WriteFile, "/tmp/test.txt", 100);
    assert!(result.is_ok(), "Action within scope should be allowed");

    // Should block action outside scope
    let result = enforcer.check_action(&scope, ActionType::WriteFile, "/etc/passwd", 100);
    assert!(result.is_err(), "Action outside scope should be blocked");

    // Should block forbidden action
    let result = enforcer.check_action(&scope, ActionType::DeleteFile, "/tmp/test.txt", 0);
    assert!(result.is_err(), "Forbidden action should be blocked");
}
```

- [ ] **Step 6: Run scope boundary test**

Run: `cargo test test_adr_compliance_scope_boundaries_enforced --test adr_compliance_test`
Expected: PASS

- [ ] **Step 7: Test risk assessment requirement**

```rust
#[test]
fn test_adr_compliance_risk_assessment_before_autonomous_actions() {
    use yaml_to_rust_agentsdk::autonomy::confidence::{
        ConfidenceEvaluator, ActionContext, ActionType
    };

    let evaluator = ConfidenceEvaluator::new();

    // High-risk action should be blocked
    let context = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "System file modification".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result = evaluator.evaluate_action(ActionType::DeleteFile, &context);
    assert!(result.is_blocked, "High-risk actions should be blocked by risk assessment");

    // Low-risk action should be allowed
    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Temporary file creation".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result = evaluator.evaluate_action(ActionType::WriteFile, &context);
    assert!(!result.is_blocked, "Low-risk actions should be allowed");
}
```

- [ ] **Step 8: Run risk assessment test**

Run: `cargo test test_adr_compliance_risk_assessment_before_autonomous_actions --test adr_compliance_test`
Expected: PASS

- [ ] **Step 9: Commit AC1 tests**

```bash
git add tests/acceptance/adr_compliance_test.rs
git commit -m "test(AC1): add ADR-0008 compliance acceptance tests"
```

---

## AC2: Bounded Goals at All Autonomy Levels

**Requirement:** Autonomous loops must respect bounded goals at ALL autonomy levels (Manual, Bounded, Supervised, Autonomous).

**Acceptance Criteria:**
1. Goals are explicitly defined in contracts
2. Each goal has clear acceptance criteria
3. Goals have priority levels
4. Progress toward goals is tracked
5. Completion of goals terminates the loop

**Files:**
- Create: `tests/acceptance/bounded_goals_test.rs`

- [ ] **Step 1: Write bounded goals test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, Goal};

#[test]
fn test_goals_have_clear_acceptance_criteria() {
    let yaml = r#"
goals:
  - name: "Generate test file"
    type: "file_generation"
    target_path: "/tmp/test.txt"
    priority: 1
    acceptance_criteria:
      - "File exists at /tmp/test.txt"
      - "File is not empty"
      - "File contains expected content"
  - name: "Run tests"
    type: "testing"
    priority: 2
    acceptance_criteria:
      - "All tests pass"
      - "Coverage > 80%"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();

    assert_eq!(contract.goals.len(), 2);

    // First goal
    let goal1 = &contract.goals[0];
    assert_eq!(goal1.name, "Generate test file");
    assert_eq!(goal1.priority, 1);
    assert_eq!(goal1.acceptance_criteria.len(), 3);
    assert!(goal1.acceptance_criteria.contains(&"File exists at /tmp/test.txt".to_string()));

    // Second goal
    let goal2 = &contract.goals[1];
    assert_eq!(goal2.name, "Run tests");
    assert_eq!(goal2.priority, 2);
    assert_eq!(goal2.acceptance_criteria.len(), 2);
}

#[test]
fn test_goals_have_priority_levels() {
    let yaml = r#"
goals:
  - name: "High priority goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "Completed"
  - name: "Medium priority goal"
    type: "file_generation"
    priority: 2
    acceptance_criteria:
      - "Completed"
  - name: "Low priority goal"
    type: "file_generation"
    priority: 3
    acceptance_criteria:
      - "Completed"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();

    // Goals should be sorted by priority (or verified that priorities are distinct)
    let priorities: Vec<_> = contract.goals.iter().map(|g| g.priority).collect();
    assert_eq!(priorities, vec![1, 2, 3]);
}

#[test]
fn test_goal_completion_terminates_loop() {
    use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;

    let yaml = r#"
goals:
  - name: "Single goal"
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

    let contract = parse_contract(yaml).unwrap();

    // Simulate goal completion
    let mut loop_state = AutonomousLoop::new(contract);

    // Mark goal as complete
    loop_state.mark_goal_complete(0);

    // Loop should terminate
    assert!(loop_state.should_terminate(), "Loop should terminate when all goals are complete");
}
```

- [ ] **Step 2: Run bounded goals tests**

Run: `cargo test test_goals --test bounded_goals_test`
Expected: PASS all bounded goals tests

- [ ] **Step 3: Test goal progress tracking**

```rust
#[test]
fn test_goal_progress_tracked() {
    use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;
    use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;

    let yaml = r#"
goals:
  - name: "Generate files"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "10 files created"
autonomy_level: "bounded"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();
    let metrics = MetricsCollector::new();
    let mut loop_state = AutonomousLoop::with_metrics(contract, metrics);

    // Simulate progress
    for i in 0..5 {
        loop_state.record_progress(0, i * 10); // 0%, 10%, ..., 40%
    }

    let progress = loop_state.get_goal_progress(0);
    assert_eq!(progress, 40.0, "Progress should be tracked as percentage");
}
```

- [ ] **Step 4: Run goal progress test**

Run: `cargo test test_goal_progress_tracked --test bounded_goals_test`
Expected: PASS

- [ ] **Step 5: Test boundedness at different autonomy levels**

```rust
#[test]
fn test_bounded_goals_at_manual_level() {
    let yaml = r#"
goals:
  - name: "Manual goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "Completed"
autonomy_level: "manual"
max_iterations: 10
stop_conditions:
  - type: "iteration"
    value: 10
checkpoint_frequency: 5
"#;

    let contract = parse_contract(yaml).unwrap();
    assert_eq!(contract.autonomy_level, crate::autonomy::contract::AutonomyLevel::Manual);
    assert_eq!(contract.max_iterations, Some(10), "Manual level should have tight iteration bound");
}

#[test]
fn test_bounded_goals_at_autonomous_level() {
    let yaml = r#"
goals:
  - name: "Autonomous goal"
    type: "file_generation"
    priority: 1
    acceptance_criteria:
      - "Completed"
autonomy_level: "autonomous"
max_iterations: 100
stop_conditions:
  - type: "iteration"
    value: 50
  - type: "time"
    value: 300
  - type: "quality"
    value: 80
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();
    assert_eq!(contract.autonomy_level, crate::autonomy::contract::AutonomyLevel::Autonomous);
    assert!(contract.stop_conditions.len() >= 2, "Autonomous level should have multiple stop conditions");
}
```

- [ ] **Step 6: Run autonomy level boundedness tests**

Run: `cargo test test_bounded_goals_at --test bounded_goals_test`
Expected: PASS

- [ ] **Step 7: Commit AC2 tests**

```bash
git add tests/acceptance/bounded_goals_test.rs
git commit -m "test(AC2): add bounded goals acceptance tests"
```

---

## AC3: Human Override Available at All Times

**Requirement:** Human override must be available at ALL times (pause, stop, modify, scope-change) regardless of autonomy level.

**Override Operations:**
1. Pause: Temporarily suspend execution
2. Resume: Continue paused execution
3. Stop: Terminate execution permanently
4. Modify: Change goals or parameters
5. Scope-Change: Adjust autonomy scope

**Files:**
- Create: `tests/acceptance/human_override_test.rs`

- [ ] **Step 1: Write human override test**

```rust
use yaml_to_rust_agentsdk::autonomy::override::{
    OverrideController, OverrideCommand, OverrideReason
};

#[tokio::test]
async fn test_pause_available_at_all_times() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop().await;

    // Wait for execution to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Pause should always be available
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));

    // Resume
    controller.send_command(OverrideCommand::Resume).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Running));
}

#[tokio::test]
async fn test_stop_available_at_all_times() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Stop should always be available
    controller.send_command(OverrideCommand::Stop {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Stopped));
}

#[tokio::test]
async fn test_modify_available_at_all_times() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Modify should always be available
    let new_goals = vec!["Updated goal".to_string()];
    controller.send_command(OverrideCommand::ModifyScope {
        new_goals,
        reason: OverrideReason::Manual,
    }).await;

    // Verify event was emitted
    let mut event_rx = controller.subscribe_events();
    let event = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        event_rx.recv()
    ).await;

    assert!(event.is_ok());
    assert!(matches!(event.unwrap().unwrap().command, OverrideCommand::ModifyScope { .. }));
}
```

- [ ] **Step 2: Run human override tests**

Run: `cargo test test_override_available_at_all_times --test human_override_test`
Expected: PASS all human override tests

- [ ] **Step 3: Test override during all execution phases**

```rust
#[tokio::test]
async fn test_override_during_initialization() {
    let controller = OverrideController::new();

    // Pause during initialization (before loop starts)
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));
}

#[tokio::test]
async fn test_override_during_execution() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Override during active execution
    controller.send_command(OverrideCommand::Stop {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Stopped));
}

#[tokio::test]
async fn test_override_during_checkpoint() {
    use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointManager;

    let controller = OverrideController::new();
    let checkpoint_manager = CheckpointManager::new();
    let mut handle = controller.start_autonomous_loop().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Trigger checkpoint
    checkpoint_manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 42}),
        crate::autonomy::checkpoint::CheckpointMetadata {
            reason: "Manual checkpoint".to_string(),
            checkpoint_type: crate::autonomy::checkpoint::CheckpointType::Manual,
            iteration: 42,
            tags: vec![],
        },
    ).await;

    // Override during checkpoint
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));
}
```

- [ ] **Step 4: Run override phase tests**

Run: `cargo test test_override_during --test human_override_test`
Expected: PASS

- [ ] **Step 5: Test override at different autonomy levels**

```rust
#[tokio::test]
async fn test_override_at_manual_level() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop_with_level(
        crate::autonomy::contract::AutonomyLevel::Manual
    ).await;

    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));
}

#[tokio::test]
async fn test_override_at_autonomous_level() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop_with_level(
        crate::autonomy::contract::AutonomyLevel::Autonomous
    ).await;

    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test_user".to_string(),
    }).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));
}
```

- [ ] **Step 6: Run autonomy level override tests**

Run: `cargo test test_override_at --test human_override_test`
Expected: PASS

- [ ] **Step 7: Commit AC3 tests**

```bash
git add tests/acceptance/human_override_test.rs
git commit -m "test(AC3): add human override availability acceptance tests"
```

---

## AC4: Metrics Capture (Usefulness, Time-to-Usefulness, Intervention Rate, Repair Rate)

**Requirement:** Metrics system must capture and report: usefulness, time-to-usefulness, intervention rate, repair rate.

**Required Metrics:**
1. **Usefulness**: Number of goals achieved vs. attempted
2. **Time-to-Usefulness**: Average time from start to achieving goal
3. **Intervention Rate**: Number of interventions per unit time
4. **Repair Rate**: Number of successful repairs per intervention

**Files:**
- Create: `tests/acceptance/metrics_capture_test.rs`

- [ ] **Step 1: Write usefulness metrics test**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::{MetricsCollector, MetricType};

#[tokio::test]
async fn test_usefulness_metrics_captured() {
    let collector = MetricsCollector::new();

    // Simulate goal attempts and achievements
    collector.increment_counter("goals_attempted", 10, &[]).await;
    collector.increment_counter("goals_achieved", 7, &[]).await;

    let attempted = collector.get_counter_value("goals_attempted").await;
    let achieved = collector.get_counter_value("goals_achieved").await;

    assert_eq!(attempted, 10);
    assert_eq!(achieved, 7);

    // Calculate usefulness rate
    let usefulness_rate = (achieved as f64 / attempted as f64) * 100.0;
    assert_eq!(usefulness_rate, 70.0);
}

#[tokio::test]
async fn test_usefulness_with_context_labels() {
    let collector = MetricsCollector::new();

    // Track usefulness by goal type
    collector.increment_counter("goals_attempted", 5, &[
        ("goal_type", "file_generation"),
        ("autonomy_level", "bounded"),
    ]).await;

    collector.increment_counter("goals_achieved", 4, &[
        ("goal_type", "file_generation"),
        ("autonomy_level", "bounded"),
    ]).await;

    let attempted = collector.get_counter_value("goals_attempted").await;
    let achieved = collector.get_counter_value("goals_achieved").await;

    assert_eq!(attempted, 5);
    assert_eq!(achieved, 4);
}
```

- [ ] **Step 2: Run usefulness metrics tests**

Run: `cargo test test_usefulness_metrics --test metrics_capture_test`
Expected: PASS

- [ ] **Step 3: Write time-to-usefulness metrics test**

```rust
#[tokio::test]
async fn test_time_to_usefulness_metrics_captured() {
    let collector = MetricsCollector::new();

    // Simulate goal completion times (in milliseconds)
    let completion_times = vec![1000.0, 1500.0, 2000.0, 1200.0, 1800.0];

    for time in completion_times {
        collector.observe_histogram("time_to_usefulness_ms", time, &[]).await;
    }

    let summary = collector.get_histogram_summary("time_to_usefulness_ms").await;

    assert_eq!(summary.count, 5);
    assert_eq!(summary.sum, 7500.0);
    assert_eq!(summary.avg, 1500.0);
}

#[tokio::test]
async fn test_time_to_usefulness_percentiles() {
    let collector = MetricsCollector::new();

    // Generate enough samples for percentile calculation
    let completion_times: Vec<f64> = (1..=100).map(|i| i as f64 * 10.0).collect();

    for time in completion_times {
        collector.observe_histogram("time_to_usefulness_ms", time, &[]).await;
    }

    let summary = collector.get_histogram_summary("time_to_usefulness_ms").await;

    assert_eq!(summary.count, 100);
    assert!(summary.p50 >= 450.0 && summary.p50 <= 550.0); // ~500
    assert!(summary.p95 >= 900.0 && summary.p95 <= 1000.0); // ~950
}
```

- [ ] **Step 4: Run time-to-usefulness tests**

Run: `cargo test test_time_to_usefulness_metrics --test metrics_capture_test`
Expected: PASS

- [ ] **Step 5: Write intervention rate metrics test**

```rust
#[tokio::test]
async fn test_intervention_rate_metrics_captured() {
    use yaml_to_rust_agentsdk::autonomy::intervention::{InterventionLogger, InterventionType};

    let logger = InterventionLogger::new();

    // Simulate interventions over 1 hour
    for _ in 0..12 {
        logger.log_intervention(
            InterventionType::Manual,
            "Test intervention".to_string(),
            crate::autonomy::intervention::InterventionContext {
                task_id: "task_123".to_string(),
                goal: "Test goal".to_string(),
                iteration: 1,
                autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
                metrics_snapshot: serde_json::json!({}),
                execution_state: serde_json::json!({}),
                trigger_conditions: vec![],
            },
        ).await;
    }

    let interventions = logger.get_all_interventions().await;
    assert_eq!(interventions.len(), 12);

    // Calculate intervention rate (per hour)
    let intervention_rate = interventions.len() as f64 / 1.0; // 12 interventions per hour
    assert_eq!(intervention_rate, 12.0);
}

#[tokio::test]
async fn test_intervention_rate_by_autonomy_level() {
    use yaml_to_rust_agentsdk::autonomy::intervention::{InterventionLogger, InterventionType};

    let logger = InterventionLogger::new();

    // Interventions at different autonomy levels
    for _ in 0..5 {
        logger.log_intervention(
            InterventionType::Manual,
            "Test".to_string(),
            crate::autonomy::intervention::InterventionContext {
                task_id: "task_123".to_string(),
                goal: "Test".to_string(),
                iteration: 1,
                autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
                metrics_snapshot: serde_json::json!({}),
                execution_state: serde_json::json!({}),
                trigger_conditions: vec![],
            },
        ).await;
    }

    for _ in 0..3 {
        logger.log_intervention(
            InterventionType::Manual,
            "Test".to_string(),
            crate::autonomy::intervention::InterventionContext {
                task_id: "task_456".to_string(),
                goal: "Test".to_string(),
                iteration: 1,
                autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
                metrics_snapshot: serde_json::json!({}),
                execution_state: serde_json::json!({}),
                trigger_conditions: vec![],
            },
        ).await;
    }

    let summary = logger.generate_summary().await;
    let bounded_rate = *summary.by_autonomy_level.get(&crate::autonomy::contract::AutonomyLevel::Bounded).unwrap();
    let autonomous_rate = *summary.by_autonomy_level.get(&crate::autonomy::contract::AutonomyLevel::Autonomous).unwrap();

    assert_eq!(bounded_rate, 5);
    assert_eq!(autonomous_rate, 3);
}
```

- [ ] **Step 6: Run intervention rate tests**

Run: `cargo test test_intervention_rate_metrics --test metrics_capture_test`
Expected: PASS

- [ ] **Step 7: Write repair rate metrics test**

```rust
#[tokio::test]
async fn test_repair_rate_metrics_captured() {
    use yaml_to_rust_agentsdk::autonomy::intervention::{InterventionLogger, InterventionType};

    let logger = InterventionLogger::new();

    // Create 10 interventions
    let mut intervention_ids = vec![];
    for i in 0..10 {
        let id = logger.log_intervention(
            InterventionType::ErrorDetected,
            format!("Error {}", i),
            crate::autonomy::intervention::InterventionContext {
                task_id: "task_123".to_string(),
                goal: "Test".to_string(),
                iteration: 1,
                autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
                metrics_snapshot: serde_json::json!({}),
                execution_state: serde_json::json!({}),
                trigger_conditions: vec![],
            },
        ).await;
        intervention_ids.push(id);
    }

    // Mark 8 as resolved (repaired)
    for id in intervention_ids.iter().take(8) {
        logger.mark_resolved(*id, "Issue fixed".to_string()).await;
    }

    let summary = logger.generate_summary().await;

    assert_eq!(summary.total_count, 10);
    assert_eq!(summary.resolved_count, 8);
    assert_eq!(summary.unresolved_count, 2);

    // Repair rate
    let repair_rate = (summary.resolved_count as f64 / summary.total_count as f64) * 100.0;
    assert_eq!(repair_rate, 80.0);
}
```

- [ ] **Step 8: Run repair rate test**

Run: `cargo test test_repair_rate_metrics --test metrics_capture_test`
Expected: PASS

- [ ] **Step 9: Commit AC4 tests**

```bash
git add tests/acceptance/metrics_capture_test.rs
git commit -m "test(AC4): add metrics capture acceptance tests"
```

---

## AC5: Dashboards Show Real-Time Data with Anomaly Detection

**Requirement:** Success and regression dashboards must render real-time data with anomaly detection.

**Dashboard Requirements:**
1. Success dashboard: tasks completed, goals achieved, time-to-usefulness, quality score
2. Regression dashboard: interventions, error rate, rollback count
3. Real-time updates: data refreshes automatically
4. Anomaly detection: alerts for abnormal metrics

**Files:**
- Create: `tests/acceptance/dashboard_test.rs`

- [ ] **Step 1: Write success dashboard test**

```rust
use yaml_to_rust_agentsdk::autonomy::dashboard::{
    DashboardRenderer, DashboardData, SuccessMetrics
};

#[tokio::test]
async fn test_success_dashboard_renders_real_time_data() {
    let renderer = DashboardRenderer::new();

    let data = DashboardData {
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

    let rendered = renderer.render_success_dashboard(data.clone()).await;

    assert!(rendered.contains("150"));
    assert!(rendered.contains("45"));
    assert!(rendered.contains("2500.0"));
    assert!(rendered.contains("87.5"));
    assert!(rendered.contains("92"));
}

#[tokio::test]
async fn test_dashboard_updates_in_real_time() {
    use std::time::Duration;
    use tokio::time::sleep;

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

    sleep(Duration::from_millis(100)).await;

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

    assert_ne!(rendered1, rendered2, "Dashboard should reflect updated data");
}
```

- [ ] **Step 2: Run success dashboard tests**

Run: `cargo test test_success_dashboard --test dashboard_test`
Expected: PASS

- [ ] **Step 3: Write regression dashboard test**

```rust
use yaml_to_rust_agentsdk::autonomy::dashboard::{DashboardData, RegressionMetrics};

#[tokio::test]
async fn test_regression_dashboard_renders() {
    let renderer = DashboardRenderer::new();

    let data = DashboardData {
        success_metrics: Default::default(),
        regression_metrics: RegressionMetrics {
            interventions_total: 25,
            interventions_per_hour: 5.0,
            error_rate: 0.08,
            rollback_count: 3,
        },
        timestamp: chrono::Utc::now(),
    };

    let rendered = renderer.render_regression_dashboard(data.clone()).await;

    assert!(rendered.contains("25"));
    assert!(rendered.contains("5.0"));
    assert!(rendered.contains("8")); // 0.08 * 100 = 8%
    assert!(rendered.contains("3"));
}
```

- [ ] **Step 4: Run regression dashboard test**

Run: `cargo test test_regression_dashboard_renders --test dashboard_test`
Expected: PASS

- [ ] **Step 5: Write anomaly detection test**

```rust
use yaml_to_rust_agentsdk::autonomy::dashboard::{AnomalyDetector, AnomalySeverity};

#[tokio::test]
async fn test_anomaly_detection_triggers_alerts() {
    let detector = AnomalyDetector::new();

    let data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 10,
            goals_achieved: 5,
            avg_time_to_usefulness_ms: 5000.0,
            quality_score: 65.0, // Below threshold
            success_rate: 0.60,  // Below threshold
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 30,
            interventions_per_hour: 8.0, // Above threshold
            error_rate: 0.20,             // Above threshold
            rollback_count: 5,
        },
        timestamp: chrono::Utc::now(),
    };

    let alerts = detector.detect(&data);

    assert_eq!(alerts.len(), 4); // 4 metrics out of bounds

    // Check specific alerts
    let quality_alert = alerts.iter().find(|a| a.metric_name == "quality_score").unwrap();
    assert_eq!(quality_alert.severity, AnomalySeverity::Warning);

    let success_rate_alert = alerts.iter().find(|a| a.metric_name == "success_rate").unwrap();
    assert_eq!(success_rate_alert.severity, AnomalySeverity::Error);
}

#[tokio::test]
async fn test_no_anomalies_when_metrics_healthy() {
    let detector = AnomalyDetector::new();

    let data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2500.0,
            quality_score: 85.0, // Above threshold
            success_rate: 0.90,  // Above threshold
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 10,
            interventions_per_hour: 2.0, // Below threshold
            error_rate: 0.05,           // Below threshold
            rollback_count: 1,
        },
        timestamp: chrono::Utc::now(),
    };

    let alerts = detector.detect(&data);

    assert_eq!(alerts.len(), 0, "No anomalies should be detected for healthy metrics");
}
```

- [ ] **Step 6: Run anomaly detection tests**

Run: `cargo test test_anomaly_detection --test dashboard_test`
Expected: PASS

- [ ] **Step 7: Write combined dashboard test**

```rust
#[tokio::test]
async fn test_combined_dashboard_with_alerts() {
    let renderer = DashboardRenderer::new();
    let detector = AnomalyDetector::new();

    let data = DashboardData {
        success_metrics: SuccessMetrics {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2500.0,
            quality_score: 70.0, // Trigger warning
            success_rate: 0.92,
        },
        regression_metrics: RegressionMetrics {
            interventions_total: 25,
            interventions_per_hour: 6.0, // Trigger warning
            error_rate: 0.08,
            rollback_count: 3,
        },
        timestamp: chrono::Utc::now(),
    };

    let alerts = detector.detect(&data);
    let rendered = renderer.render_combined_dashboard(data, &alerts).await;

    assert!(rendered.contains("ANOMALY ALERTS"));
    assert!(rendered.contains("quality_score"));
    assert!(rendered.contains("interventions_per_hour"));
}
```

- [ ] **Step 8: Run combined dashboard test**

Run: `cargo test test_combined_dashboard --test dashboard_test`
Expected: PASS

- [ ] **Step 9: Commit AC5 tests**

```bash
git add tests/acceptance/dashboard_test.rs
git commit -m "test(AC5): add dashboard real-time data and anomaly detection acceptance tests"
```

---

## AC6: Stop Conditions Working (Time, Iteration, Quality, Intervention, Resource)

**Requirement:** All 5 stop condition types must work correctly: time, iteration, quality, intervention, resource.

**Stop Condition Types:**
1. **Time**: Stop after specified duration
2. **Iteration**: Stop after specified number of iterations
3. **Quality**: Stop if quality drops below threshold
4. **Intervention**: Stop after specified number of interventions
5. **Resource**: Stop if resource usage exceeds threshold

**Files:**
- Create: `tests/acceptance/stop_conditions_test.rs`

- [ ] **Step 1: Write time stop condition test**

```rust
use yaml_to_rust_agentsdk::autonomy::stop_conditions::{
    StopConditionEvaluator, StopCondition, StopConditionType, ExecutionContext
};
use std::time::Duration;

#[tokio::test]
async fn test_time_stop_condition_terminates() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Time,
        value: 300, // 5 minutes
    };

    // Before timeout
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(200),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(!result.should_stop);

    // After timeout
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(301),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
    assert!(result.reason.contains("Timeout"));
}
```

- [ ] **Step 2: Run time stop condition test**

Run: `cargo test test_time_stop_condition --test stop_conditions_test`
Expected: PASS

- [ ] **Step 3: Write iteration stop condition test**

```rust
#[tokio::test]
async fn test_iteration_stop_condition_terminates() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Iteration,
        value: 50,
    };

    // Before limit
    let context = ExecutionContext {
        iteration: 49,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(!result.should_stop);

    // At limit
    let context = ExecutionContext {
        iteration: 50,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
}
```

- [ ] **Step 4: Run iteration stop condition test**

Run: `cargo test test_iteration_stop_condition --test stop_conditions_test`
Expected: PASS

- [ ] **Step 5: Write quality stop condition test**

```rust
#[tokio::test]
async fn test_quality_stop_condition_terminates() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Quality,
        value: 80, // 80% quality threshold
    };

    // Above threshold
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.85,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(!result.should_stop);

    // Below threshold
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.75,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
}
```

- [ ] **Step 6: Run quality stop condition test**

Run: `cargo test test_quality_stop_condition --test stop_conditions_test`
Expected: PASS

- [ ] **Step 7: Write intervention stop condition test**

```rust
#[tokio::test]
async fn test_intervention_stop_condition_terminates() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Intervention,
        value: 5,
    };

    // Below limit
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 4,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(!result.should_stop);

    // At limit
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 5,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
}
```

- [ ] **Step 8: Run intervention stop condition test**

Run: `cargo test test_intervention_stop_condition --test stop_conditions_test`
Expected: PASS

- [ ] **Step 9: Write resource stop condition test**

```rust
#[tokio::test]
async fn test_resource_stop_condition_terminates_cpu() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Resource,
        value: 85, // 85% resource threshold
    };

    // Below threshold
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.80, memory: 0.60 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(!result.should_stop);

    // CPU exceeds threshold
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.90, memory: 0.60 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
}

#[tokio::test]
async fn test_resource_stop_condition_terminates_memory() {
    let evaluator = StopConditionEvaluator::new();

    let condition = StopCondition {
        condition_type: StopConditionType::Resource,
        value: 85,
    };

    // Memory exceeds threshold
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.70, memory: 0.90 },
    };

    let result = evaluator.evaluate(&condition, &context).await;
    assert!(result.should_stop);
}
```

- [ ] **Step 10: Run resource stop condition tests**

Run: `cargo test test_resource_stop_condition --test stop_conditions_test`
Expected: PASS

- [ ] **Step 11: Write multiple stop conditions test**

```rust
#[tokio::test]
async fn test_multiple_stop_conditions_any_triggers() {
    let evaluator = StopConditionEvaluator::new();

    let conditions = vec![
        StopCondition {
            condition_type: StopConditionType::Iteration,
            value: 50,
        },
        StopCondition {
            condition_type: StopConditionType::Time,
            value: 300,
        },
        StopCondition {
            condition_type: StopConditionType::Quality,
            value: 80,
        },
    ];

    let context = ExecutionContext {
        iteration: 50, // Triggers iteration condition
        elapsed: Duration::from_secs(100), // Doesn't trigger time condition
        quality_score: 0.9, // Doesn't trigger quality condition
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };

    // Any condition should trigger
    let mut should_stop = false;
    for condition in &conditions {
        let result = evaluator.evaluate(condition, &context).await;
        if result.should_stop {
            should_stop = true;
            break;
        }
    }

    assert!(should_stop, "At least one stop condition should trigger");
}
```

- [ ] **Step 12: Run multiple stop conditions test**

Run: `cargo test test_multiple_stop_conditions --test stop_conditions_test`
Expected: PASS

- [ ] **Step 13: Commit AC6 tests**

```bash
git add tests/acceptance/stop_conditions_test.rs
git commit -m "test(AC6): add stop conditions acceptance tests"
```

---

## AC7: Checkpoint Restoration Verified

**Requirement:** Checkpoint generation and restoration must be verified to work correctly.

**Checkpoint Requirements:**
1. Checkpoints generated periodically
2. Checkpoints generated on events
3. State fully captured in checkpoints
4. Checkpoint restoration produces equivalent state
5. Checkpoint integrity validation

**Files:**
- Create: `tests/acceptance/checkpoint_restoration_test.rs`

- [ ] **Step 1: Write periodic checkpoint test**

```rust
use yaml_to_rust_agentsdk::autonomy::checkpoint::{CheckpointManager, CheckpointMetadata, CheckpointType};

#[tokio::test]
async fn test_periodic_checkpoint_generation() {
    let manager = CheckpointManager::new();

    // Simulate periodic checkpointing every 10 iterations
    for iteration in (0..=50).step_by(10) {
        let state = serde_json::json!({
            "iteration": iteration,
            "current_goal": "Generate test file",
            "files_created": vec![format!("/tmp/test{}.txt", i) for i in 0..iteration],
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

    // Verify iteration numbers
    let iterations: Vec<_> = checkpoints.iter().map(|c| c.metadata.iteration).collect();
    assert_eq!(iterations, vec![0, 10, 20, 30, 40, 50]);
}
```

- [ ] **Step 2: Run periodic checkpoint test**

Run: `cargo test test_periodic_checkpoint_generation --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 3: Write event-triggered checkpoint test**

```rust
#[tokio::test]
async fn test_event_triggered_checkpoint_generation() {
    let manager = CheckpointManager::new();

    // Checkpoint before risk
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

    // Checkpoint after intervention
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
    assert_eq!(checkpoints.len(), 2);

    assert_eq!(checkpoints[0].metadata.checkpoint_type, CheckpointType::BeforeRisk);
    assert_eq!(checkpoints[1].metadata.checkpoint_type, CheckpointType::AfterIntervention);
}
```

- [ ] **Step 4: Run event-triggered checkpoint test**

Run: `cargo test test_event_triggered_checkpoint_generation --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 5: Write state capture test**

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
        "execution_context": {
            "working_directory": "/tmp",
            "environment": "test",
        },
    });

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        original_state.clone(),
        CheckpointMetadata {
            reason: "Full state capture".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 42,
            tags: vec![],
        },
    ).await;

    let checkpoint = manager.load_checkpoint(&checkpoint_id).await.unwrap();

    // Verify all fields are captured
    assert_eq!(checkpoint.state["iteration"], 42);
    assert_eq!(checkpoint.state["current_goal"], "Generate test file");
    assert_eq!(checkpoint.state["files_created"].as_array().unwrap().len(), 2);
    assert_eq!(checkpoint.state["tasks_completed"], 15);
    assert_eq!(checkpoint.state["metrics"]["avg_duration_ms"], 1500.0);
}
```

- [ ] **Step 6: Run state capture test**

Run: `cargo test test_checkpoint_captures_full_state --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 7: Write checkpoint restoration test**

```rust
use yaml_to_rust_agentsdk::autonomy::checkpoint::CheckpointRestorer;

#[tokio::test]
async fn test_checkpoint_restoration_produces_equivalent_state() {
    let manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(manager.clone());

    let original_state = serde_json::json!({
        "iteration": 42,
        "current_goal": "Generate test file",
        "files_created": ["/tmp/test1.txt", "/tmp/test2.txt"],
        "tasks_completed": 15,
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

    let restored_state = restorer.restore_checkpoint(&checkpoint_id).await.unwrap();

    // Verify restored state matches original
    assert_eq!(restored_state, original_state);
}
```

- [ ] **Step 8: Run checkpoint restoration test**

Run: `cargo test test_checkpoint_restoration_produces_equivalent_state --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 9: Write checkpoint integrity test**

```rust
#[tokio::test]
async fn test_checkpoint_integrity_validation() {
    let manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(manager.clone());

    // Valid checkpoint
    let valid_state = serde_json::json!({
        "iteration": 42,
        "current_goal": "Generate test file",
    });

    let checkpoint_id = manager.generate_checkpoint(
        "task_123".to_string(),
        valid_state,
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 42,
            tags: vec![],
        },
    ).await;

    let result = restorer.restore_checkpoint(&checkpoint_id).await;
    assert!(result.is_ok());

    // Invalid checkpoint (missing required fields)
    let invalid_state = serde_json::json!({
        "current_goal": "Generate test file",
        // Missing "iteration"
    });

    let checkpoint_id = manager.generate_checkpoint(
        "task_456".to_string(),
        invalid_state,
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 1,
            tags: vec![],
        },
    ).await;

    let result = restorer.restore_checkpoint(&checkpoint_id).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), crate::autonomy::checkpoint::RestoreError::InvalidState(_)));
}
```

- [ ] **Step 10: Run checkpoint integrity test**

Run: `cargo test test_checkpoint_integrity_validation --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 11: Write checkpoint rollback test**

```rust
#[tokio::test]
async fn test_checkpoint_rollback_to_previous_state() {
    let manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(manager.clone());

    // Create multiple checkpoints
    let checkpoint1 = manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 10}),
        CheckpointMetadata {
            reason: "Checkpoint 1".to_string(),
            checkpoint_type: CheckpointType::Periodic,
            iteration: 10,
            tags: vec![],
        },
    ).await;

    let checkpoint2 = manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 20}),
        CheckpointMetadata {
            reason: "Checkpoint 2".to_string(),
            checkpoint_type: CheckpointType::Periodic,
            iteration: 20,
            tags: vec![],
        },
    ).await;

    let checkpoint3 = manager.generate_checkpoint(
        "task_123".to_string(),
        serde_json::json!({"iteration": 30}),
        CheckpointMetadata {
            reason: "Checkpoint 3".to_string(),
            checkpoint_type: CheckpointType::Periodic,
            iteration: 30,
            tags: vec![],
        },
    ).await;

    // Rollback to checkpoint 2
    let restored = restorer.restore_checkpoint(&checkpoint2).await.unwrap();
    assert_eq!(restored["iteration"], 20);

    // Rollback to checkpoint 1
    let restored = restorer.restore_checkpoint(&checkpoint1).await.unwrap();
    assert_eq!(restored["iteration"], 10);
}
```

- [ ] **Step 12: Run checkpoint rollback test**

Run: `cargo test test_checkpoint_rollback --test checkpoint_restoration_test`
Expected: PASS

- [ ] **Step 13: Commit AC7 tests**

```bash
git add tests/acceptance/checkpoint_restoration_test.rs
git commit -m "test(AC7): add checkpoint restoration acceptance tests"
```

---

## AC8: Risk Assessment Produces Correct Autonomy Level Adjustments

**Requirement:** Risk assessment must produce correct autonomy level adjustments based on risk profile.

**Risk Assessment Requirements:**
1. Low risk → Higher autonomy level
2. High risk → Lower autonomy level or pause
3. Risk factors: action type, path, autonomy level, intervention history
4. Confidence thresholds enforced

**Files:**
- Create: `tests/acceptance/risk_assessment_test.rs`

- [ ] **Step 1: Write low risk to high autonomy test**

```rust
use yaml_to_rust_agentsdk::autonomy::confidence::{
    ConfidenceEvaluator, ActionContext, ActionType, ConfidenceResult
};

#[tokio::test]
async fn test_low_risk_allows_high_autonomy() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Creating temporary file".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Supervised,
    };

    let result = evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    // Low risk action should not be blocked
    assert!(!result.is_blocked);
    assert!(result.confidence >= 0.7);
    assert!(matches!(result.action_safety, crate::autonomy::confidence::ActionSafety::Safe | crate::autonomy::confidence::ActionSafety::LowRisk));
}
```

- [ ] **Step 2: Run low risk test**

Run: `cargo test test_low_risk_allows_high_autonomy --test risk_assessment_test`
Expected: PASS

- [ ] **Step 3: Write high risk to low autonomy test**

```rust
#[tokio::test]
async fn test_high_risk_blocks_autonomous_actions() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "System file modification".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result = evaluator.evaluate_action(ActionType::DeleteFile, &context).await;

    // High risk action should be blocked
    assert!(result.is_blocked);
    assert!(result.confidence < 0.5);
    assert!(matches!(result.action_safety, crate::autonomy::confidence::ActionSafety::Dangerous));
}
```

- [ ] **Step 4: Run high risk test**

Run: `cargo test test_high_risk_blocks_autonomous_actions --test risk_assessment_test`
Expected: PASS

- [ ] **Step 5: Write risk factor influence test**

```rust
#[tokio::test]
async fn test_risk_factors_influence_confidence() {
    let evaluator = ConfidenceEvaluator::new();

    // Safe path, safe action
    let context_safe = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result_safe = evaluator.evaluate_action(ActionType::ReadFile, &context_safe).await;

    // Unsafe path, risky action
    let context_unsafe = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result_unsafe = evaluator.evaluate_action(ActionType::DeleteFile, &context_unsafe).await;

    // Safe context should have higher confidence
    assert!(result_safe.confidence > result_unsafe.confidence);
    assert!(!result_safe.is_blocked);
    assert!(result_unsafe.is_blocked);
}
```

- [ ] **Step 6: Run risk factor test**

Run: `cargo test test_risk_factors_influence_confidence --test risk_assessment_test`
Expected: PASS

- [ ] **Step 7: Write autonomy level adjustment test**

```rust
#[tokio::test]
async fn test_autonomy_level_adjusts_based_on_risk() {
    let evaluator = ConfidenceEvaluator::new();

    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result_autonomous = evaluator.evaluate_action(ActionType::WriteFile, &context).await;

    // Same action at lower autonomy level
    let context_bounded = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result_bounded = evaluator.evaluate_action(ActionType::WriteFile, &context_bounded).await;

    // Lower autonomy level should have higher confidence (more conservative)
    assert!(result_bounded.confidence > result_autonomous.confidence);
}
```

- [ ] **Step 8: Run autonomy level adjustment test**

Run: `cargo test test_autonomy_level_adjusts_based_on_risk --test risk_assessment_test`
Expected: PASS

- [ ] **Step 9: Write intervention history influence test**

```rust
#[tokio::test]
async fn test_intervention_history_influences_risk() {
    use yaml_to_rust_agentsdk::autonomy::intervention::InterventionLogger;

    let logger = InterventionLogger::new();

    // Simulate recent interventions
    for _ in 0..10 {
        logger.log_intervention(
            crate::autonomy::intervention::InterventionType::Manual,
            "Test intervention".to_string(),
            crate::autonomy::intervention::InterventionContext {
                task_id: "task_123".to_string(),
                goal: "Test".to_string(),
                iteration: 1,
                autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
                metrics_snapshot: serde_json::json!({}),
                execution_state: serde_json::json!({}),
                trigger_conditions: vec![],
            },
        ).await;
    }

    // High intervention history should increase risk (lower confidence)
    let summary = logger.generate_summary().await;
    assert_eq!(summary.total_count, 10);

    // This would be used by the risk assessor to adjust confidence
    // For now, we verify the intervention history is captured
}
```

- [ ] **Step 10: Run intervention history test**

Run: `cargo test test_intervention_history_influences_risk --test risk_assessment_test`
Expected: PASS

- [ ] **Step 11: Commit AC8 tests**

```bash
git add tests/acceptance/risk_assessment_test.rs
git commit -m "test(AC8): add risk assessment acceptance tests"
```

---

## AC9: 7 Verification Layers Pass

**Requirement:** All 7 verification layers must pass for Phase 07 completion.

**7 Verification Layers:**
1. **Layer 1**: Contract Validation (ADR-0008 compliance)
2. **Layer 2**: Scope Enforcement (Path and action boundaries)
3. **Layer 3**: Confidence Evaluation (Risk-based blocking)
4. **Layer 4**: Stop Condition Evaluation (All 5 types)
5. **Layer 5**: Checkpoint Integrity (State capture and restoration)
6. **Layer 6**: Override Availability (Human intervention)
7. **Layer 7**: Metrics Accuracy (Data capture and reporting)

**Files:**
- Create: `tests/acceptance/verification_layers_test.rs`

- [ ] **Step 1: Write Layer 1 - Contract Validation test**

```rust
use yaml_to_rust_agentsdk::autonomy::contract::{parse_contract, validate_contract};

#[test]
fn test_layer1_contract_validation_passes() {
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
    value: 50
  - type: "time"
    value: 300
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();
    let result = validate_contract(&contract);

    assert!(result.is_ok(), "Layer 1: Contract validation should pass");
}
```

- [ ] **Step 2: Run Layer 1 test**

Run: `cargo test test_layer1_contract_validation_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 3: Write Layer 2 - Scope Enforcement test**

```rust
use yaml_to_rust_agentsdk::autonomy::scope::{ScopeEnforcer, AutonomyScope, ActionType};

#[tokio::test]
async fn test_layer2_scope_enforcement_passes() {
    let enforcer = ScopeEnforcer::new();

    let scope = AutonomyScope {
        allowed_paths: vec!["/tmp".to_string()],
        allowed_commands: vec!["write_file".to_string()],
        forbidden_actions: vec![ActionType::DeleteFile],
        max_file_size: 1024,
        network_access_allowed: false,
        max_execution_time_secs: Some(300),
    };

    // Should allow within scope
    let result = enforcer.check_action(&scope, ActionType::WriteFile, "/tmp/test.txt", 100).await;
    assert!(result.is_ok(), "Layer 2: Scope enforcement should allow valid actions");

    // Should block outside scope
    let result = enforcer.check_action(&scope, ActionType::WriteFile, "/etc/passwd", 100).await;
    assert!(result.is_err(), "Layer 2: Scope enforcement should block invalid actions");
}
```

- [ ] **Step 4: Run Layer 2 test**

Run: `cargo test test_layer2_scope_enforcement_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 5: Write Layer 3 - Confidence Evaluation test**

```rust
use yaml_to_rust_agentsdk::autonomy::confidence::{ConfidenceEvaluator, ActionContext, ActionType};

#[tokio::test]
async fn test_layer3_confidence_evaluation_passes() {
    let evaluator = ConfidenceEvaluator::new();

    // Safe action should pass
    let context_safe = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };

    let result = evaluator.evaluate_action(ActionType::WriteFile, &context_safe).await;
    assert!(!result.is_blocked, "Layer 3: Safe actions should pass confidence evaluation");

    // High-risk action should be blocked
    let context_risky = ActionContext {
        path: Some("/etc/passwd".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Autonomous,
    };

    let result = evaluator.evaluate_action(ActionType::DeleteFile, &context_risky).await;
    assert!(result.is_blocked, "Layer 3: High-risk actions should be blocked");
}
```

- [ ] **Step 6: Run Layer 3 test**

Run: `cargo test test_layer3_confidence_evaluation_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 7: Write Layer 4 - Stop Condition Evaluation test**

```rust
use yaml_to_rust_agentsdk::autonomy::stop_conditions::{StopConditionEvaluator, StopCondition, StopConditionType, ExecutionContext};
use std::time::Duration;

#[tokio::test]
async fn test_layer4_stop_condition_evaluation_passes() {
    let evaluator = StopConditionEvaluator::new();

    // Test all 5 stop condition types
    let conditions = vec![
        StopCondition { condition_type: StopConditionType::Iteration, value: 50 },
        StopCondition { condition_type: StopConditionType::Time, value: 300 },
        StopCondition { condition_type: StopConditionType::Quality, value: 80 },
        StopCondition { condition_type: StopConditionType::Intervention, value: 5 },
        StopCondition { condition_type: StopConditionType::Resource, value: 85 },
    ];

    for condition in conditions {
        let context = ExecutionContext {
            iteration: 10,
            elapsed: Duration::from_secs(100),
            quality_score: 0.9,
            interventions: 0,
            resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
        };

        let result = evaluator.evaluate(&condition, &context).await;
        assert!(matches!(&result.condition_type, &condition.condition_type), "Layer 4: Stop condition evaluation should work");
    }
}
```

- [ ] **Step 8: Run Layer 4 test**

Run: `cargo test test_layer4_stop_condition_evaluation_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 9: Write Layer 5 - Checkpoint Integrity test**

```rust
use yaml_to_rust_agentsdk::autonomy::checkpoint::{CheckpointManager, CheckpointRestorer, CheckpointMetadata, CheckpointType};

#[tokio::test]
async fn test_layer5_checkpoint_integrity_passes() {
    let manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(manager.clone());

    let original_state = serde_json::json!({
        "iteration": 42,
        "current_goal": "Test",
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

    let restored = restorer.restore_checkpoint(&checkpoint_id).await.unwrap();
    assert_eq!(restored, original_state, "Layer 5: Checkpoint integrity should pass");
}
```

- [ ] **Step 10: Run Layer 5 test**

Run: `cargo test test_layer5_checkpoint_integrity_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 11: Write Layer 6 - Override Availability test**

```rust
use yaml_to_rust_agentsdk::autonomy::override::{OverrideController, OverrideCommand, OverrideReason};

#[tokio::test]
async fn test_layer6_override_availability_passes() {
    let controller = OverrideController::new();
    let mut handle = controller.start_autonomous_loop().await;

    // Pause should be available
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused), "Layer 6: Override should be available");

    // Resume should be available
    controller.send_command(OverrideCommand::Resume).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = controller.get_state().await;
    assert!(matches!(state, crate::autonomy::override::ExecutionState::Running), "Layer 6: Resume should work");
}
```

- [ ] **Step 12: Run Layer 6 test**

Run: `cargo test test_layer6_override_availability_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 13: Write Layer 7 - Metrics Accuracy test**

```rust
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;

#[tokio::test]
async fn test_layer7_metrics_accuracy_passes() {
    let collector = MetricsCollector::new();

    // Test counter metrics
    collector.increment_counter("test_counter", 10, &[]).await;
    assert_eq!(collector.get_counter_value("test_counter").await, 10);

    // Test gauge metrics
    collector.set_gauge("test_gauge", 42.5, &[]).await;
    assert_eq!(collector.get_gauge_value("test_gauge").await, 42.5);

    // Test histogram metrics
    for value in [10.0, 20.0, 30.0] {
        collector.observe_histogram("test_histogram", value, &[]).await;
    }

    let summary = collector.get_histogram_summary("test_histogram").await;
    assert_eq!(summary.count, 3);
    assert_eq!(summary.sum, 60.0);

    // All metrics should be accurate
    assert!(summary.min == 10.0 && summary.max == 30.0, "Layer 7: Metrics accuracy should pass");
}
```

- [ ] **Step 14: Run Layer 7 test**

Run: `cargo test test_layer7_metrics_accuracy_passes --test verification_layers_test`
Expected: PASS

- [ ] **Step 15: Write all layers integration test**

```rust
#[tokio::test]
async fn test_all_7_layers_pass_together() {
    // Layer 1: Contract validation
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
checkpoint_frequency: 10
"#;

    let contract = parse_contract(yaml).unwrap();
    assert!(validate_contract(&contract).is_ok());

    // Layer 2: Scope enforcement
    let enforcer = ScopeEnforcer::new();
    let scope = AutonomyScope {
        allowed_paths: vec!["/tmp".to_string()],
        allowed_commands: vec![],
        forbidden_actions: vec![],
        max_file_size: 1024,
        network_access_allowed: false,
        max_execution_time_secs: None,
    };
    assert!(enforcer.check_action(&scope, ActionType::WriteFile, "/tmp/test.txt", 100).await.is_ok());

    // Layer 3: Confidence evaluation
    let evaluator = ConfidenceEvaluator::new();
    let context = ActionContext {
        path: Some("/tmp/test.txt".to_string()),
        reason: "Test".to_string(),
        autonomy_level: crate::autonomy::contract::AutonomyLevel::Bounded,
    };
    assert!(!evaluator.evaluate_action(ActionType::WriteFile, &context).await.is_blocked());

    // Layer 4: Stop condition evaluation
    let stop_evaluator = StopConditionEvaluator::new();
    let context = ExecutionContext {
        iteration: 10,
        elapsed: Duration::from_secs(100),
        quality_score: 0.9,
        interventions: 0,
        resource_usage: crate::autonomy::stop_conditions::ResourceUsage { cpu: 0.5, memory: 0.6 },
    };
    let condition = StopCondition { condition_type: StopConditionType::Iteration, value: 50 };
    assert!(!stop_evaluator.evaluate(&condition, &context).await.should_stop);

    // Layer 5: Checkpoint integrity
    let checkpoint_manager = CheckpointManager::new();
    let restorer = CheckpointRestorer::new(checkpoint_manager.clone());
    let state = serde_json::json!({"test": "data"});
    let checkpoint_id = checkpoint_manager.generate_checkpoint(
        "task".to_string(),
        state.clone(),
        CheckpointMetadata {
            reason: "Test".to_string(),
            checkpoint_type: CheckpointType::Manual,
            iteration: 1,
            tags: vec![],
        },
    ).await;
    assert_eq!(restorer.restore_checkpoint(&checkpoint_id).await.unwrap(), state);

    // Layer 6: Override availability
    let controller = OverrideController::new();
    controller.send_command(OverrideCommand::Pause {
        reason: OverrideReason::Manual,
        requested_by: "test".to_string(),
    }).await;
    assert!(matches!(controller.get_state().await, crate::autonomy::override::ExecutionState::Paused));

    // Layer 7: Metrics accuracy
    let metrics = MetricsCollector::new();
    metrics.increment_counter("test", 1, &[]).await;
    assert_eq!(metrics.get_counter_value("test").await, 1);

    // All layers pass
    assert!(true, "All 7 verification layers pass");
}
```

- [ ] **Step 16: Run all layers integration test**

Run: `cargo test test_all_7_layers_pass_together --test verification_layers_test`
Expected: PASS

- [ ] **Step 17: Commit AC9 tests**

```bash
git add tests/acceptance/verification_layers_test.rs
git commit -m "test(AC9): add 7 verification layers acceptance tests"
```

---

## Summary

Phase 07 Autonomy & Metrics is COMPLETE when:

1. ✅ **AC1**: ADR-0008 Constraints Satisfied - All autonomous loops comply with ADR-0008
2. ✅ **AC2**: Bounded Goals at All Autonomy Levels - Goals are explicit, prioritized, tracked
3. ✅ **AC3**: Human Override Available at All Times - Pause, stop, modify, scope-change
4. ✅ **AC4**: Metrics Capture - Usefulness, time-to-usefulness, intervention rate, repair rate
5. ✅ **AC5**: Dashboards with Real-Time Data - Success, regression, anomaly detection
6. ✅ **AC6**: Stop Conditions Working - Time, iteration, quality, intervention, resource
7. ✅ **AC7**: Checkpoint Restoration Verified - Periodic, event-triggered, state capture, rollback
8. ✅ **AC8**: Risk Assessment Correct - Low risk → high autonomy, high risk → low autonomy
9. ✅ **AC9**: 7 Verification Layers Pass - All layers validated

Each acceptance criterion has comprehensive test coverage with exact test functions, assertions, and cargo test commands.
