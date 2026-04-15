# Task 05: Stop Condition Evaluation

**Estimated Time**: 5 days
**Priority**: HIGH - Critical for bounded execution
**Dependencies**: Task 00 (contracts), Task 01 (metrics)

## Overview

Evaluate stop conditions during autonomous loop execution. This enforces bounded execution as required by ADR-0008.

## Files

### Create
- `src/stop_conditions/mod.rs` - Module exports
- `src/stop_conditions/types.rs` - Stop condition types
- `src/stop_conditions/parser.rs` - Parser for stop conditions
- `src/stop_conditions/validator.rs` - Validator for stop conditions
- `src/stop_conditions/evaluation.rs` - Runtime evaluation logic
- `tests/stop_conditions/stop_conditions_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod stop_conditions;`

---

## Implementation Steps

### Step 1: Define stop condition types

**File**: `src/stop_conditions/types.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::collections::HashMap;

/// Stop condition
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ComparisonOperator {
    Above,
    Below,
    Equal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    pub max_cost_usd: Option<f64>,
    pub max_cpu_hours: Option<f64>,
    pub max_memory_gb_hours: Option<f64>,
}

/// Stop condition evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopConditionResult {
    Continue,
    Stop {
        condition: StopCondition,
        reason: String,
        triggered_at: Utc,
    },
}
```

---

### Step 2: Implement parser

**File**: `src/stop_conditions/parser.rs`

```rust
use super::types::*;

pub struct StopConditionParser;

impl StopConditionParser {
    pub fn parse_duration(duration_str: &str) -> Result<Duration, ParseError> {
        let duration_str = duration_str.trim().to_lowercase();

        if duration_str.ends_with('h') {
            let hours: i64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::hours(hours))
        } else if duration_str.ends_with('m') {
            let minutes: i64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::minutes(minutes))
        } else if duration_str.ends_with('s') {
            let seconds: i64 = duration_str[..duration_str.len()-1]
                .parse()
                .map_err(|_| ParseError::InvalidDurationFormat(duration_str.clone()))?;
            Ok(Duration::seconds(seconds))
        } else {
            Err(ParseError::InvalidDurationFormat(duration_str.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid duration format: {0}")]
    InvalidDurationFormat(String),
}
```

---

### Step 3: Implement validator

**File**: `src/stop_conditions/validator.rs`

```rust
use super::types::*;
use super::parser::*;

pub struct StopConditionValidator;

impl StopConditionValidator {
    pub fn validate(condition: &StopCondition) -> Result<(), ValidationError> {
        match condition {
            StopCondition::Time { max_duration } => {
                let duration = StopConditionParser::parse_duration(max_duration)
                    .map_err(|_| ValidationError::InvalidDuration(max_duration.clone()))?;
                if duration.num_seconds() < 60 {
                    return Err(ValidationError::TimeLimitTooShort);
                }
            }
            StopCondition::Iteration { max_actions } => {
                if *max_actions == 0 {
                    return Err(ValidationError::IterationLimitZero);
                }
            }
            StopCondition::Quality { threshold, .. } => {
                if *threshold < 0.0 || *threshold > 1.0 {
                    return Err(ValidationError::ThresholdOutOfRange);
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    #[error("Time limit must be at least 60 seconds")]
    TimeLimitTooShort,
    #[error("Iteration limit must be > 0")]
    IterationLimitZero,
    #[error("Threshold must be in [0, 1]")]
    ThresholdOutOfRange,
}
```

---

### Step 4: Implement evaluation logic

**File**: `src/stop_conditions/evaluation.rs`

