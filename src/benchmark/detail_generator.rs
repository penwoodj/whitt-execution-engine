//! Per-model markdown detail file generator for benchmark results.
//!
//! Template-driven: when `detail_template` is None, uses `DEFAULT_DETAIL_TEMPLATE`
//! (preserves original behavior). When Some, applies user-supplied template from
//! workflow YAML at `workspace.detail_template`.
//!
//! Supported template variables (scalar):
//!   {{model_id}}, {{model_path}}, {{file_size_bytes}}, {{file_size_mb}},
//!   {{load_duration_secs}}, {{unload_duration_secs}}, {{total_duration_secs}},
//!   {{tokens_per_second}}, {{avg_latency_ms}}, {{p50_latency_ms}},
//!   {{p95_latency_ms}}, {{p99_latency_ms}}, {{gpu_mode}},
//!   {{speedup_factor}} (e.g. "2.50x" or ""), {{error}} (message or ""),
//!   {{inference_count}}
//!
//! Loop block (optional):
//!   {{#each inference}}...{{/each}}
//!   Inside: {{prompt_preview}}, {{response_preview}}, {{duration_secs}},
//!   {{prompt_tokens}}, {{completion_tokens}}, {{total_tokens}},
//!   {{tokens_per_second}}

use std::fs;
use std::path::Path;
use crate::benchmark::{InferenceResult, ModelBenchmarkResult};
use anyhow::{Context, Result};

pub const DEFAULT_DETAIL_TEMPLATE: &str = "# Model Detail: {{model_id}}\n\n\
## Summary\n\n\
| Metric | Value |\n\
|--------|-------|\n\
| Model ID | {{model_id}} |\n\
| Model Path | {{model_path}} |\n\
| File Size | {{file_size_mb}} MB |\n\
| Total Prompts | {{inference_count}} |\n\
| Successful | {{inference_count}} |\n\
| Failed | 0 |\n\
| Load Duration | {{load_duration_secs}}s |\n\
| Unload Duration | {{unload_duration_secs}}s |\n\
| Total Duration | {{total_duration_secs}}s |\n\
| Tokens/Second | {{tokens_per_second}} |\n\
| Avg Latency | {{avg_latency_ms}}ms |\n\
| P50 Latency | {{p50_latency_ms}}ms |\n\
| P95 Latency | {{p95_latency_ms}}ms |\n\
| P99 Latency | {{p99_latency_ms}}ms |\n\
| GPU Mode | {{gpu_mode}} |\n\
{{speedup_row}}\
{{error_row}}\n\
## Prompt Results\n\n\
{{#each inference}}\
### Prompt {{index}}\n\n\
- **Input**: {{prompt_preview}}\n\
- **Duration**: {{duration_secs}}s\n\
- **Tokens**: {{prompt_tokens}} prompt + {{completion_tokens}} completion = {{total_tokens}} total\n\
- **Speed**: {{tokens_per_second}} tokens/sec\n\
- **Output Preview**: {{response_preview}}\n\n\
#### Full Response\n\n\
```\n\
{{response_full}}\n\
```\n\n\
{{/each}}";

pub struct DetailGenerator {
    output_dir: String,
    template: Option<String>,
}

