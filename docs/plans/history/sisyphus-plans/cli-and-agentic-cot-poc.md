# POC Plan: CLI Tool + Agentic Chain-of-Thought

## Objective

Build a unified `whitt` CLI binary that provides:
1. **Chat interface** — one-shot queries + interactive REPL, model load/unload, streaming output
2. **Model management** — list, load, unload, swap models via CLI (pure HTTP to Docker container)
3. **Agentic chain-of-thought** — ReAct loop that reasons through tasks step-by-step using local LLMs

All POC scope. Must be easy for anyone to use for basic chat and trying different models.

---

## Research Findings

### Reference Projects

| Project | Stars | Key Pattern | URL |
|---------|-------|-------------|-----|
| sigoden/aichat | 9,884 | Full-featured LLM CLI, REPL+CMD modes, session management | github.com/sigoden/aichat |
| 0xPlaygrounds/rig | 6,736 | Rust agent framework, ReAct, tool dispatch, streaming | github.com/0xPlaygrounds/rig |
| graniet/llm | 337 | Multi-provider CLI, builder pattern, streaming | github.com/graniet/llm |
| jandrus/rtwo | — | Simple Ollama CLI, SQLite history, config hierarchy | github.com/jandrus/rtwo |

### Key Design Decisions

1. **Subcommands > flat flags** — CLI has distinct modes (chat vs model vs agent), needs clear separation
2. **Reuse existing `LlamaHttpClient`** — already has health, chat, streaming, model load/unload
3. **Reuse existing `DockerManager`** — already has start/stop/restart
4. **Streaming via existing SSE parser** — `chat_completion_stream()` returns `Stream<Item=ChatCompletionChunk>>`
5. **REPL via rustyline** — async-compatible, history, Ctrl-C handling
6. **No external agent framework** — build ReAct loop from scratch (small, self-contained)
7. **Small model CoT** — explicit format instructions in system prompt, validation + retry pipeline
8. **SmolLM3 for agent** — 3B params, toggleable reasoning mode, best small model for CoT

### Structured Output from Small Models

Prompting alone unreliable for <7B models. Use:
- **Explicit format instructions**: "ONLY return JSON in this exact schema"
- **XML-style tags**: `<thought>`, `<action>`, `<result>` (works better than raw JSON for small models)
- **Validation + retry pipeline**: parse → validate → feed errors back → retry (max 3)
- **No constrained decoding** — llama.cpp grammar constraints require container rebuild; skip for POC

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    whitt CLI (clap)                         │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐  │
│  │  chat    │  │  model   │  │  server  │  │   agent    │  │
│  │ (REPL/   │  │ (load/   │  │ (health/ │  │  (ReAct    │  │
│  │   CMD)   │  │  unload) │  │  status) │  │   CoT)    │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └─────┬──────┘  │
│       └──────────────┴──────────────┴──────────────┘        │
│                           │                                  │
│                    ┌──────▼──────┐                           │
│                    │  LlamaHttp  │                           │
│                    │   Client    │                           │
│                    └──────┬──────┘                           │
│                           │                                  │
│                    Docker :8080                               │
└─────────────────────────────────────────────────────────────┘
```

---

## CLI Command Design

```
whitt [OPTIONS] [COMMAND]

Commands:
  chat [PROMPT]     Chat with a model (REPL if no prompt, one-shot if prompt given)
  model             Manage models (list, load, unload, swap)
  server            Server health and status
  agent <TASK>      Run agentic chain-of-thought on a task

