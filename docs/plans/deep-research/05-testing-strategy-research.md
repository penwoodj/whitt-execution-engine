# Testing Strategy Research Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Research and document comprehensive testing strategy including unit testing with rstest, property-based testing with proptest, integration testing with mock backends, E2E testing with real LLMs, CLI testing with assert_cmd, benchmark testing, 7-layer verification framework, mock strategies for external dependencies, test data management, and CI integration.

**Architecture**: Multi-layer testing framework covering unit, integration, property-based, E2E, CLI, and benchmark testing with mock strategies and CI automation for Phase 0 (Testing Infrastructure) through Phase 3 (Production).

**Tech Stack**: Rust testing framework, rstest fixtures, proptest property-based testing, wiremock HTTP mocking, assert_cmd CLI testing, criterion benchmarks, GitHub Actions CI.

---

## Research Questions Being Answered

### Question 1: What is the 7-Layer Verification Framework?
**Why it matters**: A comprehensive verification framework ensures quality across all layers of the system, preventing bugs at each level.

**Success criteria**: Documented 7-layer verification framework with test strategies, tools, and integration points.

**Integration point**: Phase 0 (Testing Infrastructure), Phase 1 (Implementation), Phase 2 (Testing), Phase 3 (Production)

---

### Question 2: How to Mock External Dependencies for Testing?
**Why it matters**: Mocking enables reliable, fast testing without external dependencies, improving test coverage and CI speed.

**Success criteria**: Documented mock strategies for all external dependencies (LLM backends, HTTP, file system, database) with implementation examples.

**Integration point**: Phase 1 (Testing Infrastructure), Phase 2 (Backend Testing)

---

### Question 3: How to Manage Test Data and Fixtures?
**Why it matters**: Test data management ensures reproducible tests, easy test maintenance, and efficient test execution.

**Success criteria**: Documented test data management strategies, fixtures, and test data generation approaches.

**Integration point**: Phase 0 (Testing Infrastructure), Phase 1 (Implementation)

---

## Findings with Evidence

### Finding 1: 7-Layer Verification Framework

**Evidence sources**:
- Test pyramid: https://martinfowler.com/articles/practical-test-pyramid.html
- Rust testing best practices: https://rust-lang.github.io/what-we-want-from-a-testing-story/
- Comprehensive testing: https://testing.googleblog.com/2015/04/just-say-no-to-more-end-to-end-tests.html

**Summary**:
The 7-layer verification framework provides comprehensive coverage across all system layers, from unit tests to E2E tests.

**Layers**:

**Layer 1: Unit Tests**
- Scope: Individual functions, methods, traits
- Tools: Rust built-in testing, rstest
- Execution: Fast (<1s per test)
- Isolation: Full (no external dependencies)

**Layer 2: Property-Based Tests**
- Scope: Invariants, edge cases, randomness
- Tools: proptest
- Execution: Fast to medium (<10s per test)
- Isolation: Full (proptest generates test data)

**Layer 3: Integration Tests**
- Scope: Multiple components working together
- Tools: Rust integration tests, wiremock
- Execution: Medium (<30s per test)
- Isolation: High (mocked external dependencies)

**Layer 4: API Tests**
- Scope: Backend APIs, HTTP endpoints
- Tools: wiremock, reqwest
- Execution: Medium (<30s per test)
- Isolation: Medium (mocked LLM backends)

**Layer 5: CLI Tests**
- Scope: Command-line interface
- Tools: assert_cmd
- Execution: Fast (<5s per test)
- Isolation: Medium (mocked dependencies)

**Layer 6: E2E Tests**
- Scope: Complete workflows, real LLMs
- Tools: cargo test, real backends
- Execution: Slow (<5min per test)
- Isolation: Low (real dependencies)

**Layer 7: Performance Tests**
- Scope: Benchmarks, scalability, load tests
- Tools: criterion, cargo bench
- Execution: Slow (<10min per test)
- Isolation: High (isolated benchmark runs)

