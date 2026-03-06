# Transpiler Architecture

**Scope**: Transpiler-specific components only (YAML schema, parsing, code generation, testing)

**Version**: 1.0
**Date**: 2026-03-05

---

## Overview

The transpiler converts YAML workflow definitions into executable Rust code that uses the AgentSDK runtime. The architecture follows a multi-stage pipeline:

```
YAML Input → Parser → AST → Validator → WorkflowIR → Normalizer → Codegen → Rust Output
```

Each stage is independent, deterministic, and testable.

---

## Architecture Layers

### Layer 1: Input & Parsing

**Components:**
- **Schema Definition** (`schema.rs`)
  - YAML/JSON Schema for workflow DSL
  - Node type definitions: `step`, `tool`, `model`, `verify`, `repair`, `review`, `effect`
  - Field constraints, validation rules, versioning

- **Parser** (`parser.rs`)
  - YAML → AST conversion
  - Source location preservation (line numbers, column positions)
  - Stable node ID generation
  - Error reporting with context

- **Static Validation** (`validator.rs`)
  - Required field checking
  - Type validation
  - Constraint validation
  - Cycle detection
  - Permission violation detection

**Dependencies:**
- `serde-saphyr`: YAML parsing with garde validation
- `garde`: Validation rules
- Schema definition

**Outputs:**
- AST with source locations
- Validation errors (if any)

---

### Layer 2: Intermediate Representation

**Components:**
- **AST** (`ast.rs`)
  - Tree structure of parsed YAML
  - Node types mirroring schema
  - Stable IDs for cross-references
  - Location metadata

- **WorkflowIR** (`ir.rs`)
  - Typed intermediate representation
  - Graph structure (DAG)
  - State machine semantics
  - Policy bindings
  - Effect declarations

- **Normalizer** (`normalize.rs`)
  - Canonical form transformation
  - Stable sorting
  - ID canonicalization
  - Deterministic output

**Dependencies:**
- `petgraph`: Graph data structures and algorithms
- AST layer

**Outputs:**
- Typed IR with stable IDs
- Normalized IR (cacheable)

---

### Layer 3: Code Generation

**Components:**
- **Template Engine** (`templates/`)
  - Askama templates for code generation
  - Agent creation templates
  - Tool binding templates
  - Hook generation templates
  - Layout templates

- **AgentSDK Mapping** (`mapping.rs`)
  - IR → AgentSDK trait call generation
  - Agent creation code
  - Tool call code
  - State management code
  - Scheduling hook code

- **Tool Bindings** (`tools.rs`)
  - YAML tool → Rust trait implementation
  - Serialization/deserialization
  - Permission check integration
  - Cancellation support
  - Logging integration

- **Runtime Hooks** (`hooks.rs`)
  - Run ID tracking
  - Checkpoint generation
  - Status updates
  - Artifact path emission

- **Project Layout** (`layout.rs`)
  - Cargo.toml generation
  - `src/` structure generation
  - Module file generation
  - Build metadata

- **Packaging Selector** (`packaging.rs`)
  - Standalone strategy (include runtime)
  - Import backend strategy (reference crate)
  - Inline runtime strategy (embed runtime)

**Dependencies:**
- `Askama`: Template engine
- IR layer
- AgentSDK crate

**Outputs:**
- Rust source files
- Cargo project structure
- Compiled binary

---

### Layer 4: Quality & Safety

**Components:**
- **Linter** (`linter.rs`)
  - Timeout lint (missing timeouts)
  - Name lint (ambiguous names)
  - Loop lint (unbounded loops)
  - Permission lint (unapproved operations)

- **Effect Analyzer** (`effects.rs`)
  - Preflight effect reports
  - File operation tracking
  - Tool usage tracking
  - Model usage tracking

- **Verifier Integration** (`verifier.rs`)
  - Schema verifier stubs
  - Diff verifier stubs
  - Test verifier stubs
  - Verifier call site generation

- **Sandbox** (`sandbox.rs`)
  - Path guards (no outside project directory)
  - Dependency allowlists
  - Import restriction enforcement