Options:
  -u, --url <URL>         Server URL [default: http://localhost:8080]
  -m, --model <MODEL>     Model to use [default: Qwen2.5-0.5B-Instruct-Q4_K_M]
  -v, --verbose           Enable debug logging
  -h, --help              Print help
  -V, --version           Print version

Chat flags (when using `chat` command):
  -s, --system <PROMPT>   System prompt
  -t, --temperature <F>   Temperature [default: 0.7]
  -n, --max-tokens <N>    Max tokens [default: 512]
      --no-stream         Disable streaming output
      --save <FILE>       Save conversation to file

Model subcommands:
  whitt model list              List available models and their status
  whitt model load <MODEL>      Load a model by ID
  whitt model unload <MODEL>    Unload a model by ID
  whitt model swap <MODEL>      Unload current, load target

Agent flags:
  whitt agent -m SmolLM3-Q4_K_M "Write a Python function that reverses a string"
      --max-steps <N>      Maximum reasoning steps [default: 10]
      --model <MODEL>       Model for reasoning [default: SmolLM3-Q4_K_M]
      --verbose             Show thought process
```

### UX Flow

```bash
# Quick one-shot (loads model, answers, unloads)
whitt chat "What is 2+2?"

# Interactive REPL (loads model, loops until /exit)
whitt chat
> Hello!
<response streams in>
> /model SmolLM3-Q4_K_M    # switch model mid-session
> /system You are a pirate  # change system prompt
> /exit

# One-shot with non-default model
whitt chat -m SmolLM3-Q4_K_M "Explain Rust ownership"

# Non-streaming (wait for full response)
whitt chat --no-stream "Count to 10"

# Pipe input
echo "Summarize this file" | whitt chat --save summary.txt

# Model management
whitt model list
whitt model load SmolLM3-Q4_K_M
whitt model unload Qwen2.5-0.5B-Instruct-Q4_K_M

# Agentic CoT
whitt agent "Analyze the project structure and list all test files"
whitt agent --verbose "Plan how to add a new model"
```

---

## Implementation Plan

### Phase 1: CLI Scaffolding

**File: `src/bin/whitt.rs`** (NEW)

Clap derive with subcommands:

```rust
#[derive(Parser)]
#[command(name = "whitt", version, about = "CLI for Whitt Execution Engine")]
struct Cli {
    #[arg(short, long, global = true, default_value = "http://localhost:8080")]
    url: String,

    #[arg(short, long, global = true)]
    model: Option<String>,

    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Chat {
        prompt: Option<String>,
        #[arg(short, long)]
        system: Option<String>,
        #[arg(short, long, default_value_t = 0.7)]
        temperature: f32,
        #[arg(short = 'n', long, default_value_t = 512)]
        max_tokens: usize,
        #[arg(long)]
        no_stream: bool,
        #[arg(long)]
        save: Option<PathBuf>,
    },
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },
    Server,
    Agent {
        task: String,
        #[arg(short, long)]
        model: Option<String>,
        #[arg(long, default_value_t = 10)]
        max_steps: usize,
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum ModelAction {
    List,
    Load { model: String },
    Unload { model: String },
    Swap { model: String },
}
```

### Phase 2: Chat Command

**File: `src/bin/whitt.rs`** (chat module)

#### One-shot mode (prompt provided):
1. Initialize `LlamaHttpClient`
2. Auto-load model if not loaded (via `list_models` → `load_model`)
3. Build `ChatCompletionRequest` with messages
4. If `--no-stream`: call `chat_completion()`, print response
5. If streaming (default): call `chat_completion_stream()`, print tokens to stdout with flush
6. Print token usage to stderr
7. If `--save`: write messages to file (JSON)

#### Interactive REPL mode (no prompt):
1. Load model
2. Enter REPL loop with `rustyline`:
   - Read input line
   - Handle commands: `/exit`, `/model <name>`, `/system <prompt>`, `/help`
   - Send as chat message, stream response
   - Maintain conversation history (append to messages vec)
3. Exit on EOF or `/exit`

**Dependencies to add:**
```toml
rustyline = "15"
```

### Phase 3: Model Command

**File: `src/bin/whitt.rs`** (model module)

Reuse existing `LlamaHttpClient` methods directly:

```rust
async fn handle_model_list(client: &LlamaHttpClient) -> Result<()> {
    let models = client.list_models().await?;
    // Format as table: ID | Status | Size
}

async fn handle_model_load(client: &LlamaHttpClient, model_id: &str) -> Result<()> {
    client.load_model(model_id).await?;
}

async fn handle_model_unload(client: &LlamaHttpClient, model_id: &str) -> Result<()> {
    client.unload_model(model_id).await?;
}

async fn handle_model_swap(client: &LlamaHttpClient, model_id: &str) -> Result<()> {
    // Find currently loaded model
    let models = client.list_models().await?;
    let loaded = models.iter().find(|m| m.status.value == "loaded");
    if let Some(m) = loaded {
        client.unload_model(&m.id).await?;
    }
    client.load_model(model_id).await?;
}
```

### Phase 4: Server Command

Simple health + status display:

```rust
async fn handle_server(client: &LlamaHttpClient) -> Result<()> {
    let health = client.health().await?;
    let models = client.list_models().await?;
    // Print: server status, loaded model, available models
}
```

### Phase 5: Agentic Chain-of-Thought

**File: `src/bin/whitt.rs`** (agent module)

#### ReAct Loop Architecture

```
User Task → [System Prompt with Tools] → Model
                                      ↓
                              ┌── Thought ──┐
                              │  (reasoning) │
                              └──────┬───────┘
                                     ↓
                              ┌── Action ────┐
                              │ (tool call)  │
                              └──────┬───────┘
                                     ↓
                              ┌── Result ────┐
                              │ (tool output) │
                              └──────┬───────┘
                                     ↓
                              Loop or FINAL_ANSWER
```

#### Tool System

```rust
#[async_trait]
trait AgentTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: serde_json::Value) -> Result<String>;
}

struct ModelLoadTool { client: Arc<LlamaHttpClient> }
struct ModelUnloadTool { client: Arc<LlamaHttpClient> }
struct ModelListTool { client: Arc<LlamaHttpClient> }
struct ChatTool { client: Arc<LlamaHttpClient> }
struct FileReadTool;
struct ShellExecuteTool;
struct FinalAnswerTool;
```

#### System Prompt for Small Model CoT

```
You are a task execution agent. Break tasks into steps using the Think-Act-Observe loop.

For each step, output EXACTLY in this format:

<thinkYour reasoning about what to do next</think<acttool_name:json_arguments</act<resulttool output here</result>

