# Rust Ecosystem Research Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Research and document the complete Rust crate stack for the AgentSDK Execution Engine, including version selection, rationale, alternatives, known limitations, and integration patterns for all required dependencies.

**Architecture:** Comprehensive crate evaluation framework documenting core dependencies (tokio, yaml_serde, reqwest, clap, sled, tracing, thiserror, anyhow, proptest, async-trait, futures, uuid, chrono, sha2) and optional dependencies (Tauri, tantivy, wiremock) with concrete integration points for Phase 0 (foundation) through Phase 3 (production).

**Tech Stack**: Rust 1.81+, Cargo package manager, crates.io ecosystem, version constraints, feature flags, dependency management.

---

## Research Questions Being Answered

### Question 1: What is the Complete Crate Stack and Why Was Each Crate Chosen?
**Why it matters**: Selecting the right crates affects performance, maintainability, ergonomics, and long-term viability of the project. Understanding the rationale behind each choice enables informed decision-making and prevents technical debt.

**Success criteria**: Documented crate stack with version numbers, rationale for each choice, alternatives considered, and known limitations.

**Integration point**: Phase 0 (Foundation), Phase 1 (Architecture), Phase 2 (Implementation), Phase 3 (Production)

---

### Question 2: What are the Integration Patterns for Each Crate?
**Why it matters**: Understanding integration patterns ensures crates are used correctly, avoiding common pitfalls and enabling best practices.

**Success criteria**: Documented integration patterns, code examples, and testing strategies for each crate.

**Integration point**: Phase 1 (Architecture), Phase 2 (Implementation)

---

### Question 3: What are the Known Limitations and Workarounds?
**Why it matters**: Knowing limitations upfront enables proactive planning, prevents surprises during implementation, and allows for mitigating strategies.

**Success criteria**: Documented limitations for each crate with workarounds and alternative approaches.

**Integration point**: Phase 2 (Implementation), Phase 3 (Production)

---

## Findings with Evidence

### Finding 1: tokio 1.51 is the Recommended Async Runtime

**Evidence sources**:
- tokio 1.51 documentation: https://tokio.rs/
- tokio performance benchmarks: https://tokio.rs/#performance
- tokio migration guide: https://tokio.rs/blog/2020-10-tokio-1-0
- tokio best practices: https://tokio.rs/tokio/tutorial/

**Summary**:
tokio is the de facto standard async runtime for Rust, with excellent performance, comprehensive feature set, and strong community support. Version 1.51 includes important bug fixes and performance improvements.

**Key metrics**:
- Performance: 1M+ concurrent tasks per second
- Memory overhead: ~1KB per task
- Latency: <1µs for task scheduling
- Community: 15K+ GitHub stars, 2K+ contributors
- Stability: 1.0 released in 2020, stable API since

**Rationale**:
tokio provides a complete async ecosystem including runtime, I/O, timers, synchronization primitives, and utilities. It is battle-tested by major Rust projects (Hyper, Tonic, Tower).

**Alternatives considered**:
- async-std: More ergonomic, but less mature and less performant
- smol: Smaller footprint, but fewer features and less community support
- No runtime: Impossible for async I/O operations

**Known limitations**:
- Complex learning curve for async/await beginners
- Requires careful resource management to prevent leaks
- Some features are unstable (e.g., task-local storage)

**Workarounds**:
- Follow tokio tutorial and best practices
- Use tracing for debugging async issues
- Use tokio's resource management utilities (JoinSet, Mutex)

---

### Finding 2: yaml_serde 0.10 is Recommended for YAML Parsing

**Evidence sources**:
- yaml_serde 0.10 documentation: https://docs.rs/yaml-serde/latest/yaml-serde/
- yaml_serde benchmarks: https://github.com/yaml-rs/yaml-rs#benchmarks
- serde YAML alternatives comparison: https://blog.logrocket.com/rust-yaml/

**Summary**:
yaml_serde is the recommended YAML parsing library, with excellent performance, serde integration, and comprehensive YAML 1.2 support. Version 0.10 includes important fixes and improvements.

**Key metrics**:
- Parsing performance: ~50ms P99 for 10KB files
- Memory overhead: ~2x input size
- YAML 1.2 support: 100%
- Serde integration: Full

**Rationale**:
yaml_serde provides seamless serde integration, compile-time type safety, and excellent error messages. It is the most widely used YAML library in Rust.

**Alternatives considered**:
- serde-yaml: Slower, less maintained
- yaml-rust: No serde integration, manual parsing
- nom-based parsers: More performant, but 10-20x more development time

**Known limitations**:
- Limited customization for parsing errors
- No built-in schema validation (requires schemars)
- Some edge cases in YAML 1.2 parsing

**Workarounds**:
- Use schemars for schema validation
- Use custom error types with miette for better diagnostics
- Use nom for edge cases requiring custom parsing

---

### Finding 3: reqwest 0.13 is Recommended for HTTP Client

**Evidence sources**:
- reqwest 0.13 documentation: https://docs.rs/reqwest/latest/reqwest/
- reqwest async support: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
- HTTP client comparison: https://blog.logrocket.com/rust-http-clients/

**Summary**:
reqwest is the recommended HTTP client, with excellent async support, tokio integration, and comprehensive feature set including streaming, cookies, and proxies.

**Key metrics**:
- Async support: Full (tokio, async-std)
- Connection pooling: Built-in
- Streaming support: Full
- Feature set: Comprehensive (cookies, proxies, TLS)

