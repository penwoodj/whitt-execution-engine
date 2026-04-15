# Task 00: Autonomous Loop Contracts

**Estimated Time**: 5 days
**Priority**: HIGH - Foundation for all autonomy features
**Dependencies**: None (but depends on all prior phases being complete)

## Overview

Implement the contract system for autonomous loops that defines goals, stop conditions, and autonomy levels. This is the foundation for ALL autonomy features.

## Files

### Create
- `src/autonomy/mod.rs` - Module exports
- `src/autonomy/contracts.rs` - Core contract types
- `src/autonomy/contracts/parser.rs` - YAML parser for contracts
- `src/autonomy/contracts/validator.rs` - Contract validation logic
- `src/autonomy/contracts/enforcer.rs` - Bounded execution enforcement
- `src/autonomy/contracts/goal.rs` - Goal definition and tracking
- `src/autonomy/contracts/level.rs` - Autonomy level definitions
- `tests/autonomy/contracts_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod autonomy;`
- `Cargo.toml` - Add dependencies: `serde`, `serde_yaml`, `chrono`, `thiserror`, `regex`

---

## Implementation Steps

### Step 1: Define core contract types

**File**: `src/autonomy/contracts.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Autonomous loop contract defining goals, limits, and autonomy level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutonomousLoopContract {
    pub version: String,
    pub autonomy_level: AutonomyLevel,
    pub goal: Goal,
    pub stop_conditions: Vec<StopCondition>,
    pub scope: ScopeBoundary,
    pub confidence_thresholds: Option<ConfidenceThresholds>,
}

/// Autonomy level determines human interaction requirements
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AutonomyLevel {
    /// Every action requires explicit human confirmation
    Low,
    /// Batch confirmation for groups of actions (configurable batch size)
    Medium { batch_size: usize },
    /// Bounded autonomous execution with periodic checkpoints and override available
    High { checkpoint_interval: Duration },
}

/// Goal definition for autonomous loop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Goal {
    pub goal_type: GoalType,
    pub target_workflow: String,
    pub success_criteria: Vec<SuccessCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum GoalType {
    WorkflowCompletion,
    TaskCompletion,
    MetricTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessCriterion {
    pub criterion_type: CriterionType,
    pub threshold: f64,
    pub operator: ComparisonOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CriterionType {
    DeploymentSuccess,
    TaskSuccess,
    MetricValue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ComparisonOperator {
    Above,
    Below,
    Equal,
}

/// Stop conditions that trigger autonomous loop termination
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum StopCondition {
    Time { max_duration: String },
    Iteration { max_actions: usize },
    Quality {
        metric: String,
        threshold: f64,
        operator: ComparisonOperator,
    },
    Intervention { stop_on_any: bool },
    Resource { budget: ResourceBudget },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceBudget {
    pub max_cost_usd: Option<f64>,
    pub max_cpu_hours: Option<f64>,
    pub max_memory_gb_hours: Option<f64>,
}

/// Scope boundary defining what the autonomous loop can modify
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBoundary {
    pub allowed_workflows: Vec<String>,
    pub allowed_actions: Vec<String>,
    pub forbidden_workflows: Vec<String>,
    pub forbidden_actions: Vec<String>,
}

/// Confidence thresholds for autonomous decision-making
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfidenceThresholds {
    pub action_threshold: f64,
    pub batch_threshold: f64,
    pub loop_threshold: f64,
}

/// Contract validation errors
#[derive(Debug, thiserror::Error)]
pub enum ContractValidationError {
    #[error("Invalid autonomy level: {0}")]
    InvalidAutonomyLevel(String),
    #[error("Missing goal definition")]
    MissingGoal,
    #[error("No stop conditions defined")]
    NoStopConditions,
    #[error("Stop condition out of range: {0}")]
    StopConditionOutOfRange(String),
    #[error("Invalid time duration: {0}")]
    InvalidDuration(String),
    #[error("Invalid threshold: {0}")]
    InvalidThreshold(f64),
    #[error("Scope conflict: {0}")]
    ScopeConflict(String),
}
```

