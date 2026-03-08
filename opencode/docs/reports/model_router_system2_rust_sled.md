# Model Router System Design 2: Rust/Sled/Tokio Stack

## Executive Summary

A high-performance, memory-efficient model routing system built in Rust using Sled for embedded storage, Tokio for async runtime, and ndarray for numerical computing. Designed for low-latency operation on minimal hardware with strong concurrency and safety guarantees.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         HTTP Server (Axum)                          │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Tokio Runtime (multi-threaded, async/await)               │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Routing Service Layer                              │
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │ Vector Encoder   │  │ Similarity Engine│  │ Rank Engine    │  │
│  │ (Rust ONNX)      │  │ (ndarray + BLAS) │  │ (Weighted)     │  │
│  └──────────────────┘  └──────────────────┘  └────────────────┘  │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Storage Layer (Sled)                               │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Tree 0: specializations        Tree 1: embeddings            │   │
│  │  Tree 2: execution_logs        Tree 3: model_stats            │   │
│  │  Tree 4: config                                               │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Dependencies

| Component | Crate | Memory | Notes |
|-----------|-------|---------|-------|
| HTTP Server | axum 0.7 | ~10MB | Async, type-safe routing |
| Async Runtime | tokio 1.35 | ~5MB | Multi-threaded scheduler |
| Embedded DB | sled 0.34 | ~8MB | Key-value, ACID, zero-config |
| Numerical | ndarray 0.15 | ~15MB | N-dimensional arrays |
| BLAS | ndarray-linalg 0.16 | ~8MB | Linear algebra backend |
| Serialization | serde 1.0 | ~5MB | Zero-copy deserialization |
| ONNX Runtime | ort 2.0 | ~120MB | Vector encoding |
| Hashing | blake3 1.5 | ~1MB | Fast hashing |

### Total Memory Footprint
- **Minimal runtime**: ~30MB (without ONNX model)
- **With ONNX model**: ~150MB (all-MiniLM-L6-v2)
- **With embeddings in RAM**: ~250MB (100 rows cached)

## Data Schema

### Sled Tree Structure

```rust
use sled::{IVec, Db};

// Tree 0: specializations
// Key: specialization_id (u8)
// Value: Serialization of Specialization struct
#[derive(Debug, Serialize, Deserialize)]
struct Specialization {
    row_kind: String,
    specialization_category: String,
    specialization_synonyms: String,
    related_synonym_tags: String,
    top1_model: String,
    top1_ctx: u32,
    top2_model: String,
    top2_ctx: u32,
    top3_model: String,
    top3_ctx: u32,
    wildcard_model: String,
    wildcard_ctx: u32,
    primary_strength: String,
    secondary_strengths: String,
    preferred_input_shape: String,
    preferred_output_shape: String,
    task_granularity: String,
    context_need: String,
    output_rigidity: String,
    verification_need: String,
    recommended_decoding: String,
    latency_bias: String,
    router_keywords: String,
    avoid_when: String,
    selection_notes: String,
}

// Tree 1: embeddings
// Key: specialization_id (u8)
// Value: 384 * f32 (1536 bytes)
type Embedding = [f32; 384];

// Tree 2: execution_logs
// Key: log_id (u64 auto-increment)
// Value: ExecutionLog struct
#[derive(Debug, Serialize, Deserialize)]
struct ExecutionLog {
    prompt_hash: [u8; 32],  // BLAKE3
    prompt_prefix: String,
    selected_models: Vec<String>,
    successful_model: Option<String>,
    attempts: u8,
    success: bool,
    latency_ms: f32,
    timestamp: i64,  // Unix seconds
    specialization_id: u8,
}

// Tree 3: model_stats
// Key: model_name
// Value: ModelStats struct
#[derive(Debug, Serialize, Deserialize)]
struct ModelStats {
    total_attempts: u64,
    successes: u64,
    failures: u64,
    avg_attempts_to_success: f32,
    penalty_score: f32,  // 0.0 to 1.0
    last_updated: i64,
}

// Tree 4: config
// Key: config_key
// Value: config_value
```

## Core Implementation

### Vector Encoder (ONNX)

