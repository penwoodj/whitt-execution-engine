# Task 03: Intervention Tracking

**Estimated Time**: 5 days
**Priority**: HIGH - Essential for observability
**Dependencies**: Task 01 (metrics), Task 02 (override controls)

## Overview

Track all human interventions with full context and reason tracking. This ensures no silent failures and provides comprehensive audit trails.

## Files

### Create
- `src/intervention/mod.rs` - Module exports
- `src/intervention/events.rs` - Intervention event structures
- `src/intervention/context.rs` - Context capture
- `src/intervention/reason.rs` - Reason tracking
- `src/intervention/logging.rs` - Structured logging
- `src/intervention/analysis.rs` - Analysis logic (trends, frequency)
- `tests/intervention/intervention_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod intervention;`

---

## Implementation Steps

### Step 1: Define intervention event structures

**File**: `src/intervention/events.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Intervention event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionEvent {
    pub event_id: String,
    pub workflow_id: String,
    pub intervention_type: InterventionType,
    pub timestamp: DateTime<Utc>,
    pub operator: String,
    pub reason: InterventionReason,
    pub context: InterventionContext,
    pub impact: InterventionImpact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterventionType {
    Pause,
    Stop,
    Modify,
    ScopeChange,
    Override,
    Correction,
}

impl InterventionType {
    pub fn description(&self) -> &'static str {
        match self {
            InterventionType::Pause => "Paused execution",
            InterventionType::Stop => "Stopped execution",
            InterventionType::Modify => "Modified execution parameters",
            InterventionType::ScopeChange => "Changed autonomy scope",
            InterventionType::Override => "Overrode autonomous decision",
            InterventionType::Correction => "Corrected system behavior",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionReason {
    pub category: ReasonCategory,
    pub description: String,
    pub severity: Severity,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCategory {
    Safety,
    Performance,
    Correctness,
    UserRequest,
    ResourceLimit,
    UnexpectedBehavior,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionImpact {
    pub workflow_stopped: bool,
    pub actions_cancelled: usize,
    pub resources_freed: Option<ResourceImpact>,
    pub time_saved: Option<chrono::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceImpact {
    pub cpu_cores_freed: f64,
    pub memory_gb_freed: f64,
    pub cost_saved_usd: Option<f64>,
}
```

---

### Step 2: Implement context capture

**File**: `src/intervention/context.rs`

```rust
use super::events::*;
use std::collections::HashMap;

/// Intervention context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionContext {
    pub workflow_state: WorkflowState,
    pub current_action: Option<ActionContext>,
    pub metrics_snapshot: MetricsSnapshot,
    pub environment: EnvironmentContext,
    pub checkpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub autonomy_level: String,
    pub start_time: DateTime<Utc>,
    pub actions_completed: usize,
    pub actions_failed: usize,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub action_id: String,
    pub action_type: String,
    pub started_at: DateTime<Utc>,
    pub duration_ms: Option<f64>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub active_workflows: usize,
    pub active_actions: usize,
    pub cpu_usage_cores: f64,
    pub memory_usage_gb: f64,
    pub success_rate_last_hour: f64,
    pub intervention_count_last_hour: usize,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentContext {
    pub hostname: String,
    pub timezone: String,
    pub system_load: f64,
    pub available_memory_gb: f64,
}

pub struct InterventionContextBuilder {
    context: InterventionContext,
}

impl InterventionContextBuilder {
    pub fn new() -> Self {
        Self {
            context: InterventionContext {
                workflow_state: WorkflowState {
                    workflow_id: String::new(),
                    autonomy_level: String::new(),
                    start_time: Utc::now(),
                    actions_completed: 0,
                    actions_failed: 0,
                    success_rate: 0.0,
                },
                current_action: None,
                metrics_snapshot: MetricsSnapshot {
                    active_workflows: 0,
                    active_actions: 0,
                    cpu_usage_cores: 0.0,
                    memory_usage_gb: 0.0,
                    success_rate_last_hour: 0.0,
                    intervention_count_last_hour: 0,
                    custom_metrics: HashMap::new(),
                },
                environment: EnvironmentContext {
                    hostname: "unknown".to_string(),
                    timezone: "UTC".to_string(),
                    system_load: 0.0,
                    available_memory_gb: 0.0,
                },
                checkpoints: Vec::new(),
            },
        }
    }

    pub fn workflow_id(mut self, id: String) -> Self {
        self.context.workflow_state.workflow_id = id;
        self
    }

    pub fn autonomy_level(mut self, level: String) -> Self {
        self.context.workflow_state.autonomy_level = level;
        self
    }

    pub fn actions_completed(mut self, count: usize) -> Self {
        self.context.workflow_state.actions_completed = count;
        self
    }

    pub fn current_action(mut self, action: ActionContext) -> Self {
        self.context.current_action = Some(action);
        self
    }

    pub fn metrics_snapshot(mut self, snapshot: MetricsSnapshot) -> Self {
        self.context.metrics_snapshot = snapshot;
        self
    }

    pub fn add_checkpoint(mut self, checkpoint_id: String) -> Self {
        self.context.checkpoints.push(checkpoint_id);
        self
    }

    pub fn build(self) -> InterventionContext {
        self.context
    }
}

impl Default for InterventionContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 3: Implement reason tracking

**File**: `src/intervention/reason.rs`

```rust
use super::events::*;

