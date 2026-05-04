//! Model selector for diverse benchmark sampling.
//!
//! Selects models from candidates with diversity across:
//! - Size tiers (0.5-1.5B, 1.5-3B, 3-4B, 4-6B)
//! - Architectures (Qwen, Llama, Mistral, Gemma, Phi, Falcon, etc.)
//! - Authors/organizations

use crate::client::model_discovery::ModelCandidate;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

/// Size tiers for model selection based on GGUF file size (proxy for parameter count).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeTier {
    Tiny,      // 0.5-1.5B (~0.5-1.5GB)
    Small,     // 1.5-3B (~1.5-3GB)
    Medium,    // 3-4B (~3-4GB)
    Large,     // 4-6B (~4-6GB)
}

 impl SizeTier {
    /// Determine tier from file size in bytes.
    pub fn from_file_size(file_size_bytes: u64) -> Self {
        match file_size_bytes {
            s if s <= 1_500_000_000 => SizeTier::Tiny,
            s if s <= 3_000_000_000 => SizeTier::Small,
            s if s <= 4_000_000_000 => SizeTier::Medium,
            s if s <= 6_000_000_000 => SizeTier::Large,
            _ => SizeTier::Large, // Fallback: treat as Large
        }
    }

    /// Get max file size for this tier (bytes).
    pub fn max_size_bytes(&self) -> u64 {
        match self {
            SizeTier::Tiny => 1_500_000_000,
            SizeTier::Small => 3_000_000_000,
            SizeTier::Medium => 4_000_000_000,
            SizeTier::Large => 6_000_000_000,
        }
    }
}

/// Model selector with tier-diverse, architecture-diverse, author-diverse selection.
pub struct ModelSelector;

impl ModelSelector {
    /// Select N models from candidates with diversity optimization.
    ///
    /// Selection strategy:
    /// 1. Allocate models to size tiers proportionally to available candidates
    /// 2. Within each tier, prioritize architecture diversity
    /// 3. Within each architecture, prioritize author diversity
    /// 4. Prefer Q4_K_M quantization and instruct/chat variants
    pub fn select_n_models(candidates: &[ModelCandidate], n: usize) -> Vec<ModelCandidate> {
        if candidates.is_empty() {
            warn!("[model_selector] no candidates available");
            return Vec::new();
        }

        if n == 0 {
            debug!("[model_selector] requested 0 models");
            return Vec::new();
        }

        if n >= candidates.len() {
            info!("[model_selector] requested {} models, returning all {} candidates",
                n, candidates.len());
            return candidates.iter().map(|c| ModelCandidate {
                path: c.path.clone(),
                model_id: c.model_id.clone(),
                file_size_bytes: c.file_size_bytes,
                estimated_vram_bytes: c.estimated_vram_bytes,
                fits_in_vram: c.fits_in_vram,
                author: c.author.clone(),
            }).collect();
        }

        // Group candidates by size tier
        let mut tier_buckets: HashMap<SizeTier, Vec<&ModelCandidate>> = HashMap::new();
        for candidate in candidates {
            let tier = SizeTier::from_file_size(candidate.file_size_bytes);
            tier_buckets.entry(tier).or_default().push(candidate);
        }

        // Calculate tier allocation (proportional to available models)
        let mut tier_allocation: HashMap<SizeTier, usize> = HashMap::new();

        for (tier, models) in tier_buckets.iter() {
            let tier_count = models.len();
            let allocation = ((tier_count * n) as f64 / candidates.len() as f64).round() as usize;
            let allocation = allocation.min(tier_count);
            tier_allocation.insert(*tier, allocation.max(1)); // At least 1 per tier with candidates
        }

        // Adjust if allocation doesn't sum to n
        let allocated_sum: usize = tier_allocation.values().sum();
        if allocated_sum != n {
            let diff = (n as i64 - allocated_sum as i64).unsigned_abs() as usize;
            debug!("[model_selector] adjusting allocation: sum={}, target={}, diff={}",
                allocated_sum, n, diff);

            // Add to tiers with most available candidates
            let mut tiers_with_capacity: Vec<_> = tier_buckets.iter()
                .filter(|(tier, models)| {
                    tier_allocation.get(tier).copied().unwrap_or(0) < models.len()
                })
                .collect();
            tiers_with_capacity.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

            for (tier, models) in tiers_with_capacity.iter().take(diff) {
                let current = tier_allocation.get(tier).copied().unwrap_or(0);
                tier_allocation.insert(**tier, (current + 1).min(models.len()));
            }
        }

        // Calculate tier allocation (proportional to available models)
        let mut tier_allocation: HashMap<SizeTier, usize> = HashMap::new();

        for (tier, models) in tier_buckets.iter() {
            let tier_count = models.len();
            let allocation = ((tier_count * n) as f64 / candidates.len() as f64).round() as usize;
            let allocation = allocation.min(tier_count);
            tier_allocation.insert(*tier, allocation.max(1)); // At least 1 per tier with candidates
        }

        // Adjust if allocation doesn't sum to n
        let allocated_sum: usize = tier_allocation.values().sum();
        if allocated_sum != n {
            let diff = (n as i64 - allocated_sum as i64).unsigned_abs() as usize;
            debug!("[model_selector] adjusting allocation: sum={}, target={}, diff={}",
                allocated_sum, n, diff);

            // Add to tiers with most available candidates
            let mut tiers_with_capacity: Vec<_> = tier_buckets.iter()
                .filter(|(tier, models)| {
                    tier_allocation.get(tier).copied().unwrap_or(0) < models.len()
                })
                .collect();
            tiers_with_capacity.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

            for (tier, models) in tiers_with_capacity.iter().take(diff) {
                let current = tier_allocation.get(*tier).copied().unwrap_or(0);
                tier_allocation.insert(**tier, (current + 1).min(models.len()));
            }
        }

        info!("[model_selector] tier allocation: {:?}", tier_allocation);

        // Select models from each tier with architecture/author diversity
        let mut selected: Vec<ModelCandidate> = Vec::new();
        for tier in [SizeTier::Tiny, SizeTier::Small, SizeTier::Medium, SizeTier::Large] {
            if let Some(&allocation) = tier_allocation.get(&tier) {
                if allocation > 0 {
                    if let Some(candidates_in_tier) = tier_buckets.get(&tier) {
                        let tier_selected = Self::select_from_tier(candidates_in_tier, allocation);
                        selected.extend(tier_selected);
                    }
                }
            }
        }

        // Trim if we somehow selected too many
        selected.truncate(n);
        info!("[model_selector] selected {} models from {} candidates",
            selected.len(), candidates.len());

        selected
    }