```rust
use ort::{Environment, Session, Value};

pub struct VectorEncoder {
    session: Session,
    env: Environment,
}

impl VectorEncoder {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let env = Environment::builder()
            .with_execution_providers([ort::CPUExecutionProvider::default()])
            .build()?;

        // Load ONNX model (all-MiniLM-L6-v2 quantized)
        let session = Session::builder(&env)?
            .with_model_from_file("./models/all-MiniLM-L6-v2-q8.onnx")?;

        Ok(Self { session, env })
    }

    pub fn encode_text(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        // Tokenize (simplified - use proper tokenizer in production)
        let tokens = self.tokenize(text);

        // Run inference
        let outputs = self.session.run(vec![
            ("input_ids".into(), Value::from_array(
                vec![tokens].into()
            )?)
        ])?;

        // Get embedding (mean pooling)
        let embedding = outputs["last_hidden_state"]
            .try_extract_tensor::<f32>()?
            .mean_axis(ndarray::Axis(1))?;

        Ok(embedding.to_vec())
    }

    fn tokenize(&self, text: &str) -> Vec<i64> {
        // Placeholder - use tokenizers crate or rust-bert
        text.split_whitespace()
            .map(|_| 1)  // Simplified
            .take(512)
            .collect()
    }
}
```

### Similarity Search (ndarray)

```rust
use ndarray::{Array2, ArrayView1, s};

pub struct SimilarityEngine {
    embeddings_cache: Option<Array2<f32>>,
}

impl SimilarityEngine {
    pub fn new() -> Self {
        Self { embeddings_cache: None }
    }

    pub fn load_embeddings(&mut self, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
        let tree = db.open_tree("embeddings")?;

        let mut rows = Vec::new();
        for result in tree.iter() {
            let (_, embedding_bytes) = result?;
            let embedding: Embedding = bincode::deserialize(&embedding_bytes)?;
            rows.push(embedding);
        }

        // Convert to ndarray (N x 384)
        let n = rows.len();
        let mut arr = Array2::zeros((n, 384));
        for (i, emb) in rows.into_iter().enumerate() {
            arr.row_mut(i).assign(&ArrayView1::from(&emb));
        }

        self.embeddings_cache = Some(arr);
        Ok(())
    }

    pub fn find_closest(
        &self,
        input: &[f32; 384],
        top_k: usize
    ) -> Vec<(usize, f32)> {
        let embeddings = self.embeddings_cache.as_ref().unwrap();
        let input_arr = ArrayView1::from(input);

        // Cosine similarity = dot product (normalized vectors)
        let similarities = embeddings.dot(&input_arr);

        // Sort and get top-k
        let mut indexed: Vec<_> = similarities
            .iter()
            .enumerate()
            .map(|(i, &s)| (i, s))
            .collect();

        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        indexed.into_iter().take(top_k).collect()
    }
}
```

### Ranking Engine

```rust
use std::collections::HashMap;

pub struct RankingEngine {
    penalty_weight: f32,
    context_weight: f32,
}

impl RankingEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            penalty_weight: config.penalty_weight,
            context_weight: config.context_weight,
        }
    }

    pub fn rank_models(
        &self,
        spec: &Specialization,
        similarity: f32,
        stats: &HashMap<String, ModelStats>
    ) -> Vec<(String, f32)> {
        let mut ranked = Vec::new();

        // Model candidates with position weights
        let candidates = [
            (&spec.top1_model, 1.0, spec.top1_ctx),
            (&spec.top2_model, 0.9, spec.top2_ctx),
            (&spec.top3_model, 0.8, spec.top3_ctx),
            (&spec.wildcard_model, 0.6, spec.wildcard_ctx),
        ];

        for (model, pos_weight, ctx_size) in candidates {
            if model.is_empty() {
                continue;
            }

            // Stage 1: Base score
            let base = similarity * pos_weight;

            // Stage 2: Apply log penalty
            let penalty = stats.get(model)
                .map(|s| s.penalty_score)
                .unwrap_or(0.0);
            let log_adjusted = base * (1.0 - penalty);

            // Stage 3: Context bonus
            let ctx_bonus = (ctx_size as f32 / 524288.0).min(1.0) * self.context_weight;

            // Stage 4: Final score
            let final_score = log_adjusted + ctx_bonus;

            ranked.push((model.clone(), final_score));
        }

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        ranked
    }
}
```

