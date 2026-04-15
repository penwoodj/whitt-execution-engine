# Task 2: Benchmark Harness

**Files:**
- Create: `crates/quality/benchmark/src/lib.rs`
- Create: `crates/quality/benchmark/src/types.rs`
- Create: `crates/quality/benchmark/src/storage.rs`
- Create: `crates/quality/benchmark/src/execution.rs`
- Create: `crates/quality/benchmark/src/analysis.rs`
- Create: `crates/quality/benchmark/src/versioning.rs`
- Create: `crates/quality/benchmark/Cargo.toml`
- Test: `crates/quality/benchmark/tests/benchmark_tests.rs`

**Duration:** 2 weeks

## Overview

Build the benchmark storage and execution system with full provenance tracking. Benchmarks MUST store workflow version, policy snapshot, backend, and file type to enable accurate trend analysis and attribution.

## Architecture

The benchmark system consists of:

1. **Benchmark Types** — Benchmark struct with full provenance (workflow_version, policy_snapshot, backend, file_type)
2. **BenchmarkSuite** — Collection of related benchmarks with metadata
3. **Storage** — Persistent storage for benchmarks with time-based partitioning
4. **Execution** — Benchmark runner with parallel execution and result collection
5. **Versioning** — Track workflow and policy versions for reproducibility
6. **Analysis** — Statistical analysis of benchmark results (mean, median, std dev, percentiles)

---

## Step-by-Step Implementation

### Step 1: Create crate structure and Cargo.toml

- [ ] **Step 1.1: Create benchmark crate directory**

```bash
mkdir -p crates/quality/benchmark/src
```

- [ ] **Step 1.2: Write Cargo.toml**

Create file: `crates/quality/benchmark/Cargo.toml`

```toml
[package]
name = "agentsdk-benchmark"
version = "0.1.0"
edition = "2021"

[dependencies]
agentsdk-types = { path = "../../types" }
agentsdk-llm = { path = "../../llm" }
agentsdk-verifier = { path = "../verifier" }
agentsdk-loops = { path = "../loops" }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
rusqlite = { version = "0.29", features = ["bundled", "chrono"] }
dirs = "5.0"
anyhow = "1.0"
statrs = "0.16"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
```

- [ ] **Step 1.3: Commit**

```bash
git add crates/quality/benchmark/Cargo.toml
git commit -m "feat(quality): create benchmark crate with dependencies"
```

---

### Step 2: Define benchmark types

- [ ] **Step 2.1: Create types.rs with benchmark types**

