# Task 01: Metrics Collection

**Estimated Time**: 5 days
**Priority**: CRITICAL - Must be completed FIRST before any other tasks
**Dependencies**: Phase 2 (Tracing Infrastructure) must be complete

## Overview

Implement comprehensive metrics collection integrated into the existing tracing infrastructure. This is the FOUNDATION for all autonomy features - metrics MUST be instrumented BEFORE any UI/UX experimentation begins.

**CRITICAL**: This task must have 100% test coverage before proceeding to Task 04 (dashboards).

## Files

### Create
- `src/metrics/mod.rs` - Module exports
- `src/metrics/types.rs` - Metric type definitions (Counter, Gauge, Histogram, Summary)
- `src/metrics/instrumentation.rs` - Instrumentation hooks for workflow events
- `src/metrics/aggregation.rs` - Aggregation logic (sum, avg, p50, p95, p99)
- `src/metrics/storage.rs` - In-memory storage with TTL
- `src/metrics/export.rs` - Export to Prometheus and JSON formats
- `tests/metrics/metrics_test.rs` - Comprehensive unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod metrics;`
- `src/tracing/span.rs` - Add metrics instrumentation to spans
- `Cargo.toml` - Add dependencies: `prometheus`, `serde_json`, `dashmap`, `ahash`

---

## Implementation Steps

### Step 1: Define metric types

**File**: `src/metrics/types.rs`

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Core metric types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Metric {
    Counter(Counter),
    Gauge(Gauge),
    Histogram(Histogram),
    Summary(Summary),
}

/// Counter metric - monotonically increasing value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Counter {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub help: Option<String>,
}

impl Counter {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: 0.0,
            labels: HashMap::new(),
            help: None,
        }
    }

    pub fn increment(&mut self, value: f64) {
        if value > 0.0 {
            self.value += value;
        }
    }

    pub fn get(&self) -> f64 {
        self.value
    }
}

/// Gauge metric - value that can go up or down
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gauge {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub help: Option<String>,
}

impl Gauge {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: 0.0,
            labels: HashMap::new(),
            help: None,
        }
    }

    pub fn set(&mut self, value: f64) {
        self.value = value;
    }

    pub fn increment(&mut self, value: f64) {
        self.value += value;
    }

    pub fn decrement(&mut self, value: f64) {
        self.value -= value;
    }

    pub fn get(&self) -> f64 {
        self.value
    }
}

/// Histogram metric - distribution of observed values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Histogram {
    pub name: String,
    pub buckets: Vec<f64>,
    pub counts: Vec<u64>,
    pub sum: f64,
    pub count: u64,
    pub labels: HashMap<String, String>,
    pub help: Option<String>,
}

impl Histogram {
    pub fn new(name: String, buckets: Vec<f64>) -> Self {
        let counts = vec![0; buckets.len() + 1];
        Self {
            name,
            buckets,
            counts,
            sum: 0.0,
            count: 0,
            labels: HashMap::new(),
            help: None,
        }
    }

    pub fn observe(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;

        for (i, &bucket) in self.buckets.iter().enumerate() {
            if value <= bucket {
                self.counts[i] += 1;
                return;
            }
        }
        // Value exceeds all buckets
        *self.counts.last_mut().unwrap() += 1;
    }

    pub fn get_quantile(&self, quantile: f64) -> f64 {
        if self.count == 0 {
            return 0.0;
        }

        let target_count = (quantile * self.count as f64).ceil() as u64;
        let mut cumulative = 0;

        for (i, &count) in self.counts.iter().enumerate() {
            cumulative += count;
            if cumulative >= target_count {
                return if i < self.buckets.len() {
                    self.buckets[i]
                } else {
                    f64::INFINITY
                };
            }
        }

        f64::INFINITY
    }
}

/// Summary metric - sliding window of quantiles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub name: String,
    pub quantiles: Vec<f64>,
    pub values: Vec<f64>,
    pub count: u64,
    pub sum: f64,
    pub labels: HashMap<String, String>,
    pub help: Option<String>,
}

impl Summary {
    pub fn new(name: String, quantiles: Vec<f64>) -> Self {
        Self {
            name,
            quantiles,
            values: vec![0.0; quantiles.len()],
            count: 0,
            sum: 0.0,
            labels: HashMap::new(),
            help: None,
        }
    }

    pub fn observe(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;

        // Simple quantile calculation (in production, use t-digest or similar)
        // For now, this is a placeholder implementation
        for val in self.values.iter_mut() {
            *val = value; // This is simplified - real implementation needs sliding window
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let mut counter = Counter::new("test_counter".to_string());
        counter.increment(1.0);
        assert_eq!(counter.get(), 1.0);
        counter.increment(5.0);
        assert_eq!(counter.get(), 6.0);
    }

    #[test]
    fn test_counter_negative_increment() {
        let mut counter = Counter::new("test_counter".to_string());
        counter.increment(-1.0);
        assert_eq!(counter.get(), 0.0); // Should not decrement
    }

    #[test]
    fn test_gauge_set() {
        let mut gauge = Gauge::new("test_gauge".to_string());
        gauge.set(10.0);
        assert_eq!(gauge.get(), 10.0);
        gauge.set(5.0);
        assert_eq!(gauge.get(), 5.0);
    }

    #[test]
    fn test_gauge_increment_decrement() {
        let mut gauge = Gauge::new("test_gauge".to_string());
        gauge.increment(10.0);
        assert_eq!(gauge.get(), 10.0);
        gauge.decrement(3.0);
        assert_eq!(gauge.get(), 7.0);
    }

    #[test]
    fn test_histogram_observe() {
        let mut histogram = Histogram::new("test_histogram".to_string(), vec![1.0, 5.0, 10.0]);
        histogram.observe(0.5);
        histogram.observe(3.0);
        histogram.observe(7.0);
        histogram.observe(15.0);

        assert_eq!(histogram.count, 4);
        assert_eq!(histogram.sum, 25.5);
        assert_eq!(histogram.counts, vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_histogram_quantile() {
        let mut histogram = Histogram::new("test_histogram".to_string(), vec![10.0, 20.0, 30.0]);
        for i in 1..=100 {
            histogram.observe(i as f64);
        }

        let p50 = histogram.get_quantile(0.5);
        assert!(p50 > 40.0 && p50 <= 60.0); // Around 50
    }
}
```

