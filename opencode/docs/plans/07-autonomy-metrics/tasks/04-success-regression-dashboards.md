# Task 04: Success Regression Dashboards

**Estimated Time**: 5 days
**Priority**: HIGH - Critical for UX experimentation
**Dependencies**: Task 01 (metrics), Task 03 (intervention tracking)

## Overview

Build real-time dashboards for success metrics with regression detection. This enables UX experimentation by providing visibility into success metrics and alerting on regressions.

## Files

### Create
- `src/dashboard/mod.rs` - Module exports
- `src/dashboard/layout.rs` - Dashboard layout definitions
- `src/dashboard/success_metrics.rs` - Success metrics visualization
- `src/dashboard/regression.rs` - Regression detection algorithm
- `src/dashboard/streaming.rs` - Real-time streaming updates
- `src/dashboard/anomaly.rs` - Anomaly detection alerts
- `tests/dashboard/dashboard_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod dashboard;`

---

## Implementation Steps

### Step 1: Define dashboard layout

**File**: `src/dashboard/layout.rs`

```rust
use serde::{Deserialize, Serialize};

/// Dashboard layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub dashboard_id: String,
    pub title: String,
    pub widgets: Vec<Widget>,
    pub refresh_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    pub widget_id: String,
    pub widget_type: WidgetType,
    pub title: String,
    pub position: Position,
    pub size: Size,
    pub configuration: WidgetConfiguration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WidgetType {
    SuccessRate,
    InterventionRate,
    ActionDuration,
    ResourceUsage,
    AnomalyAlerts,
    TrendAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfiguration {
    pub time_window_hours: u64,
    pub threshold: Option<f64>,
    pub show_trend: bool,
    pub show_alerts: bool,
}

impl Default for WidgetConfiguration {
    fn default() -> Self {
        Self {
            time_window_hours: 24,
            threshold: None,
            show_trend: true,
            show_alerts: true,
        }
    }
}
```

---

### Step 2: Implement success metrics

**File**: `src/dashboard/success_metrics.rs`

```rust
use super::layout::*;
use chrono::{DateTime, Utc, Duration};

/// Success metric data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetricDataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub labels: std::collections::HashMap<String, String>,
}

/// Success metric series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetricSeries {
    pub metric_name: String,
    pub data_points: Vec<SuccessMetricDataPoint>,
}

/// Success metrics collector
pub struct SuccessMetricsCollector {
    metrics: std::collections::HashMap<String, SuccessMetricSeries>,
}

impl SuccessMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: std::collections::HashMap::new(),
        }
    }

    pub fn add_data_point(&mut self, name: &str, value: f64, labels: std::collections::HashMap<String, String>) {
        let data_point = SuccessMetricDataPoint {
            timestamp: Utc::now(),
            value,
            labels,
        };

        let series = self.metrics.entry(name.to_string())
            .or_insert_with(|| SuccessMetricSeries {
                metric_name: name.to_string(),
                data_points: Vec::new(),
            });

        series.data_points.push(data_point);

        // Keep only last 1000 points to prevent unbounded growth
        if series.data_points.len() > 1000 {
            series.data_points.remove(0);
        }
    }

    pub fn get_metric(&self, name: &str) -> Option<&SuccessMetricSeries> {
        self.metrics.get(name)
    }

    pub fn get_metrics_in_window(&self, name: &str, window_hours: u64) -> Vec<&SuccessMetricDataPoint> {
        if let Some(series) = self.metrics.get(name) {
            let cutoff = Utc::now() - Duration::hours(window_hours as i64);
            series.data_points.iter()
                .filter(|dp| dp.timestamp > cutoff)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn calculate_success_rate(&self, name: &str, window_hours: u64) -> Option<f64> {
        let data_points = self.get_metrics_in_window(name, window_hours);
        if data_points.is_empty() {
            return None;
        }

        let sum: f64 = data_points.iter().map(|dp| dp.value).sum();
        Some(sum / data_points.len() as f64)
    }
}

impl Default for SuccessMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 3: Implement regression detection

**File**: `src/dashboard/regression.rs`

```rust
use super::success_metrics::*;
use chrono::{DateTime, Utc, Duration};

/// Regression detection using EWMA control chart
pub struct RegressionDetector {
    alpha: f64, // Smoothing factor (0-1)
    control_limit_multiplier: f64, // Number of standard deviations
}

