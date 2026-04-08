# Phase 2: CLI & LLM Backends Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the CLI interface and LLM backend abstraction layer with support for multiple providers (LM Studio, Ollama, llama.cpp, OpenAI), tool permissions system, code generation, RAG integration, and self-improvement loop infrastructure.

**Architecture:** CLI-first design using clap for argument parsing, async trait-based backend abstraction with wiremock for HTTP mocking, streaming support via SSE/NDJSON parsers, permission-guarded tool execution, and dual execution modes (direct and code-generated).

**Tech Stack:** Rust (1.75+), clap 4.6 (CLI), reqwest 0.13 (HTTP), async-trait, tokio, serde, thiserror, anyhow, wiremock (HTTP mocking), Askama (templating), futures (streaming).

---

## Phase Overview

This phase delivers the core user-facing CLI and the LLM backend abstraction layer that enables AgentSDK to interact with multiple LLM providers. This is the most technically complex phase, requiring careful design of:

1. **CLI Foundation** - User interface with subcommands, configuration, and output formatting
2. **LLM Backend Abstraction** - Unified trait for multiple providers with streaming support
3. **Backend Implementations** - Concrete implementations for LM Studio, Ollama, llama.cpp, and OpenAI
4. **Backend Registry** - Discovery, selection, fallback, and health monitoring
5. **Tool Permissions** - Security layer for tool execution with allow/deny lists and confirmation flow
6. **Tool Execution Framework** - Built-in tools (file, web, shell) and custom tool registration
7. **Sub-Workflow Execution** - Workflow nesting, composition, and isolation
8. **Code Generation** - WorkflowIR to Rust code generation with Askama templates
9. **RAG Integration** - Document indexing, embeddings, and semantic retrieval
10. **Self-Improvement Loop** - Execution analysis, diff generation, and automated updates

**Estimated Time:** 8-10 weeks

**Dependencies:**
- **Phase 0:** WorkflowSpec, WorkflowIR, parser, .glyphnova/ storage
- **Phase 1:** Queue, Scheduler, StepExecutor, LoopRunner

---

## ADR-0003 Compliance

This phase adheres to ADR-0003 (Schema Domain Ownership) with the following domain assignments:

### Schema Domain: Sections 6, 10, 11, 12, 19

- **Section 6: Tool Permissions** - This phase owns the complete tool permissions system including:
  - File operation permissions (read/write/delete/move)
  - Web operation permissions (fetch/scrape)
  - Shell execution permissions
  - AI operation permissions
  - Allow/deny list configuration
  - Confirmation flow implementation
  - Guardrails enforcement

- **Section 10: Orchestration** - Complete orchestration support:
  - Step coordination and execution
  - Sub-agent spawning and management
  - Validation aggregation
  - Checkpoint coordination
  - Workflow nesting and isolation

- **Section 11: Provider Configuration** - LLM provider configs:
  - LM Studio host/port settings
  - Ollama host/port settings
  - llama.cpp host/port settings
  - OpenAI API key and endpoints
  - Provider selection and fallback

- **Section 12: RAG Configuration** - RAG system:
  - Knowledge base configuration
  - Embedding model selection
  - Retrieval parameters
  - Context injection settings

- **Section 19: Duplicate Config Systems** - Clarification for:
  - Parallelism configuration
  - Permissions configuration (single source of truth)
  - Orchestration configuration (single source of truth)

---

## Key Concepts

### CLI-First Design
- All functionality exposed through CLI with clear subcommands
- Configuration loaded from `~/.glyphnova/config.yaml`
- Tab completion support for all commands and options
- Output formatting (plain, JSON, table) for different use cases

### Backend Provider Abstraction
- Unified `LlmBackend` trait abstracts provider differences
- Streaming support via async streams
- Capability-based feature detection
- Health monitoring and automatic fallback
- Wiremock for HTTP mocking in tests

### Networking Opt-In
- No network access by default
- Explicit opt-in via configuration
- Provider selection respects networking policy
- Clear error messages when network is required but disabled

### Tool Permissions
- Default-deny policy for all tools
- Allow lists configured per workflow or globally
- Per-step restrictions for granular control
- Interactive confirmation flow for sensitive operations
- Guardrails enforcement (path traversal, command injection prevention)

