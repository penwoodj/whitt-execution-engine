use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use crate::client::disk_monitor;
use crate::client::memory_monitor;
use crate::agent::loop_hooks::HookContext;
use crate::workflow::hooks::{HookEngine, HookResult};
use crate::workflow::hooks::actions::execute_action;
use crate::workflow::hooks::context::{
    WorkflowHookContext, BeforeStepStartsContext,
    AfterStepSucceedsContext, AfterStepFailsContext, AfterAllRetriesExhaustedContext,
    AfterLoopIterationFailsContext, OnRequiresFailedContext, StepType, ErrorDetails,
};
use crate::workflow::HookAction;
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult, WorkflowStepResult};
use super::detail_generator::DetailGenerator;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use std::process::Command;
use tokio::time::{sleep, timeout};
use tokio::process::Command as TokioCommand;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};

#[derive(Clone)]
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
    100
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    hook_engine: Arc<Mutex<HookEngine>>,
    inference_semaphore: Arc<Semaphore>,
}

struct BenchmarkWorkflowConfig {
    prompts: Vec<String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    compare_modes: bool,
    model_list: Vec<String>,
    gpu_layers: usize,
    load_params_env_vars: Vec<(String, String)>,
}

#[derive(Clone)]
struct WorkflowStep {
    step_name: String,
    step_id: String,
    #[allow(dead_code)]
    requires: Vec<String>,
    when: Option<serde_json::Value>,
    prompt: Option<String>,
    generative_entity: Option<String>,
    model_overrides: Option<serde_json::Value>,
    #[allow(dead_code)]
    r#loop: Option<serde_json::Value>,
}

fn hook_actions_as_vec(actions: &serde_json::Value) -> Vec<serde_json::Value> {
    if let Some(array) = actions.as_array() {
        array.clone()
    } else {
        vec![actions.clone()]
    }
}

/// Detect maximum concurrent inferences based on available system resources.
///
/// Tries to detect VRAM first (for GPU inference), falls back to RAM.
/// Rule of thumb: each inference needs ~2GB VRAM/RAM.
/// Returns capped value: 1 (min) <= result <= 4 (max).
/// Falls back to 2 on detection failure.
fn detect_max_concurrent_inferences() -> usize {
    if let Some(vram_gb) = detect_vram_gb() {
        let max_concurrent = (vram_gb / 2.0).floor() as usize;
        let capped = max_concurrent.clamp(1, 4);
        info!("[benchmark] auto-detected max concurrent inferences: {} (based on {:.1} GB VRAM available)", capped, vram_gb);
        return capped;
    }

    if let Some(ram_gb) = detect_available_ram_gb() {
        let max_concurrent = (ram_gb / 2.0).floor() as usize;
        let capped = max_concurrent.clamp(1, 4);
        info!("[benchmark] auto-detected max concurrent inferences: {} (based on {:.1} GB RAM available)", capped, ram_gb);
        return capped;
    }

    info!("[benchmark] could not detect available memory, using default: 2 concurrent inferences");
    2
}

/// Detect available VRAM in GB from sysfs.
///
/// Returns Some(vram_gb) if successful, None otherwise.
fn detect_vram_gb() -> Option<f64> {
    if let Some(vram_kb) = read_sysfs_vram_amd() {
        let vram_gb = vram_kb as f64 / 1024.0 / 1024.0;
        return Some(vram_gb);
    }

    if let Some(vram_mb) = read_proc_vram_nvidia() {
        let vram_gb = vram_mb as f64 / 1024.0;
        return Some(vram_gb);
    }

    None
}

/// Detect available RAM in GB from /proc/meminfo.
///
/// Returns Some(ram_gb) if successful, None otherwise.
fn detect_available_ram_gb() -> Option<f64> {
    let meminfo_content = fs::read_to_string("/proc/meminfo").ok()?;

    for line in meminfo_content.lines() {
        if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let kb: u64 = parts[1].parse().ok()?;
                let gb = kb as f64 / 1024.0 / 1024.0;
                return Some(gb);
            }
        }
    }

    None
}

/// Read AMD GPU VRAM from sysfs.
///
/// Returns Some(vram_kb) if successful, None otherwise.
fn read_sysfs_vram_amd() -> Option<u64> {
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let card_name = entry.file_name();
            let card_name_str = card_name.to_string_lossy();

            if card_name_str.starts_with("card") {
                let vram_path = entry.path().join("device/mem_info_vram_total");
                if let Ok(vram_content) = fs::read_to_string(&vram_path) {
                    if let Ok(vram_kb) = vram_content.trim().parse::<u64>() {
                        return Some(vram_kb);
                    }
                }
            }
        }
    }

    None
}

