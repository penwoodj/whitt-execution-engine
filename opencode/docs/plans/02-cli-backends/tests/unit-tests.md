# Unit Tests Specification

This document specifies the unit tests required for Phase 2. Unit tests focus on individual components in isolation.

---

## Test Organization

```
tests/
├── cli/
│   ├── commands_test.rs
│   ├── config_test.rs
│   └── output_test.rs
├── backends/
│   ├── trait_test.rs
│   ├── types_test.rs
│   ├── streaming_test.rs
│   ├── lmstudio_test.rs
│   ├── ollama_test.rs
│   ├── llamacpp_test.rs
│   ├── openai_test.rs
│   └── registry_test.rs
├── tools/
│   ├── permissions_test.rs
│   └── execution_test.rs
├── workflows/
│   └── subworkflow_test.rs
├── codegen/
│   └── generator_test.rs
├── rag/
│   └── rag_test.rs
└── self_improvement/
    └── loop_test.rs
```

---

## CLI Tests

### CLI Command Parsing
```rust
#[test]
fn test_cli_parse_basic_run()
#[test]
fn test_cli_parse_with_options()
#[test]
fn test_cli_parse_global_options()
#[test]
fn test_cli_parse_queue_add()
#[test]
fn test_cli_parse_queue_remove()
#[test]
fn test_cli_parse_queue_list()
#[test]
fn test_cli_parse_queue_clear()
#[test]
fn test_cli_parse_status_with_id()
#[test]
fn test_cli_parse_status_detailed()
#[test]
fn test_cli_parse_config_show()
#[test]
fn test_cli_parse_config_set()
#[test]
fn test_cli_parse_config_reset()
#[test]
fn test_cli_parse_config_validate()
```

### Configuration Loading
```rust
#[test]
fn test_load_default_config()
#[test]
fn test_load_custom_config()
#[test]
fn test_save_and_load_config()
#[test]
fn test_validate_config_valid()
#[test]
fn test_validate_config_invalid_provider()
#[test]
fn test_validate_config_invalid_policy()
#[test]
fn test_config_defaults()
#[test]
fn test_config_env_var_expansion()
```

### Output Formatting
```rust
#[test]
fn test_plain_formatter_success()
#[test]
fn test_plain_formatter_error()
#[test]
fn test_plain_formatter_workflow_status()
#[test]
fn test_plain_formatter_queue_item()
#[test]
fn test_json_formatter_success()
#[test]
fn test_json_formatter_error()
#[test]
fn test_json_formatter_serialization()
#[test]
fn test_table_formatter_success()
#[test]
fn test_table_formatter_column_widths()
#[test]
fn test_formatter_factory()
```

---

## Backend Tests

### Types and Trait
```rust
#[test]
fn test_chat_request_serialization()
#[test]
fn test_chat_request_with_tools()
#[test]
fn test_chat_request_with_temperature()
#[test]
fn test_chat_response_deserialization()
#[test]
fn test_chat_response_with_usage()
#[test]
fn test_stream_chunk_deserialization()
#[test]
fn test_message_system()
#[test]
fn test_message_user()
#[test]
fn test_message_assistant()
#[test]
fn test_message_tool()
#[test]
fn test_backend_capabilities_default()
#[test]
fn test_backend_capabilities_custom()
```

### Mock Backend
```rust
#[tokio::test]
async fn test_mock_backend_chat()
#[tokio::test]
async fn test_mock_backend_chat_stream()
#[tokio::test]
async fn test_mock_backend_list_models()
#[tokio::test]
async fn test_mock_backend_health_check()
#[tokio::test]
async fn test_mock_backend_load_model()
#[tokio::test]
async fn test_mock_backend_unload_model()
#[test]
fn test_mock_backend_capabilities()
#[test]
fn test_mock_backend_name()
```

