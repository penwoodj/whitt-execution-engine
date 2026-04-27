# Phase 02: QA Criteria

## Overview
Phase 02 implements CLI interface and LLM backend abstraction layer with support for multiple providers, tool permissions, code generation, RAG integration, and self-improvement loop. This phase provides user-facing interface and enables actual LLM execution.

## Dependencies
- **Phase 00**: WorkflowSpec, WorkflowIR, parser, ./workspace/ storage
- **Phase 01**: Queue, Scheduler, StepExecutor, LoopRunner

## QA Areas

### QA-02-01: CLI Foundation
- **Plan Ref**: Task 00 (`docs/plans/02-cli-and-llm-backend-integration/tasks/00-cli-foundation.md`)
- **Schema Ref**: N/A (CLI interface)
- **Priority**: P0
- **Test Type**: Unit + Integration
- **Criteria**:
  - CLI entry point implemented with clap derive API
  - All subcommands defined: run, generate, queue, status, config
  - Global options: output-format (plain|json|table), verbose, config-path
  - Configuration loaded from `~/./workspace/config.yaml` or specified path
  - Tab completion support for all commands and options
  - Output formatting works for plain, JSON, and table formats
- **Commands**:
  - `cargo test cli --lib`
  - `cargo test cli_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-02: LLM Backend Trait
- **Plan Ref**: Task 01 (`docs/plans/02-cli-and-llm-backend-integration/tasks/01-llm-backend-trait.md`)
- **Schema Ref**: Lines 27-57 (providers section - backend interface)
- **Priority**: P0
- **Test Type**: Unit
- **Criteria**:
  - Unified `LlmBackend` trait with all required methods: chat, chat_stream, list_models, health_check, load_model, unload_model, capabilities, base_url
  - Streaming support via async streams (futures::Stream)
  - Shared types: ChatRequest, ChatResponse, StreamChunk
  - BackendCapabilities struct: streaming, tools, function_calling
  - HealthStatus enum: Healthy, Degraded, Unhealthy
  - LlmError enum: Network, Parse, Backend, RateLimited, Timeout
  - Trait is object-safe (can use `dyn LlmBackend`)
- **Commands**:
  - `cargo test backend_trait --lib`
  - `cargo clippy -- -D warnings`

### QA-02-03: LM Studio Backend
- **Plan Ref**: Task 02 (`docs/plans/02-cli-and-llm-backend-integration/tasks/02-lmstudio-backend.md`)
- **Schema Ref**: Lines 27-57 (lmstudio provider)
- **Priority**: P0
- **Test Type**: Integration (wiremock)
- **Criteria**:
  - POST /v1/chat/completions sends correct request format
  - SSE streaming response parsed correctly (data: lines)
  - Health check (custom endpoint)
  - Model listing (custom endpoint or inferred)
  - Connection timeout, request timeout configured
  - Wiremock test scenarios: success, slow streaming, connection drops, rate limiting
- **Commands**:
  - `cargo test lmstudio_backend --lib`
  - `cargo test lmstudio_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-04: Ollama Backend
- **Plan Ref**: Task 03 (`docs/plans/02-cli-and-llm-backend-integration/tasks/03-ollama-backend.md`)
- **Schema Ref**: Lines 53-57 (ollama provider)
- **Priority**: P0
- **Test Type**: Integration (wiremock)
- **Criteria**:
  - POST /api/chat with format=json, stream=true, think=true
  - NDJSON streaming response parsed correctly
  - Model listing (GET /api/tags)
  - Connection timeout, request timeout configured
  - Wiremock test scenarios: success, slow streaming, connection drops, rate limiting
- **Commands**:
  - `cargo test ollama_backend --lib`
  - `cargo test ollama_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-05: llama.cpp Backend
- **Plan Ref**: Task 04 (`docs/plans/02-cli-and-llm-backend-integration/tasks/04-llamacpp-backend.md`)
- **Schema Ref**: Lines 56-57 (llama_cpp_with_vulkan provider)
- **Priority**: P0
- **Test Type**: Integration (wiremock)
- **Criteria**:
  - POST /v1/chat/completions with stream=true
  - SSE streaming response parsed correctly
  - GET /health returns status
  - GET /slots returns slot information
  - POST /v1/models/load loads model
  - POST /v1/models/unload unloads model
  - Connection timeout, request timeout configured
  - Wiremock test scenarios: success, slow streaming, connection drops, rate limiting, health failures
- **Commands**:
  - `cargo test llamacpp_backend --lib`
  - `cargo test llamacpp_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-06: OpenAI Backend
