use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult};
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use std::process::Command;
use tokio::time::sleep;
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
    pub compare_gpu_cpu: bool,
    pub output_dir: Option<String>,
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Ensure output directories exist.
    fn ensure_output_dirs(&self) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let logs_dir = Path::new(output_dir).join("logs");
            let output_file_dir = Path::new(output_dir).join("output");

            fs::create_dir_all(&logs_dir)
                .with_context(|| format!("Failed to create logs directory: {}", logs_dir.display()))?;
            fs::create_dir_all(&output_file_dir)
                .with_context(|| format!("Failed to create output directory: {}", output_file_dir.display()))?;

            info!("[benchmark] output directories ready: {}", output_dir);
        }
        Ok(())
    }

    /// Log step start to benchmark.log.
    fn log_step_start(&self, model_id: &str, index: usize, total: usize, gpu_mode: &str) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let log_path = Path::new(output_dir).join("logs/benchmark.log");
            let timestamp = format!("{:?}", std::time::SystemTime::now());
            let line = format!(
                "{} [START] Model [{}/{}]: {} (mode: {})\n",
                timestamp, index + 1, total, model_id, gpu_mode
            );

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .with_context(|| format!("Failed to open log file: {}", log_path.display()))?;

            file.write_all(line.as_bytes())
                .context("Failed to write to benchmark.log")?;
        }
        Ok(())
    }

    /// Log step result to benchmark_results.yaml.
    fn log_step_result(&self, result: &ModelBenchmarkResult) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let yaml_path = Path::new(output_dir).join("output/benchmark_results.yaml");

            let yaml_fragment = format!(
                "- model_id: \"{}\"\n  tokens_per_second: {}\n  avg_latency_ms: {}\n  gpu_mode: \"{}\"\n  load_duration_ms: {}\n  total_duration_ms: {}\n  inference_count: {}\n",
                result.model_id,
                result.tokens_per_second,
                result.avg_latency_ms,
                result.gpu_mode,
                result.load_duration.as_millis(),
                result.total_duration.as_millis(),
                result.inference_results.len(),
            );

            if let Some(ref err) = result.error {
                let yaml_with_error = format!("{}  error: \"{}\"\n", yaml_fragment, err);
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&yaml_path)
                    .with_context(|| format!("Failed to open YAML file: {}", yaml_path.display()))?;

                file.write_all(yaml_with_error.as_bytes())
                    .context("Failed to write to benchmark_results.yaml")?;
            } else {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&yaml_path)
                    .with_context(|| format!("Failed to open YAML file: {}", yaml_path.display()))?;

                file.write_all(yaml_fragment.as_bytes())
                    .context("Failed to write to benchmark_results.yaml")?;
            }
        }
        Ok(())
    }

    /// Log step error to benchmark-errors.log.
    fn log_step_error(&self, model_id: &str, error: &str) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let error_log_path = Path::new(output_dir).join("logs/benchmark-errors.log");
            let timestamp = format!("{:?}", std::time::SystemTime::now());
            let line = format!("{} [ERROR] Model: {} - {}\n", timestamp, model_id, error);

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&error_log_path)
                .with_context(|| format!("Failed to open error log: {}", error_log_path.display()))?;

            file.write_all(line.as_bytes())
                .context("Failed to write to benchmark-errors.log")?;
        }
        Ok(())
    }

    /// Write final JSON report to benchmark_report.json.
    fn write_final_report(&self, suite_result: &BenchmarkSuiteResult) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let report_path = Path::new(output_dir).join("output/benchmark_report.json");
            let json_output = serde_json::to_string_pretty(suite_result)
                .context("Failed to serialize suite result to JSON")?;

            fs::write(&report_path, json_output)
                .with_context(|| format!("Failed to write benchmark report: {}", report_path.display()))?;

            info!("[benchmark] final report written to {}", report_path.display());
        }
        Ok(())
    }

    /// Restart Docker container with specific GPU layers setting.
    ///
    /// `n_gpu_layers`: 0 for CPU-only, high value (e.g., 999) for GPU mode
    async fn restart_docker_with_gpu_layers(&self, n_gpu_layers: u32) -> Result<()> {
        info!("[benchmark] restarting Docker with n-gpu-layers={}", n_gpu_layers);

        // Stop the container
        let stop_status = Command::new("docker")
            .args(["compose", "down"])
            .output()
            .context("Failed to stop Docker container")?;

        if !stop_status.status.success() {
            let stderr = String::from_utf8_lossy(&stop_status.stderr);
            anyhow::bail!("docker compose down failed: {}", stderr);
        }

        // Wait for container to stop
        sleep(Duration::from_secs(2)).await;

        // Start with GPU layers override
        let start_status = Command::new("docker")
            .args(["compose", "up", "-d"])
            .env("LLAMA_ARG_N_GPU_LAYERS", n_gpu_layers.to_string())
            .output()
            .context("Failed to start Docker container")?;

        if !start_status.status.success() {
            let stderr = String::from_utf8_lossy(&start_status.stderr);
            anyhow::bail!("docker compose up failed: {}", stderr);
        }

        // Wait for server to be ready
        info!("[benchmark] waiting for server to be ready...");
        let mut retries = 30;
        while retries > 0 {
            if reqwest::get(format!("{}/health", self.config.server_url)).await.is_ok() {
                info!("[benchmark] server is ready");
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
            retries -= 1;
        }

        anyhow::bail!("Server did not become ready after 30 seconds");
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
        self.ensure_output_dirs()
            .context("Failed to ensure output directories")?;

        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client")?;
        let models = self.discover_models()
            .context("Failed to discover models")?;

        if models.is_empty() {
            anyhow::bail!("No models found for benchmarking");
        }

        info!("[benchmark] found {} models to benchmark", models.len());
        info!("[benchmark] GPU/CPU comparison mode: {}", self.config.compare_gpu_cpu);

        let mut results = Vec::new();

        if self.config.compare_gpu_cpu {
            info!("[benchmark] Running in GPU/CPU comparison mode");

            for (i, model_id) in models.iter().enumerate() {
                self.log_step_start(model_id, i, models.len(), "gpu")?;

                info!("[benchmark] [{}/{}] loading {} (GPU mode)", i+1, models.len(), model_id);

                let gpu_result = self.benchmark_single_model(&client, model_id, "gpu").await;

                if let Some(ref err) = gpu_result.error {
                    self.log_step_error(model_id, err)?;
                }
                self.log_step_result(&gpu_result)?;

                if gpu_result.error.is_none() {
                    self.log_step_start(model_id, i, models.len(), "cpu")?;

                    info!("[benchmark] [{}/{}] loading {} (CPU mode)", i+1, models.len(), model_id);

                    self.restart_docker_with_gpu_layers(0).await
                        .context("Failed to restart Docker in CPU mode")?;

                    let cpu_result = self.benchmark_single_model(&client, model_id, "cpu").await;

                    if let Some(ref err) = cpu_result.error {
                        self.log_step_error(model_id, err)?;
                    }
                    self.log_step_result(&cpu_result)?;

                    if cpu_result.error.is_none() {
                        let speedup = cpu_result.tokens_per_second / gpu_result.tokens_per_second;
                        info!("[benchmark] {} speedup factor: {:.2}x", model_id, speedup);

                        let mut gpu_result_with_speedup = gpu_result.clone();
                        gpu_result_with_speedup.speedup_factor = Some(speedup);
                        results.push(gpu_result_with_speedup);

                        let mut cpu_result_with_speedup = cpu_result.clone();
                        cpu_result_with_speedup.speedup_factor = Some(speedup);
                        results.push(cpu_result_with_speedup);
                    } else {
                        results.push(gpu_result);
                        results.push(cpu_result);
                    }

                    self.restart_docker_with_gpu_layers(999).await
                        .context("Failed to restart Docker in GPU mode")?;
                } else {
                    results.push(gpu_result);
                }

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
            }
        } else {
            for (i, model_id) in models.iter().enumerate() {
                self.log_step_start(model_id, i, models.len(), "gpu")?;

                info!("[benchmark] [{}/{}] loading {}", i+1, models.len(), model_id);

                let model_result = self.benchmark_single_model(&client, model_id, "gpu").await;

                if let Some(ref err) = model_result.error {
                    self.log_step_error(model_id, err)?;
                }
                self.log_step_result(&model_result)?;

                results.push(model_result);

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
            }
        }

        let successful = results.iter().filter(|r| r.error.is_none()).count();
        let failed = results.len() - successful;
        let timestamp = format!("{:?}", std::time::SystemTime::now());

        let suite_result = BenchmarkSuiteResult {
            timestamp,
            server_url: self.config.server_url.clone(),
            total_models: results.len(),
            successful,
            failed,
            results,
        };

        self.write_final_report(&suite_result)?;

        Ok(suite_result)
    }

    async fn benchmark_single_model(&self, client: &LlamaHttpClient, model_id: &str, gpu_mode: &str) -> ModelBenchmarkResult {
        let server_model_id = model_id.strip_suffix(".gguf").unwrap_or(model_id);
        let start = Instant::now();

        if let Ok(models) = client.list_models().await {
            for m in models {
                if m.status.value == "loaded" && m.id != server_model_id {
                    let _ = client.unload_model(&m.id).await;
                }
            }
        }

        let load_start = Instant::now();
        let load_result = client.load_model(server_model_id).await;
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
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
            };
        }

        let mut inference_results = Vec::new();

        for prompt in &self.config.prompts {
            let request = ChatCompletionRequest {
                model: server_model_id.to_string(),
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
        let _ = client.unload_model(server_model_id).await;
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
            gpu_mode: gpu_mode.to_string(),
            speedup_factor: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_benchmark_config_with_compare_gpu_cpu() {
        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: Some("/models".to_string()),
            model_list_file: None,
            prompts: vec!["test prompt".to_string()],
            max_tokens: 100,
            filter_size_max: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(2),
            compare_gpu_cpu: true,
            output_dir: None,
        };

        assert!(config.compare_gpu_cpu, "compare_gpu_cpu should be true");
    }

    #[test]
    fn test_model_benchmark_result_with_gpu_mode_fields() {
        let result = ModelBenchmarkResult {
            model_id: "test-model.gguf".to_string(),
            model_path: "/models/test-model.gguf".to_string(),
            file_size_bytes: 1_000_000_000,
            load_duration: Duration::from_secs(5),
            inference_results: vec![],
            unload_duration: Duration::from_secs(1),
            total_duration: Duration::from_secs(6),
            tokens_per_second: 100.0,
            avg_latency_ms: 500.0,
            p50_latency_ms: 450.0,
            p95_latency_ms: 550.0,
            p99_latency_ms: 600.0,
            error: None,
            gpu_mode: "gpu".to_string(),
            speedup_factor: Some(2.5),
        };

        assert_eq!(result.gpu_mode, "gpu", "gpu_mode should be 'gpu'");
        assert_eq!(result.speedup_factor, Some(2.5), "speedup_factor should be Some(2.5)");
    }

    #[test]
    fn test_csv_output_includes_gpu_mode_and_speedup() {
        let gpu_result = ModelBenchmarkResult {
            model_id: "test-model.gguf".to_string(),
            model_path: "/models/test-model.gguf".to_string(),
            file_size_bytes: 1_000_000_000,
            load_duration: Duration::from_secs(5),
            inference_results: vec![],
            unload_duration: Duration::from_secs(1),
            total_duration: Duration::from_secs(6),
            tokens_per_second: 100.0,
            avg_latency_ms: 500.0,
            p50_latency_ms: 450.0,
            p95_latency_ms: 550.0,
            p99_latency_ms: 600.0,
            error: None,
            gpu_mode: "gpu".to_string(),
            speedup_factor: Some(2.5),
        };

        let suite_result = BenchmarkSuiteResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            server_url: "http://localhost:8080".to_string(),
            total_models: 1,
            successful: 1,
            failed: 0,
            results: vec![gpu_result],
        };

        let csv = suite_result.to_csv();
        assert!(csv.contains("gpu_mode"), "CSV should contain 'gpu_mode' column");
        assert!(csv.contains("speedup_factor"), "CSV should contain 'speedup_factor' column");
        assert!(csv.contains("gpu"), "CSV should contain 'gpu' value");
        assert!(csv.contains("2.50"), "CSV should contain '2.50' speedup value");
    }

    #[test]
    fn test_table_output_includes_gpu_mode_and_speedup() {
        let gpu_result = ModelBenchmarkResult {
            model_id: "test-model.gguf".to_string(),
            model_path: "/models/test-model.gguf".to_string(),
            file_size_bytes: 1_000_000_000,
            load_duration: Duration::from_secs(5),
            inference_results: vec![],
            unload_duration: Duration::from_secs(1),
            total_duration: Duration::from_secs(6),
            tokens_per_second: 100.0,
            avg_latency_ms: 500.0,
            p50_latency_ms: 450.0,
            p95_latency_ms: 550.0,
            p99_latency_ms: 600.0,
            error: None,
            gpu_mode: "gpu".to_string(),
            speedup_factor: Some(2.5),
        };

        let suite_result = BenchmarkSuiteResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            server_url: "http://localhost:8080".to_string(),
            total_models: 1,
            successful: 1,
            failed: 0,
            results: vec![gpu_result],
        };

        let table = suite_result.to_table();
        assert!(table.contains("GPU"), "Table should contain 'GPU' column header");
        assert!(table.contains("Speedup"), "Table should contain 'Speedup' column header");
        assert!(table.contains("GPU Mode:"), "Table should contain 'GPU Mode:' label");
        assert!(table.contains("Speedup Factor:"), "Table should contain 'Speedup Factor:' label");
        assert!(table.contains("2.50x"), "Table should contain '2.50x' speedup value");
    }
}
