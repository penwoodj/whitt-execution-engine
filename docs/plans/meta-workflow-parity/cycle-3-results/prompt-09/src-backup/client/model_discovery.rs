//! Model discovery for GGUF files.
//!
//! Recursively scans directories for `.gguf` model files and provides
//! metadata for model selection in benchmarking.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Information about a candidate model file.
pub struct ModelCandidate {
    /// Full path to GGUF file.
    pub path: PathBuf,
    /// Model identifier derived from filename (without .gguf extension).
    pub model_id: String,
    /// File size in bytes.
    pub file_size_bytes: u64,
    /// Estimated VRAM requirement (same as file size for GGUF).
    pub estimated_vram_bytes: u64,
    /// Whether the model fits in available VRAM budget.
    pub fits_in_vram: bool,
    /// Author/organization from parent directory name.
    pub author: String,
}

/// Discover GGUF models in a directory tree.
///
/// Returns vector of model candidates sorted by file size (smallest first).
pub fn discover_models(models_dir: &Path, max_vram_bytes: u64) -> Result<Vec<ModelCandidate>> {
    let mut candidates = Vec::new();

    for entry in WalkDir::new(models_dir).into_iter() {
        let entry = entry.context("Failed to read directory entry")?;

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let parent_dir = match path.parent() {
            Some(p) => p,
            None => continue,
        };

        let parent_name = parent_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Skip files in hidden subdirectories, but allow files directly in models_dir root
        // (tempfile creates dirs like /tmp/.tmpXXXX which would otherwise be filtered)
        let is_in_root = parent_dir == models_dir;
        if !is_in_root && parent_name.starts_with('.') {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        if extension.as_deref() != Some("gguf") {
            continue;
        }

        let file_size_bytes = std::fs::metadata(path)
            .context("Failed to get file metadata")?
            .len();

        if file_size_bytes == 0 {
            continue;
        }

        let file_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename: {:?}", path))?;

        let model_id = file_name.replace('-', " ");

        let author = parent_name.to_string();

        let fits_in_vram = file_size_bytes <= max_vram_bytes;

        candidates.push(ModelCandidate {
            path: path.to_path_buf(),
            model_id,
            file_size_bytes,
            estimated_vram_bytes: file_size_bytes,
            fits_in_vram,
            author,
        });
    }

    candidates.sort_by_key(|c| c.file_size_bytes);

    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    const MAX_VRAM_BYTES: u64 = 6_442_450_944; // 6GB

    #[test]
    fn test_model_candidate_vram_boundary() {
        let candidate_6gb = ModelCandidate {
            path: PathBuf::from("/test/model.gguf"),
            model_id: "test model".to_string(),
            file_size_bytes: MAX_VRAM_BYTES,
            estimated_vram_bytes: MAX_VRAM_BYTES,
            fits_in_vram: true,
            author: "test".to_string(),
        };
        assert!(candidate_6gb.fits_in_vram);

        let candidate_6gb_plus_1 = ModelCandidate {
            path: PathBuf::from("/test/model.gguf"),
            model_id: "test model".to_string(),
            file_size_bytes: MAX_VRAM_BYTES + 1,
            estimated_vram_bytes: MAX_VRAM_BYTES + 1,
            fits_in_vram: false,
            author: "test".to_string(),
        };
        assert!(!candidate_6gb_plus_1.fits_in_vram);
    }

    #[test]
    fn test_model_id_from_filename() {
        let candidate = ModelCandidate {
            path: PathBuf::from("/test/llama-3-2b-instruct.gguf"),
            model_id: "llama 3 2b instruct".to_string(),
            file_size_bytes: 1_000_000,
            estimated_vram_bytes: 1_000_000,
            fits_in_vram: true,
            author: "test".to_string(),
        };
        assert_eq!(candidate.model_id, "llama 3 2b instruct");
    }

    #[test]
    fn test_discover_models_filters_gguf_only() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let models_dir = temp_dir.path();

        let mut gguf_file = File::create(models_dir.join("model1.gguf")).expect("Failed to create .gguf");
        gguf_file.write_all(b"test data").expect("Failed to write");
        File::create(models_dir.join("model2.txt")).expect("Failed to create .txt");
        File::create(models_dir.join("model3.json")).expect("Failed to create .json");

        let models = discover_models(models_dir, MAX_VRAM_BYTES).expect("Failed to discover models");
        assert_eq!(models.len(), 1);
        assert!(models[0].path.ends_with("model1.gguf"));
    }

    #[test]
    fn test_discover_models_skips_hidden_dirs() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let models_dir = temp_dir.path();

        let hidden_dir = models_dir.join(".hidden");
        fs::create_dir(&hidden_dir).expect("Failed to create hidden dir");
        let mut hidden_file = File::create(hidden_dir.join("hidden-model.gguf")).expect("Failed to create hidden model");
        hidden_file.write_all(b"hidden").expect("Failed to write");

        let mut visible_file = File::create(models_dir.join("visible-model.gguf")).expect("Failed to create visible model");
        visible_file.write_all(b"visible").expect("Failed to write");

        let models = discover_models(models_dir, MAX_VRAM_BYTES).expect("Failed to discover models");
        assert_eq!(models.len(), 1);
        assert!(models[0].path.ends_with("visible-model.gguf"));
    }

    #[test]
    fn test_discover_models_skips_zero_bytes() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let models_dir = temp_dir.path();

        File::create(models_dir.join("empty.gguf")).expect("Failed to create empty file");

        let mut file = File::create(models_dir.join("valid.gguf")).expect("Failed to create file");
        use std::io::Write;
        file.write_all(b"test").expect("Failed to write");

        let models = discover_models(models_dir, MAX_VRAM_BYTES).expect("Failed to discover models");
        assert_eq!(models.len(), 1);
        assert!(models[0].path.ends_with("valid.gguf"));
    }

    #[test]
    fn test_discover_models_sorts_by_size() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let models_dir = temp_dir.path();

        let mut large_file = File::create(models_dir.join("large.gguf")).expect("Failed to create file");
        large_file.write_all(&vec![0u8; 1000]).expect("Failed to write");

        let mut small_file = File::create(models_dir.join("small.gguf")).expect("Failed to create file");
        small_file.write_all(&vec![0u8; 100]).expect("Failed to write");

        let models = discover_models(models_dir, MAX_VRAM_BYTES).expect("Failed to discover models");
        assert_eq!(models.len(), 2);
        assert!(models[0].path.ends_with("small.gguf"));
        assert_eq!(models[0].file_size_bytes, 100);
        assert!(models[1].path.ends_with("large.gguf"));
        assert_eq!(models[1].file_size_bytes, 1000);
    }
}
