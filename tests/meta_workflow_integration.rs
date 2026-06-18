//! Meta-workflow generator integration tests.
//!
//! Validates the hook/action/template patterns used by SW1-SW5 sub-workflows.
//! These tests exercise the engine machinery WITHOUT requiring a live LLM —
//! they verify that hooks fire correctly, templates resolve, and state passes
//! between steps as the meta-workflow design expects.

use std::collections::HashMap;
use std::fs;

use serde_json::json;
use tempfile::tempdir;
use whitt_execution_engine::workflow::hooks::{
    HookEngine, HookResult,
    context::{
        WorkflowHookContext, BeforeStepStartsContext, AfterStepSucceedsContext, StepType,
    },
    actions::execute_action,
};
use whitt_execution_engine::workflow::{
    HookAction, LogAction, GwtClause, SaveToAction, BookmarkAction, BookmarkActionDetail,
    FailAction, ShellAction, RouteToAction,
};

fn before_ctx(step: &str) -> WorkflowHookContext {
    WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_name: step.into(),
        step_type: StepType::Generative,
        model_name: "Qwen3-5-9B".into(),
        prompt_preview: "preview".into(),
        workflow_variables: HashMap::new(),
    })
}

fn after_success_ctx(step: &str, output: &str) -> WorkflowHookContext {
    WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: step.into(),
        output: output.into(),
        duration_ms: 1000,
        quality_score: None,
        token_count: 100,
        model_name: "Qwen3-5-9B".into(),
    })
}

#[test]
fn given_shell_action_when_executed_then_stdout_stored_in_bookmarks() {
    let tmp = tempdir().unwrap();
    let action = HookAction::Shell(ShellAction {
        command: "echo 'hello_from_shell'".to_string(),
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(true),
    });

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &before_ctx("test_step"), &mut engine, None);
    assert!(result.is_continue(), "shell action should return Continue");

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output bookmark should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert!(stdout.contains("hello_from_shell"), "stdout should contain echo output");
}

#[test]
fn given_save_to_file_when_executed_then_file_created_with_content() {
    let tmp = tempdir().unwrap();
    let file_path = tmp.path().join("output.txt").to_string_lossy().into_owned();
    let action = HookAction::SaveTo(SaveToAction::FilePath(file_path.clone()));

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("test_step", "test output content"), &mut engine, None);
    assert!(result.is_continue());

    let content = fs::read_to_string(&file_path).expect("file should exist");
    assert_eq!(content, "test output content");
}

#[test]
fn given_save_to_variable_when_executed_then_bookmark_stored() {
    let action = HookAction::SaveTo(SaveToAction::Variable("$my_var".to_string()));

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("test_step", "stored value"), &mut engine, None);
    assert!(result.is_continue());

    let bookmark = engine.get_bookmark("my_var").expect("variable bookmark should exist");
    assert_eq!(
        bookmark.as_str().unwrap_or(""),
        "stored value",
        "variable should contain step output"
    );
}

#[test]
fn given_gwt_clause_matching_condition_when_executed_then_routes_to_target() {
    let action = HookAction::Gwt(vec![GwtClause {
        given: Some("true".to_string()),
        when: None,
        then: RouteToAction::Multiple(vec!["step_05_assemble_final".to_string()]),
    }]);

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("step_03_evaluate", "VERDICT: PASS"), &mut engine, None);
    match result {
        HookResult::RouteTo { targets } => {
            assert_eq!(targets, vec!["step_05_assemble_final"]);
        }
        other => panic!("expected RouteTo, got {:?}", other),
    }
}

#[test]
fn given_gwt_clause_non_matching_when_executed_then_falls_through() {
    let action = HookAction::Gwt(vec![
        GwtClause {
            given: Some("false".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["should_not_route".to_string()]),
        },
        GwtClause {
            given: Some("true".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["default_target".to_string()]),
        },
    ]);

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("eval", "irrelevant"), &mut engine, None);
    match result {
        HookResult::RouteTo { targets } => {
            assert_eq!(targets, vec!["default_target"], "should fall through to true clause");
        }
        other => panic!("expected RouteTo, got {:?}", other),
    }
}