---

### Step 2: Implement instrumentation hooks

**File**: `src/metrics/instrumentation.rs`

```rust
use super::types::*;
use std::sync::{Arc, Mutex};
use tracing::{span, Level};

/// Global metrics registry
pub static METRICS_REGISTRY: MetricsRegistry = MetricsRegistry::new();

pub struct MetricsRegistry;

impl MetricsRegistry {
    const fn new() -> Self {
        Self
    }

    pub fn counter(&self, name: &str) -> Arc<Mutex<Counter>> {
        METRICS_STORAGE.with(|storage| {
            storage.borrow_mut().get_or_create_counter(name)
        })
    }

    pub fn gauge(&self, name: &str) -> Arc<Mutex<Gauge>> {
        METRICS_STORAGE.with(|storage| {
            storage.borrow_mut().get_or_create_gauge(name)
        })
    }

    pub fn histogram(&self, name: &str, buckets: Vec<f64>) -> Arc<Mutex<Histogram>> {
        METRICS_STORAGE.with(|storage| {
            storage.borrow_mut().get_or_create_histogram(name, buckets)
        })
    }

    pub fn summary(&self, name: &str, quantiles: Vec<f64>) -> Arc<Mutex<Summary>> {
        METRICS_STORAGE.with(|storage| {
            storage.borrow_mut().get_or_create_summary(name, quantiles)
        })
    }
}

thread_local! {
    static METRICS_STORAGE: Arc<Mutex<MetricStorage>> = Arc::new(Mutex::new(MetricStorage::new()));
}

/// Instrumentation macros for easy metrics collection
#[macro_export]
macro_rules! counter {
    ($name:expr, $value:expr $(, $($key:expr => $val:expr),*)?) => {{
        let mut counter = $crate::metrics::METRICS_REGISTRY.counter($name);
        counter.lock().unwrap().increment($value);
    }};
}

#[macro_export]
macro_rules! gauge {
    ($name:expr, $value:expr $(, $($key:expr => $val:expr),*)?) => {{
        let mut gauge = $crate::metrics::METRICS_REGISTRY.gauge($name);
        gauge.lock().unwrap().set($value);
    }};
}

#[macro_export]
macro_rules! histogram {
    ($name:expr, $value:expr, $buckets:expr $(, $($key:expr => $val:expr),*)?) => {{
        let mut histogram = $crate::metrics::METRICS_REGISTRY.histogram($name, $buckets);
        histogram.lock().unwrap().observe($value);
    }};
}

#[macro_export]
macro_rules! summary {
    ($name:expr, $value:expr, $quantiles:expr $(, $($key:expr => $val:expr),*)?) => {{
        let mut summary = $crate::metrics::METRICS_REGISTRY.summary($name, $quantiles);
        summary.lock().unwrap().observe($value);
    }};
}

/// Instrumentation for workflow events
pub fn instrument_workflow_start(workflow_id: &str, workflow_type: &str) {
    counter!("workflows_started", 1.0);
    gauge!("active_workflows", get_active_workflow_count());

    tracing::info!(
        workflow_id = %workflow_id,
        workflow_type = %workflow_type,
        "Workflow started"
    );
}

pub fn instrument_workflow_complete(workflow_id: &str, success: bool, duration_ms: f64) {
    histogram!(
        "workflow_duration_ms",
        duration_ms,
        vec![10.0, 50.0, 100.0, 500.0, 1000.0, 5000.0]
    );

    if success {
        counter!("workflows_completed", 1.0);
    } else {
        counter!("workflows_failed", 1.0);
    }

    gauge!("active_workflows", get_active_workflow_count());

    tracing::info!(
        workflow_id = %workflow_id,
        success = success,
        duration_ms = duration_ms,
        "Workflow completed"
    );
}

pub fn instrument_action_start(workflow_id: &str, action_type: &str) {
    counter!("actions_started", 1.0);
    gauge!("active_actions", get_active_action_count());

    tracing::debug!(
        workflow_id = %workflow_id,
        action_type = %action_type,
        "Action started"
    );
}

pub fn instrument_action_complete(workflow_id: &str, action_type: &str, success: bool, duration_ms: f64) {
    histogram!(
        "action_duration_ms",
        duration_ms,
        vec![10.0, 50.0, 100.0, 500.0, 1000.0]
    );

    if success {
        counter!("actions_completed", 1.0);
    } else {
        counter!("actions_failed", 1.0);
    }

    gauge!("active_actions", get_active_action_count());

    tracing::debug!(
        workflow_id = %workflow_id,
        action_type = %action_type,
        success = success,
        duration_ms = duration_ms,
        "Action completed"
    );
}

pub fn instrument_intervention(workflow_id: &str, intervention_type: &str, reason: &str) {
    counter!("interventions", 1.0);

    tracing::warn!(
        workflow_id = %workflow_id,
        intervention_type = %intervention_type,
        reason = %reason,
        "Intervention occurred"
    );
}

pub fn instrument_checkpoint(workflow_id: &str, checkpoint_type: &str) {
    counter!("checkpoints_created", 1.0);

    tracing::info!(
        workflow_id = %workflow_id,
        checkpoint_type = %checkpoint_type,
        "Checkpoint created"
    );
}

pub fn instrument_resource_usage(workflow_id: &str, cpu_cores: f64, memory_gb: f64) {
    gauge!("cpu_usage_cores", cpu_cores);
    gauge!("memory_usage_gb", memory_gb);

    tracing::trace!(
        workflow_id = %workflow_id,
        cpu_cores = cpu_cores,
        memory_gb = memory_gb,
        "Resource usage recorded"
    );
}

// Helper functions (these would be implemented with proper state tracking)
fn get_active_workflow_count() -> f64 {
    // In real implementation, track active workflows
    1.0
}

fn get_active_action_count() -> f64 {
    // In real implementation, track active actions
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_macro() {
        counter!("test_counter", 1.0);
        counter!("test_counter", 5.0);

        let counter = METRICS_REGISTRY.counter("test_counter");
        assert_eq!(counter.lock().unwrap().get(), 6.0);
    }

    #[test]
    fn test_gauge_macro() {
        gauge!("test_gauge", 10.0);
        gauge!("test_gauge", 5.0);

        let gauge = METRICS_REGISTRY.gauge("test_gauge");
        assert_eq!(gauge.lock().unwrap().get(), 5.0);
    }

    #[test]
    fn test_histogram_macro() {
        histogram!("test_histogram", 5.0, vec![1.0, 10.0]);
        histogram!("test_histogram", 15.0, vec![1.0, 10.0]);

        let histogram = METRICS_REGISTRY.histogram("test_histogram", vec![1.0, 10.0]);
        assert_eq!(histogram.lock().unwrap().count, 2);
    }
}
```