### Dual Execution Modes
- **Direct Mode:** Execute workflows directly from WorkflowIR
- **Code-Generated Mode:** Generate Rust code from WorkflowIR, compile, and execute
- Mode selection based on configuration and complexity

### Self-Improvement Loop
- Execution logging captures all decisions and outcomes
- Analysis identifies patterns and improvement opportunities
- Diff generation shows changes to apply
- Automated updates with confirmation

---

## Rust Crate Integration

### Core Dependencies

```toml
[dependencies]
# CLI
clap = { version = "4.6", features = ["derive", "env"] }

# Async runtime
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"
futures = "0.3"

# HTTP client
reqwest = { version = "0.13", features = ["json", "stream"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"  # For library errors
anyhow = "1.0"     # For application errors

# Templating (code generation)
askama = "0.12"

# Stream parsing
bytes = "1.5"

# Configuration
config = "0.14"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
# HTTP mocking
wiremock = "0.6"

# Testing
tokio-test = "0.4"
proptest = "1.4"
```

---

## Task Overview and Dependencies

```
┌─────────────────────────────────────────────────────────────┐
│ 00: CLI Foundation                                          │
│    clap derive API, subcommands, global options             │
└────────────┬────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│ 01: LLM Backend Trait                                       │
│    unified LlmBackend trait, streaming support              │
└────────────┬────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│ 02-05: Backend Implementations                              │
│    LM Studio, Ollama, llama.cpp, OpenAI                      │
└────────────┬────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│ 06: Backend Registry                                        │
│    discovery, selection, fallback, health monitoring        │
└────────────┬────────────────────────────────────────────────┘
             │
             ├──────────────────────────────────┐
             ▼                                  ▼
┌─────────────────────────────┐  ┌─────────────────────────────┐
│ 07: Tool Permissions       │  │ 08: Tool Execution          │
│    allow/deny, confirm      │  │    built-in + custom tools  │
└────────────┬────────────────┘  └────────────┬────────────────┘
             │                                  │
             └──────────────┬───────────────────┘
                            ▼
              ┌─────────────────────────────┐
              │ 09: Sub-Workflow Execution  │
              │    nesting, composition     │
              └────────────┬────────────────┘
                           │
                           ├────────────────────────┐
                           ▼                        ▼
            ┌──────────────────────────┐  ┌──────────────────────────┐
            │ 10: Code Generation      │  │ 11: RAG Integration       │
            │    WorkflowIR → Rust     │  │    indexing, retrieval    │
            └────────────┬─────────────┘  └────────────┬─────────────┘
                         │                              │
                         └──────────────┬───────────────┘
                                        ▼
                         ┌──────────────────────────┐
                         │ 12: Self-Improvement Loop│
                         │    analysis, updates      │
                         └──────────────────────────┘
```

### Task Dependencies

- **Task 00** (CLI Foundation) is independent - can start immediately
- **Task 01** (LLM Backend Trait) depends on no other tasks - can start immediately
- **Tasks 02-05** (Backend Implementations) depend on Task 01 - can run in parallel
- **Task 06** (Backend Registry) depends on Tasks 02-05
- **Task 07** (Tool Permissions) is independent - can start immediately
- **Task 08** (Tool Execution) depends on Task 07
- **Task 09** (Sub-Workflow) depends on Task 08
- **Task 10** (Code Generation) depends on Task 01
- **Task 11** (RAG) depends on Task 06 (for backend selection)
- **Task 12** (Self-Improvement) depends on Task 10

---

## File Structure Mapping

### New Modules

