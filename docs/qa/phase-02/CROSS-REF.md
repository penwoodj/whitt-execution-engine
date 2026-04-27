# Phase 02: Cross-References

## Plan Files
- **Main Plan**: `docs/plans/02-cli-and-llm-backend-integration/plan.md` (613 lines)
- **Task 00**: `docs/plans/02-cli-and-llm-backend-integration/tasks/00-cli-foundation.md`
- **Task 01**: `docs/plans/02-cli-and-llm-backend-integration/tasks/01-llm-backend-trait.md`
- **Task 02**: `docs/plans/02-cli-and-llm-backend-integration/tasks/02-lmstudio-backend.md`
- **Task 03**: `docs/plans/02-cli-and-llm-backend-integration/tasks/03-ollama-backend.md`
- **Task 04**: `docs/plans/02-cli-and-llm-backend-integration/tasks/04-llamacpp-backend.md`
- **Task 05**: `docs/plans/02-cli-and-llm-backend-integration/tasks/05-openai-backend.md`
- **Task 06**: `docs/plans/02-cli-and-llm-backend-integration/tasks/06-backend-registry.md`
- **Task 07**: `docs/plans/02-cli-and-llm-backend-integration/tasks/07-tool-permissions.md`
- **Task 08**: `docs/plans/02-cli-and-llm-backend-integration/tasks/08-tool-execution-framework.md`
- **Task 09**: `docs/plans/02-cli-and-llm-backend-integration/tasks/09-sub-workflow-execution.md`
- **Task 10**: `docs/plans/02-cli-and-llm-backend-integration/tasks/10-code-generation.md`
- **Task 11**: `docs/plans/02-cli-and-llm-backend-integration/tasks/11-rag-integration.md`
- **Task 12**: `docs/plans/02-cli-and-llm-backend-integration/tasks/12-self-improvement-loop.md`

## Schema Sections
- **Section 6**: Tool Permissions (Lines 606-677)
  - File operations: read, write, delete
  - Web operations: fetch, scrape
  - Shell operations: exec
  - Content operations: generate, web_search
  - System operations: process_management, network_operations, file_system_mount, environment_variables, service_management

- **Section 10**: Orchestration Configuration (Lines 558-598)
  - Step coordination and execution
  - Sub-agent spawning and management
  - Validation aggregation
  - Checkpoint coordination
  - Workflow nesting and isolation
  - Reference resolution strategies
  - Policy inheritance and overrides

- **Section 11**: Provider Configuration (Lines 27-57)
  - LM Studio: host, port, connection_timeout
  - Ollama: host, port, timeout
  - llama.cpp: host, port, timeout
  - OpenAI: api_key, timeout

- **Section 12**: RAG Configuration (Lines 682-697)
  - Knowledge base: path, format, chunk_size, chunk_overlap
  - Embedding model: model_ref, dimension, batch_size
  - Retrieval: max_results, similarity_threshold, include_sources
  - Context injection: configuration and parameters

- **Section 19**: Duplicate Config Systems (Lines 792-802)
  - Parallelism configuration (moved to agent-queue)
  - Permissions configuration (single source of truth)
  - Orchestration configuration (single source of truth)

## Related QA
- **Phase 00**: Foundation (depends on WorkflowSpec, WorkflowIR, storage layer)
  - Schema parsing and validation
  - IR compilation
  - Persistence layer
  - Reference: `docs/qa/phase-00/QA-CRITERIA.md`

- **Phase 01**: Core Execution Engine (depends on queue, scheduler, step executor)
  - ChatSession containers
  - Queue state machine
  - Scheduler core
  - Step executor interface
  - Reference: `docs/qa/phase-01/QA-CRITERIA.md`

## Validation Criteria
- **Framework**: `docs/plans/validation-criteria/framework.md` (571 lines)
  - 7 Verification Layers: Unit, Integration, Property, E2E, System Log, CLI, Benchmark
  - Evidence collection requirements
  - Phase entry/exit criteria
  - Checkpoint gate criteria
  - Task-level acceptance criteria
  - Cross-phase regression tests
  - Schema coverage audit
  - ADR compliance verification
  - Anti-goal-drift detection

- **ADR-0003 Compliance**: Schema Domain Ownership
  - Section 6: Tool Permissions - complete tool permissions system
  - Section 10: Orchestration - complete orchestration support
  - Section 11: Provider Configuration - LLM provider configs
  - Section 12: RAG Configuration - RAG system
  - Section 19: Duplicate Config Systems - clarification

## External References
- **HTTP API Details**: LM Studio, Ollama, llama.cpp, OpenAI endpoints (Lines 371-444 in plan.md)
- **Wiremock Testing Strategies**: HTTP mocking for backend tests (Lines 356-365 in plan.md)
- **Streaming Support**: SSE and NDJSON parsing (Lines 280-304 in plan.md)
- **Error Handling**: Library errors (thiserror), application errors (anyhow) (Lines 473-491 in plan.md)

---

## Related Documentation
- **Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
- **Extended POC QA**: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md` (335 lines)
- **Extended POC Test Procedures**: `docs/qa/extended-poc/QA-TEST-PROCEDURES-EXTENDED-POC.md` (1405 lines)
