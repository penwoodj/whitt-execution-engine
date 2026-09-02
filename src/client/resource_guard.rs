//! Resource guard — admission control + pressure watchdog for local-LLM runs.
//!
//! Box boundary: pure decision logic + sensor parsing. No runner deps, no
//! HTTP, no docker. The runner (and only the runner) consumes
//! [`max_concurrent_for`], [`admit_load`], and [`PressureTracker`].
//!
//! Invariants (SAFETY.md):
//! I1  byte/GB unit math is exact (1024-based).
//! I2  an 8 GB-total GPU admits exactly ONE concurrent inference.
//! I3  weights + KV cache + compute + VRAM reserve must fit vram_free.
//! I4  host RAM floor: MemAvailable minus model footprint stays >= 2.5 GB.
//! I6  pressure transitions are hysteretic (RED latches; GREEN is earned).
//! I7  once RED, stays RED until a long clean streak resets it.

use std::path::Path;

// ── policy constants (L1/L2) ───────────────────────────────────────────────

/// Headroom kept free on the GPU at all times (GB).
pub const RESERVE_VRAM_GB: f64 = 1.5;
/// Host RAM floor below MemAvailable after accounting the model footprint (GB).
pub const RAM_FLOOR_GB: f64 = 2.5;
/// VRAM used/total fractions that trip AMBER and RED.
pub const VRAM_AMBER_FRAC: f64 = 0.80;
pub const VRAM_RED_FRAC: f64 = 0.92;
/// RAM MemAvailable/total fractions that trip AMBER and RED.
pub const RAM_AMBER_FRAC: f64 = 0.15;
pub const RAM_RED_FRAC: f64 = 0.08;
/// Sustained-sample counts for hysteresis.
pub const AMBER_STREAK: u32 = 3;
pub const GREEN_STREAK: u32 = 30;

const KB: u64 = 1024;
const GB_U64: u64 = 1024 * 1024 * 1024;
const GB_F64: f64 = 1024.0 * 1024.0 * 1024.0;

// ── sensors ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VramSensors {
    pub total_bytes: u64,
    pub free_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RamSensors {
    pub mem_available_kb: u64,
    pub swap_free_kb: u64,
    pub mem_total_kb: u64,
}

pub fn read_vram_sensors_sysfs() -> Option<VramSensors> {
    for card_index in 0..16 {
        let base = Path::new("/sys/class/drm")
            .join(format!("card{card_index}"))
            .join("device");
        if let Some(sensors) = read_vram_sensors_from_device(&base) {
            return Some(sensors);
        }
    }
    None
}

fn read_vram_sensors_from_device(base: &Path) -> Option<VramSensors> {
    let total = read_u64_file(&base.join("mem_info_vram_total"))?;
    let free = read_u64_file(&base.join("mem_info_vram_free")).or_else(|| {
        read_u64_file(&base.join("mem_info_vram_used"))
            .and_then(|used| total.checked_sub(used))
    })?;
    Some(VramSensors { total_bytes: total, free_bytes: free })
}

pub fn read_ram_sensors_proc() -> Option<RamSensors> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    parse_meminfo(&text)
}

/// Parse `/proc/meminfo` text (pure, testable).
pub fn parse_meminfo(text: &str) -> Option<RamSensors> {
    let mut mem_total_kb = None;
    let mut mem_available_kb = None;
    let mut swap_free_kb = None;
    for line in text.lines() {
        let mut it = line.split_whitespace();
        let key = it.next()?;
        let value = it.next().and_then(|v| v.parse::<u64>().ok())?;
        match key.trim_end_matches(':') {
            "MemTotal" => mem_total_kb = Some(value),
            "MemAvailable" => mem_available_kb = Some(value),
            "SwapFree" => swap_free_kb = Some(value),
            _ => {}
        }
    }
    Some(RamSensors {
        mem_total_kb: mem_total_kb?,
        mem_available_kb: mem_available_kb?,
        swap_free_kb: swap_free_kb.unwrap_or(0),
    })
}

fn read_u64_file(path: &Path) -> Option<u64> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

// ── unit math (I1) ─────────────────────────────────────────────────────────

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / GB_F64
}

pub fn kb_to_gb(kb: u64) -> f64 {
    kb as f64 * KB as f64 / GB_F64
}

// ── L1: concurrency + admission ────────────────────────────────────────────

/// I2: safe concurrent-inference count for a GPU of `vram_total_gb`.
///
/// RX 580 8 GB crashed repeatedly under the old `total/2` heuristic; an
/// 8 GB card admits exactly one inference. Growth resumes above 8 GB and
/// caps at four regardless of card size.
pub fn max_concurrent_for(vram_total_gb: f64) -> usize {
    if vram_total_gb <= 8.0 {
        1
    } else {
        1 + ((vram_total_gb - 8.0) / 4.0) as usize
    }
    .clamp(1, 4)
}

