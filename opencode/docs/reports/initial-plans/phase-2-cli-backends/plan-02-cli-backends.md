# Plan-02: CLI, Backends, and Networking Boundary

**Plan ID**: plan-02
**Phase**: Phase 2 - CLI & Backends
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0003 (CLI, backends, and networking boundary)
**Related Research Plan**: research-plan-03-networking-ui-backends.yml
**Estimated Time**: 6-8 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue & Scheduler)

---

## Overview

This plan implements CLI interface, backend provider abstraction, networking boundary controls, and tool node support for the YAML to Rust AgentSDK transpiler.

**Key Features**:
- **CLI-first approach**: Rich command-line interface for all runtime operations
- **Backend provider abstraction**: Normalize access to multiple local model runners (llama.cpp, LM Studio, vLLM)
- **Packaging strategy**: Generated Rust projects can use either standalone binaries or shared backend crate
- **Networking boundary**: Explicit opt-in with provenance tracking for remote operations
- **Progressive result delivery**: Stream partial results for interactive flows
- **Custom Rust tool support**: Tool nodes with declared effects and permissions
- **Dual execution modes**: Direct execution vs code generation
- **Self-improvement loop**: Execution logs → workflow improvements → regeneration

**Key Deliverables**:
- CLI with rich ergonomics and tab completion
- Provider abstraction trait and implementations
- Packaging framework (standalone vs shared backend)
- Networking opt-in controls with audit logging
- Progressive result streaming infrastructure
- Tool node framework with permission system
- Code generation templates for production mode
- Self-improvement pipeline infrastructure

---

## Code Review

### ADR-0003 Summary

**Decision**: CLI-first path before UI with shell-agnostic runtime core, provider abstraction for local runners, explicit networking boundary, and dual execution modes.

**Key Requirements**:
1. **Runtime Core**: Shell-agnostic, exposed through CLI/TUI first
2. **Model Access**: Provider abstraction normalizing local runners (llama.cpp, LM Studio, vLLM)
3. **Packaging**: Generated Rust project depending on shared backend crate (or standalone)
4. **Networking**: Explicit opt-in capability boundary with provenance tracking
5. **Progressive Delivery**: Partial results for interactive flows
6. **Tool Nodes**: Custom Rust tools with declared effects and permissions
7. **Dual Execution Modes**:
   - Execution Engine: Direct YAML execution with rich logging
   - Code Generation: YAML → Rust code → reusable compilation
8. **Self-Improvement**: Execution logs → workflow improvements → regeneration

**Scope**:
- **Included**: CLI commands and ergonomics, backend provider abstraction, packaging strategy, networking policy boundary, progressive result delivery, custom Rust tool support, execution modes, workflow improvement pipeline, self-improving agentic workflows
- **Excluded**: Rich desktop UI (deferred to ADR-0004), full autonomous networking

**Positive Consequences**:
- CLI provides fast iteration and inspection
- Provider abstraction enables backend swapping
- Networking opt-in ensures privacy by default

**Negative Consequences**:
- CLI may limit discoverability vs GUI
- Packaging decision affects deployment complexity

### Requirements Review

From `requirements.mdc` and `schema-consolidated-report.md`:

**R05**: CLI/TUI-first path
**R18**: Tools and custom Rust tools as first-class nodes
**R23**: Standalone Rust project vs backend crate vs inlining backend code
**R27**: Progressive results and partial answers
**R32**: Model provider abstraction for multiple local runners

**Validation Focus for v0.1.0**:
- CLI exposes all necessary queue and scheduler operations
- Provider abstraction loads and unloads models correctly
- Networking actions are opt-in with explicit user confirmation
- Progressive results stream to CLI for long-running workflows
- Packaging strategy produces distributable artifacts

### Research Plan Review

**research-plan-03-networking-ui-backends.yml** is **COMPLETED** with 5 research domains:

1. **Local Model Runners**: Which local runner classes should provider abstraction target first?
2. **Provider Capability Normalization**: What capability schema is sufficient for routing and benchmarking?
3. **Packaging Strategy**: What packaging strategy best balances generated-project portability and shared-backend maintainability?
4. **Desktop Shell Architecture**: How should a desktop shell expose runtime state without duplicating logic?
5. **Networking Policy Boundaries**: Which networking actions must remain opt-in and explicitly observable?

**Key Findings**:
- All 4 questions completed with evidence reviews
- Report outputs: `backends-ui-research-report.md`, `provider-capability-matrix.csv`, `packaging-options.csv`, `networking-boundary-controls.csv`
- Quality gates met: Backend recommendations distinguish CPU-first and GPU-first environments, UI recommendations preserve shared runtime abstractions, networking guidance includes explicit default-deny behaviors

