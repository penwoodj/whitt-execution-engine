# Task 6: Quality Dashboard Integration

**Files:**
- Create: `crates/quality/dashboard/src/lib.rs`
- Create: `crates/quality/dashboard/src/types.rs`
- Create: `crates/quality/dashboard/src/routes.rs`
- Create: `crates/quality/dashboard/src/handlers.rs`
- Create: `crates/quality/dashboard/src/trends.rs`
- Create: `crates/quality/dashboard/Cargo.toml`
- Test: `crates/quality/dashboard/tests/dashboard_tests.rs`

**Duration:** 1 week

## Overview

Build the dashboard API with quality trends and benchmark visualization. Provides real-time access to quality metrics, trends analysis, and interactive benchmark comparisons.

## Architecture

The dashboard consists of:

1. **API Layer** — RESTful endpoints for quality data
2. **Trends Engine** — Calculate quality trends over time
3. **Visualization** — Prepare data for charts and graphs
4. **Dashboard Layout** — Web interface structure

---

## Implementation Steps

### Step 1: Crate Setup

- [ ] **Create dashboard crate with Cargo.toml**
  ```toml
  [package]
  name = "agentsdk-dashboard"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  agentsdk-types = { path = "../../types" }
  agentsdk-benchmark = { path = "../benchmark" }
  agentsdk-reports = { path = "../reports" }
  axum = "0.7"
  tower = "0.4"
  tower-http = { version = "0.5", features = ["cors", "trace"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  thiserror = "1.0"
  tokio = { version = "1.0", features = ["full"] }
  chrono = { version = "0.4", features = ["serde"] }
  tracing = "0.1"
  ```

### Step 2: Define Types

