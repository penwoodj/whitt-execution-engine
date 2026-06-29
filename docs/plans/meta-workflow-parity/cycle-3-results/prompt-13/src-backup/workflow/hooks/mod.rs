//! Hook execution engine and control flow types.
//!
//! This module provides:
//! - `HookResult`: Control flow enum for hook execution outcomes
//! - `NotifyMessage`: Message type for sub-workflow notifications
//! - `HookEngine`: Engine for hook execution with state management
//!
//! See execution protocol in docs/schema/unified-workflow-schema.yml lines 384-432

use serde::Serialize;
use std::collections::HashMap;

pub mod context;
pub mod gwt;
pub mod actions;

/// Control flow result from hook execution.
///
/// Determines how workflow execution proceeds after hook processing.
/// Hooks can modify execution flow by returning different variants.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum HookResult {
    /// Continue execution normally.
    #[default]
    Continue,
    /// Skip this step, proceed to next.
    SkipStep,
    /// Skip current loop iteration (not whole loop).
    SkipLoop,
    /// Skip all remaining steps in workflow.
    SkipRemaining,
    /// Fail the workflow with a reason.
    Fail { reason: String },
    /// Route to different step(s).
    RouteTo { targets: Vec<String> },
}

impl HookResult {
    /// Merge two HookResults, returning the dominant result.
    ///
    /// Priority order (highest to lowest):
    /// 1. Fail - always wins, terminates workflow
    /// 2. SkipRemaining - terminates remaining execution
    /// 3. RouteTo - changes execution path
    /// 4. SkipLoop - skips current iteration
    /// 5. SkipStep - skips single step
    /// 6. Continue - identity element
    ///
    /// This allows multiple hooks to express control flow intentions,
    /// with the most restrictive outcome taking precedence.
    pub fn merge(current: HookResult, new: HookResult) -> HookResult {
        if current.is_terminal() {
            return current;
        }

        if matches!(&new, HookResult::Fail { .. }) {
            return new;
        }

        if matches!(&current, HookResult::Fail { .. }) {
            return current;
        }

        match (&current, &new) {
            (_, HookResult::SkipRemaining) => HookResult::SkipRemaining,
            (HookResult::SkipRemaining, _) => current.clone(),
            (HookResult::Fail { .. }, _) => current.clone(),
            (_, HookResult::Fail { .. }) => new.clone(),
            (_, HookResult::RouteTo { .. }) => new.clone(),
            (HookResult::RouteTo { .. }, _) => current.clone(),
            (_, HookResult::SkipLoop) => HookResult::SkipLoop,
            (HookResult::SkipLoop, _) => current.clone(),
            (_, HookResult::SkipStep) => HookResult::SkipStep,
            (HookResult::SkipStep, _) => current.clone(),
            (HookResult::Continue, _) => new.clone(),
        }
    }

    /// Check if this result allows normal continuation.
    pub fn is_continue(&self) -> bool {
        matches!(self, HookResult::Continue)
    }

    /// Check if this result is terminal (terminates workflow).
    ///
    /// Terminal results: SkipRemaining, Fail
    pub fn is_terminal(&self) -> bool {
        matches!(self, HookResult::SkipRemaining | HookResult::Fail { .. })
    }
}

/// Notification message from sub-workflow execution.
#[derive(Debug, Clone, Serialize)]
pub struct NotifyMessage {
    /// Source step that sent the notification.
    pub from_step: String,
    /// Notification message content.
    pub message: String,
    /// Optional output data from the notification.
    pub output: Option<String>,
}

/// Hook execution engine with state management.
///
/// Manages hook execution, bookmarks for state persistence,
/// and notifications for sub-workflow coordination.
pub struct HookEngine {
    /// Stored bookmarks from bookmark actions.
    pub bookmarks: HashMap<String, serde_json::Value>,
    /// Channel for sub-workflow notifications.
    pub notify_tx: Option<tokio::sync::mpsc::Sender<NotifyMessage>>,
}

impl HookEngine {
    /// Create a new HookEngine with empty state.
    pub fn new() -> Self {
        HookEngine {
            bookmarks: HashMap::new(),
            notify_tx: None,
        }
    }

    /// Create a HookEngine with a notification channel.
    pub fn with_notify_channel(tx: tokio::sync::mpsc::Sender<NotifyMessage>) -> Self {
        HookEngine {
            bookmarks: HashMap::new(),
            notify_tx: Some(tx),
        }
    }

    /// Get a bookmark value by name.
    pub fn get_bookmark(&self, name: &str) -> Option<&serde_json::Value> {
        self.bookmarks.get(name)
    }

    /// Store a bookmark value.
    pub fn store_bookmark(&mut self, name: String, value: serde_json::Value) {
        self.bookmarks.insert(name, value);
    }

