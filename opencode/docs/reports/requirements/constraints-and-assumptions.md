# Constraints and Assumptions

## Overview

This document articulates the constraints and assumptions underpinning the AutoAgents SDK project. These decisions shape the architecture, scope, and implementation approach.

---

## Assumptions

### A1: Local-First Execution Model

**Assumption**: The AutoAgents SDK focuses on local LLM backends (LM Studio, Ollama, llama.cpp) rather than cloud APIs (OpenAI, Anthropic, Google).

**Rationale**:
- **Privacy**: User data stays on local machine
- **No API costs**: No per-token billing, fixed hardware costs
- **Deterministic**: Local inference more predictable than cloud
- **Offline capable**: Can run without internet (after initial model download)
- **Full control**: Direct access to model weights, quantization, backend

**Implications**:
- Model management handled locally (download, install, cache, unload)
- No rate limiting from cloud providers
- User responsible for GPU/CPU resources
- Network access only for model downloads, not per-request

---

### A2: YAML-First Configuration

**Assumption**: Workflows are defined in YAML files and transpiled to executable Rust code.

**Rationale**:
- **Declarative**: YAML is human-readable, version-controlled
- **Separation of concerns**: Workflow definition (YAML) separate from execution logic (Rust)
- **Tool ecosystem**: Leverage existing YAML tooling (editors, validators)
- **Schema validation**: Catch errors before compilation, not at runtime

**Implications**:
- Transpiler is critical path (YAML → AST → IR → Rust code)
- Schema changes require re-transpilation of workflows
- Runtime errors require looking at transpiled code, not YAML
- Debugging may need to inspect generated Rust code

---

### A3: Rust-First Tooling and Ecosystem

**Assumption**: The project uses Rust throughout, leveraging the mature Rust ecosystem for async runtime, serialization, and web clients.

**Rationale**:
- **Performance**: Zero-cost abstractions, memory safety without GC overhead
- **Maturity**: Tokio ecosystem (5M+ stars), production-proven
- **Type safety**: Compile-time guarantees, no runtime type errors
- **Package manager**: Cargo for dependency management, reproducible builds

**Implications**:
- Dependency choices fixed based on critical evaluation (serde-saphyr, tokio, reqwest)
- C-style bindings acceptable (llama.cpp via llamacpp-2)
- Async/await used throughout (tokio ecosystem)
- Standard async patterns (Arc, Mutex, channels, async traits)

---

### A4: Single-Process Execution Model

**Assumption**: The workflow engine executes one workflow per process initially, with optional future support for distributed execution.

**Rationale**:
- **Simplicity**: Start with in-process execution, add distribution later if needed
- **Predictability**: Single-process easier to reason about, test, debug
- **Resource control**: Process-level resource limits straightforward
- **State management**: In-memory state sufficient for single-process

**Implications**:
- No worker pools, message queues, or cluster coordination initially
- Parallelism limited to within one process (tokio tasks, threads)
- Concurrency: Multiple workflows can run in parallel across processes, not within
- Future enhancement: Distributed execution can add complexity (gRPC, consensus, fault tolerance)

---

### A5: Unified Schema as Source of Truth

**Assumption**: The unified workflow schema v2.0 is the authoritative source for workflow structure, validation, and transpilation.

**Rationale**:
- **Single definition point**: Prevents schema drift across YAML files, docs, and transpiler
- **Type safety**: Schema-driven code generation guarantees valid Rust types
- **Validation at load time**: Catch errors before execution, not during
- **Versioning**: Semver tracking for schema migrations

**Implications**:
- All workflow YAMLs must validate against unified schema
- Transpiler codegen driven by schema (schema.rs → templates → Rust code)
- Schema changes require updating transpiler and re-running workflows
- Documentation generated from schema (auto-generated, always in sync)

---

### A6: Developer Hardware Constraints

**Assumption**: Development and initial testing happens on developer machines with realistic hardware constraints.

**Rationale**:
- **Performance realism**: Test on hardware users have, not development servers
- **Resource constraints**: Typical developer laptops/desktops (8-32GB RAM, 1-2 GPUs)
- **Platform diversity**: Linux primary (Vulkan support), Windows/macOS secondary
- **Scalability testing**: Large-scale benchmarking on higher-end hardware reveals limits

**Target Hardware Profile:**
- RAM: 16GB minimum, 32GB recommended
- GPU: 1 GPU minimum (NVIDIA RTX 3060 or equivalent), 2 GPUs recommended
- Storage: 50GB free space for models, KV-cache, outputs
- Network: Gigabit ethernet for local providers (Ollama, llama.cpp)

---

