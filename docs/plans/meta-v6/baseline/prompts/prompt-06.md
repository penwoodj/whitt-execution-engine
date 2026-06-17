# Baseline User Prompt #6

**Source Session ID**: `ses_295ab428bffe4o3gM1FlZy6HI4`  
**Session Title**: Expand Phase 05 test files (@Sisyphus-Junior subagent)  
**Message ID**: `msg_d6a54bd95001PnuKzPh7GaNfAF`  
**Prompt Length**: 13067 characters  
**Total Message Size**: 3367668 bytes  

---

## Original User Prompt

```
# Developer Guide

Welcome to the YAML to Rust AgentSDK framework! This guide will help you get started with development.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Project Structure](#project-structure)
3. [Development Environment](#development-environment)
4. [Building the Project](#building-the-project)
5. [Running Tests](#running-tests)
6. [Debugging](#debugging)
7. [Code Organization](#code-organization)
8. [Workflow Development](#workflow-development)
9. [Common Tasks](#common-tasks)
10. [Performance Profiling](#performance-profiling)
11. [Best Practices](#best-practices)

---

## Quick Start

### Prerequisites

- **Rust 1.70+**: Install via https://rustup.rs/
- **Git**: For cloning and version control
- **Editor**: VS Code, IntelliJ, or similar
- **LLM Provider**: LM Studio, Ollama, or OpenAI (for testing)

### Initial Setup

```bash
# Clone repository
git clone https://github.com/penwoodj/yaml-to-rust-agentsdk.git
cd yaml-to-rust-agentsdk

# Build in debug mode
cargo build

# Run tests
cargo test

# Run example (when implementation is complete)
cargo run -- run opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml
```

---

## Project Structure

```
yaml-to-rust-agentsdk/
├── src/                          # Source code
│   ├── main.rs                  # CLI entry point (not yet implemented)
│   ├── lib.rs                   # Library API surface (not yet implemented)
│   ├── parser.rs                # YAML parsing (not yet implemented)
│   ├── generator.rs             # Code generation (not yet implemented)
│   ├── scheduler.rs             # Queue and scheduling (not yet implemented)
│   ├── models.rs                # Model management (not yet implemented)
│   ├── tools.rs                 # Tool implementations (not yet implemented)
│   ├── agents.rs                # Agent scaffolding (not yet implemented)
│   ├── state.rs                 # State management (not yet implemented)
│   ├── metrics.rs               # Metrics collection (not yet implemented)
│   ├── logging.rs               # Logging system (not yet implemented)
│   ├── validation.rs            # Workflow validation (not yet implemented)
│   └── error.rs                 # Error types (IMPLEMENTED)
├── tests/                         # Integration tests
│   └── integration_test.rs     # Integration tests (to be added)
├── opencode/                     # System-of-record documentation
│   ├── docs/                    # Documentation (173 files)
│   │   ├── reports/             # Research and requirements
│   │   │   ├── requirements/   # Schema and workflows
│   │   │   ├── roadmap/       # ADRs and research
│   │   │   ├── initial-plans/ # Phase plans
│   │   │   └── benchmark-100-model-userflows/
│   │   └── plans/            # Implementation plans
│   └── sync-iteration-1.md    # Development sync notes
├── Cargo.toml                    # Package manifest (documented)
├── README.md                     # Project documentation (650+ lines)
├── LICENSE                       # Dual license (MIT OR Apache-2.0)
├── INSTALL.md                    # Installation guide
├── ENVIRONMENT_VARIABLES.md        # Configuration reference
├── CONTRIBUTING.md               # Contribution guidelines
├── CHANGELOG.md                 # Version history
├── AUTHORS.md                    # Project contributors
├── CODE_OF_CONDUCT.md           # Community guidelines
└── .github/                      # GitHub configuration
    └── workflows/              # CI/CD workflows
        └── ci.yml            # Main CI pipeline
