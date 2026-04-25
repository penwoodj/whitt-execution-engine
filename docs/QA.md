# POC QA Instructions

## Prerequisites

Docker container running:
```bash
docker ps --filter name=llama
# Expected: whitt-llama-server Up X hours (healthy)
```

Build all binaries:
```bash
cargo build --bin whitt --bin model_chain --bin poc_client --features client
```

Run unit tests:
```bash
cargo test --features client
# Expected: test result: ok. 5 passed; 0 failed; 15 ignored
# (15 integration tests need live server; ignored when no server detected by test harness)
```

---

## 1. Server Management

### Server status
```bash
./target/debug/whitt server status
```

Expected:
```
Server: http://localhost:8080
Health: ok
Slots: idle=N, processing=N

Loaded models:
  - <model_name>
```

### Start server
```bash
./target/debug/whitt server start
```

Expected: Auto-detects GPU (nvidia/amd/cpu), runs appropriate docker compose command.

### Stop server
```bash
./target/debug/whitt server stop
```

Expected: Runs `docker compose down`, prints "Server stopped".

### GPU detection
```bash
./target/debug/whitt server gpu
```

Expected: Prints GPU type (nvidia/amd/cpu) and recommended docker compose command.

### Server logs
```bash
./target/debug/whitt server logs
```

Expected: Follows docker compose logs (Ctrl+C to exit).

---

## 2. Model Management

### List all models
```bash
./target/debug/whitt model list
```

Expected: Table with 4 models (Qwen2.5-0.5B-Instruct-Q4_K_M, SmolLM3-Q4_K_M, qwen2.5-0.5b-instruct-q4_k_m, tinyllama-1.1b-chat-v1.0.Q4_K_M) showing loaded/unloaded status.

### Swap to Qwen (recommended for chat tests)
```bash
./target/debug/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
```

Expected: Unloads current model, loads Qwen, prints "Swapped to model 'Qwen2.5-0.5B-Instruct-Q4_K_M'".
Logs "Loaded per-model config override" when per-model config exists.

### Explicit load/unload
```bash
./target/debug/whitt model unload Qwen2.5-0.5B-Instruct-Q4_K_M
./target/debug/whitt model load Qwen2.5-0.5B-Instruct-Q4_K_M
```

Expected: Each prints success message with model name.

---

## 3. Chat

### Non-streaming (one-shot)
```bash
./target/debug/whitt chat "What is 2+2? Answer with just the number." --no-stream
```

Expected: Single line answer ("4"), token usage printed.

### Streaming (one-shot)
```bash
```bash
./target/debug/whitt chat "Say hi"
```

Expected: Text streams in real-time to terminal. Logs "Loaded per-model config override" for model's YAML config.

### All sampling flags
```bash
./target/debug/whitt chat "What is 2+2?" --no-stream \
  --temperature 0.7 --max-tokens 100 \
  --top-p 0.9 --top-k 50 \
  --repeat-penalty 1.1 \
  --presence-penalty 0.5 --frequency-penalty 0.5 \
  --stop "hello" "world" \
  --seed 42
```

Expected: All flags accepted, response generated with specified parameters.

### Config-driven defaults
```bash
# Per-model config at configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml sets temperature=0.7, max_tokens=1024
./target/debug/whitt chat "Hello" --verbose
# Expected: Uses config defaults. CLI flags override config values.
```

### Save conversation to file
```bash
./target/debug/whitt chat "What color is the sky?" --no-stream --save /tmp/whitt-qa.json
cat /tmp/whitt-qa.json
```

Expected: JSON array with user/assistant message objects.

### REPL (interactive)
```bash
./target/debug/whitt chat
```

Expected: `whitt> ` prompt. Type messages, `/exit` to quit.
REPL commands: `/exit`, `/model <name>`, `/system <prompt>`, `/clear`, `/help`.

---

## 4. Benchmark

```bash
./target/debug/whitt benchmark --max-tokens 50
```

Expected:
```
Model: <name>
Server: http://localhost:8080
Prompt: The quick brown fox jumps over the lazy dog.
Max tokens: 50
Concurrent: 1

