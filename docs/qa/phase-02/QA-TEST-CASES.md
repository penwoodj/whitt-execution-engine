# Phase 02: Test Cases

### P02-001: CLI Subcommands and Options
- **Area**: QA-02-01
- **Type**: Unit
- **Command**: `cargo test cli_commands --lib`
- **Setup**: N/A (unit test)
- **Expected**: All subcommands (run, generate, queue, status, config) parse correctly. Global options (output-format, verbose, config-path) recognized.
- **Pass Criteria**: ✅ Clap parsing succeeds, all subcommands available

### P02-002: CLI Configuration Loading
- **Area**: QA-02-01
- **Type**: Integration
- **Command**: `cargo test cli_config_loading --test`
- **Setup**: Valid config file at `~/./workspace/config.yaml`
- **Expected**: Configuration loaded correctly, provider settings, tool permissions, RAG settings all parsed
- **Pass Criteria**: ✅ Config deserializes without errors, all sections populated

### P02-003: CLI Output Formatting
- **Area**: QA-02-01
- **Type**: Unit
- **Command**: `cargo test cli_output_format --lib`
- **Setup**: Mock CLI commands returning structured data
- **Expected**: Output formatted as plain, JSON, or table based on flag
- **Pass Criteria**: ✅ Each format produces correct output type and structure

### P02-004: LLM Backend Trait Object-Safety
- **Area**: QA-02-02
- **Type**: Compile-time
- **Command**: `cargo check --lib 2>&1 | grep -i "object.*safe\|dyn.*LlmBackend"`
- **Setup**: N/A (compile check)
- **Expected**: No errors. Trait compiles and can be used as `dyn LlmBackend`
- **Pass Criteria**: ✅ `cargo check` passes

### P02-005: LLM Backend Error Variants
- **Area**: QA-02-02
- **Type**: Unit
- **Command**: `cargo test llm_errors --lib`
- **Setup**: Mock error conditions
- **Expected**: All LlmError variants constructible: Network, Parse, Backend, RateLimited, Timeout
- **Pass Criteria**: ✅ Each variant displays correct error message

### P02-006: LM Studio Chat Completion
- **Area**: QA-02-03
- **Type**: Integration (wiremock)
- **Command**: `cargo test lmstudio_chat --test`
- **Setup**: Wiremock server at localhost:1234 mocking /v1/chat/completions
- **Expected**: ChatCompletionRequest sent correctly, SSE response parsed
- **Pass Criteria**: ✅ Response has choices[0].message.content non-empty

### P02-007: LM Studio Streaming
- **Area**: QA-02-03
- **Type**: Integration (wiremock)
- **Command**: `cargo test lmstudio_streaming --test`
- **Setup**: Wiremock server returning SSE stream
- **Expected**: StreamingResponse collects chunks correctly, done chunk terminates stream
- **Pass Criteria**: ✅ Full text assembled from SSE chunks

### P02-008: Ollama NDJSON Streaming
- **Area**: QA-02-04
- **Type**: Integration (wiremock)
- **Command**: `cargo test ollama_streaming --test`
- **Setup**: Wiremock server at localhost:11434 mocking /api/chat
- **Expected**: NDJSON stream parsed correctly
- **Pass Criteria**: ✅ All JSON lines parsed, streaming completes

### P02-009: llama.cpp Health and Slots
- **Area**: QA-02-05
- **Type**: Integration (wiremock)
- **Command**: `cargo test llamacpp_health --test`
- **Setup**: Wiremock server at localhost:8080 mocking /health and /slots
- **Expected**: health_check() returns HealthStatus::Healthy, slots info retrieved
- **Pass Criteria**: ✅ Health check returns Healthy, slots parsed correctly

### P02-010: OpenAI Rate Limiting
- **Area**: QA-02-06
- **Type**: Integration (wiremock)
- **Command**: `cargo test openai_rate_limit --test`
- **Setup**: Wiremock server returning HTTP 429 with Retry-After header
- **Expected**: Rate limit detected, retry delay calculated from Retry-After header
- **Pass Criteria**: ✅ Retry respects Retry-After delay

### P02-011: Backend Registry Selection
- **Area**: QA-02-07
- **Type**: Unit
- **Command**: `cargo test backend_registry --lib`
- **Setup**: Multiple backends configured
- **Expected**: Backend selected based on provider config, fallback on failure
- **Pass Criteria**: ✅ Selection logic correct, fallback triggers on failure

