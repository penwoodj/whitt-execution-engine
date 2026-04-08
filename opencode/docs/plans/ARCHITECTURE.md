# AgentSDK Execution Engine — System Architecture

**Version**: 2.0
**Created**: 2026-04-06
**Status**: Planning Phase

---

## 1. System Identity

The AgentSDK Execution Engine is a **compiler-centered, local-first agentic orchestration framework** that:

1. Reads YAML workflow definitions conforming to the unified schema (1705 lines)
2. Compiles them through a typed pipeline: YAML → WorkflowSpec → WorkflowIR
3. Executes WorkflowIR against local LLM backends (LM Studio, Ollama, llama.cpp)
4. Provides dual execution: Direct (development) and Code Generation (production)
5. Persists all artifacts in `.glyphnova/` system-of-record

**NOT** a thin chat wrapper. NOT a simple YAML executor. This is a compiler-runtime system.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          USER INTERFACE LAYER                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────────────┐  │
│  │  CLI (clap)  │  │  TUI (ratatui)│  │  Glyphnova UI (Tauri + React)   │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┬───────────────────┘  │
│         └─────────────────┼──────────────────────────┘                     │
│                           │                                                │
├───────────────────────────┼────────────────────────────────────────────────┤
│                    SHARED API LAYER (REST + WebSocket)                      │
│  ┌────────────────────────┼─────────────────────────────────────────────┐  │
│  │                 Unified Backend API                                 │  │
│  │  Queue API │ Workflow API │ Scheduler API │ Storage API │ Metrics API │  │
│  └────────────────────────┼─────────────────────────────────────────────┘  │
├───────────────────────────┼────────────────────────────────────────────────┤
│                     EXECUTION ENGINE LAYER                                  │
│  ┌────────────────────────┼─────────────────────────────────────────────┐  │
│  │                                                                    │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────────┐    │  │
│  │  │   QUEUE &   │  │  SCHEDULER   │  │  HUMAN GATING           │    │  │
│  │  │  ChatSession │  │  (tokio)     │  │  (confirm/diff/review)  │    │  │
│  │  └──────┬──────┘  └──────┬───────┘  └────────────┬────────────┘    │  │
│  │         │                │                       │                  │  │
│  │  ┌──────┴────────────────┴───────────────────────┴────────────┐   │  │
│  │  │              WORKFLOW EXECUTION ENGINE                      │   │  │
│  │  │  Step Executor │ Loop Runner │ Branch Evaluator │ Paralleler│   │  │
│  │  └──────┬──────────────────────────────────────────────────────┘   │  │
│  │         │                                                         │  │
│  │  ┌──────┴──────────────────────────────────────────────────────┐   │  │
│  │  │              TOOL EXECUTION FRAMEWORK                       │   │  │
│  │  │  File Tools │ Web Tools │ Shell Tools │ Custom Rust Tools    │   │  │
│  │  └──────┬──────────────────────────────────────────────────────┘   │  │
│  └─────────┼───────────────────────────────────────────────────────────┘  │
├────────────┼──────────────────────────────────────────────────────────────┤
│            │                  LLM BACKEND LAYER                           │
│  ┌─────────┴───────────────────────────────────────────────────────────┐ │
│  │               LlmBackend Trait (async)                              │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌────────────┐ │ │
│  │  │  LM Studio   │ │   Ollama     │ │  llama.cpp   │ │  OpenAI    │ │ │
│  │  │  (SSE/1234)  │ │ (NDJSON/11434)│ │ (SSE/8080)   │ │ (SSE/cloud)│ │ │
│  │  └──────────────┘ └──────────────┘ └──────────────┘ └────────────┘ │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────────────────────────┤
│                       INFRASTRUCTURE LAYER                                │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │
│  │    sled      │ │   tracing    │ │  checkpoints │ │   metrics    │   │
│  │  (state)     │ │ (observability)│ │ (persistence)│ │  (collect)   │   │
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Compiler Pipeline

