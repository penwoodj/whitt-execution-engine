use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Step type classification for hooks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepType {
    Generative,
    Tool,
    ControlFlow,
}

impl StepType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepType::Generative => "generative",
            StepType::Tool => "tool",
            StepType::ControlFlow => "control_flow",
        }
    }
}

/// Context for `before_step_starts` trigger.
#[derive(Debug, Clone)]
pub struct BeforeStepStartsContext {
    pub step_name: String,
    pub step_type: StepType,
    pub model_name: String,
    pub prompt_preview: String,
    pub workflow_variables: HashMap<String, JsonValue>,
}

impl BeforeStepStartsContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "step_type": self.step_type.as_str(),
            "model_name": self.model_name,
            "prompt_preview": self.prompt_preview,
            "workflow_variables": self.workflow_variables,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "step_type" => Some(self.step_type.as_str().to_string()),
            "model_name" => Some(self.model_name.clone()),
            "prompt_preview" => Some(self.prompt_preview.clone()),
            _ => None,
        }
    }
}

/// Context for `during_step_streaming` trigger.
#[derive(Debug, Clone)]
pub struct DuringStepStreamingContext {
    pub step_name: String,
    pub chunk_text: String,
    pub tokens_so_far: u32,
    pub elapsed_ms: u64,
}

impl DuringStepStreamingContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "chunk_text": self.chunk_text,
            "tokens_so_far": self.tokens_so_far,
            "elapsed_ms": self.elapsed_ms,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "chunk_text" => Some(self.chunk_text.clone()),
            "tokens_so_far" => Some(self.tokens_so_far.to_string()),
            "elapsed_ms" => Some(self.elapsed_ms.to_string()),
            _ => None,
        }
    }
}

/// Context for `after_step_succeeds` trigger.
#[derive(Debug, Clone)]
pub struct AfterStepSucceedsContext {
    pub step_name: String,
    pub output: String,
    pub duration_ms: u64,
    pub quality_score: Option<f32>,
    pub token_count: u32,
    pub model_name: String,
}

impl AfterStepSucceedsContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "output": self.output,
            "duration_ms": self.duration_ms,
            "quality_score": self.quality_score,
            "token_count": self.token_count,
            "model_name": self.model_name,
            "json_parsable": serde_json::from_str::<serde_json::Value>(&self.output).is_ok(),
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "output" => Some(self.output.clone()),
            "duration_ms" => Some(self.duration_ms.to_string()),
            "quality_score" => self.quality_score.map(|s| s.to_string()),
            "token_count" => Some(self.token_count.to_string()),
            "model_name" => Some(self.model_name.clone()),
            "json_parsable" => Some(serde_json::from_str::<serde_json::Value>(&self.output).is_ok().to_string()),
            _ => None,
        }
    }
}

/// Error details for failed steps.
#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub is_retryable: bool,
    pub count: u32,
}

/// Context for `after_step_fails` trigger.
#[derive(Debug, Clone)]
pub struct AfterStepFailsContext {
    pub step_name: String,
    pub error_type: String,
    pub error_message: String,
    pub error: ErrorDetails,
    pub attempt_number: u32,
    pub model_name: String,
}

impl AfterStepFailsContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "error_type": self.error_type,
            "error_message": self.error_message,
            "error": {
                "is_retryable": self.error.is_retryable,
                "count": self.error.count,
            },
            "attempt_number": self.attempt_number,
            "model_name": self.model_name,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "error_type" => Some(self.error_type.clone()),
            "error_message" => Some(self.error_message.clone()),
            "error.is_retryable" => Some(self.error.is_retryable.to_string()),
            "error.count" => Some(self.error.count.to_string()),
            "attempt_number" => Some(self.attempt_number.to_string()),
            "model_name" => Some(self.model_name.clone()),
            _ => None,
        }
    }
}

/// Context for `after_all_retries_exhausted` trigger.
#[derive(Debug, Clone)]
pub struct AfterAllRetriesExhaustedContext {
    pub step_name: String,
    pub total_attempts: u32,
    pub last_error: String,
    pub last_error_type: String,
}

impl AfterAllRetriesExhaustedContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "total_attempts": self.total_attempts,
            "last_error": self.last_error,
            "last_error_type": self.last_error_type,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "total_attempts" => Some(self.total_attempts.to_string()),
            "last_error" => Some(self.last_error.clone()),
            "last_error_type" => Some(self.last_error_type.clone()),
            _ => None,
        }
    }
}

/// Context for `after_step_starts` trigger.
#[derive(Debug, Clone)]
pub struct AfterStepStartsContext {
    pub step_name: String,
    pub step_type: StepType,
}

impl AfterStepStartsContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "step_type": self.step_type.as_str(),
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "step_type" => Some(self.step_type.as_str().to_string()),
            _ => None,
        }
    }
}

/// Context for `before_gwt_evaluates` trigger.
#[derive(Debug, Clone)]
pub struct BeforeGwtEvaluatesContext {
    pub step_name: String,
    pub input_value: JsonValue,
}

impl BeforeGwtEvaluatesContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "input_value": self.input_value,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "input_value" => Some(self.input_value.to_string()),
            _ => None,
        }
    }
}

/// Context for `after_gwt_evaluates` trigger.
#[derive(Debug, Clone)]
pub struct AfterGwtEvaluatesContext {
    pub step_name: String,
    pub decision: String,
    pub quality_score: Option<f32>,
    pub route_target: String,
}

impl AfterGwtEvaluatesContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "decision": self.decision,
            "quality_score": self.quality_score,
            "route_target": self.route_target,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "decision" => Some(self.decision.clone()),
            "quality_score" => self.quality_score.map(|s| s.to_string()),
            "route_target" => Some(self.route_target.clone()),
            _ => None,
        }
    }
}

/// Context for `on_requires_failed` trigger.
#[derive(Debug, Clone)]
pub struct OnRequiresFailedContext {
    pub failed_step: String,
    pub reason: String,
    pub dependency_chain: Vec<String>,
}

impl OnRequiresFailedContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "failed_step": self.failed_step,
            "reason": self.reason,
            "dependency_chain": self.dependency_chain,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "failed_step" => Some(self.failed_step.clone()),
            "reason" => Some(self.reason.clone()),
            "dependency_chain" => Some(format!("{:?}", self.dependency_chain)),
            _ => None,
        }
    }
}

/// Context for `after_loop_iteration_fails` trigger.
#[derive(Debug, Clone)]
pub struct AfterLoopIterationFailsContext {
    pub step_name: String,
    pub iteration: u32,
    pub error_message: String,
    pub loop_type: String,
}

impl AfterLoopIterationFailsContext {
    pub fn to_json_value(&self) -> JsonValue {
        serde_json::json!({
            "step_name": self.step_name,
            "iteration": self.iteration,
            "error_message": self.error_message,
            "loop_type": self.loop_type,
        })
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "step_name" => Some(self.step_name.clone()),
            "iteration" => Some(self.iteration.to_string()),
            "error_message" => Some(self.error_message.clone()),
            "loop_type" => Some(self.loop_type.clone()),
            _ => None,
        }
    }
}

/// Workflow hook context enum wrapping all specific trigger contexts.
#[derive(Debug, Clone)]
pub enum WorkflowHookContext {
    BeforeStepStarts(BeforeStepStartsContext),
    DuringStepStreaming(DuringStepStreamingContext),
    AfterStepSucceeds(AfterStepSucceedsContext),
    AfterStepFails(AfterStepFailsContext),
    AfterAllRetriesExhausted(AfterAllRetriesExhaustedContext),
    AfterStepStarts(AfterStepStartsContext),
    BeforeGwtEvaluates(BeforeGwtEvaluatesContext),
    AfterGwtEvaluates(AfterGwtEvaluatesContext),
    OnRequiresFailed(OnRequiresFailedContext),
    AfterLoopIterationFails(AfterLoopIterationFailsContext),
}

