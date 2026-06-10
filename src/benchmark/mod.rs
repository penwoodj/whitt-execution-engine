//! Benchmark module for multi-model performance testing.

pub mod runner;
pub mod model_selector;
pub mod error_types;
pub mod circuit_breaker;
pub mod detail_generator;

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
    pub response_text: String,
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
    pub gpu_mode: String,
    pub speedup_factor: Option<f64>,
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

/// Result of executing a single workflow step, including routing information.
#[derive(Debug, Clone)]
pub struct WorkflowStepResult {
    /// The benchmark result from model inference
    pub benchmark_result: ModelBenchmarkResult,
    /// Routing directive from hooks (step IDs to jump to next)
    pub route_to: Option<Vec<String>>,
    /// Whether to skip all remaining steps
    pub skip_remaining: bool,
    /// Whether to skip the current loop iteration (continue to next iteration)
    pub skip_loop: bool,
}

impl BenchmarkSuiteResult {
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("model_id,model_path,file_size_bytes,load_duration_ms,inference_count,");
        csv.push_str("unload_duration_ms,total_duration_ms,tokens_per_second,");
        csv.push_str("avg_latency_ms,p50_latency_ms,p95_latency_ms,p99_latency_ms,");
        csv.push_str("gpu_mode,speedup_factor,error\n");

        for result in &self.results {
            let speedup = result.speedup_factor.map(|f| format!("{:.2}", f)).unwrap_or_else(|| "-".to_string());
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
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
                result.gpu_mode,
                speedup,
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

        table.push_str(&format!("{:<60} {:>10} {:>12} {:>10} {:>8} {:>10}\n", "Model", "Size (MB)", "Tokens/s", "Avg (ms)", "GPU", "Speedup"));
        table.push_str(&"-".repeat(116));
        table.push('\n');

        for result in &self.results {
            let size_mb = result.file_size_bytes as f64 / (1024.0 * 1024.0);
            let tps = if result.error.is_some() { "-" } else { &format!("{:.2}", result.tokens_per_second) };
            let avg_ms = if result.error.is_some() { "-" } else { &format!("{:.2}", result.avg_latency_ms) };
            let gpu_mode = if result.error.is_some() { "-" } else { &result.gpu_mode };
            let speedup = result.speedup_factor.map(|f| format!("{:.2}x", f)).unwrap_or_else(|| "-".to_string());
            let error_marker = if result.error.is_some() { " [FAILED]" } else { "" };

            table.push_str(&format!(
                "{:<60}{:>10.1} {:>12} {:>10} {:>8} {:>10}{}\n",
                format!("{}{}", result.model_id, error_marker),
                size_mb,
                tps,
                avg_ms,
                gpu_mode,
                speedup,
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
            table.push_str(&format!("GPU Mode: {}\n", result.gpu_mode));
            table.push_str(&format!("Load Duration: {:.2}s\n", result.load_duration.as_secs_f64()));
            table.push_str(&format!("Unload Duration: {:.2}s\n", result.unload_duration.as_secs_f64()));
            table.push_str(&format!("Total Duration: {:.2}s\n", result.total_duration.as_secs_f64()));
            table.push_str(&format!("Tokens/Second: {:.2}\n", result.tokens_per_second));
            if let Some(speedup) = result.speedup_factor {
                table.push_str(&format!("Speedup Factor: {:.2}x\n", speedup));
            }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result_to_csv() {
        let result = ModelBenchmarkResult {
            model_id: "test-model".to_string(),
            model_path: "/path/to/model.gguf".to_string(),
            file_size_bytes: 1_000_000_000,
            load_duration: Duration::from_secs(5),
            inference_results: vec![],
            unload_duration: Duration::from_secs(2),
            total_duration: Duration::from_secs(10),
            tokens_per_second: 25.5,
            avg_latency_ms: 100.0,
            p50_latency_ms: 95.0,
            p95_latency_ms: 150.0,
            p99_latency_ms: 200.0,
            error: None,
            gpu_mode: "gpu".to_string(),
            speedup_factor: Some(2.5),
        };

        let suite = BenchmarkSuiteResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            server_url: "http://localhost:8080".to_string(),
            total_models: 1,
            successful: 1,
            failed: 0,
            results: vec![result],
        };

        let csv = suite.to_csv();
        assert!(csv.contains("test-model"));
        assert!(csv.contains("1000000000")); // 1GB in bytes
        assert!(csv.contains("2.50"));
    }

}