**Test**: `tests/autonomy/contracts_test.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::autonomy::contracts::*;

    #[test]
    fn test_autonomy_level_serialization() {
        let low = AutonomyLevel::Low;
        let yaml = serde_yaml::to_string(&low).unwrap();
        assert_eq!(yaml, "low\n");
    }

    #[test]
    fn test_goal_creation() {
        let goal = Goal {
            goal_type: GoalType::WorkflowCompletion,
            target_workflow: "deploy_production".to_string(),
            success_criteria: vec![SuccessCriterion {
                criterion_type: CriterionType::DeploymentSuccess,
                threshold: 0.95,
                operator: ComparisonOperator::Above,
            }],
        };
        assert_eq!(goal.target_workflow, "deploy_production");
    }

    #[test]
    fn test_stop_condition_time() {
        let condition = StopCondition::Time {
            max_duration: "24h".to_string(),
        };
        assert!(matches!(condition, StopCondition::Time { .. }));
    }
}
```

---

### Step 2: Implement contract parser

**File**: `src/autonomy/contracts/parser.rs`

```rust
use super::contracts::*;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid YAML: {0}")]
    InvalidYaml(#[from] serde_yaml::Error),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid duration format: {0}")]
    InvalidDurationFormat(String),
}

impl ContractParser {
    pub fn parse(yaml_str: &str) -> Result<AutonomousLoopContract, ParseError> {
        let contract: AutonomousLoopContract = serde_yaml::from_str(yaml_str)?;
        Self::validate_structure(&contract)?;
        Ok(contract)
    }

    fn validate_structure(contract: &AutonomousLoopContract) -> Result<(), ParseError> {
        if contract.goal.target_workflow.is_empty() {
            return Err(ParseError::MissingField("goal.target_workflow".to_string()));
        }
        if contract.stop_conditions.is_empty() {
            return Err(ParseError::MissingField("stop_conditions".to_string()));
        }
        Ok(())
    }

    pub fn parse_duration(duration_str: &str) -> Result<Duration, ParseError> {
        let duration_str = duration_str.trim().to_lowercase();

        if duration_str.ends_with('h') {
            let hours: u64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::hours(hours as i64))
        } else if duration_str.ends_with('m') {
            let minutes: u64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::minutes(minutes as i64))
        } else if duration_str.ends_with('s') {
            let seconds: u64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::seconds(seconds as i64))
        } else {
            Err(ParseError::InvalidDurationFormat(duration_str))
        }
    }
}
```

**Test**: `tests/autonomy/contracts_test.rs`

```rust
#[test]
fn test_parse_valid_contract() {
    let yaml = r#"
version: "1.0"
autonomy_level: low
goal:
  type: WorkflowCompletion
  target_workflow: "deploy_production"
  success_criteria:
    - type: DeploymentSuccess
      threshold: 0.95
      operator: above
stop_conditions:
  - type: time
    max_duration: "24h"
scope:
  allowed_workflows: ["deploy_production"]
  allowed_actions: ["deploy", "verify"]
  forbidden_workflows: []
  forbidden_actions: []
"#;
    let result = ContractParser::parse(yaml);
    assert!(result.is_ok());
}

#[test]
fn test_parse_missing_goal() {
    let yaml = r#"
version: "1.0"
autonomy_level: low
stop_conditions:
  - type: time
    max_duration: "24h"
scope:
  allowed_workflows: ["*"]
  allowed_actions: ["*"]
  forbidden_workflows: []
  forbidden_actions: []
"#;
    let result = ContractParser::parse(yaml);
    assert!(result.is_err());
}
```

---

### Step 3: Implement contract validator

**File**: `src/autonomy/contracts/validator.rs`

