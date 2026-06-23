//! Per-model markdown detail file generator for benchmark results.

use std::fs;
use std::path::Path;
use crate::benchmark::ModelBenchmarkResult;
use anyhow::{Context, Result};

pub struct DetailGenerator {
    output_dir: String,
    include_chat_history: bool,
    include_metrics: bool,
}

impl DetailGenerator {
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            include_chat_history: true,
            include_metrics: true,
        }
    }

    pub fn generate_model_detail(&self, result: &ModelBenchmarkResult) -> Result<String> {
        let mut md = String::new();

        // Header
        md.push_str(&format!("# Model Detail: {}\n\n", result.model_id));

        // Summary table
        md.push_str("## Summary\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Model ID | {} |\n", result.model_id));
        md.push_str(&format!("| Model Path | {} |\n", result.model_path));
        md.push_str(&format!("| File Size | {:.2} MB |\n", result.file_size_bytes as f64 / (1024.0 * 1024.0)));
        md.push_str(&format!("| Total Prompts | {} |\n", result.inference_results.len()));
        md.push_str(&format!("| Successful | {} |\n", result.inference_results.len()));
        md.push_str("| Failed | 0 |\n");
        md.push_str(&format!("| Load Duration | {:.2}s |\n", result.load_duration.as_secs_f64()));
        md.push_str(&format!("| Unload Duration | {:.2}s |\n", result.unload_duration.as_secs_f64()));
        md.push_str(&format!("| Total Duration | {:.2}s |\n", result.total_duration.as_secs_f64()));
        md.push_str(&format!("| Tokens/Second | {:.2} |\n", result.tokens_per_second));
        md.push_str(&format!("| Avg Latency | {:.2}ms |\n", result.avg_latency_ms));
        md.push_str(&format!("| P50 Latency | {:.2}ms |\n", result.p50_latency_ms));
        md.push_str(&format!("| P95 Latency | {:.2}ms |\n", result.p95_latency_ms));
        md.push_str(&format!("| P99 Latency | {:.2}ms |\n", result.p99_latency_ms));
        md.push_str(&format!("| GPU Mode | {} |\n", result.gpu_mode));
        if let Some(speedup) = result.speedup_factor {
            md.push_str(&format!("| Speedup Factor | {:.2}x |\n", speedup));
        }

        // Error if present
        if let Some(ref err) = result.error {
            md.push_str(&format!("| Error | {} |\n", err));
        }

        // Per-prompt results
        md.push_str("\n## Prompt Results\n\n");

        if result.inference_results.is_empty() {
            md.push_str("No inference results available.\n");
        } else {
            for (i, inference) in result.inference_results.iter().enumerate() {
                md.push_str(&format!("### Prompt {}\n\n", i + 1));

                // Input preview
                let preview_len = 100.min(inference.prompt.len());
                md.push_str(&format!("- **Input**: {}{}\n",
                    &inference.prompt[..preview_len],
                    if inference.prompt.len() > 100 { "..." } else { "" }));

                // Timing and tokens
                md.push_str(&format!("- **Duration**: {:.2}s\n", inference.duration.as_secs_f64()));
                md.push_str(&format!("- **Tokens**: {} prompt + {} completion = {} total\n",
                    inference.prompt_tokens,
                    inference.completion_tokens,
                    inference.total_tokens));
                md.push_str(&format!("- **Speed**: {:.2} tokens/sec\n", inference.tokens_per_second));

                // Output preview
                let output_preview_len = 200.min(inference.response_text.len());
                md.push_str(&format!("- **Output Preview**: {}{}\n\n",
                    &inference.response_text[..output_preview_len],
                    if inference.response_text.len() > 200 { "..." } else { "" }));

                // Full output section
                if self.include_chat_history {
                    md.push_str("#### Full Response\n\n");
                    md.push_str("```\n");
                    md.push_str(&inference.response_text);
                    md.push_str("\n```\n\n");
                }
            }
        }

        Ok(md)
    }

    pub fn write_model_detail(&self, result: &ModelBenchmarkResult) -> Result<()> {
        let content = self.generate_model_detail(result)?;
        let safe_name = result.model_id
            .replace(['/', '\\'], "_")
            .replace(".gguf", "")
            .replace('.', "_");
        let path = Path::new(&self.output_dir).join(format!("detail-{}.md", safe_name));

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
        }

        fs::write(&path, content)
            .with_context(|| format!("Failed to write detail file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark::InferenceResult;
    use std::time::Duration;

    fn make_test_inference_result() -> InferenceResult {
        InferenceResult {
            prompt: "What is the capital of France?".to_string(),
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
            duration: Duration::from_millis(500),
            tokens_per_second: 60.0,
            response_text: "The capital of France is Paris.".to_string(),
        }
    }

    fn make_test_model_result() -> ModelBenchmarkResult {
        ModelBenchmarkResult {
            model_id: "test-model-q4_k_m.gguf".to_string(),
            model_path: "/models/test-model-q4_k_m.gguf".to_string(),
            file_size_bytes: 2_000_000_000,
            load_duration: Duration::from_secs(5),
            inference_results: vec![
                make_test_inference_result(),
                InferenceResult {
                    prompt: "Explain quantum computing.".to_string(),
                    prompt_tokens: 15,
                    completion_tokens: 50,
                    total_tokens: 65,
                    duration: Duration::from_millis(1000),
                    tokens_per_second: 65.0,
                    response_text: "Quantum computing uses quantum bits...".to_string(),
                },
            ],
            unload_duration: Duration::from_secs(2),
            total_duration: Duration::from_secs(10),
            tokens_per_second: 62.5,
            avg_latency_ms: 750.0,
            p50_latency_ms: 700.0,
            p95_latency_ms: 900.0,
            p99_latency_ms: 1000.0,
            error: None,
            gpu_mode: "gpu".to_string(),
            speedup_factor: Some(2.5),
        }
    }

    #[test]
    fn given_model_result_when_detail_generated_then_contains_header() {
        let result = make_test_model_result();
        let gen = DetailGenerator::new("/tmp");
        let md = gen.generate_model_detail(&result).unwrap();

        assert!(md.contains("# Model Detail: test-model-q4_k_m.gguf"));
    }

    #[test]
    fn given_model_result_when_detail_generated_then_contains_summary_table() {
        let result = make_test_model_result();
        let gen = DetailGenerator::new("/tmp");
        let md = gen.generate_model_detail(&result).unwrap();

        assert!(md.contains("## Summary"));
        assert!(md.contains("| Metric | Value |"));
        assert!(md.contains("| Model ID | test-model-q4_k_m.gguf |"));
        assert!(md.contains("| Total Prompts | 2 |"));
        assert!(md.contains("| Successful | 2 |"));
        assert!(md.contains("| Failed | 0 |"));
        assert!(md.contains("| File Size | 1907.35 MB |")); // 2GB
        assert!(md.contains("| Tokens/Second | 62.50 |"));
        assert!(md.contains("| GPU Mode | gpu |"));
        assert!(md.contains("| Speedup Factor | 2.50x |"));
    }

    #[test]
    fn given_model_result_when_detail_generated_then_contains_prompt_results() {
        let result = make_test_model_result();
        let gen = DetailGenerator::new("/tmp");
        let md = gen.generate_model_detail(&result).unwrap();

        assert!(md.contains("## Prompt Results"));
        assert!(md.contains("### Prompt 1"));
        assert!(md.contains("### Prompt 2"));
        assert!(md.contains("What is the capital of France?"));
        assert!(md.contains("Explain quantum computing."));
        assert!(md.contains("The capital of France is Paris."));
        assert!(md.contains("Quantum computing uses quantum bits..."));
    }

    #[test]
    fn given_model_result_with_error_when_detail_generated_then_contains_error() {
        let mut result = make_test_model_result();
        result.error = Some("Model failed to load".to_string());
        result.inference_results.clear();

        let gen = DetailGenerator::new("/tmp");
        let md = gen.generate_model_detail(&result).unwrap();

        assert!(md.contains("| Error | Model failed to load |"));
        assert!(md.contains("No inference results available."));
    }

    #[test]
    fn given_model_result_when_written_then_file_exists() {
        let result = make_test_model_result();
        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().to_str().unwrap();
        let gen = DetailGenerator::new(output_path);

        gen.write_model_detail(&result).unwrap();

        let expected_file = output_dir.path().join("detail-test-model-q4_k_m.md");
        assert!(expected_file.exists());
    }

    #[test]
    fn given_multiple_results_when_written_then_separate_files() {
        let result1 = make_test_model_result();
        let mut result2 = make_test_model_result();
        result2.model_id = "another-model-q8_0.gguf".to_string();
        result2.model_path = "/models/another-model-q8_0.gguf".to_string();

        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().to_str().unwrap();
        let gen = DetailGenerator::new(output_path);

        gen.write_model_detail(&result1).unwrap();
        gen.write_model_detail(&result2).unwrap();

        let file1 = output_dir.path().join("detail-test-model-q4_k_m.md");
        let file2 = output_dir.path().join("detail-another-model-q8_0.md");

        assert!(file1.exists());
        assert!(file2.exists());

        let content1 = fs::read_to_string(&file1).unwrap();
        let content2 = fs::read_to_string(&file2).unwrap();

        assert!(content1.contains("test-model-q4_k_m.gguf"));
        assert!(content2.contains("another-model-q8_0.gguf"));
        assert!(!content1.contains("another-model-q8_0.gguf"));
        assert!(!content2.contains("test-model-q4_k_m.gguf"));
    }

    #[test]
    fn given_long_prompt_when_detail_generated_then_truncates_preview() {
        let mut result = make_test_model_result();
        result.inference_results[0].prompt = "a".repeat(200);
        result.inference_results[0].response_text = "b".repeat(400);

        let gen = DetailGenerator::new("/tmp");
        let md = gen.generate_model_detail(&result).unwrap();

        // Should contain truncated preview with ...
        assert!(md.contains("- **Input**:"));
        assert!(md.contains("..."));

        // Full response should still be in the code block
        assert!(md.contains("#### Full Response"));
        assert!(md.contains("```"));
    }

    #[test]
    fn given_model_result_with_special_chars_when_written_then_sanitizes_filename() {
        let mut result = make_test_model_result();
        result.model_id = "model/with\\special.chars.gguf".to_string();

        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().to_str().unwrap();
        let gen = DetailGenerator::new(output_path);

        gen.write_model_detail(&result).unwrap();

        let file = output_dir.path().join("detail-model_with_special_chars.md");
        assert!(file.exists());
    }
}