# Task 07: Autonomy Scope & Risk

**Estimated Time**: 5 days
**Priority**: HIGH - Critical for safety
**Dependencies**: Task 00 (contracts), Task 01 (metrics)

## Overview

Define scope boundaries and implement risk assessment models. This ensures autonomous actions stay within safe boundaries.

## Files

### Create
- `src/risk/mod.rs` - Module exports
- `src/risk/scope.rs` - Scope boundary definitions
- `src/risk/assessment.rs` - Risk assessment model
- `src/risk/enforcement.rs` - Boundary enforcement
- `src/risk/alerting.rs` - Risk alerting
- `tests/risk/risk_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod risk;`

---

## Implementation Steps

### Step 1: Define scope boundaries

**File**: `src/risk/scope.rs`

```rust
use serde::{Deserialize, Serialize};

/// Scope boundary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeBoundary {
    pub allowed_workflows: Vec<String>,
    pub allowed_actions: Vec<String>,
    pub forbidden_workflows: Vec<String>,
    pub forbidden_actions: Vec<String>,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_cores: f64,
    pub max_memory_gb: f64,
    pub max_cost_usd: f64,
}

impl ScopeBoundary {
    pub fn is_workflow_allowed(&self, workflow_id: &str) -> bool {
        // Check forbidden first
        if self.forbidden_workflows.iter().any(|w| w == workflow_id) {
            return false;
        }

        // If no allowed list, allow all (except forbidden)
        if self.allowed_workflows.is_empty() {
            return true;
        }

        // Check allowed list
        self.allowed_workflows.iter().any(|w| w == workflow_id)
    }

    pub fn is_action_allowed(&self, action_type: &str) -> bool {
        // Check forbidden first
        if self.forbidden_actions.iter().any(|a| a == action_type) {
            return false;
        }

        // If no allowed list, allow all (except forbidden)
        if self.allowed_actions.is_empty() {
            return true;
        }

        // Check allowed list
        self.allowed_actions.iter().any(|a| a == action_type)
    }
}
```

---

### Step 2: Implement risk assessment model

**File**: `src/risk/assessment.rs`

```rust
use super::scope::*;
use serde::{Deserialize, Serialize};

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub action_id: String,
    pub probability: RiskProbability,
    pub impact: RiskImpact,
    pub severity: RiskSeverity,
    pub factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskProbability {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskImpact {
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub weight: f64,
    pub value: f64,
}

/// Risk assessment model
pub struct RiskAssessmentModel {
    scope: ScopeBoundary,
}

impl RiskAssessmentModel {
    pub fn new(scope: ScopeBoundary) -> Self {
        Self { scope }
    }

    pub fn assess_action(
        &self,
        action_id: String,
        action_type: &str,
        workflow_id: &str,
        metrics: &[f64],
    ) -> RiskAssessment {
        // Check scope boundaries
        let in_scope = self.scope.is_action_allowed(action_type) &&
                       self.scope.is_workflow_allowed(workflow_id);

        // Calculate probability based on historical metrics
        let probability = self.calculate_probability(metrics);

        // Calculate impact based on action type and scope
        let impact = self.calculate_impact(action_type, in_scope);

        // Calculate severity from probability and impact
        let severity = self.calculate_severity(probability, impact);

        // Collect risk factors
        let factors = self.collect_risk_factors(probability, impact, metrics);

        RiskAssessment {
            action_id,
            probability,
            impact,
            severity,
            factors,
        }
    }

    fn calculate_probability(&self, metrics: &[f64]) -> RiskProbability {
        if metrics.is_empty() {
            return RiskProbability::Low;
        }

        let avg_success_rate: f64 = metrics.iter().sum::<f64>() / metrics.len() as f64;

        if avg_success_rate > 0.95 {
            RiskProbability::VeryLow
        } else if avg_success_rate > 0.90 {
            RiskProbability::Low
        } else if avg_success_rate > 0.80 {
            RiskProbability::Medium
        } else if avg_success_rate > 0.70 {
            RiskProbability::High
        } else {
            RiskProbability::VeryHigh
        }
    }

    fn calculate_impact(&self, action_type: &str, in_scope: bool) -> RiskImpact {
        if !in_scope {
            return RiskImpact::Critical;
        }

        match action_type {
            "deploy" => RiskImpact::High,
            "delete" => RiskImpact::Critical,
            "modify" => RiskImpact::Medium,
            _ => RiskImpact::Low,
        }
    }

    fn calculate_severity(
        &self,
        probability: RiskProbability,
        impact: RiskImpact,
    ) -> RiskSeverity {
        let prob_score = match probability {
            RiskProbability::VeryLow => 1,
            RiskProbability::Low => 2,
            RiskProbability::Medium => 3,
            RiskProbability::High => 4,
            RiskProbability::VeryHigh => 5,
        };

        let impact_score = match impact {
            RiskImpact::Negligible => 1,
            RiskImpact::Low => 2,
            RiskImpact::Medium => 3,
            RiskImpact::High => 4,
            RiskImpact::Critical => 5,
        };

        let risk_score = prob_score * impact_score;

        match risk_score {
            1..=4 => RiskSeverity::Low,
            5..=9 => RiskSeverity::Medium,
            10..=16 => RiskSeverity::High,
            _ => RiskSeverity::Critical,
        }
    }

    fn collect_risk_factors(
        &self,
        probability: RiskProbability,
        impact: RiskImpact,
        metrics: &[f64],
    ) -> Vec<RiskFactor> {
        let mut factors = Vec::new();

        factors.push(RiskFactor {
            name: "historical_success_rate".to_string(),
            weight: 0.4,
            value: match probability {
                RiskProbability::VeryLow => 0.95,
                RiskProbability::Low => 0.85,
                RiskProbability::Medium => 0.75,
                RiskProbability::High => 0.65,
                RiskProbability::VeryHigh => 0.55,
            },
        });

        factors.push(RiskFactor {
            name: "action_impact".to_string(),
            weight: 0.6,
            value: match impact {
                RiskImpact::Negligible => 0.1,
                RiskImpact::Low => 0.3,
                RiskImpact::Medium => 0.5,
                RiskImpact::High => 0.7,
                RiskImpact::Critical => 1.0,
            },
        });

        factors
    }
}
```

