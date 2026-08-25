use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value as JsonValue;

use super::schema::{ComparisonOperator, LogLevel, UserInputType};
pub use super::execution::RetryConfig;

/// Agentic workflow configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgenticWorkflow {
    #[serde(default)]
    pub steps: HashMap<String, WorkflowStep>,
    #[serde(default)]
    pub hardcoded_values: Option<HashMap<String, JsonValue>>,
    #[serde(default)]
    pub inputs: Option<HashMap<String, WorkflowInput>>,
    #[serde(default)]
    pub user_inputs: Option<UserInputConfig>,
    #[serde(default)]
    pub retry: Option<RetryConfig>,
    #[serde(default)]
    pub r#when: Option<HashMap<String, Vec<HookAction>>>,
}

/// Sub-workflow reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubWorkflowRef {
    /// External sub-workflow reference (path to YAML file).
    Path(String),
    /// Inline sub-workflow definition.
    Inline(InlineSubWorkflow),
}

/// Inline sub-workflow definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct InlineSubWorkflow {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub default_inputs: Option<HashMap<String, JsonValue>>,
    #[serde(default)]
    pub inputs: Option<HashMap<String, WorkflowInput>>,
    #[serde(default)]
    pub steps: Option<HashMap<String, WorkflowStep>>,
    #[serde(default)]
    pub r#when: Option<HashMap<String, Vec<HookAction>>>,
}

/// Workflow step.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WorkflowStep {
    #[serde(default)]
    pub generative_entity: Option<String>,
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(default)]
    pub model_overrides: Option<ModelOverrides>,
    #[serde(default)]
    pub tool: Option<String>,
    #[serde(default)]
    pub input: Option<JsonValue>,
    #[serde(default)]
    pub retry: Option<StepRetryConfig>,
    #[serde(default)]
    pub depends_on: Option<Vec<String>>,
    #[serde(default)]
    pub requires: Option<Vec<RequireCondition>>,
    #[serde(default)]
    pub r#when: Option<HashMap<String, Vec<HookAction>>>,
    #[serde(default)]
    pub r#loop: Option<LoopConfig>,
    #[serde(default)]
    pub sub_workflow: Option<String>,
    #[serde(default)]
    pub user_input: Option<UserInputPrompt>,
    #[serde(default)]
    pub on_requires_failed: Option<HashMap<String, Vec<HookAction>>>,
}

/// Model overrides.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ModelOverrides {
    #[serde(default)]
    pub max_turns: Option<u32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub top_k: Option<u32>,
}

/// Loop configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LoopConfig {
    #[serde(default)]
    pub validation: Option<ValidationLoopConfig>,
    #[serde(default)]
    pub count: Option<CountLoopConfig>,
    /// 🔵 DEFERRED: Modes for benchmark loops (gpu/cpu switching). Not yet in schema.
    #[serde(default)]
    pub modes: Option<Vec<String>>,
    /// 🔵 DEFERRED: Variable name for current mode in benchmark loops.
    #[serde(default)]
    pub mode_variable: Option<String>,
}

/// Validation loop configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ValidationLoopConfig {
    #[serde(default)]
    pub tolerance: Option<f32>,
    #[serde(default)]
    pub max_iterations: Option<u32>,
    #[serde(default)]
    pub exact_criteria: Option<Vec<ExactCriteria>>,
}

/// Exact criteria for validation loop.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExactCriteria {
    #[serde(default)]
    pub metric: Option<String>,
    #[serde(default)]
    pub operator: Option<ComparisonOperator>,
    #[serde(default)]
    pub target: Option<f32>,
}

/// Count loop configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CountLoopConfig {
    #[serde(default)]
    pub max_iterations: Option<u32>,
    #[serde(default)]
    pub iteration_variable: Option<String>,
}

/// Require condition — accepts string shorthand or full object form.
/// Schema reference: docs/schema/unified-workflow-schema.yml line 384-387
///   requires:
///     - analyze_code                        # string shorthand
///     - step: read_config                   # object form
///       condition: "result.config_loaded == true"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequireCondition {
    /// String shorthand: just the step name.
    Shorthand(String),
    /// Full form with step and optional condition.
    Full(FullRequireCondition),
}

/// Full require condition with explicit fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FullRequireCondition {
    #[serde(default)]
    pub step: Option<String>,
    #[serde(default)]
    pub condition: Option<String>,
}

/// Hook action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookAction {
    /// Log action.
    Log(LogAction),
    /// GWT (Given When Then) action.
    Gwt(Vec<GwtClause>),
    /// Append to action.
    AppendTo(AppendToAction),
    /// Save to action.
    SaveTo(SaveToAction),
    /// Route to action.
    RouteTo(RouteToAction),
    /// Bookmark action.
    Bookmark(BookmarkAction),
    /// Notify action.
    Notify(NotifyAction),
    /// Fail action.
    Fail(FailAction),
    /// Shell action.
    Shell(ShellAction),
    /// Skip step action.
    SkipStep(bool),
    /// Skip remaining action.
    SkipRemaining(bool),
    /// Configuration carrier for step iteration. Execution handled at runner level
    /// via `extract_iterate_values` (reads from `before_step_starts.iterate_values`);
    /// this variant exists for schema/serde compatibility and returns Continue when dispatched.
    IterateValues(std::collections::HashMap<String, Vec<String>>),
}

/// Log action.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LogAction {
    #[serde(default)]
    pub to_file_path: Option<String>,
    #[serde(default)]
    pub event_fields: Option<Vec<String>>,
    #[serde(default)]
    pub level: Option<LogLevel>,
}

/// GWT (Given When Then) clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GwtClause {
    #[serde(default)]
    pub given: Option<String>,
    #[serde(default)]
    pub r#when: Option<String>,
    pub r#then: RouteToAction,
}