- **Capability-Aware Lowering** (`lowering.rs`)
  - Device capability profiles
  - Task splitting for low-memory
  - Resource adaptation
  - Lowering strategies

**Dependencies:**
- AST/IR layers
- Validation layer
- Generation layer

**Outputs:**
- Lint warnings/errors
- Effect reports
- Verifier call sites
- Sanitized generated code

---

### Layer 5: Build & Testing

**Components:**
- **Build Orchestration** (`build.rs`)
  - Reproducible builds
  - Cargo.lock generation
  - Pinned toolchain
  - Offline build support

- **Compilation Cache** (`cache.rs`)
  - Cache key generation (hash YAML + schema + toolchain)
  - Cache storage
  - Cache lookup
  - Invalidation logic

- **Golden Tests** (`tests/golden/`)
  - Snapshot testing infrastructure
  - Sample YAML inputs
  - Expected Rust outputs
  - Regression detection

- **Property Tests** (`tests/properties/`)
  - Parser property tests
  - Validator property tests
  - Normalizer property tests
  - Randomized input generation

**Dependencies:**
- Generated code
- Test frameworks (`insta`, `proptest`)

**Outputs:**
- Compiled binary
- Test results
- Cache hits/misses

---

### Layer 6: CLI & Documentation

**Components:**
- **CLI** (`main.rs`)
  - Command line parsing (clap)
  - Subcommands: `compile`, `validate`, `lint`, `migrate`
  - Configuration loading
  - Output formatting

- **Schema Evolution** (`migrate.rs`)
  - Migration scripts
  - Version transformation
  - Metadata preservation

- **Schema Docs Generator** (`docs.rs`)
  - Reference documentation
  - Node type docs
  - Field descriptions
  - Version change docs

- **Examples** (`examples/`)
  - Sample workflows
  - Getting started examples
  - Feature demonstrations

**Dependencies:**
- All transpiler layers
- Clap crate

**Outputs:**
- CLI binary
- Documentation
- Example workflows

---

## Data Flow

### Compilation Flow

```
1. User provides YAML workflow file
   ↓
2. Parser reads YAML, builds AST with source locations
   ↓
3. Validator checks AST against schema
   - If invalid: return validation errors
   ↓
4. AST converts to WorkflowIR (graph + state machine)
   ↓
5. Normalizer creates canonical IR (sorted, stable IDs)
   ↓
6. Effect analyzer generates preflight report
   - User reviews effects
   ↓
7. Capability-aware lowering adapts IR for device profile
   ↓
8. Code generator applies Askama templates
   - AgentSDK mapping layer generates trait calls
   - Tool bindings generate trait implementations
   - Runtime hooks generate instrumentation
   ↓
9. Layout generator creates Cargo project structure
   ↓
10. Packaging selector applies strategy (standalone/import/inline)
    ↓
11. Build orchestration compiles generated code
    ↓
12. Binary is output to user
```

### Caching Flow

```
1. Compute cache key: hash(YAML + schema_version + toolchain + backend_contract)
2. Check cache for existing compilation
   - If hit: Return cached binary
   - If miss: Continue compilation
3. Run compilation pipeline
4. Store result in cache
5. Return binary
```

---

## Module Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── schema.rs            # Schema definitions and versioning
├── parser.rs            # YAML → AST parser
├── ast.rs              # AST data structures
├── validator.rs         # Static validation
├── linter.rs           # Linting rules
├── ir.rs               # WorkflowIR data structures
├── normalize.rs         # Deterministic normalization
├── mapping.rs          # IR → AgentSDK mapping
├── tools.rs            # Tool binding generation
├── hooks.rs           # Runtime hooks generation
├── layout.rs           # Project layout generation
├── packaging.rs        # Packaging strategy selector
├── effects.rs          # Effect analysis
├── verifier.rs         # Verifier integration
├── sandbox.rs          # Sandboxing and path guards
├── lowering.rs         # Capability-aware lowering
├── build.rs            # Build orchestration
├── cache.rs           # Compilation caching
├── migrate.rs          # Schema migrations
└── docs.rs            # Documentation generation