---

### Finding 2: Unit Testing with rstest

**Evidence sources**:
- rstest documentation: https://docs.rs/rstest/latest/rstest/
- Rust testing patterns: https://rust-lang.github.io/what-we-want-from-a-testing-story/
- Test fixtures: https://martinfowler.com/bliki/ObjectMother.html

**Summary**:
rstest provides fixture-based testing, parameterized tests, and async test support, reducing boilerplate and improving test readability.

**Code examples**:

```rust
use rstest::*;

#[fixture]
fn agent() -> MockAgent {
    MockAgent::new()
}

#[fixture]
fn context() -> AgentContext {
    AgentContext::new("test task".to_string())
}

#[rstest]
fn test_agent_thinks(agent: MockAgent, context: AgentContext) {
    let thought = agent.think(&context).unwrap();
    assert!(!thought.content.is_empty());
}

#[rstest]
#[case("task1", "result1")]
#[case("task2", "result2")]
fn test_agent_executes_tasks(agent: MockAgent, #[case] task: &str, #[case] expected: &str) {
    let result = agent.execute_task(task.to_string()).unwrap();
    assert_eq!(result.content, expected);
}

#[rstest]
#[tokio::test]
async fn test_async_agent(agent: MockAgent) {
    let result = agent.async_execute().await.unwrap();
    assert!(!result.is_empty());
}
```

---

### Finding 3: Property-Based Testing with proptest

**Evidence sources**:
- proptest documentation: https://docs.rs/proptest/latest/proptest/
- Property-based testing book: https://propertesting.com/
- proptest examples: https://altsysrq.github.io/proptest-book/intro.html

**Summary**:
proptest enables property-based testing by generating random test inputs, finding edge cases that example-based tests miss.

**Code examples**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_yaml_roundtrip(yaml_str in "[a-zA-Z0-9\\s\\-_,.:]{0,1000}") {
        let spec = parse_workflow(&yaml_str);
        if spec.is_ok() {
            let serialized = serialize_workflow(&spec.unwrap());
            let parsed = parse_workflow(&serialized);
            assert!(parsed.is_ok());
        }
    }

    #[test]
    fn test_agent_output_stability(task in "[a-zA-Z0-9\\s]{1,100}") {
        let agent = MockAgent::new();
        let output1 = agent.execute_task(task.clone()).unwrap();
        let output2 = agent.execute_task(task.clone()).unwrap();
        assert_eq!(output1.content, output2.content);
    }

    #[test]
    fn test_merge_commutativity(
        outputs1 in vec(agent_output_strategy(), 0..10),
        outputs2 in vec(agent_output_strategy(), 0..10)
    ) {
        let merged1 = merge_outputs(&outputs1, &outputs2);
        let merged2 = merge_outputs(&outputs2, &outputs1);
        assert_eq!(merged1.content, merged2.content);
    }
}

fn agent_output_strategy() -> impl Strategy<Value = AgentOutput> {
    "[a-zA-Z0-9\\s]{0,100}".prop_map(|content| AgentOutput {
        content,
        metadata: HashMap::new(),
    })
}
```

---

### Finding 4: Integration Testing with wiremock

**Evidence sources**:
- wiremock documentation: https://docs.rs/wiremock/latest/wiremock/
- HTTP mocking patterns: https://blog.logrocket.com/rust-http-mocking/
- Integration testing best practices: https://testingjavascript.com/

**Summary**:
wiremock enables HTTP mocking for integration tests, allowing testing without real LLM backends or external APIs.

**Code examples**:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, json_body};

#[tokio::test]
async fn test_backend_integration() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "chatcmpl-123",
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
        temperature: None,
        max_tokens: None,
        stream: false,
        tools: None,
        extra_params: HashMap::new(),
    };

    let response = backend.chat(request).await.unwrap();
    assert_eq!(response.choices[0].message.content, "Hello, world!");
}
```

---