```
src/
├── cli/                          # CLI implementation
│   ├── mod.rs                    # Module exports
│   ├── main.rs                   # CLI entry point
│   ├── commands/                 # Subcommand implementations
│   │   ├── mod.rs
│   │   ├── run.rs                # Run workflow
│   │   ├── generate.rs           # Generate code
│   │   ├── queue.rs              # Queue management
│   │   ├── status.rs             # Status queries
│   │   └── config.rs             # Config management
│   ├── config/                   # Configuration loading
│   │   ├── mod.rs
│   │   └── loader.rs             # Config file loader
│   └── output/                   # Output formatting
│       ├── mod.rs
│       ├── plain.rs              # Plain text output
│       ├── json.rs               # JSON output
│       └── table.rs              # Table output
│
├── backends/                     # LLM backends
│   ├── mod.rs                    # Module exports
│   ├── trait.rs                  # LlmBackend trait definition
│   ├── types.rs                  # Shared types (ChatRequest, etc.)
│   ├── lmstudio/                 # LM Studio implementation
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── ollama/                   # Ollama implementation
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── llamacpp/                 # llama.cpp implementation
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── openai/                   # OpenAI implementation
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── registry.rs               # Backend registry
│   ├── streaming/                # Streaming parsers
│   │   ├── mod.rs
│   │   ├── sse.rs                # SSE parser
│   │   └── ndjson.rs             # NDJSON parser
│   └── errors.rs                 # Backend-specific errors
│
├── tools/                        # Tool execution
│   ├── mod.rs                    # Module exports
│   ├── trait.rs                  # Tool trait definition
│   ├── permissions/              # Permission system
│   │   ├── mod.rs
│   │   ├── manager.rs            # Permission manager
│   │   └── config.rs             # Permission configuration
│   ├── builtin/                  # Built-in tools
│   │   ├── mod.rs
│   │   ├── file.rs               # File operations
│   │   ├── web.rs                # Web operations
│   │   └── shell.rs              # Shell operations
│   ├── registry.rs               # Tool registry
│   └── execution.rs              # Tool execution engine
│
├── workflows/                    # Workflow execution (new in Phase 2)
│   ├── mod.rs                    # Module exports
│   ├── nesting.rs                # Sub-workflow execution
│   ├── isolation.rs              # Isolation mechanisms
│   └── composition.rs            # Workflow composition
│
├── codegen/                      # Code generation
│   ├── mod.rs                    # Module exports
│   ├── generator.rs              # Code generator
│   └── templates/                # Askama templates
│       ├── mod.rs
│       ├── workflow.rs           # Workflow template
│       └── step.rs               # Step template
│
└── rag/                          # RAG integration
    ├── mod.rs                    # Module exports
    ├── index.rs                  # Document indexing
    ├── embeddings.rs             # Embedding generation
    ├── retrieval.rs              # Semantic retrieval
    └── context.rs                # Context injection
```

### Modified Files

```
src/
├── lib.rs                        # Add new modules
├── executor/                     # Extend from Phase 1
│   └── step_executor.rs         # Add tool execution integration
└── config.rs                     # Add provider config section
```

---

## Testing Strategy

### Unit Tests
- Individual trait implementations
- Streaming parser logic
- Permission evaluation
- Tool registration
- Code generation templates

### Integration Tests
- Backend implementations with wiremock
- End-to-end workflow execution
- Permission enforcement
- Sub-workflow nesting
- Code generation and execution

### Property Tests
- Streaming parser invariants
- Permission logic properties
- Backend selection behavior

### Mock Strategies

**CRITICAL:** All LLM backend tests use wiremock for HTTP mocking:

- **LM Studio:** Mock POST /v1/chat/completions with SSE streaming
- **Ollama:** Mock POST /api/chat with NDJSON streaming
- **llama.cpp:** Mock POST /v1/chat/completions with SSE, GET /health, GET /slots
- **OpenAI:** Mock POST /v1/chat/completions with SSE, 429 rate limiting responses

See `tests/backend-mock-strategies.md` for detailed mock implementations.

---

## HTTP API Details

### LM Studio (localhost:1234)
```
POST /v1/chat/completions
Content-Type: application/json

{
  "model": "model-id",
  "messages": [...],
  "tools": [...],
  "stream": true
}

Response: Server-Sent Events (SSE)
data: {"choices":[{"delta":{"content":"..."}}]}
```

### Ollama (localhost:11434)
```
POST /api/chat
Content-Type: application/json

{
  "model": "model-id",
  "messages": [...],
  "tools": [...],
  "stream": true,
  "format": "json",
  "think": true
}

Response: NDJSON stream
{"model":"...","message":{"role":"assistant","content":"..."},"done":false}
```