templates/
├── agent.askama         # Agent creation template
├── tool.askama          # Tool binding template
├── hook.askama          # Runtime hook template
├── main.askama         # Main function template
├── cargo.askama         # Cargo.toml template
└── module.askama        # Module file template

tests/
├── golden/             # Snapshot tests
│   ├── simple_step.yml
│   ├── complex_workflow.yml
│   └── ...
└── properties/         # Property tests
    ├── parser_proptest.rs
    ├── validator_proptest.rs
    └── ...

examples/
├── simple.yml          # Simple step workflow
├── with_loops.yml      # Loop examples
├── multi_agent.yml     # Multi-agent workflow
└── tools.yml          # Tool usage examples
```

---

## Key Design Decisions

### 1. Why Separate AST and IR?
- **AST**: Preserves source structure and location data for error reporting
- **IR**: Optimized for code generation, graph operations, and caching
- **Separation**: Enables independent evolution of parsing and codegen

### 2. Why Askama for Templates?
- Type-safe: Compile-time template validation
- Fast: Pre-compiled templates
- Maintainable: Clear separation of logic and presentation
- Deterministic: No runtime string manipulation

### 3. Why Graph-Based IR?
- Natural fit for workflow DAGs
- Enables cycle detection and topological sorting
- Supports visualization and analysis
- Leverages petgraph ecosystem

### 4. Why Separate Validation and Linting?
- **Validation**: Binary pass/fail, prevents compilation
- **Linting**: Warnings, suggestions, configurable severity
- **Separation**: Allows workflows to compile with warnings if desired

### 5. Why Three Packaging Strategies?
- **Standalone**: Easy distribution, larger binary
- **Import Backend**: Smaller binary, requires runtime installation
- **Inline Runtime**: Self-contained, medium binary size
- **Choice**: Tradeoffs vary by deployment scenario

---

## Invariants

1. **Determinism**: Same YAML input always produces same Rust output
2. **Idempotence**: Running transpiler twice on unchanged input produces identical results
3. **Locality**: Transpiler does not write outside output directory
4. **Transparency**: All effects are previewed before code generation
5. **Safety**: Malicious YAML cannot escape sandbox or import unsafe crates
6. **Reproducibility**: Generated builds are reproducible with Cargo.lock

---

## Performance Targets

- **Parsing**: <100ms for 1000-line workflow
- **Validation**: <200ms for medium workflow
- **Code Generation**: <5s for complex workflow
- **Build**: <30s for generated project (cold cache)
- **Cache Hit**: <1s (no compilation needed)

---

## Testing Strategy

### Golden Tests
- Input: Sample YAML workflows
- Expected: Compiled Rust code snapshots
- Purpose: Detect regressions in code generation
- Framework: `insta` crate

### Property Tests
- Input: Randomized YAML (valid and invalid)
- Expected: Parsers validate, validators catch errors
- Purpose: Explore edge cases, ensure robustness
- Framework: `proptest` crate

### Integration Tests
- Input: Full workflow YAMLs
- Expected: Compiled binary runs correctly
- Purpose: End-to-end validation
- Framework: Rust built-in testing

### Benchmarks
- Metrics: Parse time, validation time, codegen time, build time
- Purpose: Performance regression detection
- Framework: `criterion` crate

---

## Future Extensions

### Potential Enhancements
- **IDE Integration**: LSP for YAML autocomplete and validation
- **Visual Editor**: Drag-and-drop workflow builder
- **Live Preview**: Show generated code as user edits YAML
- **Incremental Codegen**: Regenerate only changed modules
- **Multi-Language Targets**: Generate Python, Go, JavaScript

### Out of Scope (Runtime)
- **Scheduler**: Queue management, worker pools
- **Execution**: Running workflows, managing state
- **Monitoring**: Tracing, metrics collection
- **Model Management**: Loading, unloading models

---

## References

- **Requirements**: R0054-R0075 (Transpiler Internals)
- **Research Plan**: `transpiler_research_plan.yml`
- **Implementation Plan**: `transpiler_implementation_plan.yml`
- **Related Projects**:
  - GitHub Actions (workflow syntax)
  - Argo Workflows (YAML DSL)
  - Prefect (flow schemas)
  - Rust compiler (AST/IR design)
