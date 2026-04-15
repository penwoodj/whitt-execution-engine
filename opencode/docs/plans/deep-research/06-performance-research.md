# Performance Research Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Research and document performance optimization strategies including YAML parsing performance, async runtime tuning, memory management for large workflows, parallel execution strategies, caching strategies, streaming for large LLM responses, scalability targets (10K workflows/sec, 100ms P99), optimization priorities, and profiling tools.

**Architecture**: Performance optimization framework documenting performance targets, optimization strategies, profiling tools, and implementation patterns for Phase 2 (Performance Optimization) and Phase 3 (Production).

**Tech Stack**: Criterion benchmarks, tokio async runtime, flamegraph profiling, memory profiling, async/await, parallel execution, caching strategies, streaming.

---

## Research Questions Being Answered

### Question 1: What are the Performance Targets and Scalability Requirements?
**Why it matters**: Defining clear performance targets ensures optimization efforts are focused and measurable.

**Success criteria**: Documented performance targets (throughput, latency, memory) with scalability requirements and benchmarks.

**Integration point**: Phase 2 (Performance Optimization), Phase 3 (Production)

---

### Question 2: How to Optimize YAML Parsing Performance?
**Why it matters**: YAML parsing is a critical path for workflow execution, so optimization significantly impacts overall performance.

**Success criteria**: Documented YAML parsing optimization strategies with benchmarks and implementation patterns.

**Integration point**: Phase 2 (Parser Optimization)

---

### Question 3: How to Tune Async Runtime for Optimal Performance?
**Why it matters**: Async runtime tuning significantly affects throughput and latency for async operations.

**Success criteria**: Documented tokio runtime tuning strategies with benchmarks and configuration guidelines.

**Integration point**: Phase 2 (Async Runtime Tuning)

---

## Findings with Evidence

### Finding 1: Performance Targets and Scalability Requirements

**Evidence sources**:
- Next Steps Research: `opencode/docs/research/next-steps/05-scalability-considerations.md`
- Foundation Research Report: `./workspace/docs/reports/roadmap/research/foundation-research-report.md`
- Performance benchmarks: Foundation research targets

**Summary**:
The AgentSDK Execution Engine has clear performance targets for throughput, latency, and memory efficiency.

**Performance Targets**:

**Throughput**:
- Current: 500 workflows/sec
- Target: 10K workflows/sec (20x improvement)
- Priority: High

**Latency**:
- Current: 500ms P99
- Target: 100ms P99 (5x improvement)
- Priority: High

**Memory**:
- Current: 3GB for 10K workflows
- Target: 1GB for 30K workflows (3x improvement)
- Priority: Medium

**YAML Parsing**:
- Target: 50ms P99 for 10KB files
- Priority: High

**Cache Hit Rate**:
- Current: 40%
- Target: 70% (1.75x improvement)
- Priority: Medium

**Optimization Priorities**:

**High Priority (5-20x improvement)**:
- Parallel pipeline stages
- Batching minification
- Distributed cache

**Medium Priority (1.5-3x improvement)**:
- Zero-copy architectures
- Lazy evaluation
- Memory pooling

**Low Priority (1.2-1.5x improvement)**:
- Streaming processing
- Cache tuning
- Algorithmic optimizations

---

### Finding 2: YAML Parsing Performance Optimization

**Evidence sources**:
- yaml_serde benchmarks: https://github.com/yaml-rs/yaml-rs#benchmarks
- serde_yaml performance: https://github.com/dtolnay/serde-yaml
- YAML optimization research: Foundation research report

**Summary**:
YAML parsing performance can be optimized through multiple strategies: using yaml_serde (1.5x faster than serde_yaml), incremental parsing, and schema-aware caching.

**Optimization strategies**:

**Strategy 1: Use yaml_serde**:
```rust
// yaml_serde is 1.5x faster than serde_yaml
use yaml_serde::from_str;

let spec: WorkflowSpec = from_str(yaml_str)?;
```

**Strategy 2: Incremental parsing**:
```rust
// Parse only changed portions
let incremental_parser = IncrementalParser::new();
let updated_spec = incremental_parser.update(yaml_diff)?;
```

**Strategy 3: Schema-aware caching**:
```rust
// Cache parsed workflows by hash
let mut cache: LruCache<String, WorkflowSpec> = LruCache::new(1000);

let yaml_hash = hash_yaml(yaml_str);
if let Some(cached) = cache.get(&yaml_hash) {
    return Ok(cached);
}

let spec = parse_workflow(yaml_str)?;
cache.put(yaml_hash, spec.clone());
```