### Streaming Parsers
```rust
#[test]
fn test_sse_parser_single_chunk()
#[test]
fn test_sse_parser_multiple_chunks()
#[test]
fn test_sse_parser_done_marker()
#[test]
fn test_sse_parser_malformed_data()
#[test]
fn test_sse_parser_buffer_overflow()
#[test]
fn test_ndjson_parser_single_line()
#[test]
fn test_ndjson_parser_multiple_lines()
#[test]
fn test_ndjson_parser_empty_lines()
#[test]
fn test_ndjson_parser_malformed_json()
```

### LM Studio Backend
```rust
#[tokio::test]
async fn test_lmstudio_chat_success()
#[tokio::test]
async fn test_lmstudio_chat_with_tools()
#[tokio::test]
async fn test_lmstudio_chat_stream()
#[tokio::test]
async fn test_lmstudio_list_models()
#[tokio::test]
async fn test_lmstudio_health_check()
#[tokio::test]
async fn test_lmstudio_backend_error()
#[tokio::test]
async fn test_lmstudio_timeout()
```

### Ollama Backend
```rust
#[tokio::test]
async fn test_ollama_chat_success()
#[tokio::test]
async fn test_ollama_chat_with_think()
#[tokio::test]
async fn test_ollama_chat_with_format()
#[tokio::test]
async fn test_ollama_chat_stream()
#[tokio::test]
async fn test_ollama_list_models()
#[tokio::test]
async fn test_ollama_health_check()
#[tokio::test]
async fn test_ollama_load_model()
#[tokio::test]
async fn test_ollama_format_conversion()
```

### llama.cpp Backend
```rust
#[tokio::test]
async fn test_llamacpp_chat_success()
#[tokio::test]
async fn test_llamacpp_chat_stream()
#[tokio::test]
async fn test_llamacpp_list_models()
#[tokio::test]
async fn test_llamacpp_health_check()
#[tokio::test]
async fn test_llamacpp_slots_endpoint()
#[tokio::test]
async fn test_llamacpp_with_tools()
```

### OpenAI Backend
```rust
#[tokio::test]
async fn test_openai_chat_success()
#[tokio::test]
async fn test_openai_chat_stream()
#[tokio::test]
async fn test_openai_list_models()
#[tokio::test]
async fn test_openai_health_check()
#[tokio::test]
async fn test_openai_rate_limit()
#[tokio::test]
async fn test_openai_rate_limit_retry_after()
#[tokio::test]
async fn test_openai_parallel_requests_capability()
#[tokio::test]
async fn test_openai_api_key_authentication()
```

### Backend Registry
```rust
#[tokio::test]
async fn test_registry_register_and_get()
#[tokio::test]
async fn test_registry_get_nonexistent()
#[tokio::test]
async fn test_registry_default()
#[tokio::test]
async fn test_registry_set_default()
#[tokio::test]
async fn test_registry_fallback()
#[tokio::test]
async fn test_registry_fallback_unhealthy_default()
#[tokio::test]
async fn test_registry_health_check_all()
#[tokio::test]
async fn test_registry_list_backends()
#[tokio::test]
async fn test_registry_from_config()
#[tokio::test]
async fn test_registry_fallback_order()
```

---

## Tool Tests

### Permissions
```rust
#[test]
fn test_permission_evaluation_allowed()
#[test]
fn test_permission_evaluation_denied()
#[test]
fn test_permission_evaluation_confirmation_required()
#[test]
fn test_allow_list()
#[test]
fn test_deny_list()
#[test]
fn test_default_policy_allow()
#[test]
fn test_default_policy_deny()
#[test]
fn test_step_restrictions()
#[test]
fn test_step_restrictions_override_global()
#[test]
fn test_path_traversal_prevention()
#[test]
fn test_path_traversal_with_backslash()
#[test]
fn test_command_injection_prevention()
#[test]
fn test_command_injection_with_semicolon()
#[test]
fn test_denied_paths()
#[test]
fn test_allowed_paths()
#[test]
fn test_guardrails_with_valid_path()
```

