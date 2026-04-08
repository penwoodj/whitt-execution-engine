# Phase 07 Autonomy & Metrics - Autonomy Mocks

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define mock strategies for testing autonomy and metrics components

**Architecture:** Mocks provide controllable test doubles for external dependencies

**Tech Stack:** Rust, mockall, async/await, tokio

---

## Mock Strategy Overview

Mock strategies provide:
1. Mock Autonomous Workflow - Pauses at predetermined points
2. Mock Metrics Collector - Generates synthetic metrics for testing dashboards
3. Mock Risk Assessor - Configurable risk levels
4. Mock Checkpoint Store - Injectable state
5. Mock Override Event Stream - Tests intervention logging

---

## Mock1: Autonomous Workflow Mock

**Files:**
- Create: `tests/mocks/autonomous_workflow_mock.rs`

**Purpose:** Mock autonomous workflow that pauses at predetermined points for testing override functionality.

- [ ] **Step 1: Write autonomous workflow mock**

```rust
use mockall::mock;
use yaml_to_rust_agentsdk::autonomy::executor::{AutonomousLoop, IterationResult};
use yaml_to_rust_agentsdk::autonomy::contract::AutonomyContract;
use yaml_to_rust_agentsdk::autonomy::metrics::MetricsCollector;
use std::sync::Arc;
use std::collections::HashSet;

#[automock]
pub trait AutonomousWorkflow {
    async fn start(&mut self) -> Result<(), String>;
    async fn execute_iteration(&mut self) -> IterationResult;
    async fn stop(&mut self) -> Result<(), String>;
    fn is_complete(&self) -> bool;
    fn get_state(&self) -> serde_json::Value;
}

pub struct MockAutonomousWorkflow {
    pause_at_iterations: HashSet<u32>,
    current_iteration: u32,
    max_iterations: u32,
    is_paused: bool,
    is_stopped: bool,
    state: serde_json::Value,
}

impl MockAutonomousWorkflow {
    pub fn new(pause_at_iterations: Vec<u32>) -> Self {
        Self {
            pause_at_iterations: pause_at_iterations.into_iter().collect(),
            current_iteration: 0,
            max_iterations: 100,
            is_paused: false,
            is_stopped: false,
            state: serde_json::json!({
                "iteration": 0,
                "status": "not_started",
            }),
        }
    }

    pub fn with_max_iterations(mut self, max: u32) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn pause_at(mut self, iteration: u32) -> Self {
        self.pause_at_iterations.insert(iteration);
        self
    }
}

#[async_trait::async_trait]
impl AutonomousWorkflow for MockAutonomousWorkflow {
    async fn start(&mut self) -> Result<(), String> {
        self.state = serde_json::json!({
            "iteration": self.current_iteration,
            "status": "running",
        });
        Ok(())
    }

    async fn execute_iteration(&mut self) -> IterationResult {
        if self.is_stopped {
            return IterationResult {
                should_terminate: true,
                reason: "Workflow stopped".to_string(),
            };
        }

        if self.is_paused {
            return IterationResult {
                should_terminate: false,
                reason: "Workflow paused".to_string(),
            };
        }

        self.current_iteration += 1;

        // Check if should pause
        if self.pause_at_iterations.contains(&self.current_iteration) {
            self.is_paused = true;
            self.state = serde_json::json!({
                "iteration": self.current_iteration,
                "status": "paused",
                "pause_reason": format!("Paused at iteration {}", self.current_iteration),
            });

            return IterationResult {
                should_terminate: false,
                reason: format!("Paused at iteration {}", self.current_iteration),
            };
        }

        // Update state
        self.state = serde_json::json!({
            "iteration": self.current_iteration,
            "status": "running",
        });

        // Check if complete
        let should_terminate = self.current_iteration >= self.max_iterations;

        IterationResult {
            should_terminate,
            reason: if should_terminate {
                format!("Completed {} iterations", self.current_iteration)
            } else {
                "Iteration complete".to_string()
            },
        }
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.is_stopped = true;
        self.state = serde_json::json!({
            "iteration": self.current_iteration,
            "status": "stopped",
        });
        Ok(())
    }

    fn is_complete(&self) -> bool {
        self.current_iteration >= self.max_iterations || self.is_stopped
    }

    fn get_state(&self) -> serde_json::Value {
        self.state.clone()
    }
}
```