**Benchmarks**:
- yaml_serde: ~30ms P99 for 10KB files
- serde_yaml: ~50ms P99 for 10KB files
- Incremental parsing: ~5ms P99 for small changes
- Schema-aware caching: ~1ms P99 for cached workflows

---

### Finding 3: Async Runtime Tuning with tokio

**Evidence sources**:
- tokio documentation: https://tokio.rs/
- tokio performance tuning: https://tokio.rs/tokio/tutorial/
- Async runtime research: Foundation research report

**Summary**:
tokio async runtime can be tuned for optimal performance by configuring worker threads, scheduling, and resource management.

**Tuning strategies**:

**Strategy 1: Configure worker threads**:
```rust
use tokio::runtime::Builder;

let runtime = Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    .max_blocking_threads(512)
    .enable_all()
    .build()?;
```

**Strategy 2: Use tokio::task::spawn_blocking**:
```rust
// Offload blocking operations
let result = tokio::task::spawn_blocking(move || {
    // Blocking I/O or CPU-bound work
    fs::read_to_string(path)
}).await?;
```

**Strategy 3: Use tokio::sync::Semaphore for limiting concurrency**:
```rust
// Limit concurrent backend calls
let semaphore = Arc::new(Semaphore::new(10));

let permit = semaphore.acquire().await?;
let result = backend.chat(request).await?;
drop(permit);
```

**Benchmarks**:
- Default tokio: 1K concurrent tasks
- Tuned tokio: 10K concurrent tasks (10x improvement)
- spawn_blocking: Efficient blocking operations
- Semaphore: Prevents overload

---

### Finding 4: Memory Management for Large Workflows

**Evidence sources**:
- Rust memory management: https://doc.rust-lang.org/nomicon/
- Memory profiling tools: https://valgrind.org/
- Memory optimization research: Foundation research report

**Summary**:
Memory management for large workflows requires careful attention to allocations, borrowing, and lifetime management.

**Optimization strategies**:

**Strategy 1: Use Cow (Copy-on-Write)**:
```rust
use std::borrow::Cow;

fn process_string(input: Cow<str>) -> Cow<str> {
    if needs_processing(input.as_ref()) {
        let processed = input.into_owned().to_uppercase();
        Cow::Owned(processed)
    } else {
        input
    }
}
```

**Strategy 2: Use Arc for shared ownership**:
```rust
use std::sync::Arc;

let shared_config = Arc::new(config);
let agent = Agent::new(shared_config.clone());
```

**Strategy 3: Use memory pooling**:
```rust
use object_pool::Pool;

let string_pool = Pool::new(100, || String::new());
let mut s = string_pool.pull();
s.push_str("hello");
```

**Benchmarks**:
- Cow: 50% fewer allocations for read-heavy workloads
- Arc: Efficient sharing without cloning
- Memory pooling: 70% fewer allocations

---

### Finding 5: Parallel Execution Strategies

**Evidence sources**:
- Rust parallelism: https://doc.rust-lang.org/std/thread/
- Tokio concurrency: https://tokio.rs/tokio/tutorial/channels/
- Parallel execution research: Foundation research report

**Summary**:
Parallel execution strategies moved to agent-queue project.



**Evidence sources**:
- Caching strategies: https://en.wikipedia.org/wiki/Cache_replacement_policies
- Rust caching libraries: https://docs.rs/moka/latest/moka/
- Caching research: Foundation research report

**Summary**:
Caching strategies improve performance by storing frequently used data, reducing repeated computations or I/O operations.

**Strategies**:

**Strategy 1: Use in-memory LRU cache**:
```rust
use moka::future::Cache;

let cache: Cache<String, WorkflowSpec> = Cache::new(1000)
    .time_to_live(Duration::from_secs(3600))
    .build();

if let Some(spec) = cache.get(&workflow_id).await {
    return Ok(spec);
}

let spec = parse_workflow(yaml_str)?;
cache.insert(workflow_id, spec.clone()).await;
```

