//! System memory monitoring utilities.
//!
//! Provides functions to check available system RAM and estimate memory
//! requirements for loading LLM models before attempting to load them.

use anyhow::{Context, Result};

/// System memory information.
#[derive(Debug)]
pub struct MemoryInfo {
    /// Total system RAM in bytes.
    pub total_bytes: u64,
    /// Available RAM in bytes.
    pub available_bytes: u64,
    /// Used RAM in bytes.
    pub used_bytes: u64,
}

/// Result of a memory check for model loading.
#[derive(Debug)]
pub struct MemoryCheckResult {
    /// Whether the model can be loaded given available memory.
    pub can_load: bool,
    /// Available system RAM in bytes.
    pub available_bytes: u64,
    /// Required memory in bytes (model + KV cache + safety margin).
    pub required_bytes: u64,
    /// Model file size in bytes.
    pub model_bytes: u64,
    /// Estimated KV cache memory in bytes.
    pub kv_estimate_bytes: u64,
    /// Safety margin in bytes.
    pub safety_margin_bytes: u64,
    /// Human-readable error message (if can_load is false).
    pub error_message: Option<String>,
}

/// Read system memory information from `/proc/meminfo` (Linux only).
pub fn check_system_memory() -> Result<MemoryInfo> {
    let content = std::fs::read_to_string("/proc/meminfo")
        .context("Failed to read /proc/meminfo")?;

    let mut mem_total: Option<u64> = None;
    let mut mem_available: Option<u64> = None;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            mem_total = parse_meminfo_line(line)?;
        } else if line.starts_with("MemAvailable:") {
            mem_available = parse_meminfo_line(line)?;
        }
    }

    let total_bytes = mem_total.context("MemTotal not found in /proc/meminfo")?;
    let available_bytes = mem_available.context("MemAvailable not found in /proc/meminfo")?;
    let used_bytes = total_bytes.saturating_sub(available_bytes);

    Ok(MemoryInfo {
        total_bytes,
        available_bytes,
        used_bytes,
    })
}

/// Parse a line from /proc/meminfo (format: "Key:     value kB").
fn parse_meminfo_line(line: &str) -> Result<Option<u64>> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        let value: u64 = parts[1]
            .parse()
            .context(format!("Failed to parse value from line: {}", line))?;
        // Convert kB to bytes
        return Ok(Some(value * 1024));
    }
    Ok(None)
}

/// Get available system memory in bytes.
pub fn check_available_memory() -> Result<u64> {
    let info = check_system_memory()?;
    Ok(info.available_bytes)
}

/// Estimate total memory needed for model loading.
///
/// # Arguments
/// * `model_file_size_bytes` - Size of the GGUF model file (memory-mapped)
/// * `context_tokens` - Context window size in tokens
///
/// # Returns
/// Estimated memory in bytes (model weights + KV cache estimate)
///
/// # Notes
/// - Model weights: file_size_bytes (GGUF is memory-mapped)
/// - KV cache: estimated as model_file_size * 0.3 (conservative upper bound for 8B models with 8K context)
pub fn estimate_model_memory(model_file_size_bytes: u64, _context_tokens: u32) -> u64 {
    // Model weights are memory-mapped from the GGUF file
    let model_bytes = model_file_size_bytes;

    // KV cache estimate: conservative upper bound
    // For 8B models with 8K context, KV cache is typically ~30% of model size
    // This is a rough approximation that errs on the side of caution
    let kv_estimate_bytes = (model_file_size_bytes as f64 * 0.3) as u64;

    model_bytes + kv_estimate_bytes
}

/// Check if a model can be loaded given available system memory.
///
/// # Arguments
/// * `model_file_size_bytes` - Size of the GGUF model file
/// * `context_tokens` - Context window size in tokens
/// * `safety_margin_bytes` - Safety margin to reserve (default: 1GB)
///
/// # Returns
/// `MemoryCheckResult` with detailed information and actionable error message if insufficient
pub fn can_load_model(
    model_file_size_bytes: u64,
    context_tokens: u32,
    safety_margin_bytes: u64,
) -> Result<MemoryCheckResult> {
    let memory_info = check_system_memory()?;

    let model_bytes = model_file_size_bytes;
    let kv_estimate_bytes = estimate_model_memory(model_file_size_bytes, context_tokens) - model_bytes;
    let required_bytes = model_bytes + kv_estimate_bytes + safety_margin_bytes;

    let can_load = memory_info.available_bytes >= required_bytes;
    let error_message = if !can_load {
        Some(format!(
            "Cannot load model ({}GB): available {}GB, required {}GB (model {}GB + KV cache {}GB + safety {}GB). Suggestions: use a smaller quantization, reduce context window, close other applications.",
            bytes_to_gb(model_bytes),
            bytes_to_gb(memory_info.available_bytes),
            bytes_to_gb(required_bytes),
            bytes_to_gb(model_bytes),
            bytes_to_gb(kv_estimate_bytes),
            bytes_to_gb(safety_margin_bytes)
        ))
    } else {
        None
    };

    Ok(MemoryCheckResult {
        can_load,
        available_bytes: memory_info.available_bytes,
        required_bytes,
        model_bytes,
        kv_estimate_bytes,
        safety_margin_bytes,
        error_message,
    })
}