pub struct InterventionReasonBuilder {
    reason: InterventionReason,
}

impl InterventionReasonBuilder {
    pub fn new(category: ReasonCategory) -> Self {
        Self {
            reason: InterventionReason {
                category,
                description: String::new(),
                severity: Severity::Medium,
                tags: Vec::new(),
            },
        }
    }

    pub fn description(mut self, desc: String) -> Self {
        self.reason.description = desc;
        self
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.reason.severity = severity;
        self
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        self.reason.tags.push(tag);
        self
    }

    pub fn build(self) -> InterventionReason {
        self.reason
    }
}

impl InterventionReason {
    pub fn is_safety_critical(&self) -> bool {
        matches!(self.category, ReasonCategory::Safety) &&
        matches!(self.severity, Severity::High | Severity::Critical)
    }

    pub fn requires_immediate_action(&self) -> bool {
        matches!(self.severity, Severity::Critical)
    }
}
```

---

### Step 4: Implement structured logging

**File**: `src/intervention/logging.rs`

```rust
use super::events::*;
use super::context::*;
use std::path::PathBuf;

/// Intervention logger
pub struct InterventionLogger {
    log_dir: PathBuf,
}

impl InterventionLogger {
    pub fn new(log_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&log_dir).ok();
        Self { log_dir }
    }

    pub fn log_intervention(&self, event: &InterventionEvent) -> std::io::Result<()> {
        let filename = format!(
            "{}_{}.json",
            event.workflow_id,
            event.timestamp.format("%Y%m%d_%H%M%S")
        );

        let filepath = self.log_dir.join("interventions").join(filename);
        std::fs::create_dir_all(filepath.parent().unwrap())?;

        let json = serde_json::to_string_pretty(event)?;
        std::fs::write(filepath, json)?;

        tracing::info!(
            workflow_id = %event.workflow_id,
            intervention_type = ?event.intervention_type,
            reason = %event.reason.description,
            "Intervention logged to {}",
            filepath.display()
        );

        Ok(())
    }

    pub fn log_to_tracing(&self, event: &InterventionEvent) {
        let level = match event.reason.severity {
            Severity::Low => tracing::Level::INFO,
            Severity::Medium => tracing::Level::WARN,
            Severity::High => tracing::Level::WARN,
            Severity::Critical => tracing::Level::ERROR,
        };

        tracing::event!(
            level,
            event_id = %event.event_id,
            workflow_id = %event.workflow_id,
            intervention_type = ?event.intervention_type,
            reason_category = ?event.reason.category,
            reason_description = %event.reason.description,
            severity = ?event.reason.severity,
            operator = %event.operator,
            "Human intervention occurred"
        );
    }
}

impl Default for InterventionLogger {
    fn default() -> Self {
        Self::new(PathBuf::from(".glyphnova/interventions"))
    }
}
```

---

### Step 5: Implement analysis logic

**File**: `src/intervention/analysis.rs`

```rust
use super::events::*;
use std::collections::HashMap;

/// Intervention analyzer
pub struct InterventionAnalyzer {
    interventions: Vec<InterventionEvent>,
}

impl InterventionAnalyzer {
    pub fn new() -> Self {
        Self {
            interventions: Vec::new(),
        }
    }

    pub fn add_intervention(&mut self, event: InterventionEvent) {
        self.interventions.push(event);
    }

    pub fn intervention_frequency(&self, window_hours: u64) -> f64 {
        let cutoff = Utc::now() - chrono::Duration::hours(window_hours as i64);
        let count = self.interventions.iter()
            .filter(|e| e.timestamp > cutoff)
            .count();
        count as f64 / window_hours as f64
    }

    pub fn intervention_rate_by_type(&self) -> HashMap<InterventionType, usize> {
        let mut counts = HashMap::new();
        for intervention in &self.interventions {
            *counts.entry(intervention.intervention_type).or_insert(0) += 1;
        }
        counts
    }