- [ ] **Step 2: Write autonomous workflow mock tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_workflow_starts() {
        let mut workflow = MockAutonomousWorkflow::new(vec![]);
        workflow.start().await.unwrap();
        let state = workflow.get_state();

        assert_eq!(state["status"], "running");
    }

    #[tokio::test]
    async fn test_mock_workflow_pauses_at_specified_iteration() {
        let mut workflow = MockAutonomousWorkflow::new(vec![5, 10, 15]);
        workflow.start().await.unwrap();

        // Execute to iteration 5
        for _ in 1..=5 {
            let result = workflow.execute_iteration().await;
            assert!(!result.should_terminate);
        }

        // Should be paused
        let state = workflow.get_state();
        assert_eq!(state["status"], "paused");
        assert!(state["pause_reason"].as_str().unwrap().contains("5"));
    }

    #[tokio::test]
    async fn test_mock_workflow_stops_at_max_iterations() {
        let mut workflow = MockAutonomousWorkflow::new(vec![])
            .with_max_iterations(10);
        workflow.start().await.unwrap();

        // Execute all iterations
        for _ in 1..=10 {
            let result = workflow.execute_iteration().await;
        }

        // Should be complete
        assert!(workflow.is_complete());
    }
}
```

- [ ] **Step 3: Run autonomous workflow mock tests**

Run: `cargo test test_mock_workflow --test autonomous_workflow_mock`
Expected: PASS

- [ ] **Step 4: Commit Mock1**

```bash
git add tests/mocks/autonomous_workflow_mock.rs
git commit -m "test(Mock1): add mock autonomous workflow strategy"
```

---

## Mock2: Metrics Collector Mock

**Files:**
- Create: `tests/mocks/metrics_collector_mock.rs`

**Purpose:** Mock metrics collector generating synthetic metrics for testing dashboards.

- [ ] **Step 1: Write metrics collector mock**

```rust
use mockall::mock;
use yaml_to_rust_agentsdk::autonomy::metrics::{MetricsCollector, HistogramSummary};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[automock]
pub trait MetricsCollector: Send + Sync {
    async fn increment_counter(&self, name: &str, value: u64, labels: &[(&str, &str)]);
    async fn get_counter_value(&self, name: &str) -> u64;
    async fn set_gauge(&self, name: &str, value: f64, labels: &[(&str, &str)]);
    async fn get_gauge_value(&self, name: &str) -> f64;
    async fn observe_histogram(&self, name: &str, value: f64, labels: &[(&str, &str)]);
    async fn get_histogram_summary(&self, name: &str) -> HistogramSummary;
    async fn export_all(&self) -> Vec<crate::autonomy::metrics::MetricSnapshot>;
}

pub struct MockMetricsCollector {
    counters: Arc<RwLock<HashMap<String, u64>>>,
    gauges: Arc<RwLock<HashMap<String, f64>>>,
    histograms: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    synthetic_data: SyntheticMetricsConfig,
}

#[derive(Clone)]
pub struct SyntheticMetricsConfig {
    pub tasks_completed: u64,
    pub goals_achieved: u64,
    pub avg_time_to_usefulness_ms: f64,
    pub quality_score: f64,
    pub success_rate: f64,
    pub interventions_total: u64,
    pub interventions_per_hour: f64,
    pub error_rate: f64,
    pub rollback_count: u64,
}

impl Default for SyntheticMetricsConfig {
    fn default() -> Self {
        Self {
            tasks_completed: 150,
            goals_achieved: 45,
            avg_time_to_usefulness_ms: 2500.0,
            quality_score: 87.5,
            success_rate: 0.92,
            interventions_total: 12,
            interventions_per_hour: 2.4,
            error_rate: 0.08,
            rollback_count: 2,
        }
    }
}

impl MockMetricsCollector {
    pub fn new() -> Self {
        Self::with_config(SyntheticMetricsConfig::default())
    }

    pub fn with_config(config: SyntheticMetricsConfig) -> Self {
        Self {
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
            synthetic_data: config,
        }
    }

    pub async fn generate_synthetic_metrics(&self) {
        let mut counters = self.counters.write().await;
        let mut gauges = self.gauges.write().await;
        let mut histograms = self.histograms.write().await;

        // Generate synthetic counter metrics
        counters.insert("tasks_completed".to_string(), self.synthetic_data.tasks_completed);
        counters.insert("goals_achieved".to_string(), self.synthetic_data.goals_achieved);
        counters.insert("interventions_total".to_string(), self.synthetic_data.interventions_total);
        counters.insert("rollback_count".to_string(), self.synthetic_data.rollback_count);

        // Generate synthetic gauge metrics
        gauges.insert("avg_time_to_usefulness_ms".to_string(), self.synthetic_data.avg_time_to_usefulness_ms);
        gauges.insert("quality_score".to_string(), self.synthetic_data.quality_score);
        gauges.insert("success_rate".to_string(), self.synthetic_data.success_rate);
        gauges.insert("interventions_per_hour".to_string(), self.synthetic_data.interventions_per_hour);
        gauges.insert("error_rate".to_string(), self.synthetic_data.error_rate);

        // Generate synthetic histogram samples
        let durations = (0..10).map(|i| {
            self.synthetic_data.avg_time_to_usefulness_ms * (0.8 + (i as f64 * 0.04))
        }).collect();

        histograms.insert("task_duration_ms".to_string(), durations);
    }
}