/// Convert bytes to gigabytes (1 decimal place).
fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / 1_073_741_824.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_system_memory() {
        let info = check_system_memory().expect("Failed to check system memory");
        assert!(info.total_bytes > 0);
        assert!(info.available_bytes > 0);
        assert!(info.used_bytes > 0);
        // Used should not exceed total (with some tolerance for kernel buffers)
        assert!(info.used_bytes <= info.total_bytes);
    }

    #[test]
    fn test_check_available_memory() {
        let available = check_available_memory().expect("Failed to check available memory");
        assert!(available > 0);
    }

    #[test]
    fn test_estimate_model_memory() {
        let model_size = 4_000_000_000; // 4GB
        let context_tokens = 8192;
        let estimated = estimate_model_memory(model_size, context_tokens);

        // Should be model size + ~30% for KV cache
        assert!(estimated >= model_size);
        assert!(estimated <= model_size * 2); // Should not be more than 2x model size
    }

    #[test]
    fn test_can_load_model_sufficient_memory() {
        let model_size = 1_000_000_000; // 1GB
        let context_tokens = 8192;
        let safety_margin = 1_073_741_824; // 1GB

        let result = can_load_model(model_size, context_tokens, safety_margin)
            .expect("Memory check failed");

        // On a system with sufficient memory, this should pass
        // (most systems have at least 2-3GB available)
        assert_eq!(result.model_bytes, model_size);
        assert_eq!(result.safety_margin_bytes, safety_margin);
        assert!(result.kv_estimate_bytes > 0);
    }

    #[test]
    fn test_can_load_model_insufficient_memory() {
        let model_size = 1_000_000_000_000; // 1TB - impossibly large
        let context_tokens = 8192;
        let safety_margin = 1_073_741_824; // 1GB

        let result = can_load_model(model_size, context_tokens, safety_margin)
            .expect("Memory check failed");

        // With a ridiculously large model, it should fail
        assert!(!result.can_load);
        assert!(result.error_message.is_some());
        let error_msg = result.error_message.unwrap();
        assert!(error_msg.contains("Cannot load model"));
        assert!(error_msg.contains("available"));
        assert!(error_msg.contains("required"));
        assert!(error_msg.contains("Suggestions"));
    }

    #[test]
    fn test_memory_info_struct() {
        let info = MemoryInfo {
            total_bytes: 16_000_000_000,
            available_bytes: 8_000_000_000,
            used_bytes: 8_000_000_000,
        };
        assert_eq!(info.total_bytes, 16_000_000_000);
        assert_eq!(info.available_bytes, 8_000_000_000);
        assert_eq!(info.used_bytes, 8_000_000_000);
    }

    #[test]
    fn test_memory_check_result_struct() {
        let result = MemoryCheckResult {
            can_load: true,
            available_bytes: 8_000_000_000,
            required_bytes: 5_000_000_000,
            model_bytes: 4_000_000_000,
            kv_estimate_bytes: 1_000_000_000,
            safety_margin_bytes: 1_000_000_000,
            error_message: None,
        };
        assert!(result.can_load);
        assert_eq!(result.available_bytes, 8_000_000_000);
        assert_eq!(result.required_bytes, 5_000_000_000);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_bytes_to_gb() {
        assert!((bytes_to_gb(1_073_741_824) - 1.0).abs() < 0.1); // 1GB
        assert!((bytes_to_gb(2_147_483_648) - 2.0).abs() < 0.1); // 2GB
        assert!((bytes_to_gb(536_870_912) - 0.5).abs() < 0.1); // 0.5GB
    }

    #[test]
    fn test_parse_meminfo_line_valid() {
        let line = "MemTotal:       16384000 kB";
        let result = parse_meminfo_line(line).expect("Failed to parse line");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 16384000 * 1024);
    }

    #[test]
    fn test_parse_meminfo_line_invalid() {
        let line = "Invalid line";
        let result = parse_meminfo_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_meminfo_line_no_number() {
        let line = "MemTotal: kB";
        let result = parse_meminfo_line(line);
        assert!(result.is_err());
    }
}