#[test]
fn given_gwt_with_quality_score_when_above_threshold_then_routes_to_finalize() {
    let action = HookAction::Gwt(vec![
        GwtClause {
            given: Some("quality_score >= 0.8".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_06_assemble_final".to_string()]),
        },
        GwtClause {
            given: Some("true".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_05_fix".to_string()]),
        },
    ]);

    let mut engine = HookEngine::new();
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "step_04".into(),
        output: "all checks passed".into(),
        duration_ms: 1000,
        quality_score: Some(0.85),
        token_count: 100,
        model_name: "Qwen3-5-9B".into(),
    });
    let result = execute_action(&action, &ctx, &mut engine, None);
    match result {
        HookResult::RouteTo { targets } => {
            assert_eq!(
                targets, vec!["step_06_assemble_final"],
                "quality_score 0.85 >= 0.8 should route to finalize"
            );
        }
        other => panic!("expected RouteTo to finalize, got {:?}", other),
    }
}

#[test]
fn given_bookmark_detailed_when_executed_then_file_and_state_persisted() {
    let tmp = tempdir().unwrap();
    let file_path = tmp.path().join("checkpoint.cp").to_string_lossy().into_owned();
    let action = HookAction::Bookmark(BookmarkAction::Detailed(BookmarkActionDetail {
        path: Some(file_path.clone()),
    }));

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("checkpoint_step", "checkpoint data"), &mut engine, None);
    assert!(result.is_continue());
    assert!(std::path::Path::new(&file_path).exists(), "bookmark file should exist");
    assert!(engine.get_bookmark("checkpoint_step").is_some(), "bookmark state should be stored");
}

#[test]
fn given_hook_results_when_merged_then_fail_dominates() {
    let fail = HookResult::Fail { reason: "critical error".to_string() };
    let route = HookResult::RouteTo { targets: vec!["next".to_string()] };
    let continue_ = HookResult::Continue;

    let merged = HookResult::merge(HookResult::merge(fail.clone(), route.clone()), continue_);
    match merged {
        HookResult::Fail { reason } => assert_eq!(reason, "critical error"),
        other => panic!("Fail should dominate, got {:?}", other),
    }
}

#[test]
fn given_hook_results_when_merged_then_route_dominates_continue() {
    let route = HookResult::RouteTo { targets: vec!["target".to_string()] };
    let continue_ = HookResult::Continue;

    let merged = HookResult::merge(continue_, route.clone());
    match merged {
        HookResult::RouteTo { targets } => assert_eq!(targets, vec!["target"]),
        other => panic!("RouteTo should dominate Continue, got {:?}", other),
    }
}

#[test]
fn given_log_action_when_executed_then_file_written_with_step_name() {
    let tmp = tempdir().unwrap();
    let log_path = tmp.path().join("workflow.log").to_string_lossy().into_owned();
    let action = HookAction::Log(LogAction {
        to_file_path: Some(log_path.clone()),
        event_fields: Some(vec!["step_name".to_string(), "duration_ms".to_string()]),
        level: None,
    });

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("logged_step", "done"), &mut engine, None);
    assert!(result.is_continue());

    let log_content = fs::read_to_string(&log_path).expect("log file should exist");
    assert!(log_content.contains("step_name=logged_step"), "log should contain step name");
}

#[test]
fn given_multi_shell_sequence_when_executed_then_latest_overwrites_shell_output() {
    let tmp = tempdir().unwrap();
    let workdir = tmp.path().to_string_lossy().into_owned();
    let ctx = before_ctx("multi_shell");
    let mut engine = HookEngine::new();

    let action1 = HookAction::Shell(ShellAction {
        command: "echo 'first'".to_string(),
        args: Some(vec![]),
        working_dir: Some(workdir.clone()),
        env: Some(HashMap::new()),
        fail_on_error: Some(true),
    });
    execute_action(&action1, &ctx, &mut engine, None);

    let action2 = HookAction::Shell(ShellAction {
        command: "echo 'second'".to_string(),
        args: Some(vec![]),
        working_dir: Some(workdir),
        env: Some(HashMap::new()),
        fail_on_error: Some(true),
    });
    execute_action(&action2, &ctx, &mut engine, None);

    let bookmark = engine.get_bookmark("shell_output").expect("bookmark should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        stdout.contains("second"),
        "second shell should be latest: {}",
        stdout
    );
}

#[test]
fn given_fail_action_when_executed_then_returns_fail_result_with_reason() {
    let action = HookAction::Fail(FailAction {
        message: Some("quality gate failed".to_string()),
    });

    let mut engine = HookEngine::new();
    let result = execute_action(&action, &after_success_ctx("eval", "VERDICT: FAIL"), &mut engine, None);
    match result {
        HookResult::Fail { reason } => {
            assert!(reason.contains("quality gate failed"));
        }
        other => panic!("expected Fail, got {:?}", other),
    }
}

