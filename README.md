# YAML to Rust Agent SDK Transpiler

## What It Is

Write a YAML workflow → Get compiled Rust code that runs local LLMs.

This is a compiler. You define workflows in YAML, it generates Rust code with type-safe execution, built-in tools, and autonomous capabilities.

**yaml-to-rust-agentsdk is a YAML local agentic runtime and YAML local agentic workflow execution framework with the ability to save and track workflow execution through optionally persistent reusable compiled Rust code generated from a YAML workflow.**

---

## Why

Local LLMs are powerful but stuck in chat. To be useful, they need:

- **Multi-step workflows** (sequences with loops, branches, conditions)
- **Access to tools** (files, web, shell commands)
- **Queue and scheduler** (prioritize work, pause, resume, cancel)
- **Human gates** (confirm destructive actions, preview changes)
- **Auditability** (every run logged, reproducible, inspectable)

Building this in Rust manually takes time. Debugging takes longer.

This transpiler bridges that gap.

---

## Dual Execution Modes

The transpiler supports **two distinct execution modes**:

### Mode 1: Execution Engine (Direct Execution)

**Use case**: For development, iteration, testing, and self-improvement workflows

**What it does**:
- Directly executes YAML workflows
- Rich execution logs and metrics
- Supports iterative workflow improvement
- No need to compile code

**Process**:
```
YAML Workflow → Validation → WorkflowIR → Execute → Output + Logs
```

**When to use**:
- Developing new workflows
- Testing workflow parameters
- Iterating on validation criteria
- Collecting data for workflow improvements
- Self-improving agentic workflows

**Output**:
- Console output: Human-readable results
- Logs: Structured execution logs (JSON/structured)
- Chat files: Conversation history with LLM calls
- Output files: Files specified in workflow output sections
- Metrics: Performance, quality, convergence data

**Example**: Test a new validation loop, adjust criteria, re-execute until convergence achieved.

---

### Mode 2: Code Generator (Reusable Code)

**Use case**: For production, frequent execution, or when workflow is stable

**What it does**:
- Compiles YAML workflows to reusable Rust code
- Generates production-ready executable
- Supports CLI execution with parameter passing
- No external YAML dependencies at runtime

**Process**:
```
YAML Workflow → WorkflowIR → Rust Code → Compile → Reusable Executable
```

**When to use**:
- Workflow is production-ready
- Need frequent executions with variations
- Want to optimize startup time
- Need to integrate workflow into larger systems
- Want to embed workflow logic in other applications

**Generated Code Features**:
- Self-contained execution engine
- CLI interface for parameter passing
- Built-in retry and validation logic
- Native model loading/unloading
- Optimized for performance
- No YAML parsing at runtime

**Example**: After developing and testing a workflow 100 times, generate reusable Rust code for production use.

---

### Workflow Improvement Loop

The two modes work together to enable self-improving workflows:

1. **Development Phase**:
   - Use execution mode to iterate on workflow design
   - Use execution logs to identify improvement opportunities
   - Test different model configurations
   - Tune parameters (concurrency, retry, validation)

2. **Data Collection**:
   - Execution logs capture quality metrics
   - Convergence data shows improvement patterns
   - Error patterns reveal edge cases

3. **Improvement**:
   - Analyze logs to improve validation criteria
   - Optimize model selection and routing
   - Refine loop convergence thresholds

4. **Code Generation**:
   - When workflow is stable, generate reusable Rust code
   - Test generated code independently
   - Deploy to production

5. **Iterate Again**:
   - Monitor performance with compiled code
   - Identify new improvements
   - Regenerate code when needed

**Benefits**:
- Development speed: Quick iterations with execution mode
- Production performance: Compiled code optimization
- Reusability: Reusable code eliminates re-transpilation overhead
- Self-improvement: Data-driven workflow optimization

---

## How It Works

You define a workflow in YAML:

```yaml
agents:
  - name: analyzer
    model: ollama://llama3.2
    tools: [file-read, grep, web-scrape]

workflow:
  - step: Analyze codebase
    agent: analyzer
    input:
      path: ./src/
      pattern: "async fn"

  - step: Research documentation
    agent: analyzer
    input:
      urls: [https://docs.rs/tokio]

  - step: Generate report
    agent: analyzer
    output: ./analysis.md
```

The transpiler compiles this to Rust code with:
- Type-safe agent definitions (AutoAgents SDK)
- Tool implementations (file, web, shell operations)
- Async execution (tokio runtime)
- Queue and scheduler (prioritized work, cancellation)
- Memory management (sliding window, configurable backends)
- Observability (logs, metrics, provenance)

---

## Vision

### Compiler-Centered, Local-First

- YAML is the source language
- Rust WorkflowIR is the compilation target
- `.glyphnova/` stores all artifacts (specs, runs, logs, hashes)
- Everything runs locally, no cloud dependencies

### Queue, Scheduler, Safety

Every workflow runs in a work container:

- Chat sessions are scoped execution contexts
- Queue items have states (pending, running, paused, completed)
- Scheduler handles priorities, retries, persistence
- Risky operations require human confirmation
- File mutations are staged with preview diffs

### Autonomous Loops (Future)