```
YAML Source (1705-line schema)
        │
        ▼
┌──────────────────┐
│  YAML Parser     │  yaml_serde → serde
│  (Phase 0)       │  Validates against schema types
└────────┬─────────┘
         │ WorkflowSpec (typed Rust struct)
         ▼
┌──────────────────┐
│  Schema Validator│  schemars-derived JSON Schema
│  (Phase 0)       │  Reference validation, DAG check
└────────┬─────────┘
         │ Validated WorkflowSpec
         ▼
┌──────────────────┐
│  IR Compiler     │  WorkflowSpec → WorkflowIR
│  (Phase 0)       │  Type checking, state-machine validation
└────────┬─────────┘
         │ WorkflowIR (typed internal representation)
         ▼
    ┌───────────┐
    │  Executor  │──► Direct Execution (development mode)
    │  (Phase 1) │──► Code Generation → Compile (production mode)
    └───────────┘
```

---

## 4. Core Trait Definitions

### LlmBackend (Phase 2)

```rust
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// Non-streaming chat completion
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, LlmError>;
    
    /// Streaming chat completion
    async fn chat_stream(
        &self, 
        request: ChatRequest
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, LlmError>> + Send>>, LlmError>;
    
    /// List available models
    async fn list_models(&self) -> Result<Vec<ModelInfo>, LlmError>;
    
    /// Check backend health
    async fn health_check(&self) -> Result<HealthStatus, LlmError>;
    
    /// Load a model into memory
    async fn load_model(&self, model_id: &str) -> Result<(), LlmError>;
    
    /// Unload a model from memory
    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError>;
    
    /// Get backend capabilities
    fn capabilities(&self) -> BackendCapabilities;
}
```

### ToolExecutor (Phase 2)

```rust
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn permissions(&self) -> &ToolPermissions;
    
    async fn execute(
        &self,
        params: serde_json::Value,
        context: &ToolContext
    ) -> Result<ToolResult, ToolError>;
    
    fn validate_params(&self, params: &serde_json::Value) -> Result<(), ToolError>;
}
```

### Verifier (Phase 4)

```rust
#[async_trait]
pub trait Verifier: Send + Sync {
    fn name(&self) -> &str;
    fn supported_types(&self) -> Vec<ArtifactType>;
    
    async fn verify(
        &self,
        artifact: &Artifact,
        criteria: &VerificationCriteria
    ) -> Result<VerificationResult, VerificationError>;
}
```

---

## 5. State Machine: Model Lifecycle

```
                    ┌────────────┐
                    │   LOADED   │
                    │ (in memory)│
                    └─────┬──────┘
                          │ warmup()
                          ▼
                    ┌────────────┐
              ┌────│   WARMING  │────┐
              │    │ (caching)  │    │
              │    └─────┬──────┘    │
              │          │           │
              │    warm │           │ idle timeout
              │    done │           │
              │          ▼           │
              │    ┌────────────┐    │
              │    │   ACTIVE   │────┘
              │    │ (serving)  │
              │    └─────┬──────┘
              │          │ idle detection
              │          ▼
              │    ┌────────────┐
              │    │  COOLING   │
              │    │(flush cache)│
              │    └─────┬──────┘
              │          │
              │  re-warm │ unload complete
              └──────────┘
                         │
                         ▼
                   ┌────────────┐
                   │  UNLOADED  │
                   │(resources  │
                   │  freed)    │
                   └────────────┘
```

---

## 6. State Machine: Queue Item

```
  ┌──────────┐
  │ PENDING  │◄────── created
  └────┬─────┘
       │ schedule()
       ▼
  ┌──────────┐
  │ SCHEDULED│◄────── cron trigger / manual
  └────┬─────┘
       │ start()
       ▼
  ┌──────────┐     pause()     ┌──────────┐
  │ RUNNING  │───────────────►│  PAUSED   │
  └────┬─────┘                 └────┬─────┘
       │                            │ resume()
       │                            └──────► RUNNING
       │
       ├── success() ──► COMPLETED
       ├── fail() ─────► FAILED ────► retry() ──► SCHEDULED
       ├── cancel() ───► CANCELLED
       └── timeout() ──► TIMEOUT ───► retry() ──► SCHEDULED
```

