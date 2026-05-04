//! Benchmark module for multi-model performance testing.

pub mod runner;
pub mod model_selector;
pub mod yaml_generator;

use crate::client::model_discovery::ModelCandidate;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;

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
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("model_id,model_path,file_size_bytes,load_duration_ms,inference_count,");
        csv.push_str("unload_duration_ms,total_duration_ms,tokens_per_second,");
        csv.push_str("avg_latency_ms,p50_latency_ms,p95_latency_ms,p99_latency_ms,error\n");

        for result in &self.results {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                result.model_id,
                result.model_path,
                result.file_size_bytes,
                result.load_duration.as_millis(),
                result.inference_results.len(),
                result.unload_duration.as_millis(),
                result.total_duration.as_millis(),
                result.tokens_per_second,
                result.avg_latency_ms,
                result.p50_latency_ms,
                result.p95_latency_ms,
                result.p99_latency_ms,
                result.error.as_deref().unwrap_or("")
            ));
        }

        csv
    }

    pub fn to_table(&self) -> String {
        let mut table = String::new();
        table.push_str("Benchmark Suite Results\n");
        table.push_str(&"=".repeat(80));
        table.push_str("\n\n");

        table.push_str(&format!("Timestamp: {}\n", self.timestamp));
        table.push_str(&format!("Server URL: {}\n", self.server_url));
        table.push_str(&format!("Total Models: {}\n", self.total_models));
        table.push_str(&format!("Successful: {}\n", self.successful));
        table.push_str(&format!("Failed: {}\n", self.failed));
        table.push('\n');

        table.push_str(&format!("{:<60} {:>10} {:>12} {:>10}\n", "Model", "Size (MB)", "Tokens/s", "Avg (ms)"));
        table.push_str(&"-".repeat(96));
        table.push('\n');

        for result in &self.results {
            let size_mb = result.file_size_bytes as f64 / (1024.0 * 1024.0);
            let tps = if result.error.is_some() { "-" } else { &format!("{:.2}", result.tokens_per_second) };
            let avg_ms = if result.error.is_some() { "-" } else { &format!("{:.2}", result.avg_latency_ms) };
            let error_marker = if result.error.is_some() { " [FAILED]" } else { "" };

            table.push_str(&format!(
                "{:<60}{:>10.1} {:>12} {:>10}{}\n",
                format!("{}{}", result.model_id, error_marker),
                size_mb,
                tps,
                avg_ms,
                ""
            ));
        }

        table.push('\n');

        for result in &self.results {
            if let Some(err) = &result.error {
                table.push_str(&format!("{}: {}\n", result.model_id, err));
                continue;
            }

            table.push_str(&format!("\n{}\n", result.model_id));
            table.push_str(&"-".repeat(80));
            table.push('\n');
            table.push_str(&format!("File Size: {:.2} MB\n", result.file_size_bytes as f64 / (1024.0 * 1024.0)));
            table.push_str(&format!("Load Duration: {:.2}s\n", result.load_duration.as_secs_f64()));
            table.push_str(&format!("Unload Duration: {:.2}s\n", result.unload_duration.as_secs_f64()));
            table.push_str(&format!("Total Duration: {:.2}s\n", result.total_duration.as_secs_f64()));
            table.push_str(&format!("Tokens/Second: {:.2}\n", result.tokens_per_second));
            table.push_str(&format!("Avg Latency: {:.2}ms\n", result.avg_latency_ms));
            table.push_str(&format!("P50 Latency: {:.2}ms\n", result.p50_latency_ms));
            table.push_str(&format!("P95 Latency: {:.2}ms\n", result.p95_latency_ms));
            table.push_str(&format!("P99 Latency: {:.2}ms\n", result.p99_latency_ms));
            table.push_str(&format!("Inferences: {}\n", result.inference_results.len()));

            if !result.inference_results.is_empty() {
                table.push_str("\nInference Results:\n");
                table.push_str(&format!("{:<4} {:>10} {:>10} {:>12} {:>12}\n", "#", "Tokens", "Time (ms)", "Tokens/s", "Prompt Preview"));
                table.push_str(&"-".repeat(64));
                table.push('\n');

                for (i, inf) in result.inference_results.iter().enumerate() {
                    let preview = if inf.prompt.len() > 30 {
                        format!("{}...", &inf.prompt[..30])
                    } else {
                        inf.prompt.clone()
                    };
                    table.push_str(&format!(
                        "{:<4} {:>10} {:>10.2} {:>12.2} {:>12}\n",
                        i + 1,
                        inf.total_tokens,
                        inf.duration.as_millis(),
                        inf.tokens_per_second,
                        preview
                    ));
                }
            }
        }

        table
    }
}

/// Generate benchmark suite YAML files for 3, 5, 15, and 50 models.
///
/// Creates files in output directory:
/// - benchmark-5-models.yml
/// - benchmark-15-models.yml
/// - benchmark-50-models.yml
///
/// Uses tier-diverse model selection from model_selector module.
pub fn generate_suite_files(
    output_dir: &Path,
    candidates: &[ModelCandidate],
) -> Result<Vec<PathBuf>> {
    use crate::benchmark::model_selector::ModelSelector;
    use crate::benchmark::yaml_generator::BenchmarkYamlGenerator;

    let prompts = [
        "Explain the concept of recursion in programming, with a practical example.",
        "Write a Rust function that finds the longest increasing subsequence in a vector.",
        "Analyze the tradeoffs between microservices and monolithic architectures.",
    ];

    let max_tokens = 128;

    let sizes = [3, 5, 15, 50];
    let mut generated_files = Vec::new();

    for n in sizes {
        if n > candidates.len() {
            info!("[generate_suite] skipping {} models (only {} candidates available)", n, candidates.len());
            continue;
        }

        let selected = ModelSelector::select_n_models(candidates, n);
        let prompt_refs: Vec<&str> = prompts.iter().map(|s| s.as_ref()).collect();
        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml_with_models(
            &format!("benchmark-{}-models", n),
            &selected,
            &prompt_refs,
            max_tokens,
        )?;

        let file_path = output_dir.join(format!("benchmark-{}-models.yml", n));
        std::fs::write(&file_path, yaml)
            .with_context(|| format!("Failed to write benchmark YAML to {}", file_path.display()))?;

        info!("[generate_suite] generated {}", file_path.display());
        generated_files.push(file_path);
    }

    Ok(generated_files)
}

