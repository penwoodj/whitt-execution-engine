use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use crate::client::disk_monitor;
use crate::agent::loop_hooks::HookContext;
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult};
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use std::process::Command;
use tokio::time::{sleep, timeout};
use tokio::process::Command as TokioCommand;
use tracing::{info, warn};

pub struct BenchmarkConfig {
    pub server_url: String,
    pub models_dir: Option<String>,
    pub model_list_file: Option<String>,
    pub prompts: Vec<String>,
    pub max_tokens: usize,
    pub filter_size_max: Option<u64>,
    pub filter_size_min: Option<u64>,
    pub filter_name: Option<String>,
    pub delay_between_swaps: Duration,
    pub compare_gpu_cpu: bool,
    pub output_dir: Option<String>,
    pub workflow_file: Option<String>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub cooldown_after_unload: Duration,
    pub preflight_only: bool,
    pub model_load_timeout: Duration,
    pub min_tmp_space_mb: u64,
}

#[allow(dead_code)]
fn default_cooldown_after_unload() -> Duration {
    Duration::from_secs(3)
}

#[allow(dead_code)]
fn default_model_load_timeout() -> Duration {
    Duration::from_secs(300)
}

#[allow(dead_code)]
fn default_min_tmp_space_mb() -> u64 {
    1024
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

struct BenchmarkWorkflowConfig {
    prompts: Vec<String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    compare_modes: bool,
    model_list: Vec<String>,
}

struct WorkflowStep {
    step_name: String,
    step_id: String,
    #[allow(dead_code)]
    requires: Vec<String>,
    when: Option<serde_json::Value>,
    prompt: Option<String>,
    input: Option<serde_json::Value>,
    r#loop: Option<serde_json::Value>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Pre-flight checks: verify system health before starting benchmark.
    pub async fn preflight_check(&self) -> Result<()> {
        info!("[benchmark] starting preflight checks...");

        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client for preflight check")?;
        match client.health().await {
            Ok(health) => {
                info!("[benchmark] ✓ Docker health check passed: status={}, idle_slots={}, processing_slots={}",
                    health.status, health.slots_idle, health.slots_processing);
            }
            Err(e) => {
                let msg = format!("Docker health check failed: {}", e);
                info!("[benchmark] ✗ {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        let min_bytes = self.config.min_tmp_space_mb * 1024 * 1024;
        match disk_monitor::ensure_min_free_space(Path::new("/tmp"), min_bytes) {
            Ok(info) => {
                let available_mb = info.available_bytes / (1024 * 1024);
                info!("[benchmark] ✓ /tmp space check passed: {} MB available (min: {} MB required)",
                    available_mb, self.config.min_tmp_space_mb);
            }
            Err(e) => {
                let msg = format!("/tmp space check failed: {}", e);
                info!("[benchmark] ✗ {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        match TokioCommand::new("pgrep")
            .args(["-c", "llama-server"])
            .output()
            .await
        {
            Ok(output) => {
                let count_str = String::from_utf8_lossy(&output.stdout);
                let count: i32 = count_str.trim().parse().unwrap_or(0);
                if count <= 1 {
                    info!("[benchmark] ✓ Zombie process check passed: {} llama-server process(es) found", count);
                } else {
                    let msg = format!("Found {} zombie llama-server processes (expected 0-1)", count);
                    info!("[benchmark] ✗ {}", msg);
                    anyhow::bail!(crate::error::Error::benchmark(msg));
                }
            }
            Err(e) => {
                let msg = format!("Zombie process check failed to execute pgrep: {}", e);
                info!("[benchmark] ✗ {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        info!("[benchmark] all preflight checks passed");
        Ok(())
    }

    /// Runtime system health check: verify resources are adequate before operations.
    pub async fn check_system_health(&self) -> Result<()> {
        let min_bytes = self.config.min_tmp_space_mb * 1024 * 1024;
        match disk_monitor::check_disk_space(Path::new("/tmp")) {
            Ok(info) => {
                let available_mb = info.available_bytes / (1024 * 1024);
                info!("[benchmark] /tmp space: {} MB available (min: {} MB required)",
                    available_mb, self.config.min_tmp_space_mb);
                if info.available_bytes < min_bytes {
                    let msg = format!("Insufficient /tmp space: {} MB available, {} MB required",
                        available_mb, self.config.min_tmp_space_mb);
                    anyhow::bail!(crate::error::Error::benchmark(msg));
                }
            }
            Err(e) => {
                let msg = format!("Failed to check /tmp space: {}", e);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client for health check")?;
        match client.health().await {
            Ok(health) => {
                info!("[benchmark] Docker health: status={}, idle_slots={}, processing_slots={}",
                    health.status, health.slots_idle, health.slots_processing);
            }
            Err(e) => {
                let msg = format!("Docker health check failed: {}", e);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        Ok(())
    }

    fn log_resource_state(&self, phase: &str, model_id: &str) {
        if let Ok(info) = disk_monitor::check_disk_space(Path::new("/tmp")) {
            let available_mb = info.available_bytes / (1024 * 1024);
            info!("[benchmark] [{}] resource state {}: /tmp available={} MB", model_id, phase, available_mb);
        }

        if let Ok(output) = Command::new("free").args(["-m"]).output() {
            let mem_info = String::from_utf8_lossy(&output.stdout);
            for line in mem_info.lines().take(3) {
                info!("[benchmark] [{}] resource state {}: {}", model_id, phase, line);
            }
        }
    }

    fn clean_response_text(text: &str) -> String {
        let mut cleaned = text.trim().to_string();

        cleaned = cleaned.replace("<|im_start|>", "").replace("<|im_end|>", "").trim().to_string();

        // Extract content from markdown code fences (opening ``` anywhere in text)
        if let Some(first_fence) = cleaned.find("```") {
            let after_fence = &cleaned[first_fence..];
            let first_newline = after_fence.find('\n');
            let opening_end = first_fence + first_newline.unwrap_or(3);
            if opening_end < cleaned.len() {
                if let Some(last_fence) = cleaned[opening_end..].rfind("```") {
                    let end_pos = opening_end + last_fence;
                    if opening_end + 1 < end_pos {
                        cleaned = cleaned[opening_end + 1..end_pos].trim().to_string();
                    }
                }
            }
        }

        if cleaned.starts_with('{') || cleaned.starts_with('[') {
            if serde_json::from_str::<serde_json::Value>(&cleaned).is_ok() {
                return cleaned;
            }

            let last_brace = cleaned.rfind('}');
            let last_bracket = cleaned.rfind(']');

            for closing_pos in [last_brace, last_bracket].into_iter().flatten() {
                let truncated = &cleaned[..=closing_pos];
                if serde_json::from_str::<serde_json::Value>(truncated).is_ok() {
                    return truncated.to_string();
                }
            }

            let mut chars: Vec<char> = cleaned.chars().collect();
            for i in (1..chars.len()).rev() {
                if (chars[i] == '}' || chars[i] == ']') && i > 0 && chars[i - 1] == ',' {
                    chars.remove(i - 1);
                }
            }

            let without_trailing_commas: String = chars.into_iter().collect();
            if serde_json::from_str::<serde_json::Value>(&without_trailing_commas).is_ok() {
                return without_trailing_commas;
            }
        }

        cleaned
    }

    fn load_workflow_config(&self) -> Option<BenchmarkWorkflowConfig> {
        let wf_path = self.config.workflow_file.as_ref()?;
        let content = fs::read_to_string(wf_path).ok()?;

        // Validate the YAML against the schema before reading any config from it.
        // This rejects non-schema keys (benchmark:, model_list:, etc.).
        if let Err(e) = crate::workflow::WorkflowFile::from_yaml(&content) {
            warn!("[benchmark] workflow YAML validation failed: {}", e);
            return None;
        }

        let yaml_value: serde_json::Value = match serde_saphyr::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                warn!("[benchmark] failed to parse workflow YAML for config extraction: {}", e);
                return None;
            }
        };

        let agentic_workflow = match yaml_value.get("agentic_workflow") {
            Some(aw) => aw,
            None => {
                warn!("[benchmark] no agentic_workflow section in YAML");
                return None;
            }
        };

        // Steps can be map (named) or array (ordered) format
        let first_step = if let Some(steps_map) = agentic_workflow.as_object() {
            steps_map.values().next()
        } else if let Some(steps_arr) = agentic_workflow.as_array() {
            steps_arr.first()
        } else {
            None
        };

        let first_step = match first_step {
            Some(s) => s,
            None => {
                warn!("[benchmark] no steps found in agentic_workflow");
                return None;
            }
        };

        let prompts = if let Some(prompt) = first_step.get("prompt").and_then(|v| v.as_str()) {
            vec![prompt.to_string()]
        } else {
            // Fallback: check input.prompt when top-level prompt absent
            let input_prompt = first_step
                .get("input")
                .and_then(|i| i.get("prompt"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();
            if input_prompt.is_empty() {
                return None;
            }
            vec![input_prompt]
        };

        // max_tokens from input.max_tokens, default 4096
        let max_tokens = first_step
            .get("input")
            .and_then(|i| i.get("max_tokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(4096) as usize;

        let temperature = first_step
            .get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.7);
        let top_p = first_step
            .get("top_p")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.95);

        let model_list: Vec<String> = first_step
            .get("loop")
            .and_then(|l| l.get("models"))
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        info!("[benchmark] extracted workflow config: prompts={}, max_tokens={}, temperature={}, top_p={}, model_list={}",
            prompts.len(), max_tokens, temperature, top_p, model_list.len());

        Some(BenchmarkWorkflowConfig {
            prompts,
            max_tokens,
            temperature,
            top_p,
            compare_modes: false,
            model_list,
        })
    }

    fn load_workflow_steps(&self) -> Option<Vec<WorkflowStep>> {
        let wf_path = self.config.workflow_file.as_ref()?;
        let content = fs::read_to_string(wf_path).ok()?;

        let yaml_value: serde_json::Value = serde_saphyr::from_str(&content).ok()?;
        let agentic_workflow = yaml_value.get("agentic_workflow")?;

        let steps: Vec<WorkflowStep> = if let Some(steps_array) = agentic_workflow.as_array() {
            steps_array.iter()
                .filter_map(|step| {
                    let step_name = step.get("step")?.as_str()?.to_string();
                    let step_id = step.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let requires = step.get("requires")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    let when = step.get("when").cloned();
                    let prompt = step.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let input = step.get("input").cloned();
                    let loop_config = step.get("loop").cloned();

                    Some(WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        when,
                        prompt,
                        input,
                        r#loop: loop_config,
                    })
                })
                .collect()
        } else if let Some(steps_map) = agentic_workflow.as_object() {
            steps_map.iter()
                .map(|(step_name_key, step_value)| {
                    let step_name = step_name_key.clone();
                    let step_id = step_value.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let requires = step_value.get("requires")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    let when = step_value.get("when").cloned();
                    let prompt = step_value.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let input = step_value.get("input").cloned();
                    let loop_config = step_value.get("loop").cloned();

                    WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        when,
                        prompt,
                        input,
                        r#loop: loop_config,
                    }
                })
                .collect()
        } else {
            return None;
        };

        Some(steps)
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
            let timestamp = {
                let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                format!("unix_epoch_{}s", d.as_secs())
            };
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
            let timestamp = {
                let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                format!("unix_epoch_{}s", d.as_secs())
            };
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

    fn write_per_model_json(&self, result: &ModelBenchmarkResult) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            if result.inference_results.is_empty() {
                return Ok(());
            }
            let safe_name = result.model_id
                .replace(['/', '\\'], "_")
                .replace(".gguf", "")
                .replace('.', "_");
            let file_path = Path::new(output_dir).join(format!("output/{}.json", safe_name));

            let response_data: Vec<serde_json::Value> = result.inference_results.iter()
                .map(|inf| {
                    serde_json::json!({
                        "model_id": result.model_id,
                        "prompt": inf.prompt,
                        "response_text": inf.response_text,
                        "prompt_tokens": inf.prompt_tokens,
                        "completion_tokens": inf.completion_tokens,
                        "total_tokens": inf.total_tokens,
                        "duration_ms": inf.duration.as_millis(),
                        "tokens_per_second": inf.tokens_per_second,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "model_id": result.model_id,
                "file_size_bytes": result.file_size_bytes,
                "gpu_mode": result.gpu_mode,
                "tokens_per_second": result.tokens_per_second,
                "total_duration_ms": result.total_duration.as_millis(),
                "inferences": response_data,
            });

            let json_str = serde_json::to_string_pretty(&output)
                .context("Failed to serialize model result to JSON")?;
            fs::write(&file_path, json_str)
                .with_context(|| format!("Failed to write model JSON: {}", file_path.display()))?;

            info!("[benchmark] per-model JSON written to {}", file_path.display());
        }
        Ok(())
    }

    /// Write exhaustive per-model report file.
    ///
    /// Creates a file named after the model (sanitized for filesystem) in the
    /// output directory. Contains every detail from the benchmark run.
    fn write_per_model_report(&self, result: &ModelBenchmarkResult, suite_metadata: &str, wf_ctx: Option<&BenchmarkWorkflowConfig>) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let safe_name = result.model_id
                .replace(['/', '\\'], "_")
                .replace(".gguf", "")
                .replace('.', "_");
            let file_path = Path::new(output_dir).join(format!("{}.log", safe_name));

            let mut content = String::new();

            content.push_str(&format!("{}\n", "=".repeat(80)));
            content.push_str(&format!("BENCHMARK REPORT: {}\n", result.model_id));
            content.push_str(&format!("{}\n\n", "=".repeat(80)));

            content.push_str(&format!("{}\n", suite_metadata));

            if let Some(ctx) = wf_ctx {
                content.push_str("\n--- Workflow Context ---\n");
                content.push_str(&format!("workflow_file: {}\n", self.config.workflow_file.as_deref().unwrap_or("")));
                content.push_str("workflow_step: benchmark_performance\n");
                content.push_str(&format!("compare_modes: {}\n", ctx.compare_modes));
                content.push_str(&format!("workflow_max_tokens: {}\n", ctx.max_tokens));
                content.push_str(&format!("workflow_temperature: {}\n", ctx.temperature));
                content.push_str(&format!("workflow_top_p: {}\n", ctx.top_p));
                content.push_str("workflow_prompts:\n");
                for (pi, p) in ctx.prompts.iter().enumerate() {
                    content.push_str(&format!("  [{}]: {}\n", pi + 1, p));
                }
                if !ctx.model_list.is_empty() {
                    content.push_str("workflow_model_list:\n");
                    for (mi, m) in ctx.model_list.iter().enumerate() {
                        content.push_str(&format!("  [{}]: {}\n", mi + 1, m));
                    }
                }
            }

            content.push_str("\n--- Model Identity ---\n");
            content.push_str(&format!("model_id: {}\n", result.model_id));
            content.push_str(&format!("model_path: {}\n", result.model_path));
            content.push_str(&format!("file_size_bytes: {}\n", result.file_size_bytes));
            content.push_str(&format!("file_size_mb: {:.2}\n", result.file_size_bytes as f64 / (1024.0 * 1024.0)));
            content.push_str(&format!("gpu_mode: {}\n", result.gpu_mode));

            content.push_str("\n--- Timing ---\n");
            content.push_str(&format!("load_duration: {}ms ({:.3}s)\n", result.load_duration.as_millis(), result.load_duration.as_secs_f64()));
            content.push_str(&format!("unload_duration: {}ms ({:.3}s)\n", result.unload_duration.as_millis(), result.unload_duration.as_secs_f64()));
            content.push_str(&format!("total_duration: {}ms ({:.3}s)\n", result.total_duration.as_millis(), result.total_duration.as_secs_f64()));

            content.push_str("\n--- Performance Summary ---\n");
            content.push_str(&format!("tokens_per_second: {:.2}\n", result.tokens_per_second));
            content.push_str(&format!("avg_latency_ms: {:.2}\n", result.avg_latency_ms));
            content.push_str(&format!("p50_latency_ms: {:.2}\n", result.p50_latency_ms));
            content.push_str(&format!("p95_latency_ms: {:.2}\n", result.p95_latency_ms));
            content.push_str(&format!("p99_latency_ms: {:.2}\n", result.p99_latency_ms));
            if let Some(speedup) = result.speedup_factor {
                content.push_str(&format!("speedup_factor: {:.2}x\n", speedup));
            }

            content.push_str(&format!("\n--- Inference Results ({}) ---\n", result.inference_results.len()));
            for (i, inf) in result.inference_results.iter().enumerate() {
                content.push_str(&format!("\n  Inference [{}]:\n", i + 1));
                content.push_str(&format!("    prompt: {:?}\n", inf.prompt));
                content.push_str(&format!("    prompt_tokens: {}\n", inf.prompt_tokens));
                content.push_str(&format!("    completion_tokens: {}\n", inf.completion_tokens));
                content.push_str(&format!("    total_tokens: {}\n", inf.total_tokens));
                content.push_str(&format!("    duration: {}ms ({:.3}s)\n", inf.duration.as_millis(), inf.duration.as_secs_f64()));
                content.push_str(&format!("    tokens_per_second: {:.2}\n", inf.tokens_per_second));
                content.push_str("    response_text:\n");
                for line in inf.response_text.lines() {
                    content.push_str(&format!("      {}\n", line));
                }
            }

            if let Some(ref err) = result.error {
                content.push_str("\n--- Error ---\n");
                content.push_str(&format!("error: {}\n", err));
            }

            let status = if result.error.is_none() { "SUCCESS" } else { "FAILED" };
            content.push_str(&format!("\n--- Status: {} ---\n", status));
            content.push_str(&format!("{}\n", "=".repeat(80)));

            fs::write(&file_path, content)
                .with_context(|| format!("Failed to write per-model report: {}", file_path.display()))?;

            info!("[benchmark] per-model report written to {}", file_path.display());
        }
        Ok(())
    }

    /// Append model result to markdown chat log with timestamps.
    fn append_chat_log_markdown(&self, result: &ModelBenchmarkResult, run_timestamp: &str) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let chat_path = Path::new(output_dir).join("chat-log.md");

            let mut md = String::new();

            if !chat_path.exists() {
                md.push_str("# Benchmark Chat Log\n\n");
                md.push_str(&format!("Run: {}  \n", run_timestamp));
                md.push_str(&format!("Server: {}  \n", self.config.server_url));
                md.push_str(&format!("Models: {}  \n\n", result.model_id));
                md.push_str("---\n\n");
            }

            md.push_str(&format!("## {} ({})\n\n", result.model_id, result.gpu_mode.to_uppercase()));

            if result.error.is_some() {
                md.push_str(&format!("> **ERROR**: {}\n\n", result.error.as_deref().unwrap_or("unknown")));
                md.push_str("---\n\n");
            } else {
                md.push_str("| Metric | Value |\n|---|---|\n");
                md.push_str(&format!("| Load | {}ms |\n", result.load_duration.as_millis()));
                md.push_str(&format!("| Unload | {}ms |\n", result.unload_duration.as_millis()));
                md.push_str(&format!("| Total | {}ms |\n", result.total_duration.as_millis()));
                md.push_str(&format!("| Tokens/s | {:.2} |\n", result.tokens_per_second));
                md.push_str(&format!("| Avg Latency | {:.2}ms |\n\n", result.avg_latency_ms));

                for (i, inf) in result.inference_results.iter().enumerate() {
                    let ts = {
                        let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                        format!("unix_epoch_{}s", d.as_secs())
                    };
                    md.push_str(&format!("### Prompt {} ({})\n\n", i + 1, ts));
                    md.push_str(&format!("**User**: {}\n\n", inf.prompt));
                    md.push_str(&format!("**Model**: ({} prompt tokens, {} completion tokens, {}ms, {:.2} tok/s)\n\n",
                        inf.prompt_tokens, inf.completion_tokens, inf.duration.as_millis(), inf.tokens_per_second));
                    md.push_str(&format!("**Response**:\n\n{}\n\n", inf.response_text));
                }

                md.push_str("---\n\n");
            }

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&chat_path)
                .with_context(|| format!("Failed to open chat log: {}", chat_path.display()))?;

            file.write_all(md.as_bytes())
                .context("Failed to write chat-log.md")?;

            info!("[benchmark] chat log appended to {}", chat_path.display());
        }
        Ok(())
    }

    /// Execute hook actions: log, save_to, append_to, fail.
    ///
    /// Supports template interpolation: {{current_model}}, {{step.output}}, {{iteration}}
    fn execute_hook(&self, hook: &Option<serde_json::Value>, step_name: &str, timing: &str, context: &HookContext) -> Result<()> {
        if let Some(h) = hook {
            for action in h.as_array().unwrap_or(&vec![]) {
                if let Some(log) = action.get("log") {
                    if let Some(path) = log.get("to_file_path").and_then(|v| v.as_str()) {
                        let path = self.interpolate_template(path, context);
                        let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                        let timestamp = format!("unix_epoch_{}s", d.as_secs());
                        let line = format!("[{}] step={} timing={}\n", timestamp, step_name, timing);

                        let mut file = fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path)
                            .with_context(|| format!("Failed to open hook log file: {}", path))?;

                        file.write_all(line.as_bytes())
                            .context("Failed to write to hook log")?;
                    }
                }

                if let Some(save_to) = action.get("save_to") {
                    if let Some(path) = save_to.as_str() {
                        let path = self.interpolate_template(path, context);
                        let content = context.output.as_deref().unwrap_or("");

                        // Create parent directories if they don't exist
                        if let Some(parent) = Path::new(&path).parent() {
                            if !parent.as_os_str().is_empty() {
                                fs::create_dir_all(parent)
                                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
                            }
                        }

                        fs::write(&path, content)
                            .with_context(|| format!("Failed to save to: {}", path))?;

                        info!("[benchmark] saved output to {}", path);
                    }
                }

                if let Some(append_to) = action.get("append_to") {
                    if let Some(path) = append_to.as_str() {
                        let path = self.interpolate_template(path, context);
                        let content = context.output.as_deref().unwrap_or("");

                        let mut file = fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path)
                            .with_context(|| format!("Failed to open append file: {}", path))?;

                        file.write_all(content.as_bytes())
                            .context("Failed to append to file")?;

                        info!("[benchmark] appended to {}", path);
                    }
                }

                if let Some(_fail) = action.get("fail") {
                    anyhow::bail!("Hook triggered fail: step={}, timing={}", step_name, timing);
                }
            }
        }
        Ok(())
    }

    fn interpolate_template(&self, template: &str, context: &HookContext) -> String {
        let mut result = template.to_string();
        result = result.replace("{{current_model}}", &context.step_name);
        result = result.replace("{{loop.current_model}}", &context.step_name);

        if let Some(ref output) = context.output {
            result = result.replace("{{step.output}}", output);
        }

        result = result.replace("{{iteration}}", &context.iteration.to_string());
        result = result.replace("{{loop.iteration}}", &context.iteration.to_string());

        result
    }
}

impl BenchmarkRunner {
    fn log_step_execution(&self, step: &WorkflowStep, results: &[ModelBenchmarkResult]) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let log_path = Path::new(output_dir).join("logs/workflow_steps.log");
            let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let timestamp = format!("unix_epoch_{}s", d.as_secs());
            let line = format!("{} [STEP] id={} name={} models_benchmarked={}\n",
                timestamp, step.step_id, step.step_name, results.len());

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .with_context(|| format!("Failed to open workflow steps log: {}", log_path.display()))?;

            file.write_all(line.as_bytes())
                .context("Failed to write to workflow_steps.log")?;
        }
        Ok(())
    }

    fn copy_workflow_yaml(&self) -> Result<()> {
        if let Some(ref output_dir) = self.config.output_dir {
            let workflow_candidates = vec![
                "docs/benchmarks/workflows/benchmark-3-models.yml",
                "docs/benchmarks/workflows/benchmark-5-models.yml",
                "docs/benchmarks/workflows/benchmark-15-models.yml",
                "docs/benchmarks/workflows/benchmark-50-models.yml",
            ];

            for wf in &workflow_candidates {
                let src = Path::new(wf);
                if src.exists() {
                    let dest = Path::new(output_dir).join(src.file_name().unwrap());
                    let _ = fs::copy(src, dest);
                }
            }
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

    fn discover_models(&self, workflow_model_list: Option<&[String]>) -> Result<Vec<(String, String)>> {
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

                        if let Some(min_size) = self.config.filter_size_min {
                            if let Ok(metadata) = std::fs::metadata(path) {
                                if metadata.len() < min_size {
                                    info!("[benchmark] skipping {} (size {} < {})",
                                        model_id, metadata.len(), min_size);
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

                        models.push((model_id, path.display().to_string()));
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

                if let Some(min_size) = self.config.filter_size_min {
                    if path.exists() {
                        if let Ok(metadata) = std::fs::metadata(path) {
                            if metadata.len() < min_size {
                                info!("[benchmark] skipping {} (size {} < {})",
                                    model_id, metadata.len(), min_size);
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

                models.push((model_id, line.to_string()));
            }
        } else if let Some(wf_models) = workflow_model_list {
            for model_path in wf_models {
                let path = Path::new(model_path);
                let model_id = if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    file_name.to_string()
                } else {
                    model_path.to_string()
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

                if let Some(min_size) = self.config.filter_size_min {
                    if path.exists() {
                        if let Ok(metadata) = std::fs::metadata(path) {
                            if metadata.len() < min_size {
                                info!("[benchmark] skipping {} (size {} < {})",
                                    model_id, metadata.len(), min_size);
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

                models.push((model_id, model_path.to_string()));
            }
        }

        Ok(models)
    }

    #[allow(clippy::too_many_arguments)]
    async fn execute_workflow_step(&self, step: &WorkflowStep, client: &LlamaHttpClient, models: &[(String, String)], max_tokens: usize, temperature: f64, top_p: f64, _run_timestamp: &str) -> Result<Vec<ModelBenchmarkResult>> {
        let mut results = Vec::new();

        let (max_iterations, iteration_variable_name) = if let Some(ref loop_config) = step.r#loop {
            let count = loop_config.get("count");
            let max_iter = count.and_then(|c| c.get("max_iterations")).and_then(|m| m.as_u64()).unwrap_or(1) as usize;
            let var_name = count.and_then(|c| c.get("iteration_variable")).and_then(|v| v.as_str()).unwrap_or("iteration");
            (max_iter, var_name.to_string())
        } else {
            (models.len(), "iteration".to_string())
        };

        let default_prompt = String::new();
        let prompt = step.prompt.as_ref().unwrap_or(&default_prompt);
        let step_max_tokens = step.input.as_ref().and_then(|inp| inp.get("max_tokens").and_then(|m| m.as_u64()).map(|m| m as usize)).unwrap_or(max_tokens);

        info!("[benchmark] executing workflow step: {} (iterations: {})", step.step_id, max_iterations);

        for (i, (model_id, model_path)) in models.iter().take(max_iterations).enumerate() {
            let iteration = i + 1;

            info!("[benchmark] [{}/{}] {} with model {}", iteration, max_iterations, step.step_id, model_id);

            let model_result = self.benchmark_single_model(client, model_id, model_path, "gpu", std::slice::from_ref(prompt), step_max_tokens, temperature, top_p).await;

            let output_text = model_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();

            let context = HookContext {
                step_name: model_id.clone(),
                iteration,
                output: Some(output_text),
                error_message: model_result.error.clone(),
                loop_type: "count".to_string(),
            };

            self.execute_hook(&step.when, &step.step_id, "before_step_starts", &context)?;

            if let Some(ref err) = model_result.error {
                self.log_step_error(model_id, err)?;
                self.execute_hook(&step.when, &step.step_id, "after_step_fails", &context)?;
            } else {
                self.log_step_result(&model_result)?;
                self.execute_hook(&step.when, &step.step_id, "after_step_succeeds", &context)?;
            }

            results.push(model_result);

            if i < max_iterations.saturating_sub(1) {
                tokio::time::sleep(self.config.delay_between_swaps).await;
            }
        }

        Ok(results)
    }

    pub async fn run(&self) -> Result<BenchmarkSuiteResult> {
        self.preflight_check().await.context("Preflight checks failed")?;

        if self.config.preflight_only {
            info!("[benchmark] Preflight checks passed. Exiting (--preflight mode).");
            return Ok(BenchmarkSuiteResult {
                timestamp: {
                    let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                    format!("unix_epoch_{}s", d.as_secs())
                },
                server_url: self.config.server_url.clone(),
                total_models: 0,
                successful: 0,
                failed: 0,
                results: vec![],
            });
        }

        self.ensure_output_dirs()
            .context("Failed to ensure output directories")?;
        let _ = self.copy_workflow_yaml();

        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client")?;

        let run_timestamp = {
            let dur = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            format!("unix_epoch_{}s", dur.as_secs())
        };

        let wf_ctx = self.load_workflow_config();

        let (prompts, max_tokens, compare_gpu_cpu, temperature, top_p) = if let Some(ref ctx) = wf_ctx {
            let _default_prompt = "The quick brown fox jumps over the lazy dog.";
            let is_default_prompt = self.config.prompts.iter().any(|p| p.contains("The quick brown fox"));
            let is_default_max_tokens = self.config.max_tokens == 100;
            let is_default_compare_gpu_cpu = !self.config.compare_gpu_cpu;

            let merged_prompts = if is_default_prompt && !ctx.prompts.is_empty() {
                ctx.prompts.clone()
            } else {
                self.config.prompts.clone()
            };

            let merged_max_tokens = if is_default_max_tokens && ctx.max_tokens != 128 {
                ctx.max_tokens
            } else {
                self.config.max_tokens
            };

            let merged_compare_gpu_cpu = if is_default_compare_gpu_cpu && ctx.compare_modes {
                true
            } else {
                self.config.compare_gpu_cpu
            };

            let merged_temperature = ctx.temperature;
            let merged_top_p = ctx.top_p;

            info!("[benchmark] loaded workflow configuration");
            info!("[benchmark] merged: prompts={}, max_tokens={}, compare_gpu_cpu={}, temperature={}, top_p={}",
                merged_prompts.len(), merged_max_tokens, merged_compare_gpu_cpu, merged_temperature, merged_top_p);

            (merged_prompts, merged_max_tokens, merged_compare_gpu_cpu, merged_temperature, merged_top_p)
        } else {
            (self.config.prompts.clone(), self.config.max_tokens, self.config.compare_gpu_cpu, self.config.temperature.unwrap_or(0.7), self.config.top_p.unwrap_or(0.9))
        };

        let mut models = self.discover_models(wf_ctx.as_ref().map(|ctx| ctx.model_list.as_slice()))
            .context("Failed to discover models")?;

        // Fallback: if no models found via files/list/workflow, query server's loaded model
        if models.is_empty() {
            info!("[benchmark] no models found locally, querying server for loaded model...");
            match client.list_models().await {
                Ok(server_models) => {
                    for m in server_models {
                        info!("[benchmark] found server model: {} ({})", m.id, m.status.value);
                        let filename = if m.filename.is_empty() { m.id.clone() } else { m.filename.clone() };
                        models.push((m.id.clone(), filename));
                    }
                }
                Err(e) => {
                    warn!("[benchmark] failed to query server models: {}", e);
                    // Last resort: use a placeholder to run against whatever is loaded
                    models.push(("loaded-model".to_string(), "loaded-model".to_string()));
                }
            }
        }

        if models.is_empty() {
            anyhow::bail!("No models found for benchmarking");
        }

        info!("[benchmark] found {} models to benchmark", models.len());
        info!("[benchmark] GPU/CPU comparison mode: {}", compare_gpu_cpu);

        let mut results = Vec::new();

        let suite_metadata = format!(
            "run_timestamp: {}\nserver_url: {}\ntotal_models: {}\ncompare_gpu_cpu: {}",
            run_timestamp, self.config.server_url, models.len(), compare_gpu_cpu
        );

        let workflow_steps = self.load_workflow_steps();
        if let Some(steps) = &workflow_steps {
            info!("[benchmark] loaded {} workflow steps from YAML", steps.len());

            if let Some(first_step) = steps.first() {
                if first_step.prompt.is_some() && first_step.r#loop.is_some() {
                    info!("[benchmark] executing workflow-driven benchmark with step: {}", first_step.step_id);

                    let mut workflow_results = Vec::new();

                    for (i, (model_id, model_path)) in models.iter().enumerate() {
                        self.log_step_start(model_id, i, models.len(), "gpu")?;

                        info!("[benchmark] [{}/{}] loading {} (workflow mode)", i+1, models.len(), model_id);

                        if let Err(e) = self.check_system_health().await {
                            warn!("[benchmark] Skipping {} due to health check failure: {}", model_id, e);
                            let failed_result = ModelBenchmarkResult {
                                model_id: model_id.clone(),
                                model_path: model_path.clone(),
                                file_size_bytes: 0,
                                load_duration: Duration::ZERO,
                                inference_results: vec![],
                                unload_duration: Duration::ZERO,
                                total_duration: Duration::ZERO,
                                tokens_per_second: 0.0,
                                avg_latency_ms: 0.0,
                                p50_latency_ms: 0.0,
                                p95_latency_ms: 0.0,
                                p99_latency_ms: 0.0,
                                error: Some(format!("Health check failed: {}", e)),
                                gpu_mode: "gpu".to_string(),
                                speedup_factor: None,
                            };
                            self.log_step_result(&failed_result)?;
                            self.log_step_error(model_id, failed_result.error.as_ref().unwrap())?;
                            workflow_results.push(failed_result);
                            continue;
                        }

                        let step_results = self.execute_workflow_step(first_step, &client, &models[i..i+1], max_tokens, temperature, top_p, &run_timestamp).await?;
                        workflow_results.extend(step_results);

                        if i < models.len() - 1 {
                            tokio::time::sleep(self.config.delay_between_swaps).await;
                        }
                    }

                    results.extend(workflow_results);
                } else {
                    for step in steps.iter().skip(1) {
                        info!("[benchmark] executing step: {} (id: {})", step.step_name, step.step_id);
                        self.log_step_execution(step, &results)?;

                        self.execute_hook(&step.when, &step.step_id, "before_step_starts", &HookContext { step_name: String::new(), iteration: 0, output: None, error_message: None, loop_type: String::new() })?;
                        self.execute_hook(&step.when, &step.step_id, "after_step_succeeds", &HookContext { step_name: String::new(), iteration: 0, output: None, error_message: None, loop_type: String::new() })?;

                        info!("[benchmark] step {} completed (stub execution)", step.step_id);
                    }
                }
            }
        }

        if compare_gpu_cpu {
            info!("[benchmark] Running in GPU/CPU comparison mode");

            for (i, (model_id, model_path)) in models.iter().enumerate() {
                self.log_step_start(model_id, i, models.len(), "gpu")?;

                info!("[benchmark] [{}/{}] loading {} (GPU mode)", i+1, models.len(), model_id);

                if let Err(e) = self.check_system_health().await {
                    warn!("[benchmark] Skipping {} due to health check failure: {}", model_id, e);
                    let failed_result = ModelBenchmarkResult {
                        model_id: model_id.clone(),
                        model_path: model_path.clone(),
                        file_size_bytes: 0,
                        load_duration: Duration::ZERO,
                        inference_results: vec![],
                        unload_duration: Duration::ZERO,
                        total_duration: Duration::ZERO,
                        tokens_per_second: 0.0,
                        avg_latency_ms: 0.0,
                        p50_latency_ms: 0.0,
                        p95_latency_ms: 0.0,
                        p99_latency_ms: 0.0,
                        error: Some(format!("Health check failed: {}", e)),
                        gpu_mode: "gpu".to_string(),
                        speedup_factor: None,
                    };
                    self.log_step_result(&failed_result)?;
                    self.log_step_error(model_id, failed_result.error.as_ref().unwrap())?;
                    results.push(failed_result);
                    continue;
                }

                let gpu_result = self.benchmark_single_model(&client, model_id, model_path, "gpu", &prompts, max_tokens, temperature, top_p).await;

                if let Some(ref err) = gpu_result.error {
                    self.log_step_error(model_id, err)?;
                }
                self.log_step_result(&gpu_result)?;

                if gpu_result.error.is_none() {
                    self.log_step_start(model_id, i, models.len(), "cpu")?;

                    info!("[benchmark] [{}/{}] loading {} (CPU mode)", i+1, models.len(), model_id);

                    self.restart_docker_with_gpu_layers(0).await
                        .context("Failed to restart Docker in CPU mode")?;

                    if let Err(e) = self.check_system_health().await {
                        warn!("[benchmark] Skipping {} CPU mode due to health check failure: {}", model_id, e);
                        let cpu_result = ModelBenchmarkResult {
                            model_id: model_id.clone(),
                            model_path: model_path.clone(),
                            file_size_bytes: 0,
                            load_duration: Duration::ZERO,
                            inference_results: vec![],
                            unload_duration: Duration::ZERO,
                            total_duration: Duration::ZERO,
                            tokens_per_second: 0.0,
                            avg_latency_ms: 0.0,
                            p50_latency_ms: 0.0,
                            p95_latency_ms: 0.0,
                            p99_latency_ms: 0.0,
                            error: Some(format!("Health check failed: {}", e)),
                            gpu_mode: "cpu".to_string(),
                            speedup_factor: None,
                        };
                        self.log_step_result(&cpu_result)?;
                        self.log_step_error(model_id, cpu_result.error.as_ref().unwrap())?;
                        results.push(gpu_result);
                        results.push(cpu_result);
                        self.restart_docker_with_gpu_layers(999).await
                            .context("Failed to restart Docker in GPU mode")?;
                        continue;
                    }

                    let cpu_result = self.benchmark_single_model(&client, model_id, model_path, "cpu", &prompts, max_tokens, temperature, top_p).await;

                    if let Some(ref err) = cpu_result.error {
                        self.log_step_error(model_id, err)?;
                    }
                    self.log_step_result(&cpu_result)?;

                    if cpu_result.error.is_none() {
                        let speedup = cpu_result.tokens_per_second / gpu_result.tokens_per_second;
                        info!("[benchmark] {} speedup factor: {:.2}x", model_id, speedup);

                        let mut gpu_result_with_speedup = gpu_result.clone();
                        gpu_result_with_speedup.speedup_factor = Some(speedup);
                        self.write_per_model_report(&gpu_result_with_speedup, &suite_metadata, wf_ctx.as_ref())?;
                        self.write_per_model_json(&gpu_result_with_speedup)?;
                        self.append_chat_log_markdown(&gpu_result_with_speedup, &run_timestamp)?;
                        results.push(gpu_result_with_speedup);

                        let mut cpu_result_with_speedup = cpu_result.clone();
                        cpu_result_with_speedup.speedup_factor = Some(speedup);
                        self.write_per_model_report(&cpu_result_with_speedup, &suite_metadata, wf_ctx.as_ref())?;
                        self.write_per_model_json(&cpu_result_with_speedup)?;
                        self.append_chat_log_markdown(&cpu_result_with_speedup, &run_timestamp)?;
                        results.push(cpu_result_with_speedup);
                    } else {
                        self.write_per_model_report(&gpu_result, &suite_metadata, wf_ctx.as_ref())?;
                        self.write_per_model_json(&gpu_result)?;
                        self.append_chat_log_markdown(&gpu_result, &run_timestamp)?;
                        self.write_per_model_report(&cpu_result, &suite_metadata, wf_ctx.as_ref())?;
                        self.write_per_model_json(&cpu_result)?;
                        self.append_chat_log_markdown(&cpu_result, &run_timestamp)?;
                        results.push(gpu_result);
                        results.push(cpu_result);
                    }

                    self.restart_docker_with_gpu_layers(999).await
                        .context("Failed to restart Docker in GPU mode")?;
                } else {
                    self.write_per_model_report(&gpu_result, &suite_metadata, wf_ctx.as_ref())?;
                    self.write_per_model_json(&gpu_result)?;
                    self.append_chat_log_markdown(&gpu_result, &run_timestamp)?;
                    results.push(gpu_result);
                }

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
            }
        } else {
            for (i, (model_id, model_path)) in models.iter().enumerate() {
                self.log_step_start(model_id, i, models.len(), "gpu")?;

                info!("[benchmark] [{}/{}] loading {}", i+1, models.len(), model_id);

                if let Err(e) = self.check_system_health().await {
                    warn!("[benchmark] Skipping {} due to health check failure: {}", model_id, e);
                    let failed_result = ModelBenchmarkResult {
                        model_id: model_id.clone(),
                        model_path: model_path.clone(),
                        file_size_bytes: 0,
                        load_duration: Duration::ZERO,
                        inference_results: vec![],
                        unload_duration: Duration::ZERO,
                        total_duration: Duration::ZERO,
                        tokens_per_second: 0.0,
                        avg_latency_ms: 0.0,
                        p50_latency_ms: 0.0,
                        p95_latency_ms: 0.0,
                        p99_latency_ms: 0.0,
                        error: Some(format!("Health check failed: {}", e)),
                        gpu_mode: "gpu".to_string(),
                        speedup_factor: None,
                    };
                    self.log_step_result(&failed_result)?;
                    self.log_step_error(model_id, failed_result.error.as_ref().unwrap())?;
                    results.push(failed_result);
                    continue;
                }

                let model_result = self.benchmark_single_model(&client, model_id, model_path, "gpu", &prompts, max_tokens, temperature, top_p).await;

                if let Some(ref err) = model_result.error {
                    self.log_step_error(model_id, err)?;
                }
                self.log_step_result(&model_result)?;
                self.write_per_model_report(&model_result, &suite_metadata, wf_ctx.as_ref())?;
                self.write_per_model_json(&model_result)?;
                self.append_chat_log_markdown(&model_result, &run_timestamp)?;

                results.push(model_result);

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
            }
        }

        if let Some(steps) = workflow_steps {
            for step in steps.iter().skip(1) {
                info!("[benchmark] executing step: {} (id: {})", step.step_name, step.step_id);
                self.log_step_execution(step, &results)?;

                self.execute_hook(&step.when, &step.step_id, "before_step_starts", &HookContext { step_name: String::new(), iteration: 0, output: None, error_message: None, loop_type: String::new() })?;
                self.execute_hook(&step.when, &step.step_id, "after_step_succeeds", &HookContext { step_name: String::new(), iteration: 0, output: None, error_message: None, loop_type: String::new() })?;

                info!("[benchmark] step {} completed (stub execution)", step.step_id);
            }
        }

        let successful = results.iter().filter(|r| r.error.is_none()).count();
        let failed = results.len() - successful;
        let timestamp = {
                let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                format!("unix_epoch_{}s", d.as_secs())
            };

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

    #[allow(clippy::too_many_arguments)]
    async fn benchmark_single_model(&self, client: &LlamaHttpClient, model_id: &str, model_source_path: &str, gpu_mode: &str, prompts: &[String], max_tokens: usize, temperature: f64, top_p: f64) -> ModelBenchmarkResult {
        let server_model_id = model_id.strip_suffix(".gguf").unwrap_or(model_id);
        let start = Instant::now();

        let (resolved_path, file_size) = if let Some(ref models_dir) = self.config.models_dir {
            let full_path = Path::new(models_dir).join(model_id);
            if full_path.exists() {
                let size = std::fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);
                (full_path.display().to_string(), size)
            } else {
                (model_source_path.to_string(), 0)
            }
        } else {
            let path = Path::new(model_source_path);
            if path.exists() {
                let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                (model_source_path.to_string(), size)
            } else {
                (model_source_path.to_string(), 0)
            }
        };

        if let Ok(models) = client.list_models().await {
            for m in models {
                if m.status.value == "loaded" && m.id != server_model_id {
                    let _ = client.unload_model(&m.id).await;
                }
            }
        }

        if let Err(e) = self.check_system_health().await {
            warn!("[benchmark] Health check before load failed for {}: {}", model_id, e);
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path,
                file_size_bytes: file_size,
                load_duration: Duration::ZERO,
                inference_results: Vec::new(),
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("Health check before load failed: {}", e)),
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
            };
        }

        self.log_resource_state("before-load", model_id);

        let load_start = Instant::now();
        let load_result = timeout(self.config.model_load_timeout, client.load_model(server_model_id)).await;
        let load_duration = load_start.elapsed();

        let load_result = match load_result {
            Ok(inner) => inner,
            Err(_) => {
                let msg = format!(
                    "Model load timed out after {}s (limit: {}s). Possible GPU driver issue — aborting to prevent system crash.",
                    load_duration.as_secs(),
                    self.config.model_load_timeout.as_secs()
                );
                warn!("[benchmark] [{}] {}", model_id, msg);
                return ModelBenchmarkResult {
                    model_id: model_id.to_string(),
                    model_path: resolved_path,
                    file_size_bytes: file_size,
                    load_duration,
                    inference_results: Vec::new(),
                    unload_duration: Duration::ZERO,
                    total_duration: start.elapsed(),
                    tokens_per_second: 0.0,
                    avg_latency_ms: 0.0,
                    p50_latency_ms: 0.0,
                    p95_latency_ms: 0.0,
                    p99_latency_ms: 0.0,
                    error: Some(msg),
                    gpu_mode: gpu_mode.to_string(),
                    speedup_factor: None,
                };
            }
        };

        if let Err(e) = load_result {
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path,
                file_size_bytes: file_size,
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

        for prompt in prompts {
            const MAX_RETRIES: u32 = 3;
            let mut last_error = None;

            for attempt in 0..MAX_RETRIES {
                let request = ChatCompletionRequest {
                    model: server_model_id.to_string(),
                    messages: vec![ChatMessage::user(prompt)],
                    max_tokens: Some(max_tokens),
                    temperature: Some(temperature as f32),
                    top_p: Some(top_p as f32),
                    stream: false,
                    ..Default::default()
                };

                let inf_start = Instant::now();
                match client.chat_completion(request).await {
                    Ok(resp) => {
                        let raw_response = resp.choices.first()
                            .map(|c| c.message.content.clone())
                            .unwrap_or_default();
                        let cleaned_text = Self::clean_response_text(&raw_response);

                        if cleaned_text != raw_response {
                            info!("[benchmark] response post-processed for {} (attempt {}, {} chars → {} chars)",
                                model_id, attempt + 1, raw_response.len(), cleaned_text.len());
                        }

                        if resp.usage.completion_tokens >= max_tokens {
                            warn!("[benchmark] {} output truncated at {} tokens (hit max_tokens limit)",
                                model_id, max_tokens);
                        }

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
                            response_text: cleaned_text,
                        });
                        last_error = None;
                        break;
                    }
                    Err(e) => {
                        last_error = Some(e.to_string());
                        warn!("[benchmark] inference attempt {}/{} failed for {}: {}",
                            attempt + 1, MAX_RETRIES, model_id, e);
                        if attempt + 1 < MAX_RETRIES {
                            sleep(Duration::from_secs(2u64.pow(attempt))).await;
                        }
                    }
                }
            }

            if let Some(err) = last_error {
                warn!("[benchmark] all {} inference attempts failed for {}: {}", MAX_RETRIES, model_id, err);
                inference_results.push(InferenceResult {
                    prompt: prompt.clone(),
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    total_tokens: 0,
                    duration: Duration::ZERO,
                    tokens_per_second: 0.0,
                    response_text: format!("ERROR: All {} attempts failed: {}", MAX_RETRIES, err),
                });
            }
        }

        let unload_start = Instant::now();
        let _ = client.unload_model(server_model_id).await;
        let unload_duration = unload_start.elapsed();

        self.log_resource_state("after-unload", model_id);

        info!("[benchmark] [{}] cooldown after unload: sleeping {}s", model_id, self.config.cooldown_after_unload.as_secs());
        sleep(self.config.cooldown_after_unload).await;
        info!("[benchmark] [{}] cooldown complete", model_id);

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
            model_path: resolved_path,
            file_size_bytes: file_size,
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
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(2),
            compare_gpu_cpu: true,
            output_dir: Some("./docs/benchmarks/outputs".to_string()),
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(3),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(300),
            min_tmp_space_mb: 1024,
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

    // --- Hook Executor Tests ---

    fn make_test_config() -> BenchmarkConfig {
        BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec!["test".to_string()],
            max_tokens: 100,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: Some("./test_output_hooks".to_string()),
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(60),
            min_tmp_space_mb: 512,
        }
    }

    fn make_hook_context(step_name: &str, iteration: usize, output: Option<&str>) -> HookContext {
        HookContext {
            step_name: step_name.to_string(),
            iteration,
            output: output.map(|s| s.to_string()),
            error_message: None,
            loop_type: "count".to_string(),
        }
    }

    #[test]
    fn test_execute_hook_log_creates_file() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let path_str = log_path.to_str().unwrap();

        let hook = serde_json::json!([
            {"log": {"to_file_path": path_str}}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("hello"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("step=step_1"), "log should contain step name");
        assert!(content.contains("timing=after_step_succeeds"), "log should contain timing");
    }

    #[test]
    fn test_execute_hook_save_to_writes_file() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let path_str = save_path.to_str().unwrap();

        let hook = serde_json::json!([
            {"save_to": path_str}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("saved content here"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&save_path).unwrap();
        assert_eq!(content, "saved content here", "save_to should write output content");
    }

    #[test]
    fn test_execute_hook_append_to_appends() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let append_path = dir.path().join("results.txt");
        let path_str = append_path.to_str().unwrap();

        // Write initial content
        std::fs::write(&append_path, "first\n").unwrap();

        let hook = serde_json::json!([
            {"append_to": path_str}
        ]);

        let ctx = make_hook_context("model_b", 2, Some("appended"));
        runner.execute_hook(&Some(hook), "step_2", "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&append_path).unwrap();
        assert!(content.starts_with("first\n"), "should preserve existing content");
        assert!(content.ends_with("appended"), "should append new content");
    }

    #[test]
    fn test_execute_hook_fail_returns_error() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let hook = serde_json::json!([
            {"fail": true}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("output"));
        let result = runner.execute_hook(&Some(hook), "step_1", "after_step_fails", &ctx);

        assert!(result.is_err(), "fail hook should return error");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Hook triggered fail"), "error message should mention hook fail");
    }

    #[test]
    fn test_execute_hook_none_does_nothing() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model_a", 1, Some("output"));
        let result = runner.execute_hook(&None, "step_1", "after_step_succeeds", &ctx);
        assert!(result.is_ok(), "None hook should succeed");
    }

    #[test]
    fn test_execute_hook_multiple_actions() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let log_path = dir.path().join("log.txt");

        let hook = serde_json::json!([
            {"save_to": save_path.to_str().unwrap()},
            {"log": {"to_file_path": log_path.to_str().unwrap()}}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("multi action"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        assert!(save_path.exists(), "save_to should create file");
        assert!(log_path.exists(), "log should create file");
        assert_eq!(std::fs::read_to_string(&save_path).unwrap(), "multi action");
    }

    // --- Template Interpolation Tests ---

    #[test]
    fn test_interpolate_template_current_model() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("my_model.gguf", 1, None);
        let result = runner.interpolate_template("output/{{current_model}}.json", &ctx);
        assert_eq!(result, "output/my_model.gguf.json");
    }

    #[test]
    fn test_interpolate_template_loop_current_model() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("test_model.gguf", 2, None);
        let result = runner.interpolate_template("output/{{loop.current_model}}.txt", &ctx);
        assert_eq!(result, "output/test_model.gguf.txt");
    }

    #[test]
    fn test_interpolate_template_step_output() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 1, Some("hello world"));
        let result = runner.interpolate_template("prefix_{{step.output}}_suffix", &ctx);
        assert_eq!(result, "prefix_hello world_suffix");
    }

    #[test]
    fn test_interpolate_template_iteration() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 5, None);
        let result = runner.interpolate_template("run_{{iteration}}.json", &ctx);
        assert_eq!(result, "run_5.json");
    }

    #[test]
    fn test_interpolate_template_loop_iteration() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 3, None);
        let result = runner.interpolate_template("step_{{loop.iteration}}", &ctx);
        assert_eq!(result, "step_3");
    }

    #[test]
    fn test_interpolate_template_multiple_vars() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model_a.gguf", 2, Some("result text"));
        let result = runner.interpolate_template(
            "./outputs/{{current_model}}/iter_{{iteration}}/out.txt",
            &ctx
        );
        assert_eq!(result, "./outputs/model_a.gguf/iter_2/out.txt");
    }

    #[test]
    fn test_interpolate_template_no_vars() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 1, None);
        let result = runner.interpolate_template("plain/path.txt", &ctx);
        assert_eq!(result, "plain/path.txt");
    }

    // --- Workflow Step Parser Tests ---

    #[test]
    fn test_load_workflow_steps_parses_yaml() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_workflow
name: Test
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  run_benchmark:
    id: bench_1
    requires: []
    prompt: "Test prompt"
    input:
      max_tokens: 256
    loop:
      count:
        max_iterations: 3
        iteration_variable: current_model
    when:
      after_step_succeeds:
        - log:
            to_file_path: "./test.log"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some(), "should parse workflow steps");
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1, "should have 1 step");
        assert_eq!(steps[0].step_name, "run_benchmark");
        assert_eq!(steps[0].step_id, "bench_1");
        assert!(steps[0].prompt.is_some());
        assert!(steps[0].r#loop.is_some());
    }

    #[test]
    fn test_load_workflow_steps_no_workflow_file() {
        let config = make_test_config(); // workflow_file: None
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();
        assert!(steps.is_none(), "should return None when no workflow file");
    }

    #[test]
    fn test_load_workflow_steps_multiple_steps() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("multi.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: multi_step
name: Multi
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  step_one:
    id: s1
    requires: []
    prompt: "First prompt"
  step_two:
    id: s2
    requires: [s1]
    prompt: "Second prompt"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 2, "should have 2 steps");
        assert_eq!(steps[0].step_id, "s1");
        assert_eq!(steps[1].step_id, "s2");
        assert_eq!(steps[1].requires, vec!["s1"]);
    }

    // --- Hook Execution Edge Case Tests ---

    #[test]
    fn test_execute_hook_log_creates_nested_dirs() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let nested_path = dir.path().join("deep/nested/path/test.log");

        // Create parent directories (execute_hook doesn't do this automatically)
        std::fs::create_dir_all(nested_path.parent().unwrap()).unwrap();

        let path_str = nested_path.to_str().unwrap();

        let hook = serde_json::json!([
            {"log": {"to_file_path": path_str}}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("hello"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        assert!(nested_path.exists(), "log hook should create log file in existing nested dirs");
        let content = std::fs::read_to_string(&nested_path).unwrap();
        assert!(content.contains("step=step_1"));
    }

    #[test]
    fn test_execute_hook_save_to_overwrites() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let path_str = save_path.to_str().unwrap();

        // Write initial content
        std::fs::write(&save_path, "original content").unwrap();

        let hook = serde_json::json!([
            {"save_to": path_str}
        ]);

        let ctx = make_hook_context("model_a", 1, Some("new content"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&save_path).unwrap();
        assert_eq!(content, "new content", "save_to should overwrite existing file");
    }

    #[test]
    fn test_execute_hook_append_to_creates_new_file() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let append_path = dir.path().join("results.txt");
        let path_str = append_path.to_str().unwrap();

        // Don't create the file beforehand

        let hook = serde_json::json!([
            {"append_to": path_str}
        ]);

        let ctx = make_hook_context("model_b", 1, Some("first append"));
        runner.execute_hook(&Some(hook), "step_2", "after_step_succeeds", &ctx).unwrap();

        assert!(append_path.exists(), "append_to should create file if not exists");
        let content = std::fs::read_to_string(&append_path).unwrap();
        assert_eq!(content, "first append");
    }

    #[test]
    fn test_execute_hook_empty_object() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("test.log");

        // Empty object in hook array
        let hook = serde_json::json!([{}]);

        let ctx = make_hook_context("model_a", 1, Some("hello"));
        runner.execute_hook(&Some(hook), "step_1", "after_step_succeeds", &ctx).unwrap();

        // File should not be created since there's no valid action
        assert!(!log_path.exists(), "empty hook object should do nothing");
    }

    // --- Template Interpolation Edge Case Tests ---

    #[test]
    fn test_interpolate_template_unknown_variable() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 1, Some("output"));
        // Unknown variable should be left as-is
        let result = runner.interpolate_template("path/{{unknown_var}}/file.txt", &ctx);
        assert_eq!(result, "path/{{unknown_var}}/file.txt");
    }

    #[test]
    fn test_interpolate_template_empty() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model", 1, None);
        let result = runner.interpolate_template("", &ctx);
        assert_eq!(result, "");
    }

    #[test]
    fn test_interpolate_template_only_markers() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model_a", 3, Some("out"));
        let result = runner.interpolate_template("{{current_model}}{{iteration}}{{step.output}}", &ctx);
        assert_eq!(result, "model_a3out");
    }

    #[test]
    fn test_interpolate_template_adjacent_vars() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model_x", 5, Some("result"));
        let result = runner.interpolate_template("{{iteration}}{{current_model}}", &ctx);
        assert_eq!(result, "5model_x");
    }

    #[test]
    fn test_interpolate_template_special_chars() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_hook_context("model:with/special\\chars", 1, Some("output with \"quotes\""));
        let result = runner.interpolate_template("prefix_{{step.output}}_{{current_model}}", &ctx);
        assert_eq!(result, "prefix_output with \"quotes\"_model:with/special\\chars");
    }

    // --- Workflow Step Parsing Edge Case Tests ---

    #[test]
    fn test_load_workflow_steps_no_prompt() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_workflow