/// KV cache bytes for a GQA model (I3 math).
///
/// kv = 2 (K and V) * layers * kv_heads * head_dim * bytes_per_element * ctx
pub fn kv_cache_bytes(layers: u64, kv_heads: u64, head_dim: u64,
                      ctx_tokens: u64, bytes_per_element: u64) -> u64 {
    2 * layers * kv_heads * head_dim * bytes_per_element * ctx_tokens
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadRequest {
    pub model_name: String,
    pub weights_bytes: u64,
    pub kv_bytes: u64,
    pub compute_buffer_bytes: u64,
    pub ctx_tokens: u64,
    /// Host-RAM footprint of the running stack (llama-server + engine) in KB.
    pub host_footprint_kb: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowLoadRequest {
    pub model_name: String,
    pub weights_bytes: u64,
    pub minimum_ram_bytes: u64,
    pub minimum_vram_bytes: u64,
    pub minimum_swap_free_bytes: u64,
    pub kv_cache_bytes: u64,
    pub compute_buffer_bytes: u64,
    pub host_runtime_bytes: u64,
}

/// Greppable rejection codes (crash-debug skill contract).
pub const REJECT_VRAM_FIT: &str = "RESOURCE_REJECT_VRAM_FIT";
pub const REJECT_RAM_FLOOR: &str = "RESOURCE_REJECT_RAM_FLOOR";
pub const REJECT_WORKFLOW_RAM: &str = "RESOURCE_REJECT_WORKFLOW_RAM";
pub const REJECT_WORKFLOW_VRAM: &str = "RESOURCE_REJECT_WORKFLOW_VRAM";
pub const REJECT_WORKFLOW_SWAP: &str = "RESOURCE_REJECT_WORKFLOW_SWAP";
pub const REJECT_WORKFLOW_SENSORS: &str = "RESOURCE_REJECT_WORKFLOW_SENSORS";
pub const REJECT_WORKFLOW_MODEL_FIT: &str = "RESOURCE_REJECT_WORKFLOW_MODEL_FIT";

#[derive(Debug, Clone, PartialEq)]
pub enum Admission {
    Admitted,
    Rejected { code: &'static str, detail: String },
}

/// I3 + I4: admit a model load against current sensors.
pub fn admit_load(req: &LoadRequest, vram_free_bytes: u64, ram_sensors: &RamSensors) -> Admission {
    let reserve_bytes = (RESERVE_VRAM_GB * GB_F64) as u64;
    let need = req.weights_bytes
        .saturating_add(req.kv_bytes)
        .saturating_add(req.compute_buffer_bytes)
        .saturating_add(reserve_bytes);
    if need > vram_free_bytes {
        return Admission::Rejected {
            code: REJECT_VRAM_FIT,
            detail: format!(
                "model {} needs weights {} + kv {} + compute {} + reserve {} = {} bytes, vram free {}",
                req.model_name, req.weights_bytes, req.kv_bytes,
                req.compute_buffer_bytes, reserve_bytes, need, vram_free_bytes
            ),
        };
    }
    let floor_kb = (RAM_FLOOR_GB * GB_F64 / KB as f64) as u64;
    let ram_left_kb = ram_sensors.mem_available_kb.saturating_sub(req.host_footprint_kb);
    if ram_left_kb < floor_kb {
        return Admission::Rejected {
            code: REJECT_RAM_FLOOR,
            detail: format!(
                "model {} leaves {} MB host RAM, floor {} MB",
                req.model_name, ram_left_kb / KB, floor_kb / KB
            ),
        };
    }
    Admission::Admitted
}

pub fn admit_workflow_load(
    req: &WorkflowLoadRequest,
    vram: &VramSensors,
    ram: &RamSensors,
) -> Admission {
    let ram_available_bytes = ram.mem_available_kb.saturating_mul(KB);
    if ram_available_bytes < req.minimum_ram_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_RAM,
            detail: format!(
                "model {} requires {} bytes RAM available, found {}",
                req.model_name, req.minimum_ram_bytes, ram_available_bytes
            ),
        };
    }
    if vram.free_bytes < req.minimum_vram_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_VRAM,
            detail: format!(
                "model {} requires {} bytes VRAM free, found {}",
                req.model_name, req.minimum_vram_bytes, vram.free_bytes
            ),
        };
    }
    let swap_free_bytes = ram.swap_free_kb.saturating_mul(KB);
    if swap_free_bytes < req.minimum_swap_free_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_SWAP,
            detail: format!(
                "model {} requires {} bytes swap free, found {}",
                req.model_name, req.minimum_swap_free_bytes, swap_free_bytes
            ),
        };
    }
    let host_footprint_kb = req
        .weights_bytes
        .saturating_add(req.host_runtime_bytes)
        .saturating_add(KB - 1)
        / KB;
    match admit_load(
        &LoadRequest {
            model_name: req.model_name.clone(),
            weights_bytes: req.weights_bytes,
            kv_bytes: req.kv_cache_bytes,
            compute_buffer_bytes: req.compute_buffer_bytes,
            ctx_tokens: 0,
            host_footprint_kb,
        },
        vram.free_bytes,
        ram,
    ) {
        Admission::Admitted => Admission::Admitted,
        Admission::Rejected { detail, .. } => Admission::Rejected {
            code: REJECT_WORKFLOW_MODEL_FIT,
            detail,
        },
    }
}

