use crate::client::http_client::LlamaHttpClient;
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use crate::client::disk_monitor;
use crate::client::memory_monitor;
use crate::client::resource_guard;
use crate::model::schema::parse_resource_bytes;
use crate::agent::loop_hooks::HookContext;
use crate::workflow::hooks::{HookEngine, HookResult};
use crate::workflow::hooks::actions::execute_action;
use crate::workflow::hooks::context::{
    WorkflowHookContext, BeforeStepStartsContext,
    AfterStepSucceedsContext, AfterStepFailsContext, AfterAllRetriesExhaustedContext,
    AfterLoopIterationFailsContext, OnRequiresFailedContext, DuringStepStreamingContext,
    StepType, ErrorDetails,
};
use crate::workflow::HookAction;
use crate::workflow::ResourceAdmissionConfig;
use super::{BenchmarkSuiteResult, ModelBenchmarkResult, InferenceResult, WorkflowStepResult};
use super::detail_generator::DetailGenerator;
use super::model_selector::ModelSelector;
use crate::client::model_discovery::ModelCandidate;
use anyhow::{Context, Result};
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
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

/// Detect retry-exhausted errors via structured patterns (replaces brittle substring match).
/// Matches the formats produced by run_model_inference / benchmark_single_model when all attempts fail.
fn is_retry_exhausted_error(error: &str) -> bool {
    error.starts_with("All ") && error.contains(" attempts failed:")
}

/// Default refusal-detection patterns. Workflow YAML can override via
/// `workflow_execution_strategy.quality.refusal_patterns` (schema line ~244 area).
pub const DEFAULT_REFUSAL_PATTERS: &[&str] = &[
    "I cannot help", "I can't help", "I am unable to",
    "I'm unable to", "As an AI", "I'm not able to",
    "I am not able to", "I will not help",
    "I cannot directly", "I can't directly",
    "I cannot read", "I can't read",
    "I cannot access", "I can't access",
    "do not have access", "don't have access",
    "cannot access your local", "can't access your local",
    "I do not have", "I don't have",
    "I'm not able to read", "I am not able to read",
];

/// Compute quality_score from output text + configured refusal patterns.
/// Returns Some(0.0) if refusal detected, else passes through baseline_score.
fn detect_refusal(output: &str, patterns: &[String], baseline_score: Option<f32>) -> Option<f32> {
    let lower = output.to_lowercase();
    let matched = patterns.iter().find(|p| lower.contains(&p.to_lowercase()));
    if matched.is_some() {
        Some(0.0f32)
    } else {
        baseline_score
    }
}

fn require_step_prompt(step: &WorkflowStep) -> Result<String> {
    let is_control_flow = step.generative_entity.as_deref() == Some("control_flow");
    if is_control_flow {
        return Ok(String::new());
    }
    let prompt = step.prompt.as_ref()
        .filter(|p| !p.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!(
            "workflow step `{}` requires non-empty `prompt` field when generative_entity is not `control_flow`",
            step.step_id
        ))?;
    Ok(prompt.clone())
}

fn workflow_resource_request(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
) -> Result<resource_guard::WorkflowLoadRequest> {
    let bytes = |field: &str, value: &str| {
        parse_resource_bytes(value).ok_or_else(|| anyhow::anyhow!(
            "invalid workflow resource admission {field}: {value}"
        ))
    };
    Ok(resource_guard::WorkflowLoadRequest {
        model_name: model_name.to_string(),
        weights_bytes,
        minimum_ram_bytes: bytes("minimum_available.ram", &config.minimum_available.ram)?,
        minimum_vram_bytes: bytes("minimum_available.vram", &config.minimum_available.vram)?,
        minimum_swap_free_bytes: bytes(
            "minimum_available.swap_free",
            &config.minimum_available.swap_free,
        )?,
        kv_cache_bytes: bytes("model_estimate.kv_cache", &config.model_estimate.kv_cache)?,
        compute_buffer_bytes: bytes(
            "model_estimate.compute_buffer",
            &config.model_estimate.compute_buffer,
        )?,
        host_runtime_bytes: bytes(
            "model_estimate.host_runtime",
            &config.model_estimate.host_runtime,
        )?,
    })
}

fn workflow_resource_admission(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
    vram: &resource_guard::VramSensors,
    ram: &resource_guard::RamSensors,
) -> Result<resource_guard::Admission> {
    Ok(resource_guard::admit_workflow_load(
        &workflow_resource_request(config, model_name, weights_bytes)?,
        vram,
        ram,
    ))
}

fn workflow_preflight_resource_admission(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    vram: &resource_guard::VramSensors,
    ram: &resource_guard::RamSensors,
) -> Result<resource_guard::Admission> {
    Ok(resource_guard::admit_workflow_preflight(
        &workflow_resource_request(config, model_name, 0)?,
        vram,
        ram,
    ))
}

fn workflow_inference_resource_admission(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
    vram: &resource_guard::VramSensors,
    ram: &resource_guard::RamSensors,
) -> Result<resource_guard::Admission> {
    Ok(resource_guard::admit_workflow_inference(
        &workflow_resource_request(config, model_name, weights_bytes)?,
        vram,
        ram,
    ))
}

fn workflow_resource_admission_from_samples(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
    vram: Option<&resource_guard::VramSensors>,
    ram: Option<&resource_guard::RamSensors>,
) -> Result<resource_guard::Admission> {
    match (vram, ram) {
        (Some(vram), Some(ram)) => {
            workflow_resource_admission(config, model_name, weights_bytes, vram, ram)
        }
        _ => Ok(resource_guard::Admission::Rejected {
            code: resource_guard::REJECT_WORKFLOW_SENSORS,
            detail: "workflow resource admission requires readable RAM and VRAM sensors".into(),
        }),
    }
}

fn workflow_preflight_resource_admission_from_samples(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    vram: Option<&resource_guard::VramSensors>,
    ram: Option<&resource_guard::RamSensors>,
) -> Result<resource_guard::Admission> {
    match (vram, ram) {
        (Some(vram), Some(ram)) => {
            workflow_preflight_resource_admission(config, model_name, vram, ram)
        }
        _ => Ok(resource_guard::Admission::Rejected {
            code: resource_guard::REJECT_WORKFLOW_SENSORS,
            detail: "workflow resource admission requires readable RAM and VRAM sensors".into(),
        }),
    }
}

fn workflow_inference_resource_admission_from_samples(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
    vram: Option<&resource_guard::VramSensors>,
    ram: Option<&resource_guard::RamSensors>,
) -> Result<resource_guard::Admission> {
    match (vram, ram) {
        (Some(vram), Some(ram)) => {
            workflow_inference_resource_admission(config, model_name, weights_bytes, vram, ram)
        }
        _ => Ok(resource_guard::Admission::Rejected {
            code: resource_guard::REJECT_WORKFLOW_SENSORS,
            detail: "workflow resource admission requires readable RAM and VRAM sensors".into(),
        }),
    }
}

fn workflow_model_state_resource_admission_from_samples(
    config: &ResourceAdmissionConfig,
    model_name: &str,
    weights_bytes: u64,
    model_is_loaded: bool,
    vram: Option<&resource_guard::VramSensors>,
    ram: Option<&resource_guard::RamSensors>,
) -> Result<resource_guard::Admission> {
    if model_is_loaded {
        workflow_inference_resource_admission_from_samples(
            config,
            model_name,
            weights_bytes,
            vram,
            ram,
        )
    } else {
        workflow_resource_admission_from_samples(
            config,
            model_name,
            weights_bytes,
            vram,
            ram,
        )
    }
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    hook_engine: Arc<Mutex<HookEngine>>,
    inference_semaphore: Arc<Semaphore>,
    inference_max_attempts: u32,
    refusal_patterns: Vec<String>,
    streaming_enabled: bool,
    detail_template: Option<String>,
    resource_admission: Option<ResourceAdmissionConfig>,
    resource_admission_telemetry: Arc<Mutex<BTreeMap<String, ResourceAdmissionTelemetryRecord>>>,
    resource_admission_started_at: Arc<Instant>,
    // Issue Y: when true, load <output_dir>/checkpoint.jsonl before the
    // workflow loop and skip steps whose outputs are already recorded.
    resume: bool,
    // E1: between-step pressure gate state (hysteresis machine). Blocks
    // inference under RED pressure so desktop VRAM/RAM spikes fail the
    // step instead of crashing the machine.
    pressure_tracker: Arc<Mutex<resource_guard::PressureTracker>>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        let max_concurrent = detect_max_concurrent_inferences();
        Self {
            config,
            hook_engine: Arc::new(Mutex::new(HookEngine::new())),
            inference_semaphore: Arc::new(Semaphore::new(max_concurrent)),
            inference_max_attempts: 3,
            refusal_patterns: DEFAULT_REFUSAL_PATTERS.iter().map(|s| s.to_string()).collect(),
            streaming_enabled: false,
            detail_template: None,
            resource_admission: None,
            resource_admission_telemetry: Arc::new(Mutex::new(BTreeMap::new())),
            resource_admission_started_at: Arc::new(Instant::now()),
            resume: false,
            pressure_tracker: Arc::new(Mutex::new(resource_guard::PressureTracker::new())),
        }
    }

    fn requires_default_load_admission(
        resource_admission: Option<&ResourceAdmissionConfig>,
    ) -> bool {
        resource_admission.is_none()
    }

    pub fn with_resume(mut self, resume: bool) -> Self {
        self.resume = resume;
        self
    }

    pub fn with_streaming_enabled(mut self, enabled: bool) -> Self {
        self.streaming_enabled = enabled;
        self
    }

    pub fn streaming_enabled(&self) -> bool {
        self.streaming_enabled
    }

    pub fn with_inference_max_attempts(mut self, max_attempts: u32) -> Self {
        self.inference_max_attempts = max_attempts.max(1);
        self
    }

    pub fn inference_max_attempts(&self) -> u32 {
        self.inference_max_attempts
    }

    pub fn with_refusal_patterns(mut self, patterns: Vec<String>) -> Self {
        if !patterns.is_empty() {
            self.refusal_patterns = patterns;
        }
        self
    }

    pub fn refusal_patterns(&self) -> &[String] {
        &self.refusal_patterns
    }

    fn current_workflow_preflight_resource_admission(
        &self,
        model_name: &str,
        weights_bytes: u64,
    ) -> Result<Option<resource_guard::Admission>> {
        let Some(config) = self.resource_admission.as_ref() else {
            return Ok(None);
        };
        let vram = resource_guard::read_vram_sensors_sysfs();
        let ram = resource_guard::read_ram_sensors_proc();
        let admission = workflow_preflight_resource_admission_from_samples(
            config,
            model_name,
            vram.as_ref(),
            ram.as_ref(),
        )?;
        self.record_workflow_resource_admission(
            model_name,
            weights_bytes,
            vram.as_ref(),
            ram.as_ref(),
            &admission,
        )?;
        Ok(Some(admission))
    }

    fn current_workflow_inference_resource_admission(
        &self,
        model_name: &str,
        weights_bytes: u64,
    ) -> Result<Option<resource_guard::Admission>> {
        let Some(config) = self.resource_admission.as_ref() else {
            return Ok(None);
        };
        let vram = resource_guard::read_vram_sensors_sysfs();
        let ram = resource_guard::read_ram_sensors_proc();
        let admission = workflow_inference_resource_admission_from_samples(
            config,
            model_name,
            weights_bytes,
            vram.as_ref(),
            ram.as_ref(),
        )?;
        self.record_workflow_resource_admission(
            model_name,
            weights_bytes,
            vram.as_ref(),
            ram.as_ref(),
            &admission,
        )?;
        Ok(Some(admission))
    }

    fn current_workflow_model_state_resource_admission(
        &self,
        model_name: &str,
        weights_bytes: u64,
        model_is_loaded: bool,
    ) -> Result<Option<resource_guard::Admission>> {
        let Some(config) = self.resource_admission.as_ref() else {
            return Ok(None);
        };
        let vram = resource_guard::read_vram_sensors_sysfs();
        let ram = resource_guard::read_ram_sensors_proc();
        let admission = workflow_model_state_resource_admission_from_samples(
            config,
            model_name,
            weights_bytes,
            model_is_loaded,
            vram.as_ref(),
            ram.as_ref(),
        )?;
        self.record_workflow_resource_admission(
            model_name,
            weights_bytes,
            vram.as_ref(),
            ram.as_ref(),
            &admission,
        )?;
        Ok(Some(admission))
    }

    fn preflight_workflow_resource_admission(&self) -> Result<()> {
        let vram = resource_guard::read_vram_sensors_sysfs();
        let ram = resource_guard::read_ram_sensors_proc();
        self.preflight_workflow_resource_admission_from_samples(vram.as_ref(), ram.as_ref())
    }

    fn preflight_workflow_resource_admission_from_samples(
        &self,
        vram: Option<&resource_guard::VramSensors>,
        ram: Option<&resource_guard::RamSensors>,
    ) -> Result<()> {
        let Some(config) = self.resource_admission.as_ref() else {
            return Ok(());
        };
        let admission = workflow_preflight_resource_admission_from_samples(
            config,
            "workflow-preflight",
            vram,
            ram,
        )?;
        self.record_workflow_resource_admission(
            "workflow-preflight",
            0,
            vram,
            ram,
            &admission,
        )?;
        match admission {
            resource_guard::Admission::Admitted => Ok(()),
            resource_guard::Admission::Rejected { code, detail } => {
                Err(anyhow::anyhow!("{code}: {detail}"))
            }
        }
    }

    fn record_workflow_resource_admission(
        &self,
        model_name: &str,
        weights_bytes: u64,
        vram: Option<&resource_guard::VramSensors>,
        ram: Option<&resource_guard::RamSensors>,
        admission: &resource_guard::Admission,
    ) -> Result<()> {
        let Some(config) = self.resource_admission.as_ref() else {
            return Ok(());
        };
        if !config.telemetry.write_profile {
            return Ok(());
        }
        let Some(output_dir) = self.config.output_dir.as_deref() else {
            return Ok(());
        };

        let mut records = self
            .resource_admission_telemetry
            .lock()
            .map_err(|_| anyhow::anyhow!("resource admission telemetry lock poisoned"))?;
        let record = records.entry(model_name.to_string()).or_default();
        record.check_count = record.check_count.saturating_add(1);
        record.weights_bytes = weights_bytes;
        if let Some(ram) = ram {
            record.minimum_available_ram_bytes = minimum_observation(
                record.minimum_available_ram_bytes,
                ram.mem_available_kb.saturating_mul(1024),
            );
            record.minimum_available_swap_free_bytes = minimum_observation(
                record.minimum_available_swap_free_bytes,
                ram.swap_free_kb.saturating_mul(1024),
            );
        }
        if let Some(vram) = vram {
            record.minimum_available_vram_bytes = minimum_observation(
                record.minimum_available_vram_bytes,
                vram.free_bytes,
            );
        }
        record.latest_admission = ResourceAdmissionTelemetryDecision::from(admission);
        let profile = ResourceAdmissionTelemetryProfile {
            resource_admission: config.clone(),
            observed_runtime_secs: self.resource_admission_started_at.elapsed().as_secs_f64(),
            models: records.clone(),
        };
        drop(records);

        let output_dir = Path::new(output_dir);
        fs::create_dir_all(output_dir).with_context(|| {
            format!("create resource admission telemetry directory {}", output_dir.display())
        })?;
        let bytes = serde_json::to_vec_pretty(&profile)
            .context("serialize resource admission telemetry profile")?;
        fs::write(output_dir.join("resource-admission-profile.json"), bytes)
            .context("write resource admission telemetry profile")?;
        Ok(())
    }
}

#[derive(Clone, Default, Serialize)]
struct ResourceAdmissionTelemetryRecord {
    check_count: u64,
    weights_bytes: u64,
    minimum_available_ram_bytes: Option<u64>,
    minimum_available_vram_bytes: Option<u64>,
    minimum_available_swap_free_bytes: Option<u64>,
    latest_admission: ResourceAdmissionTelemetryDecision,
}

#[derive(Clone, Default, Serialize)]
struct ResourceAdmissionTelemetryDecision {
    admitted: bool,
    code: Option<String>,
    detail: Option<String>,
}

impl From<&resource_guard::Admission> for ResourceAdmissionTelemetryDecision {
    fn from(admission: &resource_guard::Admission) -> Self {
        match admission {
            resource_guard::Admission::Admitted => Self {
                admitted: true,
                code: None,
                detail: None,
            },
            resource_guard::Admission::Rejected { code, detail } => Self {
                admitted: false,
                code: Some((*code).to_string()),
                detail: Some(detail.clone()),
            },
        }
    }
}

#[derive(Serialize)]
struct ResourceAdmissionTelemetryProfile {
    resource_admission: ResourceAdmissionConfig,
    observed_runtime_secs: f64,
    models: BTreeMap<String, ResourceAdmissionTelemetryRecord>,
}

fn minimum_observation(previous: Option<u64>, current: u64) -> Option<u64> {
    Some(previous.map_or(current, |value| value.min(current)))
}

struct BenchmarkWorkflowConfig {
    prompts: Vec<String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    compare_modes: bool,
    model_list: Vec<String>,
    model_source_paths: BTreeMap<String, String>,
    gpu_layers: usize,
    load_params_env_vars: Vec<(String, String)>,
    inference_max_attempts: u32,
    output_root: Option<String>,
    refusal_patterns: Vec<String>,
    cooldown_after_unload_secs: Option<u64>,
    model_load_timeout_secs: Option<u64>,
    min_tmp_space_mb: Option<u64>,
    streaming_enabled: Option<bool>,
    model_selection_strategy: Option<String>,
    diverse_n_count: Option<usize>,
    model_filter: Option<String>,
    detail_template: Option<String>,
    resource_admission: Option<ResourceAdmissionConfig>,
}

#[derive(Clone)]
struct WorkflowStep {
    step_name: String,
    step_id: String,
    requires: Vec<String>,
    // Conditions attached to `requires: [{step: X, condition: <expr>}]` entries
    // (unified-workflow-schema.yml:411-414). Keyed by dependency name.
    require_conditions: std::collections::HashMap<String, String>,
    // Per-step `retry.max_attempts` override (schema:273-276, StepRetryConfig).
    // None → global inference_max_attempts applies.
    retry_max_attempts: Option<u32>,
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
/// Delegates sizing to `client::resource_guard` (single decision box):
/// VRAM-total sizing, RAM fallback, and the 8GB→1 crash fix all live there.
fn detect_max_concurrent_inferences() -> usize {
    if let Some(n) = env_concurrency_override() {
        info!("[benchmark] WHITT_MAX_CONCURRENT_INFERENCES={} overridden (clamped to {})", n, n);
        return n;
    }

    if let Some(vram) = resource_guard::read_vram_sensors_sysfs() {
        let total_gb = resource_guard::bytes_to_gb(vram.total_bytes);
        let n = resource_guard::max_concurrent_for(total_gb);
        info!("[benchmark] auto-detected max concurrent inferences: {} (based on {:.1} GB VRAM total)", n, total_gb);
        return n;
    }

    if let Some(vram_gb) = read_proc_vram_nvidia().map(|mb| mb as f64 / 1024.0) {
        let n = resource_guard::max_concurrent_for(vram_gb);
        info!("[benchmark] auto-detected max concurrent inferences: {} (based on {:.1} GB VRAM total, nvidia)", n, vram_gb);
        return n;
    }

    if let Some(ram) = resource_guard::read_ram_sensors_proc() {
        let avail_gb = resource_guard::kb_to_gb(ram.mem_available_kb);
        let n = resource_guard::max_concurrent_ram(avail_gb);
        info!("[benchmark] auto-detected max concurrent inferences: {} (based on {:.1} GB RAM available)", n, avail_gb);
        return n;
    }

    info!("[benchmark] could not detect available memory, using default: 2 concurrent inferences");
    2
}

/// Parse WHITT_MAX_CONCURRENT_INFERENCES: valid values are integers >= 1.
fn env_concurrency_override() -> Option<usize> {
    std::env::var("WHITT_MAX_CONCURRENT_INFERENCES").ok()?
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|n| *n >= 1)
}

