use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileOperation {
    Read,
    Write,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub allowed_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub allowed_patterns: Vec<String>,
    pub max_file_size_mb: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            allowed_paths: vec![".".to_string()], // Current directory by default
            forbidden_paths: Vec::new(),
            allowed_patterns: Vec::new(),
            max_file_size_mb: 10,
        }
    }
}

pub struct ToolSandbox {
    allowed_paths: Vec<PathBuf>,
    forbidden_paths: Vec<PathBuf>,
    allowed_patterns: Vec<String>,
    max_file_size_bytes: u64,
}

impl ToolSandbox {
    pub fn new(config: SandboxConfig) -> Self {
        debug!("Creating ToolSandbox with {} allowed paths", config.allowed_paths.len());

        let allowed_paths: Vec<PathBuf> = config
            .allowed_paths
            .into_iter()
            .filter_map(|p| std::fs::canonicalize(&p).ok())
            .collect();

        let forbidden_paths: Vec<PathBuf> = config
            .forbidden_paths
            .into_iter()
            .map(|p| {
                if let Ok(canonical) = std::fs::canonicalize(&p) {
                    canonical
                } else {
                    PathBuf::from(p)
                }
            })
            .collect();

        let max_file_size_bytes = config.max_file_size_mb * 1024 * 1024;

        Self {
            allowed_paths,
            forbidden_paths,
            allowed_patterns: config.allowed_patterns,
            max_file_size_bytes,
        }
    }

    pub fn is_path_allowed(&self, path: &Path) -> Result<bool, anyhow::Error> {
        // Canonicalize the path for consistent comparison
        let canonical = self.canonicalize_path(path)?;

        // Check forbidden paths first (deny list takes precedence)
        for forbidden in &self.forbidden_paths {
            if canonical.starts_with(forbidden) {
                debug!("Path denied by forbidden list: {:?}", canonical);
                return Ok(false);
            }
        }

        // Check allowed paths (allow list)
        if self.allowed_paths.is_empty() {
            // If no allowed paths specified, deny all
            debug!("Path denied: no allowed paths configured");
            return Ok(false);
        }

        for allowed in &self.allowed_paths {
            if canonical.starts_with(allowed) {
                debug!("Path allowed by allow list: {:?}", canonical);
                return Ok(true);
            }
        }

        // Check allowed patterns (regex-like patterns)
        if !self.allowed_patterns.is_empty() {
            let path_str = canonical.to_string_lossy();
            for pattern in &self.allowed_patterns {
                // Simple pattern matching (supports * wildcard)
                if self.matches_pattern(&path_str, pattern) {
                    debug!("Path allowed by pattern match: {:?}", canonical);
                    return Ok(true);
                }
            }
        }

        debug!("Path denied: not in allowed paths or patterns: {:?}", canonical);
        Ok(false)
    }

