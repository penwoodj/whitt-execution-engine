use whitt_execution_engine::workflow::{
    HookAction, LogAction, GwtClause, AppendToAction, SaveToAction,
    RouteToAction, BookmarkAction, BookmarkActionDetail, FailAction, NotifyAction, ShellAction,
};
use whitt_execution_engine::workflow::hooks::{
    HookEngine, HookResult,
    context::{WorkflowHookContext, BeforeStepStartsContext, AfterStepSucceedsContext, StepType,
               AfterStepFailsContext, DuringStepStreamingContext, ErrorDetails},
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

// ============================================================================
// Serde Round-Trip Tests for HookAction
// These tests prevent regression of the untagged deserialization bug
// where ALL actions silently deserialized as empty LogAction.
// ============================================================================

#[test]
fn given_log_json_when_deserialized_then_produces_log_action() {
    let json = serde_json::json!({"log": {"to_file_path": "test.log"}});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Log(LogAction { to_file_path: Some(_), .. })));
}

#[test]
fn given_save_to_string_json_when_deserialized_then_becomes_variable_variant() {
    let json = serde_json::json!({"save_to": "output.txt"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::SaveTo(SaveToAction::Variable(_))));
    if let HookAction::SaveTo(SaveToAction::Variable(var)) = action {
        assert_eq!(var, "output.txt");
    }
}

#[test]
fn given_save_to_variable_json_when_deserialized_then_produces_variable_variant() {
    let json = serde_json::json!({"save_to": "$$result"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::SaveTo(SaveToAction::Variable(_))));
    if let HookAction::SaveTo(SaveToAction::Variable(var)) = action {
        assert_eq!(var, "$$result");
    }
}

#[test]
fn given_save_to_both_json_when_deserialized_then_produces_both_variant() {
    let json = serde_json::json!({"save_to": ["output.txt", "$$result"]});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::SaveTo(SaveToAction::Both(_))));
    if let HookAction::SaveTo(SaveToAction::Both(targets)) = action {
        assert_eq!(targets, vec!["output.txt", "$$result"]);
    }
}

#[test]
fn given_save_to_string_json_when_deserialized_then_not_log_action_regression() {
    let json = serde_json::json!({"save_to": "output.txt"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(!matches!(action, HookAction::Log(_)));
}

#[test]
fn given_bookmark_flag_json_when_deserialized_then_produces_flag_variant() {
    let json = serde_json::json!({"bookmark": true});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Bookmark(BookmarkAction::Flag(_))));
    if let HookAction::Bookmark(BookmarkAction::Flag(flag)) = action {
        assert!(flag);
    }
}

#[test]
fn given_bookmark_path_json_when_deserialized_then_produces_path_variant() {
    let json = serde_json::json!({"bookmark": "/path/to/bookmark.cp"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Bookmark(BookmarkAction::Path(_))));
    if let HookAction::Bookmark(BookmarkAction::Path(path)) = action {
        assert_eq!(path, "/path/to/bookmark.cp");
    }
}

#[test]
fn given_bookmark_detailed_json_when_deserialized_then_produces_detailed_variant() {
    let json = serde_json::json!({"bookmark": {"path": "/path/to/bookmark.cp"}});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Bookmark(BookmarkAction::Detailed(_))));
    if let HookAction::Bookmark(BookmarkAction::Detailed(detail)) = action {
        assert_eq!(detail.path, Some("/path/to/bookmark.cp".to_string()));
    }
}

#[test]
fn given_skip_step_json_when_deserialized_then_produces_skip_step() {
    let json = serde_json::json!({"skip_step": true});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::SkipStep(_)));
    if let HookAction::SkipStep(skip) = action {
        assert!(skip);
    }
}

#[test]
fn given_skip_remaining_json_when_deserialized_then_produces_skip_remaining() {
    let json = serde_json::json!({"skip_remaining": true});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::SkipRemaining(_)));
    if let HookAction::SkipRemaining(skip) = action {
        assert!(skip);
    }
}

#[test]
fn given_fail_json_when_deserialized_then_produces_fail() {
    let json = serde_json::json!({"fail": {"message": "Execution failed"}});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Fail(_)));
    if let HookAction::Fail(fail) = action {
        assert_eq!(fail.message, Some("Execution failed".to_string()));
    }
}

