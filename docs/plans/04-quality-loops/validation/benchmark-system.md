# Benchmark System Validation Criteria

## ADR-0005 Compliance (CRITICAL)

### Benchmark Requirements

- [ ] Benchmarks executed as part of quality loops
- [ ] Benchmark results stored in /workspace/benchmarks/ (per ADR-0005)
- [ ] Benchmarks include provenance metadata (workflow version, policy snapshot)
- [ ] Benchmarks track backend information (model, version, parameters)
- [ ] Benchmarks track file type (YAML workflow)
- [ ] Benchmarks are versioned for reproducibility
- [ ] Benchmarks support comparison to baseline
- [ ] Benchmarks enable regression detection
- [ ] Benchmarks generate actionable insights

### Integration with Quality Loops

- [ ] Quality loops trigger benchmark execution
- [ ] Benchmarks run before and after quality improvements
- [ ] Benchmark results included in quality reports
- [ ] Benchmark trends used for convergence detection
- [ ] Benchmarks validate quality loop improvements

## Provenance Tracking (CRITICAL)

### Benchmark Structure

- [ ] Benchmark.id is unique UUID
- [ ] Benchmark.suite_id references valid suite
- [ ] Benchmark.workflow_version is non-empty string
- [ ] Benchmark.policy_snapshot is valid JSON
- [ ] Benchmark.backend includes backend_type, model, version
- [ ] Benchmark.file_type is non-empty string
- [ ] Benchmark.created_at is ISO 8601 timestamp
- [ ] Benchmark.updated_at is ISO 8601 timestamp
- [ ] Benchmark.hardware_info includes CPU, GPU, RAM
- [ ] Benchmark.environment includes OS, Rust version

### Workflow Version

- [ ] workflow_version is git commit hash or tag
- [ ] versioning.rs can retrieve git version
- [ ] versioning.rs can retrieve git commit
- [ ] versioning.rs can retrieve git date
- [ ] versioning.rs can retrieve changed files
- [ ] Errors handled when git not available
- [ ] Version includes semantic version if available
- [ ] Version includes branch name if applicable

### Policy Snapshot

- [ ] policy_snapshot is complete policy configuration
- [ ] Captures all policy fields
- [ ] Can be reconstructed from stored snapshot
- [ ] Serialization/deserialization round-trips correctly
- [ ] Version tracking included in snapshot
- [ ] Snapshot includes policy schema version
- [ ] Snapshot includes checksum for integrity

### Backend Info

- [ ] backend_type identifies backend (openai, anthropic, ollama, lmstudio, etc.)
- [ ] model identifies specific model (gpt-4, claude-3, llama-3.2-3b-instruct, etc.)
- [ ] version identifies backend version or config hash
- [ ] parameters captures all backend-specific settings
- [ ] parameters includes temperature, top_p, max_tokens
- [ ] parameters includes backend-specific configs (e.g., vulkan, cuda)
- [ ] Enables reproducibility
- [ ] Backend version tracked separately from model version

### Hardware Info

- [ ] cpu_model is non-empty string
- [ ] cpu_cores is positive integer
- [ ] gpu_model is non-empty string (if GPU used)
- [ ] gpu_memory_gb is positive number (if GPU used)
- [ ] total_memory_gb is positive number
- [ ] Enables reproducibility across hardware

### Environment Info

- [ ] os_name is non-empty string (e.g., "Linux", "macOS", "Windows")
- [ ] os_version is non-empty string (e.g., "22.04", "13.5", "10")
- [ ] rust_version is semantic version (e.g., "1.75.0")
- [ ] crate_versions include major dependencies
- [ ] Enables reproducibility across environments

## Benchmark Suite Management

### Suite Structure

- [ ] Suite.id is unique UUID
- [ ] Suite.name is non-empty
- [ ] Suite.description is non-empty
- [ ] Suite.benchmark_ids is vector of UUIDs
- [ ] Suite.created_at is ISO 8601 timestamp
- [ ] Suite.updated_at is ISO 8601 timestamp
- [ ] Suite.tags is vector of strings for categorization

### Suite Curation