--- Single Request ---
Elapsed time: <N>ms
Total tokens: <N>
Tokens per second: <N>.<NN>
Response: <truncated output>...

--- Benchmark Complete ---
```

Typical TPS on Qwen2.5-0.5B: 50-70.

---

## 5. Download

```bash
./target/debug/whitt download "Qwen/Qwen2.5-0.5B-Instruct-GGUF" --file "qwen2.5-0.5b-instruct-q4_k_m.gguf" --output /tmp/whitt-download
```

Expected: Progress bars, then "Downloaded <N> bytes (<N>.<N> MB)".

If file exists at destination: Error message "File already exists: <path>".

---

## 6. Agent (ReAct Loop)

```bash
./target/debug/whitt agent "List all models and tell me which one is loaded" -v
```

Expected: Step-by-step output showing thinking, tool calls (model_list), and final answer.

⚠️ DEPRECATED for POC. 0.5B models cannot reliably follow ReAct JSON format. This is a known limitation, not a bug. Needs 7B+ model for reliable agent output. Tracked for future scope.

Agent available tools: model_list, model_load, model_unload, chat, file_read, final_answer.

---

## 7. Model Chain Binary

```bash
./target/debug/model_chain
```

Expected: 3-step chain execution:
1. Planner (Qwen2.5-0.5B) — generates plan
2. Implementer (SmolLM3) — executes plan
3. Summarizer (TinyLlama) — summarizes result

Each step: load model → chat → unload model. Full logs printed.

---

## 8. SmolLM3 Reasoning (THINKING Model)

```bash
./target/debug/whitt model swap SmolLM3-Q4_K_M
./target/debug/whitt chat "Say hello in one word" --no-stream
```

Expected: Reasoning tokens are buffered and displayed as a single `[thinking]` block (dim text) before the response. No more per-word `[think]` prefix.
SmolLM3 is a THINKING model — output goes to `reasoning_content` field. With low max_tokens (default 512), most tokens consumed by reasoning. Not a bug.

---

## 9. YAML Configuration System

### 9.1 Config File Locations (Priority: highest → lowest)

| Priority | Path | Scope |
|----------|------|-------|
| 1 (highest) | CLI flags (--temperature, --top-p, etc.) | Per-invocation |
| 2 | `configs/models/<model-name>.yml` | Per-model override |
| 3 | `~/.config/whitt/config.yml` | Machine-wide |
| 4 | `/config/config.yml` | Docker-mounted |
| 5 (lowest) | Rust struct defaults | Built-in |

### 9.2 Per-Model Configs

Each loaded model has a config file in `configs/models/`:

| Model | Config | Key Overrides |
|-------|--------|---------------|
| Qwen2.5-0.5B-Instruct-Q4_K_M | `Qwen2.5-0.5B-Instruct-Q4_K_M.yml` | temp=0.7, max_tokens=1024, ctx=8192 |
| SmolLM3-Q4_K_M | `SmolLM3-Q4_K_M.yml` | temp=0.6, max_tokens=2048, ctx=4096 |
| qwen2.5-0.5b-instruct-q4_k_m | `qwen2.5-0.5b-instruct-q4_k_m.yml` | temp=0.7, max_tokens=512, ctx=4096 |
| tinyllama-1.1b-chat-v1.0.Q4_K_M | `tinyllama-1.1b-chat-v1.0.Q4_K_M.yml` | temp=0.7, max_tokens=512, ctx=4096 |

### 9.3 Available YAML Properties

All map to llama.cpp CLI flags via `LLAMA_ARG_*` env vars. Key sections:

- **model**: path, huggingface.repo, huggingface.filename, quantization
- **context**: size (default 2048), batch_size (2048), ubatch_size (512)
- **hardware**: threads (4), gpu_layers (999), use_mmap (true)
- **sampling**: temperature (0.80), top_p (0.95), top_k (40), repeat_penalty (1.00), max_tokens (512)
- **server**: host (127.0.0.1), port (8080), parallel (false), max_slots (8)
- **cache**: cache_type_k (f16), cache_type_v (f16)
- **vulkan**: visible_devices ("0"), flash_attention (None)
- **features**: log_level (info), verbose (false)

Full schema: see `src/config/mod.rs` struct definitions.

### 9.4 Available CLI Sampling Flags

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--temperature` | f32 | 0.7 (or config) | Sampling temperature |
| `--max-tokens` | usize | 512 (or config) | Max tokens to generate |
| `--top-p` | f32 | config | Top-p (nucleus) sampling |
| `--top-k` | usize | config | Top-k sampling |
| `--repeat-penalty` | f32 | config | Repeat penalty (1.0 = disabled) |
| `--presence-penalty` | f32 | config | Presence penalty |
| `--frequency-penalty` | f32 | config | Frequency penalty |
| `--stop` | Vec<String> | none | Stop sequences |
| `--seed` | u32 | config | Random seed (0 = random) |
| `--no-stream` | flag | false | Disable streaming |
| `--save` | path | none | Save conversation to JSON |