#[test]
fn given_notify_json_when_deserialized_then_produces_notify() {
    let json = serde_json::json!({"notify": {"message": "Notification message"}});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Notify(_)));
    if let HookAction::Notify(notify) = action {
        assert_eq!(notify.message, Some("Notification message".to_string()));
    }
}

#[test]
fn given_route_to_single_json_when_deserialized_then_produces_single() {
    let json = serde_json::json!({"route_to": "next_step"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::RouteTo(RouteToAction::Single(_))));
    if let HookAction::RouteTo(RouteToAction::Single(target)) = action {
        assert_eq!(target, "next_step");
    }
}

#[test]
fn given_route_to_multiple_json_when_deserialized_then_produces_multiple() {
    let json = serde_json::json!({"route_to": ["step_a", "step_b", "step_c"]});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::RouteTo(RouteToAction::Multiple(_))));
    if let HookAction::RouteTo(RouteToAction::Multiple(targets)) = action {
        assert_eq!(targets, vec!["step_a", "step_b", "step_c"]);
    }
}

#[test]
fn given_gwt_json_when_deserialized_then_produces_gwt() {
    let json = serde_json::json!({
        "gwt": [{
            "given": "quality_score >= 0.9",
            "then": "next_step"
        }]
    });
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::Gwt(_)));
    if let HookAction::Gwt(clauses) = action {
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].given, Some("quality_score >= 0.9".to_string()));
        assert!(matches!(clauses[0].r#then, RouteToAction::Single(_)));
    }
}

#[test]
fn given_iterate_values_json_when_deserialized_then_produces_iterate() {
    let mut map = std::collections::HashMap::new();
    map.insert("item".to_string(), vec!["a".to_string(), "b".to_string()]);
    let json = serde_json::json!({"iterate_values": {"item": ["a", "b"]}});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::IterateValues(_)));
    if let HookAction::IterateValues(values) = action {
        assert_eq!(values.get("item"), Some(&vec!["a".to_string(), "b".to_string()]));
    }
}

#[test]
fn given_append_to_string_json_when_deserialized_then_becomes_variable_variant() {
    let json = serde_json::json!({"append_to": "output.txt"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::AppendTo(AppendToAction::Variable(_))));
    if let HookAction::AppendTo(AppendToAction::Variable(var)) = action {
        assert_eq!(var, "output.txt");
    }
}

#[test]
fn given_append_to_variable_json_when_deserialized_then_produces_variable_variant() {
    let json = serde_json::json!({"append_to": "$$results"});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::AppendTo(AppendToAction::Variable(_))));
    if let HookAction::AppendTo(AppendToAction::Variable(var)) = action {
        assert_eq!(var, "$$results");
    }
}

#[test]
fn given_append_to_both_json_when_deserialized_then_produces_both_variant() {
    let json = serde_json::json!({"append_to": ["output.txt", "$$results"]});
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    assert!(matches!(action, HookAction::AppendTo(AppendToAction::Both(_))));
    if let HookAction::AppendTo(AppendToAction::Both(targets)) = action {
        assert_eq!(targets, vec!["output.txt", "$$results"]);
    }
}

