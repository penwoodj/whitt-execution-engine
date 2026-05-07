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
        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml_gpu_cpu_compare(
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

/// Generate benchmark YAML files with GPU/CPU comparison mode.
///
/// Creates 4 files:
/// - benchmark-3-models.yml (with specific model paths)
/// - benchmark-5-models.yml (with placeholder model_list)
/// - benchmark-15-models.yml (with placeholder model_list)
/// - benchmark-50-models.yml (with placeholder model_list)
#[allow(dead_code)]
pub fn generate_gpu_cpu_compare_benchmarks(output_dir: &Path) -> Result<()> {
    use crate::benchmark::yaml_generator::BenchmarkYamlGenerator;
    use std::path::PathBuf;

    let prompts = [
        "Explain concept of recursion in programming, with a practical example.",
        "Write a Rust function that finds the longest increasing subsequence in a vector.",
        "Analyze tradeoffs between microservices and monolithic architectures.",
    ];
    let max_tokens = 128;

    // Generate benchmark-3-models.yml with specific paths
    let models_3 = vec![
        ModelCandidate {
            path: PathBuf::from("/run/media/jon/data/models/lmstudio-community/Qwen3-4B-Instruct-2507-GGUF/Qwen3-4B-Instruct-2507-Q4_K_M.gguf"),
            model_id: "Qwen3-4B-Instruct-2507-Q4_K_M".to_string(),
            file_size_bytes: 0,
            estimated_vram_bytes: 0,
            fits_in_vram: true,
            author: "lmstudio-community".to_string(),
        },
        ModelCandidate {
            path: PathBuf::from("/run/media/jon/data/models/lmstudio-community/Phi-4-mini-reasoning-GGUF/Phi-4-mini-reasoning-Q4_K_M.gguf"),
            model_id: "Phi-4-mini-reasoning-Q4_K_M".to_string(),
            file_size_bytes: 0,
            estimated_vram_bytes: 0,
            fits_in_vram: true,
            author: "lmstudio-community".to_string(),
        },
        ModelCandidate {
            path: PathBuf::from("/run/media/jon/data/models/hugging-quants/Llama-3.2-1B-Instruct-Q8_0-GGUF/llama-3.2-1b-instruct-q8_0.gguf"),
            model_id: "Llama-3.2-1B-Instruct-Q8_0".to_string(),
            file_size_bytes: 0,
            estimated_vram_bytes: 0,
            fits_in_vram: true,
            author: "hugging-quants".to_string(),
        },
    ];

    let yaml_3 = BenchmarkYamlGenerator::generate_benchmark_yaml_gpu_cpu_compare(
        "benchmark-3-models",
        &models_3,
        &prompts,
        max_tokens,
    )?;

    let file_3 = output_dir.join("benchmark-3-models.yml");
    std::fs::write(&file_3, yaml_3)
        .with_context(|| format!("Failed to write {}", file_3.display()))?;
    info!("[generate_gpu_cpu] generated {}", file_3.display());

    // Generate benchmark-5-models.yml with placeholder list
    let models_5_placeholder: Vec<ModelCandidate> = (0..5).map(|i| ModelCandidate {
        path: PathBuf::from(format!("/path/to/model_{}.gguf", i + 1)),
        model_id: format!("model_{}", i + 1),
        file_size_bytes: 0,
        estimated_vram_bytes: 0,
        fits_in_vram: true,
        author: "placeholder".to_string(),
    }).collect();

    let yaml_5 = BenchmarkYamlGenerator::generate_benchmark_yaml_gpu_cpu_compare(
        "benchmark-5-models",
        &models_5_placeholder,
        &prompts,
        max_tokens,
    )?;

    let file_5 = output_dir.join("benchmark-5-models.yml");
    std::fs::write(&file_5, yaml_5)
        .with_context(|| format!("Failed to write {}", file_5.display()))?;
    info!("[generate_gpu_cpu] generated {}", file_5.display());

    // Generate benchmark-15-models.yml with placeholder list
    let models_15_placeholder: Vec<ModelCandidate> = (0..15).map(|i| ModelCandidate {
        path: PathBuf::from(format!("/path/to/model_{}.gguf", i + 1)),
        model_id: format!("model_{}", i + 1),
        file_size_bytes: 0,
        estimated_vram_bytes: 0,
        fits_in_vram: true,
        author: "placeholder".to_string(),
    }).collect();

    let yaml_15 = BenchmarkYamlGenerator::generate_benchmark_yaml_gpu_cpu_compare(
        "benchmark-15-models",
        &models_15_placeholder,
        &prompts,
        max_tokens,
    )?;

    let file_15 = output_dir.join("benchmark-15-models.yml");
    std::fs::write(&file_15, yaml_15)
        .with_context(|| format!("Failed to write {}", file_15.display()))?;
    info!("[generate_gpu_cpu] generated {}", file_15.display());

    // Generate benchmark-50-models.yml with placeholder list
    let models_50_placeholder: Vec<ModelCandidate> = (0..50).map(|i| ModelCandidate {
        path: PathBuf::from(format!("/path/to/model_{}.gguf", i + 1)),
        model_id: format!("model_{}", i + 1),
        file_size_bytes: 0,
        estimated_vram_bytes: 0,
        fits_in_vram: true,
        author: "placeholder".to_string(),
    }).collect();

    let yaml_50 = BenchmarkYamlGenerator::generate_benchmark_yaml_gpu_cpu_compare(
        "benchmark-50-models",
        &models_50_placeholder,
        &prompts,
        max_tokens,
    )?;

    let file_50 = output_dir.join("benchmark-50-models.yml");
    std::fs::write(&file_50, yaml_50)
        .with_context(|| format!("Failed to write {}", file_50.display()))?;
    info!("[generate_gpu_cpu] generated {}", file_50.display());

    Ok(())
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

    #[test]
    fn test_generate_gpu_cpu_compare_benchmarks() {
        let output_dir = std::path::PathBuf::from("/tmp/test-benchmarks");
        std::fs::create_dir_all(&output_dir).unwrap();

        let result = generate_gpu_cpu_compare_benchmarks(&output_dir);
        assert!(result.is_ok(), "Failed to generate benchmarks: {:?}", result.err());

        // Verify all 4 files exist
        let file_3 = output_dir.join("benchmark-3-models.yml");
        let file_5 = output_dir.join("benchmark-5-models.yml");
        let file_15 = output_dir.join("benchmark-15-models.yml");
        let file_50 = output_dir.join("benchmark-50-models.yml");

        assert!(file_3.exists(), "benchmark-3-models.yml not found");
        assert!(file_5.exists(), "benchmark-5-models.yml not found");
        assert!(file_15.exists(), "benchmark-15-models.yml not found");
        assert!(file_50.exists(), "benchmark-50-models.yml not found");

        let yaml_3 = std::fs::read_to_string(&file_3).unwrap();
        assert!(yaml_3.contains("workflow_id:"), "Missing workflow_id");
        assert!(yaml_3.contains("providers:"), "Missing providers");
        assert!(yaml_3.contains("llama_cpp_with_vulkan:"), "Missing llama_cpp_with_vulkan provider");
        assert!(yaml_3.contains("n_gpu_layers: 999"), "Missing GPU layers config");
        assert!(yaml_3.contains("benchmark_performance"), "Missing benchmark_performance step");
        assert!(yaml_3.contains("generate_speedup_report"), "Missing speedup report step");
        assert!(yaml_3.contains("comparison_mode: gpu_vs_cpu"), "Missing GPU vs CPU comparison mode");
        assert!(yaml_3.contains("refine_document"), "Missing refine_document step");
        assert!(yaml_3.contains("generate_report"), "Missing generate_report step");
        assert!(!yaml_3.contains("benchmark:"), "Must not contain non-schema 'benchmark:' key");
        assert!(!yaml_3.contains("model_list:"), "Must not contain non-schema 'model_list:' key");
        assert!(!yaml_3.contains("connection_settings:"), "Must not contain redundant connection_settings");

        let yaml_5 = std::fs::read_to_string(&file_5).unwrap();
        assert!(yaml_5.contains("max_iterations: 5"), "5-model should have max_iterations: 5");
        assert!(!yaml_5.contains("benchmark:"), "Must not contain non-schema 'benchmark:' key");

        let yaml_15 = std::fs::read_to_string(&file_15).unwrap();
        assert!(yaml_15.contains("max_iterations: 15"), "15-model should have max_iterations: 15");
        assert!(!yaml_15.contains("model_list:"), "Must not contain non-schema 'model_list:' key");

        let yaml_50 = std::fs::read_to_string(&file_50).unwrap();
        assert!(yaml_50.contains("max_iterations: 50"), "50-model should have max_iterations: 50");
        assert!(!yaml_50.contains("model_list:"), "Must not contain non-schema 'model_list:' key");

        // Cleanup
        std::fs::remove_dir_all(&output_dir).unwrap();
    }
}