impl WorkflowHookContext {
    pub fn trigger_name(&self) -> &str {
        match self {
            WorkflowHookContext::BeforeStepStarts(_) => "before_step_starts",
            WorkflowHookContext::DuringStepStreaming(_) => "during_step_streaming",
            WorkflowHookContext::AfterStepSucceeds(_) => "after_step_succeeds",
            WorkflowHookContext::AfterStepFails(_) => "after_step_fails",
            WorkflowHookContext::AfterAllRetriesExhausted(_) => "after_all_retries_exhausted",
            WorkflowHookContext::AfterStepStarts(_) => "after_step_starts",
            WorkflowHookContext::BeforeGwtEvaluates(_) => "before_gwt_evaluates",
            WorkflowHookContext::AfterGwtEvaluates(_) => "after_gwt_evaluates",
            WorkflowHookContext::OnRequiresFailed(_) => "on_requires_failed",
            WorkflowHookContext::AfterLoopIterationFails(_) => "after_loop_iteration_fails",
        }
    }

    pub fn to_json_value(&self) -> JsonValue {
        match self {
            WorkflowHookContext::BeforeStepStarts(ctx) => ctx.to_json_value(),
            WorkflowHookContext::DuringStepStreaming(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterStepSucceeds(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterStepFails(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterAllRetriesExhausted(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterStepStarts(ctx) => ctx.to_json_value(),
            WorkflowHookContext::BeforeGwtEvaluates(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterGwtEvaluates(ctx) => ctx.to_json_value(),
            WorkflowHookContext::OnRequiresFailed(ctx) => ctx.to_json_value(),
            WorkflowHookContext::AfterLoopIterationFails(ctx) => ctx.to_json_value(),
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match self {
            WorkflowHookContext::BeforeStepStarts(ctx) => ctx.get_field(field),
            WorkflowHookContext::DuringStepStreaming(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterStepSucceeds(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterStepFails(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterAllRetriesExhausted(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterStepStarts(ctx) => ctx.get_field(field),
            WorkflowHookContext::BeforeGwtEvaluates(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterGwtEvaluates(ctx) => ctx.get_field(field),
            WorkflowHookContext::OnRequiresFailed(ctx) => ctx.get_field(field),
            WorkflowHookContext::AfterLoopIterationFails(ctx) => ctx.get_field(field),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------
    // StepType tests
    // --------------------------------------------------------------------------

    #[test]
    fn steptype_generative_returns_correct_string() {
        // Given: Generative step type
        let step_type = StepType::Generative;

        // When: Converting to string
        let result = step_type.as_str();

        // Then: Returns "generative"
        assert_eq!(result, "generative");
    }

    #[test]
    fn steptype_tool_returns_correct_string() {
        // Given: Tool step type
        let step_type = StepType::Tool;

        // When: Converting to string
        let result = step_type.as_str();

        // Then: Returns "tool"
        assert_eq!(result, "tool");
    }

    #[test]
    fn steptype_controlflow_returns_correct_string() {
        // Given: ControlFlow step type
        let step_type = StepType::ControlFlow;

        // When: Converting to string
        let result = step_type.as_str();

        // Then: Returns "control_flow"
        assert_eq!(result, "control_flow");
    }

    // --------------------------------------------------------------------------
    // BeforeStepStartsContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn before_step_starts_to_json_value_contains_all_fields() {
        // Given: Context with all fields populated
        let mut vars = HashMap::new();
        vars.insert("var1".to_string(), JsonValue::String("value1".to_string()));
        let context = BeforeStepStartsContext {
            step_name: "analyze".to_string(),
            step_type: StepType::Generative,
            model_name: "llama-3.2".to_string(),
            prompt_preview: "Analyze code...".to_string(),
            workflow_variables: vars,
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "analyze");
        assert_eq!(json["step_type"], "generative");
        assert_eq!(json["model_name"], "llama-3.2");
        assert_eq!(json["prompt_preview"], "Analyze code...");
        assert_eq!(json["workflow_variables"]["var1"], "value1");
    }

    #[test]
    fn before_step_starts_get_field_returns_correct_values() {
        // Given: Context with fields
        let context = BeforeStepStartsContext {
            step_name: "step1".to_string(),
            step_type: StepType::Tool,
            model_name: "model-x".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        };

        // When: Getting fields
        let step_name = context.get_field("step_name");
        let step_type = context.get_field("step_type");
        let model = context.get_field("model_name");

        // Then: Returns correct values
        assert_eq!(step_name, Some("step1".to_string()));
        assert_eq!(step_type, Some("tool".to_string()));
        assert_eq!(model, Some("model-x".to_string()));
    }

    #[test]
    fn before_step_starts_get_field_returns_none_for_missing() {
        // Given: Context
        let context = BeforeStepStartsContext {
            step_name: "step1".to_string(),
            step_type: StepType::Generative,
            model_name: "model".to_string(),
            prompt_preview: "prompt".to_string(),
            workflow_variables: HashMap::new(),
        };

        // When: Getting non-existent field
        let result = context.get_field("non_existent");

        // Then: Returns None
        assert_eq!(result, None);
    }

    // --------------------------------------------------------------------------
    // DuringStepStreamingContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn during_step_streaming_to_json_value_contains_all_fields() {
        // Given: Context with streaming data
        let context = DuringStepStreamingContext {
            step_name: "generate".to_string(),
            chunk_text: "Hello".to_string(),
            tokens_so_far: 10,
            elapsed_ms: 500,
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "generate");
        assert_eq!(json["chunk_text"], "Hello");
        assert_eq!(json["tokens_so_far"], 10);
        assert_eq!(json["elapsed_ms"], 500);
    }

    #[test]
    fn during_step_streaming_get_field_returns_correct_values() {
        // Given: Context
        let context = DuringStepStreamingContext {
            step_name: "stream".to_string(),
            chunk_text: "chunk".to_string(),
            tokens_so_far: 100,
            elapsed_ms: 1000,
        };

        // When: Getting fields
        let tokens = context.get_field("tokens_so_far");
        let elapsed = context.get_field("elapsed_ms");

        // Then: Returns correct string values
        assert_eq!(tokens, Some("100".to_string()));
        assert_eq!(elapsed, Some("1000".to_string()));
    }

    // --------------------------------------------------------------------------
    // AfterStepSucceedsContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_step_succeeds_to_json_value_contains_all_fields() {
        // Given: Context with success data
        let context = AfterStepSucceedsContext {
            step_name: "validate".to_string(),
            output: "Validation passed".to_string(),
            duration_ms: 2000,
            quality_score: Some(0.95),
            token_count: 500,
            model_name: "llama-3.2".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "validate");
        assert_eq!(json["output"], "Validation passed");
        assert_eq!(json["duration_ms"], 2000);
        let qs = json["quality_score"].as_f64().unwrap();
        assert!((qs - 0.95).abs() < 0.01, "quality_score approx 0.95, got {}", qs);
        assert_eq!(json["token_count"], 500);
        assert_eq!(json["model_name"], "llama-3.2");
    }

    #[test]
    fn after_step_succeeds_quality_score_none_in_json() {
        // Given: Context without quality score
        let context = AfterStepSucceedsContext {
            step_name: "step".to_string(),
            output: "done".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: quality_score is null
        assert!(json["quality_score"].is_null());
    }

    #[test]
    fn after_step_succeeds_json_parsable_field() {
        let ctx_valid = AfterStepSucceedsContext {
            step_name: "s".into(), output: r#"{"key":"value"}"#.into(),
            duration_ms: 100, quality_score: None, token_count: 10, model_name: "m".into(),
        };
        assert_eq!(ctx_valid.get_field("json_parsable").unwrap(), "true");
        let json = ctx_valid.to_json_value();
        assert_eq!(json["json_parsable"], true);

        let ctx_invalid = AfterStepSucceedsContext {
            step_name: "s".into(), output: "not json at all".into(),
            duration_ms: 100, quality_score: None, token_count: 10, model_name: "m".into(),
        };
        assert_eq!(ctx_invalid.get_field("json_parsable").unwrap(), "false");
        let json = ctx_invalid.to_json_value();
        assert_eq!(json["json_parsable"], false);
    }

    // --------------------------------------------------------------------------
    // AfterStepFailsContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_step_fails_to_json_value_contains_nested_error() {
        // Given: Context with error details
        let context = AfterStepFailsContext {
            step_name: "failing_step".to_string(),
            error_type: "NetworkError".to_string(),
            error_message: "Connection timeout".to_string(),
            error: ErrorDetails {
                is_retryable: true,
                count: 2,
            },
            attempt_number: 1,
            model_name: "model".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains nested error object
        assert_eq!(json["step_name"], "failing_step");
        assert_eq!(json["error_type"], "NetworkError");
        assert_eq!(json["error"]["is_retryable"], true);
        assert_eq!(json["error"]["count"], 2);
        assert_eq!(json["attempt_number"], 1);
    }

    #[test]
    fn after_step_fails_get_field_handles_nested_error_fields() {
        // Given: Context
        let context = AfterStepFailsContext {
            step_name: "step".to_string(),
            error_type: "Error".to_string(),
            error_message: "msg".to_string(),
            error: ErrorDetails {
                is_retryable: false,
                count: 3,
            },
            attempt_number: 2,
            model_name: "model".to_string(),
        };

        // When: Getting nested error fields
        let retryable = context.get_field("error.is_retryable");
        let count = context.get_field("error.count");

        // Then: Returns correct values
        assert_eq!(retryable, Some("false".to_string()));
        assert_eq!(count, Some("3".to_string()));
    }

    // --------------------------------------------------------------------------
    // AfterAllRetriesExhaustedContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_all_retries_exhausted_to_json_value_contains_all_fields() {
        // Given: Context
        let context = AfterAllRetriesExhaustedContext {
            step_name: "failing".to_string(),
            total_attempts: 5,
            last_error: "All retries failed".to_string(),
            last_error_type: "PermanentError".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "failing");
        assert_eq!(json["total_attempts"], 5);
        assert_eq!(json["last_error"], "All retries failed");
        assert_eq!(json["last_error_type"], "PermanentError");
    }

    // --------------------------------------------------------------------------
    // AfterStepStartsContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_step_starts_to_json_value_minimal_context() {
        // Given: Minimal context
        let context = AfterStepStartsContext {
            step_name: "minimal".to_string(),
            step_type: StepType::ControlFlow,
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON has minimal fields
        assert_eq!(json["step_name"], "minimal");
        assert_eq!(json["step_type"], "control_flow");
    }

    // --------------------------------------------------------------------------
    // BeforeGwtEvaluatesContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn before_gwt_evaluates_to_json_value_contains_input_value() {
        // Given: Context with complex input
        let context = BeforeGwtEvaluatesContext {
            step_name: "gwt_step".to_string(),
            input_value: JsonValue::Object(serde_json::json!({"key": "value"}).as_object().unwrap().clone()),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON preserves input value
        assert_eq!(json["step_name"], "gwt_step");
        assert_eq!(json["input_value"]["key"], "value");
    }

    // --------------------------------------------------------------------------
    // AfterGwtEvaluatesContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_gwt_evaluates_to_json_value_contains_all_fields() {
        // Given: Context
        let context = AfterGwtEvaluatesContext {
            step_name: "decision".to_string(),
            decision: "proceed".to_string(),
            quality_score: Some(0.87),
            route_target: "next_step".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "decision");
        assert_eq!(json["decision"], "proceed");
        let qs = json["quality_score"].as_f64().unwrap();
        assert!((qs - 0.87).abs() < 0.01, "quality_score approx 0.87, got {}", qs);
        assert_eq!(json["route_target"], "next_step");
    }

    // --------------------------------------------------------------------------
    // OnRequiresFailedContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn on_requires_failed_to_json_value_contains_dependency_chain() {
        // Given: Context with dependency chain
        let context = OnRequiresFailedContext {
            failed_step: "dependent".to_string(),
            reason: "Dependency not satisfied".to_string(),
            dependency_chain: vec!["step1".to_string(), "step2".to_string()],
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains dependency chain array
        assert_eq!(json["failed_step"], "dependent");
        assert_eq!(json["reason"], "Dependency not satisfied");
        assert_eq!(json["dependency_chain"][0], "step1");
        assert_eq!(json["dependency_chain"][1], "step2");
    }

    // --------------------------------------------------------------------------
    // AfterLoopIterationFailsContext tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_loop_iteration_fails_to_json_value_contains_all_fields() {
        // Given: Context
        let context = AfterLoopIterationFailsContext {
            step_name: "loop_step".to_string(),
            iteration: 3,
            error_message: "Iteration failed".to_string(),
            loop_type: "validation".to_string(),
        };

        // When: Converting to JSON
        let json = context.to_json_value();

        // Then: JSON contains all fields
        assert_eq!(json["step_name"], "loop_step");
        assert_eq!(json["iteration"], 3);
        assert_eq!(json["error_message"], "Iteration failed");
        assert_eq!(json["loop_type"], "validation");
    }

    // --------------------------------------------------------------------------
    // WorkflowHookContext enum tests
    // --------------------------------------------------------------------------

    #[test]
    fn workflow_hook_context_trigger_name_returns_correct_name_for_all_variants() {
        // Given: Context variants
        let before = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: "s".to_string(),
            step_type: StepType::Generative,
            model_name: "m".to_string(),
            prompt_preview: "p".to_string(),
            workflow_variables: HashMap::new(),
        });
        let during = WorkflowHookContext::DuringStepStreaming(DuringStepStreamingContext {
            step_name: "s".to_string(),
            chunk_text: "c".to_string(),
            tokens_so_far: 0,
            elapsed_ms: 0,
        });
        let after_success = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "s".to_string(),
            output: "o".to_string(),
            duration_ms: 0,
            quality_score: None,
            token_count: 0,
            model_name: "m".to_string(),
        });

        // When: Getting trigger names
        let before_name = before.trigger_name();
        let during_name = during.trigger_name();
        let after_success_name = after_success.trigger_name();

        // Then: Returns correct trigger names
        assert_eq!(before_name, "before_step_starts");
        assert_eq!(during_name, "during_step_streaming");
        assert_eq!(after_success_name, "after_step_succeeds");
    }

    #[test]
    fn workflow_hook_context_to_json_value_delegates_to_inner_context() {
        // Given: Context with variant
        let ctx = WorkflowHookContext::AfterAllRetriesExhausted(AfterAllRetriesExhaustedContext {
            step_name: "step".to_string(),
            total_attempts: 3,
            last_error: "error".to_string(),
            last_error_type: "type".to_string(),
        });

        // When: Converting to JSON
        let json = ctx.to_json_value();

        // Then: JSON contains inner context data
        assert_eq!(json["step_name"], "step");
        assert_eq!(json["total_attempts"], 3);
    }

    #[test]
    fn workflow_hook_context_get_field_delegates_to_inner_context() {
        // Given: Context
        let ctx = WorkflowHookContext::AfterStepStarts(AfterStepStartsContext {
            step_name: "step".to_string(),
            step_type: StepType::Tool,
        });

        // When: Getting field
        let result = ctx.get_field("step_name");

        // Then: Returns value from inner context
        assert_eq!(result, Some("step".to_string()));
    }

    #[test]
    fn workflow_hook_context_all_trigger_names_unique() {
        // Given: All context variants
        let contexts = vec![
            WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
                step_name: "s".to_string(),
                step_type: StepType::Generative,
                model_name: "m".to_string(),
                prompt_preview: "p".to_string(),
                workflow_variables: HashMap::new(),
            }),
            WorkflowHookContext::DuringStepStreaming(DuringStepStreamingContext {
                step_name: "s".to_string(),
                chunk_text: "c".to_string(),
                tokens_so_far: 0,
                elapsed_ms: 0,
            }),
            WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: "s".to_string(),
                output: "o".to_string(),
                duration_ms: 0,
                quality_score: None,
                token_count: 0,
                model_name: "m".to_string(),
            }),
            WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
                step_name: "s".to_string(),
                error_type: "e".to_string(),
                error_message: "msg".to_string(),
                error: ErrorDetails { is_retryable: false, count: 0 },
                attempt_number: 0,
                model_name: "m".to_string(),
            }),
            WorkflowHookContext::AfterAllRetriesExhausted(AfterAllRetriesExhaustedContext {
                step_name: "s".to_string(),
                total_attempts: 0,
                last_error: "e".to_string(),
                last_error_type: "t".to_string(),
            }),
            WorkflowHookContext::AfterStepStarts(AfterStepStartsContext {
                step_name: "s".to_string(),
                step_type: StepType::ControlFlow,
            }),
            WorkflowHookContext::BeforeGwtEvaluates(BeforeGwtEvaluatesContext {
                step_name: "s".to_string(),
                input_value: JsonValue::Null,
            }),
            WorkflowHookContext::AfterGwtEvaluates(AfterGwtEvaluatesContext {
                step_name: "s".to_string(),
                decision: "d".to_string(),
                quality_score: None,
                route_target: "r".to_string(),
            }),
            WorkflowHookContext::OnRequiresFailed(OnRequiresFailedContext {
                failed_step: "s".to_string(),
                reason: "r".to_string(),
                dependency_chain: vec![],
            }),
            WorkflowHookContext::AfterLoopIterationFails(AfterLoopIterationFailsContext {
                step_name: "s".to_string(),
                iteration: 0,
                error_message: "e".to_string(),
                loop_type: "l".to_string(),
            }),
        ];

        // When: Collecting all trigger names
        let names: Vec<&str> = contexts.iter().map(|c| c.trigger_name()).collect();

        // Then: All names are unique
        let unique_names: std::collections::HashSet<_> = names.into_iter().collect();
        assert_eq!(unique_names.len(), 10);
    }

    // --------------------------------------------------------------------------
    // Missing get_field tests
    // --------------------------------------------------------------------------

    #[test]
    fn after_step_succeeds_get_field_returns_all_fields() {
        // Given: Context with quality_score Some
        let context = AfterStepSucceedsContext {
            step_name: "generate".to_string(),
            output: "{\"key\":\"val\"}".to_string(),
            duration_ms: 2000,
            quality_score: Some(0.85f32),
            token_count: 500,
            model_name: "llama-3.2".to_string(),
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let output = context.get_field("output");
        let duration_ms = context.get_field("duration_ms");
        let quality_score = context.get_field("quality_score");
        let token_count = context.get_field("token_count");
        let model_name = context.get_field("model_name");
        let json_parsable = context.get_field("json_parsable");

        // Then: Returns correct values
        assert_eq!(step_name, Some("generate".to_string()));
        assert_eq!(output, Some("{\"key\":\"val\"}".to_string()));
        assert_eq!(duration_ms, Some("2000".to_string()));
        assert_eq!(quality_score, Some("0.85".to_string()));
        assert_eq!(token_count, Some("500".to_string()));
        assert_eq!(model_name, Some("llama-3.2".to_string()));
        assert_eq!(json_parsable, Some("true".to_string()));
    }

    #[test]
    fn after_step_succeeds_get_field_quality_score_none_returns_none() {
        // Given: Context with quality_score None
        let context = AfterStepSucceedsContext {
            step_name: "step".to_string(),
            output: "done".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        };

        // When: Getting quality_score field
        let quality_score = context.get_field("quality_score");

        // Then: Returns None
        assert_eq!(quality_score, None);
    }

    #[test]
    fn after_step_succeeds_get_field_returns_none_for_missing() {
        // Given: Context
        let context = AfterStepSucceedsContext {
            step_name: "step".to_string(),
            output: "done".to_string(),
            duration_ms: 100,
            quality_score: None,
            token_count: 10,
            model_name: "model".to_string(),
        };

        // When: Getting non-existent field
        let result = context.get_field("non_existent");

        // Then: Returns None
        assert_eq!(result, None);
    }

    #[test]
    fn after_all_retries_exhausted_get_field_returns_all_fields() {
        // Given: Context
        let context = AfterAllRetriesExhaustedContext {
            step_name: "failing".to_string(),
            total_attempts: 5,
            last_error: "All retries failed".to_string(),
            last_error_type: "PermanentError".to_string(),
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let total_attempts = context.get_field("total_attempts");
        let last_error = context.get_field("last_error");
        let last_error_type = context.get_field("last_error_type");

        // Then: Returns correct values
        assert_eq!(step_name, Some("failing".to_string()));
        assert_eq!(total_attempts, Some("5".to_string()));
        assert_eq!(last_error, Some("All retries failed".to_string()));
        assert_eq!(last_error_type, Some("PermanentError".to_string()));
    }

    #[test]
    fn after_all_retries_exhausted_get_field_returns_none_for_missing() {
        // Given: Context
        let context = AfterAllRetriesExhaustedContext {
            step_name: "step".to_string(),
            total_attempts: 3,
            last_error: "error".to_string(),
            last_error_type: "type".to_string(),
        };

        // When: Getting non-existent field
        let result = context.get_field("non_existent");

        // Then: Returns None
        assert_eq!(result, None);
    }

    #[test]
    fn after_step_starts_get_field_returns_all_fields() {
        // Given: Context with Generative step type
        let context = AfterStepStartsContext {
            step_name: "minimal".to_string(),
            step_type: StepType::Generative,
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let step_type = context.get_field("step_type");

        // Then: Returns correct values
        assert_eq!(step_name, Some("minimal".to_string()));
        assert_eq!(step_type, Some("generative".to_string()));
    }

    #[test]
    fn before_gwt_evaluates_get_field_returns_all_fields() {
        // Given: Context with complex input
        let context = BeforeGwtEvaluatesContext {
            step_name: "gwt_step".to_string(),
            input_value: serde_json::json!({"key": "value"}),
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let input_value = context.get_field("input_value");

        // Then: Returns correct values
        assert_eq!(step_name, Some("gwt_step".to_string()));
        assert!(input_value.is_some());
        assert!(input_value.unwrap().contains("key"));
    }

    #[test]
    fn after_gwt_evaluates_get_field_returns_all_fields() {
        // Given: Context with quality_score Some
        let context = AfterGwtEvaluatesContext {
            step_name: "decision".to_string(),
            decision: "proceed".to_string(),
            quality_score: Some(0.87f32),
            route_target: "next_step".to_string(),
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let decision = context.get_field("decision");
        let quality_score = context.get_field("quality_score");
        let route_target = context.get_field("route_target");

        // Then: Returns correct values
        assert_eq!(step_name, Some("decision".to_string()));
        assert_eq!(decision, Some("proceed".to_string()));
        assert_eq!(quality_score, Some("0.87".to_string()));
        assert_eq!(route_target, Some("next_step".to_string()));
    }

    #[test]
    fn on_requires_failed_get_field_returns_all_fields() {
        // Given: Context with dependency chain
        let context = OnRequiresFailedContext {
            failed_step: "dependent".to_string(),
            reason: "Dependency not satisfied".to_string(),
            dependency_chain: vec!["dep1".to_string()],
        };

        // When: Getting all fields
        let failed_step = context.get_field("failed_step");
        let reason = context.get_field("reason");
        let dependency_chain = context.get_field("dependency_chain");

        // Then: Returns correct values
        assert_eq!(failed_step, Some("dependent".to_string()));
        assert_eq!(reason, Some("Dependency not satisfied".to_string()));
        assert!(dependency_chain.is_some());
        assert!(dependency_chain.unwrap().contains("dep1"));
    }

    #[test]
    fn after_loop_iteration_fails_get_field_returns_all_fields() {
        // Given: Context
        let context = AfterLoopIterationFailsContext {
            step_name: "loop_step".to_string(),
            iteration: 3,
            error_message: "Iteration failed".to_string(),
            loop_type: "validation".to_string(),
        };

        // When: Getting all fields
        let step_name = context.get_field("step_name");
        let iteration = context.get_field("iteration");
        let error_message = context.get_field("error_message");
        let loop_type = context.get_field("loop_type");

        // Then: Returns correct values
        assert_eq!(step_name, Some("loop_step".to_string()));
        assert_eq!(iteration, Some("3".to_string()));
        assert_eq!(error_message, Some("Iteration failed".to_string()));
        assert_eq!(loop_type, Some("validation".to_string()));
    }
}