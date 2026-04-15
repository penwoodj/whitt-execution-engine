# Task 11: Metrics Collection

**Goal:** Implement metrics collection for pipeline/step/model/tool/custom metrics with JSON serialization.

**Files:**
- Create: `src/observability/metrics.rs`
- Create: `tests/unit/metrics_test.rs`

---

## Rust Definitions

### `src/observability/metrics.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Summary {
        count: u64,
        sum: f64,
        min: f64,
        max: f64,
        avg: f64,
    },
}

impl MetricValue {
    pub fn as_counter(&self) -> Option<u64> {
        match self {
            MetricValue::Counter(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_gauge(&self) -> Option<f64> {
        match self {
            MetricValue::Gauge(v) => Some(*v),
            _ => None,
        }
    }
}

/// Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: MetricValue,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
}

impl Metric {
    pub fn new(name: String, metric_type: MetricType, value: MetricValue) -> Self {
        Self {
            name,
            metric_type,
            value,
            timestamp: Utc::now(),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: String, value: String) -> Self {
        self.labels.insert(key, value);
        self
    }

    pub fn counter(name: String, value: u64) -> Self {
        Self::new(name, MetricType::Counter, MetricValue::Counter(value))
    }

    pub fn gauge(name: String, value: f64) -> Self {
        Self::new(name, MetricType::Gauge, MetricValue::Gauge(value))
    }

    pub fn histogram(name: String, values: Vec<f64>) -> Self {
        Self::new(name, MetricType::Histogram, MetricValue::Histogram(values))
    }
}

/// Pipeline-level metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetrics {
    pub pipeline_id: String,
    pub workflow_id: String,
    pub metrics: Vec<Metric>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl PipelineMetrics {
    pub fn new(pipeline_id: String, workflow_id: String) -> Self {
        Self {
            pipeline_id,
            workflow_id,
            metrics: Vec::new(),
            start_time: Utc::now(),
            end_time: None,
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn complete(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn duration_secs(&self) -> Option<f64> {
        match self.end_time {
            Some(end) => Some((end - self.start_time).num_seconds() as f64),
            None => None,
        }
    }
}

/// Step-level metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepMetrics {
    pub step_id: String,
    pub pipeline_id: String,
    pub metrics: Vec<Metric>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub success: bool,
}

impl StepMetrics {
    pub fn new(step_id: String, pipeline_id: String) -> Self {
        Self {
            step_id,
            pipeline_id,
            metrics: Vec::new(),
            start_time: Utc::now(),
            end_time: None,
            success: false,
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn complete(&mut self, success: bool) {
        self.end_time = Some(Utc::now());
        self.success = success;
    }

    pub fn duration_secs(&self) -> Option<f64> {
        match self.end_time {
            Some(end) => Some((end - self.start_time).num_seconds() as f64),
            None => None,
        }
    }
}

/// Model usage metrics (placeholders for Phase 2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub model_name: String,
    pub pipeline_id: String,
    pub metrics: Vec<Metric>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl ModelMetrics {
    pub fn new(model_name: String, pipeline_id: String) -> Self {
        Self {
            model_name,
            pipeline_id,
            metrics: Vec::new(),
            start_time: Utc::now(),
            end_time: None,
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn complete(&mut self) {
        self.end_time = Some(Utc::now());
    }
}

/// Tool execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetrics {
    pub tool_name: String,
    pub pipeline_id: String,
    pub step_id: String,
    pub metrics: Vec<Metric>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub success: bool,
}

impl ToolMetrics {
    pub fn new(tool_name: String, pipeline_id: String, step_id: String) -> Self {
        Self {
            tool_name,
            pipeline_id,
            step_id,
            metrics: Vec::new(),
            start_time: Utc::now(),
            end_time: None,
            success: false,
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn complete(&mut self, success: bool) {
        self.end_time = Some(Utc::now());
        self.success = success;
    }

    pub fn duration_secs(&self) -> Option<f64> {
        match self.end_time {
            Some(end) => Some((end - self.start_time).num_seconds() as f64),
            None => None,
        }
    }
}

/// Custom user-defined metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetrics {
    pub name: String,
    pub metrics: Vec<Metric>,
    pub start_time: DateTime<Utc>,
}

impl CustomMetrics {
    pub fn new(name: String) -> Self {
        Self {
            name,
            metrics: Vec::new(),
            start_time: Utc::now(),
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }
}

/// Metrics collector
pub struct MetricsCollector {
    pipeline_metrics: HashMap<String, PipelineMetrics>,
    step_metrics: HashMap<String, StepMetrics>,
    tool_metrics: HashMap<String, ToolMetrics>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            pipeline_metrics: HashMap::new(),
            step_metrics: HashMap::new(),
            tool_metrics: HashMap::new(),
        }
    }

    pub fn start_pipeline(&mut self, pipeline_id: String, workflow_id: String) {
        let metrics = PipelineMetrics::new(pipeline_id.clone(), workflow_id);
        self.pipeline_metrics.insert(pipeline_id, metrics);
    }

    pub fn complete_pipeline(&mut self, pipeline_id: &str) {
        if let Some(metrics) = self.pipeline_metrics.get_mut(pipeline_id) {
            metrics.complete();
        }
    }

    pub fn start_step(&mut self, step_id: String, pipeline_id: String) {
        let metrics = StepMetrics::new(step_id.clone(), pipeline_id);
        self.step_metrics.insert(step_id, metrics);
    }

    pub fn complete_step(&mut self, step_id: &str, success: bool) {
        if let Some(metrics) = self.step_metrics.get_mut(step_id) {
            metrics.complete(success);
        }
    }

    pub fn start_tool(&mut self, tool_name: String, pipeline_id: String, step_id: String) {
        let key = format!("{}:{}", pipeline_id, step_id);
        let metrics = ToolMetrics::new(tool_name, pipeline_id, step_id);
        self.tool_metrics.insert(key, metrics);
    }

    pub fn complete_tool(&mut self, pipeline_id: &str, step_id: &str, success: bool) {
        let key = format!("{}:{}", pipeline_id, step_id);
        if let Some(metrics) = self.tool_metrics.get_mut(&key) {
            metrics.complete(success);
        }
    }

    pub fn get_pipeline_metrics(&self, pipeline_id: &str) -> Option<&PipelineMetrics> {
        self.pipeline_metrics.get(pipeline_id)
    }

    pub fn get_step_metrics(&self, step_id: &str) -> Option<&StepMetrics> {
        self.step_metrics.get(step_id)
    }

    pub fn get_tool_metrics(&self, pipeline_id: &str, step_id: &str) -> Option<&ToolMetrics> {
        let key = format!("{}:{}", pipeline_id, step_id);
        self.tool_metrics.get(&key)
    }

    pub fn export_all(&self) -> serde_json::Value {
        serde_json::json!({
            "pipelines": self.pipeline_metrics,
            "steps": self.step_metrics,
            "tools": self.tool_metrics,
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_counter() {
        let metric = Metric::counter("test_counter".to_string(), 42)
            .with_label("label1".to_string(), "value1".to_string());

        assert_eq!(metric.name, "test_counter");
        assert_eq!(metric.as_counter(), Some(42));
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();

        collector.start_pipeline("pipeline-1".to_string(), "workflow-1".to_string());
        collector.complete_pipeline("pipeline-1");

        let metrics = collector.get_pipeline_metrics("pipeline-1");
        assert!(metrics.is_some());
        assert!(metrics.unwrap().duration_secs().is_some());
    }
}
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement metrics.rs**

- [ ] **Step 3: Add to observability/mod.rs**

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**