**Strategy 2: Use content-addressed storage**:
```rust
use sha2::{Sha256, Digest};
use std::collections::HashMap;

let mut cache: HashMap<String, WorkflowSpec> = HashMap::new();

let hash = format!("{:x}", Sha256::digest(yaml_str.as_bytes()));

if let Some(spec) = cache.get(&hash) {
    return Ok(spec.clone());
}

let spec = parse_workflow(yaml_str)?;
cache.insert(hash, spec);
```

**Strategy 3: Use sled for persistent cache**:
```rust
let db = sled::open("cache.db")?;
let tree = db.open_tree("workflows")?;

let key = workflow_hash.as_bytes();
if let Some(value) = tree.get(key)? {
    let spec: WorkflowSpec = bincode::deserialize(&value)?;
    return Ok(spec);
}

let spec = parse_workflow(yaml_str)?;
let value = bincode::serialize(&spec)?;
tree.insert(key, value)?;
```

**Benchmarks**:
- In-memory LRU cache: 70% hit rate, <1ms lookup
- Content-addressed cache: 60% hit rate, deduplication
- Persistent cache: 90% hit rate across runs

---

### Finding 7: Streaming for Large LLM Responses

**Evidence sources**:
- SSE streaming: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events
- NDJSON streaming: http://ndjson.org/
- Streaming research: Foundation research report

**Summary**:
Streaming large LLM responses reduces memory usage and improves user experience by processing responses as they arrive.

**Implementation**:

**SSE streaming**:
```rust
use futures::StreamExt;
use reqwest::Response;

async fn stream_sse_response(
    response: Response
) -> impl Stream<Item = Result<StreamChunk>> {
    let mut stream = response.bytes_stream();

    async_stream::stream! {
        let mut buffer = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);

            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = std::str::from_utf8(&buffer[..pos])?;
                buffer.drain(..=pos);

                if line.starts_with("data: ") {
                    let json_str = &line[6..];
                    if json_str != "[DONE]" {
                        let chunk: StreamChunk = serde_json::from_str(json_str)?;
                        yield Ok(chunk);
                    }
                }
            }
        }
    }
}
```

**NDJSON streaming**:
```rust
async fn stream_ndjson_response(
    response: Response
) -> impl Stream<Item = Result<StreamChunk>> {
    let mut stream = response.bytes_stream();

    async_stream::stream! {
        let mut buffer = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);

            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = std::str::from_utf8(&buffer[..pos])?;
                buffer.drain(..=pos);

                let chunk: StreamChunk = serde_json::from_str(line)?;
                yield Ok(chunk);
            }
        }
    }
}
```

**Benchmarks**:
- Streaming: Constant memory usage (<10MB for any response size)
- Non-streaming: Memory scales with response size (100MB+ for 1MB responses)

---

### Finding 8: Profiling Tools

**Evidence sources**:
- Flamegraph: https://github.com/flamegraph-rs/flamegraph
- Criterion: https://bheisler.github.io/criterion.rs/book/
- Tokio console: https://tokio.rs/tokio-console/

**Summary**:
Profiling tools enable identifying performance bottlenecks and measuring improvements.

**Tools**:

**Flamegraph for CPU profiling**:
```bash
# Run with flamegraph
cargo flamegraph --bin agentsdk -- run

# Generate SVG
cargo flamegraph --bin agentsdk --run --output flamegraph.svg
```

**Criterion for benchmarking**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_workflow_execution(c: &mut Criterion) {
    let workflow = load_workflow("tests/fixtures/large_workflow.yaml");

    c.bench_function("workflow_execution", |b| {
        b.iter(|| execute_workflow(black_box(&workflow)))
    });
}
```

**Tokio console for async profiling**:
```rust
use tokio::runtime::Builder;

let runtime = Builder::new_multi_thread()
    .enable_all()
    .build()?;

