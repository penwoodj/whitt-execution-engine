//! Hook action implementations.
//!
//! This module provides execution logic for all hook action types.
//! Each action handler takes action data + context + HookEngine reference,
//! performs the action, and returns HookResult.

use super::{HookResult, HookEngine, NotifyMessage};
use crate::workflow::step::{
    HookAction, LogAction, AppendToAction, SaveToAction, BookmarkAction,
    BookmarkActionDetail, NotifyAction, FailAction, RouteToAction, GwtClause
};
use crate::workflow::schema::LogLevel;
use crate::workflow::hooks::context::WorkflowHookContext;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Execute a single hook action.
///
/// Returns HookResult indicating control flow effect.
/// Most actions return Continue, but some control flow actions
/// return SkipStep, SkipRemaining, Fail, or RouteTo.
pub fn execute_action(
    action: &HookAction,
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    match action {
        HookAction::Log(log_action) => execute_log(log_action, context, engine),
        HookAction::AppendTo(append_action) => execute_append_to(append_action, context, engine),
        HookAction::SaveTo(save_action) => execute_save_to(save_action, context, engine),
        HookAction::RouteTo(route_action) => execute_route_to(route_action, context, engine),
        HookAction::Bookmark(bookmark_action) => execute_bookmark(bookmark_action, context, engine),
        HookAction::Notify(notify_action) => execute_notify(notify_action, context, engine),
        HookAction::Fail(fail_action) => execute_fail(fail_action, context, engine),
        HookAction::SkipStep(skip) => execute_skip_step(*skip, context, engine),
        HookAction::SkipRemaining(skip) => execute_skip_remaining(*skip, context, engine),
        HookAction::Gwt(clauses) => execute_gwt(clauses, context, engine),
        HookAction::IterateValues(_) => HookResult::Continue,
    }
}

/// Execute log action.
///
/// Writes formatted log line to file path. Fields from context via get_field().
/// Level-based formatting. Creates parent dirs if needed.
fn execute_log(
    action: &LogAction,
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let level = action.level.unwrap_or_default();
    let json = context.to_json_value();

    // Format log output
    let log_content = format_log(level, &action.event_fields, context, &json);

    // Write to file if path specified
    if let Some(ref path) = action.to_file_path {
        if let Err(e) = write_log_to_file(path, &log_content) {
            eprintln!("Failed to write log to {}: {}", path, e);
        }
    }

    // Always log to tracing
    info!("Hook log: {}", log_content);

    HookResult::Continue
}

/// Format log content based on level and fields.
fn format_log(
    level: LogLevel,
    event_fields: &Option<Vec<String>>,
    context: &WorkflowHookContext,
    json: &serde_json::Value,
) -> String {
    let level_str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Debug => "DEBUG",
        LogLevel::Warning => "WARN",
        LogLevel::Error => "ERROR",
        LogLevel::Critical => "CRITICAL",
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let mut fields_str = String::new();
    if let Some(fields) = event_fields {
        for field in fields {
            if let Some(value) = context.get_field(field) {
                if !fields_str.is_empty() {
                    fields_str.push(' ');
                }
                fields_str.push_str(&format!("{}={}", field, value));
            }
        }
    }

    if fields_str.is_empty() {
        format!("[{} {}] {}", timestamp, level_str, json)
    } else {
        format!("[{} {}] {} {}", timestamp, level_str, fields_str, json)
    }
}

/// Write log content to file, creating parent dirs if needed.
fn write_log_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

/// Execute append to action.
///
/// Append context output to file path or variable.
/// Supports Variable/FilePath/Both variants.
fn execute_append_to(
    action: &AppendToAction,
    context: &WorkflowHookContext,
    _engine: &mut HookEngine,
) -> HookResult {
    let output = extract_context_output(context);

    match action {
        AppendToAction::Variable(var_name) => {
            // TODO: Store in workflow variables (requires engine ref to workflow)
            info!("Append to variable: {} (not yet implemented)", var_name);
        }
        AppendToAction::FilePath(path) => {
            if let Err(e) = append_to_file(path, &output) {
                eprintln!("Failed to append to {}: {}", path, e);
            }
        }
        AppendToAction::Both(targets) => {
            for target in targets {
                if target.starts_with('$') {
                    // Variable reference
                    let var_name = &target[1..];
                    info!("Append to variable: {} (not yet implemented)", var_name);
                } else {
                    // File path
                    if let Err(e) = append_to_file(target, &output) {
                        eprintln!("Failed to append to {}: {}", target, e);
                    }
                }
            }
        }
    }

    HookResult::Continue
}