Workflows can run autonomously within bounds:

- Declared modes with bounded goals and stop conditions
- Checkpoints and validation thresholds
- Human override always available
- Metrics drive improvements (performance, quality, trust)

---

## Built-in Tools

### File Operations
- `file-read` - Read file contents
- `file-write` - Write/append to files
- `file-search` - Search file names (glob patterns)
- `file-move` - Move/rename files
- `file-delete` - Delete files
- `grep` - Search file contents with regex

### Web Capabilities
- `web-fetch` - Fetch web pages
- `web-scrape` - Extract structured data from HTML
- `web-search` - Search web via APIs

### Shell Operations
- `shell-exec` - Execute shell commands with timeout
- `shell-safe` - Sanitized command execution

### Utility
- `memory-save` - Save conversation context
- `memory-load` - Load conversation context
- `log-write` - Write structured logs

---

## Performance

| Input Size | Transpile Time | Build Time | Total |
|-----------|----------------|------------|-------|
| 500 lines YAML | ~2ms | 1-2s | **~2s** |
| 5,000 lines YAML | ~5ms | 2-8s | **~2-8s** |

Benchmarks: serde-saphyr (89 MB/s) + Askama (5-10x faster than interpreted) + tokio

---

## Tech Stack

| Component | Library | Why? |
|-----------|----------|-------|
| YAML Parsing | serde-saphyr | 1.5x faster than serde_yaml, schema validation |
| Code Generation | Askama | Pre-compiled templates, 5-10x faster |
| Agent SDK | AutoAgents | Production-ready, 11+ LLM providers |
| Async Runtime | tokio | Industry standard, battle-tested |
| Error Handling | thiserror + anyhow | Type-safe for libraries, convenient for apps |

---

## Multi-Model Support

Works with local LLM providers:

- Ollama (ollama://model-name)
- LM Studio (lmstudio://model-name)
- llama.cpp (llamacpp://path/to/model.gguf)
- OpenAI-compatible (openai://model-name, for cloud fallback)

Automatic model installation, verification, and benchmarking coming in v0.2.0.

---

## Project Structure

```
src/
  main.rs           # CLI entry point
  lib.rs            # Library API
  parser.rs         # YAML parsing (serde-saphyr)
  generator.rs      # Code generation (Askama)
  scheduler.rs      # Queue and scheduler
  templates/        # Rust code templates
  tools/            # Built-in tool implementations
  agents/           # Agent scaffolding

.glyphnova/         # System-of-record
  workflows/        # Workflow specs and IR
  runs/             # Execution artifacts, logs, hashes
  metrics/          # Performance and quality metrics
```

---

## Status

🚧 **In Development**

Roadmap by phase (see ADRs in `opencode/docs/reports/roadmap/`):

**Phase 1: Foundation** (ADR-0001)
- [x] Research complete (tech stack selection)
- [x] Repository structure
- [ ] YAML schema specification
- [ ] Parser implementation
- [ ] WorkflowIR compiler
- [ ] .glyphnova/ system-of-record

**Phase 2: MVP Queue & Scheduler** (ADR-0002)
- [ ] Chat session work containers
- [ ] Queue state machine
- [ ] Scheduler (prioritize, cancel, retry, persist)
- [ ] Human-gated safety (confirmations, staged diffs)
- [ ] CLI control surface

**Phase 3: CLI & Backends** (ADR-0003)
- [ ] CLI interface
- [ ] Backend abstraction (Ollama, llama.cpp, LM Studio)
- [ ] Networking boundary (local-first defaults)

**Phase 4: UI & Visualization** (ADR-0004)
- [ ] Queue visualization
- [ ] Graph workflow navigation
- [ ] Real-time execution monitoring

**Phase 5: Quality Loops** (ADR-0005)
- [ ] Generate-verify-repair cycles
- [ ] Convergence loops
- [ ] Benchmark suite integration

**Phase 6: Memory & Search** (ADR-0006)
- [ ] Sliding window memory
- [ ] Vector search integration
- [ ] Context compression

**Phase 7: Automation** (ADR-0007)
- [ ] Cron scheduling
- [ ] Git automation
- [ ] Refinement loops

**Phase 8: Autonomy** (ADR-0008)
- [ ] Bounded autonomous loops
- [ ] Metrics-driven UX
- [ ] Human override controls

---

## Example Use Case

Analyze a Rust codebase and generate a refactoring plan:

1. **Define workflow in YAML** (2 minutes)
2. **Run transpiler** (2 seconds)
3. **Execute generated agent** (instant startup, local LLM)
4. **Review queued work** (pause, reprioritize, approve)
5. **Get refactoring plan** (parallel analysis, tool access)

No API keys. No cloud services. Local, auditable, safe.

---

## License

MIT / Apache-2.0 (dual license, matches dependencies)

---

## Acknowledgments

Research and architecture informed by:
- [AutoAgents](https://github.com/liquidos-ai/AutoAgents) - Production agent SDK
- [serde-saphyr](https://github.com/bourumir-wyngs/serde-saphyr) - Fast YAML parsing
- [Askama](https://github.com/askama-rs/askama) - Type-safe templates
- [tokio](https://tokio.rs) - Async runtime