### A7: Security Model: Per-Tool Sandboxing

**Assumption**: Tools executed by workflows must be sandboxed with granular permissions (file access, network, shell).

**Rationale**:
- **Least privilege**: Tools run with minimum required permissions
- **Prevent escapes**: Sandboxing prevents accessing project directory or system files
- **Auditability**: All tool calls logged with inputs and outputs
- **Reproducibility**: Deterministic execution, no randomness from environment

**Implications**:
- Sandlock v0.5.0 used for Linux sandboxing (Landlock + seccomp)
- Windows/macOS sandboxing deferred (less mature tooling, no Landlock equivalent)
- Tool permissions configured per tool type (file read/write, network allowlists)
- Workflow-level errors don't expose system state

---

## Constraints

### C1: Platform Support

**Constraint**: Primary platform is Linux with Vulkan support. Windows and macOS support is secondary.

**Rationale**:
- **Vulkan availability**: Cross-vendor GPU acceleration via MoltenVK on macOS, native on Linux/Windows
- **Sandboxing**: Landlock (Linux-only) provides fine-grained permissions
- **LLM backends**: LM Studio, Ollama, llama.cpp all support Linux natively
- **CI/CD**: Most CI runners are Linux-based (GitHub Actions, GitLab)

**Windows/macOS Support**:
- **Initial version**: CPU-only inference (no Vulkan acceleration)
- **Limited features**: No per-tool sandboxing (no Landlock equivalent)
- **File system**: Different path handling (Windows: `C:\`, macOS: `/Users/`, Linux: `/home/`)
- **Priority**: Linux features ship first, Windows/macOS features deferred

---

### C2: Resource Limits

**Constraint**: The workflow engine must respect configurable resource limits (memory, CPU, GPU, concurrency).

**Rationale**:
- **Prevent OOM**: Avoid swapping which kills performance
- **Fair sharing**: Multiple workflows/agents shouldn't monopolize system
- **Predictable performance**: Bounded resources enable consistent timing
- **Stability**: Limits prevent runaway processes from consuming all resources

**Resource Limits:**
- **Memory per workflow**: Configurable (default: 4GB, max: 16GB)
- **CPU per workflow**: Configurable (default: 2 cores, max: 8 cores)
- **GPU per workflow**: Configurable (default: 50%, max: 100% of one GPU)
- **Concurrent workflows**: Configurable (default: 4, max: 16 processes)
- **Timeout per operation**: Configurable (default: 5min, max: 1hour)

---

### C3: File System and Security

**Constraint**: Workflows run in isolated directories with restricted file access permissions.

**Rationale**:
- **Prevent escapes**: Workflows can't access files outside designated workspace
- **Reproducibility**: Isolated execution prevents state pollution
- **Safety**: Prevent accidental deletion of system files
- **Auditability**: Clear boundary for logging and debugging

**File System Constraints:**
- **Workspace root**: Configurable (default: `$HOME/.agent-sdk/workflows/`)
- **Path guards**: Absolute path escape prevention (`../../../`, `/etc/`)
- **Allowed directories**: Configurable allowlists (e.g., `./workspace`, `./output`)
- **Temporary files**: Scoped to workspace (no system-wide `/tmp`)

---

### C4: Dependency Constraints

**Constraint**: Use fixed dependency versions based on critical evaluation. No runtime dependency resolution.

**Rationale**:
- **Reproducibility**: Fixed versions guarantee identical builds across machines
- **Security**: Vetted dependencies reduce vulnerability surface
- **Performance**: Benchmark-validated choices (serde-saphyr, tokio)
- **Stability**: Mature crates with long maintenance history

**Fixed Dependencies:**
```toml
[dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
serde-saphyr = "0.0.23"  # Fixed, fastest, pure Rust
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }  # HTTP client

# Async runtime
async-trait = "0.1"

# YAML
garde = { version = "0.20", features = ["rules"] }

# Template engine
minijinja = "0.2"  # Jinja2-compatible
askama = "1.0"  # For codegen only

# Agent framework (future)
# rig = { version = "0.1" }  # Deferred to Phase 02

# Workflow engine
# treadle = { version = "0.2" }  # Deferred to Phase 01

# Testing
proptest = "1.0"
insta = "1.0"

# Optional Vulkan support
llamacpp-2 = { version = "0.1", optional = true }  # For local GPU
vulkan-rs = { version = "0.1", optional = true }
```

**Allowed Dependency Criteria:**
- Must appear in `critical-evaluation.md` with justification
- Must have >10K downloads/month or be in rust-lang organization
- Must have stable release in last 6 months
- Must pass `cargo audit` and `cargo geiger` checks

---

### C5: Scope Constraints

**Constraint**: Initial scope is core workflow execution engine only. Multi-provider abstraction, distributed execution, and advanced features are out of scope.

**Rationale**:
- **MVP first**: Ship working workflow engine before expanding
- **Learn before building**: Real-world usage patterns inform design of advanced features
- **Complexity management**: Smaller scope enables deeper quality
- **Documentation quality**: Focused scope enables comprehensive docs

**Out of Scope (Deferred to Future Phases):**
- Multi-provider abstraction (beyond current `model_router`)
- Distributed workflow execution (worker pools, message queues)
- Real-time monitoring (metrics collection, tracing, dashboards)
- Advanced scheduling (event-driven triggers, cron workflows)
- Autonomy metrics (self-improvement, optimization loops)
- Cloud provider support (OpenAI, Anthropic, Google APIs)

---

### C6: Performance Constraints

**Constraint**: The workflow engine must meet performance targets for common operations.

**Rationale**:
- **User experience**: Long operations frustrate users, short feedback loop is essential
- **Adoption threshold**: Slower than alternatives prevents tool usage
- **Competitive parity**: Match or exceed open-source competitors
- **Resource efficiency**: Don't waste CPU/GPU cycles

**Performance Targets:**
| Operation | Target | Measurement | Success Criteria |
|-----------|--------|-------------|------------------|
| **Transpilation** | <5s for complex workflow | End-to-end time | 95% of workflows < 5s |
| **Model load** | <30s for 7B model | Load time | 95% of loads < 30s |
| **Prompt execution** | <10s P95 latency | Per-request timing | 95% of prompts < 10s P95 |
| **YAML validation** | <500ms for medium workflow | Parse + validate | 95% < 500ms |
| **Startup time** | <2s cold start | First prompt latency | 95% of first prompts < 2s |
| **Memory overhead** | <500MB per workflow | RSS measurement | Peak memory - base < 500MB |

---

### C7: Testing Constraints

**Constraint**: Comprehensive testing is required before release. Unit tests, integration tests, and benchmarks must pass.

**Rationale**:
- **Quality assurance**: Catch regressions, ensure correctness
- **Performance regression**: Benchmarks detect performance degradation
- **Edge cases**: Property-based testing explores failure modes
- **Documentation**: Tests serve as usage examples

**Testing Requirements:**
- **Unit tests**: >90% code coverage for core modules
- **Integration tests**: All example workflows (52 YAMLs) must pass
- **Golden tests**: Snapshot testing for code generation
- **Property tests**: `proptest` for parsers, validators, normalization
- **Benchmarks**: `criterion` for performance regression detection
- **CI/CD**: All tests must pass on every commit

---

### C8: Documentation Constraints

**Constraint**: Documentation must be comprehensive, accurate, and kept in sync with code changes.

**Rationale**:
- **Onboarding**: Good docs reduce learning curve
- **Maintenance**: Accurate docs reduce support burden
- **Adoption**: Users evaluate tool quality from docs
- **Trust**: Complete docs build user confidence

**Documentation Requirements:**
- **README.md** for every major module/package
- **API docs**: Generated from schema (doc.rs module)
- **Examples**: At least one example per feature
- **Migration guide**: How to upgrade between schema versions
- **Error messages**: Clear, actionable, with line numbers
- **Architecture docs**: Explain design decisions and trade-offs
- **ADR process**: Document all architectural decisions with evidence

---

## Constraint Interactions

### Assumptions × Constraints Matrix

| Assumption | C1 Platform | C2 Resources | C3 File System | C4 Dependencies | C5 Scope | C6 Performance | C7 Testing | C8 Docs |
|-----------|-------------|-----------------|-----------------|-----------------|---------|--------------|---------|--------|
| **A1: Local-first** | ✅ Supports | ✅ Must respect | ✅ Enforces | ✅ Local deps only | ✅ Single proc | ✅ Measurable | ✅ Required | ✅ Comprehensive |
| **A2: YAML-first** | ✅ Cross-platform | ✅ Resource limits apply | ✅ Workspace dirs | ✅ Fixed versions | ✅ In scope | ✅ Fast enough | ✅ Schema-driven |
| **A3: Rust-first** | ✅ Native support | ✅ Concurrency limits | ⚠️ No Windows sandbox | ✅ Fixed ecosystem | ✅ In scope | ✅ Fast enough | ✅ Rust idioms |
| **A4: Single-process** | ✅ Works | ✅ Per-workflow limits | ✅ Isolated | ✅ Tokio-based | ✅ In scope | ✅ Responsive | ✅ Inline docs |
| **A5: Schema-driven** | ✅ Cross-platform | ✅ Validation required | ✅ Type-safe | ✅ Schema in Cargo | ✅ In scope | ✅ Type-safe | ✅ Auto-generated |
| **A6: Dev hardware** | ✅ Supports | ✅ Must respect | ⚠️ Path differences | ✅ Fixed versions | ✅ Realistic targets | ✅ Test coverage | ⚠️ Assumes dev env |
| **A7: Security model** | ✅ Linux full | ✅ Limits enforce | ✅ Workspace enforced | ✅ Sandlock dep | ✅ In scope | ✅ Fast enough | ✅ Security docs |

**Legend**:
- ✅ = Fully compatible
- ⚠️ = Partial compatibility or constraint requires work
- ❌ = Incompatible

### Key Frictions

1. **Windows/macOS Sandbox Gap**: No equivalent to Landlock, per-tool sandboxing limited
2. **Platform testing gap**: CI primarily Linux, Windows/macOS testing deferred
3. **Scope creep risk**: Many features could be in scope, strict discipline needed
4. **Performance vs. correctness trade-off**: Emphasis on Rust safety may reduce raw performance

---

## Mitigation Strategies

### M1: Windows/macOS Sandboxing

**Mitigation**: Use alternative approaches for Windows/macOS sandboxing:
- **User permission prompts**: Request explicit permission before tool execution
- **Path validation**: Stricter path checks on Windows/macOS
- **Containerization**: Recommend Docker/podman for production Windows/macOS
- **Deferred feature**: Document sandboxing requirements for Windows/macOS, implement later

### M2: Platform Testing Gap

**Mitigation**: Cross-platform testing strategy:
- **GitHub Actions**: Add Windows/macOS runners for CI
- **Smoke tests**: Basic functionality tests on all platforms
- **Community testing**: Early beta program for Windows/macOS users
- **Feature flags**: Use feature gates for platform-specific code

### M3: Scope Creep Prevention

**Mitigation**: Strict scope management:
- **ADR process**: Document all out-of-scope requests with justification
- **Phase planning**: Each phase has explicit boundaries and success criteria
- **TODO tracking**: Public TODO list for deferred features, not hidden in comments
- **Review checkpoints**: Phase gates review scope before proceeding

---

## Decision Impact

### How Constraints Shape Architecture

**Single-Process + Resource Limits**:
- In-process state management (no distributed complexity initially)
- Tokio runtime for async tasks and concurrent workflows
- Resource accounting per workflow (memory, CPU, GPU)
- Bounded queues for fair resource sharing

**YAML-First + Schema-Driven**:
- Transpiler architecture: YAML → AST → IR → Rust code (4-layer pipeline)
- Schema validation at parse time (not runtime)
- Type-safe code generation via Askama templates
- Schema versioning for migrations

**Local-First + Security Model**:
- Local LLM backends: ModelProvider trait abstraction
- Per-tool sandboxing via Sandlock (Linux) or prompts (Windows/macOS)
- Workspace isolation with configurable bounds
- Audit logging of all tool calls with inputs/outputs

---

## Assumption Validation

### Validation Checklist

| Assumption | How to Validate | Evidence of Success |
|-----------|------------------|---------------------|
| A1: Local-first | Run all examples on local hardware | Benchmark completes successfully |
| A2: YAML-first | All 52 example workflows validate | CI pipeline green |
| A3: Rust-first | No alternative languages needed | Repository uses Rust only |
| A4: Single-process | Single workflow execution works | Integration tests pass |
| A5: Schema-driven | All workflows pass schema validation | No runtime validation errors |
| A6: Dev hardware | Performance targets met on dev machines | Benchmarks meet targets |
| A7: Security model | Tools sandboxed successfully | Sandlock integration tested |
| A8: Docs | Users can follow docs to use tool | User acceptance testing |

---

## Summary

The AutoAgents SDK operates under these fundamental constraints:

1. **Local-first execution**: No cloud APIs, user data stays local
2. **YAML-based configuration**: Declarative workflows transpiled to Rust
3. **Rust ecosystem**: Fixed dependencies from critical evaluation
4. **Single-process model**: In-process execution initially
5. **Schema-driven**: Unified schema v2.0 as source of truth
6. **Resource bounded**: Configurable limits per workflow
7. **Security-focused**: Per-tool sandboxing with workspace isolation
8. **Platform-constrained**: Linux primary, Windows/macOS secondary

These constraints intentionally limit initial scope but provide a solid foundation for expansion (multi-provider, distributed execution, advanced scheduling) while ensuring quality and usability from day one.