### Finding 5: E2E Testing with Real LLMs

**Evidence sources**:
- E2E testing best practices: https://testing.googleblog.com/2015/04/just-say-no-to-more-end-to-end-tests.html
- LLM testing: https://arxiv.org/abs/2308.12665
- E2E test patterns: https://martinfowler.com/articles/microservice-testing/

**Summary**:
E2E tests verify complete workflows with real LLM backends, ensuring end-to-end functionality. These tests are slow but critical for confidence.

**Code examples**:

```rust
#[tokio::test]
#[ignore] // Ignored by default, run with: cargo test -- --ignored
async fn test_e2e_workflow_with_real_llm() {
    // Skip if real LLM not available
    if !std::env::var("RUN_E2E_TESTS").is_ok() {
        return;
    }

    let backend = LMStudioBackend::new("http://localhost:1234".to_string());
    let agent = ReactAgent::new(backend);

    let workflow = load_workflow("tests/fixtures/simple_workflow.yaml").unwrap();
    let result = agent.execute_workflow(workflow).await.unwrap();

    assert!(!result.content.is_empty());
    assert!(result.success);
}
```

---

### Finding 6: CLI Testing with assert_cmd

**Evidence sources**:
- assert_cmd documentation: https://docs.rs/assert_cmd/latest/assert_cmd/
- CLI testing best practices: https://clig.dev/
- Command-line testing: https://github.com/assert-rs/assert_cmd

**Summary**:
assert_cmd enables CLI testing by spawning subprocesses, capturing output, and asserting on results.

**Code examples**:

```rust
use assert_cmd::Command;

#[test]
fn test_cli_run_command() {
    Command::cargo_bin("agentsdk")
        .unwrap()
        .arg("run")
        .arg("--workflow")
        .arg("tests/fixtures/simple_workflow.yaml")
        .assert()
        .success()
        .stdout(predicates::str::contains("Workflow completed"));
}

#[test]
fn test_cli_generate_command() {
    Command::cargo_bin("agentsdk")
        .unwrap()
        .arg("generate")
        .arg("--workflow")
        .arg("tests/fixtures/simple_workflow.yaml")
        .arg("--output")
        .arg("target/generated")
        .assert()
        .success();
}

#[test]
fn test_cli_invalid_workflow() {
    Command::cargo_bin("agentsdk")
        .unwrap()
        .arg("run")
        .arg("--workflow")
        .arg("tests/fixtures/invalid_workflow.yaml")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid workflow"));
}
```

---

### Finding 7: Benchmark Testing with Criterion

**Evidence sources**:
- criterion documentation: https://bheisler.github.io/criterion.rs/book/
- Rust benchmarking: https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html
- Performance testing: https://www.troyhunt.com/the-80-20-rule-of-performance/

**Summary**:
criterion provides statistical benchmarking, comparing performance across iterations and detecting regressions.

**Code examples**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_yaml_parsing(c: &mut Criterion) {
    let yaml = std::fs::read_to_string("tests/fixtures/large_workflow.yaml").unwrap();

    c.bench_function("yaml_parsing", |b| {
        b.iter(|| parse_workflow(black_box(&yaml)));
    });
}

fn bench_agent_execution(c: &mut Criterion) {
    let agent = MockAgent::new();
    let task = "Analyze this codebase".to_string();

    c.bench_function("agent_execution", |b| {
        b.iter(|| agent.execute_task(black_box(task.clone())));
    });
}