/// Append to action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppendToAction {
    /// Append to variable.
    Variable(String),
    /// Append to file.
    FilePath(String),
    /// Append to both.
    Both(Vec<String>),
}

/// Save to action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SaveToAction {
    /// Save to variable.
    Variable(String),
    /// Save to file.
    FilePath(String),
    /// Save to both.
    Both(Vec<String>),
}

/// Route to action.
///
/// Accepts the shorthand forms (`then: X`, `then: [X, Y]`) and the verbose
/// documented form (`then: { route_to: X }`, unified-workflow-schema.yml
/// examples) — Issue R: the verbose form previously failed serde, so
/// workflows copied from the schema doc errored out.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RouteToAction {
    /// Route to single step.
    Single(String),
    /// Route to multiple steps.
    Multiple(Vec<String>),
}

impl<'de> serde::Deserialize<'de> for RouteToAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        fn list(arr: Vec<serde_json::Value>) -> Result<RouteToAction, String> {
            let mut targets = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    serde_json::Value::String(s) => targets.push(s),
                    other => {
                        return Err(format!(
                            "route target list entries must be strings, got: {}",
                            other
                        ))
                    }
                }
            }
            Ok(RouteToAction::Multiple(targets))
        }
        match value {
            serde_json::Value::String(s) => Ok(RouteToAction::Single(s)),
            serde_json::Value::Array(arr) => {
                list(arr).map_err(serde::de::Error::custom)
            }
            serde_json::Value::Object(mut map) => match map.remove("route_to") {
                Some(serde_json::Value::String(s)) => Ok(RouteToAction::Single(s)),
                Some(serde_json::Value::Array(arr)) => {
                    list(arr).map_err(serde::de::Error::custom)
                }
                Some(other) => Err(serde::de::Error::custom(format!(
                    "route_to must be a step name or list of step names, got: {}",
                    other
                ))),
                None => Err(serde::de::Error::custom(
                    "expected a step name, a list of step names, or the verbose form {route_to: ...}",
                )),
            },
            other => Err(serde::de::Error::custom(format!(
                "expected a step name, a list of step names, or the verbose form {{route_to: ...}}, got: {}",
                other
            ))),
        }
    }
}

/// Bookmark action.
///
/// Accepts multiple YAML forms:
/// - `bookmark: true` → store in engine bookmarks, no file
/// - `bookmark: "path"` → store in engine bookmarks AND save to file
/// - `bookmark: {path: "path"}` → same as string form
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookmarkAction {
    /// Boolean form: `bookmark: true`
    Flag(bool),
    /// String form: `bookmark: "path/to/file"`
    Path(String),
    /// Struct form: `bookmark: {path: "path/to/file"}`
    Detailed(BookmarkActionDetail),
}

/// Detailed bookmark action with explicit fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct BookmarkActionDetail {
    #[serde(default)]
    pub path: Option<String>,
}

/// Notify action.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct NotifyAction {
    #[serde(default)]
    pub message: Option<String>,
}

/// Fail action.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FailAction {
    #[serde(default)]
    pub message: Option<String>,
}

/// Shell action — execute an external command.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ShellAction {
    /// Command to execute (e.g., "free", "docker", "echo")
    pub command: String,
    /// Arguments to pass
    #[serde(default)]
    pub args: Option<Vec<String>>,
    /// Working directory
    #[serde(default)]
    pub working_dir: Option<String>,
    /// Environment variables
    #[serde(default)]
    pub env: Option<std::collections::HashMap<String, String>>,
    /// Whether to return Fail on non-zero exit (default: true)
    #[serde(default)]
    pub fail_on_error: Option<bool>,
}

/// User input configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct UserInputConfig {
    #[serde(default = "default_execution_mode")]
    pub execution_mode: ExecutionMode,
    #[serde(default)]
    pub workflow_level: Option<WorkflowLevelInputs>,
}

/// Execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    #[default]
    Interactive,
    Automated,
    Hybrid,
}

/// Workflow level user inputs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WorkflowLevelInputs {
    #[serde(default)]
    pub ask_at_start: Option<bool>,
    #[serde(default)]
    pub inputs: Option<Vec<WorkflowLevelInput>>,
}

/// Workflow level input definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WorkflowLevelInput {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default = "default_user_input_type")]
    pub r#type: UserInputType,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub validation: Option<InputValidation>,
}

/// Input validation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct InputValidation {
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub pattern: Option<String>,
}

/// User input prompt.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct UserInputPrompt {
    #[serde(default = "default_user_input_type")]
    pub r#type: UserInputType,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub default: Option<bool>,
}

/// Workflow input.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WorkflowInput {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub default: Option<JsonValue>,
}

/// Step retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct StepRetryConfig {
    #[serde(default)]
    pub max_attempts: Option<u32>,
    #[serde(default)]
    pub backoff: Option<super::schema::BackoffStrategy>,
    #[serde(default)]
    pub initial_delay: Option<String>,
    #[serde(default)]
    pub max_delay: Option<String>,
    #[serde(default)]
    pub multiplier: Option<f32>,
    #[serde(default)]
    pub jitter: Option<bool>,
    #[serde(default)]
    pub level: Option<super::schema::RetryLevel>,
    #[serde(default)]
    pub adjustment_strategy: Option<super::schema::RetryAdjustment>,
    #[serde(default)]
    pub tolerance_adjustment: Option<f32>,
    #[serde(default)]
    pub checkpoint_after_retry: Option<bool>,
}

// ============================================================================
// Default functions
// ============================================================================

fn default_user_input_type() -> UserInputType {
    UserInputType::Confirm
}

fn default_execution_mode() -> ExecutionMode {
    ExecutionMode::Interactive
}