- **Plan Ref**: Task 05 (`docs/plans/02-cli-and-llm-backend-integration/tasks/05-openai-backend.md`)
- **Schema Ref**: N/A (OpenAI external provider)
- **Priority**: P1
- **Test Type**: Integration (wiremock)
- **Criteria**:
  - POST /v1/chat/completions with Bearer token authorization
  - SSE streaming response parsed correctly
  - HTTP 429 rate limiting handled with Retry-After header
  - API key from environment variable or config
  - Connection timeout, request timeout configured
  - Wiremock test scenarios: success, slow streaming, connection drops, rate limiting (429)
- **Commands**:
  - `cargo test openai_backend --lib`
  - `cargo test openai_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-07: Backend Registry
- **Plan Ref**: Task 06 (`docs/plans/02-cli-and-llm-backend-integration/tasks/06-backend-registry.md`)
- **Schema Ref**: Lines 64-91 (model routing + backend selection)
- **Priority**: P0
- **Test Type**: Unit + Integration
- **Criteria**:
  - Backend discovery from configuration
  - Backend selection based on provider config
  - Fallback on backend failures
  - Health monitoring with automatic switching
  - Backend capability detection
  - Default router: automatic vs manual
- **Commands**:
  - `cargo test backend_registry --lib`
  - `cargo test registry_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-08: Tool Permissions
- **Plan Ref**: Task 07 (`docs/plans/02-cli-and-llm-backend-integration/tasks/07-tool-permissions.md`)
- **Schema Ref**: Lines 606-677 (tool_permissions)
- **Priority**: P0
- **Test Type**: Unit + Integration
- **Criteria**:
  - Default-deny policy for all tools
  - Allow lists configured per workflow or globally
  - Per-step restrictions for granular control
  - Interactive confirmation flow for sensitive operations
  - Guardrails enforcement (path traversal, command injection prevention)
  - File operations: read/write/delete permissions, allowed/forbidden paths, max_file_size
  - Web operations: fetch/scrape permissions, allowed/forbidden domains, rate limits
  - Shell operations: exec permissions, allowed/forbidden commands, timeout
  - AI operations: generate/web_search permissions, rate limits
- **Commands**:
  - `cargo test tool_permissions --lib`
  - `cargo test permissions_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-09: Tool Execution Framework
- **Plan Ref**: Task 08 (`docs/plans/02-cli-and-llm-backend-integration/tasks/08-tool-execution-framework.md`)
- **Schema Ref**: Lines 606-677 (tool permissions)
- **Priority**: P0
- **Test Type**: Integration
- **Criteria**:
  - Tool trait defined with execute() method
  - Built-in tools: file, web, shell operations
  - Tool registration and discovery
  - Tool execution with permission checks
  - Tool sandboxing enforced
  - Tool output captured and returned
  - Custom tool registration API
  - Tool errors handled gracefully
- **Commands**:
  - `cargo test tool_execution --lib`
  - `cargo test tool_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-10: Sub-Workflow Execution
- **Plan Ref**: Task 09 (`docs/plans/02-cli-and-llm-backend-integration/tasks/09-sub-workflow-execution.md`)
- **Schema Ref**: Lines 165-191 (sub_workflows)
- **Priority**: P1
- **Test Type**: Unit + Integration
- **Criteria**:
  - Nested workflow execution supported
  - Sub-workflow isolation and scoping
  - Sub-workflow result handling
  - Sub-workflow error propagation
  - Nested execution depth limits enforced
  - Sub-workflow input/output passing
  - Sub-workflow reference resolution (local_first, hierarchical, registry_only)
- **Commands**:
  - `cargo test sub_workflow --lib`
  - `cargo test workflow_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-11: Code Generation
- **Plan Ref**: Task 10 (`docs/plans/02-cli-and-llm-backend-integration/tasks/10-code-generation.md`)
- **Schema Ref**: Lines 1-805 (full schema → Rust code)
- **Priority**: P1
- **Test Type**: Integration
- **Criteria**:
  - WorkflowIR → Rust code generation using Askama templates
  - Generated code compiles without errors
  - Generated code executes correctly
  - Template for main workflow, steps, and types
  - Generated code includes all workflow features
  - Compile and execute generated code
- **Commands**:
  - `cargo test code_generation --lib`
  - `cargo test codegen_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-12: RAG Integration