- [ ] Benchmarks can be added to suite
- [ ] Benchmarks can be removed from suite
- [ ] Benchmarks can be reordered within suite
- [ ] Suite can be duplicated
- [ ] Suite can be exported
- [ ] Suite can be imported
- [ ] Suite validation checks for duplicate benchmarks
- [ ] Suite validation checks for missing benchmarks

### Suite Organization

- [ ] Suites support hierarchical organization (parent-child)
- [ ] Suites can be tagged for easy discovery
- [ ] Suites can be filtered by tags
- [ ] Suites can be searched by name/description
- [ ] Suites can be shared across teams
- [ ] Suites support access control (read/write)

## Storage Validation

### Database Schema

- [ ] All required columns exist
- [ ] Primary key on id
- [ ] Foreign key on suite_id (with cascade delete)
- [ ] Indexes on file_type, backend_type, workflow_version
- [ ] Timestamp columns (created_at, updated_at)
- [ ] Checkpoint constraint for hardware_info completeness
- [ ] Unique constraint on workflow_version + backend_type + file_type

### CRUD Operations

- [ ] store_benchmark() INSERT or REPLACE
- [ ] get_benchmark() retrieves by id
- [ ] list_benchmarks() retrieves all in suite
- [ ] query_benchmarks() supports filters
- [ ] delete_benchmark() removes record and indexes
- [ ] update_benchmark() updates record and updated_at
- [ ] Operations are thread-safe
- [ ] Operations support transactions

### Query Filtering

- [ ] Supports filtering by file_type
- [ ] Supports filtering by backend_type
- [ ] Supports filtering by workflow_version
- [ ] Supports filtering by model
- [ ] Supports filtering by min_score
- [ ] Supports filtering by time range (start_time, end_time)
- [ ] Supports filtering by tags
- [ ] Multiple filters combined with AND
- [ ] Supports sorting by any field
- [ ] Supports pagination (limit, offset)

### Data Integrity

- [ ] Serde serialization round-trips correctly
- [ ] JSON stored and retrieved without corruption
- [ ] Foreign keys enforced
- [ ] Not NULL constraints enforced
- [ ] Timestamps auto-updated on update
- [ ] Checksum validation on stored data
- [ ] Data type validation on input

## Execution Validation

### Execution Context

- [ ] ExecutionContext contains valid backend
- [ ] ExecutionContext contains valid registry
- [ ] ExecutionContext contains valid suite_id
- [ ] ExecutionContext contains hardware_info
- [ ] ExecutionContext contains environment_info
- [ ] ExecutionContext contains execution_timeout
- [ ] ExecutionContext contains retry_config

### Benchmark Execution

- [ ] execute_benchmark() runs quality loop
- [ ] Captures all metrics from loop state
- [ ] Calculates quality score from verification
- [ ] Calculates cost from token usage
- [ ] Records duration_ms accurately
- [ ] Records iteration count
- [ ] Records convergence status
- [ ] Records error count and warning count

### Parallel Execution

- [ ] execute_parallel() respects max_concurrency
- [ ] Semaphore limits concurrent tasks
- [ ] Results collected in order
- [ ] Errors don't stop other benchmarks
- [ ] All benchmarks complete before returning
- [ ] Progress updates emitted during execution
- [ ] Timeout handled per benchmark

### Execution Retry Logic

- [ ] Failed benchmarks retried based on retry_config
- [ ] Retry count tracked
- [ ] Retry delay calculated based on backoff strategy
- [ ] Max retries enforced
- [ ] Retry history included in benchmark result

## Statistical Analysis

### Calculations

- [ ] mean_quality_score() accurate arithmetic
- [ ] median_quality_score() correct sort and selection
- [ ] std_dev_quality_score() correct formula
- [ ] percentile_quality_score() correct index calculation
- [ ] convergence_rate() accurate percentage
- [ ] avg_iterations_to_converge() excludes non-converged
- [ ] avg_duration_ms() accurate arithmetic
- [ ] avg_cost() accurate arithmetic
- [ ] success_rate() accurate percentage
- [ ] error_rate() accurate percentage

### Significance Testing

