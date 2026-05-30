use whitt_execution_engine::workflow::{
    HookAction, LogAction, GwtClause, AppendToAction, SaveToAction,
    RouteToAction, BookmarkAction, BookmarkActionDetail, FailAction,
};
use whitt_execution_engine::workflow::hooks::{
    HookEngine, HookResult,
    context::{WorkflowHookContext, BeforeStepStartsContext, AfterStepSucceedsContext, StepType},
    actions::execute_action,
    gwt,
};
use std::collections::HashMap;
use std::fs;

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(format!("tests/fixtures/hooks/{}", name))
}

#[test]
fn given_all_triggers_fixture_when_read_then_contains_all_triggers() {
    let content = fs::read_to_string(fixture_path("all-triggers.yml")).expect("read");
    assert!(content.contains("before_step_starts"));
    assert!(content.contains("after_step_succeeds"));
    assert!(content.contains("after_step_fails"));
    assert!(content.contains("during_step_streaming"));
}

#[test]
fn given_log_action_when_executed_then_file_created_with_content() {
    let temp = tempfile::tempdir().expect("tempdir");
    let log_path = temp.path().join("test.log").to_string_lossy().to_string();
    let action = HookAction::Log(LogAction {
        to_file_path: Some(log_path.clone()),
        event_fields: Some(vec!["step_name".into()]),
        level: None,
    });
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "my_step".into(), step_type: StepType::Generative,
        model_name: "m".into(), prompt_preview: "p".into(), workflow_variables: HashMap::new(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert!(fs::read_to_string(&log_path).unwrap().contains("my_step"));
}

#[test]
fn given_save_to_action_when_executed_then_file_contains_output() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("out.yaml").to_string_lossy().to_string();
    let action = HookAction::SaveTo(SaveToAction::FilePath(path.clone()));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "gen".into(), output: "content".into(), duration_ms: 100,
        quality_score: None, token_count: 0, model_name: "m".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert_eq!(fs::read_to_string(&path).unwrap(), "content");
}

#[test]
fn given_append_to_action_when_executed_then_content_appended_to_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("results.yaml");
    fs::write(&path, "initial\n").unwrap();
    let action = HookAction::AppendTo(AppendToAction::FilePath(path.to_string_lossy().to_string()));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "s2".into(), output: "appended".into(), duration_ms: 50,
        quality_score: None, token_count: 0, model_name: "m".into(),
    });
    let mut engine = HookEngine::new();
    execute_action(&action, &ctx, &mut engine);
    let c = fs::read_to_string(&path).unwrap();
    assert!(c.contains("initial") && c.contains("appended"));
}

#[test]
fn given_bookmark_action_when_executed_then_file_and_memory_stored() {
    let temp = tempfile::tempdir().expect("tempdir");
    let cp = temp.path().join("cp.cp").to_string_lossy().to_string();
    let action = HookAction::Bookmark(BookmarkAction::Detailed(BookmarkActionDetail { path: Some(cp.clone()) }));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "cp_step".into(), output: "data".into(), duration_ms: 200,
        quality_score: Some(0.95), token_count: 100, model_name: "m".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert!(std::path::Path::new(&cp).exists());
    assert!(engine.get_bookmark("cp_step").is_some());
}

#[test]
fn given_fail_action_when_executed_then_hook_result_is_fail() {
    let action = HookAction::Fail(FailAction { message: Some("fail".into()) });
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "t".into(), step_type: StepType::Generative,
        model_name: "m".into(), prompt_preview: "p".into(), workflow_variables: HashMap::new(),
    });
    let mut engine = HookEngine::new();
    assert!(matches!(execute_action(&action, &ctx, &mut engine), HookResult::Fail { .. }));
}

#[test]
fn given_skip_step_action_when_executed_then_hook_result_is_skip_step() {
    let action = HookAction::SkipStep(true);
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "t".into(), step_type: StepType::Generative,
        model_name: "m".into(), prompt_preview: "p".into(), workflow_variables: HashMap::new(),
    });
    assert_eq!(execute_action(&action, &ctx, &mut HookEngine::new()), HookResult::SkipStep);
}

#[test]
fn given_route_to_action_when_executed_then_hook_result_routes_to_target() {
    let action = HookAction::RouteTo(RouteToAction::Single("next".into()));
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "t".into(), step_type: StepType::ControlFlow,
        model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
    });
    match execute_action(&action, &ctx, &mut HookEngine::new()) {
        HookResult::RouteTo { targets } => assert_eq!(targets, vec!["next"]),
        r => panic!("Expected RouteTo, got {:?}", r),
    }
}

#[test]
fn given_gwt_matching_clause_when_executed_then_routes_to_then_target() {
    let action = HookAction::Gwt(vec![GwtClause {
        given: Some("step_name == \"test\"".into()),
        r#when: None, r#then: RouteToAction::Single("target".into()),
    }]);
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "test".into(), step_type: StepType::ControlFlow,
        model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
    });
    match execute_action(&action, &ctx, &mut HookEngine::new()) {
        HookResult::RouteTo { targets } => assert_eq!(targets, vec!["target"]),
        r => panic!("Expected RouteTo, got {:?}", r),
    }
}

#[test]
fn given_gwt_non_matching_clause_when_executed_then_continues() {
    let action = HookAction::Gwt(vec![GwtClause {
        given: Some("step_name == \"wrong\"".into()),
        r#when: None, r#then: RouteToAction::Single("x".into()),
    }]);
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "test".into(), step_type: StepType::ControlFlow,
        model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
    });
    assert!(execute_action(&action, &ctx, &mut HookEngine::new()).is_continue());
}

#[test]
fn given_gwt_comparison_expression_when_evaluated_then_correct() {
    let ctx = serde_json::json!({"quality_score": 0.95});
    assert!(gwt::evaluate("quality_score >= 0.9", &ctx).unwrap());
}

#[test]
fn given_gwt_logical_and_when_evaluated_then_correct() {
    let ctx = serde_json::json!({"a": true, "b": false});
    assert!(gwt::evaluate("a == true && b == false", &ctx).unwrap());
}

#[test]
fn given_gwt_dot_path_when_evaluated_then_navigates_nested() {
    let ctx = serde_json::json!({"error": {"is_retryable": true}});
    assert!(gwt::evaluate("error.is_retryable == true", &ctx).unwrap());
}

#[test]
fn given_hook_results_merged_then_highest_priority_wins() {
    let merged = HookResult::merge(HookResult::Continue, HookResult::SkipStep);
    let merged = HookResult::merge(merged, HookResult::Fail { reason: "e".into() });
    assert!(matches!(merged, HookResult::Fail { .. }));
}

#[test]
fn given_invalid_gwt_expression_when_evaluated_then_error() {
    assert!(gwt::evaluate("=== &&&", &serde_json::json!({})).is_err());
}

#[test]
fn given_missing_field_gwt_when_evaluated_then_false() {
    assert!(!gwt::evaluate("missing == \"val\"", &serde_json::json!({})).unwrap());
}