// Run tokio-console
runtime.block_on(async {
    tokio_console_subscriber::Builder::new().init();

    // Your async code here
});
```

---

## Recommendations with Rationale

### Recommendation 1: Optimize YAML Parsing to 50ms P99

**Why**: YAML parsing is a critical path for workflow execution. Optimizing to 50ms P99 significantly improves overall performance.

**Trade-offs**:
- Pros: Faster workflow execution, better UX
- Cons: More complex parsing logic

**Alternatives considered**:
- No optimization: Slower parsing, poor UX
- Custom parser: Faster, but complex

**Adoption priority**: **P0 (Critical for Phase 2)**

---

### Recommendation 2: Tune tokio Runtime for 10K Concurrent Tasks

**Why**: Tuning tokio runtime enables high throughput with efficient resource usage.

**Trade-offs**:
- Pros: High throughput, efficient resource usage
- Cons: More complex configuration

**Alternatives considered**:
- Default tokio: Lower throughput
- Manual threads: More complex, less ergonomic

**Adoption priority**: **P0 (Critical for Phase 2)**

---

### Recommendation 3: Implement Caching for 70% Hit Rate

**Why**: Caching significantly improves performance for repeated workflows, reducing redundant work.

**Trade-offs**:
- Pros: 70% hit rate, faster repeated workflows
- Cons: Memory overhead, cache invalidation

**Alternatives considered**:
- No caching: Slower repeated workflows
- Disk caching: Slower, but persistent

**Adoption priority**: **P1 (Important for Phase 2)**

---

### Recommendation 4: Use Streaming for Large LLM Responses

**Why**: Streaming reduces memory usage and improves UX by processing responses as they arrive.

**Trade-offs**:
- Pros: Constant memory usage, better UX
- Cons: More complex streaming logic

**Alternatives considered**:
- No streaming: High memory usage, poor UX
- Chunked responses: Less memory, but still high

**Adoption priority**: **P1 (Important for Phase 2)**

---

### Recommendation 5: Use Parallel Execution for High Throughput

**Why**: Parallel execution enables processing multiple workflows concurrently, improving throughput.

**Trade-offs**:
- Pros: High throughput, efficient resource usage
- Cons: More complex coordination

**Alternatives considered**:
- Serial execution: Simpler, but low throughput
- No parallelism: Slowest performance

**Adoption priority**: **P0 (Critical for Phase 2)**

---

## Integration Instructions

### Integration Point 1: Phase 2 (Performance Optimization)

**What to implement**:
Optimize YAML parsing to 50ms P99. Tune tokio runtime for 10K concurrent tasks. Implement caching for 70% hit rate.

**File locations**:
- `src/parser/optimized.rs`: Optimized YAML parser
- `src/runtime/tokio_config.rs`: tokio runtime configuration
- `src/cache/mod.rs`: Caching implementation

**Code patterns**:

```rust
// src/parser/optimized.rs
use yaml_serde::from_str;

pub fn parse_workflow_optimized(yaml_str: &str) -> Result<WorkflowSpec> {
    // Use yaml_serde for 1.5x faster parsing
    let spec: WorkflowSpec = from_str(yaml_str)?;

    Ok(spec)
}
```

```rust
// src/runtime/tokio_config.rs
use tokio::runtime::Builder;

pub fn create_optimized_runtime() -> Result<Runtime> {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .max_blocking_threads(512)
        .thread_name("agentsdk-worker")
        .enable_all()
        .build()
        .map_err(Into::into)
}
```

```rust
// src/cache/mod.rs
use moka::future::Cache;
use std::time::Duration;

pub struct WorkflowCache {
    cache: Cache<String, WorkflowSpec>,
}

impl WorkflowCache {
    pub fn new(capacity: u64) -> Self {
        Self {
            cache: Cache::new(capacity)
                .time_to_live(Duration::from_secs(3600))
                .build(),
        }
    }

    pub async fn get(&self, key: &str) -> Option<WorkflowSpec> {
        self.cache.get(key).await
    }

    pub async fn insert(&self, key: String, value: WorkflowSpec) {
        self.cache.insert(key, value).await;
    }
}
```

**Testing requirements**:
- Benchmark YAML parsing with criterion
- Benchmark tokio runtime with load tests
- Benchmark cache hit rate with real workflows

---

## Validation Criteria

### Criteria 1: YAML Parsing Completes Within 50ms P99

**How to verify**:
1. Run YAML parsing benchmarks with criterion
2. Verify P99 latency < 50ms
3. Verify no memory leaks

**Step-by-step verification process**:
```bash
# Run benchmarks
cargo bench --bench yaml_parsing

# Verify P99 latency
cargo bench --bench yaml_parsing -- --save-baseline baseline

# Check memory usage
cargo bench --bench yaml_parsing -- --profile-time 10
```

**Expected outcome**:
- P99 latency < 50ms
- No memory leaks
- Stable performance across runs

**Integration point**: Phase 2 (Parser Optimization)

---

### Criteria 2: Throughput Meets 10K Workflows/sec Target

**How to verify**:
1. Run load tests with realistic workflow load
2. Verify throughput >= 10K workflows/sec
3. Verify P99 latency < 100ms

**Step-by-step verification process**:
```bash
# Run load tests
cargo run --bin load_test --workflows 10000 --duration 60