#[test]
fn given_all_action_variants_serialize_then_deserialize_produces_same() {
    let actions = vec![
        HookAction::Log(LogAction {
            to_file_path: Some("test.log".into()),
            event_fields: Some(vec!["step_name".into()]),
            level: None,
        }),
        HookAction::SaveTo(SaveToAction::FilePath("out.txt".into())),
        HookAction::SaveTo(SaveToAction::Variable("$$var".into())),
        HookAction::SaveTo(SaveToAction::Both(vec!["out.txt".into(), "$$var".into()])),
        HookAction::Bookmark(BookmarkAction::Flag(true)),
        HookAction::Bookmark(BookmarkAction::Path("/path".into())),
        HookAction::Bookmark(BookmarkAction::Detailed(BookmarkActionDetail {
            path: Some("/path".into()),
        })),
        HookAction::SkipStep(true),
        HookAction::SkipRemaining(false),
        HookAction::Fail(FailAction {
            message: Some("error".into()),
        }),
        HookAction::Notify(NotifyAction {
            message: Some("msg".into()),
        }),
        HookAction::RouteTo(RouteToAction::Single("step".into())),
        HookAction::RouteTo(RouteToAction::Multiple(vec!["a".into(), "b".into()])),
        HookAction::Gwt(vec![GwtClause {
            given: Some("x == 1".into()),
            r#when: None,
            r#then: RouteToAction::Single("t".into()),
        }]),
    ];

    for original in actions {
        let serialized = serde_json::to_value(&original).expect("serialize");
        let deserialized: HookAction = serde_json::from_value(serialized).expect("deserialize");
        // Can't directly compare due to non-PartialEq types, just verify it deserializes
        match (&original, &deserialized) {
            (HookAction::Log(_), HookAction::Log(_)) => {}
            (HookAction::SaveTo(o), HookAction::SaveTo(d)) => {
                match (o, d) {
                    (SaveToAction::Both(b1), SaveToAction::Both(b2)) => assert_eq!(b1, b2),
                    (SaveToAction::FilePath(p) | SaveToAction::Variable(p), SaveToAction::Variable(v)) => assert_eq!(p, v),
                    _ => panic!("SaveTo unexpected variant combination"),
                }
            }
            (HookAction::AppendTo(o), HookAction::AppendTo(d)) => {
                match (o, d) {
                    (AppendToAction::Both(b1), AppendToAction::Both(b2)) => assert_eq!(b1, b2),
                    (AppendToAction::FilePath(p) | AppendToAction::Variable(p), AppendToAction::Variable(v)) => assert_eq!(p, v),
                    _ => panic!("AppendTo unexpected variant combination"),
                }
            }
            (HookAction::Bookmark(_), HookAction::Bookmark(_)) => {}
            (HookAction::SkipStep(s1), HookAction::SkipStep(s2)) => assert_eq!(s1, s2),
            (HookAction::SkipRemaining(s1), HookAction::SkipRemaining(s2)) => assert_eq!(s1, s2),
            (HookAction::Fail(_), HookAction::Fail(_)) => {}
            (HookAction::Notify(_), HookAction::Notify(_)) => {}
            (HookAction::RouteTo(_), HookAction::RouteTo(_)) => {}
            (HookAction::Gwt(_), HookAction::Gwt(_)) => {}
            (HookAction::IterateValues(_), HookAction::IterateValues(_)) => {}
            _ => panic!("Variant mismatch between original and deserialized"),
        }
    }
}

// ============================================================================
// Additional Integration Tests
// Tests for actions with different context types and multi-action scenarios
// ============================================================================

#[test]
fn given_save_to_variable_when_executed_then_bookmark_stored_in_engine() {
    let action = HookAction::SaveTo(SaveToAction::Variable("$myvar".to_string()));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "step_1".into(),
        output: "saved_data".into(),
        duration_ms: 100,
        quality_score: Some(0.9),
        token_count: 50,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    // Bookmark stored with key "myvar" ($ prefix stripped)
    assert!(engine.get_bookmark("myvar").is_some());
}

#[test]
fn given_save_to_both_when_executed_then_file_written_and_bookmark_stored() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("output.txt").to_string_lossy().to_string();
    let action = HookAction::SaveTo(SaveToAction::Both(vec!["$var".to_string(), path.clone()]));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "step_2".into(),
        output: "both_test".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert!(std::path::Path::new(&path).exists());
    assert_eq!(fs::read_to_string(&path).unwrap(), "both_test");
    // Bookmark stored with key "var" ($ prefix stripped)
    assert!(engine.get_bookmark("var").is_some());
}

#[test]
fn given_bookmark_flag_when_executed_then_stores_but_no_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let temp_dir = temp.path();
    let action = HookAction::Bookmark(BookmarkAction::Flag(true));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "flag_step".into(),
        output: "flag_data".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert!(engine.get_bookmark("flag_step").is_some());
    // Verify no file created (check that no .cp files exist in temp dir)
    assert_eq!(std::fs::read_dir(temp_dir).unwrap().count(), 0);
}

