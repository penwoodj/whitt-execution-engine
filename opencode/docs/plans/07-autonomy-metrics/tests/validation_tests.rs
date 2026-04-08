use glyphnova_engine::autonomy::*;
use glyphnova_engine::override_controls::*;
use glyphnova_engine::risk::*;

#[test]
fn test_bounded_execution_always_terminates() {
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
  - type: time
    max_duration: "1h"
  - type: iteration
    max_actions: 10
scope:
  allowed_workflows: ["*"]
  allowed_actions: ["*"]
  forbidden_workflows: []
  forbidden_actions: []
"#;

    let contract = ContractParser::parse(yaml).expect("Failed to parse contract");
    ContractValidator::validate(&contract).expect("Contract validation failed");

    let enforcer = BoundedExecutionEnforcer::new(contract);
    let metrics = std::collections::HashMap::new();

    for i in 0..10 {
        assert!(enforcer.check_before_action(&metrics).is_ok());
        enforcer.increment_action_count();
    }

    assert!(enforcer.check_before_action(&metrics).is_err());
}

#[test]
fn test_human_override_never_disabled() {
    let handler = OverrideHandler::new();
    let workflow_id = "test_workflow";

    let pause_event = OverrideEvent::new(
        OverrideType::Pause,
        workflow_id.to_string(),
        "Test".to_string(),
        "operator".to_string(),
    );

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let response = handler.handle_override(pause_event).await;
            assert!(response.success);
            assert!(handler.is_paused(workflow_id).await);

            handler.get_emergency_stop().global_stop();
            assert!(handler.is_stopped(workflow_id).await);
        });
}

#[test]
fn test_scope_boundaries_enforced() {
    let scope = ScopeBoundary {
        allowed_workflows: vec!["deploy_production".to_string()],
        allowed_actions: vec!["deploy".to_string(), "verify".to_string()],
        forbidden_workflows: vec!["delete_data".to_string()],
        forbidden_actions: vec!["delete".to_string()],
        resource_limits: glyphnova_engine::risk::ResourceLimits {
            max_cpu_cores: 10.0,
            max_memory_gb: 32.0,
            max_cost_usd: 100.0,
        },
    };

    assert!(scope.is_workflow_allowed("deploy_production"));
    assert!(!scope.is_workflow_allowed("delete_data"));

    assert!(scope.is_action_allowed("deploy"));
    assert!(!scope.is_action_allowed("delete"));

    assert!(!scope.is_action_allowed("deploy") || scope.is_workflow_allowed("deploy_production"));
}

#[test]
fn test_risk_assessment_blocks_dangerous_actions() {
    let scope = ScopeBoundary {
        allowed_workflows: vec!["deploy_production".to_string()],
        allowed_actions: vec!["deploy".to_string()],
        forbidden_workflows: vec![],
        forbidden_actions: vec![],
        resource_limits: glyphnova_engine::risk::ResourceLimits {
            max_cpu_cores: 10.0,
            max_memory_gb: 32.0,
            max_cost_usd: 100.0,
        },
    };

    let model = RiskAssessmentModel::new(scope);
    let assessment = model.assess_action(
        "action_123".to_string(),
        "delete",
        "deploy_production",
        &[0.5, 0.6, 0.55],
    );

    assert_eq!(assessment.impact, glyphnova_engine::risk::RiskImpact::Critical);
    assert!(matches!(
        assessment.severity,
        glyphnova_engine::risk::RiskSeverity::High | glyphnova_engine::risk::RiskSeverity::Critical
    ));
}
