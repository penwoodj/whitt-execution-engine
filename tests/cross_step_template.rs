//! Integration test: cross-step template resolution in hook arg paths.
//!
//! Verifies engine's `resolve_context_templates()` correctly resolves
//! `{{step.X.output}}` templates via `engine.bookmarks` populated by runner
//! on each step completion.

use std::fs;
use std::path::Path;
use tempfile::TempDir;

use whitt_execution_engine::workflow::hooks::actions::execute_action;
use whitt_execution_engine::workflow::hooks::context::{
    AfterStepSucceedsContext, WorkflowHookContext,
};
use whitt_execution_engine::workflow::hooks::HookEngine;
use whitt_execution_engine::workflow::{
    HookAction, LogAction, LogLevel, SaveToAction,
};

#[test]
fn given_engine_with_step_bookmark_when_save_to_path_uses_cross_step_template_then_resolved() {
    let tmp = TempDir::new().expect("tempdir");

    let mut engine = HookEngine::new();
    engine.store_bookmark(
        "step_01_producer".to_string(),
        serde_json::Value::String("PRODUCER_OUTPUT_MARKER_12345".to_string()),
    );

    let save_to = SaveToAction::FilePath(
        tmp.path()
            .join("consumed-{{step.step_01_producer.output}}.txt")
            .to_string_lossy()
            .to_string(),
    );
    let action = HookAction::SaveTo(save_to);

    let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "step_02_consumer".to_string(),
        output: "consumer_result".to_string(),
        duration_ms: 100,
        quality_score: None,
        token_count: 4,
        model_name: "mock-model".to_string(),
        refusal_detected: false,
    });

    let result = execute_action(&action, &context, &mut engine, None);
    assert!(result.is_continue(), "action should continue");

    let expected_file = tmp.path().join("consumed-PRODUCER_OUTPUT_MARKER_12345.txt");
    assert!(
        expected_file.exists(),
        "Expected file with resolved cross-step template. Got path: {:?}",
        expected_file
    );
}

#[test]
fn given_engine_with_step_bookmark_when_log_path_uses_cross_step_template_then_resolved() {
    let tmp = TempDir::new().expect("tempdir");
    let mut engine = HookEngine::new();
    engine.store_bookmark(
        "step_01_producer".to_string(),
        serde_json::Value::String("log_target_marker".to_string()),
    );

    let log_action = LogAction {
        to_file_path: Some(
            tmp.path()
                .join("{{step.step_01_producer.output}}-event.log")
                .to_string_lossy()
                .to_string(),
        ),
        event_fields: Some(vec!["step_name".to_string()]),
        level: Some(LogLevel::Info),
    };
    let action = HookAction::Log(log_action);

    let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "step_02_consumer".to_string(),
        output: "consumer".to_string(),
        duration_ms: 100,
        quality_score: None,
        token_count: 4,
        model_name: "mock".to_string(),
        refusal_detected: false,
    });

    let result = execute_action(&action, &context, &mut engine, None);
    assert!(result.is_continue());

    let expected_log = tmp.path().join("log_target_marker-event.log");
    assert!(
        expected_log.exists(),
        "Expected log file with resolved cross-step template"
    );
}

#[test]
fn given_engine_empty_when_cross_step_template_used_then_unresolved_silently() {
    let tmp = TempDir::new().expect("tempdir");
    let mut engine = HookEngine::new();

    let save_to = SaveToAction::FilePath(
        tmp.path()
            .join("{{step.nonexistent.output}}.txt")
            .to_string_lossy()
            .to_string(),
    );
    let action = HookAction::SaveTo(save_to);

    let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "consumer".to_string(),
        output: "result".to_string(),
        duration_ms: 100,
        quality_score: None,
        token_count: 4,
        model_name: "mock".to_string(),
        refusal_detected: false,
    });

    let result = execute_action(&action, &context, &mut engine, None);
    assert!(result.is_continue(), "should continue even if unresolved");

    let unresolved_file = tmp.path().join("{{step.nonexistent.output}}.txt");
    assert!(
        unresolved_file.exists(),
        "File should still be written with literal template name"
    );
}

#[test]
fn given_engine_with_nested_bookmark_when_dot_path_used_then_resolved() {
    fs::create_dir_all("./outputs").ok();
    let mut engine = HookEngine::new();
    engine.store_bookmark(
        "shell_output".to_string(),
        serde_json::json!({"stdout": "injected-file-marker"}),
    );

    let save_to = SaveToAction::FilePath(
        "./outputs/inject-{{bookmarks.shell_output.stdout}}.txt".to_string(),
    );
    let action = HookAction::SaveTo(save_to);

    let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "consumer".to_string(),
        output: "r".to_string(),
        duration_ms: 100,
        quality_score: None,
        token_count: 4,
        model_name: "mock".to_string(),
        refusal_detected: false,
    });

    let result = execute_action(&action, &context, &mut engine, None);
    assert!(result.is_continue());

    let expected = Path::new("./outputs/inject-injected-file-marker.txt");
    assert!(
        expected.exists(),
        "Expected file with nested bookmark resolution"
    );

    fs::remove_file(expected).ok();
    fs::remove_dir("./outputs").ok();
}