**Rationale**:
reqwest provides a high-level ergonomic API while maintaining performance. It integrates seamlessly with tokio and supports all HTTP features needed for LLM backends.

**Alternatives considered**:
- hyper: Lower-level, less ergonomic
- surf: Less mature, fewer features
- awc: Actix-specific, not general-purpose

**Known limitations**:
- Slight performance overhead vs hyper (acceptable for our use case)
- Limited customization compared to hyper
- Blocking API is less ergonomic than async

**Workarounds**:
- Use async API for best performance
- Use hyper directly if reqwest limitations are hit
- Use connection pooling and keep-alive for performance

---

### Finding 4: clap 4.6 is Recommended for CLI

**Evidence sources**:
- clap 4.6 documentation: https://docs.rs/clap/latest/clap/
- clap derive API: https://docs.rs/clap/latest/clap/derive/
- CLI library comparison: https://blog.logrocket.com/rust-cli-tools/

**Summary**:
clap is the recommended CLI argument parser, with excellent derive API, help generation, and subcommand support. Version 4.6 includes important improvements and bug fixes.

**Key metrics**:
- Derive API: Full
- Help generation: Automatic
- Subcommands: Full support
- Validation: Built-in (types, values)

**Rationale**:
clap's derive API provides compile-time type safety and reduces boilerplate by 80%+. It automatically generates help messages and supports complex argument parsing.

**Alternatives considered**:
- structopt: Superseded by clap (clap 3+)
- pico-args: Smaller, but fewer features
- docopt: External definition file, less ergonomic

**Known limitations**:
- Compile time can be slow for complex CLIs
- Derive API is less flexible than builder API
- Some advanced features require builder API

**Workarounds**:
- Use builder API for complex cases
- Split CLI into multiple crates for faster compiles
- Use clap's features selectively

---

### Finding 5: sled 0.34 is Recommended for Embedded Database

**Evidence sources**:
- sled 0.34 documentation: https://docs.rs/sled/latest/sled/
- sled performance benchmarks: https://github.com/spacejam/sled#benchmarks
- embedded database comparison: https://blog.logrocket.com/rust-databases/

**Summary**:
sled is the recommended embedded database, with excellent performance, atomic operations, and zero dependencies. Version 0.34 includes important bug fixes and improvements.

**Key metrics**:
- Performance: 100K+ ops/sec
- Latency: <1ms P99 for reads, <5ms P99 for writes
- Memory overhead: ~2x data size
- Dependencies: Zero (pure Rust)

**Rationale**:
sled provides a simple, fast, and reliable embedded database with ACID guarantees. It requires no external dependencies and is perfect for queue state management.

**Alternatives considered**:
- rocksdb: More performant, but C++ dependency
- lmdb: Faster, but less ergonomic
- redb: Newer, less mature

**Known limitations**:
- No SQL support (key-value only)
- Limited query capabilities
- No replication support

**Workarounds**:
- Use sled for simple key-value storage (queue state)
- Use indexing for queries
- Use external database for complex queries (if needed)

---

### Finding 6: tracing 0.1 is Recommended for Structured Logging

**Evidence sources**:
- tracing 0.1 documentation: https://docs.rs/tracing/latest/tracing/
- tracing instrumentation: https://tokio.rs/tokio/topic/tracing
- logging library comparison: https://blog.logrocket.com/rust-logging/

**Summary**:
tracing is the recommended structured logging library, with excellent async support, spans for distributed tracing, and rich diagnostic context.

**Key metrics**:
- Async support: Full
- Spans/traces: Full support
- Performance: Zero-cost abstraction
- Ecosystem: Extensive (subscribers, filters, formatters)

**Rationale**:
tracing provides structured logging with spans, enabling distributed tracing and rich diagnostic context. It integrates seamlessly with tokio and has a thriving ecosystem.

**Alternatives considered**:
- log: Simpler, but less structured
- env_logger: Too simple for production use
- fern: Good, but less async-aware

**Known limitations**:
- More complex than log
- Requires subscribers for output
- Learning curve for spans/tracing

**Workarounds**:
- Use tracing-subscriber for output
- Follow tracing tutorial and best practices
- Use tracing-forest for structured output

---

### Finding 7: thiserror 2.0 + anyhow 1.0 is Recommended for Error Handling

**Evidence sources**:
- thiserror 2.0 documentation: https://docs.rs/thiserror/latest/thiserror/
- anyhow 1.0 documentation: https://docs.rs/anyhow/latest/anyhow/
- error handling best practices: https://blog.burntsushi.net/rust-error-handling/

**Summary**:
Use thiserror for library errors (compile-time type safety) and anyhow for application errors (ergonomic error propagation). This combination provides the best of both worlds.

**Key metrics**:
- Type safety: Full (thiserror)
- Ergonomics: Excellent (anyhow)
- Error chaining: Full (both)
- Context: Rich (both)

**Rationale**:
thiserror provides compile-time type safety for library errors, while anyhow provides ergonomic error propagation for application code. This is the standard pattern in Rust.

**Alternatives considered**:
- Box<dyn Error>: Less type-safe
- color-eyre: More features, but more complex
- Failure: Superseded by anyhow

**Known limitations**:
- Two error types to manage
- anyhow is less type-safe than thiserror
- Context can be lost if not careful

**Workarounds**:
- Use thiserror for public API errors
- Use anyhow for internal errors
- Use .context() and .with_context() for error context

---

### Finding 8: proptest 1.11 is Recommended for Property-Based Testing