impl RegressionDetector {
    pub fn new(alpha: f64, control_limit_multiplier: f64) -> Self {
        assert!((0.0..=1.0).contains(&alpha), "alpha must be in [0, 1]");
        assert!(control_limit_multiplier > 0.0, "control limit multiplier must be > 0");
        Self { alpha, control_limit_multiplier }
    }

    /// Default detector with alpha=0.1, 3-sigma control limits
    pub fn default() -> Self {
        Self::new(0.1, 3.0)
    }

    /// Detect regression in a metric series
    pub fn detect_regression(&self, series: &SuccessMetricSeries) -> Option<RegressionAlert> {
        if series.data_points.len() < 10 {
            return None; // Need sufficient data
        }

        let values: Vec<f64> = series.data_points.iter()
            .map(|dp| dp.value)
            .collect();

        let (ewma, std_dev) = self.calculate_ewma_and_std_dev(&values);

        // Check for significant drop
        let current_value = *values.last().unwrap();
        let lower_control_limit = ewma - (self.control_limit_multiplier * std_dev);

        if current_value < lower_control_limit {
            Some(RegressionAlert {
                metric_name: series.metric_name.clone(),
                detected_at: Utc::now(),
                current_value,
                baseline_value: ewma,
                lower_control_limit,
                severity: self.calculate_severity(current_value, lower_control_limit),
            })
        } else {
            None
        }
    }

    fn calculate_ewma_and_std_dev(&self, values: &[f64]) -> (f64, f64) {
        let mut ewma = values[0];
        for &value in &values[1..] {
            ewma = self.alpha * value + (1.0 - self.alpha) * ewma;
        }

        let variance: f64 = values.iter()
            .map(|&v| (v - ewma).powi(2))
            .sum::<f64>() / (values.len() as f64 - 1.0);
        let std_dev = variance.sqrt();

        (ewma, std_dev)
    }

    fn calculate_severity(&self, current: f64, limit: f64) -> RegressionSeverity {
        let deviation = (limit - current).abs();
        if deviation > limit * 0.5 {
            RegressionSeverity::Critical
        } else if deviation > limit * 0.25 {
            RegressionSeverity::High
        } else {
            RegressionSeverity::Medium
        }
    }
}

/// Regression alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAlert {
    pub metric_name: String,
    pub detected_at: DateTime<Utc>,
    pub current_value: f64,
    pub baseline_value: f64,
    pub lower_control_limit: f64,
    pub severity: RegressionSeverity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RegressionSeverity {
    Medium,
    High,
    Critical,
}
```

---

### Step 4: Implement real-time streaming

**File**: `src/dashboard/streaming.rs`

```rust
use super::success_metrics::*;
use super::regression::*;
use tokio::sync::broadcast;
use std::time::Duration;

/// Stream update
#[derive(Debug, Clone)]
pub enum StreamUpdate {
    MetricUpdate(String, SuccessMetricDataPoint),
    RegressionAlert(RegressionAlert),
}

/// Dashboard streamer
pub struct DashboardStreamer {
    metrics_collector: SuccessMetricsCollector,
    regression_detector: RegressionDetector,
    sender: broadcast::Sender<StreamUpdate>,
}

impl DashboardStreamer {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);

        Self {
            metrics_collector: SuccessMetricsCollector::new(),
            regression_detector: RegressionDetector::default(),
            sender,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StreamUpdate> {
        self.sender.subscribe()
    }

    pub async fn update_metric(
        &mut self,
        name: String,
        value: f64,
        labels: std::collections::HashMap<String, String>,
    ) {
        // Add data point
        self.metrics_collector.add_data_point(&name, value, labels);

        // Create update
        let data_point = SuccessMetricDataPoint {
            timestamp: Utc::now(),
            value,
            labels,
        };

        let _ = self.sender.send(StreamUpdate::MetricUpdate(name.clone(), data_point));

        // Check for regression
        if let Some(series) = self.metrics_collector.get_metric(&name) {
            if let Some(alert) = self.regression_detector.detect_regression(series) {
                let _ = self.sender.send(StreamUpdate::RegressionAlert(alert));
            }
        }
    }

    pub fn get_metrics_collector(&self) -> &SuccessMetricsCollector {
        &self.metrics_collector
    }
}

impl Default for DashboardStreamer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 5: Implement anomaly detection