#[async_trait::async_trait]
impl MetricsCollector for MockMetricsCollector {
    async fn increment_counter(&self, name: &str, value: u64, _labels: &[(&str, &str)]) {
        let mut counters = self.counters.write().await;
        *counters.entry(name.to_string()).or_insert(0) += value;
    }

    async fn get_counter_value(&self, name: &str) -> u64 {
        let counters = self.counters.read().await;
        counters.get(name).copied().unwrap_or(0)
    }

    async fn set_gauge(&self, name: &str, value: f64, _labels: &[(&str, &str)]) {
        let mut gauges = self.gauges.write().await;
        gauges.insert(name.to_string(), value);
    }

    async fn get_gauge_value(&self, name: &str) -> f64 {
        let gauges = self.gauges.read().await;
        gauges.get(name).copied().unwrap_or(0.0)
    }

    async fn observe_histogram(&self, name: &str, value: f64, _labels: &[(&str, &str)]) {
        let mut histograms = self.histograms.write().await;
        histograms.entry(name.to_string()).or_insert_with(Vec::new).push(value);
    }

    async fn get_histogram_summary(&self, name: &str) -> HistogramSummary {
        let histograms = self.histograms.read().await;
        if let Some(samples) = histograms.get(name) {
            let mut sorted = samples.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let count = samples.len();
            let sum: f64 = samples.iter().sum();
            let min = *samples.first().unwrap_or(&0.0);
            let max = *samples.last().unwrap_or(&0.0);
            let avg = if count > 0 { sum / count as f64 } else { 0.0 };

            let p50 = sorted[(count as f64 * 0.5) as usize];
            let p95 = sorted[(count as f64 * 0.95) as usize];
            let p99 = sorted[(count as f64 * 0.99) as usize];

            HistogramSummary {
                count,
                sum,
                min,
                max,
                avg,
                p50,
                p95,
                p99,
            }
        } else {
            HistogramSummary::default()
        }
    }

    async fn export_all(&self) -> Vec<crate::autonomy::metrics::MetricSnapshot> {
        let counters = self.counters.read().await;
        let gauges = self.gauges.read().await;

        let mut snapshots = Vec::new();

        // Export counters
        for (name, value) in counters.iter() {
            snapshots.push(crate::autonomy::metrics::MetricSnapshot::Counter(
                crate::autonomy::metrics::Counter {
                    name: name.clone(),
                    value: *value,
                    labels: vec![],
                }
            ));
        }

        // Export gauges
        for (name, value) in gauges.iter() {
            snapshots.push(crate::autonomy::metrics::MetricSnapshot::Gauge(
                crate::autonomy::metrics::Gauge {
                    name: name.clone(),
                    value: *value,
                    labels: vec![],
                }
            ));
        }

        snapshots
    }
}
```

- [ ] **Step 2: Write metrics collector mock tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_metrics_collector_generates_synthetic_metrics() {
        let collector = MockMetricsCollector::new();
        collector.generate_synthetic_metrics().await;

        let tasks_completed = collector.get_counter_value("tasks_completed").await;
        assert_eq!(tasks_completed, 150);

        let quality_score = collector.get_gauge_value("quality_score").await;
        assert_eq!(quality_score, 87.5);
    }

    #[tokio::test]
    async fn test_mock_metrics_collector_custom_config() {
        let config = SyntheticMetricsConfig {
            tasks_completed: 200,
            goals_achieved: 60,
            avg_time_to_usefulness_ms: 3000.0,
            quality_score: 95.0,
            success_rate: 0.95,
            interventions_total: 5,
            interventions_per_hour: 1.0,
            error_rate: 0.05,
            rollback_count: 0,
        };

        let collector = MockMetricsCollector::with_config(config.clone());
        collector.generate_synthetic_metrics().await;

        let tasks_completed = collector.get_counter_value("tasks_completed").await;
        assert_eq!(tasks_completed, config.tasks_completed);

        let quality_score = collector.get_gauge_value("quality_score").await;
        assert_eq!(quality_score, config.quality_score);
    }

    #[tokio::test]
    async fn test_mock_metrics_collector_histogram_summary() {
        let collector = MockMetricsCollector::new();

        // Add samples manually
        for value in [10.0, 20.0, 30.0, 40.0, 50.0] {
            collector.observe_histogram("test_histogram", value, &[]).await;
        }

        let summary = collector.get_histogram_summary("test_histogram").await;

        assert_eq!(summary.count, 5);
        assert_eq!(summary.sum, 150.0);
        assert_eq!(summary.avg, 30.0);
        assert_eq!(summary.min, 10.0);
        assert_eq!(summary.max, 50.0);
    }
}
```