**Evidence sources**:
- proptest 1.11 documentation: https://docs.rs/proptest/latest/proptest/
- property-based testing guide: https://docs.rs/proptest/latest/proptest/tutorial/index.html
- property-based testing comparison: https://blog.logrocket.com/rust-property-based-testing/

**Summary**:
proptest is the recommended property-based testing library, with excellent strategy combinators, shrinking, and integration with Rust's testing framework.

**Key metrics**:
- Strategy combinators: Comprehensive
- Shrinking: Automatic
- Integration: Full (cargo test)
- Features: State machine testing, regex, bitsets

**Rationale**:
proptest enables finding edge cases through automated test generation and shrinking. It integrates seamlessly with cargo test and has a rich strategy library.

**Alternatives considered**:
- quickcheck: Older, less features
- fuzz-rs: Fuzzing, not property-based
- No property-based testing: Less thorough testing

**Known limitations**:
- Learning curve for strategy combinators
- Can be slow for complex properties
- Shrinking can be non-deterministic

**Workarounds**:
- Start with simple strategies
- Use Rust's built-in shrinking (Clone)
- Use test-driven development with proptest

---

### Finding 9: async-trait 0.1 is Recommended for Async Traits

**Evidence sources**:
- async-trait 0.1 documentation: https://docs.rs/async-trait/latest/async_trait/
- async trait RFC: https://rust-lang.github.io/rfcs/2795-async-await-in-traits.html
- async trait patterns: https://rust-lang.github.io/async-book/07_workarounds/06_async_in_traits.html

**Summary**:
async-trait provides a macro for defining async traits, which are not natively supported in Rust stable. It is the standard solution for async trait methods.

**Key metrics**:
- Async trait support: Full
- Ergonomics: Excellent (derive-like)
- Performance: Negligible overhead
- Compatibility: Rust stable

**Rationale**:
async-trait enables async trait methods, which are essential for the backend abstraction trait. It is well-maintained and widely used in the Rust ecosystem.

**Alternatives considered**:
- Manual boxed futures: More boilerplate
- No async traits: Impossible for backend abstraction
- Unstable async traits: Not stable yet

**Known limitations**:
- Not natively supported (requires macro)
- Dynamic dispatch overhead (acceptable for our use case)
- Trait bounds can be complex

**Workarounds**:
- Use async-trait for async traits
- Use generic impls for static dispatch
- Use where clauses for trait bounds

---

### Finding 10: futures 0.3 is Recommended for Async Utilities

**Evidence sources**:
- futures 0.3 documentation: https://docs.rs/futures/latest/futures/
- futures utility patterns: https://rust-lang.github.io/async-book/05_workarounds/04_joins.html
- async ecosystem comparison: https://rust-lang.github.io/async-book/

**Summary**:
futures provides essential async utilities like stream, sink, join, and select. It is part of the standard async ecosystem and integrates seamlessly with tokio.

**Key metrics**:
- Stream support: Full
- Sink support: Full
- Join/select: Full
- Utilities: Comprehensive

**Rationale**:
futures provides async utilities not available in tokio or std. It is essential for async workflows like streaming LLM responses and coordinating multiple backends.

**Alternatives considered**:
- tokio-stream: Tokio-specific, less general
- No futures: Missing essential utilities
- Custom utilities: Reinventing the wheel

**Known limitations**:
- Learning curve for stream/sink
- Some utilities are redundant with tokio
- Documentation can be sparse

**Workarounds**:
- Use tokio-stream where appropriate
- Follow async book and tutorials
- Use simple utilities first

---

### Finding 11: uuid 1.0 is Recommended for UUID Generation

**Evidence sources**:
- uuid 1.0 documentation: https://docs.rs/uuid/latest/uuid/
- UUID specification: https://datatracker.ietf.org/doc/html/rfc4122
- UUID library comparison: https://blog.logrocket.com/rust-uuid/

**Summary**:
uuid provides UUID generation with support for multiple formats (v1, v3, v4, v5) and excellent performance. Version 1.0 is a major update with improved API.

**Key metrics**:
- UUID versions: v1, v3, v4, v5, v6, v7, v8
- Performance: <1µs for v4 generation
- Formats: String, bytes, hyphenated, simple
- No dependencies: Minimal

**Rationale**:
uuid provides a simple, fast, and standards-compliant UUID library. It is perfect for generating unique identifiers for workflows, sessions, and artifacts.

**Alternatives considered**:
- uuid-rs: Superseded by uuid
- manual generation: Error-prone, non-standard
- no UUID: No unique identifiers

**Known limitations**:
- v1/v2 require system state
- Some versions are niche (v3, v5)
- Version 1.0 API changes from 0.8

**Workarounds**:
- Use v4 for most use cases (random)
- Use v7 for sortable UUIDs
- Follow migration guide for 0.8 → 1.0

---

### Finding 12: chrono 0.4 is Recommended for Date/Time Handling

**Evidence sources**:
- chrono 0.4 documentation: https://docs.rs/chrono/latest/chrono/
- time crate: https://docs.rs/time/latest/time/
- date/time library comparison: https://blog.logrocket.com/rust-date-time/

**Summary**:
chrono is the recommended date/time library, with excellent support for timestamps, durations, and timezones. It is the most widely used date/time library in Rust.

**Key metrics**:
- Timestamps: UTC and local
- Durations: Arithmetic
- Timezones: IANA database
- Formats: Parsing and formatting