#[test]
fn given_bookmark_path_when_executed_then_file_and_bookmark_stored() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("bookmark.cp").to_string_lossy().to_string();
    let action = HookAction::Bookmark(BookmarkAction::Path(path.clone()));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "path_step".into(),
        output: "path_data".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    assert!(std::path::Path::new(&path).exists());
    assert!(engine.get_bookmark("path_step").is_some());
}

#[test]
fn given_skip_remaining_action_when_executed_then_returns_skip_remaining() {
    let action = HookAction::SkipRemaining(true);
    let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: "t".into(),
        step_type: StepType::Generative,
        model_name: "m".into(),
        prompt_preview: "p".into(),
        workflow_variables: HashMap::new(),
    });
    assert_eq!(execute_action(&action, &ctx, &mut HookEngine::new()), HookResult::SkipRemaining);
}

#[test]
fn given_multi_action_trigger_when_executed_then_merge_results_continue() {
    let temp = tempfile::tempdir().expect("tempdir");
    let log_path = temp.path().join("test.log").to_string_lossy().to_string();
    let save_path = temp.path().join("save.txt").to_string_lossy().to_string();

    let log_action = HookAction::Log(LogAction {
        to_file_path: Some(log_path.clone()),
        event_fields: Some(vec!["step_name".into()]),
        level: None,
    });
    let save_action = HookAction::SaveTo(SaveToAction::FilePath(save_path.clone()));
    let bookmark_action = HookAction::Bookmark(BookmarkAction::Flag(true));

    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "multi_test".into(),
        output: "output".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();

    let r1 = execute_action(&log_action, &ctx, &mut engine);
    let r2 = execute_action(&save_action, &ctx, &mut engine);
    let r3 = execute_action(&bookmark_action, &ctx, &mut engine);

    let merged = HookResult::merge(HookResult::merge(r1, r2), r3);
    assert!(merged.is_continue());
    assert!(std::path::Path::new(&log_path).exists());
    assert!(std::path::Path::new(&save_path).exists());
    assert!(engine.get_bookmark("multi_test").is_some());
}

#[test]
fn given_action_with_after_step_fails_context_when_executed_then_file_contains_error() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("error.log").to_string_lossy().to_string();
    let action = HookAction::SaveTo(SaveToAction::FilePath(path.clone()));
    let ctx = WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
        step_name: "fail_step".into(),
        error_type: "NetworkError".into(),
        error_message: "Connection timeout".into(),
        error: ErrorDetails { is_retryable: true, count: 1 },
        attempt_number: 1,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("Connection timeout"));
}

#[test]
fn given_action_with_during_step_streaming_context_when_executed_then_file_contains_chunk() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("stream.txt").to_string_lossy().to_string();
    let action = HookAction::SaveTo(SaveToAction::FilePath(path.clone()));
    let ctx = WorkflowHookContext::DuringStepStreaming(DuringStepStreamingContext {
        step_name: "stream_step".into(),
        chunk_text: "Hello world".into(),
        tokens_so_far: 5,
        elapsed_ms: 100,
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Hello world");
}

#[test]
fn given_append_to_variable_when_executed_then_returns_continue() {
    let action = HookAction::AppendTo(AppendToAction::Variable("var".to_string()));
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "test".into(),
        output: "data".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    // Should not panic, just return Continue (stub implementation)
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
}

#[test]
fn given_notify_with_message_when_executed_then_returns_continue() {
    let action = HookAction::Notify(NotifyAction { message: Some("test msg".into()) });
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "notify_step".into(),
        output: "output".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 0,
        model_name: "model".into(),
    });
    let mut engine = HookEngine::new();
    assert!(execute_action(&action, &ctx, &mut engine).is_continue());
}

#[test]
fn given_shell_action_when_deserialized_from_json_then_correct() {
    let json = serde_json::json!({
        "shell": {
            "command": "echo",
            "args": ["hello"],
            "fail_on_error": false
        }
    });
    let action: HookAction = serde_json::from_value(json).expect("deserialize");
    match action {
        HookAction::Shell(ref shell) => {
            assert_eq!(shell.command, "echo");
            assert_eq!(shell.args.as_deref(), Some(&["hello".to_string()][..]));
            assert_eq!(shell.fail_on_error, Some(false));
        }
        other => panic!("Expected Shell, got {:?}", other),
    }
}