pub fn admit_workflow_preflight(
    req: &WorkflowLoadRequest,
    vram: &VramSensors,
    ram: &RamSensors,
) -> Admission {
    let ram_available_bytes = ram.mem_available_kb.saturating_mul(KB);
    if ram_available_bytes < req.minimum_ram_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_RAM,
            detail: format!(
                "model {} requires {} bytes RAM available, found {}",
                req.model_name, req.minimum_ram_bytes, ram_available_bytes
            ),
        };
    }
    if vram.total_bytes < req.minimum_vram_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_VRAM,
            detail: format!(
                "model {} requires {} bytes total VRAM, found {}",
                req.model_name, req.minimum_vram_bytes, vram.total_bytes
            ),
        };
    }
    let swap_free_bytes = ram.swap_free_kb.saturating_mul(KB);
    if swap_free_bytes < req.minimum_swap_free_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_SWAP,
            detail: format!(
                "model {} requires {} bytes swap free, found {}",
                req.model_name, req.minimum_swap_free_bytes, swap_free_bytes
            ),
        };
    }
    Admission::Admitted
}

pub fn admit_workflow_inference(
    req: &WorkflowLoadRequest,
    vram: &VramSensors,
    ram: &RamSensors,
) -> Admission {
    let ram_available_bytes = ram.mem_available_kb.saturating_mul(KB);
    if ram_available_bytes < req.minimum_ram_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_RAM,
            detail: format!(
                "model {} requires {} bytes RAM available, found {}",
                req.model_name, req.minimum_ram_bytes, ram_available_bytes
            ),
        };
    }
    let swap_free_bytes = ram.swap_free_kb.saturating_mul(KB);
    if swap_free_bytes < req.minimum_swap_free_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_SWAP,
            detail: format!(
                "model {} requires {} bytes swap free, found {}",
                req.model_name, req.minimum_swap_free_bytes, swap_free_bytes
            ),
        };
    }
    if vram.free_bytes < req.compute_buffer_bytes {
        return Admission::Rejected {
            code: REJECT_WORKFLOW_VRAM,
            detail: format!(
                "model {} requires {} bytes inference VRAM headroom, found {}",
                req.model_name, req.compute_buffer_bytes, vram.free_bytes
            ),
        };
    }
    Admission::Admitted
}

// ── L2: pressure hysteresis ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pressure {
    Green,
    Amber,
    Red,
}

/// Classify one sensor sample into a raw pressure level (pre-hysteresis).
pub fn classify_sample(vram: &VramSensors, ram: &RamSensors) -> Pressure {
    let vram_used_frac = 1.0 - (vram.free_bytes as f64 / vram.total_bytes.max(1) as f64);
    let ram_avail_frac = ram.mem_available_kb as f64 / ram.mem_total_kb.max(1) as f64;
    if vram_used_frac >= VRAM_RED_FRAC || ram_avail_frac <= RAM_RED_FRAC {
        Pressure::Red
    } else if vram_used_frac >= VRAM_AMBER_FRAC || ram_avail_frac <= RAM_AMBER_FRAC {
        Pressure::Amber
    } else {
        Pressure::Green
    }
}

/// I6/I7: hysteretic pressure tracker. RED latches; GREEN must be earned
/// by `GREEN_STREAK` consecutive green samples.
#[derive(Debug)]
pub struct PressureTracker {
    state: Pressure,
    amber_streak: u32,
    green_streak: u32,
}

impl PressureTracker {
    pub fn new() -> Self {
        Self { state: Pressure::Green, amber_streak: 0, green_streak: 0 }
    }

    pub fn state(&self) -> Pressure {
        self.state
    }

    /// Feed one raw sample; returns the (possibly unchanged) state.
    pub fn transition(&mut self, raw: Pressure) -> Pressure {
        match raw {
            Pressure::Red => {
                self.state = Pressure::Red;
                self.amber_streak = 0;
                self.green_streak = 0;
            }
            Pressure::Amber => {
                self.green_streak = 0;
                self.amber_streak += 1;
                if self.amber_streak >= AMBER_STREAK && self.state == Pressure::Green {
                    self.state = Pressure::Amber;
                }
            }
            Pressure::Green => {
                self.amber_streak = 0;
                if self.state == Pressure::Red {
                    self.green_streak += 1;
                    if self.green_streak >= GREEN_STREAK {
                        self.state = Pressure::Green;
                        self.green_streak = 0;
                    }
                }
            }
        }
        self.state
    }
}

impl Default for PressureTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ── E2: in-flight inference cancellation ────────────────────────────────────