- [ ] t_test_two_sample() calculates p-value correctly
- [ ] z_test_two_sample() calculates p-value correctly
- [ ] Wilcoxon_signed_rank_test() for non-parametric comparison
- [ ] Mann_Whitney_u_test() for independent samples
- [ ] ANOVA() for multiple group comparison
- [ ] Chi_square_test() for categorical data
- [ ] Results include test statistic and p-value
- [ ] Significance level configurable (default: 0.05)

### Outlier Detection

- [ ] IQR method detects outliers
- [ ] Z-score method detects outliers
- [ ] Modified Z-score method detects outliers
- [ ] Outlier threshold configurable (default: 3.0)
- [ ] Outliers flagged but not removed (by default)
- [ ] Outlier removal configurable

### Grouping

- [ ] group_by_file_type() creates correct groups
- [ ] group_by_backend() includes backend_type and model
- [ ] group_by_workflow_version() groups by version ranges
- [ ] group_by_tags() groups by tags
- [ ] All benchmarks assigned to groups
- [ ] Empty groups not created
- [ ] Grouping respects filter criteria

### Trend Analysis

- [ ] linear_regression() calculates slope and intercept
- [ ] moving_average() calculates n-period moving average
- [ ] exponential_smoothing() applies smoothing factor
- [ ] trend_direction() returns increasing/decreasing/stable
- [ ] trend_strength() returns weak/moderate/strong
- [ ] Seasonality detection (for periodic data)
- [ ] Forecast generation (optional)

## Benchmark Comparison

### Baseline Comparison

- [ ] compare_to_baseline() calculates deltas
- [ ] Deltas include: quality_score, duration, cost, iterations
- [ ] Deltas include: absolute and relative change
- [ ] Deltas include: significance test results
- [ ] Comparison includes confidence intervals
- [ ] Visual comparison generated (charts)

### Regression Detection

- [ ] detect_regression() flags performance degradation
- [ ] Regression threshold configurable (default: 5%)
- [ ] Regression detection includes significance test
- [ ] False positive rate controlled (alpha configurable)
- [ ] Regression alerts emitted immediately
- [ ] Regression history tracked

### Comparison Reports

- [ ] Comparison report includes: before/after metrics
- [ ] Comparison report includes: delta calculations
- [ ] Comparison report includes: significance tests
- [ ] Comparison report includes: charts and graphs
- [ ] Comparison report includes: actionable insights
- [ ] Comparison report generated in multiple formats

## Versioning Validation

### Git Integration

- [ ] get_git_version() returns non-empty string
- [ ] get_git_commit() returns 40-character hex
- [ ] get_git_date() returns valid timestamp
- [ ] get_changed_files() returns file list
- [ ] get_branch_name() returns branch name
- [ ] get_commit_message() returns commit message
- [ ] Handles missing git directory gracefully
- [ ] Handles shallow clones gracefully
- [ ] Caches git info to avoid repeated calls

### Policy Snapshot

- [ ] create_policy_snapshot() parses file correctly
- [ ] Handles JSON syntax errors
- [ ] Returns valid Value object
- [ ] Preserves all fields
- [ ] Includes schema version
- [ ] Includes content hash for integrity

### Benchmark Versioning

- [ ] Each benchmark assigned version number
- [ ] Version numbers increment on change
- [ ] Version history maintained
- [ ] Version comparison supported
- [ ] Version rollback supported
- [ ] Version diff supported

## Mock Strategies

### Mock Metric Collection

```rust
struct MockMetricCollector {
    metrics: HashMap<String, Metric>,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: benchmark execution with mock metrics
#[test]
fn test_benchmark_with_mock_metrics() {
    let mock = MockMetricCollector {
        metrics: vec![
            ("quality_score".to_string(), Metric::new(0.92)),
            ("duration_ms".to_string(), Metric::new(5000)),
        ].into_iter().collect(),
        latency_ms: 10,
        fail_after: None,
    };

    let benchmark = execute_benchmark(&mock, &context).unwrap();
    assert_eq!(benchmark.quality_score, 0.92);
}
```

### Mock Execution Engine