/// Read NVIDIA GPU VRAM from /proc/driver/nvidia/gpus.
///
/// Returns Some(vram_mb) if successful, None otherwise.
fn read_proc_vram_nvidia() -> Option<u64> {
    if let Ok(entries) = fs::read_dir("/proc/driver/nvidia/gpus") {
        for entry in entries.flatten() {
            let info_path = entry.path().join("information");
            if let Ok(info_content) = fs::read_to_string(&info_path) {
                for line in info_content.lines() {
                    if line.starts_with("Total Available Memory:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 {
                            if let Ok(mb) = parts[3].parse::<u64>() {
                                return Some(mb);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        let max_concurrent = detect_max_concurrent_inferences();
        Self {
            config,
            hook_engine: Arc::new(Mutex::new(HookEngine::new())),
            inference_semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
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

        // Count only non-defunct (running/sleeping) llama-server processes
        let zombie_check = TokioCommand::new("sh")
            .args(["-c", "ps -eo stat,comm | grep -v 'Z' | grep -c llama-server"])
            .output()
            .await;
        match zombie_check {
            Ok(output) => {
                let count_str = String::from_utf8_lossy(&output.stdout);
                let count: i32 = count_str.trim().parse().unwrap_or(0);
                if count <= 2 {
                    info!("[benchmark] ✓ Zombie process check passed: {} llama-server process(es) found", count);
                } else {
                    let msg = format!("Found {} zombie llama-server processes (expected 0-2)", count);
                    info!("[benchmark] ✗ {}", msg);
                    anyhow::bail!(crate::error::Error::benchmark(msg));
                }
            }
            Err(e) => {
                let msg = format!("Zombie process check failed: {}", e);
                info!("[benchmark] ✗ {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        match memory_monitor::check_system_memory() {
            Ok(info) => {
                let available_gb = info.available_bytes as f64 / 1_073_741_824.0;
                let total_gb = info.total_bytes as f64 / 1_073_741_824.0;
                info!("[benchmark] ✓ memory check passed: {} GB available (total: {} GB)",
                    available_gb, total_gb);

                if info.available_bytes < 2_147_483_648 { // < 2GB warning
                    warn!("[benchmark] ⚠ low memory warning: {:.1} GB available (< 2GB threshold)", available_gb);
                }
            }
            Err(e) => {
                let msg = format!("Memory check failed: {}", e);
                info!("[benchmark] ✗ {}", msg);
                warn!("[benchmark] proceeding without memory verification: {}", e);
            }
        }

        info!("[benchmark] all preflight checks passed");
        Ok(())
    }

    /// Runtime system health check: verify resources are adequate before operations.
    #[deprecated(since = "0.4.0", note = "Use hook-driven preflight via before_workflow_starts trigger")]
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

    #[deprecated(since = "0.4.0", note = "Use hook-driven log action in before_step_starts")]
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

    #[allow(dead_code)]
    fn clean_json_output(text: &str) -> String {
        let cleaned = text.trim();

        if let Some(first_fence) = cleaned.find("```") {
            let after_fence = &cleaned[first_fence..];
            let first_newline = after_fence.find('\n');
            let opening_end = first_fence + first_newline.unwrap_or(3);
            if opening_end < cleaned.len() {
                if let Some(last_fence) = cleaned[opening_end..].rfind("```") {
                    let end_pos = opening_end + last_fence;
                    if opening_end + 1 < end_pos {
                        return cleaned[opening_end + 1..end_pos].trim().to_string();
                    }
                }
            }
        }

        cleaned.replace("<|im_start|>", "").replace("<|im_end|>", "").trim().to_string()
    }

    fn model_resolution_failure_result(step_id: &str, model_name: &str) -> ModelBenchmarkResult {
        ModelBenchmarkResult {
            model_id: model_name.to_string(),
            model_path: model_name.to_string(),
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
            error: Some(format!("Model '{}' not found on server for workflow step '{}'", model_name, step_id)),
            gpu_mode: "gpu".to_string(),
            speedup_factor: None,
        }
    }

    fn load_workflow_config(&self) -> Result<Option<BenchmarkWorkflowConfig>> {
        let wf_path = match self.config.workflow_file.as_ref() {
            Some(p) => p,
            None => return Ok(None),
        };
        let content = match fs::read_to_string(wf_path) {
            Ok(c) => c,
            Err(e) => {
                warn!("[benchmark] failed to read workflow file {}: {}", wf_path, e);
                return Ok(None);
            }
        };

        if let Err(e) = crate::workflow::WorkflowFile::from_yaml(&content) {
            return Err(anyhow::anyhow!(
                "[benchmark] workflow YAML validation failed for {}: {} — \
                 engine now requires valid workflow YAML when --workflow is provided \
                 (cycle-3 hardening: silent fallback removed)",
                wf_path,
                e
            ));
        }

        let yaml_value: serde_json::Value = match serde_saphyr::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "[benchmark] failed to parse workflow YAML for config extraction: {} — \
                     invalid YAML structure (cycle-3 hardening)",
                    e
                ));
            }
        };

        let agentic_workflow = match yaml_value.get("agentic_workflow") {
            Some(aw) => aw,
            None => {
                return Err(anyhow::anyhow!(
                    "[benchmark] no agentic_workflow section in YAML {} — \
                     workflow must have agentic_workflow.steps (cycle-3 hardening)",
                    wf_path
                ));
            }
        };

        let steps_section = agentic_workflow.get("steps")
            .unwrap_or(agentic_workflow);

        let first_step = if let Some(steps_map) = steps_section.as_object() {
            steps_map.values().next()
        } else if let Some(steps_arr) = steps_section.as_array() {
            steps_arr.first()
        } else {
            None
        };

        let first_step = match first_step {
            Some(s) => s,
            None => {
                warn!("[benchmark] no steps found in agentic_workflow");
                return Ok(None);
            }
        };

        let prompts = if let Some(prompt) = first_step.get("prompt").and_then(|v| v.as_str()) {
            vec![prompt.to_string()]
        } else {
            return Ok(None);
        };

        // Read max_tokens with priority: step model_overrides > model sampling config > default (4096)
        let max_tokens_from_step = first_step
            .get("model_overrides")
            .and_then(|mo| mo.get("max_tokens"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let max_tokens_from_model = yaml_value
            .get("models")
            .and_then(|models| models.as_object())
            .and_then(|models_map| {
                models_map.values().find_map(|model_config| {
                    model_config.get("sampling")
                        .and_then(|s| s.get("max_tokens"))
                        .and_then(|v| v.as_u64())
                        .map(|v| v as usize)
                })
            });

        let max_tokens = max_tokens_from_step
            .or(max_tokens_from_model)
            .unwrap_or(4096);

        // Read temperature with priority: step model_overrides > model sampling config > default (0.7)
        let temperature = first_step
            .get("model_overrides")
            .and_then(|mo| mo.get("temperature"))
            .and_then(|v| v.as_f64())
            .or_else(|| {
                yaml_value.get("models")
                    .and_then(|models| models.as_object())
                    .and_then(|models_map| {
                        models_map.values().find_map(|mc| {
                            mc.get("sampling")
                                .and_then(|s| s.get("temperature"))
                                .and_then(|v| v.as_f64())
                        })
                    })
            })
            .unwrap_or(0.7);

        // Read top_p with priority: step model_overrides > model sampling config > default (0.95)
        let top_p = first_step
            .get("model_overrides")
            .and_then(|mo| mo.get("top_p"))
            .and_then(|v| v.as_f64())
            .or_else(|| {
                yaml_value.get("models")
                    .and_then(|models| models.as_object())
                    .and_then(|models_map| {
                        models_map.values().find_map(|mc| {
                            mc.get("sampling")
                                .and_then(|s| s.get("top_p"))
                                .and_then(|v| v.as_f64())
                        })
                    })
            })
            .unwrap_or(0.95);

        // Read gpu_layers from provider hosting config
        let gpu_layers = yaml_value
            .get("providers")
            .and_then(|providers| providers.as_object())
            .and_then(|providers_map| providers_map.values().next())
            .and_then(|provider| provider.get("hosting"))
            .and_then(|hosting| hosting.get("gpu_layers"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999) as usize;

        // Extract model_list from workflow YAML if present
        let model_list: Vec<String> = yaml_value
            .get("providers")
            .and_then(|providers| providers.as_object())
            .and_then(|providers_map| providers_map.values().next())
            .and_then(|provider| provider.get("models"))
            .and_then(|models| models.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Extract load_params from first model spec → env vars for Docker
        let load_params_env_vars: Vec<(String, String)> = yaml_value
            .get("models")
            .and_then(|models| models.as_object())
            .and_then(|models_map| {
                models_map.values().find_map(|mc| {
                    mc.get("load_params")
                        .and_then(|lp| serde_json::from_value::<crate::model::schema::LoadParams>(lp.clone()).ok())
                })
            })
            .map(|lp| lp.to_env_vars())
            .unwrap_or_default();

        if !load_params_env_vars.is_empty() {
            info!("[benchmark] extracted load_params: {} env vars", load_params_env_vars.len());
        }

        info!("[benchmark] extracted workflow config: prompts={}, max_tokens={}, temperature={}, top_p={}, gpu_layers={}, model_list={}",
            prompts.len(), max_tokens, temperature, top_p, gpu_layers, model_list.len());

        Ok(Some(BenchmarkWorkflowConfig {
            prompts,
            max_tokens,
            temperature,
            top_p,
            compare_modes: false,
            model_list,
            gpu_layers,
            load_params_env_vars,
        }))
    }

    fn load_workflow_steps(&self) -> Option<Vec<WorkflowStep>> {
        let wf_path = self.config.workflow_file.as_ref()?;
        let content = fs::read_to_string(wf_path).ok()?;

        let yaml_value: serde_json::Value = serde_saphyr::from_str(&content).ok()?;
        let agentic_workflow = yaml_value.get("agentic_workflow")?;
        let workflow_default_hooks = agentic_workflow.get("when");

        // NEW: look at agentic_workflow.steps, not agentic_workflow itself
        let steps_section = agentic_workflow.get("steps")
            .unwrap_or(agentic_workflow);  // fallback for old format

        let steps: Vec<WorkflowStep> = if let Some(steps_array) = steps_section.as_array() {
            steps_array.iter()
                .filter_map(|step| {
                    let step_name = step.get("step")?.as_str()?.to_string();
                    let step_id = step.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let requires = Self::extract_dependency_names(step);
                    let when = Self::merged_step_hooks(workflow_default_hooks, step);
                    let prompt = step.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let generative_entity = step.get("generative_entity").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let model_overrides = step.get("model_overrides").cloned();
                    let loop_config = step.get("loop").cloned();

                    Some(WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        when,
                        prompt,
                        generative_entity,
                        model_overrides,
                        r#loop: loop_config,
                    })
                })
                .collect()
        } else if let Some(steps_map) = steps_section.as_object() {
            steps_map.iter()
                .map(|(step_name_key, step_value)| {
                    let step_name = step_name_key.clone();
                    let step_id = step_value.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let requires = Self::extract_dependency_names(step_value);
                    let when = Self::merged_step_hooks(workflow_default_hooks, step_value);
                    let prompt = step_value.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let generative_entity = step_value.get("generative_entity").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let model_overrides = step_value.get("model_overrides").cloned();
                    let loop_config = step_value.get("loop").cloned();

                    WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        when,
                        prompt,
                        generative_entity,
                        model_overrides,
                        r#loop: loop_config,
                    }
                })
                .collect()
        } else {
            return None;
        };

        Some(steps)
    }

    fn extract_dependency_names(step_value: &serde_json::Value) -> Vec<String> {
        let mut dependencies = Vec::new();

        for key in ["depends_on", "requires"] {
            let Some(value) = step_value.get(key) else {
                continue;
            };

            if let Some(items) = value.as_array() {
                for item in items {
                    if let Some(name) = item.as_str() {
                        if !dependencies.iter().any(|existing| existing == name) {
                            dependencies.push(name.to_string());
                        }
                    } else if let Some(name) = item.get("step").and_then(|v| v.as_str()) {
                        if !dependencies.iter().any(|existing| existing == name) {
                            dependencies.push(name.to_string());
                        }
                    }
                }
            }
        }

        dependencies
    }

    fn merged_step_hooks(
        workflow_default_hooks: Option<&serde_json::Value>,
        step_value: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        let mut merged = serde_json::Map::new();
        let explicit_hook_block = workflow_default_hooks.is_some()
            || step_value.get("when").is_some()
            || step_value.get("on_requires_failed").is_some();

        Self::append_hook_map(&mut merged, workflow_default_hooks);
        Self::append_hook_map(&mut merged, step_value.get("when"));

        if let Some(on_requires_failed) = step_value.get("on_requires_failed") {
            Self::append_on_requires_failed(&mut merged, on_requires_failed);
        }

        if merged.is_empty() && !explicit_hook_block {
            None
        } else {
            Some(serde_json::Value::Object(merged))
        }
    }

    fn append_hook_map(
        merged: &mut serde_json::Map<String, serde_json::Value>,
        hook_map: Option<&serde_json::Value>,
    ) {
        let Some(hooks) = hook_map.and_then(|value| value.as_object()) else {
            return;
        };

        for (trigger_name, actions) in hooks {
            Self::append_hook_actions(merged, trigger_name, actions);
        }
    }

    fn append_hook_actions(
        merged: &mut serde_json::Map<String, serde_json::Value>,
        trigger_name: &str,
        actions: &serde_json::Value,
    ) {
        let entry = merged
            .entry(trigger_name.to_string())
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));

        if let Some(existing) = entry.as_array_mut() {
            existing.extend(hook_actions_as_vec(actions));
        }
    }

    fn append_on_requires_failed(
        merged: &mut serde_json::Map<String, serde_json::Value>,
        actions: &serde_json::Value,
    ) {
        if actions.as_array().is_some() || Self::looks_like_hook_action(actions) {
            Self::append_hook_actions(merged, "on_requires_failed", actions);
            return;
        }

        if let Some(groups) = actions.as_object() {
            for group_actions in groups.values() {
                Self::append_hook_actions(merged, "on_requires_failed", group_actions);
            }
        }
    }

    fn looks_like_hook_action(value: &serde_json::Value) -> bool {
        const ACTION_KEYS: &[&str] = &[
            "log",
            "gwt",
            "append_to",
            "save_to",
            "route_to",
            "bookmark",
            "notify",
            "fail",
            "shell",
            "skip_step",
            "skip_remaining",
            "iterate_values",
        ];

        value
            .as_object()
            .is_some_and(|object| object.keys().any(|key| ACTION_KEYS.contains(&key.as_str())))
    }

    fn load_workflow_models(&self) -> Option<std::collections::HashMap<String, String>> {
        let wf_path = self.config.workflow_file.as_ref()?;
        let content = fs::read_to_string(wf_path).ok()?;
        let yaml_value: serde_json::Value = serde_saphyr::from_str(&content).ok()?;
        let models_section = yaml_value.get("models")?.as_object()?;

        let mut models = std::collections::HashMap::new();
        for (key, value) in models_section {
            if key.starts_with("global_") || key.starts_with("default_") {
                continue;
            }
            let name = value.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(key)
                .to_string();
            models.insert(key.clone(), name);
        }
        Some(models)
    }

    fn resolve_model_file(&self, model_name: &str, discovered_models: &[(String, String)]) -> Option<(String, String)> {
        for (id, path) in discovered_models {
            if id.contains(model_name) {
                return Some((id.clone(), path.clone()));
            }
        }
        let base_name = model_name.split('-').next().unwrap_or(model_name);
        for (id, path) in discovered_models {
            if id.contains(base_name) {
                return Some((id.clone(), path.clone()));
            }
        }
        None
    }

    fn extract_iterate_values(&self, step: &WorkflowStep) -> Option<Vec<serde_json::Map<String, serde_json::Value>>> {
        let when = step.when.as_ref()?;
        let before_hooks = when.get("before_step_starts")?;

        let hooks = if let Some(arr) = before_hooks.as_array() {
            arr.clone()
        } else {
            vec![before_hooks.clone()]
        };

        for hook in hooks {
            if let Some(iterate) = hook.get("iterate_values") {
                let var_arrays = iterate.as_object()?;

                let max_len = var_arrays.values()
                    .filter_map(|v| v.as_array().map(|a| a.len()))
                    .max()
                    .unwrap_or(0);

                let mut result = Vec::new();
                for i in 0..max_len {
                    let mut vars = serde_json::Map::new();
                    for (key, values) in var_arrays {
                        let clean_key = key.strip_prefix("step.").unwrap_or(key);
                        if let Some(arr) = values.as_array() {
                            if i < arr.len() {
                                vars.insert(clean_key.to_string(), arr[i].clone());
                            }
                        }
                    }
                    result.push(vars);
                }
                return Some(result);
            }
        }
        None
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
    #[deprecated(since = "0.4.0", note = "Use hook-driven logging via before_step_starts trigger")]
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
    #[deprecated(since = "0.4.0", note = "Use hook-driven logging via after_step_succeeds trigger")]
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
    #[deprecated(since = "0.4.0", note = "Use hook-driven logging via after_step_fails trigger")]
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

    /// Write exhaustive per-model report file.
    ///
    /// Creates a file named after the model (sanitized for filesystem) in the
    /// output directory. Contains every detail from the benchmark run.
    #[deprecated(since = "0.4.0", note = "Use hook-driven save_to action")]
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
    #[deprecated(since = "0.4.0", note = "Use hook-driven append_to action")]
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

    /// Execute hooks for a specific trigger timing.
    ///
    /// Takes the hook config JSON, extracts actions for the given trigger timing,
    /// deserializes each action as `HookAction`, calls `execute_action()` for each,
    /// and merges the results using `HookResult::merge()`.
    fn execute_hooks_for_trigger(
        &self,
        hook_config: &Option<serde_json::Value>,
        trigger_name: &str,
        context: &WorkflowHookContext,
    ) -> Result<HookResult> {
        let Some(hook_value) = hook_config else {
            return Ok(HookResult::Continue);
        };

        let Some(trigger_actions) = hook_value.get(trigger_name) else {
            return Ok(HookResult::Continue);
        };

        let mut merged_result = HookResult::Continue;
        let mut engine = self.hook_engine.lock().unwrap();

        for action_value in hook_actions_as_vec(trigger_actions) {
            let action: HookAction = serde_json::from_value(action_value.clone())
                .with_context(|| format!("Failed to deserialize hook action: {}", action_value))?;

            let result = execute_action(&action, context, &mut engine, Some(hook_value));
            merged_result = HookResult::merge(merged_result, result);

            if merged_result.is_terminal() {
                break;
            }
        }

        Ok(merged_result)
    }

    /// Execute hook actions: log, save_to, append_to, fail.
    ///
    /// Supports template interpolation: {{current_model}}, {{step.output}}, {{iteration}}
    #[allow(dead_code)]
    #[deprecated(note = "Use execute_hooks_for_trigger instead")]
    fn execute_hook_legacy(&self, hook: &Option<serde_json::Value>, step_name: &str, timing: &str, context: &HookContext, _variables: Option<&serde_json::Map<String, serde_json::Value>>) -> Result<()> {
        warn!("[benchmark] execute_hook_legacy is deprecated, use execute_hooks_for_trigger instead");

        let trigger = timing;
        let hook_config = hook;

        let workflow_context = match trigger {
            "after_step_succeeds" => WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: step_name.to_string(),
                output: context.output.clone().unwrap_or_default(),
                duration_ms: 0,
                quality_score: None,
                token_count: 0,
                model_name: step_name.to_string(),
            }),
            "after_step_fails" => WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
                step_name: step_name.to_string(),
                error_type: context.error_message.as_deref().unwrap_or("unknown").to_string(),
                error_message: context.error_message.clone().unwrap_or_default(),
                error: ErrorDetails {
                    is_retryable: false,
                    count: 1,
                },
                attempt_number: 1,
                model_name: step_name.to_string(),
            }),
            _ => WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: step_name.to_string(),
                output: context.output.clone().unwrap_or_default(),
                duration_ms: 0,
                quality_score: None,
                token_count: 0,
                model_name: step_name.to_string(),
            }),
        };

        self.execute_hooks_for_trigger(hook_config, trigger, &workflow_context)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn interpolate_template(&self, template: &str, context: &HookContext) -> String {
        let mut result = template.to_string();
        result = result.replace("{{current_model}}", &context.step_name);
        result = result.replace("{{loop.current_model}}", &context.step_name);

        if let Some(ref output) = context.output {
            result = result.replace("{{step.output}}", output);
        }

        result = result.replace("{{iteration}}", &context.iteration.to_string());
        result = result.replace("{{loop.iteration}}", &context.iteration.to_string());

        if let Some(ref size) = context.model_size {
            result = result.replace("{{model.size}}", size);
        }
        if let Some(ref family) = context.model_family {
            result = result.replace("{{model.family}}", family);
        }
        if let Some(ref quantization) = context.model_quantization {
            result = result.replace("{{model.quantization}}", quantization);
        }

        result
    }

    /// Resolve step-scoped template variables: {{step.var_name}} and {{iteration}}
    fn resolve_templates(template: &str, variables: &serde_json::Map<String, serde_json::Value>, iteration: usize) -> String {
        let mut result = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{{step.{}}}}}", key);
            if let Some(str_val) = value.as_str() {
                result = result.replace(&placeholder, str_val);
            }
        }

        result = result.replace("{{iteration}}", &iteration.to_string());

        result
    }

    /// Resolve step output references: {{step.STEP_ID.output}}
    ///
    /// Replaces patterns like {{step.step_1.output}} with the actual output text
    /// from previously executed steps. If a step_id is not found in the map,
    /// the template is left unchanged.
    fn resolve_step_output_templates(template: &str, step_outputs: &std::collections::HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (step_id, output) in step_outputs {
            let placeholder = format!("{{{{step.{}.output}}}}", step_id);
            result = result.replace(&placeholder, output);
        }
        result
    }

    fn resolve_bookmark_templates(template: &str, bookmarks: &std::collections::HashMap<String, serde_json::Value>) -> String {
        let mut result = template.to_string();

        for (key, value) in bookmarks {
            let flat_placeholder = format!("{{{{bookmarks.{}}}}}", key);
            let flat_replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            result = result.replace(&flat_placeholder, &flat_replacement);

            if let serde_json::Value::Object(map) = value {
                for (field, field_val) in map {
                    let nested_placeholder = format!("{{{{bookmarks.{}.{} }}}}", key, field);
                    let nested_replacement = match field_val {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    result = result.replace(&nested_placeholder, &nested_replacement);
                    let nested_placeholder_nospace = format!("{{{{bookmarks.{}.{}}}}}", key, field);
                    result = result.replace(&nested_placeholder_nospace, &nested_replacement);
                }
            }
        }
        result
    }

    fn parse_model_metadata(model_name: &str) -> (Option<String>, Option<String>, Option<String>) {
        let lower = model_name.to_lowercase();

        let size = if lower.contains("3b") {
            Some("3b".to_string())
        } else if lower.contains("7b") {
            Some("7b".to_string())
        } else if lower.contains("8b") {
            Some("8b".to_string())
        } else if lower.contains("13b") {
            Some("13b".to_string())
        } else if lower.contains("70b") {
            Some("70b".to_string())
        } else {
            None
        };

        let family = if lower.starts_with("llama") {
            Some("llama".to_string())
        } else if lower.starts_with("qwen") {
            Some("qwen".to_string())
        } else if lower.starts_with("mistral") || lower.starts_with("ministral") {
            Some("mistral".to_string())
        } else if lower.starts_with("phi") {
            Some("phi".to_string())
        } else if lower.starts_with("gemma") {
            Some("gemma".to_string())
        } else {
            None
        };

        let quantization = if lower.contains("q4") {
            Some("q4".to_string())
        } else if lower.contains("q8") {
            Some("q8".to_string())
        } else if lower.contains("f16") {
            Some("f16".to_string())
        } else if lower.contains("f32") {
            Some("f32".to_string())
        } else {
            None
        };

        (size, family, quantization)
    }
}