- [ ] **Step 3: Run metrics collector mock tests**

Run: `cargo test test_mock_metrics_collector --test metrics_collector_mock`
Expected: PASS

- [ ] **Step 4: Commit Mock2**

```bash
git add tests/mocks/metrics_collector_mock.rs
git commit -m "test(Mock2): add mock metrics collector strategy"
```

---

## Mock3: Risk Assessor Mock

**Files:**
- Create: `tests/mocks/risk_assessor_mock.rs`

**Purpose:** Mock risk assessor with configurable risk levels.

- [ ] **Step 1: Write risk assessor mock**

```rust
use mockall::mock;
use yaml_to_rust_agentsdk::autonomy::risk::{RiskAssessor, RiskProfile, Probability, Impact, Severity};
use yaml_to_rust_agentsdk::autonomy::contract::AutonomyLevel;

#[automock]
pub trait RiskAssessor: Send + Sync {
    fn compute_probability(&self, profile: &RiskProfile) -> Probability;
    fn compute_impact(&self, profile: &RiskProfile) -> Impact;
    fn compute_severity(&self, probability: Probability, impact: Impact) -> Severity;
    fn compute_risk_score(&self, profile: &RiskProfile) -> f64;
    fn adjust_autonomy_level(&self, profile: &RiskProfile) -> AutonomyLevel;
}

#[derive(Clone, Copy)]
pub enum MockRiskLevel {
    Low,
    Medium,
    High,
}

pub struct MockRiskAssessor {
    risk_level: MockRiskLevel,
}

impl MockRiskAssessor {
    pub fn new() -> Self {
        Self {
            risk_level: MockRiskLevel::Medium,
        }
    }

    pub fn with_risk_level(mut self, level: MockRiskLevel) -> Self {
        self.risk_level = level;
        self
    }

    fn get_risk_level(&self) -> MockRiskLevel {
        self.risk_level
    }
}

impl RiskAssessor for MockRiskAssessor {
    fn compute_probability(&self, _profile: &RiskProfile) -> Probability {
        match self.risk_level {
            MockRiskLevel::Low => Probability::Low,
            MockRiskLevel::Medium => Probability::Medium,
            MockRiskLevel::High => Probability::High,
        }
    }

    fn compute_impact(&self, _profile: &RiskProfile) -> Impact {
        match self.risk_level {
            MockRiskLevel::Low => Impact::Low,
            MockRiskLevel::Medium => Impact::Medium,
            MockRiskLevel::High => Impact::High,
        }
    }

    fn compute_severity(&self, probability: Probability, impact: Impact) -> Severity {
        match (probability, impact) {
            (Probability::Low, Impact::Low) => Severity::Low,
            (Probability::Medium, Impact::Medium) => Severity::Medium,
            (Probability::High, Impact::High) => Severity::Critical,
            _ => Severity::Medium,
        }
    }

    fn compute_risk_score(&self, _profile: &RiskProfile) -> f64 {
        match self.risk_level {
            MockRiskLevel::Low => 0.2,
            MockRiskLevel::Medium => 0.5,
            MockRiskLevel::High => 0.8,
        }
    }

    fn adjust_autonomy_level(&self, _profile: &RiskProfile) -> AutonomyLevel {
        match self.risk_level {
            MockRiskLevel::Low => AutonomyLevel::Supervised,
            MockRiskLevel::Medium => AutonomyLevel::Bounded,
            MockRiskLevel::High => AutonomyLevel::Manual,
        }
    }
}
```