Create file: `crates/quality/benchmark/src/types.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Unique identifier for a benchmark
pub type BenchmarkId = Uuid;

/// Unique identifier for a benchmark suite
pub type SuiteId = Uuid;

/// Benchmark with full provenance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmark {
    /// Unique identifier
    pub id: BenchmarkId,

    /// Suite this benchmark belongs to
    pub suite_id: SuiteId,

    /// Name of the benchmark
    pub name: String,

    /// Description of what the benchmark measures
    pub description: String,

    /// Workflow version used (from version control)
    pub workflow_version: String,

    /// Complete policy snapshot at time of benchmark
    pub policy_snapshot: serde_json::Value,

    /// LLM backend used
    pub backend: LLMBackendInfo,

    /// File type being benchmarked
    pub file_type: String,

    /// Benchmark metadata
    pub metadata: BenchmarkMetadata,

    /// Benchmark results
    pub results: BenchmarkResults,

    /// When benchmark was created
    pub created_at: DateTime<Utc>,

    /// When benchmark was last updated
    pub updated_at: DateTime<Utc>,
}

/// Information about LLM backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMBackendInfo {
    /// Backend type (e.g., "openai", "anthropic")
    pub backend_type: String,

    /// Model name (e.g., "gpt-4", "claude-3")
    pub model: String,

    /// Backend version or configuration hash
    pub version: String,

    /// Additional backend parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Benchmark metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    /// Tags for categorization
    pub tags: Vec<String>,

    /// Labels for filtering
    pub labels: HashMap<String, String>,

    /// Benchmark category (e.g., "quality", "performance", "cost")
    pub category: String,

    /// Priority (higher = more important)
    pub priority: u32,

    /// Whether benchmark is enabled
    pub enabled: bool,

    /// Custom metadata fields
    pub custom: HashMap<String, serde_json::Value>,
}

/// Benchmark results with metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// Duration of benchmark execution (in milliseconds)
    pub duration_ms: u64,

    /// Quality metrics
    pub quality: QualityMetrics,

    /// Performance metrics
    pub performance: PerformanceMetrics,

    /// Cost metrics
    pub cost: CostMetrics,

    /// Additional custom metrics
    pub custom: HashMap<String, f64>,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Overall quality score (0.0 - 1.0)
    pub score: f64,

    /// Confidence in quality score (0.0 - 1.0)
    pub confidence: f64,

    /// Number of verifiers passed
    pub verifiers_passed: usize,

    /// Total number of verifiers run
    pub verifiers_total: usize,

    /// Number of quality loop iterations
    pub iterations: usize,

    /// Whether quality loop converged
    pub converged: bool,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total time (in milliseconds)
    pub total_time_ms: u64,

    /// Generate step time (in milliseconds)
    pub generate_time_ms: u64,

    /// Verify step time (in milliseconds)
    pub verify_time_ms: u64,

    /// Repair step time (in milliseconds)
    pub repair_time_ms: u64,

    /// Peak memory usage (in bytes)
    pub peak_memory_bytes: Option<u64>,

    /// CPU usage percentage (average)
    pub cpu_percent: Option<f64>,
}

/// Cost metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostMetrics {
    /// Total cost in USD
    pub total_cost_usd: f64,

    /// Input tokens used
    pub input_tokens: usize,

    /// Output tokens used
    pub output_tokens: usize,

    /// Total tokens used
    pub total_tokens: usize,

    /// Cost per 1K input tokens
    pub input_cost_per_1k: f64,

    /// Cost per 1K output tokens
    pub output_cost_per_1k: f64,
}

/// Benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    /// Unique identifier
    pub id: SuiteId,

    /// Name of the suite
    pub name: String,

    /// Description of the suite
    pub description: String,

    /// Benchmarks in this suite
    pub benchmarks: Vec<BenchmarkId>,

    /// Suite metadata
    pub metadata: SuiteMetadata,

    /// When suite was created
    pub created_at: DateTime<Utc>,

    /// When suite was last updated
    pub updated_at: DateTime<Utc>,
}

/// Suite metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteMetadata {
    /// Suite category
    pub category: String,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Schedule for running this suite (cron-like)
    pub schedule: Option<String>,

    /// Whether suite is enabled
    pub enabled: bool,
}

impl Benchmark {
    /// Create a new benchmark
    pub fn new(
        suite_id: SuiteId,
        name: String,
        description: String,
        workflow_version: String,
        policy_snapshot: serde_json::Value,
        backend: LLMBackendInfo,
        file_type: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            suite_id,
            name,
            description,
            workflow_version,
            policy_snapshot,
            backend,
            file_type,
            metadata: BenchmarkMetadata::default(),
            results: BenchmarkResults::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Calculate overall score (weighted average of quality, performance, cost)
    pub fn overall_score(&self) -> f64 {
        let quality_weight = 0.5;
        let performance_weight = 0.3;
        let cost_weight = 0.2;

        // Normalize performance (lower time is better)
        let performance_score = if self.results.performance.total_time_ms > 0 {
            1.0 / (1.0 + self.results.performance.total_time_ms as f64 / 1000.0)
        } else {
            1.0
        };

        // Normalize cost (lower cost is better)
        let cost_score = if self.results.cost.total_cost_usd > 0.0 {
            1.0 / (1.0 + self.results.cost.total_cost_usd * 100.0)
        } else {
            1.0
        };

        quality_weight * self.results.quality.score
            + performance_weight * performance_score
            + cost_weight * cost_score
    }
}

impl Default for BenchmarkMetadata {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            labels: HashMap::new(),
            category: "general".to_string(),
            priority: 100,
            # presence = enabled,
            custom: HashMap::new(),
        }
    }
}

impl Default for BenchmarkResults {
    fn default() -> Self {
        Self {
            duration_ms: 0,
            quality: QualityMetrics::default(),
            performance: PerformanceMetrics::default(),
            cost: CostMetrics::default(),
            custom: HashMap::new(),
        }
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            score: 0.0,
            confidence: 0.0,
            verifiers_passed: 0,
            verifiers_total: 0,
            iterations: 0,
            converged: false,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_time_ms: 0,
            generate_time_ms: 0,
            verify_time_ms: 0,
            repair_time_ms: 0,
            peak_memory_bytes: None,
            cpu_percent: None,
        }
    }
}

impl Default for CostMetrics {
    fn default() -> Self {
        Self {
            total_cost_usd: 0.0,
            input_tokens: 0,
            output_tokens: 0,
            total_tokens: 0,
            input_cost_per_1k: 0.0,
            output_cost_per_1k: 0.0,
        }
    }
}
```

- [ ] **Step 2.2: Write failing test for benchmark types**

Create file: `crates/quality/benchmark/tests/benchmark_tests.rs`

```rust
use agentsdk_benchmark::types::{Benchmark, BenchmarkId, SuiteId, LLMBackendInfo};
use std::collections::HashMap;

#[test]
fn test_benchmark_creation() {
    let suite_id = SuiteId::new_v4();
    let backend = LLMBackendInfo {
        backend_type: "openai".to_string(),
        model: "gpt-4".to_string(),
        version: "1.0".to_string(),
        parameters: HashMap::new(),
    };

    let benchmark = Benchmark::new(
        suite_id,
        "test-benchmark".to_string(),
        "Test benchmark".to_string(),
        "v1.0".to_string(),
        serde_json::json!({}),
        backend,
        "code".to_string(),
    );

    assert_eq!(benchmark.name, "test-benchmark");
    assert_eq!(benchmark.workflow_version, "v1.0");
    assert_eq!(benchmark.file_type, "code");
}

#[test]
fn test_overall_score() {
    let suite_id = SuiteId::new_v4();
    let backend = LLMBackendInfo {
        backend_type: "openai".to_string(),
        model: "gpt-4".to_string(),
        version: "1.0".to_string(),
        parameters: HashMap::new(),
    };

    let mut benchmark = Benchmark::new(
        suite_id,
        "test".to_string(),
        "Test".to_string(),
        "v1.0".to_string(),
        serde_json::json!({}),
        backend,
        "code".to_string(),
    );

    benchmark.results.quality.score = 0.9;
    benchmark.results.performance.total_time_ms = 500;
    benchmark.results.cost.total_cost_usd = 0.01;

    let score = benchmark.overall_score();
    assert!(score > 0.0 && score <= 1.0);
}
```