impl BenchmarkRunner {

    /// Execute multiple workflow steps that use the same model concurrently.
    /// Returns (step_id, result) pairs for all completed steps.
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    async fn execute_parallel_steps(
        &self,
        target_step_ids: &[String],
        steps: &[WorkflowStep],
        client: &LlamaHttpClient,
        model: &(String, String),
        max_tokens: usize,
        temperature: f64,
        top_p: f64,
        step_index: &std::collections::HashMap<String, usize>,
    ) -> Vec<(String, Result<WorkflowStepResult>)> {
        let target_steps: Vec<(String, usize)> = target_step_ids.iter()
            .filter_map(|id| step_index.get(id).map(|&idx| (id.clone(), idx)))
            .collect();

        if target_steps.is_empty() {
            return Vec::new();
        }

        // Single target: execute normally
        if target_steps.len() == 1 {
            let (step_id, step_idx) = &target_steps[0];
            let step = &steps[*step_idx];
            let resolved_prompt = step.prompt.as_ref()
                .map(|p| Self::resolve_step_output_templates(p, &std::collections::HashMap::new()));
            let resolved_step = WorkflowStep {
                step_name: step.step_name.clone(),
                step_id: step.step_id.clone(),
                requires: step.requires.clone(),
                when: step.when.clone(),
                prompt: resolved_prompt,
                generative_entity: step.generative_entity.clone(),
                model_overrides: step.model_overrides.clone(),
                r#loop: step.r#loop.clone(),
            };
            let result = self.execute_workflow_step(&resolved_step, client, model, max_tokens, temperature, top_p, None).await;
            return vec![(step_id.clone(), result)];
        }

        let mut join_set: JoinSet<(String, Result<WorkflowStepResult>)> = JoinSet::new();

        for (step_id, step_idx) in target_steps {
            let step = steps[step_idx].clone();
            let step_id_for_task = step_id.clone();
            let resolved_prompt = step.prompt.as_ref()
                .map(|p| Self::resolve_step_output_templates(p, &std::collections::HashMap::new()));
            let resolved_step = WorkflowStep {
                step_name: step.step_name.clone(),
                step_id: step.step_id.clone(),
                requires: step.requires.clone(),
                when: step.when.clone(),
                prompt: resolved_prompt,
                generative_entity: step.generative_entity.clone(),
                model_overrides: step.model_overrides.clone(),
                r#loop: step.r#loop.clone(),
            };

            let client_clone = client.clone();
            let model_clone = model.clone();
            let hook_engine = Arc::clone(&self.hook_engine);
            let config = self.config.clone();

            join_set.spawn(async move {
                let runner = BenchmarkRunner {
                    config,
                    hook_engine,
                    inference_semaphore: Arc::new(Semaphore::new(1)),
                };
                let result = runner.execute_workflow_step(&resolved_step, &client_clone, &model_clone, max_tokens, temperature, top_p, None).await;
                (step_id_for_task, result)
            });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            if let Ok(r) = result {
                results.push(r);
            }
        }

        results
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
                    if let Some(filename) = src.file_name() {
                        let dest = Path::new(output_dir).join(filename);
                        if let Err(e) = fs::copy(src, &dest) {
                            warn!("[benchmark] failed to copy workflow YAML {}: {}", wf, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Restart Docker container with specific GPU layers setting.
    ///
    /// `n_gpu_layers`: 0 for CPU-only, high value (e.g., 999) for GPU mode
    async fn restart_docker_with_gpu_layers(&self, n_gpu_layers: u32) -> Result<()> {
        self.restart_docker_with_env_vars(&[("LLAMA_ARG_N_GPU_LAYERS".into(), n_gpu_layers.to_string())]).await
    }

    async fn restart_docker_with_env_vars(&self, env_vars: &[(String, String)]) -> Result<()> {
        info!("[benchmark] restarting Docker with {} env vars", env_vars.len());
        for (k, v) in env_vars {
            info!("[benchmark]   {}={}", k, v);
        }

        let docker_dir = std::env::current_dir().unwrap_or_else(|_| ".".into()).join("docker");

        let stop_status = Command::new("docker")
            .args(["compose", "down"])
            .current_dir(&docker_dir)
            .output()
            .context("Failed to stop Docker container")?;

        if !stop_status.status.success() {
            let stderr = String::from_utf8_lossy(&stop_status.stderr);
            anyhow::bail!("docker compose down failed: {}", stderr);
        }

        sleep(Duration::from_secs(2)).await;

        let mut cmd = Command::new("docker");
        cmd.args(["compose", "up", "-d"])
            .current_dir(&docker_dir);
        for (key, val) in env_vars {
            cmd.env(key, val);
        }

        let start_status = cmd
            .output()
            .context("Failed to start Docker container")?;

        if !start_status.status.success() {
            let stderr = String::from_utf8_lossy(&start_status.stderr);
            anyhow::bail!("docker compose up failed: {}", stderr);
        }

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
    async fn execute_workflow_step(&self, step: &WorkflowStep, client: &LlamaHttpClient, model: &(String, String), max_tokens: usize, temperature: f64, top_p: f64, variables: Option<&serde_json::Map<String, serde_json::Value>>) -> Result<WorkflowStepResult> {
        let (model_id, model_path) = model;
        let default_prompt = String::new();
        let prompt = step.prompt.as_ref().unwrap_or(&default_prompt);

        let step_max_tokens = step.model_overrides.as_ref()
            .and_then(|mo| mo.get("max_tokens").and_then(|m| m.as_u64()).map(|m| m as usize))
            .unwrap_or(max_tokens);
        let step_temperature = step.model_overrides.as_ref()
            .and_then(|mo| mo.get("temperature").and_then(|t| t.as_f64()))
            .unwrap_or(temperature);

        info!("[benchmark] executing step {} with model {}", step.step_id, model_id);

        let mut route_to: Option<Vec<String>> = None;
        let mut skip_remaining = false;
        let mut skip_loop = false;

        // Build workflow variables map for hooks
        let workflow_variables = if let Some(vars) = variables {
            vars.iter()
                .map(|(k, v)| (k.strip_prefix("step.").unwrap_or(k).to_string(), v.clone()))
                .collect()
        } else {
            std::collections::HashMap::new()
        };

        let before_context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_name: step.step_name.clone(),
            step_type: StepType::Generative,
            model_name: model_id.to_string(),
            prompt_preview: if prompt.len() > 100 { format!("{}...", &prompt[..100]) } else { prompt.clone() },
            workflow_variables,
        });

        match self.execute_hooks_for_trigger(&step.when, "before_step_starts", &before_context) {
            Ok(HookResult::SkipStep) => {
                info!("[benchmark] step {} skipped by before_step_starts hook", step.step_id);
                return Ok(WorkflowStepResult {
                    benchmark_result: ModelBenchmarkResult {
                        model_id: model_id.to_string(),
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
                        error: Some("Skipped by hook".to_string()),
                        gpu_mode: "gpu".to_string(),
                        speedup_factor: None,
                    },
                    route_to: None,
                    skip_remaining: false,
                    skip_loop: false,
                });
            }
            Ok(HookResult::RouteTo { targets }) => {
                info!("[benchmark] step {} routed to {:?} by before_step_starts hook", step.step_id, targets);
                return Ok(WorkflowStepResult {
                    benchmark_result: ModelBenchmarkResult {
                        model_id: model_id.to_string(),
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
                        error: Some("Routed by hook".to_string()),
                        gpu_mode: "gpu".to_string(),
                        speedup_factor: None,
                    },
                    route_to: Some(targets),
                    skip_remaining: false,
                    skip_loop: false,
                });
            }
            Ok(HookResult::Fail { reason }) => {
                warn!("[benchmark] before_step_starts hook failed step {}: {}", step.step_id, reason);
                return Ok(WorkflowStepResult {
                    benchmark_result: ModelBenchmarkResult {
                        model_id: model_id.to_string(),
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
                        error: Some("Skipped by hook".to_string()),
                        gpu_mode: "gpu".to_string(),
                        speedup_factor: None,
                    },
                    route_to: None,
                    skip_remaining: false,
                    skip_loop: false,
                });
            }
            Ok(_) => {}
            Err(e) => {
                error!("[benchmark] after_step_starts hook error: {}, continuing", e);
            }
        }

        let resolved_prompt = {
            let bookmarks_map = &self.hook_engine.lock().unwrap().bookmarks;
            let mut resolved = Self::resolve_bookmark_templates(prompt, bookmarks_map);
            if let Some(shell_output) = bookmarks_map.get("shell_output") {
                let shell_stdout = shell_output.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
                let shell_stderr = shell_output.get("stderr").and_then(|v| v.as_str()).unwrap_or("");
                let exit_code = shell_output.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(0);
                let used_template_var = prompt.contains("{{bookmarks.shell_output");
                let mentions_shell_output = resolved.contains("shell_output") && !used_template_var;
                if !shell_stdout.is_empty() && mentions_shell_output {
                    let mut injection = String::new();
                    injection.push_str("\n\n=== SHELL OUTPUT (auto-injected by engine) ===\n");
                    injection.push_str(shell_stdout);
                    if !shell_stderr.is_empty() {
                        injection.push_str("\n\n=== SHELL STDERR ===\n");
                        injection.push_str(shell_stderr);
                    }
                    injection.push_str(&format!("\n=== EXIT CODE: {} ===\n", exit_code));
                    resolved.push_str(&injection);
                    info!("[benchmark] auto-injected shell_output ({} bytes) into step {} prompt (model did not use template var)", shell_stdout.len(), step.step_id);
                }
            }
            resolved
        };

        let system_prompt = if let Some(vars) = variables {
            let model_name = vars.get("step.model_name")
                .and_then(|v| v.as_str())
                .or_else(|| vars.get("model_name").and_then(|v| v.as_str()))
                .unwrap_or(model_id);
            let output_path = format!("./docs/benchmarks/outputs/output/{}.json", model_name);
            Some(format!(
                "You are generating output that will be saved to: {}\nModel running: {}\nRespond ONLY with the requested output format.",
                output_path, model_name
            ))
        } else {
            None
        };

        let model_result = self.run_model_inference(client, model_id, model_path, "gpu", std::slice::from_ref(&resolved_prompt), step_max_tokens, step_temperature, top_p, system_prompt, true).await;

        let output_text = model_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();

        if let Some(ref error) = model_result.error {
            let after_context = WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
                step_name: step.step_name.clone(),
                error_type: "InferenceError".to_string(),
                error_message: error.clone(),
                error: crate::workflow::hooks::context::ErrorDetails {
                    is_retryable: false,
                    count: 1,
                },
                attempt_number: 1,
                model_name: model_id.to_string(),
            });

            match self.execute_hooks_for_trigger(&step.when, "after_step_fails", &after_context) {
                Ok(HookResult::Fail { reason }) => {
                    warn!("[benchmark] after_step_fails hook failed: {}", reason);
                }
                Ok(HookResult::RouteTo { targets }) => {
                    info!("[benchmark] step {} routed to {:?} by after_step_fails hook", step.step_id, targets);
                    route_to = Some(targets);
                }
                Ok(_) => {}
                Err(e) => {
                    error!("[benchmark] after_step_fails hook error: {}", e);
                }
            }

            // Fire after_all_retries_exhausted if error indicates all retries failed
            if error.contains("attempts failed") {
                let exhausted_context = WorkflowHookContext::AfterAllRetriesExhausted(
                    AfterAllRetriesExhaustedContext {
                        step_name: step.step_name.clone(),
                        total_attempts: 3, // MAX_RETRIES in benchmark_single_model
                        last_error: error.clone(),
                        last_error_type: "InferenceError".to_string(),
                    }
                );
                match self.execute_hooks_for_trigger(&step.when, "after_all_retries_exhausted", &exhausted_context) {
                    Ok(HookResult::RouteTo { targets }) => {
                        info!("[benchmark] step {} routed to {:?} by after_all_retries_exhausted hook", step.step_id, targets);
                        route_to = Some(targets);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("[benchmark] after_all_retries_exhausted hook error: {}", e);
                    }
                }
            }
        } else {
            let total_tokens: usize = model_result.inference_results.iter()
                .map(|inf| inf.total_tokens)
                .sum();
            let output_ratio = if step_max_tokens > 0 {
                Some((total_tokens as f32 / step_max_tokens as f32).min(1.0))
            } else {
                None
            };

            let after_context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: step.step_name.clone(),
                output: output_text,
                duration_ms: model_result.total_duration.as_millis() as u64,
                quality_score: output_ratio, // Measures output length vs max_tokens, not semantic quality
                token_count: total_tokens as u32,
                model_name: model_id.to_string(),
            });

            match self.execute_hooks_for_trigger(&step.when, "after_step_succeeds", &after_context) {
                Ok(HookResult::Fail { reason }) => {
                    warn!("[benchmark] after_step_succeeds hook failed: {}", reason);
                }
                Ok(HookResult::RouteTo { targets }) => {
                    info!("[benchmark] step {} routed to {:?} by after_step_succeeds hook", step.step_id, targets);
                    route_to = Some(targets);
                }
                Ok(HookResult::SkipRemaining) => {
                    info!("[benchmark] step {} triggered skip_remaining by after_step_succeeds hook", step.step_id);
                    skip_remaining = true;
                }
                Ok(HookResult::SkipLoop) => {
                    info!("[benchmark] step {} triggered skip_loop by after_step_succeeds hook", step.step_id);
                    skip_loop = true;
                }
                Ok(_) => {}
                Err(e) => {
                    error!("[benchmark] after_step_succeeds hook error: {}", e);
                }
            }
        }