/// Wraps an inference future with a pressure watchdog. Every `interval_ms`
/// the tracker is fed a fresh sample; a RED gate cancels the future (drop)
/// and returns the block reason. Missing sensors never block (headless-safe).
pub async fn guard_inference<F, T>(
    tracker: std::sync::Arc<std::sync::Mutex<PressureTracker>>,
    interval_ms: u64,
    sample: impl Fn() -> (Option<VramSensors>, Option<RamSensors>),
    fut: F,
) -> Result<T, String>
where
    F: std::future::Future<Output = T>,
{
    use tokio::time::{MissedTickBehavior, Duration, Interval};

    let mut ticker: Interval = tokio::time::interval(Duration::from_millis(interval_ms));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    ticker.tick().await;
    let mut fut = Box::pin(fut);
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let (vram, ram) = sample();
                let mut tracker = tracker.lock().unwrap();
                if let GateAction::Block(reason) = step_gate(&mut tracker, vram, ram) {
                    return Err(reason);
                }
            }
            out = &mut fut => return Ok(out),
        }
    }
}

// ── E1: between-step gate action ────────────────────────────────────────────

/// Greppable log token emitted when the gate blocks inference.
pub const RESOURCE_CRITICAL: &str = "RESOURCE_CRITICAL";
/// Greppable log token for amber-pressure warnings.
pub const RESOURCE_PRESSURE: &str = "RESOURCE_PRESSURE";

#[derive(Debug, Clone, PartialEq)]
pub enum GateAction {
    Proceed,
    Warn,
    Block(String),
}

/// E1 decision: what the runner does before starting an inference.
pub fn gate_action(p: Pressure) -> GateAction {
    match p {
        Pressure::Green => GateAction::Proceed,
        Pressure::Amber => GateAction::Warn,
        Pressure::Red => GateAction::Block(format!(
            "{}: host under resource pressure (latched RED); skipping inference. \
             Free VRAM/RAM or close desktop apps, then re-run.",
            RESOURCE_CRITICAL
        )),
    }
}

// ── L5: crash-signature scan of llama-server logs ──────────────────────────

const VK_CRASH_SIGNATURES: &[&str] = &[
    "vk::DeviceLost",
    "ErrorDeviceLost",
    "amdgpu_vm_validate",
    "Not enough memory for command submission",
];

/// I8: return the crash signatures present in a llama-server log excerpt.
pub fn vk_crash_signatures(log: &str) -> Vec<&'static str> {
    VK_CRASH_SIGNATURES
        .iter()
        .copied()
        .filter(|sig| log.contains(sig))
        .collect()
}

// ── L1: RAM-only concurrency fallback ──────────────────────────────────────

/// Fallback concurrency when no VRAM sensor exists: half the available
/// RAM in GB, floored at 1, capped at 4.
pub fn max_concurrent_ram(available_gb: f64) -> usize {
    let n = (available_gb / 2.0).floor() as usize;
    n.clamp(1, 4)
}

// ── E1 runner seam ─────────────────────────────────────────────────────────

/// L1 pre-load admission for the runner's load path: weights from the model
/// file, KV estimated with a conservative GQA shape (Qwen-class 36L/8H/128D,
/// q8 KV, 30k ctx), 512MB compute buffer, host footprint = weights (mmap) +
/// 700MB llama-server runtime.
pub fn admission_for_load(
    model_id: &str,
    weights_bytes: u64,
    vram_free_bytes: u64,
    ram: &RamSensors,
) -> Admission {
    admit_load(
        &LoadRequest {
            model_name: model_id.to_string(),
            weights_bytes,
            kv_bytes: kv_cache_bytes(36, 8, 128, 30_000, 1),
            compute_buffer_bytes: 512 * 1024 * 1024,
            ctx_tokens: 30_000,
            host_footprint_kb: weights_bytes / 1024 + 700 * 1024,
        },
        vram_free_bytes,
        ram,
    )
}

/// One live gate decision: feed a sensor sample through the tracker and
/// return the action the runner must take before starting an inference.
/// Missing sensors (headless/CI boxes) never gate.
pub fn step_gate(
    tracker: &mut PressureTracker,
    vram: Option<VramSensors>,
    ram: Option<RamSensors>,
) -> GateAction {
    match (vram, ram) {
        (Some(v), Some(r)) => gate_action(tracker.transition(classify_sample(&v, &r))),
        _ => GateAction::Proceed,
    }
}

// ── tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // I1 units
    #[test]
    fn t01_bytes_to_gb_exact() {
        assert!((bytes_to_gb(8 * 1024 * 1024 * 1024) - 8.0).abs() < 1e-9);
        assert!((bytes_to_gb(0) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn t02_kb_to_gb_exact() {
        assert!((kb_to_gb(2_097_152) - 2.0).abs() < 1e-9);
    }

    // I2 concurrency
    #[test]
    fn t03_eight_gb_gpu_admits_exactly_one() {
        assert_eq!(max_concurrent_for(8.0), 1);
    }

    #[test]
    fn t04_small_gpu_clamps_to_one() {
        assert_eq!(max_concurrent_for(4.0), 1);
        assert_eq!(max_concurrent_for(6.5), 1);
    }

    #[test]
    fn t05_big_gpu_grows_but_caps_at_four() {
        assert_eq!(max_concurrent_for(12.0), 2);
        assert_eq!(max_concurrent_for(24.0), 4);
        assert_eq!(max_concurrent_for(64.0), 4);
    }

    // I3 KV math — Qwen3-4B class: 36 layers, 8 kv heads, 128 head dim, q8 KV.
    #[test]
    fn t06_kv_cache_bytes_qwen3_4b_ctx30k() {
        // 2 * 36 * 8 * 128 * 1 * 30000 = 2_211_840_000 bytes (~2.06 GiB)
        assert_eq!(kv_cache_bytes(36, 8, 128, 30_000, 1), 2_211_840_000);
    }

    #[test]
    fn t07_kv_cache_bytes_f16_doubles() {
        assert_eq!(kv_cache_bytes(36, 8, 128, 30_000, 2), 2 * 2_211_840_000);
    }

    fn ram(avail_kb: u64) -> RamSensors {
        RamSensors { mem_available_kb: avail_kb, swap_free_kb: 0, mem_total_kb: 16_000_000 }
    }

    fn req(weights: u64, kv: u64) -> LoadRequest {
        LoadRequest {
            model_name: "m".into(),
            weights_bytes: weights,
            kv_bytes: kv,
            compute_buffer_bytes: 512 * 1024 * 1024,
            ctx_tokens: 30_000,
            host_footprint_kb: 1_500_000,
        }
    }

    // I3 vram fit — 8GB card: 2.5G weights + 2.06G kv + 0.5G compute
    // + 1.5G reserve = 6.56G <= 8.0G free ⇒ admitted when VRAM empty.
    #[test]
    fn t08_admit_fits_empty_eight_gb_card() {
        let free = 8 * 1024 * 1024 * 1024u64;
        assert_eq!(admit_load(&req(2_684_354_560, 2_211_840_000), free, &ram(6_000_000)),
                   Admission::Admitted);
    }

    // Same load when Storybook already ate 4G of VRAM ⇒ only 4G free ⇒ reject.
    #[test]
    fn t09_reject_when_storybook_ate_vram() {
        let free = 4 * 1024 * 1024 * 1024u64;
        match admit_load(&req(2_684_354_560, 2_211_840_000), free, &ram(6_000_000)) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_VRAM_FIT),
            other => panic!("expected reject, got {:?}", other),
        }
    }

    // The -c 100000 class: KV alone (6.9G) + weights does not fit 8G even empty.
    #[test]
    fn t10_reject_hundred_k_ctx_on_eight_gb() {
        let free = 8 * 1024 * 1024 * 1024u64;
        let kv = kv_cache_bytes(36, 8, 128, 100_000, 1);
        match admit_load(&req(2_684_354_560, kv), free, &ram(6_000_000)) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_VRAM_FIT),
            other => panic!("expected reject, got {:?}", other),
        }
    }

    // I4 RAM floor: MemAvailable 3.5G, footprint 1.5G ⇒ 2.0G left < 2.5G floor.
    #[test]
    fn t11_reject_when_ram_floor_broken() {
        let free = 8 * 1024 * 1024 * 1024u64;
        match admit_load(&req(2_684_354_560, 2_211_840_000), free, &ram(3_600_000)) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_RAM_FLOOR),
            other => panic!("expected reject, got {:?}", other),
        }
    }

    // I5 classification
    #[test]
    fn t12_classify_green_when_healthy() {
        let vram = VramSensors { total_bytes: 8 * KB * KB * KB, free_bytes: 6 * KB * KB * KB };
        let r = ram(8_000_000);
        assert_eq!(classify_sample(&vram, &r), Pressure::Green);
    }

    #[test]
    fn t13_classify_red_when_vram_critical() {
        // used/total = 7.6/8 = 0.95 ≥ 0.92
        let vram = VramSensors { total_bytes: 8 * KB * KB * KB, free_bytes: 410 * MB() };
        let r = ram(8_000_000);
        assert_eq!(classify_sample(&vram, &r), Pressure::Red);
    }

    #[test]
    fn t14_classify_red_when_ram_critical() {
        let vram = VramSensors { total_bytes: 8 * KB * KB * KB, free_bytes: 6 * KB * KB * KB };
        // available/total = 1.0/16.0 = 0.0625 ≤ 0.08
        let r = RamSensors { mem_available_kb: 1_000_000, swap_free_kb: 0, mem_total_kb: 16_000_000 };
        assert_eq!(classify_sample(&vram, &r), Pressure::Red);
    }

    #[test]
    fn t15_classify_amber_between() {
        // used/total = 6.6/8 = 0.825 ≥ 0.80, < 0.92
        let vram = VramSensors { total_bytes: 8 * KB * KB * KB, free_bytes: 1_400 * MB() };
        let r = ram(8_000_000);
        assert_eq!(classify_sample(&vram, &r), Pressure::Amber);
    }

    #[test]
    fn t44_reads_vram_from_non_card0_device_path() {
        let device = std::env::temp_dir().join(format!(
            "whitt-vram-sensor-{}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&device).expect("create test sensor path");
        std::fs::write(device.join("mem_info_vram_total"), "8589934592\n")
            .expect("write total VRAM");
        std::fs::write(device.join("mem_info_vram_free"), "6442450944\n")
            .expect("write free VRAM");

        assert_eq!(
            read_vram_sensors_from_device(&device),
            Some(VramSensors {
                total_bytes: 8 * 1024 * 1024 * 1024,
                free_bytes: 6 * 1024 * 1024 * 1024,
            }),
        );

        std::fs::remove_dir_all(device).expect("remove test sensor path");
    }

    #[test]
    fn t45_derives_free_vram_from_total_and_used() {
        let device = std::env::temp_dir().join(format!(
            "whitt-vram-used-sensor-{}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&device).expect("create test sensor path");
        std::fs::write(device.join("mem_info_vram_total"), "8589934592\n")
            .expect("write total VRAM");
        std::fs::write(device.join("mem_info_vram_used"), "2147483648\n")
            .expect("write used VRAM");

        assert_eq!(
            read_vram_sensors_from_device(&device),
            Some(VramSensors {
                total_bytes: 8 * 1024 * 1024 * 1024,
                free_bytes: 6 * 1024 * 1024 * 1024,
            }),
        );

        std::fs::remove_dir_all(device).expect("remove test sensor path");
    }

    fn MB() -> u64 {
        1024 * 1024
    }

    // I6/I7 hysteresis
    #[test]
    fn t16_amber_needs_three_sustained_samples() {
        let mut t = PressureTracker::new();
        assert_eq!(t.transition(Pressure::Amber), Pressure::Green);
        assert_eq!(t.transition(Pressure::Amber), Pressure::Green);
        assert_eq!(t.transition(Pressure::Amber), Pressure::Amber);
    }

    #[test]
    fn t17_amber_streak_resets_on_green() {
        let mut t = PressureTracker::new();
        t.transition(Pressure::Amber);
        t.transition(Pressure::Amber);
        assert_eq!(t.transition(Pressure::Green), Pressure::Green);
        assert_eq!(t.transition(Pressure::Amber), Pressure::Green);
        assert_eq!(t.transition(Pressure::Amber), Pressure::Green);
        assert_eq!(t.transition(Pressure::Amber), Pressure::Amber);
    }

    #[test]
    fn t18_red_fires_immediately_and_latches() {
        let mut t = PressureTracker::new();
        assert_eq!(t.transition(Pressure::Red), Pressure::Red);
        for _ in 0..10 {
            assert_eq!(t.transition(Pressure::Green), Pressure::Red, "RED must latch");
        }
    }

    #[test]
    fn t19_red_clears_only_after_long_clean_streak() {
        let mut t = PressureTracker::new();
        t.transition(Pressure::Red);
        for _ in 0..(GREEN_STREAK - 1) {
            assert_eq!(t.transition(Pressure::Green), Pressure::Red);
        }
        assert_eq!(t.transition(Pressure::Green), Pressure::Green);
    }

    #[test]
    fn t20_single_amber_spike_does_not_flap_state() {
        let mut t = PressureTracker::new();
        for _ in 0..100 {
            t.transition(Pressure::Green);
        }
        assert_eq!(t.transition(Pressure::Amber), Pressure::Green);
    }

    // sensor parsing
    #[test]
    fn t21_parse_meminfo_realistic() {
        let text = "MemTotal:       16384000 kB\n\
                    MemFree:         1234567 kB\n\
                    MemAvailable:    6111000 kB\n\
                    Buffers:          123456 kB\n\
                    SwapTotal:             0 kB\n\
                    SwapFree:              0 kB\n";
        let s = parse_meminfo(text).unwrap();
        assert_eq!(s.mem_total_kb, 16_384_000);
        assert_eq!(s.mem_available_kb, 6_111_000);
        assert_eq!(s.swap_free_kb, 0);
    }

    #[test]
    fn t22_parse_meminfo_missing_available_is_none() {
        let text = "MemTotal:       16384000 kB\n";
        assert!(parse_meminfo(text).is_none());
    }

    // ── E1 gate action ──────────────────────────────────────────────

    #[test]
    fn t23_gate_green_proceeds() {
        assert!(matches!(gate_action(Pressure::Green), GateAction::Proceed));
    }

    #[test]
    fn t24_gate_amber_warns_but_proceeds() {
        assert!(matches!(gate_action(Pressure::Amber), GateAction::Warn));
    }

    #[test]
    fn t25_gate_red_blocks_with_greppable_token() {
        match gate_action(Pressure::Red) {
            GateAction::Block(reason) => {
                assert!(reason.contains(RESOURCE_CRITICAL), "must carry token: {reason}");
            }
            other => panic!("expected Block, got {other:?}"),
        }
    }

    // ── L5 vk crash signature scan ──────────────────────────────────

    #[test]
    fn t26_vk_scan_clean_log_is_empty() {
        assert!(vk_crash_signatures("slot update_slots: id 0\nsrv: done request: 200\n").is_empty());
    }

    #[test]
    fn t27_vk_scan_finds_device_lost() {
        let log = "srv  log_server_r: gor error: vk::DeviceLostError\nsomething\n";
        let hits = vk_crash_signatures(log);
        assert_eq!(hits.len(), 1);
        assert!(hits[0].contains("vk::DeviceLost"));
    }

    #[test]
    fn t28_vk_scan_finds_amdgpu_and_submission_errors() {
        let log = "[drm:amdgpu_vm_validate] *ERROR* failed\n\
                   kernel: Not enough memory for command submission\n\
                   amdgpu_vm_bo_map: ok\n";
        let hits = vk_crash_signatures(log);
        assert!(hits.len() >= 2, "expected >=2 hits, got {hits:?}");
        assert!(hits.iter().any(|h| h.contains("amdgpu_vm_validate")));
        assert!(hits.iter().any(|h| h.contains("command submission")));
    }

    #[test]
    fn t29_vk_scan_no_false_positive_on_normal_lines() {
        let log = "amdgpu driver loaded\nvk::Instance created ok\n";
        assert!(vk_crash_signatures(log).is_empty());
    }

    // ── RAM concurrency fallback ────────────────────────────────────

    #[test]
    fn t30_max_concurrent_ram_small_floor_one() {
        assert_eq!(max_concurrent_ram(1.5), 1);
        assert_eq!(max_concurrent_ram(2.0), 1);
        assert_eq!(max_concurrent_ram(3.9), 1);
    }

    #[test]
    fn t31_max_concurrent_ram_scales_capped() {
        assert_eq!(max_concurrent_ram(4.0), 2);
        assert_eq!(max_concurrent_ram(8.0), 4);
        assert_eq!(max_concurrent_ram(64.0), 4);
    }

    // ── E1 runner seam: one live gate decision ─────────────────────

    #[test]
    fn t32_step_gate_blocks_on_red_sample() {
        let mut tracker = PressureTracker::new();
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 0 };
        let ram = RamSensors { mem_available_kb: 15_000_000, swap_free_kb: 0, mem_total_kb: 16_000_000 };
        match step_gate(&mut tracker, Some(vram), Some(ram)) {
            GateAction::Block(reason) => assert!(reason.contains(RESOURCE_CRITICAL)),
            other => panic!("expected Block, got {other:?}"),
        }
    }

    #[test]
    fn t33_step_gate_proceeds_on_green_sample() {
        let mut tracker = PressureTracker::new();
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 6 * GB_U64 };
        let ram = RamSensors { mem_available_kb: 8_000_000, swap_free_kb: 0, mem_total_kb: 16_000_000 };
        assert!(matches!(
            step_gate(&mut tracker, Some(vram), Some(ram)),
            GateAction::Proceed
        ));
    }

    #[test]
    fn t34_step_gate_proceeds_when_sensors_missing() {
        let mut tracker = PressureTracker::new();
        assert!(matches!(
            step_gate(&mut tracker, None, None),
            GateAction::Proceed
        ));
    }

    // ── E2: in-flight cancellation ──────────────────────────────────

    fn green_sample() -> (Option<VramSensors>, Option<RamSensors>) {
        (Some(VramSensors { total_bytes: 8 * GB_U64, free_bytes: 6 * GB_U64 }),
         Some(RamSensors { mem_available_kb: 8_000_000, swap_free_kb: 0, mem_total_kb: 16_000_000 }))
    }

    fn red_sample() -> (Option<VramSensors>, Option<RamSensors>) {
        (Some(VramSensors { total_bytes: 8 * GB_U64, free_bytes: 0 }),
         Some(RamSensors { mem_available_kb: 1_000_000, swap_free_kb: 0, mem_total_kb: 16_000_000 }))
    }

    #[tokio::test]
    async fn t35_guard_inference_green_completes() {
        let tracker = std::sync::Arc::new(std::sync::Mutex::new(PressureTracker::new()));
        let out = guard_inference(tracker, 10, green_sample, async { 41 + 1 }).await;
        assert_eq!(out.unwrap(), 42);
    }

    #[tokio::test]
    async fn t36_guard_inference_red_cancels_before_body_runs() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let tracker = std::sync::Arc::new(std::sync::Mutex::new(PressureTracker::new()));
        let ran = std::sync::Arc::new(AtomicBool::new(false));
        let flag = ran.clone();
        let out = guard_inference(tracker, 10, red_sample, async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            flag.store(true, Ordering::SeqCst);
            "done"
        })
        .await;
        match out {
            Err(reason) => {
                assert!(reason.contains(RESOURCE_CRITICAL), "reason={reason}");
                assert!(!ran.load(Ordering::SeqCst), "future body ran despite cancel");
            }
            Ok(v) => panic!("expected Err, got Ok({v})"),
        }
    }

    #[tokio::test]
    async fn t37_guard_inference_missing_sensors_proceeds() {
        let tracker = std::sync::Arc::new(std::sync::Mutex::new(PressureTracker::new()));
        let out = guard_inference(tracker, 10, || (None, None), async { "ok" }).await;
        assert_eq!(out.unwrap(), "ok");
    }

    // ── L1 load-admission wrapper ───────────────────────────────────

    #[test]
    fn t38_admission_for_load_admits_roomy_host() {
        let free = 8 * GB_U64;
        match admission_for_load("m", 2_684_354_560, free, &ram(6_000_000)) {
            Admission::Admitted => {}
            other => panic!("expected Admitted, got {other:?}"),
        }
    }

    #[test]
    fn t39_admission_for_load_rejects_storybook_squeeze() {
        let free = 4 * GB_U64;
        match admission_for_load("m", 2_684_354_560, free, &ram(6_000_000)) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_VRAM_FIT),
            other => panic!("expected Reject, got {other:?}"),
        }
    }

    fn workflow_req() -> WorkflowLoadRequest {
        WorkflowLoadRequest {
            model_name: "m".into(),
            weights_bytes: GB_U64,
            minimum_ram_bytes: 6 * GB_U64,
            minimum_vram_bytes: 6 * GB_U64,
            minimum_swap_free_bytes: 4 * GB_U64,
            kv_cache_bytes: GB_U64,
            compute_buffer_bytes: 512 * 1024 * 1024,
            host_runtime_bytes: 700 * 1024 * 1024,
        }
    }

    #[test]
    fn t40_workflow_admission_rejects_declared_available_resource_minimums() {
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 8 * GB_U64 };
        let ram = RamSensors {
            mem_available_kb: 5 * 1024 * 1024,
            swap_free_kb: 5 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match admit_workflow_load(&workflow_req(), &vram, &ram) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_WORKFLOW_RAM),
            other => panic!("expected declared-RAM rejection, got {other:?}"),
        }
    }

    #[test]
    fn t41_workflow_admission_rejects_declared_model_fit() {
        let mut request = workflow_req();
        request.minimum_ram_bytes = 2 * GB_U64;
        request.minimum_vram_bytes = 2 * GB_U64;
        request.minimum_swap_free_bytes = GB_U64;
        request.weights_bytes = 4 * GB_U64;
        request.kv_cache_bytes = 4 * GB_U64;
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 8 * GB_U64 };
        let ram = RamSensors {
            mem_available_kb: 12 * 1024 * 1024,
            swap_free_kb: 12 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match admit_workflow_load(&request, &vram, &ram) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_WORKFLOW_MODEL_FIT),
            other => panic!("expected declared-model-fit rejection, got {other:?}"),
        }
    }

    #[test]
    fn t42_workflow_admission_rejects_declared_vram_minimum() {
        let mut request = workflow_req();
        request.minimum_ram_bytes = 2 * GB_U64;
        request.minimum_vram_bytes = 7 * GB_U64;
        request.minimum_swap_free_bytes = GB_U64;
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 6 * GB_U64 };
        let ram = RamSensors {
            mem_available_kb: 12 * 1024 * 1024,
            swap_free_kb: 12 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match admit_workflow_load(&request, &vram, &ram) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_WORKFLOW_VRAM),
            other => panic!("expected declared-VRAM rejection, got {other:?}"),
        }
    }

    #[test]
    fn t43_workflow_admission_rejects_declared_swap_minimum() {
        let mut request = workflow_req();
        request.minimum_ram_bytes = 2 * GB_U64;
        request.minimum_vram_bytes = 2 * GB_U64;
        request.minimum_swap_free_bytes = 4 * GB_U64;
        let vram = VramSensors { total_bytes: 8 * GB_U64, free_bytes: 8 * GB_U64 };
        let ram = RamSensors {
            mem_available_kb: 12 * 1024 * 1024,
            swap_free_kb: 3 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };
        match admit_workflow_load(&request, &vram, &ram) {
            Admission::Rejected { code, .. } => assert_eq!(code, REJECT_WORKFLOW_SWAP),
            other => panic!("expected declared-swap rejection, got {other:?}"),
        }
    }

    #[test]
    fn t46_workflow_inference_admission_uses_compute_headroom_after_load() {
        let request = workflow_req();
        let vram = VramSensors {
            total_bytes: 8 * GB_U64,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(matches!(
            admit_workflow_inference(&request, &vram, &ram),
            Admission::Admitted
        ));
    }

    #[test]
    fn workflow_preflight_admits_total_vram_capacity_when_model_is_already_loaded() {
        let request = workflow_req();
        let vram = VramSensors {
            total_bytes: 8 * GB_U64,
            free_bytes: 712 * 1024 * 1024,
        };
        let ram = RamSensors {
            mem_available_kb: 8 * 1024 * 1024,
            swap_free_kb: 8 * 1024 * 1024,
            mem_total_kb: 16 * 1024 * 1024,
        };

        assert!(matches!(
            admit_workflow_preflight(&request, &vram, &ram),
            Admission::Admitted
        ));
    }
}
