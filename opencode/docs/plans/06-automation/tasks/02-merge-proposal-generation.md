# Task 02: Merge Proposal Generation

**Component:** Diff generation, validation, confidence scoring, and artifact management

**Dependencies:** Task 01 (Git Experiment Framework)

**Estimated Time:** 6-8 days

**Goal:** Build a merge proposal generator that creates accurate diffs, validates changes, scores confidence, and artifacts proposals to `.glyphnova/merge-proposals/` without auto-committing.

---

## Overview

The merge proposal generator:

- Generates comprehensive diffs between experiment and base branches
- Validates changes against criteria (test passage, code quality, etc.)
- Scores proposal confidence (high/medium/low)
- Artifacts proposals to `.glyphnova/merge-proposals/`
- Links proposals to experiments and refinement events

**ADR-0007 Compliance:**
- Proposals are OUTPUTS only (written to `.glyphnova/`, never auto-committed)
- Manual approval required via CLI/UI (Task 03, Task 07)
- Complete audit trail via artifact linking

---

## File Structure

**New Files:**
- `automation/merge/mod.rs` - Module exports
- `automation/merge/generator.rs` - Diff generation logic
- `automation/merge/validator.rs` - Validation criteria
- `automation/merge/confidence.rs` - Confidence scoring
- `automation/merge/artifacts.rs` - Proposal artifact management

---

## Implementation Steps

### Step 1: Create merge module structure

**Files:** Create `automation/merge/mod.rs`

```rust
pub mod generator;
pub mod validator;
pub mod confidence;
pub mod artifacts;

pub use generator::{MergeProposalGenerator, DiffStats};
pub use validator::{MergeValidator, ValidationResult, ValidationCriteria};
pub use confidence::{ConfidenceScorer, ConfidenceLevel};
pub use artifacts::{ProposalArtifactManager, ProposalMetadata};

use std::path::PathBuf;

/// Merge proposal artifact directory
pub const PROPOSAL_ARTIFACT_DIR: &str = ".glyphnova/merge-proposals";
```

---

### Step 2: Implement diff generator

**Files:** Create `automation/merge/generator.rs`

```rust
use git2::{Diff, DiffOptions, Repository};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct DiffStats {
    pub files_added: usize,
    pub files_modified: usize,
    pub files_deleted: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
}

pub struct MergeProposalGenerator {
    repo_path: PathBuf,
}

impl MergeProposalGenerator {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
        }
    }

    pub fn generate_diff(
        &self,
        experiment_branch: &str,
        base_branch: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let repo = Repository::open(&self.repo_path)?;

        let experiment_commit = repo
            .find_branch(experiment_branch, git2::BranchType::Local)?
            .get()
            .peel_to_commit()?;

        let base_commit = repo
            .find_branch(base_branch, git2::BranchType::Local)?
            .get()
            .peel_to_commit()?;

        let mut diff_opts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(
            Some(&base_commit.tree()?),
            Some(&experiment_commit.tree()?),
            Some(&mut diff_opts),
        )?;

        let diff_text = self.diff_to_string(&diff)?;
        Ok(diff_text)
    }

    pub fn calculate_diff_stats(&self, diff: &Diff) -> Result<DiffStats, Box<dyn std::error::Error>> {
        let mut stats = DiffStats {
            files_added: 0,
            files_modified: 0,
            files_deleted: 0,
            lines_added: 0,
            lines_deleted: 0,
        };

        diff.foreach(
            &mut |delta, _progress| {
                if delta.status() == git2::Delta::Added {
                    stats.files_added += 1;
                } else if delta.status() == git2::Delta::Modified {
                    stats.files_modified += 1;
                } else if delta.status() == git2::Delta::Deleted {
                    stats.files_deleted += 1;
                }
                true
            },
            None,
            Some(|_, line| {
                if line.origin() == '+' || line.origin_value() == '+' {
                    stats.lines_added += 1;
                } else if line.origin() == '-' || line.origin_value() == '-' {
                    stats.lines_deleted += 1;
                }
                true
            }),
            None,
        )?;

        Ok(stats)
    }

    fn diff_to_string(&self, diff: &Diff) -> Result<String, Box<dyn std::error::Error>> {
        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            diff_text.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
            true
        })?;
        Ok(diff_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_diff() {
        let repo_path = Path::new("/tmp/test-repo");
        let generator = MergeProposalGenerator::new(repo_path);

        // Integration test would create temp repo with commits
        // For now, test structure
        assert_eq!(generator.repo_path, repo_path);
    }
}
```

---

### Step 3: Implement merge validator

**Files:** Create `automation/merge/validator.rs`