**File**: `src/dashboard/anomaly.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Anomaly alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    pub alert_id: String,
    pub detected_at: DateTime<Utc>,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyType {
    SuddenDrop,
    SuddenSpike,
    TrendChange,
    UnusualPattern,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Anomaly detector
pub struct AnomalyDetector;

impl AnomalyDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect anomalies in metric data
    pub fn detect_anomalies(
        &self,
        metric_name: &str,
        values: &[f64],
    ) -> Vec<AnomalyAlert> {
        let mut alerts = Vec::new();

        if values.len() < 10 {
            return alerts;
        }

        // Detect sudden drops
        if let Some(alert) = self.detect_sudden_drop(metric_name, values) {
            alerts.push(alert);
        }

        // Detect sudden spikes
        if let Some(alert) = self.detect_sudden_spike(metric_name, values) {
            alerts.push(alert);
        }

        alerts
    }

    fn detect_sudden_drop(&self, metric_name: &str, values: &[f64]) -> Option<AnomalyAlert> {
        let current = *values.last()?;
        let baseline: f64 = values.iter().take(values.len() - 5).sum::<f64>() / (values.len() - 5) as f64;
        let drop = (baseline - current) / baseline;

        if drop > 0.2 { // More than 20% drop
            Some(AnomalyAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                detected_at: Utc::now(),
                anomaly_type: AnomalyType::SuddenDrop,
                severity: if drop > 0.5 { AnomalySeverity::Critical } else { AnomalySeverity::High },
                description: format!("Sudden drop of {:.1}% detected", drop * 100.0),
                metrics: vec![metric_name.to_string()],
            })
        } else {
            None
        }
    }

    fn detect_sudden_spike(&self, metric_name: &str, values: &[f64]) -> Option<AnomalyAlert> {
        let current = *values.last()?;
        let baseline: f64 = values.iter().take(values.len() - 5).sum::<f64>() / (values.len() - 5) as f64;
        let spike = (current - baseline) / baseline;

        if spike > 0.5 { // More than 50% spike
            Some(AnomalyAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                detected_at: Utc::now(),
                anomaly_type: AnomalyType::SuddenSpike,
                severity: if spike > 2.0 { AnomalySeverity::Critical } else { AnomalySeverity::Medium },
                description: format!("Sudden spike of {:.1}% detected", spike * 100.0),
                metrics: vec![metric_name.to_string()],
            })
        } else {
            None
        }
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 6: Create module exports

**File**: `src/dashboard/mod.rs`

```rust
pub mod layout;
pub mod success_metrics;
pub mod regression;
pub mod streaming;
pub mod anomaly;

pub use layout::*;
pub use success_metrics::*;
pub use regression::*;
pub use streaming::*;
pub use anomaly::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod dashboard;
```

---

### Step 7: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test dashboard --verbose
```

**Expected Output**: All tests pass

---

### Step 8: Integration test for dashboard workflow

**File**: `tests/dashboard/integration_test.rs`

```rust
use glyphnova_engine::dashboard::*;
use std::time::Duration;

#[tokio::test]
async fn test_full_dashboard_workflow() {
    let mut streamer = DashboardStreamer::new();

    // Subscribe to updates
    let mut receiver = streamer.subscribe();

    // Simulate metric updates
    let mut task = tokio::spawn(async move {
        for i in 0..100 {
            let value = if i > 80 { 0.7 } else { 0.95 }; // Simulate regression
            streamer.update_metric(
                "success_rate".to_string(),
                value,
                std::collections::HashMap::new(),
            ).await;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    // Receive updates
    let mut alerts_received = 0;
    tokio::time::timeout(Duration::from_secs(2), async {
        while let Ok(update) = receiver.recv().await {
            if matches!(update, StreamUpdate::RegressionAlert(_)) {
                alerts_received += 1;
            }
        }
    }).await.ok();

    assert!(alerts_received > 0, "Should have received regression alerts");
}
```

---

### Step 9: Commit

```bash
git add src/dashboard/ tests/dashboard/
git commit -m "feat: implement success regression dashboards

- Define dashboard layout with widgets
- Implement success metrics collector
- Implement regression detection using EWMA control chart
- Implement real-time streaming updates
- Implement anomaly detection for drops/spikes
- Add comprehensive unit and integration tests

Relates to ADR-0008: Metrics instrumented before UX experimentation"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ Regression detection catches real regressions
- ✅ Streaming updates work in real-time
- ✅ Anomaly detection flags unusual patterns

---

## Next Steps

After completing Task 04:
1. Proceed to Task 05: Stop Condition Evaluation

---

**End of Task 04**