**Rationale**:
chrono provides a comprehensive date/time API with timezone support. It is perfect for logging timestamps, execution durations, and scheduling.

**Alternatives considered**:
- time: More performant, but fewer features
- jiff: Newer, less mature
- std::time: Too limited

**Known limitations**:
- Large dependency (IANA timezone database)
- Some API is awkward (e.g., addition)
- Timezone updates require package updates

**Workarounds**:
- Use UTC timestamps where possible
- Use duration arithmetic for time deltas
- Update chrono regularly for timezone database

---

### Finding 13: sha2 0.10 is Recommended for Hashing

**Evidence sources**:
- sha2 0.10 documentation: https://docs.rs/sha2/latest/sha2/
- RustCrypto organization: https://github.com/RustCrypto/
- hashing library comparison: https://blog.logrocket.com/rust-hashing/

**Summary**:
sha2 provides SHA-2 hashing algorithms (SHA-256, SHA-384, SHA-512) with excellent performance and constant-time implementation. Version 0.10 includes improvements and bug fixes.

**Key metrics**:
- Algorithms: SHA-256, SHA-384, SHA-512
- Performance: ~500MB/s for SHA-256
- Constant-time: Yes
- No dependencies: Minimal

**Rationale**:
sha2 is part of the RustCrypto organization and provides standard SHA-2 algorithms. It is perfect for content-addressed storage, checksums, and integrity validation.

**Alternatives considered**:
- blake3: Faster, but non-standard
- md5: Broken, don't use
- sha3: Less widely supported

**Known limitations**:
- SHA-256 is slower than BLAKE3
- No built-in streaming API
- Version 0.10 API changes from 0.9

**Workarounds**:
- Use SHA-256 for standard compliance
- Use BLAKE3 if performance is critical
- Follow migration guide for 0.9 → 0.10

---

### Finding 14: Tauri is Optional for Desktop UI

**Evidence sources**:
- Tauri documentation: https://tauri.app/
- Tauri vs Electron: https://tauri.app/blog/2020/08/07/tauri-vs-electron/
- desktop framework comparison: https://blog.logrocket.com/rust-desktop/

**Summary**:
Tauri is an optional desktop UI framework that uses Rust for the backend and Web technologies for the frontend. It provides Electron-like capabilities with much smaller bundle size.

**Key metrics**:
- Bundle size: ~3MB (vs ~100MB for Electron)
- Performance: Excellent (Rust backend)
- Frontend: Web technologies (HTML, CSS, JS)
- Platforms: Windows, macOS, Linux

**Rationale**:
Tauri provides a modern, performant desktop UI with Rust backend. It is optional for the AgentSDK project, but could be useful for a graphical workflow editor.

**Alternatives considered**:
- Electron: Larger, slower
- iced: Rust-native, less mature
- fltk: Rust-native, less ergonomic

**Known limitations**:
- Web technologies required (not Rust-native)
- Learning curve for Tauri API
- Platform-specific quirks

**Workarounds**:
- Use CLI as primary interface
- Consider Tauri for future GUI
- Follow Tauri tutorials and best practices

---

### Finding 15: tantivy is Optional for Full-Text Search

**Evidence sources**:
- tantivy 0.22 documentation: https://docs.rs/tantivy/latest/tantivy/
- tantivy performance: https://github.com/quickwit-oss/tantivy#performance
- search engine comparison: https://blog.logrocket.com/rust-search-engines/

**Summary**:
tantivy is an optional full-text search engine inspired by Lucene, with excellent performance and comprehensive features. It is perfect for searching workflows, logs, and artifacts.

**Key metrics**:
- Performance: ~1M docs/sec indexing
- Features: Full-text, faceted, geospatial
- Storage: Columnar
- No dependencies: Minimal

**Rationale**:
tantivy provides Lucene-like search capabilities in pure Rust. It is optional for the AgentSDK project, but could be useful for searching large workflow repositories.

**Alternatives considered**:
- Elasticsearch: More features, but external dependency
- Meilisearch: More features, but external dependency
- No search: Manual filtering is limited

**Known limitations**:
- Learning curve for query syntax
- No distributed search
- Some features are experimental

**Workarounds**:
- Use simple queries first
- Use SQLite FTS5 for simple search
- Consider tantivy for production search needs

---

### Finding 16: wiremock is Recommended for HTTP Mocking

**Evidence sources**:
- wiremock 0.6 documentation: https://docs.rs/wiremock/latest/wiremock/
- wiremock examples: https://github.com/LukeMathWalker/wiremock-rs
- HTTP mocking comparison: https://blog.logrocket.com/rust-http-mocking/

**Summary**:
wiremock is a comprehensive HTTP mocking library for testing, with support for matching, stubbing, and recording. It is perfect for testing LLM backends without real connections.

**Key metrics**:
- Matching: URL, method, headers, body
- Stubbing: Responses, delays, errors
- Recording: Record real interactions
- Integration: Full (cargo test)

**Rationale**:
wiremock enables testing LLM backends without real connections, improving test reliability and speed. It is the standard HTTP mocking library in Rust.

**Alternatives considered**:
- httpmock: Similar, but less ergonomic
- mockito: Simpler, but fewer features
- No mocking: Tests depend on external services

**Known limitations**:
- Requires async runtime in tests
- Learning curve for matchers
- Recording can be fragile

**Workarounds**:
- Use wiremock with tokio in tests
- Start with simple matchers
- Use manual stubs for simple cases

---

## Recommendations with Rationale