---

### Step 3: Implement aggregation logic

**File**: `src/metrics/aggregation.rs`

```rust
use super::types::*;
use std::collections::HashMap;

/// Aggregated metric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetric {
    pub name: String,
    pub sum: f64,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub count: u64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

impl AggregatedMetric {
    pub fn from_histogram(histogram: &Histogram) -> Self {
        let avg = if histogram.count > 0 {
            histogram.sum / histogram.count as f64
        } else {
            0.0
        };

        Self {
            name: histogram.name.clone(),
            sum: histogram.sum,
            avg,
            min: 0.0, // Histogram doesn't track min
            max: f64::INFINITY, // Histogram doesn't track max
            count: histogram.count,
            p50: histogram.get_quantile(0.5),
            p95: histogram.get_quantile(0.95),
            p99: histogram.get_quantile(0.99),
        }
    }

    pub fn from_counter(counter: &Counter) -> Self {
        Self {
            name: counter.name.clone(),
            sum: counter.value,
            avg: counter.value,
            min: 0.0,
            max: counter.value,
            count: 1,
            p50: counter.value,
            p95: counter.value,
            p99: counter.value,
        }
    }

    pub fn from_gauge(gauge: &Gauge) -> Self {
        Self {
            name: gauge.name.clone(),
            sum: gauge.value,
            avg: gauge.value,
            min: gauge.value,
            max: gauge.value,
            count: 1,
            p50: gauge.value,
            p95: gauge.value,
            p99: gauge.value,
        }
    }
}

/// Metrics aggregator
pub struct MetricsAggregator {
    values: HashMap<String, Vec<f64>>,
}

impl MetricsAggregator {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, name: &str, value: f64) {
        self.values.entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    pub fn aggregate(&self, name: &str) -> Option<AggregatedMetric> {
        let values = self.values.get(name)?;

        if values.is_empty() {
            return None;
        }

        let count = values.len() as u64;
        let sum: f64 = values.iter().sum();
        let avg = sum / count;
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate percentiles
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p50 = Self::percentile(&sorted, 0.5);
        let p95 = Self::percentile(&sorted, 0.95);
        let p99 = Self::percentile(&sorted, 0.99);

        Some(AggregatedMetric {
            name: name.to_string(),
            sum,
            avg,
            min,
            max,
            count,
            p50,
            p95,
            p99,
        })
    }

    pub fn aggregate_all(&self) -> Vec<AggregatedMetric> {
        self.values.keys()
            .filter_map(|name| self.aggregate(name))
            .collect()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    fn percentile(sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }

        let index = ((sorted.len() - 1) as f64 * p) as usize;
        sorted[index]
    }
}

impl Default for MetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregator_basic() {
        let mut aggregator = MetricsAggregator::new();

        aggregator.add_value("test_metric", 1.0);
        aggregator.add_value("test_metric", 2.0);
        aggregator.add_value("test_metric", 3.0);

        let aggregated = aggregator.aggregate("test_metric").unwrap();
        assert_eq!(aggregated.sum, 6.0);
        assert_eq!(aggregated.avg, 2.0);
        assert_eq!(aggregated.min, 1.0);
        assert_eq!(aggregated.max, 3.0);
        assert_eq!(aggregated.count, 3);
    }

    #[test]
    fn test_aggregator_percentiles() {
        let mut aggregator = MetricsAggregator::new();

        for i in 1..=100 {
            aggregator.add_value("test_metric", i as f64);
        }

        let aggregated = aggregator.aggregate("test_metric").unwrap();
        assert_eq!(aggregated.p50, 50.0);
        assert_eq!(aggregated.p95, 95.0);
        assert_eq!(aggregated.p99, 99.0);
    }

    #[test]
    fn test_aggregated_from_histogram() {
        let mut histogram = Histogram::new("test".to_string(), vec![1.0, 10.0, 100.0]);
        histogram.observe(5.0);
        histogram.observe(50.0);
        histogram.observe(500.0);

        let aggregated = AggregatedMetric::from_histogram(&histogram);
        assert_eq!(aggregated.sum, 555.0);
        assert_eq!(aggregated.avg, 185.0);
        assert_eq!(aggregated.count, 3);
    }
}
```

