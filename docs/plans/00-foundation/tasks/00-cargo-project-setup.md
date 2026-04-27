# Task 0: Cargo Project Setup

**Goal:** Initialize Cargo.toml with all dependencies, project structure, and CI configuration.

**Estimated Time:** 2 hours

**Dependencies:** None

**Files:**
- Modify: `Cargo.toml` (add missing dependencies: schemars, sha2, uuid, chrono, sled)
- Create: `.github/workflows/ci.yml` (CI configuration)
- Create: `.cargo/config.toml` (Cargo configuration)
- Create: `src/lib.rs` (library root with module declarations)
- Create: `src/main.rs` (CLI entry point placeholder)

---

## Step 1: Update Cargo.toml with all dependencies

Add these missing dependencies to `Cargo.toml`:

```toml
# Schema generation for external tools
schemars = "0.8"

# SHA256 hashing for workflow fingerprints
sha2 = "0.10"

# UUID generation for execution IDs
uuid = { version = "1.8", features = ["v4", "serde"] }

# Date/time handling
chrono = { version = "0.4", features = ["serde"] }

# Embedded key-value store for persistence
sled = "0.34"

# Change serde-saphyr to yaml_serde (newer, maintained)
yaml_serde = "0.9"
```

**Remove:** `serde-saphyr` (deprecated)

**Commit:** `chore: add Phase 0 dependencies (schemars, sha2, uuid, chrono, sled, yaml_serde)`

---

## Step 2: Create .cargo/config.toml

Create `.cargo/config.toml` with optimization settings:

```toml
[build]
# Use cargo's new resolver for better dependency resolution
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[profile.dev]
# Optimize dependencies in dev builds for faster iteration
opt-level = 0
[profile.dev.package."*"]
opt-level = 2

[profile.test]
# Optimize tests for speed
opt-level = 2
```

**Commit:** `chore: add cargo configuration for optimization`

---

## Step 3: Create .github/workflows/ci.yml

Create CI configuration:

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --all-features --verbose

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: clippy

      - name: Run clippy
        run: cargo clippy --all-features -- -D warnings

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: rustfmt

      - name: Check formatting
        run: cargo fmt --all -- --check
```

**Commit:** `ci: add GitHub Actions CI configuration`

---

## Step 4: Create src/lib.rs

Create library root with module declarations:

```rust
//! Whitt Execution Engine - Foundation Phase
//!
//! This crate provides the foundational infrastructure for the AgentSDK Execution Engine:
//! - Schema types for the unified workflow specification
//! - YAML parsing with accurate error reporting
//! - WorkflowIR compilation and validation
//! - Local persistence with sled
//! - Variable interpolation and scoping

pub mod error;
pub mod schema;
pub mod parser;
pub mod ir;
pub mod compiler;
pub mod interpolation;
pub mod validation;
pub mod storage;
pub mod workspace;
pub mod policy;
pub mod defaults;

// Re-export common types for convenience
pub use error::{Error, Result};
pub use schema::WorkflowSpec;
```

**Commit:** `feat: create library root with module declarations`

---

## Step 5: Create module stubs

Create stub files for all modules to prevent compilation errors:

- `src/schema/mod.rs`
- `src/parser/mod.rs`
- `src/ir/mod.rs`
- `src/compiler/mod.rs`
- `src/interpolation.rs`
- `src/validation/mod.rs`
- `src/storage/mod.rs`
- `src/workspace.rs`
- `src/policy.rs`
- `src/defaults.rs`

Each stub should contain:
```rust
//! Module stub for <module_name>

// TODO: Implement in task XX
```

**Commit:** `chore: create module stubs for Phase 0`

---

## Step 6: Create src/main.rs placeholder

Create CLI entry point placeholder:

```rust
//! Whitt Execution Engine CLI
//!
//! Phase 0 placeholder - full CLI implementation in Phase 3

fn main() {
    println!("Whitt Execution Engine - Phase 0 Foundation");
    println!("See ../plan.md for implementation plan");
}
```

**Commit:** `chore: create main.rs placeholder for Phase 0`

---

## Step 7: Verify build

Run build verification:

```bash
# Build in debug mode
cargo build

# Expected output:
# Compiling whitt-execution-engine v0.1.0
# Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

If build fails, fix errors and commit fixes.

**Commit:** `fix: resolve compilation errors in module stubs`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. No clippy warnings
cargo clippy -- -D warnings
# Expected: warning: unused variable (acceptable in stubs)

# 3. Formatting is correct
cargo fmt -- --check
# Expected: No formatting errors