    pub fn validate_file_access(
        &self,
        path: &Path,
        operation: FileOperation,
    ) -> Result<(), anyhow::Error> {
        debug!("Validating file access: {:?} for operation: {:?}", path, operation);

        // Check if path is allowed
        if !self.is_path_allowed(path)? {
            anyhow::bail!("Access denied: path '{:?}' is not in allowed paths", path);
        }

        // For read and delete operations, check if file exists
        if matches!(operation, FileOperation::Read | FileOperation::Delete) && !path.exists() {
            anyhow::bail!("File not found: {:?}", path);
        }

        // For write operations, check parent directory exists
        if matches!(operation, FileOperation::Write) {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    anyhow::bail!("Parent directory does not exist: {:?}", parent);
                }
            }
        }

        // Check file size for read operations
        if matches!(operation, FileOperation::Read) && path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                let file_size = metadata.len();
                if file_size > self.max_file_size_bytes {
                    anyhow::bail!(
                        "File too large: {} bytes (max: {} bytes)",
                        file_size,
                        self.max_file_size_bytes
                    );
                }
            }
        }

        debug!("File access validated successfully");
        Ok(())
    }

    pub fn sanitize_path(&self, path: &Path) -> Result<PathBuf, anyhow::Error> {
        debug!("Sanitizing path: {:?}", path);

        // Canonicalize the path to resolve symlinks and relative components
        let canonical = self.canonicalize_path(path)?;

        // Validate the canonicalized path
        if !self.is_path_allowed(&canonical)? {
            anyhow::bail!(
                "Sanitized path '{:?}' is not in allowed paths",
                canonical
            );
        }

        debug!("Path sanitized successfully: {:?}", canonical);
        Ok(canonical)
    }

    fn canonicalize_path(&self, path: &Path) -> Result<PathBuf, anyhow::Error> {
        // Try to canonicalize the path
        match std::fs::canonicalize(path) {
            Ok(canonical) => Ok(canonical),
            Err(_) => {
                // If canonicalization fails (e.g., path doesn't exist), 
                // try to canonicalize the parent and append the filename
                if let Some(parent) = path.parent() {
                    if let Ok(canonical_parent) = std::fs::canonicalize(parent) {
                        if let Some(filename) = path.file_name() {
                            return Ok(canonical_parent.join(filename));
                        }
                    }
                }
                
                // Last resort: return the path as-is
                Ok(path.to_path_buf())
            }
        }
    }

    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        // Simple wildcard matching (supports * and **)
        let pattern_parts: Vec<&str> = pattern.split('*').collect();
        
        if pattern_parts.len() == 1 {
            // No wildcards, exact match
            return path == pattern;
        }

        let mut path_pos = 0;
        
        for (i, part) in pattern_parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if let Some(pos) = path[path_pos..].find(part) {
                path_pos += pos + part.len();
                
                // Check for ** (match zero or more directories)
                if i > 0 && pattern_parts[i - 1].is_empty() {
                    // ** allows any number of path components
                    continue;
                }

                // Single * only matches within one path component
                if !path[..path_pos].contains('/') || !part.contains('/') {
                    continue;
                }
            } else {
                return false;
            }
        }

        // Check if remaining pattern allows anything
        if pattern.ends_with('*') {
            true
        } else {
            path_pos == path.len()
        }
    }
}

impl Default for ToolSandbox {
    fn default() -> Self {
        Self::new(SandboxConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_new() {
        let config = SandboxConfig {
            allowed_paths: vec!["/tmp".to_string()],
            forbidden_paths: vec!["/tmp/secret".to_string()],
            allowed_patterns: vec![],
            max_file_size_mb: 5,
        };

        let sandbox = ToolSandbox::new(config);
        assert_eq!(sandbox.allowed_paths.len(), 1);
        assert_eq!(sandbox.max_file_size_bytes, 5 * 1024 * 1024);
    }

    #[test]
    fn test_sanitize_path() {
        let config = SandboxConfig {
            allowed_paths: vec![".".to_string()],
            forbidden_paths: vec![],
            allowed_patterns: vec![],
            max_file_size_mb: 10,
        };

        let sandbox = ToolSandbox::new(config);

        let current_dir = std::env::current_dir().unwrap();
        let result = sandbox.sanitize_path(Path::new("."));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current_dir);
    }

    #[test]
    fn test_is_path_allowed() {
        let config = SandboxConfig {
            allowed_paths: vec!["/tmp".to_string()],
            forbidden_paths: vec!["/tmp/secret".to_string()],
            allowed_patterns: vec![],
            max_file_size_mb: 10,
        };

        let sandbox = ToolSandbox::new(config);

        // Test allowed path
        let allowed = sandbox.is_path_allowed(Path::new("/tmp/test.txt"));
        assert!(allowed.is_ok());
        assert!(allowed.unwrap());

        // Test forbidden path
        let forbidden = sandbox.is_path_allowed(Path::new("/tmp/secret/test.txt"));
        assert!(forbidden.is_ok());
        assert!(!forbidden.unwrap());

        // Test not in allowed paths
        let not_allowed = sandbox.is_path_allowed(Path::new("/var/test.txt"));
        assert!(not_allowed.is_ok());
        assert!(!not_allowed.unwrap());
    }

    #[test]
    fn test_matches_pattern() {
        let config = SandboxConfig::default();
        let sandbox = ToolSandbox::new(config);

        // Test simple wildcard
        assert!(sandbox.matches_pattern("/tmp/test.txt", "/tmp/*.txt"));
        
        // Test exact match
        assert!(sandbox.matches_pattern("/tmp/test.txt", "/tmp/test.txt"));
        
        // Test no match
        assert!(!sandbox.matches_pattern("/tmp/test.txt", "/tmp/test.log"));
    }
}
