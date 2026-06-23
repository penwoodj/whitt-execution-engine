//! Disk space monitoring utilities.
//!
//! Provides functions to check available disk space and ensure minimum
//! free space for operations like model downloads.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Disk space information for a path.
#[derive(Debug)]
pub struct DiskSpaceInfo {
    /// Total disk capacity in bytes.
    pub total_bytes: u64,
    /// Available free space in bytes.
    pub available_bytes: u64,
    /// Path that was queried.
    pub path: PathBuf,
}

/// Check disk space for a given path using `df -B1`.
pub fn check_disk_space(path: &Path) -> Result<DiskSpaceInfo> {
    let output = std::process::Command::new("df")
        .arg("-B1")
        .arg(path)
        .output()
        .context("Failed to execute df command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("df command failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() < 2 {
        anyhow::bail!("Unexpected df output: {}", stdout);
    }

    let data_line = lines[1];
    let parts: Vec<&str> = data_line
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() < 4 {
        anyhow::bail!("Unexpected df format: {}", data_line);
    }

    let total_bytes: u64 = parts[1]
        .parse()
        .context("Failed to parse total bytes")?;

    let available_bytes: u64 = parts[3]
        .parse()
        .context("Failed to parse available bytes")?;

    Ok(DiskSpaceInfo {
        total_bytes,
        available_bytes,
        path: path.to_path_buf(),
    })
}

/// Ensure minimum free space is available. Returns error if below threshold.
pub fn ensure_min_free_space(path: &Path, min_bytes: u64) -> Result<DiskSpaceInfo> {
    let info = check_disk_space(path)?;

    if info.available_bytes < min_bytes {
        anyhow::bail!(
            "Insufficient disk space on {}: {} bytes available, {} bytes required (minimum: {} bytes)",
            path.display(),
            info.available_bytes,
            min_bytes,
            min_bytes
        );
    }

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_disk_space_root() {
        let info = check_disk_space(Path::new("/")).expect("Failed to check root filesystem");
        assert!(info.total_bytes > 0);
        assert!(info.available_bytes > 0);
        assert_eq!(info.path, Path::new("/"));
    }

    #[test]
    fn test_ensure_min_free_space_root() {
        const MIN_BYTES: u64 = 1; // 1 byte - should always pass on root
        let info = ensure_min_free_space(Path::new("/"), MIN_BYTES)
            .expect("Root filesystem should have at least 1 byte free");
        assert!(info.available_bytes >= MIN_BYTES);
    }

    #[test]
    fn test_ensure_min_free_space_insufficient() {
        const MIN_BYTES: u64 = u64::MAX; // Impossible threshold
        let result = ensure_min_free_space(Path::new("/"), MIN_BYTES);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Insufficient disk space"));
        assert!(err_msg.contains("available"));
        assert!(err_msg.contains("required"));
    }

    #[test]
    fn test_disk_space_info_struct() {
        let info = DiskSpaceInfo {
            total_bytes: 1_000_000_000,
            available_bytes: 500_000_000,
            path: PathBuf::from("/test"),
        };
        assert_eq!(info.total_bytes, 1_000_000_000);
        assert_eq!(info.available_bytes, 500_000_000);
        assert_eq!(info.path, PathBuf::from("/test"));
    }
}