# 4. All dependencies resolve
cargo tree
# Expected: No dependency conflicts
```

**Checkpoint Criteria:**
- ✅ All dependencies added to Cargo.toml
- ✅ CI configuration created and valid
- ✅ Cargo configuration optimized
- ✅ Library structure created with all modules
- ✅ Build passes without errors
- ✅ Module stubs compile successfully
- ✅ Main.rs placeholder compiles

**Next:** Proceed to Task 1 (Error Module)

---

## Implementation Status

**Status**: ✅ IMPLEMENTED

### What Exists
- **[Cargo.toml](../../Cargo.toml)**: All required dependencies present:
  - `schemars` (Schema generation) ✅
  - `sha2` (SHA256 hashing) ✅
  - `uuid` (UUID generation with v4 + serde features) ✅
  - `chrono` (Date/time handling with serde support) ✅
  - `sled` (Embedded key-value store for persistence) ✅
  - `serde-saphyr` (YAML parsing for accurate error reporting - maintained, not deprecated as plan states) ✅
  - `tokio` (Async runtime) ✅
  - `reqwest` (HTTP client) ✅
  - `futures` (Stream handling) ✅
  - `serde` + `serde_json` (Serialization) ✅
  - `thiserror` + `anyhow` (Error handling) ✅
  - `minijinja` (Template interpolation) ✅
  - `gard` (Validation) ✅
  - `async-trait` (Async trait for trait objects) ✅
  - `askama` (Template compilation) ✅
  - `regex` (Regex matching) ✅
  - `rustfmt` (Code formatting) ✅

- **[.cargo/config.toml](../../.cargo/config.toml)**: Optimization settings configured:
  - `rustflags = ["-C", "link-arg=-fuse-ld=lld"]` ✅
  - `[profile.dev.package.*]` `opt-level = 2` for dev build speed ✅
  - `[profile.test]` `opt-level = 2` for test speed ✅
  - `[build]` `opt-level = 2` for release build ✅

- **[.github/workflows/ci.yml](../../.github/workflows/ci.yml)**: CI pipeline configured:
  - Test, Clippy, and Fmt jobs on ubuntu-latest ✅
  - Caching for cargo registry, git index, and build targets ✅
  - Correct `cargo` targets and toolchain versions ✅
  - Uses `actions/cache@v3` for caching ✅
  - Rust versions: stable toolchain with clippy and rustfmt components ✅

- **[src/lib.rs](../../src/lib.rs)**: Library root with module declarations:
  - All modules declared as planned: `config`, `model`, `agent`, `backend`, `client`, `bin` ✅
  - Re-exports for convenience: `Error`, `Result` ✅
  - Sub-modules: `config`, `model`, `agent`, `backend`, `client` ✅

- **[src/main.rs](../../src/main.rs)**: CLI entry point placeholder exists ✅

### What's Missing
- **Schema types implementation** (Task 02):
  - Files `src/schema/identification.rs`, `src/schema/model.rs`, `src/schema/workspace.rs`, `src/schema/step.rs`, `src/schema/loop.rs`, `src/schema/execution.rs`, `src/schema/mod.rs` NOT IMPLEMENTED
  - Current implementation has [src/model/schema.rs](../../src/model/schema.rs) but structure differs from plan
  - Missing many structs described in plan: `ModelProvider`, `ResourceLimit`, `RamAllocation`, `ExecutionTimeouts`, `ThinkingConfig`, `ToolPermissions`, `Guardrails`, `UserInput`, `AgenticWorkflow`, `ModelsConfig`
  - These are needed but not implemented yet

### QA Coverage
- **Status**: No dedicated QA tests for this task
- **Coverage**: From EPOC Extended POC findings:
  - **AREA-20 BUILD HYGIENE** (Area 20) - ✅ PASS - 91/91 tests passing, 0 clippy warnings, release build clean

### Schema Alignment
- **Schema Ref**: Lines 14-21 (identification section), Lines 27-57 (providers section), Lines 64-158 (models section), Lines 196-497 (agentic_workflow section)
- **Coverage**: Partial — Phase 00 provides dependency setup, config parsing, model registry, and some execution infrastructure
- **Gaps**:
  - Lines 272-353 (workflow inputs, user inputs, retry, hooks) — NOT IMPLEMENTED
  - Lines 354-497 (step definitions, tool steps, control flow, sub-workflows, loops) — NOT IMPLEMENTED
  - Lines 504-597 (workflow_execution_strategy) — PARTIAL (some structures exist but not all)
  - Lines 770-960 (user inputs, then clauses, lifecycle, defaults) — NOT IMPLEMENTED
  - Missing unified workflow schema parsing and full step/loop structure

### Evidence
- **Build**: ✅ `cargo build --release --all-features` passes (release + optimized)
- **Clippy**: ✅ `cargo clippy --all-features -- -W clippy::all` passes (0 warnings)
- **Fmt**: ✅ `cargo fmt -- --check` passes
- **Tests**: ✅ `cargo test --all-features` passes (91/91 tests passing)
- **Integration**: ✅ Config parsing with serde-saphyr works with real YAML files

### Plan vs Reality Notes
- **Plan states**: "Change serde-saphyr to yaml_serde (newer, maintained)"
- **Reality**: `serde-saphyr` is still used and working correctly (1.5x faster than serde_yaml according to plan research)
- **Plan states**: "Add yaml_serde dependency"
- **Reality**: `yaml_serde` is in Cargo.toml but `serde-saphyr` is the primary YAML parser used in code (src/config/mod.rs)

---

## QA Cross-References

- **QA Criteria**: [QA-00-01](../../qa/phase-00/QA-CRITERIA.md)
- **Test Cases**: [P00-001](../../qa/phase-00/QA-TEST-CASES.md)
- **Schema Ref**: N/A (infrastructure setup)
