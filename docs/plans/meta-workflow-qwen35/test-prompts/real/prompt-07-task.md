<!--
Source Session: ses_20fdf6892ffeSv95SCOT87k7NX
Part ID: prt_df0209772002ODHNL7Wiv6Lg8v
Character Count: 15561
Extracted: 2026-06-17T07:50:39Z
-->

## TASK
Create a new `src/benchmark/` module with result types and a multi-model benchmark runner. Then modify `src/bin/whitt.rs` to wire up the new benchmark CLI.

## EXPECTED OUTCOME
- `src/benchmark/mod.rs` — Module with result types
- `src/benchmark/runner.rs` — BenchmarkRunner that sequentially swaps models
- `src/lib.rs` updated to include `pub mod benchmark;`
- `src/bin/whitt.rs` updated: remove dead `--concurrent`, add `--models-dir`, `--model-list`, `--prompts`, `--output`, `--filter-size-max`, `--filter-name`
- `cargo test --all-features` passes
- `cargo clippy --all-features -- -W clippy::all` passes with 0 warnings

## REQUIRED TOOLS
- Read, Write, Edit (for file creation/modification)
- Bash (for cargo test/clippy verification)
- lsp_diagnostics (for changed files)

## MUST DO

### 1. Create `src/benchmark/mod.rs`

```rust
//! Benchmark module for multi-model performance testing.

pub mod runner;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Metrics for a single inference call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub prompt: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub duration: Duration,
    pub tokens_per_second: f64,
}

/// Benchmark result for a single model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBenchmarkResult {
    pub model_id: String,
    pub model_path: String,
    pub file_size_bytes: u64,
    pub load_duration: Duration,
    pub inference_results: Vec<InferenceResult>,
    pub unload_duration: Duration,
    pub total_duration: Duration,
    pub tokens_per_second: f64,
    pub avg_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub error: Option<String>,
}

/// Aggregate benchmark result across all models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteResult {
    pub timestamp: String,
    pub server_url: String,
    pub total_models: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<ModelBenchmarkResult>,
}

impl BenchmarkSuiteResult {
    pub fn to_csv(&self) -> String { /* CSV header + rows */ }
    pub fn to_table(&self) -> String { /* aligned console table */ }
}
```

Implement `to_csv()` and `to_table()` fully. For percentiles: sort latencies, index at `len() * percentile`. For `tokens_per_second`: sum completion_tokens / sum duration_secs. For `avg_latency_ms`: mean of per-request duration_ms.

### 2. Create `src/benchmark/runner.rs`

