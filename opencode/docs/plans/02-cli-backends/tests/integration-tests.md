# Integration Tests Specification

This document specifies the integration tests required for Phase 2. Integration tests verify that components work together correctly.

---

## Test Organization

```
tests/integration/
├── cli/
│   ├── end_to_end_workflow_test.rs
│   ├── cli_backend_integration_test.rs
│   └── cli_tools_integration_test.rs
├── backends/
│   ├── backend_registry_integration_test.rs
│   ├── multi_backend_test.rs
│   └── streaming_integration_test.rs
├── tools/
│   ├── tool_execution_integration_test.rs
│   └── tool_permissions_integration_test.rs
├── workflows/
│   ├── workflow_execution_test.rs
│   └── subworkflow_integration_test.rs
├── codegen/
│   └── codegen_integration_test.rs
├── rag/
│   └── rag_integration_test.rs
└── self_improvement/
    └── loop_integration_test.rs
```

---

## CLI Integration Tests

### End-to-End Workflow Execution
```rust
#[tokio::test]
async fn test_cli_run_workflow_success()
#[tokio::test]
async fn test_cli_run_workflow_with_tools()
#[tokio::test]
async fn test_cli_run_workflow_with_rag()
#[tokio::test]
async fn test_cli_run_workflow_dry_run()
#[tokio::test]
async fn test_cli_run_workflow_from_checkpoint()
```

### CLI-Backend Integration
```rust
#[tokio::test]
async fn test_cli_uses_lmstudio_backend()
#[tokio::test]
async fn test_cli_uses_ollama_backend()
#[tokio::test]
async fn test_cli_uses_llamacpp_backend()
#[tokio::test]
async fn test_cli_uses_openai_backend()
#[tokio::test]
async fn test_cli_backend_fallback()
#[tokio::test]
async fn test_cli_backend_health_check()
```

### CLI-Tools Integration
```rust
#[tokio::test]
async fn test_cli_tool_execution_with_permissions()
#[tokio::test]
async fn test_cli_tool_execution_with_confirmation()
#[tokio::test]
async fn test_cli_tool_execution_guardrails()
```

---

## Backend Integration Tests

### Backend Registry
```rust
#[tokio::test]
async fn test_backend_registry_with_all_backends()
#[tokio::test]
async fn test_backend_registry_with_unavailable_backend()
#[tokio::test]
async fn test_backend_registry_fallback_chain()
#[tokio::test]
async fn test_backend_registry_health_monitoring()
```

### Multi-Backend Support
```rust
#[tokio::test]
async fn test_switch_backends()
#[tokio::test]
async fn test_parallel_requests_with_openai()
#[tokio::test]
async fn test_backend_selection_from_config()
```

### Streaming Integration
```rust
#[tokio::test]
async fn test_sse_streaming_integration()
#[tokio::test]
async fn test_ndjson_streaming_integration()
#[tokio::test]
async fn test_streaming_with_tools()
#[tokio::test]
async fn test_streaming_error_handling()
```

---

## Tool Integration Tests

### Tool Execution
```rust
#[tokio::test]
async fn test_tool_execution_with_llm_backend()
#[tokio::test]
async fn test_tool_execution_chain()
#[tokio::test]
async fn test_tool_execution_with_context()
```

### Tool Permissions
```rust
#[tokio::test]
async fn test_tool_permissions_with_real_tools()
#[tokio::test]
async fn test_step_permissions_override()
#[tokio::test]
async fn test_tool_permissions_guardrails()
```

---

## Workflow Integration Tests

### Workflow Execution
```rust
#[tokio::test]
async fn test_workflow_execution_with_backend()
#[tokio::test]
async fn test_workflow_execution_with_tools()
#[tokio::test]
async fn test_workflow_execution_with_rag()
#[tokio::test]
async fn test_workflow_execution_with_permissions()
```

### Sub-Workflow
```rust
#[tokio::test]
async fn test_subworkflow_execution()
#[tokio::test]
async fn test_nested_subworkflows()
#[tokio::test]
async fn test_subworkflow_with_isolation()
#[tokio::test]
async fn test_subworkflow_with_permissions()
```

---

## Code Generation Integration Tests

```rust
#[tokio::test]
async fn test_codegen_from_real_workflow()
#[tokio::test]
async fn test_codegen_compilation()
#[tokio::test]
async fn test_codegen_execution()
#[tokio::test]
async fn test_codegen_with_subworkflows()
```

---

## RAG Integration Tests

```rust
#[tokio::test]
async fn test_rag_with_backend()
#[tokio::test]
async fn test_rag_with_workflow()
#[tokio::test]
async fn test_rag_with_tools()
#[tokio::test]
async fn test_rag_context_injection()
```

---

## Self-Improvement Integration Tests

```rust
#[tokio::test]
async fn test_self_improvement_with_workflow()
#[tokio::test]
async fn test_self_improvement_analysis()
#[tokio::test]
async fn test_self_improvement_suggestions()
```

---

## Running Integration Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test cli_integration

# Run with actual backends (requires running backends)
RAG_ENABLED=true NETWORK_ENABLED=true cargo test --test integration

# Run integration tests only (skip unit tests)
cargo test --test '* integration*' --lib
```

---

## Test Environment

### Prerequisites
- LM Studio running on `localhost:1234` (optional, uses wiremock by default)
- Ollama running on `localhost:11434` (optional, uses wiremock by default)
- llama.cpp running on `localhost:8080` (optional, uses wiremock by default)
- OpenAI API key set (optional, uses wiremock by default)

### Environment Variables
```bash
# Network access (default: false)
NETWORK_ENABLED=true

# RAG enabled (default: false)
RAG_ENABLED=true

# OpenAI API key
OPENAI_API_KEY=sk-...

# Test backend endpoint (for wiremock)
TEST_BACKEND_URL=http://localhost:8080
```

### Wiremock Integration
All backend tests use wiremock by default for fast, reliable testing:
- LM Studio: Mock on random port
- Ollama: Mock on random port
- llama.cpp: Mock on random port
- OpenAI: Mock on random port

To test with real backends:
```bash
# Skip wiremock, use real backends
TEST_REAL_BACKENDS=true cargo test --test integration
```