- [ ] **Step 2: Write risk assessor mock tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_risk_assessor_low_risk() {
        let assessor = MockRiskAssessor::new()
            .with_risk_level(MockRiskLevel::Low);

        assert_eq!(assessor.get_risk_level(), MockRiskLevel::Low);

        let profile = RiskProfile::default();
        let probability = assessor.compute_probability(&profile);
        let impact = assessor.compute_impact(&profile);
        let risk_score = assessor.compute_risk_score(&profile);
        let autonomy_level = assessor.adjust_autonomy_level(&profile);

        assert_eq!(probability, Probability::Low);
        assert_eq!(impact, Impact::Low);
        assert!(risk_score < 0.5);
        assert_eq!(autonomy_level, AutonomyLevel::Supervised);
    }

    #[test]
    fn test_mock_risk_assessor_high_risk() {
        let assessor = MockRiskAssessor::new()
            .with_risk_level(MockRiskLevel::High);

        assert_eq!(assessor.get_risk_level(), MockRiskLevel::High);

        let profile = RiskProfile::default();
        let probability = assessor.compute_probability(&profile);
        let impact = assessor.compute_impact(&profile);
        let risk_score = assessor.compute_risk_score(&profile);
        let autonomy_level = assessor.adjust_autonomy_level(&profile);

        assert_eq!(probability, Probability::High);
        assert_eq!(impact, Impact::High);
        assert!(risk_score > 0.5);
        assert_eq!(autonomy_level, AutonomyLevel::Manual);
    }

    #[test]
    fn test_mock_risk_assessor_severity_computation() {
        let assessor = MockRiskAssessor::new()
            .with_risk_level(MockRiskLevel::Low);

        let profile = RiskProfile::default();
        let probability = assessor.compute_probability(&profile);
        let impact = assessor.compute_impact(&profile);

        let severity = assessor.compute_severity(probability, impact);

        assert_eq!(severity, Severity::Low);
    }
}
```

- [ ] **Step 3: Run risk assessor mock tests**

Run: `cargo test test_mock_risk_assessor --test risk_assessor_mock`
Expected: PASS

- [ ] **Step 4: Commit Mock3**

```bash
git add tests/mocks/risk_assessor_mock.rs
git commit -m "test(Mock3): add mock risk assessor strategy"
```

---

## Mock4: Checkpoint Store Mock

**Files:**
- Create: `tests/mocks/checkpoint_store_mock.rs`

**Purpose:** Mock checkpoint store with injectable state.

- [ ] **Step 1: Write checkpoint store mock**

```rust
use mockall::mock;
use yaml_to_rust_agentsdk::autonomy::checkpoint::{
    CheckpointManager, CheckpointMetadata, CheckpointType, RestoreError
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[automock]
pub trait CheckpointStore: Send + Sync {
    async fn generate_checkpoint(
        &self,
        task_id: String,
        state: serde_json::Value,
        metadata: CheckpointMetadata,
    ) -> String;

    async fn load_checkpoint(&self, id: &str) -> Result<Checkpoint, RestoreError>;
    async fn list_checkpoints(&self, task_id: &str) -> Vec<Checkpoint>;
    async fn delete_checkpoint(&self, id: &str) -> bool;
    async fn delete_all_checkpoints(&self, task_id: &str) -> usize;
}

pub struct MockCheckpointStore {
    checkpoints: Arc<RwLock<HashMap<String, Checkpoint>>>,
    inject_state: Option<serde_json::Value>,
}

pub struct Checkpoint {
    pub id: String,
    pub task_id: String,
    pub timestamp: std::time::SystemTime,
    pub state: serde_json::Value,
    pub metadata: CheckpointMetadata,
}

impl MockCheckpointStore {
    pub fn new() -> Self {
        Self {
            checkpoints: Arc::new(RwLock::new(HashMap::new())),
            inject_state: None,
        }
    }

    pub fn with_injected_state(mut self, state: serde_json::Value) -> Self {
        self.inject_state = Some(state);
        self
    }

    pub async fn inject_checkpoint(
        &self,
        task_id: String,
        state: serde_json::Value,
        metadata: CheckpointMetadata,
    ) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let checkpoint = Checkpoint {
            id: id.clone(),
            task_id,
            timestamp: std::time::SystemTime::now(),
            state,
            metadata,
        };

        let mut checkpoints = self.checkpoints.write().await;
        checkpoints.insert(id.clone(), checkpoint);

        id
    }
}

#[async_trait::async_trait]
impl CheckpointStore for MockCheckpointStore {
    async fn generate_checkpoint(
        &self,
        task_id: String,
        state: serde_json::Value,
        metadata: CheckpointMetadata,
    ) -> String {
        let actual_state = self.inject_state.as_ref().unwrap_or(&state).clone();

        let id = uuid::Uuid::new_v4().to_string();
        let checkpoint = Checkpoint {
            id: id.clone(),
            task_id,
            timestamp: std::time::SystemTime::now(),
            state: actual_state,
            metadata,
        };

        let mut checkpoints = self.checkpoints.write().await;
        checkpoints.insert(id.clone(), checkpoint);

        id
    }