impl DetailGenerator {
    pub fn new(output_dir: &str, template: Option<String>) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            template,
        }
    }

    pub fn generate_model_detail(&self, result: &ModelBenchmarkResult) -> Result<String> {
        let template = self.template.as_deref().unwrap_or(DEFAULT_DETAIL_TEMPLATE);
        Ok(render_detail(template, result))
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

fn fmt2(v: f64) -> String { format!("{:.2}", v) }

fn substitute_scalars(template: &str, result: &ModelBenchmarkResult) -> String {
    let mut out = template
        .replace("{{model_id}}", &result.model_id)
        .replace("{{model_path}}", &result.model_path)
        .replace("{{file_size_bytes}}", &result.file_size_bytes.to_string())
        .replace("{{file_size_mb}}", &fmt2(result.file_size_bytes as f64 / (1024.0 * 1024.0)))
        .replace("{{load_duration_secs}}", &fmt2(result.load_duration.as_secs_f64()))
        .replace("{{unload_duration_secs}}", &fmt2(result.unload_duration.as_secs_f64()))
        .replace("{{total_duration_secs}}", &fmt2(result.total_duration.as_secs_f64()))
        .replace("{{tokens_per_second}}", &fmt2(result.tokens_per_second))
        .replace("{{avg_latency_ms}}", &fmt2(result.avg_latency_ms))
        .replace("{{p50_latency_ms}}", &fmt2(result.p50_latency_ms))
        .replace("{{p95_latency_ms}}", &fmt2(result.p95_latency_ms))
        .replace("{{p99_latency_ms}}", &fmt2(result.p99_latency_ms))
        .replace("{{gpu_mode}}", &result.gpu_mode)
        .replace("{{inference_count}}", &result.inference_results.len().to_string())
        .replace(
            "{{speedup_factor}}",
            &result.speedup_factor.map(|s| format!("{}x", fmt2(s))).unwrap_or_default(),
        )
        .replace("{{error}}", result.error.as_deref().unwrap_or(""));

    let speedup_row = match result.speedup_factor {
        Some(s) => format!("| Speedup Factor | {:.2}x |\n", s),
        None => String::new(),
    };
    out = out.replace("{{speedup_row}}", &speedup_row);

    let error_row = match result.error {
        Some(ref e) => format!("| Error | {} |\n", e),
        None => String::new(),
    };
    out = out.replace("{{error_row}}", &error_row);

    out
}

fn render_inference_block(body: &str, inference: &InferenceResult, index: usize) -> String {
    let prompt_preview = truncate_preview(&inference.prompt, 100);
    let response_preview = truncate_preview(&inference.response_text, 200);
    body.replace("{{index}}", &(index + 1).to_string())
        .replace("{{prompt_preview}}", &prompt_preview)
        .replace("{{response_preview}}", &response_preview)
        .replace("{{response_full}}", &inference.response_text)
        .replace("{{duration_secs}}", &fmt2(inference.duration.as_secs_f64()))
        .replace("{{prompt_tokens}}", &inference.prompt_tokens.to_string())
        .replace("{{completion_tokens}}", &inference.completion_tokens.to_string())
        .replace("{{total_tokens}}", &inference.total_tokens.to_string())
        .replace("{{tokens_per_second}}", &fmt2(inference.tokens_per_second))
}

fn truncate_preview(s: &str, max: usize) -> String {
    let len = max.min(s.len());
    let suffix = if s.len() > max { "..." } else { "" };
    format!("{}{}", &s[..len], suffix)
}

pub fn render_detail(template: &str, result: &ModelBenchmarkResult) -> String {
    if let (Some(start), Some(end)) = (template.find("{{#each inference}}"), template.find("{{/each}}")) {
        let prefix = &template[..start];
        let body = &template[start + "{{#each inference}}".len()..end];
        let suffix = &template[end + "{{/each}}".len()..];

        let prefix_rendered = substitute_scalars(prefix, result);
        let suffix_rendered = substitute_scalars(suffix, result);

        let mut body_rendered = String::new();
        if result.inference_results.is_empty() {
            body_rendered.push_str("No inference results available.\n");
        } else {
            for (i, inf) in result.inference_results.iter().enumerate() {
                body_rendered.push_str(&render_inference_block(body, inf, i));
            }
        }

        let mut out = prefix_rendered;
        out.push_str(&body_rendered);
        out.push_str(&suffix_rendered);
        out
    } else {
        substitute_scalars(template, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn given_default_template_when_rendered_then_contains_header() {
        let result = make_test_model_result();
        let md = render_detail(DEFAULT_DETAIL_TEMPLATE, &result);
        assert!(md.contains("# Model Detail: test-model-q4_k_m.gguf"));
    }

    #[test]
    fn given_default_template_when_rendered_then_contains_summary_table() {
        let result = make_test_model_result();
        let md = render_detail(DEFAULT_DETAIL_TEMPLATE, &result);
        assert!(md.contains("## Summary"));
        assert!(md.contains("| Model ID | test-model-q4_k_m.gguf |"));
        assert!(md.contains("| Speedup Factor | 2.50x |"));
    }

    #[test]
    fn given_custom_template_when_rendered_then_substitutes_scalars() {
        let result = make_test_model_result();
        let template = "Model {{model_id}} ran at {{tokens_per_second}} tok/s on {{gpu_mode}}.";
        let md = render_detail(template, &result);
        assert_eq!(md, "Model test-model-q4_k_m.gguf ran at 62.50 tok/s on gpu.");
    }

    #[test]
    fn given_custom_template_with_each_block_when_rendered_then_loops_inferences() {
        let result = make_test_model_result();
        let template = "{{#each inference}}P{{index}}: {{prompt_preview}} | {{/each}}";
        let md = render_detail(template, &result);
        assert!(md.contains("P1: What is the capital of France"));
        assert!(md.contains("P2: Explain quantum computing."));
    }

    #[test]
    fn given_detail_generator_with_none_template_when_used_then_uses_default() {
        let result = make_test_model_result();
        let gen = DetailGenerator::new("/tmp", None);
        let md = gen.generate_model_detail(&result).unwrap();
        assert!(md.contains("# Model Detail: test-model-q4_k_m.gguf"));
        assert!(md.contains("## Summary"));
    }

    #[test]
    fn given_detail_generator_with_custom_template_when_used_then_applies_template() {
        let result = make_test_model_result();
        let gen = DetailGenerator::new("/tmp", Some("Custom: {{model_id}}".to_string()));
        let md = gen.generate_model_detail(&result).unwrap();
        assert_eq!(md, "Custom: test-model-q4_k_m.gguf");
    }

    #[test]
    fn given_model_result_when_written_then_file_exists() {
        let result = make_test_model_result();
        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().to_str().unwrap();
        let gen = DetailGenerator::new(output_path, None);

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
        let gen = DetailGenerator::new(output_path, None);

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

        let gen = DetailGenerator::new("/tmp", None);
        let md = gen.generate_model_detail(&result).unwrap();

        assert!(md.contains("- **Input**:"));
        assert!(md.contains("..."));
        assert!(md.contains("#### Full Response"));
        assert!(md.contains("```"));
    }

    #[test]
    fn given_model_result_with_special_chars_when_written_then_sanitizes_filename() {
        let mut result = make_test_model_result();
        result.model_id = "model/with\\special.chars.gguf".to_string();

        let output_dir = tempfile::tempdir().unwrap();
        let output_path = output_dir.path().to_str().unwrap();
        let gen = DetailGenerator::new(output_path, None);

        gen.write_model_detail(&result).unwrap();

        let file = output_dir.path().join("detail-model_with_special_chars.md");
        assert!(file.exists());
    }

    #[test]
    fn given_model_result_with_error_when_rendered_then_error_row_present() {
        let mut result = make_test_model_result();
        result.error = Some("Model failed to load".to_string());

        let md = render_detail(DEFAULT_DETAIL_TEMPLATE, &result);
        assert!(md.contains("| Error | Model failed to load |"));
    }

    #[test]
    fn given_no_speedup_when_rendered_then_no_speedup_row() {
        let mut result = make_test_model_result();
        result.speedup_factor = None;

        let md = render_detail(DEFAULT_DETAIL_TEMPLATE, &result);
        assert!(!md.contains("Speedup Factor"));
    }
}