    /// Select models from a single tier with architecture/author diversity.
    fn select_from_tier(candidates: &[&ModelCandidate], n: usize) -> Vec<ModelCandidate> {
        if candidates.is_empty() || n == 0 {
            return Vec::new();
        }

        if n >= candidates.len() {
            return candidates.iter().map(|c| ModelCandidate {
                path: c.path.clone(),
                model_id: c.model_id.clone(),
                file_size_bytes: c.file_size_bytes,
                estimated_vram_bytes: c.estimated_vram_bytes,
                fits_in_vram: c.fits_in_vram,
                author: c.author.clone(),
            }).collect();
        }

        // Score candidates based on:
        // 1. Architecture diversity (unique architecture names)
        // 2. Author diversity (unique authors)
        // 3. Quantization preference (Q4_K_M > others)
        // 4. Specialization preference (instruct/chat > base)

        let mut scored: Vec<(usize, ModelCandidate)> = candidates.iter().enumerate().map(|(i, c)| {
            (i, ModelCandidate {
                path: c.path.clone(),
                model_id: c.model_id.clone(),
                file_size_bytes: c.file_size_bytes,
                estimated_vram_bytes: c.estimated_vram_bytes,
                fits_in_vram: c.fits_in_vram,
                author: c.author.clone(),
            })
        }).collect();

        // Group by architecture and author
        let mut arch_counts: HashMap<String, usize> = HashMap::new();
        let mut author_counts: HashMap<String, usize> = HashMap::new();

        for (_, candidate) in &scored {
            let arch = Self::extract_architecture(&candidate.model_id);
            let author = &candidate.author;
            *arch_counts.entry(arch.clone()).or_insert(0) += 1;
            *author_counts.entry(author.clone()).or_insert(0) += 1;
        }

        // Sort by diversity score
        scored.sort_by(|a, b| {
            let a_arch = Self::extract_architecture(&a.1.model_id);
            let b_arch = Self::extract_architecture(&b.1.model_id);
            let a_author = &a.1.author;
            let b_author = &b.1.author;

            // Prefer architectures with fewer selections
            let a_arch_score = arch_counts.get(&a_arch).copied().unwrap_or(0);
            let b_arch_score = arch_counts.get(&b_arch).copied().unwrap_or(0);

            // Prefer authors with fewer selections
            let a_author_score = author_counts.get(a_author).copied().unwrap_or(0);
            let b_author_score = author_counts.get(b_author).copied().unwrap_or(0);

            // Prefer Q4_K_M quantization
            let a_q4 = a.1.model_id.contains("Q4_K_M") || a.1.model_id.contains("q4km");
            let b_q4 = b.1.model_id.contains("Q4_K_M") || b.1.model_id.contains("q4km");

            // Prefer instruct/chat variants
            let a_instruct = a.1.model_id.contains("instruct") || a.1.model_id.contains("chat");
            let b_instruct = b.1.model_id.contains("instruct") || b.1.model_id.contains("chat");

            // Comparison: minimize arch count, minimize author count, maximize Q4, maximize instruct
            a_arch_score.cmp(&b_arch_score)
                .then_with(|| a_author_score.cmp(&b_author_score))
                .then_with(|| b_q4.cmp(&a_q4))
                .then_with(|| b_instruct.cmp(&a_instruct))
        });

        // Greedy selection with diversity constraints
        let mut selected: Vec<ModelCandidate> = Vec::new();
        let mut used_archs: HashSet<String> = HashSet::new();
        let mut used_authors: HashSet<String> = HashSet::new();

        for (_, candidate) in &scored {
            if selected.len() >= n {
                break;
            }

            let arch = Self::extract_architecture(&candidate.model_id);
            let author = &candidate.author;

            // Skip if we already have this architecture (unless we need more models)
            let allow_repeat_arch = used_archs.len() >= 4 || selected.len() < n / 2;

            if !allow_repeat_arch && used_archs.contains(&arch) {
                continue;
            }

            selected.push(ModelCandidate {
                path: candidate.path.clone(),
                model_id: candidate.model_id.clone(),
                file_size_bytes: candidate.file_size_bytes,
                estimated_vram_bytes: candidate.estimated_vram_bytes,
                fits_in_vram: candidate.fits_in_vram,
                author: candidate.author.clone(),
            });
            used_archs.insert(arch);
            used_authors.insert(author.clone());
        }

        // If we still need more, fill with remaining candidates
        if selected.len() < n {
            let already_selected: HashSet<_> = selected.iter()
                .map(|c| c.path.clone())
                .collect();

            for candidate in candidates {
                if selected.len() >= n {
                    break;
                }

                if !already_selected.contains(&candidate.path) {
                    selected.push(ModelCandidate {
                        path: candidate.path.clone(),
                        model_id: candidate.model_id.clone(),
                        file_size_bytes: candidate.file_size_bytes,
                        estimated_vram_bytes: candidate.estimated_vram_bytes,
                        fits_in_vram: candidate.fits_in_vram,
                        author: candidate.author.clone(),
                    });
                }
            }
        }

        selected
    }

