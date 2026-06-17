<!--
Source Session: ses_243bd50a1ffe5ux8BzV2RL3gKb
Part ID: prt_dbc42af63002I54NegQfTcTRdv
Character Count: 11728
Extracted: 2026-06-17T07:50:39Z
-->

## TASK
Create a Rust binary `src/bin/model_chain.rs` that executes a 3-model workflow chain with lifecycle hooks, retry logic, model download, and structured logging. Also create `src/client/model_download.rs` for HF model downloads.

## EXPECTED OUTCOME
1. `src/client/model_download.rs` — HF model download via reqwest
2. `src/bin/model_chain.rs` — 3-model workflow chain binary
3. `tests/model_chain_test.rs` — Skipped test descriptions only (no test code)
4. `Cargo.toml` updated with new binary entry
5. `src/client/mod.rs` updated with new module
6. `cargo check --bin model_chain` compiles with zero errors
7. Binary runs against live server: `cargo run --bin model_chain`

## REQUIRED TOOLS
- Read, Edit, Write, Bash (cargo check, cargo run)
- lsp_diagnostics

## CONTEXT

**Project root**: `/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/playful-star`

**Server**: llama.cpp in router mode at `http://localhost:8080`, `--models-dir /models`

**Available models** (already in `./models/`):
- `Qwen2.5-0.5B-Instruct-Q4_K_M` (469MB) — planner
- `SmolLM3-Q4_K_M` (1.8GB) — implementer

**Summarizer model** (must be downloaded if not present):
- HF repo: `Qwen/Qwen2.5-0.5B-Instruct-GGUF`
- HF filename: `qwen2.5-0.5b-instruct-q4_k_m.gguf`
- Local filename: `Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf` (use a different name to avoid collision)
- Model ID (for router): `Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer`
- For testing: manually `rm ./models/Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf` before running

**Existing code you MUST use** (read these files for patterns):
- `src/client/http_client.rs` — has `LlamaHttpClient` with `load_model()`, `unload_model()`, `list_models()`, `chat_completion()`, `wait_for_healthy()`
- `src/client/types.rs` — has `ChatCompletionRequest`, `ChatMessage`, `ModelInfo`, etc.
- `src/client/prompt_chain.rs` — has `PromptChain` for multi-step conversations
- `src/client/mod.rs` — module declarations

**Feature flag**: The `client` feature gates reqwest/serde_json/etc. The binary must use `required-features = ["client"]`.

**Key constraint**: `--models-dir` only scans top-level `.gguf` files. Models MUST be in the flat `./models/` directory.

## MUST DO

### File 1: `src/client/model_download.rs`

Create a module for downloading models from HuggingFace:

```rust
use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;

pub async fn download_model_from_hf(
    repo: &str,
    filename: &str,
    dest_path: &str,
) -> Result<u64> {
    // Download from: https://huggingface.co/{repo}/resolve/main/{filename}
    // Save to dest_path
    // Return: file size in bytes
    // Use reqwest with streaming to avoid loading entire file into memory
    // Create parent directory if needed
    // Log progress (every 10MB or 10% whichever comes first)
    // Use the reqwest client from the 'client' feature
}
```

Implementation details:
- URL format: `https://huggingface.co/{repo}/resolve/main/{filename}`
- Use `reqwest::Client::builder().timeout(Duration::from_secs(600)).build()?`
- Stream the response body with `bytes_stream()`
- Write chunks to file with `tokio::fs::File` + `tokio::io::AsyncWriteExt`
- Log progress every 10MB
- Return file size in bytes on success

### File 2: `src/bin/model_chain.rs`

This is the main binary. Structure:

```rust
use anyhow::Result;
use std::time::Instant;
use tracing::{info, warn, error};
use tracing_subscriber::EnvFilter;
use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::model_download::download_model_from_hf;

const SERVER_URL: &str = "http://localhost:8080";
const MODELS_DIR: &str = "./models";

struct StepConfig {
    role: String,
    model_id: String,
    system_prompt: String,
    hf_repo: Option<String>,
    hf_filename: Option<String>,
    local_filename: Option<String>,
    max_tokens: usize,
}

struct StepResult {
    role: String,
    model_id: String,
    output: String,
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
    load_ms: u64,
    inference_ms: u64,
    unload_ms: u64,
    download_ms: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("model_chain=info"))
        .with_target(false)
        .with_thread_ids(false)
        .init();

    let client = LlamaHttpClient::new(SERVER_URL)?;
    info!("Connecting to {}", SERVER_URL);
    client.wait_for_healthy(Duration::from_secs(30)).await?;
    info!("Server healthy");

    // Step 1: Planner
    // Step 2: Implementer  
    // Step 3: Summarizer (download if needed)
    // Each step: download (if needed) -> load -> chat -> unload
    // Log every lifecycle event with timing

    Ok(())
}
```

**Step execution pattern** (for each step):
1. Log `step_start` with role + model_id
2. Check if model file exists locally (for download steps)
3. If not present: download from HF, log `model_download_start/complete`
4. Load model via `client.load_model()`, log `model_load_start/complete` with timing
5. Send chat completion with system prompt + user prompt, log `chat_start/complete`
6. Unload model via `client.unload_model()`, log `model_unload_start/complete` with timing
7. Log `step_complete` with full result including timings
8. Return `StepResult`

**Specific step configs**:
- **Planner**: model_id=`Qwen2.5-0.5B-Instruct-Q4_K_M`, system="You are a planning assistant. Create a structured plan.", user="Create a plan for building a simple REST API in Rust using axum. Output a numbered list of steps.", max_tokens=512
- **Implementer**: model_id=`SmolLM3-Q4_K_M`, system="You are an expert Rust developer. Implement the plan you are given.", user=[planner output], max_tokens=1024
- **Summarizer**: model_id=`Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer`, hf_repo=`Qwen/Qwen2.5-0.5B-Instruct-GGUF`, hf_filename=`qwen2.5-0.5b-instruct-q4_k_m.gguf`, local_filename=`Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf`, system="You are a documentation specialist. Summarize the implementation.", user=[implementer output], max_tokens=512

**Retry logic**:
- For load/unload/chat operations, catch errors and retry up to 3 times with 2s backoff
- Log each retry attempt with the error
- For download: retry up to 2 times
- Non-retryable errors (400 bad request with non-"already running" message): log and fail immediately

**Logging format** (use `tracing::info!` with structured fields):
```
[step_start] role=planner model_id=Qwen2.5-0.5B-Instruct-Q4_K_M
[model_load_start] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M
[model_load_complete] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M duration_ms=1050
[chat_start] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M prompt_chars=142
[chat_complete] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M duration_ms=420 prompt_tokens=45 completion_tokens=128
[model_unload_start] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M
[model_unload_complete] model_id=Qwen2.5-0.5B-Instruct-Q4_K_M duration_ms=530
[step_complete] role=planner duration_ms=2000 tokens=173
[chain_complete] total_duration_ms=8500 steps=3
```

**Final output**: Print a summary table:
```
========================================
  Model Chain Results
========================================
Step 1: planner (Qwen2.5-0.5B-Instruct-Q4_K_M)
  Load:     1050ms
  Infer:    420ms
  Unload:   530ms
  Tokens:   173 (prompt=45, completion=128)

Step 2: implementer (SmolLM3-Q4_K_M)
  Load:     4679ms
  Infer:    1200ms
  Unload:   532ms
  Tokens:   520 (prompt=180, completion=340)

Step 3: summarizer (Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer)
  Download: 15200ms (469MB)
  Load:     1060ms
  Infer:    380ms
  Unload:   525ms
  Tokens:   210 (prompt=160, completion=50)

Total: 25.5s | Total tokens: 903
```

### File 3: `tests/model_chain_test.rs`