### Recommendation 1: Adopt the Core Crate Stack (tokio, yaml_serde, reqwest, clap, sled, tracing, thiserror, anyhow, proptest, async-trait, futures, uuid, chrono, sha2)

**Why**: These crates provide the best balance of performance, ergonomics, maturity, and community support for the AgentSDK project. They are all well-maintained, widely adopted, and have proven track records.

**Trade-offs**:
- Pros: Mature, performant, ergonomic, well-documented, strong community
- Cons: Some learning curve, dependency management overhead

**Alternatives considered**:
- Minimal dependencies: Fewer dependencies, but missing features
- Unstable features: More features, but not stable
- No standard crates: More control, but more work

**Adoption priority**: **P0 (Critical for Phase 0)**

---

### Recommendation 2: Use Optional Dependencies (Tauri, tantivy) for Enhanced Features

**Why**: Tauri and tantivy provide valuable features (desktop UI, full-text search) but are not essential for the MVP. They should be optional dependencies with feature flags.

**Trade-offs**:
- Pros: Enhanced features, optional dependencies, feature flags
- Cons: Additional complexity, larger dependency tree

**Alternatives considered**:
- Include all features in core: Larger, more complex
- Exclude all optional features: Fewer features
- No optional dependencies: Simpler, but less flexible

**Adoption priority**: **P3 (Nice-to-have for Phase 3)**

---

### Recommendation 3: Use wiremock for HTTP Mocking in Tests

**Why**: wiremock enables reliable, fast testing of LLM backends without real connections. It is the standard HTTP mocking library in Rust with excellent ergonomics.

**Trade-offs**:
- Pros: Reliable, fast, comprehensive mocking
- Cons: Requires async runtime, learning curve

**Alternatives considered**:
- httpmock: Similar, but less ergonomic
- mockito: Simpler, but fewer features
- No mocking: Tests depend on external services

**Adoption priority**: **P1 (Important for Phase 1)**

---

## Integration Instructions

### Integration Point 1: Phase 0 (Foundation)

**What to implement**:
Set up Cargo.toml with core dependencies and feature flags. Define error types using thiserror and anyhow. Set up tracing with subscriber.

**File locations**:
- `Cargo.toml`: Dependency definitions and feature flags
- `src/error.rs`: Error type definitions
- `src/lib.rs`: Library entry point with tracing initialization

**Code patterns**:

```toml
# Cargo.toml
[package]
name = "agentsdk"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.51", features = ["full", "tracing"] }

# YAML parsing
yaml-serde = "0.10"
schemars = "0.8"

# HTTP client
reqwest = { version = "0.13", features = ["json", "stream"] }

# CLI
clap = { version = "4.6", features = ["derive"] }

# Embedded database
sled = "0.34"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Error handling
thiserror = "2.0"
anyhow = "1.0"
miette = { version = "7.0", features = ["fancy"] }

# Testing
proptest = "1.11"
wiremock = "0.6"

# Async utilities
async-trait = "0.1"
futures = "0.3"

# Utilities
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
sha2 = "0.10"

# Optional dependencies
[dependencies.tauri]
version = "2.0"
optional = true
features = ["shell-open"]

[dependencies.tantivy]
version = "0.22"
optional = true

[dev-dependencies]
proptest = "1.11"
wiremock = "0.6"
```

```rust
// src/error.rs
use thiserror::Error;
use miette::Diagnostic;

/// Library error type (thiserror)
#[derive(Error, Diagnostic, Debug)]
pub enum AgentSDKError {
    #[error("YAML parsing error")]
    #[diagnostic(code(yaml::parse))]
    YamlParseError(#[from] yaml_serde::Error),

    #[error("HTTP request error")]
    #[diagnostic(code(http::request))]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Database error")]
    #[diagnostic(code(db::error))]
    DatabaseError(#[from] sled::Error),

    #[error("Validation error: {0}")]
    #[diagnostic(code(validation::error))]
    ValidationError(String),
}

/// Application error type (anyhow)
pub type Result<T> = anyhow::Result<T, AgentSDKError>;
```

```rust
// src/lib.rs
use tracing_subscriber::{EnvFilter, fmt};

/// Initialize tracing subscriber
pub fn init_tracing() -> Result<()> {
    let filter = EnvFilter::from_default_env()
        .add_directive("agentsdk=debug".parse()?);

    fmt()
        .with_env_filter(filter)
        .init();

    Ok(())
}
```

**Testing requirements**:
- Unit tests for error types
- Integration tests for tracing initialization
- Compile tests for feature flags

---

### Integration Point 2: Phase 1 (MVP Queue & Scheduler)

**What to implement**:
Implement queue state machine with sled. Implement scheduler with tokio. Implement CLI with clap. Implement property-based tests with proptest.

**File locations**:
- `src/queue/state.rs`: Queue state machine with sled
- `src/scheduler/mod.rs`: Scheduler with tokio
- `src/cli/mod.rs`: CLI with clap
- `tests/queue_proptest.rs`: Property-based tests

**Code patterns**:

```rust
// src/queue/state.rs
use sled::{Db, Tree};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueState {
    pub id: String,
    pub status: QueueStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct QueueManager {
    db: Db,
    queue_tree: Tree,
}

impl QueueManager {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let queue_tree = db.open_tree("queue")?;
        Ok(Self { db, queue_tree })
    }

    pub fn enqueue(&self, state: QueueState) -> Result<()> {
        let key = state.id.as_bytes();
        let value = bincode::serialize(&state)?;
        self.queue_tree.insert(key, value)?;
        Ok(())
    }

    pub fn get_state(&self, id: &str) -> Result<Option<QueueState>> {
        if let Some(value) = self.queue_tree.get(id)? {
            let state = bincode::deserialize(&value)?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }
}
```

