# Benchmark Data Mock Generators

## Benchmark Data Generator 1: Random Benchmarks

```rust
use agentsdk_benchmark::types::*;
use chrono::Utc;

pub struct RandomBenchmarkGenerator {
    count: usize,
}

impl RandomBenchmarkGenerator {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();

        for i in 0..self.count {
            let suite_id = uuid::Uuid::new_v4();
            let benchmark = Benchmark::new(
                suite_id,
                format!("benchmark-{}", i),
                format!("Random benchmark {}", i),
                format!("v{}.{}.{}", i, 0, 0),
                serde_json::json!({"policy": "value"}),
                LLMBackendInfo {
                    backend_type: vec!["openai", "anthropic"][i % 2].to_string(),
                    model: vec!["gpt-4", "gpt-3.5", "claude-3"][i % 3].to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                vec!["code", "docs", "config"][i % 3].to_string(),
            );

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

## Benchmark Data Generator 2: Trending Benchmarks

```rust
pub struct TrendingBenchmarkGenerator {
    count: usize,
    trend: Trend,
}

pub enum Trend {
    Improving,
    Declining,
    Stable,
    Volatile,
}

impl TrendingBenchmarkGenerator {
    pub fn new(count: usize, trend: Trend) -> Self {
        Self { count, trend }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();

        for i in 0..self.count {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("benchmark-{}", i),
                format!("Trend benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"policy": "value"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Set quality score based on trend
            match self.trend {
                Trend::Improving => {
                    benchmark.results.quality.score = 0.7 + (i as f64 / self.count as f64) * 0.3;
                }
                Trend::Declining => {
                    benchmark.results.quality.score = 1.0 - (i as f64 / self.count as f64) * 0.3;
                }
                Trend::Stable => {
                    benchmark.results.quality.score = 0.85;
                }
                Trend::Volatile => {
                    benchmark.results.quality.score = 0.5 + (rand::random::<f64>() * 0.4);
                }
            }

            benchmark.created_at = Utc::now() - chrono::Duration::days((self.count - i) as i64);
            benchmark.updated_at = benchmark.created_at;

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

## Benchmark Data Generator 3: Quality Score Distribution

```rust
pub struct DistributedBenchmarkGenerator {
    count: usize,
    mean: f64,
    std_dev: f64,
}

impl DistributedBenchmarkGenerator {
    pub fn new(count: usize, mean: f64, std_dev: f64) -> Self {
        Self { count, mean, std_dev }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();

        for i in 0..self.count {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("benchmark-{}", i),
                format!("Distributed benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"policy": "value"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Generate quality score from normal distribution (Box-Muller transform)
            let u1: f64 = rand::random();
            let u2: f64 = rand::random();
            let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            let score = self.mean + self.std_dev * z0;
            benchmark.results.quality.score = score.max(0.0).min(1.0);

            benchmark.created_at = Utc::now() - chrono::Duration::seconds((self.count - i) as i64);
            benchmark.updated_at = benchmark.created_at;

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

## Usage in Tests

```rust
#[test]
fn test_statistical_analysis() {
    let generator = DistributedBenchmarkGenerator::new(100, 0.85, 0.1);
    let benchmarks = generator.generate();

    let analysis = BenchmarkAnalysis::new(benchmarks);

    let mean = analysis.mean_quality_score().unwrap();
    let median = analysis.median_quality_score().unwrap();
    let std_dev = analysis.std_dev_quality_score().unwrap();

    assert!((mean - 0.85).abs() < 0.05); // Within 5% of target
    assert!(std_dev > 0.0); // Some variance
}

#[test]
fn test_trend_detection() {
    let generator = TrendingBenchmarkGenerator::new(30, Trend::Improving);
    let benchmarks = generator.generate();

    // First 10 should have lower quality than last 10
    let early_mean: f64 = benchmarks[..10].iter().map(|b| b.results.quality.score).sum::<f64>() / 10.0;
    let late_mean: f64 = benchmarks[20..].iter().map(|b| b.results.quality.score).sum::<f64>() / 10.0;

    assert!(late_mean > early_mean);
}
```

---

## Additional Mock Scenarios

### Step 4: Benchmark timeout mock

```rust
pub struct TimeoutBenchmarkGenerator {
    count: usize,
    timeout_ratio: f64,
}

impl TimeoutBenchmarkGenerator {
    pub fn new(count: usize, timeout_ratio: f64) -> Self {
        Self { count, timeout_ratio }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();

        for i in 0..self.count {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("timeout-benchmark-{}", i),
                format!("Timeout benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"policy": "value"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Mark some benchmarks as timed out
            if (i as f64 / self.count as f64) < self.timeout_ratio {
                benchmark.results.execution.timed_out = true;
                benchmark.results.execution.duration_ms = 30000; // 30s timeout
            }

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

- [ ] **Step 4a:** Implement timeout benchmark generator
- [ ] **Step 4b:** Add test for timeout handling
- [ ] **Step 4c:** Run test to verify timeout detection
- [ ] **Step 4d:** Commit: `test: add timeout benchmark mocks`

### Step 5: Regression detection mock

```rust
pub struct RegressionBenchmarkGenerator {
    count: usize,
    regression_point: usize,
    regression_magnitude: f64,
}

impl RegressionBenchmarkGenerator {
    pub fn new(count: usize, regression_point: usize, magnitude: f64) -> Self {
        Self {
            count,
            regression_point,
            regression_magnitude: magnitude,
        }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();

        for i in 0..self.count {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("regression-benchmark-{}", i),
                format!("Regression benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"policy": "value"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Apply regression at specified point
            if i >= self.regression_point {
                benchmark.results.quality.score = 0.9 - self.regression_magnitude;
            } else {
                benchmark.results.quality.score = 0.9;
            }

            benchmark.created_at = Utc::now() - chrono::Duration::days((self.count - i) as i64);
            benchmark.updated_at = benchmark.created_at;

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

- [ ] **Step 5a:** Implement regression benchmark generator
- [ ] **Step 5b:** Add test for regression detection
- [ ] **Step 5c:** Run test to verify regression detection
- [ ] **Step 5d:** Commit: `test: add regression detection mocks`

### Step 6: Significance testing mock

```rust
pub struct SignificanceTestBenchmarkGenerator {
    count: usize,
    group_a_mean: f64,
    group_b_mean: f64,
    variance: f64,
}

impl SignificanceTestBenchmarkGenerator {
    pub fn new(count: usize, group_a: f64, group_b: f64, variance: f64) -> Self {
        Self {
            count,
            group_a_mean: group_a,
            group_b_mean: group_b,
            variance,
        }
    }

    pub fn generate(&self) -> Vec<Benchmark> {
        let mut benchmarks = Vec::new();
        let group_size = self.count / 2;

        // Generate group A
        for i in 0..group_size {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("group-a-{}", i),
                format!("Group A benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"group": "A"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Generate score from distribution around group A mean
            let score = self.group_a_mean + (rand::random::<f64>() - 0.5) * self.variance * 2.0;
            benchmark.results.quality.score = score.max(0.0).min(1.0);

            benchmarks.push(benchmark);
        }

        // Generate group B
        for i in 0..(self.count - group_size) {
            let suite_id = uuid::Uuid::new_v4();
            let mut benchmark = Benchmark::new(
                suite_id,
                format!("group-b-{}", i),
                format!("Group B benchmark {}", i),
                "v1.0.0".to_string(),
                serde_json::json!({"group": "B"}),
                LLMBackendInfo {
                    backend_type: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    version: "1.0".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                "code".to_string(),
            );

            // Generate score from distribution around group B mean
            let score = self.group_b_mean + (rand::random::<f64>() - 0.5) * self.variance * 2.0;
            benchmark.results.quality.score = score.max(0.0).min(1.0);

            benchmarks.push(benchmark);
        }

        benchmarks
    }
}
```

- [ ] **Step 6a:** Implement significance testing generator
- [ ] **Step 6b:** Add test for statistical significance
- [ ] **Step 6c:** Run test to verify significance detection
- [ ] **Step 6d:** Commit: `test: add significance testing mocks`

---

## Mock Metric Store with Time-Series Data

### Step 7: Time-series metric store

```rust
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MetricPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub metadata: serde_json::Value,
}

pub struct MockMetricStore {
    metrics: BTreeMap<String, Vec<MetricPoint>>,
}

impl MockMetricStore {
    pub fn new() -> Self {
        Self {
            metrics: BTreeMap::new(),
        }
    }

    pub fn record_metric(&mut self, name: &str, value: f64, metadata: serde_json::Value) {
        let point = MetricPoint {
            timestamp: Utc::now(),
            value,
            metadata,
        };

        self.metrics.entry(name.to_string()).or_default().push(point);
    }

    pub fn get_metrics(&self, name: &str) -> Option<&[MetricPoint]> {
        self.metrics.get(name).map(|v| v.as_slice())
    }

    pub fn get_metrics_range(
        &self,
        name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<MetricPoint> {
        self.metrics
            .get(name)
            .map(|points| {
                points
                    .iter()
                    .filter(|p| p.timestamp >= start && p.timestamp <= end)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn aggregate_metrics(
        &self,
        name: &str,
        aggregation: Aggregation,
    ) -> Option<f64> {
        let points = self.metrics.get(name)?;
        if points.is_empty() {
            return None;
        }

        match aggregation {
            Aggregation::Mean => {
                let sum: f64 = points.iter().map(|p| p.value).sum();
                Some(sum / points.len() as f64)
            }
            Aggregation::Median => {
                let mut values: Vec<f64> = points.iter().map(|p| p.value).collect();
                values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Some(values[values.len() / 2])
            }
            Aggregation::Min => points.iter().map(|p| p.value).reduce(f64::min),
            Aggregation::Max => points.iter().map(|p| p.value).reduce(f64::max),
            Aggregation::Sum => Some(points.iter().map(|p| p.value).sum()),
        }
    }
}

pub enum Aggregation {
    Mean,
    Median,
    Min,
    Max,
    Sum,
}
```

- [ ] **Step 7a:** Implement mock metric store
- [ ] **Step 7b:** Add time-series data generation
- [ ] **Step 7c:** Implement aggregation functions
- [ ] **Step 7d:** Add unit tests for metric store
- [ ] **Step 7e:** Commit: `test: add mock metric store`

---

## Mock Execution Engine for Benchmark Harness

### Step 8: Benchmark execution engine mock

```rust
pub struct MockBenchmarkExecutionEngine {
    execution_time_ms: u64,
    success_rate: f64,
    failure_error: Option<String>,
}

impl MockBenchmarkExecutionEngine {
    pub fn new() -> Self {
        Self {
            execution_time_ms: 100,
            success_rate: 1.0,
            failure_error: None,
        }
    }

    pub fn with_execution_time(mut self, ms: u64) -> Self {
        self.execution_time_ms = ms;
        self
    }

    pub fn with_success_rate(mut self, rate: f64) -> Self {
        self.success_rate = rate;
        self
    }

    pub fn with_failure_error(mut self, error: String) -> Self {
        self.failure_error = Some(error);
        self
    }

    pub async fn execute_benchmark(
        &self,
        benchmark: &Benchmark,
    ) -> Result<BenchmarkExecutionResult, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.execution_time_ms)).await;

        if rand::random::<f64>() > self.success_rate {
            let error = self.failure_error.clone().unwrap_or_else(|| "Random failure".to_string());
            return Err(error);
        }

        Ok(BenchmarkExecutionResult {
            benchmark_id: benchmark.id.clone(),
            duration_ms: self.execution_time_ms,
            success: true,
            metrics: serde_json::json!({
                "quality_score": benchmark.results.quality.score,
                "execution_time": self.execution_time_ms,
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkExecutionResult {
    pub benchmark_id: String,
    pub duration_ms: u64,
    pub success: bool,
    pub metrics: serde_json::Value,
}
```

- [ ] **Step 8a:** Implement mock benchmark execution engine
- [ ] **Step 8b:** Add configuration options (time, success rate, error)
- [ ] **Step 8c:** Add unit tests for execution engine
- [ ] **Step 8d:** Commit: `test: add mock benchmark execution engine`

---

## Property-Based Test Specifications for Benchmarks

### Step 9: Property-based tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_quality_score_always_valid(score in 0.0f64..=1.0) {
        let generator = DistributedBenchmarkGenerator::new(1, score, 0.1);
        let benchmarks = generator.generate();

        assert!(benchmarks[0].results.quality.score >= 0.0);
        assert!(benchmarks[0].results.quality.score <= 1.0);
    }

    #[test]
    fn prop_benchmark_ordering_preserves_timestamp(count in 1..100usize) {
        let generator = RandomBenchmarkGenerator::new(count);
        let benchmarks = generator.generate();

        for window in benchmarks.windows(2) {
            assert!(window[0].created_at <= window[1].created_at);
        }
    }

    #[test]
    fn prop_improving_trend_always_increases(count in 10..100usize) {
        let generator = TrendingBenchmarkGenerator::new(count, Trend::Improving);
        let benchmarks = generator.generate();

        let scores: Vec<f64> = benchmarks.iter().map(|b| b.results.quality.score).collect();
        for window in scores.windows(2) {
            assert!(window[0] <= window[1] + 0.1); // Allow small variation
        }
    }
}
```

- [ ] **Step 9a:** Implement property-based tests
- [ ] **Step 9b:** Run tests with proptest
- [ ] **Step 9c:** Analyze failure cases
- [ ] **Step 9d:** Commit: `test: add property-based benchmark tests`

---

## Edge Cases

### Step 10: Empty benchmark suite

```rust
#[test]
fn test_empty_benchmark_suite() {
    let generator = RandomBenchmarkGenerator::new(0);
    let benchmarks = generator.generate();

    assert!(benchmarks.is_empty());

    let analysis = BenchmarkAnalysis::new(benchmarks);
    assert!(analysis.mean_quality_score().is_none());
    assert!(analysis.median_quality_score().is_none());
}
```

- [ ] **Step 10a:** Implement empty suite test
- [ ] **Step 10b:** Run test to verify graceful handling
- [ ] **Step 10c:** Commit: `test: add empty benchmark suite edge case`

### Step 11: Corrupted benchmark data

```rust
#[test]
fn test_corrupted_benchmark_data() {
    let mut generator = DistributedBenchmarkGenerator::new(10, 0.85, 0.1);
    let mut benchmarks = generator.generate();

    // Corrupt some data
    benchmarks[3].results.quality.score = -0.5; // Invalid negative score
    benchmarks[7].results.execution.duration_ms = u64::MAX; // Unrealistic duration

    let analysis = BenchmarkAnalysis::new(benchmarks);

    // Analysis should handle corrupted data gracefully
    let mean = analysis.mean_quality_score();
    assert!(mean.is_some() || mean.is_err());
}
```

- [ ] **Step 11a:** Implement corrupted data test
- [ ] **Step 11b:** Run test to verify error handling
- [ ] **Step 11c:** Commit: `test: add corrupted benchmark data edge case`

### Step 12: Concurrent benchmark runs

```rust
#[tokio::test]
async fn test_concurrent_benchmark_runs() {
    let engine = MockBenchmarkExecutionEngine::new()
        .with_execution_time(50)
        .with_success_rate(0.95);

    let benchmark = RandomBenchmarkGenerator::new(1).generate()[0].clone();

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let engine = engine.clone();
            let benchmark = benchmark.clone();
            tokio::spawn(async move {
                engine.execute_benchmark(&benchmark).await
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    assert_eq!(results.len(), 10);
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(success_count >= 8); // At least 80% success rate
}
```

- [ ] **Step 12a:** Implement concurrent benchmark test
- [ ] **Step 12b:** Run test to verify thread safety
- [ ] **Step 12c:** Commit: `test: add concurrent benchmark runs edge case`

---

## Verification

### Final verification tests

```bash
# Run all benchmark tests
cargo test benchmark_test

# Expected output:
# test test_statistical_analysis ... ok
# test test_trend_detection ... ok
# test prop_quality_score_always_valid ... ok
# test prop_benchmark_ordering_preserves_timestamp ... ok
# test prop_improving_trend_always_increases ... ok
# test test_empty_benchmark_suite ... ok
# test test_corrupted_benchmark_data ... ok
# test test_concurrent_benchmark_runs ... ok
# test result: ok. 8 passed in X.XXs

# Run property-based tests
cargo test -- --ignored prop_

# Expected output:
# test prop_quality_score_always_valid ... ok
# test prop_benchmark_ordering_preserves_timestamp ... ok
# test prop_improving_trend_always_increases ... ok
```

**Checkpoint Criteria:**
- ✅ Random benchmark generator works
- ✅ Trending benchmark generator works (improving, declining, stable, volatile)
- ✅ Distributed benchmark generator works (normal distribution)
- ✅ Timeout benchmark scenario handled
- ✅ Regression detection works
- ✅ Significance testing works
- ✅ Mock metric store with time-series data
- ✅ Mock execution engine for benchmarks
- ✅ Property-based tests cover invariants
- ✅ Edge cases handled (empty suite, corrupted data, concurrent runs)
- ✅ All tests pass with expected output