criterion_group!(benches, bench_yaml_parsing, bench_agent_execution);
criterion_main!(benches);
```

---

### Finding 8: Mock Strategies for External Dependencies

**Evidence sources**:
- Mocking strategies: https://testing.googleblog.com/2013/05/testing-on-toilet-few-best-practices.html
- wiremock HTTP mocking: https://docs.rs/wiremock/latest/wiremock/
- Dependency injection: https://martinfowler.com/articles/injection.html

**Summary**:
Mock strategies enable testing without external dependencies, improving test reliability and speed.

**Mock strategies**:

**HTTP mocks (wiremock)**:
- Mock LLM backend APIs
- Mock HTTP requests/responses
- Mock streaming responses

**File system mocks**:
- Mock file operations (read, write, delete)
- Mock directory operations
- Use tempfile crate for temporary files

**Database mocks**:
- Mock sled database with in-memory database
- Mock persistence layer
- Use sled::Config().temporary(true)

**Time mocks**:
- Mock current time for time-dependent tests
- Use mock_instant crate

**Randomness mocks**:
- Mock random number generation
- Use seedable RNG for reproducibility

---

### Finding 9: Test Data Management

**Evidence sources**:
- Test data management: https://testing.googleblog.com/2015/03/ testing-on-toilet-test-truth.html
- Test fixtures: https://martinfowler.com/bliki/ObjectMother.html
- Test data generation: https://docs.rs/fake/latest/fake/

**Summary**:
Test data management ensures reproducible tests, easy maintenance, and efficient test execution.

**Strategies**:

**Fixtures directory**:
```
tests/
├── fixtures/
│   ├── workflows/
│   │   ├── simple_workflow.yaml
│   │   ├── complex_workflow.yaml
│   │   └── edge_case_workflow.yaml
│   ├── agents/
│   │   ├── mock_agent.rs
│   │   └── test_agent.rs
│   └── backends/
│       ├── mock_backend.rs
│       └── test_backend.rs
```

**Test data generation**:
```rust
use fake::{Fake, Faker};

fn generate_random_workflow() -> WorkflowSpec {
    WorkflowSpec {
        workflow_id: fake::uuid::UUIDv4.fake(),
        name: fake::lorem::en::Word().fake(),
        steps: vec![generate_random_step()],
    }
}

fn generate_random_step() -> WorkflowStep {
    WorkflowStep {
        id: fake::uuid::UUIDv4.fake(),
        name: fake::lorem::en::Word().fake(),
        model: "llama-3.2".to_string(),
    }
}
```

---

### Finding 10: CI Integration with GitHub Actions

**Evidence sources**:
- GitHub Actions documentation: https://docs.github.com/en/actions
- Rust CI best practices: https://rust-lang.github.io/rust-clippy/
- CI/CD patterns: https://martinfowler.com/articles/continuousIntegration.html

**Summary**:
GitHub Actions provides CI/CD automation, running tests on every commit, pull request, and push.

**Workflow example**:

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy, rustfmt

      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Run cargo test
        run: cargo test --workspace

      - name: Run cargo clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Run cargo fmt
        run: cargo fmt --all -- --check

      - name: Run E2E tests (optional)
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        run: cargo test -- --ignored
```

---

## Recommendations with Rationale

### Recommendation 1: Implement 7-Layer Verification Framework

**Why**: The 7-layer framework provides comprehensive coverage across all system layers, preventing bugs at each level.

**Trade-offs**:
- Pros: Comprehensive coverage, early bug detection, confidence
- Cons: More test code, longer test suites

**Alternatives considered**:
- Only unit tests: Fast, but misses integration issues
- Only E2E tests: Slow, expensive, misses unit bugs

**Adoption priority**: **P0 (Critical for Phase 0)**

---

### Recommendation 2: Use rstest for Unit Tests

**Why**: rstest provides fixture-based testing, reducing boilerplate and improving test readability.

**Trade-offs**:
- Pros: Fixtures, parameterized tests, async support
- Cons: Additional dependency

**Alternatives considered**:
- Rust built-in testing: Simpler, but more boilerplate
- No fixtures: More verbose tests

**Adoption priority**: **P1 (Important for Phase 0)**

---

### Recommendation 3: Use proptest for Property-Based Tests

**Why**: proptest enables property-based testing, finding edge cases that example-based tests miss.

**Trade-offs**:
- Pros: Finds edge cases, automatic test data generation
- Cons: Slower, learning curve

