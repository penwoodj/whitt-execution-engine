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
# Expected: test result: ok. 2 passed; 0 failed; 14 ignored
# (14 integration tests need live server; ignored when no server detected by test harness)
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
Note: 0.5B models struggle with ReAct format. Works mechanically but may produce malformed tool calls.

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

Expected: May show `[reasoning]` prefix if model uses reasoning tokens, or direct answer.
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

## Known Limitations

- **Agent ReAct**: 0.5B models can't reliably follow ReAct JSON format. Needs 7B+ for reliable agent output.
- **SmolLM3**: THINKING model consumes all max_tokens on reasoning. Increase `--max-tokens` for content output.
- **amdgpu-container-cli**: Not installed, needs root. Legacy `/dev/dri` passthrough works fine.
- **MODEL_HOST_PATH**: Set `MODEL_HOST_PATH=/data/models` in `.env` to mount models from external drive.
- **Slots**: Config `ctx_size=2048, max_slots=4` (512 tokens/slot). Small for production, fine for POC.