```

---

## Development Environment

### Recommended Tools

**Editor/IDE**:
- **VS Code**: With rust-analyzer extension
- **IntelliJ Rust**: Full IDE support
- **Vim/Neovim**: With coc-rust or rust-tools.nvim

**CLI Tools**:
```bash
# Cargo extensions for development
cargo install cargo-watch        # Auto-rebuild on file changes
cargo install cargo-edit          # Easy dependency management
cargo install cargo-tarpaulin    # Test coverage
cargo install cargo-outdated      # Check for outdated deps
cargo install cargo-tree         # Dependency visualization
```

**VS Code Extensions**:
- **rust-analyzer**: Official Rust language server
- **CodeLLDB**: Rust debugger
- **Even Better TOML**: Enhanced TOML syntax
- **Error Lens**: Inline error messages
- **Crates**: Dependency management

---

## Building the Project

### Debug Build (Fast Iteration)

```bash
# Build in debug mode (no optimization)
cargo build

# Run debug build
cargo run -- run my_workflow.yml

# Watch mode (auto-rebuild on changes)
cargo watch -- run -- run my_workflow.yml
```

**Debug Build Characteristics**:
- Fast compilation (~2-5 seconds)
- No optimization
- Larger binary size (~50-100 MB)
- Includes debug symbols
- Suitable for development

### Release Build (Optimized)

```bash
# Build in release mode (maximum optimization)
cargo build --release

# Run release build
./target/release/yaml-to-rust-agentsdk --run my_workflow.yml
```

**Release Build Characteristics**:
- Slower compilation (~2-5 minutes)
- Maximum optimization (opt-level 3, LTO)
- Small binary size (~5-15 MB)
- Stripped symbols
- Suitable for production

### Build Features

```bash
# Build with specific features (when implemented)
cargo build --release --features "llamacpp,vulkan"

# Build with all features
cargo build --release --all-features

# Build without default features
cargo build --release --no-default-features
```

---

## Running Tests

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_workflow

# Run tests in parallel
cargo test -- --test-threads=4
```

### Integration Tests

```bash
# Run integration tests only
cargo test --test integration

# Run integration tests with output
cargo test --test integration -- --nocapture
```

### Test Coverage

```bash
# Install tarpaulin (coverage tool)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View coverage report
# Open tarpaulin-report.html in browser
```

**Coverage Goals**:
- **80%+** for new code
- **90%+** for critical paths (parser, scheduler)
- **100%** for error handling

---

## Debugging

### Logging

Enable debug logging:

```bash
# Set log level via environment variable
export RUST_LOG=debug

# Run workflow
cargo run -- run my_workflow.yml
```

### Debugger

**Using VS Code**:
1. Install CodeLLDB extension
2. Set breakpoints in code
3. Press F5 or click "Run and Debug"
4. Use debug console for variable inspection

**Using GDB**:
```bash
# Build debug symbols
cargo build

# Start GDB
gdb target/debug/yaml-to-rust-agentsdk

# GDB commands
(gdb) break main.rs:42
(gdb) run
(gdb) print variable_name
(gdb) continue
```

### Common Debugging Techniques

**Logging Strategy**:
```rust
use tracing::{info, debug, error, instrument};

#[instrument]
pub fn parse_workflow(yaml: &str) -> Result<WorkflowIR> {
    debug!("Parsing workflow: {}", yaml);
    // Implementation
    info!("Workflow parsed successfully");
    Ok(workflow)
}
```

**Assertions**:
```rust
use anyhow::Result;

pub fn validate_workflow(workflow: &WorkflowIR) -> Result<()> {
    assert!(workflow.steps.len() > 0, "Workflow must have at least one step");
    // Implementation
    Ok(())
}
```

---

## Code Organization

### Module Structure

```rust
// lib.rs - Library API surface
pub mod parser;
pub mod generator;
pub mod scheduler;
pub mod models;
pub mod tools;
pub mod agents;
pub mod state;
pub mod metrics;
pub mod logging;
pub mod validation;
pub mod error;

pub use error::Error;
```

### Error Handling

**Library Errors** (using `thiserror`):
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid YAML syntax: {0}")]
    InvalidSyntax(#[from] serde_saphyr::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;
```

**Application Errors** (using `anyhow`):
```rust
use anyhow::{Context, Result};

pub fn execute_workflow(workflow: &WorkflowIR) -> Result<()> {
    let parsed = parse_workflow(&yaml).context("Failed to parse workflow")?;
    Ok(())
}
```

### Async Patterns

```rust
use tokio::task::JoinSet;

pub async fn execute_parallel(steps: Vec<Step>) -> Result<Vec<Output>> {
    let mut join_set = JoinSet::new();

    for step in steps {
        let task = tokio::spawn(async move {
            execute_step(step).await
        });
        join_set.spawn(task);
    }

    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        results.push(result??);
    }

    Ok(results)
}
```

---

## Workflow Development

### Creating Test Workflows

Create test workflows in `tests/workflows/`:

```yaml
# tests/workflows/test_simple.yml
workflow_id: test_simple
name: "Test Simple Workflow"