#[test]
fn given_workflow_context_when_serialized_then_contains_required_fields() {
    let ctx = AfterStepSucceedsContext {
        step_name: "step_05_final".to_string(),
        output: "final output".to_string(),
        duration_ms: 50000,
        quality_score: Some(0.85),
        token_count: 1500,
        model_name: "Qwen3-5-9B".to_string(),
    };

    let json_val = ctx.to_json_value();
    assert_eq!(json_val.get("step_name").and_then(|v| v.as_str()), Some("step_05_final"));
    assert_eq!(json_val.get("model_name").and_then(|v| v.as_str()), Some("Qwen3-5-9B"));
    assert_eq!(json_val.get("token_count").and_then(|v| v.as_u64()), Some(1500));
    assert!(json_val.get("quality_score").is_some());
}

#[test]
fn given_hook_engine_when_storing_multiple_bookmarks_then_all_retrievable() {
    let mut engine = HookEngine::new();

    engine.store_bookmark("tasks_output".to_string(), json!("T1, T2, T3"));
    engine.store_bookmark("eval_verdict".to_string(), json!("PASS"));
    engine.store_bookmark("chunk_count".to_string(), json!(4));

    assert_eq!(
        engine.get_bookmark("tasks_output").and_then(|v| v.as_str()),
        Some("T1, T2, T3")
    );
    assert_eq!(
        engine.get_bookmark("eval_verdict").and_then(|v| v.as_str()),
        Some("PASS")
    );
    assert_eq!(
        engine.get_bookmark("chunk_count").and_then(|v| v.as_u64()),
        Some(4)
    );
}

#[test]
fn given_continue_result_when_checked_then_is_continue_true_and_is_terminal_false() {
    let result = HookResult::Continue;
    assert!(result.is_continue());
    assert!(!result.is_terminal());
}

#[test]
fn given_fail_result_when_checked_then_is_terminal_true() {
    let result = HookResult::Fail { reason: "test".to_string() };
    assert!(result.is_terminal());
    assert!(!result.is_continue());
}

#[test]
fn given_skip_remaining_result_when_checked_then_is_terminal_true() {
    let result = HookResult::SkipRemaining;
    assert!(result.is_terminal());
}

