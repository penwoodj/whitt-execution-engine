# POC Plan: 3-Model Workflow Chain (Planner → Implementer → Summarizer)

## Objective

Build a Rust binary/script that executes a 3-model workflow chain:
1. **Planner** model: Analyzes a task, produces a structured plan
2. **Implementer** model: Takes the plan, produces implementation
3. **Summarizer** model: Downloads on-demand, summarizes the implementation output

Each model is loaded/unloaded dynamically via the llama.cpp router API. The summarizer model must be downloaded if not present (simulating a "cold start" scenario).

## Architecture

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐
│ Planner  │────▶│ Implementer  │────▶│ Summarizer   │
│ (loaded) │     │ (swap in)    │     │ (download+   │
│          │     │              │     │  load)       │
└──────────┘     └──────────────┘     └──────────────┘
```

### Models

| Role | Model | Source | Notes |
|------|-------|--------|-------|
| Planner | Qwen2.5-0.5B-Instruct-Q4_K_M | Already in ./models | Small, good for planning |
| Implementer | SmolLM3-3B-Q4_K_M | Copy from /run/media/jon/data | Better for implementation (3B params) |
| Summarizer | SmolLM3-3B-Q4_K_M (or different) | Download from HF on-demand | Tests download + load lifecycle |

### Key Constraint
llama.cpp `--models-dir` only scans top-level .gguf files. Models in subdirectories won't be found. All models must be in the flat `./models/` directory.

## Implementation Plan

### Phase 1: Model Preparation
- [ ] Copy SmolLM3-3B-Q4_K_M.gguf from external drive to `./models/`
- [ ] Verify both models appear in `GET /v1/models`
- [ ] Manually remove summarizer model to test download flow

### Phase 2: Rust Workflow Engine (`src/bin/model_chain.rs`)

#### Core Types
```rust
struct ChainStep {
    id: usize,
    role: String,         // "planner", "implementer", "summarizer"
    model_id: String,     // model ID for router API
    system_prompt: String,
    user_prompt: String,  // may include output from previous step
    require_download: bool,
    hf_repo: Option<String>,
    hf_filename: Option<String>,
}

struct StepResult {
    step_id: usize,
    model_id: String,
    output: String,
    prompt_tokens: usize,
    completion_tokens: usize,
    load_time_ms: u64,
    inference_time_ms: u64,
    unload_time_ms: u64,
}

struct ChainResult {
    steps: Vec<StepResult>,
    total_time_ms: u64,
    success: bool,
}
```

#### Lifecycle Hooks (per step)
Each lifecycle event calls registered hooks with structured context:
```
EVENTS:
- step_start(step_id, role, model_id)
- model_check(model_id)          -- is it available?
- model_download_start(repo, filename)
- model_download_complete(path, size, time_ms)
- model_load_start(model_id)
- model_load_complete(model_id, time_ms)
- chat_start(model_id, prompt_len)
- chat_chunk_received(token, cumulative)
- chat_complete(output, tokens, time_ms)
- model_unload_start(model_id)
- model_unload_complete(model_id, time_ms)
- step_complete(step_id, result)
- chain_complete(result)
```

#### Logging
- `tracing` crate with structured fields
- Every hook emits a `tracing::info!` with timing + context
- Configurable via `RUST_LOG=model_chain=debug`
- Each event logs: timestamp, event name, model_id, duration_ms, token counts

#### Error Handling + Retry
- Retry config: max_attempts, backoff_multiplier, max_backoff_ms
- Retryable errors: connection refused, timeout, 503, incomplete message
- Non-retryable: 400 bad request (wrong model), auth errors
- Each retry logs: attempt number, error, backoff duration
- After max retries: log full context, mark step as failed, continue or abort

#### Model Download
- Use existing `download_model()` pattern from entrypoint.sh but in Rust
- Use `reqwest` to call HuggingFace API: `https://huggingface.co/{repo}/resolve/{branch}/{filename}`
- Download to `./models/{model_id}.gguf`
- Verify download: check file exists and size > 0
- After download, the model should appear in `GET /v1/models` automatically (router watches the dir)

### Phase 3: Skipped Integration Tests (`tests/model_chain_test.rs`)

Tests are `#[ignore]` with descriptions only as validation criteria. No test code — verified manually against live system.

#### Test Descriptions
1. **test_planner_step**: Load planner model, send planning prompt, verify structured output, unload. Validate: model loads, response is non-empty, tokens counted.

2. **test_implementer_step**: Load implementer model, feed planner output, verify implementation output, unload. Validate: model loads, output references plan, model unloads cleanly.

3. **test_summarizer_download_and_run**: Ensure summarizer model NOT present, trigger download via chain, verify download completes, load model, run summarization, unload. Validate: file appears in ./models/, model loads after download, summary references implementation.

4. **test_full_chain**: Execute all 3 steps in sequence. Each step receives previous output. Validate: all 3 steps complete, total time logged, each step output feeds into next step's input.

5. **test_retry_on_load_failure**: Kill model during load (simulate crash), verify retry logic kicks in. Validate: retry logged, model eventually loads.

6. **test_hook_visibility**: Run chain with RUST_LOG=debug, verify every lifecycle event is logged with correct fields. Validate: all events present in output.

### Phase 4: Iteration + Verification

Run against live Docker container:
```bash
# 1. Start server in router mode
docker compose up -d

# 2. Prepare models (copy SmolLM3 to ./models/)
cp /run/media/jon/data/models/ggml-org/SmolLM3-3B-GGUF/SmolLM3-Q4_K_M.gguf ./models/

# 3. Run chain with debug logging
RUST_LOG=model_chain=debug cargo run --bin model_chain

# 4. Test download flow: remove summarizer model first
rm ./models/SmolLM3-Q4_K_M.gguf
RUST_LOG=model_chain=debug cargo run --bin model_chain

# 5. Verify all tests pass
cargo test --test model_chain_test -- --ignored --test-threads=1
```

## Validation Criteria

| Criterion | How to Verify |
|-----------|--------------|
| Planner step works | Loads Qwen model, produces plan, unloads |
| Implementer step works | Loads SmolLM3, takes plan input, produces implementation |
| Summarizer download works | Downloads .gguf from HF, verifies file, loads model |
| Chain execution | All 3 steps run sequentially, output flows correctly |
| Hooks fire | Every lifecycle event logged with timing + context |
| Retry logic | Connection failures trigger retry with backoff |
| Error handling | Non-retryable errors abort step with clear message |
| Total timing | Chain reports per-step and total elapsed time |

## Key Decisions

1. **Binary vs library**: Binary (`src/bin/model_chain.rs`) — this is a standalone workflow executor
2. **Model download in Rust**: Use reqwest to download from HF direct URL, not shell script
3. **Hook system**: Trait-based hooks with a default logging implementation
4. **Model ID convention**: Filename without .gguf extension (router mode requirement)
5. **Single-threaded**: Models loaded one at a time (Qwen too small for concurrent)

## Files to Create/Modify

| File | Change |
|------|--------|
| `src/bin/model_chain.rs` | NEW: Main workflow binary |
| `src/client/model_download.rs` | NEW: HF model download via reqwest |
| `src/client/mod.rs` | Add `pub mod model_download;` |
| `tests/model_chain_test.rs` | NEW: Skipped test descriptions only |
| `Cargo.toml` | Add `model_chain` binary + any new deps |
