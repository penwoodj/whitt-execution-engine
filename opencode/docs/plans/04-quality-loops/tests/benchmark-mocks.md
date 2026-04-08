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
