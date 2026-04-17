# Contributing to Whitt Execution Engine

Thank you for your interest in contributing! This guide will help you get started.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing](#testing)
6. [Documentation](#documentation)
7. [Pull Request Process](#pull-request-process)
8. [Issue Reporting](#issue-reporting)

---

## Code of Conduct

Be respectful, inclusive, and constructive. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

---

## Getting Started

### Prerequisites

- **Rust**: 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git**: 2.20+
- **Editor**: VS Code, IntelliJ, or similar
- **LLM Provider**: LM Studio, Ollama, or OpenAI (for testing)

### Clone and Build

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/whitt-execution-engine.git
cd whitt-execution-engine

# Add upstream remote
git remote add upstream https://github.com/penwoodj/whitt-execution-engine.git

# Build in debug mode
cargo build

# Run tests
cargo test
```

### Development Setup

```bash
# Install pre-commit hooks (if available)
cargo install cargo-prebuilt

# Install development dependencies
cargo install cargo-watch
cargo install cargo-edit
```

---

## Development Workflow

### Branch Strategy

- **main**: Stable releases only
- **develop**: Integration branch for features
- **feature/xxx**: Feature branches
- **bugfix/xxx**: Bug fix branches
- **hotfix/xxx**: Urgent fixes

### Workflow Example

```bash
# 1. Sync with upstream
git checkout develop
git pull upstream develop

# 2. Create feature branch
git checkout -b feature/my-new-feature

# 3. Make changes
# Edit code, write tests, update documentation

# 4. Test changes
cargo test
cargo clippy -- -D warnings

# 5. Commit changes
git add .
git commit -m "feat: add my new feature"

# 6. Push to fork
git push origin feature/my-new-feature

# 7. Create Pull Request
# Visit GitHub and create PR from feature/my-new-feature to develop
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

**Examples**:
```
feat(parser): add support for custom YAML schemas

Fix validation error when using nested workflows.

Closes #123
```

```
fix(scheduler): resolve race condition in parallel execution

Added mutex lock to shared state to prevent concurrent access issues.

Fixes #456
```

---

## Coding Standards

### Rust Conventions

- Use **rustfmt** for formatting (`cargo fmt`)
- Use **clippy** for linting (`cargo clippy`)
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Prefer **idiomatic Rust** (Result, Option, iterators)
- Use **thiserror** for error types
- Use **anyhow** for application-level errors
- Document **all public APIs** with `///` doc comments
- Use **module-level docs** with `//!` comments

### Code Organization

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library API surface
├── parser.rs         # YAML parsing and validation
├── generator.rs      # Code generation from WorkflowIR
├── scheduler.rs      # Queue, scheduling, execution
├── models.rs         # Model management and routing
├── tools.rs         # Tool implementations (file, web, shell)
├── agents.rs         # Agent scaffolding and orchestration
├── state.rs         # State management and checkpointing
├── metrics.rs       # Metrics collection and reporting
├── logging.rs       # Hierarchical logging system
├── validation.rs    # Workflow validation logic
└── error.rs         # Error types (already implemented)
```

### Error Handling

- Use **thiserror** for library errors:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid YAML syntax: {0}")]
    InvalidSyntax(#[from] serde_saphyr::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),
}
```

- Use **anyhow** for application errors:
```rust
use anyhow::Result;

pub fn execute_workflow() -> Result<()> {
    let workflow = parse_workflow()?;
    Ok(())
}
```

### Async Programming

- Use **tokio** for async operations
- Prefer `async fn` and `.await` syntax
- Use `tokio::spawn` for concurrent tasks
- Use `tokio::select!` for multiple async sources

### Logging

- Use **tracing** for structured logging
- Use appropriate log levels:
  - `error!`: Critical failures
  - `warn!`: Warning conditions
  - `info!`: General informational messages
  - `debug!`: Detailed debugging information
  - `trace!`: Very detailed tracing

```rust
use tracing::{info, error, debug};

info!("Starting workflow execution");
debug!("Model loaded: {}", model_name);
error!("Failed to parse YAML: {:?}", err);
```

---

## Testing

### Unit Tests

Write tests alongside code in `src/`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_workflow() {
        let yaml = "workflow_id: test";
        let result = parse_workflow(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_workflow_returns_error() {
        let yaml = "invalid: yaml";
        let result = parse_workflow(yaml);
        assert!(result.is_err());
    }
}
```

### Integration Tests

Add integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use whitt_execution_engine::*;

#[tokio::test]
async fn test_execute_simple_workflow() {
    let workflow = load_workflow_from_file("test_workflow.yml").await?;
    let result = execute_workflow(&workflow).await?;
    assert!(result.success);
    Ok(())
}
```

### Test Coverage

- Aim for **80%+ coverage** on new code
- Run tests with coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```
- View coverage report: `tarpaulin-report.html`

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_simple_workflow

# Run tests in parallel
cargo test -- --test-threads=4
```

---

## Documentation

### Code Documentation

Document all public APIs:

```rust
/// Parses a YAML workflow string into a WorkflowIR.
///
/// This function validates the YAML structure and ensures all required
/// fields are present. It performs the following validations:
///
/// - YAML syntax is valid
/// - All required sections exist
/// - No circular references in nested workflows
/// - All variable references resolve correctly
///
/// # Arguments
///
/// * `yaml_str` - The YAML workflow string to parse
///
/// # Returns
///
/// Returns a `Result<WorkflowIR, ParseError>` where:
/// - `Ok(WorkflowIR)` - Successfully parsed workflow
/// - `Err(ParseError)` - Validation or parsing error
///
/// # Examples
///
/// ```
/// use whitt_execution_engine::parse_workflow;
///
/// let yaml = r#"
/// workflow_id: test
/// name: "Test Workflow"
/// "#;
/// let result = parse_workflow(yaml);
/// assert!(result.is_ok());
/// ```
pub fn parse_workflow(yaml_str: &str) -> Result<WorkflowIR, ParseError> {
    // Implementation
}
```

### README Documentation

- Keep README updated with:
  - New features
  - API changes
  - Example workflows
  - Known issues

### API Documentation

Generate and host rustdoc:

```bash
# Generate documentation
cargo doc --open

# Build documentation for all dependencies
cargo doc --document-private-items
```

---

## Pull Request Process

### Before Submitting

- [ ] Code follows [coding standards](#coding-standards)
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Commit messages follow [conventional commits](#commit-message-format)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Formatted with `cargo fmt`
- [ ] Tests pass (`cargo test`)

### Pull Request Template

Create a PR with:

```markdown
## Description
Brief description of changes.

## Type
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests pass

## Documentation
- [ ] Code documentation updated
- [ ] README updated (if needed)
- [ ] Examples added (if needed)

## Checklist
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Tests pass (`cargo test`)
- [ ] Breaking changes documented (if any)
```

### Review Process

1. **Automated Checks**: CI runs tests, linting, formatting
2. **Peer Review**: Maintainers review code
3. **Changes Required**: Address review feedback
4. **Approval**: At least one maintainer approval required
5. **Merge**: Squash and merge to `develop`

---

## Issue Reporting

### Bug Reports

Use the bug report template:

```markdown
## Description
Clear and concise description of the bug.

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Framework version: [e.g., 0.1.0]
- LLM Provider: [e.g., LM Studio]

## Logs
Relevant log output.

## Additional Context
Screenshots, example workflows, or other context.
```

### Feature Requests

Use the feature request template:

```markdown
## Description
Clear and concise description of the feature.

## Problem
What problem does this feature solve?

## Proposed Solution
How should this feature work?

## Alternatives
What alternatives have you considered?

## Additional Context
Screenshots, examples, or other context.
```

### Good First Issues

Look for issues labeled `good first issue` or `help wanted` to get started.

---

## Project Structure

```
whitt-execution-engine/
├── src/                    # Source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library API
│   ├── parser.rs          # YAML parsing
│   ├── generator.rs       # Code generation
│   ├── scheduler.rs       # Queue and scheduling
│   └── error.rs          # Error types
├── tests/                 # Integration tests
├── examples/              # Example workflows
├── opencode/             # System-of-record
│   └── docs/            # Documentation
├── Cargo.toml            # Package manifest
├── README.md             # Project documentation
├── CONTRIBUTING.md        # This file
├── LICENSE               # License file
└── .github/             # GitHub configuration
    └── workflows/        # CI/CD workflows
```

---

## Design Philosophy

- **Declarative over Imperative**: Define what, not how
- **Type Safety**: Leverage Rust's type system
- **Performance**: Optimize for execution speed
- **Observability**: Comprehensive logging and metrics
- **Developer Experience**: Easy to use, debug, and extend

---

## Resources

- [README](README.md) - Project overview
- [INSTALL.md](INSTALL.md) - Installation guide
- [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) - Configuration reference
- [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) - Developer onboarding
- [Example Workflows](opencode/docs/reports/requirements/example-workflows/) - Usage examples

---

## Contact

- **Issues**: https://github.com/penwoodj/whitt-execution-engine/issues
- **Discussions**: https://github.com/penwoodj/whitt-execution-engine/discussions
- **Email**: your.email@example.com

---

**Thank you for contributing!**