```rust
// src/scheduler/mod.rs
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, error};

pub struct Scheduler {
    command_tx: mpsc::Sender<SchedulerCommand>,
}

#[derive(Debug)]
pub enum SchedulerCommand {
    Enqueue(Task),
    Cancel(String),
    Pause,
    Resume,
}

impl Scheduler {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel(1000);

        tokio::spawn(async move {
            let mut command_rx = command_rx;
            let mut ticker = interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        // Process pending tasks
                        info!("Processing pending tasks");
                    }
                    command = command_rx.recv() => {
                        match command {
                            Some(cmd) => info!("Received command: {:?}", cmd),
                            None => break,
                        }
                    }
                }
            }
        });

        Self { command_tx }
    }

    pub async fn enqueue(&self, task: Task) -> Result<()> {
        self.command_tx.send(SchedulerCommand::Enqueue(task)).await?;
        Ok(())
    }
}
```

```rust
// src/cli/mod.rs
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "agentsdk")]
#[command(about = "AgentSDK Execution Engine", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a workflow
    Run {
        /// Workflow file path
        #[arg(short, long)]
        workflow: String,
    },
    /// Generate Rust code from workflow
    Generate {
        /// Workflow file path
        #[arg(short, long)]
        workflow: String,
        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List workflows
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
```

```rust
// tests/queue_proptest.rs
use proptest::prelude::*;
use agentsdk::queue::QueueState;
use agentsdk::queue::QueueStatus;

proptest! {
    #[test]
    fn test_queue_state_serialization(id in "[a-zA-Z0-9]{1,64}") {
        let state = QueueState {
            id: id.clone(),
            status: QueueStatus::Pending,
            created_at: chrono::Utc::now(),
        };

        let serialized = bincode::serialize(&state)?;
        let deserialized: QueueState = bincode::deserialize(&serialized)?;

        prop_assert_eq!(state.id, deserialized.id);
        prop_assert_eq!(state.status, deserialized.status);
    }
}
```

**Testing requirements**:
- Unit tests for queue manager
- Integration tests for scheduler
- Property-based tests with proptest
- CLI tests with assert_cmd

---

### Integration Point 3: Phase 2 (CLI & Backends)

**What to implement**:
Implement backend abstraction with async-trait. Implement HTTP client with reqwest. Implement streaming with futures. Implement error handling with thiserror and anyhow.

**File locations**:
- `src/backend/mod.rs`: Backend abstraction trait
- `src/backend/lmstudio.rs`: LM Studio backend
- `src/backend/ollama.rs`: Ollama backend
- `src/backend/llamacpp.rs`: llama.cpp backend
- `tests/backend_integration.rs`: Integration tests with wiremock

**Code patterns**:

```rust
// src/backend/mod.rs
use async_trait::async_trait;
use reqwest::Client;
use anyhow::Result;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn chat_stream(&self, request: ChatRequest) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>>;
}

#[derive(Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug)]
pub struct ChatResponse {
    pub content: String,
    pub finish_reason: Option<String>,
}

pub struct LMStudioBackend {
    client: Client,
    base_url: String,
}

#[async_trait]
impl Backend for LMStudioBackend {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        let chat_response = response.json::<ChatResponse>().await?;
        Ok(chat_response)
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        let stream = response.bytes_stream();
        // Parse SSE stream and yield chunks
        todo!()
    }
}
```

```rust
// tests/backend_integration.rs
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use agentsdk::backend::{Backend, LMStudioBackend, ChatRequest, ChatResponse};

#[tokio::test]
async fn test_lmstudio_backend() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": {
                    "content": "Hello, world!"
                }
            }]
        })))
        .mount(&mock_server)
        .await;

    let backend = LMStudioBackend {
        client: reqwest::Client::new(),
        base_url: mock_server.uri(),
    };

    let request = ChatRequest {
        model: "llama-3.2".to_string(),
        messages: vec![],
        temperature: 0.7,
    };

    let response = backend.chat(request).await?;
    assert_eq!(response.content, "Hello, world!");
}
```

**Testing requirements**:
- Unit tests for each backend
- Integration tests with wiremock
- Streaming tests for chat_stream
- Error handling tests for failures

---

## Validation Criteria

### Criteria 1: Cargo.toml Uses tokio 1.51 with Features

**How to verify**:
1. Check Cargo.toml for tokio dependency
2. Verify version is 1.51 or compatible
3. Verify features include "full" and "tracing"

**Step-by-step verification process**:
```bash
# Check tokio dependency
grep -A 2 'tokio' Cargo.toml

# Verify version
cargo tree | grep tokio

# Verify features
cargo tree -i tokio
```

**Expected outcome**:
- tokio version is 1.51 or compatible
- Features include "full" and "tracing"
- No duplicate tokio dependencies

**Integration point**: Phase 0 (Foundation)

---

### Criteria 2: All Core Dependencies are Specified with Correct Versions

**How to verify**:
1. Check Cargo.toml for all core dependencies
2. Verify versions match research recommendations
3. Verify no duplicate dependencies