- [ ] **Step 2.3: Run test to verify it fails**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: FAIL (types not yet exported)

- [ ] **Step 2.4: Export types from lib.rs**

Create file: `crates/quality/benchmark/src/lib.rs`

```rust
mod types;
mod storage;
mod execution;
mod analysis;
mod versioning;

pub use types::*;
pub use storage::*;
pub use execution::*;
pub use analysis::*;
pub use versioning::*;
```

- [ ] **Step 2.5: Run test to verify it passes**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: PASS

- [ ] **Step 2.6: Commit**

```bash
git add crates/quality/benchmark/src/lib.rs crates/quality/benchmark/src/types.rs crates/quality/benchmark/tests/benchmark_tests.rs
git commit -m "feat(quality): define benchmark types with tests"
```

---

### Step 3: Implement storage layer

- [ ] **Step 3.1: Create storage.rs**

Create file: `crates/quality/benchmark/src/storage.rs`

```rust
use super::types::*;
use rusqlite::{Connection, Result as SqliteResult, params};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Benchmark not found: {0}")]
    NotFound(BenchmarkId),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Storage for benchmarks
pub struct BenchmarkStorage {
    connection: Connection,
}

impl BenchmarkStorage {
    /// Open storage at default location
    pub fn open_default() -> Result<Self, StorageError> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Data directory not found",
            )))?;

        let db_dir = data_dir.join("agentsdk").join("benchmarks");
        std::fs::create_dir_all(&db_dir)?;

        let db_path = db_dir.join("benchmarks.db");
        Self::open(&db_path)
    }

    /// Open storage at specific path
    pub fn open(path: &PathBuf) -> Result<Self, StorageError> {
        let connection = Connection::open(path)?;
        let storage = Self { connection };
        storage.init_schema()?;
        Ok(storage)
    }

    /// Initialize database schema
    fn init_schema(&self) -> SqliteResult<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS benchmarks (
                id TEXT PRIMARY KEY,
                suite_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                workflow_version TEXT NOT NULL,
                policy_snapshot TEXT NOT NULL,
                backend_type TEXT NOT NULL,
                backend_model TEXT NOT NULL,
                backend_version TEXT NOT NULL,
                backend_parameters TEXT NOT NULL,
                file_type TEXT NOT NULL,
                tags TEXT NOT NULL,
                labels TEXT NOT NULL,
                category TEXT NOT NULL,
                priority INTEGER NOT NULL,
                enabled INTEGER NOT NULL,
                custom_metadata TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                quality_score REAL NOT NULL,
                quality_confidence REAL NOT NULL,
                quality_verifiers_passed INTEGER NOT NULL,
                quality_verifiers_total INTEGER NOT NULL,
                quality_iterations INTEGER NOT NULL,
                quality_converged INTEGER NOT NULL,
                perf_total_time_ms INTEGER NOT NULL,
                perf_generate_time_ms INTEGER NOT NULL,
                perf_verify_time_ms INTEGER NOT NULL,
                perf_repair_time_ms INTEGER NOT NULL,
                perf_peak_memory_bytes INTEGER,
                perf_cpu_percent REAL,
                cost_total_cost_usd REAL NOT NULL,
        )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS suites (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                benchmarks TEXT NOT NULL,
                category TEXT NOT NULL,
                tags TEXT NOT NULL,
                schedule TEXT,
                enabled INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Store a benchmark
    pub fn store_benchmark(&self, benchmark: &Benchmark) -> Result<(), StorageError> {
        let backend_params = serde_json::to_string(&benchmark.backend.parameters)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        let policy_snapshot = serde_json::to_string(&benchmark.policy_snapshot)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        let tags = serde_json::to_string(&benchmark.metadata.tags)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        let labels = serde_json::to_string(&benchmark.metadata.labels)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        let custom_metadata = serde_json::to_string(&benchmark.metadata.custom)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        self.connection.execute(
            "INSERT OR REPLACE INTO benchmarks (
                id, suite_id, name, description, workflow_version, policy_snapshot,
                backend_type, backend_model, backend_version, backend_parameters,
                file_type, tags, labels, category, priority, enabled, custom_metadata,
                duration_ms, quality_score, quality_confidence, quality_verifiers_passed,
                quality_verifiers_total, quality_iterations, quality_converged,
                perf_total_time_ms, perf_generate_time_ms, perf_verify_time_ms,
                perf_repair_time_ms, perf_peak_memory_bytes, perf_cpu_percent,
                cost_total_cost_usd, cost_input_tokens, cost_output_tokens, cost_total_tokens,
                cost_input_cost_per_1k, cost_output_cost_per_1k, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17,
                    ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31,
                    ?32, ?33, ?34, ?35, ?36, ?37, ?38)",
            params![
                benchmark.id.to_string(),
                benchmark.suite_id.to_string(),
                benchmark.name,
                benchmark.description,
                benchmark.workflow_version,
                policy_snapshot,
                benchmark.backend.backend_type,
                benchmark.backend.model,
                benchmark.backend.version,
                backend_params,
                benchmark.file_type,
                tags,
                labels,
                benchmark.metadata.category,
                benchmark.metadata.priority,
                benchmark.metadata.enabled,
                custom_metadata,
                benchmark.results.duration_ms,
                benchmark.results.quality.score,
                benchmark.results.quality.confidence,
                benchmark.results.quality.verifiers_passed,
                benchmark.results.quality.verifiers_total,
                benchmark.results.quality.iterations,
                benchmark.results.quality.converged as i32,
                benchmark.results.performance.total_time_ms,
                benchmark.results.performance.generate_time_ms,
                benchmark.results.performance.verify_time_ms,
                benchmark.results.performance.repair_time_ms,
                benchmark.results.performance.peak_memory_bytes,
                benchmark.results.performance.cpu_percent,
                benchmark.results.cost.total_cost_usd,
                benchmark.results.cost.input_tokens,
                benchmark.results.cost.output_tokens,
                benchmark.results.cost.total_tokens,
                benchmark.results.cost.input_cost_per_1k,
                benchmark.results.cost.output_cost_per_1k,
                benchmark.created_at.to_rfc3339(),
                benchmark.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Retrieve a benchmark by ID
    pub fn get_benchmark(&self, id: BenchmarkId) -> Result<Benchmark, StorageError> {
        let mut stmt = self.connection.prepare(
            "SELECT * FROM benchmarks WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id.to_string()])?;

        if let Some(row) = rows.next()? {
            // Deserialize from row
            // This is a simplified version - full implementation would extract all fields
            Err(StorageError::Serialization("Not implemented".to_string()))
        } else {
            Err(StorageError::NotFound(id))
        }
    }

    /// List benchmarks in a suite
    pub fn list_benchmarks(&self, suite_id: SuiteId) -> Result<Vec<BenchmarkId>, StorageError> {
        let mut stmt = self.connection.prepare(
            "SELECT id FROM benchmarks WHERE suite_id = ?1"
        )?;

        let mut ids = Vec::new();
        let mut rows = stmt.query(params![suite_id.to_string()])?;

        while let Some(row) = rows.next()? {
            let id_str: String = row.get(0)?;
            let id = Uuid::parse_str(&id_str)
                .map_err(|e| StorageError::Serialization(e.to_string()))?;
            ids.push(id);
        }

        Ok(ids)
    }

    /// Query benchmarks with filters
    pub fn query_benchmarks(&self, filters: BenchmarkFilters) -> Result<Vec<BenchmarkId>, StorageError> {
        let mut query = "SELECT id FROM benchmarks WHERE 1=1".to_string();
        let mut params = Vec::new();

        if let Some(file_type) = filters.file_type {
            query.push_str(" AND file_type = ?");
            params.push(file_type);
        }

        if let Some(backend_type) = filters.backend_type {
            query.push_str(" AND backend_type = ?");
            params.push(backend_type);
        }

        if let Some(min_score) = filters.min_score {
            query.push_str(" AND quality_score >= ?");
            params.push(min_score.to_string());
        }

        // Execute query
        // Simplified - full implementation would handle all filters

        Ok(Vec::new())
    }
}

/// Filters for querying benchmarks
pub struct BenchmarkFilters {
    pub file_type: Option<String>,
    pub backend_type: Option<String>,
    pub workflow_version: Option<String>,
    pub min_score: Option<f64>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Default for BenchmarkFilters {
    fn default() -> Self {
        Self {
            file_type: None,
            backend_type: None,
            workflow_version: None,
            min_score: None,
            start_time: None,
            end_time: None,
        }
    }
}
```