/// Append content to file, creating parent dirs if needed.
fn append_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

/// Execute save to action.
///
/// Save context output to file path (overwrite, not append).
/// Supports Variable/FilePath/Both variants.
/// Stores in HookEngine bookmarks if variable name, writes to file if path.
/// Single strings: treated as file path (unless prefixed with $$ for variable).
fn execute_save_to(
    action: &SaveToAction,
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let output = extract_context_output(context);
    let json_value: serde_json::Value = serde_json::from_str(&output).unwrap_or_else(|_| {
        serde_json::Value::String(output.clone())
    });

    match action {
        SaveToAction::Variable(var_name) => {
            if var_name.starts_with('$') {
                engine.store_bookmark(var_name[1..].to_string(), json_value);
            } else {
                if let Err(e) = save_to_file(var_name, &output) {
                    eprintln!("Failed to save to {}: {}", var_name, e);
                }
            }
        }
        SaveToAction::FilePath(path) => {
            if let Err(e) = save_to_file(path, &output) {
                eprintln!("Failed to save to {}: {}", path, e);
            }
        }
        SaveToAction::Both(targets) => {
            for target in targets {
                if target.starts_with('$') {
                    let var_name = &target[1..];
                    engine.store_bookmark(var_name.to_string(), json_value.clone());
                } else {
                    if let Err(e) = save_to_file(target, &output) {
                        eprintln!("Failed to save to {}: {}", target, e);
                    }
                }
            }
        }
    }

    HookResult::Continue
}

/// Save content to file (overwrite), creating parent dirs if needed.
fn save_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;
    Ok(())
}

/// Execute route to action.
///
/// Return HookResult::RouteTo with target step names.
fn execute_route_to(
    action: &RouteToAction,
    _context: &WorkflowHookContext,
    _engine: &mut HookEngine,
) -> HookResult {
    let targets = match action {
        RouteToAction::Single(step_name) => vec![step_name.clone()],
        RouteToAction::Multiple(step_names) => step_names.clone(),
    };

    HookResult::RouteTo { targets }
}

/// Execute bookmark action.
///
/// Write checkpoint file to path. Content = step output serialized as JSON.
/// Also store in HookEngine.bookmarks HashMap.
fn execute_bookmark(
    action: &BookmarkAction,
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let output = extract_context_output(context);
    let json_value: serde_json::Value = serde_json::from_str(&output).unwrap_or_else(|_| {
        serde_json::Value::String(output.clone())
    });

    let step_name = context.get_field("step_name").unwrap_or_else(|| "unknown".to_string());
    let formatted_json = serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| output.clone());

    engine.store_bookmark(step_name.clone(), json_value);

    let file_path: Option<&str> = match action {
        BookmarkAction::Flag(_) => None,
        BookmarkAction::Path(path) => Some(path),
        BookmarkAction::Detailed(detail) => detail.path.as_deref(),
    };

    if let Some(path) = file_path {
        if let Err(e) = save_to_file(path, &formatted_json) {
            eprintln!("Failed to write bookmark to {}: {}", path, e);
        }
    }

    HookResult::Continue
}

/// Execute notify action.
///
/// Send NotifyMessage through HookEngine.notify_tx channel.
/// If no channel, log the notification.
fn execute_notify(
    action: &NotifyAction,
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let message = action.message.as_deref()
        .unwrap_or("Sub-workflow completed");

    let step_name = context.get_field("step_name")
        .unwrap_or_else(|| "unknown".to_string());

    let output = extract_context_output(context);

    let notify_msg = NotifyMessage {
        from_step: step_name,
        message: message.to_string(),
        output: Some(output),
    };

    if let Some(ref tx) = engine.notify_tx {
        // TODO: Handle async send properly
        // For now, just log that we would send
        info!("Would send notification: {:?}", notify_msg);
    } else {
        info!("No notification channel, logging: {:?}", notify_msg);
    }

    HookResult::Continue
}