### P02-012: Tool Permission Evaluation
- **Area**: QA-02-08
- **Type**: Unit
- **Command**: `cargo test permission_evaluation --lib`
- **Setup**: Tool permissions config, tool operation requests
- **Expected**: Permissions evaluated correctly, operations allowed/blocked based on policy
- **Pass Criteria**: ✅ Allow/deny decisions match configuration

### P02-013: Tool Execution with Sandbox
- **Area**: QA-02-08
- **Type**: Integration
- **Command**: `cargo test tool_sandbox --test`
- **Setup**: Tool with sandbox enabled, operation attempting to access forbidden path
- **Expected**: Tool execution blocked by sandbox, error returned
- **Pass Criteria**: ✅ Sandbox prevents unauthorized access

### P02-014: Built-in Tool Functionality
- **Area**: QA-02-09
- **Type**: Integration
- **Command**: `cargo test builtin_tools --test`
- **Setup**: Test file operations, web fetch, shell exec
- **Expected**: Each tool executes correctly, output returned
- **Pass Criteria**: ✅ All built-in tools work as expected

### P02-015: Sub-Workflow Isolation
- **Area**: QA-02-10
- **Type**: Integration
- **Command**: `cargo test subworkflow_isolation --test`
- **Setup**: Parent workflow calls sub-workflow
- **Expected**: Sub-workflow executes in isolated scope, no variable leakage
- **Pass Criteria**: ✅ Isolation enforced, no cross-workflow variable access

### P02-016: Code Generation Templates
- **Area**: QA-02-11
- **Type**: Unit
- **Command**: `cargo test codegen_templates --lib`
- **Setup**: WorkflowIR with various features
- **Expected**: Rust code generated from templates compiles
- **Pass Criteria**: ✅ Generated code compiles without errors

### P02-017: Code Generation Integration
- **Area**: QA-02-11
- **Type**: Integration
- **Command**: `cargo test codegen_integration --test`
- **Setup**: Full workflow YAML → code generation → compile → execute
- **Expected**: Generated code executes and produces expected output
- **Pass Criteria**: ✅ End-to-end code generation works

### P02-018: RAG Document Indexing
- **Area**: QA-02-12
- **Type**: Integration
- **Command**: `cargo test rag_indexing --test`
- **Setup**: Test documents, embedding model configured
- **Expected**: Documents indexed with embeddings stored
- **Pass Criteria**: ✅ Index created, embeddings generated correctly

### P02-019: RAG Semantic Retrieval
- **Area**: QA-02-12
- **Type**: Integration
- **Command**: `cargo test rag_retrieval --test`
- **Setup**: Knowledge base indexed, query executed
- **Expected**: Top-k documents retrieved by similarity, context generated
- **Pass Criteria**: ✅ Relevant documents retrieved, context contains sources

### P02-020: Self-Improvement Analysis
- **Area**: QA-02-13
- **Type**: Integration
- **Command**: `cargo test self_improvement --test`
- **Setup**: Workflow execution logs available
- **Expected**: Patterns identified, improvement suggestions generated
- **Pass Criteria**: ✅ Analysis produces actionable insights

### P02-021: Self-Improvement Diff Generation
- **Area**: QA-02-13
- **Type**: Integration
- **Command**: `cargo test self_improvement_diff --test`
- **Setup**: Before and after workflow configurations
- **Expected**: Diff generated showing changes
- **Pass Criteria**: ✅ Diff accurate, changes clearly presented

### P02-022: Backend Registry Health Monitoring
- **Area**: QA-02-07
- **Type**: Integration
- **Command**: `cargo test registry_health --test`
- **Setup**: Multiple backends, one unhealthy
- **Expected**: Unhealthy backend detected, healthy backend selected
- **Pass Criteria**: ✅ Health monitoring switches to healthy backend

---

## Test Execution Order
**Phase 2 Group 1: CLI Foundation**
P02-001 → P02-002 → P02-003

**Phase 2 Group 2: Backend Traits**
P02-004 → P02-005

**Phase 2 Group 3: Backend Implementations**
P02-006 → P02-007 → P02-008 → P02-009 → P02-010

**Phase 2 Group 4: Backend Registry**
P02-011 → P02-022

**Phase 2 Group 5: Tools & Sub-Workflows**
P02-012 → P02-013 → P02-014 → P02-015

**Phase 2 Group 6: Code Generation**
P02-016 → P02-017

**Phase 2 Group 7: RAG Integration**
P02-018 → P02-019

**Phase 2 Group 8: Self-Improvement**
P02-020 → P02-021