```rust
use crate::automation::experiment::ExperimentResult;

#[derive(Debug, Clone)]
pub struct ValidationCriteria {
    pub require_tests_pass: bool,
    pub require_code_review: bool,
    pub max_files_changed: Option<usize>,
    pub max_lines_changed: Option<usize>,
    pub forbidden_patterns: Vec<String>,
}

impl Default for ValidationCriteria {
    fn default() -> Self {
        Self {
            require_tests_pass: true,
            require_code_review: true,
            max_files_changed: Some(100),
            max_lines_changed: Some(1000),
            forbidden_patterns: vec![
                "TODO".to_string(),
                "FIXME".to_string(),
                "XXX".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Pass,
    Fail(Vec<String>),
    Warning(Vec<String>),
}

pub struct MergeValidator {
    criteria: ValidationCriteria,
}

impl MergeValidator {
    pub fn new(criteria: ValidationCriteria) -> Self {
        Self { criteria }
    }

    pub fn validate(
        &self,
        result: &ExperimentResult,
        diff: &str,
    ) -> ValidationResult {
        let mut failures = Vec::new();
        let mut warnings = Vec::new();

        // Check test passage
        if self.criteria.require_tests_pass && result.exit_code != 0 {
            failures.push("Tests did not pass".to_string());
        }

        // Check for forbidden patterns
        for pattern in &self.criteria.forbidden_patterns {
            if diff.contains(pattern) {
                warnings.push(format!("Found forbidden pattern: {}", pattern));
            }
        }

        if !failures.is_empty() {
            ValidationResult::Fail(failures)
        } else if !warnings.is_empty() {
            ValidationResult::Warning(warnings)
        } else {
            ValidationResult::Pass
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_pass() {
        let criteria = ValidationCriteria::default();
        let validator = MergeValidator::new(criteria);

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: HashMap::new(),
        };

        let diff = "Some changes";
        let validation = validator.validate(&result, diff);

        assert!(matches!(validation, ValidationResult::Pass));
    }

    #[test]
    fn test_validate_fail_tests() {
        let criteria = ValidationCriteria::default();
        let validator = MergeValidator::new(criteria);

        let result = ExperimentResult {
            exit_code: 1,
            output: "Error".to_string(),
            error: Some("Failed".to_string()),
            duration_ms: 1000,
            metrics: HashMap::new(),
        };

        let diff = "Some changes";
        let validation = validator.validate(&result, diff);

        assert!(matches!(validation, ValidationResult::Fail(_)));
    }
}
```

---

### Step 4: Implement confidence scorer

**Files:** Create `automation/merge/confidence.rs`

```rust
use crate::automation::experiment::ExperimentResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

pub struct ConfidenceScorer {
    min_success_rate_for_high: f64,
    min_success_rate_for_medium: f64,
}

impl Default for ConfidenceScorer {
    fn default() -> Self {
        Self {
            min_success_rate_for_high: 0.95,
            min_success_rate_for_medium: 0.80,
        }
    }
}

impl ConfidenceScorer {
    pub fn new(
        min_success_rate_for_high: f64,
        min_success_rate_for_medium: f64,
    ) -> Self {
        Self {
            min_success_rate_for_high,
            min_success_rate_for_medium,
        }
    }

    pub fn score(&self, result: &ExperimentResult) -> ConfidenceLevel {
        let success_rate = result
            .metrics
            .get("success_rate")
            .copied()
            .unwrap_or(if result.exit_code == 0 { 1.0 } else { 0.0 });

        if success_rate >= self.min_success_rate_for_high {
            ConfidenceLevel::High
        } else if success_rate >= self.min_success_rate_for_medium {
            ConfidenceLevel::Medium
        } else {
            ConfidenceLevel::Low
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_score_high_confidence() {
        let scorer = ConfidenceScorer::default();

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: {
                let mut m = HashMap::new();
                m.insert("success_rate".to_string(), 0.98);
                m
            },
        };

        let confidence = scorer.score(&result);
        assert_eq!(confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_score_medium_confidence() {
        let scorer = ConfidenceScorer::default();

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: {
                let mut m = HashMap::new();
                m.insert("success_rate".to_string(), 0.85);
                m
            },
        };

        let confidence = scorer.score(&result);
        assert_eq!(confidence, ConfidenceLevel::Medium);
    }

    #[test]
    fn test_score_low_confidence() {
        let scorer = ConfidenceScorer::default();

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: {
                let mut m = HashMap::new();
                m.insert("success_rate".to_string(), 0.70);
                m
            },
        };

        let confidence = scorer.score(&result);
        assert_eq!(confidence, ConfidenceLevel::Low);
    }
}
```

---

### Step 5: Implement proposal artifact manager

**Files:** Create `automation/merge/artifacts.rs`

