# Task 08: Confidence Thresholds

**Estimated Time**: 5 days
**Priority**: HIGH - Critical for intelligent decisions
**Dependencies**: Task 01 (metrics), Task 07 (risk assessment)

## Overview

Compute confidence from metrics and enforce threshold-based decisions. This enables intelligent autonomous decision-making.

## Files

### Create
- `src/confidence/mod.rs` - Module exports
- `src/confidence/threshold.rs` - Threshold structure
- `src/confidence/computation.rs` - Confidence computation from metrics
- `src/confidence/enforcement.rs` - Threshold enforcement
- `src/confidence/tuning.rs` - Threshold tuning
- `tests/confidence/confidence_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod confidence;`

---

## Implementation Steps

### Step 1: Define threshold structure

**File**: `src/confidence/threshold.rs`

```rust
use serde::{Deserialize, Serialize};

/// Confidence thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceThresholds {
    pub action_threshold: f64,
    pub batch_threshold: f64,
    pub loop_threshold: f64,
    pub auto_approve_threshold: f64,
}

impl ConfidenceThresholds {
    pub fn new(action_threshold: f64, batch_threshold: f64, loop_threshold: f64) -> Self {
        Self {
            action_threshold,
            batch_threshold,
            loop_threshold,
            auto_approve_threshold: 0.9,
        }
    }

    pub fn conservative() -> Self {
        Self::new(0.8, 0.85, 0.9)
    }

    pub fn moderate() -> Self {
        Self::new(0.6, 0.7, 0.8)
    }

    pub fn aggressive() -> Self {
        Self::new(0.4, 0.5, 0.6)
    }

    pub fn validate(&self) -> Result<(), ThresholdError> {
        if !Self::is_valid_threshold(self.action_threshold) {
            return Err(ThresholdError::InvalidThreshold(self.action_threshold));
        }
        if !Self::is_valid_threshold(self.batch_threshold) {
            return Err(ThresholdError::InvalidThreshold(self.batch_threshold));
        }
        if !Self::is_valid_threshold(self.loop_threshold) {
            return Err(ThresholdError::InvalidThreshold(self.loop_threshold));
        }
        if !Self::is_valid_threshold(self.auto_approve_threshold) {
            return Err(ThresholdError::InvalidThreshold(self.auto_approve_threshold));
        }

        Ok(())
    }

    fn is_valid_threshold(value: f64) -> bool {
        (0.0..=1.0).contains(&value)
    }
}

impl Default for ConfidenceThresholds {
    fn default() -> Self {
        Self::moderate()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ThresholdError {
    #[error("Invalid threshold: {0}")]
    InvalidThreshold(f64),
}
```

---

### Step 2: Implement confidence computation

**File**: `src/confidence/computation.rs`

```rust
use super::threshold::*;
use std::collections::HashMap;

/// Confidence computation result
#[derive(Debug, Clone)]
pub struct ConfidenceResult {
    pub confidence: f64,
    pub factors: ConfidenceFactors,
}

#[derive(Debug, Clone)]
pub struct ConfidenceFactors {
    pub historical_success_rate: f64,
    pub recent_success_rate: f64,
    pub intervention_rate: f64,
    pub risk_score: f64,
}

pub struct ConfidenceComputer {
    historical_window: usize,
    recent_window: usize,
}

impl ConfidenceComputer {
    pub fn new(historical_window: usize, recent_window: usize) -> Self {
        Self { historical_window, recent_window }
    }

    pub fn compute(
        &self,
        metrics: &HashMap<String, f64>,
        history: &[f64],
        intervention_count: usize,
        total_actions: usize,
    ) -> ConfidenceResult {
        let factors = self.extract_factors(metrics, history, intervention_count, total_actions);
        let confidence = self.calculate_confidence(&factors);

        ConfidenceResult { confidence, factors }
    }

    fn extract_factors(
        &self,
        metrics: &HashMap<String, f64>,
        history: &[f64],
        intervention_count: usize,
        total_actions: usize,
    ) -> ConfidenceFactors {
        let historical_success_rate = if history.len() >= self.historical_window {
            let slice = &history[history.len() - self.historical_window..];
            slice.iter().sum::<f64>() / slice.len() as f64
        } else if !history.is_empty() {
            history.iter().sum::<f64>() / history.len() as f64
        } else {
            1.0
        };

        let recent_success_rate = if history.len() >= self.recent_window {
            let slice = &history[history.len() - self.recent_window..];
            slice.iter().sum::<f64>() / slice.len() as f64
        } else {
            historical_success_rate
        };

        let intervention_rate = if total_actions > 0 {
            intervention_count as f64 / total_actions as f64
        } else {
            0.0
        };

        let risk_score = metrics.get("risk_score").copied().unwrap_or(0.0);

        ConfidenceFactors {
            historical_success_rate,
            recent_success_rate,
            intervention_rate,
            risk_score,
        }
    }

    fn calculate_confidence(&self, factors: &ConfidenceFactors) -> f64 {
        // Weighted combination of factors
        let confidence = (0.4 * factors.historical_success_rate) +
                        (0.3 * factors.recent_success_rate) +
                        (0.2 * (1.0 - factors.intervention_rate.min(1.0))) +
                        (0.1 * (1.0 - factors.risk_score.min(1.0)));

        confidence.clamp(0.0, 1.0)
    }
}

impl Default for ConfidenceComputer {
    fn default() -> Self {
        Self::new(100, 10)
    }
}
```