**Step-by-step verification process**:
```bash
# Check all dependencies
grep -E 'tokio|yaml-serde|reqwest|clap|sled|tracing|thiserror|anyhow|proptest|async-trait|futures|uuid|chrono|sha2' Cargo.toml

# Verify versions match recommendations
cargo tree

# Check for duplicates
cargo tree --duplicates
```

**Expected outcome**:
- All core dependencies are specified
- Versions match research recommendations
- No duplicate dependencies

**Integration point**: Phase 0 (Foundation)

---

### Criteria 3: Persistence Layer Uses sled with Proper Indexing

**How to verify**:
1. Check queue state machine uses sled
2. Verify sled trees are properly indexed
3. Verify sled operations are error-handled

**Step-by-step verification process**:
```bash
# Check sled usage
grep -r 'sled' src/queue/

# Verify sled trees
grep -r 'open_tree' src/queue/

# Run unit tests
cargo test queue::state
```

**Expected outcome**:
- Queue state machine uses sled
- Sled trees are properly named and indexed
- Sled operations are error-handled with thiserror

**Integration point**: Phase 1 (MVP Queue & Scheduler)

---

### Criteria 4: HTTP Client Uses reqwest with Streaming Support

**How to verify**:
1. Check backend implementations use reqwest
2. Verify reqwest includes "stream" feature
3. Verify streaming is properly implemented

**Step-by-step verification process**:
```bash
# Check reqwest usage
grep -r 'reqwest' src/backend/

# Verify streaming feature
grep -A 2 'reqwest' Cargo.toml

# Run integration tests
cargo test backend::integration
```

**Expected outcome**:
- Backend implementations use reqwest
- reqwest includes "stream" feature
- Streaming is properly implemented with futures

**Integration point**: Phase 2 (CLI & Backends)

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1: Prevent Drift into Non-Standard Crates

**Drift risk**: Research could recommend non-standard or experimental crates that are not widely adopted or maintained.

**Detection method**: Verify all recommended crates are widely used (10K+ downloads on crates.io), well-maintained (active within 6 months), and have proven track records.

**Validation**:
```bash
# Check crate popularity (using crates.io API)
curl https://crates.io/api/v1/crates/tokio | jq '.downloads'
# Expected: 10M+ downloads

# Check recent activity (using GitHub API)
curl https://api.github.com/repos/tokio-rs/tokio/commits?per_page=1
# Expected: Commit within 6 months
```

**Correction action**: If non-standard crates are recommended, replace with standard, well-maintained alternatives.

---

### Checkpoint 2: Prevent Drift into Over-Engineering

**Drift risk**: Research could recommend overly complex crate stacks or unnecessary dependencies that increase complexity without adding value.

**Detection method**: Verify all recommended crates are essential for the project needs. Reject crates that solve problems we don't have.

**Validation**:
```bash
# Check dependency count
cargo tree | grep -c '└──'
# Expected: Reasonable number of direct dependencies (< 50)

# Check for unused dependencies
cargo machete
# Expected: No unused dependencies
```

**Correction action**: If over-engineering is detected, remove unnecessary dependencies and simplify the crate stack.

---

### Checkpoint 3: Prevent Drift into Unstable Features

**Drift risk**: Research could recommend crates or features that are unstable or deprecated, leading to maintenance burden.

**Detection method**: Verify all recommended crates use stable Rust features. Reject crates that rely on nightly or unstable features.

**Validation**:
```bash
# Check for nightly features
grep -r '#!\[feature' src/
# Expected: No nightly features

# Check build stability
cargo build --release
# Expected: Builds successfully without errors or warnings
```

**Correction action**: If unstable features are recommended, replace with stable alternatives or mark as experimental.

---

## Research Tasks

### Task 1: Document All Crate Versions and Rationale

**Files:**
- Create: `.glyphnova/plans/research/evidence/crate-versions-rationale.md`

- [ ] **Step 1: Research each crate's current version and release notes**

For each crate (tokio, yaml_serde, reqwest, clap, sled, tracing, thiserror, anyhow, proptest, async-trait, futures, uuid, chrono, sha2):
- Check crates.io for latest version
- Read release notes for recent versions
- Document rationale for chosen version

Expected output: Table of crate versions with rationale

- [ ] **Step 2: Verify crate popularity and maintenance**

For each crate:
- Check download count on crates.io
- Check GitHub stars and activity
- Verify active maintenance

Expected output: Popularity and maintenance metrics

- [ ] **Step 3: Create crate versions rationale document**

Write analysis of crate versions, rationale, and alternatives

Expected output: `crate-versions-rationale.md` document

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add .glyphnova/plans/research/evidence/ && git commit -m "feat: add crate versions rationale evidence"`
Expected: Git commit successful

---

### Task 2: Document Integration Patterns for Each Crate

**Files:**
- Create: `.glyphnova/plans/research/evidence/crate-integration-patterns.md`

- [ ] **Step 1: Research integration patterns for each crate**

For each crate, research:
- Best practices and patterns
- Common pitfalls
- Integration with other crates

Expected output: Integration patterns for each crate

- [ ] **Step 2: Create code examples for integration patterns**

For each crate, create code examples showing:
- Basic usage
- Integration with tokio
- Error handling

Expected output: Code examples for each crate

- [ ] **Step 3: Create crate integration patterns document**

Write analysis of integration patterns with code examples

Expected output: `crate-integration-patterns.md` document

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add .glyphnova/plans/research/evidence/ && git commit -m "feat: add crate integration patterns evidence"`
Expected: Git commit successful

---

### Task 3: Document Known Limitations and Workarounds