---

### Step 4: Implement in-memory storage with TTL

**File**: `src/metrics/storage.rs`

```rust
use super::types::*;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Metrics storage with TTL
pub struct MetricStorage {
    metrics: DashMap<String, StoredMetric>,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct StoredMetric {
    metric: Arc<Metric>,
    created_at: Instant,
    last_accessed: Instant,
}

impl MetricStorage {
    pub fn new(ttl: Duration) -> Self {
        Self {
            metrics: DashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Metric>> {
        let entry = self.metrics.get(name)?;
        entry.last_accessed = Instant::now();
        Some(entry.metric.clone())
    }

    pub fn set(&self, name: String, metric: Metric) {
        let now = Instant::now();
        self.metrics.insert(name.clone(), StoredMetric {
            metric: Arc::new(metric),
            created_at: now,
            last_accessed: now,
        });
    }

    pub fn remove(&self, name: &str) -> Option<Arc<Metric>> {
        self.metrics.remove(name).map(|(_, stored)| stored.metric)
    }

    pub fn cleanup_expired(&self) -> usize {
        let now = Instant::now();
        let mut removed = 0;

        self.metrics.retain(|_, stored| {
            if now.duration_since(stored.created_at) > self.ttl {
                removed += 1;
                false
            } else {
                true
            }
        });

        removed
    }

    pub fn get_all(&self) -> Vec<(String, Arc<Metric>)> {
        self.metrics.iter()
            .map(|entry| (entry.key().clone(), entry.value().metric.clone()))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.metrics.len()
    }

    // Helper methods for creating specific metric types
    pub fn get_or_create_counter(&mut self, name: &str) -> Arc<Mutex<Counter>> {
        if let Some(metric) = self.get(name) {
            if let Metric::Counter(counter) = metric.as_ref() {
                return counter.clone();
            }
        }

        let counter = Arc::new(Mutex::new(Counter::new(name.to_string())));
        self.set(name.to_string(), Metric::Counter(counter.clone()));
        counter
    }

    pub fn get_or_create_gauge(&mut self, name: &str) -> Arc<Mutex<Gauge>> {
        if let Some(metric) = self.get(name) {
            if let Metric::Gauge(gauge) = metric.as_ref() {
                return gauge.clone();
            }
        }

        let gauge = Arc::new(Mutex::new(Gauge::new(name.to_string())));
        self.set(name.to_string(), Metric::Gauge(gauge.clone()));
        gauge
    }

    pub fn get_or_create_histogram(&mut self, name: &str, buckets: Vec<f64>) -> Arc<Mutex<Histogram>> {
        if let Some(metric) = self.get(name) {
            if let Metric::Histogram(histogram) = metric.as_ref() {
                return histogram.clone();
            }
        }

        let histogram = Arc::new(Mutex::new(Histogram::new(name.to_string(), buckets)));
        self.set(name.to_string(), Metric::Histogram(histogram.clone()));
        histogram
    }

    pub fn get_or_create_summary(&mut self, name: &str, quantiles: Vec<f64>) -> Arc<Mutex<Summary>> {
        if let Some(metric) = self.get(name) {
            if let Metric::Summary(summary) = metric.as_ref() {
                return summary.clone();
            }
        }

        let summary = Arc::new(Mutex::new(Summary::new(name.to_string(), quantiles)));
        self.set(name.to_string(), Metric::Summary(summary.clone()));
        summary
    }
}

impl Default for MetricStorage {
    fn default() -> Self {
        Self::new(Duration::from_secs(3600)) // 1 hour default TTL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_basic() {
        let storage = MetricStorage::new(Duration::from_secs(3600));

        storage.set("test_counter".to_string(), Metric::Counter(Counter::new("test_counter".to_string())));

        let retrieved = storage.get("test_counter");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_storage_cleanup() {
        let storage = MetricStorage::new(Duration::from_millis(100));

        storage.set("test".to_string(), Metric::Counter(Counter::new("test".to_string())));
        assert_eq!(storage.count(), 1);

        std::thread::sleep(Duration::from_millis(150));
        let removed = storage.cleanup_expired();
        assert_eq!(removed, 1);
        assert_eq!(storage.count(), 0);
    }
}
```