```rust
use super::contracts::*;

pub struct ContractValidator;

impl ContractValidator {
    pub fn validate(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        Self::validate_autonomy_level(contract)?;
        Self::validate_goal(contract)?;
        Self::validate_stop_conditions(contract)?;
        Self::validate_scope(contract)?;
        Self::validate_confidence_thresholds(contract)?;
        Ok(())
    }

    fn validate_autonomy_level(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        match contract.autonomy_level {
            AutonomyLevel::Medium { batch_size } => {
                if batch_size == 0 {
                    return Err(ContractValidationError::InvalidAutonomyLevel(
                        "Medium autonomy requires batch_size > 0".to_string()
                    ));
                }
            }
            AutonomyLevel::High { checkpoint_interval } => {
                if checkpoint_interval.num_seconds() < 60 {
                    return Err(ContractValidationError::InvalidAutonomyLevel(
                        "High autonomy requires checkpoint_interval >= 60s".to_string()
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_goal(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        if contract.goal.target_workflow.is_empty() {
            return Err(ContractValidationError::MissingGoal);
        }
        if contract.goal.success_criteria.is_empty() {
            return Err(ContractValidationError::MissingGoal);
        }
        for criterion in &contract.goal.success_criteria {
            if criterion.threshold < 0.0 || criterion.threshold > 1.0 {
                return Err(ContractValidationError::InvalidThreshold(criterion.threshold));
            }
        }
        Ok(())
    }

    fn validate_stop_conditions(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        if contract.stop_conditions.is_empty() {
            return Err(ContractValidationError::NoStopConditions);
        }

        for condition in &contract.stop_conditions {
            match condition {
                StopCondition::Time { max_duration } => {
                    let duration = ContractParser::parse_duration(max_duration)
                        .map_err(|_| ContractValidationError::InvalidDuration(max_duration.clone()))?;
                    if duration.num_seconds() < 60 {
                        return Err(ContractValidationError::StopConditionOutOfRange(
                            "Time limit must be at least 60s".to_string()
                        ));
                    }
                }
                StopCondition::Iteration { max_actions } => {
                    if *max_actions == 0 {
                        return Err(ContractValidationError::StopConditionOutOfRange(
                            "Iteration limit must be > 0".to_string()
                        ));
                    }
                }
                StopCondition::Quality { threshold, .. } => {
                    if *threshold < 0.0 || *threshold > 1.0 {
                        return Err(ContractValidationError::InvalidThreshold(*threshold));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate_scope(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        // Check for conflicts between allowed and forbidden lists
        for forbidden in &contract.scope.forbidden_workflows {
            if contract.scope.allowed_workflows.contains(forbidden) {
                return Err(ContractValidationError::ScopeConflict(
                    format!("Workflow {} is both allowed and forbidden", forbidden)
                ));
            }
        }
        for forbidden in &contract.scope.forbidden_actions {
            if contract.scope.allowed_actions.contains(forbidden) {
                return Err(ContractValidationError::ScopeConflict(
                    format!("Action {} is both allowed and forbidden", forbidden)
                ));
            }
        }
        Ok(())
    }

    fn validate_confidence_thresholds(contract: &AutonomousLoopContract) -> Result<(), ContractValidationError> {
        if let Some(thresholds) = &contract.confidence_thresholds {
            for (name, value) in [
                ("action_threshold", thresholds.action_threshold),
                ("batch_threshold", thresholds.batch_threshold),
                ("loop_threshold", thresholds.loop_threshold),
            ] {
                if value < 0.0 || value > 1.0 {
                    return Err(ContractValidationError::InvalidThreshold(*value));
                }
            }
        }
        Ok(())
    }
}
```

**Test**: `tests/autonomy/contracts_test.rs`

```rust
#[test]
fn test_validate_valid_contract() {
    let contract = create_test_contract();
    let result = ContractValidator::validate(&contract);
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_autonomy_level() {
    let mut contract = create_test_contract();
    contract.autonomy_level = AutonomyLevel::Medium { batch_size: 0 };
    let result = ContractValidator::validate(&contract);
    assert!(result.is_err());
}

#[test]
fn test_validate_no_stop_conditions() {
    let mut contract = create_test_contract();
    contract.stop_conditions = vec![];
    let result = ContractValidator::validate(&contract);
    assert!(result.is_err());
}
```

---

### Step 4: Implement bounded execution enforcer

**File**: `src/autonomy/contracts/enforcer.rs`