### Log Penalty Calculation

```rust
use sled::Db;

pub struct StatsUpdater {
    db: Db,
}

impl StatsUpdater {
    pub fn update_execution(
        &self,
        model: &str,
        prompt_hash: [u8; 32],
        attempts: u8,
        success: bool,
        latency_ms: f32
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stats_tree = self.db.open_tree("model_stats")?;
        let logs_tree = self.db.open_tree("execution_logs")?;

        // Get or create stats
        let key = model.as_bytes();
        let stats: Option<ModelStats> = stats_tree.get(key)?
            .map(|bytes| bincode::deserialize(&bytes).ok())
            .flatten();

        let new_stats = if let Some(mut s) = stats {
            s.total_attempts += 1;
            if success {
                s.successes += 1;
                // Update avg attempts
                s.avg_attempts_to_success =
                    (s.avg_attempts_to_success * (s.successes - 1) as f32 + attempts as f32)
                    / s.successes as f32;
            } else {
                s.failures += 1;
            }

            // Calculate penalty
            let failure_rate = s.failures as f32 / s.total_attempts as f32;
            let attempt_penalty = (s.avg_attempts_to_success / 10.0).min(1.0);
            s.penalty_score = (failure_rate * 0.7) + (attempt_penalty * 0.3);
            s.last_updated = chrono::Utc::now().timestamp();

            s
        } else {
            ModelStats {
                total_attempts: 1,
                successes: if success { 1 } else { 0 },
                failures: if success { 0 } else { 1 },
                avg_attempts_to_success: attempts as f32,
                penalty_score: if success { 0.0 } else 0.1,
                last_updated: chrono::Utc::now().timestamp(),
            }
        };

        // Save stats
        stats_tree.insert(key, bincode::serialize(&new_stats)?)?;

        // Log execution
        let log_id = self.db.generate_id()?;
        let log = ExecutionLog {
            prompt_hash,
            prompt_prefix: String::new(),  // Filled by caller
            selected_models: vec![],
            successful_model: if success { Some(model.to_string()) } else { None },
            attempts,
            success,
            latency_ms,
            timestamp: chrono::Utc::now().timestamp(),
            specialization_id: 0,  // Filled by caller
        };

        logs_tree.insert(log_id.to_be_bytes(), bincode::serialize(&log)?)?;

        Ok(())
    }
}
```

### Agentic Executor (Tokio)

```rust
use axum::{Json, extract::State};
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use blake3::hash;

pub struct AgenticExecutor {
    config: Arc<Config>,
    stats_updater: Arc<StatsUpdater>,
}

impl AgenticExecutor {
    pub async fn execute_with_retry(
        &self,
        prompt: &str,
        ranked_models: Vec<(String, f32)>,
        verifier: impl Fn(&str) -> bool + Send + Sync + 'static
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let prompt_hash = hash(prompt.as_bytes()).into_bytes();

        let mut models_iter = ranked_models
            .into_iter()
            .take(self.config.max_models_to_try as usize)
            .enumerate();

        while let Some((model_idx, (model_name, _score))) = models_iter.next() {
            let mut model_success = false;
            let mut model_attempts = 0u8;
            let start = std::time::Instant::now();

            for attempt in 0..self.config.max_retries_per_model {
                model_attempts += 1;

                match timeout(
                    Duration::from_secs(self.config.timeout_seconds),
                    self.call_model(&model_name, prompt)
                ).await {
                    Ok(Ok(result)) => {
                        if verifier(&result) {
                            model_success = true;
                            let latency = start.elapsed().as_millis() as f32;

                            self.stats_updater.update_execution(
                                &model_name,
                                prompt_hash,
                                model_attempts,
                                true,
                                latency
                            )?;

                            return Ok((result, model_name));
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("Model {} attempt {} failed: {}", model_name, attempt, e);
                    }
                    Err(_) => {
                        eprintln!("Model {} attempt {} timed out", model_name, attempt);
                    }
                }
            }

            // Model failed all attempts
            let latency = start.elapsed().as_millis() as f32;
            self.stats_updater.update_execution(
                &model_name,
                prompt_hash,
                model_attempts,
                false,
                latency
            )?;
        }

        Err("All models failed".into())
    }

    async fn call_model(
        &self,
        model: &str,
        prompt: &str
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for actual LLM call
        // Could use reqwest to call local LLM API
        Ok(format!("Output from {} for: {}", model, prompt))
    }
}
```