```rust
use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult};
use anyhow::Result;
use std::path::Path;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Configuration for a benchmark run.
pub struct BenchmarkConfig {
    pub server_url: String,
    pub models_dir: Option<String>,        // scan directory for GGUF files
    pub model_list_file: Option<String>,   // read paths from file
    pub prompts: Vec<String>,              // prompts to run per model
    pub max_tokens: usize,
    pub filter_size_max: Option<u64>,      // skip models larger than this (bytes)
    pub filter_name: Option<String>,       // regex filter on model name
    pub delay_between_swaps: Duration,     // delay for VRAM reclamation (default 2s)
}

/// Runs sequential multi-model benchmarks.
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self { Self { config } }

    /// Discover models from directory or file.
    fn discover_models(&self) -> Result<Vec<String>> {
        // If models_dir: walk dir, find .gguf files, use filename as model_id
        // If model_list_file: read file, one path per line
        // Apply filter_size_max and filter_name
        // Return Vec of model_id strings
    }

    /// Run benchmark on all discovered models.
    pub async fn run(&self) -> Result<BenchmarkSuiteResult> {
        let client = LlamaHttpClient::new(&self.config.server_url)?;
        let models = self.discover_models()?;
        let mut results = Vec::new();

        for (i, model_id) in models.iter().enumerate() {
            info!("[benchmark] {}/{} loading {}", i+1, models.len(), model_id);

            let model_result = self.benchmark_single_model(&client, model_id).await;
            results.push(model_result);

            // Delay between swaps for VRAM reclamation
            if i < models.len() - 1 {
                tokio::time::sleep(self.config.delay_between_swaps).await;
            }
        }

        // Build BenchmarkSuiteResult
        let successful = results.iter().filter(|r| r.error.is_none()).count();
        let failed = results.len() - successful;
        Ok(BenchmarkSuiteResult {
            timestamp: chrono::Utc::now().to_rfc3339(),  // or use custom format
            server_url: self.config.server_url.clone(),
            total_models: results.len(),
            successful,
            failed,
            results,
        })
    }

    async fn benchmark_single_model(&self, client: &LlamaHttpClient, model_id: &str) -> ModelBenchmarkResult {
        let start = Instant::now();

        // 1. Unload any currently loaded model
        if let Ok(models) = client.list_models().await {
            for m in models {
                if m.status.value == "loaded" {
                    let _ = client.unload_model(&m.id).await;
                }
            }
        }

        // 2. Load target model
        let load_start = Instant::now();
        let load_result = client.load_model(model_id).await;
        let load_duration = load_start.elapsed();

        if let Err(e) = load_result {
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: String::new(),
                file_size_bytes: 0,
                load_duration,
                inference_results: Vec::new(),
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("Load failed: {}", e)),
            };
        }

        // 3. Run prompts
        let mut inference_results = Vec::new();
        for prompt in &self.config.prompts {
            let request = ChatCompletionRequest {
                model: model_id.to_string(),
                messages: vec![ChatMessage::user(prompt)],
                max_tokens: Some(self.config.max_tokens),
                stream: false,
                ..Default::default()
            };

            let inf_start = Instant::now();
            match client.chat_completion(request).await {
                Ok(resp) => {
                    let duration = inf_start.elapsed();
                    let tps = if duration.as_secs_f64() > 0.0 {
                        resp.usage.completion_tokens as f64 / duration.as_secs_f64()
                    } else { 0.0 };
                    inference_results.push(InferenceResult {
                        prompt: prompt.clone(),
                        prompt_tokens: resp.usage.prompt_tokens,
                        completion_tokens: resp.usage.completion_tokens,
                        total_tokens: resp.usage.total_tokens,
                        duration,
                        tokens_per_second: tps,
                    });
                }
                Err(e) => {
                    warn!("[benchmark] inference failed for {}: {}", model_id, e);
                }
            }
        }

        // 4. Unload model
        let unload_start = Instant::now();
        let _ = client.unload_model(model_id).await;
        let unload_duration = unload_start.elapsed();

        // 5. Calculate aggregate metrics
        let total_duration = start.elapsed();
        let completion_tokens_sum: usize = inference_results.iter().map(|r| r.completion_tokens).sum();
        let duration_sum: f64 = inference_results.iter().map(|r| r.duration.as_secs_f64()).sum();
        let tps = if duration_sum > 0.0 { completion_tokens_sum as f64 / duration_sum } else { 0.0 };

        let mut latencies: Vec<f64> = inference_results.iter().map(|r| r.duration.as_secs_f64() * 1000.0).collect();
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let avg_ms = if latencies.is_empty() { 0.0 } else { latencies.iter().sum::<f64>() / latencies.len() as f64 };
        let p50 = percentile(&latencies, 0.50);
        let p95 = percentile(&latencies, 0.95);
        let p99 = percentile(&latencies, 0.99);

        ModelBenchmarkResult {
            model_id: model_id.to_string(),
            model_path: String::new(),
            file_size_bytes: 0,
            load_duration,
            inference_results,
            unload_duration,
            total_duration,
            tokens_per_second: tps,
            avg_latency_ms: avg_ms,
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            error: None,
        }
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let idx = ((sorted.len() as f64) * p).min(sorted.len() as f64 - 1.0) as usize;
    sorted[idx]
}
```

DO NOT use `chrono` crate. Use `std::time::SystemTime` for timestamp instead, or just use a string like `format!("{:?}", std::time::SystemTime::now())`.

For `discover_models()`: use `walkdir::WalkDir` to scan `models_dir`, find files ending in `.gguf`, extract model_id from filename (strip `.gguf`). If `model_list_file`, read file lines. Apply size filter with `std::fs::metadata`. Apply name filter with `regex::Regex`.

### 3. Modify `src/bin/whitt.rs`

**Remove** the `--concurrent` flag from `Benchmark` subcommand. Add new flags:

```rust
/// Benchmark model performance
Benchmark {
    /// Prompt text (can be specified multiple times)
    #[arg(long, default_value = "The quick brown fox jumps over the lazy dog.")]
    prompt: Option<String>,

    /// Max tokens to generate
    #[arg(long, default_value = "100")]
    max_tokens: usize,

    /// Directory to scan for GGUF models (multi-model benchmark)
    #[arg(long)]
    models_dir: Option<String>,

    /// File containing model paths, one per line
    #[arg(long)]
    model_list: Option<String>,

    /// Number of prompts to run per model
    #[arg(long, default_value = "3")]
    prompts: usize,

    /// Output format: table, json, csv
    #[arg(long, default_value = "table")]
    output: String,

    /// Skip models larger than this (bytes)
    #[arg(long)]
    filter_size_max: Option<u64>,

    /// Regex filter on model name
    #[arg(long)]
    filter_name: Option<String>,
},
```