```rust
use super::contracts::*;
use chrono::{DateTime, Utc};
use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool}};

#[derive(Debug, Clone)]
pub struct BoundedExecutionState {
    pub start_time: DateTime<Utc>,
    pub action_count: Arc<AtomicUsize>,
    pub intervention_count: Arc<AtomicUsize>,
    pub is_stopped: Arc<AtomicBool>,
}

pub struct BoundedExecutionEnforcer {
    contract: AutonomousLoopContract,
    state: BoundedExecutionState,
}

impl BoundedExecutionEnforcer {
    pub fn new(contract: AutonomousLoopContract) -> Self {
        let state = BoundedExecutionState {
            start_time: Utc::now(),
            action_count: Arc::new(AtomicUsize::new(0)),
            intervention_count: Arc::new(AtomicUsize::new(0)),
            is_stopped: Arc::new(AtomicBool::new(false)),
        };
        Self { contract, state }
    }

    pub fn check_before_action(&self, metrics: &HashMap<String, f64>) -> Result<(), BoundedExecutionError> {
        if self.state.is_stopped.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(BoundedExecutionError::Stopped);
        }

        self.check_time_limit()?;
        self.check_iteration_limit()?;
        self.check_quality_gates(metrics)?;
        self.check_intervention_limit()?;
        self.check_resource_limits(metrics)?;

        Ok(())
    }

    pub fn increment_action_count(&self) {
        self.state.action_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn increment_intervention_count(&self) {
        self.state.intervention_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.state.is_stopped.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn check_time_limit(&self) -> Result<(), BoundedExecutionError> {
        for condition in &self.contract.stop_conditions {
            if let StopCondition::Time { max_duration } = condition {
                let max_duration = ContractParser::parse_duration(max_duration)
                    .map_err(|_| BoundedExecutionError::InvalidDuration)?;
                let elapsed = Utc::now() - self.state.start_time;
                if elapsed > max_duration {
                    return Err(BoundedExecutionError::TimeLimitExceeded(elapsed));
                }
            }
        }
        Ok(())
    }

    fn check_iteration_limit(&self) -> Result<(), BoundedExecutionError> {
        for condition in &self.contract.stop_conditions {
            if let StopCondition::Iteration { max_actions } = condition {
                let count = self.state.action_count.load(std::sync::atomic::Ordering::Relaxed);
                if count >= *max_actions {
                    return Err(BoundedExecutionError::IterationLimitExceeded(count));
                }
            }
        }
        Ok(())
    }

    fn check_quality_gates(&self, metrics: &HashMap<String, f64>) -> Result<(), BoundedExecutionError> {
        for condition in &self.contract.stop_conditions {
            if let StopCondition::Quality { metric, threshold, operator } = condition {
                if let Some(value) = metrics.get(metric) {
                    let violated = match operator {
                        ComparisonOperator::Below => *value < *threshold,
                        ComparisonOperator::Above => *value > *threshold,
                        ComparisonOperator::Equal => (*value - *threshold).abs() < 0.001,
                    };
                    if violated {
                        return Err(BoundedExecutionError::QualityGateViolated(
                            metric.clone(),
                            *value,
                            *threshold
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_intervention_limit(&self) -> Result<(), BoundedExecutionError> {
        for condition in &self.contract.stop_conditions {
            if let StopCondition::Intervention { stop_on_any: true } = condition {
                let count = self.state.intervention_count.load(std::sync::atomic::Ordering::Relaxed);
                if count > 0 {
                    return Err(BoundedExecutionError::InterventionDetected(count));
                }
            }
        }
        Ok(())
    }

    fn check_resource_limits(&self, metrics: &HashMap<String, f64>) -> Result<(), BoundedExecutionError> {
        for condition in &self.contract.stop_conditions {
            if let StopCondition::Resource { budget } = condition {
                if let Some(max_cost) = budget.max_cost_usd {
                    if let Some(current_cost) = metrics.get("cost_usd") {
                        if *current_cost > max_cost {
                            return Err(BoundedExecutionError::ResourceLimitExceeded(
                                "cost_usd".to_string(),
                                *current_cost,
                                max_cost
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_state(&self) -> &BoundedExecutionState {
        &self.state
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoundedExecutionError {
    #[error("Execution stopped")]
    Stopped,
    #[error("Time limit exceeded: {0:?}")]
    TimeLimitExceeded(chrono::Duration),
    #[error("Iteration limit exceeded: {0} actions")]
    IterationLimitExceeded(usize),
    #[error("Quality gate violated: {0} = {1} (threshold: {2})")]
    QualityGateViolated(String, f64, f64),
    #[error("Intervention detected: {0} interventions")]
    InterventionDetected(usize),
    #[error("Resource limit exceeded: {0} = {1} (limit: {2})")]
    ResourceLimitExceeded(String, f64, f64),
    #[error("Invalid duration format")]
    InvalidDuration,
}
```

**Test**: `tests/autonomy/contracts_test.rs`

```rust
#[test]
fn test_enforcer_check_before_action() {
    let contract = create_test_contract();
    let enforcer = BoundedExecutionEnforcer::new(contract);
    let metrics = HashMap::new();

    let result = enforcer.check_before_action(&metrics);
    assert!(result.is_ok());

    enforcer.increment_action_count();
    assert_eq!(enforcer.get_state().action_count.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn test_enforcer_iteration_limit() {
    let mut contract = create_test_contract();
    contract.stop_conditions = vec![StopCondition::Iteration { max_actions: 5 }];
    let enforcer = BoundedExecutionEnforcer::new(contract);
    let metrics = HashMap::new();

    // Should allow 5 actions
    for _ in 0..5 {
        assert!(enforcer.check_before_action(&metrics).is_ok());
        enforcer.increment_action_count();
    }

    // 6th action should fail
    assert!(matches!(
        enforcer.check_before_action(&metrics),
        Err(BoundedExecutionError::IterationLimitExceeded(5))
    ));
}
```