### Tool Execution
```rust
#[tokio::test]
async fn test_tool_execution_allowed()
#[tokio::test]
async fn test_tool_execution_denied()
#[tokio::test]
async fn test_tool_execution_confirmation_required()
#[tokio::test]
async fn test_tool_execution_guardrails_failed()
#[tokio::test]
async fn test_file_read_tool()
#[tokio::test]
async fn test_file_write_tool()
#[tokio::test]
async fn test_file_delete_tool()
#[tokio::test]
async fn test_web_fetch_tool()
#[tokio::test]
async fn test_web_scrape_tool()
#[tokio::test]
async fn test_shell_exec_tool()
#[tokio::test]
async fn test_tool_not_found()
#[tokio::test]
async fn test_tool_execution_metadata()
```

---

## Workflow Tests

### Sub-Workflow Execution
```rust
#[test]
fn test_workflow_graph_cycle_detection()
#[test]
fn test_workflow_graph_with_cycle()
#[test]
fn test_workflow_graph_without_cycle()
#[test]
fn test_workflow_graph_multiple_cycles()
#[test]
fn test_workflow_graph_execution_order()
#[test]
fn test_workflow_graph_topological_sort()
#[test]
fn test_workflow_isolation_validate_path()
#[test]
fn test_workflow_isolation_path_outside_workspace()
#[test]
fn test_workflow_isolation_readonly()
#[test]
fn test_workflow_composition_simple()
#[test]
fn test_workflow_composition_nested()
#[test]
fn test_workflow_composition_circular_reference()
#[test]
fn test_workflow_composition_validation()
#[test]
fn test_workflow_composition_report()
```

---

## Code Generation Tests

```rust
#[test]
fn test_code_generation()
#[test]
fn test_code_generation_with_steps()
#[test]
fn test_code_generation_to_file()
#[test]
fn test_code_generation_project_structure()
#[test]
fn test_code_generation_cargo_toml()
#[test]
fn test_code_generation_main_rs()
#[test]
fn test_code_generation_compilation()
#[test]
fn test_code_generation_with_subworkflow()
```

---

## RAG Tests

```rust
#[test]
fn test_index_document()
#[test]
fn test_index_directory()
#[test]
fn test_index_nonexistent_file()
#[test]
fn test_load_document()
#[test]
fn test_list_documents()
#[test]
fn test_delete_document()
#[tokio::test]
async fn test_embedding_generation()
#[tokio::test]
async fn test_embedding_vector_size()
#[tokio::test]
async fn test_compute_similarity()
#[tokio::test]
async fn test_compute_similarity_identical()
#[tokio::test]
async fn test_compute_similarity_orthogonal()
#[tokio::test]
async fn test_retrieve()
#[tokio::test]
async fn test_retrieve_by_category()
#[tokio::test]
async fn test_retrieve_by_tags()
#[tokio::test]
async fn test_context_injection()
#[tokio::test]
async fn test_context_injection_with_documents()
#[tokio::test]
async fn test_context_injection_truncation()
```

---

## Self-Improvement Tests

```rust
#[tokio::test]
async fn test_execution_logging()
#[tokio::test]
async fn test_load_execution()
#[tokio::test]
async fn test_list_executions()
#[tokio::test]
async fn test_list_executions_by_workflow()
#[tokio::test]
async fn test_execution_analysis()
#[tokio::test]
async fn test_analysis_with_failures()
#[tokio::test]
async fn test_analysis_with_slow_steps()
#[tokio::test]
async fn test_improvement_suggestions()
#[tokio::test]
async fn test_assessment_scoring()
#[tokio::test]
async fn test_workflow_diff()
#[tokio::test]
async fn test_workflow_diff_metadata()
#[tokio::test]
async fn test_workflow_diff_steps()
```

---

## Test Coverage Goals

| Component | Target Coverage |
|----------|----------------|
| CLI | 85% |
| Backends | 80% |
| Tools | 85% |
| Workflows | 80% |
| Codegen | 75% |
| RAG | 75% |
| Self-Improvement | 75% |
| **Overall** | **80%** |

---

## Running Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run tests for specific module
cargo test cli --lib
cargo test backends --lib
cargo test tools --lib

# Run tests with output
cargo test --lib -- --nocapture

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Run tests in parallel
cargo test --lib -- --test-threads=4
```