Skipped test descriptions only — NO test code. Each test is a `#[ignore]` empty function with a docstring describing validation criteria:

```rust
//! Integration test descriptions for model_chain binary.
//! Run manually: RUST_LOG=model_chain=debug cargo run --bin model_chain
//! 
//! To test download flow: rm ./models/Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf

/// 1. Planner step: Load Qwen model, send planning prompt, verify structured output, unload.
///    Validate: model loads, response is non-empty, tokens counted, unload succeeds.
///    Command: RUST_LOG=model_chain=debug cargo run --bin model_chain 2>&1 | grep -A5 "step_complete.*planner"
#[tokio::test]
#[ignore]
async fn test_planner_step() {}

/// 2. Implementer step: Load SmolLM3, feed planner output, verify implementation output, unload.
///    Validate: model loads (may take ~5s for 1.8GB), output references plan content, model unloads.
///    Command: RUST_LOG=model_chain=debug cargo run --bin model_chain 2>&1 | grep -A5 "step_complete.*implementer"
#[tokio::test]
#[ignore]
async fn test_implementer_step() {}

/// 3. Summarizer download + run: Ensure summarizer NOT present, trigger download, verify execution.
///    Validate: file appears in ./models/, download logged with size, model loads after download.
///    Precondition: rm ./models/Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf
///    Command: rm ./models/Qwen2.5-0.5B-Instruct-Q4_K_M_Summarizer.gguf && RUST_LOG=model_chain=debug cargo run --bin model_chain 2>&1 | grep "summarizer"
#[tokio::test]
#[ignore]
async fn test_summarizer_download_and_run() {}

/// 4. Full chain: All 3 steps execute sequentially, output flows correctly.
///    Validate: all 3 step_complete events logged, total time reported, each output feeds next input.
///    Command: RUST_LOG=model_chain=debug cargo run --bin model_chain
#[tokio::test]
#[ignore]
async fn test_full_chain() {}

/// 5. Hook visibility: Every lifecycle event logged with correct fields.
///    Validate: step_start, model_load_start/complete, chat_start/complete, model_unload_start/complete all present.
///    Command: RUST_LOG=model_chain=trace cargo run --bin model_chain 2>&1 | grep -E "\[(step_start|model_load|chat_|model_unload|step_complete|chain_complete)\]"
#[tokio::test]
#[ignore]
async fn test_hook_visibility() {}

/// 6. Retry logic: Simulate failure by loading non-existent model, verify retry logged.
///    Validate: retry attempts logged with backoff, error message clear.
///    NOTE: This requires modifying the binary temporarily. Skip for now.
#[tokio::test]
#[ignore]
async fn test_retry_on_failure() {}
```

### File 4: `Cargo.toml` changes

Add the new binary:
```toml
[[bin]]
name = "model_chain"
path = "src/bin/model_chain.rs"
required-features = ["client"]
```

### File 5: `src/client/mod.rs` changes

Add the new module:
```rust
pub mod model_download;
```

## MUST NOT DO
- Do NOT add doc comments (`///` or `//!`) to any new code — code must be self-documenting
- Do NOT add unnecessary comments — the logging IS the documentation
- Do NOT modify existing files except `src/client/mod.rs` and `Cargo.toml` (additions only)
- Do NOT use `as any`, `@ts-ignore`, or suppress errors
- Do NOT create new dependencies — use only what's already in Cargo.toml (reqwest, tokio, tracing, anyhow, serde_json, bytes, futures)
- Do NOT implement actual test logic — only descriptions as shown above
- Do NOT hardcode absolute paths — use relative `./models/` 
- Do NOT use unwrap() on fallible operations — use `?` with anyhow context

## VERIFICATION
1. `cargo check --bin model_chain` — must compile with zero errors
2. `cargo check --all-targets` — no regressions
3. Read the compiled binary source back and verify structure matches plan
4. Do NOT run `cargo run --bin model_chain` — the user will test against the live system

<!-- OMO_INTERNAL_INITIATOR -->