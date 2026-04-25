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

## 1. Server Health

```bash
./target/debug/whitt server
```

Expected:
```
Server: http://localhost:8080
Health: ok
Slots: idle=N, processing=N

Loaded models:
  - <model_name>
```

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
./target/debug/whitt chat "Say hi"
```

Expected: Text streams in real-time to terminal.

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

### Pipe input (acts as REPL)
```bash
echo "What is 3+3?" | ./target/debug/whitt chat --no-stream
```

Expected: Enters REPL, processes line, exits on EOF.

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

## 9. Shell Scripts (Deprecated)

These still work but are superseded by `whitt` CLI:

| Script | Replacement |
|--------|-------------|
| `scripts/benchmark.sh` | `whitt benchmark` |
| `scripts/benchmark-model-swap.sh` | `whitt benchmark` + `whitt model swap` |
| `scripts/download-model.sh` | `whitt download` |
| `scripts/switch-model.sh` | `whitt model swap` |
| `scripts/status.sh` | `whitt server` |
| `scripts/start.sh` | Docker management (retained) |
| `scripts/stop.sh` | Docker management (retained) |
| `scripts/detect-gpu.sh` | Docker management (retained) |
| `scripts/logs.sh` | Docker management (retained) |

---

## 10. YAML Configuration System

### 10.1 Config File Locations (Priority: highest → lowest)

| Priority | Path | Scope |
|----------|------|-------|
| 1 (highest) | CLI flags | Per-invocation |
| 2 | `configs/models/<model-name>.yml` | Per-model override |
| 3 | `~/.config/whitt/config.yml` | Machine-wide |
| 4 | `/config/config.yml` | Docker-mounted |
| 5 (lowest) | Rust struct defaults | Built-in |

### 10.2 Available YAML Properties

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

### 10.3 Config Merge Behavior

When multiple config sources exist, they merge field-by-field. The model section is entirely replaced by the highest-priority source. All other sections merge: fields present in higher-priority source override the same field from lower-priority source; fields NOT present are preserved from lower-priority source.

### 10.4 QA Steps

**QA-C1: Default config load**
```bash
RUST_LOG=info ./target/debug/whitt server
# Expected: shows loaded config or falls back to defaults
```

**QA-C2: Machine-wide config**
```bash
mkdir -p ~/.config/whitt
cat > ~/.config/whitt/config.yml << 'EOF'
model:
  path: /models/qwen2.5-0.5b-instruct-q4_k_m.gguf
sampling:
  temperature: 0.42
EOF
RUST_LOG=info ./target/debug/whitt server
# Expected: logs "Loaded machine-wide config", temperature=0.42
```

**QA-C3: Per-model override**
```bash
cat > configs/models/SmolLM3-Q4_K_M.yml << 'EOF'
model:
  path: /models/SmolLM3-Q4_K_M.gguf
sampling:
  temperature: 0.60
  max_tokens: 2048
EOF
./target/debug/whitt model swap SmolLM3-Q4_K_M
# Expected: loads with temperature=0.60, max_tokens=2048
```

**QA-C4: Graceful degradation**
```bash
rm -rf ~/.config/whitt/config.yml
rm -f configs/models/SmolLM3-Q4_K_M.yml
./target/debug/whitt server
# Expected: works fine with defaults
```

**QA-C5: Docker config reload**
```bash
# Edit config.yml, then restart container:
docker compose down && docker compose up -d
# Note: llama.cpp does NOT support hot-reload. Restart required.
```

---

## Known Limitations

- **Config Hot Reload**: llama.cpp upstream does not support live config changes. Container restart required.
- **Per-Model Configs**: `configs/models/*.yml` files exist as reference examples but are not yet wired into the Docker container's entrypoint. Only the `whitt` CLI uses `ConfigLoader` directly.
- **Config Validation**: No JSON Schema validation yet. Invalid YAML keys are silently ignored by serde defaults.
- **Agent ReAct**: 0.5B models can't reliably follow ReAct JSON format. Needs 7B+ for reliable agent output.
- **SmolLM3**: THINKING model consumes most token budget on reasoning. Increase `--max-tokens` and ensure slot context is large enough (see Slots below).
- **amdgpu-container-cli**: Not installed, needs root. Legacy `/dev/dri` passthrough works fine.
- **MODEL_HOST_PATH**: Set `MODEL_HOST_PATH=/data/models` in `.env` to mount models from external drive.
- **Slots**: Context is divided equally among slots. `ctx_size` / `max_slots` = tokens per slot. If generation gets cut off mid-sentence with `finish_reason=length`, the slot context is too small. Fix: increase `context.size` or reduce `max_slots` in config.yml, then restart the container.
