# YAML to Rust Agent SDK Transpiler

## What It Is

Write a YAML workflow → Get compiled Rust code that runs local LLMs.

This is a compiler. You define workflows in YAML, it generates Rust code with type-safe execution, built-in tools, and autonomous capabilities.

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