---

### Step 5: Implement goal tracker

**File**: `src/autonomy/contracts/goal.rs`

```rust
use super::contracts::*;
use std::collections::HashMap;

pub struct GoalTracker {
    goal: Goal,
    progress: HashMap<String, f64>,
}

impl GoalTracker {
    pub fn new(goal: Goal) -> Self {
        let progress = HashMap::new();
        Self { goal, progress }
    }

    pub fn update_metric(&mut self, name: &str, value: f64) {
        self.progress.insert(name.to_string(), value);
    }

    pub fn check_progress(&self) -> GoalProgress {
        let mut all_met = true;
        let mut any_met = false;
        let mut criteria_status = Vec::new();

        for criterion in &self.goal.success_criteria {
            let metric_name = Self::criterion_to_metric_name(&criterion.criterion_type);
            let value = self.progress.get(&metric_name).copied().unwrap_or(0.0);

            let met = match criterion.operator {
                ComparisonOperator::Above => value >= criterion.threshold,
                ComparisonOperator::Below => value <= criterion.threshold,
                ComparisonOperator::Equal => (value - criterion.threshold).abs() < 0.001,
            };

            if !met {
                all_met = false;
            } else {
                any_met = true;
            }

            criteria_status.push(CriterionStatus {
                criterion_type: criterion.criterion_type.clone(),
                threshold: criterion.threshold,
                current_value: value,
                met,
            });
        }

        if all_met {
            GoalProgress::Complete
        } else if any_met {
            GoalProgress::Partial(criteria_status)
        } else {
            GoalProgress::NotStarted(criteria_status)
        }
    }

    pub fn get_progress(&self) -> &HashMap<String, f64> {
        &self.progress
    }

    fn criterion_to_metric_name(criterion_type: &CriterionType) -> String {
        match criterion_type {
            CriterionType::DeploymentSuccess => "deployment_success_rate".to_string(),
            CriterionType::TaskSuccess => "task_success_rate".to_string(),
            CriterionType::MetricValue => "metric_value".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoalProgress {
    Complete,
    Partial(Vec<CriterionStatus>),
    NotStarted(Vec<CriterionStatus>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriterionStatus {
    pub criterion_type: CriterionType,
    pub threshold: f64,
    pub current_value: f64,
    pub met: bool,
}
```

**Test**: `tests/autonomy/contracts_test.rs`

```rust
#[test]
fn test_goal_tracker_progress() {
    let goal = Goal {
        goal_type: GoalType::WorkflowCompletion,
        target_workflow: "test_workflow".to_string(),
        success_criteria: vec![
            SuccessCriterion {
                criterion_type: CriterionType::TaskSuccess,
                threshold: 0.95,
                operator: ComparisonOperator::Above,
            },
        ],
    };

    let mut tracker = GoalTracker::new(goal);

    // Not started
    assert!(matches!(
        tracker.check_progress(),
        GoalProgress::NotStarted(_)
    ));

    // Partial progress
    tracker.update_metric("task_success_rate", 0.8);
    assert!(matches!(
        tracker.check_progress(),
        GoalProgress::Partial(_)
    ));

    // Complete
    tracker.update_metric("task_success_rate", 0.96);
    assert_eq!(tracker.check_progress(), GoalProgress::Complete);
}
```

---

### Step 6: Implement autonomy level module

**File**: `src/autonomy/contracts/level.rs`

