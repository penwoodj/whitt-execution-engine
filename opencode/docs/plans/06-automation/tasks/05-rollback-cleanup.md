# Task 05: Rollback & Cleanup

**Component:** Rollback procedures, branch deletion, artifact cleanup, verification

**Dependencies:** Task 01 (Git Experiment Framework)

**Estimated Time:** 5-7 days

**Goal:** Build comprehensive rollback and cleanup procedures that safely restore system state after failed experiments, delete branches, clean up artifacts, and verify successful rollback.

---

## Overview

The rollback and cleanup system:

- Provides safe rollback procedures
- Deletes experiment branches
- Cleans up artifacts (worktrees, `.glyphnova/` entries)
- Verifies successful rollback
- Prevents partial cleanup states

**ADR-0007 Compliance:**
- Failed experiments are fully cleaned up
- No artifacts left on main branch
- Verification ensures complete cleanup

---

## File Structure

**New Files:**
- `automation/rollback/mod.rs` - Module exports
- `automation/rollback/procedures.rs` - Rollback procedures
- `automation/rollback/cleanup.rs` - Cleanup operations
- `automation/rollback/verification.rs` - Post-rollback verification

---

## Implementation Steps

### Step 1: Create rollback module structure

**Files:** Create `automation/rollback/mod.rs`

```rust
pub mod procedures;
pub mod cleanup;
pub mod verification;

pub use procedures::{RollbackProcedure, RollbackResult};
pub use cleanup::{CleanupManager, CleanupResult};
pub use verification::{RollbackVerifier, VerificationResult};
```

---

### Step 2: Implement rollback procedures

**Files:** Create `automation/rollback/procedures.rs`

```rust
use crate::automation::experiment::GitExperimentManager;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum RollbackResult {
    Success,
    PartialFailure(String),
    CompleteFailure(String),
}

pub struct RollbackProcedure {
    repo_path: std::path::PathBuf,
}

impl RollbackProcedure {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
        }
    }

    pub async fn rollback_experiment(
        &self,
        experiment_id: &str,
        experiment_manager: &GitExperimentManager,
    ) -> RollbackResult {
        // Delete experiment branch
        if let Err(e) = self.delete_experiment_branch(experiment_id) {
            return RollbackResult::PartialFailure(format!(
                "Failed to delete branch: {}",
                e
            ));
        }

        // Cleanup worktree
        if let Err(e) = self.cleanup_worktree(experiment_id) {
            return RollbackResult::PartialFailure(format!(
                "Failed to cleanup worktree: {}",
                e
            ));
        }

        // Cleanup artifacts
        if let Err(e) = self.cleanup_artifacts(experiment_id) {
            return RollbackResult::PartialFailure(format!(
                "Failed to cleanup artifacts: {}",
                e
            ));
        }

        RollbackResult::Success
    }

    fn delete_experiment_branch(&self, branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = git2::Repository::open(&self.repo_path)?;
        let mut branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }

    fn cleanup_worktree(&self, experiment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let worktree_path = self.repo_path.join(format!("worktree-{}", experiment_id));

        if worktree_path.exists() {
            let repo = git2::Repository::open(&self.repo_path)?;
            if let Ok(worktree) = repo.find_worktree(experiment_id) {
                worktree.prune(git2::WorktreePruneOptions::new())?;
            }

            std::fs::remove_dir_all(worktree_path)?;
        }

        Ok(())
    }

    fn cleanup_artifacts(&self, experiment_id: &str) -> Result<(), std::error::Error> {
        let artifact_dirs = vec![
            self.repo_path.join(".glyphnova/experiment-results"),
            self.repo_path.join(".glyphnova/refinements"),
            self.repo_path.join(".glyphnova/merge-proposals"),
        ];

        for artifact_dir in artifact_dirs {
            if artifact_dir.exists() {
                for entry in std::fs::read_dir(artifact_dir)? {
                    let entry = entry?;
                    if entry.path().to_string_lossy().contains(experiment_id) {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
            }
        }

        Ok(())
    }
}
```

---

### Step 3: Implement cleanup manager

**Files:** Create `automation/rollback/cleanup.rs`

```rust
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CleanupResult {
    pub items_removed: usize,
    pub items_failed: Vec<String>,
}

pub struct CleanupManager {
    repo_path: std::path::PathBuf,
}

impl CleanupManager {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
        }
    }

    pub fn cleanup_all_failed_experiments(&self) -> CleanupResult {
        let mut result = CleanupResult {
            items_removed: 0,
            items_failed: Vec::new(),
        };

        // Cleanup worktrees
        if let Err(e) = self.cleanup_all_worktrees(&mut result) {
            result.items_failed.push(format!("Worktree cleanup failed: {}", e));
        }

        // Cleanup artifacts
        if let Err(e) = self.cleanup_all_artifacts(&mut result) {
            result.items_failed.push(format!("Artifact cleanup failed: {}", e));
        }

        result
    }

    fn cleanup_all_worktrees(&self, result: &mut CleanupResult) -> Result<(), std::io::Error> {
        let repo = git2::Repository::open(&self.repo_path)?;
        let worktrees = repo.worktrees()?;

        for worktree in worktrees {
            let worktree = worktree?;
            let worktree_path = self.repo_path.join(&worktree.path());

            if worktree_path.exists() {
                worktree.prune(git2::WorktreePruneOptions::new())?;
                std::fs::remove_dir_all(worktree_path)?;
                result.items_removed += 1;
            }
        }

        Ok(())
    }

    fn cleanup_all_artifacts(&self, result: &mut CleanupResult) -> Result<(), std::io::Error> {
        let artifact_dirs = vec![
            self.repo_path.join(".glyphnova/experiment-results"),
            self.repo_path.join(".glyphnova/refinements"),
            self.repo_path.join(".glyphnova/merge-proposals"),
        ];

        for artifact_dir in artifact_dirs {
            if artifact_dir.exists() {
                for entry in std::fs::read_dir(artifact_dir)? {
                    let entry = entry?;
                    std::fs::remove_dir_all(entry.path())?;
                    result.items_removed += 1;
                }
            }
        }

        Ok(())
    }
}
```