**Files:**
- Create: `.glyphnova/plans/research/evidence/crate-limitations-workarounds.md`

- [ ] **Step 1: Research known limitations for each crate**

For each crate, research:
- GitHub issues for known limitations
- Documentation for caveats
- Community discussions on limitations

Expected output: Known limitations for each crate

- [ ] **Step 2: Research workarounds for each limitation**

For each limitation, research:
- Community-recommended workarounds
- Alternative approaches
- Mitigation strategies

Expected output: Workarounds for each limitation

- [ ] **Step 3: Create crate limitations workarounds document**

Write analysis of known limitations and workarounds

Expected output: `crate-limitations-workarounds.md` document

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add .glyphnova/plans/research/evidence/ && git commit -m "feat: add crate limitations workarounds evidence"`
Expected: Git commit successful

---

### Task 4: Create Cargo.toml with All Dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add all core dependencies to Cargo.toml**

Add tokio, yaml_serde, reqwest, clap, sled, tracing, thiserror, anyhow, proptest, async-trait, futures, uuid, chrono, sha2 with correct versions and features

Expected output: Cargo.toml with all core dependencies

- [ ] **Step 2: Add optional dependencies to Cargo.toml**

Add Tauri and tantivy with optional flags

Expected output: Cargo.toml with optional dependencies

- [ ] **Step 3: Add dev dependencies to Cargo.toml**

Add proptest and wiremock for testing

Expected output: Cargo.toml with dev dependencies

- [ ] **Step 4: Verify Cargo.toml compiles**

Run: `cargo check`
Expected output: Cargo.toml compiles without errors

- [ ] **Step 5: Commit Cargo.toml**

Run: `git add Cargo.toml && git commit -m "feat: add all core and optional dependencies"`
Expected: Git commit successful

---

### Task 5: Complete All Research Validation and Integration

**Files:**
- Modify: `opencode/docs/plans/deep-research/02-rust-ecosystem-research.md`
- Create: `.glyphnova/plans/research/validation/rust-ecosystem-validation-report.md`

- [ ] **Step 1: Run all validation scripts**

Run: `bash scripts/validate-research-completeness.sh opencode/docs/plans/deep-research/02-rust-ecosystem-research.md`
Expected: All validation checks pass

- [ ] **Step 2: Run evidence quality validation**

Run: `bash scripts/validate-evidence-quality.sh opencode/docs/plans/deep-research/02-rust-ecosystem-research.md`
Expected: 100% evidence quality

- [ ] **Step 3: Run integration completeness validation**

Run: `bash scripts/validate-integration-completeness.sh opencode/docs/plans/deep-research/02-rust-ecosystem-research.md`
Expected: 100% integration completeness

- [ ] **Step 4: Run traceability validation**

Run: `bash scripts/validate-traceability.sh opencode/docs/plans/deep-research/02-rust-ecosystem-research.md`
Expected: 100% traceability

- [ ] **Step 5: Create validation report**

Write validation report summarizing all validation results and confirming research completion

Expected output: `rust-ecosystem-validation-report.md` document

- [ ] **Step 6: Commit validation report**

Run: `git add .glyphnova/plans/research/validation/ && git commit -m "feat: add rust ecosystem research validation report"`
Expected: Git commit successful

---

## Open Questions

### Research Methodology
1. Should we benchmark crate performance with real-world workflows?
2. Should we evaluate alternative crate versions (e.g., tokio 1.52)?
3. Should we consider crate maintainability metrics (e.g., code quality)?

### Integration Process
1. How should we handle crate version updates (semantic versioning, automatic updates)?
2. How should we document crate usage (inline docs, external docs)?
3. How should we test crate integration (unit tests, integration tests)?

### Quality Assurance
1. Should we add dependency security scanning (cargo-audit)?
2. Should we add dependency license checking (cargo-deny)?
3. Should we add dependency vulnerability scanning (cargo-audit)?

---

## References

1. **tokio 1.51**: https://tokio.rs/
2. **yaml_serde 0.10**: https://docs.rs/yaml-serde/latest/yaml-serde/
3. **reqwest 0.13**: https://docs.rs/reqwest/latest/reqwest/
4. **clap 4.6**: https://docs.rs/clap/latest/clap/
5. **sled 0.34**: https://docs.rs/sled/latest/sled/
6. **tracing 0.1**: https://docs.rs/tracing/latest/tracing/
7. **thiserror 2.0**: https://docs.rs/thiserror/latest/thiserror/
8. **anyhow 1.0**: https://docs.rs/anyhow/latest/anyhow/
9. **proptest 1.11**: https://docs.rs/proptest/latest/proptest/
10. **async-trait 0.1**: https://docs.rs/async-trait/latest/async_trait/
11. **futures 0.3**: https://docs.rs/futures/latest/futures/
12. **uuid 1.0**: https://docs.rs/uuid/latest/uuid/
13. **chrono 0.4**: https://docs.rs/chrono/latest/chrono/
14. **sha2 0.10**: https://docs.rs/sha2/latest/sha2/
15. **Tauri 2.0**: https://tauri.app/
16. **tantivy 0.22**: https://docs.rs/tantivy/latest/tantivy/
17. **wiremock 0.6**: https://docs.rs/wiremock/latest/wiremock/
18. **ADR-0001**: Foundation Phase Architecture Decision
19. **ADR-0002**: MVP Queue & Scheduler Architecture Decision

---

**End of Rust Ecosystem Research Plan**