---

### Step 3: Implement boundary enforcement

**File**: `src/risk/enforcement.rs`

```rust
use super::scope::*;
use super::assessment::*;

#[derive(Debug, thiserror::Error)]
pub enum EnforcementError {
    #[error("Action '{0}' is forbidden")]
    ActionForbidden(String),
    #[error("Workflow '{0}' is forbidden")]
    WorkflowForbidden(String),
    #[error("Risk severity {0:?} exceeds threshold")]
    RiskExceedsThreshold(RiskSeverity),
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
}

pub struct ScopeEnforcer {
    scope: ScopeBoundary,
    max_allowed_severity: RiskSeverity,
}

impl ScopeEnforcer {
    pub fn new(scope: ScopeBoundary, max_allowed_severity: RiskSeverity) -> Self {
        Self { scope, max_allowed_severity }
    }

    pub fn check_action(
        &self,
        action_id: &str,
        action_type: &str,
        workflow_id: &str,
        assessment: &RiskAssessment,
    ) -> Result<(), EnforcementError> {
        // Check scope boundaries
        if !self.scope.is_action_allowed(action_type) {
            return Err(EnforcementError::ActionForbidden(action_type.to_string()));
        }

        if !self.scope.is_workflow_allowed(workflow_id) {
            return Err(EnforcementError::WorkflowForbidden(workflow_id.to_string()));
        }

        // Check risk severity
        if assessment.severity > self.max_allowed_severity {
            return Err(EnforcementError::RiskExceedsThreshold(assessment.severity));
        }

        Ok(())
    }
}
```

---

### Step 4: Implement alerting

**File**: `src/risk/alerting.rs`

```rust
use super::assessment::*;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum RiskAlert {
    HighRiskAction {
        action_id: String,
        severity: RiskSeverity,
        probability: RiskProbability,
        impact: RiskImpact,
    },
    OutOfScopeAction {
        action_id: String,
        action_type: String,
        workflow_id: String,
    },
}

pub struct RiskAlertManager {
    sender: broadcast::Sender<RiskAlert>,
}

impl RiskAlertManager {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RiskAlert> {
        self.sender.subscribe()
    }

    pub fn send_alert(&self, alert: RiskAlert) {
        let _ = self.sender.send(alert);
    }
}

impl Default for RiskAlertManager {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 5: Create module exports

**File**: `src/risk/mod.rs`

```rust
pub mod scope;
pub mod assessment;
pub mod enforcement;
pub mod alerting;

pub use scope::*;
pub use assessment::*;
pub use enforcement::*;
pub use alerting::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod risk;
```

---

### Step 6: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test risk --verbose
```

**Expected Output**: All tests pass

---

### Step 7: Integration test

**File**: `tests/risk/integration_test.rs`

```rust
use glyphnova_engine::risk::*;

#[test]
fn test_risk_assessment_workflow() {
    let scope = ScopeBoundary {
        allowed_workflows: vec!["deploy_production".to_string()],
        allowed_actions: vec!["deploy".to_string(), "verify".to_string()],
        forbidden_workflows: vec!["delete_data".to_string()],
        forbidden_actions: vec!["delete".to_string()],
        resource_limits: ResourceLimits {
            max_cpu_cores: 10.0,
            max_memory_gb: 32.0,
            max_cost_usd: 100.0,
        },
    };

    let model = RiskAssessmentModel::new(scope);

    let assessment = model.assess_action(
        "action_123".to_string(),
        "deploy",
        "deploy_production",
        &[0.95, 0.96, 0.94],
    );

    assert!(matches!(assessment.severity, RiskSeverity::Low | RiskSeverity::Medium));
}
```

---

### Step 8: Commit

```bash
git add src/risk/ tests/risk/
git commit -m "feat: implement autonomy scope and risk

- Define scope boundaries with allowed/forbidden workflows and actions
- Implement risk assessment model with probability, impact, severity
- Implement boundary enforcement logic
- Implement risk alerting system
- Add comprehensive unit and integration tests

Relates to ADR-0008: Bounded autonomous execution"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ Scope boundaries are correctly enforced
- ✅ Risk assessment accurately calculates severity
- ✅ High-risk actions are blocked

---

## Next Steps

After completing Task 07:
1. Proceed to Task 08: Confidence Thresholds

---

**End of Task 07**