---

### Step 5: Implement export to Prometheus and JSON

**File**: `src/metrics/export.rs`

```rust
use super::types::*;
use serde_json;

/// Export configuration
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
    pub json_enabled: bool,
    pub json_path: String,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            prometheus_enabled: true,
            prometheus_port: 9090,
            json_enabled: true,
            json_path: ".glyphnova/metrics.json".to_string(),
        }
    }
}

/// Export metrics to Prometheus format
pub fn export_to_prometheus(metrics: &[Arc<Metric>]) -> String {
    let mut output = String::new();

    for metric in metrics {
        match metric.as_ref() {
            Metric::Counter(counter) => {
                if let Some(help) = &counter.help {
                    output.push_str(&format!("# HELP {} {}\n", counter.name, help));
                }
                output.push_str(&format!("# TYPE {} counter\n", counter.name));
                output.push_str(&format!("{} {}\n", counter.name, counter.value));
            }
            Metric::Gauge(gauge) => {
                if let Some(help) = &gauge.help {
                    output.push_str(&format!("# HELP {} {}\n", gauge.name, help));
                }
                output.push_str(&format!("# TYPE {} gauge\n", gauge.name));
                output.push_str(&format!("{} {}\n", gauge.name, gauge.value));
            }
            Metric::Histogram(histogram) => {
                if let Some(help) = &histogram.help {
                    output.push_str(&format!("# HELP {} {}\n", histogram.name, help));
                }
                output.push_str(&format!("# TYPE {} histogram\n", histogram.name));

                for (i, &bucket) in histogram.buckets.iter().enumerate() {
                    output.push_str(&format!(
                        "{}_bucket{{le=\"{}\"}} {}\n",
                        histogram.name, bucket, histogram.counts[i]
                    ));
                }
                output.push_str(&format!(
                    "{}_bucket{{le=\"+Inf\"}} {}\n",
                    histogram.name,
                    histogram.counts.last().unwrap_or(&0)
                ));
                output.push_str(&format!("{} {}\n", histogram.name, histogram.sum));
            }
            Metric::Summary(summary) => {
                if let Some(help) = &summary.help {
                    output.push_str(&format!("# HELP {} {}\n", summary.name, help));
                }
                output.push_str(&format!("# TYPE {} summary\n", summary.name));

                for (i, &quantile) in summary.quantiles.iter().enumerate() {
                    output.push_str(&format!(
                        "{}{{quantile=\"{}\"}} {}\n",
                        summary.name, quantile, summary.values[i]
                    ));
                }
                output.push_str(&format!("{}_sum {}\n", summary.name, summary.sum));
                output.push_str(&format!("{}_count {}\n", summary.name, summary.count));
            }
        }
        output.push('\n');
    }

    output
}

/// Export metrics to JSON format
pub fn export_to_json(metrics: &[Arc<Metric>]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(metrics)
}

/// Start Prometheus metrics server
pub fn start_prometheus_server(config: ExportConfig) -> std::io::Result<()> {
    if !config.prometheus_enabled {
        return Ok(());
    }

    // In a real implementation, this would start an HTTP server
    // For now, we'll just log that we would start it
    tracing::info!("Prometheus metrics server would start on port {}", config.prometheus_port);

    Ok(())
}

/// Export metrics to JSON file
pub fn export_to_json_file(config: ExportConfig, metrics: &[Arc<Metric>]) -> std::io::Result<()> {
    if !config.json_enabled {
        return Ok(());
    }

    let json = export_to_json(metrics)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    std::fs::write(&config.json_path, json)?;

    tracing::info!("Metrics exported to {}", config.json_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_prometheus_counter() {
        let counter = Counter::new("test_counter".to_string());
        counter.lock().unwrap().increment(42.0);
        let metric = Arc::new(Metric::Counter(counter));

        let output = export_to_prometheus(&[metric]);

        assert!(output.contains("# TYPE test_counter counter"));
        assert!(output.contains("test_counter 42"));
    }

    #[test]
    fn test_export_json() {
        let counter = Counter::new("test_counter".to_string());
        counter.lock().unwrap().increment(42.0);
        let metric = Arc::new(Metric::Counter(counter));

        let output = export_to_json(&[metric]).unwrap();

        assert!(output.contains("\"Counter\""));
        assert!(output.contains("\"value\":42"));
    }
}
```