All sampling flags override per-model and machine-wide config values.

### 9.5 Config Merge Behavior

When multiple config sources exist, they merge field-by-field. The model section is entirely replaced by the highest-priority source. All other sections merge: fields present in higher-priority source override the same field from lower-priority source; fields NOT present are preserved from lower-priority source.

### 9.6 QA Steps

**QA-C1: Per-model config auto-load**
```bash
./target/debug/whitt chat "Hello" --verbose
# Expected: logs "Loaded per-model config override" with model name and config path
```

**QA-C2: CLI flag overrides config**
```bash
./target/debug/whitt chat "Hello" --no-stream --temperature 0.42
# Expected: Uses 0.42 temperature regardless of per-model config
```

**QA-C3: Machine-wide config**
```bash
mkdir -p ~/.config/whitt
cat > ~/.config/whitt/config.yml << 'EOF'
sampling:
  temperature: 0.42
EOF
./target/debug/whitt chat "Hello" --verbose
# Expected: logs "Loaded machine-wide config"
```

**QA-C4: Graceful degradation**
```bash
rm -rf ~/.config/whitt/config.yml
./target/debug/whitt chat "Hello"
# Expected: works fine with defaults
```

---

## Known Limitations

- **Config Hot Reload**: llama.cpp upstream does not support live config changes. Container restart required.
- **Per-Model Configs**: `configs/models/*.yml` files are wired into the `whitt` CLI via ConfigLoader. They are NOT wired into the Docker container's entrypoint (which uses `/config/config.yml` only).
- **Config Validation**: No JSON Schema validation yet. Invalid YAML keys are silently ignored by serde defaults.
- **Agent ReAct**: 0.5B models can't reliably follow ReAct JSON format. Needs 7B+ for reliable agent output.
- **SmolLM3**: THINKING model consumes most token budget on reasoning. Increase `--max-tokens` and ensure slot context is large enough (see Slots below).
- **amdgpu-container-cli**: Not installed, needs root. Legacy `/dev/dri` passthrough works fine.
- **MODEL_HOST_PATH**: Set `MODEL_HOST_PATH=/data/models` in `.env` to mount models from external drive.
- **Slots**: Context is divided equally among slots. `ctx_size` / `max_slots` = tokens per slot. If generation gets cut off mid-sentence with `finish_reason=length`, the slot context is too small. Fix: increase `context.size` or reduce `max_slots` in config.yml, then restart the container.
- **Shell Scripts**: All shell scripts removed. Use `whitt server start/stop/gpu/logs/status` instead.