    async fn load_checkpoint(&self, id: &str) -> Result<Checkpoint, RestoreError> {
        let checkpoints = self.checkpoints.read().await;
        checkpoints.get(id).cloned()
            .ok_or_else(|| RestoreError::CheckpointNotFound(id.to_string()))
    }

    async fn list_checkpoints(&self, task_id: &str) -> Vec<Checkpoint> {
        let checkpoints = self.checkpoints.read().await;
        checkpoints.values()
            .filter(|c| c.task_id == task_id)
            .cloned()
            .collect()
    }

    async fn delete_checkpoint(&self, id: &str) -> bool {
        let mut checkpoints = self.checkpoints.write().await;
        checkpoints.remove(id).is_some()
    }

    async fn delete_all_checkpoints(&self, task_id: &str) -> usize {
        let mut checkpoints = self.checkpoints.write().await;
        let mut count = 0;

        checkpoints.retain(|id, checkpoint| {
            if checkpoint.task_id == task_id {
                false
            } else {
                true
            }
        });

        count
    }
}
```

- [ ] **Step 2: Write checkpoint store mock tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_checkpoint_store_inject_state() {
        let injected_state = serde_json::json!({
            "iteration": 100,
            "custom_field": "injected_value",
        });

        let store = MockCheckpointStore::new()
            .with_injected_state(injected_state.clone());

        let id = store.generate_checkpoint(
            "test_task".to_string(),
            serde_json::json!({"iteration": 1}), // This will be overridden
            CheckpointMetadata {
                reason: "Test".to_string(),
                checkpoint_type: CheckpointType::Manual,
                iteration: 1,
                tags: vec![],
            },
        ).await;

        let checkpoint = store.load_checkpoint(&id).await.unwrap();

        // Should have injected state
        assert_eq!(checkpoint.state, injected_state);
    }

    #[tokio::test]
    async fn test_mock_checkpoint_store_list_by_task() {
        let store = MockCheckpointStore::new();

        // Create checkpoints for different tasks
        let id1 = store.generate_checkpoint(
            "task1".to_string(),
            serde_json::json!({"iteration": 1}),
            CheckpointMetadata {
                reason: "Checkpoint 1".to_string(),
                checkpoint_type: CheckpointType::Manual,
                iteration: 1,
                tags: vec![],
            },
        ).await;

        let id2 = store.generate_checkpoint(
            "task1".to_string(),
            serde_json::json!({"iteration": 2}),
            CheckpointMetadata {
                reason: "Checkpoint 2".to_string(),
                checkpoint_type: CheckpointType::Manual,
                iteration: 2,
                tags: vec![],
            },
        ).await;

        let id3 = store.generate_checkpoint(
            "task2".to_string(),
            serde_json::json!({"iteration": 3}),
            CheckpointMetadata {
                reason: "Checkpoint 3".to_string(),
                checkpoint_type: CheckpointType::Manual,
                iteration: 3,
                tags: vec![],
            },
        ).await;

        // List checkpoints for task1
        let checkpoints = store.list_checkpoints("task1").await;
        assert_eq!(checkpoints.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_checkpoint_store_delete() {
        let store = MockCheckpointStore::new();

        let id = store.generate_checkpoint(
            "test_task".to_string(),
            serde_json::json!({"iteration": 1}),
            CheckpointMetadata {
                reason: "Test".to_string(),
                checkpoint_type: CheckpointType::Manual,
                iteration: 1,
                tags: vec![],
            },
        ).await;

        // Verify checkpoint exists
        let _checkpoint = store.load_checkpoint(&id).await.unwrap();

        // Delete checkpoint
        let deleted = store.delete_checkpoint(&id).await;
        assert!(deleted);

        // Verify checkpoint is deleted
        let result = store.load_checkpoint(&id).await;
        assert!(result.is_err());
    }
}
```

- [ ] **Step 3: Run checkpoint store mock tests**

Run: `cargo test test_mock_checkpoint_store --test checkpoint_store_mock`
Expected: PASS

- [ ] **Step 4: Commit Mock4**

```bash
git add tests/mocks/checkpoint_store_mock.rs
git commit -m "test(Mock4): add mock checkpoint store strategy"
```

---

## Mock5: Override Event Stream Mock

**Files:**
- Create: `tests/mocks/override_event_stream_mock.rs`

**Purpose:** Mock override event stream for testing intervention logging.

- [ ] **Step 1: Write override event stream mock**