- [ ] **Create types.rs with dashboard types**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub quality_trends: QualityTrendsData,
    pub file_type_comparison: FileTypeComparisonData,
    pub benchmark_visualization: BenchmarkVisualizationData,
    pub summary: DashboardSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrendsData {
    pub time_series: Vec<QualityDataPoint>,
    pub moving_average: Vec<QualityDataPoint>,
    pub targets: QualityTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDataPoint {
    pub timestamp: DateTime<Utc>,
    pub quality_score: f64,
    pub confidence: f64,
    pub convergence_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTargets {
    pub min_quality_score: f64,
    pub target_quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeComparisonData {
    pub file_types: Vec<FileTypeMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeMetrics {
    pub file_type: String,
    pub avg_quality_score: f64,
    pub avg_confidence: f64,
    pub convergence_rate: f64,
    pub avg_cost_usd: f64,
    pub benchmark_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkVisualizationData {
    pub benchmarks: Vec<BenchmarkVisualization>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkVisualization {
    pub id: Uuid,
    pub name: String,
    pub quality_score: f64,
    pub duration_ms: u64,
    pub cost_usd: f64,
    pub file_type: String,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_benchmarks: usize,
    pub avg_quality_score: f64,
    pub avg_convergence_rate: f64,
    pub total_cost_usd: f64,
    pub active_file_types: usize,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 3: Implement Trends Engine

- [ ] **Create trends.rs**

```rust
use super::types::*;
use agentsdk_benchmark::{Benchmark, BenchmarkStorage};

pub struct TrendsEngine {
    storage: Arc<BenchmarkStorage>,
}

impl TrendsEngine {
    pub fn new(storage: Arc<BenchmarkStorage>) -> Self {
        Self { storage }
    }

    pub async fn calculate_quality_trends(
        &self,
        period: ReportPeriod,
    ) -> Result<QualityTrendsData, TrendsError> {
        let benchmarks = self.storage.query_benchmarks(BenchmarkFilters {
            start_time: Some(period.start),
            end_time: Some(period.end),
            ..Default::default()
        }).await?;

        let mut time_series: Vec<QualityDataPoint> = benchmarks
            .into_iter()
            .map(|b| QualityDataPoint {
                timestamp: b.created_at,
                quality_score: b.results.quality.score,
                confidence: b.results.quality.confidence,
                convergence_rate: if b.results.quality.converged { 1.0 } else { 0.0 },
            })
            .collect();

        time_series.sort_by_key(|p| p.timestamp);

        let moving_average = self.calculate_moving_average(&time_series, 7)?;

        Ok(QualityTrendsData {
            time_series,
            moving_average,
            targets: QualityTargets {
                min_quality_score: 0.8,
                target_quality_score: 0.95,
            },
        })
    }

    fn calculate_moving_average(&self, data: &[QualityDataPoint], window: usize) -> Result<Vec<QualityDataPoint>, TrendsError> {
        if data.len() < window {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        for i in window..=data.len() {
            let window_data = &data[i - window..i];
            let avg_score: f64 = window_data.iter().map(|d| d.quality_score).sum::<f64>() / window as f64;
            let avg_confidence: f64 = window_data.iter().map(|d| d.confidence).sum::<f64>() / window as f64;
            let avg_convergence: f64 = window_data.iter().map(|d| d.convergence_rate).sum::<f64>() / window as f64;

            result.push(QualityDataPoint {
                timestamp: window_data.last().unwrap().timestamp,
                quality_score: avg_score,
                confidence: avg_confidence,
                convergence_rate: avg_convergence,
            });
        }

        Ok(result)
    }

    pub async fn calculate_file_type_comparison(&self) -> Result<FileTypeComparisonData, TrendsError> {
        let benchmarks = self.storage.list_all_benchmarks().await?;

        let mut by_file_type: std::collections::HashMap<String, Vec<&Benchmark>> = std::collections::HashMap::new();
        for b in &benchmarks {
            by_file_type
                .entry(b.file_type.clone())
                .or_insert_with(Vec::new)
                .push(b);
        }

        let mut file_types = Vec::new();
        for (file_type, benchmarks) in by_file_type {
            let avg_quality: f64 = benchmarks.iter().map(|b| b.results.quality.score).sum::<f64>() / benchmarks.len() as f64;
            let avg_confidence: f64 = benchmarks.iter().map(|b| b.results.quality.confidence).sum::<f64>() / benchmarks.len() as f64;
            let converged_count = benchmarks.iter().filter(|b| b.results.quality.converged).count();
            let convergence_rate = converged_count as f64 / benchmarks.len() as f64;
            let avg_cost: f64 = benchmarks.iter().map(|b| b.results.cost.total_cost_usd).sum::<f64>() / benchmarks.len() as f64;

            file_types.push(FileTypeMetrics {
                file_type,
                avg_quality_score: avg_quality,
                avg_confidence,
                convergence_rate,
                avg_cost_usd: avg_cost,
                benchmark_count: benchmarks.len(),
            });
        }

        file_types.sort_by(|a, b| b.avg_quality_score.partial_cmp(&a.avg_quality_score).unwrap());

        Ok(FileTypeComparisonData { file_types })
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 4: Implement API Routes

- [ ] **Create routes.rs**

```rust
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn create_routes(
    storage: Arc<BenchmarkStorage>,
    trends: Arc<TrendsEngine>,
) -> Router {
    Router::new()
        .route("/api/dashboard", get(dashboard_handler))
        .route("/api/trends", get(trends_handler))
        .route("/api/file-types", get(file_types_handler))
        .route("/api/benchmarks", get(benchmarks_handler))
        .layer(axum::extract::DefaultBodyLimit::max(10_000_000))
}

async fn dashboard_handler(
    State(trends): State<Arc<TrendsEngine>>,
) -> Json<DashboardData> {
    let period = ReportPeriod {
        start: Utc::now() - chrono::Duration::days(30),
        end: Utc::now(),
    };

    let quality_trends = trends.calculate_quality_trends(period).await.unwrap();
    let file_type_comparison = trends.calculate_file_type_comparison().await.unwrap();

    Json(DashboardData {
        quality_trends,
        file_type_comparison,
        benchmark_visualization: BenchmarkVisualizationData { benchmarks: Vec::new() },
        summary: DashboardSummary {
            total_benchmarks: 0,
            avg_quality_score: 0.0,
            avg_convergence_rate: 0.0,
            total_cost_usd: 0.0,
            active_file_types: 0,
        },
    })
}

async fn trends_handler(
    State(trends): State<Arc<TrendsEngine>>,
    Query(params): Query<TrendsQuery>,
) -> Json<QualityTrendsData> {
    let period = ReportPeriod {
        start: params.start.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)),
        end: params.end.unwrap_or_else(Utc::now),
    };

    let data = trends.calculate_quality_trends(period).await.unwrap();
    Json(data)
}

#[derive(Debug, Deserialize)]
struct TrendsQuery {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 5: Implement Handlers

- [ ] **Create handlers.rs**

```rust
use axum::{
    extract::{Path, State, Query},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use super::types::*;

pub async fn dashboard_handler(
    State(storage): State<Arc<BenchmarkStorage>>,
    State(trends): State<Arc<TrendsEngine>>,
) -> Result<Json<DashboardData>, ApiError> {
    // Calculate all dashboard data
    // Return combined response
}

pub async fn trends_handler(
    State(trends): State<Arc<TrendsEngine>>,
    Query(params): Query<TrendsQuery>,
) -> Result<Json<QualityTrendsData>, ApiError> {
    let period = ReportPeriod {
        start: params.start.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)),
        end: params.end.unwrap_or_else(Utc::now),
    };

    let data = trends.calculate_quality_trends(period).await?;
    Ok(Json(data))
}

pub async fn file_types_handler(
    State(trends): State<Arc<TrendsEngine>>,
) -> Result<Json<FileTypeComparisonData>, ApiError> {
    let data = trends.calculate_file_type_comparison().await?;
    Ok(Json(data))
}

pub async fn benchmarks_handler(
    State(storage): State<Arc<BenchmarkStorage>>,
    Query(params): Query<BenchmarkQuery>,
) -> Result<Json<Vec<BenchmarkVisualization>>, ApiError> {
    let filters = BenchmarkFilters {
        file_type: params.file_type,
        backend_type: params.backend_type,
        ..Default::default()
    };

    let benchmark_ids = storage.query_benchmarks(filters).await?;

    let mut benchmarks = Vec::new();
    for id in benchmark_ids {
        if let Ok(b) = storage.get_benchmark(id).await {
            benchmarks.push(BenchmarkVisualization {
                id: b.id,
                name: b.name,
                quality_score: b.results.quality.score,
                duration_ms: b.results.duration_ms,
                cost_usd: b.results.cost.total_cost_usd,
                file_type: b.file_type,
                backend: format!("{}:{}", b.backend.backend_type, b.backend.model),
            });
        }
    }

    Ok(Json(benchmarks))
}

#[derive(Debug, Deserialize)]
pub struct BenchmarkQuery {
    pub file_type: Option<String>,
    pub backend_type: Option<String>,
    pub limit: Option<usize>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 6: Documentation

- [ ] **Create README.md with API documentation**

## Completion Criteria

Task 6 is complete when:

- ✅ Dashboard data types defined
- ✅ Trends engine with moving averages
- ✅ File-type comparison calculation
- ✅ API routes implemented (/api/dashboard, /api/trends, /api/file-types, /api/benchmarks)
- ✅ Response handlers with error handling
- ✅ All tests passing
- ✅ API documentation complete
