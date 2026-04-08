use glyphnova_engine::autonomy::*;
use glyphnova_engine::metrics::*;
use glyphnova_engine::override_controls::*;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::test]
async fn test_bounded_execution_enforcement() {
    let yaml = r#"
version: "1.0"
autonomy_level:
  medium:
    batch_size: 10
goal:
  type: WorkflowCompletion
  target_workflow: "test_workflow"
  success_criteria:
    - type: TaskSuccess
      threshold: 0.9
      operator: above
stop_conditions:
  - type: time
    max_duration: "1h"
  - type: iteration
    max_actions: 50
scope:
  allowed_workflows: ["test_workflow"]
  allowed_actions: ["test_action"]
  forbidden_workflows: []
  forbidden_actions: []
"#;

    let contract = ContractParser::parse(yaml).expect("Failed to parse contract");
    ContractValidator::validate(&contract).expect("Contract validation failed");

    let enforcer = BoundedExecutionEnforcer::new(contract);

    let metrics = HashMap::new();

    for i in 0..50 {
        enforcer.check_before_action(&metrics).expect("Check failed");
        enforcer.increment_action_count();
    }

    assert_eq!(
        enforcer.get_state().action_count.load(std::sync::atomic::Ordering::Relaxed),
        50
    );

    let result = enforcer.check_before_action(&metrics);
    assert!(matches!(result, Err(BoundedExecutionError::IterationLimitExceeded(50))));
}

#[tokio::test]
async fn test_human_override_available() {
    let handler = OverrideHandler::new();
    let workflow_id = "test_workflow_123";

    let pause_event = OverrideEvent::new(
        OverrideType::Pause,
        workflow_id.to_string(),
        "Test pause".to_string(),
        "operator_1".to_string(),
    );

    let response = handler.handle_override(pause_event).await;
    assert!(response.success);
    assert!(handler.is_paused(workflow_id).await);

    handler.get_pause_manager().resume(workflow_id);
    assert!(!handler.is_paused(workflow_id).await);
}

#[tokio::test]
async fn test_metrics_collection() {
    counter!("test_actions", 1.0);
    gauge!("active_workflows", 5.0);

    let counter = METRICS_REGISTRY.counter("test_actions");
    assert_eq!(counter.lock().unwrap().get(), 1.0);

    let gauge = METRICS_REGISTRY.gauge("active_workflows");
    assert_eq!(gauge.lock().unwrap().get(), 5.0);
}

#[tokio::test]
async fn test_quality_gate_enforcement() {
    let yaml = r#"
version: "1.0"
autonomy_level: high
goal:
  type: WorkflowCompletion
  target_workflow: "test_workflow"
  success_criteria:
    - type: TaskSuccess
      threshold: 0.9
      operator: above
stop_conditions:
  - type: quality
    metric: "success_rate"
    threshold: 0.8
    operator: below
scope:
  allowed_workflows: ["*"]
  allowed_actions: ["*"]
  forbidden_workflows: []
  forbidden_actions: []
"#;

    let contract = ContractParser::parse(yaml).expect("Failed to parse contract");
    let enforcer = BoundedExecutionEnforcer::new(contract);

    let mut metrics = HashMap::new();
    metrics.insert("success_rate".to_string(), 0.95);

    assert!(enforcer.check_before_action(&metrics).is_ok());

    metrics.insert("success_rate".to_string(), 0.75);

    let result = enforcer.check_before_action(&metrics);
    assert!(matches!(result, Err(BoundedExecutionError::QualityGateViolated(_, 0.75, 0.8))));
}