**Alternatives considered**:
- Only example-based tests: Faster, but misses edge cases
- quickcheck: Older, less features

**Adoption priority**: **P1 (Important for Phase 0)**

---

### Recommendation 4: Use wiremock for HTTP Mocking

**Why**: wiremock enables HTTP mocking for integration tests, allowing testing without real LLM backends.

**Trade-offs**:
- Pros: Reliable, fast, comprehensive mocking
- Cons: Additional dependency, learning curve

**Alternatives considered**:
- httpmock: Similar, but less ergonomic
- Real backends: Slower, unreliable

**Adoption priority**: **P1 (Important for Phase 1)**

---

### Recommendation 5: Use assert_cmd for CLI Testing

**Why**: assert_cmd enables CLI testing by spawning subprocesses, capturing output, and asserting on results.

**Trade-offs**:
- Pros: CLI testing, output capture, cross-platform
- Cons: Additional dependency

**Alternatives considered**:
- Manual CLI testing: Error-prone, not automated
- No CLI testing: Untested CLI

**Adoption priority**: **P1 (Important for Phase 1)**

---

### Recommendation 6: Use criterion for Benchmark Testing

**Why**: criterion provides statistical benchmarking, comparing performance across iterations and detecting regressions.

**Trade-offs**:
- Pros: Statistical benchmarks, regression detection
- Cons: Slow, additional dependency

**Alternatives considered**:
- No benchmarks: No performance tracking
- Simple timing: Less accurate, no statistics

**Adoption priority**: **P2 (Nice-to-have for Phase 2)**

---

## Integration Instructions

### Integration Point 1: Phase 0 (Testing Infrastructure)

**What to implement**:
Set up testing infrastructure with rstest, proptest, wiremock. Create test fixtures directory. Set up CI with GitHub Actions.

**File locations**:
- `tests/`: Integration tests
- `tests/fixtures/`: Test fixtures
- `.github/workflows/test.yml`: GitHub Actions workflow
- `Cargo.toml`: Test dependencies

**Code patterns**:

```toml
# Cargo.toml
[dev-dependencies]
rstest = "0.18"
proptest = "1.11"
wiremock = "0.6"
assert_cmd = "2.0"
tempfile = "3.8"
fake = "2.9"
criterion = "0.5"

[[bench]]
name = "benchmarks"
harness = false
```

**Testing requirements**:
- Unit tests for all functions
- Integration tests for components
- CI runs all tests on every commit

---

### Integration Point 2: Phase 1 (Implementation)

**What to implement**:
Implement unit tests for all code. Implement property-based tests for critical invariants. Implement integration tests with wiremock.

**File locations**:
- `src/*/tests.rs`: Unit tests
- `tests/integration/`: Integration tests
- `tests/property/`: Property-based tests

**Code patterns**:

```rust
// tests/integration/backend_integration.rs
use wiremock::{MockServer, Mock, ResponseTemplate};

#[tokio::test]
async fn test_backend_chat() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "chatcmpl-123",
            "choices": [{
                "message": { "content": "Hello!" }
            }]
        })))
        .mount(&mock_server)
        .await;

    let backend = LMStudioBackend::new(mock_server.uri());
    let response = backend.chat(request).await.unwrap();

    assert_eq!(response.choices[0].message.content, "Hello!");
}
```

**Testing requirements**:
- 80%+ test coverage
- All unit tests pass
- All integration tests pass
- CI passes on every commit

---

### Integration Point 3: Phase 2 (Testing)

**What to implement**:
Implement CLI tests with assert_cmd. Implement E2E tests with real LLMs. Implement benchmarks with criterion.

**File locations**:
- `tests/cli/`: CLI tests
- `tests/e2e/`: E2E tests
- `benches/`: Benchmarks

**Code patterns**:

