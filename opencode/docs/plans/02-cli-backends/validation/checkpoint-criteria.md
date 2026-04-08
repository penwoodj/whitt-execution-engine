# Checkpoint Criteria

This document defines the criteria for validating completion of each task in Phase 2. These checkpoints serve as quality gates to ensure each component is properly implemented and tested.

---

## Task 00: CLI Foundation

### Checkpoint Criteria
- [ ] CLI parses all subcommands (run, generate, queue, status, config)
- [ ] Global options (--config, --output, --verbose) work correctly
- [ ] Configuration file loads from `~/.glyphnova/config.yaml`
- [ ] Configuration validates against schema
- [ ] Output formatters (plain, JSON, table) produce correct output
- [ ] All command handlers exist and execute without panics
- [ ] Tests cover all CLI parsing scenarios

### Verification Commands
```bash
# Test CLI help
cargo run -- --help

# Test specific subcommand help
cargo run -- run --help

# Test configuration loading
cargo run -- config show

# Test output formats
cargo run -- status --output json
cargo run -- status --output table
cargo run -- status --output plain
```

---

## Task 01: LLM Backend Trait

### Checkpoint Criteria
- [ ] `LlmBackend` trait defines all required methods
- [ ] All types (ChatRequest, ChatResponse, etc.) serialize/deserialize correctly
- [ ] MockBackend implements all trait methods
- [ ] Streaming parsers handle both SSE and NDJSON
- [ ] Error types cover all expected failure modes
- [ ] Tests cover trait methods and streaming

### Verification Commands
```bash
# Test trait compilation
cargo test test_trait --lib

# Test streaming
cargo test test_streaming --lib

# Test types
cargo test test_types --lib
```

---

## Task 02: LM Studio Backend

### Checkpoint Criteria
- [ ] Backend implements `LlmBackend` trait
- [ ] Non-streaming chat completes successfully
- [ ] Streaming chat parses SSE correctly
- [ ] Model list returns expected data
- [ ] Health check reports correct status
- [ ] Wiremock tests cover all methods

### Verification Commands
```bash
# Test backend with wiremock
cargo test test_lmstudio --lib

# Verify SSE streaming
cargo test test_lmstudio_chat_stream --lib
```

---

## Task 03: Ollama Backend

### Checkpoint Criteria
- [ ] Backend implements `LlmBackend` trait
- [ ] Request/response format conversion works
- [ ] NDJSON streaming parses correctly
- [ ] 'think' and 'format' parameters are handled
- [ ] Model load/unload methods work
- [ ] Wiremock tests cover all methods

### Verification Commands
```bash
# Test backend with wiremock
cargo test test_ollama --lib

# Verify NDJSON streaming
cargo test test_ollama_chat_stream --lib
```

---

## Task 04: llama.cpp Backend

### Checkpoint Criteria
- [ ] Backend implements `LlmBackend` trait
- [ ] SSE streaming parses correctly
- [ ] /slots endpoint works
- [ ] Health check uses /health endpoint
- [ ] Wiremock tests cover all methods

### Verification Commands
```bash
# Test backend with wiremock
cargo test test_llamacpp --lib

# Verify slots endpoint
cargo test test_llamacpp_list_models --lib
```

---

## Task 05: OpenAI Backend

### Checkpoint Criteria
- [ ] Backend implements `LlmBackend` trait
- [ ] API key authentication works
- [ ] Rate limiting (429) is handled correctly
- [ ] SSE streaming parses correctly
- [ ] Parallel requests capability is set
- [ ] Wiremock tests cover all methods including rate limiting

### Verification Commands
```bash
# Test backend with wiremock
cargo test test_openai --lib

# Verify rate limiting
cargo test test_openai_rate_limit --lib
```

---

## Task 06: Backend Registry

### Checkpoint Criteria
- [ ] All backends register successfully
- [ ] Default backend selection works
- [ ] Fallback mechanism activates on unhealthy backend
- [ ] Health check reports status for all backends
- [ ] Configuration-based registry creation works
- [ ] Tests cover registry operations

### Verification Commands
```bash
# Test registry operations
cargo test test_registry --lib

# Test health monitoring
cargo test test_registry_health_check --lib
```

---

## Task 07: Tool Permissions