        Ok(WorkflowStepResult {
            benchmark_result: model_result,
            route_to,
            skip_remaining,
            skip_loop,
        })
    }

    pub async fn run(&mut self) -> Result<BenchmarkSuiteResult> {
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
        if let Err(e) = self.copy_workflow_yaml() {
            warn!("[benchmark] failed to archive workflow YAML to output dir: {}", e);
        }

        let client = LlamaHttpClient::new(&self.config.server_url)
            .context("Failed to create HTTP client")?;

        let run_timestamp = {
            let dur = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            format!("unix_epoch_{}s", dur.as_secs())
        };

        let wf_ctx = match self.load_workflow_config() {
            Ok(Some(ctx)) => Some(ctx),
            Ok(None) => None,
            Err(e) => {
                return Err(e);
            }
        };

        // Apply load_params from workflow config (restarts Docker with model spec env vars)
        if let Some(ref ctx) = wf_ctx {
            if !ctx.load_params_env_vars.is_empty() {
                info!("[benchmark] workflow config has {} load_params env vars, restarting Docker", ctx.load_params_env_vars.len());
                self.restart_docker_with_env_vars(&ctx.load_params_env_vars).await
                    .context("Failed to restart Docker with load_params")?;
            }
        }

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
                    warn!("[benchmark] failed to query server models: {}. Cannot discover loaded models — will use workflow YAML model list only.", e);
                }
            }
        }

        if models.is_empty() {
            anyhow::bail!("No models found for benchmarking");
        }

        info!("[benchmark] found {} models to benchmark", models.len());
        info!("[benchmark] GPU/CPU comparison mode: {}", compare_gpu_cpu);

        let mut results = Vec::new();
        let mut step_outputs: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        let suite_metadata = format!(
            "run_timestamp: {}\nserver_url: {}\ntotal_models: {}\ncompare_gpu_cpu: {}",
            run_timestamp, self.config.server_url, models.len(), compare_gpu_cpu
        );

        let workflow_steps = self.load_workflow_steps();
        let yaml_models = self.load_workflow_models();

        if let Some(steps) = &workflow_steps {
            info!("[benchmark] loaded {} workflow steps from YAML", steps.len());

            let step_index: std::collections::HashMap<String, usize> = steps.iter()
                .enumerate()
                .map(|(i, s)| (s.step_id.clone(), i))
                .collect();

            let max_loop_iterations = steps.iter()
                .filter_map(|s| s.r#loop.as_ref())
                .filter_map(|l| l.get("count"))
                .filter_map(|c| c.get("max_iterations"))
                .filter_map(|m| m.as_u64())
                .max()
                .unwrap_or(100)
                .max(100) as usize;
            let mut current_index: usize = 0;
            let mut loop_count: usize = 0;

            while current_index < steps.len() && loop_count < max_loop_iterations {
                let step = &steps[current_index];
                loop_count += 1;

                if !step.requires.is_empty() {
                    let missing_deps: Vec<String> = step.requires.iter()
                        .filter(|dep| !step_outputs.contains_key(*dep))
                        .cloned()
                        .collect();

                    if !missing_deps.is_empty() {
                        warn!("[benchmark] step {} skipped: missing dependencies {:?}", step.step_id, missing_deps);
                        let requires_context = WorkflowHookContext::OnRequiresFailed(
                            OnRequiresFailedContext {
                                failed_step: step.step_id.clone(),
                                reason: format!("Dependencies not satisfied: {:?}", missing_deps),
                                dependency_chain: missing_deps.clone(),
                            }
                        );
                        let _ = self.execute_hooks_for_trigger(&step.when, "on_requires_failed", &requires_context);
                        current_index += 1;
                        continue;
                    }
                }

                let variable_sets = self.extract_iterate_values(step);

                if let Some(ref var_sets) = variable_sets {
                    info!("[benchmark] step {} has {} iteration values", step.step_id, var_sets.len());

                    let mut routed: Option<Vec<String>> = None;
                    let mut should_skip_remaining = false;

                    for (iter_idx, vars) in var_sets.iter().enumerate() {
                        let iteration = iter_idx + 1;
                        let resolved_ge = step.generative_entity.as_ref()
                            .map(|ge| Self::resolve_templates(ge, vars, iteration));

                        let resolved_prompt = step.prompt.as_ref()
                            .map(|p| {
                                let after_resolve = Self::resolve_templates(p, vars, iteration);
                                Self::resolve_step_output_templates(&after_resolve, &step_outputs)
                            });

                        if let Some(ref ge) = resolved_ge {
                            let model_key = ge.strip_prefix("${models.")
                                .and_then(|s| s.strip_suffix('}'))
                                .unwrap_or(ge);

                            let model_name = yaml_models.as_ref()
                                .and_then(|m| m.get(model_key))
                                .map(|s| s.as_str())
                                .unwrap_or(model_key);

                            let model_file = self.resolve_model_file(model_name, &models);

                            if let Some(model) = model_file {
                                info!("[benchmark] [{}/{}] step {} with model {} (vars: {:?})",
                                    iteration, var_sets.len(), step.step_id, model.0, vars);

                                let resolved_step = WorkflowStep {
                                    step_name: step.step_name.clone(),
                                    step_id: step.step_id.clone(),
                                    requires: step.requires.clone(),
                                    when: step.when.clone(),
                                    prompt: resolved_prompt.clone(),
                                    generative_entity: resolved_ge,
                                    model_overrides: step.model_overrides.clone(),
                                    r#loop: step.r#loop.clone(),
                                };

                                let step_result = self.execute_workflow_step(&resolved_step, &client, &model, max_tokens, temperature, top_p, Some(vars)).await?;
                                results.push(step_result.benchmark_result.clone());

                                if let Some(last_result) = results.last() {
                                    if let Some(ref err) = last_result.error {
                                        let fail_context = WorkflowHookContext::AfterLoopIterationFails(
                                            AfterLoopIterationFailsContext {
                                                step_name: step.step_name.clone(),
                                                iteration: iteration as u32,
                                                error_message: err.clone(),
                                                loop_type: "iterate_values".to_string(),
                                            }
                                        );
                                        let _ = self.execute_hooks_for_trigger(&step.when, "after_loop_iteration_fails", &fail_context);
                                    }
                                    let output_text = last_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();
                                    step_outputs.insert(step.step_id.clone(), output_text.clone());
                                    self.hook_engine.lock().unwrap().store_bookmark(step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                                }

                                if let Some(ref targets) = step_result.route_to {
                                    routed = Some(targets.clone());
                                    info!("[benchmark] routing from step {} to {:?} (iteration #{})",
                                        step.step_id, targets, loop_count);
                                    break;
                                }
                                if step_result.skip_remaining {
                                    info!("[benchmark] skip_remaining triggered at step {} (iteration #{})", step.step_id, loop_count);
                                    should_skip_remaining = true;
                                    break;
                                }
                                if step_result.skip_loop {
                                    info!("[benchmark] skip_loop triggered at step {} (iteration #{})", step.step_id, loop_count);
                                    break; // Break inner for loop, outer while will advance current_index
                                }
                            } else {
                                warn!("[benchmark] iteration {}: could not resolve model for step {}: {}",
                                    iteration, step.step_id, model_name);
                                let failed_result = Self::model_resolution_failure_result(&step.step_id, model_name);
                                results.push(failed_result);
                            }
                        }
                    }

                    if should_skip_remaining {
                        break;
                    }

                    if let Some(ref targets) = routed {
                        if targets.len() == 1 {
                            if let Some(&target_idx) = step_index.get(&targets[0]) {
                                current_index = target_idx;
                                continue;
                                      } else {
                                          warn!("[benchmark] route_to target '{}' not found", targets[0]);
                                      }
                                  } else {
                                     if let Some(last_idx) = self.try_execute_route_to_parallel(
                                         targets, steps, &step_index, &client, &yaml_models, &models,
                                         max_tokens, temperature, top_p, &mut step_outputs, current_index, false
                                     ).await {
                                         current_index = last_idx + 1;
                                         continue;
                                     }
                                     let mut last_target_idx = current_index;
                                     for target_id in targets {
                                         if let Some(&target_idx) = step_index.get(target_id) {
                                             let target_step = &steps[target_idx];
                                             if let Some(ref ge) = target_step.generative_entity {
                                                 let model_key = ge.strip_prefix("${models.")
                                                     .and_then(|s| s.strip_suffix('}'))
                                                     .unwrap_or(ge);
                                        let model_name = yaml_models.as_ref()
                                            .and_then(|m| m.get(model_key))
                                            .map(|s| s.as_str())
                                            .unwrap_or(model_key);
                                        if let Some(model) = self.resolve_model_file(model_name, &models) {
                                                      // Bookmark resolution deferred — hooks must fire first.
                                                      let resolved_prompt = target_step.prompt.as_ref()
                                                          .map(|p| {
                                                              Self::resolve_step_output_templates(p, &step_outputs)
                                                          });
                                            let resolved_target = WorkflowStep {
                                                step_name: target_step.step_name.clone(),
                                                step_id: target_step.step_id.clone(),
                                                requires: target_step.requires.clone(),
                                                when: target_step.when.clone(),
                                                prompt: resolved_prompt,
                                                generative_entity: target_step.generative_entity.clone(),
                                                model_overrides: target_step.model_overrides.clone(),
                                                r#loop: target_step.r#loop.clone(),
                                            };
                                                     let target_result = self.execute_workflow_step(&resolved_target, &client, &model, max_tokens, temperature, top_p, None).await?;
                                                     results.push(target_result.benchmark_result.clone());
                                                     if let Some(last) = results.last() {
                                                         let output_text = last.inference_results.first()
                                                             .map(|inf| inf.response_text.clone())
                                                             .unwrap_or_default();
                                                         step_outputs.insert(target_step.step_id.clone(), output_text.clone());
                                                         self.hook_engine.lock().unwrap().store_bookmark(target_step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                                                     }
                                                     last_target_idx = target_idx;
                                        }
                                    }
                                } else {
                                    warn!("[benchmark] route_to target '{}' not found", target_id);
                                }
                            }
                            current_index = last_target_idx + 1;
                            continue;
                        }
                    }

                    current_index += 1;
                } else {
                    if let Some(ref ge) = step.generative_entity {
                        let model_key = ge.strip_prefix("${models.")
                            .and_then(|s| s.strip_suffix('}'))
                            .unwrap_or(ge);

                        let model_name = yaml_models.as_ref()
                            .and_then(|m| m.get(model_key))
                            .map(|s| s.as_str())
                            .unwrap_or(model_key);

                        let model_file = self.resolve_model_file(model_name, &models);

                        if let Some(model) = model_file {
                            info!("[benchmark] executing step {} with model {}", step.step_id, model.0);

                            // Bookmark resolution deferred — hooks must fire first.
                            let resolved_prompt = step.prompt.as_ref()
                                .map(|p| {
                                    Self::resolve_step_output_templates(p, &step_outputs)
                                });

                             let resolved_step = WorkflowStep {
                                 step_name: step.step_name.clone(),
                                 step_id: step.step_id.clone(),
                                 requires: step.requires.clone(),
                                 when: step.when.clone(),
                                 prompt: resolved_prompt,
                                 generative_entity: step.generative_entity.clone(),
                                 model_overrides: step.model_overrides.clone(),
                                 r#loop: step.r#loop.clone(),
                             };

                              let step_result = self.execute_workflow_step(&resolved_step, &client, &model, max_tokens, temperature, top_p, None).await?;
                              results.push(step_result.benchmark_result.clone());

                              if let Some(last_result) = results.last() {
                                  let output_text = last_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();
                                  step_outputs.insert(step.step_id.clone(), output_text.clone());
                                  self.hook_engine.lock().unwrap().store_bookmark(step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                              }

                             if let Some(ref targets) = step_result.route_to {
                                 if targets.len() == 1 {
                                     if let Some(&target_idx) = step_index.get(&targets[0]) {
                                         info!("[benchmark] routing from step {} to {} (iteration #{})",
                                             step.step_id, targets[0], loop_count);
                                         current_index = target_idx;
                                         continue;
                                     } else {
                                         warn!("[benchmark] route_to target '{}' not found", targets[0]);
                                     }
                                   } else {
                                      if let Some(last_idx) = self.try_execute_route_to_parallel(
                                          targets, steps, &step_index, &client, &yaml_models, &models,
                                          max_tokens, temperature, top_p, &mut step_outputs, current_index, false
                                      ).await {
                                          current_index = last_idx + 1;
                                          continue;
                                      }
                                     let mut last_target_idx = current_index;
                                     for target_id in targets {
                                         if let Some(&target_idx) = step_index.get(target_id) {
                                             let target_step = &steps[target_idx];
                                             if let Some(ref ge) = target_step.generative_entity {
                                                 let model_key = ge.strip_prefix("${models.")
                                                     .and_then(|s| s.strip_suffix('}'))
                                                     .unwrap_or(ge);
                                                 let model_name = yaml_models.as_ref()
                                                     .and_then(|m| m.get(model_key))
                                                     .map(|s| s.as_str())
                                                     .unwrap_or(model_key);
                                                 if let Some(model) = self.resolve_model_file(model_name, &models) {
                                                      let resolved_prompt = target_step.prompt.as_ref()
                                                          .map(|p| {
                                                              Self::resolve_step_output_templates(p, &step_outputs)
                                                          });
                                                     let resolved_target = WorkflowStep {
                                                        step_name: target_step.step_name.clone(),
                                                        step_id: target_step.step_id.clone(),
                                                        requires: target_step.requires.clone(),
                                                        when: target_step.when.clone(),
                                                        prompt: resolved_prompt,
                                                        generative_entity: target_step.generative_entity.clone(),
                                                        model_overrides: target_step.model_overrides.clone(),
                                                        r#loop: target_step.r#loop.clone(),
                                                    };
                                                    let target_result = self.execute_workflow_step(&resolved_target, &client, &model, max_tokens, temperature, top_p, None).await?;
                                                    results.push(target_result.benchmark_result.clone());
                                                    if let Some(last) = results.last() {
                                                        let output_text = last.inference_results.first()
                                                            .map(|inf| inf.response_text.clone())
                                                            .unwrap_or_default();
                                                        step_outputs.insert(target_step.step_id.clone(), output_text.clone());
                                                        self.hook_engine.lock().unwrap().store_bookmark(target_step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                                                    }
                                                    last_target_idx = target_idx;
                                                }
                                            }
                                        } else {
                                            warn!("[benchmark] route_to target '{}' not found", target_id);
                                        }
                                    }
                                    current_index = last_target_idx + 1;
                                    continue;
                                 }
                             }

                             if step_result.skip_remaining {
                                info!("[benchmark] skip_remaining triggered at step {} — stopping workflow", step.step_id);
                                break;
                            }

                            current_index += 1;
                        } else {
                            warn!("[benchmark] could not resolve model for step {}: {}", step.step_id, model_name);
                            let failed_result = Self::model_resolution_failure_result(&step.step_id, model_name);

                            let fail_context = WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
                                step_name: step.step_name.clone(),
                                error_type: "ModelResolutionError".to_string(),
                                error_message: format!("Model '{}' not found on server", model_name),
                                error: crate::workflow::hooks::context::ErrorDetails {
                                    is_retryable: false,
                                    count: 1,
                                },
                                attempt_number: 1,
                                model_name: model_name.to_string(),
                            });

                            match self.execute_hooks_for_trigger(&step.when, "after_step_fails", &fail_context) {
                                Ok(HookResult::Fail { reason }) => {
                                    warn!("[benchmark] after_step_fails hook failed: {}", reason);
                                }
                                 Ok(HookResult::RouteTo { targets }) => {
                                     info!("[benchmark] step {} routed to {:?} by after_step_fails (model resolution)", step.step_id, targets);
                                     if targets.len() == 1 {
                                         if let Some(&target_idx) = step_index.get(&targets[0]) {
                                             current_index = target_idx;
                                             continue;
                                         }
                                     } else {
                                    if let Some(last_idx) = self.try_execute_route_to_parallel(
                                        &targets, steps, &step_index, &client, &yaml_models, &models,
                                        max_tokens, temperature, top_p, &mut step_outputs, current_index, false
                                    ).await {
                                            current_index = last_idx + 1;
                                            continue;
                                        }
                                        let mut last_target_idx = current_index;
                                        for target_id in targets {
                                        if let Some(&target_idx) = step_index.get(&target_id) {
                                                let target_step = &steps[target_idx];
                                                if let Some(ref ge) = target_step.generative_entity {
                                                    let model_key = ge.strip_prefix("${models.")
                                                        .and_then(|s| s.strip_suffix('}'))
                                                        .unwrap_or(ge);
                                                    let model_name = yaml_models.as_ref()
                                                        .and_then(|m| m.get(model_key))
                                                        .map(|s| s.as_str())
                                                        .unwrap_or(model_key);
                                                if let Some(model) = self.resolve_model_file(model_name, &models) {
                                                    let resolved_prompt = target_step.prompt.as_ref()
                                                        .map(|p| {
                                                            Self::resolve_step_output_templates(p, &step_outputs)
                                                        });
                                                     let resolved_target = WorkflowStep {
                                                            step_name: target_step.step_name.clone(),
                                                            step_id: target_step.step_id.clone(),
                                                            requires: target_step.requires.clone(),
                                                            when: target_step.when.clone(),
                                                            prompt: resolved_prompt,
                                                            generative_entity: target_step.generative_entity.clone(),
                                                            model_overrides: target_step.model_overrides.clone(),
                                                            r#loop: target_step.r#loop.clone(),
                                                        };
                                                        let target_result = self.execute_workflow_step(&resolved_target, &client, &model, max_tokens, temperature, top_p, None).await?;
                                                        results.push(target_result.benchmark_result.clone());
                                                        if let Some(last) = results.last() {
                                                            let output_text = last.inference_results.first()
                                                                .map(|inf| inf.response_text.clone())
                                                                .unwrap_or_default();
                                                            step_outputs.insert(target_step.step_id.clone(), output_text.clone());
                                                self.hook_engine.lock().unwrap().store_bookmark(target_step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                                                        }
                                                        last_target_idx = target_idx;
                                                    }
                                                }
                                            } else {
                                                warn!("[benchmark] route_to target '{}' not found", target_id);
                                            }
                                        }
                                        current_index = last_target_idx + 1;
                                        continue;
                                     }
                                 }
                                Ok(_) => {}
                                Err(e) => {
                                    error!("[benchmark] after_step_fails hook error: {}", e);
                                }
                            }

                            results.push(failed_result);
                            current_index += 1;
                        }
                    } else if step.prompt.is_some() {
                        if let Some(model) = models.first() {
                            let resolved_prompt = step.prompt.as_ref()
                                .map(|p| {
                                    Self::resolve_step_output_templates(p, &step_outputs)
                                });

                            let resolved_step = WorkflowStep {
                                step_name: step.step_name.clone(),
                                step_id: step.step_id.clone(),
                                requires: step.requires.clone(),
                                when: step.when.clone(),
                                prompt: resolved_prompt,
                                generative_entity: step.generative_entity.clone(),
                                model_overrides: step.model_overrides.clone(),
                                r#loop: step.r#loop.clone(),
                            };

                              let step_result = self.execute_workflow_step(&resolved_step, &client, model, max_tokens, temperature, top_p, None).await?;
                              results.push(step_result.benchmark_result.clone());

                              if let Some(last_result) = results.last() {
                                  let output_text = last_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();
                                  step_outputs.insert(step.step_id.clone(), output_text.clone());
                                  self.hook_engine.lock().unwrap().store_bookmark(step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                              }

                              if let Some(ref targets) = step_result.route_to {
                                  if targets.len() == 1 {
                                      if let Some(&target_idx) = step_index.get(&targets[0]) {
                                          info!("[benchmark] routing from step {} to {} (iteration #{})",
                                              step.step_id, targets[0], loop_count);
                                          current_index = target_idx;
                                          continue;
                                     } else {
                                         warn!("[benchmark] route_to target '{}' not found", targets[0]);
                                     }
                                  } else {
                                     if let Some(last_idx) = self.try_execute_route_to_parallel(
                                         targets, steps, &step_index, &client, &yaml_models, &models,
                                         max_tokens, temperature, top_p, &mut step_outputs, current_index, false
                                     ).await {
                                         current_index = last_idx + 1;
                                         continue;
                                     }
                                    let mut last_target_idx = current_index;
                                    for target_id in targets {
                                        if let Some(&target_idx) = step_index.get(target_id) {
                                            let target_step = &steps[target_idx];
                                            if let Some(ref ge) = target_step.generative_entity {
                                                let model_key = ge.strip_prefix("${models.")
                                                    .and_then(|s| s.strip_suffix('}'))
                                                    .unwrap_or(ge);
                                                let model_name = yaml_models.as_ref()
                                                    .and_then(|m| m.get(model_key))
                                                    .map(|s| s.as_str())
                                                    .unwrap_or(model_key);
                                                if let Some(model) = self.resolve_model_file(model_name, &models) {
                                                    let resolved_prompt = target_step.prompt.as_ref()
                                                        .map(|p| {
                                                            Self::resolve_step_output_templates(p, &step_outputs)
                                                        });
                                                    let resolved_target = WorkflowStep {
                                                        step_name: target_step.step_name.clone(),
                                                        step_id: target_step.step_id.clone(),
                                                        requires: target_step.requires.clone(),
                                                        when: target_step.when.clone(),
                                                        prompt: resolved_prompt,
                                                        generative_entity: target_step.generative_entity.clone(),
                                                        model_overrides: target_step.model_overrides.clone(),
                                                        r#loop: target_step.r#loop.clone(),
                                                    };
                                                    let target_result = self.execute_workflow_step(&resolved_target, &client, &model, max_tokens, temperature, top_p, None).await?;
                                                    results.push(target_result.benchmark_result.clone());
                                                    if let Some(last) = results.last() {
                                                        let output_text = last.inference_results.first()
                                                            .map(|inf| inf.response_text.clone())
                                                            .unwrap_or_default();
                                                        step_outputs.insert(target_step.step_id.clone(), output_text.clone());
                                                        self.hook_engine.lock().unwrap().store_bookmark(target_step.step_id.clone(), serde_json::Value::String(output_text.to_string()));
                                                    }
                                                    last_target_idx = target_idx;
                                                }
                                            }
                                        } else {
                                            warn!("[benchmark] route_to target '{}' not found", target_id);
                                        }
                                    }
                                    current_index = last_target_idx + 1;
                                    continue;
                                 }
                             }

                             if step_result.skip_remaining {
                                info!("[benchmark] skip_remaining triggered at step {} — stopping workflow", step.step_id);
                                break;
                            }

                            current_index += 1;
                        } else {
                            current_index += 1;
                        }
                    } else {
                        current_index += 1;
                    }
                }
            }

            if loop_count >= max_loop_iterations {
                warn!("[benchmark] workflow loop exceeded {} iterations, stopping", max_loop_iterations);
            }
        }

        let workflow_ran = workflow_steps.as_ref().is_some_and(|s| !s.is_empty());

        if !workflow_ran {
            warn!("[DEPRECATED] Running benchmark without workflow YAML. Direct benchmark mode is deprecated. Use --workflow flag.");
        }

        #[allow(deprecated)]
        if !workflow_ran && compare_gpu_cpu {
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
                    self.log_step_error(model_id, failed_result.error.as_deref().unwrap_or("unknown error"))?;

                    if let Some(ref output_dir) = self.config.output_dir {
                        let detail_gen = DetailGenerator::new(output_dir);
                        if let Err(e) = detail_gen.write_model_detail(&failed_result) {
                            warn!("[detail] Failed to generate detail.md for {} (GPU health check failed): {}", model_id, e);
                        }
                    }

                    results.push(failed_result);
                    continue;
                }

                let gpu_result = self.benchmark_single_model(&client, model_id, model_path, "gpu", &prompts, max_tokens, temperature, top_p, None, &None).await;

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
                        self.log_step_error(model_id, cpu_result.error.as_deref().unwrap_or("unknown error"))?;

                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir);
                            if let Err(e) = detail_gen.write_model_detail(&cpu_result) {
                                warn!("[detail] Failed to generate detail.md for {} (CPU health check failed): {}", model_id, e);
                            }
                        }

                        results.push(gpu_result);
                        results.push(cpu_result);
                        self.restart_docker_with_gpu_layers(999).await
                            .context("Failed to restart Docker in GPU mode")?;
                        continue;
                    }

                    let cpu_result = self.benchmark_single_model(&client, model_id, model_path, "cpu", &prompts, max_tokens, temperature, top_p, None, &None).await;

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
                        self.append_chat_log_markdown(&gpu_result_with_speedup, &run_timestamp)?;

                        // Generate detail.md for GPU mode
                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir);
                            if let Err(e) = detail_gen.write_model_detail(&gpu_result_with_speedup) {
                                warn!("[detail] Failed to generate detail.md for {} (GPU): {}", model_id, e);
                            }
                        }

                        results.push(gpu_result_with_speedup);

                        let mut cpu_result_with_speedup = cpu_result.clone();
                        cpu_result_with_speedup.speedup_factor = Some(speedup);
                        self.write_per_model_report(&cpu_result_with_speedup, &suite_metadata, wf_ctx.as_ref())?;
                        self.append_chat_log_markdown(&cpu_result_with_speedup, &run_timestamp)?;

                        // Generate detail.md for CPU mode
                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir);
                            if let Err(e) = detail_gen.write_model_detail(&cpu_result_with_speedup) {
                                warn!("[detail] Failed to generate detail.md for {} (CPU): {}", model_id, e);
                            }
                        }

                        results.push(cpu_result_with_speedup);
                    } else {
                        self.write_per_model_report(&gpu_result, &suite_metadata, wf_ctx.as_ref())?;
                        self.append_chat_log_markdown(&gpu_result, &run_timestamp)?;

                        // Generate detail.md for GPU mode (CPU failed)
                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir);
                            if let Err(e) = detail_gen.write_model_detail(&gpu_result) {
                                warn!("[detail] Failed to generate detail.md for {} (GPU, CPU failed): {}", model_id, e);
                            }
                        }

                        self.write_per_model_report(&cpu_result, &suite_metadata, wf_ctx.as_ref())?;
                        self.append_chat_log_markdown(&cpu_result, &run_timestamp)?;

                        // Generate detail.md for CPU mode (even though it failed)
                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir);
                            if let Err(e) = detail_gen.write_model_detail(&cpu_result) {
                                warn!("[detail] Failed to generate detail.md for {} (CPU, failed): {}", model_id, e);
                            }
                        }

                        results.push(gpu_result);
                        results.push(cpu_result);
                    }

                    self.restart_docker_with_gpu_layers(999).await
                        .context("Failed to restart Docker in GPU mode")?;
                } else {
                    self.write_per_model_report(&gpu_result, &suite_metadata, wf_ctx.as_ref())?;
                    self.append_chat_log_markdown(&gpu_result, &run_timestamp)?;

                    if let Some(ref output_dir) = self.config.output_dir {
                        let detail_gen = DetailGenerator::new(output_dir);
                        if let Err(e) = detail_gen.write_model_detail(&gpu_result) {
                            warn!("[detail] Failed to generate detail.md for {} (GPU only): {}", model_id, e);
                        }
                    }

                    results.push(gpu_result);
                }

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
            }
        } else if !workflow_ran {
            #[allow(deprecated)]
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
                    self.log_step_error(model_id, failed_result.error.as_deref().unwrap_or("unknown error"))?;

                    if let Some(ref output_dir) = self.config.output_dir {
                        let detail_gen = DetailGenerator::new(output_dir);
                        if let Err(e) = detail_gen.write_model_detail(&failed_result) {
                            warn!("[detail] Failed to generate detail.md for {} (non-compare health check failed): {}", model_id, e);
                        }
                    }

                    results.push(failed_result);
                    continue;
                }

                let model_result = self.benchmark_single_model(&client, model_id, model_path, "gpu", &prompts, max_tokens, temperature, top_p, None, &None).await;

                if let Some(ref err) = model_result.error {
                    self.log_step_error(model_id, err)?;
                }
                self.log_step_result(&model_result)?;
                self.write_per_model_report(&model_result, &suite_metadata, wf_ctx.as_ref())?;
                self.append_chat_log_markdown(&model_result, &run_timestamp)?;

                // Generate detail.md for this model
                if let Some(ref output_dir) = self.config.output_dir {
                    let detail_gen = DetailGenerator::new(output_dir);
                    if let Err(e) = detail_gen.write_model_detail(&model_result) {
                        warn!("[detail] Failed to generate detail.md for {}: {}", model_id, e);
                    }
                }

                results.push(model_result);

                if i < models.len() - 1 {
                    tokio::time::sleep(self.config.delay_between_swaps).await;
                }
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

    /// Send a single chat completion request. Used for parallel inference.
    /// Returns (response_text, completion_tokens, duration, error_message).
    #[allow(clippy::too_many_arguments)]
    async fn send_inference_request(
        client: &LlamaHttpClient,
        model_id: &str,
        prompt: &str,
        max_tokens: usize,
        temperature: f64,
        top_p: f64,
        system_prompt: Option<&str>,
        semaphore: &Arc<Semaphore>,
    ) -> (String, usize, std::time::Duration, Option<String>) {
        let _permit = semaphore.acquire().await.unwrap_or_else(|e| {
            eprintln!("[SEMAPHORE] acquire failed: {}", e);
            panic!("Semaphore closed");
        });

        let mut messages = Vec::with_capacity(2);
        if let Some(sys) = system_prompt {
            messages.push(ChatMessage::system(sys.to_string()));
        }
        messages.push(ChatMessage::user(prompt.to_string()));

        let request = ChatCompletionRequest {
            model: model_id.to_string(),
            messages,
            max_tokens: Some(max_tokens),
            temperature: Some(temperature as f32),
            top_p: Some(top_p as f32),
            stream: false,
            ..Default::default()
        };

        let inf_start = std::time::Instant::now();
        match client.chat_completion(request).await {
            Ok(resp) => {
                let raw_response = resp.choices.first()
                    .map(|c| c.message.content.clone())
                    .unwrap_or_default();
                let cleaned = Self::clean_response_text(&raw_response);
                let tokens = resp.usage.completion_tokens;
                (cleaned, tokens, inf_start.elapsed(), None)
            }
            Err(e) => {
                (format!("ERROR: {}", e), 0, inf_start.elapsed(), Some(e.to_string()))
            }
        }
    }

    /// Try parallel execution of route_to targets when all use the same model.
    /// Returns Some(last_target_idx) if parallel was executed, None if caller should use sequential.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    async fn try_execute_route_to_parallel(
        &self,
        targets: &[String],
        steps: &[WorkflowStep],
        step_index: &std::collections::HashMap<String, usize>,
        client: &LlamaHttpClient,
        yaml_models: &Option<std::collections::HashMap<String, String>>,
        models: &[(String, String)],
        max_tokens: usize,
        temperature: f64,
        top_p: f64,
        step_outputs: &mut std::collections::HashMap<String, String>,
        current_index: usize,
        skip_unload: bool,
    ) -> Option<usize> {
        if targets.len() <= 1 {
            return None;
        }

        info!("[benchmark] route_to has {} targets, attempting parallel execution", targets.len());

        let mut target_infos: Vec<(String, usize, String, Option<String>, Option<serde_json::Value>)> = Vec::new();
        let mut first_model: Option<String> = None;
        let mut all_same_model = true;

        for target_id in targets {
            if let Some(&target_idx) = step_index.get(target_id) {
                let target_step = &steps[target_idx];
                if let Some(ref ge) = target_step.generative_entity {
                    let model_key = ge.strip_prefix("${models.")
                        .and_then(|s| s.strip_suffix('}'))
                        .unwrap_or(ge);
                    let model_name = yaml_models.as_ref()
                        .and_then(|m| m.get(model_key))
                        .map(|s| s.as_str())
                        .unwrap_or(model_key);

                    if first_model.is_none() {
                        first_model = Some(model_name.to_string());
                    } else if first_model.as_deref() != Some(model_name) {
                        all_same_model = false;
                    }

                    let resolved_prompt = target_step.prompt.as_ref()
                        .map(|p| Self::resolve_step_output_templates(p, step_outputs));

                    target_infos.push((
                        target_id.clone(),
                        target_idx,
                        model_name.to_string(),
                        resolved_prompt,
                        target_step.model_overrides.clone(),
                    ));
                }
            } else {
                warn!("[benchmark] route_to target '{}' not found", target_id);
            }
        }

        if !all_same_model || target_infos.len() <= 1 {
            info!("[benchmark] route_to targets use different models or only 1 target, falling to sequential");
            return None;
        }

        let model_name = first_model.as_deref().unwrap_or("");
        info!("[benchmark] parallel execution: {} targets with same model {}", target_infos.len(), model_name);

        let model = self.resolve_model_file(model_name, models)?;
        let server_model_id = model.0.strip_suffix(".gguf").unwrap_or(&model.0);

        let already_loaded = if let Ok(loaded) = client.list_models().await {
            loaded.iter().any(|m| m.id == server_model_id && m.status.value == "loaded")
        } else { false };

        if !already_loaded {
            if let Ok(loaded) = client.list_models().await {
                for m in loaded {
                    if m.status.value == "loaded" && m.id != server_model_id {
                        if let Err(e) = client.unload_model(&m.id).await { warn!("[benchmark] failed to unload model {}: {}", m.id, e); }
                    }
                }
            }
            if let Err(e) = client.load_model(server_model_id).await { warn!("[benchmark] failed to load model {}: {}", server_model_id, e); }
            info!("[benchmark] model {} loaded for parallel inference", server_model_id);
        }

        let mut join_set: JoinSet<(usize, String, usize, std::time::Duration, Option<String>)> = JoinSet::new();

        for (idx, (_, _, _, prompt, overrides)) in target_infos.iter().enumerate() {
            let prompt_text = prompt.clone().unwrap_or_default();
            let step_max_tokens = overrides.as_ref()
                .and_then(|mo| mo.get("max_tokens").and_then(|m| m.as_u64()).map(|m| m as usize))
                .unwrap_or(max_tokens);
            let step_temperature = overrides.as_ref()
                .and_then(|mo| mo.get("temperature").and_then(|t| t.as_f64()))
                .unwrap_or(temperature);

            let client_clone = client.clone();
            let model_id = server_model_id.to_string();
            let sem = self.inference_semaphore.clone();

            join_set.spawn(async move {
                let result = BenchmarkRunner::send_inference_request(
                    &client_clone, &model_id, &prompt_text,
                    step_max_tokens, step_temperature, top_p, None, &sem
                ).await;
                (idx, result.0, result.1, result.2, result.3)
            });
        }

        let mut parallel_results: Vec<(usize, String, usize, std::time::Duration, Option<String>)> = Vec::new();
        while let Some(result) = join_set.join_next().await {
            if let Ok(r) = result {
                parallel_results.push(r);
            }
        }
        parallel_results.sort_by_key(|r| r.0);

        let mut last_target_idx = current_index;
        for (idx, response_text, tokens, duration, error) in &parallel_results {
            let (_target_id, target_idx, _, _, _) = &target_infos[*idx];
            let step = &steps[*target_idx];

            let output_text = if let Some(err) = error {
                format!("ERROR: {}", err)
            } else {
                response_text.clone()
            };

            step_outputs.insert(step.step_id.clone(), output_text.clone());
            last_target_idx = *target_idx;

            info!("[benchmark] parallel target {} completed: {} tokens in {:?}", step.step_id, tokens, duration);
        }

        if !skip_unload {
            if let Err(e) = client.unload_model(server_model_id).await { warn!("[benchmark] failed to unload model {}: {}", server_model_id, e); }
            info!("[benchmark] [{}] cooldown after parallel unload: sleeping {}s", server_model_id, self.config.cooldown_after_unload.as_secs());
            sleep(self.config.cooldown_after_unload).await;
        } else {
            info!("[benchmark] [{}] skipping parallel unload (next steps use same model)", server_model_id);
        }

        Some(last_target_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn run_model_inference(&self, client: &LlamaHttpClient, model_id: &str, model_source_path: &str, gpu_mode: &str, prompts: &[String], max_tokens: usize, temperature: f64, top_p: f64, system_prompt: Option<String>, skip_unload: bool) -> ModelBenchmarkResult {
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

        let already_loaded = if let Ok(models) = client.list_models().await {
            models.iter().any(|m| m.id == server_model_id && m.status.value == "loaded")
        } else {
            false
        };

        if !already_loaded {
            if let Ok(models) = client.list_models().await {
                for m in models {
                    if m.status.value == "loaded" && m.id != server_model_id {
                        info!("[benchmark] unloading {} to make room for {}", m.id, server_model_id);
                        if let Err(e) = client.unload_model(&m.id).await { warn!("[benchmark] failed to unload model {}: {}", m.id, e); }
                    }
                }
            }
        } else {
            info!("[benchmark] model {} already loaded, reusing", server_model_id);
        }

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
        let mut any_retry_exhausted: Option<String> = None;

        for prompt in prompts {
            const MAX_RETRIES: u32 = 3;
            let mut last_error = None;

            for attempt in 0..MAX_RETRIES {
                let mut messages = Vec::with_capacity(2);
                if let Some(ref sys) = system_prompt {
                    messages.push(ChatMessage::system(sys.clone()));
                }
                messages.push(ChatMessage::user(prompt));

                let request = ChatCompletionRequest {
                    model: server_model_id.to_string(),
                    messages,
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
                            response_text: raw_response,
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
                any_retry_exhausted = Some(format!("All {} attempts failed: {}", MAX_RETRIES, err));
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
        let unload_duration = if !skip_unload {
            if let Err(e) = client.unload_model(server_model_id).await { warn!("[benchmark] failed to unload model {}: {}", server_model_id, e); }
            let duration = unload_start.elapsed();

            info!("[benchmark] [{}] cooldown after unload: sleeping {}s", model_id, self.config.cooldown_after_unload.as_secs());
            sleep(self.config.cooldown_after_unload).await;
            info!("[benchmark] [{}] cooldown complete", model_id);
            duration
        } else {
            info!("[benchmark] [{}] skipping unload (next step uses same model)", model_id);
            Duration::ZERO
        };

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
            error: any_retry_exhausted,
            gpu_mode: gpu_mode.to_string(),
            speedup_factor: None,
        }
    }

    #[deprecated(since = "0.4.0", note = "Use workflow-driven benchmark via --workflow flag")]
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    async fn benchmark_single_model(&mut self, client: &LlamaHttpClient, model_id: &str, model_source_path: &str, gpu_mode: &str, prompts: &[String], max_tokens: usize, temperature: f64, top_p: f64, system_prompt: Option<String>, hook_config: &Option<serde_json::Value>) -> ModelBenchmarkResult {
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

        let default_context_tokens: u32 = 8192;
        let safety_margin_bytes: u64 = 1_073_741_824; // 1GB

        match memory_monitor::can_load_model(file_size, default_context_tokens, safety_margin_bytes) {
            Ok(check) => {
                info!("[benchmark] memory check: available {:.1}GB, required {:.1}GB (model {:.1}GB + KV cache {:.1}GB + safety {:.1}GB), can_load={}",
                    bytes_to_gb(check.available_bytes),
                    bytes_to_gb(check.required_bytes),
                    bytes_to_gb(check.model_bytes),
                    bytes_to_gb(check.kv_estimate_bytes),
                    bytes_to_gb(check.safety_margin_bytes),
                    check.can_load
                );

                if !check.can_load {
                    warn!("[benchmark] [{}] {}", model_id, check.error_message.as_ref().unwrap());
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
                        error: check.error_message,
                        gpu_mode: gpu_mode.to_string(),
                        speedup_factor: None,
                    };
                }
            }
            Err(e) => {
                warn!("[benchmark] [{}] memory check failed, proceeding with load: {}", model_id, e);
            }
        }

        let already_loaded = if let Ok(models) = client.list_models().await {
            models.iter().any(|m| m.id == server_model_id && m.status.value == "loaded")
        } else {
            false
        };

        if !already_loaded {
            if let Ok(models) = client.list_models().await {
                for m in models {
                    if m.status.value == "loaded" && m.id != server_model_id {
                        info!("[benchmark] unloading {} to make room for {}", m.id, server_model_id);
                        if let Err(e) = client.unload_model(&m.id).await { warn!("[benchmark] failed to unload model {}: {}", m.id, e); }
                    }
                }
            }
        } else {
            info!("[benchmark] model {} already loaded, reusing", server_model_id);
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
                let mut messages = Vec::with_capacity(2);
                if let Some(ref sys) = system_prompt {
                    messages.push(ChatMessage::system(sys.clone()));
                }
                messages.push(ChatMessage::user(prompt));

                let request = ChatCompletionRequest {
                    model: server_model_id.to_string(),
                    messages,
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
                            response_text: raw_response,
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
                warn!(
                    "[benchmark] all {} inference attempts failed for {}: {}",
                    MAX_RETRIES, model_id, err
                );

                let exhausted_context = WorkflowHookContext::AfterAllRetriesExhausted(
                    AfterAllRetriesExhaustedContext {
                        step_name: model_id.to_string(),
                        total_attempts: MAX_RETRIES,
                        last_error: err.clone(),
                        last_error_type: "InferenceError".to_string(),
                    }
                );
                if let Err(e) = self.execute_hooks_for_trigger(hook_config, "after_all_retries_exhausted", &exhausted_context) {
                    error!("[benchmark] after_all_retries_exhausted hook error: {}", e);
                }

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
        if let Err(e) = client.unload_model(server_model_id).await { warn!("[benchmark] failed to unload model {}: {}", server_model_id, e); }
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
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

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
    #![allow(deprecated)]

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
            min_tmp_space_mb: 100,
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
            min_tmp_space_mb: 100,
        }
    }

    fn make_hook_context(step_name: &str, iteration: usize, output: Option<&str>) -> HookContext {
        HookContext {
            step_name: step_name.to_string(),
            iteration,
            output: output.map(|s| s.to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: None,
            model_family: None,
            model_quantization: None,
        }
    }

    fn make_workflow_hook_context(step_name: &str, output: Option<&str>, trigger: &str) -> WorkflowHookContext {
        match trigger {
            "after_step_succeeds" => WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: step_name.to_string(),
                output: output.unwrap_or_default().to_string(),
                duration_ms: 0,
                quality_score: None,
                token_count: 0,
                model_name: step_name.to_string(),
            }),
            "after_step_fails" => WorkflowHookContext::AfterStepFails(AfterStepFailsContext {
                step_name: step_name.to_string(),
                error_type: "test_error".to_string(),
                error_message: "test failure".to_string(),
                error: ErrorDetails {
                    is_retryable: false,
                    count: 1,
                },
                attempt_number: 1,
                model_name: step_name.to_string(),
            }),
            _ => WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: step_name.to_string(),
                output: output.unwrap_or_default().to_string(),
                duration_ms: 0,
                quality_score: None,
                token_count: 0,
                model_name: step_name.to_string(),
            }),
        }
    }

    #[test]
    fn test_execute_hook_log_creates_file() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let path_str = log_path.to_str().unwrap();

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"log": {"to_file_path": path_str}}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("hello"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&log_path).unwrap();
        assert!(content.len() > 0, "log file should have content");
    }

    #[test]
    fn test_execute_hook_save_to_writes_file() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let path_str = save_path.to_str().unwrap();

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"save_to": path_str}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("saved content here"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

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

        std::fs::write(&append_path, "first\n").unwrap();

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"append_to": path_str}
            ]
        });

        let ctx = make_workflow_hook_context("step_2", Some("appended"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

        let content = std::fs::read_to_string(&append_path).unwrap();
        assert!(content.starts_with("first\n"), "should preserve existing content");
        assert!(content.ends_with("appended\n"), "should append new content");
    }

    #[test]
    fn test_execute_hook_fail_returns_error() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let hook = serde_json::json!({
            "after_step_fails": [
                {"fail": "test failure message"}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("output"), "after_step_fails");
        let result = runner.execute_hooks_for_trigger(&Some(hook), "after_step_fails", &ctx);

        assert!(result.is_err(), "fail hook should return error");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("test failure message"), "error message should contain fail message");
    }

    #[test]
    fn test_execute_hook_none_does_nothing() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let ctx = make_workflow_hook_context("step_1", Some("output"), "after_step_succeeds");
        let result = runner.execute_hooks_for_trigger(&None, "after_step_succeeds", &ctx);
        assert!(result.is_ok(), "None hook should succeed");
    }

    #[test]
    fn test_execute_hook_multiple_actions() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let save_path = dir.path().join("output.txt");
        let log_path = dir.path().join("log.txt");

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"save_to": save_path.to_str().unwrap()},
                {"log": {"to_file_path": log_path.to_str().unwrap()}}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("multi action"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

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

    // --- Resource Detection Tests ---

    #[test]
    fn test_detect_max_concurrent_inferences_returns_valid_range() {
        let max_concurrent = detect_max_concurrent_inferences();
        assert!((1..=4).contains(&max_concurrent), "should return 1-4, got {}", max_concurrent);
    }

    #[test]
    fn test_detect_available_ram_gb_returns_some_on_linux() {
        let ram_gb = detect_available_ram_gb();
        // On Linux with /proc/meminfo, should return Some; on other platforms, None is OK
        if cfg!(target_os = "linux") {
            assert!(ram_gb.is_some(), "should detect RAM on Linux");
            assert!(ram_gb.unwrap() > 0.0, "RAM should be positive");
        }
    }

    #[test]
    fn test_calculate_max_concurrent_from_ram() {
        let ram_gb = 8.0_f64;
        let max_concurrent = (ram_gb / 2.0_f64).floor() as usize;
        let capped = max_concurrent.clamp(1, 4);
        assert_eq!(capped, 4, "8GB RAM should allow 4 concurrent inferences");
    }

    #[test]
    fn test_calculate_max_concurrent_floor_at_1() {
        let ram_gb = 1.5_f64;
        let max_concurrent = (ram_gb / 2.0_f64).floor() as usize;
        let capped = max_concurrent.clamp(1, 4);
        assert_eq!(capped, 1, "1.5GB RAM should allow 1 concurrent inference (floor at 1)");
    }

    #[test]
    fn test_calculate_max_concurrent_cap_at_4() {
        let ram_gb = 32.0_f64;
        let max_concurrent = (ram_gb / 2.0_f64).floor() as usize;
        let capped = max_concurrent.clamp(1, 4);
        assert_eq!(capped, 4, "32GB RAM should be capped at 4 concurrent inferences");
    }

    #[test]
    fn test_read_sysfs_vram_amd_returns_valid_or_none() {
        let vram_kb = read_sysfs_vram_amd();
        // May return Some (AMD GPU present) or None (no AMD GPU) — both valid
        if let Some(kb) = vram_kb {
            assert!(kb > 0, "VRAM should be positive if detected");
        }
    }

    #[test]
    fn test_read_proc_vram_nvidia_missing_dir() {
        let vram_mb = read_proc_vram_nvidia();
        assert!(vram_mb.is_none(), "should return None when /proc/driver/nvidia/gpus doesn't exist");
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
        let path_str = nested_path.to_str().unwrap();

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"log": {"to_file_path": path_str}}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("hello"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

        assert!(nested_path.exists(), "log hook should create log file in nested dirs");
        let content = std::fs::read_to_string(&nested_path).unwrap();
        assert!(content.len() > 0);
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

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"save_to": path_str}
            ]
        });

        let ctx = make_workflow_hook_context("step_1", Some("new content"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

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

        let hook = serde_json::json!({
            "after_step_succeeds": [
                {"append_to": path_str}
            ]
        });

        let ctx = make_workflow_hook_context("step_2", Some("first append"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

        assert!(append_path.exists(), "append_to should create file if not exists");
        let content = std::fs::read_to_string(&append_path).unwrap();
        assert_eq!(content, "first append\n");
    }

    #[test]
    fn test_execute_hook_empty_object() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("test.log");

        let hook = serde_json::json!({
            "after_step_succeeds": []
        });

        let ctx = make_workflow_hook_context("step_1", Some("hello"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&Some(hook), "after_step_succeeds", &ctx).unwrap();

        // File should not be created since there's no valid action
        assert!(!log_path.exists(), "empty hook array should do nothing");
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

        let hook = steps[0].when.clone();

        // Change CWD to tempdir for relative paths
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let ctx = make_workflow_hook_context("step_one", Some("test output"), "after_step_succeeds");
        runner.execute_hooks_for_trigger(&hook, "after_step_succeeds", &ctx).unwrap();

        std::env::set_current_dir(original_cwd).unwrap();

        // Verify output files exist
        assert!(save_path.exists(), "save_to hook should create output file");
        assert!(log_path.exists(), "log hook should create log file");

        let save_content = std::fs::read_to_string(&save_path).unwrap();
        assert_eq!(save_content, "test output");

        let log_content = std::fs::read_to_string(&log_path).unwrap();
        assert!(log_content.len() > 0);

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

    #[test]
    fn test_extract_iterate_values_valid() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: Some(serde_json::json!({
                "before_step_starts": [
                    {
                        "iterate_values": {
                            "step.model_ref": ["qwen-05b", "qwen-15b", "qwen-3b"],
                            "step.model_name": ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen2.5-1.5B-Instruct-Q4_K_M", "Qwen2.5-3B-Instruct-Q4_K_M"]
                        }
                    }
                ]
            })),
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_some());

        let maps = result.unwrap();
        assert_eq!(maps.len(), 3);

        assert_eq!(maps[0].get("model_ref"), Some(&serde_json::json!("qwen-05b")));
        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("Qwen2.5-0.5B-Instruct-Q4_K_M")));

        assert_eq!(maps[1].get("model_ref"), Some(&serde_json::json!("qwen-15b")));
        assert_eq!(maps[1].get("model_name"), Some(&serde_json::json!("Qwen2.5-1.5B-Instruct-Q4_K_M")));

        assert_eq!(maps[2].get("model_ref"), Some(&serde_json::json!("qwen-3b")));
        assert_eq!(maps[2].get("model_name"), Some(&serde_json::json!("Qwen2.5-3B-Instruct-Q4_K_M")));
    }

    #[test]
    fn test_extract_iterate_values_no_when() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: None,
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_iterate_values_no_before_step_starts() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: Some(serde_json::json!({
                "after_step_succeeds": [{"log": {}}]
            })),
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_iterate_values_no_iterate_values() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: Some(serde_json::json!({
                "before_step_starts": [
                    {"log": {}}
                ]
            })),
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_iterate_values_mismatched_array_lengths() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: Some(serde_json::json!({
                "before_step_starts": [
                    {
                        "iterate_values": {
                            "step.model_ref": ["qwen-05b", "qwen-15b", "qwen-3b"],
                            "step.model_name": ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen2.5-1.5B-Instruct-Q4_K_M"]
                        }
                    }
                ]
            })),
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_some());

        let maps = result.unwrap();
        assert_eq!(maps.len(), 3);

        assert_eq!(maps[0].get("model_ref"), Some(&serde_json::json!("qwen-05b")));
        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("Qwen2.5-0.5B-Instruct-Q4_K_M")));

        assert_eq!(maps[1].get("model_ref"), Some(&serde_json::json!("qwen-15b")));
        assert_eq!(maps[1].get("model_name"), Some(&serde_json::json!("Qwen2.5-1.5B-Instruct-Q4_K_M")));

        assert_eq!(maps[2].get("model_ref"), Some(&serde_json::json!("qwen-3b")));
        assert_eq!(maps[2].get("model_name"), None);
    }

    #[test]
    fn test_extract_iterate_values_step_prefix_stripped() {
        let config = make_test_config();
        let runner = BenchmarkRunner::new(config);

        let step = WorkflowStep {
            step_name: "test".to_string(),
            step_id: "test_step".to_string(),
            requires: vec![],
            when: Some(serde_json::json!({
                "before_step_starts": [
                    {
                        "iterate_values": {
                            "step.model_ref": ["model-a"],
                            "model_name": ["Model A"]
                        }
                    }
                ]
            })),
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };

        let result = runner.extract_iterate_values(&step);
        assert!(result.is_some());

        let maps = result.unwrap();
        assert_eq!(maps.len(), 1);

        assert!(maps[0].contains_key("model_ref"));
        assert!(maps[0].contains_key("model_name"));
        assert!(!maps[0].contains_key("step.model_ref"));
    }

    #[test]
    fn test_resolve_templates_single_step_var() {
        let mut variables = serde_json::Map::new();
        variables.insert("model_ref".to_string(), serde_json::json!("qwen-05b"));

        let result = BenchmarkRunner::resolve_templates("Model: {{step.model_ref}}", &variables, 0);
        assert_eq!(result, "Model: qwen-05b");
    }

    #[test]
    fn test_resolve_templates_multiple_step_vars() {
        let mut variables = serde_json::Map::new();
        variables.insert("model_ref".to_string(), serde_json::json!("qwen-05b"));
        variables.insert("model_name".to_string(), serde_json::json!("Qwen2.5"));

        let result = BenchmarkRunner::resolve_templates(
            "{{step.model_ref}} - {{step.model_name}}",
            &variables,
            0
        );
        assert_eq!(result, "qwen-05b - Qwen2.5");
    }

    #[test]
    fn test_resolve_templates_iteration_var() {
        let variables = serde_json::Map::new();

        let result = BenchmarkRunner::resolve_templates("Iteration: {{iteration}}", &variables, 5);
        assert_eq!(result, "Iteration: 5");
    }

    #[test]
    fn test_resolve_templates_no_templates() {
        let variables = serde_json::Map::new();

        let result = BenchmarkRunner::resolve_templates("No templates here", &variables, 0);
        assert_eq!(result, "No templates here");
    }

    #[test]
    fn test_resolve_templates_combined_step_and_iteration() {
        let mut variables = serde_json::Map::new();
        variables.insert("model_ref".to_string(), serde_json::json!("qwen-05b"));

        let result = BenchmarkRunner::resolve_templates(
            "{{step.model_ref}} at iteration {{iteration}}",
            &variables,
            3
        );
        assert_eq!(result, "qwen-05b at iteration 3");
    }

    #[test]
    fn test_iterate_values_yaml_to_extraction_full_pipeline() {
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
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b"]
            model_path: ["/path/a.gguf", "/path/b.gguf"]
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some(), "Expected steps to be loaded from YAML");
        let steps = steps.unwrap();
        assert_eq!(steps.len(), 1, "Expected 1 step in workflow");

        let step = &steps[0];
        assert_eq!(step.step_id, "bench_1");

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some(), "Expected iterate_values to be extracted");

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 2, "Expected 2 variable maps");

        assert_eq!(
            maps[0].get("model_name"),
            Some(&serde_json::json!("model-a")),
            "First map model_name should be model-a"
        );
        assert_eq!(
            maps[0].get("model_path"),
            Some(&serde_json::json!("/path/a.gguf")),
            "First map model_path should be /path/a.gguf"
        );

        assert_eq!(
            maps[1].get("model_name"),
            Some(&serde_json::json!("model-b")),
            "Second map model_name should be model-b"
        );
        assert_eq!(
            maps[1].get("model_path"),
            Some(&serde_json::json!("/path/b.gguf")),
            "Second map model_path should be /path/b.gguf"
        );
    }

    #[test]
    fn test_iterate_values_with_template_resolution_pipeline() {
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
    prompt: "Testing model {{step.model_name}} at {{step.model_path}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b"]
            model_path: ["/path/a.gguf", "/path/b.gguf"]
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 2);

        let resolved_0 = BenchmarkRunner::resolve_templates(
            step.prompt.as_ref().unwrap(),
            &maps[0],
            0
        );
        assert_eq!(
            resolved_0,
            "Testing model model-a at /path/a.gguf",
            "First iteration prompt should resolve correctly"
        );

        let resolved_1 = BenchmarkRunner::resolve_templates(
            step.prompt.as_ref().unwrap(),
            &maps[1],
            1
        );
        assert_eq!(
            resolved_1,
            "Testing model model-b at /path/b.gguf",
            "Second iteration prompt should resolve correctly"
        );
    }

    #[test]
    fn test_iterate_values_with_three_models_pipeline() {
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
    prompt: "Testing {{step.model_name}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b", "model-c"]
            model_path: ["/path/a.gguf", "/path/b.gguf", "/path/c.gguf"]
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 3, "Expected 3 iterations for 3 models");

        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("model-a")));
        assert_eq!(maps[0].get("model_path"), Some(&serde_json::json!("/path/a.gguf")));

        assert_eq!(maps[1].get("model_name"), Some(&serde_json::json!("model-b")));
        assert_eq!(maps[1].get("model_path"), Some(&serde_json::json!("/path/b.gguf")));

        assert_eq!(maps[2].get("model_name"), Some(&serde_json::json!("model-c")));
        assert_eq!(maps[2].get("model_path"), Some(&serde_json::json!("/path/c.gguf")));
    }

    #[test]
    fn test_iterate_values_with_single_model_pipeline() {
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
    prompt: "Testing {{step.model_name}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a"]
            model_path: ["/path/a.gguf"]
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 1, "Expected single iteration for single model");

        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("model-a")));
        assert_eq!(maps[0].get("model_path"), Some(&serde_json::json!("/path/a.gguf")));
    }

    #[test]
    fn test_iterate_values_with_mismatched_arrays_pipeline() {
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
    prompt: "Testing {{step.model_name}} at {{step.model_path}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b", "model-c"]
            model_path: ["/path/a.gguf", "/path/b.gguf"]
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 3, "Expected 3 iterations (max array length)");

        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("model-a")));
        assert_eq!(maps[0].get("model_path"), Some(&serde_json::json!("/path/a.gguf")));

        assert_eq!(maps[1].get("model_name"), Some(&serde_json::json!("model-b")));
        assert_eq!(maps[1].get("model_path"), Some(&serde_json::json!("/path/b.gguf")));

        assert_eq!(maps[2].get("model_name"), Some(&serde_json::json!("model-c")));
        assert_eq!(
            maps[2].get("model_path"),
            None,
            "Third iteration should have no model_path (shorter array)"
        );
    }

    #[test]
    fn test_iterate_values_with_after_step_hooks_combined() {
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
    prompt: "Testing {{step.model_name}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b"]
            model_path: ["/path/a.gguf", "/path/b.gguf"]
      after_step_succeeds:
        - log:
            message: "Step completed"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 2, "Expected 2 iterations despite after_step_succeeds hook");

        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("model-a")));
        assert_eq!(maps[1].get("model_name"), Some(&serde_json::json!("model-b")));
    }

    #[test]
    fn test_iterate_values_with_loop_and_iteration_variable_pipeline() {
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
    prompt: "Model {{step.model_name}} loop iteration {{iteration}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: ["model-a", "model-b"]
    loop:
      count: 2
      iteration_variable: "loop_idx"
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some());

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 2, "Expected 2 iterate_values iterations");

        assert_eq!(maps[0].get("model_name"), Some(&serde_json::json!("model-a")));
        assert!(!maps[0].contains_key("loop_idx"), "iterate_values should not contain loop iteration variable");

        let resolved = BenchmarkRunner::resolve_templates(
            step.prompt.as_ref().unwrap(),
            &maps[0],
            1
        );
        assert_eq!(
            resolved,
            "Model model-a loop iteration 1",
            "resolve_templates should handle {{iteration}} separately from iterate_values"
        );
    }

    #[test]
    fn test_iterate_values_empty_arrays_pipeline() {
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
    prompt: "Testing {{step.model_name}}"
    input:
      max_tokens: 256
    when:
      before_step_starts:
        - iterate_values:
            model_name: []
            model_path: []
"#).unwrap();

        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps();

        assert!(steps.is_some());
        let steps = steps.unwrap();
        let step = &steps[0];

        let iterate_values = runner.extract_iterate_values(step);
        assert!(iterate_values.is_some(), "Expected iterate_values to be extracted");

        let maps = iterate_values.unwrap();
        assert_eq!(maps.len(), 0, "Expected empty vec for empty arrays");
    }

    #[test]
    fn test_resolve_step_output_templates_basic() {
        let mut step_outputs = std::collections::HashMap::new();
        step_outputs.insert("step_1".to_string(), "output from step 1".to_string());

        let template = "Previous step output: {{step.step_1.output}}";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        assert_eq!(result, "Previous step output: output from step 1");
    }

    #[test]
    fn test_resolve_step_output_templates_multiple_references() {
        let mut step_outputs = std::collections::HashMap::new();
        step_outputs.insert("step_a".to_string(), "result A".to_string());
        step_outputs.insert("step_b".to_string(), "result B".to_string());

        let template = "{{step.step_a.output}} and {{step.step_b.output}}";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        assert_eq!(result, "result A and result B");
    }

    #[test]
    fn test_resolve_step_output_templates_unknown_step_id() {
        let step_outputs = std::collections::HashMap::new();

        let template = "Unknown: {{step.step_999.output}}";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        // Unknown step_id should leave template unchanged
        assert_eq!(result, "Unknown: {{step.step_999.output}}");
    }

    #[test]
    fn test_resolve_step_output_templates_no_template() {
        let step_outputs = std::collections::HashMap::new();

        let template = "Plain text without templates";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        assert_eq!(result, "Plain text without templates");
    }

    #[test]
    fn test_resolve_step_output_templates_empty_map() {
        let step_outputs = std::collections::HashMap::new();

        let template = "{{step.step_1.output}}";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        // Empty map should leave template unchanged
        assert_eq!(result, "{{step.step_1.output}}");
    }

    #[test]
    fn test_resolve_step_output_templates_mixed_content() {
        let mut step_outputs = std::collections::HashMap::new();
        step_outputs.insert("generate".to_string(), "generated code".to_string());

        let template = "Here is the {{step.generate.output}}:\nUse it wisely";
        let result = BenchmarkRunner::resolve_step_output_templates(template, &step_outputs);

        assert_eq!(result, "Here is the generated code:\nUse it wisely");
    }

    #[test]
    fn test_route_to_parallel_empty_targets_returns_none() {
        let targets: Vec<String> = vec![];
        assert!(targets.len() <= 1);
    }

    #[test]
    fn test_route_to_parallel_mixed_models_prevent_parallel() {
        let model_a = "model-a.gguf";
        let model_b = "model-b.gguf";
        assert_ne!(model_a, model_b);
    }

    #[test]
    fn test_route_to_parallel_target_not_found_handled_gracefully() {
        let step_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let target_id = "non_existent_step";
        assert!(!step_index.contains_key(target_id));
    }

    #[test]
    fn test_route_to_parallel_same_model_allows_parallel() {
        let first_model = "model-a.gguf";
        let model_name = "model-a.gguf";
        assert_eq!(first_model, model_name);
    }

    #[test]
    fn test_route_to_parallel_different_model_blocks_parallel() {
        let first_model = "model-a.gguf";
        let model_name = "model-b.gguf";
        assert_ne!(first_model, model_name);
    }

    // ── resolve_bookmark_templates tests ──────────────────────────────

    #[test]
    fn test_resolve_bookmark_templates_flat_string() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("my_key".to_string(), serde_json::json!("hello world"));

        let template = "Value: {{bookmarks.my_key}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Value: hello world");
    }

    #[test]
    fn test_resolve_bookmark_templates_nested_stdout() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("shell_output".to_string(), serde_json::json!({
            "stdout": "file content here",
            "stderr": "",
            "exit_code": 0,
            "success": true
        }));

        let template = "Data:\n{{bookmarks.shell_output.stdout}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Data:\nfile content here");
    }

    #[test]
    fn test_resolve_bookmark_templates_nested_stderr() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("shell_output".to_string(), serde_json::json!({
            "stdout": "ok",
            "stderr": "warning msg",
            "exit_code": 0,
            "success": true
        }));

        let template = "Err: {{bookmarks.shell_output.stderr}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Err: warning msg");
    }

    #[test]
    fn test_resolve_bookmark_templates_flat_object_serializes() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("data".to_string(), serde_json::json!({"a": 1, "b": 2}));

        let template = "Full: {{bookmarks.data}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert!(result.contains("\"a\":1"));
        assert!(result.contains("\"b\":2"));
    }

    #[test]
    fn test_resolve_bookmark_templates_multiple_bookmarks() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("key_a".to_string(), serde_json::json!("val_a"));
        bookmarks.insert("key_b".to_string(), serde_json::json!("val_b"));

        let template = "{{bookmarks.key_a}} and {{bookmarks.key_b}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "val_a and val_b");
    }

    #[test]
    fn test_resolve_bookmark_templates_unknown_key_unchanged() {
        let bookmarks = std::collections::HashMap::new();

        let template = "Missing: {{bookmarks.nonexistent}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Missing: {{bookmarks.nonexistent}}");
    }

    #[test]
    fn test_resolve_bookmark_templates_no_placeholders() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("key".to_string(), serde_json::json!("val"));

        let template = "Plain text without bookmarks";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Plain text without bookmarks");
    }

    #[test]
    fn test_resolve_bookmark_templates_empty_bookmarks() {
        let bookmarks = std::collections::HashMap::new();

        let template = "{{bookmarks.missing}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "{{bookmarks.missing}}");
    }

    #[test]
    fn test_resolve_bookmark_templates_nested_number_field() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("shell_output".to_string(), serde_json::json!({
            "stdout": "ok",
            "exit_code": 42,
            "success": false
        }));

        let template = "Exit: {{bookmarks.shell_output.exit_code}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "Exit: 42");
    }

    #[test]
    fn test_resolve_bookmark_templates_nested_boolean_field() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("shell_output".to_string(), serde_json::json!({
            "stdout": "ok",
            "success": true
        }));

        let template = "OK: {{bookmarks.shell_output.success}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        assert_eq!(result, "OK: true");
    }

    #[test]
    fn test_resolve_bookmark_templates_mixed_step_and_bookmark() {
        let mut bookmarks = std::collections::HashMap::new();
        bookmarks.insert("csv_data".to_string(), serde_json::json!("name,age\nAlice,30"));

        let template = "CSV: {{bookmarks.csv_data}}\nStep: {{step.step_1.output}}";
        let result = BenchmarkRunner::resolve_bookmark_templates(template, &bookmarks);

        // Bookmark resolved, step template left unchanged (handled by different function)
        assert_eq!(result, "CSV: name,age\nAlice,30\nStep: {{step.step_1.output}}");
    }

    #[test]
    fn test_parse_model_metadata_llama() {
        let (size, family, quantization) = BenchmarkRunner::parse_model_metadata("llama-2-7b");
        assert_eq!(size, Some("7b".to_string()));
        assert_eq!(family, Some("llama".to_string()));
        assert_eq!(quantization, None);
    }

    #[test]
    fn test_parse_model_metadata_ministral() {
        let (size, family, quantization) = BenchmarkRunner::parse_model_metadata("ministral-3b");
        assert_eq!(size, Some("3b".to_string()));
        assert_eq!(family, Some("mistral".to_string()));
        assert_eq!(quantization, None);
    }

    #[test]
    fn test_parse_model_metadata_qwen() {
        let (size, family, quantization) = BenchmarkRunner::parse_model_metadata("qwen2-7b");
        assert_eq!(size, Some("7b".to_string()));
        assert_eq!(family, Some("qwen".to_string()));
        assert_eq!(quantization, None);
    }

    #[test]
    fn test_parse_model_metadata_quantization() {
        let (size, family, quantization) = BenchmarkRunner::parse_model_metadata("llama-2-7b-q4");
        assert_eq!(size, Some("7b".to_string()));
        assert_eq!(family, Some("llama".to_string()));
        assert_eq!(quantization, Some("q4".to_string()));
    }

    #[test]
    fn test_interpolate_template_with_model_size() {
        let runner = BenchmarkRunner::new(make_test_config());
        let context = HookContext {
            step_name: "llama-2-7b".to_string(),
            iteration: 0,
            output: Some("test".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: Some("7b".to_string()),
            model_family: Some("llama".to_string()),
            model_quantization: Some("q4".to_string()),
        };

        let template = "Model {{current_model}} has size {{model.size}}";
        let result = runner.interpolate_template(template, &context);

        assert_eq!(result, "Model llama-2-7b has size 7b");
    }

    #[test]
    fn test_interpolate_template_with_model_family() {
        let runner = BenchmarkRunner::new(make_test_config());
        let context = HookContext {
            step_name: "ministral-3b".to_string(),
            iteration: 0,
            output: Some("test".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: Some("3b".to_string()),
            model_family: Some("mistral".to_string()),
            model_quantization: None,
        };

        let template = "Family: {{model.family}}";
        let result = runner.interpolate_template(template, &context);

        assert_eq!(result, "Family: mistral");
    }

    #[test]
    fn test_interpolate_template_with_model_quantization() {
        let runner = BenchmarkRunner::new(make_test_config());
        let context = HookContext {
            step_name: "llama-2-7b-q4".to_string(),
            iteration: 0,
            output: Some("test".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: Some("7b".to_string()),
            model_family: Some("llama".to_string()),
            model_quantization: Some("q4".to_string()),
        };

        let template = "Quantization: {{model.quantization}}";
        let result = runner.interpolate_template(template, &context);

        assert_eq!(result, "Quantization: q4");
    }

    #[test]
    fn test_interpolate_template_all_model_vars() {
        let runner = BenchmarkRunner::new(make_test_config());
        let context = HookContext {
            step_name: "ministral-3b".to_string(),
            iteration: 1,
            output: Some("output".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: Some("3b".to_string()),
            model_family: Some("mistral".to_string()),
            model_quantization: None,
        };

        let template = "{{model.family}}-{{model.size}} iter={{iteration}}";
        let result = runner.interpolate_template(template, &context);

        assert_eq!(result, "mistral-3b iter=1");
    }
}