```rust
struct MockExecutionEngine {
    results: Vec<BenchmarkResult>,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: benchmark suite with mock engine
#[test]
fn test_suite_with_mock_engine() {
    let mock = MockExecutionEngine {
        results: vec![
            BenchmarkResult { quality_score: 0.90, .. },
            BenchmarkResult { quality_score: 0.95, .. },
        ],
        latency_ms: 100,
        fail_after: None,
    };

    let suite = execute_suite(&mock, &suite).unwrap();
    assert_eq!(suite.benchmarks.len(), 2);
}
```

### Mock Backend

```rust
struct MockBackend {
    model: String,
    responses: Vec<String>,
    latency_ms: u64,
}

// Test: quality loop with mock backend
#[test]
fn test_quality_loop_with_mock_backend() {
    let mock = MockBackend {
        model: "mock-model".to_string(),
        responses: vec!["artifact 1".to_string(), "repaired".to_string()],
        latency_ms: 200,
    };

    let result = run_quality_loop(&mock, &config).unwrap();
    assert!(result.converged);
}
```

## Cargo Test Commands

### Unit Tests

```bash
# Run all benchmark system unit tests
cargo test --lib benchmark::tests::unit -- --nocapture

# Test benchmark creation
cargo test --lib benchmark::model::tests::create_benchmark -- --exact

# Test benchmark.overall_score() calculation
cargo test --lib benchmark::model::tests::overall_score -- --exact

# Test storage store and retrieve
cargo test --lib benchmark::storage::tests::store_and_retrieve -- --exact

# Test storage query with filters
cargo test --lib benchmark::storage::tests::query_with_filters -- --exact

# Test analysis mean, median, std dev
cargo test --lib benchmark::analysis::tests::statistical_calculations -- --exact

# Test analysis grouping operations
cargo test --lib benchmark::analysis::tests::grouping_operations -- --exact

# Test git integration
cargo test --lib benchmark::versioning::tests::git_integration -- --exact

# Expected output:
# test benchmark::model::tests::create_benchmark ... ok
# test benchmark::model::tests::overall_score ... ok
# test benchmark::storage::tests::store_and_retrieve ... ok
# test benchmark::storage::tests::query_with_filters ... ok
# test benchmark::analysis::tests::statistical_calculations ... ok
# test benchmark::analysis::tests::grouping_operations ... ok
# test benchmark::versioning::tests::git_integration ... ok
```

### Integration Tests

```bash
# Execute benchmark with real backend (mock)
cargo test --test benchmark_integration test_execute_benchmark -- --exact --nocapture

# Execute suite with multiple benchmarks
cargo test --test benchmark_integration test_execute_suite -- --exact

# Parallel execution respects concurrency
cargo test --test benchmark_integration test_parallel_execution -- --exact

# Full benchmark lifecycle (create, execute, store, query)
cargo test --test benchmark_integration test_full_lifecycle -- --exact

# Statistical analysis on real data
cargo test --test benchmark_integration test_statistical_analysis -- --exact

# Expected output:
# test benchmark_integration::test_execute_benchmark ... ok
# test benchmark_integration::test_execute_suite ... ok
# test benchmark_integration::test_parallel_execution ... ok
# test benchmark_integration::test_full_lifecycle ... ok
# test benchmark_integration::test_statistical_analysis ... ok
```

### Mock Strategy Tests

```bash
# Test with mock metric collector
cargo test --test mock_strategies test_benchmark_with_mock_metrics -- --exact

# Test with mock execution engine
cargo test --test mock_strategies test_suite_with_mock_engine -- --exact

# Test with mock backend
cargo test --test mock_strategies test_quality_loop_with_mock_backend -- --exact

# Expected output:
# test mock_strategies::test_benchmark_with_mock_metrics ... ok
# test mock_strategies::test_suite_with_mock_engine ... ok
# test mock_strategies::test_quality_loop_with_mock_backend ... ok
```

### Comparison Tests

