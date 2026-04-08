# Acceptance Criteria

This document defines the overall acceptance criteria for Phase 2: CLI & LLM Backends. All criteria must be met for the phase to be considered complete.

---

## Functional Requirements

### CLI Interface
- [ ] **FR-CLI-001:** User can run workflows via CLI: `glyphnova run workflow.yaml`
- [ ] **FR-CLI-002:** User can generate code from workflows: `glyphnova generate workflow.yaml`
- [ ] **FR-CLI-003:** User can manage workflow queue: `glyphnova queue add/list/remove/clear`
- [ ] **FR-CLI-004:** User can query status: `glyphnova status [--queue-id ID]`
- [ ] **FR-CLI-005:** User can manage configuration: `glyphnova config show/set/validate`
- [ ] **FR-CLI-006:** CLI supports multiple output formats: `--output {plain,json,table}`
- [ ] **FR-CLI-007:** CLI provides tab completion for all commands
- [ ] **FR-CLI-008:** Configuration loads from `~/.glyphnova/config.yaml`

### LLM Backends
- [ ] **FR-LLM-001:** System supports LM Studio backend (localhost:1234)
- [ ] **FR-LLM-002:** System supports Ollama backend (localhost:11434)
- [ ] **FR-LLM-003:** System supports llama.cpp backend (localhost:8080)
- [ ] **FR-LLM-004:** System supports OpenAI backend (api.openai.com)
- [ ] **FR-LLM-005:** All backends support streaming responses
- [ ] **FR-LLM-006:** All backends support tool calling
- [ ] **FR-LLM-007:** Backend registry manages provider selection and fallback
- [ ] **FR-LLM-008:** Health monitoring detects unavailable backends

### Tool Execution
- [ ] **FR-TOOL-001:** System provides built-in tools: file.read/write/delete, web.fetch/scrape, shell.exec
- [ ] **FR-TOOL-002:** Users can register custom tools
- [ ] **FR-TOOL-003:** Tool execution enforces permissions
- [ ] **FR-TOOL-004:** Guardrails prevent path traversal and command injection
- [ ] **FR-TOOL-005:** Confirmation flow for sensitive operations
- [ ] **FR-TOOL-006:** Tool registry discovers and manages all tools

### Workflow Orchestration
- [ ] **FR-WF-001:** System supports sub-workflow execution
- [ ] **FR-WF-002:** Workflow composition allows nested workflows
- [ ] **FR-WF-003:** Circular reference detection prevents infinite loops
- [ ] **FR-WF-004:** Isolation mechanisms (sandboxing, readonly) work
- [ ] **FR-WF-005:** Topological sort produces correct execution order

### Code Generation
- [ ] **FR-CODE-001:** System generates Rust code from WorkflowIR
- [ ] **FR-CODE-002:** Generated code compiles successfully
- [ ] **FR-CODE-003:** Generated project includes Cargo.toml
- [ ] **FR-CODE-004:** Templates are customizable via Askama
- [ ] **FR-CODE-005:** Optional compilation step works

### RAG Integration
- [ ] **FR-RAG-001:** System indexes documents from files/directories
- [ ] **FR-RAG-002:** Embedding generation produces vector representations
- [ ] **FR-RAG-003:** Semantic retrieval finds relevant documents
- [ ] **FR-RAG-004:** Context injection enhances LLM prompts
- [ ] **FR-RAG-005:** Retrieval supports top-k and filtering

### Self-Improvement
- [ ] **FR-SI-001:** System logs execution details
- [ ] **FR-SI-002:** Analyzer identifies performance and reliability issues
- [ ] **FR-SI-003:** Improvement suggestions are generated with priorities
- [ ] **FR-SI-004:** Workflow diff tracks changes
- [ ] **FR-SI-005:** Assessment scoring provides quality metrics

---

## Non-Functional Requirements

### Performance
- [ ] **NFR-PERF-001:** CLI commands respond within 100ms (excluding network calls)
- [ ] **NFR-PERF-002:** Streaming chat delivers first chunk within 500ms
- [ ] **NFR-PERF-003:** Tool execution completes within timeout limits
- [ ] **NFR-PERF-004:** Backend health checks complete within 5s

### Reliability
- [ ] **NFR-REL-001:** System handles network failures gracefully
- [ ] **NFR-REL-002:** System handles malformed inputs without crashing
- [ ] **NFR-REL-003:** System recovers from backend unavailability
- [ ] **NFR-REL-004:** Error messages are clear and actionable

### Security
- [ ] **NFR-SEC-001:** Default-deny policy for all tools
- [ ] **NFR-SEC-002:** Path traversal attempts are blocked
- [ ] **NFR-SEC-003:** Command injection attempts are blocked
- [ ] **NFR-SEC-004:** API keys are not logged or exposed
- [ ] **NFR-SEC-005:** Sandbox isolation prevents unauthorized file access