#[test]
fn given_shell_verdict_pass_when_gwt_checks_string_equality_then_routes_correctly() {
    let tmp = tempfile::tempdir().unwrap();
    let eval_file = tmp.path().join("eval.txt");
    fs::write(&eval_file, "Some eval output\nVERDICT: PASS\n").unwrap();

    let shell_action = HookAction::Shell(ShellAction {
        command: format!(
            "grep -q 'VERDICT: PASS' '{}' && printf '%s' PASS || printf '%s' FAIL",
            eval_file.to_string_lossy()
        ),
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "evaluate".into(),
        output: "eval output".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 50,
        model_name: "qwen35".into(),
    });

    execute_action(&shell_action, &ctx, &mut engine, None);

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(stdout, "PASS", "stdout should be exactly PASS");

    let gwt_pass = vec![GwtClause {
        given: Some(r#""PASS" == "PASS""#.to_string()),
        when: None,
        then: RouteToAction::Multiple(vec!["finalize".to_string()]),
    }];
    let pass_action = HookAction::Gwt(gwt_pass);
    let pass_result = execute_action(&pass_action, &ctx, &mut engine, None);
    assert!(
        matches!(pass_result, HookResult::RouteTo { .. }),
        "GWT with matching string equality should route"
    );
}

#[test]
fn given_shell_verdict_fail_when_gwt_checks_string_equality_then_falls_through() {
    let tmp = tempfile::tempdir().unwrap();
    let eval_file = tmp.path().join("eval.txt");
    fs::write(&eval_file, "Issues found\nVERDICT: FAIL\n").unwrap();

    let shell_action = HookAction::Shell(ShellAction {
        command: format!(
            "grep -q 'VERDICT: PASS' '{}' && printf '%s' PASS || printf '%s' FAIL",
            eval_file.to_string_lossy()
        ),
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
        step_name: "evaluate".into(),
        output: "eval output".into(),
        duration_ms: 100,
        quality_score: None,
        token_count: 50,
        model_name: "qwen35".into(),
    });

    execute_action(&shell_action, &ctx, &mut engine, None);

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(stdout, "FAIL", "stdout should be exactly FAIL");

    let gwt_clauses = vec![
        GwtClause {
            given: Some(r#""FAIL" == "PASS""#.to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["finalize".to_string()]),
        },
        GwtClause {
            given: Some("true".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["fix".to_string()]),
        },
    ];
    let gwt_action = HookAction::Gwt(gwt_clauses);
    let result = execute_action(&gwt_action, &ctx, &mut engine, None);
    assert!(
        matches!(&result, HookResult::RouteTo { targets } if targets == &vec!["fix".to_string()]),
        "GWT should fall through to fix on FAIL verdict"
    );
}

/// Regression test: SW2/SW3 lenient coverage gate pattern (≥80% = PASS).
///
/// Confirmed behavior in live testing (commit f44aa2f):
/// 1. Shell computes coverage percentage and outputs COVERAGE_GATE: PASS or FAIL
/// 2. Bookmark stores shell_output.stdout
/// 3. GWT clause compares stdout to "PASS"
/// 4. Routes to assemble_final or fix based on shell output
///
/// Without this pattern, evaluator triggered infinite fix loops because
/// any missing task caused auto-FAIL even when 80%+ were covered.
#[test]
fn given_shell_coverage_gate_pass_at_threshold_when_gwt_routes_then_goes_to_assemble() {
    let tmp = tempfile::tempdir().unwrap();

    // Simulate a categorized.md with 8 of 10 tasks covered (80% threshold)
    let task_list = tmp.path().join("tasks.txt");
    let categorized = tmp.path().join("categorized.md");
    fs::write(&task_list, "T1\nT2\nT3\nT4\nT5\nT6\nT7\nT8\nT9\nT10\n").unwrap();
    fs::write(&categorized, "### T1 x\n### T2 x\n### T3 x\n### T4 x\n### T5 x\n### T6 x\n### T7 x\n### T8 x\n").unwrap();

    // Mirror the actual SW3 evaluator shell command (coverage % + COVERAGE_GATE)
    let shell_cmd = format!(
        "TOTAL=$(grep -o 'T[0-9]*' '{tasks}' | sort -u | wc -l) && \
         COVERED=$(grep -o '### T[0-9]*' '{cat}' | grep -o 'T[0-9]*' | sort -u | wc -l) && \
         PCT=$((TOTAL > 0 ? COVERED * 100 / TOTAL : 0)) && \
         if [ \"$PCT\" -ge 80 ]; then printf '%s' PASS; else printf '%s' FAIL; fi",
        tasks = task_list.to_string_lossy(),
        cat = categorized.to_string_lossy()
    );

    let shell_action = HookAction::Shell(ShellAction {
        command: shell_cmd,
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = before_ctx("evaluate");

    execute_action(&shell_action, &ctx, &mut engine, None);

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(stdout, "PASS", "80% coverage should produce PASS");

    // GWT routes to assemble on PASS
    let gwt_clauses = vec![
        GwtClause {
            given: Some(r#""PASS" == "PASS""#.to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_06_assemble_final".to_string()]),
        },
        GwtClause {
            given: Some("true".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_05_fix".to_string()]),
        },
    ];
    let result = execute_action(&HookAction::Gwt(gwt_clauses), &ctx, &mut engine, None);
    assert!(
        matches!(&result, HookResult::RouteTo { targets } if targets == &vec!["step_06_assemble_final".to_string()]),
        "80% coverage should route to assemble_final"
    );
}

/// Regression test: Below-threshold coverage (e.g., 70%) produces FAIL
/// and routes to fix step.
#[test]
fn given_shell_coverage_gate_below_threshold_when_gwt_routes_then_goes_to_fix() {
    let tmp = tempfile::tempdir().unwrap();

    // 7 of 10 tasks covered = 70% < 80% threshold → FAIL
    let task_list = tmp.path().join("tasks.txt");
    let categorized = tmp.path().join("categorized.md");
    fs::write(&task_list, "T1\nT2\nT3\nT4\nT5\nT6\nT7\nT8\nT9\nT10\n").unwrap();
    fs::write(&categorized, "### T1 x\n### T2 x\n### T3 x\n### T4 x\n### T5 x\n### T6 x\n### T7 x\n").unwrap();

    let shell_cmd = format!(
        "TOTAL=$(grep -o 'T[0-9]*' '{tasks}' | sort -u | wc -l) && \
         COVERED=$(grep -o '### T[0-9]*' '{cat}' | grep -o 'T[0-9]*' | sort -u | wc -l) && \
         PCT=$((TOTAL > 0 ? COVERED * 100 / TOTAL : 0)) && \
         if [ \"$PCT\" -ge 80 ]; then printf '%s' PASS; else printf '%s' FAIL; fi",
        tasks = task_list.to_string_lossy(),
        cat = categorized.to_string_lossy()
    );

    let shell_action = HookAction::Shell(ShellAction {
        command: shell_cmd,
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = before_ctx("evaluate");

    execute_action(&shell_action, &ctx, &mut engine, None);

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(stdout, "FAIL", "70% coverage should produce FAIL");

    let gwt_clauses = vec![
        GwtClause {
            given: Some(r#""FAIL" == "PASS""#.to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_06_assemble_final".to_string()]),
        },
        GwtClause {
            given: Some("true".to_string()),
            when: None,
            then: RouteToAction::Multiple(vec!["step_05_fix".to_string()]),
        },
    ];
    let result = execute_action(&HookAction::Gwt(gwt_clauses), &ctx, &mut engine, None);
    assert!(
        matches!(&result, HookResult::RouteTo { targets } if targets == &vec!["step_05_fix".to_string()]),
        "70% coverage should route to fix"
    );
}

/// Regression test: Shell action with multiline stdout properly captures
/// the COVERAGE_GATE field for downstream GWT evaluation.
///
/// Confirmed behavior: SW2/SW3 shell hooks output multiple lines including
/// "COVERAGE_GATE: PASS" or "COVERAGE_GATE: FAIL". This test verifies the
/// shell bookmark captures the full multiline output.
#[test]
fn given_shell_multiline_output_when_captured_then_full_stdout_in_bookmark() {
    let tmp = tempfile::tempdir().unwrap();

    let shell_action = HookAction::Shell(ShellAction {
        command: r#"echo '=== MACHINE CHECK ==='; echo 'Tasks total: 10'; echo 'Tasks categorized: 9'; echo 'Coverage pct: 90'; echo 'COVERAGE_GATE: PASS'; echo '=== END CHECK ==='"#.to_string(),
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = before_ctx("evaluate");

    let result = execute_action(&shell_action, &ctx, &mut engine, None);
    assert!(matches!(result, HookResult::Continue), "Shell should return Continue");

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert!(stdout.contains("COVERAGE_GATE: PASS"), "Multiline output should contain gate field");
    assert!(stdout.contains("Coverage pct: 90"), "Multiline output should contain percentage");
    assert!(stdout.lines().count() >= 5, "Should capture all lines");
}

/// Regression test: Shell action with empty output (no tasks) doesn't crash
/// and produces a defined gate value.
#[test]
fn given_shell_empty_input_when_coverage_computed_then_no_div_by_zero() {
    let tmp = tempfile::tempdir().unwrap();

    let empty_tasks = tmp.path().join("tasks.txt");
    let empty_cat = tmp.path().join("categorized.md");
    fs::write(&empty_tasks, "").unwrap();
    fs::write(&empty_cat, "").unwrap();

    // Use same pattern as SW2/SW3 evaluator — must not divide by zero
    let shell_cmd = format!(
        "TOTAL=$(grep -o 'T[0-9]*' '{tasks}' 2>/dev/null | sort -u | wc -l) && \
         COVERED=$(grep -o '### T[0-9]*' '{cat}' 2>/dev/null | grep -o 'T[0-9]*' | sort -u | wc -l) && \
         PCT=$((TOTAL > 0 ? COVERED * 100 / TOTAL : 0)) && \
         printf '%s' \"$PCT\"",
        tasks = empty_tasks.to_string_lossy(),
        cat = empty_cat.to_string_lossy()
    );

    let shell_action = HookAction::Shell(ShellAction {
        command: shell_cmd,
        args: Some(vec![]),
        working_dir: Some(tmp.path().to_string_lossy().into_owned()),
        env: Some(HashMap::new()),
        fail_on_error: Some(false),
    });

    let mut engine = HookEngine::new();
    let ctx = before_ctx("evaluate");

    let result = execute_action(&shell_action, &ctx, &mut engine, None);
    assert!(matches!(result, HookResult::Continue), "Shell with empty input should not fail");

    let bookmark = engine.get_bookmark("shell_output").expect("shell_output should exist");
    let stdout = bookmark.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(stdout, "0", "Empty input should produce 0% not divide-by-zero error");
}