    /// Extract architecture name from model ID.
    fn extract_architecture(model_id: &str) -> String {
        let lower = model_id.to_lowercase();

        // Known architectures
        if lower.contains("qwen") {
            return "Qwen".to_string();
        }
        if lower.contains("llama") || lower.contains("llama2") || lower.contains("llama3") {
            return "Llama".to_string();
        }
        if lower.contains("mistral") {
            return "Mistral".to_string();
        }
        if lower.contains("gemma") {
            return "Gemma".to_string();
        }
        if lower.contains("phi") {
            return "Phi".to_string();
        }
        if lower.contains("falcon") {
            return "Falcon".to_string();
        }
        if lower.contains("granite") {
            return "Granite".to_string();
        }
        if lower.contains("yi") {
            return "Yi".to_string();
        }
        if lower.contains("deepseek") {
            return "DeepSeek".to_string();
        }
        if lower.contains("internlm") {
            return "InternLM".to_string();
        }

        // Fallback: extract first word
        model_id.split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string()
    }
}

 #[cfg(test)]
 mod tests {
     use super::*;
     use std::path::PathBuf;

     fn create_candidate(
        path: &str,
        model_id: &str,
        file_size_bytes: u64,
        author: &str,
    ) -> ModelCandidate {
        ModelCandidate {
            path: PathBuf::from(path),
            model_id: model_id.to_string(),
            file_size_bytes,
            estimated_vram_bytes: file_size_bytes,
            fits_in_vram: true,
            author: author.to_string(),
        }
    }

    #[test]
    fn test_size_tier_classification() {
        // Tiny: 0.5-1.5B (~0.5-1.5GB)
        assert_eq!(SizeTier::from_file_size(1_000_000_000), SizeTier::Tiny);
        assert_eq!(SizeTier::from_file_size(1_500_000_000), SizeTier::Tiny);

        // Small: 1.5-3B (~1.5-3GB)
        assert_eq!(SizeTier::from_file_size(1_500_000_001), SizeTier::Small);
        assert_eq!(SizeTier::from_file_size(3_000_000_000), SizeTier::Small);

        // Medium: 3-4B (~3-4GB)
        assert_eq!(SizeTier::from_file_size(3_000_000_001), SizeTier::Medium);
        assert_eq!(SizeTier::from_file_size(4_000_000_000), SizeTier::Medium);

        // Large: 4-6B (~4-6GB)
        assert_eq!(SizeTier::from_file_size(4_000_000_001), SizeTier::Large);
        assert_eq!(SizeTier::from_file_size(6_000_000_000), SizeTier::Large);
    }

    #[test]
    fn test_select_from_empty_candidates() {
        let candidates: Vec<ModelCandidate> = Vec::new();
        let selected = ModelSelector::select_n_models(&candidates, 3);
        assert!(selected.is_empty());
    }

    #[test]
    fn test_select_zero_models() {
        let candidates = vec![
            create_candidate("/models/model1.gguf", "model 1", 1_000_000_000, "test"),
        ];
        let selected = ModelSelector::select_n_models(&candidates, 0);
        assert!(selected.is_empty());
    }

    #[test]
    fn test_select_more_than_available() {
        let candidates = vec![
            create_candidate("/models/model1.gguf", "model 1", 1_000_000_000, "test"),
            create_candidate("/models/model2.gguf", "model 2", 2_000_000_000, "test"),
        ];
        let selected = ModelSelector::select_n_models(&candidates, 5);
        assert_eq!(selected.len(), 2);
    }

    #[test]
    fn test_select_all_candidates() {
        let candidates = vec![
            create_candidate("/models/model1.gguf", "model 1", 1_000_000_000, "test"),
            create_candidate("/models/model2.gguf", "model 2", 2_000_000_000, "test"),
        ];
        let selected = ModelSelector::select_n_models(&candidates, 2);
        assert_eq!(selected.len(), 2);
    }

    #[test]
    fn test_tier_allocation_proportional() {
        let candidates = vec![
            create_candidate("/models/tiny1.gguf", "tiny 1", 1_000_000_000, "test"),  // Tiny
            create_candidate("/models/tiny2.gguf", "tiny 2", 1_000_000_000, "test"),  // Tiny
            create_candidate("/models/small1.gguf", "small 1", 2_000_000_000, "test"), // Small
            create_candidate("/models/medium1.gguf", "medium 1", 3_500_000_000, "test"), // Medium
            create_candidate("/models/large1.gguf", "large 1", 5_000_000_000, "test"), // Large
        ];

        let selected = ModelSelector::select_n_models(&candidates, 3);

        // Should pick from diverse tiers
        let tiers: HashSet<SizeTier> = selected.iter()
            .map(|c| SizeTier::from_file_size(c.file_size_bytes))
            .collect();

        // With 3 models and 4 tiers, we should get diverse representation
        assert!(tiers.len() >= 2, "Should have diverse tier representation");
    }

    #[test]
    fn test_architecture_diversity() {
        let candidates = vec![
            create_candidate("/models/qwen.gguf", "qwen 4b q4km", 2_500_000_000, "alibaba"),
            create_candidate("/models/llama.gguf", "llama 3 2b q4km", 2_000_000_000, "meta"),
            create_candidate("/models/mistral.gguf", "mistral 7b q4km", 4_500_000_000, "mistral"),
            create_candidate("/models/gemma.gguf", "gemma 2b q4km", 1_800_000_000, "google"),
            create_candidate("/models/phi.gguf", "phi 3 mini q4km", 2_300_000_000, "microsoft"),
        ];

        let selected = ModelSelector::select_n_models(&candidates, 3);

        // Should have diverse architectures
        let archs: HashSet<String> = selected.iter()
            .map(|c| ModelSelector::extract_architecture(&c.model_id))
            .collect();

        assert!(archs.len() >= 2, "Should have diverse architectures");
    }

    #[test]
    fn test_extract_architecture() {
        assert_eq!(ModelSelector::extract_architecture("qwen 4b instruct q4km"), "Qwen");
        assert_eq!(ModelSelector::extract_architecture("llama 3 2b chat q4km"), "Llama");
        assert_eq!(ModelSelector::extract_architecture("mistral 7b instruct q4km"), "Mistral");
        assert_eq!(ModelSelector::extract_architecture("gemma 2b it q4km"), "Gemma");
        assert_eq!(ModelSelector::extract_architecture("phi 3 mini q4km"), "Phi");
        assert_eq!(ModelSelector::extract_architecture("falcon 7b q4km"), "Falcon");
    }
}