name: Test
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  run_step:
    id: step_1
    requires: []
    input:
      max_tokens: 256
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].step_name, "run_step");
        assert!(steps[0].prompt.is_none(), "step without prompt should have None");
    }

    #[test]
    fn test_load_workflow_steps_empty_when() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_workflow
name: Test
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  run_step:
    id: step_1
    requires: []
    prompt: "test"
    when:
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1);
        // Empty when clause should still parse as Some(serde_json::Value::Null)
        assert!(steps[0].when.is_some());
    }

    #[test]
    fn test_load_workflow_steps_no_schema_version() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_workflow
name: Test
version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  run_step:
    id: step_1
    requires: []
    prompt: "test"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        // Should still parse for MVP
        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].step_name, "run_step");
    }

    #[test]
    fn test_load_workflow_steps_loop_no_iteration_var() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_workflow
name: Test
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  run_step:
    id: step_1
    requires: []
    prompt: "test"
    loop:
      count:
        max_iterations: 3
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1);
        assert!(steps[0].r#loop.is_some(), "loop without iteration_var should still parse");
    }

    // --- End-to-End Workflow Pipeline Tests ---

    #[test]
    fn test_workflow_pipeline_parse_and_hooks() {
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let log_path = dir.path().join("log.txt");

        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: test_pipeline
name: Test Pipeline
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  step_one:
    id: s1
    requires: []
    prompt: "First prompt"
    when:
      after_step_succeeds:
        - save_to: "output.txt"
        - log:
            to_file_path: "log.txt"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);

        // Parse steps
        let steps = runner.load_workflow_steps();
        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].step_name, "step_one");

        // Extract hook array from when clause (when is object with timing keys)
        let hook = steps[0].when.as_ref().and_then(|w| w.get("after_step_succeeds")).cloned();
        
        // Change CWD to tempdir for relative paths
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        
        let ctx = make_hook_context("model_test", 1, Some("test output"));
        runner.execute_hook(&hook, "step_one", "after_step_succeeds", &ctx).unwrap();
        
        std::env::set_current_dir(original_cwd).unwrap();

        // Verify output files exist
        assert!(save_path.exists(), "save_to hook should create output file");
        assert!(log_path.exists(), "log hook should create log file");

        let save_content = std::fs::read_to_string(&save_path).unwrap();
        assert_eq!(save_content, "test output");

        let log_content = std::fs::read_to_string(&log_path).unwrap();
        assert!(log_content.contains("step=step_one"));
        assert!(log_content.contains("timing=after_step_succeeds"));

        std::fs::remove_file(save_path).ok();
        std::fs::remove_file(log_path).ok();
    }

    #[test]
    fn test_workflow_pipeline_multi_step_ordering() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("multi.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: multi_step_ordering
name: Multi Step Ordering
version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  step_first:
    id: step_1
    requires: []
    prompt: "First step prompt"

  step_second:
    id: step_2
    requires: [step_1]
    prompt: "Second step prompt"

  step_third:
    id: step_3
    requires: [step_2]
    prompt: "Third step prompt"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 3);

        // Verify ordering matches YAML order
        assert_eq!(steps[0].step_id, "step_1");
        assert_eq!(steps[0].step_name, "step_first");
        assert_eq!(steps[0].requires.len(), 0);

        assert_eq!(steps[1].step_id, "step_2");
        assert_eq!(steps[1].step_name, "step_second");
        assert_eq!(steps[1].requires, vec!["step_1"]);

        assert_eq!(steps[2].step_id, "step_3");
        assert_eq!(steps[2].step_name, "step_third");
        assert_eq!(steps[2].requires, vec!["step_2"]);
    }
}