```rust
use mockall::mock;
use yaml_to_rust_agentsdk::autonomy::override::{OverrideController, OverrideCommand, OverrideReason, OverrideEvent};
use std::sync::Arc;
use std::collections::VecDeque;
use tokio::sync::{broadcast, RwLock};

#[automock]
pub trait OverrideEventStream: Send + Sync {
    async fn send_command(&self, command: OverrideCommand);
    async fn get_state(&self) -> crate::autonomy::override::ExecutionState;
    async fn subscribe_events(&self) -> broadcast::Receiver<OverrideEvent>;
    async fn get_event_history(&self) -> Vec<OverrideEvent>;
}

pub struct MockOverrideEventStream {
    state: Arc<RwLock<crate::autonomy::override::ExecutionState>>,
    event_tx: broadcast::Sender<OverrideEvent>,
    event_history: Arc<RwLock<VecDeque<OverrideEvent>>>,
    max_history: usize,
}

impl MockOverrideEventStream {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(100);

        Self {
            state: Arc::new(RwLock::new(crate::autonomy::override::ExecutionState::Running)),
            event_tx,
            event_history: Arc::new(RwLock::new(VecDeque::new())),
            max_history: 1000,
        }
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    pub fn with_initial_state(mut self, state: crate::autonomy::override::ExecutionState) -> Self {
        *self.state.try_write().unwrap() = state;
        self
    }

    async fn inject_event(&self, event: OverrideEvent) {
        let mut history = self.event_history.write().await;

        if history.len() >= self.max_history {
            history.pop_front();
        }

        history.push_back(event);

        let _ = self.event_tx.send(event);
    }
}

#[async_trait::async_trait]
impl OverrideEventStream for MockOverrideEventStream {
    async fn send_command(&self, command: OverrideCommand) {
        let mut state = self.state.write().await;
        let previous_state = state.clone();

        match &command {
            OverrideCommand::Pause { .. } => {
                *state = crate::autonomy::override::ExecutionState::Paused;
            },
            OverrideCommand::Resume => {
                if matches!(previous_state, crate::autonomy::override::ExecutionState::Paused) {
                    *state = crate::autonomy::override::ExecutionState::Running;
                }
            },
            OverrideCommand::Stop { .. } => {
                *state = crate::autonomy::override::ExecutionState::Stopped;
            },
            OverrideCommand::ModifyScope { .. } | OverrideCommand::AdjustAutonomy { .. } => {
                // State doesn't change
            },
        }

        let new_state = state.clone();

        // Create and emit event
        let event = OverrideEvent {
            id: uuid::Uuid::new_v4(),
            command,
            timestamp: std::time::SystemTime::now(),
            previous_state,
            new_state,
            context: crate::autonomy::override::OverrideContext {
                iteration: 0,
                current_goal: "test".to_string(),
                metrics_snapshot: serde_json::json!({}),
            },
        };

        self.inject_event(event).await;
    }

    async fn get_state(&self) -> crate::autonomy::override::ExecutionState {
        *self.state.read().await
    }

    async fn subscribe_events(&self) -> broadcast::Receiver<OverrideEvent> {
        self.event_tx.subscribe()
    }

    async fn get_event_history(&self) -> Vec<OverrideEvent> {
        self.event_history.read().await.iter().cloned().collect()
    }
}
```