/// Execute fail action.
///
/// Return HookResult::Fail with reason message.
fn execute_fail(
    action: &FailAction,
    _context: &WorkflowHookContext,
    _engine: &mut HookEngine,
) -> HookResult {
    let reason = action.message.as_deref()
        .unwrap_or("Hook execution failed");

    HookResult::Fail {
        reason: reason.to_string(),
    }
}

/// Execute skip step action.
///
/// If true, return HookResult::SkipStep.
fn execute_skip_step(
    skip: bool,
    _context: &WorkflowHookContext,
    _engine: &mut HookEngine,
) -> HookResult {
    if skip {
        HookResult::SkipStep
    } else {
        HookResult::Continue
    }
}

/// Execute skip remaining action.
///
/// If true, return HookResult::SkipRemaining.
fn execute_skip_remaining(
    skip: bool,
    _context: &WorkflowHookContext,
    _engine: &mut HookEngine,
) -> HookResult {
    if skip {
        HookResult::SkipRemaining
    } else {
        HookResult::Continue
    }
}

/// Execute GWT (Given-When-Then) clauses.
///
/// For each clause, if `given` expression evaluates to true against
/// context.to_json_value(), return the `then` action's result.
///
/// Placeholder: Checks simple `field == value` patterns using string split on "==".
/// The full evaluator from Phase 1 will be integrated later.
fn execute_gwt(
    clauses: &[GwtClause],
    context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let json = context.to_json_value();

    for clause in clauses {
        if let Some(ref given) = clause.given {
            if evaluate_gwt_condition(given, &json) {
                // Evaluate the `then` clause
                return execute_route_to(&clause.r#then, context, engine);
            }
        }
    }

    HookResult::Continue
}

fn evaluate_gwt_condition(condition: &str, json: &serde_json::Value) -> bool {
    super::gwt::evaluate(condition, json).unwrap_or(false)
}

/// Extract output string from context based on context type.
fn extract_context_output(context: &WorkflowHookContext) -> String {
    match context {
        WorkflowHookContext::AfterStepSucceeds(ctx) => ctx.output.clone(),
        WorkflowHookContext::AfterStepFails(ctx) => ctx.error_message.clone(),
        WorkflowHookContext::DuringStepStreaming(ctx) => ctx.chunk_text.clone(),
        _ => context.to_json_value().to_string(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::hooks::context::{
        AfterStepSucceedsContext, BeforeStepStartsContext, StepType
    };
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn given_log_action_with_file_path_when_execute_log_then_file_created() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: Some(LogLevel::Info),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test_step".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_log(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        assert!(Path::new(&log_path).exists());
    }

    #[test]
    fn given_log_action_with_nested_path_when_execute_log_then_dirs_created() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("nested/dir/test.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: None,
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_log(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        assert!(Path::new(&log_path).exists());
    }

    #[test]
    fn given_append_to_file_path_when_execute_then_appends_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.txt").to_string_lossy().to_string();
        let action = AppendToAction::FilePath(file_path.clone());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "line 1".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        execute_append_to(&action, &context, &mut engine);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("line 1"));
    }

    #[test]
    fn given_append_to_variable_when_execute_then_returns_continue() {
        let action = AppendToAction::Variable("my_var".to_string());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "data".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_append_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_save_to_file_path_when_execute_then_overwrites_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.json").to_string_lossy().to_string();
        let action = SaveToAction::FilePath(file_path.clone());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "{\"key\":\"value\"}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_save_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"key\""));
    }

    #[test]
    fn given_save_to_variable_when_execute_then_stores_in_bookmarks() {
        let action = SaveToAction::Variable("$checkpoint".to_string());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            output: "{\"status\":\"done\"}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_save_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let bookmark = engine.get_bookmark("checkpoint");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().get("status").and_then(|v| v.as_str()), Some("done"));
    }

    #[test]
    fn given_route_to_single_when_execute_then_returns_single_target() {
        let action = RouteToAction::Single("next_step".to_string());
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_route_to(&action, &context, &mut engine);

        match result {
            HookResult::RouteTo { targets } => {
                assert_eq!(targets, vec!["next_step"]);
            }
            _ => panic!("Expected RouteTo result"),
        }
    }

    #[test]
    fn given_route_to_multiple_when_execute_then_returns_multiple_targets() {
        let action = RouteToAction::Multiple(vec![
            "step_a".to_string(),
            "step_b".to_string(),
            "step_c".to_string(),
        ]);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_route_to(&action, &context, &mut engine);

        match result {
            HookResult::RouteTo { targets } => {
                assert_eq!(targets.len(), 3);
                assert!(targets.contains(&"step_a".to_string()));
                assert!(targets.contains(&"step_b".to_string()));
                assert!(targets.contains(&"step_c".to_string()));
            }
            _ => panic!("Expected RouteTo result"),
        }
    }

    #[test]
    fn given_bookmark_with_path_when_execute_then_writes_checkpoint_file() {
        let temp_dir = TempDir::new().unwrap();
        let bookmark_path = temp_dir.path().join("checkpoint.json").to_string_lossy().to_string();
        let action = BookmarkAction::Detailed(BookmarkActionDetail {
            path: Some(bookmark_path.clone()),
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            output: "{\"checkpoint\":true}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_bookmark(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        assert!(Path::new(&bookmark_path).exists());
        let content = fs::read_to_string(&bookmark_path).unwrap();
        assert!(content.contains("\"checkpoint\""));
    }

    #[test]
    fn given_bookmark_without_path_when_execute_then_stores_in_engine() {
        let action = BookmarkAction::Flag(true);
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "checkpoint_step".to_string(),
            output: "{\"saved\":true}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_bookmark(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let bookmark = engine.get_bookmark("checkpoint_step");
        assert!(bookmark.is_some());
    }

    #[test]
    fn given_notify_without_channel_when_execute_then_returns_continue() {
        let action = NotifyAction::default();
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "output".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_notify(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_fail_with_message_when_execute_then_returns_fail_with_reason() {
        let action = FailAction {
            message: Some("Validation failed".to_string()),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_fail(&action, &context, &mut engine);

        match result {
            HookResult::Fail { reason } => {
                assert_eq!(reason, "Validation failed");
            }
            _ => panic!("Expected Fail result"),
        }
    }

    #[test]
    fn given_skip_step_true_when_execute_then_returns_skip_step() {
        let action = HookAction::SkipStep(true);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        assert_eq!(result, HookResult::SkipStep);
    }

    #[test]
    fn given_skip_remaining_true_when_execute_then_returns_skip_remaining() {
        let action = HookAction::SkipRemaining(true);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        assert_eq!(result, HookResult::SkipRemaining);
    }

    #[test]
    fn given_gwt_clause_matching_when_execute_then_routes_to_target() {
        let clauses = vec![GwtClause {
            given: Some("step_name == \"test\"".to_string()),
            r#when: None,
            r#then: RouteToAction::Single("next_step".to_string()),
        }];
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_gwt(&clauses, &context, &mut engine);

        match result {
            HookResult::RouteTo { targets } => {
                assert_eq!(targets, vec!["next_step"]);
            }
            _ => panic!("Expected RouteTo result"),
        }
    }

    #[test]
    fn given_gwt_clause_not_matching_when_execute_then_returns_continue() {
        let clauses = vec![GwtClause {
            given: Some("step_name == \"other\"".to_string()),
            r#when: None,
            r#then: RouteToAction::Single("skip_step".to_string()),
        }];
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_gwt(&clauses, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_gwt_invalid_condition_when_execute_then_treats_as_false() {
        let clauses = vec![GwtClause {
            given: Some("invalid condition syntax".to_string()),
            r#when: None,
            r#then: RouteToAction::Single("should_not_match".to_string()),
        }];
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_gwt(&clauses, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_fail_action_when_execute_action_then_returns_fail() {
        let action = HookAction::Fail(FailAction {
            message: Some("Critical error".to_string()),
        });
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        match result {
            HookResult::Fail { reason } => {
                assert_eq!(reason, "Critical error");
            }
            _ => panic!("Expected Fail result"),
        }
    }
}