```rust
// tests/cli/cli_tests.rs
use assert_cmd::Command;

#[test]
fn test_cli_run() {
    Command::cargo_bin("agentsdk")
        .unwrap()
        .arg("run")
        .arg("--workflow")
        .arg("tests/fixtures/simple.yaml")
        .assert()
        .success();
}
```

**Testing requirements**:
- CLI tests for all commands
- E2E tests for critical workflows
- Benchmarks for performance-critical code

---

## Validation Criteria

### Criteria 1: Test Suite Passes with rstest Fixtures

**How to verify**:
1. Run unit tests with rstest fixtures
2. Verify all fixtures are loaded correctly
3. Verify parameterized tests work

**Step-by-step verification process**:
```bash
# Run all unit tests
cargo test

# Run tests with specific fixtures
cargo test -- test_agent_thinks

# Verify parameterized tests
cargo test -- test_agent_executes_tasks
```

**Expected outcome**:
- All unit tests pass
- Fixtures are loaded correctly
- Parameterized tests work

**Integration point**: Phase 0 (Testing Infrastructure)

---

### Criteria 2: Property-Based Tests Cover Critical Invariants

**How to verify**:
1. Run property-based tests with proptest
2. Verify invariants are tested
3. Verify edge cases are found

**Step-by-step verification process**:
```bash
# Run property-based tests
cargo test proptest

# Run with increased iterations for thorough testing
PROPTEST_CASES=1000 cargo test proptest

# Verify invariants are tested
cargo test -- test_yaml_roundtrip
cargo test -- test_agent_output_stability
```

**Expected outcome**:
- All property-based tests pass
- Invariants are tested
- Edge cases are found

**Integration point**: Phase 0 (Testing Infrastructure)

---

### Criteria 3: All Integration Tests Pass with wiremock

**How to verify**:
1. Run integration tests with wiremock
2. Verify HTTP mocks are set up correctly
3. Verify all scenarios pass

**Step-by-step verification process**:
```bash
# Run integration tests
cargo test --test integration

# Verify backend integration tests
cargo test --test backend_integration

# Verify wiremock setup
cargo test -- test_backend_integration -- --nocapture
```

**Expected outcome**:
- All integration tests pass
- HTTP mocks are set up correctly
- All scenarios pass

**Integration point**: Phase 1 (Testing Infrastructure)

---

### Criteria 4: CI Pipeline Runs All Tests on Every Commit

**How to verify**:
1. Push a commit to trigger CI
2. Verify GitHub Actions workflow runs
3. Verify all tests pass

**Step-by-step verification process**:
```bash
# Push test commit
git commit -m "test: verify CI"
git push

# Check GitHub Actions
# https://github.com/<org>/<repo>/actions
```

**Expected outcome**:
- GitHub Actions workflow runs
- All tests pass
- No CI failures

**Integration point**: Phase 0 (Testing Infrastructure)

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1: Prevent Drift into Over-Testing

**Drift risk**: Research could recommend too much testing, leading to slow test suites and high maintenance burden.

**Detection method**: Verify test suite runs in <5 minutes for fast feedback. Verify test-to-code ratio is reasonable (<3:1).

**Validation**:
```bash
# Run test suite and measure time
time cargo test --workspace

# Count test lines vs code lines
find tests/ -name "*.rs" -exec wc -l {} + | tail -1
find src/ -name "*.rs" -exec wc -l {} + | tail -1
```

**Correction action**: If over-testing is detected, remove redundant tests and optimize test suite.

---

## Research Tasks

### Task 1: Document 7-Layer Verification Framework

**Files:**
- Create: `./workspace/plans/research/evidence/7-layer-verification-framework.md`

- [ ] **Step 1: Document each verification layer**

Document unit, property-based, integration, API, CLI, E2E, and performance testing layers

Expected output: 7-layer verification framework documentation

- [ ] **Step 2: Document tools and strategies for each layer**

Document tools, execution time, isolation level for each layer

Expected output: Tools and strategies documentation

- [ ] **Step 3: Create 7-layer verification framework document**

Write analysis of 7-layer verification framework with examples