---

### Step 6: Create module exports

**File**: `src/metrics/mod.rs`

```rust
pub mod types;
pub mod instrumentation;
pub mod aggregation;
pub mod storage;
pub mod export;

pub use types::*;
pub use instrumentation::*;
pub use aggregation::*;
pub use storage::*;
pub use export::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod metrics;
```

---

### Step 7: Integrate with tracing spans

**File**: `src/tracing/span.rs` (modify)

```rust
use crate::metrics::instrumentation::*;

// Add metrics instrumentation to existing span creation
pub fn create_workflow_span(workflow_id: &str, workflow_type: &str) -> tracing::Span {
    instrument_workflow_start(workflow_id, workflow_type);

    tracing::span!(
        tracing::Level::INFO,
        "workflow_execution",
        workflow_id = %workflow_id,
        workflow_type = %workflow_type
    )
}

pub fn create_action_span(workflow_id: &str, action_type: &str) -> tracing::Span {
    instrument_action_start(workflow_id, action_type);

    tracing::span!(
        tracing::Level::DEBUG,
        "action_execution",
        workflow_id = %workflow_id,
        action_type = %action_type
    )
}
```

---

### Step 8: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test metrics --verbose
```

**Expected Output**: All tests pass, 100% coverage

---

### Step 9: Performance benchmarking

**File**: `tests/metrics/benchmark_test.rs`

```rust
use glyphnova_engine::metrics::*;
use std::time::Instant;