```rust
use super::contracts::*;

impl AutonomyLevel {
    pub fn confirmation_required(&self, action_count: usize) -> bool {
        match self {
            AutonomyLevel::Low => true,
            AutonomyLevel::Medium { batch_size } => action_count % batch_size == 0,
            AutonomyLevel::High { .. } => false,
        }
    }

    pub fn checkpoint_required(&self, action_count: usize) -> bool {
        match self {
            AutonomyLevel::Low => false,
            AutonomyLevel::Medium { .. } => action_count % 100 == 0,
            AutonomyLevel::High { checkpoint_interval } => {
                let interval_secs = checkpoint_interval.num_seconds() as usize;
                action_count % interval_secs == 0
            }
        }
    }

    pub fn requires_human_intervention(&self) -> bool {
        matches!(self, AutonomyLevel::Low)
    }

    pub fn description(&self) -> &'static str {
        match self {
            AutonomyLevel::Low => "Every action requires confirmation",
            AutonomyLevel::Medium { batch_size } => "Batch confirmation",
            AutonomyLevel::High { .. } => "Bounded autonomous with checkpoints",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_level_confirmation() {
        let level = AutonomyLevel::Low;
        assert!(level.confirmation_required(1));
        assert!(level.confirmation_required(100));
        assert!(level.requires_human_intervention());
    }

    #[test]
    fn test_medium_level_confirmation() {
        let level = AutonomyLevel::Medium { batch_size: 10 };
        assert!(level.confirmation_required(0));
        assert!(level.confirmation_required(10));
        assert!(!level.confirmation_required(5));
        assert!(!level.requires_human_intervention());
    }

    #[test]
    fn test_high_level_confirmation() {
        let level = AutonomyLevel::High { checkpoint_interval: chrono::Duration::seconds(60) };
        assert!(!level.confirmation_required(1));
        assert!(!level.confirmation_required(100));
        assert!(!level.requires_human_intervention());
    }
}
```

---

### Step 7: Create module exports

**File**: `src/autonomy/mod.rs`

```rust
pub mod contracts;

pub use contracts::*;
pub use contracts::parser::ContractParser;
pub use contracts::validator::ContractValidator;
pub use contracts::enforcer::{BoundedExecutionEnforcer, BoundedExecutionError, BoundedExecutionState};
pub use contracts::goal::{GoalTracker, GoalProgress, CriterionStatus};
```

**File**: `src/lib.rs` (modify)

```rust
pub mod autonomy;
```

---

### Step 8: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test autonomy::contracts --verbose
```

**Expected Output**: All tests pass

---

### Step 9: Integration test for full contract workflow

**File**: `tests/autonomy/integration_test.rs`

```rust
use glyphnova_engine::autonomy::*;
use std::collections::HashMap;

#[test]
fn test_full_contract_workflow() {
    let yaml = r#"
version: "1.0"
autonomy_level:
  medium:
    batch_size: 5
goal:
  type: WorkflowCompletion
  target_workflow: "deploy_production"
  success_criteria:
    - type: DeploymentSuccess
      threshold: 0.95
      operator: above
stop_conditions:
  - type: time
    max_duration: "1h"
  - type: iteration
    max_actions: 100
scope:
  allowed_workflows: ["deploy_production"]
  allowed_actions: ["deploy", "verify"]
  forbidden_workflows: []
  forbidden_actions: ["delete"]
"#;

    // Parse contract
    let contract = ContractParser::parse(yaml).expect("Failed to parse contract");

    // Validate contract
    ContractValidator::validate(&contract).expect("Failed to validate contract");

    // Create enforcer
    let enforcer = BoundedExecutionEnforcer::new(contract.clone());

    // Create goal tracker
    let goal_tracker = GoalTracker::new(contract.goal);

    // Simulate execution
    let mut metrics = HashMap::new();
    metrics.insert("deployment_success_rate".to_string(), 0.9);

    for i in 0..10 {
        enforcer.check_before_action(&metrics).expect("Check failed");
        enforcer.increment_action_count();

        // Update goal progress
        goal_tracker.update_metric("deployment_success_rate", 0.9 + (i as f64 * 0.01));

        let progress = goal_tracker.check_progress();
        assert!(matches!(progress, GoalProgress::Partial(_) | GoalProgress::Complete));
    }

    // Check final state
    assert_eq!(enforcer.get_state().action_count.load(std::sync::atomic::Ordering::Relaxed), 10);
}
```

---

### Step 10: Commit

```bash
git add src/autonomy/ tests/autonomy/
git commit -m "feat: implement autonomous loop contracts

- Define AutonomousLoopContract with goals, stop conditions, autonomy levels
- Implement ContractParser for YAML parsing
- Implement ContractValidator for validation logic
- Implement BoundedExecutionEnforcer for bounded execution
- Implement GoalTracker for progress tracking
- Implement AutonomyLevel utilities
- Add comprehensive unit and integration tests

Relates to ADR-0008: Autonomous Execution Safety"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration test passes
- ✅ Contract validation catches all edge cases
- ✅ Bounded execution enforcer enforces all stop conditions
- ✅ Goal tracker correctly tracks progress
- ✅ Autonomy level behavior matches specification

---

## Next Steps

After completing Task 00:
1. Proceed to Task 01: Metrics Collection
2. Metrics collection will integrate with the contract system for quality gates

---

**End of Task 00**