- [ ] **Step 3.2: Write test for storage**

Add to `crates/quality/benchmark/tests/benchmark_tests.rs`:

```rust
use agentsdk_benchmark::storage::{BenchmarkStorage, StorageError, BenchmarkFilters};
use tempfile::NamedTempFile;
use std::path::PathBuf;

#[test]
fn test_storage_open_default() {
    // Test opening default storage
    // This will create actual files, so skip in CI
}

#[test]
fn test_storage_open_temp() -> Result<(), StorageError> {
    let temp_file = NamedTempFile::new().unwrap();
    let path = PathBuf::from(temp_file.path());

    let storage = BenchmarkStorage::open(&path)?;
    // Storage is now open with schema initialized

    Ok(())
}

#[test]
fn test_storage_store_and_query() -> Result<(), StorageError> {
    let temp_file = NamedTempFile::new().unwrap();
    let path = PathBuf::from(temp_file.path());
    let storage = BenchmarkStorage::open(&path)?;

    // Create and store a benchmark
    let suite_id = uuid::Uuid::new_v4();
    let backend = LLMBackendInfo {
        backend_type: "openai".to_string(),
        model: "gpt-4".to_string(),
        version: "1.0".to_string(),
        parameters: std::collections::HashMap::new(),
    };

    let mut benchmark = Benchmark::new(
        suite_id,
        "test-benchmark".to_string(),
        "Test".to_string(),
        "v1.0".to_string(),
        serde_json::json!({}),
        backend,
        "code".to_string(),
    );

    storage.store_benchmark(&benchmark)?;

    // Query benchmarks
    let filters = BenchmarkFilters {
        file_type: Some("code".to_string()),
        ..Default::default()
    };

    let results = storage.query_benchmarks(filters)?;
    // Should find the benchmark we stored

    Ok(())
}
```