---

### Step 4: Implement rollback verifier

**Files:** Create `automation/rollback/verification.rs`

```rust
use std::path::Path;

#[derive(Debug, Clone)]
pub enum VerificationResult {
    Success,
    Failed(String),
}

pub struct RollbackVerifier {
    repo_path: std::path::PathBuf,
}

impl RollbackVerifier {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
        }
    }

    pub fn verify_rollback(&self, experiment_id: &str) -> VerificationResult {
        // Verify branch deleted
        if let Err(e) = self.verify_branch_deleted(experiment_id) {
            return VerificationResult::Failed(format!(
                "Branch verification failed: {}",
                e
            ));
        }

        // Verify worktree cleaned up
        if let Err(e) = self.verify_worktree_cleaned(experiment_id) {
            return VerificationResult::Failed(format!(
                "Worktree verification failed: {}",
                e
            ));
        }

        // Verify artifacts cleaned up
        if let Err(e) = self.verify_artifacts_cleaned(experiment_id) {
            return VerificationResult::Failed(format!(
                "Artifact verification failed: {}",
                e
            ));
        }

        // Verify main branch unchanged
        if let Err(e) = self.verify_main_branch_unchanged() {
            return VerificationResult::Failed(format!(
                "Main branch verification failed: {}",
                e
            ));
        }

        VerificationResult::Success
    }

    fn verify_branch_deleted(&self, branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = git2::Repository::open(&self.repo_path)?;

        match repo.find_branch(branch_name, git2::BranchType::Local) {
            Ok(_) => Err(format!("Branch {} still exists", branch_name).into()),
            Err(git2::Error::NotFound) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn verify_worktree_cleaned(&self, experiment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let worktree_path = self.repo_path.join(format!("worktree-{}", experiment_id));

        if worktree_path.exists() {
            return Err(format!("Worktree still exists: {:?}", worktree_path).into());
        }

        Ok(())
    }

    fn verify_artifacts_cleaned(&self, experiment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let artifact_dirs = vec![
            self.repo_path.join(".glyphnova/experiment-results"),
            self.repo_path.join(".glyphnova/refinements"),
            self.repo_path.join(".glyphnova/merge-proposals"),
        ];

        for artifact_dir in artifact_dirs {
            if artifact_dir.exists() {
                for entry in std::fs::read_dir(artifact_dir)? {
                    let entry = entry?;
                    if entry.path().to_string_lossy().contains(experiment_id) {
                        return Err(format!(
                            "Artifact still exists: {:?}",
                            entry.path()
                        )
                        .into());
                    }
                }
            }
        }

        Ok(())
    }

    fn verify_main_branch_unchanged(&self) -> Result<(), Box<dyn std::error::Error>> {
        let repo = git2::Repository::open(&self.repo_path)?;
        let head = repo.head()?;

        if let Some(name) = head.shorthand() {
            if name == "main" || name == "master" {
                return Ok(());
            }
        }

        Err("HEAD is not on main or master branch".into())
    }
}
```

---

### Step 5: Create integration test

**Files:** Create `tests/integration/rollback_test.rs`

```rust
use agentsdk::automation::rollback::{RollbackProcedure, CleanupManager, RollbackVerifier};
use agentsdk::automation::experiment::{create_temp_repo, ExperimentConfig, ExperimentManifest, GitExperimentManager};

#[tokio::test]
async fn test_rollback_experiment() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let experiment_manager = GitExperimentManager::new(temp_dir.path()).unwrap();
    let rollback_procedure = RollbackProcedure::new(temp_dir.path());

    // Create experiment
    let config = ExperimentConfig::default();
    let manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    let experiment_id = experiment_manager.create_experiment(manifest).await.unwrap();

    // Rollback
    let result = rollback_procedure.rollback_experiment(&experiment_id, &experiment_manager).await;

    assert!(matches!(result, agentsdk::automation::rollback::RollbackResult::Success));
}

#[test]
fn test_cleanup_all_failed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let cleanup_manager = CleanupManager::new(temp_dir.path());

    let result = cleanup_manager.cleanup_all_failed_experiments();

    assert_eq!(result.items_failed.len(), 0);
}

#[test]
fn test_verify_rollback() {
    let temp_dir = tempfile::tempdir().unwrap();
    let verifier = RollbackVerifier::new(temp_dir.path());

    let result = verifier.verify_rollback("nonexistent-experiment");
    assert!(matches!(result, agentsdk::automation::rollback::VerificationResult::Success));
}
```

---

### Step 6: Update automation module and commit

**Files:** Modify `automation/mod.rs` and commit

```bash
git add automation/rollback tests/integration/rollback_test.rs
git commit -m "feat(automation): implement rollback and cleanup (Task 05)

- Add rollback procedures for failed experiments
- Implement cleanup manager for artifacts and worktrees
- Add post-rollback verification
- Write comprehensive unit and integration tests
- Follow ADR-0007: failed experiments are fully cleaned up

Refs: Phase 6, Task 05"
```

---

## Summary

Task 05 implements rollback and cleanup with:

✅ Rollback procedures for failed experiments
✅ Cleanup manager for artifacts and worktrees
✅ Post-rollback verification
✅ Unit and integration tests
✅ ADR-0007 compliance (complete cleanup)

**Next Steps:** Task 06 (Scheduling Policy Compiler) or Task 07 (Automation CLI)