```bash
# Test baseline comparison
cargo test --test comparison test_baseline_comparison -- --exact --nocapture

# Test regression detection
cargo test --test comparison test_regression_detection -- --exact

# Test comparison report generation
cargo test --test comparison test_comparison_report -- --exact

# Expected output:
# test comparison::test_baseline_comparison ... ok
# test comparison::test_regression_detection ... ok
# test comparison::test_comparison_report ... ok
```

### Versioning Tests

```bash
# Test git version retrieval
cargo test --test versioning test_git_version -- --exact

# Test policy snapshot creation
cargo test --test versioning test_policy_snapshot -- --exact

# Test benchmark versioning
cargo test --test versioning test_benchmark_versioning -- --exact

# Expected output:
# test versioning::test_git_version ... ok
# test versioning::test_policy_snapshot ... ok
# test versioning::test_benchmark_versioning ... ok
```

## Log Verification Patterns

### Benchmark Execution Log Pattern

```json
{
  "level": "info",
  "target": "benchmark::executor",
  "event": "benchmark_execution_start",
  "benchmark_id": "<uuid>",
  "suite_id": "<uuid>",
  "workflow_version": "abc123",
  "backend": {
    "backend_type": "ollama",
    "model": "llama-3.2-3b-instruct",
    "version": "1.0.0"
  }
}
```

**Verification**: `grep '"event":"benchmark_execution_start"' logs/benchmark.log | jq '.backend.model == "llama-3.2-3b-instruct"' | grep true`

### Benchmark Completion Log Pattern

```json
{
  "level": "info",
  "target": "benchmark::executor",
  "event": "benchmark_execution_complete",
  "benchmark_id": "<uuid>",
  "quality_score": 0.92,
  "duration_ms": 5000,
  "iterations": 3,
  "converged": true,
  "cost_estimate": 0.05
}
```

**Verification**: `grep '"event":"benchmark_execution_complete"' logs/benchmark.log | jq '.quality_score | tonumber >= 0.9' | grep true`

### Quality Loop Execution Log Pattern

```json
{
  "level": "debug",
  "target": "benchmark::quality_loop",
  "event": "quality_loop_iteration",
  "benchmark_id": "<uuid>",
  "iteration": 1,
  "step": "generate",
  "duration_ms": 1200,
  "tokens": 500
}
```

**Verification**: `grep '"event":"quality_loop_iteration"' logs/benchmark.log | jq '.iteration | tonumber >= 1' | grep true`

### Metric Collection Log Pattern

```json
{
  "level": "debug",
  "target": "benchmark::metrics",
  "event": "metric_collected",
  "benchmark_id": "<uuid>",
  "metric_name": "quality_score",
  "metric_value": 0.92,
  "unit": "score",
  "timestamp": "2026-04-07T12:00:00Z"
}
```

**Verification**: `grep '"event":"metric_collected"' logs/benchmark.log | jq '.metric_name == "quality_score"' | grep true`

### Statistical Analysis Log Pattern

```json
{
  "level": "info",
  "target": "benchmark::analysis",
  "event": "statistical_analysis_complete",
  "suite_id": "<uuid>",
  "statistics": {
    "mean_quality_score": 0.91,
    "median_quality_score": 0.92,
    "std_dev_quality_score": 0.02,
    "p95_quality_score": 0.95,
    "p99_quality_score": 0.97
  },
  "benchmark_count": 100
}
```

**Verification**: `grep '"event":"statistical_analysis_complete"' logs/benchmark.log | jq '.statistics.mean_quality_score | tonumber >= 0.9' | grep true`

### Baseline Comparison Log Pattern

```json
{
  "level": "info",
  "target": "benchmark::comparison",
  "event": "baseline_comparison_complete",
  "benchmark_id": "<uuid>",
  "baseline_id": "<uuid>",
  "deltas": {
    "quality_score": {
      "absolute": 0.05,
      "relative": 0.0575,
      "significant": true,
      "p_value": 0.02
    }
  }
}
```

**Verification**: `grep '"event":"baseline_comparison_complete"' logs/benchmark.log | jq '.deltas.quality_score.significant == true' | grep true`

### Regression Detection Log Pattern