**Modify** the `Commands::Benchmark` match arm:

When `--models-dir` or `--model-list` is provided:
1. Build `BenchmarkConfig`
2. Create `BenchmarkRunner`
3. Call `runner.run().await`
4. Format output based on `--output` flag (table/json/csv)
5. Print to stdout

When neither is provided (legacy single-model mode):
- Keep existing `benchmark_command()` behavior for backward compatibility

The match arm at line 275 needs to handle both modes. Structure it like:

```rust
Commands::Benchmark { prompt, max_tokens, models_dir, model_list, prompts, output, filter_size_max, filter_name } => {
    if models_dir.is_some() || model_list.is_some() {
        // Multi-model benchmark
        let config = BenchmarkConfig { ... };
        let runner = BenchmarkRunner::new(config);
        let result = runner.run().await?;
        match output.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&result)?),
            "csv" => println!("{}", result.to_csv()),
            _ => println!("{}", result.to_table()),
        }
    } else {
        // Legacy single-model benchmark
        benchmark_command(&cli.url, prompt, max_tokens).await
    }
}
```

### 4. Update `src/lib.rs`

Add `pub mod benchmark;` after the existing module declarations. Do NOT put it behind a feature gate — benchmark module should always be available (it uses types from `client` module which is already feature-gated).

Wait — the benchmark module uses `LlamaHttpClient` which is behind the `client` feature. So benchmark module ALSO needs to be behind the `client` feature:

```rust
#[cfg(feature = "client")]
pub mod benchmark;
```

### 5. Add necessary imports in whitt.rs

```rust
use whitt_execution_engine::benchmark::runner::{BenchmarkRunner, BenchmarkConfig};
use whitt_execution_engine::benchmark::BenchmarkSuiteResult;
```

## MUST NOT DO
- Do NOT modify any files in `src/client/` — Phase A handles that
- Do NOT add new crate dependencies — use only what's in Cargo.toml already (walkdir, regex, serde, serde_json, tokio, anyhow, tracing)
- Do NOT use `chrono` — use `std::time::SystemTime` for timestamps
- Do NOT modify `src/agent/`, `src/config/`, `src/model/`, `src/backend/` — only `src/benchmark/`, `src/lib.rs`, `src/bin/whitt.rs`
- Do NOT suppress any warnings or errors with `#[allow(...)]`
- Do NOT leave TODO comments in code — implement everything fully
- Do NOT use `as any`, `@ts-ignore`, or similar type suppression (this is Rust, but same principle)

## CONTEXT

### Existing code patterns to follow
- Error handling: `anyhow::Result` for functions, `.context("message")?` for error chains
- Logging: `tracing::{info, warn, error}` with structured fields: `tracing::info!(model = %model_id, "message")`
- Client usage: `LlamaHttpClient::new(url)?`, then `client.load_model()`, `client.chat_completion()`, etc.
- Types: `ChatCompletionRequest { model, messages, max_tokens, stream, ..Default::default() }`
- Messages: `ChatMessage::user("text")`
- CLI: `clap` derive macros, `#[arg(long, default_value = "...")]`

### File locations
- `src/bin/whitt.rs` — Main CLI (1263 lines)
- `src/client/http_client.rs` — LlamaHttpClient with load/unload/list/wait methods
- `src/client/types.rs` — ChatCompletionRequest, ChatCompletionResponse, Usage, ModelInfo, ModelLoadRequest, etc.
- `src/lib.rs` — Root module declarations
- `Cargo.toml` — Dependencies (walkdir, regex, serde, serde_json, tokio, etc.)

### Key API signatures (from http_client.rs)
```rust
impl LlamaHttpClient {
    pub fn new(base_url: &str) -> Result<Self>;
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>>;          // GET v1/models
    pub async fn load_model(&self, model_id: impl Into<String>) -> Result<()>;  // POST models/load, waits for "loaded" status (120s timeout)
    pub async fn unload_model(&self, model_id: impl Into<String>) -> Result<()>; // POST models/unload, waits for "unloaded" status (60s timeout)
    pub async fn chat_completion(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse>;
}

struct ModelInfo { pub id: String, pub status: ModelStatus }
struct ModelStatus { pub value: String }  // "loaded", "loading", "unloaded"
struct Usage { pub prompt_tokens: usize, pub completion_tokens: usize, pub total_tokens: usize }
```

### Verification
After implementing, run:
1. `cargo test --all-features` — ALL tests must pass
2. `cargo clippy --all-features -- -W clippy::all` — 0 warnings
3. `cargo build --release --all-features` — exit code 0

If any of these fail, fix the issues before reporting completion.

<!-- OMO_INTERNAL_INITIATOR -->