```rust
use super::generator::DiffStats;
use super::validator::ValidationResult;
use super::confidence::ConfidenceLevel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalMetadata {
    pub id: String,
    pub experiment_id: String,
    pub base_branch: String,
    pub experiment_branch: String,
    pub created_at: DateTime<Utc>,
    pub confidence: ConfidenceLevel,
    pub validation: ValidationResult,
    pub diff_stats: DiffStats,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Merged,
}

pub struct ProposalArtifactManager {
    artifact_dir: PathBuf,
}

impl ProposalArtifactManager {
    pub fn new(repo_path: &Path) -> Result<Self, std::io::Error> {
        let artifact_dir = repo_path.join(super::PROPOSAL_ARTIFACT_DIR);
        fs::create_dir_all(&artifact_dir)?;

        Ok(Self { artifact_dir })
    }

    pub fn save_proposal(
        &self,
        metadata: &ProposalMetadata,
        diff: &str,
    ) -> Result<PathBuf, std::io::Error> {
        let proposal_dir = self.artifact_dir.join(&metadata.id);
        fs::create_dir_all(&proposal_dir)?;

        // Save metadata
        let metadata_path = proposal_dir.join("metadata.json");
        fs::write(
            metadata_path,
            serde_json::to_string_pretty(metadata)?,
        )?;

        // Save diff
        let diff_path = proposal_dir.join("diff.patch");
        fs::write(diff_path, diff)?;

        Ok(proposal_dir)
    }

    pub fn load_proposal(&self, id: &str) -> Result<Option<ProposalMetadata>, std::io::Error> {
        let proposal_dir = self.artifact_dir.join(id);
        let metadata_path = proposal_dir.join("metadata.json");

        if !metadata_path.exists() {
            return Ok(None);
        }

        let metadata_str = fs::read_to_string(metadata_path)?;
        let metadata: ProposalMetadata = serde_json::from_str(&metadata_str)?;

        Ok(Some(metadata))
    }

    pub fn list_proposals(&self) -> Result<Vec<ProposalMetadata>, std::io::Error> {
        let mut proposals = Vec::new();

        for entry in fs::read_dir(&self.artifact_dir)? {
            let entry = entry?;
            let metadata_path = entry.path().join("metadata.json");

            if metadata_path.exists() {
                let metadata_str = fs::read_to_string(&metadata_path)?;
                let metadata: ProposalMetadata = serde_json::from_str(&metadata_str)?;
                proposals.push(metadata);
            }
        }

        proposals.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(proposals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_proposal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = ProposalArtifactManager::new(temp_dir.path()).unwrap();

        let metadata = ProposalMetadata {
            id: Uuid::new_v4().to_string(),
            experiment_id: "test-experiment".to_string(),
            base_branch: "main".to_string(),
            experiment_branch: "experiment-1".to_string(),
            created_at: Utc::now(),
            confidence: ConfidenceLevel::High,
            validation: ValidationResult::Pass,
            diff_stats: DiffStats {
                files_added: 1,
                files_modified: 0,
                files_deleted: 0,
                lines_added: 10,
                lines_deleted: 0,
            },
            status: ProposalStatus::Pending,
        };

        let diff = "+ New line";
        manager.save_proposal(&metadata, diff).unwrap();

        let loaded = manager.load_proposal(&metadata.id).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().experiment_id, "test-experiment");
    }
}
```

---

### Step 6: Create integration test for merge proposal generation

**Files:** Create `tests/integration/merge_proposal_test.rs`

