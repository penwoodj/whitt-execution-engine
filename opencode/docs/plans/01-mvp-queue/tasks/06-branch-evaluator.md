# Task 06: Branch Evaluator

**Goal:** Implement branch evaluator for conditional branching and event-based routing with strict yes/no decisions.

**Files:**
- Create: `src/control_flow/branch.rs`
- Create: `tests/unit/branch_evaluator_test.rs`

---

## Rust Definitions

### `src/control_flow/branch.rs`

```rust
use crate::executor::step::ExecutionContext;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Branch condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCondition {
    pub variable: String,
    pub operator: String,
    pub value: serde_json::Value,
}

/// Branch (conditional path)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub conditions: Vec<BranchCondition>,
    pub target_step_id: String,
}

/// Event for event-based routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

/// Branch evaluation error
#[derive(Debug, Error)]
pub enum BranchError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

    #[error("Type mismatch in condition")]
    TypeMismatch,

    #[error("No matching branch found")]
    NoMatchingBranch,
}

/// Branch evaluator
pub struct BranchEvaluator;

impl BranchEvaluator {
    /// Evaluate branch conditions and return matching branch
    pub fn evaluate_branches(
        branches: &[Branch],
        context: &ExecutionContext,
    ) -> Result<&Branch, BranchError> {
        for branch in branches {
            if Self::evaluate_branch(branch, context)? {
                return Ok(branch);
            }
        }

        Err(BranchError::NoMatchingBranch)
    }

    /// Evaluate a single branch (all conditions must be true)
    fn evaluate_branch(branch: &Branch, context: &ExecutionContext) -> Result<bool, BranchError> {
        for condition in &branch.conditions {
            if !Self::evaluate_condition(condition, context)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Evaluate a single condition
    fn evaluate_condition(
        condition: &BranchCondition,
        context: &ExecutionContext,
    ) -> Result<bool, BranchError> {
        let variable_value = context
            .get_variable(&condition.variable)
            .ok_or_else(|| BranchError::VariableNotFound(condition.variable.clone()))?;

        match condition.operator.as_str() {
            "eq" => Ok(Self::compare_eq(variable_value, &condition.value)),
            "ne" => Ok(!Self::compare_eq(variable_value, &condition.value)),
            "gt" => Ok(Self::compare_gt(variable_value, &condition.value)),
            "lt" => Ok(Self::compare_lt(variable_value, &condition.value)),
            "gte" => Ok(Self::compare_gte(variable_value, &condition.value)),
            "lte" => Ok(Self::compare_lte(variable_value, &condition.value)),
            "contains" => Ok(Self::compare_contains(variable_value, &condition.value)),
            "in" => Ok(Self::compare_in(variable_value, &condition.value)),
            "is_null" => Ok(variable_value.is_null()),
            "is_not_null" => Ok(!variable_value.is_null()),
            _ => Err(BranchError::InvalidOperator(condition.operator.clone())),
        }
    }

    /// Compare equality
    fn compare_eq(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        a == b
    }

    /// Compare greater than
    fn compare_gt(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match (a.as_i64(), b.as_i64()) {
            (Some(ai), Some(bi)) => ai > bi,
            _ => false,
        }
    }

    /// Compare less than
    fn compare_lt(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match (a.as_i64(), b.as_i64()) {
            (Some(ai), Some(bi)) => ai < bi,
            _ => false,
        }
    }

    /// Compare greater than or equal
    fn compare_gte(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match (a.as_i64(), b.as_i64()) {
            (Some(ai), Some(bi)) => ai >= bi,
            _ => false,
        }
    }

    /// Compare less than or equal
    fn compare_lte(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match (a.as_i64(), b.as_i64()) {
            (Some(ai), Some(bi)) => ai <= bi,
            _ => false,
        }
    }

    /// Compare contains (for strings and arrays)
    fn compare_contains(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match (a, b) {
            (serde_json::Value::String(a_str), serde_json::Value::String(b_str)) => {
                a_str.contains(b_str)
            }
            (serde_json::Value::Array(a_arr), b_val) => a_arr.contains(b_val),
            _ => false,
        }
    }

    /// Compare in (for membership)
    fn compare_in(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        match b {
            serde_json::Value::Array(b_arr) => b_arr.contains(a),
            _ => false,
        }
    }

    /// Evaluate event-based routing
    pub fn evaluate_event_routing(
        event: &RoutingEvent,
        branches: &[Branch],
    ) -> Result<&Branch, BranchError> {
        // Find branch with matching event type
        for branch in branches {
            if branch.name == event.event_type {
                return Ok(branch);
            }
        }

        // Check for default branch
        for branch in branches {
            if branch.name == "default" {
                return Ok(branch);
            }
        }

        Err(BranchError::NoMatchingBranch)
    }

    /// Strict yes/no decision for user confirmation
    pub fn evaluate_yes_no_decision(value: &str) -> Result<bool, BranchError> {
        match value.to_lowercase().as_str() {
            "yes" | "y" => Ok(true),
            "no" | "n" => Ok(false),
            _ => Err(BranchError::TypeMismatch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_eq() {
        let result = BranchEvaluator::compare_eq(&serde_json::json!(5), &serde_json::json!(5));
        assert!(result);
    }

    #[test]
    fn test_evaluate_gt() {
        let result = BranchEvaluator::compare_gt(&serde_json::json!(10), &serde_json::json!(5));
        assert!(result);
    }

    #[test]
    fn test_evaluate_contains() {
        let result = BranchEvaluator::compare_contains(
            &serde_json::json!("hello world"),
            &serde_json::json!("world"),
        );
        assert!(result);
    }

    #[test]
    fn test_yes_no_decision() {
        assert!(BranchEvaluator::evaluate_yes_no_decision("yes").unwrap());
        assert!(BranchEvaluator::evaluate_yes_no_decision("Y").unwrap());
        assert!(!BranchEvaluator::evaluate_yes_no_decision("no").unwrap());
        assert!(BranchEvaluator::evaluate_yes_no_decision("maybe").is_err());
    }
}
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement branch.rs**

- [ ] **Step 3: Add to control_flow/mod.rs**

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**