- **Plan Ref**: Task 11 (`docs/plans/02-cli-and-llm-backend-integration/tasks/11-rag-integration.md`)
- **Schema Ref**: Lines 682-697 (memory/rag section)
- **Priority**: P1
- **Test Type**: Integration
- **Criteria**:
  - Knowledge base configuration (path, format, chunk_size, chunk_overlap)
  - Embedding model selection and configuration
  - Retrieval parameters (max_results, similarity_threshold, include_sources)
  - Document indexing with embeddings
  - Semantic retrieval with context injection
  - Context injection into prompts
- **Commands**:
  - `cargo test rag_integration --lib`
  - `cargo test rag_integration --test`
  - `cargo clippy -- -D warnings`

### QA-02-13: Self-Improvement Loop
- **Plan Ref**: Task 12 (`docs/plans/02-cli-and-llm-backend-integration/tasks/12-self-improvement-loop.md`)
- **Schema Ref**: N/A (meta-optimization)
- **Priority**: P2
- **Test Type**: Integration
- **Criteria**:
  - Execution logging captures all decisions and outcomes
  - Analysis identifies patterns and improvement opportunities
  - Diff generation shows changes to apply
  - Automated updates with confirmation
  - Log directory and auto_apply configuration
  - Self-improvement disabled by default
- **Commands**:
  - `cargo test self_improvement --lib`
  - `cargo test self_improvement_integration --test`
  - `cargo clippy -- -D warnings`

---

## Verification Summary
**Total QA Areas**: 13
**P0 Areas**: 8 (02-01, 02-02, 02-03, 02-04, 02-05, 02-07, 02-08, 02-09)
**P1 Areas**: 5 (02-06, 02-10, 02-11, 02-12, 02-13)
**P2 Areas**: 1 (02-13)

**Schema Coverage**: 100% of Sections 6, 10, 11, 12, 19

---

## Related Plan Tasks

- [Task 00](../../plans/02-cli-and-llm-backend-integration/tasks/00-cli-foundation.md) — CLI entry point with clap derive API, all subcommands, global options, configuration loading, and tab completion
- [Task 01](../../plans/02-cli-and-llm-backend-integration/tasks/01-llm-backend-trait.md) — Unified LlmBackend trait with all required methods, streaming support, shared types, and capabilities
- [Task 02](../../plans/02-cli-and-llm-backend-integration/tasks/02-lmstudio-backend.md) — LM Studio backend with SSE streaming response parsing, health check, model listing
- [Task 03](../../plans/02-cli-and-llm-backend-integration/tasks/03-ollama-backend.md) — Ollama backend with NDJSON streaming response parsing, model listing, and configuration
- [Task 04](../../plans/02-cli-and-llm-backend-integration/tasks/04-llamacpp-backend.md) — llama.cpp backend with health check, slots info, model loading/unloading
- [Task 05](../../plans/02-cli-and-llm-backend-integration/tasks/05-openai-backend.md) — OpenAI backend with Bearer token authorization, SSE streaming, rate limit handling
- [Task 06](../../plans/02-cli-and-llm-backend-integration/tasks/06-backend-registry.md) — Backend discovery, selection, fallback, health monitoring, and capability detection
- [Task 07](../../plans/02-cli-and-llm-backend-integration/tasks/07-tool-permissions.md) — Default-deny policy, allow/deny lists, per-step restrictions, guardrails enforcement
- [Task 08](../../plans/02-cli-and-llm-backend-integration/tasks/08-tool-execution-framework.md) — Tool trait, built-in tools, registration, discovery, sandboxing, custom tools
- [Task 09](../../plans/02-cli-and-llm-backend-integration/tasks/09-sub-workflow-execution.md) — Nested workflow execution with isolation, scoping, result handling, and depth limits
- [Task 10](../../plans/02-cli-and-llm-backend-integration/tasks/10-code-generation.md) — WorkflowIR to Rust code generation with Askama templates, compilation, and execution
- [Task 11](../../plans/02-cli-and-llm-backend-integration/tasks/11-rag-integration.md) — Knowledge base configuration, embedding model selection, retrieval parameters, document indexing, and context injection
- [Task 12](../../plans/02-cli-and-llm-backend-integration/tasks/12-self-improvement-loop.md) — Execution logging, pattern identification, diff generation, automated updates

---

## References
- **Plan**: `docs/plans/02-cli-and-llm-backend-integration/plan.md` (613 lines, 13 tasks)
- **Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
- **Validation Framework**: `docs/plans/validation-criteria/framework.md` (571 lines)
- **Existing QA**: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md` (335 lines)