---

## Web Research

### Research Area 1: Local Model Runner APIs

**Research Question**: What are the standard APIs and protocols for llama.cpp, LM Studio, vLLM, and other local model runners?

**Recommended Sources**:
- [llama.cpp](https://github.com/ggerganov/llama.cpp) - GGUF model loading and inference
- [LM Studio API](https://lmstudio.ai/docs) - Server protocol and integration
- [vLLM Documentation](https://docs.vllm.ai) - OpenAI-compatible API
- [Ollama API](https://github.com/ollama/ollama) - CLI and HTTP interface
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference) - Industry standard

**Expected Findings**:
- HTTP/REST API endpoints common patterns
- Streaming response protocols
- Model loading/unloading APIs
- Parameter passing conventions
- Error handling patterns
- Authentication mechanisms (if any)

**Status**: Not Started

---

### Research Area 2: Provider Abstraction Patterns

**Research Question**: What abstraction patterns exist for normalizing multiple AI provider APIs in Rust?

**Recommended Sources**:
- [docs.rs - async-openai](https://docs.rs/async-openai) - OpenAI client
- [docs.rs - reqwest](https://docs.rs/reqwest) - HTTP client patterns
- [tokio.rs - Async patterns](https://tokio.rs)
- [github.com - provider patterns](https://github.com) - Search for "provider abstraction" Rust projects

**Expected Findings**:
- Trait-based abstraction patterns
- Adapter pattern implementations
- Async runtime integration
- Error type normalization
- Configuration management
- Capability enumeration

**Status**: Not Started

---

### Research Area 3: Rust CLI Best Practices

**Research Question**: What are the current best practices for building CLIs in Rust?

**Recommended Sources**:
- [clap.rs](https://docs.rs/clap) - Argument parsing library
- [termion](https://docs.rs/termion) - Terminal manipulation
- [crossterm](https://docs.rs/crossterm) - Cross-platform terminal
- [indicatif](https://docs.rs/indicatif) - Progress bars and spinners
- [dialoguer](https://docs.rs/dialoguer) - Interactive prompts

**Expected Findings**:
- Command structure and subcommands
- Argument parsing and validation
- Output formatting (table, JSON, TUI)
- Progress indication for long operations
- Interactive prompts and confirmations
- Tab completion support
- Error messaging best practices

**Status**: Not Started

---

### Research Area 4: Rust Code Generation Strategies

**Research Question**: What patterns and tools exist for generating Rust code from DSLs?

**Recommended Sources**:
- [Askama](https://docs.rs/askama) - Type-safe templates
- [Tera](https://docs.rs/tera) - Template engine
- [Handlebars.rs](https://docs.rs/handlebars) - Handlebars templates
- [quote](https://docs.rs/quote) - Quasi-quoting for macros
- [proc-macro patterns](https://doc.rust-lang.org/reference/procedural-macros.html) - Procedural macros

**Expected Findings**:
- Template engine selection and usage
- Type-safe code generation
- Code organization and modularity
- Error handling in generated code
- Testing generated code
- Compilation and build integration

**Status**: Not Started

---

### Research Area 5: Networking Security and Audit Logging

**Research Question**: What patterns exist for secure, auditable networking with explicit opt-in controls?

**Recommended Sources**:
- [OWASP Logging Guide](https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html) - Security logging
- [Rust security patterns](https://doc.rust-lang.org/nomicon/safe-unsafe.html) - Memory safety
- [auditable-rs](https://github.com/sharksforarms/auditable-rs) - Audit logging
- [reqwest middleware](https://docs.rs/reqwest) - HTTP middleware for logging

**Expected Findings**:
- Audit log formats and standards
- Network request/response logging
- Consent tracking and provenance
- Rate limiting and quota management
- Secure credential handling
- Privacy-preserving logging techniques

**Status**: Not Started

---

## Implementation Plan

### Phase 1: CLI Foundation

**Description**: Implement basic CLI structure with command parsing and output formatting

**Tasks**:
1. Set up `clap` with subcommands (run, generate, validate, status, logs)
2. Implement global options (verbose, quiet, config, workspace)
3. Add output formatting (text, JSON, table)
4. Implement error handling with helpful messages
5. Add tab completion shell scripts
6. Implement config file loading (TOML)
7. Add workspace discovery logic

**Related Requirements**: R05
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 2: Backend Provider Abstraction

**Description**: Define and implement provider trait and initial implementations

**Tasks**:
1. Define `ModelProvider` trait with core methods
2. Define `ProviderCapabilities` struct
3. Implement `llama.cpp` provider adapter
4. Implement `LM Studio` provider adapter
5. Implement `vLLM` provider adapter
6. Implement `Ollama` provider adapter
7. Implement provider registry and selection logic
8. Add provider configuration (models, backends, endpoints)
9. Implement model loading/unloading
10. Add error handling and retry logic

**Related Requirements**: R32
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 3: Packaging Framework

**Description**: Implement packaging options for generated Rust projects

**Tasks**:
1. Define packaging modes (standalone, shared-backend)
2. Implement standalone binary generation
3. Implement shared-backend crate structure
4. Create code generation templates for both modes
5. Add build system integration (Cargo.toml generation)
6. Implement dependency resolution
7. Add packaging validation
8. Document packaging tradeoffs and selection criteria

**Related Requirements**: R23
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 4: Networking Boundary

**Description**: Implement opt-in networking with audit logging

**Tasks**:
1. Define `NetworkingPolicy` struct (allowlist/denylist modes)
2. Implement opt-in confirmation prompts
3. Add network request audit logging
4. Implement provenance tracking
5. Add rate limiting and quota enforcement
6. Implement credential management (secure storage)
7. Add network error handling
8. Implement telemetry opt-out
9. Add privacy-by-default configuration

**Related Requirements**: Networking (ADR-0003 scope)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 5: Progressive Result Delivery

**Description**: Implement streaming partial results for interactive workflows

**Tasks**:
1. Define streaming output channel
2. Implement partial result serialization
3. Add CLI rendering for streaming results
4. Implement buffering strategies
5. Add progress indication for long-running workflows
6. Implement result aggregation
7. Add streaming to TUI output
8. Test streaming with various workflow types

**Related Requirements**: R27
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Custom Rust Tool Support

**Description**: Implement tool node framework with permissions and effects

**Tasks**:
1. Define `Tool` trait and `ToolCapabilities`
2. Implement tool permission system (read/write/exec)
3. Define tool effects (side effects, resources)
4. Implement tool discovery and loading
5. Add tool validation and sandboxing
6. Implement built-in tools (file, web, shell, grep)
7. Add custom tool registration API
8. Document tool development guide

**Related Requirements**: R18
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 7: Self-Improvement Loop Infrastructure

**Description**: Implement workflow improvement pipeline from execution logs

**Tasks**:
1. Define execution log schema
2. Implement log aggregation and analysis
3. Define improvement suggestion types
4. Implement workflow diff generation
5. Add improvement ranking and selection
6. Implement automated workflow updates
7. Add improvement validation
8. Document self-improvement best practices
9. Add manual review workflow

**Related Requirements**: ADR-0003 self-improvement scope
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- CLI argument parsing and validation
- Provider trait method implementations
- Packaging mode logic
- Networking policy enforcement
- Streaming output buffering
- Tool permission checks
- Log analysis functions

**Test Framework**: `cargo test --lib`

**Success Criteria**:
- 90%+ code coverage on core modules
- All edge cases tested (invalid inputs, network failures, etc.)
- Mock all external dependencies

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- CLI commands with real provider implementations
- Provider loading and model inference
- Packaging and build integration
- Networking requests with policy enforcement
- Progressive result streaming through full pipeline
- Tool execution with permissions
- Self-improvement pipeline end-to-end

**Test Framework**: `cargo test --test '*'`

**Success Criteria**:
- All CLI commands execute without errors
- Provider abstraction works with multiple backends
- Generated code compiles and runs
- Networking is blocked without opt-in
- Streaming works for long-running workflows
- Tool permissions are enforced correctly
- Workflow improvements are generated and applied

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- CLI argument parsing with random inputs
- Provider capability normalization
- Packaging configuration validation
- Networking policy enforcement invariants
- Streaming output ordering guarantees
- Tool permission transitivity
- Log analysis correctness

**Test Framework**: `proptest` with 1000 iterations each

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (e.g., opt-in enforced, permissions bounded)
- Output correctness verified with properties

---

### Layer 4: End-to-End Tests

**Scope**: Full workflow execution

**Coverage Areas**:
- CLI: `run` command executes workflow end-to-end
- CLI: `generate` command produces compilable Rust code
- CLI: `validate` command catches all schema errors
- Provider: Switch between llama.cpp, LM Studio, vLLM seamlessly
- Networking: Request blocked without opt-in, logged with opt-in
- Streaming: Long workflow shows partial results in real-time
- Tools: Custom tool executes with correct permissions
- Self-improvement: Workflow improves based on execution logs

**Test Framework**: `cargo test --test '*e2e*'`

**Success Criteria**:
- Complete workflows execute successfully
- All CLI commands work as documented
- Generated code passes all tests
- Networking violations are caught and logged
- Users see partial results during execution
- Tool permissions prevent unauthorized actions
- Workflow improvements are meaningful

---

## Verification Checkpoints

### Checkpoint 1: CLI Foundation Complete

**Target Date**: Week 2
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] All CLI commands parse arguments correctly
- [ ] Help text is complete and helpful
- [ ] Tab completion works
- [ ] Error messages are clear and actionable
- [ ] Config file loading works
- [ ] Workspace discovery finds `.glyphnova/`

**Status**: Not Started

---

### Checkpoint 2: Provider Abstraction Working

**Target Date**: Week 4
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] `ModelProvider` trait is well-defined
- [ ] At least 2 providers implemented (llama.cpp + LM Studio)
- [ ] Models load and unload correctly
- [ ] Inference works through abstraction
- [ ] Error handling is robust
- [ ] Property tests pass on normalization logic

**Status**: Not Started

---

### Checkpoint 3: Packaging Framework Complete

**Target Date**: Week 5
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Both packaging modes (standalone, shared-backend) work
- [ ] Generated code compiles successfully
- [ ] Generated executables run workflows correctly
- [ ] Build integration works (Cargo.toml generation)
- [ ] E2E test: full generate → build → run cycle

**Status**: Not Started

---

### Checkpoint 4: Networking Boundary Enforced

**Target Date**: Week 6
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Networking is blocked by default
- [ ] Opt-in prompts work
- [ ] Network requests are logged with provenance
- [ ] Rate limiting is enforced
- [ ] Credentials are stored securely
- [ ] E2E test: networking blocked without opt-in

**Status**: Not Started

---

### Checkpoint 5: Progressive Results Streaming

**Target Date**: Week 6
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Streaming output channel works
- [ ] CLI renders partial results
- [ ] Long workflows show progress
- [ ] Result aggregation is correct
- [ ] E2E test: 5-minute workflow shows updates

**Status**: Not Started

---

### Checkpoint 6: Custom Tool Support Working

**Target Date**: Week 7
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] `Tool` trait is defined and used
- [ ] Permission system prevents unauthorized access
- [ ] Built-in tools (file, web, shell) work
- [ ] Custom tool can be registered
- [ ] Tool validation catches invalid tools
- [ ] Property tests pass on permission logic

**Status**: Not Started

---

### Checkpoint 7: Self-Improvement Loop Functional

**Target Date**: Week 8
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Execution logs are collected and parsed
- [ ] Improvements are generated from logs
- [ ] Improvements are ranked by quality
- [ ] Workflow updates are validated
- [ ] Manual review workflow works
- [ ] Property tests pass on log analysis

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 7 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 7 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | CLI Foundation | Not Started | 0% |
| 2 | Backend Provider Abstraction | Not Started | 0% |
| 3 | Packaging Framework | Not Started | 0% |
| 4 | Networking Boundary | Not Started | 0% |
| 5 | Progressive Result Delivery | Not Started | 0% |
| 6 | Custom Rust Tool Support | Not Started | 0% |
| 7 | Self-Improvement Loop | Not Started | 0% |

---

### Verification Layer Progress

| Layer | Description | Tests Written | Tests Passing | Coverage |
|-------|-------------|----------------|---------------|----------|
| 1 | Unit Tests | 0 | 0 | 0% |
| 2 | Integration Tests | 0 | 0 | 0% |
| 3 | Property-Based Tests | 0 | 0 | 0% |
| 4 | End-to-End Tests | 0 | 0 | 0% |

---

### Research Progress

| Research Area | Status | Evidence Collected | Synthesized |
|--------------|--------|-------------------|-------------|
| Local Model Runner APIs | Not Started | 0 | No |
| Provider Abstraction Patterns | Not Started | 0 | No |
| Rust CLI Best Practices | Not Started | 0 | No |
| Rust Code Generation Strategies | Not Started | 0 | No |
| Networking Security and Audit Logging | Not Started | 0 | No |

---

## Dependencies

### Blocks

None (foundation and queue plans precede this)

### Unblocks

- plan-03: Glyphnova UI (depends on CLI and provider abstraction)
- plan-04: Quality Loops (depends on execution infrastructure)
- plan-05: Memory & Search (depends on tool system)
- plan-06: Automation (depends on CLI)
- plan-07: Autonomy & Metrics (depends on all infrastructure)

### Integration Points

- **plan-00 (Foundation)**: Uses WorkflowSpec and WorkflowIR
- **plan-01 (MVP Queue)**: CLI commands control queue operations
- **plan-04 (Quality Loops)**: Execution logs feed improvement pipeline

---

## Quality Gates

### ADR-0003 Quality Gates

1. **CLI Exposes All Queue Operations**: CLI can create, cancel, pause, resume, and inspect workflows
2. **Provider Abstraction Works**: Multiple local runners work through unified interface
3. **Networking Opt-in Enforced**: No network request succeeds without user confirmation
4. **Progressive Results Stream**: Long workflows show partial results to CLI
5. **Packaging Produces Artifacts**: Both standalone and shared-backend modes build successfully

### Critical Review Upstream Factors

1. **Provider Abstraction Stability**: Interface supports new providers without breaking existing code
2. **Networking Policy Correctness**: Opt-in is enforced even in error cases and race conditions
3. **CLI Usability**: Help text and error messages enable independent operation without documentation
4. **Code Generation Quality**: Generated Rust code follows project conventions and passes clippy
5. **Self-Improvement Safety**: Improvements are validated and can be reviewed before application
6. **Streaming Performance**: Partial results don't degrade overall execution performance
7. **Tool Permission Correctness**: Permission checks can't be bypassed through tool composition

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0003 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (CLI Foundation)
5. **Implement Phase 1**: Build CLI foundation with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1 and 2 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-7
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Proceed**: Mark plan complete and move to plan-03

### Starting Point

Begin with Phase 1 (CLI Foundation):
1. Set up clap with basic subcommands
2. Implement argument parsing
3. Write unit tests for parsing logic
4. Write integration tests for CLI execution
5. Implement command handlers
6. Verify Checkpoint 1

---

## Execution Commands

### Verify All Tests

```bash
# Run all test layers
cargo test --lib              # Layer 1: Unit tests
cargo test --test '*'           # Layer 2: Integration tests
cargo test --test '*proptest*'  # Layer 3: Property-based tests
cargo test --test '*e2e*'       # Layer 4: End-to-end tests
```

### Verify Single Phase

```bash
# Verify Phase 1 (CLI Foundation)
cargo test --lib cli
cargo test --test cli_integration

# Verify Phase 2 (Provider Abstraction)
cargo test --lib provider
cargo test --test provider_integration
cargo test --test provider_proptest
```

### Run Specific Workflow

```bash
# Execute workflow directly
yaml-to-rust-agentsdk run workflow.yml --verbose

# Generate Rust code
yaml-to-rust-agentsdk generate workflow.yml --output ./generated
cd ./generated && cargo build --release

# Validate workflow
yaml-to-rust-agentsdk validate workflow.yml
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-02

# Resume from checkpoint
./implement.sh resume plan-02 cp3

# Check progress
./implement.sh status plan-02

# Generate progress report
./implement.sh report plan-02
```

---

## References

### Related Documents

- **ADR-0003**: [CLI, backends, and networking boundary](../roadmap/adr-0003-cli-backends-networking-boundary.yml)
- **Research Plan 03**: [Networking boundary, backend providers, and Glyphnova UI research](../roadmap/research-plan-03-networking-ui-backends.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **clap**: [docs.rs/clap](https://docs.rs/clap) - CLI argument parsing
- **llama.cpp**: [github.com/ggerganov/llama.cpp](https://github.com/ggerganov/llama.cpp) - GGUF inference
- **LM Studio**: [lmstudio.ai](https://lmstudio.ai) - Local model server
- **vLLM**: [docs.vllm.ai](https://docs.vllm.ai) - OpenAI-compatible API
- **Ollama**: [github.com/ollama/ollama](https://github.com/ollama/ollama) - Model management
- **Askama**: [docs.rs/askama](https://docs.rs/askama) - Code generation templates
- **reqwest**: [docs.rs/reqwest](https://docs.rs/reqwest) - HTTP client

### Tool and Plugin References

- **OpenCode Tools**: File operations, web scraping, shell execution, grep search
- **bash tool**: Command execution with proper error handling
- **lsp_diagnostics**: Type checking and lint verification
- **glob tool**: File pattern matching for workspace discovery

---

**Last Updated**: 2026-03-27
**Status**: Not Started