Expected output: `7-layer-verification-framework.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add 7-layer verification framework evidence"`
Expected: Git commit successful

---

### Task 2: Document Mock Strategies for External Dependencies

**Files:**
- Create: `./workspace/plans/research/evidence/mock-strategies.md`

- [ ] **Step 1: Document HTTP mocking strategies**

Document wiremock HTTP mocking for LLM backends and APIs

Expected output: HTTP mocking strategies

- [ ] **Step 2: Document file system mocking strategies**

Document tempfile and file system mocking strategies

Expected output: File system mocking strategies

- [ ] **Step 3: Document database mocking strategies**

Document sled database mocking with in-memory databases

Expected output: Database mocking strategies

- [ ] **Step 4: Create mock strategies document**

Write analysis of mock strategies with code examples

Expected output: `mock-strategies.md`

- [ ] **Step 5: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add mock strategies evidence"`
Expected: Git commit successful

---

### Task 3: Create GitHub Actions CI Workflow

**Files:**
- Create: `.github/workflows/test.yml`

- [ ] **Step 1: Create GitHub Actions workflow**

Create workflow that runs all tests on every commit and pull request

Expected output: GitHub Actions workflow

- [ ] **Step 2: Add caching for cargo registry**

Add caching for cargo registry and dependencies

Expected output: Cached dependencies

- [ ] **Step 3: Add test matrix for different Rust versions**

Add test matrix for stable, beta, and nightly Rust

Expected output: Test matrix

- [ ] **Step 4: Commit workflow**

Run: `git add .github/workflows/test.yml && git commit -m "feat: add GitHub Actions CI workflow"`
Expected: Git commit successful

---

### Task 4: Complete All Research Validation and Integration

**Files:**
- Modify: `opencode/docs/plans/deep-research/05-testing-strategy-research.md`
- Create: `./workspace/plans/research/validation/testing-strategy-validation-report.md`

- [ ] **Step 1: Run all validation scripts**

Run: `bash scripts/validate-research-completeness.sh opencode/docs/plans/deep-research/05-testing-strategy-research.md`
Expected: All validation checks pass

- [ ] **Step 2: Run evidence quality validation**

Run: `bash scripts/validate-evidence-quality.sh opencode/docs/plans/deep-research/05-testing-strategy-research.md`
Expected: 100% evidence quality

- [ ] **Step 3: Run integration completeness validation**

Run: `bash scripts/validate-integration-completeness.sh opencode/docs/plans/deep-research/05-testing-strategy-research.md`
Expected: 100% integration completeness

- [ ] **Step 4: Run traceability validation**

Run: `bash scripts/validate-traceability.sh opencode/docs/plans/deep-research/05-testing-strategy-research.md`
Expected: 100% traceability

- [ ] **Step 5: Create validation report**

Write validation report summarizing all validation results and confirming research completion

Expected output: `testing-strategy-validation-report.md`

- [ ] **Step 6: Commit validation report**

Run: `git add ./workspace/plans/research/validation/ && git commit -m "feat: add testing strategy research validation report"`
Expected: Git commit successful

---

## References

1. **Test Pyramid**: https://martinfowler.com/articles/practical-test-pyramid.html
2. **Rust Testing**: https://rust-lang.github.io/what-we-want-from-a-testing-story/
3. **rstest**: https://docs.rs/rstest/latest/rstest/
4. **proptest**: https://docs.rs/proptest/latest/proptest/
5. **wiremock**: https://docs.rs/wiremock/latest/wiremock/
6. **assert_cmd**: https://docs.rs/assert_cmd/latest/assert_cmd/
7. **criterion**: https://bheisler.github.io/criterion.rs/book/
8. **GitHub Actions**: https://docs.github.com/en/actions
9. **ADR-0001**: Foundation Phase Architecture Decision
10. **ADR-0002**: MVP Queue & Scheduler Architecture Decision

---

**End of Testing Strategy Research Plan**