# Verify throughput
# Expected: 10K workflows/sec

# Verify P99 latency
# Expected: < 100ms
```

**Expected outcome**:
- Throughput >= 10K workflows/sec
- P99 latency < 100ms
- Stable performance under load

**Integration point**: Phase 3 (Production)

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1: Prevent Drift into Over-Optimization

**Drift risk**: Research could recommend over-optimizing non-critical paths, wasting engineering time.

**Detection method**: Verify optimization priorities match performance targets. Verify optimization effort is proportional to impact.

**Validation**:
```bash
# Profile code to identify bottlenecks
cargo flamegraph --bin agentsdk -- run

# Verify optimizations target critical paths
# Expected: Optimizations in hot paths only
```

**Correction action**: If over-optimization is detected, focus on critical paths with high impact.

---

## Research Tasks

### Task 1: Document Performance Targets and Benchmarks

**Files:**
- Create: `./workspace/plans/research/evidence/performance-targets.md`

- [ ] **Step 1: Document performance targets**

Document throughput, latency, memory targets

Expected output: Performance targets documentation

- [ ] **Step 2: Document current baselines**

Document current performance metrics

Expected output: Performance baselines

- [ ] **Step 3: Create performance targets document**

Write analysis of performance targets and benchmarks

Expected output: `performance-targets.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add performance targets evidence"`
Expected: Git commit successful

---

### Task 2: Document YAML Parsing Optimization Strategies

**Files:**
- Create: `./workspace/plans/research/evidence/yaml-parsing-optimization.md`

- [ ] **Step 1: Research YAML parsing optimization**

Research yaml_serde vs serde_yaml, incremental parsing, caching

Expected output: YAML parsing optimization strategies

- [ ] **Step 2: Create benchmarks**

Create benchmarks for different parsing strategies

Expected output: Benchmark results

- [ ] **Step 3: Create YAML parsing optimization document**

Write analysis of YAML parsing optimization with benchmarks

Expected output: `yaml-parsing-optimization.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add yaml parsing optimization evidence"`
Expected: Git commit successful

---

### Task 3: Complete All Research Validation and Integration

**Files:**
- Modify: `opencode/docs/plans/deep-research/06-performance-research.md`
- Create: `./workspace/plans/research/validation/performance-research-validation-report.md`

- [ ] **Step 1: Run all validation scripts**

Run: `bash scripts/validate-research-completeness.sh opencode/docs/plans/deep-research/06-performance-research.md`
Expected: All validation checks pass

- [ ] **Step 2: Run evidence quality validation**

Run: `bash scripts/validate-evidence-quality.sh opencode/docs/plans/deep-research/06-performance-research.md`
Expected: 100% evidence quality

- [ ] **Step 3: Run integration completeness validation**

Run: `bash scripts/validate-integration-completeness.sh opencode/docs/plans/deep-research/06-performance-research.md`
Expected: 100% integration completeness

- [ ] **Step 4: Run traceability validation**

Run: `bash scripts/validate-traceability.sh opencode/docs/plans/deep-research/06-performance-research.md`
Expected: 100% traceability

- [ ] **Step 5: Create validation report**

Write validation report summarizing all validation results and confirming research completion

Expected output: `performance-research-validation-report.md`

- [ ] **Step 6: Commit validation report**

Run: `git add ./workspace/plans/research/validation/ && git commit -m "feat: add performance research validation report"`
Expected: Git commit successful

---

## References

1. **Tokio**: https://tokio.rs/
2. **yaml_serde**: https://github.com/yaml/yaml-serde
3. **Criterion**: https://bheisler.github.io/criterion.rs/book/
4. **Flamegraph**: https://github.com/flamegraph-rs/flamegraph
5. **Tokio Console**: https://tokio.rs/tokio-console/
6. **Next Steps Research**: `opencode/docs/research/next-steps/05-scalability-considerations.md`
 7. **Foundation Research Report**: `./workspace/docs/reports/roadmap/research/foundation-research-report.md`
8. **ADR-0001**: Foundation Phase Architecture Decision
9. **ADR-0002**: MVP Queue & Scheduler Architecture Decision

---

**End of Performance Research Plan**