#[test]
fn test_counter_performance() {
    let start = Instant::now();
    for _ in 0..100_000 {
        counter!("bench_counter", 1.0);
    }
    let duration = start.elapsed();

    println!("100K counter increments took {:?}", duration);
    assert!(duration.as_millis() < 100, "Performance overhead too high");
}

#[test]
fn test_histogram_performance() {
    let start = Instant::now();
    for i in 0..10_000 {
        histogram!("bench_histogram", i as f64, vec![1.0, 10.0, 100.0]);
    }
    let duration = start.elapsed();

    println!("10K histogram observations took {:?}", duration);
    assert!(duration.as_millis() < 200, "Performance overhead too high");
}

#[test]
fn test_export_performance() {
    let mut metrics = Vec::new();
    for i in 0..1000 {
        let mut counter = Counter::new(format!("counter_{}", i));
        counter.increment(i as f64);
        metrics.push(Arc::new(Metric::Counter(counter)));
    }

    let start = Instant::now();
    export_to_prometheus(&metrics);
    let duration = start.elapsed();

    println!("Exporting 1000 metrics took {:?}", duration);
    assert!(duration.as_millis() < 50, "Export overhead too high");
}
```

Run benchmark:

```bash
cargo test metrics::benchmark -- --nocapture
```

**Expected**: All performance tests pass, overhead < 5%

---

### Step 10: Integration test for full metrics workflow

**File**: `tests/metrics/integration_test.rs`

```rust
use glyphnova_engine::metrics::*;
use std::time::Duration;