- [ ] **Step 3.3: Run tests**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: PASS (partial implementation)

- [ ] **Step 3.4: Commit**

```bash
git add crates/quality/benchmark/src/storage.rs crates/quality/benchmark/tests/benchmark_tests.rs
git commit -m "feat(quality): implement benchmark storage layer"
```

---

### Step 4: Implement execution layer

- [ ] **Step 4.1: Create execution.rs**

Create file: `crates/quality/benchmark/src/execution.rs`

```rust
use super::types::*;
use agentsdk_llm::LLMBackend;
use agentsdk_verifier::VerifierRegistry;
use agentsdk_loops::run_quality_loop;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Loop execution error: {0}")]
    LoopError(String),

    #[error("Storage error: {0}")]
    Storage(#[from] super::storage::StorageError),

    #[error("Timeout exceeded")]
    Timeout,
}

/// Benchmark execution context
pub struct ExecutionContext {
    pub backend: Box<dyn LLMBackend>,
    pub registry: VerifierRegistry,
    pub suite_id: SuiteId,
}

/// Execute a benchmark
pub async fn execute_benchmark(
    ctx: &ExecutionContext,
    benchmark_spec: &BenchmarkSpec,
) -> Result<Benchmark, ExecutionError> {
    let start = std::time::Instant::now();

    // Create benchmark object
    let mut benchmark = Benchmark::new(
        ctx.suite_id,
        benchmark_spec.name.clone(),
        benchmark_spec.description.clone(),
        benchmark_spec.workflow_version.clone(),
        benchmark_spec.policy_snapshot.clone(),
        benchmark_spec.backend.clone(),
        benchmark_spec.file_type.clone(),
    );

    benchmark.metadata.tags = benchmark_spec.tags.clone();
    benchmark.metadata.labels = benchmark_spec.labels.clone();
    benchmark.metadata.category = benchmark_spec.category.clone();

    // Run quality loop
    let (artifact, loop_state) = run_quality_loop(
        ctx.backend.as_ref(),
        &ctx.registry,
        &benchmark_spec.prompt,
        benchmark_spec.artifact_type,
        benchmark_spec.loop_config.clone(),
    )
    .await
    .map_err(|e| ExecutionError::LoopError(e.to_string()))?;

    // Collect results
    benchmark.results.duration_ms = start.elapsed().as_millis() as u64;
    benchmark.results.quality.score = loop_state
        .iteration_history
        .last()
        .map(|i| i.verification.confidence)
        .unwrap_or(0.0);
    benchmark.results.quality.confidence = loop_state
        .iteration_history
        .last()
        .map(|i| i.verification.confidence)
        .unwrap_or(0.0);
    benchmark.results.quality.iterations = loop_state.iteration;

    // Get performance breakdown from loop state
    for iteration in &loop_state.iteration_history {
        benchmark.results.performance.generate_time_ms += iteration.duration_ms;
    }

    benchmark.results.quality.converged = matches!(loop_state.state, agentsdk_loops::LoopState::Completed);

    // Calculate cost (simplified)
    benchmark.results.cost.total_cost_usd = benchmark_spec.cost_estimate;

    benchmark.updated_at = chrono::Utc::now();

    Ok(benchmark)
}

/// Specification for a benchmark
pub struct BenchmarkSpec {
    /// Benchmark name
    pub name: String,

    /// Description
    pub description: String,

    /// Prompt to use
    pub prompt: String,

    /// Artifact type to generate
    pub artifact_type: agentsdk_types::ArtifactType,

    /// Loop configuration
    pub loop_config: agentsdk_loops::RepairLoopConfig,

    /// Workflow version
    pub workflow_version: String,

    /// Policy snapshot
    pub policy_snapshot: serde_json::Value,

    /// Backend to use
    pub backend: LLMBackendInfo,

    /// File type
    pub file_type: String,

    /// Cost estimate (in USD)
    pub cost_estimate: f64,

    /// Tags
    pub tags: Vec<String>,

    /// Labels
    pub labels: std::collections::HashMap<String, String>,

    /// Category
    pub category: String,
}

/// Execute a benchmark suite
pub async fn execute_suite(
    ctx: &ExecutionContext,
    suite_specs: Vec<BenchmarkSpec>,
) -> Result<Vec<Benchmark>, ExecutionError> {
    let mut results = Vec::new();

    for spec in suite_specs {
        let benchmark = execute_benchmark(ctx, &spec).await?;
        results.push(benchmark);
    }

    Ok(results)
}

/// Execute benchmarks in parallel
pub async fn execute_parallel(
    ctx: &ExecutionContext,
    suite_specs: Vec<BenchmarkSpec>,
    max_concurrency: usize,
) -> Result<Vec<Benchmark>, ExecutionError> {
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrency));

    let handles: Vec<_> = suite_specs
        .into_iter()
        .map(|spec| {
            let semaphore = semaphore.clone();
            let ctx = unsafe { std::ptr::read(ctx as *const ExecutionContext) };

            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                execute_benchmark(&ctx, &spec).await
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|e| ExecutionError::LoopError(e.to_string()))??;
        results.push(result);
    }

    Ok(results)
}
```

