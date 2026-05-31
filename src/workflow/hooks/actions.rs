//! Hook action implementations.
//!
//! This module provides execution logic for all hook action types.
//! Each action handler takes action data + context + HookEngine reference,
//! performs the action, and returns HookResult.

use super::{HookResult, HookEngine, NotifyMessage};
use crate::workflow::step::{
    HookAction, LogAction, AppendToAction, SaveToAction, BookmarkAction,
    BookmarkActionDetail, NotifyAction, FailAction, RouteToAction, GwtClause, ShellAction
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
        HookAction::Shell(shell_action) => execute_shell(shell_action, context, engine),
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
    engine: &mut HookEngine,
) -> HookResult {
    let output = extract_context_output(context);

    match action {
        AppendToAction::Variable(var_name) => {
            if let Some(var_key) = var_name.strip_prefix('$') {
                let existing = engine.get_bookmark(var_key)
                    .and_then(|v| v.as_str().map(String::from))
                    .unwrap_or_default();
                let combined = if existing.is_empty() {
                    output.clone()
                } else {
                    format!("{}\n{}", existing, output)
                };
                engine.store_bookmark(var_key.to_string(), serde_json::Value::String(combined));
            } else {
                if let Err(e) = append_to_file(var_name, &output) {
                    eprintln!("Failed to append to {}: {}", var_name, e);
                }
            }
        }
        AppendToAction::FilePath(path) => {
            if let Err(e) = append_to_file(path, &output) {
                eprintln!("Failed to append to {}: {}", path, e);
            }
        }
        AppendToAction::Both(targets) => {
            for target in targets {
                if let Some(var_name) = target.strip_prefix('$') {
                    // Variable reference
                    let existing = engine.get_bookmark(var_name)
                        .and_then(|v| v.as_str().map(String::from))
                        .unwrap_or_default();
                    let combined = if existing.is_empty() {
                        output.clone()
                    } else {
                        format!("{}\n{}", existing, output)
                    };
                    engine.store_bookmark(var_name.to_string(), serde_json::Value::String(combined));
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
            if let Some(var_key) = var_name.strip_prefix('$') {
                engine.store_bookmark(var_key.to_string(), json_value);
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
                if let Some(var_name) = target.strip_prefix('$') {
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
        let _ = tx.try_send(notify_msg);
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

/// Execute shell action.
///
/// Run external command via std::process::Command.
/// Store output in engine.bookmarks["shell_output"].
fn execute_shell(
    action: &ShellAction,
    _context: &WorkflowHookContext,
    engine: &mut HookEngine,
) -> HookResult {
    let mut cmd = std::process::Command::new(&action.command);

    if let Some(ref args) = action.args {
        cmd.args(args);
    }
    if let Some(ref dir) = action.working_dir {
        cmd.current_dir(dir);
    }
    if let Some(ref env_vars) = action.env {
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
    }

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            let success = output.status.success();

            let result_json = serde_json::json!({
                "stdout": stdout,
                "stderr": stderr,
                "exit_code": exit_code,
                "success": success,
            });
            engine.store_bookmark("shell_output".to_string(), result_json);

            info!(
                "[shell] {} {:?} → exit={}, stdout={} bytes, stderr={} bytes",
                action.command,
                action.args.as_deref().unwrap_or(&[]),
                exit_code,
                stdout.len(),
                stderr.len()
            );

            if !success {
                let fail = action.fail_on_error.unwrap_or(true);
                if fail {
                    return HookResult::Fail {
                        reason: format!("Shell command '{}' failed with exit code {}", action.command, exit_code),
                    };
                }
                info!("[shell] Command failed but fail_on_error=false, continuing");
            }

            HookResult::Continue
        }
        Err(e) => {
            let fail = action.fail_on_error.unwrap_or(true);
            if fail {
                return HookResult::Fail {
                    reason: format!("Shell command '{}' failed to execute: {}", action.command, e),
                };
            }
            info!("[shell] Command failed to execute: {}, continuing", e);
            HookResult::Continue
        }
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
    info!("[hook] before_gwt_evaluates: step={} clauses={}", 
        context.get_field("step_name").unwrap_or_default(), clauses.len());

    for clause in clauses {
        if let Some(ref given) = clause.given {
            if evaluate_gwt_condition(given, &json) {
                let target = match &clause.r#then {
                    RouteToAction::Single(t) => t.clone(),
                    RouteToAction::Multiple(ts) => ts.join(","),
                };
                info!("[hook] after_gwt_evaluates: decision=routed target={}", target);
                return execute_route_to(&clause.r#then, context, engine);
            }
        }
    }

    info!("[hook] after_gwt_evaluates: decision=continue (no clause matched)");
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
    fn given_append_to_variable_when_execute_then_stores_in_bookmarks() {
        let action = AppendToAction::Variable("$my_var".to_string());
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

        let bookmark = engine.get_bookmark("my_var");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().as_str(), Some("data"));
    }

    #[test]
    fn given_append_to_variable_with_dollar_prefix_when_execute_then_strips_dollar() {
        let action = AppendToAction::Variable("$my_var".to_string());
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
        let bookmark = engine.get_bookmark("my_var");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().as_str(), Some("data"));
    }

    #[test]
    fn given_append_to_variable_twice_when_execute_then_concatenates_values() {
        let action = AppendToAction::Variable("$accum".to_string());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "line1".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        // First append
        execute_append_to(&action, &context, &mut engine);
        let bookmark = engine.get_bookmark("accum");
        assert_eq!(bookmark.unwrap().as_str(), Some("line1"));

        // Second append with different output
        let context2 = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "line2".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        execute_append_to(&action, &context2, &mut engine);

        let bookmark = engine.get_bookmark("accum");
        assert_eq!(bookmark.unwrap().as_str(), Some("line1\nline2"));
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
    fn given_notify_with_channel_when_execute_then_message_sent() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<NotifyMessage>(1);
        let engine = HookEngine::with_notify_channel(tx);
        let action = NotifyAction {
            message: Some("Test notification".to_string()),
        };
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            output: "test_output".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });

        // Use tokio runtime for async test
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let result = execute_notify(&action, &context, &mut { engine });

            assert_eq!(result, HookResult::Continue);

            // Verify message received
            let msg = rx.try_recv().unwrap();
            assert_eq!(msg.from_step, "test_step");
            assert_eq!(msg.message, "Test notification");
            assert_eq!(msg.output, Some("test_output".to_string()));
        });
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

    #[test]
    fn given_save_to_both_when_execute_then_file_and_variable_stored() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.json").to_string_lossy().to_string();
        let action = SaveToAction::Both(vec![
            "$myvar".to_string(),
            file_path.clone(),
        ]);
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            output: "{\"saved\":true}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_save_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let bookmark = engine.get_bookmark("myvar");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().get("saved").and_then(|v| v.as_bool()), Some(true));
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"saved\""));
    }

    #[test]
    fn given_append_to_both_when_execute_then_file_and_variable_appended() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.txt").to_string_lossy().to_string();
        let action = AppendToAction::Both(vec![
            "$accum".to_string(),
            file_path.clone(),
        ]);
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "line 2".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_append_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("line 2"));
        let bookmark = engine.get_bookmark("accum");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().as_str(), Some("line 2"));
    }

    #[test]
    fn given_bookmark_path_string_when_execute_then_file_and_bookmark_stored() {
        let temp_dir = TempDir::new().unwrap();
        let bookmark_path = temp_dir.path().join("checkpoint.json").to_string_lossy().to_string();
        let action = BookmarkAction::Path(bookmark_path.clone());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            output: "{\"bookmark\":true}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_bookmark(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let bookmark = engine.get_bookmark("test_step");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().get("bookmark").and_then(|v| v.as_bool()), Some(true));
        assert!(Path::new(&bookmark_path).exists());
    }

    #[test]
    fn given_bookmark_detailed_without_path_when_execute_then_bookmark_stored_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let action = BookmarkAction::Detailed(BookmarkActionDetail {
            path: None,
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "mem_only".to_string(),
            output: "{\"saved\":true}".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_bookmark(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        let bookmark = engine.get_bookmark("mem_only");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().get("saved").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn given_skip_step_false_when_execute_then_returns_continue() {
        let action = HookAction::SkipStep(false);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_skip_remaining_false_when_execute_then_returns_continue() {
        let action = HookAction::SkipRemaining(false);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_fail_no_message_when_execute_then_returns_fail_with_default_reason() {
        let action = FailAction {
            message: None,
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
                assert_eq!(reason, "Hook execution failed");
            }
            _ => panic!("Expected Fail result"),
        }
    }

    #[test]
    fn given_log_with_warning_level_when_execute_then_log_contains_warn() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("warn.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: Some(LogLevel::Warning),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test_step".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        execute_log(&action, &context, &mut engine);

        let content = fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("WARN"));
    }

    #[test]
    fn given_log_with_error_level_when_execute_then_log_contains_error() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("error.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: Some(LogLevel::Error),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test_step".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        execute_log(&action, &context, &mut engine);

        let content = fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("ERROR"));
    }

    #[test]
    fn given_log_with_critical_level_when_execute_then_log_contains_critical() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("critical.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: Some(LogLevel::Critical),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test_step".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        execute_log(&action, &context, &mut engine);

        let content = fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("CRITICAL"));
    }

    #[test]
    fn given_log_with_debug_level_when_execute_then_log_contains_debug() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("debug.log").to_string_lossy().to_string();
        let action = LogAction {
            to_file_path: Some(log_path.clone()),
            event_fields: None,
            level: Some(LogLevel::Debug),
        };
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test_step".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        execute_log(&action, &context, &mut engine);

        let content = fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("DEBUG"));
    }

    #[test]
    fn given_multi_clause_gwt_when_execute_then_routes_to_second_matching_clause() {
        let clauses = vec![
            GwtClause {
                given: Some("step_name == \"other\"".to_string()),
                r#when: None,
                r#then: RouteToAction::Single("first_target".to_string()),
            },
            GwtClause {
                given: Some("step_name == \"test\"".to_string()),
                r#when: None,
                r#then: RouteToAction::Single("second_target".to_string()),
            },
            GwtClause {
                given: Some("step_name == \"another\"".to_string()),
                r#when: None,
                r#then: RouteToAction::Single("third_target".to_string()),
            },
        ];
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
                assert_eq!(targets, vec!["second_target"]);
            }
            _ => panic!("Expected RouteTo result"),
        }
    }

    #[test]
    fn given_iterate_values_when_execute_then_returns_continue() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), vec!["value1".to_string(), "value2".to_string()]);
        map.insert("key2".to_string(), vec!["value3".to_string()]);
        let action = HookAction::IterateValues(map);
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        let result = execute_action(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_save_to_variable_without_dollar_when_execute_then_treated_as_file_path() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("plain_path.txt").to_string_lossy().to_string();
        let action = SaveToAction::Variable("plain_path.txt".to_string());
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test".to_string(),
            output: "file content".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        });
        let mut engine = HookEngine::new();

        let result = execute_save_to(&action, &context, &mut engine);

        assert_eq!(result, HookResult::Continue);
        assert!(Path::new("plain_path.txt").exists());
        let content = fs::read_to_string("plain_path.txt").unwrap();
        assert!(content.contains("file content"));
        std::fs::remove_file("plain_path.txt").unwrap();
    }

    #[test]
    fn given_shell_echo_when_execute_then_output_captured() {
        let action = ShellAction {
            command: "echo".to_string(),
            args: Some(vec!["hello world".to_string()]),
            working_dir: None,
            env: None,
            fail_on_error: None,
        };
        let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "test".into(), step_type: StepType::Generative,
            model_name: "m".into(), prompt_preview: "p".into(), workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();
        let result = execute_shell(&action, &ctx, &mut engine);
        assert!(result.is_continue());
        let bookmark = engine.get_bookmark("shell_output").expect("bookmark");
        assert_eq!(bookmark["stdout"].as_str().unwrap().trim(), "hello world");
        assert_eq!(bookmark["exit_code"], 0);
        assert_eq!(bookmark["success"], true);
    }

    #[test]
    fn given_shell_false_command_when_execute_then_fails() {
        let action = ShellAction {
            command: "false".to_string(),
            args: None, working_dir: None, env: None, fail_on_error: None,
        };
        let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "t".into(), step_type: StepType::Generative,
            model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
        });
        let result = execute_shell(&action, &ctx, &mut HookEngine::new());
        assert!(matches!(result, HookResult::Fail { .. }));
    }

    #[test]
    fn given_shell_fail_on_error_false_when_execute_then_continues() {
        let action = ShellAction {
            command: "false".to_string(),
            fail_on_error: Some(false),
            ..Default::default()
        };
        let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "t".into(), step_type: StepType::Generative,
            model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
        });
        let result = execute_shell(&action, &ctx, &mut HookEngine::new());
        assert!(result.is_continue());
    }

    #[test]
    fn given_shell_missing_command_when_execute_then_fails() {
        let action = ShellAction {
            command: "nonexistent_command_xyz_12345".to_string(),
            ..Default::default()
        };
        let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "t".into(), step_type: StepType::Generative,
            model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
        });
        let result = execute_shell(&action, &ctx, &mut HookEngine::new());
        assert!(matches!(result, HookResult::Fail { .. }));
    }

    #[test]
    fn given_shell_with_env_when_execute_then_env_passed() {
        let mut env = HashMap::new();
        env.insert("MY_TEST_VAR".to_string(), "test_value_123".to_string());
        let action = ShellAction {
            command: "sh".to_string(),
            args: Some(vec!["-c".to_string(), "echo $MY_TEST_VAR".to_string()]),
            env: Some(env),
            ..Default::default()
        };
        let ctx = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "t".into(), step_type: StepType::Generative,
            model_name: "m".into(), prompt_preview: "".into(), workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();
        let result = execute_shell(&action, &ctx, &mut engine);
        assert!(result.is_continue());
        let bookmark = engine.get_bookmark("shell_output").expect("bookmark");
        assert_eq!(bookmark["stdout"].as_str().unwrap().trim(), "test_value_123");
    }
}