---

## 7. Variable Interpolation

Two forms, resolved at different times:

### Parse-Time (Static Structural References)
```
${models.primary}          → Resolved to model definition
${workflow.output_dir}     → Resolved to workflow variable
${workspace.codebase}      → Resolved to workspace path
```

### Runtime (Dynamic Values)
```
{{step.step_1.output}}     → Step output capture
{{now}}                    → Current timestamp
{{workflow_id}}            → Workflow identifier
{{run.number}}             → Execution run number
```

**Implementation**: Two-pass resolution. `${...}` resolved during YAML→WorkflowSpec. `{{...}}` resolved during WorkflowIR execution.

---

## 8. Directory Layout (.glyphnova/)

```
.glyphnova/
├── workflows/              # Workflow specs (versioned)
│   └── {workflow_id}/
│       ├── spec.yml        # Workflow definition
│       ├── ir.json         # Compiled WorkflowIR
│       └── policy.json     # Compiled policy snapshot
├── runs/                   # Execution artifacts
│   └── {run_id}/
│       ├── state.json      # Execution state
│       ├── logs/           # Step-level logs
│       ├── metrics/        # Performance metrics
│       └── outputs/        # Step outputs
├── checkpoints/            # State checkpoints
│   └── {checkpoint_id}.json
├── memory/                 # Local memory store
│   ├── structured/         # Key-value relationships
│   ├── unstructured/       # Document blobs
│   └── search-index/       # Full-text + semantic index
├── experiments/            # Git experiment tracking
│   └── {experiment_id}/
├── benchmarks/             # Benchmark results
│   └── {benchmark_id}.json
└── config/                 # Global configuration
    ├── settings.json
    └── policies.json
```

---

## 9. Key Architectural Constraints (from ADRs)

These are NON-NEGOTIABLE:

1. **Compiler-centered** (ADR-0001): YAML → WorkflowSpec → WorkflowIR pipeline, not direct execution
2. **Local-first** (ADR-0001): All defaults local, networking opt-in
3. **Typed representation** (ADR-0001): WorkflowSpec and WorkflowIR are strongly-typed Rust structs
4. **Dual execution modes** (ADR-0002): Direct (dev) and Code Generation (prod)
5. **ChatSession as work container** (ADR-0002): Every chat is a scoped executable
6. **Human gating** (ADR-0002): Risky operations require confirmation
7. **CLI-first** (ADR-0003): CLI is primary control surface
8. **Backend abstraction** (ADR-0003): Unified trait for all LLM providers
9. **UI as shell** (ADR-0004): Glyphnova is a projection of runtime, not separate
10. **Quality loops as semantics** (ADR-0005): Generate-verify-repair is runtime, not prompt
11. **Local memory first** (ADR-0006): Memory before external search
12. **Cron compiles to IR** (ADR-0007): Scheduling policies in WorkflowIR, not interpreted
13. **Bounded autonomy** (ADR-0008): Autonomous loops have goals, checkpoints, stop conditions

---

## 10. SDK vs Transpiler Coverage

### SDK Provides (~40%)
- Model management (multi-provider, parameter tuning, lifecycle)
- Basic agent orchestration (tools, sub-agent spawning)
- Retry logic (exponential/linear backoff)
- Async execution (Tokio runtime)
- Basic permissions (tool-level access control)

### Transpiler Must Build (~60%)
- YAML schema DSL parsing and compilation
- WorkflowIR intermediate representation
- Variable interpolation (`${...}` and `{{...}}`)
- Validation/convergence loops
- Multi-agent coordination (merge/vote/collect)
- Conditional branching with event-based routing
- State persistence (checkpoint save/restore, versioning)
- Workflow composition and nesting
- Hierarchical logging (9-level scope system)
- Comprehensive metrics collection
- Human gating and staged diffs
- Event-driven orchestration