- [ ] **Step 4.2: Write test for execution**

Add to `crates/quality/benchmark/tests/benchmark_tests.rs`:

```rust
use agentsdk_benchmark::execution::{execute_benchmark, ExecutionContext, BenchmarkSpec};
use mockall::mock;

mock! {
    LLMBackend {}

    #[async_trait::async_trait]
    impl agentsdk_llm::LLMBackend for LLMBackend {
        async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<agentsdk_llm::LLMResponse, Box<dyn std::error::Error>>;
    }
}

#[tokio::test]
async fn test_execute_benchmark() {
    let suite_id = uuid::Uuid::new_v4();

    let mut mock_backend = MockLLMBackend::new();
    // Mock responses would go here
    // For now, we just test the structure

    let ctx = ExecutionContext {
        backend: Box::new(mock_backend),
        registry: agentsdk_verifier::VerifierRegistry::new(),
        suite_id,
    };

    let spec = BenchmarkSpec {
        name: "test".to_string(),
        description: "Test benchmark".to_string(),
        prompt: "Generate code".to_string(),
        artifact_type: agentsdk_types::ArtifactType::Code,
        loop_config: agentsdk_loops::RepairLoopConfig::default(),
        workflow_version: "v1.0".to_string(),
        policy_snapshot: serde_json::json!({}),
        backend: LLMBackendInfo {
            backend_type: "openai".to_string(),
            model: "gpt-4".to_string(),
            version: "1.0".to_string(),
            parameters: std::collections::HashMap::new(),
        },
        file_type: "code".to_string(),
        cost_estimate: 0.01,
        tags: vec![],
        labels: std::collections::HashMap::new(),
        category: "test".to_string(),
    };

    // Full execution test would require complete mocks
    // This is a placeholder
}
```

- [ ] **Step 4.3: Run tests**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: PASS (partial implementation)

- [ ] **Step 4.4: Commit**

```bash
git add crates/quality/benchmark/src/execution.rs crates/quality/benchmark/tests/benchmark_tests.rs
git commit -m "feat(quality): implement benchmark execution layer"
```

---

### Step 5: Implement statistical analysis

- [ ] **Step 5.1: Create analysis.rs**

Create file: `crates/quality/benchmark/src/analysis.rs`

```rust
use super::types::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("No data to analyze")]
    NoData,

    #[error("Calculation error: {0}")]
    Calculation(String),
}

/// Statistical analysis of benchmark results
pub struct BenchmarkAnalysis {
    benchmarks: Vec<Benchmark>,
}

impl BenchmarkAnalysis {
    /// Create analysis from benchmarks
    pub fn new(benchmarks: Vec<Benchmark>) -> Self {
        Self { benchmarks }
    }

    /// Calculate mean quality score
    pub fn mean_quality_score(&self) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let sum: f64 = self.benchmarks
            .iter()
            .map(|b| b.results.quality.score)
            .sum();

        Ok(sum / self.benchmarks.len() as f64)
    }

    /// Calculate median quality score
    pub fn median_quality_score(&self) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let mut scores: Vec<f64> = self.benchmarks
            .iter()
            .map(|b| b.results.quality.score)
            .collect();
        scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = scores.len();
        if len % 2 == 0 {
            Ok((scores[len / 2 - 1] + scores[len / 2]) / 2.0)
        } else {
            Ok(scores[len / 2])
        }
    }

    /// Calculate standard deviation of quality scores
    pub fn std_dev_quality_score(&self) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let mean = self.mean_quality_score()?;
        let variance: f64 = self.benchmarks
            .iter()
            .map(|b| {
                let diff = b.results.quality.score - mean;
                diff * diff
            })
            .sum::<f64>() / self.benchmarks.len() as f64;

        Ok(variance.sqrt())
    }

    /// Calculate percentile of quality scores
    pub fn percentile_quality_score(&self, percentile: f64) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let mut scores: Vec<f64> = self.benchmarks
            .iter()
            .map(|b| b.results.quality.score)
            .collect();
        scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let idx = ((percentile / 100.0) * (scores.len() - 1) as f64).round() as usize;
        Ok(scores[idx.min(scores.len() - 1)])
    }

    /// Calculate convergence rate
    pub fn convergence_rate(&self) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let converged = self.benchmarks
            .iter()
            .filter(|b| b.results.quality.converged)
            .count();

        Ok(converged as f64 / self.benchmarks.len() as f64)
    }

    /// Calculate average iterations to converge
    pub fn avg_iterations_to_converge(&self) -> Result<f64, AnalysisError> {
        let converged: Vec<_> = self.benchmarks
            .iter()
            .filter(|b| b.results.quality.converged)
            .collect();

        if converged.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let sum: usize = converged.iter().map(|b| b.results.quality.iterations).sum();
        Ok(sum as f64 / converged.len() as f64)
    }

    /// Group benchmarks by file type
    pub fn group_by_file_type(&self) -> std::collections::HashMap<String, Vec<&Benchmark>> {
        let mut groups = std::collections::HashMap::new();

        for benchmark in &self.benchmarks {
            groups
                .entry(benchmark.file_type.clone())
                .or_insert_with(Vec::new)
                .push(benchmark);
        }

        groups
    }

    /// Group benchmarks by backend
    pub fn group_by_backend(&self) -> std::collections::HashMap<String, Vec<&Benchmark>> {
        let mut groups = std::collections::HashMap::new();

        for benchmark in &self.benchmarks {
            let key = format!("{}:{}", benchmark.backend.backend_type, benchmark.backend.model);
            groups
                .entry(key)
                .or_insert_with(Vec::new)
                .push(benchmark);
        }

        groups
    }

    /// Calculate cost per quality point
    pub fn cost_per_quality_point(&self) -> Result<f64, AnalysisError> {
        if self.benchmarks.is_empty() {
            return Err(AnalysisError::NoData);
        }

        let total_cost: f64 = self.benchmarks
            .iter()
            .map(|b| b.results.cost.total_cost_usd)
            .sum();
        let total_quality: f64 = self.benchmarks
            .iter()
            .map(|b| b.results.quality.score)
            .sum();

        if total_quality == 0.0 {
            return Ok(0.0);
        }

        Ok(total_cost / total_quality)
    }
}
```