### llama.cpp (localhost:8080)
```
POST /v1/chat/completions
Content-Type: application/json

{
  "model": "model-id",
  "messages": [...],
  "tools": [...],
  "stream": true
}

Response: Server-Sent Events (SSE)
data: {"choices":[{"delta":{"content":"..."}}]}

GET /health
Response: {"status":"ok"}

GET /slots
Response: {"slots":[...]}
```

### OpenAI (api.openai.com)
```
POST /v1/chat/completions
Authorization: Bearer <api-key>
Content-Type: application/json

{
  "model": "gpt-4",
  "messages": [...],
  "tools": [...],
  "stream": true
}

Response: Server-Sent Events (SSE)
data: {"choices":[{"delta":{"content":"..."}}]}

Rate Limit: HTTP 429 with Retry-After header
```

---

## Execution Flow

### Direct Mode
```
CLI → Parser → WorkflowIR → Executor → Tools → LLM Backend
                                                          ↓
                                                       Result
```

### Code-Generated Mode
```
CLI → Parser → WorkflowIR → Code Generator → Rust Code
                                      ↓
                                   Compiler
                                      ↓
                                  Binary → Tools → LLM Backend
                                                          ↓
                                                       Result
```

---

## Error Handling

### Library Errors (thiserror)
```rust
#[derive(thiserror::Error, Debug)]
pub enum LlmError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Backend error: {0}")]
    Backend(String),
}
```

### Application Errors (anyhow)
```rust
// Use anyhow::Result<()> for application-level functions
// Use anyhow::Context!() for error context
```

---

## Configuration Structure

```yaml
# ~/.glyphnova/config.yaml
general:
  network_enabled: false  # Opt-in for network access
  output_format: plain   # plain | json | table

providers:
  default: lmstudio

  lmstudio:
    host: localhost
    port: 1234
    timeout: 300

  ollama:
    host: localhost
    port: 11434
    timeout: 300

  llamacpp:
    host: localhost
    port: 8080
    timeout: 300

  openai:
    api_key: ${OPENAI_API_KEY}  # Environment variable
    timeout: 300

permissions:
  default_policy: deny  # allow | deny

  tools:
    allow:
      - file.read
      - file.write
    deny:
      - shell.exec

  confirmation_required:
    - file.delete
    - shell.exec

rag:
  enabled: false
  knowledge_base: ~/.glyphnova/knowledge
  embedding_model: nomic-embed-text
  max_context: 2000
  retrieval_limit: 5

codegen:
  enabled: false
  output_dir: ~/.glyphnova/generated
  compile_on_generate: true

self_improvement:
  enabled: false
  log_dir: ~/.glyphnova/logs
  auto_apply: false
```

---

## Task Files Reference

Individual task files are located in `tasks/`:

- `00-cli-foundation.md` - CLI implementation
- `01-llm-backend-trait.md` - Backend trait and types
- `02-lmstudio-backend.md` - LM Studio implementation
- `03-ollama-backend.md` - Ollama implementation
- `04-llamacpp-backend.md` - llama.cpp implementation
- `05-openai-backend.md` - OpenAI implementation
- `06-backend-registry.md` - Backend registry
- `07-tool-permissions.md` - Permission system
- `08-tool-execution-framework.md` - Tool execution
- `09-sub-workflow-execution.md` - Sub-workflows
- `10-code-generation.md` - Code generation
- `11-rag-integration.md` - RAG integration
- `12-self-improvement-loop.md` - Self-improvement

Each task file contains:
- Exact file paths to create/modify
- Complete code implementations
- Test strategies with wiremock examples
- Step-by-step TDD process
- Commit messages

---

## Validation and Acceptance Criteria

See `validation/` directory:
- `checkpoint-criteria.md` - Criteria for each task completion
- `acceptance-criteria.md` - Overall phase acceptance criteria

---

## Test Specifications

See `tests/` directory:
- `unit-tests.md` - Unit test specifications
- `integration-tests.md` - Integration test specifications
- `property-tests.md` - Property test specifications
- `backend-mock-strategies.md` - Wiremock mock strategies (CRITICAL)

---

## Next Steps

**After this plan is complete and reviewed:**

Choose execution approach:

1. **Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration
2. **Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