/// NVIDIA /proc fallback (MiB); AMD sysfs + unit math live in resource_guard.
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
    /// Pre-flight checks: verify system health before starting benchmark.
    /// Zombie detector must count only llama-server children, never
    /// the router parent (router-mode legitimately runs 1 router +
    /// N loaded models; counting the router false-positives at N=2).
    fn zombie_threshold() -> i32 {
        std::env::var("WHITT_ZOMBIE_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2)
    }

    fn zombie_check_command(own_port: u16) -> String {
        format!(
            "ps -eo stat,args | grep -v 'Z' | grep llama-server | grep -v -- --models-dir | grep -v -- --port {} | grep -c llama-server",
            own_port
        )
    }

    /// Server port parsed from the configured server_url; used to exclude
    /// our own live server from the zombie count.
    fn own_server_port(server_url: &str) -> u16 {
        server_url
            .rsplit(':')
            .next()
            .and_then(|p| p.trim_matches('/').parse().ok())
            .unwrap_or(8080)
    }

    /// Temp dir used for disk-space checks. Honors TMPDIR (non-empty),
    /// falls back to /tmp — preflight previously hardcoded /tmp and
    /// aborted when a foreign process filled it despite TMPDIR pointing
    /// at a roomier volume (SUMMARY-overcontext.md:88-91).
    fn tmp_root() -> std::path::PathBuf {
        std::env::var("TMPDIR")
            .ok()
            .filter(|v| !v.is_empty())
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new("/tmp").to_path_buf())
    }

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
        let tmp = Self::tmp_root();
        match disk_monitor::ensure_min_free_space(&tmp, min_bytes) {
            Ok(info) => {
                let available_mb = info.available_bytes / (1024 * 1024);
                info!("[benchmark] ✓ {} space check passed: {} MB available (min: {} MB required)",
                    tmp.display(), available_mb, self.config.min_tmp_space_mb);
            }
            Err(e) => {
                let msg = format!("{} space check failed: {}", tmp.display(), e);
                info!("[benchmark] ✗ {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
        }

        // Count only non-defunct (running/sleeping) llama-server processes,
        // excluding the router parent and our own live server
        let own_port = Self::own_server_port(&self.config.server_url);
        let zombie_check = TokioCommand::new("sh")
            .args(["-c", &Self::zombie_check_command(own_port)])
            .output()
            .await;
        match zombie_check {
            Ok(output) => {
                let count_str = String::from_utf8_lossy(&output.stdout);
                let count: i32 = count_str.trim().parse().unwrap_or(0);
                if count <= Self::zombie_threshold() {
                    info!("[benchmark] ✓ Zombie process check passed: {} llama-server process(es) found", count);
                } else {
                    let msg = format!("Found {} llama-server processes (threshold {})",
                    count, Self::zombie_threshold());
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

        // L5 crash recovery: scan server logs for Vulkan/driver crash signatures
        // from THIS server incarnation (restarts don't rotate docker logs, so
        // scan from the container's StartedAt); proceeding would compound the fault.
        let container = Self::server_container_name();
        let started_at = TokioCommand::new("docker")
            .args(["inspect", "-f", "{{.State.StartedAt}}", &container])
            .output()
            .await
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty());
        let mut logs_cmd = TokioCommand::new("docker");
        logs_cmd.arg("logs");
        if let Some(ts) = &started_at {
            logs_cmd.args(["--since", ts]);
        }
        logs_cmd.args(["--tail", "300", &container]);
        let log_output = logs_cmd.output().await;
        match log_output {
            Ok(output) => {
                let logs = String::from_utf8_lossy(&output.stdout).to_string()
                    + &String::from_utf8_lossy(&output.stderr);
                let hits = resource_guard::vk_crash_signatures(&logs);
                if hits.is_empty() {
                    info!("[benchmark] ✓ server crash-signature scan clean ({} logs)", container);
                } else {
                    let msg = format!(
                        "RESOURCE_ABORT: llama server log shows {} prior crash signature(s): {} \
                         — restart the server before running (docker restart {})",
                        hits.len(), hits.join(", "), container
                    );
                    info!("[benchmark] ✗ {}", msg);
                    anyhow::bail!(crate::error::Error::benchmark(msg));
                }
            }
            Err(e) => {
                warn!("[benchmark] ⚠ server log scan skipped (docker logs failed: {})", e);
            }
        }

        info!("[benchmark] all preflight checks passed");
        Ok(())
    }

    fn server_container_name() -> String {
        std::env::var("WHITT_SERVER_CONTAINER").unwrap_or_else(|_| "whitt-llama-server".to_string())
    }

    /// Runtime system health check: verify resources are adequate before operations.
    #[deprecated(since = "0.4.0", note = "Use hook-driven preflight via before_workflow_starts trigger")]
    pub async fn check_system_health(&self) -> Result<()> {
        let min_bytes = self.config.min_tmp_space_mb * 1024 * 1024;
        let tmp = Self::tmp_root();
        match disk_monitor::check_disk_space(&tmp) {
            Ok(info) => {
                let available_mb = info.available_bytes / (1024 * 1024);
                info!("[benchmark] {} space: {} MB available (min: {} MB required)",
                    tmp.display(), available_mb, self.config.min_tmp_space_mb);
                if info.available_bytes < min_bytes {
                    let msg = format!("Insufficient {} space: {} MB available, {} MB required",
                        tmp.display(), available_mb, self.config.min_tmp_space_mb);
                    anyhow::bail!(crate::error::Error::benchmark(msg));
                }
            }
            Err(e) => {
                let msg = format!("Failed to check {} space: {}", tmp.display(), e);
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
        let tmp = Self::tmp_root();
        if let Ok(info) = disk_monitor::check_disk_space(&tmp) {
            let available_mb = info.available_bytes / (1024 * 1024);
            info!("[benchmark] [{}] resource state {}: {} available={} MB", model_id, phase, tmp.display(), available_mb);
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
            step_id: Some(step_id.to_string()),
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
                return Err(anyhow::anyhow!(
                    "[benchmark] cannot read --workflow file {}: {} — \
                     refusing to fall back to discovery benchmark \
                     (silent-fallback fix; original: flan-t5 loaded by accident)",
                    wf_path,
                    e
                ));
            }
        };

        let workflow = crate::workflow::WorkflowFile::from_yaml(&content)
            .map_err(|e| anyhow::anyhow!(
                "[benchmark] workflow YAML validation failed for {}: {} — \
                 engine now requires valid workflow YAML when --workflow is provided \
                 (cycle-3 hardening: silent fallback removed)",
                wf_path,
                e
            ))?;
        let resource_admission = workflow
            .workflow_execution_strategy
            .as_ref()
            .and_then(|strategy| strategy.resource_admission.clone());
        let model_source_paths = workflow
            .models
            .as_ref()
            .map(|models| {
                models
                    .models
                    .values()
                    .filter_map(|model| {
                        model.source_path.as_ref().and_then(|source_path| {
                            (!model.name.is_empty() && !source_path.is_empty())
                                .then(|| (model.name.clone(), source_path.clone()))
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

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
        // CRITICAL: Skip bootstrap steps (max_tokens < 100) — they use low values intentionally.
        // Using bootstrap's max_tokens as workflow default causes ALL steps to generate ~4 tokens.
        let max_tokens_from_step = first_step
            .get("model_overrides")
            .and_then(|mo| mo.get("max_tokens"))
            .and_then(|v| v.as_u64())
            .filter(|&v| v >= 100)
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

        let model_list: Vec<String> = yaml_value
            .get("models")
            .and_then(|models| models.as_object())
            .map(|arr| {
                arr.iter()
                    .filter_map(|(_, model)| model.get("name").and_then(|v| v.as_str()))
                    .filter(|name| !name.is_empty())
                    .map(String::from)
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

        let inference_max_attempts: u32 = yaml_value
            .get("workflow_execution_strategy")
            .and_then(|wes| wes.get("error_handling"))
            .and_then(|eh| eh.get("retry"))
            .and_then(|r| r.get("max_attempts_per_step"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32)
            .unwrap_or(3);

        let output_root: Option<String> = yaml_value
            .get("workspace")
            .and_then(|w| w.get("directories"))
            .and_then(|d| d.get("output"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let refusal_patterns: Vec<String> = yaml_value
            .get("workflow_execution_strategy")
            .and_then(|wes| wes.get("quality"))
            .and_then(|q| q.get("refusal_patterns"))
            .and_then(|p| p.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let timing = yaml_value
            .get("workflow_execution_strategy")
            .and_then(|wes| wes.get("timing"));

        let cooldown_after_unload_secs: Option<u64> = timing
            .and_then(|t| t.get("cooldown_after_unload_secs"))
            .and_then(|v| v.as_u64());

        let model_load_timeout_secs: Option<u64> = timing
            .and_then(|t| t.get("model_load_timeout_secs"))
            .and_then(|v| v.as_u64());

        let min_tmp_space_mb: Option<u64> = timing
            .and_then(|t| t.get("min_tmp_space_mb"))
            .and_then(|v| v.as_u64());

        let streaming_enabled: Option<bool> = yaml_value
            .get("workflow_execution_strategy")
            .and_then(|wes| wes.get("streaming"))
            .and_then(|s| s.get("enabled"))
            .and_then(|v| v.as_bool());

        let model_selection = yaml_value
            .get("workspace")
            .and_then(|w| w.get("model_selection"));

        let model_selection_strategy: Option<String> = model_selection
            .and_then(|ms| ms.get("strategy"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let diverse_n_count: Option<usize> = model_selection
            .and_then(|ms| ms.get("diverse_n_count"))
            .and_then(|v| v.as_u64())
            .map(|n| n as usize);

        let model_filter: Option<String> = model_selection
            .and_then(|ms| ms.get("model_filter"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let detail_template: Option<String> = yaml_value
            .get("workspace")
            .and_then(|w| w.get("detail_template"))
            .and_then(|v| v.as_str())
            .map(String::from);

        info!("[benchmark] extracted workflow config: prompts={}, max_tokens={}, temperature={}, top_p={}, gpu_layers={}, model_list={}, inference_max_attempts={}, output_root={:?}, refusal_patterns={}, timing={{cooldown:{:?}, load_timeout:{:?}, min_tmp_mb:{:?}}}, streaming={:?}",
            prompts.len(), max_tokens, temperature, top_p, gpu_layers, model_list.len(), inference_max_attempts, output_root, refusal_patterns.len(),
            cooldown_after_unload_secs, model_load_timeout_secs, min_tmp_space_mb, streaming_enabled);

        Ok(Some(BenchmarkWorkflowConfig {
            prompts,
            max_tokens,
            temperature,
            top_p,
            compare_modes: false,
            model_list,
            model_source_paths,
            gpu_layers,
            load_params_env_vars,
            inference_max_attempts,
            output_root,
            refusal_patterns,
            cooldown_after_unload_secs,
            model_load_timeout_secs,
            min_tmp_space_mb,
            streaming_enabled,
            model_selection_strategy,
            diverse_n_count,
            model_filter,
            detail_template,
            resource_admission,
        }))
    }

    fn load_workflow_steps(&self) -> Result<Option<Vec<WorkflowStep>>> {
        let wf_path = match self.config.workflow_file.as_ref() {
            Some(p) => p,
            None => return Ok(None),
        };
        let content = match fs::read_to_string(wf_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "[benchmark] cannot read --workflow file {}: {} — \
                     refusing to fall back to discovery benchmark \
                     (Issue T: steps-loading seam; original: flan-t5 loaded by accident)",
                    wf_path,
                    e
                ));
            }
        };

        let yaml_value: serde_json::Value = match serde_saphyr::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "[benchmark] failed to parse workflow YAML {}: {} — \
                     refusing to fall back to discovery benchmark (Issue T)",
                    wf_path,
                    e
                ));
            }
        };
        let agentic_workflow = match yaml_value.get("agentic_workflow") {
            Some(aw) => aw,
            None => {
                return Err(anyhow::anyhow!(
                    "[benchmark] no agentic_workflow section in {} — \
                     refusing to fall back to discovery benchmark (Issue T)",
                    wf_path
                ));
            }
        };
        let workflow_default_hooks = agentic_workflow.get("when");

        // NEW: look at agentic_workflow.steps, not agentic_workflow itself
        let steps_section = agentic_workflow.get("steps")
            .unwrap_or(agentic_workflow);  // fallback for old format

        let steps: Vec<WorkflowStep> = if let Some(steps_array) = steps_section.as_array() {
            steps_array.iter()
                .enumerate()
                .map(|(i, step)| {
                    let step_name = step.get("step")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .ok_or_else(|| anyhow::anyhow!(
                            "[benchmark] array-format step {} (0-based index {}) in {} is missing the `step:` name key — \
                             refusing to silently drop it (previously filter_map discarded it) — \
                             refusing to fall back to discovery benchmark (Issue T)",
                            i + 1, i, wf_path
                        ))?;
                    let step_id = step.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let (requires, require_conditions) = Self::extract_dependency_names(step);
                    let when = Self::merged_step_hooks(workflow_default_hooks, step);
                    let prompt = step.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let generative_entity = step.get("generative_entity").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let model_overrides = step.get("model_overrides").cloned();
                    let loop_config = step.get("loop").cloned();
                    let retry_max_attempts = step.get("retry")
                        .and_then(|r| r.get("max_attempts"))
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);

                    Ok(WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        require_conditions,
                        retry_max_attempts,
                        when,
                        prompt,
                        generative_entity,
                        model_overrides,
                        r#loop: loop_config,
                    })
                })
                .collect::<Result<Vec<_>>>()?
        } else if let Some(steps_map) = steps_section.as_object() {
            steps_map.iter()
                .map(|(step_name_key, step_value)| {
                    let step_name = step_name_key.clone();
                    let step_id = step_value.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| step_name.clone());
                    let (requires, require_conditions) = Self::extract_dependency_names(step_value);
                    let when = Self::merged_step_hooks(workflow_default_hooks, step_value);
                    let prompt = step_value.get("prompt").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let generative_entity = step_value.get("generative_entity").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let model_overrides = step_value.get("model_overrides").cloned();
                    let loop_config = step_value.get("loop").cloned();
                    let retry_max_attempts = step_value.get("retry")
                        .and_then(|r| r.get("max_attempts"))
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);

                    WorkflowStep {
                        step_name,
                        step_id,
                        requires,
                        require_conditions,
                        retry_max_attempts,
                        when,
                        prompt,
                        generative_entity,
                        model_overrides,
                        r#loop: loop_config,
                    }
                })
                .collect()
        } else {
            return Err(anyhow::anyhow!(
                "[benchmark] agentic_workflow.steps in {} is neither a list nor a map — \
                 refusing to fall back to discovery benchmark (Issue T)",
                wf_path
            ));
        };

        if steps.is_empty() {
            return Err(anyhow::anyhow!(
                "[benchmark] agentic_workflow.steps in {} is empty — \
                 refusing to fall back to discovery benchmark (Issue T)",
                wf_path
            ));
        }

        Ok(Some(steps))
    }

    fn extract_dependency_names(
        step_value: &serde_json::Value,
    ) -> (Vec<String>, std::collections::HashMap<String, String>) {
        let mut dependencies = Vec::new();
        let mut conditions = std::collections::HashMap::new();

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
                        if let Some(condition) =
                            item.get("condition").and_then(|v| v.as_str())
                        {
                            conditions.insert(name.to_string(), condition.to_string());
                        }
                    }
                }
            }
        }

        (dependencies, conditions)
    }

    /// Evaluate a `requires: [{step: X, condition: expr}]` condition against
    /// the upstream step's output. JSON outputs are exposed as `result.<field>`
    /// paths; non-JSON outputs fall back to `result.output` holding the raw
    /// string. Uses the GWT evaluator (same expression language as hooks).
    /// Errors are returned loud (malformed expression), never treated as
    /// satisfied.
    fn eval_require_condition(output: &str, expr: &str) -> Result<bool> {
        let parsed: serde_json::Value = match serde_json::from_str::<serde_json::Value>(output) {
            Ok(v) => serde_json::json!({ "result": v }),
            Err(_) => serde_json::json!({ "result": { "output": output } }),
        };
        crate::workflow::hooks::gwt::evaluate(expr, &parsed)
            .map_err(|e| anyhow::anyhow!("requirement condition {:?} failed to evaluate: {}", expr, e))
    }

    /// Effective inference attempts for a step: per-step `retry.max_attempts`
    /// (schema:273-276) overrides the global `inference_max_attempts`;
    /// result is clamped to >= 1 (a step must always get one shot).
    fn effective_max_attempts(global: u32, step_override: Option<u32>) -> u32 {
        step_override.unwrap_or(global).max(1)
    }

    /// Topologically sort workflow steps by their dependency graph.
    /// `requires` field is populated by `extract_dependency_names` which merges
    /// both YAML `depends_on` and `requires` keys into a single list.
    /// Steps with no dependencies come first; steps depending on them follow.
    /// Uses Kahn's algorithm. Steps in cycles are appended at end (logged as warning).
    fn topological_sort_steps(steps: Vec<WorkflowStep>) -> Vec<WorkflowStep> {
        use std::collections::{HashMap, HashSet, VecDeque};

        let id_to_idx: HashMap<String, usize> = steps.iter()
            .enumerate()
            .map(|(i, s)| (s.step_id.clone(), i))
            .collect();

        let mut in_degree: Vec<usize> = vec![0; steps.len()];
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); steps.len()];

        for (i, step) in steps.iter().enumerate() {
            for dep in &step.requires {
                if let Some(&dep_idx) = id_to_idx.get(dep) {
                    if !adjacency[dep_idx].contains(&i) {
                        adjacency[dep_idx].push(i);
                        in_degree[i] += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<usize> = (0..steps.len())
            .filter(|&i| in_degree[i] == 0)
            .collect();

        let mut result_indices: Vec<usize> = Vec::with_capacity(steps.len());
        while let Some(i) = queue.pop_front() {
            result_indices.push(i);
            for &neighbor in &adjacency[i] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if result_indices.len() == steps.len() {
            result_indices.into_iter().map(|i| steps[i].clone()).collect()
        } else {
            let dropped: HashSet<usize> = result_indices.iter().copied().collect();
            let mut kept: Vec<WorkflowStep> = result_indices.into_iter()
                .map(|i| steps[i].clone())
                .collect();
            for (i, step) in steps.into_iter().enumerate() {
                if !dropped.contains(&i) {
                    warn!("[benchmark] step {} in depends_on cycle, appended at end", step.step_id);
                    kept.push(step);
                }
            }
            kept
        }
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

    /// Resolve a workflow model name to a discovered model file.
    ///
    /// Matching layers (first hit wins):
    /// 1. exact id (with or without `.gguf`)
    /// 2. normalized equality (case-insensitive, `.`/`-`/`_` unified, `.gguf` stripped)
    /// 3. substring containment (legacy behavior)
    /// 4. base-name containment (legacy behavior)
    ///
    /// Layers 2+ exist because YAML names drift from filenames in case and
    /// separator spelling ("Qwen3.5-9B" vs "Qwen3-5-9B.gguf"); before them,
    /// such steps failed with "Model not found" or bound an arbitrary
    /// sibling variant (atomic-reasoning/benchmarks/SUMMARY.md:125,
    /// reasoning-enhancer-plus/docs/07-TRACKING.md:61-63).
    /// Hint text when a model resolved through a fuzzy layer: the engine
    /// proceeds, but raw API calls (shell-hook curl) need the exact name.
    fn model_name_drift_hint(requested: &str, resolved_id: &str) -> Option<String> {
        let exact = requested == resolved_id
            || requested.strip_suffix(".gguf") == Some(resolved_id)
            || resolved_id.strip_suffix(".gguf") == Some(requested)
            || format!("{}.gguf", requested) == resolved_id;
        if exact {
            None
        } else {
            Some(format!(
                "model name drift: workflow says {:?}, server filename is {:?} — engine resolved it, but raw API calls (e.g. shell-hook curl) need the exact filename",
                requested, resolved_id
            ))
        }
    }

    fn resolve_model_file(&self, model_name: &str, discovered_models: &[(String, String)]) -> Option<(String, String)> {
        let want_gguf = format!("{}.gguf", model_name);
        if let Some(hit) = discovered_models
            .iter()
            .find(|(id, _)| *id == model_name || *id == want_gguf)
        {
            return Some(hit.clone());
        }

        let normalize = |s: &str| {
            s.strip_suffix(".gguf")
                .unwrap_or(s)
                .to_lowercase()
                .chars()
                .map(|c| match c { '.' | '-' | '_' => '-', c => c })
                .collect::<String>()
        };
        let want = normalize(model_name);
        if let Some(hit) = discovered_models
            .iter()
            .find(|(id, _)| normalize(id) == want)
        {
            if let Some(hint) = Self::model_name_drift_hint(model_name, &hit.0) {
                warn!("[benchmark] {}", hint);
            }
            return Some(hit.clone());
        }

        for (id, path) in discovered_models {
            if id.contains(model_name) {
                if let Some(hint) = Self::model_name_drift_hint(model_name, id) {
                    warn!("[benchmark] {}", hint);
                }
                return Some((id.clone(), path.clone()));
            }
        }
        let base_name = model_name.split('-').next().unwrap_or(model_name);
        for (id, path) in discovered_models {
            if id.contains(base_name) {
                if let Some(hint) = Self::model_name_drift_hint(model_name, id) {
                    warn!("[benchmark] {}", hint);
                }
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
                refusal_detected: false,
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
                refusal_detected: false,
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
                require_conditions: Default::default(),
                retry_max_attempts: None,
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
                require_conditions: Default::default(),
                retry_max_attempts: None,
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
            let resource_admission = self.resource_admission.clone();
            let resource_admission_telemetry = Arc::clone(&self.resource_admission_telemetry);
            let resource_admission_started_at = Arc::clone(&self.resource_admission_started_at);

            join_set.spawn(async move {
                let runner = BenchmarkRunner {
                    config,
                    hook_engine,
                    inference_semaphore: Arc::new(Semaphore::new(1)),
                    inference_max_attempts: 3,
                    refusal_patterns: DEFAULT_REFUSAL_PATTERS.iter().map(|s| s.to_string()).collect(),
                    streaming_enabled: false,
                    pressure_tracker: Arc::new(Mutex::new(resource_guard::PressureTracker::new())),
                    detail_template: None,
                    resource_admission,
                    resource_admission_telemetry,
                    resource_admission_started_at,
                    resume: false,
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

    fn discover_models(
        &self,
        workflow_model_list: Option<&[String]>,
        workflow_model_source_paths: Option<&BTreeMap<String, String>>,
    ) -> Result<Vec<(String, String)>> {
        let mut models = Vec::new();

        if let (Some(workflow_model_list), Some(workflow_model_source_paths)) =
            (workflow_model_list, workflow_model_source_paths)
        {
            if !workflow_model_source_paths.is_empty() {
                for model_id in workflow_model_list {
                    let source_path = workflow_model_source_paths.get(model_id).ok_or_else(|| {
                        anyhow::anyhow!(
                            "[benchmark] workflow model {} has no source_path; resource admission requires a host-stattable model file",
                            model_id
                        )
                    })?;
                    let source = Path::new(source_path);
                    if !source.is_file() {
                        anyhow::bail!(
                            "[benchmark] workflow model {} source_path is not a regular file: {}",
                            model_id,
                            source.display()
                        );
                    }
                    if source.file_name().and_then(|name| name.to_str()) != Some(model_id.as_str()) {
                        anyhow::bail!(
                            "[benchmark] workflow model {} source_path filename does not match model name: {}",
                            model_id,
                            source.display()
                        );
                    }
                    models.push((model_id.clone(), source_path.clone()));
                }
                return Ok(models);
            }
        }

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
        let prompt: String = require_step_prompt(step)
            .map_err(|e| {
                error!("[benchmark] step {} rejected: {}", step.step_id, e);
                e
            })?;
        let is_control_flow = step.generative_entity.as_deref() == Some("control_flow");
        let prompt = if is_control_flow { String::new() } else { prompt };

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
            prompt_preview: if prompt.chars().count() > 100 { let end = prompt.char_indices().nth(100).map(|(i, _)| i).unwrap_or(100); format!("{}...", &prompt[..end]) } else { prompt.clone() },
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
                        step_id: Some(step.step_id.clone()),
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
                        step_id: Some(step.step_id.clone()),
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
                        step_id: Some(step.step_id.clone()),
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

        // E1 gate: sample live sensors, advance the pressure hysteresis
        // machine, and fail the step cleanly under RED pressure instead
        // of letting inference crash the machine (storybook crash class).
        {
            let vram = resource_guard::read_vram_sensors_sysfs();
            let ram = resource_guard::read_ram_sensors_proc();
            let action = {
                let mut tracker = self.pressure_tracker.lock().unwrap();
                resource_guard::step_gate(&mut tracker, vram, ram)
            };
            match action {
                resource_guard::GateAction::Block(reason) => {
                    warn!("[benchmark] RESOURCE_CRITICAL step {} blocked: {}",
                        step.step_id, reason);
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
                            error: Some(reason),
                            gpu_mode: "gpu".to_string(),
                            speedup_factor: None,
                            step_id: Some(step.step_id.clone()),
                        },
                        route_to: None,
                        skip_remaining: false,
                        skip_loop: false,
                    });
                }
                resource_guard::GateAction::Warn => {
                    warn!("[benchmark] RESOURCE_PRESSURE amber at step {} — proceeding, close to limits",
                        step.step_id);
                }
                resource_guard::GateAction::Proceed => {}
            }
        }

        let resolved_prompt = {
            let bookmarks_map = &self.hook_engine.lock().unwrap().bookmarks;
            let mut resolved = Self::resolve_bookmark_templates(&prompt, bookmarks_map);
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

        let mut model_result = if self.streaming_enabled && Self::has_during_streaming_hook(&step.when) {
            self.run_model_inference_streaming(client, model_id, model_path, &resolved_prompt, step_max_tokens, step_temperature, top_p, system_prompt.as_deref(), &step.when, &step.step_name).await
        } else {
            self.run_model_inference(client, model_id, model_path, "gpu", std::slice::from_ref(&resolved_prompt), step_max_tokens, step_temperature, top_p, system_prompt, true, step.retry_max_attempts).await
        };
        model_result.step_id = Some(step.step_id.clone());

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
            if is_retry_exhausted_error(error) {
                let exhausted_context = WorkflowHookContext::AfterAllRetriesExhausted(
                    AfterAllRetriesExhaustedContext {
                        step_name: step.step_name.clone(),
                        total_attempts: self.inference_max_attempts,
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
                output: output_text.clone(),
                duration_ms: model_result.total_duration.as_millis() as u64,
                quality_score: {
                    let matched_pattern = {
                        let lower = output_text.to_lowercase();
                        self.refusal_patterns.iter().find(|p| lower.contains(&p.to_lowercase())).cloned()
                    };
                    if let Some(pattern) = matched_pattern {
                        warn!("[benchmark] step {} output is a REFUSAL (matched '{}', {} bytes), setting quality_score=0.0", step.step_id, pattern, output_text.len());
                        Some(0.0f32)
                    } else {
                        output_ratio
                    }
                },
                token_count: total_tokens as u32,
                model_name: model_id.to_string(),
                refusal_detected: {
                    let lower = output_text.to_lowercase();
                    self.refusal_patterns.iter().any(|p| lower.contains(&p.to_lowercase()))
                },
            });

            match self.execute_hooks_for_trigger(&step.when, "after_step_succeeds", &after_context) {
                Ok(HookResult::Fail { reason }) => {
                    warn!("[benchmark] after_step_succeeds hook failed: {}", reason);
                    model_result.error = Some(format!("after_step_succeeds hook failed: {}", reason));
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

    /// Iteration cap for the linear workflow loop: guards against
    /// route_to/loop cycles without capping total steps in large
    /// linear workflows. Floor 100, loop overrides win, otherwise
    /// scale with step count (4x) so every step can execute once
    /// plus bounded re-visits.
    fn max_workflow_iterations(steps: &[WorkflowStep]) -> usize {
        let loop_override = steps.iter()
            .filter_map(|s| s.r#loop.as_ref())
            .filter_map(|l| l.get("count"))
            .filter_map(|c| c.get("max_iterations"))
            .filter_map(|m| m.as_u64())
            .max()
            .unwrap_or(100);
        (loop_override.max(100).max(steps.len() as u64 * 4)) as usize
    }

    /// Error message when the workflow loop stopped because the iteration
    /// cap was hit while steps remain unexecuted. Returns `None` when the
    /// workflow completed (cap not reached, or reached exactly as the last
    /// step finished) — those are successes, not failures.
    /// Regression (Issue E): cap exhaustion previously warned and returned
    /// Ok, so abandoned workflows exited 0 (SUMMARY-benchmark.md:69-71).
    fn premature_cap_error(
        loop_count: usize,
        max_loop_iterations: usize,
        current_index: usize,
        steps_len: usize,
    ) -> Option<String> {
        if loop_count >= max_loop_iterations && current_index < steps_len {
            Some(format!(
                "workflow loop exceeded {} iterations with {} step(s) unexecuted — stopping and failing instead of exiting 0 (silent-exit-0 fix)",
                max_loop_iterations,
                steps_len - current_index
            ))
        } else {
            None
        }
    }

    /// Error message when topological sorting dropped steps because the
    /// depends_on graph contains a cycle. Dropped steps would silently
    /// never execute.
    fn cycle_error(sorted_len: usize, original_len: usize) -> Option<String> {
        if sorted_len < original_len {
            Some(format!(
                "cycle detected in depends_on graph: {} of {} steps dropped by topological sort — refusing to run a workflow that silently skips steps",
                original_len - sorted_len,
                original_len
            ))
        } else {
            None
        }
    }

    /// Issue Y: read a checkpoint.jsonl written by a previous (killed or
    /// crashed) run into a step_id → output map. Malformed lines are
    /// skipped; when a step id appears more than once, the LAST entry wins
    /// (it is the most recent output). A missing file is an empty map, not
    /// an error — nothing was checkpointed.
    fn load_checkpoint(path: &Path) -> std::collections::HashMap<String, String> {
        #[derive(serde::Deserialize)]
        struct CheckpointEntry {
            step_id: String,
            output: String,
        }
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return std::collections::HashMap::new(),
        };
        let mut map = std::collections::HashMap::new();
        for line in contents.lines() {
            if let Ok(entry) = serde_json::from_str::<CheckpointEntry>(line) {
                map.insert(entry.step_id, entry.output);
            }
        }
        map
    }

    /// Issue Y: record a completed step's output in-memory and append it to
    /// `<output_dir>/checkpoint.jsonl` so a watchdog kill no longer loses
    /// all completed work (REVIEW-CYCLES.md OC-4). Checkpoint IO failures
    /// are logged, never fatal — the run itself continues.
    fn record_step_output(
        &self,
        step_outputs: &mut std::collections::HashMap<String, String>,
        step_id: &str,
        output: &str,
    ) {
        step_outputs.insert(step_id.to_string(), output.to_string());
        let Some(ref output_dir) = self.config.output_dir else {
            return;
        };
        let checkpoint = Path::new(output_dir).join("checkpoint.jsonl");
        let line = serde_json::json!({ "step_id": step_id, "output": output });
        let append = || -> std::io::Result<()> {
            if let Some(parent) = checkpoint.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::OpenOptions::new().create(true).append(true).open(&checkpoint)?;
            use std::io::Write;
            writeln!(file, "{}", line)
        };
        if let Err(e) = append() {
            warn!("[benchmark] checkpoint append failed for step {}: {}", step_id, e);
        }
    }

    /// Apply scalar/timing overrides from the workflow YAML to the runner
    /// config. MUST be called before `preflight_check()` so gates like the
    /// disk-space minimum consume the YAML value, not just the CLI value
    /// (Issue D: `timing.min_tmp_space_mb` used to be applied only after
    /// preflight had already run — 07-TRACKING.md:284-286).
    fn apply_workflow_timing_overrides(&mut self, ctx: &BenchmarkWorkflowConfig) {
        self.resource_admission = ctx.resource_admission.clone();
        if ctx.inference_max_attempts != self.inference_max_attempts {
            info!("[benchmark] workflow YAML overrides inference_max_attempts: {} → {}", self.inference_max_attempts, ctx.inference_max_attempts);
            self.inference_max_attempts = ctx.inference_max_attempts;
        }
        if !ctx.refusal_patterns.is_empty() {
            info!("[benchmark] workflow YAML overrides refusal_patterns: {} → {} patterns", self.refusal_patterns.len(), ctx.refusal_patterns.len());
            self.refusal_patterns = ctx.refusal_patterns.clone();
        }
        if self.config.output_dir.is_none() {
            if let Some(ref yaml_out) = ctx.output_root {
                info!("[benchmark] workflow YAML provides output_root: {}", yaml_out);
                self.config.output_dir = Some(yaml_out.clone());
            }
        }
        if let Some(secs) = ctx.cooldown_after_unload_secs {
            info!("[benchmark] workflow YAML overrides cooldown_after_unload: {:?} → {}s", self.config.cooldown_after_unload, secs);
            self.config.cooldown_after_unload = Duration::from_secs(secs);
        }
        if let Some(secs) = ctx.model_load_timeout_secs {
            info!("[benchmark] workflow YAML overrides model_load_timeout: {:?} → {}s", self.config.model_load_timeout, secs);
            self.config.model_load_timeout = Duration::from_secs(secs);
        }
        if let Some(mb) = ctx.min_tmp_space_mb {
            info!("[benchmark] workflow YAML overrides min_tmp_space_mb: {} → {}", self.config.min_tmp_space_mb, mb);
            self.config.min_tmp_space_mb = mb;
        }
        if let Some(enabled) = ctx.streaming_enabled {
            if enabled != self.streaming_enabled {
                info!("[benchmark] workflow YAML overrides streaming_enabled: {} → {}", self.streaming_enabled, enabled);
                self.streaming_enabled = enabled;
            }
        }
        if let Some(ref tmpl) = ctx.detail_template {
            if self.detail_template.as_ref() != Some(tmpl) {
                info!("[benchmark] workflow YAML overrides detail_template: {} chars", tmpl.len());
                self.detail_template = Some(tmpl.clone());
            }
        }
        if let Some(ref filter) = ctx.model_filter {
            if self.config.filter_name.as_ref() != Some(filter) {
                info!("[benchmark] workflow YAML overrides filter_name: {:?} → {}", self.config.filter_name, filter);
                self.config.filter_name = Some(filter.clone());
            }
        }
    }

    /// Absolutize the output dir against the process CWD. Relative
    /// `--out-dir` / YAML `output_root` values previously leaked into hook
    /// paths (WHITT_OUTPUT_DIR, save_to base) where they resolved against
    /// whichever CWD the hook process happened to have — breaking shell
    /// hooks silently (Issue G, 07-TRACKING.md:109).
    fn normalized_output_dir(dir: &str) -> std::path::PathBuf {
        let p = std::path::PathBuf::from(dir);
        if p.is_relative() {
            std::path::absolute(&p).unwrap_or(p)
        } else {
            p
        }
    }

    pub async fn run(&mut self) -> Result<BenchmarkSuiteResult> {
        // Workflow config loads and timing overrides apply BEFORE preflight
        // so preflight gates (e.g. min disk space) honor YAML values
        // (Issue D: timing.min_tmp_space_mb previously unreachable).
        let wf_ctx = match self.load_workflow_config() {
            Ok(Some(ctx)) => Some(ctx),
            Ok(None) => None,
            Err(e) => {
                return Err(e);
            }
        };

        if let Some(ref ctx) = wf_ctx {
            self.apply_workflow_timing_overrides(ctx);
        }

        if let Some(dir) = self.config.output_dir.clone() {
            let abs = Self::normalized_output_dir(&dir);
            if abs.to_string_lossy() != dir {
                info!("[benchmark] --out-dir was relative: {} → absolutized to {} (hook-path stability fix)", dir, abs.display());
                self.config.output_dir = Some(abs.to_string_lossy().to_string());
            }
        }

        self.preflight_workflow_resource_admission()?;
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
                planned_steps: None,
                results: vec![],
            });
        }

        self.ensure_output_dirs()
            .context("Failed to ensure output directories")?;

        if let Some(ref output_dir) = self.config.output_dir {
            self.hook_engine.lock().unwrap().output_dir = Some(std::path::PathBuf::from(output_dir));
            info!("[benchmark] hook engine output_dir set to {}", output_dir);
        }

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

        let mut models = self.discover_models(
            wf_ctx.as_ref().map(|ctx| ctx.model_list.as_slice()),
            wf_ctx.as_ref().map(|ctx| &ctx.model_source_paths),
        )
            .context("Failed to discover models")?;

        if let Some(ref ctx) = wf_ctx {
            if ctx.model_selection_strategy.as_deref() == Some("diverse_n") {
                if let Some(n) = ctx.diverse_n_count {
                    if n > 0 && models.len() > n {
                        info!("[benchmark] applying model_selection_strategy=diverse_n count={} (was {} candidates)",
                            n, models.len());
                        let candidates: Vec<ModelCandidate> = models.iter().filter_map(|(id, path)| {
                            let p = std::path::Path::new(path);
                            let size = std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
                            let author = p.parent()
                                .and_then(|p| p.file_name())
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            Some(ModelCandidate {
                                path: p.to_path_buf(),
                                model_id: id.clone(),
                                file_size_bytes: size,
                                estimated_vram_bytes: size,
                                fits_in_vram: true,
                                author,
                            })
                        }).collect();
                        let selected = ModelSelector::select_n_models(&candidates, n);
                        let new_models: Vec<(String, String)> = selected.into_iter()
                            .map(|c| (c.model_id, c.path.to_string_lossy().to_string()))
                            .collect();
                        info!("[benchmark] diverse_n selected {} models", new_models.len());
                        models = new_models;
                    }
                }
            }
        }

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

        // Issue Y: pre-populate from a previous run's checkpoint so a
        // watchdog kill no longer discards completed work
        // (REVIEW-CYCLES.md OC-4). resumed_steps is a snapshot — runtime
        // revisits of the same step are NOT skipped, only the steps whose
        // outputs came from the checkpoint file.
        let mut resumed_steps: std::collections::HashSet<String> = std::collections::HashSet::new();
        if self.resume {
            if let Some(ref output_dir) = self.config.output_dir {
                let checkpoint = Path::new(output_dir).join("checkpoint.jsonl");
                let restored = Self::load_checkpoint(&checkpoint);
                if !restored.is_empty() {
                    info!(
                        "[benchmark] resume: restored {} completed step output(s) from {}",
                        restored.len(),
                        checkpoint.display()
                    );
                    resumed_steps = restored.keys().cloned().collect();
                    step_outputs = restored;
                }
            }
        }

        let suite_metadata = format!(
            "run_timestamp: {}\nserver_url: {}\ntotal_models: {}\ncompare_gpu_cpu: {}",
            run_timestamp, self.config.server_url, models.len(), compare_gpu_cpu
        );

        let workflow_steps = self.load_workflow_steps()?;
        let yaml_models = self.load_workflow_models();

        if let Some(steps) = &workflow_steps {
            info!("[benchmark] loaded {} workflow steps from YAML", steps.len());

            // Topological sort by depends_on: ensures upstream steps run before downstream.
            // Without this, runner iterates in YAML order which may not match dep chain.
            // Bug symptom: step_t1_1 skipped because step_t1 hadn't run yet.
            let sorted_steps = Self::topological_sort_steps(steps.clone());
            if let Some(msg) = Self::cycle_error(sorted_steps.len(), steps.len()) {
                error!("[benchmark] {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
            }
            let steps: &[WorkflowStep] = &sorted_steps;

            let step_index: std::collections::HashMap<String, usize> = steps.iter()
                .enumerate()
                .map(|(i, s)| (s.step_id.clone(), i))
                .collect();

            let max_loop_iterations = Self::max_workflow_iterations(&steps);
            let mut current_index: usize = 0;
            let mut loop_count: usize = 0;

            while current_index < steps.len() && loop_count < max_loop_iterations {
                let step = &steps[current_index];
                loop_count += 1;

                if !step.requires.is_empty() {
                    let mut missing_deps: Vec<String> = Vec::new();
                    let mut unsatisfied_conditions: Vec<String> = Vec::new();

                    for dep in &step.requires {
                        let Some(output) = step_outputs.get(dep) else {
                            missing_deps.push(dep.clone());
                            continue;
                        };
                        if let Some(condition) = step.require_conditions.get(dep) {
                            match Self::eval_require_condition(output, condition) {
                                Ok(true) => {}
                                Ok(false) => unsatisfied_conditions.push(format!(
                                    "{} (condition {:?} not met)",
                                    dep, condition
                                )),
                                // Loud failure: a malformed condition must never
                                // silently pass the dependency check.
                                Err(e) => anyhow::bail!(e),
                            }
                        }
                    }

                    if !missing_deps.is_empty() || !unsatisfied_conditions.is_empty() {
                        let mut all_missing = missing_deps.clone();
                        all_missing.extend(unsatisfied_conditions.iter().cloned());
                        warn!("[benchmark] step {} skipped: missing/unsatisfied dependencies {:?}", step.step_id, all_missing);
                        let requires_context = WorkflowHookContext::OnRequiresFailed(
                            OnRequiresFailedContext {
                                failed_step: step.step_id.clone(),
                                reason: format!("Dependencies not satisfied: {:?}", all_missing),
                                dependency_chain: all_missing.clone(),
                            }
                        );
                        match self.execute_hooks_for_trigger(&step.when, "on_requires_failed", &requires_context) {
                            Ok(_) => {}
                            Err(e) => {
                                error!("[benchmark] on_requires_failed hook error: {}", e);
                            }
                        }
                        current_index += 1;
                        continue;
                    }
                }

                if self.resume && resumed_steps.contains(&step.step_id) {
                    info!("[benchmark] resumed: skipping completed step {}", step.step_id);
                    current_index += 1;
                    continue;
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
                                    require_conditions: Default::default(),
                                    retry_max_attempts: None,
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
                                        match self.execute_hooks_for_trigger(&step.when, "after_loop_iteration_fails", &fail_context) {
                                            Ok(_) => {}
                                            Err(e) => {
                                                error!("[benchmark] after_loop_iteration_fails hook error: {}", e);
                                            }
                                        }
                                    }
                                    let output_text = last_result.inference_results.first().map(|inf| inf.response_text.clone()).unwrap_or_default();
                                    self.record_step_output(&mut step_outputs, &step.step_id, &output_text);
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
                                                require_conditions: Default::default(),
                                                retry_max_attempts: None,
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
                                                         self.record_step_output(&mut step_outputs, &target_step.step_id, &output_text);
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
                            // No step-start log here: execute_workflow_step
                            // owns the single "executing step" emission
                            // (Issue O duplicate-log regression).

                            // Bookmark resolution deferred — hooks must fire first.
                            let resolved_prompt = step.prompt.as_ref()
                                .map(|p| {
                                    Self::resolve_step_output_templates(p, &step_outputs)
                                });

                             let resolved_step = WorkflowStep {
                                 step_name: step.step_name.clone(),
                                 step_id: step.step_id.clone(),
                                 requires: step.requires.clone(),
                                 require_conditions: Default::default(),
                                 retry_max_attempts: None,
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
                                  self.record_step_output(&mut step_outputs, &step.step_id, &output_text);
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
                                                        require_conditions: Default::default(),
                                                        retry_max_attempts: None,
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
                                                        self.record_step_output(&mut step_outputs, &target_step.step_id, &output_text);
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
                                                            require_conditions: Default::default(),
                                                            retry_max_attempts: None,
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
                                                            self.record_step_output(&mut step_outputs, &target_step.step_id, &output_text);
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
                                require_conditions: Default::default(),
                                retry_max_attempts: None,
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
                                  self.record_step_output(&mut step_outputs, &step.step_id, &output_text);
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
                                                        require_conditions: Default::default(),
                                                        retry_max_attempts: None,
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
                                                        self.record_step_output(&mut step_outputs, &target_step.step_id, &output_text);
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

            if let Some(msg) = Self::premature_cap_error(
                loop_count,
                max_loop_iterations,
                current_index,
                steps.len(),
            ) {
                error!("[benchmark] {}", msg);
                anyhow::bail!(crate::error::Error::benchmark(msg));
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
                        step_id: None,
                    };
                    self.log_step_result(&failed_result)?;
                    self.log_step_error(model_id, failed_result.error.as_deref().unwrap_or("unknown error"))?;

                    if let Some(ref output_dir) = self.config.output_dir {
                        let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                            step_id: None,
                        };
                        self.log_step_result(&cpu_result)?;
                        self.log_step_error(model_id, cpu_result.error.as_deref().unwrap_or("unknown error"))?;

                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                            let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                            let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                            let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
                            if let Err(e) = detail_gen.write_model_detail(&gpu_result) {
                                warn!("[detail] Failed to generate detail.md for {} (GPU, CPU failed): {}", model_id, e);
                            }
                        }

                        self.write_per_model_report(&cpu_result, &suite_metadata, wf_ctx.as_ref())?;
                        self.append_chat_log_markdown(&cpu_result, &run_timestamp)?;

                        // Generate detail.md for CPU mode (even though it failed)
                        if let Some(ref output_dir) = self.config.output_dir {
                            let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                        let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                        step_id: None,
                    };
                    self.log_step_result(&failed_result)?;
                    self.log_step_error(model_id, failed_result.error.as_deref().unwrap_or("unknown error"))?;

                    if let Some(ref output_dir) = self.config.output_dir {
                        let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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
                    let detail_gen = DetailGenerator::new(output_dir, self.detail_template.clone());
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

        let suite_result = BenchmarkRunner::suite_result_from(
            results,
            workflow_steps.as_ref().map(|s| s.len()),
            &self.config.server_url,
        );

        self.write_final_report(&suite_result)?;

        Ok(suite_result)
    }

    /// Assemble the final suite result. `planned_steps` states how many
    /// steps the workflow declared (Issue J: under early exit,
    /// total_models alone invited "3/5 ran" miscounts because
    /// unexecuted steps simply never appeared in results).
    fn suite_result_from(
        results: Vec<ModelBenchmarkResult>,
        planned_steps: Option<usize>,
        server_url: &str,
    ) -> BenchmarkSuiteResult {
        let successful = results.iter().filter(|r| r.error.is_none()).count();
        let failed = results.len() - successful;
        let timestamp = {
            let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            format!("unix_epoch_{}s", d.as_secs())
        };
        BenchmarkSuiteResult {
            timestamp,
            server_url: server_url.to_string(),
            total_models: results.len(),
            successful,
            failed,
            planned_steps,
            results,
        }
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
        pressure_tracker: &Arc<Mutex<resource_guard::PressureTracker>>,
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
        let inf_start = Instant::now();
        let outcome = match resource_guard::guard_inference(
            pressure_tracker.clone(),
            750,
            || (resource_guard::read_vram_sensors_sysfs(), resource_guard::read_ram_sensors_proc()),
            client.chat_completion(request),
        )
        .await
        {
            Ok(inner) => inner,
            Err(reason) => Err(anyhow::anyhow!("{}", reason)),
        };
        match outcome {
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

    fn resource_admission_allows_parallel(
        resource_admission: Option<&ResourceAdmissionConfig>,
    ) -> bool {
        resource_admission.is_none()
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
        if !Self::resource_admission_allows_parallel(self.resource_admission.as_ref()) {
            info!("[benchmark] route_to resource contract requires sequential execution");
            return None;
        }

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

        if !already_loaded && Self::requires_default_load_admission(self.resource_admission.as_ref()) {
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
            let tracker = self.pressure_tracker.clone();

            join_set.spawn(async move {
                let result = BenchmarkRunner::send_inference_request(
                    &client_clone, &model_id, &prompt_text,
                    step_max_tokens, step_temperature, top_p, None, &sem, &tracker
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

            self.record_step_output(step_outputs, &step.step_id, &output_text);
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

    fn has_during_streaming_hook(step_when: &Option<serde_json::Value>) -> bool {
        step_when.as_ref()
            .and_then(|w| w.get("during_step_streaming"))
            .is_some()
    }

    fn fire_during_streaming_hook(
        &self,
        step_when: &Option<serde_json::Value>,
        step_name: &str,
        _model_name: &str,
        chunk_text: &str,
        chunks_received: u32,
    ) {
        let ctx = WorkflowHookContext::DuringStepStreaming(DuringStepStreamingContext {
            step_name: step_name.to_string(),
            chunk_text: chunk_text.to_string(),
            tokens_so_far: chunks_received,
            elapsed_ms: 0,
        });
        if let Err(e) = self.execute_hooks_for_trigger(step_when, "during_step_streaming", &ctx) {
            warn!("[benchmark] during_step_streaming hook error: {}", e);
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn run_model_inference_streaming(
        &self,
        client: &LlamaHttpClient,
        model_id: &str,
        model_source_path: &str,
        prompt: &str,
        max_tokens: usize,
        temperature: f64,
        top_p: f64,
        system_prompt: Option<&str>,
        step_when: &Option<serde_json::Value>,
        step_name: &str,
    ) -> ModelBenchmarkResult {
        use futures::StreamExt;

        let inf_start = std::time::Instant::now();
        let weights_bytes = std::fs::metadata(model_source_path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);
        let workflow_admission = match self.current_workflow_inference_resource_admission(model_id, weights_bytes) {
            Ok(admission) => admission,
            Err(error) => Some(resource_guard::Admission::Rejected {
                code: "RESOURCE_REJECT_WORKFLOW_CONFIG",
                detail: error.to_string(),
            }),
        };
        if let Some(resource_guard::Admission::Rejected { code, detail }) = workflow_admission {
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: model_source_path.to_string(),
                file_size_bytes: weights_bytes,
                load_duration: Duration::ZERO,
                inference_results: vec![],
                unload_duration: Duration::ZERO,
                total_duration: inf_start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("{}: {}", code, detail)),
                gpu_mode: "gpu".to_string(),
                speedup_factor: None,
                step_id: None,
            };
        }
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
            stream: true,
            ..Default::default()
        };

        match client.chat_completion_stream(request).await {
            Ok(mut stream) => {
                let mut aggregated = String::new();
                let mut chunks_received: u32 = 0;
                while let Some(chunk_result) = stream.next().await {
                    match chunk_result {
                        Ok(chunk) => {
                            chunks_received += 1;
                            if let Some(delta) = chunk.choices.first().and_then(|c| c.delta.content.as_deref()) {
                                aggregated.push_str(&delta);
                                self.fire_during_streaming_hook(step_when, step_name, model_id, &delta, chunks_received);
                            }
                        }
                        Err(e) => {
                            warn!("[benchmark] stream chunk error: {}", e);
                        }
                    }
                }
                let cleaned = Self::clean_response_text(&aggregated);
                let tokens = cleaned.split_whitespace().count() as usize;
                let elapsed = inf_start.elapsed();
                let tps = if elapsed.as_secs_f64() > 0.0 { tokens as f64 / elapsed.as_secs_f64() } else { 0.0 };
                ModelBenchmarkResult {
                    model_id: model_id.to_string(),
                    model_path: String::new(),
                    file_size_bytes: 0,
                    load_duration: std::time::Duration::from_secs(0),
                    inference_results: vec![InferenceResult {
                        prompt: prompt.to_string(),
                        prompt_tokens: 0,
                        completion_tokens: tokens,
                        total_tokens: tokens,
                        duration: elapsed,
                        tokens_per_second: tps,
                        response_text: cleaned,
                    }],
                    unload_duration: std::time::Duration::from_secs(0),
                    total_duration: elapsed,
                    tokens_per_second: tps,
                    avg_latency_ms: 0.0,
                    p50_latency_ms: 0.0,
                    p95_latency_ms: 0.0,
                    p99_latency_ms: 0.0,
                    error: None,
                    gpu_mode: "gpu".to_string(),
                    speedup_factor: None,
                    step_id: None,
                }
            }
            Err(e) => {
                ModelBenchmarkResult {
                    model_id: model_id.to_string(),
                    model_path: String::new(),
                    file_size_bytes: 0,
                    load_duration: std::time::Duration::from_secs(0),
                    inference_results: vec![],
                    unload_duration: std::time::Duration::from_secs(0),
                    total_duration: inf_start.elapsed(),
                    tokens_per_second: 0.0,
                    avg_latency_ms: 0.0,
                    p50_latency_ms: 0.0,
                    p95_latency_ms: 0.0,
                    p99_latency_ms: 0.0,
                    error: Some(format!("Streaming failed: {}", e)),
                    gpu_mode: "gpu".to_string(),
                    speedup_factor: None,
                    step_id: None,
                }
            }
        }
    }

    fn model_source_metadata(
        models_dir: Option<&str>,
        model_id: &str,
        model_source_path: &str,
    ) -> (String, u64) {
        let explicit_path = Path::new(model_source_path);
        if explicit_path.is_file() {
            let weights_bytes = std::fs::metadata(explicit_path)
                .map(|metadata| metadata.len())
                .unwrap_or(0);
            return (explicit_path.display().to_string(), weights_bytes);
        }

        if let Some(models_dir) = models_dir {
            let configured_path = Path::new(models_dir).join(model_id);
            if configured_path.is_file() {
                let weights_bytes = std::fs::metadata(&configured_path)
                    .map(|metadata| metadata.len())
                    .unwrap_or(0);
                return (configured_path.display().to_string(), weights_bytes);
            }
        }

        (model_source_path.to_string(), 0)
    }

    async fn prepare_serial_model_load(client: &LlamaHttpClient, target_model_id: &str) -> bool {
        let Ok(models) = client.list_models().await else {
            return false;
        };
        let already_loaded = models
            .iter()
            .any(|model| model.id == target_model_id && model.status.value == "loaded");
        if already_loaded {
            return true;
        }
        for model in models {
            if model.status.value == "loaded" && model.id != target_model_id {
                info!("[benchmark] unloading {} to make room for {}", model.id, target_model_id);
                if let Err(error) = client.unload_model(&model.id).await {
                    warn!("[benchmark] failed to unload model {}: {}", model.id, error);
                }
            }
        }
        false
    }

    async fn prepare_serial_model_load_then_admit<T>(
        client: &LlamaHttpClient,
        target_model_id: &str,
        admit: impl FnOnce() -> T,
    ) -> (bool, T) {
        let already_loaded = Self::prepare_serial_model_load(client, target_model_id).await;
        (already_loaded, admit())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn run_model_inference(&self, client: &LlamaHttpClient, model_id: &str, model_source_path: &str, gpu_mode: &str, prompts: &[String], max_tokens: usize, temperature: f64, top_p: f64, system_prompt: Option<String>, skip_unload: bool, step_max_attempts: Option<u32>) -> ModelBenchmarkResult {
        let server_model_id = model_id.strip_suffix(".gguf").unwrap_or(model_id);
        let start = Instant::now();

        let (resolved_path, file_size) = Self::model_source_metadata(
            self.config.models_dir.as_deref(),
            model_id,
            model_source_path,
        );

        let (already_loaded, workflow_admission) = Self::prepare_serial_model_load_then_admit(
            client,
            server_model_id,
            || match self.current_workflow_preflight_resource_admission(server_model_id, file_size) {
                Ok(admission) => admission,
                Err(error) => Some(resource_guard::Admission::Rejected {
                    code: "RESOURCE_REJECT_WORKFLOW_CONFIG",
                    detail: error.to_string(),
                }),
            },
        )
        .await;
        if let Some(resource_guard::Admission::Rejected { code, detail }) = workflow_admission {
            warn!("[benchmark] {} {}: workflow resource admission refused {}", code, detail, server_model_id);
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path.clone(),
                file_size_bytes: file_size,
                load_duration: Duration::ZERO,
                inference_results: vec![],
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("{}: {}", code, detail)),
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
                step_id: None,
            };
        }

        let workflow_admission = match self.current_workflow_model_state_resource_admission(
            server_model_id,
            file_size,
            already_loaded,
        ) {
            Ok(admission) => admission,
            Err(error) => Some(resource_guard::Admission::Rejected {
                code: "RESOURCE_REJECT_WORKFLOW_CONFIG",
                detail: error.to_string(),
            }),
        };
        if let Some(resource_guard::Admission::Rejected { code, detail }) = workflow_admission {
            warn!("[benchmark] {} {}: workflow resource admission refused {}", code, detail, server_model_id);
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path.clone(),
                file_size_bytes: file_size,
                load_duration: Duration::ZERO,
                inference_results: vec![],
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("{}: {}", code, detail)),
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
                step_id: None,
            };
        }

        if !already_loaded {
            // L1 admission: refuse the load itself when weights+KV+compute+reserve
            // cannot fit current free VRAM/RAM (external consumers counted via free).
            if Self::requires_default_load_admission(self.resource_admission.as_ref()) {
                if let (Some(vram), Some(ram)) = (
                    resource_guard::read_vram_sensors_sysfs(),
                    resource_guard::read_ram_sensors_proc(),
                ) {
                    if let resource_guard::Admission::Rejected { code, detail } =
                        resource_guard::admission_for_load(server_model_id, file_size, vram.free_bytes, &ram)
                    {
                        warn!("[benchmark] {} {}: load refused for {}", code, detail, server_model_id);
                        return ModelBenchmarkResult {
                            model_id: model_id.to_string(),
                            model_path: resolved_path.clone(),
                            file_size_bytes: file_size,
                            load_duration: Duration::ZERO,
                            inference_results: vec![],
                            unload_duration: Duration::ZERO,
                            total_duration: start.elapsed(),
                            tokens_per_second: 0.0,
                            avg_latency_ms: 0.0,
                            p50_latency_ms: 0.0,
                            p95_latency_ms: 0.0,
                            p99_latency_ms: 0.0,
                            error: Some(format!("{}: {}", code, detail)),
                            gpu_mode: gpu_mode.to_string(),
                            speedup_factor: None,
                            step_id: None,
                        };
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
                    step_id: None,
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
                step_id: None,
            };
        }

        let mut inference_results = Vec::new();
        let mut any_retry_exhausted: Option<String> = None;

        for prompt in prompts {
            let max_retries: u32 = Self::effective_max_attempts(self.inference_max_attempts, step_max_attempts);
            let mut last_error = None;

            for attempt in 0..max_retries {
                match self.current_workflow_inference_resource_admission(server_model_id, file_size) {
                    Ok(Some(resource_guard::Admission::Rejected { code, detail })) => {
                        last_error = Some(format!("{}: {}", code, detail));
                        break;
                    }
                    Ok(_) => {}
                    Err(error) => {
                        last_error = Some(format!("RESOURCE_REJECT_WORKFLOW_CONFIG: {}", error));
                        break;
                    }
                }
                let mut messages = Vec::with_capacity(2);
                if let Some(ref sys) = system_prompt {
                    messages.push(ChatMessage::system(sys.clone()));
                }

                let effective_prompt = if attempt == max_retries - 1 && prompt.len() > 4000 {
                    let truncated = &prompt[prompt.len() - 3000..];
                    warn!("[benchmark] smart retry: truncating prompt to last 3000 chars (was {} chars)", prompt.len());
                    truncated.to_string()
                } else {
                    prompt.clone()
                };

                let effective_temp = if attempt == max_retries - 1 {
                    (temperature + 0.3).min(1.0)
                } else {
                    temperature
                };

                messages.push(ChatMessage::user(effective_prompt));

                let request = ChatCompletionRequest {
                    model: server_model_id.to_string(),
                    messages,
                    max_tokens: Some(max_tokens),
                    temperature: Some(effective_temp as f32),
                    top_p: Some(top_p as f32),
                    stream: false,
                    ..Default::default()
                };

                let inf_start = Instant::now();
                let guarded = resource_guard::guard_inference(
                    self.pressure_tracker.clone(),
                    750,
                    || (resource_guard::read_vram_sensors_sysfs(), resource_guard::read_ram_sensors_proc()),
                    client.chat_completion(request),
                );
                let outcome = match guarded.await {
                    Ok(inner) => inner,
                    Err(reason) => Err(anyhow::anyhow!("{}", reason)),
                };
                match outcome {
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
                        let err_str = e.to_string();
                        warn!("[benchmark] inference attempt {}/{} failed for {}: {}",
                            attempt + 1, max_retries, model_id, e);

                        if err_str.contains(resource_guard::RESOURCE_CRITICAL) {
                            warn!("[benchmark] RESOURCE_CRITICAL: in-flight inference cancelled for {} — not retrying under pressure", model_id);
                            break;
                        }

                        if err_str.contains("500") || err_str.contains("Could not establish") || err_str.contains("connection refused") {
                            warn!("[benchmark] Docker error detected, checking health...");
                            if let Ok(health_client) = LlamaHttpClient::new(&self.config.server_url) {
                                match health_client.health().await {
                                    Ok(h) => {
                                        info!("[benchmark] Docker health: status={}, idle={}, processing={}",
                                            h.status, h.slots_idle, h.slots_processing);
                                    }
                                    Err(he) => {
                                        warn!("[benchmark] Docker health FAILED: {}. Waiting 30s...", he);
                                        sleep(Duration::from_secs(30)).await;
                                    }
                                }
                            }
                        }

                        if attempt + 1 < max_retries {
                            sleep(Duration::from_secs(2u64.pow(attempt))).await;
                        }
                    }
                }
            }

            if let Some(err) = last_error {
                warn!("[benchmark] all {} inference attempts failed for {}: {}", max_retries, model_id, err);
                any_retry_exhausted = Some(format!("All {} attempts failed: {}", max_retries, err));
                inference_results.push(InferenceResult {
                    prompt: prompt.clone(),
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    total_tokens: 0,
                    duration: Duration::ZERO,
                    tokens_per_second: 0.0,
                    response_text: format!("ERROR: All {} attempts failed: {}", max_retries, err),
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
            step_id: None,
        }
    }

    #[deprecated(since = "0.4.0", note = "Use workflow-driven benchmark via --workflow flag")]
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    async fn benchmark_single_model(&mut self, client: &LlamaHttpClient, model_id: &str, model_source_path: &str, gpu_mode: &str, prompts: &[String], max_tokens: usize, temperature: f64, top_p: f64, system_prompt: Option<String>, hook_config: &Option<serde_json::Value>) -> ModelBenchmarkResult {
        let server_model_id = model_id.strip_suffix(".gguf").unwrap_or(model_id);
        let start = Instant::now();

        let (resolved_path, file_size) = Self::model_source_metadata(
            self.config.models_dir.as_deref(),
            model_id,
            model_source_path,
        );

        let workflow_admission = match self.current_workflow_preflight_resource_admission(server_model_id, file_size) {
            Ok(admission) => admission,
            Err(error) => Some(resource_guard::Admission::Rejected {
                code: "RESOURCE_REJECT_WORKFLOW_CONFIG",
                detail: error.to_string(),
            }),
        };
        if let Some(resource_guard::Admission::Rejected { code, detail }) = workflow_admission {
            warn!("[benchmark] {} {}: workflow resource admission refused {}", code, detail, server_model_id);
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path.clone(),
                file_size_bytes: file_size,
                load_duration: Duration::ZERO,
                inference_results: vec![],
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("{}: {}", code, detail)),
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
                step_id: None,
            };
        }

        let already_loaded = if let Ok(models) = client.list_models().await {
            models.iter().any(|m| m.id == server_model_id && m.status.value == "loaded")
        } else {
            false
        };

        let workflow_admission = match self.current_workflow_model_state_resource_admission(
            server_model_id,
            file_size,
            already_loaded,
        ) {
            Ok(admission) => admission,
            Err(error) => Some(resource_guard::Admission::Rejected {
                code: "RESOURCE_REJECT_WORKFLOW_CONFIG",
                detail: error.to_string(),
            }),
        };
        if let Some(resource_guard::Admission::Rejected { code, detail }) = workflow_admission {
            warn!("[benchmark] {} {}: workflow resource admission refused {}", code, detail, server_model_id);
            return ModelBenchmarkResult {
                model_id: model_id.to_string(),
                model_path: resolved_path.clone(),
                file_size_bytes: file_size,
                load_duration: Duration::ZERO,
                inference_results: vec![],
                unload_duration: Duration::ZERO,
                total_duration: start.elapsed(),
                tokens_per_second: 0.0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                error: Some(format!("{}: {}", code, detail)),
                gpu_mode: gpu_mode.to_string(),
                speedup_factor: None,
                step_id: None,
            };
        }

        if !already_loaded && Self::requires_default_load_admission(self.resource_admission.as_ref()) {
        let default_context_tokens: u32 = 32768;  // AGENTS.md rule: 32k default (2x buffer of 16k max response)
        let safety_margin_bytes: u64 = 1_073_741_824; // 1GB default; per-model override via max_allowed.memory_safety_margin_bytes

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
                        step_id: None,
                    };
                }
            }
            Err(e) => {
                warn!("[benchmark] [{}] memory check failed, proceeding with load: {}", model_id, e);
            }
        }

        }

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
                step_id: None,
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
                    step_id: None,
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
                step_id: None,
            };
        }

        let mut inference_results = Vec::new();

        for prompt in prompts {
            let max_retries: u32 = self.inference_max_attempts;
            let mut last_error = None;

            for attempt in 0..max_retries {
                match self.current_workflow_inference_resource_admission(server_model_id, file_size) {
                    Ok(Some(resource_guard::Admission::Rejected { code, detail })) => {
                        last_error = Some(format!("{}: {}", code, detail));
                        break;
                    }
                    Ok(_) => {}
                    Err(error) => {
                        last_error = Some(format!("RESOURCE_REJECT_WORKFLOW_CONFIG: {}", error));
                        break;
                    }
                }
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
                let tracker = self.pressure_tracker.clone();
                let guarded = resource_guard::guard_inference(
                    tracker, 750,
                    || (resource_guard::read_vram_sensors_sysfs(), resource_guard::read_ram_sensors_proc()),
                    client.chat_completion(request),
                );
                let outcome = match guarded.await {
                    Ok(inner) => inner,
                    Err(reason) => Err(anyhow::anyhow!("{}", reason)),
                };
                match outcome {
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
                        let err_str = e.to_string();
                        warn!("[benchmark] inference attempt {}/{} failed for {}: {}",
                            attempt + 1, max_retries, model_id, e);

                        if err_str.contains(resource_guard::RESOURCE_CRITICAL) {
                            warn!("[benchmark] {} in-flight inference cancelled — not retrying under resource pressure", resource_guard::RESOURCE_CRITICAL);
                            break;
                        }

                        if attempt + 1 < max_retries {
                            sleep(Duration::from_secs(2u64.pow(attempt))).await;
                        }
                    }
                }
            }

            if let Some(err) = last_error {
                warn!(
                    "[benchmark] all {} inference attempts failed for {}: {}",
                    max_retries, model_id, err
                );

                let exhausted_context = WorkflowHookContext::AfterAllRetriesExhausted(
                    AfterAllRetriesExhaustedContext {
                        step_name: model_id.to_string(),
                        total_attempts: max_retries,
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
                    response_text: format!("ERROR: All {} attempts failed: {}", max_retries, err),
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
            step_id: None,
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
    fn given_router_mode_zombie_command_when_built_then_excludes_router_process() {
        let cmd = BenchmarkRunner::zombie_check_command(8080);
        assert!(cmd.contains("models-dir"),
            "zombie check must exclude the router process so legitimately \
        loaded router children are not false-positive zombies");
    }

    // Issue S residual: the live server this run talks to (identified by its
    // --port) is not a zombie — excluding it gives foreign-process detection
    // the full threshold budget instead of burning one slot on ourselves.
    #[test]
    fn given_own_server_port_when_zombie_command_built_then_excludes_live_server() {
        let cmd = BenchmarkRunner::zombie_check_command(8080);
        assert!(cmd.contains("grep -v -- --port 8080"),
            "zombie check must exclude the live server on our own port");
        let other = BenchmarkRunner::zombie_check_command(9999);
        assert!(other.contains("grep -v -- --port 9999"));
        assert!(!other.contains("--port 8080"));
    }

    #[test]
    fn given_tmpdir_env_when_tmp_root_resolved_then_env_wins_over_hardcoded_tmp() {
        // Regression: preflight tmp-space checks hardcoded /tmp and ignored
        // TMPDIR (SUMMARY-overcontext.md:88-91; workaround was a PATH shim
        // faking `df -B1 /tmp` output in experiments/reasoning-enhancer/scripts/shims/df).
        let saved = std::env::var("TMPDIR").ok();
        let custom = std::env::temp_dir().join("whitt-tmpdir-probe");
        std::fs::create_dir_all(&custom).unwrap();
        std::env::set_var("TMPDIR", &custom);
        assert_eq!(BenchmarkRunner::tmp_root(), custom,
            "TMPDIR must override the hardcoded /tmp in space checks");
        std::env::set_var("TMPDIR", "");
        assert_eq!(BenchmarkRunner::tmp_root(), Path::new("/tmp"),
            "empty TMPDIR must fall back to /tmp");
        std::env::remove_var("TMPDIR");
        assert_eq!(BenchmarkRunner::tmp_root(), Path::new("/tmp"),
            "unset TMPDIR must fall back to /tmp");
        if let Some(prev) = saved {
            std::env::set_var("TMPDIR", prev);
        }
    }

    #[test]
    fn is_retry_exhausted_error_matches_known_patterns() {
        assert!(is_retry_exhausted_error("All 3 attempts failed: timeout"));
        assert!(is_retry_exhausted_error("All 1 attempts failed: connection refused"));
        assert!(!is_retry_exhausted_error("Load failed: docker not running"));
        assert!(!is_retry_exhausted_error("attempts failed without prefix"));
        assert!(!is_retry_exhausted_error(""));
    }

    #[tokio::test]
    async fn given_loaded_conflicting_model_when_serial_admission_prepared_then_unloads_before_admission() {
        use std::io::{Read, Write};
        use std::sync::{Arc, Mutex};

        // Given: server has worker loaded and heavy not loaded.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let server_requests = Arc::clone(&requests);
        std::thread::spawn(move || {
            for (request_number, stream) in listener.incoming().take(3).enumerate() {
                let mut stream = stream.unwrap();
                let mut buffer = [0u8; 8192];
                let count = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..count]);
                let first_line = request.lines().next().unwrap_or_default().to_string();
                server_requests.lock().unwrap().push(first_line.clone());
                let body = if request_number == 2 {
                    r#"{"data":[{"id":"worker","status":{"value":"unloaded"}},{"id":"heavy","status":{"value":"available"}}]}"#
                } else if first_line.starts_with("GET /v1/models") {
                    r#"{"data":[{"id":"worker","status":{"value":"loaded"}},{"id":"heavy","status":{"value":"available"}}]}"#
                } else {
                    "{}"
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body,
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        });
        let client = LlamaHttpClient::new(&format!("http://127.0.0.1:{port}")).unwrap();

        // When: heavy prepares for serial loading before admission.
        let admission_requests = Arc::clone(&requests);
        let (already_loaded, admission) = BenchmarkRunner::prepare_serial_model_load_then_admit(
            &client,
            "heavy",
            move || {
                admission_requests.lock().unwrap().push("admit".to_string());
                "admitted"
            },
        )
        .await;

        // Then: worker unloads before caller evaluates heavy load admission.
        assert!(!already_loaded);
        assert_eq!(admission, "admitted");
        assert_eq!(
            *requests.lock().unwrap(),
            vec![
                "GET /v1/models HTTP/1.1".to_string(),
                "POST /models/unload HTTP/1.1".to_string(),
                "GET /v1/models HTTP/1.1".to_string(),
                "admit".to_string(),
            ],
        );
    }

    #[test]
    fn given_linear_workflow_exceeding_100_steps_when_iteration_cap_computed_then_allows_full_run() {
        let steps: Vec<WorkflowStep> = (0..385)
            .map(|i| WorkflowStep {
                step_name: format!("s{}", i),
                step_id: format!("s{}", i),
                requires: vec![],
                require_conditions: Default::default(),
                retry_max_attempts: None,
                            when: None,
                prompt: None,
                generative_entity: None,
                model_overrides: None,
                r#loop: None,
            })
            .collect();
        assert!(BenchmarkRunner::max_workflow_iterations(&steps) >= 385);
    }

    #[test]
    fn given_no_loop_steps_when_iteration_cap_computed_then_floor_is_100() {
        let steps = vec![WorkflowStep {
            step_name: "only".into(),
            step_id: "only".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        }];
        assert_eq!(BenchmarkRunner::max_workflow_iterations(&steps), 100);
    }

    #[test]
    fn given_loop_max_iterations_when_iteration_cap_computed_then_respects_override_and_scale() {
        let mut with_loop = WorkflowStep {
            step_name: "looped".into(),
            step_id: "looped".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: Some(serde_json::json!({"count": {"max_iterations": 5000}})),
        };
        let steps = vec![
            with_loop.clone(),
            WorkflowStep {
                step_name: "tail".into(),
                step_id: "tail".into(),
                requires: vec![],
                require_conditions: Default::default(),
                retry_max_attempts: None,
                            when: None,
                prompt: None,
                generative_entity: None,
                model_overrides: None,
                r#loop: None,
            },
        ];
        assert_eq!(BenchmarkRunner::max_workflow_iterations(&steps), 5000);
        with_loop.r#loop = Some(serde_json::json!({"count": {"max_iterations": 3}}));
        assert_eq!(BenchmarkRunner::max_workflow_iterations(&[with_loop]), 100);
    }

    // Regression (Issue J, silent-wrong-behavior): "metrics.json lied
    // under early exit ('3/5 angles')" — correction-atom/results/
    // SUMMARY-v7.md:15 — every entry carried only the model FILE name
    // (identical across steps), so consumers could not attribute
    // results to steps nor distinguish executed vs hook-skipped.
    // Workflow results must now carry the step id.
    #[tokio::test]
    async fn given_hook_skipped_step_when_executed_then_result_attributed_to_step() {
        let runner = BenchmarkRunner::new(make_test_config());
        let step = WorkflowStep {
            step_name: "angle_one".into(),
            step_id: "angle_one".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: Some(serde_json::json!({
                "before_step_starts": [ {"skip_step": true} ]
            })),
            prompt: Some("say hi".into()),
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };
        // Dead port is safe: the hook-skip arm returns before any client use.
        let client = LlamaHttpClient::new("http://127.0.0.1:9").expect("client ctor only builds");
        let model = ("m.gguf".to_string(), "/models/m.gguf".to_string());
        let result = runner
            .execute_workflow_step(&step, &client, &model, 10, 0.1, 0.9, None)
            .await
            .expect("hook-skip arm must return Ok");
        assert_eq!(result.benchmark_result.error.as_deref(), Some("Skipped by hook"));
        assert_eq!(result.benchmark_result.step_id.as_deref(), Some("angle_one"));
    }

    // Regression (Issue AD): a `fail` action on after_step_succeeds is a
    // hook verdict that the step's output is unacceptable (e.g. a verifier
    // hook). The step result must record that verdict as an error so the
    // suite/report cannot count the step as successful. Previously the Fail
    // arm only warned and model_result.error stayed None.
    #[tokio::test]
    async fn given_success_hook_fail_verdict_when_step_executes_then_result_marked_failed() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = match stream { Ok(s) => s, Err(_) => continue };
                let mut buf = [0u8; 8192];
                let mut n = stream.read(&mut buf).unwrap_or(0);
                let mut req = String::from_utf8_lossy(&buf[..n]).to_string();
                while let Some(cl) = content_length(&req) {
                    let body_start = req.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
                    if req.len() >= body_start + cl { break; }
                    n = stream.read(&mut buf).unwrap_or(0);
                    if n == 0 { break; }
                    req.push_str(&String::from_utf8_lossy(&buf[..n]));
                }
                let first = req.lines().next().unwrap_or_default().to_string();
                let (status, body) = if first.starts_with("GET /v1/models") {
                    ("200 OK", r#"{"data":[{"id":"m.gguf","status":{"value":"loaded"}},{"id":"m","status":{"value":"loaded"}}]}"#.to_string())
                } else if first.starts_with("POST /v1/chat/completions") {
                    ("200 OK", r#"{"id":"x","object":"chat.completion","created":0,"model":"m","choices":[{"index":0,"message":{"role":"assistant","content":"hello world"},"finish_reason":"stop"}],"usage":{"prompt_tokens":2,"completion_tokens":2,"total_tokens":4}}"#.to_string())
                } else if first.starts_with("POST /models/load") {
                    ("200 OK", r#"{"success":true}"#.to_string())
                } else if first.starts_with("POST /models/unload") {
                    ("200 OK", "{}".to_string())
                } else {
                    ("200 OK", "{}".to_string())
                };
                let resp = format!(
                    "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status, body.len(), body
                );
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.flush();
            }
        });

        fn content_length(req: &str) -> Option<usize> {
            for line in req.lines() {
                if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    return v.trim().parse().ok();
                }
            }
            None
        }

        let runner = BenchmarkRunner::new(make_test_config());
        let step = WorkflowStep {
            step_name: "verified_step".into(),
            step_id: "verified_step".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
            when: Some(serde_json::json!({
                "after_step_succeeds": [ {"fail": {"message": "verifier rejected output"}} ]
            })),
            prompt: Some("say hi".into()),
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };
        let client = LlamaHttpClient::new(&format!("http://127.0.0.1:{}", port)).expect("client ctor");
        let model = ("m.gguf".to_string(), "/models/m.gguf".to_string());
        let result = runner
            .execute_workflow_step(&step, &client, &model, 10, 0.1, 0.9, None)
            .await
            .expect("step must execute against mock server");
        let err = result
            .benchmark_result
            .error
            .expect("hook fail verdict must mark the step result as failed");
        assert!(
            err.contains("verifier rejected output"),
            "error must carry the hook's reason, got: {}",
            err
        );
    }

    // Regression (Issue J): BenchmarkSuiteResult carried only
    // total_models=results.len() with no planned-step count, so under
    // early exit consumers mis-derived "3/5 angles" from a suite that
    // silently dropped unexecuted steps. The suite must state how many
    // steps the workflow planned.
    #[test]
    fn given_results_and_planned_steps_when_suite_assembled_then_counts_honest() {
        fn base_result(error: Option<String>) -> ModelBenchmarkResult {
            ModelBenchmarkResult {
                model_id: "m.gguf".to_string(),
                model_path: "/models/m.gguf".to_string(),
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
                error,
                gpu_mode: "gpu".to_string(),
                speedup_factor: None,
                step_id: None,
            }
        }
        let r1 = base_result(None);
        let r2 = base_result(None);
        let r3 = base_result(Some("Skipped by hook".to_string()));

        let suite = BenchmarkRunner::suite_result_from(vec![r1, r2, r3], Some(5), "http://x");
        assert_eq!(suite.total_models, 3);
        assert_eq!(suite.successful, 2);
        assert_eq!(suite.failed, 1);
        assert_eq!(suite.planned_steps, Some(5));

        let suite_plain = BenchmarkRunner::suite_result_from(vec![], None, "http://x");
        assert_eq!(suite_plain.planned_steps, None);
        // Model-discovery mode JSON stays byte-identical: planned_steps
        // is omitted when None.
        let v = serde_json::to_value(&suite_plain).expect("serialize suite");
        assert!(v.get("planned_steps").is_none());
    }

    // Regression (Issue G, silent-wrong-behavior): "relative --out-dir
    // breaks engine shell hooks (always absolute)" —
    // experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:109.
    // Relative output dirs resolved against process CWD made hook
    // paths (WHITT_OUTPUT_DIR, save_to base, working_dir) land in
    // unpredictable locations. Engine must absolutize at ingestion.
    #[test]
    fn given_relative_output_dir_when_normalized_then_absolute_against_cwd() {
        let abs = BenchmarkRunner::normalized_output_dir("outputs/run-42");
        assert!(abs.is_absolute(), "relative --out-dir must become absolute: {abs:?}");
        let cwd = std::env::current_dir().expect("cwd");
        assert!(abs.starts_with(&cwd), "absolute must be cwd-joined: {abs:?} vs {cwd:?}");

        let already_abs = BenchmarkRunner::normalized_output_dir("/tmp/abs-out");
        assert_eq!(already_abs, std::path::PathBuf::from("/tmp/abs-out"));
    }

    // Regression (Issue K, silent-wrong-behavior): "Engine-managed swap →
    // Model not found" — experiments/atomic-reasoning/benchmarks/SUMMARY.md:125.
    // resolve_model_file matched by raw case-sensitive substring with a
    // base-name fallback, so valid models failed resolution (and steps died
    // "Model not found") on case or '.'/'-' spelling drift between YAML and
    // filename (07-TRACKING.md:61-63: "Qwen3-5-9B not Qwen3.5-9B").
    #[test]
    fn given_dot_dash_spelling_drift_when_model_resolved_then_normalized_match() {
        let runner = BenchmarkRunner::new(make_test_config());
        let discovered = vec![(
            "Qwen3-5-9B-Q4_K_M.gguf".to_string(),
            "/models/Qwen3-5-9B-Q4_K_M.gguf".to_string(),
        )];
        // YAML spelled it "Qwen3.5-9B": neither substring nor base-name
        // ("Qwen3.5") matches the file — resolution returned None before.
        let hit = runner.resolve_model_file("Qwen3.5-9B-Q4_K_M", &discovered);
        assert!(hit.is_some(), "dot/dash drift must resolve via normalization");
        assert_eq!(hit.unwrap().0, "Qwen3-5-9B-Q4_K_M.gguf");
    }

    // Regression (Issue K / N): case-sensitive matching made valid
    // lowercase YAML names miss capitalized filenames entirely.
    #[test]
    fn given_case_mismatch_when_model_resolved_then_case_insensitive_match() {
        let runner = BenchmarkRunner::new(make_test_config());
        let discovered = vec![(
            "Ministral-3B-instruct.gguf".to_string(),
            "/models/Ministral-3B-instruct.gguf".to_string(),
        )];
        let hit = runner.resolve_model_file("ministral-3b-instruct", &discovered);
        assert!(hit.is_some(), "case drift must resolve via normalization");
        assert_eq!(hit.unwrap().0, "Ministral-3B-instruct.gguf");
    }

    // Regression (Issue K): substring-first matching let an arbitrary
    // sibling variant win over the exact file the step asked for.
    #[test]
    fn given_exact_name_and_variant_when_model_resolved_then_exact_wins() {
        let runner = BenchmarkRunner::new(make_test_config());
        let discovered = vec![
            (
                "Model-X-Q8_0.gguf".to_string(),
                "/models/Model-X-Q8_0.gguf".to_string(),
            ),
            (
                "Model-X.gguf".to_string(),
                "/models/Model-X.gguf".to_string(),
            ),
        ];
        let hit = runner.resolve_model_file("Model-X", &discovered);
        assert_eq!(hit.expect("must resolve").0, "Model-X.gguf");
    }

    // Regression (Issue E, silent-wrong-behavior): "workflow loop hard cap
    // 100 iterations (silent exit 0 after — log shows `workflow loop
    // exceeded 100 iterations`)" — experiments/reasoning-enhancer/results/
    // SUMMARY-benchmark.md:69-71. Cap exhaustion with steps unexecuted MUST
    // be an error, not a warn-and-return-Ok.
    #[test]
    fn given_cap_exhausted_with_steps_unexecuted_when_cap_error_computed_then_some() {
        let msg = BenchmarkRunner::premature_cap_error(100, 100, 12, 100);
        assert!(msg.is_some(), "cap exhausted with 88 steps unexecuted must yield error");
        let msg = msg.unwrap();
        assert!(msg.contains("100"), "message should name the cap: {msg}");
        assert!(msg.contains("88"), "message should count unexecuted steps: {msg}");
    }

    #[test]
    fn given_workflow_completed_at_cap_when_cap_error_computed_then_none() {
        // Boundary: workflow finished exactly at the cap — NOT an error.
        assert!(BenchmarkRunner::premature_cap_error(100, 100, 100, 100).is_none());
        // Cap not reached — normal case.
        assert!(BenchmarkRunner::premature_cap_error(42, 100, 42, 42).is_none());
    }

    // Regression (Issue O, loud-crash/log pollution): "Every log line
    // written 2× — deleted duplicate `info!` at main-loop call site (kept
    // inner log, runner.rs:1975) — live-verified: 1 line/step ... source
    // restored byte-exact" — experiments/correction-atom/results/
    // SUMMARY-v7.md:14. The restoration un-shipped the fix; the duplicate
    // re-emerged (call site + executor). Exactly one emission site allowed.
    #[test]
    fn given_workflow_step_execution_when_emission_sites_counted_then_exactly_one() {
        let src = include_str!("runner.rs");
        let needle = concat!("executing step", " {} with model");
        let count = src.matches(needle).count();
        assert_eq!(
            count, 1,
            "duplicate 'executing step' log emission sites found ({count}) — every step logs 2×; keep the executor's inner log only"
        );
    }

    // Regression (Issue F2): on_requires_failed / after_loop_iteration_fails
    // hook triggers fired with `let _ =` — hook execution errors (bad shell,
    // unwritable path, ...) vanished, unlike every other trigger which logs
    // an error. Zero swallowed-trigger call sites allowed.
    #[test]
    fn given_hook_trigger_call_sites_when_counted_then_none_swallow_errors() {
        let src = include_str!("runner.rs");
        let needle = concat!("let _ = self.", "execute_hooks_for_trigger");
        let count = src.matches(needle).count();
        assert_eq!(
            count, 0,
            "swallowed hook-trigger errors found ({count} sites) — every trigger must match/Err-log like after_step_fails"
        );
    }

    #[test]
    fn refusal_patterns_default_and_override_via_builder() {
        let cfg = BenchmarkConfig {
            server_url: "http://x".into(), models_dir: None, model_list_file: None,
            prompts: vec![], max_tokens: 1, filter_size_max: None, filter_size_min: None,
            filter_name: None, delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false, output_dir: None, workflow_file: None,
            temperature: None, top_p: None, cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false, model_load_timeout: Duration::from_secs(1), min_tmp_space_mb: 1,
        };
        let runner = BenchmarkRunner::new(cfg);
        assert!(runner.refusal_patterns().len() >= 20, "default should have 20+ patterns, got {}", runner.refusal_patterns().len());
        assert!(runner.refusal_patterns().iter().any(|p| p.contains("I cannot help")));

        let cfg2 = BenchmarkConfig {
            server_url: "http://x".into(), models_dir: None, model_list_file: None,
            prompts: vec![], max_tokens: 1, filter_size_max: None, filter_size_min: None,
            filter_name: None, delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false, output_dir: None, workflow_file: None,
            temperature: None, top_p: None, cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false, model_load_timeout: Duration::from_secs(1), min_tmp_space_mb: 1,
        };
        let runner2 = BenchmarkRunner::new(cfg2).with_refusal_patterns(vec!["CUSTOM_REFUSAL".into()]);
        assert_eq!(runner2.refusal_patterns().len(), 1);
        assert_eq!(runner2.refusal_patterns()[0], "CUSTOM_REFUSAL");
    }

    #[test]
    fn detect_refusal_returns_zero_on_match_else_baseline() {
        let patterns = vec!["I cannot help".to_string()];
        assert_eq!(detect_refusal("Sorry, I cannot help with that", &patterns, Some(0.9)), Some(0.0));
        assert_eq!(detect_refusal("Sure, here is the answer", &patterns, Some(0.9)), Some(0.9));
        assert_eq!(detect_refusal("", &patterns, None), None);
    }

    #[test]
    fn inference_max_attempts_defaults_to_3_and_overrides() {
        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let runner = BenchmarkRunner::new(config);
        assert_eq!(runner.inference_max_attempts(), 3, "default should be 3");

        let runner2 = BenchmarkRunner::new(BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        }).with_inference_max_attempts(5);
        assert_eq!(runner2.inference_max_attempts(), 5, "override should propagate");

        let runner3 = BenchmarkRunner::new(BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        }).with_inference_max_attempts(0);
        assert_eq!(runner3.inference_max_attempts(), 1, "0 clamped to 1");
    }

    #[test]
    fn timing_config_deserializes_from_yaml() {
        let yaml = r#"
cooldown_after_unload_secs: 7
model_load_timeout_secs: 600
min_tmp_space_mb: 2048
"#;
        let cfg: crate::workflow::TimingConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(cfg.cooldown_after_unload_secs, Some(7));
        assert_eq!(cfg.model_load_timeout_secs, Some(600));
        assert_eq!(cfg.min_tmp_space_mb, Some(2048));
    }

    #[test]
    fn timing_config_accepts_partial_yaml() {
        let yaml = r#"
model_load_timeout_secs: 120
"#;
        let cfg: crate::workflow::TimingConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(cfg.cooldown_after_unload_secs, None);
        assert_eq!(cfg.model_load_timeout_secs, Some(120));
        assert_eq!(cfg.min_tmp_space_mb, None);
    }

    // Regression (Issue D, silent-wrong-behavior): "YAML
    // timing.min_tmp_space_mb did NOT reach preflight [config built
    // pre-parse] — CLI flag is the reliable path" —
    // experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:284-286.
    // Overrides must apply to config BEFORE preflight consumes it.
    #[test]
    fn given_workflow_yaml_min_tmp_space_when_timing_overrides_applied_then_config_updated() {
        let dir = std::env::temp_dir().join("whitt_issue_d_timing_override");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let yaml_path = dir.join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: issue-d-timing
name: issue-d-timing
schema_version: "2.0.0"
workspace:
  directories:
    output: /tmp/out
workflow_execution_strategy:
  timing:
    min_tmp_space_mb: 2048
agentic_workflow:
  steps:
    s1:
      prompt: "hello"
"#).unwrap();

        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: Some(yaml_path.to_string_lossy().to_string()),
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let mut runner = BenchmarkRunner::new(config);
        let ctx = runner.load_workflow_config()
            .expect("load_workflow_config should succeed")
            .expect("YAML should produce Some(config)");

        runner.apply_workflow_timing_overrides(&ctx);

        assert_eq!(runner.config.min_tmp_space_mb, 2048,
            "YAML timing.min_tmp_space_mb must override config before preflight reads it");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn timing_config_rejects_unknown_field() {
        let yaml = r#"
bogus_field: 1
"#;
        let result: Result<crate::workflow::TimingConfig, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err(), "deny_unknown_fields must reject bogus_field");
    }

    #[test]
    fn workflow_resource_admission_extracts_from_validated_yaml() {
        let dir = std::env::temp_dir().join("whitt_resource_admission_extract");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let yaml_path = dir.join("workflow.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: resource-admission-extract
name: resource-admission-extract
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
agentic_workflow:
  steps:
    s1:
      prompt: "hello"
"#).unwrap();

        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: Some(yaml_path.to_string_lossy().to_string()),
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let extracted = BenchmarkRunner::new(config)
            .load_workflow_config()
            .expect("load workflow config")
            .expect("workflow config");
        let admission = extracted.resource_admission.expect("resource admission");
        assert_eq!(admission.minimum_available.ram, "6GiB");
        assert_eq!(admission.model_estimate.expected_runtime_secs, 900);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn workflow_resource_admission_blocks_before_model_load() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-block
name: resource-admission-block
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let vram = resource_guard::VramSensors {
            total_bytes: 8 * 1024 * 1024 * 1024,
            free_bytes: 8 * 1024 * 1024 * 1024,
        };
        let ram = resource_guard::RamSensors {
            mem_available_kb: 5 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match workflow_resource_admission(&admission, "model", 1024, &vram, &ram)
            .expect("admission decision")
        {
            resource_guard::Admission::Rejected { code, .. } => {
                assert_eq!(code, resource_guard::REJECT_WORKFLOW_RAM)
            }
            other => panic!("expected workflow RAM rejection, got {other:?}"),
        }
    }

    #[test]
    fn workflow_inference_admission_uses_compute_headroom_after_model_load() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: inference-admission
name: inference-admission
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 180
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let vram = resource_guard::VramSensors {
            total_bytes: 8 * 1024 * 1024 * 1024,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = resource_guard::RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(matches!(
            workflow_inference_resource_admission_from_samples(
                &admission,
                "model-a",
                5_027_783_968,
                Some(&vram),
                Some(&ram),
            ).expect("admission"),
            resource_guard::Admission::Admitted,
        ));
    }

    #[test]
    fn workflow_preflight_admission_uses_total_vram_capacity_when_model_is_loaded() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: preflight-capacity-admission
name: preflight-capacity-admission
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 180
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let vram = resource_guard::VramSensors {
            total_bytes: 8 * 1024 * 1024 * 1024,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = resource_guard::RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(matches!(
            workflow_preflight_resource_admission_from_samples(
                &admission,
                "model-a",
                Some(&vram),
                Some(&ram),
            ).expect("admission"),
            resource_guard::Admission::Admitted,
        ));
    }

    #[test]
    fn preflight_runner_admits_total_vram_capacity_when_model_is_loaded() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: preflight-runner-capacity-admission
name: preflight-runner-capacity-admission
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 180
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let mut runner = BenchmarkRunner::new(make_test_config());
        runner.resource_admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission;
        let vram = resource_guard::VramSensors {
            total_bytes: 8 * 1024 * 1024 * 1024,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = resource_guard::RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(runner
            .preflight_workflow_resource_admission_from_samples(Some(&vram), Some(&ram))
            .is_ok());
    }

    #[test]
    fn workflow_admission_uses_inference_headroom_when_target_model_is_loaded() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: loaded-model-admission
name: loaded-model-admission
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 180
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let vram = resource_guard::VramSensors {
            total_bytes: 8 * 1024 * 1024 * 1024,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = resource_guard::RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(matches!(
            workflow_model_state_resource_admission_from_samples(
                &admission,
                "model-a",
                5_027_783_968,
                true,
                Some(&vram),
                Some(&ram),
            ).expect("admission"),
            resource_guard::Admission::Admitted,
        ));
        assert!(matches!(
            workflow_model_state_resource_admission_from_samples(
                &admission,
                "model-a",
                5_027_783_968,
                false,
                Some(&vram),
                Some(&ram),
            ).expect("admission"),
            resource_guard::Admission::Rejected {
                code: resource_guard::REJECT_WORKFLOW_VRAM,
                ..
            },
        ));
    }

    #[test]
    fn resource_contract_replaces_duplicate_default_load_estimate() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: declared-load-estimate
name: declared-load-estimate
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 180
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");

        assert!(!BenchmarkRunner::requires_default_load_admission(Some(&admission)));
        assert!(BenchmarkRunner::requires_default_load_admission(None));
    }

    #[test]
    fn workflow_model_specs_feed_benchmark_discovery() {
        let mut config = make_test_config();
        config.workflow_file = Some(
            "docs/benchmarks/workflows/resource-admission-reject.yml".to_string(),
        );
        let workflow = BenchmarkRunner::new(config)
            .load_workflow_config()
            .expect("load workflow config")
            .expect("workflow config");
        assert_eq!(
            workflow.model_list,
            vec!["Qwen3-8B-Q4_K_M.gguf"],
            "top-level schema-valid model specs must drive benchmark discovery",
        );
    }

    #[test]
    fn workflow_resource_admission_collects_host_model_source_paths() {
        let mut config = make_test_config();
        config.workflow_file = Some(
            "docs/benchmarks/workflows/resource-admission-simple.yml".to_string(),
        );
        let workflow = BenchmarkRunner::new(config)
            .load_workflow_config()
            .expect("load workflow config")
            .expect("workflow config");

        assert_eq!(
            workflow
                .model_source_paths
                .get("Qwen3-8B-Q4_K_M.gguf")
                .map(String::as_str),
            Some("/run/media/jon/data/models/Qwen3-8B-Q4_K_M.gguf"),
        );
    }

    #[test]
    fn workflow_host_source_path_overrides_model_name_fallback() {
        let source_dir = std::env::temp_dir().join(format!(
            "whitt-workflow-source-path-{}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&source_dir).expect("create source directory");
        let source_path = source_dir.join("model-a.gguf");
        std::fs::write(&source_path, b"weights").expect("write source file");
        let model_id = "model-a.gguf".to_string();
        let model_list = vec![model_id.clone()];
        let model_source_paths = BTreeMap::from([(
            model_id.clone(),
            source_path.to_string_lossy().to_string(),
        )]);

        let models = BenchmarkRunner::new(make_test_config())
            .discover_models(Some(&model_list), Some(&model_source_paths))
            .expect("discover workflow model");

        assert_eq!(models, vec![(model_id, source_path.to_string_lossy().to_string())]);
        std::fs::remove_dir_all(source_dir).expect("remove source directory");
    }

    #[test]
    fn resource_admission_telemetry_records_minima_and_latest_decision() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-telemetry
name: resource-admission-telemetry
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let output_dir = std::env::temp_dir().join(format!(
            "whitt-resource-admission-telemetry-{}",
            std::process::id(),
        ));
        let mut config = make_test_config();
        config.output_dir = Some(output_dir.to_string_lossy().to_string());
        let mut runner = BenchmarkRunner::new(config);
        runner.resource_admission_started_at = Arc::new(Instant::now() - Duration::from_secs(42));
        runner.resource_admission = Some(admission);

        runner.record_workflow_resource_admission(
            "model-a.gguf",
            1_024,
            Some(&resource_guard::VramSensors {
                total_bytes: 8 * 1024 * 1024 * 1024,
                free_bytes: 7 * 1024 * 1024 * 1024,
            }),
            Some(&resource_guard::RamSensors {
                mem_available_kb: 8 * 1024 * 1024,
                swap_free_kb: 6 * 1024 * 1024,
                mem_total_kb: 16 * 1024 * 1024,
            }),
            &resource_guard::Admission::Admitted,
        ).expect("first telemetry write");
        runner.record_workflow_resource_admission(
            "model-a.gguf",
            1_024,
            Some(&resource_guard::VramSensors {
                total_bytes: 8 * 1024 * 1024 * 1024,
                free_bytes: 6 * 1024 * 1024 * 1024,
            }),
            Some(&resource_guard::RamSensors {
                mem_available_kb: 7 * 1024 * 1024,
                swap_free_kb: 5 * 1024 * 1024,
                mem_total_kb: 16 * 1024 * 1024,
            }),
            &resource_guard::Admission::Rejected {
                code: resource_guard::REJECT_WORKFLOW_RAM,
                detail: "test rejection".into(),
            },
        ).expect("second telemetry write");

        let profile_path = output_dir.join("resource-admission-profile.json");
        let profile: serde_json::Value = serde_json::from_slice(
            &std::fs::read(&profile_path).expect("telemetry profile"),
        ).expect("valid telemetry JSON");
        let model = &profile["models"]["model-a.gguf"];
        assert_eq!(model["check_count"], 2);
        assert_eq!(model["weights_bytes"], 1_024);
        assert_eq!(model["minimum_available_ram_bytes"], 7 * 1024 * 1024 * 1024u64);
        assert_eq!(model["minimum_available_vram_bytes"], 6 * 1024 * 1024 * 1024u64);
        assert_eq!(model["minimum_available_swap_free_bytes"], 5 * 1024 * 1024 * 1024u64);
        assert_eq!(model["latest_admission"]["code"], resource_guard::REJECT_WORKFLOW_RAM);
        assert!(profile["observed_runtime_secs"].as_f64().expect("runtime") >= 42.0);

        std::fs::remove_dir_all(output_dir).expect("remove telemetry output");
    }

    #[tokio::test]
    async fn workflow_resource_admission_rejects_before_preflight_http() {
        let mut config = make_test_config();
        config.server_url = "http://127.0.0.1:9".to_string();
        config.workflow_file = Some(
            "docs/benchmarks/workflows/resource-admission-reject.yml".to_string(),
        );
        let error = BenchmarkRunner::new(config)
            .run()
            .await
            .expect_err("impossible workflow resources must reject before preflight HTTP")
            .to_string();
        assert!(
            error.contains("RESOURCE_REJECT_WORKFLOW"),
            "expected resource rejection before unreachable HTTP, got: {error}",
        );
    }

    #[tokio::test]
    async fn workflow_resource_admission_rejects_before_http_calls() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-network-block
name: resource-admission-network-block
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 999GiB
      vram: 1GiB
      swap_free: 1GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let config = BenchmarkConfig {
            server_url: "http://127.0.0.1:9".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let mut runner = BenchmarkRunner::new(config);
        runner.resource_admission = Some(admission);
        let client = LlamaHttpClient::new("http://127.0.0.1:9").expect("client");
        let result = runner
            .run_model_inference(
                &client,
                "model.gguf",
                "/missing/model.gguf",
                "gpu",
                &["prompt".to_string()],
                1,
                0.7,
                0.9,
                None,
                false,
                None,
            )
            .await;
        assert!(
            result.error.as_deref().is_some_and(|error| error.contains("RESOURCE_REJECT_WORKFLOW")),
            "must reject before contacting unreachable server: {:?}",
            result.error
        );
    }

    #[tokio::test]
    async fn legacy_benchmark_path_rejects_resource_contract_before_http_calls() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-legacy-block
name: resource-admission-legacy-block
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 999GiB
      vram: 1GiB
      swap_free: 1GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let config = BenchmarkConfig {
            server_url: "http://127.0.0.1:9".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let mut runner = BenchmarkRunner::new(config);
        runner.resource_admission = Some(admission);
        let client = LlamaHttpClient::new("http://127.0.0.1:9").expect("client");
        let hook_config = None;
        let result = runner
            .benchmark_single_model(
                &client,
                "model.gguf",
                "/missing/model.gguf",
                "gpu",
                &["prompt".to_string()],
                1,
                0.7,
                0.9,
                None,
                &hook_config,
            )
            .await;
        assert!(
            result.error.as_deref().is_some_and(|error| error.contains("RESOURCE_REJECT_WORKFLOW")),
            "must reject before contacting unreachable server: {:?}",
            result.error
        );
    }

    #[tokio::test]
    async fn streaming_path_rejects_resource_contract_before_http_calls() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-streaming-block
name: resource-admission-streaming-block
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 999GiB
      vram: 1GiB
      swap_free: 1GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let config = BenchmarkConfig {
            server_url: "http://127.0.0.1:9".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: None,
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let mut runner = BenchmarkRunner::new(config);
        runner.resource_admission = Some(admission);
        let client = LlamaHttpClient::new("http://127.0.0.1:9").expect("client");
        let step_when = None;
        let result = runner
            .run_model_inference_streaming(
                &client,
                "model.gguf",
                "/missing/model.gguf",
                "prompt",
                1,
                0.7,
                0.9,
                None,
                &step_when,
                "step",
            )
            .await;
        assert!(
            result.error.as_deref().is_some_and(|error| error.contains("RESOURCE_REJECT_WORKFLOW")),
            "must reject before opening unreachable stream: {:?}",
            result.error
        );
    }

    #[test]
    fn configured_resource_admission_rejects_missing_sensors() {
        let workflow = crate::workflow::WorkflowFile::from_yaml(r#"
workflow_id: resource-admission-sensors
name: resource-admission-sensors
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 1GiB
      vram: 1GiB
      swap_free: 1GiB
    model_estimate:
      kv_cache: 1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#).expect("valid workflow");
        let admission = workflow
            .workflow_execution_strategy
            .expect("strategy")
            .resource_admission
            .expect("admission");
        let ram = resource_guard::RamSensors {
            mem_available_kb: 12 * 1024 * 1024,
            swap_free_kb: 12 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match workflow_resource_admission_from_samples(&admission, "model", 1024, None, Some(&ram))
            .expect("admission decision")
        {
            resource_guard::Admission::Rejected { code, .. } => {
                assert_eq!(code, resource_guard::REJECT_WORKFLOW_SENSORS)
            }
            other => panic!("expected missing-sensor rejection, got {other:?}"),
        }
    }

    #[test]
    fn model_selection_yaml_fields_extract_correctly() {
        let dir = std::env::temp_dir().join("ms_test_yaml");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let yaml_path = dir.join("test.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: ms-test
name: ms-test
schema_version: "2.0.0"
workspace:
  directories:
    output: /tmp/out
  model_selection:
    strategy: diverse_n
    diverse_n_count: 3
    model_filter: "Qwen3-[45]"
  detail_template: "Model {{model_id}}: {{tokens_per_second}} tok/s"
agentic_workflow:
  steps:
    s1:
      prompt: "hello"
"#).unwrap();

        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: Some(yaml_path.to_string_lossy().to_string()),
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let runner = BenchmarkRunner::new(config);
        let extracted = runner.load_workflow_config()
            .expect("load_workflow_config should succeed")
            .expect("YAML should produce Some(config)");
        assert_eq!(extracted.model_selection_strategy.as_deref(), Some("diverse_n"));
        assert_eq!(extracted.diverse_n_count, Some(3));
        assert_eq!(extracted.model_filter.as_deref(), Some("Qwen3-[45]"));
        assert_eq!(extracted.detail_template.as_deref(), Some("Model {{model_id}}: {{tokens_per_second}} tok/s"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn model_selection_yaml_defaults_to_none_when_absent() {
        let dir = std::env::temp_dir().join("ms_test_yaml_default");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let yaml_path = dir.join("test.yml");
        std::fs::write(&yaml_path, r#"
workflow_id: ms-test-default
name: ms-test-default
schema_version: "2.0.0"
workspace:
  directories:
    output: /tmp/out
agentic_workflow:
  steps:
    s1:
      prompt: "hello"
"#).unwrap();

        let config = BenchmarkConfig {
            server_url: "http://localhost:8080".to_string(),
            models_dir: None,
            model_list_file: None,
            prompts: vec![],
            max_tokens: 1,
            filter_size_max: None,
            filter_size_min: None,
            filter_name: None,
            delay_between_swaps: Duration::from_secs(0),
            compare_gpu_cpu: false,
            output_dir: None,
            workflow_file: Some(yaml_path.to_string_lossy().to_string()),
            temperature: None,
            top_p: None,
            cooldown_after_unload: Duration::from_secs(0),
            preflight_only: false,
            model_load_timeout: Duration::from_secs(1),
            min_tmp_space_mb: 1,
        };
        let runner = BenchmarkRunner::new(config);
        let extracted = runner.load_workflow_config()
            .expect("load_workflow_config should succeed")
            .expect("YAML should produce Some(config)");
        assert_eq!(extracted.model_selection_strategy, None);
        assert_eq!(extracted.diverse_n_count, None);
        assert_eq!(extracted.model_filter, None);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn require_step_prompt_allows_control_flow_without_prompt() {
        let step = WorkflowStep {
            step_name: "decide".into(),
            step_id: "s1".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: None,
            generative_entity: Some("control_flow".into()),
            model_overrides: None,
            r#loop: None,
        };
        let result = require_step_prompt(&step).unwrap();
        assert!(result.is_empty(), "control_flow steps return empty prompt");
    }

    #[test]
    fn require_step_prompt_rejects_generative_without_prompt() {
        let step = WorkflowStep {
            step_name: "infer".into(),
            step_id: "s2".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: None,
            generative_entity: Some("agent".into()),
            model_overrides: None,
            r#loop: None,
        };
        let err = require_step_prompt(&step).unwrap_err();
        assert!(format!("{}", err).contains("requires non-empty `prompt`"),
            "got: {}", err);
    }

    #[test]
    fn require_step_prompt_rejects_whitespace_only_prompt() {
        let step = WorkflowStep {
            step_name: "infer".into(),
            step_id: "s3".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: Some("   \n\t  ".into()),
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        };
        let err = require_step_prompt(&step).unwrap_err();
        assert!(format!("{}", err).contains("requires non-empty `prompt`"));
    }

    #[test]
    fn require_step_prompt_accepts_real_prompt() {
        let step = WorkflowStep {
            step_name: "infer".into(),
            step_id: "s4".into(),
            requires: vec![],
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: Some("Hello world".into()),
            generative_entity: Some("agent".into()),
            model_overrides: None,
            r#loop: None,
        };
        let result = require_step_prompt(&step).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn streaming_config_deserializes_from_yaml() {
        let yaml = r#"
enabled: true
"#;
        let cfg: crate::workflow::StreamingConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(cfg.enabled, Some(true));
    }

    #[test]
    fn streaming_config_default_is_none() {
        let cfg: crate::workflow::StreamingConfig = serde_yaml::from_str("").unwrap();
        assert_eq!(cfg.enabled, None);
    }

    #[test]
    fn has_during_streaming_hook_detects_config() {
        let with_hook: serde_json::Value = serde_json::json!({
            "during_step_streaming": { "log": { "to_file_path": "/tmp/x.log" } }
        });
        let without_hook: serde_json::Value = serde_json::json!({
            "after_step_succeeds": { "log": {} }
        });
        assert!(BenchmarkRunner::has_during_streaming_hook(&Some(with_hook)));
        assert!(!BenchmarkRunner::has_during_streaming_hook(&Some(without_hook)));
        assert!(!BenchmarkRunner::has_during_streaming_hook(&None));
    }

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
            step_id: None,
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
            step_id: None,
        };

        let suite_result = BenchmarkSuiteResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            server_url: "http://localhost:8080".to_string(),
            total_models: 1,
            successful: 1,
            failed: 0,
            planned_steps: None,
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
            step_id: None,
        };

        let suite_result = BenchmarkSuiteResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            server_url: "http://localhost:8080".to_string(),
            total_models: 1,
            successful: 1,
            failed: 0,
            planned_steps: None,
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
                refusal_detected: false,
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
                refusal_detected: false,
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
    fn given_new_runner_when_pressure_tracker_polled_then_starts_green() {
        let tracker = resource_guard::PressureTracker::new();
        assert!(matches!(tracker.state(), resource_guard::Pressure::Green));
    }

    #[test]
    fn given_no_env_when_server_container_name_then_defaults() {
        std::env::remove_var("WHITT_SERVER_CONTAINER");
        assert_eq!(BenchmarkRunner::server_container_name(), "whitt-llama-server");
    }

    #[test]
    fn test_calculate_max_concurrent_from_ram() {
        assert_eq!(resource_guard::max_concurrent_ram(8.0), 4,
            "8GB RAM should allow 4 concurrent inferences");
    }

    #[test]
    fn test_calculate_max_concurrent_floor_at_1() {
        assert_eq!(resource_guard::max_concurrent_ram(1.5), 1,
            "1.5GB RAM should allow 1 concurrent inference (floor at 1)");
    }

    #[test]
    fn test_calculate_max_concurrent_cap_at_4() {
        assert_eq!(resource_guard::max_concurrent_ram(32.0), 4,
            "32GB RAM should be capped at 4 concurrent inferences");
    }

    #[test]
    fn test_read_proc_vram_nvidia_missing_dir() {
        let vram_mb = read_proc_vram_nvidia();
        assert!(vram_mb.is_none(), "should return None when /proc/driver/nvidia/gpus doesn't exist");
    }

    // --- Issue I regression: VRAM unit misreport + lost env override ---
    // Evidence: experiments/atomic-reasoning/SAFETY.md:5-9,88 — "8192.0 GB VRAM
    // available" on an 8GB RX 580; sysfs mem_info_vram_total reports BYTES but the
    // engine divided as KB → 8589934592/1024/1024 = 8192.0 GB → concurrency 4 → OOM.
    // Workaround WHITT_MAX_CONCURRENT_INFERENCES=1 (run-atom.sh:81) worked only in
    // an uncommitted experiment build — never landed in committed src.

    #[test]
    fn given_rx580_vram_bytes_when_converted_then_reports_8gb_not_8192() {
        assert!((resource_guard::bytes_to_gb(8_589_934_592) - 8.0).abs() < 1e-9,
            "8GiB in bytes must convert to 8.0 GB, not 8192.0 GB");
        assert!((resource_guard::bytes_to_gb(0) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn given_env_concurrency_override_when_parsed_then_valid_wins_and_invalid_ignored() {
        // Workaround contract from experiments (run-atom.sh:81, SAFETY.md):
        // WHITT_MAX_CONCURRENT_INFERENCES forces sequential inference on 8GB boxes.
        let saved = std::env::var("WHITT_MAX_CONCURRENT_INFERENCES").ok();
        std::env::set_var("WHITT_MAX_CONCURRENT_INFERENCES", "1");
        assert_eq!(env_concurrency_override(), Some(1));
        std::env::set_var("WHITT_MAX_CONCURRENT_INFERENCES", "0");
        assert_eq!(env_concurrency_override(), None, "0 must be rejected (floor is 1)");
        std::env::set_var("WHITT_MAX_CONCURRENT_INFERENCES", "abc");
        assert_eq!(env_concurrency_override(), None);
        std::env::remove_var("WHITT_MAX_CONCURRENT_INFERENCES");
        assert_eq!(env_concurrency_override(), None);
        if let Some(v) = saved {
            std::env::set_var("WHITT_MAX_CONCURRENT_INFERENCES", v);
        }
    }

    // --- Workflow Step Parser Tests ---

    /// Helper: build a WorkflowStep with given id + dependencies.
    fn make_step(id: &str, requires: Vec<&str>) -> WorkflowStep {
        WorkflowStep {
            step_name: id.to_string(),
            step_id: id.to_string(),
            requires: requires.into_iter().map(String::from).collect(),
            require_conditions: Default::default(),
            retry_max_attempts: None,
                    when: None,
            prompt: None,
            generative_entity: None,
            model_overrides: None,
            r#loop: None,
        }
    }

    #[test]
    fn test_topological_sort_orders_deps_first() {
        // C depends on B, B depends on A. Input order: C, A, B (wrong order).
        let steps = vec![
            make_step("C", vec!["B"]),
            make_step("A", vec![]),
            make_step("B", vec!["A"]),
        ];
        let sorted = BenchmarkRunner::topological_sort_steps(steps);
        let ids: Vec<&str> = sorted.iter().map(|s| s.step_id.as_str()).collect();
        assert_eq!(ids, vec!["A", "B", "C"],
            "Topological sort must put A before B before C");
    }

    #[test]
    fn test_topological_sort_preserves_independent_order() {
        // A, B have no deps; C depends on A. Original order should be preserved for A,B.
        let steps = vec![
            make_step("A", vec![]),
            make_step("B", vec![]),
            make_step("C", vec!["A"]),
        ];
        let sorted = BenchmarkRunner::topological_sort_steps(steps);
        let ids: Vec<&str> = sorted.iter().map(|s| s.step_id.as_str()).collect();
        let a_pos = ids.iter().position(|&x| x == "A").unwrap();
        let b_pos = ids.iter().position(|&x| x == "B").unwrap();
        let c_pos = ids.iter().position(|&x| x == "C").unwrap();
        assert!(a_pos < c_pos, "A must come before C");
        assert!(a_pos < b_pos, "Original order: A before B (both no deps)");
    }

    #[test]
    fn test_topological_sort_handles_cycle_gracefully() {
        // A → B → A (cycle). Both should still be in output (appended at end).
        let steps = vec![
            make_step("A", vec!["B"]),
            make_step("B", vec!["A"]),
        ];
        let sorted = BenchmarkRunner::topological_sort_steps(steps);
        assert_eq!(sorted.len(), 2,
            "Cycle steps must still be in output (appended, not dropped)");
    }

    #[test]
    fn test_topological_sort_handles_p11_scenario() {
        // Reproduces P11 bug: step_t1_1 depends on step_t1, but t1_1 came first in YAML.
        // Without topo sort, t1_1 would be skipped because t1 hadn't run yet.
        let steps = vec![
            make_step("step_00_bootstrap", vec![]),
            make_step("step_t1_1_analyze_immutability", vec!["step_t1_analyze_docs"]),
            make_step("step_t1_analyze_docs", vec![]),
            make_step("step_t2_identify_core_files", vec!["step_t1_1_analyze_immutability"]),
            make_step("step_final_synthesize", vec!["step_t2_identify_core_files"]),
        ];
        let sorted = BenchmarkRunner::topological_sort_steps(steps);
        let ids: Vec<&str> = sorted.iter().map(|s| s.step_id.as_str()).collect();
        let bootstrap = ids.iter().position(|&x| x == "step_00_bootstrap").unwrap();
        let t1 = ids.iter().position(|&x| x == "step_t1_analyze_docs").unwrap();
        let t1_1 = ids.iter().position(|&x| x == "step_t1_1_analyze_immutability").unwrap();
        let t2 = ids.iter().position(|&x| x == "step_t2_identify_core_files").unwrap();
        let synthesize = ids.iter().position(|&x| x == "step_final_synthesize").unwrap();
        assert!(t1 < t1_1, "t1 must come before t1_1 (was bug)");
        assert!(t1_1 < t2, "t1_1 must come before t2");
        assert!(t2 < synthesize, "t2 must come before synthesize");
        assert_eq!(bootstrap, 0, "Bootstrap with no deps should be first");
    }

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();
        assert!(steps.is_none(), "should return None when no workflow file");
    }

    // Regression: missing --workflow file must hard-error, not silently fall back
    // to discovery benchmark. Original evidence: flan-t5 loaded by accident
    // (experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:331-332,684).
    #[test]
    fn given_missing_workflow_file_when_config_loaded_then_hard_error() {
        let mut config = make_test_config();
        config.workflow_file = Some("/nonexistent/whitt-missing-workflow.yml".to_string());
        let runner = BenchmarkRunner::new(config);
        let result = runner.load_workflow_config();
        let msg = match result {
            Err(e) => e.to_string(),
            Ok(_) => panic!(
                "missing --workflow file must hard-error instead of silent discovery fallback"
            ),
        };
        assert!(
            msg.contains("whitt-missing-workflow.yml"),
            "error must name the missing file, got: {}",
            msg
        );
    }

    // Issue AE: map-format workflows must take CLI-default sampling values
    // (temperature/max_tokens) from the FIRST step in YAML document order,
    // not the alphabetically-first key (serde_json Map = BTreeMap sorted
    // keys unless preserve_order is enabled).
    #[test]
    fn given_map_format_steps_when_first_step_sampled_then_yaml_order_wins() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        let yaml = r#"
workflow_id: map_order_workflow
name: MapOrder
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
  steps:
    z_first:
      prompt: "first in document"
      model_overrides:
        temperature: 0.2
        max_tokens: 512
    a_second:
      prompt: "second in document"
      model_overrides:
        temperature: 0.9
        max_tokens: 2048
"#;
        std::fs::write(&yaml_path, yaml).unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let ctx = match runner.load_workflow_config() {
            Ok(Some(ctx)) => ctx,
            Ok(None) => panic!("workflow with valid steps must load a config"),
            Err(e) => panic!("valid map-format workflow must parse, got: {}", e),
        };
        assert_eq!(
            ctx.temperature, 0.2,
            "temperature must come from z_first (YAML-first), got {} (alphabetical pick?)",
            ctx.temperature
        );
        assert_eq!(
            ctx.max_tokens, 512,
            "max_tokens must come from z_first (YAML-first), got {} (alphabetical pick?)",
            ctx.max_tokens
        );
    }

    // Issue Z: schema documents conditional dependencies
    // (`requires: [{step: X, condition: "result.field == true"}]`,
    // docs/schema/unified-workflow-schema.yml:411-414) but the engine
    // previously dropped `condition:` at parse time and treated mere output
    // presence as satisfaction — conditional deps silently unconditional.
    #[test]
    fn given_conditional_requirement_when_steps_loaded_then_condition_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        let yaml = r#"
workflow_id: cond_dep_workflow
name: CondDep
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
  steps:
    upstream:
      prompt: "produce result"
    downstream:
      prompt: "consume result"
      requires:
        - step: upstream
          condition: "result.ready == true"
"#;
        std::fs::write(&yaml_path, yaml).unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps().unwrap().unwrap();
        let downstream = steps.iter().find(|s| s.step_id == "downstream").unwrap();
        assert_eq!(
            downstream.require_conditions.get("upstream").map(|c| c.as_str()),
            Some("result.ready == true"),
            "conditional dependency must be preserved for evaluation"
        );
    }

    // Issue AC: schema declares per-step `retry.max_attempts`
    // (step.rs StepRetryConfig; unified-workflow-schema.yml:273-276) but the
    // benchmark path never consulted it — global inference_max_attempts
    // silently applied to every step regardless of per-step config.
    #[test]
    fn given_step_retry_config_when_steps_loaded_then_max_attempts_parsed() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        let yaml = r#"
workflow_id: retry_workflow
name: Retry
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
  steps:
    fragile_step:
      prompt: "try hard"
      retry:
        max_attempts: 1
    normal_step:
      prompt: "default attempts"
"#;
        std::fs::write(&yaml_path, yaml).unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let steps = runner.load_workflow_steps().unwrap().unwrap();
        let fragile = steps.iter().find(|s| s.step_id == "fragile_step").unwrap();
        let normal = steps.iter().find(|s| s.step_id == "normal_step").unwrap();
        assert_eq!(fragile.retry_max_attempts, Some(1), "per-step retry.max_attempts must be parsed");
        assert_eq!(normal.retry_max_attempts, None, "steps without retry config keep global default");
    }

    #[test]
    fn given_step_retry_override_when_effective_attempts_computed_then_step_wins() {
        assert_eq!(
            BenchmarkRunner::effective_max_attempts(3, Some(1)),
            1,
            "step-level retry.max_attempts overrides global"
        );
        assert_eq!(
            BenchmarkRunner::effective_max_attempts(3, Some(7)),
            7,
            "step-level override may exceed global"
        );
        assert_eq!(
            BenchmarkRunner::effective_max_attempts(3, None),
            3,
            "no step config → global applies"
        );
        assert_eq!(
            BenchmarkRunner::effective_max_attempts(3, Some(0)),
            1,
            "zero attempts clamped to 1 (never zero-shot)"
        );
    }

    // LOW parking-lot fix: an array-format step missing its `step:` name key
    // used to vanish silently via filter_map — a typo dropped a whole step
    // from the workflow with no signal.
    #[test]
    fn given_array_step_missing_name_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        let yaml = r#"
workflow_id: keyless_workflow
name: Keyless
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
  steps:
    - step: named_step
      prompt: "fine"
    - prompt: "i have no step key"
"#;
        std::fs::write(&yaml_path, yaml).unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        match runner.load_workflow_steps() {
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("step 2") && msg.contains("step") && msg.to_lowercase().contains("missing"),
                    "error must name the offending entry, got: {}",
                    msg
                );
            }
            Ok(steps) => panic!(
                "keyless array step must hard-error, got Ok with {} steps",
                steps.map(|s| s.len()).unwrap_or(0)
            ),
        }
    }

    // LOW parking-lot fix: a cycle in depends_on used to drop the cyclic
    // steps silently (warn only) — the workflow ran incomplete.
    // Issue N residual: engine resolution tolerates name drift, but raw API
    // calls (e.g. shell-hook curl to /v1/models/load) need the EXACT server
    // filename — so every fuzzy-layer hit must tell the user the drift.
    #[test]
    fn given_fuzzy_model_resolution_when_drift_hint_computed_then_names_drift() {
        let some = BenchmarkRunner::model_name_drift_hint(
            "Qwen3.5-9B",
            "Qwen3-5-9B-Q4_K_M.gguf",
        );
        assert!(some.is_some(), "fuzzy match must yield a drift hint");
        let hint = some.unwrap();
        assert!(hint.contains("Qwen3.5-9B") && hint.contains("Qwen3-5-9B-Q4_K_M.gguf"));

        assert_eq!(
            BenchmarkRunner::model_name_drift_hint("m.gguf", "m.gguf"),
            None,
            "exact match needs no hint"
        );
        assert_eq!(
            BenchmarkRunner::model_name_drift_hint("m", "m.gguf"),
            None,
            "optional-.gguf suffix match counts as exact"
        );
    }

    // Issue Y: a watchdog kill mid-run previously lost ALL completed step
    // outputs (REVIEW-CYCLES.md OC-4 — "total loss per kill"). Minimal
    // checkpoint: each completed step appends {step_id, output} to
    // checkpoint.jsonl; --resume reloads it and skips completed steps.
    #[test]
    fn given_checkpoint_file_when_loaded_then_returns_last_outputs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("checkpoint.jsonl");
        std::fs::write(
            &path,
            "{\"step_id\":\"a\",\"output\":\"first\"}\nnot json at all\n{\"step_id\":\"a\",\"output\":\"second\"}\n{\"step_id\":\"b\",\"output\":\"done\"}\n",
        )
        .unwrap();
        let map = BenchmarkRunner::load_checkpoint(&path);
        assert_eq!(map.get("a").map(|s| s.as_str()), Some("second"), "duplicate step ids: last entry must win");
        assert_eq!(map.get("b").map(|s| s.as_str()), Some("done"));
        assert_eq!(map.len(), 2, "malformed lines are skipped, not fatal");

        let missing = dir.path().join("no-such-file.jsonl");
        assert!(BenchmarkRunner::load_checkpoint(&missing).is_empty(), "missing checkpoint = empty map, not error");
    }

    #[test]
    fn given_step_output_when_recorded_then_checkpoint_appended() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = make_test_config();
        config.output_dir = Some(dir.path().to_str().unwrap().to_string());
        let runner = BenchmarkRunner::new(config);
        let mut outputs = std::collections::HashMap::new();

        runner.record_step_output(&mut outputs, "gen_one", "hello");
        runner.record_step_output(&mut outputs, "gen_two", "world");

        assert_eq!(outputs.get("gen_one").map(|s| s.as_str()), Some("hello"));
        let checkpoint = dir.path().join("checkpoint.jsonl");
        let content = std::fs::read_to_string(&checkpoint)
            .expect("checkpoint file must exist after recording");
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2, "one JSON line per recorded step");
        assert!(lines[0].contains("gen_one") && lines[0].contains("hello"));
        assert!(lines[1].contains("gen_two") && lines[1].contains("world"));
    }

    #[test]
    fn given_cycle_when_cycle_error_computed_then_some() {
        let msg = BenchmarkRunner::cycle_error(3, 5)
            .expect("cycle (steps dropped by topo sort) must produce an error");
        assert!(msg.contains("cycle") && msg.contains("2"), "message must name cycle + dropped count: {}", msg);
        assert!(BenchmarkRunner::cycle_error(5, 5).is_none(), "full sort = no error");
        assert!(BenchmarkRunner::cycle_error(0, 0).is_none(), "empty = no error");
    }

    #[test]
    fn given_upstream_json_output_when_require_condition_evaluated_then_reflects_fields() {
        // satisfied: output JSON carries the field the condition needs
        assert_eq!(
            BenchmarkRunner::eval_require_condition(r#"{"ready": true}"#, "result.ready == true").unwrap(),
            true
        );
        // unsatisfied: field false → dependency NOT satisfied
        assert_eq!(
            BenchmarkRunner::eval_require_condition(r#"{"ready": false}"#, "result.ready == true").unwrap(),
            false
        );
        // non-JSON output falls back to raw string under `result.output`
        assert_eq!(
            BenchmarkRunner::eval_require_condition("plain text", "result.output == \"plain text\"").unwrap(),
            true
        );
    }

    // Issue T: --workflow provided but steps unparseable or empty must hard-error
    // at the steps-loading seam, not silently fall back to the discovery
    // benchmark. Original evidence: missing/garbage steps shape previously
    // returned None/empty and run() fell into model discovery
    // (07-TRACKING.md:331-332 — flan-t5 loaded by accident).
    #[test]
    fn given_empty_steps_array_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, "agentic_workflow:\n  steps: []\n").unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let msg = match runner.load_workflow_steps() {
            Err(e) => e.to_string(),
            Ok(_) => panic!("empty steps must hard-error, not silently fall back to discovery"),
        };
        assert!(
            msg.contains("refusing to fall back"),
            "error must refuse discovery fallback, got: {}",
            msg
        );
    }

    #[test]
    fn given_scalar_steps_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, "agentic_workflow:\n  steps: 42\n").unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let msg = match runner.load_workflow_steps() {
            Err(e) => e.to_string(),
            Ok(_) => panic!("scalar steps must hard-error, not silently fall back to discovery"),
        };
        assert!(
            msg.contains("refusing to fall back"),
            "error must refuse discovery fallback, got: {}",
            msg
        );
    }

    #[test]
    fn given_unreadable_workflow_path_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(dir.path().to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let msg = match runner.load_workflow_steps() {
            Err(e) => e.to_string(),
            Ok(_) => panic!("unreadable workflow path must hard-error, not silently fall back to discovery"),
        };
        assert!(
            msg.contains("refusing to fall back"),
            "error must refuse discovery fallback, got: {}",
            msg
        );
    }

    #[test]
    fn given_no_agentic_workflow_section_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, "some_other_key: value\n").unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let msg = match runner.load_workflow_steps() {
            Err(e) => e.to_string(),
            Ok(_) => panic!("missing agentic_workflow must hard-error, not silently fall back to discovery"),
        };
        assert!(
            msg.contains("refusing to fall back"),
            "error must refuse discovery fallback, got: {}",
            msg
        );
    }

    #[test]
    fn given_invalid_yaml_when_steps_loaded_then_hard_error() {
        let dir = tempfile::tempdir().unwrap();
        let yaml_path = dir.path().join("workflow.yml");
        std::fs::write(&yaml_path, ": : :not yaml [\n").unwrap();
        let config = BenchmarkConfig {
            workflow_file: Some(yaml_path.to_str().unwrap().to_string()),
            ..make_test_config()
        };
        let runner = BenchmarkRunner::new(config);
        let msg = match runner.load_workflow_steps() {
            Err(e) => e.to_string(),
            Ok(_) => panic!("invalid YAML must hard-error, not silently fall back to discovery"),
        };
        assert!(
            msg.contains("refusing to fall back"),
            "error must refuse discovery fallback, got: {}",
            msg
        );
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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();
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
        let steps = runner.load_workflow_steps().unwrap();

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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
            require_conditions: Default::default(),
            retry_max_attempts: None,
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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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
    fn model_source_metadata_uses_existing_explicit_path_when_models_dir_target_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("model-a.gguf");
        std::fs::write(&source, [0_u8; 17]).unwrap();

        let (path, weights_bytes) = BenchmarkRunner::model_source_metadata(
            Some("/missing-model-directory"),
            "model-a.gguf",
            source.to_str().unwrap(),
        );

        assert_eq!(path, source.display().to_string());
        assert_eq!(weights_bytes, 17);
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
        let steps = runner.load_workflow_steps().unwrap();

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
        let steps = runner.load_workflow_steps().unwrap();

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

    #[test]
    fn test_route_to_parallel_resource_contract_forces_sequential_fallback() {
        let admission: ResourceAdmissionConfig = serde_yaml::from_str(
            "enforcement_policy: block\nminimum_available:\n  ram: 6GiB\n  vram: 6GiB\n  swap_free: 4GiB\nmodel_estimate:\n  kv_cache: 2GiB\n  compute_buffer: 512MiB\n  host_runtime: 700MiB\n  expected_runtime_secs: 60\ntelemetry:\n  write_profile: true\n",
        )
        .expect("valid admission contract");

        assert!(!BenchmarkRunner::resource_admission_allows_parallel(Some(&admission)));
        assert!(BenchmarkRunner::resource_admission_allows_parallel(None));
    }

    #[tokio::test]
    async fn route_to_parallel_with_resource_contract_avoids_http() {
        let admission: ResourceAdmissionConfig = serde_yaml::from_str(
            "enforcement_policy: block\nminimum_available:\n  ram: 6GiB\n  vram: 6GiB\n  swap_free: 4GiB\nmodel_estimate:\n  kv_cache: 2GiB\n  compute_buffer: 512MiB\n  host_runtime: 700MiB\n  expected_runtime_secs: 60\ntelemetry:\n  write_profile: true\n",
        )
        .expect("valid admission contract");
        let mut runner = BenchmarkRunner::new(make_test_config());
        runner.resource_admission = Some(admission);
        let mut first = make_step("first", vec![]);
        first.generative_entity = Some("model-a.gguf".to_string());
        first.prompt = Some("first prompt".to_string());
        let mut second = make_step("second", vec![]);
        second.generative_entity = Some("model-a.gguf".to_string());
        second.prompt = Some("second prompt".to_string());
        let steps = vec![first, second];
        let targets = vec!["first".to_string(), "second".to_string()];
        let step_index = std::collections::HashMap::from([
            ("first".to_string(), 0),
            ("second".to_string(), 1),
        ]);
        let client = LlamaHttpClient::new("http://127.0.0.1:9").expect("dead test client");
        let models = vec![("model-a.gguf".to_string(), "/missing/model-a.gguf".to_string())];
        let mut step_outputs = std::collections::HashMap::new();

        let result = runner
            .try_execute_route_to_parallel(
                &targets,
                &steps,
                &step_index,
                &client,
                &None,
                &models,
                16,
                0.0,
                1.0,
                &mut step_outputs,
                0,
                true,
            )
            .await;

        assert_eq!(result, None);
        assert!(step_outputs.is_empty());
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