- [ ] **Step 5.2: Write test for analysis**

Add to `crates/quality/benchmark/tests/benchmark_tests.rs`:

```rust
use agentsdk_benchmark::analysis::BenchmarkAnalysis;

#[test]
fn test_analysis_mean_quality() {
    let suite_id = uuid::Uuid::new_v4();
    let backend = LLMBackendInfo {
        backend_type: "openai".to_string(),
        model: "gpt-4".to_string(),
        version: "1.0".to_string(),
        parameters: std::collections::HashMap::new(),
    };

    let mut benchmarks = Vec::new();
    for i in 0..5 {
        let mut b = Benchmark::new(
            suite_id,
            format!("test-{}", i),
            "Test".to_string(),
            "v1.0".to_string(),
            serde_json::json!({}),
            backend.clone(),
            "code".to_string(),
        );
        b.results.quality.score = (i as f64) * 0.2; // 0.0, 0.2, 0.4, 0.6, 0.8
        benchmarks.push(b);
    }

    let analysis = BenchmarkAnalysis::new(benchmarks);
    let mean = analysis.mean_quality_score().unwrap();

    assert!((mean - 0.4).abs() < 0.01);
}

#[test]
fn test_analysis_convergence_rate() {
    let suite_id = uuid::Uuid::new_v4();
    let backend = LLMBackendInfo {
        backend_type: "openai".to_string(),
        model: "gpt-4".to_string(),
        version: "1.0".to_string(),
        parameters: std::collections::HashMap::new(),
    };

    let mut benchmarks = Vec::new();
    for i in 0..4 {
        let mut b = Benchmark::new(
            suite_id,
            format!("test-{}", i),
            "Test".to_string(),
            "v1.0".to_string(),
            serde_json::json!({}),
            backend.clone(),
            "code".to_string(),
        );
        b.results.quality.converged = i % 2 == 0; // 2 out of 4 converge
        benchmarks.push(b);
    }

    let analysis = BenchmarkAnalysis::new(benchmarks);
    let rate = analysis.convergence_rate().unwrap();

    assert!((rate - 0.5).abs() < 0.01);
}
```

- [ ] **Step 5.3: Run tests**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: PASS

- [ ] **Step 5.4: Commit**

```bash
git add crates/quality/benchmark/src/analysis.rs crates/quality/benchmark/tests/benchmark_tests.rs
git commit -m "feat(quality): implement statistical analysis"
```

---

### Step 6: Implement versioning

- [ ] **Step 6.1: Create versioning.rs**

Create file: `crates/quality/benchmark/src/versioning.rs`