- [ ] **Step 2: Write override event stream mock tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_override_event_stream_pause_resume() {
        let stream = MockOverrideEventStream::new();

        // Send pause command
        stream.send_command(OverrideCommand::Pause {
            reason: OverrideReason::Manual,
            requested_by: "test".to_string(),
        }).await;

        // Check state
        let state = stream.get_state().await;
        assert!(matches!(state, crate::autonomy::override::ExecutionState::Paused));

        // Send resume command
        stream.send_command(OverrideCommand::Resume).await;

        // Check state
        let state = stream.get_state().await;
        assert!(matches!(state, crate::autonomy::override::ExecutionState::Running));

        // Check event history
        let history = stream.get_event_history().await;
        assert_eq!(history.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_override_event_stream_stop() {
        let stream = MockOverrideEventStream::new();

        // Send stop command
        stream.send_command(OverrideCommand::Stop {
            reason: OverrideReason::Manual,
            requested_by: "test".to_string(),
        }).await;

        // Check state
        let state = stream.get_state().await;
        assert!(matches!(state, crate::autonomy::override::ExecutionState::Stopped));

        // Check event history
        let history = stream.get_event_history().await;
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn test_mock_override_event_stream_subscribe() {
        let stream = MockOverrideEventStream::new();
        let mut event_rx = stream.subscribe_events().await;

        // Send command
        stream.send_command(OverrideCommand::Pause {
            reason: OverrideReason::Manual,
            requested_by: "test".to_string(),
        }).await;

        // Receive event
        let event = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            event_rx.recv()
        ).await;

        assert!(event.is_ok());
        let event = event.unwrap().unwrap();

        assert!(matches!(event.command, OverrideCommand::Pause { .. }));
    }
}
```

- [ ] **Step 3: Run override event stream mock tests**

Run: `cargo test test_mock_override_event_stream --test override_event_stream_mock`
Expected: PASS

- [ ] **Step 4: Commit Mock5**

```bash
git add tests/mocks/override_event_stream_mock.rs
git commit -m "test(Mock5): add mock override event stream strategy"
```

---

## Mock Integration Example

**Files:**
- Create: `tests/mocks/mock_integration_example.rs`

**Purpose:** Example showing how to use all mocks together in integration tests.

- [ ] **Step 1: Write mock integration example**

```rust
use super::*;
use yaml_to_rust_agentsdk::autonomy::contract::AutonomyContract;
use yaml_to_rust_agentsdk::autonomy::executor::AutonomousLoop;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_all_mocks_integration() {
        // Setup mocks
        let workflow = MockAutonomousWorkflow::new(vec![5, 10, 15]);
        let metrics = MockMetricsCollector::new();
        let risk_assessor = MockRiskAssessor::new();
        let checkpoint_store = MockCheckpointStore::new();
        let override_stream = MockOverrideEventStream::new();

        // Execute workflow
        workflow.start().await.unwrap();

        for iteration in 1..=20 {
            // Execute iteration
            let result = workflow.execute_iteration().await;

            // Collect metrics
            metrics.increment_counter("iterations", 1, &[]).await;
            metrics.observe_histogram("iteration_duration_ms", 100.0, &[]).await;

            // Check for pause
            if result.reason.contains("Paused") {
                // Send override command
                override_stream.send_command(OverrideCommand::Resume).await;

                // Wait for resume
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

                // Resume execution
                let resume_result = workflow.execute_iteration().await;
                assert!(!resume_result.reason.contains("Paused"));
            }

            // Create checkpoint every 5 iterations
            if iteration % 5 == 0 {
                let state = workflow.get_state();
                checkpoint_store.generate_checkpoint(
                    "test_task".to_string(),
                    state,
                    crate::autonomy::checkpoint::CheckpointMetadata {
                        reason: format!("Checkpoint at iteration {}", iteration),
                        checkpoint_type: crate::autonomy::checkpoint::CheckpointType::Periodic,
                        iteration,
                        tags: vec![],
                    },
                ).await;
            }

            if result.should_terminate {
                break;
            }
        }

        // Verify metrics collected
        let total_iterations = metrics.get_counter_value("iterations").await;
        assert!(total_iterations >= 20);

        // Verify checkpoints created
        let checkpoints = checkpoint_store.list_checkpoints("test_task").await;
        assert!(checkpoints.len() >= 4); // At iterations 5, 10, 15, 20

        // Verify override events captured
        let event_history = override_stream.get_event_history().await;
        assert!(event_history.len() >= 1); // At least one resume event
    }
}
```

- [ ] **Step 2: Run mock integration example**

Run: `cargo test test_all_mocks_integration --test mock_integration_example`
Expected: PASS

- [ ] **Step 3: Commit Mock Integration Example**

```bash
git add tests/mocks/mock_integration_example.rs
git commit -m "test(MockIntegration): add mock integration example"
```

---

## Summary

All 5 mock strategies have been defined with comprehensive functionality:

1. **Mock1**: Autonomous Workflow Mock - 4 steps
   - Pauses at predetermined points
   - Configurable pause iterations
   - State management
   - Stop and resume functionality

2. **Mock2**: Metrics Collector Mock - 4 steps
   - Generates synthetic metrics
   - Customizable metrics config
   - Counter, gauge, histogram support
   - Histogram summary calculation

3. **Mock3**: Risk Assessor Mock - 4 steps
   - Configurable risk levels
   - Probability, impact, severity computation
   - Risk score calculation
   - Autonomy level adjustment

4. **Mock4**: Checkpoint Store Mock - 4 steps
   - Injectable state
   - Checkpoint generation and loading
   - Listing by task ID
   - Delete functionality

5. **Mock5**: Override Event Stream Mock - 4 steps
   - Command handling
   - State transitions
   - Event emission
   - Event history tracking

Each mock strategy includes:
- Trait definitions
- Implementation with mock behavior
- Configuration options
- Comprehensive test coverage
- Integration examples showing usage patterns
