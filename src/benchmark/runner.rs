use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult};
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;
use std::time::{Duration, Instant};
use tracing::{info, warn};

pub struct BenchmarkConfig {
    pub server_url: String,
    pub models_dir: Option<String>,
    pub model_list_file: Option<String>,
    pub prompts: Vec<String>,
    pub max_tokens: usize,
    pub filter_size_max: Option<u64>,
    pub filter_name: Option<String>,
    pub delay_between_swaps: Duration,
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    fn discover_models(&self) -> Result<Vec<String>> {
        let mut models = Vec::new();

        if let Some(ref models_dir) = self.config.models_dir {
            for entry in walkdir::WalkDir::new(models_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "gguf") {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        let model_id = file_name.to_string();

                        if let Some(max_size) = self.config.filter_size_max {
                            if let Ok(metadata) = std::fs::metadata(path) {
                                if metadata.len() > max_size {
                                    info!("[benchmark] skipping {} (size {} > {})",
                                        model_id, metadata.len(), max_size);
                                    continue;
                                }
                            }
                        }

                        if let Some(ref pattern) = self.config.filter_name {
                            if let Ok(re) = Regex::new(pattern) {
                                if !re.is_match(&model_id) {
                                    info!("[benchmark] skipping {} (name filter: {})",
                                        model_id, pattern);
                                    continue;
                                }
                            }
                        }

                        models.push(model_id);
                    }
                }
            }
        }

        if let Some(ref list_file) = self.config.model_list_file {
            let content = std::fs::read_to_string(list_file)
                .with_context(|| format!("Failed to read model list file: {}", list_file))?;

            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                let path = Path::new(line);
                let model_id = if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    file_name.to_string()
                } else {
                    line.to_string()
                };

                if let Some(max_size) = self.config.filter_size_max {
                    if path.exists() {
                        if let Ok(metadata) = std::fs::metadata(path) {
                            if metadata.len() > max_size {
                                info!("[benchmark] skipping {} (size {} > {})",
                                    model_id, metadata.len(), max_size);
                                continue;
                            }
                        }
                    }
                }

                if let Some(ref pattern) = self.config.filter_name {
                    if let Ok(re) = Regex::new(pattern) {
                        if !re.is_match(&model_id) {
                            info!("[benchmark] skipping {} (name filter: {})",
                                model_id, pattern);
                            continue;
                        }
                    }
                }

                models.push(model_id);
            }
        }

        Ok(models)
    }

    pub async fn run(&self) -> Result<BenchmarkSuiteResult> {
        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client")?;
        let models = self.discover_models()
            .context("Failed to discover models")?;

        if models.is_empty() {
            anyhow::bail!("No models found for benchmarking");
        }

        info!("[benchmark] found {} models to benchmark", models.len());

        let mut results = Vec::new();

        for (i, model_id) in models.iter().enumerate() {
            info!("[benchmark] [{}/{}] loading {}", i+1, models.len(), model_id);

            let model_result = self.benchmark_single_model(&client, model_id).await;
            results.push(model_result);

            if i < models.len() - 1 {
                tokio::time::sleep(self.config.delay_between_swaps).await;
            }
        }

        let successful = results.iter().filter(|r| r.error.is_none()).count();
        let failed = results.len() - successful;
        let timestamp = format!("{:?}", std::time::SystemTime::now());

        Ok(BenchmarkSuiteResult {
            timestamp,
            server_url: self.config.server_url.clone(),
            total_models: results.len(),
            successful,
            failed,
            results,
        })
    }

    async fn benchmark_single_model(&self, client: &LlamaHttpClient, model_id: &str) -> ModelBenchmarkResult {
        let start = Instant::now();

        if let Ok(models) = client.list_models().await {
            for m in models {
                if m.status.value == "loaded" && m.id != model_id {
                    let _ = client.unload_model(&m.id).await;
                }
            }
        }

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
                    } else {
                        0.0
                    };
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

        let unload_start = Instant::now();
        let _ = client.unload_model(model_id).await;
        let unload_duration = unload_start.elapsed();

        let total_duration = start.elapsed();
        let completion_tokens_sum: usize = inference_results.iter()
            .map(|r| r.completion_tokens)
            .sum();
        let duration_sum: f64 = inference_results.iter()
            .map(|r| r.duration.as_secs_f64())
            .sum();
        let tps = if duration_sum > 0.0 {
            completion_tokens_sum as f64 / duration_sum
        } else {
            0.0
        };

        let mut latencies: Vec<f64> = inference_results.iter()
            .map(|r| r.duration.as_secs_f64() * 1000.0)
            .collect();
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let avg_ms = if latencies.is_empty() {
            0.0
        } else {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        };

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
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64) * p).min(sorted.len() as f64 - 1.0) as usize;
    sorted[idx]
}