---

### Step 3: Implement threshold enforcement

**File**: `src/confidence/enforcement.rs`

```rust
use super::threshold::*;
use super::computation::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementDecision {
    Approve,
    Reject,
    RequireConfirmation,
}

pub struct ConfidenceEnforcer {
    thresholds: ConfidenceThresholds,
}

impl ConfidenceEnforcer {
    pub fn new(thresholds: ConfidenceThresholds) -> Self {
        Self { thresholds }
    }

    pub fn enforce_action(&self, confidence: f64) -> EnforcementDecision {
        if confidence >= self.thresholds.auto_approve_threshold {
            EnforcementDecision::Approve
        } else if confidence >= self.thresholds.action_threshold {
            EnforcementDecision::RequireConfirmation
        } else {
            EnforcementDecision::Reject
        }
    }

    pub fn enforce_batch(&self, confidence: f64) -> EnforcementDecision {
        if confidence >= self.thresholds.batch_threshold {
            EnforcementDecision::Approve
        } else {
            EnforcementDecision::RequireConfirmation
        }
    }

    pub fn enforce_loop(&self, confidence: f64) -> EnforcementDecision {
        if confidence >= self.thresholds.loop_threshold {
            EnforcementDecision::Approve
        } else {
            EnforcementDecision::Reject
        }
    }

    pub fn set_thresholds(&mut self, thresholds: ConfidenceThresholds) -> Result<(), ThresholdError> {
        thresholds.validate()?;
        self.thresholds = thresholds;
        Ok(())
    }

    pub fn get_thresholds(&self) -> &ConfidenceThresholds {
        &self.thresholds
    }
}

impl Default for ConfidenceEnforcer {
    fn default() -> Self {
        Self::new(ConfidenceThresholds::default())
    }
}
```

---

### Step 4: Implement threshold tuning

**File**: `src/confidence/tuning.rs`

```rust
use super::threshold::*;
use super::computation::*;

pub struct ConfidenceTuner {
    current_thresholds: ConfidenceThresholds,
    target_success_rate: f64,
}

impl ConfidenceTuner {
    pub fn new(thresholds: ConfidenceThresholds, target_success_rate: f64) -> Self {
        Self {
            current_thresholds: thresholds,
            target_success_rate,
        }
    }

    pub fn tune(&mut self, current_success_rate: f64, intervention_rate: f64) {
        // Adjust thresholds based on performance
        if current_success_rate < self.target_success_rate {
            // Lower thresholds to allow more autonomy
            self.current_thresholds.action_threshold *= 0.95;
            self.current_thresholds.batch_threshold *= 0.95;
            self.current_thresholds.loop_threshold *= 0.95;
        } else if intervention_rate > 0.1 {
            // Raise thresholds to reduce interventions
            self.current_thresholds.action_threshold *= 1.05;
            self.current_thresholds.batch_threshold *= 1.05;
            self.current_thresholds.loop_threshold *= 1.05;
        }

        // Clamp to valid range
        self.current_thresholds.action_threshold = self.current_thresholds.action_threshold.clamp(0.1, 0.9);
        self.current_thresholds.batch_threshold = self.current_thresholds.batch_threshold.clamp(0.1, 0.95);
        self.current_thresholds.loop_threshold = self.current_thresholds.loop_threshold.clamp(0.1, 1.0);
    }

    pub fn get_thresholds(&self) -> &ConfidenceThresholds {
        &self.current_thresholds
    }
}
```

---

### Step 5: Create module exports

**File**: `src/confidence/mod.rs`

```rust
pub mod threshold;
pub mod computation;
pub mod enforcement;
pub mod tuning;

pub use threshold::*;
pub use computation::*;
pub use enforcement::*;
pub use tuning::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod confidence;
```

---

### Step 6: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test confidence --verbose
```

**Expected Output**: All tests pass

---

### Step 7: Integration test

**File**: `tests/confidence/integration_test.rs`

```rust
use glyphnova_engine::confidence::*;

#[test]
fn test_confidence_workflow() {
    let thresholds = ConfidenceThresholds::moderate();
    let computer = ConfidenceComputer::default();
    let enforcer = ConfidenceEnforcer::new(thresholds);

    let metrics = std::collections::HashMap::new();
    metrics.insert("risk_score".to_string(), 0.2);

    let history = vec![0.95, 0.96, 0.94, 0.97, 0.95, 0.96, 0.93, 0.98, 0.95, 0.97];

    let result = computer.compute(&metrics, &history, 5, 100);

    let decision = enforcer.enforce_action(result.confidence);

    assert!(result.confidence > 0.8);
    assert!(matches!(decision, EnforcementDecision::Approve | EnforcementDecision::RequireConfirmation));
}
```

---

### Step 8: Commit

```bash
git add src/confidence/ tests/confidence/
git commit -m "feat: implement confidence thresholds

- Define confidence threshold structure
- Implement confidence computation from metrics
- Implement threshold enforcement logic
- Implement threshold tuning based on performance
- Add comprehensive unit and integration tests

Relates to ADR-0008: Intelligent autonomous decision-making"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ Confidence is correctly computed from metrics
- ✅ Thresholds are enforced correctly
- ✅ Tuning adjusts thresholds appropriately

---

## Next Steps

After completing Task 08:
1. Proceed to Task 09: Autonomy CLI/UI Integration

---

**End of Task 08**