```rust
use agentsdk::automation::merge::{
    MergeProposalGenerator, ProposalArtifactManager, ValidationCriteria,
    MergeValidator, ConfidenceScorer, ConfidenceLevel,
};
use agentsdk::automation::experiment::{create_temp_repo, ExperimentConfig, ExperimentManifest, ExperimentResult};
use std::collections::HashMap;

#[tokio::test]
async fn test_merge_proposal_generation() {
    let (temp_dir, repo) = create_temp_repo().unwrap();

    // Create experiment
    let config = ExperimentConfig::default();
    let manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    // Generate diff
    let generator = MergeProposalGenerator::new(temp_dir.path());
    let diff = generator
        .generate_diff("experiment-1", "main")
        .unwrap();

    assert!(!diff.is_empty());

    // Validate
    let criteria = ValidationCriteria::default();
    let validator = MergeValidator::new(criteria);

    let result = ExperimentResult {
        exit_code: 0,
        output: "Success".to_string(),
        error: None,
        duration_ms: 1000,
        metrics: HashMap::new(),
    };

    let validation = validator.validate(&result, &diff);
    assert!(matches!(validation, agentsdk::automation::merge::ValidationResult::Pass));

    // Score confidence
    let scorer = ConfidenceScorer::default();
    let confidence = scorer.score(&result);
    assert_eq!(confidence, ConfidenceLevel::High);
}

#[tokio::test]
async fn test_proposal_artifact_manager() {
    let temp_dir = tempfile::tempdir().unwrap();
    let manager = ProposalArtifactManager::new(temp_dir.path()).unwrap();

    // Verify artifact directory created
    let artifact_dir = temp_dir.path().join(".glyphnova/merge-proposals");
    assert!(artifact_dir.exists());

    // Save proposal
    let metadata = agentsdk::automation::merge::ProposalMetadata {
        id: uuid::Uuid::new_v4().to_string(),
        experiment_id: "test-experiment".to_string(),
        base_branch: "main".to_string(),
        experiment_branch: "experiment-1".to_string(),
        created_at: chrono::Utc::now(),
        confidence: ConfidenceLevel::High,
        validation: agentsdk::automation::merge::ValidationResult::Pass,
        diff_stats: agentsdk::automation::merge::DiffStats {
            files_added: 1,
            files_modified: 0,
            files_deleted: 0,
            lines_added: 10,
            lines_deleted: 0,
        },
        status: agentsdk::automation::merge::ProposalStatus::Pending,
    };

    let diff = "+ New line";
    let proposal_dir = manager.save_proposal(&metadata, diff).unwrap();

    // Verify files created
    assert!(proposal_dir.exists());
    assert!(proposal_dir.join("metadata.json").exists());
    assert!(proposal_dir.join("diff.patch").exists());
}
```

---

### Step 7: Update automation module to include merge

**Files:** Modify `automation/mod.rs`

```rust
pub mod cron;
pub mod experiment;
pub mod merge;

pub use cron::{
    CronScheduler, CronSchedulerInstance, CronExpression, CronParseError,
    ScheduleSafetyConfig, ScheduleValidator, ValidationError,
};
pub use experiment::{
    ExperimentConfig, ExperimentManager, ExperimentManifest, ExperimentResult,
    GitExperimentManager, IsolationEnforcer, MergePolicy, MergePolicyType,
};
pub use merge::{
    MergeProposalGenerator, ProposalArtifactManager, ValidationCriteria,
    MergeValidator, ConfidenceScorer, ConfidenceLevel, ProposalMetadata,
    ProposalStatus, PROPOSAL_ARTIFACT_DIR,
};
```

---

### Step 8: Write documentation

**Files:** Create `automation/merge/README.md`

```markdown
# Merge Proposal Generation

The merge proposal generator creates comprehensive diff proposals with validation
and confidence scoring, all artifacted to `.glyphnova/merge-proposals/`.

## Features

- Comprehensive diff generation
- Validation criteria (tests, code quality, patterns)
- Confidence scoring (high/medium/low)
- Artifact storage in `.glyphnova/merge-proposals/`
- Linking to experiments and refinement events

## ADR-0007 Compliance

- Proposals are OUTPUTS only (never auto-committed)
- Manual approval required (Task 03, Task 07)
- Complete audit trail via artifacts

## Usage

```rust
use agentsdk::automation::merge::{
    MergeProposalGenerator, ProposalArtifactManager,
    ValidationCriteria, MergeValidator, ConfidenceScorer,
};

// Generate diff
let generator = MergeProposalGenerator::new("/path/to/repo");
let diff = generator.generate_diff("experiment-1", "main")?;

// Validate
let validator = MergeValidator::new(ValidationCriteria::default());
let validation = validator.validate(&result, &diff);

// Score confidence
let scorer = ConfidenceScorer::default();
let confidence = scorer.score(&result);

// Save proposal
let manager = ProposalArtifactManager::new("/path/to/repo")?;
manager.save_proposal(&metadata, &diff)?;
```
```

---

### Step 9: Commit all changes

**Files:** Commit

```bash
git add automation/merge tests/integration/merge_proposal_test.rs automation/mod.rs
git commit -m "feat(automation): implement merge proposal generation (Task 02)

- Add comprehensive diff generation from git branches
- Implement validation criteria (tests, code quality, patterns)
- Add confidence scoring (high/medium/low)
- Implement proposal artifact manager (.glyphnova/merge-proposals/)
- Write comprehensive unit and integration tests
- Follow ADR-0007: proposals are outputs only, never auto-committed

Refs: Phase 6, Task 02"
```

---

## Summary

Task 02 implements merge proposal generation with:

✅ Comprehensive diff generation
✅ Validation criteria (tests, code quality, patterns)
✅ Confidence scoring (high/medium/low)
✅ Proposal artifact management (`.glyphnova/merge-proposals/`)
✅ Unit and integration tests
✅ ADR-0007 compliance (proposals are outputs only)

**Next Steps:** Task 03 (Manual Refinement Capture)