Available tools:
- model_list: List available models (no args)
- model_load: Load a model {"model": "model_id"}
- model_unload: Unload a model {"model": "model_id"}
- chat: Send a chat message to the loaded model {"message": "your prompt"}
- file_read: Read a file {"path": "/path/to/file"}
- final_answer: Provide the final answer {"answer": "your complete answer"}

Rules:
- Always use <think<act<result> format
- Each step has exactly ONE tool call
- When done, use final_answer tool
- Do NOT add text outside the tags
```

#### Parse Pipeline

```rust
fn parse_agent_response(raw: &str) -> Result<AgentStep> {
    // 1. Extract <think...</think content
    // 2. Extract <act>tool_name:json_args</act
    // 3. Execute tool
    // 4. Return AgentStep { thought, action, result }
}
```

#### Validation + Retry

```rust
async fn run_agent_loop(
    client: &LlamaHttpClient,
    task: &str,
    model_id: &str,
    max_steps: usize,
    verbose: bool,
) -> Result<String> {
    let mut messages = vec![
        ChatMessage::system(SYSTEM_PROMPT),
        ChatMessage::user(task),
    ];

    for step in 0..max_steps {
        let response = client.chat_completion(ChatCompletionRequest {
            model: model_id.into(),
            messages: messages.clone(),
            ..Default::default()
        }).await?;

        let agent_step = parse_agent_response(&response.choices[0].message.content)?;

        if verbose {
            eprintln!("[Step {}] Think: {}", step + 1, agent_step.thought);
            eprintln!("[Step {}] Act: {}({})", step + 1, agent_step.action, agent_step.args);
        }

        if agent_step.action == "final_answer" {
            return Ok(agent_step.args["answer"].as_str().unwrap_or("").into());
        }

        // Execute tool
        let result = execute_tool(&client, &agent_step.action, &agent_step.args).await?;
        if verbose {
            eprintln!("[Step {}] Result: {}", step + 1, result);
        }

        // Feed result back
        messages.push(ChatMessage::assistant(format!(
            "<think{}<act{}:{}</act<result{}</result>",
            agent_step.thought, agent_step.action, agent_step.args, result
        )));
        messages.push(ChatMessage::user(result));
    }

    anyhow::bail!("Agent exceeded max steps ({})", max_steps);
}
```

---

## Files to Create/Modify

| File | Change | Notes |
|------|--------|-------|
| `src/bin/whitt.rs` | NEW | Main CLI binary (~400-500 lines) |
| `Cargo.toml` | MODIFY | Add `whitt` binary target, add `rustyline` dep |
| `tests/cli_test.rs` | NEW | Skipped test descriptions for CLI commands |

---

## Dependencies

### To Add
```toml
rustyline = "15"  # Interactive REPL with history
```

### Already Available (no new deps needed)
- `clap` 4.5 (derive) — CLI parsing
- `tokio` 1.40 (full) — async runtime
- `reqwest` 0.13 (stream) — HTTP client
- `anyhow` — error handling
- `tracing` + `tracing-subscriber` — logging
- `serde` + `serde_json` — serialization
- `futures` — StreamExt for streaming

---

## Validation Criteria

| Criterion | How to Verify |
|-----------|--------------|
| `whitt chat "hello"` works | Streams response from default model |
| `whitt chat` (REPL) works | Interactive loop, `/exit` exits cleanly |
| `whitt model list` works | Shows all 4 models with status |
| `whitt model load X` works | Model status changes to loaded |
| `whitt model unload X` works | Model status changes to unloaded |
| `whitt model swap X` works | Unloads current, loads target |
| `whitt server` works | Shows health + loaded model |
| `whitt agent "task"` works | ReAct loop runs, produces answer |
| `whitt --help` works | Shows all commands and flags |
| Streaming visible | Tokens appear incrementally, not batched |
| Error handling | Bad model name, server down → clear error message |
| /model in REPL | Switches model mid-conversation |
| --save flag | Conversation written to JSON file |

---

## Implementation Order

1. **Phase 1**: CLI scaffolding (clap subcommands, argument parsing) — commit
2. **Phase 2**: Chat command (one-shot + streaming) — commit
3. **Phase 3**: Chat REPL (rustyline, /commands) — commit
4. **Phase 4**: Model + Server commands — commit
5. **Phase 5**: Agentic CoT (ReAct loop, tool system, parsing) — commit
6. **Phase 6**: Skipped tests, final polish — commit
7. Each commit pushed to origin/poc

---

## Key Decisions

1. **Single binary** (`whitt`) not multiple — simpler to install, one entry point
2. **REPL in same binary** — no separate `whitt-repl`, just `whitt chat` without prompt
3. **No new client modules** — reuse existing `LlamaHttpClient`, `DockerManager` directly
4. **Agent built from scratch** — no rig/autoagents dependency for agent loop (keep it POC-simple)
5. **XML tags for agent** — `<think<act<result` works better than JSON for small models
6. **No conversation persistence** — in-memory only for POC (no SQLite)
7. **No markdown rendering** — raw text output for POC (add mdansi later if needed)