models:
  primary:
    provider: lmstudio
    model: "llama-3.2-3b-instruct"

execution:
  mode: serial

agentic_workflow:
  - step: test_step
    id: step_1
    model: "${models.primary}"
    input:
      prompt: "Say 'Hello, World!'"
    output:
      save_to: test_output
      format: text
```

### Testing Workflows

```bash
# Test single workflow
cargo run -- run tests/workflows/test_simple.yml

# Test all example workflows
cargo run --test-workflows

# Validate workflow schema
cargo run --validate opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml
```

---

## Common Tasks

### Adding a New Dependency

```bash
# Add dependency via cargo-edit
cargo add serde_json

# Add dependency with features
cargo add tokio --features "full,fs"

# Add dev dependency
cargo add --dev tempfile

# Remove dependency
cargo remove serde_json
```

### Running Clippy

```bash
# Run clippy with warnings as errors
cargo clippy -- -D warnings

# Run clippy on specific crate
cargo clippy --package parser -- -D warnings

# Fix clippy suggestions automatically
cargo clippy --fix --allow-dirty --allow-staged
```

### Formatting Code

```bash
# Format all code
cargo fmt

# Format specific files
cargo fmt -- src/parser.rs

# Check formatting without modifying
cargo fmt -- --check
```

---

## Performance Profiling

### Flame Graph

```bash
# Install flamegraph
cargo install flamegraph

# Generate flame graph
cargo flamegraph -- run my_workflow.yml

# View flame graph
# Open flamegraph.svg in browser
```

### Time Analysis

```bash
# Install hyperfine
cargo install hyperfine

# Benchmark command
hyperfine 'cargo run -- run my_workflow.yml' --warmup 3 --runs 10
```

### Memory Profiling

```bash
# Run with memory profiling
RUST_LOG=memory cargo run -- run my_workflow.yml

# View memory usage
# Check /workspace/metrics/run_*.json for memory metrics
```

---

## Best Practices

### Code Style

1. **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
2. **Document public APIs**: Use `///` for items, `//!` for modules
3. **Prefer idiomatic Rust**: Use Result, Option, iterators
4. **Error handling**: Use `thiserror` for libraries, `anyhow` for applications
5. **Async**: Use `tokio` for all async operations

### Testing

1. **Unit tests**: Test individual functions
2. **Integration tests**: Test component interactions
3. **Test coverage**: Aim for 80%+ coverage
4. **Test names**: Use descriptive names (`test_parse_invalid_yaml_returns_error`)
5. **Test organization**: Group related tests together

### Documentation

1. **README**: Update for new features
2. **Code comments**: Only comment "why", not "what"
3. **Examples**: Provide runnable examples in docstrings
4. **Changelog**: Document breaking changes

### Performance

1. **Profile first**: Don't optimize without measurements
2. **Clone wisely**: Avoid unnecessary clones
3. **Use references**: Pass by reference when possible
4. **Async**: Use `tokio::spawn` for concurrent operations
5. **Memory**: Reuse buffers, avoid allocations

---

## Getting Help

### Resources

- **README**: [README.md](README.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Installation**: [INSTALL.md](INSTALL.md)
- **Environment Variables**: [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md)
- **Example Workflows**: `opencode/docs/reports/requirements/example-workflows/`

### Asking Questions

- **GitHub Discussions**: https://github.com/penwoodj/yaml-to-rust-agentsdk/discussions
- **Issues**: https://github.com/penwoodj/yaml-to-rust-agentsdk/issues

### Reporting Bugs

See [CONTRIBUTING.md](CONTRIBUTING.md#issue-reporting) for bug report template.

---

## Next Steps

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) for development workflow
2. Review [Architecture Decisions](opencode/docs/reports/roadmap/) for design rationale
3. Explore [Example Workflows](opencode/docs/reports/requirements/example-workflows/)
4. Choose a **first issue** to work on (look for "good first issue" label)
5. Start coding!

---

**Happy coding!** 🦀

```

---