```json
{
  "level": "warn",
  "target": "benchmark::comparison",
  "event": "regression_detected",
  "benchmark_id": "<uuid>",
  "baseline_id": "<uuid>",
  "metric": "quality_score",
  "baseline_value": 0.95,
  "current_value": 0.89,
  "delta_percent": -6.32,
  "threshold_percent": -5.0,
  "p_value": 0.03
}
```

**Verification**: `grep '"event":"regression_detected"' logs/benchmark.log | jq '.delta_percent | tonumber < -5.0' | grep true`

### Storage Log Pattern

```json
{
  "level": "debug",
  "target": "benchmark::storage",
  "event": "benchmark_stored",
  "benchmark_id": "<uuid>",
  "storage_path": "/workspace/benchmarks/benchmark_abc123.json",
  "file_size_bytes": 10240,
  "duration_ms": 25
}
```

**Verification**: `grep '"event":"benchmark_stored"' logs/benchmark.log | jq '.file_size_bytes | tonumber > 0' | grep true`

## Testing Criteria

### Unit Tests

- [ ] Benchmark creation with all fields
- [ ] Benchmark.overall_score() calculation
- [ ] Storage store and retrieve
- [ ] Storage query with filters
- [ ] Analysis mean, median, std dev
- [ ] Analysis grouping operations
- [ ] Significance testing (t-test, z-test, ANOVA)
- [ ] Outlier detection methods
- [ ] Trend analysis methods
- [ ] Git integration
- [ ] Policy snapshot creation
- [ ] Benchmark versioning

### Integration Tests

- [ ] Execute benchmark with real backend (mock)
- [ ] Execute suite with multiple benchmarks
- [ ] Parallel execution respects concurrency
- [ ] Full benchmark lifecycle (create, execute, store, query)
- [ ] Statistical analysis on real data
- [ ] Baseline comparison generation
- [ ] Regression detection
- [ ] Comparison report generation
- [ ] Suite management (create, add, remove, export)
- [ ] Suite filtering and searching

### Edge Cases

- [ ] Empty benchmark set
- [ ] Single benchmark
- [ ] Very large benchmark set (1000+)
- [ ] Missing workflow version
- [ ] Invalid policy snapshot
- [ ] Concurrent storage operations
- [ ] Disk full during storage
- [ ] Network timeout during execution
- [ ] Missing git directory
- [ ] Insufficient data for statistical tests
- [ ] All benchmarks have same value (zero variance)
- [ ] Extreme outliers (invalid data)

## Performance Criteria

- [ ] store_benchmark() < 100ms
- [ ] get_benchmark() < 50ms
- [ ] query_benchmarks() < 200ms (typical filters)
- [ ] execute_benchmark() completes in reasonable time
- [ ] Statistical analysis on 1000 benchmarks < 100ms
- [ ] Storage memory usage < 10MB per 1000 benchmarks
- [ ] Parallel execution respects concurrency limits
- [ ] Comparison generation < 500ms (typical)
- [ ] Regression detection < 200ms
- [ ] Suite export < 1s (100 benchmarks)

## Error Handling

- [ ] StorageError covers all failure modes
- [ ] ExecutionError covers all failure modes
- [ ] AnalysisError covers all failure modes
- [ ] VersioningError covers all failure modes
- [ ] Error messages actionable
- [ ] Error includes context (id, file path, etc.)
- [ ] Errors propagate correctly to callers
- [ ] Partial results returned where possible
- [ ] Errors logged with stack traces
- [ ] Error recovery strategies documented

## ADR-0005 Constraint Compliance Summary

| Constraint | Validation | Test Coverage |
|------------|-------------|---------------|
| Provenance tracking | Benchmark includes all metadata fields | model::tests::provenance_fields |
| /workspace/benchmarks/ storage | Benchmark path validation | storage::tests::benchmark_path |
| Workflow version tracked | Git integration tested | versioning::tests::git_integration |
| Policy snapshot included | Snapshot creation validated | versioning::tests::policy_snapshot |
| Baseline comparison supported | Comparison tests pass | comparison::tests::baseline_comparison |
| Regression detection enabled | Regression tests pass | comparison::tests::regression_detection |