### Checkpoint Criteria
- [ ] Allow/deny lists enforce correctly
- [ ] Default policy (allow/deny) works
- [ ] Step restrictions override global settings
- [ ] Confirmation flow works in interactive mode
- [ ] Guardrails (path traversal, command injection) prevent attacks
- [ ] Tests cover all permission scenarios

### Verification Commands
```bash
# Test permission evaluation
cargo test test_permission_evaluation --lib

# Test guardrails
cargo test test_path_traversal_prevention --lib
cargo test test_command_injection_prevention --lib
```

---

## Task 08: Tool Execution Framework

### Checkpoint Criteria
- [ ] Tool trait defines all required methods
- [ ] All built-in tools (file, web, shell) work
- [ ] Tool registry discovers all tools
- [ ] Execution engine enforces permissions
- [ ] Confirmation flow works for required tools
- [ ] Tests cover all built-in tools

### Verification Commands
```bash
# Test tool execution
cargo test test_tool_execution --lib

# Test permission enforcement
cargo test test_permission_denied --lib
```

---

## Task 09: Sub-Workflow Execution

### Checkpoint Criteria
- [ ] Workflow graph detects cycles correctly
- [ ] Topological sort produces correct execution order
- [ ] Isolation (sandboxing, readonly) works
- [ ] Workflow composition resolves sub-workflows
- [ ] Validation detects circular references
- [ ] Tests cover composition and nesting

### Verification Commands
```bash
# Test cycle detection
cargo test test_workflow_graph_cycle_detection --lib
cargo test test_workflow_graph_with_cycle --lib

# Test composition
cargo test test_workflow_composition --lib
```

---

## Task 10: Code Generation

### Checkpoint Criteria
- [ ] Code generator produces valid Rust code
- [ ] Askama templates render correctly
- [ ] Generated project structure is complete
- [ ] Cargo.toml includes correct dependencies
- [ ] Optional compilation step works
- [ ] Tests cover generation logic

### Verification Commands
```bash
# Test code generation
cargo test test_code_generation --lib

# Test file generation
cargo test test_code_generation_to_file --lib
```

---

## Task 11: RAG Integration

### Checkpoint Criteria
- [ ] Document indexer handles files and directories
- [ ] Embedding generation produces vectors
- [ ] Similarity computation works correctly
- [ ] Retrieval returns top-k relevant documents
- [ ] Context injection adds context to messages
- [ ] Tests cover RAG pipeline

### Verification Commands
```bash
# Test document indexing
cargo test test_index_document --lib

# Test retrieval
cargo test test_retrieval --lib

# Test context injection
cargo test test_context_injection --lib
```

---

## Task 12: Self-Improvement Loop

### Checkpoint Criteria
- [ ] Execution logger records all steps
- [ ] Analyzer detects performance and reliability issues
- [ ] Improvement suggestions are generated
- [ ] Workflow diff detects all changes
- [ ] Assessment scoring works correctly
- [ ] Tests cover loop components

### Verification Commands
```bash
# Test execution logging
cargo test test_execution_logging --lib

# Test analysis
cargo test test_execution_analysis --lib

# Test diff generation
cargo test test_workflow_diff --lib
```

---

## Global Checkpoint Criteria

### Build Verification
- [ ] `cargo build` completes without errors
- [ ] `cargo test --lib` passes all tests
- [ ] `cargo clippy` reports no warnings
- [ ] `cargo fmt` reports no formatting issues

### Documentation
- [ ] All public APIs have rustdoc comments
- [ ] `cargo doc` builds successfully
- [ ] Module documentation is complete

### Integration
- [ ] CLI can execute workflows (end-to-end)
- [ ] All backends work with registry
- [ ] Tool execution integrates with permissions
- [ ] RAG works with backends

### Security
- [ ] All tool executions go through permission checks
- [ ] Guardrails prevent path traversal and command injection
- [ ] Configuration validation prevents unsafe settings

---

## Pre-Commit Checklist

Before committing any task completion:

1. [ ] All tests pass: `cargo test --lib`
2. [ ] No clippy warnings: `cargo clippy`
3. [ ] Code formatted: `cargo fmt`
4. [ ] Documentation builds: `cargo doc`
5. [ ] Task-specific verification commands run successfully
6. [ ] Checkpoint criteria all met
7. [ ] Commit message follows conventional commit format