### Maintainability
- [ ] **NFR-MAINT-001:** Code follows Rust best practices
- [ ] **NFR-MAINT-002:** All public APIs have documentation
- [ ] **NFR-MAINT-003:** Tests cover 80% of code (measured by line coverage)
- [ ] **NFR-MAINT-004:** No clippy warnings
- [ ] **NFR-MAINT-005:** Code is formatted with rustfmt

### Usability
- [ ] **NFR-USE-001:** CLI help is comprehensive and clear
- [ ] **NFR-USE-002:** Error messages guide users to solutions
- [ ] **NFR-USE-003:** Tab completion works for all commands
- [ ] **NFR-USE-004:** Configuration validation provides clear feedback
- [ ] **NFR-USE-005:** Multiple output formats cater to different use cases

---

## Integration Requirements

### Phase Dependencies
- [ ] **INT-001:** CLI integrates with Phase 0 parser and WorkflowIR
- [ ] **INT-002:** Executor integrates with Phase 1 queue, scheduler, executor
- [ ] **INT-003:** Backends work with Phase 0 validation
- [ ] **INT-004:** Tools work with Phase 1 step executor

### External Dependencies
- [ ] **EXT-001:** System works without network access (local-only mode)
- [ ] **EXT-002:** System handles rate limiting from OpenAI
- [ ] **EXT-003:** System respects environment variables for API keys
- [ ] **EXT-004:** System handles missing backends gracefully

---

## Testing Requirements

### Unit Testing
- [ ] **TEST-UNIT-001:** All trait methods have tests
- [ ] **TEST-UNIT-002:** All streaming parsers have tests
- [ ] **TEST-UNIT-003:** All permission logic has tests
- [ ] **TEST-UNIT-004:** All guardrails have tests
- [ ] **TEST-UNIT-005:** Mock strategies cover all HTTP backends

### Integration Testing
- [ ] **TEST-INT-001:** End-to-end workflow execution works
- [ ] **TEST-INT-002:** Backend registry works with all backends
- [ ] **TEST-INT-003:** Tool execution works with permissions
- [ ] **TEST-INT-004:** Sub-workflows execute correctly
- [ ] **TEST-INT-005:** Generated code compiles and runs

### Property-Based Testing
- [ ] **TEST-PROP-001:** Streaming parser invariants are tested
- [ ] **TEST-PROP-002:** Permission logic properties are tested
- [ ] **TEST-PROP-003:** Backend selection behavior is tested

### Coverage
- [ ] **TEST-COV-001:** Line coverage ≥ 80%
- [ ] **TEST-COV-002:** All critical paths have tests
- [ ] **TEST-COV-003:** Error paths have tests

---

## Documentation Requirements

### User Documentation
- [ ] **DOC-USER-001:** CLI reference is complete
- [ ] **DOC-USER-002:** Configuration guide is provided
- [ ] **DOC-USER-003:** Backend setup instructions are provided
- [ ] **DOC-USER-004:** Examples cover common use cases

### Developer Documentation
- [ ] **DOC-DEV-001:** API documentation is complete (rustdoc)
- [ ] **DOC-DEV-002:** Architecture documentation is provided
- [ ] **DOC-DEV-003:** Contribution guidelines are provided
- [ ] **DOC-DEV-004:** Test strategy is documented

---

## Compliance Requirements

### ADR-0003 Compliance
- [ ] **ADR-001:** Section 6 (Tool Permissions) is fully implemented
- [ ] **ADR-002:** Section 10 (Orchestration) is fully implemented
- [ ] **ADR-003:** Section 11 (Provider Configuration) is fully implemented
- [ ] **ADR-004:** Section 12 (RAG Configuration) is fully implemented
- [ ] **ADR-005:** Section 19 (Duplicate Config Systems) is clarified

### Rust Best Practices
- [ ] **RUST-001:** Code uses idiomatic Rust patterns
- [ ] **RUST-002:** Error handling uses `anyhow` for app, `thiserror` for library
- [ ] **RUST-003:** Async code uses `tokio` runtime
- [ ] **RUST-004:** Serialization uses `serde`
- [ ] **RUST-005:** HTTP client uses `reqwest`

---

## Sign-Off Criteria

Phase 2 is considered complete when:

1. [ ] All functional requirements (FR-*) are met
2. [ ] All non-functional requirements (NFR-*) are met
3. [ ] All integration requirements (INT-*) are met
4. [ ] All testing requirements (TEST-*) are met
5. [ ] All documentation requirements (DOC-*) are met
6. [ ] All compliance requirements (ADR-*, RUST-*) are met
7. [ ] All checkpoint criteria are met for each task
8. [ ] `cargo build --release` succeeds
9. [ ] `cargo test --lib` passes with all tests green
10. [ ] `cargo clippy` reports no warnings
11. [ ] `cargo fmt` reports no formatting issues
12. [ ] Code review is approved
13. [ ] Documentation is published

---

## Exit Criteria

Before proceeding to Phase 3:

1. [ ] All acceptance criteria are met
2. [ ] Phase 2 implementation is merged to main
3. [ ] Release notes are prepared
4. [ ] Migration guide is available (if breaking changes)
5. [ ] Known issues are documented
6. [ ] Phase 3 planning is complete