### HTTP API (Axum)

```rust
use axum::{Router, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RouteRequest {
    prompt: String,
    max_models: Option<usize>,
}

#[derive(Serialize)]
struct RouteResponse {
    ranked_models: Vec<ModelScore>,
    top_specialization: Specialization,
    similarity: f32,
}

#[derive(Serialize)]
struct ModelScore {
    model: String,
    score: f32,
}

pub async fn create_router(
    db: sled::Db,
    encoder: VectorEncoder,
    similarity: SimilarityEngine,
    ranking: RankingEngine
) -> Router {
    let state = AppState {
        db,
        encoder,
        similarity,
        ranking,
    };

    Router::new()
        .route("/route", axum::routing::post(route_handler))
        .with_state(state)
}

struct AppState {
    db: sled::Db,
    encoder: VectorEncoder,
    similarity: SimilarityEngine,
    ranking: RankingEngine,
}

async fn route_handler(
    State(state): State<AppState>,
    Json(req): Json<RouteRequest>
) -> Result<Json<RouteResponse>, StatusCode> {
    // Encode prompt
    let embedding = state.encoder
        .encode_text(&req.prompt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut emb_array = [0.0f32; 384];
    emb_array.copy_from_slice(&embedding);

    // Find similar
    let matches = state.similarity.find_closest(&emb_array, 10);

    if matches.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Get specialization
    let top_id = matches[0].0 as u8;
    let top_score = matches[0].1;

    let spec_tree = state.db.open_tree("specializations")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let spec_bytes = spec_tree.get([top_id])
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let spec: Specialization = bincode::deserialize(&spec_bytes)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Rank models
    let ranked = state.ranking.rank_models(&spec, top_score, &Default::default());

    Ok(Json(RouteResponse {
        ranked_models: ranked
            .into_iter()
            .take(req.max_models.unwrap_or(3))
            .map(|(m, s)| ModelScore { model: m, score: s })
            .collect(),
        top_specialization: spec,
        similarity: top_score,
    }))
}
```

## Configuration

```toml
# Cargo.toml
[package]
name = "model-router"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.35", features = ["full"] }
sled = "0.34"
ndarray = { version = "0.15", features = ["rayon"] }
ndarray-linalg = "0.16"
ort = { version = "2.0", default-features = false, features = ["half"] }
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
blake3 = "1.5"
chrono = "0.4"
anyhow = "1.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Cold start time | ~500ms | Load Sled + ONNX |
| Embedding time | ~10ms | ONNX, CPU |
| Similarity search | ~1ms | ndarray, 100 rows |
| Full route | ~30ms | End-to-end |
| Memory (idle) | ~30MB | Without model |
| Memory (active) | ~150MB | With model |
| Max QPS | ~1000 | Multi-threaded |
| Concurrent requests | 10K+ | Tokio runtime |

## Advantages

1. **Extreme performance**: Zero-cost abstractions, compiled
2. **Memory safety**: Rust's ownership model
3. **Concurrency**: Tokio async, no thread blocking
4. **Small binary**: ~8MB stripped release
5. **No external deps**: Sled is embedded, no servers needed

## Limitations

1. **Complexity**: Steeper learning curve
2. **Compile time**: Slow for large projects
3. **Tooling**: Less mature than Python ecosystem
4. **ONNX setup**: Requires model conversion

## Extensions

1. **GPU support**: Use ort with CUDA provider
2. **FAISS integration**: For large-scale similarity search
3. **gRPC API**: Replace REST for high-throughput
4. **Quantization**: Use f16 embeddings to halve memory
5. **Hot reload**: Watch Sled tree changes