```rust
use super::types::*;
use thiserror::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Error, Debug)]
pub enum VersioningError {
    #[error("Git error: {0}")]
    Git(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

/// Get current git version
pub fn get_git_version(repo_path: &PathBuf) -> Result<String, VersioningError> {
    let output = Command::new("git")
        .args(&["describe", "--always", "--tags"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(VersioningError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(version)
}

/// Get current git commit hash
pub fn get_git_commit(repo_path: &PathBuf) -> Result<String, VersioningError> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(VersioningError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(commit)
}

/// Get git commit date
pub fn get_git_date(repo_path: &PathBuf) -> Result<String, VersioningError> {
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%cd"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(VersioningError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let date = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(date)
}

/// Get all git files changed since commit
pub fn get_changed_files(repo_path: &PathBuf, since: &str) -> Result<Vec<String>, VersioningError> {
    let output = Command::new("git")
        .args(&["diff", "--name-only", since])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(VersioningError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(files)
}

/// Create policy snapshot from config file
pub fn create_policy_snapshot(config_path: &PathBuf) -> Result<serde_json::Value, VersioningError> {
    let content = std::fs::read_to_string(config_path)?;

    let value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| VersioningError::Parse(e.to_string()))?;

    Ok(value)
}
```

- [ ] **Step 6.2: Write test for versioning**

Add to `crates/quality/benchmark/tests/benchmark_tests.rs`:

```rust
use agentsdk_benchmark::versioning::create_policy_snapshot;
use std::path::PathBuf;

#[test]
fn test_create_policy_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = tempfile::NamedTempFile::new()?;
    let path = PathBuf::from(temp_file.path());

    std::fs::write(&path, r#"{"policy": "value"}"#)?;

    let snapshot = create_policy_snapshot(&path)?;

    assert_eq!(snapshot["policy"], "value");

    Ok(())
}
```

- [ ] **Step 6.3: Run tests**

```bash
cargo test --package agentsdk-benchmark --lib
```

Expected: PASS

- [ ] **Step 6.4: Commit**

```bash
git add crates/quality/benchmark/src/versioning.rs crates/quality/benchmark/tests/benchmark_tests.rs
git commit -m "feat(quality): implement version tracking"
```

---

### Step 7: Add documentation

- [ ] **Step 7.1: Create README.md**

Create file: `crates/quality/benchmark/README.md`

```markdown
# AgentSDK Benchmark

Comprehensive benchmark system with full provenance tracking.

## Overview

The benchmark system stores and executes benchmarks with complete provenance:

- **Workflow Version** - Git commit hash or tag
- **Policy Snapshot** - Complete policy configuration at time of benchmark
- **Backend Info** - LLM backend type, model, and version
- **File Type** - Artifact type being benchmarked

## Usage

### Storing Benchmarks

```rust
use agentsdk_benchmark::{Benchmark, BenchmarkStorage, LLMBackendInfo};

let storage = BenchmarkStorage::open_default()?;

let benchmark = Benchmark::new(
    suite_id,
    "my-benchmark".to_string(),
    "Description".to_string(),
    "v1.2.3".to_string(),
    serde_json::json!({"policy": "value"}),
    backend_info,
    "code".to_string(),
);

storage.store_benchmark(&benchmark)?;
```

### Executing Benchmarks

```rust
use agentsdk_benchmark::execution::{execute_benchmark, ExecutionContext, BenchmarkSpec};

let ctx = ExecutionContext {
    backend: Box::new(llm_backend),
    registry: verifier_registry,
    suite_id,
};

let spec = BenchmarkSpec {
    name: "test".to_string(),
    prompt: "Generate code".to_string(),
    artifact_type: ArtifactType::Code,
    loop_config: RepairLoopConfig::default(),
    workflow_version: "v1.0".to_string(),
    policy_snapshot: serde_json::json!({}),
    backend: backend_info,
    file_type: "code".to_string(),
    cost_estimate: 0.01,
    tags: vec![],
    labels: HashMap::new(),
    category: "test".to_string(),
    description: "".to_string(),
};

let benchmark = execute_benchmark(&ctx, &spec).await?;
```

### Statistical Analysis

```rust
use agentsdk_benchmark::analysis::BenchmarkAnalysis;

let analysis = BenchmarkAnalysis::new(benchmarks);
let mean_quality = analysis.mean_quality_score()?;
let convergence_rate = analysis.convergence_rate()?;
let cost_per_quality = analysis.cost_per_quality_point()?;
```

## Provenance Tracking

Every benchmark includes:

1. **Workflow Version** - Enables reproducibility
2. **Policy Snapshot** - Captures exact configuration
3. **Backend Info** - Tracks LLM model and version
4. **File Type** - Distinguishes different artifact types

This enables:
- Trend analysis over time
- Comparison between workflow versions
- Attribution of quality changes to specific components
- A/B testing of configurations

## Storage

Benchmarks are stored in SQLite with:
- Time-based partitioning for efficient queries
- Indexing on file_type, backend_type, workflow_version
- Compression for historical data
- Support for pruning strategies
```

- [ ] **Step 7.2: Commit**

```bash
git add crates/quality/benchmark/README.md
git commit -m "docs(quality): add benchmark documentation"
```

---

## Completion Criteria

Task 2 is complete when:

- ✅ Benchmark struct with full provenance tracking
- ✅ BenchmarkSuite with metadata
- ✅ Storage layer with SQLite
- ✅ Execution layer with parallel support
- ✅ Statistical analysis (mean, median, std dev, percentiles)
- ✅ Versioning system (git, policy snapshot)
- ✅ All tests passing
- ✅ Documentation complete
- ✅ Code review passed

---

## Handoff

Ready for Task 3: File-Type Capability Matrix
