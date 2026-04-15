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
//! YAML to Rust AgentSDK - Foundation Phase
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
//! YAML to Rust AgentSDK CLI
//!
//! Phase 0 placeholder - full CLI implementation in Phase 3

fn main() {
    println!("YAML to Rust AgentSDK - Phase 0 Foundation");
    println!("See opencode/docs/plans/00-foundation/plan.md for implementation plan");
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