    pub fn intervention_rate_by_reason(&self) -> HashMap<ReasonCategory, usize> {
        let mut counts = HashMap::new();
        for intervention in &self.interventions {
            *counts.entry(intervention.reason.category).or_insert(0) += 1;
        }
        counts
    }

    pub fn trending_interventions(&self, window_hours: u64) -> Vec<Trend> {
        let cutoff = Utc::now() - chrono::Duration::hours(window_hours as i64);
        let recent: Vec<_> = self.interventions.iter()
            .filter(|e| e.timestamp > cutoff)
            .collect();

        let mut trends = HashMap::new();
        for intervention in &recent {
            let key = (intervention.intervention_type, intervention.reason.category.clone());
            *trends.entry(key).or_insert(0) += 1;
        }

        trends.into_iter()
            .filter(|&(_, count)| count >= 3)
            .map(|((intervention_type, reason_category), count)| {
                Trend {
                    intervention_type,
                    reason_category,
                    count,
                    window_hours,
                }
            })
            .collect()
    }

    pub fn get_safety_critical_interventions(&self) -> Vec<&InterventionEvent> {
        self.interventions.iter()
            .filter(|e| e.reason.is_safety_critical())
            .collect()
    }

    pub fn analyze_intervention_patterns(&self) -> AnalysisReport {
        let total = self.interventions.len();
        let by_type = self.intervention_rate_by_type();
        let by_reason = self.intervention_rate_by_reason();
        let trending = self.trending_interventions(24);
        let safety_critical = self.get_safety_critical_interventions();

        AnalysisReport {
            total_interventions: total,
            frequency_per_hour: self.intervention_frequency(24),
            by_type,
            by_reason,
            trending,
            safety_critical_count: safety_critical.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub intervention_type: InterventionType,
    pub reason_category: ReasonCategory,
    pub count: usize,
    pub window_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub total_interventions: usize,
    pub frequency_per_hour: f64,
    pub by_type: HashMap<InterventionType, usize>,
    pub by_reason: HashMap<ReasonCategory, usize>,
    pub trending: Vec<Trend>,
    pub safety_critical_count: usize,
}

impl Default for InterventionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 6: Create module exports

**File**: `src/intervention/mod.rs`

```rust
pub mod events;
pub mod context;
pub mod reason;
pub mod logging;
pub mod analysis;

pub use events::*;
pub use context::*;
pub use reason::*;
pub use logging::*;
pub use analysis::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod intervention;
```

---

### Step 7: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test intervention --verbose
```

**Expected Output**: All tests pass

---

### Step 8: Integration test for full intervention workflow

**File**: `tests/intervention/integration_test.rs`

```rust
use glyphnova_engine::intervention::*;

#[test]
fn test_full_intervention_workflow() {
    let logger = InterventionLogger::default();

    // Create intervention event
    let event = InterventionEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        workflow_id: "workflow_123".to_string(),
        intervention_type: InterventionType::Stop,
        timestamp: Utc::now(),
        operator: "operator_1".to_string(),
        reason: InterventionReasonBuilder::new(ReasonCategory::Safety)
            .description("Critical safety concern detected".to_string())
            .severity(Severity::Critical)
            .add_tag("safety".to_string())
            .build(),
        context: InterventionContextBuilder::new()
            .workflow_id("workflow_123".to_string())
            .autonomy_level("high".to_string())
            .build(),
        impact: InterventionImpact {
            workflow_stopped: true,
            actions_cancelled: 5,
            resources_freed: None,
            time_saved: None,
        },
    };

    // Log intervention
    logger.log_intervention(&event).expect("Failed to log intervention");
    logger.log_to_tracing(&event);

    // Analyze interventions
    let mut analyzer = InterventionAnalyzer::new();
    analyzer.add_intervention(event);

    let report = analyzer.analyze_intervention_patterns();
    assert_eq!(report.total_interventions, 1);
}
```

---

### Step 9: Commit

```bash
git add src/intervention/ tests/intervention/
git commit -m "feat: implement intervention tracking

- Define InterventionEvent with full context capture
- Implement InterventionContextBuilder for flexible context creation
- Implement InterventionReasonBuilder for reason tracking
- Implement InterventionLogger for structured logging to JSON
- Implement InterventionAnalyzer for pattern analysis
- Add comprehensive unit and integration tests
- Verify all interventions are logged with full context

Relates to ADR-0008: No silent failures, full audit trail"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ All interventions are logged with full context
- ✅ Analysis correctly identifies trends and patterns
- ✅ Safety critical interventions are flagged

---

## Next Steps

After completing Task 03:
1. Proceed to Task 04: Success Regression Dashboards

---

**End of Task 03**