```rust
use super::types::*;
use super::parser::*;
use std::collections::HashMap;

pub struct StopConditionEvaluator {
    conditions: Vec<StopCondition>,
    start_time: Utc,
    action_count: usize,
    intervention_count: usize,
}

impl StopConditionEvaluator {
    pub fn new(conditions: Vec<StopCondition>) -> Self {
        Self {
            conditions,
            start_time: Utc::now(),
            action_count: 0,
            intervention_count: 0,
        }
    }

    pub fn evaluate(
        &self,
        metrics: &HashMap<String, f64>,
    ) -> StopConditionResult {
        for condition in &self.conditions {
            let result = self.evaluate_condition(condition, metrics);
            if let StopConditionResult::Stop { .. } = result {
                return result;
            }
        }
        StopConditionResult::Continue
    }

    pub fn increment_action_count(&mut self) {
        self.action_count += 1;
    }

    pub fn increment_intervention_count(&mut self) {
        self.intervention_count += 1;
    }

    fn evaluate_condition(
        &self,
        condition: &StopCondition,
        metrics: &HashMap<String, f64>,
    ) -> StopConditionResult {
        match condition {
            StopCondition::Time { max_duration } => {
                self.evaluate_time_condition(max_duration)
            }
            StopCondition::Iteration { max_actions } => {
                self.evaluate_iteration_condition(max_actions)
            }
            StopCondition::Quality { metric, threshold, operator } => {
                self.evaluate_quality_condition(metric, threshold, operator, metrics)
            }
            StopCondition::Intervention { stop_on_any } => {
                self.evaluate_intervention_condition(stop_on_any)
            }
            StopCondition::Resource { budget } => {
                self.evaluate_resource_condition(budget, metrics)
            }
        }
    }

    fn evaluate_time_condition(&self, max_duration: &str) -> StopConditionResult {
        let max_duration = StopConditionParser::parse_duration(max_duration).unwrap();
        let elapsed = Utc::now() - self.start_time;

        if elapsed > max_duration {
            StopConditionResult::Stop {
                condition: StopCondition::Time { max_duration: max_duration.to_string() },
                reason: format!("Time limit exceeded: {:?}", elapsed),
                triggered_at: Utc::now(),
            }
        } else {
            StopConditionResult::Continue
        }
    }

    fn evaluate_iteration_condition(&self, max_actions: &usize) -> StopConditionResult {
        if self.action_count >= *max_actions {
            StopConditionResult::Stop {
                condition: StopCondition::Iteration { max_actions: *max_actions },
                reason: format!("Iteration limit exceeded: {}", self.action_count),
                triggered_at: Utc::now(),
            }
        } else {
            StopConditionResult::Continue
        }
    }

    fn evaluate_quality_condition(
        &self,
        metric: &str,
        threshold: &f64,
        operator: &ComparisonOperator,
        metrics: &HashMap<String, f64>,
    ) -> StopConditionResult {
        if let Some(&value) = metrics.get(metric) {
            let violated = match operator {
                ComparisonOperator::Below => value < *threshold,
                ComparisonOperator::Above => value > *threshold,
                ComparisonOperator::Equal => (value - threshold).abs() < 0.001,
            };

            if violated {
                StopConditionResult::Stop {
                    condition: StopCondition::Quality {
                        metric: metric.to_string(),
                        threshold: *threshold,
                        operator: *operator,
                    },
                    reason: format!("Quality gate violated: {} = {} (threshold: {})", metric, value, threshold),
                    triggered_at: Utc::now(),
                }
            } else {
                StopConditionResult::Continue
            }
        } else {
            StopConditionResult::Continue
        }
    }

    fn evaluate_intervention_condition(&self, stop_on_any: &bool) -> StopConditionResult {
        if *stop_on_any && self.intervention_count > 0 {
            StopConditionResult::Stop {
                condition: StopCondition::Intervention { stop_on_any: *stop_on_any },
                reason: format!("Intervention detected: {} interventions", self.intervention_count),
                triggered_at: Utc::now(),
            }
        } else {
            StopConditionResult::Continue
        }
    }

    fn evaluate_resource_condition(
        &self,
        budget: &ResourceBudget,
        metrics: &HashMap<String, f64>,
    ) -> StopConditionResult {
        if let Some(max_cost) = budget.max_cost_usd {
            if let Some(&cost) = metrics.get("cost_usd") {
                if cost > max_cost {
                    return StopConditionResult::Stop {
                        condition: StopCondition::Resource { budget: budget.clone() },
                        reason: format!("Resource limit exceeded: cost = ${} (max: ${})", cost, max_cost),
                        triggered_at: Utc::now(),
                    };
                }
            }
        }
        StopConditionResult::Continue
    }
}
```

---

### Step 5: Create module exports

**File**: `src/stop_conditions/mod.rs`

```rust
pub mod types;
pub mod parser;
pub mod validator;
pub mod evaluation;

pub use types::*;
pub use parser::*;
pub use validator::*;
pub use evaluation::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod stop_conditions;
```

---

### Step 6: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test stop_conditions --verbose
```

**Expected Output**: All tests pass

---

### Step 7: Integration test

**File**: `tests/stop_conditions/integration_test.rs`

```rust
use glyphnova_engine::stop_conditions::*;

#[test]
fn test_stop_condition_evaluation() {
    let conditions = vec![
        StopCondition::Time { max_duration: "1s".to_string() },
        StopCondition::Iteration { max_actions: 10 },
        StopCondition::Quality {
            metric: "success_rate".to_string(),
            threshold: 0.8,
            operator: ComparisonOperator::Below,
        },
    ];

    let mut evaluator = StopConditionEvaluator::new(conditions);

    let mut metrics = std::collections::HashMap::new();
    metrics.insert("success_rate".to_string(), 0.9);

    // Should continue
    assert!(matches!(evaluator.evaluate(&metrics), StopConditionResult::Continue));

    // Increment actions
    evaluator.increment_action_count();

    // Should still continue
    assert!(matches!(evaluator.evaluate(&metrics), StopConditionResult::Continue));

    // Trigger quality gate
    metrics.insert("success_rate".to_string(), 0.7);

    // Should stop
    let result = evaluator.evaluate(&metrics);
    assert!(matches!(result, StopConditionResult::Stop { .. }));
}
```

---

### Step 8: Commit

```bash
git add src/stop_conditions/ tests/stop_conditions/
git commit -m "feat: implement stop condition evaluation

- Define stop condition types (Time, Iteration, Quality, Intervention, Resource)
- Implement parser for duration strings
- Implement validator for stop conditions
- Implement runtime evaluation logic
- Add comprehensive unit and integration tests

Relates to ADR-0008: Bounded autonomous loops"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ All stop conditions are correctly evaluated
- ✅ Time limits enforce maximum execution time
- ✅ Iteration limits enforce maximum action count
- ✅ Quality gates enforce metric thresholds

---

## Next Steps

After completing Task 05:
1. Proceed to Task 06: Checkpoint Generation

---

**End of Task 05**