    /// Resolve {{step.STEP_ID.output}} and {{bookmarks.KEY}} in a template string.
    pub fn resolve_templates(&self, template: &str) -> String {
        let mut result = template.to_string();
        tracing::info!("[resolve_templates] checking {} chars against {} bookmarks: {:?}", template.len(), self.bookmarks.len(), self.bookmarks.keys().collect::<Vec<_>>());
        for (key, value) in &self.bookmarks {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            let placeholder = format!("{{{{bookmarks.{}}}}}", key);
            result = result.replace(&placeholder, &value_str);
            let step_placeholder = format!("{{{{step.{}.output}}}}", key);
            result = result.replace(&step_placeholder, &value_str);
            let short_placeholder = format!("{{{{{}.output}}}}", key);
            result = result.replace(&short_placeholder, &value_str);
        }
        result = self.resolve_nested_bookmark_fields(&result);
        if result != template {
            tracing::info!("[resolve_templates] resolved: {} chars → {} chars", template.len(), result.len());
        }
        result
    }

    /// Resolve `{{bookmarks.KEY.field1.field2}}` by navigating JSON object.
    /// Required for shell hook output which stores `{stdout, stderr, exit_code, success}`.
    fn resolve_nested_bookmark_fields(&self, template: &str) -> String {
        let mut result = template.to_string();
        let re = regex::Regex::new(r"\{\{bookmarks\.([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)+)\}\}")
            .expect("bookmark regex must compile");
        for caps in re.captures_iter(&template.to_string()) {
            let full = caps.get(0).map(|m| m.as_str()).unwrap_or("");
            let path = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let parts: Vec<&str> = path.split('.').collect();
            if parts.is_empty() { continue; }
            let key = parts[0];
            if let Some(value) = self.bookmarks.get(key) {
                let mut current = value;
                for field in &parts[1..] {
                    current = match current {
                        serde_json::Value::Object(map) => map.get(*field).unwrap_or(&serde_json::Value::Null),
                        _ => &serde_json::Value::Null,
                    };
                }
                let resolved_str = match current {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Null => String::new(),
                    other => other.to_string(),
                };
                result = result.replace(full, &resolved_str);
            }
        }
        result
    }
}

impl Default for HookEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_continue_results_when_merged_then_returns_continue() {
        let current = HookResult::Continue;
        let new = HookResult::Continue;
        let result = HookResult::merge(current, new);
        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_continue_and_skip_remaining_when_merged_then_returns_skip_remaining() {
        let current = HookResult::Continue;
        let new = HookResult::SkipRemaining;
        let result = HookResult::merge(current, new);
        assert_eq!(result, HookResult::SkipRemaining);
    }

    #[test]
    fn given_skip_step_and_fail_when_merged_then_returns_fail() {
        let current = HookResult::SkipStep;
        let new = HookResult::Fail {
            reason: "Test failure".to_string(),
        };
        let result = HookResult::merge(current, new);
        assert_eq!(
            result,
            HookResult::Fail {
                reason: "Test failure".to_string()
            }
        );
    }

    #[test]
    fn given_route_to_when_is_continue_checked_then_returns_false() {
        let result = HookResult::RouteTo {
            targets: vec!["step_a".to_string(), "step_b".to_string()],
        };
        assert!(!result.is_continue());
    }

    #[test]
    fn given_fail_when_is_terminal_checked_then_returns_true() {
        let result = HookResult::Fail {
            reason: "Test failure".to_string(),
        };
        assert!(result.is_terminal());
    }

    #[test]
    fn given_skip_remaining_when_is_terminal_checked_then_returns_true() {
        let result = HookResult::SkipRemaining;
        assert!(result.is_terminal());
    }

    #[test]
    fn given_skip_step_when_is_terminal_checked_then_returns_false() {
        let result = HookResult::SkipStep;
        assert!(!result.is_terminal());
    }

    #[test]
    fn given_hook_engine_new_when_get_bookmark_any_then_returns_none() {
        let engine = HookEngine::new();
        assert!(engine.get_bookmark("any").is_none());
    }

    #[test]
    fn given_hook_engine_with_stored_bookmark_when_get_bookmark_then_returns_value() {
        let mut engine = HookEngine::new();
        let value = serde_json::json!({"test": "data"});
        engine.store_bookmark("test_bookmark".to_string(), value.clone());
        let retrieved = engine.get_bookmark("test_bookmark");
        assert_eq!(retrieved, Some(&value));
    }

    #[test]
    fn given_skip_remaining_and_skip_step_when_merged_then_skip_remaining_wins() {
        let current = HookResult::SkipRemaining;
        let new = HookResult::SkipStep;
        let result = HookResult::merge(current, new);
        assert_eq!(result, HookResult::SkipRemaining);
    }

    #[test]
    fn given_route_to_and_skip_loop_when_merged_then_route_to_wins() {
        let current = HookResult::RouteTo {
            targets: vec!["step_a".to_string()],
        };
        let new = HookResult::SkipLoop;
        let result = HookResult::merge(current, new);
        assert_eq!(
            result,
            HookResult::RouteTo {
                targets: vec!["step_a".to_string()]
            }
        );
    }

    #[test]
    fn given_hook_result_default_when_created_then_is_continue() {
        let result = HookResult::default();
        assert_eq!(result, HookResult::Continue);
    }

    #[test]
    fn given_hook_engine_default_when_created_then_has_empty_bookmarks() {
        let engine = HookEngine::default();
        assert!(engine.bookmarks.is_empty());
        assert!(engine.notify_tx.is_none());
    }
}