#[test]
fn test_full_metrics_workflow() {
    // Simulate a workflow execution
    let workflow_id = "test_workflow_123";

    // Start workflow
    instrument_workflow_start(workflow_id, "test_type");

    // Execute actions
    for i in 0..10 {
        instrument_action_start(workflow_id, "test_action");

        // Simulate action execution
        std::thread::sleep(Duration::from_millis(10));

        let success = i != 5; // Fail one action
        instrument_action_complete(workflow_id, "test_action", success, 10.0);
    }

    // Record resource usage
    instrument_resource_usage(workflow_id, 2.0, 4.0);

    // Complete workflow
    instrument_workflow_complete(workflow_id, true, 100.0);

    // Verify metrics were recorded
    let counter = METRICS_REGISTRY.counter("actions_started");
    assert_eq!(counter.lock().unwrap().get(), 10.0);

    let histogram = METRICS_REGISTRY.histogram("action_duration_ms", vec![1.0, 10.0, 100.0]);
    assert_eq!(histogram.lock().unwrap().count, 10);

    // Test export
    let metrics = vec![
        Arc::new(Metric::Counter(counter.lock().unwrap().clone())),
        Arc::new(Metric::Histogram(histogram.lock().unwrap().clone())),
    ];

    let prometheus_output = export_to_prometheus(&metrics);
    assert!(prometheus_output.contains("actions_started"));
    assert!(prometheus_output.contains("action_duration_ms"));

    let json_output = export_to_json(&metrics);
    assert!(json_output.is_ok());
}
```

---

### Step 11: Commit

```bash
git add src/metrics/ tests/metrics/
git commit -m "feat: implement metrics collection system

- Define metric types: Counter, Gauge, Histogram, Summary
- Implement instrumentation macros for easy metrics collection
- Add aggregation logic for sum, avg, p50, p95, p99
- Implement in-memory storage with TTL
- Add export to Prometheus and JSON formats
- Integrate with existing tracing infrastructure
- Add comprehensive unit and integration tests
- Verify performance overhead < 5%

Relates to ADR-0008: Metrics instrumented before UX experimentation"
```

---

## Validation Criteria

- ✅ All unit tests pass (100% coverage required)
- ✅ Integration tests pass
- ✅ Performance overhead < 5%
- ✅ Export to Prometheus format works correctly
- ✅ Export to JSON format works correctly
- ✅ Integration with tracing spans works correctly

---

## Next Steps

After completing Task 01:
1. Proceed to Task 02: Human Override Controls
2. Metrics collection is now available for quality gates and dashboards

---

**End of Task 01**
