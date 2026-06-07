# Whitt Execution Engine

YAML-driven workflow orchestration with LLM agents, lifecycle hooks, and parallel execution.

## Quick Start

```bash
# Build
cargo build --release

# Start llama.cpp server with Vulkan
docker-compose -f docker/docker-compose.yml up -d

# Download a model
./target/release/whitt model download ministral-3b

# Chat interactively
./target/release/whitt chat --model ministral-3b

# Run a workflow
./target/release/whitt workflow --workflow workflows/example.yml
```

### Prerequisites

- Rust 1.70+ (edition 2021)
- Docker with GPU support (Vulkan compatible)
- Linux or macOS

## CLI Reference

| Command | Description |
|---------|-------------|
| `chat` | Interactive chat with a loaded model |
| `model list` | List available models |
| `model load <model>` | Load a model into memory |
| `model unload <model>` | Unload a model from memory |
| `model swap <from> <to>` | Swap between two models |
| `server start` | Start llama.cpp server |
| `server stop` | Stop llama.cpp server |
| `server status` | Check server status |
| `server gpu` | Display GPU information |
| `server logs` | View server logs |
| `agent` | Execute agent workflows |
| `benchmark` | Run model benchmarks |
| `workflow` | Execute YAML workflows |
| `download <model>` | Download model from HuggingFace |

## Architecture

```
YAML Workflow → Validation → Step Execution → Hooks → Output
                     ↓              ↓            ↓
                Schema Check   Parallel     Trigger/
                                            Action
                                            Dispatch
```

- **Schema-driven**: All workflows validated against `docs/schema/unified-workflow-schema.yml`
- **Parallel execution**: `tokio::spawn` with `Arc<Mutex<HookEngine>>` for concurrent steps
- **Lifecycle hooks**: 10 triggers × 12 actions with GWT expression evaluator
- **Backend**: llama.cpp with Vulkan backend in Docker (not LM Studio, not Ollama)

## Workflow Example

```yaml
workflow:
  name: simple-inference
  version: "2.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  - name: ministral-3b
    provider: llama_cpp_with_vulkan
    host:
      type: llama_cpp_with_vulkan

steps:
  - name: generate
    model: ministral-3b
    prompt: "Explain Rust's ownership model in 50 words."
    hooks:
      after_step_succeeds:
        - action: log
          level: info
          message: "Generated output: {{step.generate.output}}"
        - action: save_to
          path: outputs/{{step.generate.model_name}}/{{step.generate.name}}.json
          content:
            model: "{{step.generate.model_name}}"
            output: "{{step.generate.output}}"
            timestamp: "{{step.generate.timestamp}}"
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2021, MSRV 1.70+) |
| Runtime | Tokio async runtime |
| Backend | llama.cpp with Vulkan |
| Container | Docker + docker-compose |
| Serialization | serde + serde_yaml |
| CLI | clap |
| Testing | cargo test + property-based tests |

## Project Structure

```
src/
├── lib.rs              # Root module
├── error.rs            # Error types
├── config/             # YAML configuration (3 files)
├── model/              # Model specifications (4 files)
├── agent/              # Agent execution engine (6 files)
│   ├── tools.rs        # Tool registry, 6 tools
│   ├── executor.rs     # Step execution
│   ├── react.rs        # ReAct agent
│   ├── streaming.rs    # SSE streaming
│   ├── persistence.rs  # Workflow checkpointing
│   └── sandbox.rs      # Tool sandbox security
├── backend/            # LLM backends (3 files)
├── client/             # HTTP client (5 files)
└── bin/                # CLI binaries (3 files)

docs/
├── schema/             # Workflow schema source of truth
├── contributing/       # Developer guides
└── plans/              # Project plans
```

## Documentation

- [Schema Reference](docs/schema/unified-workflow-schema.yml) — Complete workflow schema definition
- [Contributing Guide](docs/contributing/README.md) — How to contribute
- [Developer Guide](docs/contributing/developer-guide.md) — Architecture and development patterns
- [Testing Guide](docs/contributing/testing-guide.md) — Test strategy and running tests
- [Hooks Reference](docs/contributing/hooks-reference.md) — Hook triggers and actions
- [Environment Variables](docs/contributing/environment-variables.md) — Configuration options
- [Installation](docs/contributing/install.md) — Build and setup instructions

## Contributing

See [docs/contributing/README.md](docs/contributing/README.md) for contribution guidelines.

## Project Status

### Implemented
- YAML workflow execution engine
- 10 lifecycle triggers, 12 action types
- GWT expression evaluator
- Parallel execution with tokio
- ReAct agent implementation
- Workflow checkpointing
- Tool sandbox security
- SSE streaming support
- Model lifecycle management
- Docker integration with llama.cpp + Vulkan
- 637 passing tests (0 failures)

### Planned
- Code generation / compilation mode
- Additional provider backends (OpenAI, Anthropic)
- Advanced scheduling and resource allocation
- Distributed workflow execution

## License

MIT License — see LICENSE file for details.